use crate::utils::{Context, Message, PassBuilderOptions, TargetMachine};
use crate::{ffi::LLVMZludaLinkWithLLD, utils::Module};
use llvm_sys::{
    core::*,
    support::LLVMParseCommandLineOptions,
    target::{
        LLVMInitializeAMDGPUAsmPrinter, LLVMInitializeAMDGPUTarget, LLVMInitializeAMDGPUTargetInfo,
        LLVMInitializeAMDGPUTargetMC,
    },
    target_machine::{
        LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMGetTargetFromTriple,
        LLVMRelocMode,
    },
    transforms::pass_builder::LLVMRunPasses,
};
use llvm_sys::{LLVMLinkage, LLVMVisibility};
use std::ffi::{CStr, CString};
use std::fs;
use tempfile::NamedTempFile;

const OCKL_MODULE: &'static [u8] = include_bytes!("device-libs/ockl.bc");
const OCML_MODULE: &'static [u8] = include_bytes!("device-libs/ocml.bc");

// https://llvm.org/docs/AMDGPUUsage.html#address-spaces
const CONSTANT_ADDRESS_SPACE: u32 = 4;

#[derive(Debug)]
pub enum LLVMError {
    Message(Message),
    Static(&'static str),
}

impl std::fmt::Display for LLVMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LLVMError::Message(message) => write!(f, "{}", message),
            LLVMError::Static(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for LLVMError {}

impl From<&'static str> for LLVMError {
    fn from(value: &'static str) -> Self {
        Self::Static(value)
    }
}

impl From<Message> for LLVMError {
    fn from(value: Message) -> Self {
        Self::Message(value)
    }
}

fn load_module(ctx: &Context, bc: &[u8], name: &std::ffi::CStr) -> Result<Module, LLVMError> {
    let module = Module::try_from_bitcode(ctx, bc, Some(name))
        .ok_or(LLVMError::Static("Failed to parse bitcode"))?;
    module.verify()?;
    Ok(module)
}

// TODO: see if there's a way to reduce duplication with attributes.rs
fn add_constant(context: &Context, module: &Module, name: &CStr, attribute: u32) {
    let attribute_type = unsafe { LLVMInt32TypeInContext(context.get()) };
    let global = unsafe {
        LLVMAddGlobalInAddressSpace(
            module.get(),
            attribute_type,
            name.as_ptr(),
            CONSTANT_ADDRESS_SPACE,
        )
    };
    unsafe { LLVMSetLinkage(global, LLVMLinkage::LLVMExternalLinkage) };
    unsafe { LLVMSetVisibility(global, LLVMVisibility::LLVMHiddenVisibility) };
    unsafe { LLVMSetInitializer(global, LLVMConstInt(attribute_type, attribute as u64, 0)) };
    unsafe { LLVMSetGlobalConstant(global, 1) };
}

fn create_oclc_constants(ctx: &Context, gcn_arch: &str) -> Result<Module, LLVMError> {
    // TODO: move the higher level Module struct from ptx and others here and use them
    let module = Module::new(ctx, c"oclc_constants");

    // used by ockl
    add_constant(ctx, &module, c"__oclc_wavefrontsize64", 0);
    // This is set by Clang
    // https://github.com/ROCm/llvm-project/blob/fb71c301a9a021ee5025b3ca7497970b4eb677df/clang/include/clang/Driver/RocmInstallationDetector.h#L21 and
    // https://github.com/ROCm/llvm-project/blob/fb71c301a9a021ee5025b3ca7497970b4eb677df/clang/lib/Driver/ToolChains/CommonArgs.cpp#L3089
    add_constant(ctx, &module, c"__oclc_ABI_version", 600);
    add_constant(
        ctx,
        &module,
        c"__oclc_ISA_version",
        gcn_arch
            .replace("gfx", "")
            .parse()
            .map_err(|_| LLVMError::Static("could not get ISA version from gcn_arch"))?,
    );

    // used by ocml
    add_constant(ctx, &module, c"__oclc_unsafe_math_opt", 0);
    add_constant(ctx, &module, c"__oclc_correctly_rounded_sqrt32", 1);
    add_constant(ctx, &module, c"__oclc_finite_only_opt", 0);
    Ok(module)
}

pub fn compile(
    ctx: &Context,
    gcn_arch: &str,
    main: Module,
    ptx_impl: &[u8],
    attributes: Module,
    compiler_hook: Option<&dyn Fn(&Vec<u8>, String)>,
) -> Result<Vec<u8>, LLVMError> {
    let common_options = vec![
        // Uncomment for LLVM debug
        //c"-debug",
        // Uncomment to save passes
        // c"-print-before-all",
        c"llvm_zluda",
        c"-ignore-tti-inline-compatible",
        // c"-amdgpu-early-inline-all=true",
        // To consider
        c"-amdgpu-internalize-symbols",
    ]
    .into_iter();
    let opt_options = if cfg!(debug_assertions) {
        vec![c"-amdgpu-precise-memory-op"]
    } else {
        vec![
            // default inlining threshold times 10
            c"-inline-threshold=2250",
            c"-inlinehint-threshold=3250",
        ]
    };
    let llvm_args_ptrs: Vec<*const i8> = common_options
        .chain(opt_options)
        .map(|s| s.as_ptr())
        .collect();
    unsafe {
        LLVMParseCommandLineOptions(
            llvm_args_ptrs.len() as i32,
            llvm_args_ptrs.as_ptr(),
            std::ptr::null(),
        )
    };

    unsafe { LLVMInitializeAMDGPUTargetInfo() };
    unsafe { LLVMInitializeAMDGPUTarget() };
    unsafe { LLVMInitializeAMDGPUTargetMC() };
    unsafe { LLVMInitializeAMDGPUAsmPrinter() };

    let linked = Module::new(ctx, c"llvm-link");

    let ptx_impl = load_module(ctx, ptx_impl, c"ptx_impl.bc")?;
    let ockl = load_module(ctx, OCKL_MODULE, c"ockl.bc")?;
    let ocml = load_module(ctx, OCML_MODULE, c"ocml.bc")?;

    let oclc_constants = create_oclc_constants(ctx, gcn_arch)?;

    linked.link(main)?;
    linked.link(attributes)?;
    linked.link(oclc_constants)?;
    linked.link(ptx_impl)?;
    linked.link(ockl)?;
    linked.link(ocml)?;

    linked.verify()?;

    if let Some(hook) = compiler_hook {
        // Run compiler hook on human-readable LLVM IR
        let message = linked.print_module_to_string();
        let data = message.to_bytes().to_vec();
        hook(&data, String::from("linked.ll"));
    }

    let triple = c"amdgcn-amd-amdhsa";
    let cpu =
        std::ffi::CString::new(gcn_arch).map_err(|_| LLVMError::Static("invalid gcn_arch"))?;
    let features = c"-wavefrontsize64,+cumode";

    let mut target = unsafe { std::mem::zeroed() };
    let mut err = std::ptr::null_mut();
    let status = unsafe { LLVMGetTargetFromTriple(triple.as_ptr(), &mut target, &mut err) };
    if status != 0 {
        return Err(LLVMError::from(Message::new(unsafe {
            CStr::from_ptr(err)
        })));
    }

    let target_machine = TargetMachine::new(
        target,
        triple,
        &cpu,
        features,
        LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
        LLVMRelocMode::LLVMRelocDefault,
        LLVMCodeModel::LLVMCodeModelDefault,
    );

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
        let result = Err(LLVMError::from(Message::new(unsafe {
            CStr::from_ptr(err_msg)
        })));
        return result;
    }

