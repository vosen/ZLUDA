use cuda_types::nvml::nvmlReturn_t;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED
}

pub(crate) fn nvmlErrorString(
    _result: cuda_types::nvml::nvmlReturn_t,
) -> *const ::core::ffi::c_char {
    c"".as_ptr()
}
