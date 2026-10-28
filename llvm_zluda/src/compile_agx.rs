//! Compilation of ZLUDA's LLVM modules for Apple GPUs with the experimental
//! AGX backend.
//!
//! Unlike AMDGPU there is no code object format or loader yet: the result is
//! the relocatable ELF object produced by the backend, with the machine code
//! of each kernel in `.text`. See docs/src/agx.md for the kernel ABI.

use crate::utils::{Context, Message, Module, PassBuilderOptions, TargetMachine};
use crate::{
    LLVMInitializeAGXAsmPrinter, LLVMInitializeAGXTarget, LLVMInitializeAGXTargetInfo,
    LLVMInitializeAGXTargetMC, LLVMZludaParseCommandLineOptions,
    LLVMZludaAgxCanonicalizeFoldedNaNs, LLVMZludaAgxKeepGlobals, LLVMZludaAgxLowerAddressSpaces,
    LLVMZludaAgxLowerClusters, LLVMZludaAgxSpillClusterShared,
    LLVMZludaLinkUsedDefinitions, LLVMZludaUpgradeFunctionAttributes,
};
#[cfg(zluda_air)]
use crate::{
    LLVMZludaAirFunctionPointers, LLVMZludaAirGlobalsToBuffer, LLVMZludaAirInlineGlobalUsers, LLVMZludaAirSharedLayout,
};
use llvm_sys::core::{
    LLVMContextSetDiagnosticHandler, LLVMDisposeMessage, LLVMGetDiagInfoDescription,
    LLVMGetDiagInfoSeverity, LLVMGetFirstFunction, LLVMGetNextFunction, LLVMGetValueName2,
    LLVMGetVisibility, LLVMIsDeclaration, LLVMSetLinkage, LLVMSetVisibility,
};
use llvm_sys::{LLVMLinkage, LLVMVisibility};
use llvm_sys::prelude::LLVMDiagnosticInfoRef;
use llvm_sys::LLVMDiagnosticSeverity;
use llvm_sys::{
    target_machine::{
        LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMGetTargetFromTriple,
        LLVMRelocMode,
    },
    transforms::pass_builder::LLVMRunPasses,
};
use std::ffi::{c_void, CStr, CString};
use std::sync::OnceLock;

pub const AGX_TRIPLE: &CStr = c"agx-apple-macosx";

/// The default processor: the M2 family, whose latencies the scheduling model
/// is based on.
pub const AGX_DEFAULT_CPU: &str = "agx-g14g";

/// Target features of the strict floating point mode: IEEE 754 binary32
/// arithmetic and canonical NaNs, implemented in software where the hardware
/// differs.
pub const AGX_STRICT_FP_FEATURES: &CStr = c"+ieee-fp32,+canonical-nan";

/// Target feature of the strict mode: the backend emits only the
/// instruction forms Mesa's asahi compiler emits for the same GPU (G13 and
/// G14), and reports any other.
pub const AGX_STRICT_FEATURE: &str = "+agx-strict";

/// Target feature of checked mode: the backend checks the kernels' memory accesses and reports the first bad one
/// in an error record the runtime passes (LLVM's AGXUsage.rst, "Checked mode").
pub const AGX_CHECKED_FEATURE: &str = "+agx-checked";

/// Code generation choices beyond the processor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AgxOptions {
    /// Exact PTX floating point semantics (`AGX_STRICT_FP_FEATURES`).
    pub strict_fp: bool,
    /// Only instructions Mesa emits (`AGX_STRICT_FEATURE`).
    pub agx_strict: bool,
    /// Check every memory access (`AGX_CHECKED_FEATURE`).
    pub checked: bool,
    /// Produce the object alone: no assembly (a second code generation) and no textual IR for the hook. The
    /// object's `.agx.kernels` records describe its kernels.
    pub object_only: bool,
}

impl AgxOptions {
    /// The LLVM target features these options select.
    pub fn features(&self) -> String {
        let mut f = Vec::new();
        if self.strict_fp {
            f.push(AGX_STRICT_FP_FEATURES.to_str().unwrap());
        }
        if self.agx_strict {
            f.push(AGX_STRICT_FEATURE);
        }
        if self.checked {
            f.push(AGX_CHECKED_FEATURE);
        }
        f.join(",")
    }
}

/// The outputs of an AGX compilation.
pub struct AgxCompilation {
    /// Relocatable ELF object with the kernels' machine code.
    pub object: Vec<u8>,
    /// The same code as assembly, with the per-kernel resource comments.
    pub assembly: Vec<u8>,
}

