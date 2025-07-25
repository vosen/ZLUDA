use cuda_types::cuda::CUerror;
use zluda_common::{define_init_state, InitState};
pub(crate) mod r#impl;

define_init_state!(INITIALIZED);

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                crate::r#impl::unimplemented()
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
                if !INITIALIZED.is_initialized() {
                    return Err(CUerror::DEINITIALIZED);
                }
                cuda_macros::cuda_normalize_fn!( crate::r#impl::$fn_name ) ($(crate::r#impl::FromCuda::from_cuda(&$arg_id)?),*)?;
                Ok(())
            }
        )*
    };
}

macro_rules! implemented_in_function {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                if !INITIALIZED.is_initialized() {
                    return Err(CUerror::DEINITIALIZED);
                }
                cuda_macros::cuda_normalize_fn!( crate::r#impl::function::$fn_name ) ($(crate::r#impl::FromCuda::from_cuda(&$arg_id)?),*)?;
                Ok(())
            }
        )*
    };
}

cuda_macros::cuda_function_declarations!(
    unimplemented,
    implemented <= [
        cuCtxCreate_v2,
        cuCtxDestroy_v2,
        cuCtxGetLimit,
        cuCtxSetCurrent,
        cuCtxGetCurrent,
        cuCtxGetDevice,
        cuCtxSetLimit,
        cuCtxSynchronize,
        cuCtxPushCurrent,
        cuCtxPushCurrent_v2,
        cuCtxPopCurrent,
        cuCtxPopCurrent_v2,
        cuDeviceComputeCapability,
        cuDeviceGet,
        cuDeviceGetAttribute,
        cuDeviceGetCount,
        cuDeviceGetLuid,
        cuDeviceGetName,
        cuDeviceGetProperties,
        cuDeviceGetUuid,
        cuDeviceGetUuid_v2,
        cuDevicePrimaryCtxRelease,
        cuDevicePrimaryCtxRetain,
        cuDevicePrimaryCtxReset,
        cuDeviceTotalMem_v2,
        cuDriverGetVersion,
        cuFuncGetAttribute,
        cuGetExportTable,
        cuGetProcAddress,
        cuGetProcAddress_v2,
        cuInit,
        cuLibraryLoadData,
        cuLibraryGetModule,
        cuLibraryUnload,
        cuMemAlloc_v2,
        cuMemFree_v2,
        cuMemHostAlloc,
        cuMemFreeHost,
        cuMemGetAddressRange_v2,
        cuMemGetInfo_v2,
        cuMemcpyDtoH_v2,
        cuMemcpyHtoD_v2,
        cuMemsetD32_v2,
        cuMemsetD8_v2,
        cuModuleGetFunction,
        cuModuleGetLoadingMode,
        cuModuleLoadData,
        cuModuleUnload,
        cuPointerGetAttribute,
        cuStreamSynchronize,
        cuProfilerStart,
        cuProfilerStop,
    ],
    implemented_in_function <= [
        cuLaunchKernel,
    ]
);
