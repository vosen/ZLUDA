use std::path::PathBuf;
use zluda_common::os;

pub(crate) struct Zluda;

pub(crate) struct Cuda {
    cuda: libloading::Library,
    cudnn: libloading::Library,
}

impl Cuda {
    #[cfg(not(windows))]
    const CUDA_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcuda.so.1";
    #[cfg(windows)]
    const CUDA_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";

    #[cfg(not(windows))]
    fn cudnn_path() -> String {
        // TODO: check this path
        "/usr/local/cuda/lib64/libcudnn.so.9".to_string()
    }
    #[cfg(windows)]
    fn cudnn_path() -> String {
        use std::path::Path;
        use windows::{core::PCSTR, Win32::System::LibraryLoader::SetDllDirectoryA};

        fn last_subdir(path: impl AsRef<Path>) -> PathBuf {
            let cudnn_dir = std::fs::read_dir(path).unwrap();
            let mut directories = cudnn_dir
                .filter_map(|entry| {
                    let entry = entry.unwrap();
                    if entry.file_type().unwrap().is_dir() {
                        Some(entry.path())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            directories.sort_by_cached_key(|dir| dir.file_name().unwrap().to_owned());
            directories.pop().unwrap()
        }

        let path = PathBuf::from(r"C:\Program Files\NVIDIA\CUDNN");
        let mut cudnn_version = last_subdir(path);
        cudnn_version.push("bin");
        let mut cuda_version = last_subdir(cudnn_version);
        cuda_version.push("x64");
        let mut extra_search_dir = cuda_version.to_str().unwrap().to_owned();
        extra_search_dir.push('\0');
        // cuDNN is dumb like this
        unsafe { SetDllDirectoryA(PCSTR(extra_search_dir.as_ptr())) }.unwrap();
        cuda_version
            .join(r"cudnn64_9.dll")
            .to_str()
            .unwrap()
            .to_string()
    }

    fn load() -> Self {
        let cuda = unsafe {
            libloading::Library::new(Self::CUDA_PATH)
                .expect("CUDA should have been loaded successfully")
        };
        let cudnn = unsafe { libloading::Library::new(Self::cudnn_path()).unwrap() };
        Self { cuda, cudnn }
    }
}

macro_rules! implemented_test {
    ($($abi:literal fn $fn_name:ident( $( $arg_id:ident : $arg_type:ty ),* ) -> $ret_type:ty;)* ) => {
        pub(crate) trait CudnnApi {
            fn new() -> Self;
            $(
                #[allow(non_snake_case, dead_code)]
                fn $fn_name(&self, $( $arg_id : $arg_type ),* ) {
                    paste::paste!{ self.[< $fn_name _unchecked >]( $( $arg_id ),* ) }.unwrap()
                }
                paste::paste!{ #[allow(non_snake_case, dead_code)] fn [< $fn_name _unchecked>](&self, $( $arg_id : $arg_type ),* ) -> $ret_type; }
            )*
        }



        impl CudnnApi for Cuda {
            fn new() -> Self { Self::load() }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    let func = unsafe { self.cudnn.get::<unsafe extern $abi fn ( $( $arg_type ),* ) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()) }.unwrap();
                    unsafe { (func)( $( $arg_id ),* ) }
                }}
            )*
        }

        impl CudnnApi for Zluda {
            fn new() -> Self { Self }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    unsafe { super::$fn_name( $( $arg_id ),* ) }
                }}
            )*
        }
    };
}

macro_rules! ignore {
    ($($abi:literal fn $fn_name:ident( $( $arg_id:ident : $arg_type:ty ),* ) -> $ret_type:ty;)* ) => {};
}

cuda_macros::cudnn9_function_declarations!(
    implemented_test,
    ignore
        <= [
            cudnnGetMaxDeviceVersion,
            cudnnGetCudartVersion,
            cudnnGetLastErrorString,
            cudnnGetErrorString,
            cudnnGetVersion
        ]
);
