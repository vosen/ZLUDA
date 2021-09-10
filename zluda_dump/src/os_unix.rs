use crate::cuda::CUuuid;
use std::ffi::{c_void, CStr};
use std::mem;

const NVCUDA_DEFAULT_PATH: &'static [u8] = b"/usr/lib/x86_64-linux-gnu/libcuda.so.1\0";

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

//RDI, RSI, RDX, RCX, R8, R9
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
    // Let's hope there's never more than 6 arguments
    dynasm!(ops
        ; .arch x64
        ; push rbp
        ; mov rbp, rsp
        ; push rdi
        ; push rsi
        ; push rdx
        ; push rcx
        ; push r8
        ; push r9
        ; mov rdi, QWORD guid as i64
        ; mov rsi, QWORD idx as i64
        ; mov rax, QWORD report_fn as i64
        ; call rax
        ; pop r9
        ; pop r8
        ; pop rcx
        ; pop rdx
        ; pop rsi
        ; pop rdi
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
