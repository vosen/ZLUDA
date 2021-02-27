use crate::common::CudaDriverFns;
use cuda_types::*;
use std::{ptr, thread};

mod common;

cuda_driver_test!(stream_moves_context_to_another_thread);

unsafe fn stream_moves_context_to_another_thread<T: CudaDriverFns + Send + 'static + Clone>(
    cuda: T,
) {
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
    let stream_ptr = stream as usize;
    let cuda_ = cuda.clone();
    let stream_ctx_on_thread = thread::spawn(move || {
        let mut stream_ctx2 = ptr::null_mut();
        assert_eq!(
            cuda_.cuStreamGetCtx(stream_ptr as *mut _, &mut stream_ctx2),
            CUresult::CUDA_SUCCESS
        );
        stream_ctx2 as usize
    })
    .join()
    .unwrap();
    assert_eq!(stream_ctx1, stream_ctx_on_thread as *mut _);
    //  Cleanup
    assert_eq!(cuda.cuStreamDestroy_v2(stream), CUresult::CUDA_SUCCESS);
    assert_eq!(cuda.cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
}
