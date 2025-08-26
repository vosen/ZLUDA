use cuda_types::nvml::*;
use rocm_smi_sys::*;
use std::mem;
use zluda_common::{from_cuda_object, ZludaObject};

pub(crate) use crate::impl_common::error_string;
pub(crate) use crate::impl_common::system_get_driver_version;

pub(crate) struct Device {
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
