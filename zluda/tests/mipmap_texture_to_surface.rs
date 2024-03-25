use crate::common::CudaDriverFns;
use cuda_types::*;
use std::{mem, ptr};

mod common;

// TODO: These two tests expose various random brokenness of mipmapped array
// and texture objects. This should be turned into extensive tests like
// kernel_sust/kernel_suld/kernel_tex

cuda_driver_test!(mipmap_texture_to_surface);

unsafe fn mipmap_texture_to_surface<T: CudaDriverFns>(cuda: T) {
    let kernel = include_str!("mipmap_texture_to_surface.ptx");
    let mut kernel = kernel.to_owned();
    kernel.push('\0');
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut module = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, kernel.as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut mipmap_array = ptr::null_mut();
    let mipmap_desc = CUDA_ARRAY3D_DESCRIPTOR {
        Width: 1368,
        Height: 770,
        Depth: 0,
        Format: CUarray_format::CU_AD_FORMAT_HALF,
        NumChannels: 4,
        Flags: 2,
    };
    assert_eq!(
        cuda.cuMipmappedArrayCreate(&mut mipmap_array, &mipmap_desc, 8),
        CUresult::CUDA_SUCCESS
    );
    let mut array_0 = mem::zeroed();
    let mut array_1 = mem::zeroed();
    assert_eq!(
        cuda.cuMipmappedArrayGetLevel(&mut array_0, mipmap_array, 0),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuMipmappedArrayGetLevel(&mut array_1, mipmap_array, 1),
        CUresult::CUDA_SUCCESS
    );
    let mut pixels = [0x3C66u16, 0x4066, 0x4299, 4466];
    let memcpy_from_host = CUDA_MEMCPY2D {
        srcXInBytes: 0,
        srcY: 0,
        srcMemoryType: CUmemorytype::CU_MEMORYTYPE_HOST,
        srcHost: pixels.as_mut_ptr() as _,
        srcDevice: CUdeviceptr_v2(ptr::null_mut()),
        srcArray: ptr::null_mut(),
        srcPitch: 4 * mem::size_of::<u16>(),
        dstXInBytes: 0,
        dstY: 0,
        dstMemoryType: CUmemorytype::CU_MEMORYTYPE_ARRAY,
        dstHost: ptr::null_mut(),
        dstDevice: CUdeviceptr_v2(ptr::null_mut()),
        dstArray: array_0,
        dstPitch: 0,
        WidthInBytes: 4 * mem::size_of::<u16>(),
        Height: 1,
    };
    assert_eq!(
        cuda.cuMemcpy2DUnaligned_v2(&memcpy_from_host),
        CUresult::CUDA_SUCCESS
    );
    let mut func = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut func, module, b"texture_to_surface\0".as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut texture = mem::zeroed();
    let texture_resource_desc = CUDA_RESOURCE_DESC {
        resType: CUresourcetype::CU_RESOURCE_TYPE_ARRAY,
        res: CUDA_RESOURCE_DESC_st__bindgen_ty_1 {
            array: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 { hArray: array_0 },
        },
        flags: 0,
    };
    let texture_desc = CUDA_TEXTURE_DESC {
        addressMode: [
            CUaddress_mode::CU_TR_ADDRESS_MODE_CLAMP,
            CUaddress_mode::CU_TR_ADDRESS_MODE_CLAMP,
            CUaddress_mode::CU_TR_ADDRESS_MODE_CLAMP,
        ],
        filterMode: CUfilter_mode::CU_TR_FILTER_MODE_LINEAR,
        flags: 2,
        maxAnisotropy: 0,
        mipmapFilterMode: CUfilter_mode::CU_TR_FILTER_MODE_POINT,
        mipmapLevelBias: 0f32,
        minMipmapLevelClamp: 0f32,
        maxMipmapLevelClamp: 0f32,
        borderColor: [0f32, 0f32, 0f32, 0f32],
        reserved: mem::zeroed(),
    };
    assert_eq!(
        cuda.cuTexObjectCreate(
            &mut texture,
            &texture_resource_desc,
            &texture_desc,
            ptr::null()
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut surface = mem::zeroed();
    let surface_resource_desc = CUDA_RESOURCE_DESC {
        resType: CUresourcetype::CU_RESOURCE_TYPE_ARRAY,
        res: CUDA_RESOURCE_DESC_st__bindgen_ty_1 {
            array: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 { hArray: array_1 },
        },
        flags: 0,
    };
    assert_eq!(
        cuda.cuSurfObjectCreate(&mut surface, &surface_resource_desc),
        CUresult::CUDA_SUCCESS
    );
    let mut params = [&mut texture, &mut surface];
    assert_eq!(
        cuda.cuLaunchKernel(
            func,
            1,
            1,
            1,
            1,
            1,
            1,
            0,
            ptr::null_mut(),
            params.as_mut_ptr().cast(),
            ptr::null_mut(),
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuStreamSynchronize(ptr::null_mut()),
        CUresult::CUDA_SUCCESS
    );
    let mut memcpy_dst = [u16::MAX; 4];
    let memcpy_to_host = CUDA_MEMCPY2D {
        srcXInBytes: 0,
        srcY: 0,
        srcMemoryType: CUmemorytype::CU_MEMORYTYPE_ARRAY,
        srcHost: ptr::null(),
        srcDevice: CUdeviceptr_v2(ptr::null_mut()),
        srcArray: array_1,
        srcPitch: 0,
        dstXInBytes: 0,
        dstY: 0,
        dstMemoryType: CUmemorytype::CU_MEMORYTYPE_HOST,
        dstHost: memcpy_dst.as_mut_ptr() as _,
        dstDevice: CUdeviceptr_v2(ptr::null_mut()),
        dstArray: ptr::null_mut(),
        dstPitch: 4 * mem::size_of::<u16>(),
        WidthInBytes: 4 * mem::size_of::<u16>(),
        Height: 1,
    };
    assert_eq!(
        cuda.cuMemcpy2DUnaligned_v2(&memcpy_to_host),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(&pixels, &memcpy_dst);
}

cuda_driver_test!(mipmap_texture_to_surface2);

unsafe fn mipmap_texture_to_surface2<T: CudaDriverFns>(cuda: T) {
    let kernel = include_str!("mipmap_texture_to_surface.ptx");
    let mut kernel = kernel.to_owned();
    kernel.push('\0');
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut module = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, kernel.as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut array_0 = mem::zeroed();
    let mipmap_desc = CUDA_ARRAY3D_DESCRIPTOR {
        Width: 1368,
        Height: 770,
        Depth: 0,
        Format: CUarray_format::CU_AD_FORMAT_HALF,
        NumChannels: 4,
        Flags: 2,
    };
    assert_eq!(
        cuda.cuArray3DCreate_v2(&mut array_0, &mipmap_desc),
        CUresult::CUDA_SUCCESS
    );
    let mut array_1 = mem::zeroed();
    let mipmap_desc = CUDA_ARRAY3D_DESCRIPTOR {
        Width: 1368 / 2,
        Height: 770 / 2,
        Depth: 0,
        Format: CUarray_format::CU_AD_FORMAT_HALF,
        NumChannels: 4,
        Flags: 2,
    };
    assert_eq!(
        cuda.cuArray3DCreate_v2(&mut array_1, &mipmap_desc),
        CUresult::CUDA_SUCCESS
    );
    let mut pixels = [0x3C66u16, 0x4066, 0x4299, 4466];
    let memcpy_from_host = CUDA_MEMCPY2D {
        srcXInBytes: 0,
        srcY: 0,
        srcMemoryType: CUmemorytype::CU_MEMORYTYPE_HOST,
        srcHost: pixels.as_mut_ptr() as _,
        srcDevice: CUdeviceptr_v2(ptr::null_mut()),
        srcArray: ptr::null_mut(),
        srcPitch: 4 * mem::size_of::<u16>(),
        dstXInBytes: 0,
        dstY: 0,
        dstMemoryType: CUmemorytype::CU_MEMORYTYPE_ARRAY,
        dstHost: ptr::null_mut(),
        dstDevice: CUdeviceptr_v2(ptr::null_mut()),
        dstArray: array_0,
        dstPitch: 0,
        WidthInBytes: 4 * mem::size_of::<u16>(),
        Height: 1,
    };
    assert_eq!(
        cuda.cuMemcpy2DUnaligned_v2(&memcpy_from_host),
        CUresult::CUDA_SUCCESS
    );
    let mut func = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut func, module, b"texture_to_surface\0".as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut texture = mem::zeroed();
    let texture_resource_desc = CUDA_RESOURCE_DESC {
        resType: CUresourcetype::CU_RESOURCE_TYPE_ARRAY,
        res: CUDA_RESOURCE_DESC_st__bindgen_ty_1 {
            array: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 { hArray: array_0 },
        },
        flags: 0,
    };
    let texture_desc = CUDA_TEXTURE_DESC {
        addressMode: [
            CUaddress_mode::CU_TR_ADDRESS_MODE_CLAMP,
            CUaddress_mode::CU_TR_ADDRESS_MODE_CLAMP,
            CUaddress_mode::CU_TR_ADDRESS_MODE_CLAMP,
        ],
        filterMode: CUfilter_mode::CU_TR_FILTER_MODE_LINEAR,
        flags: 2,
        maxAnisotropy: 0,
        mipmapFilterMode: CUfilter_mode::CU_TR_FILTER_MODE_POINT,
        mipmapLevelBias: 0f32,
        minMipmapLevelClamp: 0f32,
        maxMipmapLevelClamp: 0f32,
        borderColor: [0f32, 0f32, 0f32, 0f32],
        reserved: mem::zeroed(),
    };
    assert_eq!(
        cuda.cuTexObjectCreate(
            &mut texture,
            &texture_resource_desc,
            &texture_desc,
            ptr::null()
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut surface = mem::zeroed();
    let surface_resource_desc = CUDA_RESOURCE_DESC {
        resType: CUresourcetype::CU_RESOURCE_TYPE_ARRAY,
        res: CUDA_RESOURCE_DESC_st__bindgen_ty_1 {
            array: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 { hArray: array_1 },
        },
        flags: 0,
    };
    assert_eq!(
        cuda.cuSurfObjectCreate(&mut surface, &surface_resource_desc),
        CUresult::CUDA_SUCCESS
    );
    let mut params = [&mut texture, &mut surface];
    assert_eq!(
        cuda.cuLaunchKernel(
            func,
            1,
            1,
            1,
            1,
            1,
            1,
            0,
            ptr::null_mut(),
            params.as_mut_ptr().cast(),
            ptr::null_mut(),
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuStreamSynchronize(ptr::null_mut()),
        CUresult::CUDA_SUCCESS
    );
    let mut memcpy_dst = [u16::MAX; 4];
    let memcpy_to_host = CUDA_MEMCPY2D {
        srcXInBytes: 0,
        srcY: 0,
        srcMemoryType: CUmemorytype::CU_MEMORYTYPE_ARRAY,
        srcHost: ptr::null(),
        srcDevice: CUdeviceptr_v2(ptr::null_mut()),
        srcArray: array_1,
        srcPitch: 0,
        dstXInBytes: 0,
        dstY: 0,
        dstMemoryType: CUmemorytype::CU_MEMORYTYPE_HOST,
        dstHost: memcpy_dst.as_mut_ptr() as _,
        dstDevice: CUdeviceptr_v2(ptr::null_mut()),
        dstArray: ptr::null_mut(),
        dstPitch: 4 * mem::size_of::<u16>(),
        WidthInBytes: 4 * mem::size_of::<u16>(),
        Height: 1,
    };
    assert_eq!(
        cuda.cuMemcpy2DUnaligned_v2(&memcpy_to_host),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(&pixels, &memcpy_dst);
}
