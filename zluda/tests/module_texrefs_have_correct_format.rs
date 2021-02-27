use crate::common::CudaDriverFns;
use cuda_types::*;
use std::{mem, ptr};

mod common;

cuda_driver_test!(module_texrefs_have_correct_format);

unsafe fn module_texrefs_have_correct_format<T: CudaDriverFns>(cuda: T) {
    let kernel = include_str!("kernel_texref_2d.ptx");
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
    let mut format = mem::zeroed();
    let mut channels = mem::zeroed();
    assert_eq!(
        cuda.cuTexRefGetFormat(&mut format, &mut channels, texref),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(format, CUarray_format::CU_AD_FORMAT_FLOAT);
    assert_eq!(channels, 1);
}
