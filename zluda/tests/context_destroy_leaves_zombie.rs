use crate::common::CudaDriverFns;
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(context_destroy_leaves_zombie);

unsafe fn context_destroy_leaves_zombie<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx1 = ptr::null_mut();
    let mut ctx2 = ptr::null_mut();
    let mut ctx3 = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx1, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx2, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx3, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(cuda.cuCtxDestroy_v2(ctx2), CUresult::CUDA_SUCCESS);
    let mut popped_ctx1 = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxPopCurrent_v2(&mut popped_ctx1),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(popped_ctx1, ctx3);
    let mut popped_ctx2 = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxPopCurrent_v2(&mut popped_ctx2),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(popped_ctx2, ctx2);
    let mut popped_ctx3 = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxPopCurrent_v2(&mut popped_ctx3),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(popped_ctx3, ctx1);
    let mut temp = 0;
    assert_eq!(
        cuda.cuCtxGetApiVersion(ctx2, &mut temp),
        CUresult::CUDA_ERROR_INVALID_CONTEXT
    );
    assert_eq!(
        cuda.cuCtxPopCurrent_v2(&mut ptr::null_mut()),
        CUresult::CUDA_ERROR_INVALID_CONTEXT
    );
}
