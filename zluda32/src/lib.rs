use cuda_macros::cuda_function_declarations;
use cuda_types::cuda::*;
use dark_api::cuda::CudaDarkApi;
use dark_api::FunctionArgInfo;
use paste::paste;
use rkyv::api::high::HighSerializer;
use rkyv::de::Pool;
use rkyv::rancor::{Failure, Strategy};
use rkyv::rend::u32_le;
use rkyv::ser::allocator::ArenaHandle;
use rkyv::util::AlignedVec;
use rkyv::{Archive, Deserialize, Portable, Serialize};
use rustc_hash::FxHashMap;
use std::alloc::Layout;
use std::slice;
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, OnceLock};
use std::{cell::RefCell, ffi::c_void, ptr};
use zluda_common::{CodeLibraryRef, CodeModuleRef, CudaErrorType};
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
                if !crate::initialized() {
                    return Err(CUerror::DEINITIALIZED);
                }
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
                if !crate::initialized() {
                    return Err(CUerror::DEINITIALIZED);
                }
                paste!{ [< $fn_name:snake >] ( $( $arg_id ),* ) }
            }
        )*

    };
}

cuda_function_declarations! {
    not_implemented,
    implemented <= [
        cuCtxCreate,
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
        cuMemcpyDtoD_v2,
        cuMemcpyDtoDAsync_v2,
        cuMemcpyDtoHAsync_v2,
        cuMemcpyHtoDAsync_v2,
        cuMemFree_v2,
        cuMemFreeHost,
        cuMemGetAddressRange_v2,
        cuMemHostAlloc,
        cuMemsetD8_v2,
        cuModuleGetFunction,
        cuModuleGetGlobal_v2,
        cuModuleGetTexRef,
        cuStreamCreate,
        cuStreamDestroy_v2,
        cuTexRefSetAddress_v2,
        cuTexRefSetAddressMode,
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

struct ContextStack(RefCell<Vec<(CUcontext, CUdevice)>>);

impl ContextStack {
    fn new() -> Self {
        ContextStack(RefCell::new(Vec::new()))
    }

    fn push(&self, ctx: CUcontext, dev: CUdevice) {
        self.0.borrow_mut().push((ctx, dev));
    }

    fn unwrap(&self, ctx: CUcontext) -> Result<CUcontext, CUerror> {
        if ctx.0.is_null() {
            let stack = self.0.borrow();
            stack
                .last()
                .map(|(ctx, _)| *ctx)
                .ok_or(CUerror::INVALID_VALUE)
        } else {
            Ok(ctx)
        }
    }

    fn current(&self) -> (CUcontext, CUdevice) {
        self.0
            .borrow()
            .last()
            .copied()
            .unwrap_or((CUcontext(ptr::null_mut()), 0))
    }
}

struct GlobalState {
    server: ipc::Server,
    function_args: FxHashMap<CUfunction, Vec<FunctionArgInfo>>,
    next_context: u32,
    context_state: FxHashMap<
        cuda_types::cuda::CUcontext,
        FxHashMap<
            usize,
            (
                usize,
                Option<
                    extern "system" fn(
                        cuda_types::cuda::CUcontext,
                        *mut std::ffi::c_void,
                        *mut std::ffi::c_void,
                    ),
                >,
            ),
        >,
    >,
}

impl GlobalState {
    fn get() -> Result<&'static Mutex<Self>, CUerror> {
        static LOCK: OnceLock<Result<Mutex<GlobalState>, windows::core::Error>> = OnceLock::new();
        LOCK.get_or_init(|| {
            Ok(Mutex::new(GlobalState {
                function_args: FxHashMap::default(),
                server: unsafe { ipc::Server::start() }?,
                next_context: 1024 * 1024,
                context_state: FxHashMap::default(),
            }))
        })
        .as_ref()
        .map_err(|_| CUerror::UNKNOWN)
    }

    pub(crate) fn remote_call_zero_copy<Out: Portable + Clone>(
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<
            HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
        >,
    ) -> Result<Out, CUerror> {
        Self::get()?
            .lock()
            .map_err(|_| CUerror::UNKNOWN)?
            .server
            .remote_call_zero_copy(opcode, data)
    }

    pub(crate) fn remote_call_framed_in<Out: Portable + Clone>(
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<
            HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
        >,
    ) -> Result<Out, CUerror> {
        Self::get()?
            .lock()
            .map_err(|_| CUerror::UNKNOWN)?
            .server
            .remote_call_framed_in(opcode, data)
    }

    pub(crate) fn remote_call_framed_out<Out: Archive>(
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<
            HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
        >,
    ) -> Result<Out, CUerror>
    where
        <Out as Archive>::Archived: Deserialize<Out, Strategy<Pool, Failure>>,
    {
        Self::get()?
            .lock()
            .map_err(|_| CUerror::UNKNOWN)?
            .server
            .remote_call_framed_out(opcode, data)
    }
}

pub(crate) fn cu_init(flags: u32) -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuInitOut>(Opcode::cuInit, cuInitIn { Flags: flags })?;
    Ok(())
}

