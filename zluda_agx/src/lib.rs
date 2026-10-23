//! The CUDA driver API on Apple GPUs, for Asahi Linux.
//!
//! This is ZLUDA's AGX backend: the same exported functions as the HIP-based `zluda` crate (`libcuda`), implemented
//! with `agx_runtime` (direct DRM submission) and the LLVM AGX backend (through ZLUDA's `zoc` compiler). It needs no
//! ROCm. Functions that are not implemented return `CUDA_ERROR_NOT_SUPPORTED`.
//!
//! Supported so far: one device and context, device memory, synchronous copies and fills, modules from PTX (or an
//! AGX object), kernel launches with `kernelParams`, streams, events, device `printf` and `assert`.

mod contexts;
mod dark;
mod driver;
pub mod specialize;
mod texture;

use cuda_types::cuda::*;
use std::sync::OnceLock;

/// `ZLUDA_AGX_LOG`: report unimplemented functions and failing calls on stderr.
fn log_enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("ZLUDA_AGX_LOG").is_some())
}

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes, improper_ctypes_definitions, unused_variables, non_snake_case)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                if log_enabled() {
                    eprintln!("[zluda_agx] {} is not implemented", stringify!($fn_name));
                }
                Err(CUerror::NOT_SUPPORTED)
            }
        )*
    };
}

/// `ZLUDA_AGX_API_STATS=FILE`: every second (at the next API call), a JSON line with the Unix time in milliseconds
/// (`t`) and, per CUDA function called so far, its calls and the host time spent in it (`{"name": [calls, ns]}`),
/// cumulative: the difference of two lines is the host cost of the API between them.
mod api_stats {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    use std::time::{Instant, SystemTime, UNIX_EPOCH};

    struct Stats {
        file: std::fs::File,
        calls: HashMap<&'static str, (u64, u64)>,
        written: Instant,
    }

    fn stats() -> Option<&'static Mutex<Stats>> {
        static S: OnceLock<Option<Mutex<Stats>>> = OnceLock::new();
        S.get_or_init(|| {
            let path = std::env::var_os("ZLUDA_AGX_API_STATS")?;
            let file = std::fs::OpenOptions::new().create(true).append(true).open(path).ok()?;
            Some(Mutex::new(Stats { file, calls: HashMap::new(), written: Instant::now() }))
        })
        .as_ref()
    }

    pub(crate) fn enabled() -> bool {
        stats().is_some()
    }

    pub(crate) fn record(name: &'static str, start: Instant) {
        let Some(s) = stats() else { return };
        let ns = start.elapsed().as_nanos() as u64;
        let mut s = s.lock().unwrap_or_else(|e| e.into_inner());
        let e = s.calls.entry(name).or_insert((0, 0));
        e.0 += 1;
        e.1 += ns;
        if s.written.elapsed().as_secs() >= 1 {
            use std::io::Write;
            let t = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis());
            let mut line = format!("{{\"t\":{t}");
            for (k, (c, n)) in &s.calls {
                line.push_str(&format!(",\"{k}\":[{c},{n}]"));
            }
            line.push_str("}\n");
            let _ = s.file.write_all(line.as_bytes());
            s.written = Instant::now();
        }
    }
}

macro_rules! implemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes, improper_ctypes_definitions, non_snake_case)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let start = api_stats::enabled().then(std::time::Instant::now);
                let r = driver::$fn_name($($arg_id),*);
                if let Some(start) = start {
                    api_stats::record(stringify!($fn_name), start);
                }
                if let Err(e) = r {
                    if log_enabled() {
                        eprintln!("[zluda_agx] {} failed with CUDA error {}", stringify!($fn_name), e.0);
                    }
                }
                r
            }
        )*
    };
}

