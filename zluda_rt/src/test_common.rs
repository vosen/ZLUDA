use libloading::{Library, Symbol};
use optix_types::*;
use std::ffi::c_void;

#[cfg(windows)]
const OPTIX_DLL: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/bin/optix.6.5.0.dll");

#[cfg(not(windows))]
const OPTIX_DLL: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/bin/liboptix.so.6.5.0");

fn load_optix() -> Library {
    unsafe { Library::new(OPTIX_DLL).unwrap() }
}

fn get_proc_address<'a>(lib: &'a Library, name: &str) -> Symbol<'a, *mut c_void> {
    unsafe { lib.get::<*mut c_void>(name.as_bytes()).unwrap() }
}

pub struct Cuda {
    lib: Library
}

unsafe impl Send for Cuda {}
unsafe impl Sync for Cuda {}

#[derive(Copy, Clone)]
pub struct Zluda;

macro_rules! optix6_fn_table_void{
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* $(,)? ) -> $ret_type:ty);*) => {
        pub trait OptixFnsVoid {
            $(
                unsafe fn $fn_name (&self, $( $arg_id : $arg_type),* ) -> $ret_type;
            )*
        }

        impl OptixFnsVoid for Cuda {
            $(
                unsafe fn $fn_name (&self, $( $arg_id : $arg_type),* ) -> $ret_type {
                    let fn_ptr = crate::test_common::get_proc_address(&self.lib, concat!(stringify!($fn_name), "\0"));
                    let cu_fn = std::mem::transmute::<_, unsafe extern $abi fn( $( $arg_id : $arg_type),* )  -> $ret_type>(fn_ptr);
                    cu_fn ( $( $arg_id),* )
                }
            )*
        }

        impl OptixFnsVoid for Zluda {
            $(
                unsafe fn $fn_name (&self, $( $arg_id : $arg_type),* ) -> $ret_type {
                    crate::$fn_name ( $( $arg_id),* )
                }
            )*
        }
    };
}

macro_rules! optix6_fn_table_rtresult {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* $(,)? ) -> $ret_type:ty);*) => {
        pub trait OptixFns: OptixFnsVoid {
            fn new() -> Self;
            $(
                paste::paste! { 
                    unsafe fn $fn_name (&self, $( $arg_id : $arg_type),* ) {
                        let err = self. [< $fn_name _unchecked >] (
                            $( $arg_id),* 
                        );
                        if err != optix_types::RTresult::RT_SUCCESS {
                            panic!("{}", err.0);
                        }
                    }
                    #[allow(non_snake_case)]
                    unsafe fn [< $fn_name _unchecked >] (&self, $( $arg_id : $arg_type),* ) -> $ret_type;
                }
            )*
        }

        impl OptixFns for Cuda {
            fn new() -> Self {
                let lib = crate::test_common::load_optix();
                Self { lib }
            }
            $(
                paste::paste! {
                    unsafe fn [< $fn_name _unchecked >] (&self, $( $arg_id : $arg_type),* ) -> $ret_type {
                        let fn_ptr = crate::test_common::get_proc_address(&self.lib, concat!(stringify!($fn_name), "\0"));
                        let cu_fn = std::mem::transmute::<_, unsafe extern $abi fn( $( $arg_id : $arg_type),* )  -> $ret_type>(fn_ptr);
                        cu_fn ( $( $arg_id),* )
                    }
                }
            )*
        }

        impl OptixFns for Zluda {
            fn new() -> Self { Self }
            $(
                paste::paste! {
                    unsafe fn [< $fn_name _unchecked >] (&self, $( $arg_id : $arg_type),* ) -> $ret_type {
                        crate::$fn_name ( $( $arg_id),* )
                    }
                }
            )*
        }
    };
}

optix_base::optix6_function_declarations!(optix6_fn_table_rtresult, optix6_fn_table_void, [rtContextGetErrorString]);

#[macro_export]
macro_rules! optix_test {
    ($func:ident) => {
        paste::paste! {
            #[test]
            #[allow(non_snake_case)]
            fn [<$func _zluda>]() {
                unsafe { $func::<crate::test_common::Zluda>(crate::test_common::Zluda::new()) }
            }

            #[test]
            #[allow(non_snake_case)]
            fn [<$func _cuda>]() {
                unsafe { $func::<crate::test_common::Cuda>(crate::test_common::Cuda::new()) }
            }
        }
    };
}
