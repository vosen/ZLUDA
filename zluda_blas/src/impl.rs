use cuda_types::cublas::*;
use hip_runtime_sys::hipStream_t;
use rocblas_sys::*;
use std::mem;
use zluda_common::{constants, from_cuda_object, ZludaObject};

// This is repr(c) because it needs to be compatible with BlasHandle in zluda_blaslt.
#[repr(C)]
pub struct Handle {
    blas_lt: usize,
    handle: rocblas_handle,
}

impl Handle {
    fn new() -> Self {
        Self {
            blas_lt: 0,
            handle: unsafe { mem::zeroed() },
        }
    }
}

impl ZludaObject for Handle {
    const COOKIE: usize = constants::BLAS_HANDLE_COOKIE;
    type Error = cublasError_t;
    type CudaHandle = cublasHandle_t;
    fn drop_checked(&mut self) -> cublasStatus_t {
        // Ignore any errors from destroying embedded blas_lt handle. It may have already been destroyed.
        let _ = unsafe { zluda_blaslt::cublasLtDestroy(std::mem::transmute(self.blas_lt)) };
        unsafe { rocblas_sys::rocblas_destroy_handle(self.handle) }?;

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
    unsafe { zluda_blaslt::cublasLtCreate(std::mem::transmute(&mut zluda_blas_handle.blas_lt))? };
    unsafe { rocblas_create_handle(&mut zluda_blas_handle.handle) }?;
    *handle = Handle::wrap(zluda_blas_handle);
    Ok(())
}

pub(crate) fn get_status_name(_status: cublasStatus_t) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_status_string(status: cublasStatus_t) -> *const ::core::ffi::c_char {
    match status {
        cublasStatus_t::SUCCESS => "CUBLAS_STATUS_SUCCESS\0",
        cublasStatus_t::ERROR_NOT_INITIALIZED => "CUBLAS_STATUS_NOT_INITIALIZED\0",
        cublasStatus_t::ERROR_ALLOC_FAILED => "CUBLAS_STATUS_ALLOC_FAILED\0",
        cublasStatus_t::ERROR_INVALID_VALUE => "CUBLAS_STATUS_INVALID_VALUE\0",
        cublasStatus_t::ERROR_ARCH_MISMATCH => "CUBLAS_STATUS_ARCH_MISMATCH\0",
        cublasStatus_t::ERROR_MAPPING_ERROR => "CUBLAS_STATUS_MAPPING_ERROR\0",
        cublasStatus_t::ERROR_EXECUTION_FAILED => "CUBLAS_STATUS_EXECUTION_FAILED\0",
        cublasStatus_t::ERROR_INTERNAL_ERROR => "CUBLAS_STATUS_INTERNAL_ERROR\0",
        cublasStatus_t::ERROR_NOT_SUPPORTED => "CUBLAS_STATUS_NOT_SUPPORTED\0",
        cublasStatus_t::ERROR_LICENSE_ERROR => "CUBLAS_STATUS_LICENSE_ERROR\0",
        _ => "CUBLAS_STATUS_UNKNOWN\0",
    }
    .as_ptr() as *const ::core::ffi::c_char
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

pub(crate) unsafe fn set_pointer_mode_v2(
    handle: &Handle,
    mode: rocblas_pointer_mode,
) -> rocblas_status {
    rocblas_set_pointer_mode(handle.handle, mode)
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

pub(crate) unsafe fn hgemm(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const cuda_types::cublas::__half,
    a: *const cuda_types::cublas::__half,
    lda: ::core::ffi::c_int,
    b: *const cuda_types::cublas::__half,
    ldb: ::core::ffi::c_int,
    beta: *const cuda_types::cublas::__half,
    c: *mut cuda_types::cublas::__half,
    ldc: ::core::ffi::c_int,
) -> rocblas_status {
    rocblas_hgemm(
        handle.handle,
        transa,
        transb,
        m,
        n,
        k,
        alpha.cast(),
        a.cast(),
        lda,
        b.cast(),
        ldb,
        beta.cast(),
        c.cast(),
        ldc,
    )
}

pub(crate) unsafe fn gemm_batched_ex(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const ::core::ffi::c_void,
    a_array: *const *const ::core::ffi::c_void,
    a_type: rocblas_datatype,
    lda: ::core::ffi::c_int,
    b_array: *const *const ::core::ffi::c_void,
    b_type: rocblas_datatype,
    ldb: ::core::ffi::c_int,
    beta: *const ::core::ffi::c_void,
    c_array: *const *mut ::core::ffi::c_void,
    c_type: rocblas_datatype,
    ldc: ::core::ffi::c_int,
    batch_count: ::core::ffi::c_int,
    compute_type: rocblas_datatype,
    algo: rocblas_gemm_algo,
) -> rocblas_status {
    rocblas_gemm_batched_ex(
        handle.handle,
        transa,
        transb,
        m,
        n,
        k,
        alpha,
        a_array.cast(),
        a_type,
        lda,
        b_array.cast(),
        b_type,
        ldb,
        beta,
        c_array.cast(),
        c_type,
        ldc,
        c_array.cast_mut().cast(),
        c_type,
        ldc,
        batch_count,
        compute_type,
        algo,
        -1,
        0,
    )
}

pub(crate) unsafe fn gemm_strided_batched_ex(
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
    stride_a: ::core::ffi::c_longlong,
    b: *const ::core::ffi::c_void,
    b_type: rocblas_datatype,
    ldb: ::core::ffi::c_int,
    stride_b: ::core::ffi::c_longlong,
    beta: *const ::core::ffi::c_void,
    c: *mut ::core::ffi::c_void,
    c_type: rocblas_datatype,
    ldc: ::core::ffi::c_int,
    stride_c: ::core::ffi::c_longlong,
    batch_count: ::core::ffi::c_int,
    compute_type: rocblas_datatype,
    algo: rocblas_gemm_algo,
) -> rocblas_status {
    rocblas_gemm_strided_batched_ex(
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
        stride_a,
        b,
        b_type,
        ldb,
        stride_b,
        beta,
        c,
        c_type,
        ldc,
        stride_c,
        c,
        c_type,
        ldc,
        stride_c,
        batch_count,
        compute_type,
        algo,
        -1,
        0,
    )
}

pub(crate) unsafe fn set_vector(
    n: ::core::ffi::c_int,
    elem_size: ::core::ffi::c_int,
    x: *const ::core::ffi::c_void,
    incx: ::core::ffi::c_int,
    device_ptr: *mut ::core::ffi::c_void,
    incy: ::core::ffi::c_int,
) -> rocblas_status {
    rocblas_set_vector(n, elem_size, x, incx, device_ptr, incy)
}

pub(crate) unsafe fn get_vector(
    n: ::core::ffi::c_int,
    elem_size: ::core::ffi::c_int,
    x: *const ::core::ffi::c_void,
    incx: ::core::ffi::c_int,
    y: *mut ::core::ffi::c_void,
    incy: ::core::ffi::c_int,
) -> rocblas_status {
    rocblas_get_vector(n, elem_size, x, incx, y, incy)
}

#[cfg(test)]
mod tests {
    use cuda_macros::test_cuda;

    use crate::tests::CublasApi;
    use crate::tests::CublasLtApi;

    #[test_cuda]
    fn create_destroy(api: impl CublasApi) {
        let mut handle = unsafe { std::mem::zeroed() };
        api.cublasCreate_v2(&mut handle);
        api.cublasDestroy_v2(handle);
    }

    #[test_cuda]
    fn can_pass_into_blaslt(api: impl CublasApi) {
        use cuda_types::cublas::cublasComputeType_t;
        use cuda_types::cuda::cudaDataType_t;

        let mut handle = unsafe { std::mem::zeroed() };
        api.cublasCreate_v2(&mut handle);

        let mut pref = unsafe { std::mem::zeroed() };
        api.blaslt().cublasLtMatmulPreferenceCreate(&mut pref);

        let mut matmul_desc = unsafe { std::mem::zeroed() };
        api.blaslt().cublasLtMatmulDescCreate(
            &mut matmul_desc,
            cublasComputeType_t::CUBLAS_COMPUTE_32F,
            cudaDataType_t::CUDA_R_32F,
        );

        let m: u64 = 64;
        let n: u64 = 64;
        let k: u64 = 64;

        let mut a_desc = unsafe { std::mem::zeroed() };
        api.blaslt().cublasLtMatrixLayoutCreate(
            &mut a_desc,
            cudaDataType_t::CUDA_R_32F,
            m,
            k,
            m as i64,
        );

        let mut b_desc = unsafe { std::mem::zeroed() };
        api.blaslt().cublasLtMatrixLayoutCreate(
            &mut b_desc,
            cudaDataType_t::CUDA_R_32F,
            k,
            n,
            k as i64,
        );

        let mut c_desc = unsafe { std::mem::zeroed() };
        api.blaslt().cublasLtMatrixLayoutCreate(
            &mut c_desc,
            cudaDataType_t::CUDA_R_32F,
            m,
            n,
            m as i64,
        );

        let mut d_desc = unsafe { std::mem::zeroed() };
        api.blaslt().cublasLtMatrixLayoutCreate(
            &mut d_desc,
            cudaDataType_t::CUDA_R_32F,
            m,
            n,
            m as i64,
        );

        let mut heuristic_result = unsafe { std::mem::zeroed() };
        let mut return_algo_count: i32 = 0;
        api.blaslt().cublasLtMatmulAlgoGetHeuristic(
            unsafe { std::mem::transmute(handle) },
            matmul_desc,
            a_desc,
            b_desc,
            c_desc,
            d_desc,
            pref,
            1,
            &mut heuristic_result,
            &mut return_algo_count,
        );

        api.blaslt().cublasLtMatrixLayoutDestroy(d_desc);
        api.blaslt().cublasLtMatrixLayoutDestroy(c_desc);
        api.blaslt().cublasLtMatrixLayoutDestroy(b_desc);
        api.blaslt().cublasLtMatrixLayoutDestroy(a_desc);
        api.blaslt().cublasLtMatmulDescDestroy(matmul_desc);
        api.blaslt().cublasLtMatmulPreferenceDestroy(pref);
        api.cublasDestroy_v2(handle);
    }

    #[test_cuda]
    fn lt_destroy_blas_handle(api: impl CublasApi) {
        let mut handle = unsafe { std::mem::zeroed() };
        api.cublasCreate_v2(&mut handle);
        println!("Created handle: {:?}", handle);
        api.blaslt()
            .cublasLtDestroy(unsafe { std::mem::transmute(handle) });
        let mut math_mode = unsafe { std::mem::zeroed() };
        api.cublasGetMathMode(handle, &mut math_mode);
        println!("Math mode after blaslt destroy: {:?}", math_mode);
        api.cublasDestroy_v2(handle);
    }
}
