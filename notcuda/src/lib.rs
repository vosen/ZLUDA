extern crate level_zero_sys as l0;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::ptr;
use std::cmp;
use std::os::raw::{c_char, c_int, c_uint};

mod cu;
mod export_table;
mod ze;

use ze::Versioned;

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

macro_rules! l0_check {
    ($exp:expr) => {
        {
            let result = unsafe{ $exp };
            if result != l0::ze_result_t::ZE_RESULT_SUCCESS {
                return result;
            }
        }
    };
}

lazy_static! {
    pub static ref GLOBAL_STATE: Mutex<Option<Driver>> = Mutex::new(None);
}

pub struct Driver {
    base: l0::ze_driver_handle_t,
    devices: Vec::<l0::ze_device_handle_t>
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
        Ok(Driver{ base: handle, devices: devices })
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

    fn device_get_name(&self, name: *mut c_char, len: c_int, cu::Device(dev): cu::Device) -> l0::ze_result_t {
        if (dev as usize) >= self.devices.len() {
            return l0::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT;
        }
        let mut props = Box::new(l0::ze_device_properties_t::new());
        l0_check! { l0::zeDeviceGetProperties(self.devices[dev as usize], props.as_mut()) };
        let null_pos = props.name.iter().position(|&c| c == 0).unwrap_or(0);
        let dst_null_pos = cmp::min((len - 1) as usize, null_pos);
        unsafe { *(name.add(dst_null_pos)) = 0 };
        unsafe { std::ptr::copy_nonoverlapping(props.name.as_ptr(), name, dst_null_pos) };
        l0::ze_result_t::ZE_RESULT_SUCCESS
    }

    fn device_total_mem(&self, bytes: *mut usize, cu::Device(dev): cu::Device) -> l0::ze_result_t {
        if (dev as usize) >= self.devices.len() {
            return l0::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT;
        }
        let dev = dev as usize;
        let mut count = 0;
        l0_check! { l0::zeDeviceGetMemoryProperties(self.devices[dev], &mut count, ptr::null_mut()) };
        if count == 0 {
            return l0::ze_result_t::ZE_RESULT_ERROR_UNKNOWN;
        }
        let mut props = vec![l0::ze_device_memory_properties_t::new(); count as usize];
        l0_check! { l0::zeDeviceGetMemoryProperties(self.devices[dev], &mut count, props.as_mut_ptr()) };
        let iter_count = cmp::min(count as usize, props.len());
        if iter_count == 0 {
            return l0::ze_result_t::ZE_RESULT_ERROR_UNKNOWN;
        }
        let max_mem = props.iter().take(iter_count).map(|p| p.totalSize).max().unwrap();
        unsafe { *bytes = max_mem as usize };
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
pub extern "C" fn cuDeviceGetName(name: *mut c_char, len: c_int, dev: cu::Device) -> cu::Result {
    let cu::Device(dev_idx) = dev;
    if len <= 0 || dev_idx < 0 || name == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Driver::call(|driver| driver.device_get_name(name, len, dev))
}

#[no_mangle]
pub extern "C" fn cuDeviceTotalMem_v2(bytes: *mut usize, dev: cu::Device) -> cu::Result {
    let cu::Device(dev_idx) = dev;
    if dev_idx < 0 || bytes == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Driver::call(|driver| driver.device_total_mem(bytes, dev))
}