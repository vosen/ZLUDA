use cuda_types::cublaslt::cublasStatus_t;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cublasStatus_t {
    cublasStatus_t::CUBLAS_STATUS_NOT_SUPPORTED
}

#[allow(non_snake_case)]
pub(crate) fn cublasLtGetStatusName(
    _status: cuda_types::cublaslt::cublasStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cublasLtGetStatusString(
    _status: cuda_types::cublaslt::cublasStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cublasLtGetVersion() -> usize {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cublasLtGetCudartVersion() -> usize {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cublasLtDisableCpuInstructionsSetMask(
    _mask: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    todo!()
}
