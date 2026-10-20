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
            cuCtxGetCurrent,
            cuCtxGetDevice,
            cuCtxSetCurrent,
            cuCtxSynchronize,
            cuDeviceComputeCapability,
            cuDeviceGet,
            cuDeviceGetAttribute,
            cuDeviceGetCount,
            cuDeviceGetName,
            cuDevicePrimaryCtxRelease_v2,
            cuDevicePrimaryCtxRetain,
            cuDeviceTotalMem_v2,
            cuDriverGetVersion,
            cuEventCreate,
            cuEventDestroy_v2,
            cuEventElapsedTime,
            cuEventQuery,
            cuEventRecord,
            cuEventSynchronize,
            cuFuncGetAttribute,
            cuGetErrorName,
            cuGetErrorString,
            cuInit,
            cuLaunchKernel,
            cuMemAlloc_v2,
            cuMemFree_v2,
            cuMemGetInfo_v2,
            cuMemcpyDtoD_v2,
            cuMemcpyDtoH_v2,
            cuMemcpyHtoD_v2,
            cuMemsetD32_v2,
            cuMemsetD8_v2,
            cuModuleGetFunction,
            cuModuleGetGlobal_v2,
            cuModuleLoadData,
            cuModuleUnload,
            cuStreamCreate,
            cuStreamDestroy_v2,
            cuStreamSynchronize,
            cuStreamWaitEvent,
        ]
);
