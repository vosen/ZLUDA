use crate::common::CudaDriverFns;
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(double_destroy_fails);

unsafe fn double_destroy_fails<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(cuda.cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
    let destroy_result = cuda.cuCtxDestroy_v2(ctx);
    // original CUDA impl returns randomly one or the other
    assert!(
        destroy_result == CUresult::CUDA_ERROR_INVALID_CONTEXT
            || destroy_result == CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
    );
}
