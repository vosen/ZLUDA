use crate::common::CudaDriverFns;
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(no_current_on_init);

unsafe fn no_current_on_init<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = 1 as _;
    assert_eq!(cuda.cuCtxGetCurrent(&mut ctx), CUresult::CUDA_SUCCESS);
    assert_eq!(ctx, ptr::null_mut());
}
