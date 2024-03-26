use crate::common::CudaDriverFns;
use cuda_types::*;

mod common;

cuda_driver_test!(primary_context);

unsafe fn primary_context<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut flags = 0;
    let mut active = 0;
    assert_eq!(
        cuda.cuDevicePrimaryCtxGetState(CUdevice_v1(0), &mut flags, &mut active),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!((0, 0), (flags, active));
    assert_eq!(
        cuda.cuDevicePrimaryCtxSetFlags_v2(CUdevice_v1(0), 1),
        CUresult::CUDA_SUCCESS
    );
}
