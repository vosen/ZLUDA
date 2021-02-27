#![allow(warnings)]
mod cusparse;
pub use cusparse::*;

use cuda_types::*;
use rocsparse_sys::*;
use std::{ffi::c_void, mem, ptr};

macro_rules! call {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            unsafe {
                let result = $expr;
                if result != rocsparse_sys::rocsparse_status::rocsparse_status_success {
                    return to_cuda(result);
                }
            }
        }
    };
}

#[cfg(debug_assertions)]
pub(crate) fn unsupported() -> cusparseStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unsupported() -> cusparseStatus_t {
    cusparseStatus_t::CUSPARSE_STATUS_NOT_SUPPORTED
}

// TODO: be more thorough
fn to_cuda(status: rocsparse_status) -> cusparseStatus_t {
    match status {
        rocsparse_status::rocsparse_status_success => cusparseStatus_t::CUSPARSE_STATUS_SUCCESS,
        _ => cusparseStatus_t::CUSPARSE_STATUS_INTERNAL_ERROR,
    }
}

unsafe fn create(handle: *mut *mut cusparseContext) -> cusparseStatus_t {
    to_cuda(rocsparse_create_handle(handle as _))
}

unsafe fn create_csr(
    descr: *mut cusparseSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    csr_row_offsets: *mut std::ffi::c_void,
    csr_col_ind: *mut std::ffi::c_void,
    csr_values: *mut std::ffi::c_void,
    csr_row_offsets_type: cusparseIndexType_t,
    csr_col_ind_type: cusparseIndexType_t,
    idx_base: cusparseIndexBase_t,
    value_type: cudaDataType_t,
) -> cusparseStatus_t {
    let csr_row_offsets_type = index_type(csr_row_offsets_type);
    let csr_col_ind_type = index_type(csr_col_ind_type);
    let idx_base = index_base(idx_base);
    let value_type = data_type(value_type);
    to_cuda(rocsparse_create_csr_descr(
        descr as _,
        rows,
        cols,
        nnz,
        csr_row_offsets,
        csr_col_ind,
        csr_values,
        csr_row_offsets_type,
        csr_col_ind_type,
        idx_base,
        value_type,
    ))
}

fn data_type(data_type: cudaDataType_t) -> rocsparse_datatype {
    match data_type {
        cudaDataType_t::CUDA_R_32F => rocsparse_datatype::rocsparse_datatype_f32_r,
        cudaDataType_t::CUDA_R_64F => rocsparse_datatype::rocsparse_datatype_f64_r,
        cudaDataType_t::CUDA_C_32F => rocsparse_datatype::rocsparse_datatype_f32_c,
        cudaDataType_t::CUDA_C_64F => rocsparse_datatype::rocsparse_datatype_f64_c,
        cudaDataType_t::CUDA_R_8I => rocsparse_datatype::rocsparse_datatype_i8_r,
        cudaDataType_t::CUDA_R_8U => rocsparse_datatype::rocsparse_datatype_u8_r,
        cudaDataType_t::CUDA_R_32I => rocsparse_datatype::rocsparse_datatype_i32_r,
        cudaDataType_t::CUDA_R_32U => rocsparse_datatype::rocsparse_datatype_u32_r,
        _ => panic!(),
    }
}

fn index_type(index_type: cusparseIndexType_t) -> rocsparse_indextype {
    match index_type {
        cusparseIndexType_t::CUSPARSE_INDEX_16U => rocsparse_indextype::rocsparse_indextype_u16,
        cusparseIndexType_t::CUSPARSE_INDEX_32I => rocsparse_indextype::rocsparse_indextype_i32,
        cusparseIndexType_t::CUSPARSE_INDEX_64I => rocsparse_indextype::rocsparse_indextype_i64,
        _ => panic!(),
    }
}

fn index_base(index_base: cusparseIndexBase_t) -> rocsparse_index_base {
    match index_base {
        cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ZERO => {
            rocsparse_index_base::rocsparse_index_base_zero
        }
        cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ONE => {
            rocsparse_index_base::rocsparse_index_base_one
        }
        _ => panic!(),
    }
}

unsafe fn create_csrsv2_info(info: *mut *mut csrsv2Info) -> cusparseStatus_t {
    to_cuda(rocsparse_create_mat_info(info.cast()))
}

unsafe fn create_dn_vec(
    dn_vec_descr: *mut *mut cusparseDnVecDescr,
    size: i64,
    values: *mut std::ffi::c_void,
    value_type: cudaDataType_t,
) -> cusparseStatus_t {
    let value_type = data_type(value_type);
    to_cuda(rocsparse_create_dnvec_descr(
        dn_vec_descr.cast(),
        size,
        values,
        value_type,
    ))
}

unsafe fn create_mat_descr(descr_a: *mut *mut cusparseMatDescr) -> cusparseStatus_t {
    to_cuda(rocsparse_create_mat_descr(descr_a.cast()))
}

unsafe fn destroy_mat_descr(descr_a: *mut cusparseMatDescr) -> cusparseStatus_t {
    to_cuda(rocsparse_destroy_mat_descr(descr_a.cast()))
}

unsafe fn dcsr_sv2_analysis(
    handle: *mut cusparseContext,
    trans_a: cusparseOperation_t,
    m: i32,
    nnz: i32,
    descr_a: *mut cusparseMatDescr,
    csr_sorted_val_a: *const f64,
    csr_sorted_row_ptr_a: *const i32,
    csr_sorted_col_ind_a: *const i32,
    info: *mut csrsv2Info,
    policy: cusparseSolvePolicy_t,
    p_buffer: *mut std::ffi::c_void,
) -> cusparseStatus_t {
    let trans_a = operation(trans_a);
    let (analysis, solve) = to_policy(policy);
    to_cuda(rocsparse_dcsrsv_analysis(
        handle.cast(),
        trans_a,
        m,
        nnz,
        descr_a.cast(),
        csr_sorted_val_a,
        csr_sorted_row_ptr_a,
        csr_sorted_col_ind_a,
        info.cast(),
        analysis,
        solve,
        p_buffer,
    ))
}

