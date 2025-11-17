use cuda_types::cuda::CUuuid;
use std::borrow::Cow;
use std::os::windows::io::AsRawHandle;
use std::ptr::NonNull;
use std::{
    ffi::{c_void, CStr},
    mem, ptr,
    sync::LazyLock,
};
use windows::core::PCSTR;
use windows::Win32::System::Diagnostics::Debug::OutputDebugStringA;

pub(crate) const LIBCUDA_DEFAULT_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";
const GET_PROC_ADDRESS_NO_REDIRECT: &'static [u8] = b"ZludaGetProcAddress_NoRedirect\0";

pub fn dlopen_local_noredirect<'a>(
    path: impl Into<Cow<'a, str>>,
) -> Result<NonNull<c_void>, libloading::Error> {
    let lib: libloading::os::windows::Library =
        zluda_trace_common::dlopen_local_noredirect(path)?.into();
    NonNull::new(lib.into_raw() as *mut _).ok_or(libloading::Error::DlOpenUnknown)
}

pub unsafe fn get_proc_address(handle: *mut c_void, func: &CStr) -> *mut c_void {
    use libloading::os::windows;
    static ZLUDA_REDIRECT: LazyLock<
        Option<extern "system" fn(*mut c_void, *const i8) -> *mut c_void>,
    > = LazyLock::new(|| {
        let lib = windows::Library::open_already_loaded("zluda_redirect").ok()?;
        unsafe {
            lib.get::<extern "system" fn(*mut c_void, *const i8) -> *mut c_void>(
                GET_PROC_ADDRESS_NO_REDIRECT,
            )
        }
        .ok()
        .map(|symbol| *symbol)
    });
    unwrap_or::unwrap_some_or!(&*ZLUDA_REDIRECT, return ptr::null_mut())(handle, func.as_ptr())
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
        eprintln!("[ZLUDA_TRACE] {}", s);
    } else {
        let mut win_str = String::with_capacity("[ZLUDA_TRACE] ".len() + s.len() + 2);
        win_str.push_str("[ZLUDA_TRACE] ");
        win_str.push_str(&s);
        win_str.push_str("\n\0");
        unsafe { OutputDebugStringA(PCSTR(win_str.as_ptr() as *const _)) };
    }
}

#[cfg(target_arch = "x86")]
pub fn get_thunk(
    original_fn: *const c_void,
    report_fn: unsafe extern "system" fn(&CUuuid, usize),
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
    report_fn: unsafe extern "system" fn(&CUuuid, usize),
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

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentThreadId() -> u32;
}

pub(crate) fn current_thread() -> u32 {
    unsafe { GetCurrentThreadId() }
}
