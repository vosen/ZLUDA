use cuda_types::cusparse::*;
use rocsparse_sys::*;
use std::sync::OnceLock;

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

pub(crate) fn get_error_name(
    _status: cuda_types::cusparse::cusparseStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_error_string(
    _status: cuda_types::cusparse::cusparseStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_mat_type(
    _descr_a: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseMatrixType_t {
    todo!()
}

pub(crate) fn get_mat_fill_mode(
    _descr_a: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseFillMode_t {
    todo!()
}

pub(crate) fn get_mat_diag_type(
    _descr_a: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseDiagType_t {
    todo!()
}

pub(crate) fn get_mat_index_base(
    _descr_a: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseIndexBase_t {
    todo!()
}

pub(crate) unsafe fn create(context: *mut rocsparse_handle) -> rocsparse_status {
    rocsparse()?.rocsparse_create_handle(context)
}

pub(crate) unsafe fn destroy(context: rocsparse_handle) -> rocsparse_status {
    rocsparse()?.rocsparse_destroy_handle(context)
}
