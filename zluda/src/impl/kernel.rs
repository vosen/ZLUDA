use cuda_types::cuda::CUresult;
use hip_runtime_sys::*;

use crate::r#impl::function;

pub(crate) unsafe fn get_function(func: &mut hipFunction_t, kernel: hipFunction_t) -> CUresult {
    *func = kernel;
    Ok(())
}

pub(crate) unsafe fn set_attribute(
    attrib: hipFunction_attribute,
    val: ::core::ffi::c_int,
    kernel: hipFunction_t,
    _dev: hipDevice_t,
) -> hipError_t {
    function::set_attribute(kernel, attrib, val)
}
