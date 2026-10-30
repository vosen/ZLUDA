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
    LLVMZludaAirRemapAddressSpaces, LLVMZludaAirSharedLayout, LLVMZludaAirStaticRounding, LLVMZludaAirDropCompareFlushes, LLVMZludaAirExpandDivisions, LLVMZludaAirCanonicalNaNs, LLVMZludaAirCompareFloatBits, LLVMZludaAirKernelModule, LLVMZludaAirSaltKernels, LLVMZludaAgxCanonicalizeFoldedNaNs, LLVMZludaLinkUsedDefinitions,
    LLVMZludaUpgradeFunctionAttributes,
};
use llvm_sys::target_machine::{
    LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMGetTargetFromTriple, LLVMRelocMode,
};
use llvm_sys::transforms::pass_builder::LLVMRunPasses;
use std::ffi::{CStr, CString};

pub const AIR_TRIPLE: &CStr = c"air";

/// The Metal copy's optimization by default, light+: the pass selection of DXMT's AIR converter (Feifan He for
/// CodeWeavers, LGPL-2.1; https://github.com/3Shain/dxmt, src/airconv/airconv_context.cpp, runOptimizationPasses:
/// "some passes that really matter" instead of a full pipeline, since Metal's compiler optimizes the AIR again),
/// written here as an LLVM pipeline (no DXMT code), with the loop and redundancy passes the hash and soft-f64 loops
/// lacked: LICM, GVN and InstCombine again.
const LIGHT_PLUS: &CStr = c"function(simplifycfg,sroa,early-cse),always-inline,ipsccp,globalopt,function(mem2reg),deadargelim,function(instcombine<no-verify-fixpoint>,simplifycfg,loop-mssa(licm),gvn,instcombine<no-verify-fixpoint>,simplifycfg)";

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
    // The optimization before ZLUDA's lowering and after each of its rounds (ZLUDA_AIR_FIRST_PIPELINE): o3, or
    // light+ (the Metal copy's pipeline, after DXMT's: see LIGHT_PLUS) with what the lowering and Metal's compiler
    // need from O3
    // By default light+: with only the Metal AIR (as the Metal copy's pipeline below), on the M2 Max the same GPU
    // times and PTX suite results as O3, and Cycles' first stage 9.5 s instead of 38; with AGX code too, which O3
    // in AIR's address spaces optimizes afterwards (Geekbench's 101 kernels: 91 the same code size, +0.4% in all;
    // the same simulator results; Cycles' first stage 15.2 -> 7.0 s).
    let first_default = "light+";
    let first_pipeline: &CStr = match std::env::var("ZLUDA_AIR_FIRST_PIPELINE").as_deref().unwrap_or(first_default) {
        // (with switches as lookup tables already, globals that the lowering lays out: made by the O3 after the
        // remap, the AGX backend rejected them, "unsupported global 'switch.table.k'")
        // (and loops unrolled as O3 does, llvm.is.constant resolved, which O3's shuffle primitives fold on: a loop
        // of simd_sum left whole made Metal's compiler crash, XPC_ERROR_CONNECTION_INTERRUPTED, on SIMD-group
        // PTX tests; and tail calls as loops, as O3 does: AIR has no call stack for the recursion left)
        "light+" => LIGHT_PLUS_FIRST,
        _ => c"default<O3>",
    };
    // The reruns after the lowerings below only clean up what they made: light+ (O3 again for them doubled Blender
    // Cycles' first stage, 9 s each; the AGX path runs O3 again in AIR's address spaces). ZLUDA_AIR_RERUN_PIPELINE=o3
    // for the full pipeline.
    const LIGHT_PLUS_FIRST: &CStr = c"function(simplifycfg,sroa,early-cse),always-inline,ipsccp,globalopt,function(mem2reg),deadargelim,function(instcombine<no-verify-fixpoint>,simplifycfg,lower-constant-intrinsics,instcombine<no-verify-fixpoint>,simplifycfg,tailcallelim,loop-mssa(licm),loop(loop-rotate,indvars,loop-unroll-full),loop-unroll<O3>,gvn,instcombine<no-verify-fixpoint>,simplifycfg<switch-to-lookup>)";
    let rerun_pipeline: &CStr = match std::env::var("ZLUDA_AIR_RERUN_PIPELINE").as_deref() {
        Ok("o3") => c"default<O3>",
        _ => LIGHT_PLUS_FIRST,
    };
    unsafe { LLVMZludaSetAirPrimitives(1) };
    let lowered = (|| -> Result<(), String> {
        run(&linked, first_pipeline, &air, &options)?;
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
            run(&linked, rerun_pipeline, &air, &options)?;
        }
        let spilled = unsafe { LLVMZludaAgxSpillClusterShared(linked.get(), &mut err) };
        if spilled < 0 {
            return Err(fail(err));
        }
        if spilled > 0 {
            run(&linked, rerun_pipeline, &air, &options)?;
        }
        Ok(())
    })();
    unsafe { LLVMZludaSetAirPrimitives(0) };
    lowered?;
    stage("ZLUDA lowering (its pipeline in ZLUDA's address spaces)");
    // the primitives that lowering called anew, after O3 dropped the unused definitions
    let mut err = std::ptr::null_mut();
    if !unsafe {
        LLVMZludaLinkUsedDefinitions(linked.get(), ptx_impl.as_ptr().cast(), ptx_impl.len(), c"zluda_ptx_impl_air".as_ptr(), &mut err)
    } {
        return Err(fail(err));
    }
    // PTX's rounding modifiers as the static rounding of each operation (AIR has no mode register)
    unsafe { LLVMZludaAirDropCompareFlushes(linked.get()) };
    unsafe { LLVMZludaAirStaticRounding(linked.get(), strict_fp as i32) };
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
    // The Metal copy's optimization (ZLUDA_AIR_METAL_PIPELINE): o3 (the portable module's), o2, light+ or light, a
    // cleanup like DXMT's AIR converter's (see LIGHT_PLUS for the credit) (Metal's compiler optimizes the AIR again; DXMT runs "some passes that
    // really matter", since an unoptimized shader makes the pipeline's compilation slow and a full pipeline is
    // too heavy). Without AGX code to make, the portable module then skips its O3 too.
    // By default light+ when only the Metal AIR is made (with AGX code, the portable module's O3 serves both):
    // on the M2 Max it gives o3's GPU times and Metal compile times, and takes Cycles 16 s instead of 45.
    let metal_pipeline = std::env::var("ZLUDA_AIR_METAL_PIPELINE")
        .unwrap_or_else(|_| if agx_cpu.is_some() { "o3".into() } else { "light+".into() });
    let metal_pipeline: Option<&CStr> = match metal_pipeline.as_str() {
        "o2" => Some(c"default<O2>"),
        "light" => Some(
            c"function(simplifycfg,sroa,early-cse),always-inline,ipsccp,globalopt,function(mem2reg),deadargelim,function(instcombine<no-verify-fixpoint>,simplifycfg)",
        ),
        // light with the loop and redundancy passes the hash and soft-f64 loops lacked (LICM, GVN, InstCombine)
        "light+" => Some(LIGHT_PLUS),
        _ => None,
    };
    // With AGX code and another Metal pipeline, the Metal copy is taken before the portable module's O3; without
    // AGX code, the Metal pipeline optimizes the portable module itself, which the Metal copy is taken from (one
    // optimization for both, and portable AIR optimized as the Metal AIR is)
    let metal_source = metal_pipeline.filter(|_| agx_cpu.is_some()).map(|_| linked.clone());
    // O3 again in AIR's address spaces: a lighter cleanup (inlining, SROA, EarlyCSE, InstCombine, SimplifyCFG,
    // GVN) gives the tensor-core PTX tests 18-23% more AGX code (their loops are unrolled differently)
    match metal_pipeline {
        Some(pipeline) if agx_cpu.is_none() => {
            // (the Metal AIR's integer divisions, LLVMZludaAirExpandDivisions; also in the portable AIR here)
            unsafe { LLVMZludaAirExpandDivisions(linked.get()) };
            run(&linked, pipeline, &air, &options)?;
            stage("the Metal pipeline in AIR's address spaces");
        }
        _ => {
            run(&linked, c"default<O3>", &air, &options)?;
            stage("O3 in AIR's address spaces");
        }
    }
    finish_air(ctx, &linked, strict_fp)?;
    // Metal: what only Metal needs from ZLUDA, on a copy, before the AIR kernel signatures are made (the
    // globals buffer is a kernel parameter like the others)
    let metal = match (metal_source, metal_pipeline) {
        (Some(source), Some(pipeline)) => {
            unsafe { LLVMZludaAirExpandDivisions(source.get()) };
            run(&source, pipeline, &air, &options)?;
            stage("Metal copy: its pipeline");
            finish_air(ctx, &source, strict_fp)?;
            source
        }
        _ => {
            let metal = linked.clone();
            // (with AGX code and the portable module's O3 for both: the Metal copy's divisions expanded here, and
            // what of them depends on the divisor only hoisted out of loops)
            if agx_cpu.is_some() && unsafe { LLVMZludaAirExpandDivisions(metal.get()) } > 0 {
                run(&metal, c"function(loop-mssa(licm),instcombine<no-verify-fixpoint>,simplifycfg)", &air, &options)?;
            }
            metal
        }
    };
    unsafe { LLVMZludaAirSharedLayout(metal.get()) };
    // ZLUDA_AIR_SALT: an unused stack array, its length from the string, at the start of each kernel of the Metal
    // copy, which makes Metal compile the pipelines anew instead of taking them from its cache, which ignores
    // metadata (comparisons of the pipelines' compile times)
    if let Ok(salt) = std::env::var("ZLUDA_AIR_SALT") {
        let len = 1 + salt.bytes().fold(7u64, |h, b| h.wrapping_mul(31).wrapping_add(b as u64)) % 63;
        unsafe { LLVMZludaAirSaltKernels(metal.get(), len) };
    }
    // The functions that use a variable take the buffer's address as a parameter (LLVMZludaAirGlobalsToBuffer).
    // ZLUDA_AIR_INLINE_GLOBAL_USERS, non-empty: inline them into the kernels instead, as before (comparisons).
    if std::env::var("ZLUDA_AIR_INLINE_GLOBAL_USERS").is_ok_and(|v| !v.is_empty()) {
        for _ in 0..16 {
            if unsafe { LLVMZludaAirInlineGlobalUsers(metal.get()) } == 0 {
                break;
            }
            run(&metal, c"always-inline", &air, &options)?;
        }
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
    // (the copy is done with: freed before the AGX code generation, the largest stage of a large module)
    drop(metal);
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
            // A large module a kernel at a time: AGX code generation inlines every function into every kernel at
            // once, which for Cycles' 75 kernels did not fit in 8 GB (one kernel took 2.7 GB). Each kernel's copy of
            // the module keeps every global variable, and the objects are merged into the one a whole-module
            // compilation makes (agx_merge). ZLUDA_AGX_PER_KERNEL: 1 always (several kernels), 0 never; by default
            // for portable AIR over 4 MB.
            let kernels = kernel_names(&linked);
            let per_kernel = kernels.len() > 1
                && match std::env::var("ZLUDA_AGX_PER_KERNEL").as_deref() {
                    Ok("1") => true,
                    Ok("0") => false,
                    _ => portable.len() > 4 << 20,
                };
            if per_kernel {
                // Each kernel in a context of its own, read back from the portable AIR, which is freed whole after
                // the kernel's code (the module and its context are done with: kept, they held Cycles' compilation
                // at 4 GB before its largest kernel's 3)
                // On ZLUDA_AGX_JOBS threads (default: up to 4 of the cores, as each holds a kernel's compilation,
                // up to about 1 GB for Cycles' largest with calls), each with its own context and target machine;
                // the objects in the kernels' order.
                drop(linked);
                drop(agx);
                let jobs = std::env::var("ZLUDA_AGX_JOBS")
                    .ok()
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get()).min(4))
                    .clamp(1, kernels.len());
                let want_asm = text_hook.is_some();
                let next = std::sync::atomic::AtomicUsize::new(0);
                let results: Vec<std::sync::Mutex<Option<Result<(Vec<u8>, Vec<u8>), String>>>> =
                    kernels.iter().map(|_| std::sync::Mutex::new(None)).collect();
                std::thread::scope(|scope| {
                    for _ in 0..jobs {
                        scope.spawn(|| {
                            let agx = match target_machine(AGX_TRIPLE, &cpu) {
                                Ok(agx) => agx,
                                Err(e) => {
                                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                    if let Some(r) = results.get(i) {
                                        *r.lock().unwrap() = Some(Err(e));
                                    }
                                    return;
                                }
                            };
                            unsafe { llvm_sys::target_machine::LLVMSetTargetMachineAsmVerbosity(agx.get(), 1) };
                            loop {
                                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                let Some(name) = kernels.get(i) else { break };
                                if timings {
                                    eprintln!("[zluda air] AGX code of {}", name.to_string_lossy());
                                }
                                let one_kernel = || -> Result<(Vec<u8>, Vec<u8>), String> {
                                    let kctx = Context::new();
                                    let kdiagnostics = Diagnostics::new(&kctx);
                                    let whole = Module::try_from_bitcode(&kctx, &portable, None)
                                        .ok_or("the portable AIR could not be read back for a kernel's AGX code")?;
                                    let one = Module::from_raw(unsafe {
                                        LLVMZludaAirKernelModule(whole.get(), name.as_ptr())
                                    });
                                    drop(whole);
                                    let mut asm = Vec::new();
                                    if want_asm {
                                        asm = agx
                                            .emit_to_memory_buffer(&one.clone(), LLVMCodeGenFileType::LLVMAssemblyFile)?
                                            .to_vec();
                                        kdiagnostics.check()?;
                                    }
                                    let object =
                                        agx.emit_to_memory_buffer(&one, LLVMCodeGenFileType::LLVMObjectFile)?.to_vec();
                                    kdiagnostics.check()?;
                                    Ok((object, asm))
                                };
                                *results[i].lock().unwrap() = Some(one_kernel());
                            }
                        });
                    }
                });
                let mut objects = Vec::with_capacity(kernels.len());
                let mut asm = Vec::new();
                for r in results {
                    let (object, kernel_asm) = r.into_inner().unwrap().ok_or("a kernel's AGX code was not made")??;
                    objects.push(object);
                    asm.extend(kernel_asm);
                }
                stage("AGX objects, a kernel at a time");
                let object = crate::agx_merge::merge(&objects)?;
                stage("AGX object (merged)");
                if let Some(hook) = text_hook {
                    hook(&asm, String::from("asm"));
                }
                if let Some(hook) = compiler_hook {
                    hook(&object, String::from("o"));
                }
                return Ok(AirCompilation { portable, metal: metal_air, metal_globals, agx_object: Some(object) });
            }
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
            // (calls left to relocations, -agx-call-relocations: resolved as the per-kernel objects' are)
            let object = if crate::agx_merge::has_call_relocations(&object) {
                crate::agx_merge::merge(&[object])?
            } else {
                object
            };
            if let Some(hook) = compiler_hook {
                hook(&object, String::from("o"));
            }
            Some(object)
        }
    };
    Ok(AirCompilation { portable, metal: metal_air, metal_globals, agx_object })
}

