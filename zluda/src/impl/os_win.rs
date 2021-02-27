use std::ffi::c_void;

use winapi::um::{heapapi::HeapCreate, winnt::HEAP_NO_SERIALIZE};

pub unsafe fn heap_create() -> *mut c_void {
    HeapCreate(HEAP_NO_SERIALIZE, 0, 0)
}
