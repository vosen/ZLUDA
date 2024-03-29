// This module covers all known dark api functions. It exists to force
// convergence between main implementation and dumper implementation
use bit_vec::BitVec;
use bitflags::bitflags;
use cuda_types::*;
use hip_common::zluda_ext::CudaResult;
use paste::paste;
use std::{
    borrow::Cow,
    ffi::c_void,
    fmt::Display,
    mem,
    os::raw::{c_int, c_uchar, c_uint, c_ulong, c_ushort},
    ptr, slice,
};

macro_rules! dark_api_fn_decl {
    (SIZE_OF) => { };
    (NULL) => { };
    ($fn_:ident ( $($arg_name:ident: $arg_ty:ty),* ) -> $ret_ty:ty) => {
        #[allow(non_snake_case)]
        unsafe extern "system" fn $fn_(
            $($arg_name : $arg_ty,)*
        ) -> $ret_ty;
    }
}

macro_rules! dark_api_fn_dump_decl {
    ($name:ident $idx:literal $fn_:ident $( ( $($arg_name:ident: $arg_ty:ty),* ) -> $ret_ty:ty )?) => { };
    (#[dump] $name:ident $idx:literal $fn_:ident ( $($arg_name:ident: $arg_ty:ty),* ) -> $ret_ty:ty) => {
        #[allow(non_snake_case)]
        unsafe extern "system" fn $fn_(
            $($arg_name : $arg_ty,)*
        ) -> $ret_ty {
            paste!( Self:: [<$fn_ _impl>] )(&CudaDarkApiTable::$name, $idx, $($arg_name,)* )
        }

        paste!{
            #[allow(non_snake_case)]
            unsafe fn [<$fn_ _impl>] (guid: &[u8; 16], idx: usize, $($arg_name : $arg_ty,)*) -> $ret_ty;
        }
    };
}

macro_rules! dark_api_fn_dump_match {
    ($T:ident $fn_:ident) => {
        None
    };
    (#[dump] $T:ident $fn_:ident) => {
        Some($T::$fn_ as *const c_void)
    };
}

macro_rules! dark_api_table_setter {
    (SIZE_OF, $table_len:literal, $api:ty) => {{
        (std::mem::size_of::<usize>() * $table_len) as *const ()
    }};
    (NULL, $table_len:literal, $api:ty) => {{
        std::ptr::null()
    }};
    ($fn_:ident, $table_len:literal, $api:ty) => {{
        <$api as CudaDarkApi>::$fn_ as *const ()
    }};
}

macro_rules! dark_api_fn_passthrough {
    ($idx:literal, SIZE_OF) => { };
    ($idx:literal, NULL) => { };
    ($idx:literal, $fn_:ident ( $($arg_name:ident: $arg_ty:ty),* ) -> $ret_ty:ty) => {
        #[allow(non_snake_case)]
        pub unsafe fn $fn_(
            &self,
            $($arg_name : $arg_ty,)*
        ) -> $ret_ty {
            let ptr = self.ptr as *mut *mut c_void;
            let ptr = ptr.add($idx);
            let fn_ = std::mem::transmute::<_, unsafe extern "system" fn( $($arg_ty,)* ) -> $ret_ty >(*ptr);
            (fn_)( $($arg_name,)* )
        }
    }
}

