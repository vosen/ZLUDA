use cuda_macros::cuda_function_declarations;
use cuda_types::cuda::*;
use dark_api::cuda::CudaDarkApi;
use paste::paste;
use std::sync::{Mutex, OnceLock};
use std::{cell::RefCell, ffi::c_void, ptr};
use zluda_server_common::*;

mod ipc;

macro_rules! not_implemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            #[allow(unused_variables)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                Err(cuda_types::cuda::CUerror::NOT_SUPPORTED)
            }
        )*
    };
}

macro_rules! implemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                paste!{ [< $fn_name:snake >] ( $( $arg_id ),* ) }
            }
        )*

    };
}

cuda_function_declarations! {
    not_implemented,
    implemented <= [
        cuCtxCreate_v2,
        cuCtxDetach,
        cuCtxGetApiVersion,
        cuCtxGetCurrent,
        cuCtxGetDevice,
        cuCtxSynchronize,
        cuDeviceComputeCapability,
        cuDeviceGet,
        cuDeviceGetAttribute,
        cuDeviceGetCount,
        cuDeviceGetName,
        cuDeviceGetProperties,
        cuDeviceTotalMem_v2,
        cuDriverGetVersion,
        cuEventCreate,
        cuEventDestroy_v2,
        cuGetExportTable,
        cuInit,
        cuLaunchKernel,
        cuMemAlloc_v2,
        cuMemFreeHost,
        cuMemFree_v2,
        cuMemGetAddressRange_v2,
        cuMemHostAlloc,
        cuMemcpyDtoDAsync_v2,
        cuMemcpyDtoHAsync_v2,
        cuMemcpyHtoDAsync_v2,
        cuMemsetD8_v2,
        cuModuleGetFunction,
        cuModuleGetGlobal_v2,
        cuModuleGetTexRef,
        cuStreamCreate,
        cuStreamDestroy_v2,
        cuTexRefSetAddressMode,
        cuTexRefSetAddress_v2,
        cuTexRefSetFilterMode,
        cuTexRefSetFlags,
        cuTexRefSetFormat,
        cuTexRefSetMaxAnisotropy,
        cuTexRefSetMipmapFilterMode,
        cuTexRefSetMipmapLevelBias,
        cuTexRefSetMipmapLevelClamp,
    ]
}

thread_local! {
    pub(crate) static CONTEXT_STACK: ContextStack = ContextStack::new();
}

struct ContextStack(RefCell<Vec<CUcontext>>);

impl ContextStack {
    fn new() -> Self {
        ContextStack(RefCell::new(Vec::new()))
    }

    fn push(&self, ctx: CUcontext) {
        self.0.borrow_mut().push(ctx);
    }

    fn unwrap(&self, ctx: CUcontext) -> Result<CUcontext, CUerror> {
        if ctx.0.is_null() {
            let stack = self.0.borrow();
            stack.last().copied().ok_or(CUerror::INVALID_VALUE)
        } else {
            Ok(ctx)
        }
    }

    fn current(&self) -> CUcontext {
        self.0
            .borrow()
            .last()
            .copied()
            .unwrap_or(CUcontext(ptr::null_mut()))
    }
}

pub(crate) fn cu_init(flags: u32) -> Result<(), CUerror> {
    ipc::Server::remote_call_zero_copy::<cuInitOut>(Opcode::cuInit, cuInitIn { Flags: flags })?;
    Ok(())
}

