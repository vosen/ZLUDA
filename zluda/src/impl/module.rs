use super::ZludaObject;
use cuda_types::cuda::*;
use hip_runtime_sys::*;
use std::{ffi::CStr, mem};

pub(crate) struct Module {
    base: hipModule_t,
}

impl ZludaObject for Module {
    const COOKIE: usize = 0xe9138bd040487d4a;

    type CudaHandle = CUmodule;

    fn drop_checked(&mut self) -> CUresult {
        unsafe { hipModuleUnload(self.base) }?;
        Ok(())
    }
}

pub(crate) fn load_data(module: &mut CUmodule, image: *const std::ffi::c_void) -> CUresult {
    let text = unsafe { CStr::from_ptr(image.cast()) }
        .to_str()
        .map_err(|_| CUerror::INVALID_VALUE)?;
    let ast = ptx_parser::parse_module_checked(text).map_err(|_| CUerror::NO_BINARY_FOR_GPU)?;
    let llvm_module = ptx::to_llvm_module(ast).map_err(|_| CUerror::UNKNOWN)?;
    let mut dev = 0;
    unsafe { hipCtxGetDevice(&mut dev) }?;
    let mut props = unsafe { mem::zeroed() };
    unsafe { hipGetDevicePropertiesR0600(&mut props, dev) }?;
    let elf_module = comgr::compile_bitcode(
        unsafe { CStr::from_ptr(props.gcnArchName.as_ptr()) },
        &*llvm_module.llvm_ir,
        llvm_module.linked_bitcode(),
    )
    .map_err(|_| CUerror::UNKNOWN)?;
    let mut hip_module = unsafe { mem::zeroed() };
    unsafe { hipModuleLoadData(&mut hip_module, elf_module.as_ptr().cast()) }?;
    *module = Module { base: hip_module }.wrap();
    Ok(())
}

pub(crate) fn unload(hmod: CUmodule) -> CUresult {
    super::drop_checked::<Module>(hmod)
}

pub(crate) fn get_function(
    hfunc: &mut hipFunction_t,
    hmod: &Module,
    name: *const ::core::ffi::c_char,
) -> hipError_t {
    unsafe { hipModuleGetFunction(hfunc, hmod.base, name) }
}
