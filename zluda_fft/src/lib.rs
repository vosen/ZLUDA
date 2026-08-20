mod hipfft;
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
            #[allow(non_snake_case)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                unsafe { crate::r#impl::$fn_name($( $arg_id ),*) }
            }
        )*
    };
}

cuda_macros::cufft_function_declarations!(
    unimplemented,
    implemented
        <= [
            cufftPlan1d,
            cufftPlan2d,
            cufftPlan3d,
            cufftPlanMany,
            cufftMakePlan1d,
            cufftMakePlan2d,
            cufftMakePlan3d,
            cufftMakePlanMany,
            cufftMakePlanMany64,
            cufftGetSizeMany64,
            cufftEstimate1d,
            cufftEstimate2d,
            cufftEstimate3d,
            cufftEstimateMany,
            cufftCreate,
            cufftGetSize1d,
            cufftGetSize2d,
            cufftGetSize3d,
            cufftGetSizeMany,
            cufftGetSize,
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
