pub(crate) struct ZludaBlasLt;

pub(crate) struct CudaBlasLt(libloading::Library);

impl CudaBlasLt {
    #[cfg(not(windows))]
    const CUBLASLT_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcublasLt.so.11";
    #[cfg(windows)]
    const CUBLASLT_PATH: &'static str = "C:\\Windows\\System32\\cublasLt64_12.dll";

    fn load() -> Self {
        unsafe { Self(libloading::Library::new(Self::CUBLASLT_PATH).unwrap()) }
    }
}

macro_rules! implemented_test {
    ($($abi:literal fn $fn_name:ident( $( $arg_id:ident : $arg_type:ty ),* ) -> $ret_type:ty;)* ) => {
        pub(crate) trait CublasLtApi {
            fn new() -> Self;
            $(
                #[allow(non_snake_case, dead_code)]
                fn $fn_name(&self, $( $arg_id : $arg_type ),* ) {
                    paste::paste!{ self.[< $fn_name _unchecked >]( $( $arg_id ),* ) }.unwrap()
                }
                paste::paste!{ #[allow(non_snake_case, dead_code)] fn [< $fn_name _unchecked>](&self, $( $arg_id : $arg_type ),* ) -> $ret_type; }
            )*
        }



        impl CublasLtApi for CudaBlasLt {
            fn new() -> Self { Self::load() }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    let func = unsafe { self.0.get::<unsafe extern $abi fn ( $( $arg_type ),* ) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()) }.unwrap();
                    unsafe { (func)( $( $arg_id ),* ) }
                }}
            )*
        }

        impl CublasLtApi for ZludaBlasLt {
            fn new() -> Self { Self }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    unsafe { zluda_blaslt::$fn_name( $( $arg_id ),* ) }
                }}
            )*
        }
    };
}

macro_rules! ignore {
    ($($abi:literal fn $fn_name:ident( $( $arg_id:ident : $arg_type:ty ),* ) -> $ret_type:ty;)* ) => {};
}

cuda_macros::cublaslt_function_declarations!(
    implemented_test,
    ignore
        <= [
            cublasLtDisableCpuInstructionsSetMask,
            cublasLtGetCudartVersion,
            cublasLtGetStatusName,
            cublasLtGetStatusString,
            cublasLtGetVersion,
        ]
);

pub(crate) struct Zluda;

pub(crate) struct Cuda(libloading::Library);

impl Cuda {
    #[cfg(not(windows))]
    const CUDA_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcuda.so.1";
    #[cfg(windows)]
    const CUDA_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";

    #[cfg(not(windows))]
    const CUBLAS_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcublas.so.11";
    #[cfg(windows)]
    const CUBLAS_PATH: &'static str = "C:\\Windows\\System32\\cublas_12.dll";

    fn load() -> Self {
        // cublas will try to load cuda, so we load it first to ensure the correct version is used
        unsafe {
            libloading::Library::new(Self::CUDA_PATH)
                .expect("CUDA should have been loaded successfully")
        };

        unsafe { Self(libloading::Library::new(Self::CUBLAS_PATH).unwrap()) }
    }
}

macro_rules! implemented_test {
    ($($abi:literal fn $fn_name:ident( $( $arg_id:ident : $arg_type:ty ),* ) -> $ret_type:ty;)* ) => {
        pub(crate) trait CublasApi {
            fn new() -> Self;
            fn blaslt(&self) -> impl CublasLtApi;
            $(
                #[allow(non_snake_case, dead_code)]
                fn $fn_name(&self, $( $arg_id : $arg_type ),* ) {
                    paste::paste!{ self.[< $fn_name _unchecked >]( $( $arg_id ),* ) }.unwrap()
                }
                paste::paste!{ #[allow(non_snake_case, dead_code)] fn [< $fn_name _unchecked>](&self, $( $arg_id : $arg_type ),* ) -> $ret_type; }
            )*
        }



        impl CublasApi for Cuda {
            fn new() -> Self { Self::load() }
            fn blaslt(&self) -> impl CublasLtApi {
                CudaBlasLt::new()
            }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    let func = unsafe { self.0.get::<unsafe extern $abi fn ( $( $arg_type ),* ) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()) }.unwrap();
                    unsafe { (func)( $( $arg_id ),* ) }
                }}
            )*
        }

        impl CublasApi for Zluda {
            fn new() -> Self { Self }
            fn blaslt(&self) -> impl CublasLtApi {
                ZludaBlasLt::new()
            }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    unsafe { super::$fn_name( $( $arg_id ),* ) }
                }}
            )*
        }
    };
}

cuda_macros::cublas_function_declarations!(
    implemented_test,
    ignore
        <= [
            cublasGetStatusName,
            cublasGetStatusString,
            cublasXerbla,
            cublasGetCudartVersion
        ]
);
