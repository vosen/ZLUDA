use crate::common::CudaDriverFns;
use cuda_types::*;
use std::{ffi::c_void, mem, ptr};

mod common;

cuda_driver_test!(llama);

unsafe fn llama<T: CudaDriverFns>(cuda: T) {
    let kernel = concat!(include_str!("llama.ptx"), "\0");
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
        cuda.cuMemAlloc_v2(&mut buffer_input, 4096),
        CUresult::CUDA_SUCCESS
    );
    let mut host_buffer = include_bytes!("llama.bin").to_vec();
    assert_eq!(
        cuda.cuMemcpyHtoD_v2(buffer_input, host_buffer.as_ptr().cast(), host_buffer.len()),
        CUresult::CUDA_SUCCESS
    );
    let mut buffer_output = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut buffer_output, 97 * mem::size_of::<f32>()),
        CUresult::CUDA_SUCCESS
    );
    let mut kernel = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetFunction(
            &mut kernel,
            module,
            b"_Z21dequantize_block_q6_KPKvPf\0".as_ptr() as _
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut args = [
        &mut buffer_input as *mut _ as *mut c_void,
        &mut buffer_output as *mut _ as _,
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
            &mut args as _,
            ptr::null_mut()
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuStreamSynchronize(ptr::null_mut()),
        CUresult::CUDA_SUCCESS
    );
    host_buffer.fill(0);
    assert_eq!(
        cuda.cuMemcpyDtoH_v2(
            host_buffer.as_mut_ptr().cast(),
            buffer_output,
            host_buffer.len()
        ),
        CUresult::CUDA_SUCCESS
    );
    let host_buffer = host_buffer.align_to::<u32>().1;
    assert_eq!(host_buffer[0], 0xBC6C7800);
    assert_eq!(host_buffer[32], 0x3B260800);
    assert_eq!(host_buffer[64], 0xBC301800);
    assert_eq!(host_buffer[96], 0x3C0AFD00);
}
