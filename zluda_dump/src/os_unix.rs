use std::ffi::{c_void, CStr};

const NVCUDA_DEFAULT_PATH: &'static [u8] = b"/usr/lib/x86_64-linux-gnu/libcuda.so.1\0";

pub fn init() {}

pub unsafe fn load_cuda_library() -> *mut c_void {
    libc::dlopen(
        NVCUDA_DEFAULT_PATH.as_ptr() as *const _,
        libc::RTLD_LOCAL | libc::RTLD_NOW,
    )
}

pub unsafe fn get_proc_address(handle: *mut c_void, func: &CStr) -> *mut c_void {
    libc::dlsym(handle, func.as_ptr() as *const _)
}

#[macro_export]
macro_rules! os_log {
    ($format:tt) => {
        {
            eprintln!($format);
        }
    };
    ($format:tt, $($obj: expr),+) => {
        {
            eprintln!($format, $($obj,)+);
        }
    };
}
