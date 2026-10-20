//! The CUDA driver API on Apple GPUs, for Asahi Linux.
//!
//! This is ZLUDA's AGX backend: the same exported functions as the HIP-based `zluda` crate (`libcuda`), implemented
//! with `agx_runtime` (direct DRM submission) and the LLVM AGX backend (through ZLUDA's `zoc` compiler). It needs no
//! ROCm. Functions that are not implemented return `CUDA_ERROR_NOT_SUPPORTED`.
//!
//! Supported so far: one device and context, device memory, synchronous copies and fills, modules from PTX (or an
//! AGX object), kernel launches with `kernelParams`, streams, events, device `printf` and `assert`.

mod driver;

use cuda_types::cuda::*;

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes, improper_ctypes_definitions, unused_variables, non_snake_case)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
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
                driver::$fn_name($($arg_id),*)
            }
        )*
    };
}

cuda_macros::cuda_function_declarations!(
    unimplemented,
    implemented
        <= [
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
            cuGetProcAddress,
            cuGetProcAddress_v2,
            cuInit,
            cuKernelGetAttribute,
            cuKernelGetFunction,
            cuLaunchKernel,
            cuLaunchKernelEx,
            cuLibraryGetKernel,
            cuLibraryGetModule,
            cuLibraryLoadData,
            cuLibraryUnload,
            cuMemAllocHost_v2,
            cuMemAlloc_v2,
            cuMemFreeHost,
            cuMemFree_v2,
            cuMemGetInfo_v2,
            cuMemHostAlloc,
            cuMemcpy,
            cuMemcpyAsync,
            cuMemcpyDtoDAsync_v2,
            cuMemcpyDtoD_v2,
            cuMemcpyDtoHAsync_v2,
            cuMemcpyDtoH_v2,
            cuMemcpyHtoDAsync_v2,
            cuMemcpyHtoD_v2,
            cuMemsetD16Async,
            cuMemsetD16_v2,
            cuMemsetD32Async,
            cuMemsetD32_v2,
            cuMemsetD8Async,
            cuMemsetD8_v2,
            cuModuleGetFunction,
            cuModuleGetGlobal_v2,
            cuModuleGetLoadingMode,
            cuModuleLoadData,
            cuModuleUnload,
            cuOccupancyMaxActiveBlocksPerMultiprocessor,
            cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,
            cuPointerGetAttribute,
            cuStreamCreate,
            cuStreamCreateWithPriority,
            cuStreamDestroy,
            cuStreamDestroy_v2,
            cuStreamGetFlags,
            cuStreamQuery,
            cuStreamSynchronize,
            cuStreamWaitEvent,
        ]
);
