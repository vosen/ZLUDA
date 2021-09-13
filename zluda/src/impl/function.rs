use hip_runtime_sys::{hipError_t, hipFuncAttribute, hipFuncGetAttribute, hipFuncGetAttributes, hipFunction_attribute, hipLaunchKernel, hipModuleLaunchKernel};

use super::{CUresult, HasLivenessCookie, LiveCheck};
use crate::cuda::{CUfunction, CUfunction_attribute, CUstream};
use ::std::os::raw::{c_uint, c_void};
use std::{mem, ptr};

pub(crate) fn get_attribute(
    pi: *mut i32,
    cu_attrib: CUfunction_attribute,
    func: CUfunction,
) -> hipError_t {
    if pi == ptr::null_mut() || func == ptr::null_mut() {
        return hipError_t::hipErrorInvalidValue;
    }
    let attrib = match cu_attrib {
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK => {
            hipFunction_attribute::HIP_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK
        }
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES => {
            hipFunction_attribute::HIP_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES
        }
        _ => return hipError_t::hipErrorInvalidValue,
    };
    unsafe { hipFuncGetAttribute(pi, attrib, func as _) }
}
