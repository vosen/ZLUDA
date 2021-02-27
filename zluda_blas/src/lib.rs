#![allow(warnings)]
mod cublas;

pub use cublas::*;

use cuda_types::*;
use rocblas_sys::*;
use rocsolver_sys::{
    rocsolver_cgetrf_batched, rocsolver_cgetri_batched, rocsolver_cgetri_outofplace_batched,
    rocsolver_zgetrf_batched, rocsolver_zgetri_batched, rocsolver_zgetri_outofplace_batched,
};
use std::{mem, ptr};

#[cfg(debug_assertions)]
pub(crate) fn unsupported() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unsupported() -> cublasStatus_t {
    cublasStatus_t::CUBLAS_STATUS_NOT_SUPPORTED
}

fn to_cuda(status: rocblas_sys::rocblas_status) -> cublasStatus_t {
    match status {
        rocblas_sys::rocblas_status::rocblas_status_success => {
            cublasStatus_t::CUBLAS_STATUS_SUCCESS
        }
        _ => cublasStatus_t::CUBLAS_STATUS_INTERNAL_ERROR,
    }
}

fn to_cuda_solver(status: rocsolver_sys::rocblas_status) -> cublasStatus_t {
    match status {
        rocsolver_sys::rocblas_status::rocblas_status_success => {
            cublasStatus_t::CUBLAS_STATUS_SUCCESS
        }
        _ => cublasStatus_t::CUBLAS_STATUS_INTERNAL_ERROR,
    }
}

unsafe fn create(handle: *mut cublasHandle_t) -> cublasStatus_t {
    to_cuda(rocblas_sys::rocblas_create_handle(handle as _))
}

unsafe fn sgemv(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: *const f32,
    a: *const f32,
    lda: i32,
    x: *const f32,
    incx: i32,
    beta: *const f32,
    y: *mut f32,
    incy: i32,
) -> cublasStatus_t {
    let trans = op_from_cuda(trans);
    to_cuda(rocblas_sgemv(
        handle as _,
        trans,
        m,
        n,
        alpha,
        a,
        lda,
        x,
        incx,
        beta,
        y,
        incy,
    ))
}

fn op_from_cuda(trans: cublasOperation_t) -> rocblas_operation {
    match trans {
        cublasOperation_t::CUBLAS_OP_N => rocblas_operation::rocblas_operation_none,
        cublasOperation_t::CUBLAS_OP_T => rocblas_operation::rocblas_operation_transpose,
        cublasOperation_t::CUBLAS_OP_C => rocblas_operation::rocblas_operation_conjugate_transpose,
        _ => panic!(),
    }
}

unsafe fn destroy(handle: cublasHandle_t) -> cublasStatus_t {
    to_cuda(rocblas_destroy_handle(handle as _))
}

unsafe fn sgemm_ex(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const f32,
    a: *const std::ffi::c_void,
    atype: cudaDataType,
    lda: i32,
    b: *const std::ffi::c_void,
    btype: cudaDataType,
    ldb: i32,
    beta: *const f32,
    c: *mut std::ffi::c_void,
    ctype: cudaDataType,
    ldc: i32,
) -> cublasStatus_t {
    let transa = op_from_cuda(transa);
    let transb = op_from_cuda(transb);
    let a_type = type_from_cuda(atype);
    let b_type = type_from_cuda(btype);
    let c_type = type_from_cuda(ctype);
    to_cuda(rocblas_gemm_ex(
        handle as _,
        transa,
        transb,
        m,
        n,
        k,
        alpha as _,
        a as _,
        a_type,
        lda,
        b as _,
        b_type,
        ldb,
        beta as _,
        c as _,
        c_type,
        ldc,
        c as _,
        c_type,
        ldc,
        rocblas_datatype::rocblas_datatype_f32_r,
        rocblas_gemm_algo::rocblas_gemm_algo_standard,
        0,
        0,
    ))
}

