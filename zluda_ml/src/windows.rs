use crate::{common, nvml::*};
use atiadlxx_sys::*;
use std::borrow::Cow;
use std::ffi::CStr;
use std::io::Write;
use std::{
    alloc::Layout,
    collections::{btree_map, BTreeMap},
    ffi::c_void,
    mem, ptr,
    sync::atomic::{self, AtomicPtr, Ordering},
};

const MIB: u64 = 1_048_576;
const AMD_PCI_ID_DECIMAL: i32 = 1002i32;
const AMD_PCI_ID_HEX: u32 = 0x1002u32;
/*
    Undocumented, but internet suggests that ADLODNTemperatureType is defined as
    enum class ADLODNTemperatureType
    {
        Core = 1,
        Memory = 2,
        VrmCore = 3,
        VrmMemory = 4,
        Liquid = 5,
        Plx = 6,
        Hotspot = 7,
    };
*/
const ADLODNTEMPERATURE_TYPE_CORE: i32 = 1;

static mut ADL_CONTEXT: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static mut DEVICES: Vec<Device> = Vec::new();

struct Device {
    adapter_info: atiadlxx_sys::AdapterInfoX2,
}

macro_rules! adl_call {
    ($x:expr) => {{
        let result = $x;
        if result as u32 != atiadlxx_sys::ADL_OK {
            return Err(result.into());
        }
    }};
}

impl From<i32> for nvmlReturn_t {
    fn from(error: i32) -> Self {
        if error as u32 == atiadlxx_sys::ADL_OK {
            return nvmlReturn_t::NVML_SUCCESS;
        }
        match error {
            atiadlxx_sys::ADL_ERR_NOT_INIT => nvmlReturn_t::NVML_ERROR_UNINITIALIZED,
            atiadlxx_sys::ADL_ERR_NOT_SUPPORTED => nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED,
            _ => nvmlReturn_t::NVML_ERROR_UNKNOWN,
        }
    }
}

pub(crate) unsafe fn shutdown() -> nvmlReturn_t {
    let context = ADL_CONTEXT.load(Ordering::Relaxed);
    if context == ptr::null_mut() {
        return nvmlReturn_t::NVML_ERROR_UNINITIALIZED;
    }
    match ADL_CONTEXT.compare_exchange(
        context,
        ptr::null_mut(),
        std::sync::atomic::Ordering::SeqCst,
        std::sync::atomic::Ordering::SeqCst,
    ) {
        // TODO: should we call free after destroy?
        Ok(_) => nvmlReturn_t::from(atiadlxx_sys::ADL2_Main_Control_Destroy(context)),
        Err(ptr) if ptr == ptr::null_mut() => nvmlReturn_t::NVML_SUCCESS,
        Err(_) => nvmlReturn_t::NVML_ERROR_UNKNOWN,
    }
}

pub(crate) unsafe fn init() -> Result<(), nvmlReturn_t> {
    let mut context = ptr::null_mut();
    adl_call!(atiadlxx_sys::ADL2_Main_ControlX2_Create(
        Some(alloc),
        1,
        &mut context,
        atiadlxx_sys::ADLThreadingModel::ADL_THREADING_LOCKED,
    ));
    match ADL_CONTEXT.compare_exchange(ptr::null_mut(), context, Ordering::SeqCst, Ordering::SeqCst)
    {
        Ok(_) => {
            let mut num_adapters = 0;
            let mut adapter_info = ptr::null_mut();
            adl_call!(atiadlxx_sys::ADL2_Adapter_AdapterInfoX4_Get(
                context,
                -1,
                &mut num_adapters,
                &mut adapter_info
            ));
            let devices = (0..num_adapters as usize)
                .filter_map(|idx| {
                    let adapter_info = *adapter_info.add(idx);
                    // Having this id in decimal is a nonsense, but that's just
                    // how ADL works
                    if adapter_info.iVendorID == AMD_PCI_ID_DECIMAL
                        && adapter_info.iExist == 1
                        && adapter_info.iPresent == 1
                    {
                        Some(Device { adapter_info })
                    } else {
                        None
                    }
                })
                // For some reason ADL returns a huge number of adapters
                // I'm not sure why, probably to account for multiple displays,
                // physical and virtual
                .fold(BTreeMap::new(), |mut map, dev| {
                    if let btree_map::Entry::Vacant(entry) = map.entry((
                        dev.adapter_info.iBusNumber,
                        dev.adapter_info.iDeviceNumber,
                        dev.adapter_info.iFunctionNumber,
                    )) {
                        entry.insert(dev);
                    }
                    map
                })
                .into_iter()
                .map(|(_, device)| device)
                .collect::<Vec<_>>();
            DEVICES = devices;
            atomic::fence(Ordering::SeqCst);
            Ok(())
        }
        Err(_) => Ok(()),
    }
}

