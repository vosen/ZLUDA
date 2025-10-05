macro_rules! export_dnn9_unmangled {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                zluda_dnn::dnn9::$fn_name( $( $arg_id ),* )
            }
        )*
    };
}

cuda_macros::cudnn9_function_declarations! {
    export_dnn9_unmangled
}