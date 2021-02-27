use crate::common::CudaDriverFns;
use cuda_types::*;
use std::{mem, ptr};

mod common;

cuda_driver_test!(kernel_unused_global);

unsafe fn kernel_unused_global<T: CudaDriverFns>(cuda: T) {
    let mut kernel = include_str!("kernel_unused_global.ptx").to_string();
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
    let mut buffer_ptr = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetGlobal_v2(
            &mut buffer_ptr,
            ptr::null_mut(),
            module,
            b"global_buffer\0".as_ptr() as _
        ),
        CUresult::CUDA_SUCCESS
    );
    let values = [1u8, 2, 3, 4];
    assert_eq!(
        cuda.cuMemcpyHtoD_v2(buffer_ptr, values.as_ptr() as _, values.len()),
        CUresult::CUDA_SUCCESS
    );
    let mut buffer_ptr2 = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetGlobal_v2(
            &mut buffer_ptr2,
            ptr::null_mut(),
            module,
            b"global_buffer\0".as_ptr() as _
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(buffer_ptr.0, buffer_ptr2.0);
}
