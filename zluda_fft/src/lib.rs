mod r#impl;
mod plan;

#[cfg(test)]
mod tests;

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
                cuda_macros::cufft_normalize_fn!( crate::r#impl::$fn_name ) ($(zluda_common::FromCuda::<_, cuda_types::cufft::cufftError_t>::from_cuda(&$arg_id)?),*)?;
                Ok(())
            }
        )*
    };
}

cuda_macros::cufft_function_declarations!(
    unimplemented,
    implemented
        <= [
            cufftCreate,
            cufftEstimate1d,
            cufftEstimate2d,
            cufftEstimate3d,
            cufftEstimateMany,
            cufftGetSize,
            cufftGetSize1d,
            cufftGetSize2d,
            cufftGetSize3d,
            cufftGetSizeMany,
            cufftGetSizeMany64,
            cufftMakePlan1d,
            cufftMakePlan2d,
            cufftMakePlan3d,
            cufftMakePlanMany,
            cufftMakePlanMany64,
            cufftPlan1d,
            cufftPlan2d,
            cufftPlan3d,
            cufftPlanMany,
            cufftSetWorkArea,
            cufftSetAutoAllocation,
            cufftExecC2C,
            cufftExecR2C,
            cufftExecC2R,
            cufftExecZ2Z,
            cufftExecD2Z,
            cufftExecZ2D,
            cufftSetStream,
            cufftDestroy,
            cufftGetVersion,
            cufftGetProperty,
        ]
);

macro_rules! noop {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {};
}

#[cfg(windows)]
mod os_macro {
    macro_rules! vtable_impl {
        ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
            use hipfft_sys::*;
            struct HipfftVtable {
                _lib: libloading::os::windows::Library,
                $($fn_name: unsafe extern "C" fn($($arg_id: $arg_type),*) -> $ret_type,)*
            }

            impl HipfftVtable {
                pub unsafe fn new() -> Result<Self, hipfftError> {
                    let hmodule = zluda_windows::try_load_from_self_or_hip_with_message(&["hipfft.dll"]).ok_or(hipfftError::INTERNAL_ERROR)?;
                    let lib = libloading::os::windows::Library::from_raw(hmodule.0 as _);
                    $(
                        let $fn_name = *lib.get::<unsafe extern "C" fn($($arg_id: $arg_type),*) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()).map_err(|_| hipfftError::INTERNAL_ERROR)?;
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
mod os_macro {
    macro_rules! vtable_impl {
        ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
            use hipfft_sys::*;

            struct HipfftVtable {}

            impl HipfftVtable {
                pub unsafe fn new() -> Result<Self, hipfftError> {
                    Ok(Self {})
                }
            }

            impl HipfftVtable {
                $(
                    pub unsafe fn $fn_name(&self, $($arg_id: $arg_type),*) -> $ret_type {
                        (hipfft_sys::$fn_name)($($arg_id),*)
                    }
                )*
            }
        };
    }
    pub(crate) use vtable_impl;
}

cuda_macros::hipfft_function_declarations!(
    noop,
    os_macro::vtable_impl
        <= [
            hipfftPlan1d,
            hipfftPlan2d,
            hipfftPlan3d,
            hipfftPlanMany,
            hipfftCreate,
            hipfftMakePlan1d,
            hipfftMakePlan2d,
            hipfftMakePlan3d,
            hipfftMakePlanMany,
            hipfftMakePlanMany64,
            hipfftEstimate1d,
            hipfftEstimate2d,
            hipfftEstimate3d,
            hipfftEstimateMany,
            hipfftGetSize1d,
            hipfftGetSize2d,
            hipfftGetSize3d,
            hipfftGetSizeMany,
            hipfftGetSizeMany64,
            hipfftGetSize,
            hipfftSetAutoAllocation,
            hipfftSetWorkArea,
            hipfftExecC2C,
            hipfftExecR2C,
            hipfftExecC2R,
            hipfftExecZ2Z,
            hipfftExecD2Z,
            hipfftExecZ2D,
            hipfftSetStream,
            hipfftDestroy,
        ]
);