    if let Some(hook) = compiler_hook {
        // Run compiler hook on optimized human-readable LLVM IR
        let message = linked.print_module_to_string();
        let data = message.to_bytes().to_vec();
        hook(&data, String::from("opt.ll"));

        // Running a disassembler would be a bit of a pain, so run codegen as assembly
        let assembly = target_machine
            .emit_to_memory_buffer(&linked.clone(), LLVMCodeGenFileType::LLVMAssemblyFile)?
            .to_vec();
        hook(&assembly, String::from("asm"))
    }

    let object_file = target_machine
        .emit_to_memory_buffer(&linked, LLVMCodeGenFileType::LLVMObjectFile)?
        .to_vec();

    if let Some(hook) = compiler_hook {
        // Run compiler hook for object file
        hook(&object_file, String::from("o"));
    }

    let object_path = NamedTempFile::with_prefix("zluda.o")
        .map_err(|_| LLVMError::Static("Failed to create temporary file"))?
        .into_temp_path();
    let executable_path = NamedTempFile::with_prefix("zluda.elf")
        .map_err(|_| LLVMError::Static("Failed to create temporary file"))?
        .into_temp_path();

    fs::write(&object_path, &object_file)
        .map_err(|_| LLVMError::Static("Failed to write object file"))?;

    let object_path_cstr = CString::new(object_path.to_str().unwrap()).unwrap();
    let executable_path_cstr = CString::new(executable_path.to_str().unwrap()).unwrap();

    let mut err = std::ptr::null_mut();
    let result = unsafe {
        LLVMZludaLinkWithLLD(
            object_path_cstr.as_ptr(),
            executable_path_cstr.as_ptr(),
            &mut err,
        )
    };
    if result != 0 {
        return Err(LLVMError::from(Message::new(unsafe {
            CStr::from_ptr(err)
        })));
    }

    let executable = fs::read(&executable_path)
        .map_err(|_| LLVMError::Static("Failed to read executable file"))?;

    if let Some(hook) = compiler_hook {
        // Run compiler hook for final executable
        hook(&executable, String::from("elf"));
    }

    Ok(executable)
}
