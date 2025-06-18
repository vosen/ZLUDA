use super::{driver, ZludaObject};
use cuda_types::{
    cuda::*,
    dark_api::{FatbinFileHeader, FatbincWrapper},
};
use dark_api::fatbin::Fatbin;
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

fn get_ptx_from_wrapped_fatbin(image: *const ::core::ffi::c_void) -> Result<Vec<u8>, CUerror> {
    let fatbin = Fatbin::new(&image).map_err(|_| CUerror::UNKNOWN)?;
    let first = fatbin.get_first().map_err(|_| CUerror::UNKNOWN)?;
    let mut files = first.get_files();

    while let Some(file) = unsafe { files.next().map_err(|_| CUerror::UNKNOWN)? } {
        // Eventually we will want to get the PTX for the highest hardware version that we can get to compile. But for now we just get the first PTX we can find.
        if file.header.kind == FatbinFileHeader::HEADER_KIND_PTX {
            let decompressed = unsafe { file.decompress() }.map_err(|_| CUerror::UNKNOWN)?;
            return Ok(decompressed);
        }
    }

    Err(CUerror::NO_BINARY_FOR_GPU)
}

/// get_ptx takes an `image` that can be either a fatbin or a NULL-terminated ptx, and returns a String containing a ptx extracted from `image`.
fn get_ptx(image: *const ::core::ffi::c_void) -> Result<String, CUerror> {
    if image.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }

    let ptx = if unsafe { *(image as *const u32) } == FatbincWrapper::MAGIC {
        let ptx_bytes = get_ptx_from_wrapped_fatbin(image)?;
        str::from_utf8(&ptx_bytes)
            .map_err(|_| CUerror::UNKNOWN)?
            .to_owned()
    } else {
        unsafe { CStr::from_ptr(image.cast()) }
            .to_str()
            .map_err(|_| CUerror::INVALID_VALUE)?
            .to_owned()
    };

    Ok(ptx)
}

pub(crate) fn load_hip_module(image: *const std::ffi::c_void) -> Result<hipModule_t, CUerror> {
    let global_state = driver::global_state()?;
    let text = get_ptx(image)?;
    let ast = ptx_parser::parse_module_checked(&text).map_err(|_| CUerror::NO_BINARY_FOR_GPU)?;
    let llvm_module = ptx::to_llvm_module(ast).map_err(|_| CUerror::UNKNOWN)?;
    let mut dev = 0;
    unsafe { hipCtxGetDevice(&mut dev) }?;
    let mut props = unsafe { mem::zeroed() };
    unsafe { hipGetDevicePropertiesR0600(&mut props, dev) }?;
    let elf_module = comgr::compile_bitcode(
        &global_state.comgr,
        unsafe { CStr::from_ptr(props.gcnArchName.as_ptr()) },
        &*llvm_module.llvm_ir.write_bitcode_to_memory(),
        llvm_module.linked_bitcode(),
    )
    .map_err(|_| CUerror::UNKNOWN)?;
    let mut hip_module = unsafe { mem::zeroed() };
    unsafe { hipModuleLoadData(&mut hip_module, elf_module.as_ptr().cast()) }?;
    Ok(hip_module)
}

pub(crate) fn load_data(module: &mut CUmodule, image: &std::ffi::c_void) -> CUresult {
    let hip_module = load_hip_module(image)?;
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

pub(crate) fn get_loading_mode(mode: &mut cuda_types::cuda::CUmoduleLoadingMode) -> CUresult {
    *mode = cuda_types::cuda::CUmoduleLoadingMode::CU_MODULE_EAGER_LOADING;
    Ok(())
}
