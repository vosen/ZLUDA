use crate::cuda::CUresult;
use crate::{
    cuda::{CUcontext, CUdevice, CUmodule, CUuuid},
    cuda_impl,
};

use super::{context, device, module, Decuda, Encuda};
use std::mem;
use std::os::raw::{c_uint, c_ulong, c_ushort};
use std::{
    ffi::{c_void, CStr},
    ptr, slice,
};

pub fn get(table: *mut *const std::os::raw::c_void, id: *const CUuuid) -> CUresult {
    if table == ptr::null_mut() || id == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let id = unsafe { *id };
    match id {
        TOOLS_RUNTIME_CALLBACK_HOOKS_GUID => {
            unsafe { *table = TOOLS_RUNTIME_CALLBACK_HOOKS_VTABLE.as_ptr() as *const _ };
            CUresult::CUDA_SUCCESS
        }
        CUDART_INTERFACE_GUID => {
            unsafe { *table = CUDART_INTERFACE_VTABLE.as_ptr() as *const _ };
            CUresult::CUDA_SUCCESS
        }
        TOOLS_TLS_GUID => {
            unsafe { *table = 1 as _ };
            CUresult::CUDA_SUCCESS
        }
        CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID => {
            unsafe { *table = CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_VTABLE.as_ptr() as *const _ };
            CUresult::CUDA_SUCCESS
        }
        _ => CUresult::CUDA_ERROR_NOT_SUPPORTED,
    }
}

