use cuda_types::cusparse::*;
use rocsparse_sys::*;
use std::{ptr, sync::OnceLock};
use unwrap_or::unwrap_ok_or;

fn rocsparse() -> Result<&'static super::RocsparseVtable, rocsparse_error> {
    static LOCK: OnceLock<Result<super::RocsparseVtable, rocsparse_error>> = OnceLock::new();
    let unwrapped: &Result<super::RocsparseVtable, rocsparse_error> =
        LOCK.get_or_init(|| unsafe { super::RocsparseVtable::new() });
    unwrapped.as_ref().map_err(|x| *x)
}

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cusparseStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cusparseStatus_t {
    cusparseStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) unsafe fn get_error_name(
    status: cuda_types::cusparse::cusparseStatus_t,
) -> *const ::core::ffi::c_char {
    match status {
        Ok(()) => c"CUSPARSE_STATUS_SUCCESS".as_ptr(),
        Err(cusparseError_t::NOT_INITIALIZED) => c"CUSPARSE_STATUS_NOT_INITIALIZED".as_ptr(),
        Err(cusparseError_t::ALLOC_FAILED) => c"CUSPARSE_STATUS_ALLOC_FAILED".as_ptr(),
        Err(cusparseError_t::INVALID_VALUE) => c"CUSPARSE_STATUS_INVALID_VALUE".as_ptr(),
        Err(cusparseError_t::ARCH_MISMATCH) => c"CUSPARSE_STATUS_ARCH_MISMATCH".as_ptr(),
        Err(cusparseError_t::MAPPING_ERROR) => c"CUSPARSE_STATUS_MAPPING_ERROR".as_ptr(),
        Err(cusparseError_t::EXECUTION_FAILED) => c"CUSPARSE_STATUS_EXECUTION_FAILED".as_ptr(),
        Err(cusparseError_t::INTERNAL_ERROR) => c"CUSPARSE_STATUS_INTERNAL_ERROR".as_ptr(),
        Err(cusparseError_t::MATRIX_TYPE_NOT_SUPPORTED) => {
            c"CUSPARSE_STATUS_MATRIX_TYPE_NOT_SUPPORTED".as_ptr()
        }
        Err(cusparseError_t::ZERO_PIVOT) => c"CUSPARSE_STATUS_ZERO_PIVOT".as_ptr(),
        Err(cusparseError_t::NOT_SUPPORTED) => c"CUSPARSE_STATUS_NOT_SUPPORTED".as_ptr(),
        Err(cusparseError_t::INSUFFICIENT_RESOURCES) => {
            c"CUSPARSE_STATUS_INSUFFICIENT_RESOURCES".as_ptr()
        }
        Err(_) => c"CUSPARSE_STATUS_INTERNAL_ERROR".as_ptr(),
    }
}

pub(crate) unsafe fn get_error_string(
    status: cuda_types::cusparse::cusparseStatus_t,
) -> *const ::core::ffi::c_char {
    if cfg!(windows) && status.is_err() && status.err() == rocsparse().err().map(Into::into) {
        return c"rocsparse.dll could not be found. Please install HIP SDK: https://zluda.readthedocs.io/latest/hip_sdk.html".as_ptr();
    }
    get_error_name(status)
}

pub(crate) unsafe fn get_mat_index_base(descr_a: rocsparse_mat_descr) -> rocsparse_index_base {
    let rocsparse = unwrap_ok_or!(
        rocsparse(),
        _,
        return rocsparse_index_base::rocsparse_index_base_zero
    );
    rocsparse.rocsparse_get_mat_index_base(descr_a)
}

pub(crate) unsafe fn get_mat_type(descr: rocsparse_mat_descr) -> rocsparse_matrix_type {
    let rocsparse = unwrap_ok_or!(
        rocsparse(),
        _,
        return rocsparse_matrix_type::rocsparse_matrix_type_general
    );
    rocsparse.rocsparse_get_mat_type(descr)
}

pub(crate) unsafe fn get_mat_fill_mode(descr: rocsparse_mat_descr) -> rocsparse_fill_mode {
    let rocsparse = unwrap_ok_or!(
        rocsparse(),
        _,
        return rocsparse_fill_mode::rocsparse_fill_mode_lower
    );
    rocsparse.rocsparse_get_mat_fill_mode(descr)
}

pub(crate) unsafe fn get_mat_diag_type(descr: rocsparse_mat_descr) -> rocsparse_diag_type {
    let rocsparse = unwrap_ok_or!(
        rocsparse(),
        _,
        return rocsparse_diag_type::rocsparse_diag_type_non_unit
    );
    rocsparse.rocsparse_get_mat_diag_type(descr)
}

pub(crate) unsafe fn create(context: *mut rocsparse_handle) -> rocsparse_status {
    rocsparse()?.rocsparse_create_handle(context)
}

