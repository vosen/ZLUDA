use std::ffi::CStr;

use super::*;
use super::super::*;
use llvm_zluda::{core::*};

pub(crate) fn run(attributes: Attributes) -> Result<llvm::Module, TranslateError> {
    let context = Context::new();
    let module = llvm::Module::new(context, LLVM_UNNAMED);

    emit_attribute(&module, "clock_rate", attributes.clock_rate)?;

    if let Err(err) = module.verify() {
        panic!("{:?}", err);
    }

    Ok(module)
}

fn emit_attribute(module: &llvm::Module, name: &str, attribute: u32) -> Result<(), TranslateError> {
    let context = module.context();
    let name = format!("{}attribute_{}\0", ZLUDA_PTX_PREFIX, name).to_ascii_uppercase();
    let name = unsafe { CStr::from_bytes_with_nul_unchecked(name.as_bytes()) };
    let attribute_type = get_scalar_type(context.get(), ast::ScalarType::U32);
    let global = unsafe {
        LLVMAddGlobalInAddressSpace(
            module.get(),
            attribute_type,
            name.as_ptr(),
            get_state_space(ast::StateSpace::Global)?,
        )
    };
    unsafe { LLVMSetInitializer(global, LLVMConstInt(attribute_type, attribute as u64, 0)) };
    Ok(())
}