use std::{
    ffi::{c_void, CStr},
    mem,
    os::raw::c_ushort,
    ptr,
};

use std::os::windows::io::AsRawHandle;
use wchar::wch_c;
use winapi::{
    shared::minwindef::HMODULE,
    um::debugapi::OutputDebugStringA,
    um::libloaderapi::{GetProcAddress, LoadLibraryW},
};

use crate::cuda::CUuuid;

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
        let payload = detours_sys::DetourFindPayload(module, &PAYLOAD_NVCUDA_GUID, &mut size);
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
            return Some(mem::transmute(result));
        }
    }
    None
}

pub unsafe fn get_proc_address(handle: *mut c_void, func: &CStr) -> *mut c_void {
    GetProcAddress(handle as *mut _, func.as_ptr()) as *mut _
}

#[macro_export]
macro_rules! os_log {
    ($format:tt) => {
        {
            use crate::os::__log_impl;
            __log_impl(format!($format));
        }
    };
    ($format:tt, $($obj: expr),+) => {
        {
            use crate::os::__log_impl;
            __log_impl(format!($format, $($obj,)+));
        }
    };
}

pub fn __log_impl(s: String) {
    let log_to_stderr = std::io::stderr().as_raw_handle() != ptr::null_mut();
    if log_to_stderr {
        eprintln!("[ZLUDA_DUMP] {}\n", s);
    } else {
        let mut win_str = String::with_capacity("[ZLUDA_DUMP] ".len() + s.len() + 2);
        win_str.push_str("[ZLUDA_DUMP] ");
        win_str.push_str(&s);
        win_str.push_str("\n\0");
        unsafe { OutputDebugStringA(win_str.as_ptr() as *const _) };
    }
}

#[cfg(target_arch = "x86")]
pub fn get_thunk(
    original_fn: *const c_void,
    report_fn: unsafe extern "system" fn(*const CUuuid, usize),
    guid: *const CUuuid,
    idx: usize,
) -> *const c_void {
    use dynasmrt::{dynasm, DynasmApi};
    let mut ops = dynasmrt::x86::Assembler::new().unwrap();
    let start = ops.offset();
    dynasm!(ops
        ; .arch x86
        ; push idx as i32
        ; push guid as i32
        ; mov eax, report_fn as i32
        ; call eax
        ; mov eax, original_fn as i32
        ; jmp eax
        ; int 3
    );
    let exe_buf = ops.finalize().unwrap();
    let result_fn = exe_buf.ptr(start);
    mem::forget(exe_buf);
    result_fn as *const _
}

//RCX, RDX, R8, R9
#[cfg(target_arch = "x86_64")]
pub fn get_thunk(
    original_fn: *const c_void,
    report_fn: unsafe extern "system" fn(*const CUuuid, usize),
    guid: *const CUuuid,
    idx: usize,
) -> *const c_void {
    use dynasmrt::{dynasm, DynasmApi};
    let mut ops = dynasmrt::x86::Assembler::new().unwrap();
    let start = ops.offset();
    // Let's hope there's never more than 4 arguments
    dynasm!(ops
        ; .arch x64
        ; push rbp
        ; mov rbp, rsp
        ; push rcx
        ; push rdx
        ; push r8
        ; push r9
        ; mov rcx, QWORD guid as i64
        ; mov rdx, QWORD idx as i64
        ; mov rax, QWORD report_fn as i64
        ; call rax
        ; pop r9
        ; pop r8
        ; pop rdx
        ; pop rcx
        ; mov rax, QWORD original_fn as i64
        ; call rax
        ; pop rbp
        ; ret
        ; int 3
    );
    let exe_buf = ops.finalize().unwrap();
    let result_fn = exe_buf.ptr(start);
    mem::forget(exe_buf);
    result_fn as *const _
}