/// Collects the errors the backend reports through the context's diagnostic
/// handler (unsupported operations, missing encodings ...), which would
/// otherwise only be printed.
struct Diagnostics<'a> {
    ctx: &'a Context,
    errors: Box<Vec<String>>,
}

extern "C" fn collect_diagnostic(info: LLVMDiagnosticInfoRef, errors: *mut c_void) {
    let severity = unsafe { LLVMGetDiagInfoSeverity(info) };
    if severity != LLVMDiagnosticSeverity::LLVMDSError {
        return;
    }
    let description = unsafe { LLVMGetDiagInfoDescription(info) };
    let message = unsafe { CStr::from_ptr(description) }
        .to_string_lossy()
        .into_owned();
    unsafe { LLVMDisposeMessage(description) };
    unsafe { (*(errors as *mut Vec<String>)).push(message) };
}

impl<'a> Diagnostics<'a> {
    fn new(ctx: &'a Context) -> Self {
        let mut errors = Box::new(Vec::new());
        unsafe {
            LLVMContextSetDiagnosticHandler(
                ctx.get(),
                Some(collect_diagnostic),
                (&mut *errors as *mut Vec<String>).cast(),
            )
        };
        Self { ctx, errors }
    }

    fn check(&self) -> Result<(), String> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.join("\n"))
        }
    }
}

impl Drop for Diagnostics<'_> {
    fn drop(&mut self) {
        unsafe { LLVMContextSetDiagnosticHandler(self.ctx.get(), None, std::ptr::null_mut()) };
    }
}

fn load_module(ctx: &Context, ir: &[u8], name: &CStr) -> Result<Module, String> {
    let module = Module::try_from_ir(ctx, ir, name)?;
    module.verify()?;
    Ok(module)
}

/// The helper functions are an implementation detail of the module: make
/// them internal, so that the unused ones are removed instead of being
/// compiled as entry points (for example when every kernel of the module was
/// dropped by `--ignore-errors`).
fn internalize_helpers(module: &Module) {
    unsafe {
        let mut function = LLVMGetFirstFunction(module.get());
        while !function.is_null() {
            let mut len = 0;
            let name = LLVMGetValueName2(function, &mut len);
            let name = std::slice::from_raw_parts(name as *const u8, len);
            if LLVMIsDeclaration(function) == 0 && name.starts_with(b"__zluda_ptx_impl_") {
                LLVMSetLinkage(function, LLVMLinkage::LLVMInternalLinkage);
                LLVMSetVisibility(function, LLVMVisibility::LLVMDefaultVisibility);
            }
            function = LLVMGetNextFunction(function);
        }
    }
}

/// A linked image is complete: its functions (hidden, as the PTX translation emits every non-kernel function)
/// become internal, so that they are inlined into the kernels and the unused ones disappear.
fn internalize_functions(module: &Module) {
    unsafe {
        let mut function = LLVMGetFirstFunction(module.get());
        while !function.is_null() {
            if LLVMIsDeclaration(function) == 0 && LLVMGetVisibility(function) == LLVMVisibility::LLVMHiddenVisibility {
                LLVMSetLinkage(function, LLVMLinkage::LLVMInternalLinkage);
                LLVMSetVisibility(function, LLVMVisibility::LLVMDefaultVisibility);
            }
            function = LLVMGetNextFunction(function);
        }
    }
}

/// Link `main` with the AGX implementation of ZLUDA's PTX helper functions
/// (`ptx_impl`, textual IR or bitcode),
/// optimize, and generate code for the AGX processor `cpu` (for example
/// `agx-g13g` or `agx-g14c`), with the features `options` select.
pub fn compile_agx(
    ctx: &Context,
    cpu: &str,
    options: AgxOptions,
    main: Module,
    ptx_impl: &[u8],
    compiler_hook: Option<&dyn Fn(&Vec<u8>, String)>,
) -> Result<AgxCompilation, String> {
    compile_agx_linked(ctx, cpu, options, main, &[], ptx_impl, compiler_hook)
}

