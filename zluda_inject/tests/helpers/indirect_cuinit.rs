#![crate_type = "bin"]

use std::ffi::c_void;
use std::mem;

extern "system" {
    fn LoadLibraryA(lpFileName: *const u8) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u8) -> *mut c_void;
}

fn main() {
    let nvcuda = unsafe { LoadLibraryA(b"C:\\Windows\\System32\\nvcuda.dll\0".as_ptr()) };
    let cu_init = unsafe { GetProcAddress(nvcuda, b"cuInit\0".as_ptr()) };
    let cu_init = unsafe { mem::transmute::<_, unsafe extern "system" fn(u32) -> u32>(cu_init) };
    unsafe { cu_init(0) };
}