pub(crate) unsafe fn init_with_flags(_flags: ::std::os::raw::c_uint) -> Result<(), nvmlReturn_t> {
    init()
}

struct CountingWriter<T: std::io::Write> {
    pub base: T,
    pub len: usize,
}

impl<T: std::io::Write> std::io::Write for CountingWriter<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.len += buf.len();
        self.base.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.base.flush()
    }
}

pub(crate) unsafe fn device_get_handle_by_index(
    index: ::std::os::raw::c_uint,
    device: *mut nvmlDevice_t,
) -> Result<(), nvmlReturn_t> {
    if device == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let context = ADL_CONTEXT.load(Ordering::SeqCst);
    if context == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_UNINITIALIZED);
    }
    let device_ptr = mem::transmute(DEVICES.get(index as usize));
    *device = device_ptr;
    if device_ptr != ptr::null_mut() {
        Ok(())
    } else {
        Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT)
    }
}

pub(crate) unsafe fn device_get_handle_by_pci_bus_id(
    pci_bus_id: *const ::std::os::raw::c_char,
    device: *mut nvmlDevice_t,
) -> Result<(), nvmlReturn_t> {
    if device == ptr::null_mut() || pci_bus_id == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let context = ADL_CONTEXT.load(Ordering::SeqCst);
    if context == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_UNINITIALIZED);
    }
    let (bus, pci_device, function) = common::parse_pci_address(pci_bus_id)?;
    let device_ptr = mem::transmute(DEVICES.iter().find(|dev| {
        dev.adapter_info.iBusNumber == bus
            && dev.adapter_info.iDeviceNumber == pci_device
            && dev.adapter_info.iFunctionNumber == function
    }));
    *device = device_ptr;
    if device_ptr != ptr::null_mut() {
        Ok(())
    } else {
        Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT)
    }
}

pub(crate) unsafe fn device_get_fan_speed(
    device: nvmlDevice_t,
    speed: *mut u32,
) -> Result<(), nvmlReturn_t> {
    if device == ptr::null_mut() || speed == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let context = ADL_CONTEXT.load(Ordering::SeqCst);
    if context == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_UNINITIALIZED);
    }
    let device: &Device = mem::transmute(device);
    let mut fan_speed_info = mem::zeroed();
    adl_call!(atiadlxx_sys::ADL2_Overdrive6_FanSpeed_Get(
        context,
        device.adapter_info.iAdapterIndex,
        &mut fan_speed_info
    ));
    *speed = fan_speed_info.iFanSpeedPercent as u32;
    Ok(())
}

