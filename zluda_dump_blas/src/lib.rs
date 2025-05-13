use libloading::Library;
use std::sync::LazyLock;

static LIBRARY: LazyLock<Result<Library, libloading::Error>> = LazyLock::new(get_library);

fn get_library() -> Result<Library, libloading::Error> {
    unsafe { Library::new("/usr/local/cuda/lib64/libcublas.so") }
}

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                use ::zluda_dump_common::ReprUsize;
                let fn_ptr = (&*LIBRARY).as_ref().unwrap().get::<unsafe extern $abi fn ( $($arg_type),* ) -> $ret_type>(concat!( stringify!($fn_name), "\0").as_bytes()).unwrap();
                let export_table = ::zluda_dump_common::get_export_table().unwrap();
                ReprUsize::from_usize(export_table.logged_call(
                    stringify!($fn_name),
                    "()".to_string(),
                    &|| {
                        let result = fn_ptr(  $( $arg_id),* );
                        ReprUsize::to_usize(result)
                    },
                    <$ret_type as ReprUsize>::INTERNAL_ERROR,
                    <$ret_type as ReprUsize>::format_status)
                )
                //eprintln!(stringify!($fn_name));
                //return fn_ptr(  $( $arg_id),* );
            }
        )*
    };
}

cuda_base::cublas_function_declarations!(unimplemented);