cuda_macros::cuda_function_declarations!(
    unimplemented,
    implemented
        <= [
            cuArray3DCreate_v2,
            cuArray3DGetDescriptor_v2,
            cuArrayCreate_v2,
            cuArrayDestroy,
            cuArrayGetDescriptor_v2,
            cuCtxCreate_v2,
            cuCtxDestroy_v2,
            cuCtxGetApiVersion,
            cuCtxGetCurrent,
            cuCtxGetDevice,
            cuCtxGetFlags,
            cuCtxGetLimit,
            cuCtxGetStreamPriorityRange,
            cuCtxPopCurrent_v2,
            cuCtxPushCurrent_v2,
            cuCtxSetCurrent,
            cuCtxSetLimit,
            cuCtxSynchronize,
            cuCtxSynchronize_v2,
            cuDeviceComputeCapability,
            cuDeviceGet,
            cuDeviceGetAttribute,
            cuDeviceGetCount,
            cuDeviceGetName,
            cuDeviceGetPCIBusId,
            cuDeviceGetUuid,
            cuDeviceGetUuid_v2,
            cuDevicePrimaryCtxGetState,
            cuDevicePrimaryCtxRelease_v2,
            cuDevicePrimaryCtxReset_v2,
            cuDevicePrimaryCtxRetain,
            cuDevicePrimaryCtxSetFlags_v2,
            cuDeviceTotalMem_v2,
            cuDriverGetVersion,
            cuEventCreate,
            cuEventDestroy_v2,
            cuEventElapsedTime,
            cuEventQuery,
            cuEventRecord,
            cuEventRecordWithFlags,
            cuEventSynchronize,
            cuFuncGetAttribute,
            cuFuncSetAttribute,
            cuFuncSetCacheConfig,
            cuGetErrorName,
            cuGetErrorString,
            cuGetExportTable,
            cuGetProcAddress,
            cuGetProcAddress_v2,
            cuGraphCreate,
            cuGraphDestroy,
            cuGraphExecDestroy,
            cuGraphInstantiateWithFlags,
            cuGraphInstantiate_v2,
            cuGraphLaunch,
            cuInit,
            cuKernelGetAttribute,
            cuKernelGetFunction,
            cuLaunchCooperativeKernel,
            cuLaunchKernel,
            cuLaunchKernelEx,
            cuLibraryGetKernel,
            cuLibraryGetModule,
            cuLibraryLoadData,
            cuLibraryUnload,
            cuLinkAddData_v2,
            cuLinkAddFile_v2,
            cuLinkComplete,
            cuLinkCreate_v2,
            cuLinkDestroy,
            cuMemAdvise,
            cuMemAllocAsync,
            cuMemAllocHost_v2,
            cuMemAllocManaged,
            cuMemAllocPitch_v2,
            cuMemAlloc_v2,
            cuMemFreeAsync,
            cuMemFreeHost,
            cuMemFree_v2,
            cuMemGetAddressRange_v2,
            cuMemGetInfo_v2,
            cuMemHostAlloc,
            cuMemHostGetDevicePointer_v2,
            cuMemHostGetFlags,
            cuMemPrefetchAsync,
            cuMemcpy,
            cuMemcpy2DAsync_v2,
            cuMemcpy2DUnaligned_v2,
            cuMemcpy2D_v2,
            cuMemcpy3DAsync_v2,
            cuMemcpy3D_v2,
            cuMemcpyAsync,
            cuMemcpyAtoH_v2,
            cuMemcpyDtoDAsync_v2,
            cuMemcpyDtoD_v2,
            cuMemcpyDtoHAsync_v2,
            cuMemcpyDtoH_v2,
            cuMemcpyHtoA_v2,
            cuMemcpyHtoDAsync_v2,
            cuMemcpyHtoD_v2,
            cuMemAddressFree,
            cuMemAddressReserve,
            cuMemCreate,
            cuMemGetAccess,
            cuMemGetAllocationGranularity,
            cuMemGetAllocationPropertiesFromHandle,
            cuMemMap,
            cuMemRelease,
            cuMemRetainAllocationHandle,
            cuMemSetAccess,
            cuMemUnmap,
            cuMulticastAddDevice,
            cuMulticastBindAddr,
            cuMulticastBindMem,
            cuMulticastCreate,
            cuMulticastGetGranularity,
            cuMulticastUnbind,
            cuIpcCloseMemHandle,
            cuIpcGetMemHandle,
            cuIpcOpenMemHandle_v2,
            cuMemHostRegister_v2,
            cuMemHostUnregister,
            cuMemExportToShareableHandle,
            cuMemImportFromShareableHandle,
            cuMemcpyPeer,
            cuMemcpyPeerAsync,
            cuMemsetD16Async,
            cuMemsetD16_v2,
            cuMemsetD2D16Async,
            cuMemsetD2D16_v2,
            cuMemsetD2D32Async,
            cuMemsetD2D32_v2,
            cuMemsetD2D8Async,
            cuMemsetD2D8_v2,
            cuMemsetD32Async,
            cuMemsetD32_v2,
            cuMemsetD8Async,
            cuMemsetD8_v2,
            cuMipmappedArrayCreate,
            cuMipmappedArrayDestroy,
            cuMipmappedArrayGetLevel,
            cuModuleGetFunction,
            cuModuleGetGlobal_v2,
            cuModuleGetTexRef,
            cuModuleGetLoadingMode,
            cuModuleLoad,
            cuModuleLoadData,
            cuModuleLoadDataEx,
            cuModuleLoadFatBinary,
            cuModuleUnload,
            cuOccupancyMaxActiveBlocksPerMultiprocessor,
            cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,
            cuPointerGetAttribute,
            cuStreamBeginCapture_v2,
            cuStreamCreate,
            cuStreamCreateWithPriority,
            cuStreamDestroy,
            cuStreamDestroy_v2,
            cuStreamEndCapture,
            cuStreamGetFlags,
            cuStreamIsCapturing,
            cuStreamQuery,
            cuStreamSynchronize,
            cuStreamWaitEvent,
            cuSurfObjectCreate,
            cuSurfObjectDestroy,
            cuTensorMapEncodeIm2col,
            cuTensorMapEncodeTiled,
            cuTensorMapReplaceAddress,
            cuTexObjectCreate,
            cuTexObjectDestroy,
            cuTexRefGetAddress_v2,
            cuTexRefSetAddress2D_v3,
            cuTexRefSetAddressMode,
            cuTexRefSetAddress_v2,
            cuTexRefSetArray,
            cuTexRefSetFilterMode,
            cuTexRefSetFlags,
            cuTexRefSetFormat,
            cuTexRefSetMipmapFilterMode,
            cuTexRefSetMipmappedArray,
            cuThreadExchangeStreamCaptureMode,
        ]
);
