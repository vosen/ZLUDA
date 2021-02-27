// CUB relies on runtime reporting correct value of CU_FUNC_ATTRIBUTE_PTX_VERSION

use crate::common::CudaDriverFns;
use cuda_types::*;
use std::ptr;

mod common;

cuda_driver_test!(function_version);

static KERNEL: &str = concat!(include_str!("function_version.ptx"), "\0");

unsafe fn function_version<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ptr::null_mut(), 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut module = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, KERNEL.as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut func = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut func, module, b"foobar\0".as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut ptx_version = 0;
    assert_eq!(
        cuda.cuFuncGetAttribute(
            &mut ptx_version,
            CUfunction_attribute::CU_FUNC_ATTRIBUTE_PTX_VERSION,
            func
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut kernel_binary_version = 0;
    assert_eq!(
        cuda.cuFuncGetAttribute(
            &mut kernel_binary_version,
            CUfunction_attribute::CU_FUNC_ATTRIBUTE_BINARY_VERSION,
            func
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut cc_major = 0;
    assert_eq!(
        cuda.cuDeviceGetAttribute(
            &mut cc_major,
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR,
            CUdevice_v1(0),
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut cc_minor = 0;
    assert_eq!(
        cuda.cuDeviceGetAttribute(
            &mut cc_minor,
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR,
            CUdevice_v1(0),
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(ptx_version, 35);
    assert_eq!(kernel_binary_version, (cc_major * 10 + cc_minor));
}
