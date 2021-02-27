use crate::common::CudaDriverFns;
use cuda_types::*;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use std::{ffi::c_void, mem, ptr};

mod common;

cuda_driver_test!(kernel_texref_2d);

unsafe fn kernel_texref_2d<T: CudaDriverFns>(cuda: T) {
    let kernel = include_str!("kernel_texref_2d.ptx");
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
    let mut texref = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleGetTexRef(&mut texref, module, b"image\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut texture_memory = CUdeviceptr_v2(ptr::null_mut());
    let mut texture_pitch = 0usize;
    let width = 3;
    let height = 3;
    assert_eq!(
        cuda.cuMemAllocPitch_v2(
            &mut texture_memory,
            &mut texture_pitch,
            width * mem::size_of::<u32>(),
            height,
            4,
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0xcb42848a346f8673);
    let mut texture_host_side = (0..width * height)
        .map(|_| rng.next_u32())
        .collect::<Vec<_>>();
    assert_eq!(
        cuda.cuMemcpy2D_v2(&CUDA_MEMCPY2D {
            srcXInBytes: 0,
            srcY: 0,
            srcMemoryType: CUmemorytype::CU_MEMORYTYPE_HOST,
            srcHost: texture_host_side.as_mut_ptr() as _,
            srcDevice: CUdeviceptr_v2(ptr::null_mut()),
            srcArray: ptr::null_mut(),
            srcPitch: width * mem::size_of::<u32>(),
            dstXInBytes: 0,
            dstY: 0,
            dstMemoryType: CUmemorytype::CU_MEMORYTYPE_DEVICE,
            dstHost: ptr::null_mut(),
            dstDevice: texture_memory,
            dstArray: ptr::null_mut(),
            dstPitch: texture_pitch,
            WidthInBytes: width * mem::size_of::<u32>(),
            Height: height,
        }),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuTexRefSetFormat(texref, CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT8, 4),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuTexRefSetAddress2D_v3(
            texref,
            &CUDA_ARRAY_DESCRIPTOR {
                Width: width,
                Height: height,
                Format: CUarray_format::CU_AD_FORMAT_UNSIGNED_INT8,
                NumChannels: 4,
            },
            texture_memory,
            texture_pitch,
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut kernel = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut kernel, module, b"texref\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut out_b = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut out_b, 4 * mem::size_of::<u32>()),
        CUresult::CUDA_SUCCESS
    );
    let x = 1.0f32;
    let y = 2.0f32;
    let mut args = [
        &x as *const f32 as *const c_void,
        &y as *const f32 as *const _,
        &out_b as *const _ as *const _,
    ];
    assert_eq!(
        cuda.cuLaunchKernel(
            kernel,
            1,
            1,
            1,
            1,
            1,
            1,
            1024,
            0 as _,
            args.as_mut_ptr() as _,
            ptr::null_mut(),
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut result = vec![0f32; 4usize];
    for i in 0..result.len() {
        result[i] = mem::transmute(u32::MAX);
    }
    assert_eq!(
        cuda.cuMemcpyDtoH_v2(
            result.as_mut_ptr() as _,
            out_b,
            result.len() * mem::size_of::<u32>(),
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(cuda.cuStreamSynchronize(0 as _), CUresult::CUDA_SUCCESS);
    let pixel = texture_host_side[width * (y as usize) + (x as usize)].to_ne_bytes();
    assert_eq!(result[0] * 255f32, pixel[3] as f32);
    assert_eq!(result[1] * 255f32, pixel[2] as f32);
    assert_eq!(result[2] * 255f32, pixel[1] as f32);
    assert_eq!(result[3] * 255f32, pixel[0] as f32);
}
