use cuda_types::cuda::CUuuid;
use std::ffi::{c_void, CStr, CString};
use std::mem;

pub(crate) const LIBCUDA_DEFAULT_PATH: &str = "/usr/lib/x86_64-linux-gnu/libcuda.so.1";

pub unsafe fn load_library(libcuda_path: &str) -> *mut c_void {
    let libcuda_path = CString::new(libcuda_path).unwrap();
    libc::dlopen(
        libcuda_path.as_ptr() as *const _,
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
            eprintln!("[ZLUDA_DUMP] {}", format!($format));
        }
    };
    ($format:tt, $($obj: expr),+) => {
        {
            eprintln!("[ZLUDA_DUMP] {}", format!($format, $($obj,)+));
        }
    };
}

//RDI, RSI, RDX, RCX, R8, R9
#[cfg(target_arch = "x86_64")]
pub fn get_thunk(
    original_fn: *const c_void,
    report_fn: unsafe extern "system" fn(&CUuuid, usize),
    guid: *const CUuuid,
    idx: usize,
) -> *const c_void {
    use dynasmrt::{dynasm, DynasmApi};
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let start = ops.offset();
    dynasm!(ops
        // stack alignment
        ; sub rsp, 8
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
        ; add rsp, 8
        ; mov rax, QWORD original_fn as i64
        ; jmp rax
        ; int 3
    );
    let exe_buf = ops.finalize().unwrap();
    let result_fn = exe_buf.ptr(start);
    mem::forget(exe_buf);
    result_fn as *const _
}

#[link(name = "pthread")]
unsafe extern "C" {
    fn pthread_self() -> std::os::unix::thread::RawPthread;
}

pub(crate) fn current_thread() -> u32 {
    (unsafe { pthread_self() }) as u32
}