pub(crate) unsafe fn create_const_coo(
    descr: *mut rocsparse_const_spmat_descr,
    rows: i64,
    cols: i64,
    nnz: i64,
    coo_row_ind: *const ::core::ffi::c_void,
    coo_col_ind: *const ::core::ffi::c_void,
    coo_values: *const ::core::ffi::c_void,
    idx_type: rocsparse_indextype,
    idx_base: rocsparse_index_base,
    data_type: rocsparse_datatype,
) -> rocsparse_status {
    rocsparse()?.rocsparse_create_const_coo_descr(
        descr,
        rows,
        cols,
        nnz,
        coo_row_ind as *const _,
        coo_col_ind as *const _,
        coo_values as *const _,
        idx_type,
        idx_base,
        data_type,
    )
}

pub(crate) unsafe fn create_const_dn_mat(
    descr: *mut rocsparse_const_dnmat_descr,
    rows: i64,
    cols: i64,
    ld: i64,
    values: *const ::core::ffi::c_void,
    data_type: rocsparse_datatype,
    order: rocsparse_order,
) -> rocsparse_status {
    rocsparse()?.rocsparse_create_const_dnmat_descr(
        descr,
        rows,
        cols,
        ld,
        values as *const _,
        data_type,
        order,
    )
}

pub(crate) unsafe fn create_dn_mat(
    descr: *mut rocsparse_dnmat_descr,
    rows: i64,
    cols: i64,
    ld: i64,
    values: *mut ::core::ffi::c_void,
    data_type: rocsparse_datatype,
    order: rocsparse_order,
) -> rocsparse_status {
    rocsparse()?.rocsparse_create_dnmat_descr(
        descr,
        rows,
        cols,
        ld,
        values as *mut _,
        data_type,
        order,
    )
}

pub(crate) unsafe fn destroy(context: rocsparse_handle) -> rocsparse_status {
    rocsparse()?.rocsparse_destroy_handle(context)
}

pub(crate) unsafe fn destroy_dn_mat(descr: rocsparse_const_dnmat_descr) -> rocsparse_status {
    rocsparse()?.rocsparse_destroy_dnmat_descr(descr)
}

pub(crate) unsafe fn destroy_sp_mat(descr: rocsparse_const_spmat_descr) -> rocsparse_status {
    rocsparse()?.rocsparse_destroy_spmat_descr(descr)
}

pub(crate) unsafe fn sp_m_m_buffer_size(
    handle: rocsparse_handle,
    op_a: rocsparse_operation,
    op_b: rocsparse_operation,
    alpha: *const ::core::ffi::c_void,
    mat_a: rocsparse_const_spmat_descr,
    mat_b: rocsparse_const_dnmat_descr,
    beta: *const ::core::ffi::c_void,
    mat_c: rocsparse_dnmat_descr,
    compute_type: rocsparse_datatype,
    alg: rocsparse_spmm_alg,
    buffer_size: *mut usize,
) -> rocsparse_status {
    rocsparse()?.rocsparse_spmm(
        handle,
        op_a,
        op_b,
        alpha,
        mat_a,
        mat_b,
        beta,
        mat_c,
        compute_type,
        alg,
        rocsparse_spmm_stage::rocsparse_spmm_stage_buffer_size,
        buffer_size,
        ptr::null_mut(),
    )
}

pub(crate) unsafe fn sp_m_m_preprocess(
    handle: rocsparse_handle,
    op_a: rocsparse_operation,
    op_b: rocsparse_operation,
    alpha: *const ::core::ffi::c_void,
    mat_a: rocsparse_const_spmat_descr,
    mat_b: rocsparse_const_dnmat_descr,
    beta: *const ::core::ffi::c_void,
    mat_c: rocsparse_dnmat_descr,
    compute_type: rocsparse_datatype,
    alg: rocsparse_spmm_alg,
    external_buffer: *mut ::core::ffi::c_void,
) -> rocsparse_status {
    rocsparse()?.rocsparse_spmm(
        handle,
        op_a,
        op_b,
        alpha,
        mat_a,
        mat_b,
        beta,
        mat_c,
        compute_type,
        alg,
        rocsparse_spmm_stage::rocsparse_spmm_stage_preprocess,
        &mut 0,
        external_buffer,
    )
}

pub(crate) unsafe fn sp_m_m(
    handle: rocsparse_handle,
    op_a: rocsparse_operation,
    op_b: rocsparse_operation,
    alpha: *const ::core::ffi::c_void,
    mat_a: rocsparse_const_spmat_descr,
    mat_b: rocsparse_const_dnmat_descr,
    beta: *const ::core::ffi::c_void,
    mat_c: rocsparse_dnmat_descr,
    compute_type: rocsparse_datatype,
    alg: rocsparse_spmm_alg,
    external_buffer: *mut ::core::ffi::c_void,
) -> rocsparse_status {
    rocsparse()?.rocsparse_spmm(
        handle,
        op_a,
        op_b,
        alpha,
        mat_a,
        mat_b,
        beta,
        mat_c,
        compute_type,
        alg,
        rocsparse_spmm_stage::rocsparse_spmm_stage_compute,
        &mut 0,
        external_buffer,
    )
}