/// `compile_agx` for several PTX modules linked into one object (`cuLink*`): `others` are the LLVM IR (text or
/// bitcode) of the modules after the first, whose functions and kernels all stay.
pub fn compile_agx_linked(
    ctx: &Context,
    cpu: &str,
    options: AgxOptions,
    main: Module,
    others: &[&[u8]],
    ptx_impl: &[u8],
    compiler_hook: Option<&dyn Fn(&Vec<u8>, String)>,
) -> Result<AgxCompilation, String> {
    init_globals()?;
    let diagnostics = Diagnostics::new(ctx);

    let linked = Module::new(ctx, c"llvm-link");
    linked.link(main)?;
    for (i, ir) in others.iter().enumerate() {
        let name = std::ffi::CString::new(format!("linked_module_{}", i + 1)).unwrap();
        linked.link(load_module(ctx, ir, &name)?)?;
    }
    if !others.is_empty() {
        internalize_functions(&linked);
    }
    unsafe { LLVMZludaAgxKeepGlobals(linked.get()) };
    // only the helpers the module uses (and theirs), the library loaded lazily when it is bitcode
    let mut err = std::ptr::null_mut();
    let linked_ok = unsafe {
        LLVMZludaLinkUsedDefinitions(linked.get(), ptx_impl.as_ptr().cast(), ptx_impl.len(), c"zluda_ptx_impl_agx".as_ptr(), &mut err)
    };
    if !linked_ok {
        return Err(Message::new(unsafe { CStr::from_ptr(err) }).to_str().to_string());
    }
    internalize_helpers(&linked);
    unsafe { LLVMZludaUpgradeFunctionAttributes(linked.get()) };
    linked.verify()?;

    if let (Some(hook), false) = (compiler_hook, options.object_only) {
        let message = linked.print_module_to_string();
        hook(&message.to_bytes().to_vec(), String::from("linked.ll"));
    }

    let strict_fp = options.strict_fp;
    let cpu = CString::new(cpu).map_err(|_| ("invalid AGX processor").to_string())?;
    let features = CString::new(options.features()).unwrap();
    let mut target = unsafe { std::mem::zeroed() };
    let mut err = std::ptr::null_mut();
    let status = unsafe { LLVMGetTargetFromTriple(AGX_TRIPLE.as_ptr(), &mut target, &mut err) };
    if status != 0 {
        let message = Message::new(unsafe { CStr::from_ptr(err) });
        return Err(message.to_str().to_string());
    }
    let target_machine = TargetMachine::new(
        target,
        AGX_TRIPLE,
        &cpu,
        &features,
        LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
        LLVMRelocMode::LLVMRelocDefault,
        LLVMCodeModel::LLVMCodeModelDefault,
    );
    // The assembly lists the resources of each kernel (uniform register
    // layout, register count, threadgroup memory) as comments.
    unsafe { llvm_sys::target_machine::LLVMSetTargetMachineAsmVerbosity(target_machine.get(), 1) };

    let pb_options = PassBuilderOptions::new();
    let error = unsafe {
        LLVMRunPasses(
            linked.get(),
            c"default<O3>".as_ptr(),
            target_machine.get(),
            pb_options.get(),
        )
    };
    if !error.is_null() {
        let err_msg = unsafe { llvm_sys::error::LLVMGetErrorMessage(error) };
        let message = Message::new(unsafe { CStr::from_ptr(err_msg) });
        return Err(message.to_str().to_string());
    }

    // clusters (and the cluster registers of every kernel), on the inlined code
    let mut err = std::ptr::null_mut();
    let clusters = unsafe { LLVMZludaAgxLowerClusters(linked.get(), &mut err) };
    if clusters < 0 {
        let message = Message::new(unsafe { CStr::from_ptr(err) });
        return Err(message.to_str().to_string());
    }

    // isspacep and the generic addresses of threadgroup and private memory
    // are resolved on the optimized code, which is then optimized again
    for i in 0..4 {
        let lowered = unsafe { LLVMZludaAgxLowerAddressSpaces(linked.get()) };
        if !lowered && !(i == 0 && clusters > 0) {
            break;
        }
        let error = unsafe {
            LLVMRunPasses(
                linked.get(),
                c"default<O3>".as_ptr(),
                target_machine.get(),
                pb_options.get(),
            )
        };
        if !error.is_null() {
            let err_msg = unsafe { llvm_sys::error::LLVMGetErrorMessage(error) };
            let message = Message::new(unsafe { CStr::from_ptr(err_msg) });
            return Err(message.to_str().to_string());
        }
    }

    // cluster shared memory over 32 KiB, in device memory
    let spilled = unsafe { LLVMZludaAgxSpillClusterShared(linked.get(), &mut err) };
    if spilled < 0 {
        let message = Message::new(unsafe { CStr::from_ptr(err) });
        return Err(message.to_str().to_string());
    }
    if spilled > 0 {
        let error = unsafe {
            LLVMRunPasses(
                linked.get(),
                c"default<O3>".as_ptr(),
                target_machine.get(),
                pb_options.get(),
            )
        };
        if !error.is_null() {
            let err_msg = unsafe { llvm_sys::error::LLVMGetErrorMessage(error) };
            let message = Message::new(unsafe { CStr::from_ptr(err_msg) });
            return Err(message.to_str().to_string());
        }
    }

    if strict_fp {
        unsafe { LLVMZludaAgxCanonicalizeFoldedNaNs(linked.get()) };
    }

    // Experimental: through upstream AIR (ZLUDA_AGX_VIA_AIR=1). The optimized AGX IR is written as AIR
    // (agx-raise-to-air: what AIR does not have as hints with AIR emulations, double precision in software,
    // llvm/docs/AIR/AIRHints.rst), the AIR target's portable passes make it upstream AIR, and the AGX backend
    // compiles that (agx-lower-air restores the hinted AGX operations). The hook receives the .air bitcode.
    //
    // The .air for Metal is made from a copy, changed where Metal differs from
    // AGX: a module's .global variables are one buffer, a last kernel
    // parameter, which NAME.air.globals describes for the host
    // (LLVMZludaAirGlobalsToBuffer), the shared memory registers follow
    // Metal's numbering of threadgroup addresses (LLVMZludaAirSharedLayout),
    // and function pointers are IDs, indirect calls choices among direct ones
    // (LLVMZludaAirFunctionPointers).
    // AGX compiles the AIR of the module without those changes, so its code and
    // kernel arguments do not change.
    if via_air() {
        let metal = Some(linked.clone());
        let (mut bitcode, _) = to_air(&linked, &target_machine, &pb_options, false)?;
        let mut globals = None;
        if let Some(metal) = metal {
            (bitcode, globals) = to_air(&metal, &target_machine, &pb_options, true)?;
        }
        if let Some(hook) = compiler_hook {
            hook(&bitcode, String::from("air"));
            if let Some(json) = globals {
                hook(&json.into_bytes(), String::from("air.globals"));
            }
        }
    }

    let assembly = if options.object_only {
        Vec::new()
    } else {
        if let Some(hook) = compiler_hook {
            let message = linked.print_module_to_string();
            hook(&message.to_bytes().to_vec(), String::from("opt.ll"));
        }
        let assembly = target_machine
            .emit_to_memory_buffer(&linked.clone(), LLVMCodeGenFileType::LLVMAssemblyFile)?
            .to_vec();
        diagnostics.check()?;
        if let Some(hook) = compiler_hook {
            hook(&assembly, String::from("asm"));
        }
        assembly
    };
    let object = target_machine
        .emit_to_memory_buffer(&linked, LLVMCodeGenFileType::LLVMObjectFile)?
        .to_vec();
    diagnostics.check()?;
    if let Some(hook) = compiler_hook {
        hook(&object, String::from("o"));
    }
    Ok(AgxCompilation { object, assembly })
}

