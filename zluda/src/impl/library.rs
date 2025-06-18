use super::module;

use super::ZludaObject;

use cuda_types::cuda::*;
use hip_runtime_sys::*;

pub(crate) struct Library {
    base: hipModule_t,
}

impl ZludaObject for Library {
    const COOKIE: usize = 0xb328a916cc234d7c;

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