macro_rules! dark_api_table {
    ($($guid:expr => $name:ident [$table_len:literal] {
        $( $(#[$attr:ident])? $idx:literal =>$fn_:ident $( ( $($arg_name:ident: $arg_ty:ty),* ) -> $ret_ty:ty )? ),*
    }),+) => {
        pub trait CudaDarkApi {
            $($(
                dark_api_fn_decl!($fn_ $( ( $($arg_name: $arg_ty),* ) -> $ret_ty )?);
            )*)+
        }

        #[allow(non_snake_case)]
        pub struct CudaDarkApiTable {
            $(pub $name: [*const (); $table_len],)+
        }

        unsafe impl Sync for CudaDarkApiTable {}

        impl CudaDarkApiTable {
            $(const $name: [u8; 16] = $guid;)+
        }

        impl CudaDarkApiTable {
            pub fn get(&self, guid: &[u8; 16]) -> Option<&[*const ()]> {
                match guid {
                    $(&Self::$name => {
                        Some(&self.$name[..])
                    })+
                    _ => None
                }
            }
        }

        pub const fn init_dark_api<T:CudaDarkApi>() -> CudaDarkApiTable {
            let mut table = CudaDarkApiTable {
                $($name: [std::ptr::null(); $table_len],)+
            };
            $($(
                table.$name[$idx] = dark_api_table_setter!($fn_, $table_len, T);
            )*)+
            table
        }

        pub trait CudaDarkApiDump {
            $($(
                dark_api_fn_dump_decl!( $(#[$attr])? $name $idx $fn_ $( ( $($arg_name: $arg_ty),* ) -> $ret_ty )?);
            )*)+
        }

        pub fn get_dark_api_fn<T:CudaDarkApiDump>(guid: &[u8; 16], idx: usize) -> Option<*const c_void> {
            match (guid, idx) {
                $($(
                    (&CudaDarkApiTable::$name, $idx) => dark_api_fn_dump_match!($(#[$attr])? T $fn_ ),
                )*)+
                _ => None
            }
        }

        #[allow(non_snake_case)]
        pub struct CudaDarkApiKnownExports {
            empty: BitVec,
            $($name: BitVec,)+
        }

        impl CudaDarkApiKnownExports {
            pub fn new() -> Self {
                Self {
                    empty: BitVec::new(),
                    $(
                        $name: {
                            let mut set = BitVec::from_elem($table_len, false);
                            $( set.set($idx, true); )*
                            set
                        },
                    )+
                }
            }

            pub fn known(&self, guid: &[u8; 16] ) -> impl ExactSizeIterator<Item=bool> + '_ {
                let set = match guid {
                    $(
                        &CudaDarkApiTable::$name => {
                            &self.$name
                        }
                    )+
                    _ => &self.empty
                };
                set.iter()
            }
        }

        $(
            paste! {
                pub struct [<$name:camel>] {
                    #[allow(dead_code)]
                    ptr: *const c_void
                }

                impl [<$name:camel>] {
                    pub const GUID: CUuuid = CUuuid { bytes: $guid };

                    pub unsafe fn new(ptr: *const c_void) -> Self {
                        Self {
                            ptr
                        }
                    }
                    $(
                        dark_api_fn_passthrough!($idx, $fn_ $( ( $($arg_name: $arg_ty),* ) -> $ret_ty )?);
                    )*
                }
            }
        )+
    };
}

dark_api_table!(
    [0x6b, 0xd5, 0xfb, 0x6c, 0x5b, 0xf4, 0xe7, 0x4a, 0x89, 0x87, 0xd9, 0x39, 0x12, 0xfd, 0x9d, 0xf9]
    => CUDART_INTERFACE [10] {
        0 => SIZE_OF,
        #[dump]
        1 => get_module_from_cubin(
            module: *mut CUmodule,
            fatbinc_wrapper: *const FatbincWrapper
        ) -> CUresult,
        2 => get_primary_context(pctx: *mut CUcontext, dev: CUdevice) -> CUresult,
        #[dump]
        6 => get_module_from_cubin_ex1(
            module: *mut CUmodule,
            fatbinc_wrapper: *const FatbincWrapper,
            arg3: *mut c_void,
            arg4: *mut c_void,
            arg5: usize
        ) -> CUresult,
        7 => cudart_interface_fn7(arg1: usize) -> (),
        // From the assembly it looks like arg5 is a count of something in arg3 and arg4
        #[dump]
        8 => get_module_from_cubin_ex2(
            fatbin_header: *const FatbinHeader,
            module: *mut CUmodule,
            arg3: *mut c_void,
            arg4: *mut c_void,
            arg5: c_uint
        ) -> CUresult,
        #[dump]
        9 => launch_kernel(
            f: CUfunction,
            grid_dim_x: ::std::os::raw::c_uint,
            grid_dim_y: ::std::os::raw::c_uint,
            grid_dim_z: ::std::os::raw::c_uint,
            block_dim_x: ::std::os::raw::c_uint,
            block_dim_y: ::std::os::raw::c_uint,
            block_dim_z: ::std::os::raw::c_uint,
            shared_mem_bytes: ::std::os::raw::c_uint,
            stream: CUstream,
            extra: *mut *mut ::std::os::raw::c_void
        ) -> CUresult
    },
    [0xa0, 0x94, 0x79, 0x8c, 0x2e, 0x74, 0x2e, 0x74, 0x93, 0xf2, 0x08, 0x00, 0x20, 0x0c, 0x0a, 0x66]
    => TOOLS_RUNTIME_CALLBACK_HOOKS [7] {
        0 => SIZE_OF,
        // Both of those return some prealloacted buffers to be used by tooling?
        2 => tools_runtime_callback_hooks_fn2(ptr: *mut *mut usize, size: *mut usize) -> (),
        6 => tools_runtime_callback_hooks_fn6(ptr: *mut *mut u8, size: *mut usize) -> ()
    },
    [0x42, 0xd8, 0x5a, 0x81, 0x23, 0xf6, 0xcb, 0x47, 0x82, 0x98, 0xf6, 0xe7, 0x8a, 0x3a, 0xec, 0xdc]
    => TOOLS_TLS [3] {
        0 => SIZE_OF
    },
    [0xc6, 0x93, 0x33, 0x6e, 0x11, 0x21, 0xdf, 0x11, 0xa8, 0xc3, 0x68, 0xf3, 0x55, 0xd8, 0x95, 0x93]
    => CONTEXT_LOCAL_STORAGE_INTERFACE_V0301 [4] {
        0 => context_local_storage_insert(
            key: CUcontext,
            mgr: *mut c_void,
            value: *mut c_void,
            // clsContextDestroyCallback,  have to be called on cuDevicePrimaryCtxReset
            dtor_cb: Option<
                extern "system" fn(
                    CUcontext,
                    *mut c_void,
                    *mut c_void,
                )
            >
        ) -> CUresult,
        // some kind of dtor
        1 => context_local_storage_remove(arg1: usize, arg2: usize) -> CUresult,
        2 => context_local_storage_get(
            result: *mut *mut c_void,
            cu_ctx: CUcontext,
            key: *mut c_void
        ) -> CUresult,
        3 => NULL
    },
    // This fn table is used by PhysX
    [0x0C, 0xA5, 0x0B, 0x8C, 0x10, 0x04, 0x92, 0x9A, 0x89, 0xA7, 0xD0, 0xDF, 0x10, 0xE7, 0x72, 0x86]
    => CTX_CREATE_BYPASS [2] {
        0 => SIZE_OF,
        #[dump]
        1 => ctx_create_v2_bypass(
            pctx: *mut CUcontext,
            flags: c_uint,
            dev: CUdevice
        ) -> CUresult
    },
    [0x19, 0x5B, 0xCB, 0xF4, 0xD6, 0x7D, 0x02, 0x4A, 0xAC, 0xC5, 0x1D, 0x29, 0xCE, 0xA6, 0x31, 0xAE]
    => HEAP_ACCESS [3] {
        0 => SIZE_OF,
        1 => heap_alloc(
            halloc_ptr: *mut *mut HeapAllocRecord,
            param1: usize,
            param2: usize
        ) -> CUresult,
        2 => heap_free(halloc: *mut HeapAllocRecord, param2: *mut usize) -> CUresult
    },
    // This fn table is used by OptiX
    [0xB1u8, 0x05, 0x41, 0xE1, 0xF7, 0xC7, 0xC7, 0x4A, 0x9F, 0x64, 0xF2, 0x23, 0xBE, 0x99, 0xF1, 0xE2]
    => DEVICE_EXTENDED_RT [26] {
        0 => SIZE_OF,
        5 => device_get_attribute_ex(
            dev: CUdevice,
            attribute: c_uint,
            unknown: c_int,
            result: *mut [usize; 2]
        ) -> CUresult,
        // I don't know what this function does, but on my GTX 1060 it returns 0
        13 => device_get_something(
            result: *mut c_uchar,
            dev: CUdevice
        ) -> CUresult
    },
    // This fn table is used by DLSS
    [0x7f, 0x92, 0x12, 0xd6, 0x26, 0x1d, 0xdd, 0x4d, 0x8a, 0xf6, 0x38, 0xdd, 0x1a, 0xeb, 0x10, 0xae]
    => DLSS [58] {
        0 => SIZE_OF,
        #[dump]
        3 => dlss_cuInit() -> CUresult,
        #[dump]
        7 => dlss_start1(
            retval1: *mut *mut c_void,
            arg2: *mut c_void,
            arg3: *mut c_void,
            arg4: *mut c_void,
            // pointer to fn table in libnvidia-glcore.so
            arg5: *mut c_void
        ) -> CUresult,
        #[dump]
        9 => dlss_start2(
            handle: *mut c_void,
            arg2: *mut u32
        ) -> CUresult,
        #[dump]
        12 => dlss_module_load(
            context: CUcontext,
            result: *mut CUmodule,
            fatbin: *mut c_void,
            arg4: u32,
            arg5: *mut c_void,
            arg6: *mut c_void
        ) -> CUresult,
        #[dump]
        14 => dlss_module_get_function(
            result: *mut CUfunction,
            module: CUmodule,
            name: *const i8
        ) -> CUresult,
        #[dump]
        22 => dlss_feature_evaluate2(
            handle1: *mut c_void,
            handle2: *mut c_void,
            handle3: *mut c_void,
            arg4: u8,
            handle5: *mut c_void,
            arg6: u32
        ) -> CUresult,
        #[dump]
        37 => dlss_feature_evaluate1(
            retval1: *mut u32,
            retval2: *mut u32,
            retval3: *mut u32,
            handle: *mut c_void
        ) -> CUresult,
        #[dump]
        39 => dlss_feature_evaluate_init(
            retval1: *mut *mut c_void,
            handle: *mut c_void,
            retval2: *mut *mut c_void
        ) -> CUresult
    },
    [0x26, 0x3e, 0x88, 0x60, 0x7c, 0xd2, 0x61, 0x43, 0x92, 0xf6, 0xbb, 0xd5, 0x00, 0x6d, 0xfa, 0x7e]
    => CONTEXT_WRAPPER [3] {
        0 => SIZE_OF,
        // This function is present at least from 12.3 and the pseudocode is:
        //   let ctx = get_current_if_null(ctx);
        //   if ctx.unknown_flag == 1 {
        //       *wrapped = 1;
        //       if unwrapped_ctx != ptr::null_mut() {
        //           *unwrapped_ctx = ctx.unwrapped_ctx;
        //       }
        //   } else {
        //       *wrapped = 0;
        //   }
        // This pattern is present in other parts of the driver (cuCtxDestroy_v2).
        // Not sure how can one create context with `unknown_flag` set.
        // Different variant of cuCtxCreate and primary context all have this flag unset.
        // Maybe interop context?
        #[dump]
        2 => unwrap_context(
            ctx: CUcontext,
            wrapped: *mut u32,
            unwrapped_ctx: *mut CUcontext
        ) -> CUresult
    },
    // Functions used by NVIDIA CUDA runtime to detect presence of ZLUDA and fail if it's found
    [0xd4, 0x08, 0x20, 0x55, 0xbd, 0xe6, 0x70, 0x4b, 0x8d, 0x34, 0xba, 0x12, 0x3c, 0x66, 0xe1, 0xf2]
    => ANTI_ZLUDA [3] {
        0 => SIZE_OF,
        #[dump]
        1 => zluda_check(
            arg1: u32, // runtime version?
            timestamp: u64, // UNIX time in seconds
            result: *mut u128
        ) -> CUresult
    },
    // This table is used by ZLUDA, it's *not* exported by NVIDIA CUDA. It's necessary for library interop,
    // e.g. zluda_blas, when wrapping rocblas needs to extract HIP stream from ZLUDA stream
    // Alternatively we could have ZLUDA library just export the functions, but
    // that is not transparent to the tooling (zluda_dump). Every function would need to be
    // exported twice: firstly from ZLUDA proper and then again for passthrough in zluda_dump
    [0x70, 0xe1, 0xfa, 0xee, 0x26, 0x8, 0x40, 0x25, 0x82, 0xa7, 0xe0, 0xbc, 0xc1, 0x2e, 0xcc, 0x60]
    => ZLUDA_EXT [2] {
        0 => SIZE_OF,
        1 => get_hip_stream(stream: CUstream) -> CudaResult<*const std::os::raw::c_void>
    }
);

pub const ELF_MAGIC: c_uint = unsafe { std::mem::transmute(*b"\x7FELF") };
pub const FATBINC_MAGIC: c_uint = 0x466243B1;
pub const FATBINC_VERSION_V1: c_uint = 0x1;
pub const FATBINC_VERSION_V2: c_uint = 0x2;

#[repr(C)]
pub struct FatbincWrapper {
    magic: c_uint,
    version: c_uint,
    data: *const FatbinHeader,
    filename_or_fatbins: *const c_void,
}

pub const FATBIN_MAGIC: c_uint = 0xBA55ED50;
pub const FATBIN_VERSION: c_ushort = 0x01;

#[repr(C, align(8))]
pub struct FatbinHeader {
    magic: c_uint,
    version: c_ushort,
    header_size: c_ushort,
    files_size: c_ulong, // excluding frame header, size of all blocks framed by this frame
}

pub const FATBIN_FILE_HEADER_KIND_PTX: c_ushort = 0x01;
pub const FATBIN_FILE_HEADER_KIND_ELF: c_ushort = 0x02;
pub const FATBIN_FILE_HEADER_VERSION_CURRENT: c_ushort = 0x101;

// assembly file header is a bit different, but we don't care
#[repr(C)]
pub struct FatbinFileHeader {
    pub kind: c_ushort,
    pub version: c_ushort,
    pub header_size: c_uint,
    pub padded_payload_size: c_uint,
    pub unknown0: c_uint, // might be part of padded_payload_size

    pub payload_size: c_uint,
    pub unknown1: c_uint,    // probably bitness in case of PTX, nothing for ELF
    pub ptx_version: c_uint, // (major * 0x10000) + minor
    pub sm_version: c_uint,

    pub file_name_offset: c_uint, // optional
    pub file_name_len: c_uint,    // optional
    pub flags: FatbinFileHeaderFlags,

    pub unknown6: u64,
    pub uncompressed_payload: u64,
    // pub ptxas_arg_offset: c_uint, // optional
    // pub ptxas_arg_len: c_uint, // optional
}

bitflags! {
    #[repr(transparent)]
    pub struct FatbinFileHeaderFlags: u64 {
        const Is64Bit = 0x0000000000000001;
        const Debug = 0x0000000000000002;
        const Linux = 0x0000000000000010;
        const Mac = 0x0000000000000020;
        const Windows = 0x0000000000000040;
        const OptMask = 0x0000000000000f00;
        const CompressedOld = 0x0000000000001000;
        const CompressedNew = 0x0000000000002000;

        const _ = !0;
    }
}

pub enum ContextState {}

pub enum ContextStateManager {}

#[repr(C)]
pub struct HeapAllocRecord {
    param1: usize,
    param2: usize,
    _unknown: usize,
    global_heap: *mut c_void,
}

#[derive(Clone, Copy)]
pub enum AnyUInt {
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
}

impl Display for AnyUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyUInt::U16(x) => write!(f, "{:#x}", x),
            AnyUInt::U32(x) => write!(f, "{:#x}", x),
            AnyUInt::U64(x) => write!(f, "{:#x}", x),
            AnyUInt::USize(x) => write!(f, "{:#x}", x),
        }
    }
}

impl std::fmt::Debug for AnyUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct UnexpectedFieldError {
    pub name: &'static str,
    pub expected: Vec<AnyUInt>,
    pub observed: AnyUInt,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FatbinFileKind {
    Ptx,
    Elf,
    Archive,
}

impl FatbinFileKind {
    pub fn file_extension(self) -> &'static str {
        match self {
            FatbinFileKind::Ptx => "ptx",
            FatbinFileKind::Elf => "elf",
            FatbinFileKind::Archive => "a",
        }
    }
}

pub enum CudaFatbin {
    Version1(FatbinModuleHandle),
    Version2 {
        post_link: FatbinModuleHandle,
        pre_link: &'static [FatbinModuleHandle],
    },
}

impl CudaFatbin {
    pub unsafe fn from_wrapper(
        fatbinc_wrapper: *const FatbincWrapper,
    ) -> Result<Self, UnexpectedFieldError> {
        let fatbinc_wrapper = &*fatbinc_wrapper;
        let magic = fatbinc_wrapper.magic;
        if magic != FATBINC_MAGIC {
            return Err(UnexpectedFieldError {
                name: "FATBINC_MAGIC",
                expected: vec![AnyUInt::U32(FATBINC_MAGIC)],
                observed: AnyUInt::U32(magic),
            });
        }
        let version = fatbinc_wrapper.version;
        Ok(match version {
            FATBINC_VERSION_V1 => CudaFatbin::Version1(FatbinModuleHandle(fatbinc_wrapper.data)),
            FATBINC_VERSION_V2 => {
                let len = slice_length(fatbinc_wrapper.filename_or_fatbins.cast());
                let pre_link =
                    std::slice::from_raw_parts(fatbinc_wrapper.filename_or_fatbins.cast(), len);
                let post_link = FatbinModuleHandle(fatbinc_wrapper.data);
                CudaFatbin::Version2 {
                    post_link,
                    pre_link,
                }
            }
            _ => {
                return Err(UnexpectedFieldError {
                    name: "FATBINC_VERSION",
                    expected: vec![
                        AnyUInt::U32(FATBINC_VERSION_V1),
                        AnyUInt::U32(FATBINC_VERSION_V2),
                    ],
                    observed: AnyUInt::U32(version),
                });
            }
        })
    }

    pub unsafe fn from_header(fatbin_header: *const FatbinHeader) -> Self {
        CudaFatbin::Version1(FatbinModuleHandle(fatbin_header))
    }
}

pub enum CUmoduleContent {
    RawText(*const u8),
    Archive(&'static [u8]),
    File(*const i8),
    Fatbin(CudaFatbin),
    Elf(*const u8),
}

impl CUmoduleContent {
    pub unsafe fn from_ptr(ptr: *const u8) -> Result<Self, UnexpectedFieldError> {
        Ok(if (ptr as usize % 4) != 0 {
            CUmoduleContent::RawText(ptr)
        } else if *(ptr as *const u32) == FATBINC_MAGIC {
            CUmoduleContent::Fatbin(CudaFatbin::from_wrapper(ptr as *const FatbincWrapper)?)
        } else if *(ptr as *const u32) == FATBIN_MAGIC {
            CUmoduleContent::Fatbin(CudaFatbin::from_header(ptr as *const FatbinHeader))
        } else if *(ptr as *const u32) == ELF_MAGIC {
            CUmoduleContent::Elf(ptr)
        } else {
            CUmoduleContent::RawText(ptr)
        })
    }
}

unsafe fn slice_length(ptr: *const *const c_void) -> usize {
    let mut current = ptr;
    while *current != ptr::null() {
        current = current.add(1);
    }
    current.offset_from(ptr) as usize
}

#[repr(transparent)]
pub struct FatbinModuleHandle(*const FatbinHeader);

impl FatbinModuleHandle {
    pub unsafe fn get(&self) -> Result<FatbinModule, UnexpectedFieldError> {
        let fatbin_header = &*self.0;
        let magic = fatbin_header.magic;
        if magic == ELF_MAGIC {
            return Ok(FatbinModule::Elf(self.0 as _));
        } else if magic != FATBIN_MAGIC {
            return Err(UnexpectedFieldError {
                name: "FATBIN_MAGIC",
                expected: vec![AnyUInt::U32(FATBIN_MAGIC), AnyUInt::U32(ELF_MAGIC)],
                observed: AnyUInt::U32(magic),
            });
        }
        let version = fatbin_header.version;
        if version != FATBIN_VERSION {
            return Err(UnexpectedFieldError {
                name: "FATBIN_VERSION",
                expected: vec![AnyUInt::U16(FATBIN_VERSION)],
                observed: AnyUInt::U16(version),
            });
        }
        let current = (self.0 as *const u8).add(fatbin_header.header_size as usize);
        let end = current.add(fatbin_header.files_size as usize);
        Ok(FatbinModule::Files(FatbinModuleFiles { current, end }))
    }
}

pub struct FatbinModuleFiles {
    current: *const u8,
    end: *const u8,
}

impl Iterator for FatbinModuleFiles {
    type Item = Result<FatbinFile, UnexpectedFieldError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let fatbin_file = unsafe { &*(self.current as *const FatbinFileHeader) };
            self.current = unsafe {
                self.current.add(
                    fatbin_file.header_size as usize + fatbin_file.padded_payload_size as usize,
                )
            };
            Some(unsafe { FatbinFile::try_new(fatbin_file) })
        } else {
            None
        }
    }
}

pub enum FatbinModule {
    Elf(*const u8),
    Files(FatbinModuleFiles),
}

pub struct FatbinFile {
    data: *const u8,
    pub kind: FatbinFileKind,
    pub compression: FatbinCompression,
    pub sm_version: u32,
    padded_payload_size: usize,
    payload_size: usize,
    uncompressed_payload: usize,
}

pub enum FatbinCompression {
    None,
    Zlib,
    Lz4,
}

impl FatbinFile {
    unsafe fn try_new(fatbin_file: &FatbinFileHeader) -> Result<Self, UnexpectedFieldError> {
        let fatbin_file_version = fatbin_file.version;
        if fatbin_file_version != FATBIN_FILE_HEADER_VERSION_CURRENT {
            return Err(UnexpectedFieldError {
                name: "FATBIN_FILE_HEADER_VERSION",
                expected: vec![AnyUInt::U16(FATBIN_FILE_HEADER_VERSION_CURRENT)],
                observed: AnyUInt::U16(fatbin_file_version),
            });
        }
        let raw_fatbin_file_kind = (*fatbin_file).kind;
        let kind = match raw_fatbin_file_kind {
            FATBIN_FILE_HEADER_KIND_PTX => FatbinFileKind::Ptx,
            FATBIN_FILE_HEADER_KIND_ELF => FatbinFileKind::Elf,
            _ => {
                return Err(UnexpectedFieldError {
                    name: "FATBIN_FILE_HEADER_KIND",
                    expected: vec![
                        AnyUInt::U16(FATBIN_FILE_HEADER_KIND_ELF),
                        AnyUInt::U16(FATBIN_FILE_HEADER_KIND_PTX),
                    ],
                    observed: AnyUInt::U16(raw_fatbin_file_kind),
                });
            }
        };
        let compression = if fatbin_file
            .flags
            .contains(FatbinFileHeaderFlags::CompressedOld)
        {
            FatbinCompression::Zlib
        } else if fatbin_file
            .flags
            .contains(FatbinFileHeaderFlags::CompressedNew)
        {
            FatbinCompression::Lz4
        } else {
            FatbinCompression::None
        };
        let data = (fatbin_file as *const _ as *const u8).add(fatbin_file.header_size as usize);
        let padded_payload_size = fatbin_file.padded_payload_size as usize;
        let payload_size = fatbin_file.payload_size as usize;
        let uncompressed_payload = fatbin_file.uncompressed_payload as usize;
        let sm_version = fatbin_file.sm_version;
        Ok(Self {
            data,
            kind,
            compression,
            padded_payload_size,
            payload_size,
            uncompressed_payload,
            sm_version,
        })
    }

    // Returning static lifetime here because all known uses of this are related to fatbin files that
    // are constants inside files
    pub unsafe fn get_or_decompress(&self) -> Result<Cow<'static, [u8]>, DecompressionFailure> {
        match self.compression {
            FatbinCompression::Lz4 => {
                match self.decompress_kernel_module_lz4() {
                    Some(mut decompressed) => {
                        if self.kind == FatbinFileKind::Ptx {
                            decompressed.pop(); // remove trailing zero
                        }
                        Ok(Cow::Owned(decompressed))
                    }
                    None => Err(DecompressionFailure),
                }
            }
            FatbinCompression::Zlib => {
                let compressed =
                    std::slice::from_raw_parts(self.data.cast(), self.padded_payload_size);
                Ok(Cow::Owned(
                    cloudflare_zlib::inflate(compressed).map_err(|_| DecompressionFailure)?,
                ))
            }
            FatbinCompression::None => Ok(Cow::Borrowed(slice::from_raw_parts(
                self.data,
                self.padded_payload_size as usize,
            ))),
        }
    }

    const MAX_MODULE_DECOMPRESSION_BOUND: usize = 64 * 1024 * 1024;

    unsafe fn decompress_kernel_module_lz4(&self) -> Option<Vec<u8>> {
        let decompressed_size = usize::max(1024, self.uncompressed_payload as usize);
        let mut decompressed_vec = vec![0u8; decompressed_size];
        loop {
            match lz4_sys::LZ4_decompress_safe(
                self.data.cast(),
                decompressed_vec.as_mut_ptr() as *mut _,
                self.payload_size as c_int,
                decompressed_vec.len() as c_int,
            ) {
                error if error < 0 => {
                    let new_size = decompressed_vec.len() * 2;
                    if new_size > Self::MAX_MODULE_DECOMPRESSION_BOUND {
                        return None;
                    }
                    decompressed_vec.resize(decompressed_vec.len() * 2, 0);
                }
                real_decompressed_size => {
                    decompressed_vec.truncate(real_decompressed_size as usize);
                    return Some(decompressed_vec);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct DecompressionFailure;

pub fn anti_zluda_hash<F: FnMut(u32) -> AntiZludaHashInputDevice>(
    return_known_value: bool,
    input: AntiZludaHashInput,
    dev_getter: F,
) -> u128 {
    anti_zluda_hash_impl(
        return_known_value,
        input,
        dev_getter,
        std::process::id(),
        thread_id::get() as u32,
    )
}

const ANTI_ZLUDA_HASH_TABLE_PART1: [u8; 64] = [
    0x6b, 0xcf, 0x32, 0x0f, 0xa4, 0x49, 0xd3, 0xa8, 0x33, 0xf8, 0xd0, 0x8e, 0x12, 0x4e, 0xa8, 0x00,
    0xeb, 0x94, 0x2c, 0x8f, 0x34, 0x49, 0xde, 0xf6, 0xbf, 0x29, 0x91, 0x20, 0xc7, 0x65, 0xf6, 0xba,
    0x78, 0x5c, 0x66, 0x27, 0xa7, 0xb2, 0x73, 0x92, 0xdb, 0x22, 0x1e, 0x20, 0x14, 0x6f, 0x87, 0xff,
    0xa5, 0xc3, 0x18, 0x01, 0x66, 0x64, 0xa5, 0x0e, 0x70, 0x51, 0x52, 0xa7, 0x80, 0x4b, 0xdf, 0xef,
];

pub struct AntiZludaHashInput {
    pub cudart_export_table: *mut c_void,
    pub anti_zluda_export_table: *mut c_void,
    pub fn_ptr: *mut c_void,
    pub device_count: u32,
    pub driver_version: u32,
    pub rt_version: u32,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct AntiZludaHashInputDevice {
    pub guid: CUuuid,
    pub pci_domain: u32,
    pub pci_bus: u32,
    pub pci_device: u32,
}

fn anti_zluda_hash_impl<F: FnMut(u32) -> AntiZludaHashInputDevice>(
    return_known_value: bool,
    input: AntiZludaHashInput,
    dev_getter: F,
    process_id: u32,
    thread_id: u32,
) -> u128 {
    if return_known_value {
        0x8ED383AA1F4CD1E83341181C03CB675Cu128
    } else if input.rt_version % 10 < 2 {
        let result = 0x8ED383AA1F4CD1E83341181C03CB675Cu128;
        if input.rt_version % 10 == 1 {
            let mut result = result.to_le_bytes();
            result[7] = 24;
            u128::from_le_bytes(result)
        } else {
            result
        }
    } else {
        let mut result = [0u8; 66];
        let mut aux = [0u8; 16];
        anti_zluda_hash_impl_part1(&mut aux);
        anti_zluda_hash_impl_part2(&mut result, aux);
        anti_zluda_hash_impl_part3(&mut result, &input, process_id, thread_id);
        anti_zluda_hash_impl_part4(&mut result, &input, dev_getter);
        anti_zluda_hash_impl_part5(&mut result);
        anti_zluda_hash_round_v2(&mut result, 48);
        anti_zluda_hash_impl_part6(&mut result, aux);
        anti_zluda_hash_impl_part5(&mut result);
        anti_zluda_hash_round_v2(&mut result, 48);
        let mut output = [0u8; 16];
        output.copy_from_slice(&result[..16]);
        u128::from_le_bytes(output)
    }
}

fn anti_zluda_hash_impl_part6(result: &mut [u8; 66], aux: [u8; 16]) {
    let mut v30 = [0u8; 16];
    v30.copy_from_slice(&result[..16]);
    result[48..].fill(0u8);
    result[..16].fill(0u8);
    for i in 0..16 {
        anti_zluda_hash_round_v1(result, aux[i] ^ 0x5C);
    }
    for i in 0..16 {
        anti_zluda_hash_round_v1(result, v30[i]);
    }
}

fn anti_zluda_hash_impl_part5(result: &mut [u8; 66]) {
    let rounds = 16u8.wrapping_sub(result[64]);
    if result[64] != 16 {
        for _ in 0..rounds {
            anti_zluda_hash_round_v1(result, rounds);
        }
    }
}

fn anti_zluda_hash_impl_part4<F: FnMut(u32) -> AntiZludaHashInputDevice>(
    result: &mut [u8; 66],
    input: &AntiZludaHashInput,
    mut dev_getter: F,
) {
    for dev in 0..input.device_count {
        let dev_input = dev_getter(dev);
        let part4_input = AntiZludaHashPart4Input {
            guid: dev_input.guid,
            pci_domain: dev_input.pci_domain,
            pci_bus: dev_input.pci_bus,
            pci_device: dev_input.pci_device,
        };
        let part4_input = unsafe { mem::transmute::<_, [u8; 28]>(part4_input) };
        for i in 0..28 {
            anti_zluda_hash_round_v1(result, part4_input[i]);
        }
    }
}

#[repr(C, packed)]
struct AntiZludaHashPart4Input {
    guid: CUuuid,
    pci_domain: u32,
    pci_bus: u32,
    pci_device: u32,
}

fn anti_zluda_hash_impl_part3(
    result: &mut [u8; 66],
    input: &AntiZludaHashInput,
    process_id: u32,
    thread_id: u32,
) {
    let input = AntiZludaHashPart3Input {
        cudart_export_table: input.cudart_export_table,
        anti_zluda_export_table: input.anti_zluda_export_table,
        fn_ptr: input.fn_ptr,
        driver_version: input.driver_version,
        rt_version: input.rt_version,
        timestamp: input.timestamp,
        process_id: process_id,
        thread_id: thread_id,
    };
    let input = unsafe { mem::transmute::<_, [u8; 48]>(input) };
    for i in 0..48 {
        anti_zluda_hash_round_v1(result, input[i]);
    }
}

#[repr(C)]
struct AntiZludaHashPart3Input {
    driver_version: u32,
    rt_version: u32,
    process_id: u32,
    thread_id: u32,
    cudart_export_table: *mut c_void,
    anti_zluda_export_table: *mut c_void,
    fn_ptr: *mut c_void,
    timestamp: u64,
}

fn anti_zluda_hash_impl_part2(result: &mut [u8; 66], aux: [u8; 16]) {
    for i in 0..16 {
        anti_zluda_hash_round_v1(result, aux[i] ^ 0x36);
    }
}

fn anti_zluda_hash_impl_part1(aux: &mut [u8; 16]) {
    let mut i = 13usize;
    let mut temp: u8 = 0x8b;
    loop {
        let part1 = ANTI_ZLUDA_HASH_TABLE_PART1[i + 16]
            ^ ANTI_ZLUDA_HASH_TABLE_PART1[i + 32]
            ^ ANTI_ZLUDA_HASH_TABLE_PART1[i + 48];
        let part2 = temp ^ ANTI_ZLUDA_HASH_TABLE_PART1[i] ^ ANTI_ZLUDA_HASH_TABLE_PART1[i + 16];
        temp = !(part1 ^ temp);
        let offset = part2 as usize >> 4;
        i = (part2 & 0xF) as usize;
        aux[offset] = part1;
        if i == 13 {
            break;
        }
    }
}

const ANTI_ZLUDA_HASH_ROUND_TABLE: [u8; 256] = [
    0x29, 0x2E, 0x43, 0xC9, 0xA2, 0xD8, 0x7C, 0x01, 0x3D, 0x36, 0x54, 0xA1, 0xEC, 0xF0, 0x06, 0x13,
    0x62, 0xA7, 0x05, 0xF3, 0xC0, 0xC7, 0x73, 0x8C, 0x98, 0x93, 0x2B, 0xD9, 0xBC, 0x4C, 0x82, 0xCA,
    0x1E, 0x9B, 0x57, 0x3C, 0xFD, 0xD4, 0xE0, 0x16, 0x67, 0x42, 0x6F, 0x18, 0x8A, 0x17, 0xE5, 0x12,
    0xBE, 0x4E, 0xC4, 0xD6, 0xDA, 0x9E, 0xDE, 0x49, 0xA0, 0xFB, 0xF5, 0x8E, 0xBB, 0x2F, 0xEE, 0x7A,
    0xA9, 0x68, 0x79, 0x91, 0x15, 0xB2, 0x07, 0x3F, 0x94, 0xC2, 0x10, 0x89, 0x0B, 0x22, 0x5F, 0x21,
    0x80, 0x7F, 0x5D, 0x9A, 0x5A, 0x90, 0x32, 0x27, 0x35, 0x3E, 0xCC, 0xE7, 0xBF, 0xF7, 0x97, 0x03,
    0xFF, 0x19, 0x30, 0xB3, 0x48, 0xA5, 0xB5, 0xD1, 0xD7, 0x5E, 0x92, 0x2A, 0xAC, 0x56, 0xAA, 0xC6,
    0x4F, 0xB8, 0x38, 0xD2, 0x96, 0xA4, 0x7D, 0xB6, 0x76, 0xFC, 0x6B, 0xE2, 0x9C, 0x74, 0x04, 0xF1,
    0x45, 0x9D, 0x70, 0x59, 0x64, 0x71, 0x87, 0x20, 0x86, 0x5B, 0xCF, 0x65, 0xE6, 0x2D, 0xA8, 0x02,
    0x1B, 0x60, 0x25, 0xAD, 0xAE, 0xB0, 0xB9, 0xF6, 0x1C, 0x46, 0x61, 0x69, 0x34, 0x40, 0x7E, 0x0F,
    0x55, 0x47, 0xA3, 0x23, 0xDD, 0x51, 0xAF, 0x3A, 0xC3, 0x5C, 0xF9, 0xCE, 0xBA, 0xC5, 0xEA, 0x26,
    0x2C, 0x53, 0x0D, 0x6E, 0x85, 0x28, 0x84, 0x09, 0xD3, 0xDF, 0xCD, 0xF4, 0x41, 0x81, 0x4D, 0x52,
    0x6A, 0xDC, 0x37, 0xC8, 0x6C, 0xC1, 0xAB, 0xFA, 0x24, 0xE1, 0x7B, 0x08, 0x0C, 0xBD, 0xB1, 0x4A,
    0x78, 0x88, 0x95, 0x8B, 0xE3, 0x63, 0xE8, 0x6D, 0xE9, 0xCB, 0xD5, 0xFE, 0x3B, 0x00, 0x1D, 0x39,
    0xF2, 0xEF, 0xB7, 0x0E, 0x66, 0x58, 0xD0, 0xE4, 0xA6, 0x77, 0x72, 0xF8, 0xEB, 0x75, 0x4B, 0x0A,
    0x31, 0x44, 0x50, 0xB4, 0x8F, 0xED, 0x1F, 0x1A, 0xDB, 0x99, 0x8D, 0x33, 0x9F, 0x11, 0x83, 0x14,
];

fn anti_zluda_hash_round_v1(input: &mut [u8], constant: u8) {
    let mut current_index = input[64] as usize;
    input[current_index + 32] = input[current_index] ^ constant;
    input[current_index + 16] = constant;
    input[current_index + 48] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[(input[65] ^ constant) as usize];
    let temp = input[current_index + 48];
    current_index = set_byte_0(current_index, ((current_index + 1) & 0xF) as u8);
    input[65] = temp;
    input[64] = current_index as u8;
    if current_index as u8 == 0 {
        let mut i = 0u8;
        for j in 0u8..18 {
            let mut offset = 2;
            for _ in 0..2 {
                offset += 24;
                input[offset - 26] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[i as usize];
                input[offset - 25] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 26] as usize];
                input[offset - 24] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 25] as usize];
                input[offset - 23] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 24] as usize];
                input[offset - 22] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 23] as usize];
                input[offset - 21] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 22] as usize];
                input[offset - 20] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 21] as usize];
                input[offset - 19] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 20] as usize];
                input[offset - 18] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 19] as usize];
                input[offset - 17] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 18] as usize];
                input[offset - 16] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 17] as usize];
                input[offset - 15] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 16] as usize];
                input[offset - 14] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 15] as usize];
                input[offset - 13] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 14] as usize];
                input[offset - 12] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 13] as usize];
                input[offset - 11] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 12] as usize];
                input[offset - 10] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 11] as usize];
                input[offset - 9] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 10] as usize];
                input[offset - 8] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 9] as usize];
                input[offset - 7] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 8] as usize];
                input[offset - 6] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 7] as usize];
                input[offset - 5] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 6] as usize];
                input[offset - 4] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 5] as usize];
                input[offset - 3] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 4] as usize];
                i = input[offset - 3];
            }
            i = i.wrapping_add(j);
        }
    }
}

