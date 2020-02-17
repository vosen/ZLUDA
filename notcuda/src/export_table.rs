use super::cu;

use std::mem;
use std::ptr;
use std::os::raw::{c_int, c_ulong};

#[no_mangle]
pub unsafe extern "C" fn cuGetExportTable(
    table: *mut *const std::os::raw::c_void,
    id: *const cu::Uuid,
) -> cu::Result {
    if *id == CU_ETID_ToolsRuntimeCallbackHooks {
        *table = TABLE0.as_ptr() as *const _;
    } else if *id == CU_ETID_CudartInterface {
        *table = TABLE1.as_ptr() as *const _;
    }
    return cu::Result::SUCCESS;
}

const CU_ETID_ToolsRuntimeCallbackHooks: cu::Uuid = cu::Uuid {
    x: [
        0xa0, 0x94, 0x79, 0x8c, 0x2e, 0x74, 0x2e, 0x74, 0x93, 0xf2, 0x08, 0x00, 0x20, 0x0c, 0x0a,
        0x66,
    ],
};
#[repr(C)]
union PtrOrLength {
    ptr: *const (),
    length: usize,
}
unsafe impl Sync for PtrOrLength {}
const TABLE0_LEN: usize = 7;
static TABLE0: [PtrOrLength; TABLE0_LEN] = [
    PtrOrLength {
        length: mem::size_of::<[PtrOrLength; TABLE0_LEN]>(),
    },
    PtrOrLength { ptr: ptr::null() },
    PtrOrLength {
        ptr: table0_fn1 as *const (),
    },
    PtrOrLength { ptr: ptr::null() },
    PtrOrLength { ptr: ptr::null() },
    PtrOrLength { ptr: ptr::null() },
    PtrOrLength {
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

const TABLE1_LEN: usize = 3;
static TABLE1: [PtrOrLength; TABLE1_LEN] = [
    PtrOrLength {
        length: mem::size_of::<[PtrOrLength; TABLE1_LEN]>(),
    },
    PtrOrLength { ptr: ptr::null() },
    PtrOrLength {
        ptr: table1_fn1 as *const (),
    },
];

unsafe extern "C" fn table1_fn1(_: *mut c_ulong, _: c_int) -> c_int {
    0
}