mod r#impl;

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
                cuda_macros::cublaslt_normalize_fn!( crate::r#impl::$fn_name )  ($(zluda_common::FromCuda::<_, cuda_types::cublas::cublasError_t>::from_cuda(&$arg_id)?),*)?;
                Ok(())
            }
        )*
    };
}

macro_rules! implemented_unmapped {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::cublaslt_normalize_fn!( crate::r#impl::$fn_name ) ( $( $arg_id ),* )
            }
        )*
    };
}

cuda_macros::cublaslt_function_declarations!(
    unimplemented,
    implemented
        <= [
            cublasLtCreate,
            cublasLtDestroy,
            cublasLtMatmul,
            cublasLtMatmulAlgoGetHeuristic,
            cublasLtMatmulDescCreate,
            cublasLtMatmulDescDestroy,
            cublasLtMatmulDescSetAttribute,
            cublasLtMatmulPreferenceCreate,
            cublasLtMatmulPreferenceDestroy,
            cublasLtMatmulPreferenceSetAttribute,
            cublasLtMatrixLayoutCreate,
            cublasLtMatrixLayoutDestroy,
            cublasLtMatrixLayoutSetAttribute,
        ],
    implemented_unmapped
        <= [
            cublasLtDisableCpuInstructionsSetMask,
            cublasLtGetCudartVersion,
            cublasLtGetStatusName,
            cublasLtGetStatusString,
            cublasLtGetVersion,
        ]
);

#[cfg(windows)]
mod windows {
    use zluda_windows;
    #[no_mangle]
    static __pfnDliFailureHook2: zluda_windows::PfnDliHook = delaylink_hook;

    unsafe extern "system" fn delaylink_hook(
        dli_notify: u32,
        pdli: *const zluda_windows::DelayLoadInfo,
    ) -> *mut std::ffi::c_void {
        zluda_windows::delay_load_failure_hook("hipblaslt.dll", dli_notify, pdli)
            .map(|hm| hm.0 as *mut std::ffi::c_void)
            .unwrap_or(std::ptr::null_mut())
    }
}
