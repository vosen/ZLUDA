use super::cu;

use std::mem;
use std::ptr;

#[no_mangle]
pub unsafe extern "stdcall" fn cuGetExportTable(
    table: *mut *const std::os::raw::c_void,
    id: *const cu::Uuid,
) -> cu::Result {
    if *id == GUID0 {
        *table = TABLE0.as_ptr() as *const _;
    }
    return cu::Result::SUCCESS;
}

const GUID0: cu::Uuid = cu::Uuid {
    x: [
        0xa0, 0x94, 0x79, 0x8c, 0x2e, 0x74, 0x2e, 0x74, 0x93, 0xf2, 0x08, 0x00, 0x20, 0x0c, 0x0a,
        0x66,
    ],
};
#[repr(C)]
union Table0Member {
    count: usize,
    ptr: *const (),
}
unsafe impl Sync for Table0Member {}
const TABLE0_LEN: usize = 7;
static TABLE0: [Table0Member; TABLE0_LEN] = [
    Table0Member {
        count: mem::size_of::<[Table0Member; TABLE0_LEN]>(),
    },
    Table0Member { ptr: ptr::null() },
    Table0Member {
        ptr: table0_fn1 as *const (),
    },
    Table0Member { ptr: ptr::null() },
    Table0Member { ptr: ptr::null() },
    Table0Member { ptr: ptr::null() },
    Table0Member {
        ptr: table0_fn5 as *const (),
    },
];
static mut TABLE0_FN1_SPACE: [u8; 512] = [0; 512];
static mut TABLE0_FN5_SPACE: [u8; 2] = [0; 2];

unsafe extern "stdcall" fn table0_fn1(ptr: *mut *mut u8, size: *mut usize) -> *mut u8 {
    *ptr = TABLE0_FN1_SPACE.as_mut_ptr();
    *size = TABLE0_FN1_SPACE.len();
    return TABLE0_FN1_SPACE.as_mut_ptr();
}

unsafe extern "stdcall" fn table0_fn5(ptr: *mut *mut u8, size: *mut usize) -> *mut u8 {
    *ptr = TABLE0_FN5_SPACE.as_mut_ptr();
    *size = TABLE0_FN5_SPACE.len();
    return TABLE0_FN5_SPACE.as_mut_ptr();
}
