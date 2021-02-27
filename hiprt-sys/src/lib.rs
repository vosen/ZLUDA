#![allow(warnings)]
pub mod hiprt;
pub use hiprt::*;

use std::ffi::OsString;
use std::mem;
use std::path::PathBuf;

impl hiprt::HipRt {
    pub unsafe fn load() -> Result<Self, libloading::Error> {
        Self::new(os::HIPRT_NAME).or_else(|_| {
            let module_path = os::currently_executing().ok_or(os::UNKNOWN_ERROR)?;
            let mut path = PathBuf::from(module_path);
            if !path.pop() {
                return Err(os::UNKNOWN_ERROR);
            }
            path.push(os::HIPRT_NAME);
            Self::new(path)
        })
    }
}

#[cfg(not(target_os = "windows"))]
mod os {
    use std::ffi::CStr;
    use std::os::unix::ffi::OsStringExt;
    use std::{
        ffi::{c_char, c_int, c_void, CString, OsString},
        mem,
    };

    pub(crate) const HIPRT_NAME: &'static str = "libhiprt64.so";
    pub(crate) const UNKNOWN_ERROR: libloading::Error = libloading::Error::DlOpenUnknown;

    #[link(name = "dl")]
    extern "C" {
        fn dladdr(addr: *mut c_void, info: *mut DlInfo) -> c_int;
    }

    pub(crate) unsafe fn currently_executing() -> Option<OsString> {
        let mut dlinfo = mem::zeroed();
        if 0 == dladdr(currently_executing as _, &mut dlinfo) {
            return None;
        }
        Some(OsString::from_vec(
            CStr::from_ptr(dlinfo.dli_fname.cast_mut())
                .to_bytes()
                .to_vec(),
        ))
    }

    #[repr(C)]
    struct DlInfo {
        dli_fname: *const c_char,
        dli_fbase: *mut c_void,
        dli_sname: *const c_char,
        dli_saddr: *mut c_void,
    }
}

#[cfg(target_os = "windows")]
mod os {
    use std::ffi::OsString;
    use std::mem;
    use widestring::U16CStr;
    use winapi::shared::minwindef::HMODULE;
    use winapi::um::libloaderapi::*;

    pub(crate) const HIPRT_NAME: &'static str = "hiprt64.dll";
    pub(crate) const UNKNOWN_ERROR: libloading::Error =
        libloading::Error::GetModuleHandleExWUnknown;

    pub(crate) unsafe fn currently_executing() -> Option<OsString> {
        let mut module = mem::zeroed();
        if 0 == GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            currently_executing as _,
            &mut module,
        ) {
            return None;
        }
        Some(U16CStr::from_ptr_str(module.cast()).to_os_string())
    }
}
