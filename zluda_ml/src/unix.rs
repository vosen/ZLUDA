use crate::{common, nvml::*};
use rocm_smi_sys::*;
use std::{
    ptr,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

macro_rules! smi_call {
    ($x:expr) => {{
        let result = $x;
        if result != rsmi_status_t::RSMI_STATUS_SUCCESS {
            return Err(result.into());
        }
    }};
}

impl From<rsmi_status_t> for nvmlReturn_t {
    fn from(error: rsmi_status_t) -> Self {
        match error {
            rsmi_status_t::RSMI_STATUS_SUCCESS => nvmlReturn_t::NVML_SUCCESS,
            rsmi_status_t::RSMI_STATUS_INVALID_ARGS => nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT,
            rsmi_status_t::RSMI_STATUS_NOT_SUPPORTED => nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED,
            rsmi_status_t::RSMI_STATUS_FILE_ERROR => nvmlReturn_t::NVML_ERROR_UNKNOWN,
            rsmi_status_t::RSMI_STATUS_PERMISSION => nvmlReturn_t::NVML_ERROR_NO_PERMISSION,
            rsmi_status_t::RSMI_STATUS_OUT_OF_RESOURCES => {
                nvmlReturn_t::NVML_ERROR_INSUFFICIENT_RESOURCES
            }
            rsmi_status_t::RSMI_STATUS_INTERNAL_EXCEPTION => nvmlReturn_t::NVML_ERROR_UNKNOWN,
            rsmi_status_t::RSMI_STATUS_INPUT_OUT_OF_BOUNDS => {
                nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT
            }
            rsmi_status_t::RSMI_STATUS_INIT_ERROR => nvmlReturn_t::NVML_ERROR_UNKNOWN,
            rsmi_status_t::RSMI_STATUS_NOT_YET_IMPLEMENTED => {
                nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED
            }
            rsmi_status_t::RSMI_STATUS_NOT_FOUND => nvmlReturn_t::NVML_ERROR_NOT_FOUND,
            rsmi_status_t::RSMI_STATUS_INSUFFICIENT_SIZE => {
                nvmlReturn_t::NVML_ERROR_INSUFFICIENT_SIZE
            }
            rsmi_status_t::RSMI_STATUS_INTERRUPT => nvmlReturn_t::NVML_ERROR_UNKNOWN,
            rsmi_status_t::RSMI_STATUS_UNEXPECTED_SIZE => nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT,
            rsmi_status_t::RSMI_STATUS_NO_DATA => nvmlReturn_t::NVML_ERROR_UNKNOWN,
            rsmi_status_t::RSMI_STATUS_UNEXPECTED_DATA => nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT,
            rsmi_status_t::RSMI_STATUS_BUSY => nvmlReturn_t::NVML_ERROR_UNKNOWN,
            rsmi_status_t::RSMI_STATUS_REFCOUNT_OVERFLOW => nvmlReturn_t::NVML_ERROR_UNKNOWN,
            rsmi_status_t::RSMI_STATUS_UNKNOWN_ERROR => nvmlReturn_t::NVML_ERROR_UNKNOWN,
            _ => nvmlReturn_t::NVML_ERROR_UNKNOWN,
        }
    }
}

pub(crate) unsafe fn init() -> rsmi_status_t {
    rsmi_init(0)
}

pub(crate) unsafe fn init_with_flags(_flags: ::std::os::raw::c_uint) -> rsmi_status_t {
    init()
}

pub(crate) unsafe fn shutdown() -> nvmlReturn_t {
    rsmi_shut_down().into()
}

pub(crate) unsafe fn device_get_handle_by_index(
    index: ::std::os::raw::c_uint,
    device: *mut nvmlDevice_t,
) -> Result<(), nvmlReturn_t> {
    if device == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let mut num_devices = 0;
    smi_call! {rsmi_num_monitor_devices(&mut num_devices)};
    if index >= num_devices {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    *device = (index + 1) as nvmlDevice_t;
    Ok(())
}

pub(crate) unsafe fn device_get_handle_by_pci_bus_id(
    pci_bus_id: *const ::std::os::raw::c_char,
    device: *mut nvmlDevice_t,
) -> Result<(), nvmlReturn_t> {
    if pci_bus_id == ptr::null_mut() || device == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let device = device as *mut u32;
    let mut devices = 0;
    smi_call! {rsmi_num_monitor_devices(&mut devices)};
    let (pci_bus, pci_device, pci_function) = common::parse_pci_address(pci_bus_id)?;
    for dev in 0..devices {
        let mut packed_bdfid = 0;
        smi_call! { rsmi_dev_pci_id_get(dev, &mut packed_bdfid) };
        let dev_function = (packed_bdfid & 0x7) as i32;
        let dev_device = ((packed_bdfid >> 3) & 0x1f) as i32;
        let dev_bus = ((packed_bdfid >> 8) & 0xff) as i32;
        if dev_function == pci_function && dev_device == pci_device && dev_bus == pci_bus {
            *device = dev + 1;
            return Ok(());
        }
    }
    Err(nvmlReturn_t::NVML_ERROR_NOT_FOUND)
}

pub(crate) unsafe fn device_get_fan_speed(
    _device: nvmlDevice_t,
    _speed: *mut u32,
) -> Result<(), nvmlReturn_t> {
    Err(crate::common::unimplemented())
}

pub(crate) unsafe fn device_get_memory_info(
    device: nvmlDevice_t,
    memory: *mut nvmlMemory_t,
) -> Result<(), nvmlReturn_t> {
    if memory == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let device = device as u32 - 1;
    let mut total = 0;
    smi_call! { rsmi_dev_memory_total_get(device, rsmi_memory_type_t::RSMI_MEM_TYPE_VRAM, &mut total) };
    let mut used = 0;
    smi_call! { rsmi_dev_memory_usage_get(device, rsmi_memory_type_t::RSMI_MEM_TYPE_VRAM, &mut used) };
    *memory = nvmlMemory_t {
        total,
        used,
        free: total - used,
    };
    Ok(())
}

pub(crate) unsafe fn device_get_pci_info(
    _device: nvmlDevice_t,
    _pci: *mut nvmlPciInfo_t,
) -> Result<(), nvmlReturn_t> {
    Err(crate::common::unimplemented())
}

pub(crate) unsafe fn device_get_temperature(
    _device: nvmlDevice_t,
    _sensor_type: nvmlTemperatureSensors_t,
    _temp: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    crate::common::unimplemented()
}

pub(crate) unsafe fn device_get_utilization_rates(
    _device: nvmlDevice_t,
    _utilization: *mut nvmlUtilization_t,
) -> Result<(), nvmlReturn_t> {
    Err(crate::common::unimplemented())
}

pub(crate) unsafe fn device_get_field_values(
    device: *mut nvmlDevice_st,
    values_count: i32,
    values: *mut nvmlFieldValue_st,
) -> Result<(), nvmlReturn_t> {
    if device == ptr::null_mut() || values == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let device = ((device as usize) - 1) as u32;
    for value in 0..values_count as usize {
        get_field_value(device, values.add(value))?;
    }
    Ok(())
}

unsafe fn get_field_value(device: u32, values: *mut nvmlFieldValue_st) -> Result<(), nvmlReturn_t> {
    let values = &mut *values;
    if values.fieldId != NVML_FI_DEV_NVLINK_LINK_COUNT {
        return Err(nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED);
    }
    let start = Instant::now();
    let xgmi_links = total_xgmi_links(device)?;
    let end = Instant::now();
    let latency = end.duration_since(start).as_micros();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    values.latencyUsec = latency as i64;
    // TODO: what is expected here?
    values.scopeId = 0;
    values.value.uiVal = xgmi_links;
    values.timestamp = timestamp as i64;
    values.valueType = nvmlValueType_t::NVML_VALUE_TYPE_UNSIGNED_INT;
    values.nvmlReturn = nvmlReturn_t::NVML_SUCCESS;
    Ok(())
}

unsafe fn total_xgmi_links(device: u32) -> Result<u32, nvmlReturn_t> {
    let mut xgmi_links = 0;
    let mut total_devices = 0;
    smi_call! {rsmi_num_monitor_devices(&mut total_devices)};
    for target_dev in 0..total_devices {
        if target_dev == device {
            continue;
        }
        let mut hops = 0;
        let mut link_type = RSMI_IO_LINK_TYPE::RSMI_IOLINK_TYPE_UNDEFINED;
        smi_call! {rsmi_topo_get_link_type(device, target_dev, &mut hops, &mut link_type)};
        if link_type == RSMI_IO_LINK_TYPE::RSMI_IOLINK_TYPE_XGMI {
            xgmi_links += 1;
        }
    }
    Ok(xgmi_links)
}

pub(crate) unsafe fn device_get_count(device_count: *mut u32) -> Result<(), nvmlReturn_t> {
    if device_count == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    smi_call! {rsmi_num_monitor_devices(device_count)};
    Ok(())
}

pub(crate) unsafe fn device_get_power_management_limit(
    _device: nvmlDevice_t,
    _limit: *mut u32,
) -> Result<(), nvmlReturn_t> {
    Err(nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED)
}

pub(crate) unsafe fn device_get_power_management_limit_constraints(
    _device: *mut nvmlDevice_st,
    _min_limit: *mut u32,
    _max_limit: *mut u32,
) -> Result<(), nvmlReturn_t> {
    Err(nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED)
}

pub(crate) unsafe fn device_get_power_usage(
    _device: *mut nvmlDevice_st,
    _power: *mut u32,
) -> Result<(), nvmlReturn_t> {
    Err(nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED)
}

pub(crate) unsafe fn device_get_pcie_throughput(
    _device: *mut nvmlDevice_st,
    _counter: nvmlPcieUtilCounter_enum,
    _value: *mut u32,
) -> Result<(), nvmlReturn_t> {
    Err(nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED)
}