fn to_policy(policy: cusparseSolvePolicy_t) -> (rocsparse_analysis_policy, rocsparse_solve_policy) {
    match policy {
        cusparseSolvePolicy_t::CUSPARSE_SOLVE_POLICY_NO_LEVEL => (
            rocsparse_analysis_policy::rocsparse_analysis_policy_reuse,
            rocsparse_solve_policy::rocsparse_solve_policy_auto,
        ),
        cusparseSolvePolicy_t::CUSPARSE_SOLVE_POLICY_USE_LEVEL => (
            rocsparse_analysis_policy::rocsparse_analysis_policy_reuse,
            rocsparse_solve_policy::rocsparse_solve_policy_auto,
        ),
        _ => panic!(),
    }
}

fn operation(op: cusparseOperation_t) -> rocsparse_operation {
    match op {
        cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE => {
            rocsparse_operation::rocsparse_operation_none
        }
        cusparseOperation_t::CUSPARSE_OPERATION_TRANSPOSE => {
            rocsparse_operation::rocsparse_operation_transpose
        }
        cusparseOperation_t::CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE => {
            rocsparse_operation::rocsparse_operation_conjugate_transpose
        }
        _ => panic!(),
    }
}

unsafe fn dcsr_sv2_buffersize(
    handle: *mut cusparseContext,
    trans_a: cusparseOperation_t,
    m: i32,
    nnz: i32,
    descr_a: *mut cusparseMatDescr,
    csr_sorted_val_a: *mut f64,
    csr_sorted_row_ptr_a: *const i32,
    csr_sorted_col_ind_a: *const i32,
    info: *mut csrsv2Info,
    p_buffer_size_in_bytes: *mut i32,
) -> cusparseStatus_t {
    let trans_a = operation(trans_a);
    let mut size = *p_buffer_size_in_bytes as usize;
    let result = to_cuda(rocsparse_dcsrsv_buffer_size(
        handle.cast(),
        trans_a,
        m,
        nnz,
        descr_a.cast(),
        csr_sorted_val_a,
        csr_sorted_row_ptr_a,
        csr_sorted_col_ind_a,
        info.cast(),
        &mut size as *mut usize,
    ));
    if size > i32::MAX as usize {
        return cusparseStatus_t::CUSPARSE_STATUS_INSUFFICIENT_RESOURCES;
    }
    *p_buffer_size_in_bytes = size as i32;
    result
}

unsafe fn dcsr_sv2_solve(
    handle: *mut cusparseContext,
    trans_a: cusparseOperation_t,
    m: i32,
    nnz: i32,
    alpha: *const f64,
    descr_a: *mut cusparseMatDescr,
    csr_sorted_val_a: *const f64,
    csr_sorted_row_ptr_a: *const i32,
    csr_sorted_col_ind_a: *const i32,
    info: *mut csrsv2Info,
    f: *const f64,
    x: *mut f64,
    policy: cusparseSolvePolicy_t,
    p_buffer: *mut std::ffi::c_void,
) -> cusparseStatus_t {
    let trans_a = operation(trans_a);
    let (_, policy) = to_policy(policy);
    to_cuda(rocsparse_dcsrsv_solve(
        handle.cast(),
        trans_a,
        m,
        nnz,
        alpha,
        descr_a.cast(),
        csr_sorted_val_a,
        csr_sorted_row_ptr_a,
        csr_sorted_col_ind_a,
        info.cast(),
        f,
        x,
        policy,
        p_buffer,
    ))
}

unsafe fn dnvec_set_values(
    dn_vec_descr: *mut cusparseDnVecDescr,
    values: *mut std::ffi::c_void,
) -> cusparseStatus_t {
    to_cuda(rocsparse_dnvec_set_values(dn_vec_descr.cast(), values))
}

unsafe fn get_mat_diag_type(descr_a: *mut cusparseMatDescr) -> cusparseDiagType_t {
    diag_type(rocsparse_get_mat_diag_type(descr_a.cast()))
}

fn diag_type(diag_type: rocsparse_diag_type) -> cusparseDiagType_t {
    match diag_type {
        rocsparse_diag_type::rocsparse_diag_type_non_unit => {
            cusparseDiagType_t::CUSPARSE_DIAG_TYPE_NON_UNIT
        }
        rocsparse_diag_type::rocsparse_diag_type_unit => {
            cusparseDiagType_t::CUSPARSE_DIAG_TYPE_UNIT
        }
        _ => panic!(),
    }
}

unsafe fn set_mat_diag_type(
    descr_a: *mut cusparseMatDescr,
    diag_type: cusparseDiagType_t,
) -> cusparseStatus_t {
    to_cuda(rocsparse_set_mat_diag_type(
        descr_a.cast(),
        diag_type_reverse(diag_type),
    ))
}

fn diag_type_reverse(diag_type: cusparseDiagType_t) -> rocsparse_diag_type {
    match diag_type {
        cusparseDiagType_t::CUSPARSE_DIAG_TYPE_NON_UNIT => {
            rocsparse_diag_type::rocsparse_diag_type_non_unit
        }
        cusparseDiagType_t::CUSPARSE_DIAG_TYPE_UNIT => {
            rocsparse_diag_type::rocsparse_diag_type_unit
        }
        _ => panic!(),
    }
}

unsafe fn get_mat_fill_mode(descr_a: *mut cusparseMatDescr) -> cusparseFillMode_t {
    fill_mode(rocsparse_get_mat_fill_mode(descr_a.cast()))
}

fn fill_mode(fill_mode: rocsparse_fill_mode) -> cusparseFillMode_t {
    match fill_mode {
        rocsparse_fill_mode::rocsparse_fill_mode_lower => {
            cusparseFillMode_t::CUSPARSE_FILL_MODE_LOWER
        }
        rocsparse_fill_mode::rocsparse_fill_mode_upper => {
            cusparseFillMode_t::CUSPARSE_FILL_MODE_UPPER
        }
        _ => panic!(),
    }
}

