use level_zero_sys::*;
use super::cu;

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

    fn get_device_properties(self) -> Result<Box<ze_device_properties_t>, ze_result_t> {
        let mut props = Box::new(l0::ze_device_properties_t::new());
        l0_check_err! { l0::zeDeviceGetProperties(self.0, props.as_mut()) };
        Ok(props)
    }

    pub fn get_name(self, name: *mut c_char, len: c_int) -> l0::ze_result_t {
        let props = match self.get_device_properties() {
            Ok(props) => props,
            Err(res) => return res
        };
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

    pub fn try_get_attribute(attr: cu::DeviceAttribute) -> Option<c_int> {
        match attr {
            cu::DeviceAttribute::COMPUTE_CAPABILITY_MAJOR => Some(c_int::max_value()),
            cu::DeviceAttribute::COMPUTE_CAPABILITY_MINOR => Some(c_int::max_value()),
            cu::DeviceAttribute::GPU_OVERLAP => Some(1),
            cu::DeviceAttribute::KERNEL_EXEC_TIMEOUT => Some(0),
            _ => None
        }
    }

    fn map_cuda_attribute(attr: cu::DeviceAttribute, props: &ze_device_properties_t) -> Option<c_int> {
        match attr {
            cu::DeviceAttribute::ASYNC_ENGINE_COUNT => Some(props.numAsyncCopyEngines as i32),
            cu::DeviceAttribute::MULTIPROCESSOR_COUNT => Some((props.numSlicesPerTile * props.numSubslicesPerSlice) as i32),
            cu::DeviceAttribute::KERNEL_EXEC_TIMEOUT => Some(0),
            // FIXME
            cu::DeviceAttribute::INTEGRATED => Some(1),
            cu::DeviceAttribute::CAN_MAP_HOST_MEMORY => Some(props.unifiedMemorySupported as i32),
            _ => None
        }
    }

    pub fn get_attribute(self, pi: *mut c_int, attr: cu::DeviceAttribute) -> l0::ze_result_t {
        match self.get_device_properties() {
            Ok(props) => {
                match Device::map_cuda_attribute(attr, &props) {
                    Some(cuda_value) => {
                        unsafe { *pi = cuda_value };
                        l0::ze_result_t::ZE_RESULT_SUCCESS
                    },
                    None => l0::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_FEATURE
                }
            }
            Err(err) => err
        }
    }
}