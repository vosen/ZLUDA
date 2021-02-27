use crate::common::CudaDriverFns;
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(context_destroy_also_destroys_stream);

unsafe fn context_destroy_also_destroys_stream<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut stream = ptr::null_mut();
    assert_eq!(cuda.cuStreamCreate(&mut stream, 0), CUresult::CUDA_SUCCESS);
    assert_eq!(cuda.cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
    let mut _temp = ptr::null_mut();
    // CUDA segfaults here
    let get_stream_ctx_err = cuda.cuStreamGetCtx(stream, &mut _temp);
    assert!(
        get_stream_ctx_err == CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
            || get_stream_ctx_err == CUresult::CUDA_ERROR_INVALID_HANDLE
    );
}
