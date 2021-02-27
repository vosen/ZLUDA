use crate::common::CudaDriverFns;
use cuda_types::*;
use std::{ffi::c_void, mem, ptr};

mod common;

cuda_driver_test!(kernel_extra);

const CU_LAUNCH_PARAM_BUFFER_POINTER: *mut c_void = 1 as *mut _;
const CU_LAUNCH_PARAM_BUFFER_SIZE: *mut c_void = 2 as *mut _;
const CU_LAUNCH_PARAM_END: *mut c_void = 0 as *mut _;

unsafe fn kernel_extra<T: CudaDriverFns>(cuda: T) {
    let kernel = include_str!("kernel_extra.ptx");
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
    let mut buffer_input = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut buffer_input, 8),
        CUresult::CUDA_SUCCESS
    );
    let mut buffer_output = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut buffer_output, 8),
        CUresult::CUDA_SUCCESS
    );
    let mut kernel = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut kernel, module, b"add\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut args = [buffer_input, buffer_output];
    let mut size = mem::size_of_val(&args);
    let mut extra = [
        CU_LAUNCH_PARAM_BUFFER_POINTER,
        args.as_mut_ptr() as *mut _ as _,
        CU_LAUNCH_PARAM_BUFFER_SIZE,
        &mut size as *mut _ as _,
        CU_LAUNCH_PARAM_END,
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
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            extra.as_mut_ptr()
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuStreamSynchronize(ptr::null_mut()),
        CUresult::CUDA_SUCCESS
    );
}