unsafe fn set_mat_fill_mode(
    descr_a: *mut cusparseMatDescr,
    fill_mode: cusparseFillMode_t,
) -> cusparseStatus_t {
    to_cuda(rocsparse_set_mat_fill_mode(
        descr_a.cast(),
        fill_mode_reverse(fill_mode),
    ))
}

fn fill_mode_reverse(fill_mode: cusparseFillMode_t) -> rocsparse_fill_mode {
    match fill_mode {
        cusparseFillMode_t::CUSPARSE_FILL_MODE_LOWER => {
            rocsparse_fill_mode::rocsparse_fill_mode_lower
        }
        cusparseFillMode_t::CUSPARSE_FILL_MODE_UPPER => {
            rocsparse_fill_mode::rocsparse_fill_mode_upper
        }
        _ => panic!(),
    }
}

unsafe fn get_pointer_mode(
    handle: *mut cusparseContext,
    mode: *mut cusparsePointerMode_t,
) -> cusparseStatus_t {
    let mut pointer_mode = mem::zeroed();
    let result = to_cuda(rocsparse_get_pointer_mode(handle.cast(), &mut pointer_mode));
    if result != cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
        return result;
    }
    *mode = to_pointer_mode(pointer_mode);
    result
}

fn to_pointer_mode(pointer_mode: rocsparse_pointer_mode) -> cusparsePointerMode_t {
    match pointer_mode {
        rocsparse_pointer_mode::rocsparse_pointer_mode_host => {
            cusparsePointerMode_t::CUSPARSE_POINTER_MODE_HOST
        }
        rocsparse_pointer_mode::rocsparse_pointer_mode_device => {
            cusparsePointerMode_t::CUSPARSE_POINTER_MODE_DEVICE
        }
        _ => panic!(),
    }
}

unsafe fn set_pointer_mode(
    handle: *mut cusparseContext,
    mode: cusparsePointerMode_t,
) -> cusparseStatus_t {
    to_cuda(rocsparse_set_pointer_mode(
        handle.cast(),
        to_pointer_mode_reverse(mode),
    ))
}

fn to_pointer_mode_reverse(pointer_mode: cusparsePointerMode_t) -> rocsparse_pointer_mode {
    match pointer_mode {
        cusparsePointerMode_t::CUSPARSE_POINTER_MODE_HOST => {
            rocsparse_pointer_mode::rocsparse_pointer_mode_host
        }
        cusparsePointerMode_t::CUSPARSE_POINTER_MODE_DEVICE => {
            rocsparse_pointer_mode::rocsparse_pointer_mode_device
        }
        _ => panic!(),
    }
}

unsafe fn set_mat_index_base(
    descr_a: *mut cusparseMatDescr,
    base: cusparseIndexBase_t,
) -> cusparseStatus_t {
    to_cuda(rocsparse_set_mat_index_base(
        descr_a.cast(),
        index_base(base),
    ))
}

unsafe fn set_mat_type(
    descr_a: *mut cusparseMatDescr,
    type_: cusparseMatrixType_t,
) -> cusparseStatus_t {
    to_cuda(rocsparse_set_mat_type(descr_a.cast(), matrix_type(type_)))
}

fn matrix_type(type_: cusparseMatrixType_t) -> rocsparse_matrix_type_ {
    match type_ {
        cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_GENERAL => {
            rocsparse_matrix_type_::rocsparse_matrix_type_general
        }
        cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_SYMMETRIC => {
            rocsparse_matrix_type_::rocsparse_matrix_type_symmetric
        }
        cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_HERMITIAN => {
            rocsparse_matrix_type_::rocsparse_matrix_type_hermitian
        }
        cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_TRIANGULAR => {
            rocsparse_matrix_type_::rocsparse_matrix_type_triangular
        }
        _ => panic!(),
    }
}

unsafe fn set_stream(handle: *mut cusparseContext, stream_id: CUstream) -> cusparseStatus_t {
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
    to_cuda(rocsparse_set_stream(handle as _, stream.unwrap() as _))
}

unsafe fn spmv(
    handle: *mut cusparseContext,
    op_a: cusparseOperation_t,
    alpha: *const std::ffi::c_void,
    mat_a: cusparseSpMatDescr_t,
    vec_x: *mut cusparseDnVecDescr,
    beta: *const std::ffi::c_void,
    vec_y: *mut cusparseDnVecDescr,
    compute_type: cudaDataType_t,
    alg: cusparseSpMVAlg_t,
    external_buffer: *mut std::ffi::c_void,
) -> cusparseStatus_t {
    let op_a = operation(op_a);
    let compute_type = data_type(compute_type);
    let alg = to_spmv_alg(alg);
    // divide by 2 in case there's any arithmetic done on it
    let mut size = usize::MAX / 2;
    to_cuda(rocsparse_spmv(
        handle.cast(),
        op_a,
        alpha,
        mat_a.cast(),
        vec_x.cast(),
        beta,
        vec_y.cast(),
        compute_type,
        alg,
        &mut size,
        external_buffer,
    ))
}

unsafe fn spmv_buffersize(
    handle: *mut cusparseContext,
    op_a: cusparseOperation_t,
    alpha: *const std::ffi::c_void,
    mat_a: cusparseSpMatDescr_t,
    vec_x: *mut cusparseDnVecDescr,
    beta: *const std::ffi::c_void,
    vec_y: *mut cusparseDnVecDescr,
    compute_type: cudaDataType_t,
    alg: cusparseSpMVAlg_t,
    buffer_size: *mut usize,
) -> cusparseStatus_t {
    let op_a = operation(op_a);
    let compute_type = data_type(compute_type);
    let alg = to_spmv_alg(alg);
    to_cuda(rocsparse_spmv(
        handle.cast(),
        op_a,
        alpha,
        mat_a.cast(),
        vec_x.cast(),
        beta,
        vec_y.cast(),
        compute_type,
        alg,
        buffer_size,
        ptr::null_mut(),
    ))
}

