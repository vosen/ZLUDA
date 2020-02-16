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
pub extern "stdcall" fn cuDeviceGetCount(count: &mut std::os::raw::c_int) -> cu::Result {
    return cu::Result::SUCCESS;
}

#[no_mangle]
pub extern "stdcall" fn cuDeviceGet(device: *mut cu::Device, ordinal: ::std::os::raw::c_int) -> cu::Result {
    unimplemented!()
}