use cuda_types::cublas::*;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cublasStatus_t {
    cublasStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn get_status_name(
    _status: cuda_types::cublas::cublasStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_status_string(
    _status: cuda_types::cublas::cublasStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_version() -> usize {
    todo!()
}

pub(crate) fn get_cudart_version() -> usize {
    todo!()
}

#[allow(non_snake_case)]
pub(crate) fn disable_cpu_instructions_set_mask(_mask: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    todo!()
}
