use std::ffi::c_void;

use cuda_types::cuda::CUuuid;

pub mod fatbin;

macro_rules! dark_api_init {
    (SIZE_OF, $table_len:literal, $type_:ty) => {
        (std::mem::size_of::<usize>() * $table_len) as *const std::ffi::c_void
    };
    (NULL, $table_len:literal, $type_:ty) => {
        std::ptr::null()
    };
    ($fn_:ident, $table_len:literal, $type_:ty) => {
        <$type_>::$fn_ as *const std::ffi::c_void
    };
}

macro_rules! dark_api_fn {
    (SIZE_OF) => { };
    (NULL) => { };
    ($fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty) => {
        unsafe extern "system" fn $fn_(
            $($arg_id : $arg_type,)*
        ) -> $ret_type;
    }
}

macro_rules! dark_api_entry {
    ($idx:literal, SIZE_OF) => { };
    ($idx:literal, NULL) => { };
    ($idx:literal, $fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty) => {
        #[allow(non_snake_case)]
        pub unsafe fn $fn_(
            &self,
            $($arg_id : $arg_type,)*
        ) -> $ret_type {
            let ptr = self.ptr as *const *const std::ffi::c_void;
            let ptr = ptr.add($idx);
            let fn_ = std::mem::transmute::<_, unsafe extern "system" fn( $($arg_type,)* ) -> $ret_type >(*ptr);
            (fn_)( $($arg_id,)* )
        }
    }
}

macro_rules! dark_api_format_args {
    ($writer:ident; $arg_idx:ident; $first_arg:ident $(, $arg_id:ident)*) => {
        $writer.write_all(concat!(stringify!($first_arg), ": ").as_bytes())?;
        format::CudaDisplay::write(& $first_arg, "", $arg_idx, $writer)?;
        $(
            $arg_idx += 1;
            $writer.write_all(concat!(", ", stringify!($arg_id), ": ").as_bytes())?;
            format::CudaDisplay::write(& $arg_id, "", $arg_idx, $writer)?;
        )*
    };
    ($writer:ident; $arg_idx:ident; ) => {
    };
}

macro_rules! dark_api_is_fn {
    (SIZE_OF) => {
        false
    };
    (NULL) => {
        false
    };
    ($fn_:ident) => {
        true
    };
}

