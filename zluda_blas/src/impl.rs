use cuda_types::cublas::cublasStatus_t;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cublasStatus_t {
    CUresult::ERROR_NOT_SUPPORTED
}

pub fn cublasGetStatusName(status: cuda_types::cublas::cublasStatus_t) -> *const ::core::ffi::c_char {
    todo!()
}
pub fn cublasGetStatusString(status: cuda_types::cublas::cublasStatus_t) -> *const ::core::ffi::c_char {
    todo!()
}
pub fn cublasXerbla(srName: *const ::core::ffi::c_char, info: ::core::ffi::c_int) -> () {
    todo!()
}
pub fn cublasGetCudartVersion() -> usize {
    todo!()
}
