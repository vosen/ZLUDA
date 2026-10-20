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
    LLVMZludaAgxLowerAddressSpaces, LLVMZludaUpgradeFunctionAttributes,
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
/// `agx-g13g` or `agx-g14c`). With `strict_fp`, the strict floating point
/// features are enabled (see `AGX_STRICT_FP_FEATURES`).
pub fn compile_agx(
    ctx: &Context,
    cpu: &str,
    strict_fp: bool,
    main: Module,
    ptx_impl: &[u8],
    compiler_hook: Option<&dyn Fn(&Vec<u8>, String)>,
) -> Result<AgxCompilation, String> {
    compile_agx_linked(ctx, cpu, strict_fp, main, &[], ptx_impl, compiler_hook)
}

/// `compile_agx` for several PTX modules linked into one object (`cuLink*`): `others` are the LLVM IR (text or
/// bitcode) of the modules after the first, whose functions and kernels all stay.
pub fn compile_agx_linked(
    ctx: &Context,
    cpu: &str,
    strict_fp: bool,
    main: Module,
    others: &[&[u8]],
    ptx_impl: &[u8],
    compiler_hook: Option<&dyn Fn(&Vec<u8>, String)>,
) -> Result<AgxCompilation, String> {
    init_globals()?;
    let diagnostics = Diagnostics::new(ctx);

    let linked = Module::new(ctx, c"llvm-link");
    let ptx_impl = load_module(ctx, ptx_impl, c"zluda_ptx_impl_agx.ll")?;
    linked.link(main)?;
    for (i, ir) in others.iter().enumerate() {
        let name = std::ffi::CString::new(format!("linked_module_{}", i + 1)).unwrap();
        linked.link(load_module(ctx, ir, &name)?)?;
    }
    if !others.is_empty() {
        internalize_functions(&linked);
    }
    linked.link(ptx_impl)?;
    internalize_helpers(&linked);
    unsafe { LLVMZludaUpgradeFunctionAttributes(linked.get()) };
    linked.verify()?;

    if let Some(hook) = compiler_hook {
        let message = linked.print_module_to_string();
        hook(&message.to_bytes().to_vec(), String::from("linked.ll"));
    }

    let cpu = CString::new(cpu).map_err(|_| ("invalid AGX processor").to_string())?;
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
        if strict_fp { AGX_STRICT_FP_FEATURES } else { c"" },
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

    // isspacep and the generic addresses of threadgroup and private memory
    // are resolved on the optimized code, which is then optimized again
    for _ in 0..4 {
        if !unsafe { LLVMZludaAgxLowerAddressSpaces(linked.get()) } {
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
    let object = target_machine
        .emit_to_memory_buffer(&linked, LLVMCodeGenFileType::LLVMObjectFile)?
        .to_vec();
    diagnostics.check()?;
    if let Some(hook) = compiler_hook {
        hook(&object, String::from("o"));
    }
    Ok(AgxCompilation { object, assembly })
}

fn init_globals() -> Result<(), String> {
    static INIT_AGX: OnceLock<Result<(), Message>> = OnceLock::new();
    INIT_AGX
        .get_or_init(|| {
            let options = [c"llvm_zluda"];
            let ptrs: Vec<*const i8> = options.iter().map(|s| s.as_ptr()).collect();
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
            Ok(())
        })
        .as_ref()
        .map(|()| ())
        .map_err(|e| e.to_str().to_string())
}
