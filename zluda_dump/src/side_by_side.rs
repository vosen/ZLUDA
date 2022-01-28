use cuda_base::cuda_function_declarations;
use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::ptr::NonNull;
use std::{marker::PhantomData, os::raw::c_void};

use crate::os;

struct DynamicFn<T> {
    pointer: usize,
    _marker: PhantomData<T>,
}

impl<T> Default for DynamicFn<T> {
    fn default() -> Self {
        DynamicFn {
            pointer: 0,
            _marker: PhantomData,
        }
    }
}

impl<T> DynamicFn<T> {
    unsafe fn get(&mut self, lib: *mut c_void, name: &[u8]) -> Option<T> {
        match self.pointer {
            0 => {
                let addr = os::get_proc_address(lib, CStr::from_bytes_with_nul_unchecked(name));
                if addr == ptr::null_mut() {
                    self.pointer = 1;
                    return None;
                } else {
                    self.pointer = addr as _;
                }
            }
            1 => return None,
            _ => {}
        }
        Some(mem::transmute_copy(&self.pointer))
    }
}

pub(crate) struct CudaDynamicFns {
    lib_handle: NonNull<::std::ffi::c_void>,
    fn_table: CudaFnTable,
}

impl CudaDynamicFns {
    pub(crate) unsafe fn load_library(path: &str) -> Option<Self> {
        let lib_handle = NonNull::new(os::load_library(path));
        lib_handle.map(|lib_handle| CudaDynamicFns {
            lib_handle,
            fn_table: CudaFnTable::default(),
        })
    }
}

macro_rules! emit_cuda_fn_table {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path);*) => {
        #[derive(Default)]
        struct CudaFnTable {
            $($fn_name: DynamicFn<extern $abi fn ( $($arg_id : $arg_type),* ) -> $ret_type>),*
        }

        impl CudaDynamicFns {
            $(
                #[allow(dead_code)]
                pub(crate) fn $fn_name(&mut self, $($arg_id : $arg_type),*) -> Option<$ret_type> {
                    let func = unsafe { self.fn_table.$fn_name.get(self.lib_handle.as_ptr(), concat!(stringify!($fn_name), "\0").as_bytes()) };
                    func.map(|f| f($($arg_id),*) )
                }
            )*
        }
    };
}

cuda_function_declarations!(cuda_types, emit_cuda_fn_table, emit_cuda_fn_table, []);
