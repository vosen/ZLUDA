use crate::common::CudaDriverFns;
use cuda_types::*;

mod common;

cuda_driver_test!(context_push_invalid_should_crash);

// This test is supposed to segfault on NV runtime, but this is impossible
// to express easily in Rust right now on Windows
unsafe fn context_push_invalid_should_crash<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut fake_ctx = vec![0usize; 32];
    let result = cuda.cuCtxPushCurrent_v2(fake_ctx.as_mut_ptr() as _);
    assert_eq!(result, CUresult::CUDA_ERROR_INVALID_CONTEXT);
}
