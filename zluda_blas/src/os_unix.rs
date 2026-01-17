use std::{mem, path::PathBuf};

#[link(name = "dl")]
extern "C" {
    fn dladdr(addr: *mut core::ffi::c_void, info: *mut DlInfo) -> core::ffi::c_int;
}

#[repr(C)]
struct DlInfo {
    dli_fname: *const core::ffi::c_char,
    dli_fbase: *mut core::ffi::c_void,
    dli_sname: *const core::ffi::c_char,
    dli_saddr: *mut core::ffi::c_void,
}

pub fn self_path() -> Option<PathBuf> {
    use std::ffi::CStr;
    unsafe {
        let mut info: DlInfo = mem::zeroed();
        let result = dladdr(self_path as *mut core::ffi::c_void, &mut info);
        if result == 0 || info.dli_fname.is_null() {
            return None;
        }
        let c_str = CStr::from_ptr(info.dli_fname);
        let path_str = c_str.to_str().ok()?;
        Some(PathBuf::from(path_str))
    }
}
