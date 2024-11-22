pub(crate) mod r#impl;

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path;)*) => {
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
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path;)*) => {
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

cuda_base::cuda_function_declarations!(
    unimplemented,
    implemented <= [
        cuCtxGetLimit,
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
        cuDeviceTotalMem_v2,
        cuInit,
        cuModuleGetFunction,
        cuModuleLoadData,
        cuModuleUnload,
    ]
);