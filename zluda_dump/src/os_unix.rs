use std::ffi::{CStr, c_void};

pub unsafe fn load_cuda_library() -> *mut c_void {
    todo!()
}

pub unsafe fn get_proc_address(handle: *mut c_void, func: &CStr) -> *mut c_void {
    todo!()
}