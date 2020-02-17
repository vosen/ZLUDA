use level_zero_sys::*;

use std::cmp;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ptr;

macro_rules! assert_size_eq {
    ($x:ty, $($xs:ty),+ $(,)?) => {
        const _: fn() = || {
            $(let _ = ::std::mem::transmute::<$x, $xs>;)+
        };
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

pub trait Versioned : Sized {
    type Version;

    fn new() -> Self {
        let mut result = unsafe { std::mem::zeroed::<Self>() };
        let ver = result.version();
        *ver = Self::current();
        return result;
    }

    fn current() -> Self::Version;

    fn version(&mut self) -> &mut Self::Version;
}

impl Versioned for ze_device_memory_properties_t {
    type Version = ze_device_memory_properties_version_t;
    fn current() -> Self::Version {
        ze_device_memory_properties_version_t::ZE_DEVICE_MEMORY_PROPERTIES_VERSION_CURRENT
    }
    fn version(&mut self) -> &mut Self::Version {
        &mut self.version
    }
}

impl Versioned for ze_device_properties_t {
    type Version = ze_device_properties_version_t;
    fn current() -> Self::Version {
        ze_device_properties_version_t::ZE_DEVICE_PROPERTIES_VERSION_CURRENT
    }
    fn version(&mut self) -> &mut Self::Version {
        &mut self.version
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)] // required so a Vec<ze_device_handle_t> can be safely transmutted to Vec<Device>
pub struct Device(pub ze_device_handle_t);

impl Device {
    pub fn new_vec(v: Vec<ze_device_handle_t>) -> Vec<Device> {
        assert_size_eq!(Device, ze_device_handle_t);
        unsafe { mem::transmute(v) }
    }

    pub fn get_name(self, name: *mut c_char, len: c_int) -> l0::ze_result_t {
        let mut props = Box::new(l0::ze_device_properties_t::new());
        l0_check! { l0::zeDeviceGetProperties(self.0, props.as_mut()) };
        let null_pos = props.name.iter().position(|&c| c == 0).unwrap_or(0);
        let dst_null_pos = cmp::min((len - 1) as usize, null_pos);
        unsafe { *(name.add(dst_null_pos)) = 0 };
        unsafe { std::ptr::copy_nonoverlapping(props.name.as_ptr(), name, dst_null_pos) };
        l0::ze_result_t::ZE_RESULT_SUCCESS
    }

    pub fn total_mem(self, bytes: *mut usize) -> l0::ze_result_t {
        let mut count = 0;
        l0_check! { l0::zeDeviceGetMemoryProperties(self.0, &mut count, ptr::null_mut()) };
        if count == 0 {
            return l0::ze_result_t::ZE_RESULT_ERROR_UNKNOWN;
        }
        let mut props = vec![l0::ze_device_memory_properties_t::new(); count as usize];
        l0_check! { l0::zeDeviceGetMemoryProperties(self.0, &mut count, props.as_mut_ptr()) };
        let iter_count = cmp::min(count as usize, props.len());
        if iter_count == 0 {
            return l0::ze_result_t::ZE_RESULT_ERROR_UNKNOWN;
        }
        let max_mem = props.iter().take(iter_count).map(|p| p.totalSize).max().unwrap();
        unsafe { *bytes = max_mem as usize };
        l0::ze_result_t::ZE_RESULT_SUCCESS
    }
}