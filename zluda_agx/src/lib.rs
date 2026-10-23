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

macro_rules! implemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes, improper_ctypes_definitions, non_snake_case)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let r = driver::$fn_name($($arg_id),*);
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
