use cuda_types::cublas::*;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cublasStatus_t {
    cublasStatus_t::ERROR_NOT_SUPPORTED
}

#[allow(non_snake_case)]
pub(crate) fn cublasLtGetStatusName(
    _status: cuda_types::cublas::cublasStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn cublasLtGetStatusString(
    _status: cuda_types::cublas::cublasStatus_t,
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