fn anti_zluda_hash_round_v2(input: &mut [u8], input2_start: usize) {
    for i in 0..16 {
        let index_2 = input[input2_start + i] as usize;
        let mut index_1 = input[64] as usize;
        input[index_1 + 32] = input[index_1] ^ input[input2_start + i];
        input[index_1 + 16] = index_2 as u8;
        input[index_1 + 48] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[index_2 ^ input[65] as usize];
        let index_3 = input[index_1 + 48];
        index_1 = set_byte_0(index_1, ((index_1 + 1) & 0xF) as u8);
        input[64] = index_1 as u8;
        input[65] = index_3;
        if index_1 == 0 {
            let mut temp = 0u8;
            for j in 0..18 {
                let mut offset = 2;
                for _ in 0..2 {
                    offset += 24;
                    input[offset - 26] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[temp as usize];
                    input[offset - 25] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 26] as usize];
                    input[offset - 24] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 25] as usize];
                    input[offset - 23] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 24] as usize];
                    input[offset - 22] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 23] as usize];
                    input[offset - 21] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 22] as usize];
                    input[offset - 20] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 21] as usize];
                    input[offset - 19] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 20] as usize];
                    input[offset - 18] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 19] as usize];
                    input[offset - 17] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 18] as usize];
                    input[offset - 16] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 17] as usize];
                    input[offset - 15] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 16] as usize];
                    input[offset - 14] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 15] as usize];
                    input[offset - 13] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 14] as usize];
                    input[offset - 12] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 13] as usize];
                    input[offset - 11] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 12] as usize];
                    input[offset - 10] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 11] as usize];
                    input[offset - 9] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 10] as usize];
                    input[offset - 8] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 9] as usize];
                    input[offset - 7] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 8] as usize];
                    input[offset - 6] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 7] as usize];
                    input[offset - 5] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 6] as usize];
                    input[offset - 4] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 5] as usize];
                    input[offset - 3] ^= ANTI_ZLUDA_HASH_ROUND_TABLE[input[offset - 4] as usize];
                    temp = input[offset - 3];
                }
                temp = temp.wrapping_add(j);
            }
        }
    }
}

