use hip_runtime_sys::{hipError_t, hipFuncGetAttributes, hipLaunchKernel, hipModuleLaunchKernel};

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
    let mut hip_attrib = unsafe { mem::zeroed() };
    let err = unsafe { hipFuncGetAttributes(&mut hip_attrib, func as _) };
    if err != hipError_t::hipSuccess {
        return err;
    }
    let value = match cu_attrib {
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK => {
            hip_attrib.maxThreadsPerBlock
        }
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES => {
            hip_attrib.sharedSizeBytes as i32
        }
        _ => return hipError_t::hipErrorInvalidValue,
    };
    unsafe { *pi = value };
    hipError_t::hipSuccess
}
