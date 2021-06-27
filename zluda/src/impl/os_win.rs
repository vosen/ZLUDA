use std::ffi::c_void;

use winapi::um::heapapi::{HeapAlloc, HeapFree};
use winapi::um::{heapapi::HeapCreate, winnt::HEAP_NO_SERIALIZE};

pub unsafe fn heap_create() -> *mut c_void {
    HeapCreate(HEAP_NO_SERIALIZE, 0, 0)
}

pub unsafe fn heap_alloc(heap: *mut c_void, bytes: usize) -> *mut c_void {
    HeapAlloc(heap, 0, bytes)
}

pub unsafe fn heap_free(heap: *mut c_void, alloc: *mut c_void) {
    HeapFree(heap, 0, alloc);
}
