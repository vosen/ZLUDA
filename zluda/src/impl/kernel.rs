use crate::r#impl::function::{self, Function};
use cuda_types::cuda::*;
use hip_runtime_sys::*;

pub(crate) unsafe fn get_function<'a>(func: &mut &'a Function, kernel: &'a Function) -> CUresult {
    *func = kernel;
    Ok(())
}

pub(crate) unsafe fn set_attribute(
    attrib: CUfunction_attribute,
    val: ::core::ffi::c_int,
    kernel: &Function,
    _dev: hipDevice_t,
) -> hipError_t {
    function::set_attribute(kernel, attrib, val)
}

pub(crate) unsafe fn get_attribute(
    pi: &mut ::core::ffi::c_int,
    attrib: CUfunction_attribute,
    kernel: &Function,
    _dev: hipDevice_t,
) -> hipError_t {
    function::get_attribute(pi, attrib, kernel)
}
