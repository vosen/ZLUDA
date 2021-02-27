use crate::common::CudaDriverFns;
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(maxntid);

unsafe fn maxntid<T: CudaDriverFns>(cuda: T) {
    let kernel = include_str!("maxntid.ptx");
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
    let mut func = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut func, module, b"add\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut _unused = 0;
    let mut max_blocksize = 0;
    assert_eq!(
        cuda.cuOccupancyMaxPotentialBlockSize(&mut _unused, &mut max_blocksize, func, None, 0, 0),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(max_blocksize, 32);
}
