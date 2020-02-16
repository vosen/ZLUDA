extern crate level_zero_sys as l0;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::ptr;

mod cu;
mod export_table;

macro_rules! l0_check {
    ($exp:expr) => {
        {
            let result = unsafe{ $exp };
            if result != l0::ze_result_t::ZE_RESULT_SUCCESS {
                return Err(result)
            }
        }
    };
}

lazy_static! {
    pub static ref GLOBAL_STATE: Mutex<Option<Driver>> = Mutex::new(None);
}

pub struct Driver {
    base: l0::ze_driver_handle_t
}

unsafe impl Send for Driver {}
unsafe impl Sync for Driver {}

impl Driver {
    fn new() -> Result<Driver, l0::ze_result_t> {
        let mut driver_count = 1;
        let mut handle = ptr::null_mut();
        l0_check!{ l0::zeDriverGet(&mut driver_count, &mut handle) }; 
        Ok(Driver{ base: handle })
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
        unsafe { l0::zeDeviceGet(self.base, count as *mut _ as *mut _, ptr::null_mut()) }
    }

    fn device_get(&self, device: *mut l0::ze_device_handle_t, ordinal: ::std::os::raw::c_int) -> l0::ze_result_t {
        let count = (ordinal as u32) + 1;
        let mut devices_found = count;
        let mut handles = vec![ptr::null_mut(); count as usize];
        let result = unsafe { l0::zeDeviceGet(self.base, &mut devices_found, handles.as_mut_ptr()) };
        if  result != l0::ze_result_t::ZE_RESULT_SUCCESS {
            return result;
        }
        if devices_found < count {
            return l0::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT;
        }
        unsafe { *device = handles[(count as usize) - 1] };
        l0::ze_result_t::ZE_RESULT_SUCCESS
    }
}

#[no_mangle]
pub extern "stdcall" fn cuDriverGetVersion(version: &mut std::os::raw::c_int) -> cu::Result {
    *version = i32::max_value();
    return cu::Result::SUCCESS;
}

#[no_mangle]
pub unsafe extern "stdcall" fn cuInit(_: *const std::os::raw::c_uint) -> cu::Result {
    let l0_init = l0::zeInit(l0::ze_init_flag_t::ZE_INIT_FLAG_GPU_ONLY);
    if l0_init !=  l0::ze_result_t::ZE_RESULT_SUCCESS {
        return cu::Result::from_l0(l0_init);
    };
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
pub extern "stdcall" fn cuDeviceGetCount(count: *mut std::os::raw::c_int) -> cu::Result {
    Driver::call(|driver| driver.device_get_count(count))
}

#[no_mangle]
pub extern "stdcall" fn cuDeviceGet(device: *mut l0::ze_device_handle_t, ordinal: ::std::os::raw::c_int) -> cu::Result {
    Driver::call(|driver| driver.device_get(device, ordinal))
}