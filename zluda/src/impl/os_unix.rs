use std::ffi::c_void;

pub unsafe fn heap_create() -> *mut c_void {
    usize::MAX as *mut _
}

pub unsafe fn heap_alloc(_heap: *mut c_void, _bytes: usize) -> *mut c_void {
    todo!()
}

pub unsafe fn heap_free(_heap: *mut c_void, _alloc: *mut c_void) {
    todo!()
}
