use crate::common::CudaDriverFns;
use cuda_types::*;
use std::{mem, ptr};

mod common;

cuda_driver_test!(memcpy_pitch);

unsafe fn memcpy_pitch<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut memcpy_2d = mem::zeroed::<CUDA_MEMCPY2D>();
    let width = 2;
    let pitch = 4;
    let height = 2;
    let mut source = (0..pitch * height).map(|x| x as u8).collect::<Vec<_>>();
    let mut devptr = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut devptr, width * height),
        CUresult::CUDA_SUCCESS
    );
    memcpy_2d.srcMemoryType = CUmemorytype::CU_MEMORYTYPE_HOST;
    memcpy_2d.srcHost = source.as_mut_ptr() as _;
    memcpy_2d.srcPitch = pitch;
    memcpy_2d.dstMemoryType = CUmemorytype::CU_MEMORYTYPE_DEVICE;
    memcpy_2d.dstDevice = devptr;
    memcpy_2d.WidthInBytes = width;
    memcpy_2d.Height = height;
    assert_eq!(
        cuda.cuMemcpy2DUnaligned_v2(&memcpy_2d),
        CUresult::CUDA_SUCCESS
    );
    let mut result = vec![0u8; width * height];
    assert_eq!(
        cuda.cuMemcpyDtoH_v2(result.as_mut_ptr() as _, devptr, width * height),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(result, [0u8, 1, 4, 5]);
}

cuda_driver_test!(memcpy_pitch_dst);

unsafe fn memcpy_pitch_dst<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut memcpy_2d = mem::zeroed::<CUDA_MEMCPY2D>();
    let width = 2;
    let pitch = 4;
    let height = 2;
    let source = (0..width * height).map(|x| x as u8).collect::<Vec<_>>();
    let mut devptr = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut devptr, pitch * height),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuMemsetD8_v2(devptr, 0xff, pitch * height),
        CUresult::CUDA_SUCCESS
    );
    memcpy_2d.srcMemoryType = CUmemorytype::CU_MEMORYTYPE_HOST;
    memcpy_2d.srcHost = source.as_ptr() as _;
    memcpy_2d.dstMemoryType = CUmemorytype::CU_MEMORYTYPE_DEVICE;
    memcpy_2d.dstDevice = devptr;
    memcpy_2d.dstPitch = pitch;
    memcpy_2d.WidthInBytes = width;
    memcpy_2d.Height = height;
    assert_eq!(
        cuda.cuMemcpy2DUnaligned_v2(&memcpy_2d),
        CUresult::CUDA_SUCCESS
    );
    let mut result = vec![0u8; pitch * height];
    assert_eq!(
        cuda.cuMemcpyDtoH_v2(result.as_mut_ptr() as _, devptr, pitch * height),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(result, [0, 1, 255, 255, 2, 3, 255, 255]);
}

cuda_driver_test!(memcpy_3d_pitch);

unsafe fn memcpy_3d_pitch<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let width = 2;
    let pitch = 4;
    let height = 2;
    let depth = 1;
    let source = (0..pitch * height * depth)
        .map(|x| x as u8)
        .collect::<Vec<_>>();
    let mut devptr = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut devptr, pitch * height * depth),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuMemcpyHtoD_v2(devptr, source.as_ptr() as _, pitch * height * depth),
        CUresult::CUDA_SUCCESS
    );
    let mut array = mem::zeroed();
    let array_desc = CUDA_ARRAY3D_DESCRIPTOR {
        Width: width,
        Height: height,
        Depth: depth,
        Format: CUarray_format::CU_AD_FORMAT_UNSIGNED_INT8,
        NumChannels: 1,
        Flags: 0,
    };
    assert_eq!(
        cuda.cuArray3DCreate_v2(&mut array, &array_desc),
        CUresult::CUDA_SUCCESS
    );
    let mut copy_desc = mem::zeroed::<CUDA_MEMCPY3D>();
    copy_desc.srcMemoryType = CUmemorytype::CU_MEMORYTYPE_DEVICE;
    copy_desc.srcDevice = devptr;
    copy_desc.srcPitch = pitch;
    copy_desc.srcHeight = height;
    copy_desc.dstMemoryType = CUmemorytype::CU_MEMORYTYPE_ARRAY;
    copy_desc.dstArray = array;
    copy_desc.WidthInBytes = width;
    copy_desc.Height = height;
    copy_desc.Depth = depth;
    assert_eq!(cuda.cuMemcpy3D_v2(&copy_desc), CUresult::CUDA_SUCCESS);
    let mut result = vec![0u8; width * height * depth];
    let mut backcopy_desc = mem::zeroed::<CUDA_MEMCPY3D>();
    backcopy_desc.srcMemoryType = CUmemorytype::CU_MEMORYTYPE_ARRAY;
    backcopy_desc.srcArray = array;
    backcopy_desc.dstMemoryType = CUmemorytype::CU_MEMORYTYPE_HOST;
    backcopy_desc.dstHost = result.as_mut_ptr() as _;
    backcopy_desc.WidthInBytes = width;
    backcopy_desc.Height = height;
    backcopy_desc.Depth = depth;
    assert_eq!(cuda.cuMemcpy3D_v2(&backcopy_desc), CUresult::CUDA_SUCCESS);
    assert_eq!(result, [0, 1, 4, 5]);
}