pub(crate) unsafe fn device_get_memory_info(
    device: nvmlDevice_t,
    memory: *mut nvmlMemory_t,
) -> Result<(), nvmlReturn_t> {
    if memory == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let context = ADL_CONTEXT.load(Ordering::SeqCst);
    if context == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_UNINITIALIZED);
    }
    let device: &Device = mem::transmute(device);
    let mut memory_info = mem::zeroed();
    adl_call!(atiadlxx_sys::ADL2_Adapter_MemoryInfo2_Get(
        context,
        device.adapter_info.iAdapterIndex,
        &mut memory_info,
    ));
    let mut memory_use_in_mb = 0;
    adl_call!(atiadlxx_sys::ADL2_Adapter_VRAMUsage_Get(
        context,
        device.adapter_info.iAdapterIndex,
        &mut memory_use_in_mb,
    ));
    // visible/invisible memory in memory_info is some nonsense,
    // on my machine: iMemorySize is 15.98GiB, iInvisibleMemorySize is 15.73GiB,
    // iVisibleMemorySize is 0.25GiB
    let total = memory_info.iMemorySize.max(0) as u64;
    let used = memory_use_in_mb.max(0) as u64 * MIB;
    let free = total.saturating_sub(used);
    let nvml_memory = nvmlMemory_t { total, used, free };
    *memory = nvml_memory;
    Ok(())
}

pub(crate) unsafe fn device_get_pci_info(
    device: nvmlDevice_t,
    pci: *mut nvmlPciInfo_t,
) -> Result<(), nvmlReturn_t> {
    if device == ptr::null_mut() || pci == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let context = ADL_CONTEXT.load(Ordering::SeqCst);
    if context == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_UNINITIALIZED);
    }
    let device: &Device = mem::transmute(device);
    let mut bus_id_legacy = [0u8; 16];
    write!(
        &mut bus_id_legacy[..],
        "0:{:x}:{:x}.{:x}\0",
        device.adapter_info.iBusNumber,      // 2 digits
        device.adapter_info.iDeviceNumber,   // 2 digits
        device.adapter_info.iFunctionNumber  // 1 digit
    )
    .map_err(|_| nvmlReturn_t::NVML_ERROR_UNKNOWN)?;
    let mut bus_id = [0u8; 32];
    bus_id[..16].copy_from_slice(&bus_id_legacy);
    let pnp_string = CStr::from_ptr(device.adapter_info.strPNPString.as_ptr()).to_string_lossy();
    let subsys_id = extract_prefixed_hex_number(&pnp_string, "SUBSYS_", 8);
    let device_id = extract_prefixed_hex_number(&pnp_string, "DEV_", 4);
    let pci_location = nvmlPciInfo_t {
        busIdLegacy: mem::transmute(bus_id_legacy),
        bus: device.adapter_info.iBusNumber as u32,
        domain: 0,
        device: device.adapter_info.iDeviceNumber as u32,
        pciDeviceId: (device_id << 16) | AMD_PCI_ID_HEX,
        pciSubSystemId: subsys_id,
        busId: mem::transmute(bus_id),
    };
    *pci = pci_location;
    Ok(())
}

unsafe fn extract_prefixed_hex_number(pnp_string: &Cow<str>, prefix: &str, digits: usize) -> u32 {
    let prefix_start = pnp_string.find(prefix).unwrap();
    let value_start = prefix_start + prefix.len();
    let value_str = &pnp_string.as_bytes()[value_start..value_start + digits];
    u32::from_str_radix(std::str::from_utf8_unchecked(value_str), 16).unwrap()
}

pub(crate) unsafe fn device_get_temperature(
    device: nvmlDevice_t,
    sensor_type: nvmlTemperatureSensors_t,
    temp: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    if device == ptr::null_mut()
        || temp == ptr::null_mut()
        || sensor_type != nvmlTemperatureSensors_t::NVML_TEMPERATURE_GPU
    {
        return nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT;
    }
    let context = ADL_CONTEXT.load(Ordering::SeqCst);
    if context == ptr::null_mut() {
        return nvmlReturn_t::NVML_ERROR_UNINITIALIZED;
    }
    let device: &Device = mem::transmute(device);
    atiadlxx_sys::ADL2_OverdriveN_Temperature_Get(
        context,
        device.adapter_info.iAdapterIndex,
        ADLODNTEMPERATURE_TYPE_CORE,
        temp as _,
    )
    .into()
}