fn to_spmv_alg(alg: cusparseSpMVAlg_t) -> rocsparse_spmv_alg {
    match alg {
        cusparseSpMVAlg_t::CUSPARSE_SPMV_ALG_DEFAULT => {
            rocsparse_spmv_alg::rocsparse_spmv_alg_default
        }
        cusparseSpMVAlg_t::CUSPARSE_SPMV_COO_ALG1 => rocsparse_spmv_alg::rocsparse_spmv_alg_coo,
        cusparseSpMVAlg_t::CUSPARSE_SPMV_CSR_ALG1 => {
            rocsparse_spmv_alg::rocsparse_spmv_alg_csr_adaptive
        }
        cusparseSpMVAlg_t::CUSPARSE_SPMV_CSR_ALG2 => {
            rocsparse_spmv_alg::rocsparse_spmv_alg_csr_stream
        }
        cusparseSpMVAlg_t::CUSPARSE_SPMV_COO_ALG2 => {
            rocsparse_spmv_alg::rocsparse_spmv_alg_coo_atomic
        }
        // other vlaues definied by cuSPARSE are aliases
        _ => panic!(),
    }
}

unsafe fn destroy_sp_mat(sp_mat_descr: cusparseSpMatDescr_t) -> cusparseStatus_t {
    if sp_mat_descr == ptr::null_mut() {
        cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
    } else {
        to_cuda(rocsparse_destroy_spmat_descr(sp_mat_descr.cast()))
    }
}

unsafe fn spgemm_create_descr(descr: *mut *mut cusparseSpGEMMDescr) -> cusparseStatus_t {
    *(descr.cast()) = Box::into_raw(Box::new(SPGEMMDescr {
        temp_buffer_size: 0,
        temp_buffer: ptr::null_mut(),
        external_buffer3: ptr::null_mut(),
    }));
    cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
}

struct SPGEMMDescr {
    temp_buffer_size: usize,
    temp_buffer: *mut c_void,
    external_buffer3: *mut c_void,
}

// If the case where we are doing spgemm and the C matrix has only NULL pointers, and
// we _must_ support it (for example petsc relies on this behavior), then:
// * `rocsparse_spgemm(..., rocsparse_spgemm_stage_nnz, ...)` fails because row pointer is NULL
// * If we try to set and unset pointers through `rocsparse_csr_set_pointers(...)` this fails because two other pointers are NULL
// * If we try to create whole new matrix C', this also fails, because there are some private fields that are set
//   during `rocsparse_spgemm(..., rocsparse_spgemm_stage_buffer_size, ...)`
// * Another solution: creating a fake C' matrix during `rocsparse_spgemm(..., rocsparse_spgemm_stage_buffer_size, ...)`
//   and actually using it during `rocsparse_spgemm(..., rocsparse_spgemm_stage_nnz, ...)`, this has the problem that
//   there's no way to copy internal fields from C' to C
// * All that's left is to YOLO it and start touching matrix descriptor internals
#[repr(C)]
struct rocsparse_spmat_descr_internal {
    init: bool,
    analysed: bool,

    rows: i64,
    cols: i64,
    nnz: i64,

    row_data: *mut c_void,
    col_data: *mut c_void,
    ind_data: *mut c_void,
    val_data: *mut c_void,
}

unsafe fn spgemm_destroy_desc(descr: cusparseSpGEMMDescr_t) -> cusparseStatus_t {
    Box::from_raw(descr.cast::<SPGEMMDescr>());
    cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
}

unsafe fn spgemm_reuse_workestimation(
    handle: *mut cusparseContext,
    op_a: cusparseOperation_t,
    op_b: cusparseOperation_t,
    mat_a: cusparseSpMatDescr_t,
    mat_b: cusparseSpMatDescr_t,
    mat_c: cusparseSpMatDescr_t,
    alg: cusparseSpGEMMAlg_t,
    spgemm_descr: *mut cusparseSpGEMMDescr,
    buffer_size1: *mut usize,
    external_buffer1: *mut std::ffi::c_void,
) -> cusparseStatus_t {
    if external_buffer1 == ptr::null_mut() {
        *buffer_size1 = 1;
        /*
        let mut c_rows = 0;
        let mut c_row_ptr = ptr::null_mut();
        let mut data_type = rocsparse_datatype(0);
        let error = to_cuda(rocsparse_csr_get(
            mat_c.cast(),
            &mut c_rows,
            &mut 0,
            &mut 0,
            &mut c_row_ptr,
            &mut ptr::null_mut(),
            &mut ptr::null_mut(),
            &mut rocsparse_indextype(0),
            &mut rocsparse_indextype(0),
            &mut rocsparse_index_base(0),
            &mut data_type,
        ));
        if error != cusparseStatus_t::CUSPARSE_STATUS_SUCCESS {
            return error;
        }
        *buffer_size1 = if c_row_ptr == ptr::null_mut() {
            (c_rows + 1) as usize * element_size(data_type)
        } else {
            1
        };
         */
    } else {
        /*
        let spgemm_descr = spgemm_descr.cast::<SPGEMMDescr>().as_mut().unwrap();
        spgemm_descr.external_buffer1 = external_buffer1;
        */
    }
    cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
}

