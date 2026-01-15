use cuda_types::cuda::{CUfunction_attribute, CUresult};
use hip_runtime_sys::*;

use crate::r#impl::function;

pub(crate) unsafe fn get_function(func: &mut hipFunction_t, kernel: hipFunction_t) -> CUresult {
    *func = kernel;
    Ok(())
}

pub(crate) unsafe fn set_attribute(
    attrib: CUfunction_attribute,
    val: ::core::ffi::c_int,
    kernel: hipFunction_t,
    _dev: hipDevice_t,
) -> hipError_t {
    function::set_attribute(kernel, attrib, val)
}

pub(crate) unsafe fn get_attribute(
    pi: &mut ::core::ffi::c_int,
    attrib: CUfunction_attribute,
    kernel: hipFunction_t,
    _dev: hipDevice_t,
) -> hipError_t {
    function::get_attribute(pi, attrib, kernel)
}
