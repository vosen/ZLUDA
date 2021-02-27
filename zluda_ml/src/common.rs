use std::{
    ffi::{c_char, c_uint, CStr},
    ptr,
};

use crate::nvml::*;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED
}

macro_rules! stringify_nmvlreturn_t {
    ($x:ident => [ $($variant:ident),+ ]) => {
        match $x {
            $(
                nvmlReturn_t::$variant => Some(concat!(stringify!($variant), "\0")),
            )+
            _ => None
        }
    }
}

pub(crate) fn error_string(result: nvmlReturn_t) -> *const ::std::os::raw::c_char {
    let text = stringify_nmvlreturn_t!(
        result => [
            NVML_SUCCESS,
            NVML_ERROR_UNINITIALIZED,
            NVML_ERROR_INVALID_ARGUMENT,
            NVML_ERROR_NOT_SUPPORTED,
            NVML_ERROR_NO_PERMISSION,
            NVML_ERROR_ALREADY_INITIALIZED,
            NVML_ERROR_NOT_FOUND,
            NVML_ERROR_INSUFFICIENT_SIZE,
            NVML_ERROR_INSUFFICIENT_POWER,
            NVML_ERROR_DRIVER_NOT_LOADED,
            NVML_ERROR_TIMEOUT,
            NVML_ERROR_IRQ_ISSUE,
            NVML_ERROR_LIBRARY_NOT_FOUND,
            NVML_ERROR_FUNCTION_NOT_FOUND,
            NVML_ERROR_CORRUPTED_INFOROM,
            NVML_ERROR_GPU_IS_LOST,
            NVML_ERROR_RESET_REQUIRED,
            NVML_ERROR_OPERATING_SYSTEM,
            NVML_ERROR_LIB_RM_VERSION_MISMATCH,
            NVML_ERROR_IN_USE,
            NVML_ERROR_MEMORY,
            NVML_ERROR_NO_DATA,
            NVML_ERROR_VGPU_ECC_NOT_SUPPORTED,
            NVML_ERROR_INSUFFICIENT_RESOURCES,
            NVML_ERROR_UNKNOWN
        ]
    );
    match text {
        Some(text) => text.as_ptr() as *const _,
        None => ptr::null(),
    }
}

// For version we report NVIDIA's minimum required driver version for CUDA
// version we want to support incremented by 00.00.01:
// https://docs.nvidia.com/deploy/cuda-compatibility/index.html#minor-version-compatibility
const VERSION: &'static [u8] = b"511.09.01"; // minimum required by Arnold
const NVML_VERSION: &'static [u8] = b"11.511.09.01"; // // minimum required by Arnold

impl From<Result<(), nvmlReturn_t>> for nvmlReturn_t {
    fn from(result: Result<(), nvmlReturn_t>) -> Self {
        match result {
            Ok(()) => nvmlReturn_t::NVML_SUCCESS,
            Err(e) => e,
        }
    }
}

impl From<nvmlReturn_t> for Result<(), nvmlReturn_t> {
    fn from(result: nvmlReturn_t) -> Self {
        match result {
            nvmlReturn_t::NVML_SUCCESS => Ok(()),
            e => Err(e),
        }
    }
}

pub(crate) unsafe fn system_get_cuda_driver_version(
    cuda_driver_version: *mut ::std::os::raw::c_int,
) -> Result<(), nvmlReturn_t> {
    *cuda_driver_version = 11000;
    Ok(())
}

pub(crate) unsafe fn system_get_driver_version(
    version_ptr: *mut c_char,
    length: c_uint,
) -> Result<(), nvmlReturn_t> {
    if version_ptr == ptr::null_mut() || length == 0 {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let strlen = usize::min(VERSION.len(), (length as usize) - 1);
    std::ptr::copy_nonoverlapping(VERSION.as_ptr(), version_ptr as _, strlen);
    *version_ptr.add(strlen) = 0;
    Ok(())
}

pub(crate) unsafe fn system_get_nvml_version(
    version_ptr: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> Result<(), nvmlReturn_t> {
    if version_ptr == ptr::null_mut() || length == 0 {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let strlen = usize::min(NVML_VERSION.len(), (length as usize) - 1);
    std::ptr::copy_nonoverlapping(NVML_VERSION.as_ptr(), version_ptr as _, strlen);
    *version_ptr.add(strlen) = 0;
    Ok(())
}

pub(crate) fn device_get_temperature_threshold(
    _device: *mut crate::nvml::nvmlDevice_st,
    _threshold_type: crate::nvml::nvmlTemperatureThresholds_t,
    _temp: *mut u32,
) -> nvmlReturn_t {
    nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED
}

pub(crate) unsafe fn device_get_nvlink_state(
    _device: *mut crate::nvml::nvmlDevice_st,
    _link: u32,
    _is_active: *mut crate::nvml::nvmlEnableState_enum,
) -> nvmlReturn_t {
    return nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED;
}

pub(crate)  unsafe fn parse_pci_address(address: *const i8) -> Result<(i32, i32, i32), nvmlReturn_t> {
    let address = CStr::from_ptr(address);
    let address = address
        .to_str()
        .map_err(|_| nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT)?;
    let mut split_addr = address.split(':').rev();
    let device_function = split_addr
        .next()
        .ok_or(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT)?;
    let (device, function) = device_function
        .split_once('.')
        .ok_or(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT)?;
    let bus = split_addr
        .next()
        .ok_or(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT)?;
    let device =
        i32::from_str_radix(device, 16).map_err(|_| nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT)?;
    let function =
        i32::from_str_radix(function, 16).map_err(|_| nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT)?;
    let bus =
        i32::from_str_radix(bus, 16).map_err(|_| nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT)?;
    Ok((bus, device, function))
}