pub(crate) fn cu_ctx_create(
    pctx: *mut CUcontext,
    flags: ::core::ffi::c_uint,
    dev: CUdevice,
) -> Result<(), CUerror> {
    return cu_ctx_create_v2(pctx, flags, dev);
}

pub(crate) fn cu_ctx_create_v2(
    pctx: *mut CUcontext,
    _flags: ::core::ffi::c_uint,
    dev: CUdevice,
) -> Result<(), CUerror> {
    let ctx_ref = unsafe { pctx.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let cu_ctx: CUcontext;
    {
        let mut state = GlobalState::get()?.lock().map_err(|_| CUerror::UNKNOWN)?;
        state.next_context = state.next_context.wrapping_add(1);
        let new_ctx = CUcontext(state.next_context as *mut _);
        state.context_state.insert(new_ctx, FxHashMap::default());
        cu_ctx = new_ctx;
    }
    CONTEXT_STACK.with(|stack| {
        stack.push(cu_ctx, dev);
    });
    *ctx_ref = cu_ctx;
    Ok(())
}

pub(crate) fn cu_ctx_detach(ctx: CUcontext) -> Result<(), CUerror> {
    let ctx_storage = {
        let mut state = GlobalState::get()?.lock().map_err(|_| CUerror::UNKNOWN)?;
        state.context_state.remove(&ctx).ok_or(CUerror::UNKNOWN)?
    };
    for (key, (value, dtor_cb)) in ctx_storage {
        if let Some(dtor_cb) = dtor_cb {
            dtor_cb(ctx, key as _, value as _);
        }
    }
    Ok(())
}

pub(crate) fn cu_ctx_get_api_version(
    ctx: CUcontext,
    version: *mut ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    let version = unsafe { version.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let ctx = CONTEXT_STACK.with(|s| s.unwrap(ctx))?;
    *version = CudaEncode::decode(
        GlobalState::remote_call_zero_copy::<cuCtxGetApiVersionOut>(
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
    *pctx = CONTEXT_STACK.with(|s| s.current()).0;
    Ok(())
}

pub(crate) fn cu_ctx_get_device(device: *mut CUdevice) -> Result<(), CUerror> {
    let device = unsafe { device.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let (_, dev) = CONTEXT_STACK.with(|s| s.current());
    *device = dev;
    Ok(())
}

pub(crate) fn cu_ctx_synchronize() -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuCtxSynchronizeOut>(
        Opcode::cuCtxSynchronize,
        cuCtxSynchronizeIn {},
    )?;
    Ok(())
}

pub(crate) fn cu_device_compute_capability(
    major: *mut ::core::ffi::c_int,
    minor: *mut ::core::ffi::c_int,
    dev: CUdevice,
) -> Result<(), CUerror> {
    let major = unsafe { major.as_mut() };
    let minor = unsafe { minor.as_mut() };
    let capability = GlobalState::remote_call_zero_copy::<cuDeviceComputeCapabilityOut>(
        Opcode::cuDeviceComputeCapability,
        cuDeviceComputeCapabilityIn { dev },
    )?;
    if let Some(major) = major {
        *major = capability.major;
    }
    if let Some(minor) = minor {
        *minor = capability.minor;
    }
    Ok(())
}

pub(crate) fn cu_device_get(
    device: *mut CUdevice,
    ordinal: ::core::ffi::c_int,
) -> Result<(), CUerror> {
    let device = unsafe { device.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *device = GlobalState::remote_call_zero_copy::<cuDeviceGetOut>(
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
    if attrib.0 == 537396226 {
        *pi = 256;
        return Ok(());
    }
    *pi = GlobalState::remote_call_zero_copy::<cuDeviceGetAttributeOut>(
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
    *count = GlobalState::remote_call_zero_copy::<cuDeviceGetCountOut>(
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
    let name_out = GlobalState::remote_call_framed_out::<cuDeviceGetNameOut>(
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

pub(crate) fn cu_device_get_properties(
    result: *mut CUdevprop,
    dev: CUdevice,
) -> Result<(), CUerror> {
    let result = unsafe { result.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let prop = GlobalState::remote_call_zero_copy::<cuDeviceGetPropertiesOut>(
        Opcode::cuDeviceGetProperties,
        cuDeviceGetPropertiesIn { dev },
    )?
    .prop;
    *result = prop.into();
    Ok(())
}

pub(crate) fn cu_device_total_mem_v2(bytes: *mut usize, dev: CUdevice) -> Result<(), CUerror> {
    let bytes = unsafe { bytes.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *bytes = GlobalState::remote_call_zero_copy::<cuDeviceTotalMem_v2Out>(
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
    *driver_version = GlobalState::remote_call_zero_copy::<cuDriverGetVersionOut>(
        Opcode::cuDriverGetVersion,
        cuDriverGetVersionIn {},
    )?
    .driverVersion;
    Ok(())
}

pub(crate) fn cu_event_create(
    result: *mut CUevent,
    flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    let result = unsafe { result.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *result = CudaEncode::decode(
        GlobalState::remote_call_zero_copy::<cuEventCreateOut>(
            Opcode::cuEventCreate,
            cuEventCreateIn { Flags: flags },
        )?
        .phEvent,
    );
    Ok(())
}

pub(crate) fn cu_event_destroy_v2(event: CUevent) -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuEventDestroy_v2Out>(
        Opcode::cuEventDestroy_v2,
        cuEventDestroy_v2In {
            hEvent: CudaEncode::encode(event),
        },
    )?;
    Ok(())
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
    grid_dim_x: ::core::ffi::c_uint,
    grid_dim_y: ::core::ffi::c_uint,
    grid_dim_z: ::core::ffi::c_uint,
    block_dim_x: ::core::ffi::c_uint,
    block_dim_y: ::core::ffi::c_uint,
    block_dim_z: ::core::ffi::c_uint,
    shared_mem_bytes: ::core::ffi::c_uint,
    stream: CUstream,
    kernel_params: *mut *mut ::core::ffi::c_void,
    extra: *mut *mut ::core::ffi::c_void,
) -> Result<(), CUerror> {
    if !kernel_params.is_null() || extra.is_null() {
        return Err(CUerror::NOT_SUPPORTED);
    }
    let mut state = GlobalState::get()?.lock().map_err(|_| CUerror::UNKNOWN)?;
    let args = state.function_args.get(&f).ok_or(CUerror::INVALID_VALUE)?;
    let extra_slice = unsafe { std::slice::from_raw_parts(extra, 5) };
    if extra_slice[0] != CU_LAUNCH_PARAM_BUFFER_POINTER_AS_INT as _ {
        return Err(CUerror::INVALID_VALUE);
    }
    if extra_slice[2] != CU_LAUNCH_PARAM_BUFFER_SIZE_AS_INT as _ {
        return Err(CUerror::INVALID_VALUE);
    }
    if extra_slice[4] != CU_LAUNCH_PARAM_END_AS_INT as _ {
        return Err(CUerror::INVALID_VALUE);
    }
    let arg_ptr = extra_slice[1];
    let arg_size = unsafe { *extra_slice[3].cast::<usize>() };
    let mut total_size = 0;
    let kernel_params = args
        .iter()
        .scan(
            unsafe { Layout::from_size_align_unchecked(0, 1) },
            |layout, arg_info| {
                let (new_layout, offset) = layout.extend(arg_info.to_layout().unwrap()).unwrap();
                total_size = offset + arg_info.size as usize;
                *layout = new_layout;
                let start = arg_ptr.wrapping_byte_add(offset as usize).cast::<u8>();
                Some(Ok(unsafe {
                    std::slice::from_raw_parts(start, arg_info.size as usize)
                }
                .to_vec()))
            },
        )
        .collect::<Result<Vec<_>, CUerror>>()?;
    if total_size as usize != arg_size {
        return Err(CUerror::INVALID_VALUE);
    }
    state.server.remote_call_framed_in::<cuLaunchKernelOut>(
        Opcode::cuLaunchKernel,
        cuLaunchKernelIn {
            f: CudaEncode::encode(f),
            grid_dim_x: u32_le::from_native(grid_dim_x),
            grid_dim_y: u32_le::from_native(grid_dim_y),
            grid_dim_z: u32_le::from_native(grid_dim_z),
            block_dim_x: u32_le::from_native(block_dim_x),
            block_dim_y: u32_le::from_native(block_dim_y),
            block_dim_z: u32_le::from_native(block_dim_z),
            shared_mem_bytes: u32_le::from_native(shared_mem_bytes),
            stream: CudaEncode::encode(stream),
            kernel_params,
        },
    )?;
    Ok(())
}

pub(crate) fn cu_mem_alloc_v2(dptr: *mut CUdeviceptr, bytesize: usize) -> Result<(), CUerror> {
    let dptr = unsafe { dptr.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *dptr = CudaEncode::decode(
        GlobalState::remote_call_zero_copy::<cuMemAlloc_v2Out>(
            Opcode::cuMemAlloc_v2,
            cuMemAlloc_v2In {
                bytesize: u32_le::from_native(bytesize as u32),
            },
        )?
        .dptr,
    );
    Ok(())
}

pub(crate) fn cu_mem_free_v2(dptr: CUdeviceptr) -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuMemFree_v2Out>(
        Opcode::cuMemFree_v2,
        cuMemFree_v2In {
            dptr: CudaEncode::encode(dptr),
        },
    )?;
    Ok(())
}

pub(crate) fn cu_mem_get_address_range_v2(
    pbase: *mut CUdeviceptr,
    psize: *mut usize,
    dptr: CUdeviceptr,
) -> Result<(), CUerror> {
    let pbase = unsafe { pbase.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let psize = unsafe { psize.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let result = GlobalState::remote_call_zero_copy::<cuMemGetAddressRange_v2Out>(
        Opcode::cuMemGetAddressRange_v2,
        cuMemGetAddressRange_v2In {
            dptr: CudaEncode::encode(dptr),
        },
    )?;
    *pbase = CudaEncode::decode(result.pbase);
    *psize = result.psize.to_native() as usize;
    Ok(())
}

pub(crate) fn cu_mem_host_alloc(
    result: *mut *mut ::core::ffi::c_void,
    bytesize: usize,
    _flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    let result = unsafe { result.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *result = unsafe { libc::calloc(1, bytesize) };
    Ok(())
}

pub(crate) fn cu_mem_free_host(p: *mut ::core::ffi::c_void) -> Result<(), CUerror> {
    if p.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    unsafe { libc::free(p) };
    Ok(())
}

pub(crate) fn cu_memcpy_dto_d_v2(
    dst_device: CUdeviceptr,
    src_device: CUdeviceptr,
    byte_count: usize,
) -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuMemcpyDtoD_v2Out>(
        Opcode::cuMemcpyDtoD_v2,
        cuMemcpyDtoD_v2In {
            dstDevice: CudaEncode::encode(dst_device),
            srcDevice: CudaEncode::encode(src_device),
            ByteCount: u32_le::from_native(byte_count as u32),
        },
    )?;
    Ok(())
}

pub(crate) fn cu_memcpy_dto_d_async_v2(
    dst_device: CUdeviceptr,
    src_device: CUdeviceptr,
    byte_count: usize,
    stream: CUstream,
) -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuMemcpyDtoDAsync_v2Out>(
        Opcode::cuMemcpyDtoDAsync_v2,
        cuMemcpyDtoDAsync_v2In {
            dstDevice: CudaEncode::encode(dst_device),
            srcDevice: CudaEncode::encode(src_device),
            ByteCount: u32_le::from_native(byte_count as u32),
            hStream: CudaEncode::encode(stream),
        },
    )?;
    Ok(())
}

pub(crate) fn cu_memcpy_dto_h_async_v2(
    dst_host: *mut ::core::ffi::c_void,
    src_device: CUdeviceptr,
    byte_count: usize,
    stream: CUstream,
) -> Result<(), CUerror> {
    if dst_host.is_null() || src_device.0.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    let dst_slice = unsafe { std::slice::from_raw_parts_mut(dst_host.cast::<u8>(), byte_count) };
    let result = GlobalState::remote_call_framed_out::<cuMemcpyDtoHAsync_v2Out>(
        Opcode::cuMemcpyDtoHAsync_v2,
        cuMemcpyDtoHAsync_v2In {
            src_device: CudaEncode::encode(src_device),
            stream: CudaEncode::encode(stream),
            byte_count: u32_le::from_native(byte_count as u32),
        },
    )?;
    dst_slice.copy_from_slice(&result.dst_host);
    Ok(())
}

pub(crate) fn cu_memcpy_hto_d_async_v2(
    dst_device: CUdeviceptr,
    src_host: *const ::core::ffi::c_void,
    byte_count: usize,
    h_stream: CUstream,
) -> Result<(), CUerror> {
    if src_host.is_null() || dst_device.0.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    let slice = unsafe { std::slice::from_raw_parts(src_host.cast::<u8>(), byte_count) };
    GlobalState::remote_call_framed_in::<cuMemcpyHtoDAsync_v2Out>(
        Opcode::cuMemcpyHtoDAsync_v2,
        cuMemcpyHtoDAsync_v2In {
            dst_device: CudaEncode::encode(dst_device),
            src_host: slice.to_vec(),
            stream: CudaEncode::encode(h_stream),
        },
    )?;
    Ok(())
}

pub(crate) fn cu_memset_d8_v2(
    dst_device: CUdeviceptr,
    uc: ::core::ffi::c_uchar,
    n: usize,
) -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuMemsetD8_v2Out>(
        Opcode::cuMemsetD8_v2,
        cuMemsetD8_v2In {
            dstDevice: CudaEncode::encode(dst_device),
            uc,
            N: u32_le::from_native(n as u32),
        },
    )?;
    Ok(())
}

pub(crate) fn cu_module_get_function(
    hfunc: *mut CUfunction,
    hmod: CUmodule,
    name: *const ::core::ffi::c_char,
) -> Result<(), CUerror> {
    let hfunc = unsafe { hfunc.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let mut state = GlobalState::get()?.lock().map_err(|_| CUerror::UNKNOWN)?;
    *hfunc = CUfunction(
        state
            .server
            .remote_call_framed_in::<cuModuleGetFunctionOut>(
                Opcode::cuModuleGetFunction,
                cuModuleGetFunctionIn {
                    hmod: CudaEncode::encode(hmod),
                    name: unsafe { std::ffi::CStr::from_ptr(name) }
                        .to_bytes_with_nul()
                        .to_vec(),
                },
            )?
            .hfunc
            .to_native() as _,
    );
    let arg_sizes_out = state
        .server
        .remote_call_framed_out::<zludaGetFunctionArgsOut>(
            Opcode::zludaGetFunctionArgs,
            zludaGetFunctionArgsIn {
                f: CudaEncode::encode(*hfunc),
            },
        )?;
    state.function_args.insert(*hfunc, arg_sizes_out.args);
    Ok(())
}

pub(crate) fn cu_module_get_global_v2(
    dptr: *mut CUdeviceptr,
    bytes: *mut usize,
    hmod: CUmodule,
    name: *const ::core::ffi::c_char,
) -> Result<(), CUerror> {
    let mut bytes_fallback = 0usize;
    let dptr = unsafe { dptr.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let bytes = unsafe { bytes.as_mut() }.unwrap_or(&mut bytes_fallback);
    let result = GlobalState::remote_call_framed_in::<cuModuleGetGlobal_v2Out>(
        Opcode::cuModuleGetGlobal_v2,
        cuModuleGetGlobal_v2In {
            hmod: CudaEncode::encode(hmod),
            name: unsafe { std::ffi::CStr::from_ptr(name) }
                .to_bytes()
                .to_vec(),
        },
    )?;
    *dptr = CudaEncode::decode(result.dptr);
    *bytes = result.bytes.to_native() as usize;
    Ok(())
}

pub(crate) fn cu_module_get_tex_ref(
    texref: *mut CUtexref,
    hmod: CUmodule,
    name: *const ::core::ffi::c_char,
) -> Result<(), CUerror> {
    let texref = unsafe { texref.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    let result = GlobalState::remote_call_framed_in::<cuModuleGetTexRefOut>(
        Opcode::cuModuleGetTexRef,
        cuModuleGetTexRefIn {
            hmod: CudaEncode::encode(hmod),
            name: unsafe { std::ffi::CStr::from_ptr(name) }
                .to_bytes_with_nul()
                .to_vec(),
        },
    )?;
    *texref = CudaEncode::decode(result.texref);
    Ok(())
}

pub(crate) fn cu_stream_create(
    result: *mut CUstream,
    flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    let result = unsafe { result.as_mut() }.ok_or(CUerror::INVALID_VALUE)?;
    *result = CudaEncode::decode(
        GlobalState::remote_call_zero_copy::<cuStreamCreateOut>(
            Opcode::cuStreamCreate,
            cuStreamCreateIn { Flags: flags },
        )?
        .phStream,
    );
    Ok(())
}

pub(crate) fn cu_stream_destroy_v2(stream: CUstream) -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuStreamDestroy_v2Out>(
        Opcode::cuStreamDestroy_v2,
        cuStreamDestroy_v2In {
            hStream: CudaEncode::encode(stream),
        },
    )?;
    Ok(())
}

pub(crate) fn cu_tex_ref_set_address_mode(
    _tex_ref: CUtexref,
    _dim: ::core::ffi::c_int,
    _am: CUaddress_mode,
) -> Result<(), CUerror> {
    // TODO
    Ok(())
}

pub(crate) fn cu_tex_ref_set_address_v2(
    byte_offset: *mut usize,
    tex_ref: CUtexref,
    dptr: CUdeviceptr,
    bytes: usize,
) -> Result<(), CUerror> {
    let mut bytes_fallback = 0usize;
    let byte_offset = unsafe { byte_offset.as_mut() }.unwrap_or(&mut bytes_fallback);
    let result = GlobalState::remote_call_zero_copy::<cuTexRefSetAddress_v2Out>(
        Opcode::cuTexRefSetAddress_v2,
        cuTexRefSetAddress_v2In {
            hTexRef: CudaEncode::encode(tex_ref),
            dptr: CudaEncode::encode(dptr),
            bytes: u32_le::from_native(bytes as u32),
        },
    )?;
    *byte_offset = CudaEncode::decode(result.ByteOffset);
    Ok(())
}

pub(crate) fn cu_tex_ref_set_filter_mode(
    _tex_ref: CUtexref,
    _fm: CUfilter_mode,
) -> Result<(), CUerror> {
    // TODO
    Ok(())
}

pub(crate) fn cu_tex_ref_set_flags(
    tex_ref: CUtexref,
    flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuTexRefSetFlagsOut>(
        Opcode::cuTexRefSetFlags,
        cuTexRefSetFlagsIn {
            hTexRef: CudaEncode::encode(tex_ref),
            Flags: flags,
        },
    )?;
    Ok(())
}

pub(crate) fn cu_tex_ref_set_format(
    tex_ref: CUtexref,
    fmt: CUarray_format,
    num_packed_components: ::core::ffi::c_int,
) -> Result<(), CUerror> {
    GlobalState::remote_call_zero_copy::<cuTexRefSetFormatOut>(
        Opcode::cuTexRefSetFormat,
        cuTexRefSetFormatIn {
            hTexRef: CudaEncode::encode(tex_ref),
            fmt: fmt.0.into(),
            NumPackedComponents: num_packed_components,
        },
    )?;
    Ok(())
}

pub(crate) fn cu_tex_ref_set_max_anisotropy(
    _tex_ref: CUtexref,
    _max_aniso: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    // TODO
    Ok(())
}

pub(crate) fn cu_tex_ref_set_mipmap_filter_mode(
    _tex_ref: CUtexref,
    _fm: CUfilter_mode,
) -> Result<(), CUerror> {
    // TODO
    Ok(())
}

pub(crate) fn cu_tex_ref_set_mipmap_level_bias(
    _tex_ref: CUtexref,
    _bias: f32,
) -> Result<(), CUerror> {
    // TODO
    Ok(())
}

pub(crate) fn cu_tex_ref_set_mipmap_level_clamp(
    _tex_ref: CUtexref,
    _min_mipmap_level_clamp: f32,
    _max_mipmap_level_clamp: f32,
) -> Result<(), CUerror> {
    // TODO
    Ok(())
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

#[repr(C)]
struct AllocRecord {
    x: usize,
    y: usize,
}

impl CudaDarkApi for DarkApi32 {
    unsafe extern "system" fn get_module_from_cubin(
        result: *mut cuda_types::cuda::CUmodule,
        fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
    ) -> cuda_types::cuda::CUresult {
        let result = match result.as_mut() {
            Some(p) => p,
            None => return CUresult::ERROR_INVALID_VALUE,
        };
        let code_lib = CodeLibraryRef::try_load(fatbinc_wrapper.cast())
            .map_err(|_| CUerror::NO_BINARY_FOR_GPU)?;
        let mut modules = Vec::with_capacity(1);
        code_lib.iterate_modules(|_, module| match module {
            Ok(CodeModuleRef::Text(ptx)) => {
                modules.push(std::borrow::Cow::Borrowed(ptx.as_bytes()));
            }
            Ok(CodeModuleRef::File(file)) => {
                if file.kind() == "ptx" {
                    if let Ok(text) = file.get_or_decompress_content(false) {
                        modules.push(text);
                    }
                }
            }
            _ => {}
        });
        let mut last_module =
            unwrap_or::unwrap_some_or!(modules.pop(), return Err(CUerror::NO_BINARY_FOR_GPU));
        if last_module.last() != Some(&0) {
            last_module.to_mut().push(0);
        }
        *result = CUmodule(
            GlobalState::remote_call_framed_in::<cuModuleLoadDataOut>(
                Opcode::cuModuleLoadData,
                cuModuleLoadDataIn {
                    image: last_module.into_owned(),
                },
            )?
            .module
            .to_native() as _,
        );
        Ok(())
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
        if !initialized() {
            return Err(CUerror::DEINITIALIZED);
        }
        let cu_ctx = CONTEXT_STACK.with(|s| s.unwrap(cu_ctx))?;
        let mut state = GlobalState::get()?.lock().map_err(|_| CUerror::UNKNOWN)?;
        let ctx_storage = state
            .context_state
            .get_mut(&cu_ctx)
            .ok_or(CUerror::INVALID_VALUE)?;
        ctx_storage.insert(key as _, (value as _, _dtor_cb));
        Ok(())
    }

    unsafe extern "system" fn context_local_storage_delete(
        cu_ctx: cuda_types::cuda::CUcontext,
        key: *mut std::ffi::c_void,
    ) -> cuda_types::cuda::CUresult {
        if !initialized() {
            return Err(CUerror::DEINITIALIZED);
        }
        let cu_ctx = CONTEXT_STACK.with(|s| s.unwrap(cu_ctx))?;
        let mut state = GlobalState::get()?.lock().map_err(|_| CUerror::UNKNOWN)?;
        let ctx_storage = state
            .context_state
            .get_mut(&cu_ctx)
            .ok_or(CUerror::INVALID_VALUE)?;
        let key = key as usize;
        ctx_storage.remove(&key);
        Ok(())
    }

    unsafe extern "system" fn context_local_storage_get(
        value: *mut *mut std::ffi::c_void,
        cu_ctx: cuda_types::cuda::CUcontext,
        key: *mut std::ffi::c_void,
    ) -> cuda_types::cuda::CUresult {
        if !initialized() {
            return Err(CUerror::DEINITIALIZED);
        }
        let value = unsafe { value.as_mut() }.ok_or(cuda_types::cuda::CUerror::INVALID_VALUE)?;
        let stored_value = {
            let cu_ctx = CONTEXT_STACK.with(|s| s.unwrap(cu_ctx))?;
            let mut state = GlobalState::get()?.lock().map_err(|_| CUerror::UNKNOWN)?;
            let ctx_storage = state
                .context_state
                .get_mut(&cu_ctx)
                .ok_or(CUerror::INVALID_VALUE)?;
            let key = key as usize;
            ctx_storage.get(&key).ok_or(CUerror::INVALID_VALUE)?.0
        };
        *value = stored_value as _;
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
        let heap_alloc_record_ptr = heap_alloc_record_ptr as *mut *mut AllocRecord;
        let alloc_record = Box::into_raw(Box::new(AllocRecord { x: arg2, y: arg3 }));
        *heap_alloc_record_ptr = alloc_record;
        Ok(())
    }

    unsafe extern "system" fn heap_free(
        heap_alloc_record_ptr: *const std::ffi::c_void,
        arg2: *mut usize,
    ) -> cuda_types::cuda::CUresult {
        if !initialized() {
            return Err(CUerror::DEINITIALIZED);
        }
        let heap_alloc_record_ptr = heap_alloc_record_ptr as *mut AllocRecord;
        let record = Box::from_raw(heap_alloc_record_ptr);
        if !arg2.is_null() {
            *arg2 = record.y;
        }
        Ok(())
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

const DLL_PROCESS_DETACH: u32 = 0;

#[allow(non_snake_case, unused_variables)]
#[no_mangle]
extern "system" fn DllMain(_dll_module: *const c_void, call_reason: u32, _: *const c_void) -> bool {
    if call_reason == DLL_PROCESS_DETACH {
        deinitialize();
    }
    true
}

static INITIALIZED: AtomicBool = AtomicBool::new(true);

fn initialized() -> bool {
    INITIALIZED.load(std::sync::atomic::Ordering::SeqCst)
}

fn deinitialize() {
    INITIALIZED.store(false, std::sync::atomic::Ordering::SeqCst);
}
