pub(crate) struct Zluda;

pub(crate) struct Cuda(libloading::Library);

impl Cuda {
    #[cfg(not(windows))]
    const CUDA_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcuda.so.1";
    #[cfg(windows)]
    const CUDA_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";

    fn load() -> Self {
        unsafe { Self(libloading::Library::new(Self::CUDA_PATH).unwrap()) }
    }
}

macro_rules! implemented_test {
    ($($abi:literal fn $fn_name:ident( $( $arg_id:ident : $arg_type:ty ),* ) -> $ret_type:ty;)* ) => {
        pub(crate) trait CudaApi {
            fn new() -> Self;
            $(
                #[allow(non_snake_case, dead_code)]
                fn $fn_name(&self, $( $arg_id : $arg_type ),* ) {
                    paste::paste!{ self.[< $fn_name _unchecked >]( $( $arg_id ),* ) }.unwrap()
                }
                paste::paste!{ #[allow(non_snake_case, dead_code)] fn [< $fn_name _unchecked>](&self, $( $arg_id : $arg_type ),* ) -> $ret_type; }
            )*
        }

        

        impl CudaApi for Cuda {
            fn new() -> Self { Self::load() }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    let func = unsafe { self.0.get::<unsafe extern $abi fn ( $( $arg_type ),* ) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()) }.unwrap();
                    unsafe { (func)( $( $arg_id ),* ) }
                }}
            )*
        }

        impl CudaApi for Zluda {
            fn new() -> Self { Self }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    unsafe { super::$fn_name( $( $arg_id ),* ) }
                }}
            )*
        }
    };
}
cuda_macros::cuda_function_declarations!(implemented_test);
