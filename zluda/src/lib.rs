pub(crate) mod r#impl;

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
                cuda_base::cuda_normalize_fn!( crate::r#impl::$fn_name ) ($(crate::r#impl::FromCuda::from_cuda(&$arg_id)?),*)?;
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
                cuda_base::cuda_normalize_fn!( crate::r#impl::function::$fn_name ) ($(crate::r#impl::FromCuda::from_cuda(&$arg_id)?),*)?;
                Ok(())
            }
        )*
    };
}

cuda_base::cuda_function_declarations!(
    unimplemented,
    implemented <= [
        cuCtxGetLimit,
        cuCtxSetCurrent,
        cuCtxGetCurrent,
        cuCtxSetLimit,
        cuCtxSynchronize,
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
        cuMemAlloc_v2,
        cuMemFree_v2,
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
        cuStreamSynchronize
    ],
    implemented_in_function <= [
        cuLaunchKernel,
    ]
);
