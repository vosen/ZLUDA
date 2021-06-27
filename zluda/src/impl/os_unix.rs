use std::ptr;

unsafe fn heap_create() -> *mut c_void {
    ptr::null_mut()
}

unsafe fn heap_alloc(heap: *mut c_void, bytes: usize) -> *mut c_void {
    todo!()
}

unsafe fn heap_free(heap: *mut c_void, alloc: *mut c_void) {
    todo!()
}
