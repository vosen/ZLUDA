mod r#impl;

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            #[allow(unused_variables)]
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

macro_rules! noop {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {};
}

#[cfg(windows)]
mod os {
    macro_rules! vtable_impl {
        ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
            use hipblaslt_sys::*;
            struct HipblasltVtable {
                _lib: libloading::os::windows::Library,
                $($fn_name: unsafe extern "C" fn($($arg_id: $arg_type),*) -> $ret_type,)*
            }

            impl HipblasltVtable {
                pub unsafe fn new() -> Result<Self, hipblasLtError> {
                    let hmodule = zluda_windows::try_load_from_self_or_hip_with_message(&["hipblaslt.dll", "libhipblaslt.dll"]).ok_or(hipblasLtError::INTERNAL_ERROR)?;
                    let lib = libloading::os::windows::Library::from_raw(hmodule.0 as _);
                    $(
                        let $fn_name = *lib.get::<unsafe extern "C" fn($($arg_id: $arg_type),*) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()).map_err(|_| hipblasLtError::INTERNAL_ERROR)?;
                    )*
                    Ok(Self {
                        _lib: lib,
                        $($fn_name,)*
                    })
                }

                $(
                    pub unsafe fn $fn_name(&self, $($arg_id: $arg_type),*) -> $ret_type {
                        (self.$fn_name)($($arg_id),*)
                    }
                )*
            }
        };
    }
    pub(crate) use vtable_impl;
}

#[cfg(not(windows))]
mod os {
    macro_rules! vtable_impl {
        ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
            use hipblaslt_sys::*;

            struct HipblasltVtable {}

            impl HipblasltVtable {
                pub unsafe fn new() -> Result<Self, hipblasLtError> {
                    Ok(Self {})
                }
            }

            impl HipblasltVtable {
                $(
                    pub unsafe fn $fn_name(&self, $($arg_id: $arg_type),*) -> $ret_type {
                        (hipblaslt_sys::$fn_name)($($arg_id),*)
                    }
                )*
            }
        };
    }
    pub(crate) use vtable_impl;
}

cuda_macros::hipblaslt_function_declarations!(
    noop,
    os::vtable_impl
        <= [
            hipblasLtCreate,
            hipblasLtDestroy,
            hipblasLtMatmul,
            hipblasLtMatmulAlgoGetHeuristic,
            hipblasLtMatmulDescCreate,
            hipblasLtMatmulDescDestroy,
            hipblasLtMatmulDescSetAttribute,
            hipblasLtMatmulPreferenceCreate,
            hipblasLtMatmulPreferenceDestroy,
            hipblasLtMatmulPreferenceSetAttribute,
            hipblasLtMatrixLayoutCreate,
            hipblasLtMatrixLayoutDestroy,
            hipblasLtMatrixLayoutSetAttribute,
        ]
);
