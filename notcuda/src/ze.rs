use level_zero_sys::*;

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


