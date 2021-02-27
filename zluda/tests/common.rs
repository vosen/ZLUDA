#![allow(non_snake_case)]
use cuda_base::cuda_function_declarations;
use std::ffi::c_void;

macro_rules! unimplemented_cuda_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path);*) => {
        pub trait CudaDriverFns {
            fn new() -> Self;
            fn is_nvidia() -> bool;
            $(
                unsafe fn $fn_name (&self, $( $arg_id : $arg_type),* ) -> $ret_type;
            )*
        }

        #[derive(Copy, Clone)]
        pub struct Cuda {
            lib: *mut c_void
        }

        unsafe impl Send for Cuda {}
        unsafe impl Sync for Cuda {}

        impl CudaDriverFns for Cuda {
            fn new() -> Self {
                let lib = unsafe { os::load_cuda() };
                Self { lib }
            }
            fn is_nvidia() -> bool { true }
            $(
                unsafe fn $fn_name (&self, $( $arg_id : $arg_type),* ) -> $ret_type {
                    let fn_ptr = os::get_proc_address(self.lib, concat!(stringify!($fn_name), "\0").as_bytes());
                    let cu_fn = std::mem::transmute::<_, unsafe extern $abi fn( $( $arg_id : $arg_type),* )  -> $ret_type>(fn_ptr);
                    cu_fn ( $( $arg_id),* )
                }
            )*
        }

        #[derive(Copy, Clone)]
        pub struct Zluda;

        impl CudaDriverFns for Zluda {
            fn new() -> Self { Self }
            fn is_nvidia() -> bool { false }
            $(
                unsafe fn $fn_name (&self, $( $arg_id : $arg_type),* ) -> $ret_type {
                    zluda::cuda::$fn_name ( $( $arg_id),* )
                }
            )*
        }
    };
}

cuda_function_declarations!(cuda_types, unimplemented_cuda_fn, UNUSED, []);

#[macro_export]
macro_rules! cuda_driver_test {
    ($func:ident) => {
        paste::paste! {
            #[test]
            #[allow(non_snake_case)]
            fn [<$func _zluda>]() {
                unsafe { $func::<crate::common::Zluda>(crate::common::Zluda::new()) }
            }

            #[test]
            #[allow(non_snake_case)]
            fn [<$func _cuda>]() {
                unsafe { $func::<crate::common::Cuda>(crate::common::Cuda::new()) }
            }
        }
    };
}

#[allow(dead_code)]
pub const CU_STREAM_LEGACY: cuda_types::CUstream = 1 as *mut _;
#[allow(dead_code)]
pub const CU_STREAM_PER_THREAD: cuda_types::CUstream = 2 as *mut _;

#[cfg(windows)]
mod os {
    use std::ffi::c_void;

    pub unsafe fn load_cuda() -> *mut c_void {
        use winapi::um::libloaderapi::LoadLibraryA;
        let result = LoadLibraryA(b"C:\\Windows\\System32\\nvcuda.dll\0".as_ptr() as _);
        if result == std::ptr::null_mut() {
            panic!("{:?}", std::io::Error::last_os_error());
        }
        result as _
    }

    pub unsafe fn get_proc_address(handle: *mut c_void, func: &[u8]) -> *mut c_void {
        use winapi::um::libloaderapi::GetProcAddress;
        GetProcAddress(handle as _, func.as_ptr() as *const _) as _
    }
}

#[cfg(not(windows))]
mod os {
    use std::ffi::c_void;
    use libc;
    use std::ffi::CStr;

    #[cfg(test)]
    pub unsafe fn load_cuda() -> *mut c_void {
        // Ubuntu path
        let mut result = libc::dlopen(
            b"/usr/lib/x86_64-linux-gnu/libcuda.so.1\0".as_ptr() as _,
            libc::RTLD_LOCAL | libc::RTLD_LAZY,
        );
        // RHEL path
        if result == std::ptr::null_mut() {
            result = libc::dlopen(
                b"/usr/lib64/libcuda.so.1\0".as_ptr() as _,
                libc::RTLD_LOCAL | libc::RTLD_LAZY,
            );
        }
        if result == std::ptr::null_mut() {
            panic!("{}", CStr::from_ptr(libc::dlerror()).to_string_lossy());
        }
        result
    }

    #[cfg(test)]
    pub unsafe fn get_proc_address(handle: *mut c_void, func: &[u8]) -> *mut c_void {
        libc::dlsym(handle, func.as_ptr() as *const _)
    }
}
