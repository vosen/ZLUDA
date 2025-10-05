macro_rules! export_dnn8_unmangled {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                zluda_dnn::dnn8::$fn_name( $( $arg_id ),* )
            }
        )*
    };
}

cuda_macros::cudnn8_function_declarations! {
    export_dnn8_unmangled
}