/// ZLUDA_AGX_VIA_AIR=1: compile through upstream AIR (experimental).
fn via_air() -> bool {
    std::env::var("ZLUDA_AGX_VIA_AIR").is_ok_and(|v| v == "1")
}

/// The module in upstream AIR, in place, and its bitcode (the .air file).
/// With buffer_globals, the .global variables become one buffer
/// (LLVMZludaAirGlobalsToBuffer), described by the returned JSON.
#[cfg(zluda_air)]
fn to_air(
    module: &Module,
    agx: &TargetMachine,
    options: &PassBuilderOptions,
    buffer_globals: bool,
) -> Result<(Vec<u8>, Option<String>), String> {
    let run = |pipeline: &CStr, tm: &TargetMachine| -> Result<(), String> {
        let error = unsafe { LLVMRunPasses(module.get(), pipeline.as_ptr(), tm.get(), options.get()) };
        if error.is_null() {
            return Ok(());
        }
        let err_msg = unsafe { llvm_sys::error::LLVMGetErrorMessage(error) };
        Err(Message::new(unsafe { CStr::from_ptr(err_msg) }).to_str().to_string())
    };
    run(c"agx-raise-to-air", agx)?;
    let mut globals = None;
    if buffer_globals {
        unsafe { LLVMZludaAirSharedLayout(module.get()) };
        // function pointers as IDs and choices among direct calls (before the
        // globals buffer, whose image then holds the IDs)
        let mut err = std::ptr::null_mut();
        if unsafe { LLVMZludaAirFunctionPointers(module.get(), &mut err) } < 0 {
            return Err(Message::new(unsafe { CStr::from_ptr(err) }).to_str().to_string());
        }
        // The functions using the variables, inlined into the kernels (AIR
        // inlines every call anyway), until no other function uses them.
        for _ in 0..16 {
            if unsafe { LLVMZludaAirInlineGlobalUsers(module.get()) } == 0 {
                break;
            }
            run(c"always-inline", agx)?;
        }
        let mut json = std::ptr::null_mut();
        let mut err = std::ptr::null_mut();
        match unsafe { LLVMZludaAirGlobalsToBuffer(module.get(), &mut json, &mut err) } {
            -1 => return Err(Message::new(unsafe { CStr::from_ptr(err) }).to_str().to_string()),
            1 => globals = Some(Message::new(unsafe { CStr::from_ptr(json) }).to_str().to_string()),
            _ => {}
        }
        module
            .verify()
            .map_err(|e| format!("the .air globals buffer left a broken module: {e}"))?;
    }
    let mut target = unsafe { std::mem::zeroed() };
    let mut err = std::ptr::null_mut();
    if unsafe { LLVMGetTargetFromTriple(c"air".as_ptr(), &mut target, &mut err) } != 0 {
        return Err(Message::new(unsafe { CStr::from_ptr(err) }).to_str().to_string());
    }
    let air = TargetMachine::new(
        target,
        c"air",
        c"",
        c"",
        LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
        LLVMRelocMode::LLVMRelocDefault,
        LLVMCodeModel::LLVMCodeModelDefault,
    );
    run(c"air-portable", &air)?;
    let buffer = unsafe { llvm_sys::bit_writer::LLVMWriteBitcodeToMemoryBuffer(module.get()) };
    let bytes = unsafe {
        std::slice::from_raw_parts(
            llvm_sys::core::LLVMGetBufferStart(buffer) as *const u8,
            llvm_sys::core::LLVMGetBufferSize(buffer),
        )
        .to_vec()
    };
    unsafe { llvm_sys::core::LLVMDisposeMemoryBuffer(buffer) };
    Ok((bytes, globals))
}

