extern crate level_zero_sys as l0;
#[macro_use]
extern crate lazy_static;

use std::convert::TryFrom;
use std::sync::Mutex;
use std::ptr;
use std::os::raw::{c_char, c_int, c_uint};

#[macro_use]
macro_rules! l0_check_err {
    ($exp:expr) => {
        {
            let result = unsafe{ $exp };
            if result != l0::ze_result_t::ZE_RESULT_SUCCESS {
                return Err(result);
            }
        }
    };
}

mod cu;
mod export_table;
mod ze;

lazy_static! {
    pub static ref GLOBAL_STATE: Mutex<Option<Driver>> = Mutex::new(None);
}

pub struct Driver {
    base: l0::ze_driver_handle_t,
    devices: Vec::<ze::Device>
}
unsafe impl Send for Driver {}
unsafe impl Sync for Driver {}

impl Driver {
    fn new() -> Result<Driver, l0::ze_result_t> {
        let mut driver_count = 1;
        let mut handle = ptr::null_mut();
        l0_check_err!{ l0::zeDriverGet(&mut driver_count, &mut handle) };
        let mut count = 0;
        l0_check_err! { l0::zeDeviceGet(handle, &mut count, ptr::null_mut()) }
        let mut devices = vec![ptr::null_mut(); count as usize];
        l0_check_err! { l0::zeDeviceGet(handle, &mut count, devices.as_mut_ptr()) }
        if (count as usize) < devices.len() {
            devices.truncate(count as usize);
        }
        Ok(Driver{ base: handle, devices: ze::Device::new_vec(devices) })
    }

    fn call<F: FnOnce(&mut Driver) -> l0::ze_result_t>(f: F) -> cu::Result {
        let mut lock = GLOBAL_STATE.try_lock();
        if let Ok(ref mut mutex) = lock {
            match **mutex {
                None => return cu::Result::ERROR_NOT_INITIALIZED,
                Some(ref mut driver) => {
                    return cu::Result::from_l0(f(driver));
                }
            }
        } else {
            return cu::Result::ERROR_UNKNOWN;
        }
    }

    fn call_device<F: FnOnce(&mut ze::Device) -> l0::ze_result_t>(cu::Device(dev): cu::Device, f: F) -> cu::Result {
        if dev < 0 {
            return cu::Result::ERROR_INVALID_VALUE;
        }
        let dev = dev as usize;
        Driver::call(|driver| {
            if dev >= driver.devices.len() {
                return l0::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT;
            }
            f(&mut driver.devices[dev])
        })
    }

    fn device_get_count(&self, count: *mut i32) -> l0::ze_result_t {
        unsafe { *count = self.devices.len() as i32 };
        l0::ze_result_t::ZE_RESULT_SUCCESS
    }

    fn device_get(&self, device: *mut cu::Device, ordinal: c_int) -> l0::ze_result_t {
        if (ordinal as usize) >= self.devices.len() {
            return l0::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT;
        }
        unsafe { *device = cu::Device(ordinal) };
        l0::ze_result_t::ZE_RESULT_SUCCESS    
    }
}

#[no_mangle]
pub unsafe extern "C" fn cuDriverGetVersion(version: *mut c_int) -> cu::Result {
    if version == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    *version = i32::max_value();
    return cu::Result::SUCCESS;
}

#[no_mangle]
pub unsafe extern "C" fn cuInit(_: c_uint) -> cu::Result {
    let l0_init = l0::zeInit(l0::ze_init_flag_t::ZE_INIT_FLAG_GPU_ONLY);
    if l0_init !=  l0::ze_result_t::ZE_RESULT_SUCCESS {
        return cu::Result::from_l0(l0_init);
    }
    let mut lock = GLOBAL_STATE.try_lock();
    if let Ok(ref mut mutex) = lock {
        if let None = **mutex {
            match Driver::new() {
                Ok(state) => **mutex = Some(state),
                Err(err) => return cu::Result::from_l0(err)
            }
        }
    } else {
        return cu::Result::ERROR_UNKNOWN;
    }
    cu::Result::SUCCESS
}

#[no_mangle]
pub extern "C" fn cuDeviceGetCount(count: *mut c_int) -> cu::Result {
    if count == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Driver::call(|driver| driver.device_get_count(count))
}

#[no_mangle]
pub extern "C" fn cuDeviceGet(device: *mut cu::Device, ordinal: c_int) -> cu::Result {
    if ordinal < 0 || device == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Driver::call(|driver| driver.device_get(device, ordinal))
}

#[no_mangle]
pub extern "C" fn cuDeviceGetName(name: *mut c_char, len: c_int, dev_idx: cu::Device) -> cu::Result {
    if name == ptr::null_mut() || len <= 0 {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Driver::call_device(dev_idx, |dev| dev.get_name(name, len))
}

#[no_mangle]
pub extern "C" fn cuDeviceTotalMem_v2(bytes: *mut usize, dev_idx: cu::Device) -> cu::Result {
    if bytes == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Driver::call_device(dev_idx, |dev| dev.total_mem(bytes))
}

#[no_mangle]
pub extern "C" fn cuDeviceGetAttribute(pi: *mut c_int, attrib: c_int, dev_idx: cu::Device) -> cu::Result {

    if pi == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    let attrib = match u8::try_from(attrib) {
        Ok(a) => a,
        Err(_) => return cu::Result::ERROR_INVALID_VALUE
    };
    match cu::DeviceAttribute::try_new(attrib) {
        Some(cu::DeviceAttribute::Static(a)) => {
            unsafe { *pi = ze::Device::get_attribute_static(a) };
            cu::Result::SUCCESS
        },
        Some(cu::DeviceAttribute::Dynamic(a)) => Driver::call_device(dev_idx, |dev| dev.get_attribute(pi, a)),
        // TODO: add support for more properties
        None => cu::Result::SUCCESS
    }
}

#[no_mangle]
pub extern "C" fn cuDeviceGetUuid(uuid: *mut cu::Uuid, dev_idx: cu::Device) -> cu::Result {
    if uuid == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Driver::call_device(dev_idx, |dev| dev.get_uuid(uuid))
}


#[no_mangle]
pub extern "C" fn cuMemAlloc_v2(dptr: *mut cu::DevicePtr, bytesize: usize) -> cu::Result {
    unimplemented!()
}