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
pub fn cublasGetStatusName(
    _status: cuda_types::cublas::cublasStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

#[allow(non_snake_case)]
pub fn cublasGetStatusString(
    _status: cuda_types::cublas::cublasStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

#[allow(non_snake_case)]
pub fn cublasXerbla(_srName: *const ::core::ffi::c_char, _info: ::core::ffi::c_int) -> () {
    todo!()
}

#[allow(non_snake_case)]
pub fn cublasGetCudartVersion() -> usize {
    todo!()
}
