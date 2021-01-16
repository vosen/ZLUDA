use libc::{dlopen, dlsym};
use std::ffi::{c_void, CStr};

const NVCUDA_DEFAULT_PATH: &'static [u8] = b"/usr/lib/x86_64-linux-gnu/libcuda.so.1\0";

pub unsafe fn load_cuda_library() -> *mut c_void {
    dlopen(
        NVCUDA_DEFAULT_PATH.as_ptr() as *const _,
        libc::RTLD_LOCAL | libc::RTLD_NOW,
    )
}

pub unsafe fn get_proc_address(handle: *mut c_void, func: &CStr) -> *mut c_void {
    libc::dlsym(handle, name.as_ptr() as *const _)
}