fn type_from_cuda(type_: cudaDataType_t) -> rocblas_datatype {
    match type_ {
        cudaDataType_t::CUDA_R_16F => rocblas_datatype::rocblas_datatype_f16_r,
        cudaDataType_t::CUDA_R_32F => rocblas_datatype::rocblas_datatype_f32_r,
        cudaDataType_t::CUDA_R_64F => rocblas_datatype::rocblas_datatype_f64_r,
        cudaDataType_t::CUDA_C_16F => rocblas_datatype::rocblas_datatype_f16_c,
        cudaDataType_t::CUDA_C_32F => rocblas_datatype::rocblas_datatype_f32_c,
        cudaDataType_t::CUDA_C_64F => rocblas_datatype::rocblas_datatype_f64_c,
        cudaDataType_t::CUDA_R_8I => rocblas_datatype::rocblas_datatype_i8_r,
        cudaDataType_t::CUDA_R_8U => rocblas_datatype::rocblas_datatype_u8_r,
        cudaDataType_t::CUDA_R_32I => rocblas_datatype::rocblas_datatype_i32_r,
        cudaDataType_t::CUDA_R_32U => rocblas_datatype::rocblas_datatype_u32_r,
        cudaDataType_t::CUDA_C_8I => rocblas_datatype::rocblas_datatype_i8_c,
        cudaDataType_t::CUDA_C_8U => rocblas_datatype::rocblas_datatype_u8_c,
        cudaDataType_t::CUDA_C_32I => rocblas_datatype::rocblas_datatype_i32_c,
        cudaDataType_t::CUDA_C_32U => rocblas_datatype::rocblas_datatype_u32_c,
        cudaDataType_t::CUDA_R_16BF => rocblas_datatype::rocblas_datatype_bf16_r,
        cudaDataType_t::CUDA_C_16BF => rocblas_datatype::rocblas_datatype_bf16_c,
        _ => panic!(),
    }
}

unsafe fn set_stream(handle: cublasHandle_t, stream_id: cudaStream_t) -> cublasStatus_t {
    let lib = hip_common::zluda_ext::get_cuda_library().unwrap();
    let cu_get_export_table = lib
        .get::<unsafe extern "C" fn(
            ppExportTable: *mut *const ::std::os::raw::c_void,
            pExportTableId: *const CUuuid,
        ) -> CUresult>(b"cuGetExportTable\0")
        .unwrap();
    let mut export_table = ptr::null();
    (cu_get_export_table)(&mut export_table, &zluda_dark_api::ZludaExt::GUID);
    let zluda_ext = zluda_dark_api::ZludaExt::new(export_table);
    let stream: Result<_, _> = zluda_ext.get_hip_stream(stream_id as _).into();
    to_cuda(rocblas_set_stream(handle as _, stream.unwrap() as _))
}

fn set_math_mode(handle: cublasHandle_t, mode: cublasMath_t) -> cublasStatus_t {
    // llama.cpp uses CUBLAS_TF32_TENSOR_OP_MATH
    cublasStatus_t::CUBLAS_STATUS_SUCCESS
}

unsafe fn sgemm_v2(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const f32,
    a: *const f32,
    lda: i32,
    b: *const f32,
    ldb: i32,
    beta: *const f32,
    c: *mut f32,
    ldc: i32,
) -> cublasStatus_t {
    let transa = op_from_cuda(transa);
    let transb = op_from_cuda(transb);
    to_cuda(rocblas_sgemm(
        handle as _,
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
    ))
}

unsafe fn init() -> cublasStatus_t {
    rocblas_initialize();
    cublasStatus_t::CUBLAS_STATUS_SUCCESS
}

unsafe fn dasum_v2(
    handle: *mut cublasContext,
    n: i32,
    x: *const f64,
    incx: i32,
    result: *mut f64,
) -> cublasStatus_t {
    to_cuda(rocblas_dasum(handle as _, n, x, incx, result))
}

