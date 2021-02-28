use level_zero as l0;
use std::io::Write;
use std::{
    os::raw::{c_char, c_uint},
    ptr,
};

use crate::nvml::nvmlReturn_t;

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

pub(crate) fn init_v2() -> Result<(), nvmlReturn_t> {
    Ok(l0::init()?)
}

pub(crate) fn init_with_flags() -> Result<(), nvmlReturn_t> {
    init_v2()
}

impl From<l0::sys::ze_result_t> for nvmlReturn_t {
    fn from(l0_err: l0::sys::ze_result_t) -> Self {
        match l0_err {
            l0::sys::ze_result_t::ZE_RESULT_ERROR_UNINITIALIZED => {
                nvmlReturn_t::NVML_ERROR_UNINITIALIZED
            }
            _ => nvmlReturn_t::NVML_ERROR_UNKNOWN,
        }
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

pub(crate) fn system_get_driver_version(
    version_ptr: *mut c_char,
    length: c_uint,
) -> Result<(), nvmlReturn_t> {
    if version_ptr == ptr::null_mut() {
        return Err(nvmlReturn_t::NVML_ERROR_INVALID_ARGUMENT);
    }
    let drivers = l0::Driver::get()?;
    let output_slice =
        unsafe { std::slice::from_raw_parts_mut(version_ptr as *mut u8, (length - 1) as usize) };
    let mut output_write = CountingWriter {
        base: output_slice,
        len: 0,
    };
    for d in drivers {
        let props = d.get_properties()?;
        let driver_version = props.driverVersion;
        write!(&mut output_write, "{}", driver_version)
            .map_err(|_| nvmlReturn_t::NVML_ERROR_UNKNOWN)?;
        unsafe { *(version_ptr.add(output_write.len)) = 0 };
        return Ok(());
    }
    Err(nvmlReturn_t::NVML_ERROR_UNKNOWN)
}