/// The names of the kernels (functions with the air-kernel attribute) of `module`.
fn kernel_names(module: &Module) -> Vec<CString> {
    let mut names = Vec::new();
    let key = "air-kernel";
    unsafe {
        let mut f = llvm_sys::core::LLVMGetFirstFunction(module.get());
        while !f.is_null() {
            let attr = llvm_sys::core::LLVMGetStringAttributeAtIndex(
                f,
                llvm_sys::LLVMAttributeFunctionIndex,
                key.as_ptr().cast(),
                key.len() as u32,
            );
            if !attr.is_null() && llvm_sys::core::LLVMIsDeclaration(f) == 0 {
                let mut len = 0;
                let name = llvm_sys::core::LLVMGetValueName2(f, &mut len);
                names.push(CStr::from_ptr(name).to_owned());
            }
            f = llvm_sys::core::LLVMGetNextFunction(f);
        }
    }
    names
}

/// After the last optimization of `module` in AIR's address spaces: Metal compiles without fast math
/// (air.precise-math); strict mode's canonical NaNs and denormal comparisons; function pointers as IDs.
fn finish_air(ctx: &Context, module: &Module, strict_fp: bool) -> Result<(), String> {
    let fail = |err: *mut std::ffi::c_char| Message::new(unsafe { CStr::from_ptr(err) }).to_str().to_string();
    // PTX has no fast math by default (only the .approx / .ftz forms, which the front end writes out): Metal is
    // to compile without it (air.precise-math), which also keeps the error-free transformations of the directed
    // rounding emulations intact
    unsafe {
        let one = llvm_sys::core::LLVMConstInt(llvm_sys::core::LLVMInt32TypeInContext(ctx.get()), 1, 0);
        let key = "air.precise-math";
        llvm_sys::core::LLVMAddModuleFlag(
            module.get(),
            llvm_sys::LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorOverride,
            key.as_ptr().cast(),
            key.len(),
            llvm_sys::core::LLVMValueAsMetadata(one),
        );
    }
    // strict floating point: PTX's canonical NaNs (after the last O3, which could fold the bit tests of
    // operations whose results it proves are not NaNs, never the other way)
    if strict_fp {
        // f32 denormals where the function keeps them, which Metal's ALU flushes: AIR's portable passes
        // emulate the operations whose operands or results are denormal (air.ieee-fp32)
        unsafe {
            let one = llvm_sys::core::LLVMConstInt(llvm_sys::core::LLVMInt32TypeInContext(ctx.get()), 1, 0);
            let key = "air.ieee-fp32";
            llvm_sys::core::LLVMAddModuleFlag(
                module.get(),
                llvm_sys::LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorOverride,
                key.as_ptr().cast(),
                key.len(),
                llvm_sys::core::LLVMValueAsMetadata(one),
            );
        }
        unsafe { LLVMZludaAirCanonicalNaNs(module.get()) };
        // f32 comparisons on the bits (a flushing GPU compares a denormal as zero)
        unsafe { LLVMZludaAirCompareFloatBits(module.get()) };
        // and the NaN constants the optimizations folded (as the Agx target)
        unsafe { LLVMZludaAgxCanonicalizeFoldedNaNs(module.get()) };
    }
    // function pointers as IDs and choices among direct calls: no AIR consumer has a call stack
    let mut err = std::ptr::null_mut();
    if unsafe { LLVMZludaAirFunctionPointers(module.get(), &mut err) } < 0 {
        return Err(fail(err));
    }
    Ok(())
}