unsafe fn daxpy_v2(
    handle: *mut cublasContext,
    n: i32,
    alpha: *const f64,
    x: *const f64,
    incx: i32,
    y: *mut f64,
    incy: i32,
) -> cublasStatus_t {
    to_cuda(rocblas_daxpy(handle as _, n, alpha, x, incx, y, incy))
}

unsafe fn ddot_v2(
    handle: *mut cublasContext,
    n: i32,
    x: *const f64,
    incx: i32,
    y: *const f64,
    incy: i32,
    result: *mut f64,
) -> cublasStatus_t {
    to_cuda(rocblas_ddot(handle as _, n, x, incx, y, incy, result))
}

unsafe fn dscal_v2(
    handle: *mut cublasContext,
    n: i32,
    alpha: *const f64,
    x: *mut f64,
    incx: i32,
) -> cublasStatus_t {
    to_cuda(rocblas_dscal(handle as _, n, alpha, x, incx))
}

unsafe fn dnrm_v2(
    handle: *mut cublasContext,
    n: i32,
    x: *const f64,
    incx: i32,
    result: *mut f64,
) -> cublasStatus_t {
    to_cuda(rocblas_dnrm2(handle.cast(), n, x, incx, result))
}

unsafe fn idamax_v2(
    handle: *mut cublasContext,
    n: i32,
    x: *const f64,
    incx: i32,
    result: *mut i32,
) -> cublasStatus_t {
    to_cuda(rocblas_idamax(handle.cast(), n, x, incx, result))
}

unsafe fn set_workspace(
    handle: *mut cublasContext,
    workspace: *mut std::ffi::c_void,
    workspace_size_in_bytes: usize,
) -> cublasStatus_t {
    to_cuda(rocblas_set_workspace(
        handle.cast(),
        workspace,
        workspace_size_in_bytes,
    ))
}

unsafe fn gemm_ex(
    handle: *mut cublasContext,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const std::ffi::c_void,
    a: *const std::ffi::c_void,
    atype: cudaDataType_t,
    lda: i32,
    b: *const std::ffi::c_void,
    btype: cudaDataType_t,
    ldb: i32,
    beta: *const std::ffi::c_void,
    c: *mut std::ffi::c_void,
    ctype: cudaDataType_t,
    ldc: i32,
    compute_type: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    let transa = op_from_cuda(transa);
    let transb = op_from_cuda(transb);
    let atype = type_from_cuda(atype);
    let btype = type_from_cuda(btype);
    let ctype = type_from_cuda(ctype);
    let compute_type = to_compute_type(compute_type);
    let algo = to_algo(algo);
    to_cuda(rocblas_gemm_ex(
        handle.cast(),
        transa,
        transb,
        m,
        n,
        k,
        alpha,
        a,
        atype,
        lda,
        b,
        btype,
        ldb,
        beta,
        c,
        ctype,
        ldc,
        c,
        ctype,
        ldc,
        compute_type,
        algo,
        0,
        0,
    ))
}

fn to_algo(algo: cublasGemmAlgo_t) -> rocblas_gemm_algo_ {
    // only option
    rocblas_gemm_algo::rocblas_gemm_algo_standard
}

fn to_compute_type(compute_type: cublasComputeType_t) -> rocblas_datatype {
    match compute_type {
        cublasComputeType_t::CUBLAS_COMPUTE_16F
        | cublasComputeType_t::CUBLAS_COMPUTE_16F_PEDANTIC
        | cublasComputeType_t::CUBLAS_COMPUTE_32F_FAST_16F => {
            rocblas_datatype::rocblas_datatype_f16_r
        }
        cublasComputeType_t::CUBLAS_COMPUTE_32F
        | cublasComputeType_t::CUBLAS_COMPUTE_32F_PEDANTIC
        | cublasComputeType_t::CUBLAS_COMPUTE_32F_FAST_TF32 => {
            rocblas_datatype::rocblas_datatype_f32_r
        }
        cublasComputeType_t::CUBLAS_COMPUTE_32F_FAST_16BF => {
            rocblas_datatype::rocblas_datatype_bf16_r
        }
        cublasComputeType_t::CUBLAS_COMPUTE_64F
        | cublasComputeType_t::CUBLAS_COMPUTE_64F_PEDANTIC => {
            rocblas_datatype::rocblas_datatype_f64_r
        }
        cublasComputeType_t::CUBLAS_COMPUTE_32I
        | cublasComputeType_t::CUBLAS_COMPUTE_32I_PEDANTIC => {
            rocblas_datatype::rocblas_datatype_i32_r
        }
        _ => panic!(),
    }
}

