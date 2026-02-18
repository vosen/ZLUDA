mod r#impl;

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[allow(unused_variables)]
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
        cudnnConvolutionForward,
        cudnnCreate,
        cudnnCreateConvolutionDescriptor,
        cudnnCreateFilterDescriptor,
        cudnnCreateTensorDescriptor,
        cudnnDestroy,
        cudnnDestroyConvolutionDescriptor,
        cudnnDestroyFilterDescriptor,
        cudnnDestroyTensorDescriptor,
        cudnnGetConvolutionForwardWorkspaceSize,
        cudnnSetConvolution2dDescriptor,
        cudnnSetConvolutionMathType,
        cudnnSetFilter4dDescriptor,
        cudnnSetTensor4dDescriptor
    ]
}

macro_rules! noop {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {};
}

#[cfg(windows)]
mod os {
    macro_rules! vtable_impl {
        ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
            use miopen_sys::*;
            struct MIOpenVtable {
                _lib: libloading::os::windows::Library,
                $($fn_name: unsafe extern "C" fn($($arg_id: $arg_type),*) -> $ret_type,)*
            }

            impl MIOpenVtable {
                pub unsafe fn new() -> Result<Self, miopenError_t> {
                    let hmodule = zluda_windows::try_load_from_self_or_hip_with_message(&["MIOpen.dll"]).ok_or(miopenError_t::InternalError)?;
                    let lib = libloading::os::windows::Library::from_raw(hmodule.0 as _);
                    $(
                        let $fn_name = *lib.get::<unsafe extern "C" fn($($arg_id: $arg_type),*) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()).map_err(|_| miopenError_t::InternalError)?;
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
            use miopen_sys::*;

            struct MIOpenVtable {}

            impl MIOpenVtable {
                pub unsafe fn new() -> Result<Self, miopenError_t> {
                    Ok(Self {})
                }
            }

            impl MIOpenVtable {
                $(
                    pub unsafe fn $fn_name(&self, $($arg_id: $arg_type),*) -> $ret_type {
                        (miopen_sys::$fn_name)($($arg_id),*)
                    }
                )*
            }
        };
    }
    pub(crate) use vtable_impl;
}

cuda_macros::miopen_function_declarations!(
    noop,
    os::vtable_impl
        <= [
            miopenConvolutionForward,
            miopenConvolutionForwardGetWorkSpaceSize,
            miopenCreate,
            miopenCreateConvolutionDescriptor,
            miopenCreateTensorDescriptor,
            miopenCreateTensorDescriptor,
            miopenDestroy,
            miopenDestroyConvolutionDescriptor,
            miopenDestroyTensorDescriptor,
            miopenFindConvolutionForwardAlgorithm,
            miopenGetConvolutionDescriptor,
            miopenGetTensorDescriptor,
            miopenGetTensorDescriptor,
            miopenGetTensorNumBytes,
            miopenGetTensorNumBytes,
            miopenInitConvolutionDescriptor,
            miopenOpTensor,
            miopenSetNdTensorDescriptorWithLayout,
        ]
);