macro_rules! dark_api_format_fn {
    (SIZE_OF) => { };
    (NULL) => { };
    (#[noformat] $fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty) => { };
    ($fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty) => {
        pub fn $fn_ (
            writer: &mut (impl std::io::Write + ?Sized),
            $($arg_id: $arg_type,)*
        ) -> std::io::Result<()> {
            #[allow(unused)]
            let mut arg_idx = 0usize;
            writer.write_all(b"(")?;
            dark_api_format_args!(writer; arg_idx; $($arg_id),*);
            writer.write_all(b")")
        }
    }
}

macro_rules! dark_api {
    (
        $mod_name: ident;
        $(
            $guid:expr => $name:ident [$len:literal] {
                $(
                    $(#[$attr:ident])?
                    [$index:literal] = $fn_:ident $( ( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty )?
                ),*
            }
        ),+
    ) => {
        pub mod $mod_name {
        #[allow(non_snake_case)]
        pub struct CudaDarkApiGlobalTable {
            $(pub $name: [*const std::ffi::c_void; $len],)+
        }

        impl CudaDarkApiGlobalTable {
            $(const $name: cuda_types::cuda::CUuuid = cuda_types::cuda::CUuuid { bytes: *uuid::uuid!($guid).as_bytes() };)+
        }

        unsafe impl Sync for CudaDarkApiGlobalTable {}

        impl CudaDarkApiGlobalTable {
            pub const fn new<T: CudaDarkApi>() -> Self {
                let mut result = Self {
                    $(
                        $name: [std::ptr::null(); $len],
                    )+
                };
                $(
                    $( result.$name[$index] =  dark_api_init!($fn_, $len, T); )*
                )+
                result
            }

            pub fn get(&self, key: &cuda_types::cuda::CUuuid) -> Option<crate::DarkApiTable> {
                match key {
                    $(
                        &Self::$name => {
                            let fns = &self.$name[..];
                            let mut valid_fns = bit_vec::BitVec::from_elem($len, false);
                            $(
                                valid_fns.set($index, dark_api_is_fn!($fn_) );
                            )*
                            Some(crate::DarkApiTable {
                                fns,
                                valid_fns
                            })
                        }
                    )+
                    _ => None
                }
            }
        }

        pub trait CudaDarkApi {
            $($(
                dark_api_fn!($fn_ $( ( $($arg_id: $arg_type),* ) -> $ret_type )?);
            )*)+
        }

        pub fn guid_to_name(guid: &cuda_types::cuda::CUuuid, index: usize) -> Option<(&'static str, Option<&'static str>)> {
            let guid = uuid::Uuid::from_bytes(guid.bytes);
            $(
                if guid == uuid::uuid!($guid) {
                    let guid = stringify!($name);
                    $(
                        if index == $index {
                            return Some((guid, Some(stringify!($fn_))));
                        }
                    )*
                    return Some((guid, None));
                }
            )+
            None
        }

        $(
            paste::paste! {
                pub struct [<$name:camel>] {
                    #[allow(dead_code)]
                    ptr: *const std::ffi::c_void
                }

                impl [<$name:camel>] {
                    pub const GUID: cuda_types::cuda::CUuuid = CudaDarkApiGlobalTable::$name;

                    pub unsafe fn new(ptr: *const std::ffi::c_void) -> Self {
                        Self {
                            ptr
                        }
                    }
                    $(
                        dark_api_entry!($index, $fn_ $( ( $($arg_id: $arg_type),* ) -> $ret_type )?);
                    )*
                }
            }
        )+

        pub mod format {
            $($(
                dark_api_format_fn!($(#[$attr])? $fn_ $( ( $($arg_id: $arg_type),* ) -> $ret_type )? );
            )*)+
        }
        }
    };
}

pub struct DarkApiTable<'a> {
    fns: &'a [*const std::ffi::c_void],
    valid_fns: bit_vec::BitVec,
}

impl<'a> DarkApiTable<'a> {
    pub fn len(&self) -> usize {
        self.fns.len()
    }

    pub fn get_fn(&self, idx: usize) -> Option<*const std::ffi::c_void> {
        if self.valid_fns.get(idx).unwrap_or(false) {
            Some(self.fns[idx])
        } else {
            None
        }
    }

    pub fn start(&self) -> *const std::ffi::c_void {
        self.fns.as_ptr().cast()
    }
}

dark_api! {
    cuda;
    "{6BD5FB6C-5BF4-E74A-8987-D93912FD9DF9}" => CUDART_INTERFACE[10] {
        [0] = SIZE_OF,
        [1] = get_module_from_cubin(
            module: *mut cuda_types::cuda::CUmodule,
            fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper
        ) -> (),
        [2] = cudart_interface_fn2(
            pctx: *mut cuda_types::cuda::CUcontext,
            dev: cuda_types::cuda::CUdevice
        ) -> cuda_types::cuda::CUresult,
        [6] = get_module_from_cubin_ext1(
            result: *mut cuda_types::cuda::CUmodule,
            fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
            arg3: *mut std::ffi::c_void,
            arg4: *mut std::ffi::c_void,
            arg5: u32
        ) -> cuda_types::cuda::CUresult,
        [7] = cudart_interface_fn7(arg1: usize) -> cuda_types::cuda::CUresult,
        [8] = get_module_from_cubin_ext2(
            fatbin_header: *const cuda_types::dark_api::FatbinHeader,
            result: *mut cuda_types::cuda::CUmodule,
            arg3: *mut std::ffi::c_void,
            arg4: *mut std::ffi::c_void,
            arg5: u32
        ) -> cuda_types::cuda::CUresult
    },
    "{42D85A81-23F6-CB47-8298-F6E78A3AECDC}" => TOOLS_TLS[4] {
        [0] = SIZE_OF
    },
    "{A094798C-2E74-2E74-93F2-0800200C0A66}" => TOOLS_RUNTIME_CALLBACK_HOOKS[7] {
        [0] = SIZE_OF,
        [2] = get_unknown_buffer1(ptr: *mut *mut std::ffi::c_void, size: *mut usize) -> (),
        [6] = get_unknown_buffer2(ptr: *mut *mut std::ffi::c_void, size: *mut usize) -> ()
    },
    "{C693336E-1121-DF11-A8C3-68F355D89593}" => CONTEXT_LOCAL_STORAGE_INTERFACE_V0301[4] {
        [0] = context_local_storage_put(
            context: cuda_types::cuda::CUcontext,
            key: *mut std::ffi::c_void, 
            value: *mut std::ffi::c_void, 
            // clsContextDestroyCallback, have to be called on cuDevicePrimaryCtxReset
            dtor_cb: Option<extern "system" fn(
                cuda_types::cuda::CUcontext,
                *mut std::ffi::c_void, 
                *mut std::ffi::c_void,
            )>
        ) -> cuda_types::cuda::CUresult,
        [1] = context_local_storage_delete(
            key: *mut std::ffi::c_void,
            value: *mut *mut std::ffi::c_void
        ) -> cuda_types::cuda::CUresult,
        [2] = context_local_storage_get(
            value: *mut *mut std::ffi::c_void,
            cu_ctx: cuda_types::cuda::CUcontext,
            key: *mut std::ffi::c_void
        ) -> cuda_types::cuda::CUresult
    },
    "{0CA50B8C-1004-929A-89A7-D0DF10E77286}" => CTX_CREATE_BYPASS[2] {
        [0] = SIZE_OF,
        [1] = ctx_create_v2_bypass(
            pctx: *mut cuda_types::cuda::CUcontext,
            flags: ::std::os::raw::c_uint,
            dev: cuda_types::cuda::CUdevice
        ) -> cuda_types::cuda::CUresult
    },
    "{195BCBF4-D67D-024A-ACC5-1D29CEA631AE}" => HEAP_ACCESS[3] {
        [0] = SIZE_OF,
        [1] = heap_alloc(
            heap_alloc_record_ptr: *mut *const std::ffi::c_void, // HeapAllocRecord
            arg2: usize,
            arg3: usize
        ) -> cuda_types::cuda::CUresult,
        [2] = heap_free(
            heap_alloc_record_ptr: *const std::ffi::c_void, // HeapAllocRecord
            arg2: *mut usize
        ) -> cuda_types::cuda::CUresult
    },
    "{B10541E1-F7C7-C74A-9F64-F223BE99F1E2}" => DEVICE_EXTENDED_RT[26] {
        [0] = SIZE_OF,
        [5] = device_get_attribute_ext(
            dev: cuda_types::cuda::CUdevice,
            attribute: std::ffi::c_uint,
            unknown: std::ffi::c_int,
            result: *mut [usize; 2]
        ) -> cuda_types::cuda::CUresult,
        // I don't know is this function return, but on my GTX 1060 it returns 0
        [13] = device_get_something(
            result: *mut std::ffi::c_uchar,
            dev: cuda_types::cuda::CUdevice
        ) -> cuda_types::cuda::CUresult
    },
    "{D4082055-BDE6-704B-8D34-BA123C66E1F2}" => INTEGRITY_CHECK[3] {
        [0] = SIZE_OF,
        [1] = integrity_check(
            version: u32,
            unix_seconds: u64,
            result: *mut [u64;2]
        ) -> cuda_types::cuda::CUresult
    },
    // This functions check for some bits that are never observably set
    "{263E8860-7CD2-6143-92F6-BBD5006DFA7E}" => UNKNOWN_CHECKS[4] {
        [0] = SIZE_OF,
        [2] = context_check(
            ctx_in: cuda_types::cuda::CUcontext,
            result1: *mut u32, // seems to be always 0
            result2: *mut *const std::ffi::c_void
        ) -> cuda_types::cuda::CUresult,
        [3] = check_fn3() -> u32 // seeems to always return 0
    }
}

// Purely for internal use by ZLUDA dump
dark_api! {
    zluda_dump;
    "{0B7A5827-AF98-46AB-A951-22D19BDF5C08}" => ZLUDA_DUMP_INTERNAL[1] {
        #[noformat]
        [0] = logged_call(
            fn_name: cglue::slice::CSliceRef<'static, u8>,
            args: crate::FnFfiRef<crate::ByteVecFfi>,
            fn_: crate::FnFfiRef<usize>,
            internal_error: usize,
            format_status: extern "C" fn(usize) -> crate::ByteVecFfi
        ) -> usize
    }
}

#[repr(C)]
pub struct ByteVecFfi {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
}

impl ByteVecFfi {
    pub fn new(mut v: Vec<u8>) -> Self {
        let (ptr, len, capacity) = (v.as_mut_ptr(), v.len(), v.capacity());
        std::mem::forget(v);
        Self { ptr, len, capacity }
    }

    pub fn to_vec(self) -> Vec<u8> {
        let vec = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.capacity) };
        std::mem::forget(self);
        vec
    }
}

impl Drop for ByteVecFfi {
    fn drop(&mut self) {
        // SAFETY: We are dropping the Vec<u8> that we created in `from`
        // and we know that the pointer is valid.
        unsafe {
            let _ = Vec::from_raw_parts(self.ptr, self.len, self.capacity);
        }
    }
}

#[cglue::cglue_trait]
pub trait FnFfi {
    type Output;
    fn call(&self) -> Self::Output;
}

// We use this wrapper instead of implementing `FnFfi` for all T that implement `Fn() -> Output`
// because cglue machinery already provided blanket implementation of `FnFfi` for its own needs
// `cglue_trait_ext` does not work with `Fn` traits because they are special
#[repr(transparent)]
pub struct FnFfiWrapper<Output, T: std::ops::Fn() -> Output>(pub T);

impl<Output, T: std::ops::Fn() -> Output> FnFfi for FnFfiWrapper<Output, T> {
    type Output = Output;
    fn call(&self) -> Output {
        (self.0)()
    }
}

pub fn integrity_check(
    version: u32,
    unix_seconds: u64,
    driver_version: u32,
    current_process: u32,
    current_thread: u32,
    integrity_check_table: *const c_void,
    cudart_table: *const c_void,
    fn_address: *const c_void,
    devices: u32,
    get_device: impl FnMut(u32) -> DeviceHashinfo,
) -> [u64; 2] {
    match version % 10 {
        0 => return [0x3341181C03CB675C, 0x8ED383AA1F4CD1E8],
        1 => return [0x1841181C03CB675C, 0x8ED383AA1F4CD1E8],
        _ => {}
    }
    // There's first computation pass, but it does not use any input and effectively computes this
    let pass1_result = [
        0x14u8, 0x6A, 0xDD, 0xAE, 0x53, 0xA9, 0xA7, 0x52, 0xAA, 0x08, 0x41, 0x36, 0x0B, 0xF5, 0x5A,
        0x9F,
    ];
    let mut result = [0u8; 66];
    pass2(&mut result, &pass1_result);
    let pass3_input = Pass3Input {
        driver_version,
        version,
        current_process,
        current_thread,
        cudart_table,
        integrity_check_table,
        fn_address,
        unix_seconds,
    };
    pass3(&mut result, &pass3_input);
    pass4(&mut result, devices, get_device);
    let pass5_1 = pass5(&mut result);
    zero_result(&mut result);
    pass6(&mut result, &pass1_result);
    pass7(&mut result, &pass5_1);
    pass5(&mut result)
}

fn pass7(accumulator: &mut [u8; 66], pass5_1: &[u64; 2]) {
    hash_pass(accumulator, pass5_1, 0);
}

fn pass6(accumulator: &mut [u8; 66], pass1_result: &[u8; 16]) {
    hash_pass(accumulator, pass1_result, 0x5c);
}

fn zero_result(result: &mut [u8; 66]) {
    for i in 0..16 {
        result[i] = 0;
    }
    for i in 48..66 {
        result[i] = 0;
    }
}

fn pass5(result: &mut [u8; 66]) -> [u64; 2] {
    let temp = 16u8.wrapping_sub(result[64]);
    for _ in 0..temp {
        integrity_check_single_pass(result, temp);
    }
    let mut temp_ptr = unsafe { result.as_mut_ptr().add(0x30) };
    loop {
        let temp = unsafe { *temp_ptr };
        temp_ptr = unsafe { temp_ptr.add(1) };
        integrity_check_single_pass(result, temp);
        if temp_ptr == unsafe { result.as_mut_ptr().add(0x40) } {
            break;
        }
    }
    [
        u64::from_ne_bytes(result[0..8].try_into().unwrap()),
        u64::from_ne_bytes(result[8..16].try_into().unwrap()),
    ]
}

#[repr(C)]
struct Pass3Input {
    driver_version: u32,
    version: u32,
    current_process: u32,
    current_thread: u32,
    cudart_table: *const c_void,
    integrity_check_table: *const c_void,
    fn_address: *const c_void,
    unix_seconds: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DeviceHashinfo {
    pub guid: CUuuid,
    pub pci_domain: i32,
    pub pci_bus: i32,
    pub pci_device: i32,
}

fn pass2(accumulator: &mut [u8; 66], pass1_result: &[u8; 16]) {
    hash_pass(accumulator, pass1_result, 0x36)
}

fn pass3(accumulator: &mut [u8; 66], mixin: &Pass3Input) {
    hash_pass(accumulator, mixin, 0)
}

fn pass4(
    accumulator: &mut [u8; 66],
    devices: u32,
    mut get_device: impl FnMut(u32) -> DeviceHashinfo,
) {
    for dev in 0..devices {
        hash_pass(accumulator, &(get_device)(dev), 0)
    }
}

fn hash_pass<T: Sized>(accumulator: &mut [u8; 66], mixin: &T, xor_mask: u8) {
    for i in 0..std::mem::size_of_val(mixin) {
        integrity_check_single_pass(
            accumulator,
            unsafe { *std::ptr::from_ref(mixin).cast::<u8>().add(i) } ^ xor_mask,
        );
    }
}

fn integrity_check_single_pass(arg1: &mut [u8; 66], arg2: u8) {
    const MIXING_TABLE: [u8; 256] = [
        0x29, 0x2E, 0x43, 0xC9, 0xA2, 0xD8, 0x7C, 0x01, 0x3D, 0x36, 0x54, 0xA1, 0xEC, 0xF0, 0x06,
        0x13, 0x62, 0xA7, 0x05, 0xF3, 0xC0, 0xC7, 0x73, 0x8C, 0x98, 0x93, 0x2B, 0xD9, 0xBC, 0x4C,
        0x82, 0xCA, 0x1E, 0x9B, 0x57, 0x3C, 0xFD, 0xD4, 0xE0, 0x16, 0x67, 0x42, 0x6F, 0x18, 0x8A,
        0x17, 0xE5, 0x12, 0xBE, 0x4E, 0xC4, 0xD6, 0xDA, 0x9E, 0xDE, 0x49, 0xA0, 0xFB, 0xF5, 0x8E,
        0xBB, 0x2F, 0xEE, 0x7A, 0xA9, 0x68, 0x79, 0x91, 0x15, 0xB2, 0x07, 0x3F, 0x94, 0xC2, 0x10,
        0x89, 0x0B, 0x22, 0x5F, 0x21, 0x80, 0x7F, 0x5D, 0x9A, 0x5A, 0x90, 0x32, 0x27, 0x35, 0x3E,
        0xCC, 0xE7, 0xBF, 0xF7, 0x97, 0x03, 0xFF, 0x19, 0x30, 0xB3, 0x48, 0xA5, 0xB5, 0xD1, 0xD7,
        0x5E, 0x92, 0x2A, 0xAC, 0x56, 0xAA, 0xC6, 0x4F, 0xB8, 0x38, 0xD2, 0x96, 0xA4, 0x7D, 0xB6,
        0x76, 0xFC, 0x6B, 0xE2, 0x9C, 0x74, 0x04, 0xF1, 0x45, 0x9D, 0x70, 0x59, 0x64, 0x71, 0x87,
        0x20, 0x86, 0x5B, 0xCF, 0x65, 0xE6, 0x2D, 0xA8, 0x02, 0x1B, 0x60, 0x25, 0xAD, 0xAE, 0xB0,
        0xB9, 0xF6, 0x1C, 0x46, 0x61, 0x69, 0x34, 0x40, 0x7E, 0x0F, 0x55, 0x47, 0xA3, 0x23, 0xDD,
        0x51, 0xAF, 0x3A, 0xC3, 0x5C, 0xF9, 0xCE, 0xBA, 0xC5, 0xEA, 0x26, 0x2C, 0x53, 0x0D, 0x6E,
        0x85, 0x28, 0x84, 0x09, 0xD3, 0xDF, 0xCD, 0xF4, 0x41, 0x81, 0x4D, 0x52, 0x6A, 0xDC, 0x37,
        0xC8, 0x6C, 0xC1, 0xAB, 0xFA, 0x24, 0xE1, 0x7B, 0x08, 0x0C, 0xBD, 0xB1, 0x4A, 0x78, 0x88,
        0x95, 0x8B, 0xE3, 0x63, 0xE8, 0x6D, 0xE9, 0xCB, 0xD5, 0xFE, 0x3B, 0x00, 0x1D, 0x39, 0xF2,
        0xEF, 0xB7, 0x0E, 0x66, 0x58, 0xD0, 0xE4, 0xA6, 0x77, 0x72, 0xF8, 0xEB, 0x75, 0x4B, 0x0A,
        0x31, 0x44, 0x50, 0xB4, 0x8F, 0xED, 0x1F, 0x1A, 0xDB, 0x99, 0x8D, 0x33, 0x9F, 0x11, 0x83,
        0x14,
    ];
    let temp1 = arg1[0x40];
    arg1[temp1 as usize + 0x10] = arg2;
    let temp2 = temp1 as usize;
    let temp3 = (temp1 + 1) & 0xf;
    arg1[temp1 as usize + 0x20] = arg1[temp2] ^ arg2;
    let temp4 = MIXING_TABLE[(arg2 ^ arg1[0x41]) as usize];
    let temp1 = arg1[temp2 + 0x30];
    arg1[temp2 + 0x30] = temp4 ^ temp1;
    arg1[0x41] = temp4 ^ temp1;
    arg1[0x40] = temp3;
    if temp3 != 0 {
        return;
    }
    let mut temp1 = 0x29;
    let mut temp5 = 0x0;
    unsafe {
        loop {
            temp1 = temp1 ^ arg1[0];
            arg1[0] = temp1;
            let mut temp6 = arg1.as_mut_ptr().add(1);
            loop {
                let temp7 = temp6.add(1);
                temp1 = *temp6 ^ MIXING_TABLE[temp1 as usize];
                *temp6 = temp1;
                temp6 = temp7;
                if temp7 == arg1.as_mut_ptr().add(0x30) {
                    break;
                }
            }
            temp1 = temp1.wrapping_add(temp5);
            temp5 = temp5.wrapping_add(0x01);
            if temp5 == 0x12 {
                break;
            }
            temp1 = MIXING_TABLE[temp1 as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn integrity_check_single_pass() {
        let mut input = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x38, 0xc0, 0x9b, 0xf7, 0xff, 0x7f, 0x00, 0x00, 0xa3, 0x61, 0xe4, 0x42,
            0xf6, 0x67, 0x94, 0xff, 0x18, 0xc0, 0x9b, 0xf7, 0xff, 0x7f, 0x00, 0x00, 0xa4, 0x57,
            0x72, 0xf7, 0xff, 0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        super::integrity_check_single_pass(&mut input, 34);
        let expected = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x22, 0xc0, 0x9b, 0xf7, 0xff, 0x7f, 0x00, 0x00, 0xa3, 0x61, 0xe4, 0x42,
            0xf6, 0x67, 0x94, 0xff, 0x22, 0xc0, 0x9b, 0xf7, 0xff, 0x7f, 0x00, 0x00, 0xa4, 0x57,
            0x72, 0xf7, 0xff, 0x7f, 0x00, 0x00, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x57,
        ];
        assert_eq!(input, expected);
    }

    #[test]
    fn integrity_check_pass2() {
        let pass1_result = [
            0x14u8, 0x6A, 0xDD, 0xAE, 0x53, 0xA9, 0xA7, 0x52, 0xAA, 0x08, 0x41, 0x36, 0x0B, 0xF5,
            0x5A, 0x9F,
        ];
        let mut result = [0u8; 66];
        super::pass2(&mut result, &pass1_result);
        let expected = [
            0x8b, 0x21, 0x9a, 0x49, 0xe8, 0x6d, 0x1a, 0xee, 0xf2, 0x37, 0xf9, 0xb5, 0x4a, 0x8c,
            0x3c, 0x75, 0xc7, 0x1e, 0xee, 0x21, 0xcf, 0x29, 0x8a, 0xe5, 0x13, 0x83, 0xf4, 0xec,
            0x33, 0x04, 0xe2, 0xfd, 0xb0, 0x2f, 0x09, 0x01, 0x4f, 0xf7, 0x68, 0x6d, 0x69, 0x46,
            0x43, 0x7e, 0xb6, 0x2b, 0x21, 0xed, 0x57, 0xa1, 0x10, 0x86, 0x0e, 0x60, 0x44, 0x1e,
            0x70, 0x5f, 0x67, 0xd1, 0xeb, 0x67, 0xa1, 0x3d, 0x00, 0x3d,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn integrity_check_pass3() {
        let mut result = [
            0x8b, 0x21, 0x9a, 0x49, 0xe8, 0x6d, 0x1a, 0xee, 0xf2, 0x37, 0xf9, 0xb5, 0x4a, 0x8c,
            0x3c, 0x75, 0xc7, 0x1e, 0xee, 0x21, 0xcf, 0x29, 0x8a, 0xe5, 0x13, 0x83, 0xf4, 0xec,
            0x33, 0x04, 0xe2, 0xfd, 0xb0, 0x2f, 0x09, 0x01, 0x4f, 0xf7, 0x68, 0x6d, 0x69, 0x46,
            0x43, 0x7e, 0xb6, 0x2b, 0x21, 0xed, 0x57, 0xa1, 0x10, 0x86, 0x0e, 0x60, 0x44, 0x1e,
            0x70, 0x5f, 0x67, 0xd1, 0xeb, 0x67, 0xa1, 0x3d, 0x00, 0x3d,
        ];
        let input = super::Pass3Input {
            driver_version: 0x2f30,
            version: 12082,
            current_process: 0x002fa423,
            current_thread: 0xf79c1000,
            cudart_table: 0x00007ffff6958240 as *const _,
            integrity_check_table: 0x00007ffff6958220 as *const _,
            fn_address: 0x00007ffff2aaf4a0 as *const _,
            unix_seconds: 0x682b9cee,
        };
        super::pass3(&mut result, &input);
        let expected = [
            0x0a, 0xfd, 0xab, 0xc9, 0xff, 0x9b, 0xa0, 0xbe, 0x4d, 0x30, 0x32, 0x82, 0x74, 0x4f,
            0xa7, 0x48, 0x9d, 0x23, 0x82, 0xa3, 0x87, 0xfa, 0x6c, 0xdb, 0x92, 0x49, 0xd9, 0xb5,
            0x4b, 0x2b, 0x5e, 0x51, 0x6e, 0xf7, 0xf9, 0x4d, 0x28, 0x8a, 0x64, 0x06, 0x19, 0xb3,
            0xe6, 0xbe, 0xa4, 0xec, 0x7e, 0x54, 0x64, 0x28, 0xd9, 0xe1, 0xd4, 0x34, 0xc0, 0xa9,
            0x49, 0x88, 0xc9, 0x61, 0x58, 0xdd, 0x66, 0x74, 0x00, 0x74,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn integrity_check_pass4() {
        let mut result = [
            0x84, 0xfd, 0x93, 0x10, 0xc6, 0xdb, 0xb3, 0xbc, 0x49, 0xc2, 0x25, 0xe7, 0xda, 0x6e,
            0x22, 0x6f, 0x9b, 0xbd, 0x81, 0x59, 0xc3, 0x01, 0x9a, 0x7a, 0x26, 0x34, 0x39, 0x0f,
            0x2a, 0x56, 0x13, 0xb1, 0xf6, 0xbc, 0x7f, 0xa1, 0x8f, 0x04, 0xa5, 0x4d, 0x0d, 0x78,
            0xab, 0x20, 0xf8, 0x23, 0x20, 0xa5, 0x3f, 0x67, 0x36, 0xe2, 0xde, 0x8a, 0xe5, 0xdf,
            0xe1, 0xf2, 0x03, 0x94, 0xad, 0xdc, 0x9a, 0xda, 0x00, 0xda,
        ];
        super::pass4(&mut result, 1, |_| super::DeviceHashinfo {
            guid: super::CUuuid {
                bytes: unsafe {
                    std::mem::transmute([0x8a2bfe9au32, 0x382d25ac, 0xc5ae37ea, 0x5f32716d])
                },
            },
            pci_domain: 0,
            pci_bus: 2,
            pci_device: 0,
        });
        let expected = [
            0x1f, 0xd8, 0x25, 0xd2, 0xdf, 0xfa, 0x64, 0xc7, 0xb6, 0x1a, 0xaf, 0x22, 0xb8, 0x79,
            0xfb, 0x96, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x7c, 0x9d, 0x46, 0xd2, 0x1f, 0xd8, 0x25, 0xd2, 0xdd, 0xfa, 0x64, 0xc7, 0xb6, 0x1a,
            0xaf, 0x22, 0xe6, 0x17, 0xbd, 0x3a, 0xd7, 0xdd, 0x5f, 0x82, 0x8c, 0x87, 0xce, 0x86,
            0x66, 0xaf, 0xa0, 0x50, 0x7a, 0x7d, 0xbb, 0xbc, 0x0c, 0x50,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn integrity_check_pass5() {
        let mut result = [
            0x3e, 0x4b, 0xf2, 0x95, 0x71, 0xf5, 0x6b, 0x51, 0x07, 0xbf, 0x4b, 0xf1, 0x04, 0x0e,
            0x8e, 0x0b, 0x5f, 0x4d, 0x30, 0x0c, 0x0f, 0x0c, 0xae, 0xfb, 0x48, 0xaf, 0x23, 0xb5,
            0xea, 0x4c, 0xc2, 0xdb, 0xd7, 0xdf, 0x88, 0x74, 0x39, 0x58, 0x16, 0x3a, 0x1f, 0x7c,
            0x9b, 0x20, 0x7e, 0x7e, 0x94, 0xc8, 0x8b, 0xc6, 0xb2, 0x38, 0x0d, 0x07, 0x7d, 0xbd,
            0x90, 0xd5, 0x39, 0x63, 0xeb, 0x1d, 0x4f, 0x40, 0x00, 0x40,
        ];
        let output = super::pass5(&mut result);
        let expected_result = [
            0x00, 0x23, 0x53, 0x06, 0x5e, 0x96, 0xf6, 0x9c, 0x61, 0xaa, 0x96, 0x2d, 0x2e, 0xcd,
            0xa8, 0x58, 0xe9, 0xca, 0xc0, 0x2e, 0x35, 0xed, 0x5f, 0xca, 0xe1, 0x0e, 0xcd, 0x1f,
            0xd0, 0x8e, 0x8b, 0x9c, 0x29, 0x4d, 0x1c, 0x94, 0x6b, 0xf7, 0x10, 0xb0, 0x07, 0x08,
            0x91, 0xd6, 0x14, 0x06, 0xc0, 0xec, 0xe1, 0x9c, 0x8e, 0x33, 0xd4, 0xe9, 0x43, 0x5c,
            0x86, 0x0c, 0x72, 0x4d, 0x27, 0x98, 0x91, 0x7f, 0x00, 0x7f,
        ];
        assert_eq!(result, expected_result);
        let output = unsafe { mem::transmute::<_, [u8; 16]>(output) };
        let expected = [
            0x00, 0x23, 0x53, 0x06, 0x5e, 0x96, 0xf6, 0x9c, 0x61, 0xaa, 0x96, 0x2d, 0x2e, 0xcd,
            0xa8, 0x58,
        ];
        assert_eq!(output, expected);
    }
}
