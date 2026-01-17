use cuda_types::nvml::*;
use std::{ffi::CStr, ptr};
use zluda_common::constants::{COMPUTE_CAPABILITY_MAJOR, COMPUTE_CAPABILITY_MINOR};

const VERSION: &'static CStr = c"550.77";

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    nvmlReturn_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn system_get_driver_version(
    result: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> nvmlReturn_t {
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

pub(crate) unsafe fn device_get_cuda_compute_capability(
    _device: cuda_types::nvml::nvmlDevice_t,
    major: *mut ::core::ffi::c_int,
    minor: *mut ::core::ffi::c_int,
) -> nvmlReturn_t {
    // ZLUDA emulates a specific device so ignore the device parameter.
    *major = COMPUTE_CAPABILITY_MAJOR;
    *minor = COMPUTE_CAPABILITY_MINOR;
    Ok(())
}

pub(crate) fn error_string(_result: nvmlReturn_t) -> *const ::core::ffi::c_char {
    c"".as_ptr()
}
