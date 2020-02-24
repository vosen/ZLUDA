use super::cu;

use std::mem;
use std::ptr;
use std::os::raw::{c_int, c_ulong};

#[no_mangle]
pub unsafe extern "C" fn cuGetExportTable(
    table: *mut *const std::os::raw::c_void,
    id: *const cu::Uuid,
) -> cu::Result {
    if table == ptr::null_mut() || id == ptr::null_mut() {
        cu::Result::ERROR_INVALID_VALUE
    } else if *id == CU_ETID_ToolsRuntimeCallbackHooks {
        *table = TABLE0.as_ptr() as *const _;
        cu::Result::SUCCESS
    } else if *id == CU_ETID_CudartInterface {
        *table = TABLE1.as_ptr() as *const _;
        cu::Result::SUCCESS
    } else if *id == CU_ETID_ToolsTls {
        *table = 1 as _;
        cu::Result::SUCCESS
    } else if *id == CU_ETID_ContextLocalStorageInterface_v0301 {
        *table = ContextLocalStorageInterface_v0301_VTABLE.as_ptr() as *const _;
        cu::Result::SUCCESS
    } else {
        cu::Result::ERROR_NOT_SUPPORTED
    }
}

const CU_ETID_ToolsRuntimeCallbackHooks: cu::Uuid = cu::Uuid {
    x: [
        0xa0, 0x94, 0x79, 0x8c, 0x2e, 0x74, 0x2e, 0x74, 0x93, 0xf2, 0x08, 0x00, 0x20, 0x0c, 0x0a,
        0x66,
    ],
};
#[repr(C)]
union VTableEntry {
    ptr: *const (),
    length: usize,
}
unsafe impl Sync for VTableEntry {}
const TABLE0_LEN: usize = 7;
static TABLE0: [VTableEntry; TABLE0_LEN] = [
    VTableEntry {
        length: mem::size_of::<[VTableEntry; TABLE0_LEN]>(),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry {
        ptr: table0_fn1 as *const (),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
    VTableEntry {
        ptr: table0_fn5 as *const (),
    },
];
static mut TABLE0_FN1_SPACE: [u8; 512] = [0; 512];
static mut TABLE0_FN5_SPACE: [u8; 2] = [0; 2];

unsafe extern "C" fn table0_fn1(ptr: *mut *mut u8, size: *mut usize) -> *mut u8 {
    *ptr = TABLE0_FN1_SPACE.as_mut_ptr();
    *size = TABLE0_FN1_SPACE.len();
    return TABLE0_FN1_SPACE.as_mut_ptr();
}

unsafe extern "C" fn table0_fn5(ptr: *mut *mut u8, size: *mut usize) -> *mut u8 {
    *ptr = TABLE0_FN5_SPACE.as_mut_ptr();
    *size = TABLE0_FN5_SPACE.len();
    return TABLE0_FN5_SPACE.as_mut_ptr();
}

const CU_ETID_CudartInterface: cu::Uuid = cu::Uuid {
    x: [
        0x6b, 0xd5, 0xfb, 0x6c, 0x5b, 0xf4, 0xe7, 0x4a, 0x89, 0x87, 0xd9, 0x39, 0x12, 0xfd, 0x9d,
        0xf9
    ],
};

const TABLE1_LEN: usize = 10;
static TABLE1: [VTableEntry; TABLE1_LEN] = [
    VTableEntry {
        length: mem::size_of::<[VTableEntry; TABLE1_LEN]>(),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry {
        ptr: table1_fn1 as *const (),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
    VTableEntry {
        ptr: table1_fn6 as *const (),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
];

unsafe extern "C" fn table1_fn1(_: *mut c_ulong, _: c_int) -> c_int {
    0
}

unsafe extern "C" fn table1_fn6(_: u64) { }

const CU_ETID_ToolsTls: cu::Uuid = cu::Uuid {
    x: [0x42, 0xd8, 0x5a, 0x81, 0x23, 0xf6, 0xcb, 0x47, 0x82, 0x98, 0xf6, 0xe7, 0x8a, 0x3a, 0xec, 0xdc],
};


const CU_ETID_ContextLocalStorageInterface_v0301: cu::Uuid = cu::Uuid {
    x: [0xc6, 0x93, 0x33, 0x6e, 0x11, 0x21, 0xdf, 0x11, 0xa8, 0xc3, 0x68, 0xf3, 0x55, 0xd8, 0x95, 0x93],
};

// the table is much bigger and start earlier
static ContextLocalStorageInterface_v0301_VTABLE: [VTableEntry; 4] = [
    VTableEntry { ptr: ContextLocalStorageInterface_v0301_VTABLE_fn0 as *const () },
    VTableEntry { ptr: ContextLocalStorageInterface_v0301_VTABLE_fn1 as *const () },
    VTableEntry { ptr: ContextLocalStorageInterface_v0301_VTABLE_fn2 as *const () },
    VTableEntry { ptr: ptr::null() },
];

// some kind of ctor
unsafe extern "C" fn ContextLocalStorageInterface_v0301_VTABLE_fn0(ms: *mut usize, _: *mut (), _: *mut (), _: *mut ()) -> u32 {
    0
}

// some kind of dtor
unsafe extern "C" fn ContextLocalStorageInterface_v0301_VTABLE_fn1(ms: *mut usize, _: *mut ()) -> u32 {
    0
}

unsafe extern "C" fn ContextLocalStorageInterface_v0301_VTABLE_fn2(_: *mut *mut (), _: *mut (), _: *mut ()) -> u32 {
    0
}