#[cfg(not(zluda_air))]
fn to_air(_: &Module, _: &TargetMachine, _: &PassBuilderOptions, _: bool) -> Result<(Vec<u8>, Option<String>), String> {
    Err("ZLUDA_AGX_VIA_AIR: this LLVM build has no AIR target".to_string())
}

fn init_globals() -> Result<(), String> {
    static INIT_AGX: OnceLock<Result<(), Message>> = OnceLock::new();
    INIT_AGX
        .get_or_init(|| {
            // ZLUDA_AGX_LLVM_ARGS: extra LLVM options, separated by whitespace (backend experiments, for example
            // -agx-threadgroup-unroll=4)
            let mut options = vec![std::ffi::CString::new("llvm_zluda").unwrap()];
            // Wide constants of kernels in uniforms (.agx.kernels extension 6, which agx_runtime sets at launch):
            // fewer registers, more resident SIMD-groups. ZLUDA_AGX_UNIFORM_CONSTANTS=0 turns it off.
            if std::env::var("ZLUDA_AGX_UNIFORM_CONSTANTS").map_or(true, |v| v != "0") {
                options.push(std::ffi::CString::new("-agx-uniform-constants").unwrap());
            }
            // Division by a 32-bit kernel argument with a multiplier and shifts agx_runtime computes at launch
            // (.agx.kernels extension 7). ZLUDA_AGX_LAUNCH_DIVISORS=0 turns it off.
            if std::env::var("ZLUDA_AGX_LAUNCH_DIVISORS").map_or(true, |v| v != "0") {
                options.push(std::ffi::CString::new("-agx-launch-divisors").unwrap());
            }
            if let Ok(extra) = std::env::var("ZLUDA_AGX_LLVM_ARGS") {
                options.extend(extra.split_whitespace().filter_map(|a| std::ffi::CString::new(a).ok()));
            }
            let ptrs: Vec<*const std::ffi::c_char> = options.iter().map(|s| s.as_ptr()).collect();
            let mut err_msg = std::ptr::null_mut();
            let success = unsafe {
                LLVMZludaParseCommandLineOptions(ptrs.len() as i32, ptrs.as_ptr(), &mut err_msg)
            };
            if !success {
                return Err(Message::new(unsafe { CStr::from_ptr(err_msg) }));
            }
            unsafe { LLVMInitializeAGXTargetInfo() };
            unsafe { LLVMInitializeAGXTarget() };
            unsafe { LLVMInitializeAGXTargetMC() };
            unsafe { LLVMInitializeAGXAsmPrinter() };
            #[cfg(zluda_air)]
            unsafe {
                crate::ffi::LLVMInitializeAIRTargetInfo();
                crate::ffi::LLVMInitializeAIRTarget();
                crate::ffi::LLVMInitializeAIRTargetMC();
            }
            Ok(())
        })
        .as_ref()
        .map(|()| ())
        .map_err(|e| e.to_str().to_string())
}
