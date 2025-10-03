#![crate_type = "bin"]

use std::ffi::c_void;
use std::mem;
use std::env;
use std::path::PathBuf;
use std::ffi::CString;

extern "system" {
    fn LoadLibraryA(lpFileName: *const i8) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u8) -> *mut c_void;
}

fn main() {
    let current_exe = env::current_exe().unwrap();
    let mut dll = PathBuf::from(current_exe.parent().unwrap());
    dll.push("do_cuinit.dll");
    let dll_cstring = CString::new(dll.to_str().unwrap()).unwrap();
    let nvcuda = unsafe { LoadLibraryA(dll_cstring.as_ptr()) };
    let cu_init = unsafe { GetProcAddress(nvcuda, b"do_cuinit\0".as_ptr()) };
    let cu_init = unsafe { mem::transmute::<_, unsafe extern "system" fn(u32) -> u32>(cu_init) };
    unsafe { cu_init(0) };
}
