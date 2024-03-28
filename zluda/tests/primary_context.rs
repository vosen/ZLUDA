use crate::common::CudaDriverFns;
use cuda_types::*;
use std::{mem, ptr};
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
    let mut primary_ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuDevicePrimaryCtxRetain(&mut primary_ctx, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(primary_ctx, ptr::null_mut());
    let mut active_ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxGetCurrent(&mut active_ctx),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(primary_ctx, active_ctx);
    assert_eq!(
        cuda.cuDevicePrimaryCtxGetState(CUdevice_v1(0), &mut flags, &mut active),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!((1, 1), (flags, active));
    let mut buffer = mem::zeroed();
    assert_eq!(
        cuda.cuCtxPushCurrent_v2(primary_ctx),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(cuda.cuMemAlloc_v2(&mut buffer, 4), CUresult::CUDA_SUCCESS);
    assert_eq!(
        cuda.cuDevicePrimaryCtxRelease_v2(CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuDevicePrimaryCtxGetState(CUdevice_v1(0), &mut flags, &mut active),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(
        cuda.cuDevicePrimaryCtxRelease_v2(CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!((0, 0), (flags, active));
    // Already freed on context destruction
    // TODO: reenable when we start tracking allocations inside context
    //assert_ne!(cuda.cuMemFree_v2(buffer), CUresult::CUDA_SUCCESS);
    assert_eq!(
        cuda.cuDevicePrimaryCtxReset_v2(CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
}
