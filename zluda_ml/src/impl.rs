use std::{
    os::raw::{c_char, c_uint},
    ptr,
};

use crate::nvml::nvmlReturn_t;

const VERSION: &'static [u8] = b"418.40.04";

macro_rules! stringify_nmvlreturn_t {
    ($x:ident => [ $($variant:ident),+ ]) => {
        match $x {
            $(
                nvmlReturn_t::$variant => Some(concat!(stringify!($variant), "\0")),
            )+
            _ => None
        }
    }
}

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> nvmlReturn_t {
    nvmlReturn_t::NVML_ERROR_NOT_SUPPORTED
}

pub(crate) fn error_string(result: nvmlReturn_t) -> *const ::std::os::raw::c_char {
    let text = stringify_nmvlreturn_t!(
        result => [
            NVML_SUCCESS,
            NVML_ERROR_UNINITIALIZED,
            NVML_ERROR_INVALID_ARGUMENT,
            NVML_ERROR_NOT_SUPPORTED,
            NVML_ERROR_NO_PERMISSION,
            NVML_ERROR_ALREADY_INITIALIZED,
            NVML_ERROR_NOT_FOUND,
            NVML_ERROR_INSUFFICIENT_SIZE,
            NVML_ERROR_INSUFFICIENT_POWER,
            NVML_ERROR_DRIVER_NOT_LOADED,
            NVML_ERROR_TIMEOUT,
            NVML_ERROR_IRQ_ISSUE,
            NVML_ERROR_LIBRARY_NOT_FOUND,
            NVML_ERROR_FUNCTION_NOT_FOUND,
            NVML_ERROR_CORRUPTED_INFOROM,
            NVML_ERROR_GPU_IS_LOST,
            NVML_ERROR_RESET_REQUIRED,
            NVML_ERROR_OPERATING_SYSTEM,
            NVML_ERROR_LIB_RM_VERSION_MISMATCH,
            NVML_ERROR_IN_USE,
            NVML_ERROR_MEMORY,
            NVML_ERROR_NO_DATA,
            NVML_ERROR_VGPU_ECC_NOT_SUPPORTED,
            NVML_ERROR_INSUFFICIENT_RESOURCES,
            NVML_ERROR_UNKNOWN
        ]
    );
    match text {
        Some(text) => text.as_ptr() as *const _,
        None => ptr::null(),
    }
}

pub(crate) fn shutdown() -> nvmlReturn_t {
    nvmlReturn_t::NVML_SUCCESS
}

static mut DEVICE: Option<ocl_core::DeviceId> = None;

pub(crate) fn init() -> Result<(), nvmlReturn_t> {
    let platforms = ocl_core::get_platform_ids()?;
    let device = platforms.iter().find_map(|plat| {
        let devices = ocl_core::get_device_ids(plat, Some(ocl_core::DeviceType::GPU), None).ok()?;
        for dev in devices {
            let vendor = ocl_core::get_device_info(dev, ocl_core::DeviceInfo::VendorId).ok()?;
            match vendor {
                ocl_core::DeviceInfoResult::VendorId(0x8086)
                | ocl_core::DeviceInfoResult::VendorId(0x1002) => {}
                _ => continue,
            };
            let dev_type = ocl_core::get_device_info(dev, ocl_core::DeviceInfo::Type).ok()?;
            if let ocl_core::DeviceInfoResult::Type(ocl_core::DeviceType::GPU) = dev_type {
                return Some(dev);
            }
        }
        None
    });
    unsafe { DEVICE = device };
    if device.is_some() {
        Ok(())
    } else {
        Err(nvmlReturn_t::NVML_ERROR_UNKNOWN)
    }
}

pub(crate) fn init_with_flags() -> Result<(), nvmlReturn_t> {
    init()
}

impl From<ocl_core::Error> for nvmlReturn_t {
    fn from(_: ocl_core::Error) -> Self {
        nvmlReturn_t::NVML_ERROR_UNKNOWN
    }
}

impl From<Result<(), nvmlReturn_t>> for nvmlReturn_t {
    fn from(result: Result<(), nvmlReturn_t>) -> Self {
        match result {
            Ok(()) => nvmlReturn_t::NVML_SUCCESS,
            Err(e) => e,
        }
    }
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

pub(crate) unsafe fn system_get_driver_version(
    version_ptr: *mut c_char,
    length: c_uint,
) -> Result<(), nvmlReturn_t> {
    if version_ptr == ptr::null_mut() || length == 0 {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let strlen = usize::min(VERSION.len(), (length as usize) - 1);
    std::ptr::copy_nonoverlapping(VERSION.as_ptr(), version_ptr as _, strlen);
    *version_ptr.add(strlen) = 0;
    Ok(())
}
