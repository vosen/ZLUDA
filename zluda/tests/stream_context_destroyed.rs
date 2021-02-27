use crate::common::CudaDriverFns;
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(stream_context_destroyed);

unsafe fn stream_context_destroyed<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut stream = ptr::null_mut();
    assert_eq!(cuda.cuStreamCreate(&mut stream, 0), CUresult::CUDA_SUCCESS);
    let mut stream_ctx1 = ptr::null_mut();
    assert_eq!(
        cuda.cuStreamGetCtx(stream, &mut stream_ctx1),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(stream_ctx1, ctx);
    assert_eq!(cuda.cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
    let mut stream_ctx2 = ptr::null_mut();
    // When a context gets destroyed, its streams are also destroyed
    let cuda_result = cuda.cuStreamGetCtx(stream, &mut stream_ctx2);
    assert!(
        cuda_result == CUresult::CUDA_ERROR_INVALID_HANDLE
            || cuda_result == CUresult::CUDA_ERROR_INVALID_CONTEXT
            || cuda_result == CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
    );
    assert_eq!(
        cuda.cuStreamDestroy_v2(stream),
        CUresult::CUDA_ERROR_INVALID_HANDLE
    );
    // Check if creating another context is possible
    let mut ctx2 = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx2, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    //  Cleanup
    assert_eq!(cuda.cuCtxDestroy_v2(ctx2), CUresult::CUDA_SUCCESS);
}
