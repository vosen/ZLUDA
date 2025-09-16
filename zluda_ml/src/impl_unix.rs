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
    let mut domain: u16 = 0;
    let mut rest = s;
    if let Some(colon1) = s.find(':') {
        if colon1 == 4 {
            domain = hex_u16(&s[..4])?;
            rest = &s[5..];
        }
    }
    let mut parts = rest.split(':');
    let bus_part = parts.next()?;
    let tail = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    let mut dev_func = tail.split('.');
    let dev_part = dev_func.next()?;
    let func_part = dev_func.next();
    let function = match func_part {
        Some(f) => hex_u8(f)?,
        None => 0,
    };
    Some(PciBusId {
        domain,
        bus: hex_u8(bus_part)?,
        device: hex_u8(dev_part)?,
        function,
    })
}

fn hex_u16(s: &str) -> Option<u16> {
    if s.len() > 4 {
        return None;
    }
    u16::from_str_radix(s, 16).ok()
}

fn hex_u8(s: &str) -> Option<u8> {
    if s.len() > 2 {
        return None;
    }
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
}
