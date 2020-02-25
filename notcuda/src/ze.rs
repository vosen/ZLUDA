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

fn either<T>(r: Result<T, T>) -> T {
    match r {
        Ok(x) => x,
        Err(x) => x
    }
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

impl Versioned for ze_device_image_properties_t {
    type Version = ze_device_image_properties_version_t;
    fn current() -> Self::Version {
        ze_device_image_properties_version_t::ZE_DEVICE_IMAGE_PROPERTIES_VERSION_CURRENT
    }
    fn version(&mut self) -> &mut Self::Version {
        &mut self.version
    }
}

impl Versioned for ze_device_mem_alloc_desc_t {
    type Version = ze_device_mem_alloc_desc_version_t;
    fn current() -> Self::Version {
        ze_device_mem_alloc_desc_version_t::ZE_DEVICE_MEM_ALLOC_DESC_VERSION_CURRENT
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

    fn get_device_image_properties(self) -> Result<Box<ze_device_image_properties_t>, ze_result_t> {
        let mut props = Box::new(l0::ze_device_image_properties_t::new());
        l0_check_err! { l0::zeDeviceGetImageProperties(self.0, props.as_mut()) };
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

    pub fn get_attribute_static(attr: cu::DeviceStaticAttribute) -> c_int {
        match attr {
            cu::DeviceStaticAttribute::GPU_OVERLAP => 1,
            cu::DeviceStaticAttribute::KERNEL_EXEC_TIMEOUT => 0,
            cu::DeviceStaticAttribute::INTEGRATED => 1,
            cu::DeviceStaticAttribute::COMPUTE_CAPABILITY_MAJOR => c_int::max_value(),
            cu::DeviceStaticAttribute::COMPUTE_CAPABILITY_MINOR => c_int::max_value(),
        }
    }

    fn get_attribute_general(attr: cu::DeviceGeneralAttribute, props: &l0::ze_device_properties_t) -> c_int {
        match attr {
            cu::DeviceGeneralAttribute::CAN_MAP_HOST_MEMORY => props.unifiedMemorySupported as i32,
            cu::DeviceGeneralAttribute::ASYNC_ENGINE_COUNT => props.numAsyncCopyEngines as i32,
            cu::DeviceGeneralAttribute::MULTIPROCESSOR_COUNT => (props.numSlicesPerTile * props.numSubslicesPerSlice) as i32,
        }
    }

    fn get_attribute_texture(attr: cu::DeviceTextureAttribute, props: &l0::ze_device_image_properties_t) -> c_int {
        match attr {
            cu::DeviceTextureAttribute::MAXIMUM_TEXTURE1D_WIDTH => cmp::min(props.maxImageDims1D, c_int::max_value() as u32) as c_int,
        }
    }

    pub fn get_attribute(self, pi: *mut c_int, attr: cu::DeviceDynamicAttribute) -> l0::ze_result_t {
        let value_or_err = match attr {
            cu::DeviceDynamicAttribute::General(a) => self.get_device_properties().map(|p| Device::get_attribute_general(a, &p)),
            cu::DeviceDynamicAttribute::Texture(a) => self.get_device_image_properties().map(|p| Device::get_attribute_texture(a, &p)),
        };
        match value_or_err {
            Ok(value) => {
                unsafe { *pi = value };
                l0::ze_result_t::ZE_RESULT_SUCCESS
            }
            Err(e) => e
        }
    }

    pub fn get_uuid(self, uuid: *mut cu::Uuid) -> l0::ze_result_t {
        either(self.get_device_properties().map(|prop| {
            unsafe { *uuid = cu::Uuid{ x: prop.uuid.id } };
            l0::ze_result_t::ZE_RESULT_SUCCESS
        }))
    }
}