fn set_byte_0(x: usize, y: u8) -> usize {
    let mut x = x.to_le_bytes();
    x[0] = y;
    usize::from_le_bytes(x)
}

#[cfg(test)]
mod tests {
    use cuda_types::CUuuid;

    use crate::{
        anti_zluda_hash_impl, anti_zluda_hash_round_v1, AntiZludaHashInput,
        AntiZludaHashInputDevice,
    };

    #[test]
    fn anti_zluda_hash_round_sample() {
        let mut input: [u8; 66] = [
            0x8B, 0x21, 0x9A, 0x49, 0xE8, 0x6D, 0x1A, 0xEE, 0xF2, 0x37, 0xF9, 0xB5, 0x4A, 0x8C,
            0x3C, 0x75, 0xF4, 0x1E, 0xEE, 0x21, 0xCF, 0x29, 0x8A, 0xE5, 0x13, 0x83, 0xF4, 0xEC,
            0x33, 0x04, 0xE2, 0xFD, 0x7F, 0x2F, 0x09, 0x01, 0x4F, 0xF7, 0x68, 0x6D, 0x69, 0x46,
            0x43, 0x7E, 0xB6, 0x2B, 0x21, 0xED, 0xB6, 0xA1, 0x10, 0x86, 0x0E, 0x60, 0x44, 0x1E,
            0x70, 0x5F, 0x67, 0xD1, 0xEB, 0x67, 0xA1, 0x3D, 0x01, 0xB6,
        ];
        anti_zluda_hash_round_v1(&mut input, 0x2e);
        let expected: [u8; 66] = [
            0x8B, 0x21, 0x9A, 0x49, 0xE8, 0x6D, 0x1A, 0xEE, 0xF2, 0x37, 0xF9, 0xB5, 0x4A, 0x8C,
            0x3C, 0x75, 0xF4, 0x2E, 0xEE, 0x21, 0xCF, 0x29, 0x8A, 0xE5, 0x13, 0x83, 0xF4, 0xEC,
            0x33, 0x04, 0xE2, 0xFD, 0x7F, 0x0F, 0x09, 0x01, 0x4F, 0xF7, 0x68, 0x6D, 0x69, 0x46,
            0x43, 0x7E, 0xB6, 0x2B, 0x21, 0xED, 0xB6, 0xBD, 0x10, 0x86, 0x0E, 0x60, 0x44, 0x1E,
            0x70, 0x5F, 0x67, 0xD1, 0xEB, 0x67, 0xA1, 0x3D, 0x02, 0xBD,
        ];
        assert_eq!(input, expected);
    }

    #[test]
    fn anti_zluda_hash_impl_sample() {
        let process_id = 0x0000000000004D08;
        let thread_id = 0x0000000000002B78;
        let dev_getter = |_| AntiZludaHashInputDevice {
            pci_domain: 0,
            pci_bus: 4,
            pci_device: 0,
            guid: CUuuid {
                bytes: [
                    0x67, 0x22, 0xCB, 0xCF, 0xC6, 0x61, 0xF2, 0x92, 0x74, 0xD6, 0xED, 0x23, 0x2A,
                    0x32, 0x13, 0x1C,
                ],
            },
        };
        let input = AntiZludaHashInput {
            cudart_export_table: 0x00007FF8C80717F0u64 as _,
            anti_zluda_export_table: 0x00007FF8C825E4B0u64 as _,
            fn_ptr: 0x00007FF8C7DD0AD0u64 as _,
            device_count: 1,
            driver_version: 12020,
            rt_version: 0x2B4A,
            timestamp: 0x0000000064A365EE,
        };
        let result = anti_zluda_hash_impl(false, input, dev_getter, process_id, thread_id);
        assert_eq!(result, 0xEAF1313342BFCD84A7C34628F214707A);
    }
}
