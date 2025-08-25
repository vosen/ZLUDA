use super::module;
use cuda_types::cuda::*;
use hip_runtime_sys::*;
use zluda_common::ZludaObject;

pub(crate) struct Library {
    base: hipModule_t,
}

impl ZludaObject for Library {
    const COOKIE: usize = 0xb328a916cc234d7c;

    type Error = CUerror;
    type CudaHandle = CUlibrary;

    fn drop_checked(&mut self) -> CUresult {
        // TODO: we will want to test that we handle `cuModuleUnload` on a module that came from a library correctly, without calling `hipModuleUnload` twice.
        unsafe { hipModuleUnload(self.base) }?;
        Ok(())
    }
}

/// This implementation simply loads the code as a HIP module for now. The various JIT and library options are ignored.
pub(crate) fn load_data(
    library: &mut CUlibrary,
    code: *const ::core::ffi::c_void,
    _jit_options: &mut CUjit_option,
    _jit_options_values: &mut *mut ::core::ffi::c_void,
    _num_jit_options: ::core::ffi::c_uint,
    _library_options: &mut CUlibraryOption,
    _library_option_values: &mut *mut ::core::ffi::c_void,
    _num_library_options: ::core::ffi::c_uint,
) -> CUresult {
    let hip_module = module::load_hip_module(code)?;
    *library = Library { base: hip_module }.wrap();
    Ok(())
}

pub(crate) unsafe fn unload(library: CUlibrary) -> CUresult {
    zluda_common::drop_checked::<Library>(library)
}

pub(crate) unsafe fn get_module(out: &mut CUmodule, library: &Library) -> CUresult {
    *out = module::Module { base: library.base }.wrap();
    Ok(())
}

pub(crate) unsafe fn get_kernel(
    kernel: *mut hipFunction_t,
    library: &Library,
    name: *const ::core::ffi::c_char,
) -> hipError_t {
    hipModuleGetFunction(kernel, library.base, name)
}

pub(crate) unsafe fn get_global(
    dptr: *mut hipDeviceptr_t,
    bytes: *mut usize,
    library: &Library,
    name: *const ::core::ffi::c_char,
) -> hipError_t {
    hipModuleGetGlobal(dptr, bytes, library.base, name)
}