unsafe fn spgemm_reuse_nnz(
    handle: *mut cusparseContext,
    op_a: cusparseOperation_t,
    op_b: cusparseOperation_t,
    mat_a: cusparseSpMatDescr_t,
    mat_b: cusparseSpMatDescr_t,
    mat_c: cusparseSpMatDescr_t,
    alg: cusparseSpGEMMAlg_t,
    spgemm_descr: *mut cusparseSpGEMMDescr,
    buffer_size2: *mut usize,
    external_buffer2: *mut std::ffi::c_void,
    buffer_size3: *mut usize,
    external_buffer3: *mut std::ffi::c_void,
    buffer_size4: *mut usize,
    external_buffer4: *mut std::ffi::c_void,
) -> cusparseStatus_t {
    if external_buffer2 == ptr::null_mut() {
        *buffer_size2 = 1;
    }
    let op_a = operation(op_a);
    let op_b = operation(op_b);
    let mut data_type = rocsparse_datatype(0);
    // rocSPARSE checks later if mat_a, mat_b and mat_c are the same data type
    let mut c_rows = 0;
    let mut c_cols = 0;
    let mut c_nnz = 0;
    let mut c_row_ptr = ptr::null_mut();
    let mut c_col_ind = ptr::null_mut();
    let mut c_val = ptr::null_mut();
    let mut c_row_ptr_type = rocsparse_indextype::rocsparse_indextype_i32;
    let mut c_col_ind_type = rocsparse_indextype::rocsparse_indextype_i32;
    let mut c_idx_base = rocsparse_index_base::rocsparse_index_base_zero;
    call! { rocsparse_csr_get(
        mat_c.cast(),
        &mut c_rows,
        &mut c_cols,
        &mut c_nnz,
        &mut c_row_ptr,
        &mut c_col_ind,
        &mut c_val,
        &mut c_row_ptr_type,
        &mut c_col_ind_type,
        &mut c_idx_base,
        &mut data_type,
    ) };
    if external_buffer3 == ptr::null_mut() {
        *buffer_size3 = (c_rows + 1) as usize * element_size(c_row_ptr_type);
    }
    let spgemm_descr = spgemm_descr.cast::<SPGEMMDescr>().as_mut().unwrap();
    let stage = if external_buffer4 == ptr::null_mut() {
        rocsparse_spgemm_stage::rocsparse_spgemm_stage_buffer_size
    } else {
        if c_row_ptr == ptr::null_mut() {
            let mat_c = mat_c
                .cast::<rocsparse_spmat_descr_internal>()
                .as_mut()
                .unwrap();
            assert_eq!(mat_c.row_data, ptr::null_mut());
            mat_c.row_data = external_buffer3;
            spgemm_descr.external_buffer3 = external_buffer3;
        }
        rocsparse_spgemm_stage::rocsparse_spgemm_stage_nnz
    };
    let temp_scalar = std::f64::consts::PI;
    call! { rocsparse_spgemm(
        handle.cast(),
        op_a,
        op_b,
        &temp_scalar as *const _ as *const _,
        mat_a.cast(),
        mat_b.cast(),
        &temp_scalar as *const _ as *const _,
        mat_c.cast(),
        mat_c.cast(),
        data_type,
        rocsparse_spgemm_alg::rocsparse_spgemm_alg_default,
        stage,
        buffer_size4,
        external_buffer4,
    ) };
    if stage == rocsparse_spgemm_stage::rocsparse_spgemm_stage_nnz {
        if c_row_ptr == ptr::null_mut() {
            let mat_c = mat_c
                .cast::<rocsparse_spmat_descr_internal>()
                .as_mut()
                .unwrap();
            assert_eq!(mat_c.row_data, external_buffer3);
            mat_c.row_data = ptr::null_mut();
        }
        spgemm_descr.temp_buffer_size = *buffer_size4;
        spgemm_descr.temp_buffer = external_buffer4;
    }
    cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
}

fn element_size(data_type: rocsparse_indextype) -> usize {
    match data_type {
        rocsparse_indextype::rocsparse_indextype_u16 => 2,
        rocsparse_indextype::rocsparse_indextype_i32 => 4,
        rocsparse_indextype::rocsparse_indextype_i64 => 8,
        _ => panic!(),
    }
}

unsafe fn spgemm_reuse_copy(
    handle: *mut cusparseContext,
    op_a: cusparseOperation_t,
    op_b: cusparseOperation_t,
    mat_a: cusparseSpMatDescr_t,
    mat_b: cusparseSpMatDescr_t,
    mat_c: cusparseSpMatDescr_t,
    alg: cusparseSpGEMMAlg_t,
    spgemm_descr: *mut cusparseSpGEMMDescr,
    buffer_size5: *mut usize,
    external_buffer5: *mut std::ffi::c_void,
) -> cusparseStatus_t {
    if external_buffer5 == ptr::null_mut() {
        *buffer_size5 = 1;
    }
    let spgemm_descr = spgemm_descr.cast::<SPGEMMDescr>().as_ref().unwrap();
    if spgemm_descr.external_buffer3 != ptr::null_mut() {
        let mut c_rows = 0;
        let mut c_row_ptr = ptr::null_mut();
        let mut c_row_ptr_type = rocsparse_indextype(0);
        call! { rocsparse_csr_get(
            mat_c.cast(),
            &mut c_rows,
            &mut 0,
            &mut 0,
            &mut c_row_ptr,
            &mut ptr::null_mut(),
            &mut ptr::null_mut(),
            &mut c_row_ptr_type,
            &mut rocsparse_indextype(0),
            &mut rocsparse_index_base(0),
            &mut rocsparse_datatype(0),
        ) };
        if c_row_ptr != ptr::null_mut() {
            let size: usize = (c_rows + 1) as usize * element_size(c_row_ptr_type);
            let mut hip_stream = ptr::null_mut();
            call! { rocsparse_get_stream(handle.cast(), &mut hip_stream) };
            let error = hip_runtime_sys::hipMemcpyAsync(
                c_row_ptr,
                spgemm_descr.external_buffer3,
                size,
                hip_runtime_sys::hipMemcpyKind::hipMemcpyDefault,
                hip_stream.cast(),
            );
            if error != hip_runtime_sys::hipError_t::hipSuccess {
                return cusparseStatus_t::CUSPARSE_STATUS_INTERNAL_ERROR;
            }
        }
    }
    cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
}

