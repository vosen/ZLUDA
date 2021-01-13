use std::{
    ffi::{c_void, CStr},
    mem,
    os::raw::c_ushort,
    ptr,
};

use wchar::wch_c;
use winapi::{
    shared::minwindef::HMODULE,
    um::libloaderapi::{GetProcAddress, LoadLibraryW},
};

const NVCUDA_DEFAULT_PATH: &[u16] = wch_c!(r"C:\Windows\System32\nvcuda.dll");
const LOAD_LIBRARY_NO_REDIRECT: &'static [u8] = b"ZludaLoadLibraryW_NoRedirect\0";

include!("../../zluda_redirect/src/payload_guid.rs");

pub unsafe fn load_cuda_library() -> *mut c_void {
    let load_lib = if is_detoured() {
        match get_non_detoured_load_library() {
            Some(load_lib) => load_lib,
            None => return ptr::null_mut(),
        }
    } else {
        LoadLibraryW
    };
    load_lib(NVCUDA_DEFAULT_PATH.as_ptr()) as *mut _
}

unsafe fn is_detoured() -> bool {
    let mut module = ptr::null_mut();
    loop {
        module = detours_sys::DetourEnumerateModules(module);
        if module == ptr::null_mut() {
            break;
        }
        let mut size = 0;
        let payload = detours_sys::DetourFindPayload(module, &PAYLOAD_GUID, &mut size);
        if payload != ptr::null_mut() {
            return true;
        }
    }
    false
}

unsafe fn get_non_detoured_load_library(
) -> Option<unsafe extern "system" fn(*const c_ushort) -> HMODULE> {
    let mut module = ptr::null_mut();
    loop {
        module = detours_sys::DetourEnumerateModules(module);
        if module == ptr::null_mut() {
            break;
        }
        let result = GetProcAddress(
            module as *mut _,
            LOAD_LIBRARY_NO_REDIRECT.as_ptr() as *mut _,
        );
        if result != ptr::null_mut() {
            eprintln!("!!!!hit");
            return Some(mem::transmute(result));
        }
        eprintln!("!!!!mis");
    }
    None
}

/*
unsafe extern "C" fn get_non_detoured_load_library_callback(
    context: *mut ::std::os::raw::c_void,
    _: ::std::os::raw::c_ulong,
    name: *const ::std::os::raw::c_char,
    code: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    let mut i = 0;
    loop {
        if i == LOAD_LIBRARY_NO_REDIRECT.len() {
            *(context as *mut *mut c_void) = code;
            return 0;
        }
        if *name.add(i) as u8 != LOAD_LIBRARY_NO_REDIRECT[i] {
            return 1;
        }
        i += 1;
    }
}
 */

pub unsafe fn get_proc_address(handle: *mut c_void, func: &CStr) -> *mut c_void {
    GetProcAddress(handle as *mut _, func.as_ptr()) as *mut _
}
