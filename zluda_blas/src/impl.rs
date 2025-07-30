use cuda_types::cublas::*;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cublasStatus_t {
    cublasStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn get_status_name(_status: Option<cublasStatus_t>) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_status_string(_status: Option<cublasStatus_t>) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn xerbla(
    _sr_name: Option<*const ::core::ffi::c_char>,
    _info: Option<::core::ffi::c_int>,
) -> () {
    todo!()
}

pub(crate) fn get_cudart_version() -> usize {
    todo!()
}
