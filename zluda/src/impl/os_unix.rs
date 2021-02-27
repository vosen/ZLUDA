use std::ffi::c_void;

pub unsafe fn heap_create() -> *mut c_void {
    usize::MAX as *mut _
}

#[cfg(test)]
pub unsafe fn load_cuda() -> *mut c_void {
    use libc;
    use std::ffi::CStr;

    let result = libc::dlopen(
        b"/usr/lib/x86_64-linux-gnu/libcuda.so.1\0".as_ptr() as _,
        libc::RTLD_LOCAL | libc::RTLD_LAZY,
    );
    if result == std::ptr::null_mut() {
        panic!("{}", CStr::from_ptr(libc::dlerror()).to_string_lossy());
    }
    result
}

#[cfg(test)]
pub unsafe fn get_proc_address(handle: *mut c_void, func: &[u8]) -> *mut c_void {
    use libc;
    libc::dlsym(handle, func.as_ptr() as *const _)
}
