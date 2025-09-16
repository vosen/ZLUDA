#[cfg_attr(windows, path = "impl_win.rs")]
#[cfg_attr(unix, path = "impl_unix.rs")]
mod r#impl;
mod impl_common;

macro_rules! unimplemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            #[allow(improper_ctypes_definitions)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                impl_common::unimplemented()
            }
        )*
    };
}

macro_rules! implemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::nvml_normalize_fn!( crate::r#impl::$fn_name ) ( $( zluda_common::FromCuda::<_, cuda_types::nvml::nvmlError_t>::from_cuda(&$arg_id )?),*)?;
                Ok(())
            }
        )*
    };
}

macro_rules! implemented_unnormalized {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            #[allow(improper_ctypes_definitions)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::nvml_normalize_fn!( crate::r#impl::$fn_name ) ( $( $arg_id),*)
            }
        )*
    };
}

cuda_macros::nvml_function_declarations!(
    unimplemented_fn,
    implemented_fn
        <= [
            nvmlDeviceGetCount_v2,
            nvmlDeviceGetFieldValues,
            nvmlDeviceGetGpuFabricInfo,
            nvmlDeviceGetHandleByIndex_v2,
            nvmlDeviceGetHandleByPciBusId_v2,
            nvmlInit,
            nvmlInitWithFlags,
            nvmlInit_v2,
            nvmlShutdown,
            nvmlSystemGetDriverVersion,
        ],
    implemented_unnormalized <= [nvmlErrorString,]
);