unsafe fn spgemm_reuse_compute(
    handle: *mut cusparseContext,
    op_a: cusparseOperation_t,
    op_b: cusparseOperation_t,
    alpha: *const std::ffi::c_void,
    mat_a: cusparseSpMatDescr_t,
    mat_b: cusparseSpMatDescr_t,
    beta: *const std::ffi::c_void,
    mat_c: cusparseSpMatDescr_t,
    compute_type: cudaDataType,
    alg: cusparseSpGEMMAlg_t,
    spgemm_descr: *mut cusparseSpGEMMDescr,
) -> cusparseStatus_t {
    let spgemm_descr = spgemm_descr.cast::<SPGEMMDescr>().as_ref();
    let spgemm_descr = match spgemm_descr {
        Some(x) => x,
        None => return cusparseStatus_t::CUSPARSE_STATUS_INVALID_VALUE,
    };
    let op_a = operation(op_a);
    let op_b = operation(op_b);
    let compute_type = to_datatype(compute_type);
    let mut temp_buffer_size = spgemm_descr.temp_buffer_size;
    to_cuda(rocsparse_spgemm(
        handle.cast(),
        op_a,
        op_b,
        alpha,
        mat_a.cast(),
        mat_b.cast(),
        beta,
        mat_c.cast(),
        mat_c.cast(),
        compute_type,
        rocsparse_spgemm_alg::rocsparse_spgemm_alg_default,
        rocsparse_spgemm_stage::rocsparse_spgemm_stage_compute,
        &mut temp_buffer_size,
        spgemm_descr.temp_buffer,
    ))
}

fn to_datatype(compute_type: cudaDataType) -> rocsparse_datatype {
    match compute_type {
        cudaDataType::CUDA_R_32F => rocsparse_datatype::rocsparse_datatype_f32_r,
        cudaDataType::CUDA_R_64F => rocsparse_datatype::rocsparse_datatype_f64_r,
        cudaDataType::CUDA_C_32F => rocsparse_datatype::rocsparse_datatype_f32_c,
        cudaDataType::CUDA_C_64F => rocsparse_datatype::rocsparse_datatype_f64_c,
        cudaDataType::CUDA_R_8I => rocsparse_datatype::rocsparse_datatype_i8_r,
        cudaDataType::CUDA_R_8U => rocsparse_datatype::rocsparse_datatype_u8_r,
        cudaDataType::CUDA_R_32I => rocsparse_datatype::rocsparse_datatype_i32_r,
        cudaDataType::CUDA_R_32U => rocsparse_datatype::rocsparse_datatype_u32_r,
        _ => panic!(),
    }
}

unsafe fn spmat_get_size(
    sp_mat_descr: cusparseSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    nnz: *mut i64,
) -> cusparseStatus_t {
    to_cuda(rocsparse_spmat_get_size(
        sp_mat_descr.cast(),
        rows,
        cols,
        nnz,
    ))
}

unsafe fn csr_set_pointers(
    sp_mat_descr: cusparseSpMatDescr_t,
    csr_row_offsets: *mut c_void,
    csr_col_ind: *mut c_void,
    csr_values: *mut c_void,
) -> cusparseStatus_t {
    to_cuda(rocsparse_csr_set_pointers(
        sp_mat_descr.cast(),
        csr_row_offsets,
        csr_col_ind,
        csr_values,
    ))
}

unsafe fn sparse_destroy(handle: *mut cusparseContext) -> cusparseStatus_t {
    to_cuda(rocsparse_destroy_handle(handle.cast()))
}

unsafe fn get_max_index_base(descr_a: *mut cusparseMatDescr) -> cusparseIndexBase_t {
    to_mat_index_base(rocsparse_get_mat_index_base(descr_a.cast()))
}

fn to_mat_index_base(index_base: rocsparse_index_base) -> cusparseIndexBase_t {
    match index_base {
        rocsparse_index_base::rocsparse_index_base_zero => {
            cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ZERO
        }
        rocsparse_index_base::rocsparse_index_base_one => {
            cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ONE
        }
        _ => panic!(),
    }
}

unsafe fn get_mat_type(descr_a: *mut cusparseMatDescr) -> cusparseMatrixType_t {
    to_mat_type(rocsparse_get_mat_type(descr_a.cast()))
}

fn to_mat_type(mat_type: rocsparse_matrix_type) -> cusparseMatrixType_t {
    match mat_type {
        rocsparse_matrix_type::rocsparse_matrix_type_general => {
            cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_GENERAL
        }
        rocsparse_matrix_type::rocsparse_matrix_type_symmetric => {
            cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_SYMMETRIC
        }
        rocsparse_matrix_type::rocsparse_matrix_type_hermitian => {
            cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_HERMITIAN
        }
        rocsparse_matrix_type::rocsparse_matrix_type_triangular => {
            cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_TRIANGULAR
        }
        _ => panic!(),
    }
}

unsafe fn csr2cscex2_buffersize(
    handle: *mut cusparseContext,
    m: i32,
    n: i32,
    nnz: i32,
    csr_val: *const c_void,
    csr_row_ptr: *const i32,
    csr_col_ind: *const i32,
    csc_val: *mut c_void,
    csc_col_ptr: *mut i32,
    csc_row_ind: *mut i32,
    val_type: cudaDataType,
    copy_values: cusparseAction_t,
    idx_base: cusparseIndexBase_t,
    alg: cusparseCsr2CscAlg_t,
    buffer_size: *mut usize,
) -> cusparseStatus_t {
    let copy_values = to_action(copy_values);
    to_cuda(rocsparse_csr2csc_buffer_size(
        handle.cast(),
        m,
        n,
        nnz,
        csr_row_ptr,
        csr_col_ind,
        copy_values,
        buffer_size,
    ))
}

fn to_action(copy_values: cusparseAction_t) -> rocsparse_action {
    match copy_values {
        cusparseAction_t::CUSPARSE_ACTION_SYMBOLIC => rocsparse_action::rocsparse_action_symbolic,
        cusparseAction_t::CUSPARSE_ACTION_NUMERIC => rocsparse_action::rocsparse_action_numeric,
        _ => panic!(),
    }
}

