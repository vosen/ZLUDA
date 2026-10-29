//! Compilation of ZLUDA's LLVM modules to AIR (ptx::Target::Air).
//!
//! The PTX front end emits the module directly in AIR's terms ("air-kernel" kernels, AIR's system values and
//! SIMD-group functions through the primitives of ptx/lib/zluda_ptx_impl_air.ll), so nothing AGX-specific is in it:
//!
//! ```text
//! PTX -> portable AIR (NAME.portable.air) -+-> the AIR target's Metal pipeline (NAME.air, for Metal)
//!                                           +-> the AGX backend, agx-lower-air (NAME.o / NAME.asm)
//! ```
//!
//! NAME.air is the portable AIR with what only Metal needs from ZLUDA (the .global variables as one buffer,
//! NAME.air.globals; the shared memory registers in Metal's numbering); llc
//! -mtriple=air -filetype=obj packages it for Metal.

use crate::compile_agx::{
    init_globals, internalize_functions, internalize_helpers, load_module, Diagnostics, AGX_DEFAULT_CPU, AGX_TRIPLE,
};
use crate::utils::{Context, Message, Module, PassBuilderOptions, TargetMachine};
use crate::{
    LLVMZludaAgxKeepGlobals, LLVMZludaAgxLowerAddressSpaces, LLVMZludaAgxLowerClusters, LLVMZludaAgxSpillClusterShared,
    LLVMZludaAirFunctionPointers, LLVMZludaSetAirPrimitives, LLVMZludaAirGlobalsToBuffer, LLVMZludaAirInlineGlobalUsers,
    LLVMZludaAirRemapAddressSpaces, LLVMZludaAirSharedLayout, LLVMZludaAirStaticRounding, LLVMZludaAirCanonicalNaNs, LLVMZludaAgxCanonicalizeFoldedNaNs, LLVMZludaLinkUsedDefinitions,
    LLVMZludaUpgradeFunctionAttributes,
};
use llvm_sys::target_machine::{
    LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMGetTargetFromTriple, LLVMRelocMode,
};
use llvm_sys::transforms::pass_builder::LLVMRunPasses;
use std::ffi::{CStr, CString};

pub const AIR_TRIPLE: &CStr = c"air";

/// The result of compiling for the Air target.
pub struct AirCompilation {
    /// The portable AIR, for any AIR consumer.
    pub portable: Vec<u8>,
    /// The AIR for Metal (packaged by the AIR target's Metal pipeline).
    pub metal: Vec<u8>,
    /// NAME.air.globals: the .global variables' buffer of the Metal AIR (JSON), if the module has any.
    pub metal_globals: Option<String>,
    /// The AGX object compiled from the portable AIR, when an AGX processor was given.
    pub agx_object: Option<Vec<u8>>,
}

fn run(module: &Module, pipeline: &CStr, tm: &TargetMachine, options: &PassBuilderOptions) -> Result<(), String> {
    let error = unsafe { LLVMRunPasses(module.get(), pipeline.as_ptr(), tm.get(), options.get()) };
    if error.is_null() {
        return Ok(());
    }
    let err_msg = unsafe { llvm_sys::error::LLVMGetErrorMessage(error) };
    Err(Message::new(unsafe { CStr::from_ptr(err_msg) }).to_str().to_string())
}

fn target_machine(triple: &CStr, cpu: &CStr) -> Result<TargetMachine, String> {
    let mut target = unsafe { std::mem::zeroed() };
    let mut err = std::ptr::null_mut();
    if unsafe { LLVMGetTargetFromTriple(triple.as_ptr(), &mut target, &mut err) } != 0 {
        return Err(Message::new(unsafe { CStr::from_ptr(err) }).to_str().to_string());
    }
    Ok(TargetMachine::new(
        target,
        triple,
        cpu,
        c"",
        LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
        LLVMRelocMode::LLVMRelocDefault,
        LLVMCodeModel::LLVMCodeModelDefault,
    ))
}