unsafe fn zgemm_strided_batch(
    handle: *mut cublasContext,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const double2,
    a: *const double2,
    lda: i32,
    stride_a: i64,
    b: *const double2,
    ldb: i32,
    stride_b: i64,
    beta: *const double2,
    c: *mut double2,
    ldc: i32,
    stride_c: i64,
    batch_count: i32,
) -> cublasStatus_t {
    let transa = op_from_cuda(transa);
    let transb = op_from_cuda(transb);
    to_cuda(rocblas_zgemm_strided_batched(
        handle.cast(),
        transa,
        transb,
        m,
        n,
        k,
        alpha.cast(),
        a.cast(),
        lda,
        stride_a,
        b.cast(),
        ldb,
        stride_b,
        beta.cast(),
        c.cast(),
        ldc,
        stride_c,
        batch_count,
    ))
}

unsafe fn cgemm_strided_batch(
    handle: *mut cublasContext,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const float2,
    a: *const float2,
    lda: i32,
    stride_a: i64,
    b: *const float2,
    ldb: i32,
    stride_b: i64,
    beta: *const float2,
    c: *mut float2,
    ldc: i32,
    stride_c: i64,
    batch_count: i32,
) -> cublasStatus_t {
    let transa = op_from_cuda(transa);
    let transb = op_from_cuda(transb);
    to_cuda(rocblas_cgemm_strided_batched(
        handle.cast(),
        transa,
        transb,
        m,
        n,
        k,
        alpha.cast(),
        a.cast(),
        lda,
        stride_a,
        b.cast(),
        ldb,
        stride_b,
        beta.cast(),
        c.cast(),
        ldc,
        stride_c,
        batch_count,
    ))
}

unsafe fn dgemm_strided_batch(
    handle: *mut cublasContext,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const f64,
    a: *const f64,
    lda: i32,
    stride_a: i64,
    b: *const f64,
    ldb: i32,
    stride_b: i64,
    beta: *const f64,
    c: *mut f64,
    ldc: i32,
    stride_c: i64,
    batch_count: i32,
) -> cublasStatus_t {
    let transa = op_from_cuda(transa);
    let transb = op_from_cuda(transb);
    to_cuda(rocblas_dgemm_strided_batched(
        handle.cast(),
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
    ))
}

unsafe fn sgemm_strided_batch(
    handle: *mut cublasContext,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const f32,
    a: *const f32,
    lda: i32,
    stride_a: i64,
    b: *const f32,
    ldb: i32,
    stride_b: i64,
    beta: *const f32,
    c: *mut f32,
    ldc: i32,
    stride_c: i64,
    batch_count: i32,
) -> cublasStatus_t {
    let transa = op_from_cuda(transa);
    let transb = op_from_cuda(transb);
    to_cuda(rocblas_sgemm_strided_batched(
        handle.cast(),
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
    ))
}

unsafe fn zgetrf_batched(
    handle: *mut cublasContext,
    n: i32,
    a: *const *mut double2,
    lda: i32,
    p: *mut i32,
    info: *mut i32,
    batch_size: i32,
) -> cublasStatus_t {
    to_cuda_solver(rocsolver_zgetrf_batched(
        handle.cast(),
        n,
        n,
        a.cast(),
        lda,
        p,
        n as i64,
        info,
        batch_size,
    ))
}