pub(crate) unsafe fn device_get_utilization_rates(
    device: nvmlDevice_t,
    utilization: *mut nvmlUtilization_t,
) -> Result<(), nvmlReturn_t> {
    if device == ptr::null_mut() || utilization == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let context = ADL_CONTEXT.load(Ordering::SeqCst);
    if context == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_UNINITIALIZED);
    }
    let device: &Device = mem::transmute(device);
    let mut activity = mem::zeroed();
    adl_call!(atiadlxx_sys::ADL2_Overdrive5_CurrentActivity_Get(
        context,
        device.adapter_info.iAdapterIndex,
        &mut activity
    ));
    *utilization = nvmlUtilization_t {
        gpu: activity.iActivityPercent as u32,
        memory: activity.iActivityPercent as u32,
    };
    Ok(())
}

unsafe extern "C" fn alloc(size: i32) -> *mut c_void {
    if size < 0 {
        return ptr::null_mut();
    }
    std::alloc::alloc(Layout::from_size_align_unchecked(
        size as usize,
        mem::size_of::<u32>(),
    )) as *mut c_void
}

pub(crate) unsafe fn device_get_field_values(
    _device: *mut nvmlDevice_st,
    _values_count: i32,
    _values: *mut nvmlFieldValue_st,
) -> nvmlReturn_t {
    crate::common::unimplemented()
}

pub(crate) unsafe fn device_get_count(device_count: *mut u32) -> nvmlReturn_t {
    if device_count == ptr::null_mut() {
        return nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT;
    }
    *device_count = DEVICES.len() as u32;
    nvmlReturn_t::NVML_SUCCESS
}

pub(crate) unsafe fn device_get_power_management_limit(
    device: nvmlDevice_t,
    limit: *mut u32,
) -> Result<(), nvmlReturn_t> {
    if device == ptr::null_mut() || limit == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let context = ADL_CONTEXT.load(Ordering::SeqCst);
    if context == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_UNINITIALIZED);
    }
    let device: &Device = mem::transmute(device);
    let mut power_setting = mem::zeroed();
    adl_call!(ADL2_OverdriveN_PowerLimit_Get(
        context,
        device.adapter_info.iAdapterIndex,
        &mut power_setting
    ));
    // This function does not actually work, so I'm not sure
    // what it returns (watts, milliwats, something else?)
    *limit = (power_setting.iTDPLimit * 1000) as u32;
    Ok(())
}

pub(crate) unsafe fn device_get_power_management_limit_constraints(
    device: *mut nvmlDevice_st,
    min_limit: *mut u32,
    max_limit: *mut u32,
) -> Result<(), nvmlReturn_t> {
    // TODO: actually implement constraints
    device_get_power_management_limit(device, min_limit)?;
    device_get_power_management_limit(device, max_limit)
}

pub(crate) unsafe fn device_get_power_usage(
    device: *mut nvmlDevice_st,
    power: *mut u32,
) -> Result<(), nvmlReturn_t> {
    if device == ptr::null_mut() || power == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let context = ADL_CONTEXT.load(Ordering::SeqCst);
    if context == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_UNINITIALIZED);
    }
    let device: &Device = mem::transmute(device);
    let mut pmlog = mem::zeroed();
    adl_call!(ADL2_New_QueryPMLogData_Get(
        context,
        device.adapter_info.iAdapterIndex,
        &mut pmlog
    ));
    let sensor = pmlog.sensors[ADLSensorType::PMLOG_ASIC_POWER.0 as usize];
    if sensor.supported == 0 {
        return Err(nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED);
    }
    *power = (sensor.value * 1000) as u32;
    Ok(())
}

pub(crate) unsafe fn device_get_pcie_throughput(
    _device: *mut nvmlDevice_st,
    _counter: nvmlPcieUtilCounter_enum,
    _value: *mut u32,
) -> Result<(), nvmlReturn_t> {
    Err(nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED)
}
