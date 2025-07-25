mod r#impl;

macro_rules! unimplemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            #[allow(improper_ctypes_definitions)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                r#impl::unimplemented()
            }
        )*
    };
}

macro_rules! implemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            #[allow(improper_ctypes_definitions)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                r#impl::$fn_name($($arg_id),*)
            }
        )*
    };
}

cuda_macros::nvml_function_declarations!(
    unimplemented_fn,
    implemented_fn <= [
        nvmlErrorString,
        nvmlInit_v2,
        nvmlSystemGetDriverVersion
    ]
);