unsafe fn cgetrf_batched(
    handle: *mut cublasContext,
    n: i32,
    a: *const *mut float2,
    lda: i32,
    p: *mut i32,
    info: *mut i32,
    batch_size: i32,
) -> cublasStatus_t {
    to_cuda_solver(rocsolver_cgetrf_batched(
        handle.cast(),
        n,
        n,
        a.cast(),
        lda,
        p,
        n as i64,
        info,
        batch_size,
    ))
}

unsafe fn zgetri_batched(
    handle: *mut cublasContext,
    n: i32,
    a: *const *const double2,
    lda: i32,
    p: *const i32,
    c: *const *mut double2,
    ldc: i32,
    info: *mut i32,
    batch_size: i32,
) -> cublasStatus_t {
    to_cuda_solver(rocsolver_zgetri_outofplace_batched(
        handle.cast(),
        n,
        a.cast(),
        lda,
        p.cast_mut(),
        n as i64,
        c.cast(),
        ldc,
        info,
        batch_size,
    ))
}

unsafe fn cgetri_batched(
    handle: *mut cublasContext,
    n: i32,
    a: *const *const float2,
    lda: i32,
    p: *const i32,
    c: *const *mut float2,
    ldc: i32,
    info: *mut i32,
    batch_size: i32,
) -> cublasStatus_t {
    to_cuda_solver(rocsolver_cgetri_outofplace_batched(
        handle.cast(),
        n,
        a.cast(),
        lda,
        p.cast_mut(),
        n as i64,
        c.cast(),
        ldc,
        info,
        batch_size,
    ))
}

unsafe fn dtrmm_v2(
    handle: *mut cublasContext,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    transa: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: *const f64,
    a: *const f64,
    lda: i32,
    b: *const f64,
    ldb: i32,
    c: *mut f64,
    ldc: i32,
) -> cublasStatus_t {
    let side = to_side(side);
    let uplo = to_fill(uplo);
    let transa = op_from_cuda(transa);
    let diag = to_diag(diag);
    to_cuda(rocblas_dtrmm_outofplace(
        handle.cast(),
        side,
        uplo,
        transa,
        diag,
        m,
        n,
        alpha,
        a,
        lda,
        b,
        ldb,
        c,
        ldc,
    ))
}

fn to_side(side: cublasSideMode_t) -> rocblas_side {
    match side {
        cublasSideMode_t::CUBLAS_SIDE_LEFT => rocblas_side::rocblas_side_left,
        cublasSideMode_t::CUBLAS_SIDE_RIGHT => rocblas_side::rocblas_side_right,
        _ => panic!(),
    }
}

fn to_fill(uplo: cublasFillMode_t) -> rocblas_fill {
    match uplo {
        cublasFillMode_t::CUBLAS_FILL_MODE_LOWER => rocblas_fill::rocblas_fill_lower,
        cublasFillMode_t::CUBLAS_FILL_MODE_UPPER => rocblas_fill::rocblas_fill_upper,
        cublasFillMode_t::CUBLAS_FILL_MODE_FULL => rocblas_fill::rocblas_fill_full,
        _ => panic!(),
    }
}

fn to_diag(diag: cublasDiagType_t) -> rocblas_diagonal {
    match diag {
        cublasDiagType_t::CUBLAS_DIAG_NON_UNIT => rocblas_diagonal::rocblas_diagonal_non_unit,
        cublasDiagType_t::CUBLAS_DIAG_UNIT => rocblas_diagonal::rocblas_diagonal_unit,
        _ => panic!(),
    }
}

unsafe fn dgemv_v2(
    handle: *mut cublasContext,
    trans: cublasOperation_t,
    m: i32,
    n: i32,
    alpha: *const f64,
    a: *const f64,
    lda: i32,
    x: *const f64,
    incx: i32,
    beta: *const f64,
    y: *mut f64,
    incy: i32,
) -> cublasStatus_t {
    let trans: rocblas_operation = op_from_cuda(trans);
    to_cuda(rocblas_dgemv(
        handle.cast(),
        trans,
        m,
        n,
        alpha,
        a,
        lda,
        x,
        incx,
        beta,
        y,
        incy,
    ))
}

