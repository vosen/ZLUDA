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

pub(crate) unsafe fn device_get_handle_by_pci_bus_id_v2(
    pci_bus_id: &std::ffi::CStr,
    device: &mut cuda_types::nvml::nvmlDevice_t,
) -> nvmlReturn_t {
    let pci = parse_pci_bus_id(pci_bus_id).ok_or(nvmlError_t::INVALID_ARGUMENT)?;
    let bdfid = pci.to_bdfid();
    let mut device_count = 0;
    rsmi_num_monitor_devices(&mut device_count)?;
    for dv_ind in 0..device_count {
        let mut curr_bdfid = 0;
        rsmi_dev_pci_id_get(dv_ind, &mut curr_bdfid)?;
        if curr_bdfid == bdfid {
            *device = Device { _index: dv_ind }.wrap();
            return nvmlReturn_t::SUCCESS;
        }
    }
    nvmlReturn_t::ERROR_NOT_FOUND
}

#[derive(Clone, Copy)]
struct PciBusId {
    domain: u16,
    bus: u8,
    device: u8,
    function: u8,
}
impl PciBusId {
    fn to_bdfid(self) -> u64 {
        ((self.domain as u64) << 32)
            | ((self.bus as u64) << 8)
            | ((self.device as u64) << 3)
            | (self.function as u64)
    }
}

fn parse_pci_bus_id(id: &std::ffi::CStr) -> Option<PciBusId> {
    let s = id.to_str().ok()?.trim();
    let mut segments = s.split(':').rev();
    let func_dev = segments.next()?;
    let mut function_dev = func_dev.split('.');
    let device = function_dev.next()?;
    let function = function_dev.next();
    let function = match function {
        Some(f) => hex_u8(f)?,
        None => 0,
    };
    if function_dev.next().is_some() {
        return None;
    }
    let bus = segments.next()?;
    let domain = segments.next();
    let domain = match domain {
        Some(d) => hex_u16(d)?,
        None => 0,
    };
    if segments.next().is_some() {
        return None;
    }
    Some(PciBusId {
        domain,
        bus: hex_u8(bus)?,
        device: hex_u8(device)?,
        function,
    })
}

fn hex_u16(s: &str) -> Option<u16> {
    u16::from_str_radix(s, 16).ok()
}

fn hex_u8(s: &str) -> Option<u8> {
    u8::from_str_radix(s, 16).ok()
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

pub(crate) fn device_get_compute_running_processes(
    _device: cuda_types::nvml::nvmlDevice_t,
    info_count: &mut ::core::ffi::c_uint,
    _infos: Option<&mut cuda_types::nvml::nvmlProcessInfo_v1_t>,
) -> nvmlReturn_t {
    *info_count = 0;
    Ok(())
}

pub(crate) unsafe fn device_get_memory_info(
    device: &Device,
    memory: &mut cuda_types::nvml::nvmlMemory_t,
) -> nvmlReturn_t {
    let mut total = mem::zeroed();

    rsmi_dev_memory_total_get(
        device._index,
        rsmi_memory_type_t::RSMI_MEM_TYPE_VRAM,
        &mut total,
    )?;
    let mut used = mem::zeroed();

    rsmi_dev_memory_usage_get(
        device._index,
        rsmi_memory_type_t::RSMI_MEM_TYPE_VRAM,
        &mut used,
    )?;

    *memory = nvmlMemory_t {
        total,
        free: total.saturating_sub(used),
        used,
    };

    Ok(())
}

pub(crate) fn device_get_name(
    device: &Device,
    name: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> nvmlReturn_t {
    unsafe { rsmi_dev_name_get(device._index, name, length as usize) }?;
    zluda_common::append_suffix(name, length as usize);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_pci_bus_id_full() {
        let id = std::ffi::CString::new("0100:65:a0.f").unwrap();
        let parsed = super::parse_pci_bus_id(&id).unwrap();
        assert_eq!(parsed.domain, 0x0100);
        assert_eq!(parsed.bus, 0x65);
        assert_eq!(parsed.device, 0xa0);
        assert_eq!(parsed.function, 0xf);
    }

    #[test]
    fn parse_pci_bus_id_no_func() {
        let id = std::ffi::CString::new("0100:65:a0").unwrap();
        let parsed = super::parse_pci_bus_id(&id).unwrap();
        assert_eq!(parsed.domain, 0x0100);
        assert_eq!(parsed.bus, 0x65);
        assert_eq!(parsed.device, 0xa0);
        assert_eq!(parsed.function, 0);
    }

    #[test]
    fn parse_pci_bus_id_no_domain() {
        let id = std::ffi::CString::new("65:a0.f").unwrap();
        let parsed = super::parse_pci_bus_id(&id).unwrap();
        assert_eq!(parsed.domain, 0);
        assert_eq!(parsed.bus, 0x65);
        assert_eq!(parsed.device, 0xa0);
        assert_eq!(parsed.function, 0xf);
    }

    #[test]
    fn parse_long_pci_bus_id() {
        let id = std::ffi::CString::new("00000002:00:00.0").unwrap();
        let parsed = super::parse_pci_bus_id(&id).unwrap();
        assert_eq!(parsed.domain, 0x0002);
        assert_eq!(parsed.bus, 0x00);
        assert_eq!(parsed.device, 0x00);
        assert_eq!(parsed.function, 0x00);
    }
}
