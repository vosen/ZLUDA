use crate::common::{CudaDriverFns, CU_STREAM_LEGACY};
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(cant_destroy_default_stream);

unsafe fn cant_destroy_default_stream<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(
        cuda.cuStreamDestroy_v2(CU_STREAM_LEGACY as *mut _),
        CUresult::CUDA_SUCCESS
    );
    // Cleanup
    assert_eq!(cuda.cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
}
