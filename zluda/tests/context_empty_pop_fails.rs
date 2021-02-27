use crate::common::CudaDriverFns;
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(empty_pop_fails);

unsafe fn empty_pop_fails<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxPopCurrent_v2(&mut ctx),
        CUresult::CUDA_ERROR_INVALID_CONTEXT
    );
}
