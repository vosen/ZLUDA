use crate::common::CudaDriverFns;
use cuda_types::*;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use std::{ffi::c_void, mem, ptr};

mod common;

cuda_driver_test!(kernel_texref_1d);

unsafe fn kernel_texref_1d<T: CudaDriverFns>(cuda: T) {
    let kernel = include_str!("kernel_texref_1d.ptx");
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
    let mut texture_memory = mem::zeroed();
    let width = 3;
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut texture_memory, width * mem::size_of::<u32>()),
        CUresult::CUDA_SUCCESS
    );
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0xa6bbf6cf62886047);
    let texture_host_side = (0..width).map(|_| rng.next_u32()).collect::<Vec<_>>();
    assert_eq!(
        cuda.cuMemcpyHtoD_v2(
            texture_memory,
            texture_host_side.as_ptr() as _,
            texture_host_side.len() * mem::size_of::<u32>(),
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuTexRefSetFormat(texref, CUarray_format::CU_AD_FORMAT_UNSIGNED_INT8, 4),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuTexRefSetAddress_v2(
            ptr::null_mut(),
            texref,
            texture_memory,
            width * mem::size_of::<u32>(),
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut kernel = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut kernel, module, b"texref_1d\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut out_b = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut out_b, 4 * mem::size_of::<u32>()),
        CUresult::CUDA_SUCCESS
    );
    let x = 1i32;
    let mut args = [
        &x as *const i32 as *const c_void,
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
    let pixel = texture_host_side[x as usize].to_ne_bytes();
    assert_eq!(result[0] * 255f32, pixel[3] as f32);
    assert_eq!(result[1] * 255f32, pixel[2] as f32);
    assert_eq!(result[2] * 255f32, pixel[1] as f32);
    assert_eq!(result[3] * 255f32, pixel[0] as f32);
}