type rocsparse_csr2csc_generic = unsafe extern "C" fn(
    handle: rocsparse_handle,
    m: rocsparse_int,
    n: rocsparse_int,
    nnz: rocsparse_int,
    csr_val: *const c_void,
    csr_row_ptr: *const rocsparse_int,
    csr_col_ind: *const rocsparse_int,
    csc_val: *mut c_void,
    csc_row_ind: *mut rocsparse_int,
    csc_col_ptr: *mut rocsparse_int,
    copy_values: rocsparse_action,
    idx_base: rocsparse_index_base,
    temp_buffer: *mut ::std::os::raw::c_void,
) -> rocsparse_status;

unsafe fn csr2cscex2(
    handle: *mut cusparseContext,
    m: i32,
    n: i32,
    nnz: i32,
    csr_val: *const c_void,
    csr_row_ptr: *const i32,
    csr_col_ind: *const i32,
    csc_val: *mut c_void,
    csc_col_ptr: *mut i32,
    csc_row_ind: *mut i32,
    val_type: cudaDataType,
    copy_values: cusparseAction_t,
    idx_base: cusparseIndexBase_t,
    alg: cusparseCsr2CscAlg_t,
    buffer: *mut c_void,
) -> cusparseStatus_t {
    let rocsparse_csr2csc_generic = match val_type {
        cudaDataType::CUDA_R_32F => {
            mem::transmute::<_, rocsparse_csr2csc_generic>(rocsparse_scsr2csc as *const ())
        }
        cudaDataType::CUDA_R_64F => {
            mem::transmute::<_, rocsparse_csr2csc_generic>(rocsparse_dcsr2csc as *const ())
        }
        cudaDataType::CUDA_C_32F => {
            mem::transmute::<_, rocsparse_csr2csc_generic>(rocsparse_ccsr2csc as *const ())
        }
        cudaDataType::CUDA_C_64F => {
            mem::transmute::<_, rocsparse_csr2csc_generic>(rocsparse_zcsr2csc as *const ())
        }
        _ => panic!(),
    };
    let copy_values = to_action(copy_values);
    let idx_base = index_base(idx_base);
    to_cuda(rocsparse_csr2csc_generic(
        handle.cast(),
        m,
        n,
        nnz,
        csr_val,
        csr_row_ptr,
        csr_col_ind,
        csc_val,
        csc_row_ind,
        csc_col_ptr,
        copy_values,
        idx_base,
        buffer,
    ))
}

unsafe fn destory_dnvec(dn_vec_descr: *mut cusparseDnVecDescr) -> cusparseStatus_t {
    if dn_vec_descr == ptr::null_mut() {
        cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
    } else {
        to_cuda(rocsparse_destroy_dnvec_descr(dn_vec_descr.cast()))
    }
}

unsafe fn destory_csrilu02info(info: *mut csrilu02Info) -> cusparseStatus_t {
    if info == ptr::null_mut() {
        cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
    } else {
        to_cuda(rocsparse_destroy_mat_info(info.cast()))
    }
}

unsafe fn destroy_csric02info(info: *mut csric02Info) -> cusparseStatus_t {
    if info == ptr::null_mut() {
        cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
    } else {
        to_cuda(rocsparse_destroy_mat_info(info.cast()))
    }
}

unsafe fn spmat_set_attribute(
    sp_mat_descr: *mut cusparseSpMatDescr,
    attribute: cusparseSpMatAttribute_t,
    data: *mut c_void,
    data_size: usize,
) -> cusparseStatus_t {
    let attribute = to_attribute(attribute);
    // Both diag type and fill mode are compatible, no adjustment needed
    to_cuda(rocsparse_spmat_set_attribute(
        sp_mat_descr.cast(),
        attribute,
        data,
        data_size,
    ))
}

fn to_attribute(attribute: cusparseSpMatAttribute_t) -> rocsparse_spmat_attribute {
    match attribute {
        cusparseSpMatAttribute_t::CUSPARSE_SPMAT_DIAG_TYPE => {
            rocsparse_spmat_attribute::rocsparse_spmat_diag_type
        }
        cusparseSpMatAttribute_t::CUSPARSE_SPMAT_FILL_MODE => {
            rocsparse_spmat_attribute::rocsparse_spmat_fill_mode
        }
        _ => panic!(),
    }
}

unsafe fn create_csrilu02_info(info: *mut *mut csrilu02Info) -> cusparseStatus_t {
    to_cuda(rocsparse_create_mat_info(info.cast()))
}

struct SpSvDescr {
    external_buffer: *mut c_void,
}

unsafe fn spsv_create_descr(descr: *mut *mut cusparseSpSVDescr) -> cusparseStatus_t {
    *(descr.cast()) = Box::into_raw(Box::new(SpSvDescr {
        external_buffer: ptr::null_mut(),
    }));
    cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
}

unsafe fn spsv_drstroy_desc(descr: *mut cusparseSpSVDescr) -> cusparseStatus_t {
    if descr == ptr::null_mut() {
        cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
    } else {
        Box::from_raw(descr.cast::<SpSvDescr>());
        cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
    }
}

unsafe fn spsv_buffersize(
    handle: *mut cusparseContext,
    op_a: cusparseOperation_t,
    alpha: *const c_void,
    mat_a: *mut cusparseSpMatDescr,
    vec_x: cusparseDnVecDescr_t,
    vec_y: cusparseDnVecDescr_t,
    compute_type: cudaDataType,
    alg: cusparseSpSVAlg_t,
    spsv_descr: *mut cusparseSpSVDescr,
    buffer_size: *mut usize,
) -> cusparseStatus_t {
    let op_a = operation(op_a);
    let compute_type = data_type(compute_type);
    let alg = to_spsv_alg(alg);
    to_cuda(rocsparse_spsv(
        handle.cast(),
        op_a,
        alpha,
        mat_a.cast(),
        vec_x.cast(),
        vec_y.cast(),
        compute_type,
        alg,
        rocsparse_spsv_stage::rocsparse_spsv_stage_buffer_size,
        buffer_size,
        ptr::null_mut(),
    ))
}

