use std::ptr;

use crate::cuda::CUlimit;
use crate::cuda::CUresult;

pub(crate) unsafe fn get_limit(pvalue: *mut usize, limit: CUlimit) -> CUresult {
    if pvalue == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    if limit == CUlimit::CU_LIMIT_STACK_SIZE {
        *pvalue = 512; // GTX 1060 reports 1024
        CUresult::CUDA_SUCCESS
    } else {
        CUresult::CUDA_ERROR_NOT_SUPPORTED
    }
}

pub(crate) fn set_limit(limit: CUlimit, value: usize) -> CUresult {
    if limit == CUlimit::CU_LIMIT_STACK_SIZE {
        CUresult::CUDA_SUCCESS
    } else {
        CUresult::CUDA_ERROR_NOT_SUPPORTED
    }
}
