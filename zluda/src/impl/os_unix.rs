use std::ptr;

pub unsafe fn heap_create() -> *mut c_void {
    ptr::null_mut()
}

pub unsafe fn heap_alloc(heap: *mut c_void, bytes: usize) -> *mut c_void {
    todo!()
}

pub unsafe fn heap_free(heap: *mut c_void, alloc: *mut c_void) {
    todo!()
}
