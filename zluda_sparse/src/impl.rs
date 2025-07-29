use cuda_types::cusparse::*;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cusparseStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cusparseStatus_t {
    cusparseStatus_t::ERROR_NOT_SUPPORTED
}

#[allow(non_snake_case)]
pub(crate) fn cusparseGetErrorName(
    _status: cuda_types::cusparse::cusparseStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cusparseGetErrorString(
    _status: cuda_types::cusparse::cusparseStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cusparseGetMatType(
    _descrA: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseMatrixType_t {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cusparseGetMatFillMode(
    _descrA: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseFillMode_t {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cusparseGetMatDiagType(
    _descrA: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseDiagType_t {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cusparseGetMatIndexBase(
    _descrA: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseIndexBase_t {
    todo!()
}
