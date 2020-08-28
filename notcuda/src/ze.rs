use super::cu;
use crate::cuda;

use std::cmp;
use std::os::raw::{c_char, c_int};

pub struct Device {
    pub base: l0::Device,
    pub default_queue: l0::CommandQueue,
    properties: Option<Box<l0::sys::ze_device_properties_t>>,
    image_properties: Option<Box<l0::sys::ze_device_image_properties_t>>,
    memory_properties: Option<Vec<l0::sys::ze_device_memory_properties_t>>,
}

impl Device {
    pub fn new(ctx: &mut l0::Context, d: l0::Device) -> l0::Result<Self> {
        let queue = l0::CommandQueue::new(ctx, &d)?;
        Ok(Self {
            base: d,
            default_queue: queue,
            properties: None,
            image_properties: None,
            memory_properties: None,
        })
    }

    fn get_properties<'a>(&'a mut self) -> l0::Result<&'a l0::sys::ze_device_properties_t> {
        if let Some(ref prop) = self.properties {
            return Ok(prop);
        }
        match self.base.get_properties() {
            Ok(prop) => Ok(self.properties.get_or_insert(prop)),
            Err(e) => Err(e),
        }
    }

    fn get_image_properties(&mut self) -> l0::Result<&l0::sys::ze_device_image_properties_t> {
        if let Some(ref prop) = self.image_properties {
            return Ok(prop);
        }
        match self.base.get_image_properties() {
            Ok(prop) => Ok(self.image_properties.get_or_insert(prop)),
            Err(e) => Err(e),
        }
    }

    fn get_memory_properties(&mut self) -> l0::Result<&[l0::sys::ze_device_memory_properties_t]> {
        if let Some(ref prop) = self.memory_properties {
            return Ok(prop);
        }
        match self.base.get_memory_properties() {
            Ok(prop) => Ok(self.memory_properties.get_or_insert(prop)),
            Err(e) => Err(e),
        }
    }

    pub fn get_name(&mut self, name: *mut c_char, len: c_int) -> l0::Result<()> {
        let props = self.get_properties()?;
        let null_pos = props.name.iter().position(|&c| c == 0).unwrap_or(0);
        let dst_null_pos = cmp::min((len - 1) as usize, null_pos);
        unsafe { *(name.add(dst_null_pos)) = 0 };
        unsafe { std::ptr::copy_nonoverlapping(props.name.as_ptr(), name, dst_null_pos) };
        Ok(())
    }

    pub fn total_mem(&mut self, bytes: *mut usize) -> l0::Result<()> {
        let props = self.get_memory_properties()?;
        let max_mem = props.iter().map(|p| p.totalSize).max().unwrap();
        unsafe { *bytes = max_mem as usize };
        Ok(())
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

    fn get_attribute_general(
        attr: cu::DeviceGeneralAttribute,
        props: &l0_sys::ze_device_properties_t,
    ) -> c_int {
        match attr {
            cu::DeviceGeneralAttribute::CAN_MAP_HOST_MEMORY => 1,
            cu::DeviceGeneralAttribute::ASYNC_ENGINE_COUNT => props.maxHardwareContexts as i32,
            cu::DeviceGeneralAttribute::MULTIPROCESSOR_COUNT => {
                (props.numSlices * props.numSubslicesPerSlice * props.numEUsPerSubslice) as i32
            }
        }
    }

    fn get_attribute_texture(
        attr: cu::DeviceTextureAttribute,
        props: &l0_sys::ze_device_image_properties_t,
    ) -> c_int {
        match attr {
            cu::DeviceTextureAttribute::MAXIMUM_TEXTURE1D_WIDTH => {
                cmp::min(props.maxImageDims1D, c_int::max_value() as u32) as c_int
            }
        }
    }

    pub fn get_attribute(
        &mut self,
        pi: *mut c_int,
        attr: cu::DeviceDynamicAttribute,
    ) -> l0::Result<()> {
        let value = match attr {
            cu::DeviceDynamicAttribute::General(a) => {
                Device::get_attribute_general(a, self.get_properties()?)
            }
            cu::DeviceDynamicAttribute::Texture(a) => {
                Device::get_attribute_texture(a, self.get_image_properties()?)
            }
        };
        unsafe { *pi = value };
        Ok(())
    }

    pub fn get_uuid(&mut self, uuid: *mut cu::Uuid) -> l0::Result<()> {
        let props = self.get_properties()?;
        unsafe { *uuid = cu::Uuid { x: props.uuid.id } };
        Ok(())
    }
}

pub struct Context {
    pub cuda_manager: *mut cuda::rt::ContextStateManager,
    pub cuda_state: *mut cuda::rt::ContextState,
    pub cuda_dtor_cb:
        extern "C" fn(cu::Context, *mut cuda::rt::ContextStateManager, *mut cuda::rt::ContextState),
}

pub struct Module {
    pub ptx_text: Vec<u8>,
}
