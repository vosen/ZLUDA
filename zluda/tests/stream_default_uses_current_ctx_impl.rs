use common::{CudaDriverFns, CU_STREAM_LEGACY, CU_STREAM_PER_THREAD};
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(stream_default_uses_current_ctx_legacy);
cuda_driver_test!(stream_default_uses_current_ctx_ptsd);

unsafe fn stream_default_uses_current_ctx_legacy<T: CudaDriverFns>(cuda: T) {
    stream_default_uses_current_ctx_impl::<T>(cuda, CU_STREAM_LEGACY);
}

unsafe fn stream_default_uses_current_ctx_ptsd<T: CudaDriverFns>(cuda: T) {
    stream_default_uses_current_ctx_impl::<T>(cuda, CU_STREAM_PER_THREAD);
}

unsafe fn stream_default_uses_current_ctx_impl<T: CudaDriverFns>(cuda: T, stream: CUstream) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx1 = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx1, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut stream_ctx1 = ptr::null_mut();
    assert_eq!(
        cuda.cuStreamGetCtx(stream, &mut stream_ctx1),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(ctx1, stream_ctx1);
    let mut ctx2 = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx2, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(ctx1, ctx2);
    let mut stream_ctx2 = ptr::null_mut();
    assert_eq!(
        cuda.cuStreamGetCtx(stream, &mut stream_ctx2),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(ctx2, stream_ctx2);
    //  Cleanup
    assert_eq!(cuda.cuCtxDestroy_v2(ctx1), CUresult::CUDA_SUCCESS);
    assert_eq!(cuda.cuCtxDestroy_v2(ctx2), CUresult::CUDA_SUCCESS);
}