const TOOLS_RUNTIME_CALLBACK_HOOKS_GUID: CUuuid = CUuuid {
    bytes: [
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
const TOOLS_RUNTIME_CALLBACK_HOOKS_LENGTH: usize = 7;
static TOOLS_RUNTIME_CALLBACK_HOOKS_VTABLE: [VTableEntry; TOOLS_RUNTIME_CALLBACK_HOOKS_LENGTH] = [
    VTableEntry {
        length: mem::size_of::<[VTableEntry; TOOLS_RUNTIME_CALLBACK_HOOKS_LENGTH]>(),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry {
        ptr: runtime_callback_hooks_fn1 as *const (),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
    VTableEntry {
        ptr: runtime_callback_hooks_fn5 as *const (),
    },
];
static mut TOOLS_RUNTIME_CALLBACK_HOOKS_FN1_SPACE: [u8; 512] = [0; 512];

unsafe extern "C" fn runtime_callback_hooks_fn1(ptr: *mut *mut u8, size: *mut usize) -> *mut u8 {
    *ptr = TOOLS_RUNTIME_CALLBACK_HOOKS_FN1_SPACE.as_mut_ptr();
    *size = TOOLS_RUNTIME_CALLBACK_HOOKS_FN1_SPACE.len();
    return TOOLS_RUNTIME_CALLBACK_HOOKS_FN1_SPACE.as_mut_ptr();
}

static mut TOOLS_RUNTIME_CALLBACK_HOOKS_FN5_SPACE: [u8; 2] = [0; 2];

unsafe extern "C" fn runtime_callback_hooks_fn5(ptr: *mut *mut u8, size: *mut usize) -> *mut u8 {
    *ptr = TOOLS_RUNTIME_CALLBACK_HOOKS_FN5_SPACE.as_mut_ptr();
    *size = TOOLS_RUNTIME_CALLBACK_HOOKS_FN5_SPACE.len();
    return TOOLS_RUNTIME_CALLBACK_HOOKS_FN5_SPACE.as_mut_ptr();
}

const CUDART_INTERFACE_GUID: CUuuid = CUuuid {
    bytes: [
        0x6b, 0xd5, 0xfb, 0x6c, 0x5b, 0xf4, 0xe7, 0x4a, 0x89, 0x87, 0xd9, 0x39, 0x12, 0xfd, 0x9d,
        0xf9,
    ],
};

const CUDART_INTERFACE_LENGTH: usize = 10;
static CUDART_INTERFACE_VTABLE: [VTableEntry; CUDART_INTERFACE_LENGTH] = [
    VTableEntry {
        length: mem::size_of::<[VTableEntry; CUDART_INTERFACE_LENGTH]>(),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry {
        ptr: cudart_interface_fn1 as *const (),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
    VTableEntry {
        ptr: get_module_from_cubin as *const (),
    },
    VTableEntry {
        ptr: cudart_interface_fn6 as *const (),
    },
    VTableEntry { ptr: ptr::null() },
    VTableEntry { ptr: ptr::null() },
];

unsafe extern "C" fn cudart_interface_fn1(pctx: *mut CUcontext, dev: CUdevice) -> CUresult {
    cudart_interface_fn1_impl(pctx.decuda(), dev.decuda()).encuda()
}

fn cudart_interface_fn1_impl(
    pctx: *mut *mut context::Context,
    dev: device::Index,
) -> Result<(), CUresult> {
    let ctx_ptr = device::with_exclusive(dev, |d| &mut d.primary_context as *mut context::Context)?;
    unsafe { *pctx = ctx_ptr };
    Ok(())
}

/*
fat_cubin:
typedef struct {
  int magic;
  int version;
  const unsigned long long* data;
  void *filename_or_fatbins;  /* version 1: offline filename,
                               * version 2: array of prelinked fatbins */
} __fatBinC_Wrapper_t;

data start with this header:
#define FATBIN_MAGIC 0xBA55ED50U
#define OLD_STYLE_FATBIN_MAGIC 0x1EE55A01U
#define FATBIN_VERSION 0x0001U

struct fatbinary_ALIGN_(8) fatBinaryHeader
{
  unsigned int           magic;   // FATBIN_MAGIC
  unsigned short         version; // FATBIN_VERSION
  unsigned short         headerSize;
  unsigned long long int fatSize; // size of the entire fat binary excluding this header
};

there's binary data after header

*/

const FATBINC_MAGIC: c_uint = 0x466243B1;
const FATBINC_VERSION: c_uint = 0x1;

#[repr(C)]
struct FatbincWrapper {
    magic: c_uint,
    version: c_uint,
    data: *const FatbinHeader,
    filename_or_fatbins: *const c_void,
}

const FATBIN_MAGIC: c_uint = 0xBA55ED50;
const FATBIN_VERSION: c_ushort = 0x01;

#[repr(C, align(8))]
struct FatbinHeader {
    magic: c_uint,
    version: c_ushort,
    header_size: c_ushort,
    files_size: c_ulong, // excluding frame header, size of all blocks framed by this frame
}

const FATBIN_FILE_HEADER_KIND_PTX: c_ushort = 0x01;
const FATBIN_FILE_HEADER_VERSION_CURRENT: c_ushort = 0x101;

// assembly file header is a bit different, but we don't care
#[repr(C)]
struct FatbinFileHeader {
    kind: c_ushort,
    version: c_ushort,
    header_size: c_uint,
    padded_payload_size: c_uint,
    unknown0: c_uint, // check if it's written into separately
    payload_size: c_uint,
    unknown1: c_uint,
    unknown2: c_uint,
    sm_version: c_uint,
    bit_width: c_uint,
    unknown3: c_uint,
    unknown4: c_ulong,
    unknown5: c_ulong,
    uncompressed_payload: c_ulong,
}

unsafe extern "C" fn get_module_from_cubin(
    result: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
    _: *mut c_void,
    _: *mut c_void,
) -> CUresult {
    if result == ptr::null_mut()
        || (*fatbinc_wrapper).magic != FATBINC_MAGIC
        || (*fatbinc_wrapper).version != FATBINC_VERSION
    {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let result = result.decuda();
    let mut dev_count = 0;
    let cu_result = device::get_count(&mut dev_count);
    if cu_result != CUresult::CUDA_SUCCESS {
        return cu_result;
    }
    let fatbin_header = (*fatbinc_wrapper).data;
    if (*fatbin_header).magic != FATBIN_MAGIC || (*fatbin_header).version != FATBIN_VERSION {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let file = (fatbin_header as *const u8).add((*fatbin_header).header_size as usize);
    let end = file.add((*fatbin_header).files_size as usize);
    let mut ptx_files = get_ptx_files(file, end);
    ptx_files.sort_unstable_by_key(|f| c_uint::max_value() - (**f).sm_version);
    for file in ptx_files {
        let slice = slice::from_raw_parts(
            (file as *const u8).add((*file).header_size as usize),
            (*file).payload_size as usize,
        );
        let kernel_text =
            lz4::block::decompress(slice, Some((*file).uncompressed_payload as i32)).unwrap();
        let kernel_text_string = match CStr::from_bytes_with_nul(&kernel_text) {
            Ok(c_str) => match c_str.to_str() {
                Ok(s) => s,
                Err(_) => continue,
            },
            Err(_) => continue,
        };
        let module = module::Module::compile(kernel_text_string, dev_count as usize);
        match module {
            Ok(module) => {
                *result = Box::into_raw(Box::new(module));
                return CUresult::CUDA_SUCCESS;
            }
            Err(_) => continue,
        }
    }
    CUresult::CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE
}

unsafe fn get_ptx_files(file: *const u8, end: *const u8) -> Vec<*const FatbinFileHeader> {
    let mut index = file;
    let mut result = Vec::new();
    while index < end {
        let file = index as *const FatbinFileHeader;
        if (*file).kind == FATBIN_FILE_HEADER_KIND_PTX
            && (*file).version == FATBIN_FILE_HEADER_VERSION_CURRENT
        {
            result.push(file)
        }
        index = index.add((*file).header_size as usize + (*file).padded_payload_size as usize);
    }
    result
}

unsafe extern "C" fn cudart_interface_fn6(_: u64) {}

const TOOLS_TLS_GUID: CUuuid = CUuuid {
    bytes: [
        0x42, 0xd8, 0x5a, 0x81, 0x23, 0xf6, 0xcb, 0x47, 0x82, 0x98, 0xf6, 0xe7, 0x8a, 0x3a, 0xec,
        0xdc,
    ],
};

const CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID: CUuuid = CUuuid {
    bytes: [
        0xc6, 0x93, 0x33, 0x6e, 0x11, 0x21, 0xdf, 0x11, 0xa8, 0xc3, 0x68, 0xf3, 0x55, 0xd8, 0x95,
        0x93,
    ],
};

// the table is much bigger and starts earlier
static CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_VTABLE: [VTableEntry; 4] = [
    VTableEntry {
        ptr: context_local_storage_ctor as *const (),
    },
    VTableEntry {
        ptr: context_local_storage_dtor as *const (),
    },
    VTableEntry {
        ptr: context_local_storage_get_state as *const (),
    },
    VTableEntry { ptr: ptr::null() },
];

// some kind of ctor
unsafe extern "C" fn context_local_storage_ctor(
    cu_ctx: CUcontext, // always zero
    mgr: *mut cuda_impl::rt::ContextStateManager,
    ctx_state: *mut cuda_impl::rt::ContextState,
    // clsContextDestroyCallback,  have to be called on cuDevicePrimaryCtxReset
    dtor_cb: Option<
        extern "C" fn(
            CUcontext,
            *mut cuda_impl::rt::ContextStateManager,
            *mut cuda_impl::rt::ContextState,
        ),
    >,
) -> CUresult {
    context_local_storage_ctor_impl(cu_ctx.decuda(), mgr, ctx_state, dtor_cb).encuda()
}

fn context_local_storage_ctor_impl(
    cu_ctx: *mut context::Context,
    mgr: *mut cuda_impl::rt::ContextStateManager,
    ctx_state: *mut cuda_impl::rt::ContextState,
    dtor_cb: Option<
        extern "C" fn(
            CUcontext,
            *mut cuda_impl::rt::ContextStateManager,
            *mut cuda_impl::rt::ContextState,
        ),
    >,
) -> Result<(), CUresult> {
    if cu_ctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
    }
    unsafe { &*cu_ctx }
        .as_ref()
        .ok_or(CUresult::CUDA_ERROR_INVALID_CONTEXT)
        .and_then(|ctx| {
            ctx.mutable
                .try_lock()
                .map_err(|_| CUresult::CUDA_ERROR_ILLEGAL_STATE)
                .map(|mut mutable| {
                    mutable.cuda_manager = mgr;
                    mutable.cuda_state = ctx_state;
                    mutable.cuda_dtor_cb = dtor_cb;
                })
        })?;
    Ok(())
}

// some kind of dtor
unsafe extern "C" fn context_local_storage_dtor(_: *mut usize, _: *mut ()) -> u32 {
    0
}

unsafe extern "C" fn context_local_storage_get_state(
    ctx_state: *mut *mut cuda_impl::rt::ContextState,
    cu_ctx: CUcontext,
    state_mgr: *mut cuda_impl::rt::ContextStateManager,
) -> CUresult {
    context_local_storage_get_state_impl(ctx_state, cu_ctx.decuda(), state_mgr).encuda()
}

fn context_local_storage_get_state_impl(
    ctx_state: *mut *mut cuda_impl::rt::ContextState,
    cu_ctx: *mut context::Context,
    _: *mut cuda_impl::rt::ContextStateManager,
) -> Result<(), CUresult> {
    if cu_ctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let cuda_state = unsafe { &*cu_ctx }
        .as_ref()
        .ok_or(CUresult::CUDA_ERROR_INVALID_CONTEXT)
        .and_then(|ctx| {
            ctx.mutable
                .try_lock()
                .map_err(|_| CUresult::CUDA_ERROR_ILLEGAL_STATE)
                .map(|mutable| mutable.cuda_state)
        })?;
    unsafe { *ctx_state = cuda_state };
    Ok(())
}