fn to_spsv_alg(alg: cusparseSpSVAlg_t) -> rocsparse_spsv_alg {
    match alg {
        cusparseSpSVAlg_t::CUSPARSE_SPSV_ALG_DEFAULT => {
            rocsparse_spsv_alg::rocsparse_spsv_alg_default
        }
        _ => panic!(),
    }
}

unsafe fn spsv_analysis(
    handle: *mut cusparseContext,
    op_a: cusparseOperation_t,
    alpha: *const c_void,
    mat_a: *mut cusparseSpMatDescr,
    vec_x: *mut cusparseDnVecDescr,
    vec_y: *mut cusparseDnVecDescr,
    compute_type: cudaDataType,
    alg: cusparseSpSVAlg_t,
    spsv_descr: *mut cusparseSpSVDescr,
    external_buffer: *mut c_void,
) -> cusparseStatus_t {
    let op_a = operation(op_a);
    let compute_type = data_type(compute_type);
    let alg = to_spsv_alg(alg);
    let spsv_descr = spsv_descr.cast::<SpSvDescr>().as_mut().unwrap();
    spsv_descr.external_buffer = external_buffer;
    to_cuda(rocsparse_spsv(
        handle.cast(),
        op_a,
        alpha,
        mat_a.cast(),
        vec_x.cast(),
        vec_y.cast(),
        compute_type,
        alg,
        rocsparse_spsv_stage::rocsparse_spsv_stage_preprocess,
        ptr::null_mut(),
        external_buffer,
    ))
}

unsafe fn spsv_solve(
    handle: *mut cusparseContext,
    op_a: cusparseOperation_t,
    alpha: *const c_void,
    mat_a: *mut cusparseSpMatDescr,
    vec_x: *mut cusparseDnVecDescr,
    vec_y: *mut cusparseDnVecDescr,
    compute_type: cudaDataType,
    alg: cusparseSpSVAlg_t,
    spsv_descr: *mut cusparseSpSVDescr,
) -> cusparseStatus_t {
    let op_a = operation(op_a);
    let compute_type = data_type(compute_type);
    let alg = to_spsv_alg(alg);
    let spsv_descr = spsv_descr.cast::<SpSvDescr>().as_ref().unwrap();
    to_cuda(rocsparse_spsv(
        handle.cast(),
        op_a,
        alpha,
        mat_a.cast(),
        vec_x.cast(),
        vec_y.cast(),
        compute_type,
        alg,
        rocsparse_spsv_stage::rocsparse_spsv_stage_compute,
        ptr::null_mut(),
        spsv_descr.external_buffer,
    ))
}

unsafe fn dcsrilu02_buffersize(
    handle: *mut cusparseContext,
    m: i32,
    nnz: i32,
    descr_a: *mut cusparseMatDescr,
    csr_sorted_val_a: *mut f64,
    csr_sorted_row_ptr_a: *const i32,
    csr_sorted_col_ind_a: *const i32,
    info: *mut csrilu02Info,
    p_buffer_size_in_bytes: *mut i32,
) -> cusparseStatus_t {
    let mut buffer_size = 0;
    call! {rocsparse_dcsrilu0_buffer_size(
        handle.cast(),
        m,
        nnz,
        descr_a.cast(),
        csr_sorted_val_a,
        csr_sorted_row_ptr_a,
        csr_sorted_col_ind_a,
        info.cast(),
        &mut buffer_size,
    ) };
    if buffer_size > i32::MAX as usize {
        return cusparseStatus_t::CUSPARSE_STATUS_INTERNAL_ERROR;
    }
    *p_buffer_size_in_bytes = buffer_size as i32;
    cusparseStatus_t::CUSPARSE_STATUS_SUCCESS
}

unsafe fn dcsrilu02_analysis(
    handle: *mut cusparseContext,
    m: i32,
    nnz: i32,
    descr_a: *mut cusparseMatDescr,
    csr_sorted_val_a: *const f64,
    csr_sorted_row_ptr_a: *const i32,
    csr_sorted_col_ind_a: *const i32,
    info: *mut csrilu02Info,
    policy: cusparseSolvePolicy_t,
    p_buffer: *mut c_void,
) -> cusparseStatus_t {
    let (analysis, solve) = to_policy(policy);
    to_cuda(rocsparse_dcsrilu0_analysis(
        handle.cast(),
        m,
        nnz,
        descr_a.cast(),
        csr_sorted_val_a,
        csr_sorted_row_ptr_a,
        csr_sorted_col_ind_a,
        info.cast(),
        analysis,
        solve,
        p_buffer,
    ))
}

unsafe fn xcsrilu02_zeropivot(
    handle: *mut cusparseContext,
    info: *mut csrilu02Info,
    position: *mut i32,
) -> cusparseStatus_t {
    to_cuda(rocsparse_csrilu0_zero_pivot(
        handle.cast(),
        info.cast(),
        position,
    ))
}

unsafe fn dcsrilu02(
    handle: *mut cusparseContext,
    m: i32,
    nnz: i32,
    descr_a: *mut cusparseMatDescr,
    csr_sorted_val_a_val_m: *mut f64,
    csr_sorted_row_ptr_a: *const i32,
    csr_sorted_col_ind_a: *const i32,
    info: *mut csrilu02Info,
    policy: cusparseSolvePolicy_t,
    p_buffer: *mut c_void,
) -> cusparseStatus_t {
    let (analysis, solve) = to_policy(policy);
    to_cuda(rocsparse_dcsrilu0(
        handle.cast(),
        m,
        nnz,
        descr_a.cast(),
        csr_sorted_val_a_val_m,
        csr_sorted_row_ptr_a,
        csr_sorted_col_ind_a,
        info.cast(),
        solve,
        p_buffer,
    ))
}