unsafe fn get_pointer_mode(
    handle: cublasHandle_t,
    mode: *mut cublasPointerMode_t,
) -> cublasStatus_t {
    to_cuda(rocblas_get_pointer_mode(handle.cast(), mode.cast()))
}

unsafe fn set_pointer_mode(handle: cublasHandle_t, mode: cublasPointerMode_t) -> cublasStatus_t {
    to_cuda(rocblas_set_pointer_mode(
        handle.cast(),
        rocblas_pointer_mode_(mode.0),
    ))
}

unsafe fn drot_v2(
    handle: *mut cublasContext,
    n: i32,
    x: *mut f64,
    incx: i32,
    y: *mut f64,
    incy: i32,
    c: *const f64,
    s: *const f64,
) -> cublasStatus_t {
    to_cuda(rocblas_drot(handle.cast(), n, x, incx, y, incy, c, s))
}

unsafe fn drotg(
    handle: *mut cublasContext,
    a: *mut f64,
    b: *mut f64,
    c: *mut f64,
    s: *mut f64,
) -> cublasStatus_t {
    to_cuda(rocblas_drotg(handle.cast(), a, b, c, s))
}

unsafe fn drotm(
    handle: *mut cublasContext,
    n: i32,
    x: *mut f64,
    incx: i32,
    y: *mut f64,
    incy: i32,
    param: *const f64,
) -> cublasStatus_t {
    to_cuda(rocblas_drotm(handle.cast(), n, x, incx, y, incy, param))
}

unsafe fn drotmg(
    handle: *mut cublasContext,
    d1: *mut f64,
    d2: *mut f64,
    x1: *mut f64,
    y1: *const f64,
    param: *mut f64,
) -> cublasStatus_t {
    to_cuda(rocblas_drotmg(handle.cast(), d1, d2, x1, y1, param))
}

unsafe fn dswap(
    handle: *mut cublasContext,
    n: i32,
    x: *mut f64,
    incx: i32,
    y: *mut f64,
    incy: i32,
) -> cublasStatus_t {
    to_cuda(rocblas_dswap(handle.cast(), n, x, incx, y, incy))
}

unsafe fn dger(
    handle: *mut cublasContext,
    m: i32,
    n: i32,
    alpha: *const f64,
    x: *const f64,
    incx: i32,
    y: *const f64,
    incy: i32,
    a: *mut f64,
    lda: i32,
) -> cublasStatus_t {
    to_cuda(rocblas_dger(
        handle.cast(),
        m,
        n,
        alpha,
        x,
        incx,
        y,
        incy,
        a,
        lda,
    ))
}

unsafe fn dgemm(
    handle: *mut cublasContext,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i32,
    n: i32,
    k: i32,
    alpha: *const f64,
    a: *const f64,
    lda: i32,
    b: *const f64,
    ldb: i32,
    beta: *const f64,
    c: *mut f64,
    ldc: i32,
) -> cublasStatus_t {
    let transa = op_from_cuda(transa);
    let transb = op_from_cuda(transb);
    to_cuda(rocblas_dgemm(
        handle.cast(),
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
    ))
}

unsafe fn dtrsm(
    handle: *mut cublasContext,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i32,
    n: i32,
    alpha: *const f64,
    a: *const f64,
    lda: i32,
    b: *mut f64,
    ldb: i32,
) -> cublasStatus_t {
    let side = to_side(side);
    let uplo = to_fill(uplo);
    let trans = op_from_cuda(trans);
    let diag = to_diag(diag);
    to_cuda(rocblas_dtrsm(
        handle.cast(),
        side,
        uplo,
        trans,
        diag,
        m,
        n,
        alpha,
        a,
        lda,
        b,
        ldb,
    ))
}
