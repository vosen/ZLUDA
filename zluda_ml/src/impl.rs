use cuda_types::nvml::*;
use std::{ffi::CStr, ptr};

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED
}

#[allow(non_snake_case)]
pub(crate) fn nvmlErrorString(
    _result: cuda_types::nvml::nvmlReturn_t,
) -> *const ::core::ffi::c_char {
    c"".as_ptr()
}

#[allow(non_snake_case)]
pub(crate) fn nvmlInit_v2() -> cuda_types::nvml::nvmlReturn_t {
    nvmlReturn_t::SUCCESS
}

const VERSION: &'static CStr = c"550.77";

#[allow(non_snake_case)]
pub(crate) fn nvmlSystemGetDriverVersion(
    result: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> cuda_types::nvml::nvmlReturn_t {
    if result == ptr::null_mut() {
        return nvmlReturn_t::ERROR_INVALID_ARGUMENT;
    }
    let version = VERSION.to_bytes_with_nul();
    let copy_length = usize::min(length as usize, version.len());
    let slice = unsafe { std::slice::from_raw_parts_mut(result.cast(), copy_length) };
    slice.copy_from_slice(&version[..copy_length]);
    if let Some(null) = slice.last_mut() {
        *null = 0;
    }
    nvmlReturn_t::SUCCESS
}
