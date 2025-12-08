use cuda_types::nvml::*;

pub(crate) use crate::impl_common::error_string;
pub(crate) use crate::impl_common::system_get_driver_version;

pub(crate) unsafe fn init() -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) unsafe fn init_v2() -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) unsafe fn init_with_flags(_flags: ::core::ffi::c_uint) -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) unsafe fn shutdown() -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) unsafe fn device_get_count_v2(_device_count: &mut ::core::ffi::c_uint) -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) unsafe fn device_get_handle_by_pci_bus_id_v2(
    pci_bus_id: &std::ffi::CStr,
    device: &mut cuda_types::nvml::nvmlDevice_t,
) -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) unsafe fn device_get_field_values(
    _device: cuda_types::nvml::nvmlDevice_t,
    _values_count: ::core::ffi::c_int,
    _values: &mut cuda_types::nvml::nvmlFieldValue_t,
) -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) unsafe fn device_get_gpu_fabric_info(
    _device: cuda_types::nvml::nvmlDevice_t,
    _gpu_fabric_info: &mut cuda_types::nvml::nvmlGpuFabricInfo_t,
) -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) fn device_get_handle_by_index_v2(
    _index: ::core::ffi::c_uint,
    _device: &mut cuda_types::nvml::nvmlDevice_t,
) -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) fn device_get_compute_running_processes(
    _device: cuda_types::nvml::nvmlDevice_t,
    info_count: &mut ::core::ffi::c_uint,
    _infos: Option<&mut cuda_types::nvml::nvmlProcessInfo_v1_t>,
) -> nvmlReturn_t {
    *info_count = 0;
    Ok(())
}

pub(crate) unsafe fn device_get_memory_info(
    device: cuda_types::nvml::nvmlDevice_t,
    memory: &mut cuda_types::nvml::nvmlMemory_t,
) -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}

pub(crate) fn device_get_name(
    device: cuda_types::nvml::nvmlDevice_t,
    name: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> nvmlReturn_t {
    crate::impl_common::unimplemented()
}
