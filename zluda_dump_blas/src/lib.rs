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
                eprintln!(stringify!($fn_name));
                let fn_ptr = (&*LIBRARY).as_ref().unwrap().get::<unsafe extern $abi fn ( $($arg_type),* ) -> $ret_type>(concat!( stringify!($fn_name), "\0").as_bytes()).unwrap();
                return fn_ptr(  $( $arg_id),* );
            }
        )*
    };
}

cuda_base::cublas_function_declarations!(unimplemented);
