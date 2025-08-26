use cuda_types::nvml::*;
use rocm_smi_sys::*;
use std::{ffi::CStr, mem, ptr};
use zluda_common::{from_cuda_object, ZludaObject};

const VERSION: &'static CStr = c"550.77";

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    nvmlReturn_t::ERROR_NOT_SUPPORTED
}

pub struct Device {
    _index: u32,
}

impl ZludaObject for Device {
    const COOKIE: usize = 0x79443851e7cee0d9;

    type Error = nvmlError_t;
    type CudaHandle = nvmlDevice_t;

    fn drop_checked(&mut self) -> nvmlReturn_t {
        Ok(())
    }
}

from_cuda_object!(Device);

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

pub(crate) fn error_string(_result: nvmlReturn_t) -> *const ::core::ffi::c_char {
    c"".as_ptr()
}

pub(crate) unsafe fn init() -> rsmi_status_t {
    rsmi_init(0)
}

pub(crate) unsafe fn init_v2() -> rsmi_status_t {
    rsmi_init(0)
}

pub(crate) unsafe fn init_with_flags(_flags: ::core::ffi::c_uint) -> rsmi_status_t {
    rsmi_init(0)
}

pub(crate) unsafe fn shutdown() -> rsmi_status_t {
    rsmi_shut_down()
}

pub(crate) unsafe fn device_get_count_v2(device_count: &mut ::core::ffi::c_uint) -> rsmi_status_t {
    rsmi_num_monitor_devices(device_count)
}

pub(crate) unsafe fn device_get_field_values(
    _device: &Device,
    values_count: ::core::ffi::c_int,
    values: &mut cuda_types::nvml::nvmlFieldValue_t,
) -> nvmlReturn_t {
    for field in std::slice::from_raw_parts_mut(values, values_count as usize) {
        get_field_value(field)?;
    }
    Ok(())
}

unsafe fn get_field_value(field: &mut nvmlFieldValue_st) -> Result<(), nvmlError_t> {
    *field = mem::zeroed();
    field.nvmlReturn = nvmlReturn_t::ERROR_NOT_SUPPORTED;
    Ok(())
}

pub(crate) unsafe fn device_get_gpu_fabric_info(
    _device: &Device,
    gpu_fabric_info: &mut cuda_types::nvml::nvmlGpuFabricInfo_t,
) -> nvmlReturn_t {
    *gpu_fabric_info = mem::zeroed();
    Ok(())
}

pub(crate) fn device_get_handle_by_index_v2(
    index: ::core::ffi::c_uint,
    device: &mut cuda_types::nvml::nvmlDevice_t,
) -> nvmlReturn_t {
    *device = Device { _index: index }.wrap();
    nvmlReturn_t::SUCCESS
}
