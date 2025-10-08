mod r#impl;

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                crate::r#impl::unimplemented()?;
                Ok(())
            }
        )*
    };
}

macro_rules! implemented_no_conversion {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::cudnn_normalize_fn!( crate::r#impl::$fn_name ) ( $($arg_id),*)
            }
        )*
    };
}

macro_rules! implemented_dnn8 {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::cudnn_normalize_fn!( crate::r#impl::dnn8::$fn_name ) ( $($arg_id),*)
            }
        )*
    };
}

macro_rules! implemented_dnn9 {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::cudnn_normalize_fn!( crate::r#impl::dnn9::$fn_name ) ( $($arg_id),*)
            }
        )*
    };
}

macro_rules! implemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::cudnn_normalize_fn!( crate::r#impl::$fn_name )  ($(zluda_common::FromCuda::<_, cuda_types::cudnn9::cudnnError_t>::from_cuda(&$arg_id)?),*)?;
                Ok(())
            }
        )*
    };
}

macro_rules! dual_function_declarations {
    ($default:ident, ($dnn8:ident | $dnn9:ident) <= [$($fn_versioned:ident),*], $($class:ident <= [$($fn_:ident),*]),*) => {
        pub mod dnn8 {
            cuda_macros::cudnn8_function_declarations!(
                $default,
                $dnn8 <= [$($fn_versioned),*],
                $(
                    $class <= [$($fn_),*]
                ),*
            );
        }

        pub mod dnn9 {
            cuda_macros::cudnn9_function_declarations!(
                $default,
                $dnn9 <= [$($fn_versioned),*],
                $(
                    $class <= [$($fn_),*]
                ),*
            );
        }
    };
}

dual_function_declarations! {
    unimplemented,
    (implemented_dnn8|implemented_dnn9) <= [
        cudnnGetErrorString,
        cudnnGetConvolutionForwardAlgorithm_v7
    ],
    implemented_no_conversion <= [
        cudnnGetVersion,
        cudnnGetMaxDeviceVersion,
        cudnnGetCudartVersion,
        cudnnGetLastErrorString
    ],
    implemented <= [
        cudnnCreate,
        cudnnCreateConvolutionDescriptor,
        cudnnCreateFilterDescriptor,
        cudnnCreateTensorDescriptor,
        cudnnDestroy,
        cudnnSetConvolution2dDescriptor,
        cudnnSetConvolutionMathType,
        cudnnSetFilter4dDescriptor,
        cudnnSetTensor4dDescriptor
    ]
}