pub(crate) fn cu_ctx_create_v2(
    pctx: *mut CUcontext,
    flags: ::core::ffi::c_uint,
    dev: CUdevice,
) -> Result<(), CUerror> {
    let ctx_ref = unsafe { pctx.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let cu_ctx = CudaEncode::decode(
        ipc::Server::remote_call_zero_copy::<cuCtxCreate_v2Out>(
            Opcode::cuCtxCreate_v2,
            cuCtxCreate_v2In { flags, dev },
        )?
        .pctx,
    );
    CONTEXT_STACK.with(|stack| {
        stack.push(cu_ctx);
    });
    *ctx_ref = cu_ctx;
    Ok(())
}

pub(crate) fn cu_ctx_detach(ctx: CUcontext) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_ctx_get_api_version(
    ctx: CUcontext,
    version: *mut ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    let version = unsafe { version.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let ctx = CONTEXT_STACK.with(|s| s.unwrap(ctx))?;
    *version = CudaEncode::decode(
        ipc::Server::remote_call_zero_copy::<cuCtxGetApiVersionOut>(
            Opcode::cuCtxGetApiVersion,
            cuCtxGetApiVersionIn {
                ctx: CudaEncode::encode(ctx),
            },
        )?
        .version,
    );
    Ok(())
}

pub(crate) fn cu_ctx_get_current(pctx: *mut CUcontext) -> Result<(), CUerror> {
    let pctx = unsafe { pctx.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *pctx = CONTEXT_STACK.with(|s| s.current());
    Ok(())
}

pub(crate) fn cu_ctx_get_device(device: *mut CUdevice) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_ctx_synchronize() -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_device_compute_capability(
    major: *mut ::core::ffi::c_int,
    minor: *mut ::core::ffi::c_int,
    dev: CUdevice,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_device_get(
    device: *mut CUdevice,
    ordinal: ::core::ffi::c_int,
) -> Result<(), CUerror> {
    let device = unsafe { device.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *device = ipc::Server::remote_call_zero_copy::<cuDeviceGetOut>(
        Opcode::cuDeviceGet,
        cuDeviceGetIn { ordinal },
    )?
    .device;
    Ok(())
}

pub(crate) fn cu_device_get_attribute(
    pi: *mut ::core::ffi::c_int,
    attrib: CUdevice_attribute,
    dev: CUdevice,
) -> Result<(), CUerror> {
    let pi = unsafe { pi.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *pi = ipc::Server::remote_call_zero_copy::<cuDeviceGetAttributeOut>(
        Opcode::cuDeviceGetAttribute,
        cuDeviceGetAttributeIn {
            attrib: CudaEncode::encode(attrib),
            dev,
        },
    )?
    .pi;
    Ok(())
}

pub(crate) fn cu_device_get_count(count: *mut ::core::ffi::c_int) -> Result<(), CUerror> {
    let count = unsafe { count.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *count = ipc::Server::remote_call_zero_copy::<cuDeviceGetCountOut>(
        Opcode::cuDeviceGetCount,
        cuDeviceGetCountIn {},
    )?
    .count;
    Ok(())
}

pub(crate) fn cu_device_get_name(
    name: *mut ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    dev: CUdevice,
) -> Result<(), CUerror> {
    if name.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    let name_out = ipc::Server::remote_call_framed::<cuDeviceGetNameOut>(
        Opcode::cuDeviceGetName,
        cuDeviceGetNameIn { len, dev },
    )?;
    unsafe {
        std::ptr::copy_nonoverlapping(
            name_out.name.as_ptr() as *const i8,
            name,
            (len as usize).min(name_out.name.len()),
        )
    };
    Ok(())
}

pub(crate) fn cu_device_get_properties(prop: *mut CUdevprop, dev: CUdevice) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_device_total_mem_v2(bytes: *mut usize, dev: CUdevice) -> Result<(), CUerror> {
    let bytes = unsafe { bytes.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *bytes = ipc::Server::remote_call_zero_copy::<cuDeviceTotalMem_v2Out>(
        Opcode::cuDeviceTotalMem_v2,
        cuDeviceTotalMem_v2In { dev },
    )?
    .bytes
    .to_native()
    .min(u32::MAX as u64) as usize;
    Ok(())
}

pub(crate) fn cu_driver_get_version(
    driver_version: *mut ::core::ffi::c_int,
) -> Result<(), CUerror> {
    let driver_version = unsafe { driver_version.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *driver_version = ipc::Server::remote_call_zero_copy::<cuDriverGetVersionOut>(
        Opcode::cuDriverGetVersion,
        cuDriverGetVersionIn {},
    )?
    .driverVersion;
    Ok(())
}

pub(crate) fn cu_event_create(
    phEvent: *mut CUevent,
    Flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_event_destroy_v2(hEvent: CUevent) -> Result<(), CUerror> {
    unimplemented!()
}

static EXPORT_TABLE: ::dark_api::cuda::CudaDarkApiGlobalTable =
    ::dark_api::cuda::CudaDarkApiGlobalTable::new::<DarkApi32>();

pub(crate) fn cu_get_export_table(
    pp_export_table: *mut *const ::core::ffi::c_void,
    p_export_table_id: *const CUuuid,
) -> Result<(), CUerror> {
    if let (Some(result), Some(guid)) =
        unsafe { (pp_export_table.as_mut(), p_export_table_id.as_ref()) }
    {
        if let Some(table) = EXPORT_TABLE.get(guid) {
            *result = table.start();
            cuda_types::cuda::CUresult::SUCCESS
        } else {
            cuda_types::cuda::CUresult::ERROR_NOT_SUPPORTED
        }
    } else {
        cuda_types::cuda::CUresult::ERROR_INVALID_VALUE
    }
}

pub(crate) fn cu_launch_kernel(
    f: CUfunction,
    gridDimX: ::core::ffi::c_uint,
    gridDimY: ::core::ffi::c_uint,
    gridDimZ: ::core::ffi::c_uint,
    blockDimX: ::core::ffi::c_uint,
    blockDimY: ::core::ffi::c_uint,
    blockDimZ: ::core::ffi::c_uint,
    sharedMemBytes: ::core::ffi::c_uint,
    hStream: CUstream,
    kernelParams: *mut *mut ::core::ffi::c_void,
    extra: *mut *mut ::core::ffi::c_void,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_mem_alloc_v2(dptr: *mut CUdeviceptr, bytesize: usize) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_mem_free_host(p: *mut ::core::ffi::c_void) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_mem_free_v2(dptr: CUdeviceptr) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_mem_get_address_range_v2(
    pbase: *mut CUdeviceptr,
    psize: *mut usize,
    dptr: CUdeviceptr,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_mem_host_alloc(
    pp: *mut *mut ::core::ffi::c_void,
    bytesize: usize,
    Flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_memcpy_dto_d_async_v2(
    dstDevice: CUdeviceptr,
    srcDevice: CUdeviceptr,
    ByteCount: usize,
    hStream: CUstream,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_memcpy_dto_h_async_v2(
    dstHost: *mut ::core::ffi::c_void,
    srcDevice: CUdeviceptr,
    ByteCount: usize,
    hStream: CUstream,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_memcpy_hto_d_async_v2(
    dstDevice: CUdeviceptr,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
    hStream: CUstream,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_memset_d8_v2(
    dstDevice: CUdeviceptr,
    uc: ::core::ffi::c_uchar,
    N: usize,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_module_get_function(
    hfunc: *mut CUfunction,
    hmod: CUmodule,
    name: *const ::core::ffi::c_char,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_module_get_global_v2(
    dptr: *mut CUdeviceptr,
    bytes: *mut usize,
    hmod: CUmodule,
    name: *const ::core::ffi::c_char,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_module_get_tex_ref(
    pTexRef: *mut CUtexref,
    hmod: CUmodule,
    name: *const ::core::ffi::c_char,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_stream_create(
    phStream: *mut CUstream,
    Flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_stream_destroy_v2(hStream: CUstream) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_tex_ref_set_address_mode(
    hTexRef: CUtexref,
    dim: ::core::ffi::c_int,
    am: CUaddress_mode,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_tex_ref_set_address_v2(
    ByteOffset: *mut usize,
    hTexRef: CUtexref,
    dptr: CUdeviceptr,
    bytes: usize,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_tex_ref_set_filter_mode(
    hTexRef: CUtexref,
    fm: CUfilter_mode,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_tex_ref_set_flags(
    hTexRef: CUtexref,
    Flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_tex_ref_set_format(
    hTexRef: CUtexref,
    fmt: CUarray_format,
    NumPackedComponents: ::core::ffi::c_int,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_tex_ref_set_max_anisotropy(
    hTexRef: CUtexref,
    maxAniso: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_tex_ref_set_mipmap_filter_mode(
    hTexRef: CUtexref,
    fm: CUfilter_mode,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_tex_ref_set_mipmap_level_bias(
    hTexRef: CUtexref,
    bias: f32,
) -> Result<(), CUerror> {
    unimplemented!()
}

pub(crate) fn cu_tex_ref_set_mipmap_level_clamp(
    hTexRef: CUtexref,
    minMipmapLevelClamp: f32,
    maxMipmapLevelClamp: f32,
) -> Result<(), CUerror> {
    unimplemented!()
}

struct UnknownBuffer<const S: usize> {
    buffer: std::cell::UnsafeCell<[u32; S]>,
}

impl<const S: usize> UnknownBuffer<S> {
    const fn new() -> Self {
        UnknownBuffer {
            buffer: std::cell::UnsafeCell::new([0; S]),
        }
    }
    const fn len(&self) -> usize {
        S
    }
}

unsafe impl<const S: usize> Sync for UnknownBuffer<S> {}

static UNKNOWN_BUFFER1: UnknownBuffer<1024> = UnknownBuffer::new();
static UNKNOWN_BUFFER2: UnknownBuffer<14> = UnknownBuffer::new();

struct DarkApi32;

impl CudaDarkApi for DarkApi32 {
    unsafe extern "system" fn get_module_from_cubin(
        module: *mut cuda_types::cuda::CUmodule,
        fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn cudart_interface_fn2(
        pctx: *mut cuda_types::cuda::CUcontext,
        _dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult {
        let ctx = unsafe { pctx.as_mut() }.ok_or(cuda_types::cuda::CUerror::INVALID_VALUE)?;
        *ctx = CUcontext(0xDEADBEEFu32 as _);
        cuda_types::cuda::CUresult::SUCCESS
    }

    unsafe extern "system" fn get_module_from_cubin_ext1(
        result: *mut cuda_types::cuda::CUmodule,
        fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
        arg3: *mut std::ffi::c_void,
        arg4: *mut std::ffi::c_void,
        arg5: u32,
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn cudart_interface_fn7(arg1: usize) -> () {
        unimplemented!()
    }

    unsafe extern "system" fn get_module_from_cubin_ext2(
        fatbin_header: *const cuda_types::dark_api::FatbinHeader,
        result: *mut cuda_types::cuda::CUmodule,
        arg3: *mut std::ffi::c_void,
        arg4: *mut std::ffi::c_void,
        arg5: u32,
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn load_compilers() -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn get_unknown_buffer1(
        ptr: *mut *mut std::ffi::c_void,
        size: *mut usize,
    ) -> () {
        *ptr = UNKNOWN_BUFFER1.buffer.get() as *mut std::ffi::c_void;
        *size = UNKNOWN_BUFFER1.len();
    }

    unsafe extern "system" fn get_unknown_buffer2(
        ptr: *mut *mut std::ffi::c_void,
        size: *mut usize,
    ) -> () {
        *ptr = UNKNOWN_BUFFER2.buffer.get() as *mut std::ffi::c_void;
        *size = UNKNOWN_BUFFER2.len();
    }

    unsafe extern "system" fn context_local_storage_put(
        cu_ctx: cuda_types::cuda::CUcontext,
        key: *mut std::ffi::c_void,
        value: *mut std::ffi::c_void,
        _dtor_cb: Option<
            extern "system" fn(
                cuda_types::cuda::CUcontext,
                *mut std::ffi::c_void,
                *mut std::ffi::c_void,
            ),
        >,
    ) -> cuda_types::cuda::CUresult {
        let cu_ctx = CONTEXT_STACK.with(|s| s.unwrap(cu_ctx))?;
        ipc::Server::remote_call_zero_copy::<ContextLocalStoragePutOut>(
            Opcode::ContextLocalStoragePut,
            ContextLocalStoragePutIn {
                cu_ctx: cu_ctx.encode().into(),
                key: (key as u32).into(),
                value: (value as u32).into(),
            },
        )?;
        Ok(())
    }

    unsafe extern "system" fn context_local_storage_delete(
        context: cuda_types::cuda::CUcontext,
        key: *mut std::ffi::c_void,
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn context_local_storage_get(
        value: *mut *mut std::ffi::c_void,
        cu_ctx: cuda_types::cuda::CUcontext,
        key: *mut std::ffi::c_void,
    ) -> cuda_types::cuda::CUresult {
        let value = unsafe { value.as_mut() }.ok_or(cuda_types::cuda::CUerror::INVALID_VALUE)?;
        let cu_ctx = CONTEXT_STACK.with(|s| s.unwrap(cu_ctx))?;
        *value = ipc::Server::remote_call_zero_copy::<ContextLocalStorageGetOut>(
            Opcode::ContextLocalStorageGet,
            ContextLocalStorageGetIn {
                cu_ctx: cu_ctx.encode().into(),
                key: (key as u32).into(),
            },
        )?
        .value
        .to_native() as _;
        Ok(())
    }

    unsafe extern "system" fn ctx_create_v2_bypass(
        pctx: *mut cuda_types::cuda::CUcontext,
        flags: ::std::os::raw::c_uint,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult {
        cu_ctx_create_v2(pctx, flags, dev)
    }

    unsafe extern "system" fn heap_alloc(
        heap_alloc_record_ptr: *mut *const std::ffi::c_void,
        arg2: usize,
        arg3: usize,
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn heap_free(
        heap_alloc_record_ptr: *const std::ffi::c_void,
        arg2: *mut usize,
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn device_get_attribute_ext(
        dev: cuda_types::cuda::CUdevice,
        attribute: std::ffi::c_uint,
        unknown: std::ffi::c_int,
        result: *mut [usize; 2],
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn device_get_something(
        result: *mut std::ffi::c_uchar,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn integrity_check(
        version: u32,
        unix_seconds: u64,
        result: *mut [u64; 2],
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn context_check(
        ctx_in: cuda_types::cuda::CUcontext,
        result1: *mut u32,
        result2: *mut *const std::ffi::c_void,
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn check_fn3() -> u32 {
        unimplemented!()
    }

    unsafe extern "system" fn hybrid_runtime_load_get_proc_address(
        name: *const std::ffi::c_char,
        fn_ptr: *mut *const std::ffi::c_void,
        token: *mut usize,
    ) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }

    unsafe extern "system" fn hybrid_runtime_free(token: usize) -> cuda_types::cuda::CUresult {
        unimplemented!()
    }
}
