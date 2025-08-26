use cuda_types::cublas::*;
use hip_runtime_sys::*;
use rocblas_sys::*;
use std::mem;
use zluda_common::{from_cuda_object, ZludaObject};

pub struct Handle {
    handle: rocblas_handle,
}

impl Handle {
    fn new() -> Self {
        Self {
            handle: unsafe { mem::zeroed() },
        }
    }
}

impl ZludaObject for Handle {
    const COOKIE: usize = 0x57c3fdb0fd72b08e;

    type Error = cublasError_t;
    type CudaHandle = cublasHandle_t;

    fn drop_checked(&mut self) -> cublasStatus_t {
        Ok(())
    }
}

from_cuda_object!(Handle);

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cublasStatus_t {
    cublasStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn create_v2(handle: &mut cublasHandle_t) -> cublasStatus_t {
    let mut zluda_blas_handle = Handle::new();
    unsafe { rocblas_create_handle(&mut zluda_blas_handle.handle) }?;
    *handle = Handle::wrap(zluda_blas_handle);
    Ok(())
}

pub(crate) fn get_status_name(_status: cublasStatus_t) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_status_string(_status: cublasStatus_t) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn xerbla(_sr_name: *const ::core::ffi::c_char, _info: ::core::ffi::c_int) -> () {
    todo!()
}

pub(crate) fn get_cudart_version() -> usize {
    todo!()
}

pub(crate) fn set_math_mode(handle: &Handle, mode: rocblas_math_mode) -> cublasStatus_t {
    unsafe { rocblas_set_math_mode(handle.handle, mode) }?;
    Ok(())
}

pub(crate) fn sgemm_strided_batched(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const f32,
    a: *const f32,
    lda: ::core::ffi::c_int,
    stride_a: ::core::ffi::c_longlong,
    b: *const f32,
    ldb: ::core::ffi::c_int,
    stride_b: ::core::ffi::c_longlong,
    beta: *const f32,
    c: *mut f32,
    ldc: ::core::ffi::c_int,
    stride_c: ::core::ffi::c_longlong,
    batch_count: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe {
        rocblas_sgemm_strided_batched(
            handle.handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            lda,
            stride_a,
            b,
            ldb,
            stride_b,
            beta,
            c,
            ldc,
            stride_c,
            batch_count,
        )
    }?;
    Ok(())
}

pub(crate) fn sgemm_v2(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const f32,
    a: *const f32,
    lda: ::core::ffi::c_int,
    b: *const f32,
    ldb: ::core::ffi::c_int,
    beta: *const f32,
    c: *mut f32,
    ldc: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe {
        rocblas_sgemm(
            handle.handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            lda,
            b,
            ldb,
            beta,
            c,
            ldc,
        )
    }?;
    Ok(())
}

pub(crate) fn destroy_v2(handle: cublasHandle_t) -> cublasStatus_t {
    zluda_common::drop_checked::<Handle>(handle)
}

pub(crate) unsafe fn set_stream_v2(handle: &Handle, stream: hipStream_t) -> rocblas_status {
    rocblas_set_stream(handle.handle, stream)
}

pub(crate) unsafe fn set_workspace_v2(
    handle: &Handle,
    workspace: *mut ::core::ffi::c_void,
    size: usize,
) -> rocblas_status {
    rocblas_set_workspace(handle.handle, workspace, size)
}

pub(crate) unsafe fn get_math_mode(handle: &Handle, mode: &mut cublasMath_t) -> rocblas_status {
    let mut roc_mode = mem::zeroed();
    rocblas_get_math_mode(handle.handle, &mut roc_mode)?;
    *mode = zluda_common::FromCuda::from_cuda(&roc_mode)?;
    Ok(())
}

pub(crate) unsafe fn gemm_ex(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const ::core::ffi::c_void,
    a: *const ::core::ffi::c_void,
    a_type: rocblas_datatype,
    lda: ::core::ffi::c_int,
    b: *const ::core::ffi::c_void,
    b_type: rocblas_datatype,
    ldb: ::core::ffi::c_int,
    beta: *const ::core::ffi::c_void,
    c: *mut ::core::ffi::c_void,
    c_type: rocblas_datatype,
    ldc: ::core::ffi::c_int,
    compute_type: rocblas_datatype,
    algo: rocblas_gemm_algo,
) -> rocblas_status {
    rocblas_gemm_ex(
        handle.handle,
        transa,
        transb,
        m,
        n,
        k,
        alpha,
        a,
        a_type,
        lda,
        b,
        b_type,
        ldb,
        beta,
        c,
        c_type,
        ldc,
        c,
        c_type,
        ldc,
        compute_type,
        algo,
        0,
        0,
    )
}
