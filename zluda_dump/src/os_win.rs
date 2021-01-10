use std::ffi::{c_void, CStr};

use wchar::wch_c;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryW};

const NVCUDA_DEFAULT_PATH: &[u16] = wch_c!(r"C:\Windows\System32\nvcuda.dll");

pub unsafe fn load_cuda_library() -> *mut c_void {
    LoadLibraryW(NVCUDA_DEFAULT_PATH.as_ptr()) as *mut _
}

pub unsafe fn get_proc_address(handle: *mut c_void, func: &CStr) -> *mut c_void {
    GetProcAddress(handle as *mut _, func.as_ptr()) as *mut _
}