fn bitcode(module: &Module) -> Vec<u8> {
    let buffer = unsafe { llvm_sys::bit_writer::LLVMWriteBitcodeToMemoryBuffer(module.get()) };
    let bytes = unsafe {
        std::slice::from_raw_parts(
            llvm_sys::core::LLVMGetBufferStart(buffer) as *const u8,
            llvm_sys::core::LLVMGetBufferSize(buffer),
        )
        .to_vec()
    };
    unsafe { llvm_sys::core::LLVMDisposeMemoryBuffer(buffer) };
    bytes
}

/// Compiles the front end's module (and `others`, linked) for the Air target. `agx_cpu`: also compile the
/// portable AIR for this AGX processor.
pub fn compile_air(
    ctx: &Context,
    main: Module,
    others: &[&[u8]],
    ptx_impl: &[u8],
    agx_cpu: Option<&str>,
    compiler_hook: Option<&dyn Fn(&Vec<u8>, String)>,
    object_only: bool,
    strict_fp: bool,
) -> Result<AirCompilation, String> {
    // object_only: the compiled outputs (the AIR, the globals buffer, the AGX object) only, not the IR text or
    // the AGX assembly, which take as long again to make
    let text_hook = compiler_hook.filter(|_| !object_only);
    if ptx_impl.is_empty() {
        return Err("the Air target's helpers were not built (ptx/build.rs needs ZLUDA_LLVM_BUILD_DIR)".to_string());
    }
    init_globals()?;
    let diagnostics = Diagnostics::new(ctx);
    let linked = Module::new(ctx, c"llvm-link");
    linked.link(main)?;
    for (i, ir) in others.iter().enumerate() {
        let name = CString::new(format!("linked_module_{}", i + 1)).unwrap();
        linked.link(load_module(ctx, ir, &name)?)?;
    }
    if !others.is_empty() {
        internalize_functions(&linked);
    }
    unsafe { LLVMZludaAgxKeepGlobals(linked.get()) };
    let mut err = std::ptr::null_mut();
    let linked_ok = unsafe {
        LLVMZludaLinkUsedDefinitions(linked.get(), ptx_impl.as_ptr().cast(), ptx_impl.len(), c"zluda_ptx_impl_air".as_ptr(), &mut err)
    };
    if !linked_ok {
        return Err(Message::new(unsafe { CStr::from_ptr(err) }).to_str().to_string());
    }
    internalize_helpers(&linked);
    unsafe { LLVMZludaUpgradeFunctionAttributes(linked.get()) };
    let air = target_machine(AIR_TRIPLE, c"")?;
    let options = PassBuilderOptions::new();
    let fail = |err: *mut std::ffi::c_char| Message::new(unsafe { CStr::from_ptr(err) }).to_str().to_string();
    // ZLUDA's lowering of what PTX has and the GPU has not, on its own IR (ZLUDA's address spaces, the
    // __zluda_air.* primitives), as for the Agx target: clusters as threadgroups, isspacep and the generic
    // addresses of shared and private memory, cluster shared memory over 32 KiB in device memory
    // ZLUDA's address spaces have the layout of the AGX target (32-bit shared and private pointers) until the
    // remap, with 16- and 32-bit native integers: GPUs have no 64-bit integer unit, and a 64-bit native width
    // lets the optimizations widen 32-bit arithmetic (int8 dot products as 64-bit multiplies; EXPERIMENTS.md
    // L14-E1). Without a layout they would see no native integer width and 64-bit pointers everywhere.
    {
        let zluda_layout = target_machine(AGX_TRIPLE, &CString::new(AGX_DEFAULT_CPU).unwrap())?;
        unsafe {
            let layout = llvm_sys::target_machine::LLVMCreateTargetDataLayout(zluda_layout.get());
            let text = Message::new(CStr::from_ptr(llvm_sys::target::LLVMCopyStringRepOfTargetData(layout)))
                .to_str()
                .replace("-n16:32:64", "-n16:32");
            llvm_sys::target::LLVMDisposeTargetData(layout);
            let text = CString::new(text).unwrap();
            llvm_sys::core::LLVMSetDataLayout(linked.get(), text.as_ptr());
        }
    }
    // ZLUDA_AIR_TIMINGS: the time of each stage, on stderr
    let timings = std::env::var_os("ZLUDA_AIR_TIMINGS").is_some();
    let mut last = std::time::Instant::now();
    let mut stage = |name: &str| {
        if timings {
            eprintln!("[zluda air] {name}: {:.3} s", last.elapsed().as_secs_f64());
        }
        last = std::time::Instant::now();
    };
    unsafe { LLVMZludaSetAirPrimitives(1) };
    let lowered = (|| -> Result<(), String> {
        run(&linked, c"default<O3>", &air, &options)?;
        let mut err = std::ptr::null_mut();
        let clusters = unsafe { LLVMZludaAgxLowerClusters(linked.get(), &mut err) };
        if clusters < 0 {
            return Err(fail(err));
        }
        for i in 0..4 {
            let lowered = unsafe { LLVMZludaAgxLowerAddressSpaces(linked.get()) };
            if !lowered && !(i == 0 && clusters > 0) {
                break;
            }
            run(&linked, c"default<O3>", &air, &options)?;
        }
        let spilled = unsafe { LLVMZludaAgxSpillClusterShared(linked.get(), &mut err) };
        if spilled < 0 {
            return Err(fail(err));
        }
        if spilled > 0 {
            run(&linked, c"default<O3>", &air, &options)?;
        }
        Ok(())
    })();
    unsafe { LLVMZludaSetAirPrimitives(0) };
    lowered?;
    stage("ZLUDA lowering (O3 in ZLUDA's address spaces)");
    // the primitives that lowering called anew, after O3 dropped the unused definitions
    let mut err = std::ptr::null_mut();
    if !unsafe {
        LLVMZludaLinkUsedDefinitions(linked.get(), ptx_impl.as_ptr().cast(), ptx_impl.len(), c"zluda_ptx_impl_air".as_ptr(), &mut err)
    } {
        return Err(fail(err));
    }
    // PTX's rounding modifiers as the static rounding of each operation (AIR has no mode register)
    unsafe { LLVMZludaAirStaticRounding(linked.get()) };
    // the AIR primitives internal once nothing creates new calls of them, so that the unused ones go
    let mut f = unsafe { llvm_sys::core::LLVMGetFirstFunction(linked.get()) };
    while !f.is_null() {
        let mut len = 0;
        let name = unsafe { std::slice::from_raw_parts(llvm_sys::core::LLVMGetValueName2(f, &mut len) as *const u8, len) };
        if name.starts_with(b"__zluda_air.") && unsafe { llvm_sys::core::LLVMIsDeclaration(f) } == 0 {
            unsafe { llvm_sys::core::LLVMSetLinkage(f, llvm_sys::LLVMLinkage::LLVMInternalLinkage) };
        }
        f = unsafe { llvm_sys::core::LLVMGetNextFunction(f) };
    }
    // AIR's address spaces, triple and data layout
    let mut err = std::ptr::null_mut();
    if unsafe { LLVMZludaAirRemapAddressSpaces(linked.get(), &mut err) } < 0 {
        return Err(fail(err));
    }
    unsafe {
        llvm_sys::core::LLVMSetTarget(linked.get(), AIR_TRIPLE.as_ptr());
        let layout = llvm_sys::target_machine::LLVMCreateTargetDataLayout(air.get());
        llvm_sys::target::LLVMSetModuleDataLayout(linked.get(), layout);
        llvm_sys::target::LLVMDisposeTargetData(layout);
    }
    linked.verify()?;
    if let Some(hook) = compiler_hook {
        if !object_only {
            hook(&linked.print_module_to_string().to_bytes().to_vec(), String::from("linked.ll"));
        }
    }
    stage("rounding, address space remap");
    // O3 again in AIR's address spaces: a lighter cleanup (inlining, SROA, EarlyCSE, InstCombine, SimplifyCFG,
    // GVN) gives the tensor-core PTX tests 18-23% more AGX code (their loops are unrolled differently)
    run(&linked, c"default<O3>", &air, &options)?;
    stage("O3 in AIR's address spaces");
    // strict floating point: PTX's canonical NaNs (after the last O3, which could fold the bit tests of
    // operations whose results it proves are not NaNs, never the other way)
    if strict_fp {
        unsafe { LLVMZludaAirCanonicalNaNs(linked.get()) };
        // and the NaN constants the optimizations folded (as the Agx target)
        unsafe { LLVMZludaAgxCanonicalizeFoldedNaNs(linked.get()) };
    }
    // function pointers as IDs and choices among direct calls: no AIR consumer has a call stack
    let mut err = std::ptr::null_mut();
    if unsafe { LLVMZludaAirFunctionPointers(linked.get(), &mut err) } < 0 {
        return Err(fail(err));
    }
    // Metal: what only Metal needs from ZLUDA, on a copy, before the AIR kernel signatures are made (the
    // globals buffer is a kernel parameter like the others)
    let metal = linked.clone();
    unsafe { LLVMZludaAirSharedLayout(metal.get()) };
    for _ in 0..16 {
        if unsafe { LLVMZludaAirInlineGlobalUsers(metal.get()) } == 0 {
            break;
        }
        run(&metal, c"always-inline", &air, &options)?;
    }
    let mut json = std::ptr::null_mut();
    let mut err = std::ptr::null_mut();
    let metal_globals = match unsafe { LLVMZludaAirGlobalsToBuffer(metal.get(), &mut json, &mut err) } {
        -1 => return Err(Message::new(unsafe { CStr::from_ptr(err) }).to_str().to_string()),
        1 => Some(Message::new(unsafe { CStr::from_ptr(json) }).to_str().to_string()),
        _ => None,
    };
    stage("Metal copy: globals buffer, shared layout");
    run(&metal, c"air-portable", &air, &options)?;
    stage("Metal copy: air-portable");
    diagnostics.check()?;
    metal.verify().map_err(|e| format!("the AIR for Metal is broken: {e}"))?;
    let metal_air = bitcode(&metal);
    if let Some(hook) = compiler_hook {
        hook(&metal_air, String::from("air"));
        if let Some(json) = &metal_globals {
            hook(&json.as_bytes().to_vec(), String::from("air.globals"));
        }
    }

    run(&linked, c"air-portable", &air, &options)?;
    stage("portable: air-portable");
    diagnostics.check()?;
    let portable = bitcode(&linked);
    if let Some(hook) = compiler_hook {
        hook(&portable, String::from("portable.air"));
        if !object_only {
            hook(&linked.print_module_to_string().to_bytes().to_vec(), String::from("opt.ll"));
        }
    }

    // AGX: the portable AIR through agx-lower-air and the AGX backend
    let agx_object = match agx_cpu {
        None => None,
        Some(cpu) => {
            let cpu = CString::new(if cpu.is_empty() { AGX_DEFAULT_CPU } else { cpu }).unwrap();
            let agx = target_machine(AGX_TRIPLE, &cpu)?;
            unsafe { llvm_sys::target_machine::LLVMSetTargetMachineAsmVerbosity(agx.get(), 1) };
            if let Some(hook) = text_hook {
                stage("(writing the IR files)");
                let asm = agx.emit_to_memory_buffer(&linked.clone(), LLVMCodeGenFileType::LLVMAssemblyFile)?.to_vec();
                diagnostics.check()?;
                hook(&asm, String::from("asm"));
                stage("AGX assembly");
            }
            let object = agx.emit_to_memory_buffer(&linked, LLVMCodeGenFileType::LLVMObjectFile)?.to_vec();
            stage("AGX object");
            diagnostics.check()?;
            if let Some(hook) = compiler_hook {
                hook(&object, String::from("o"));
            }
            Some(object)
        }
    };
    Ok(AirCompilation { portable, metal: metal_air, metal_globals, agx_object })
}
