use std::io::prelude::*;
use std::{fs::File, ptr};

use cuda::{CUdeviceptr, CUfunction, CUresult, CUstream};
use libc::c_void;

macro_rules! extern_redirect {
    (pub fn $fn_name:ident ( $($arg_id:ident: $arg_type:ty),* $(,)? ) -> $ret_type:ty ;) => {
        #[no_mangle]
        pub fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
            unsafe { $crate::init_libcuda_handle() };
            let name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
            let fn_ptr = unsafe { libc::dlsym($crate::LIBCUDA_HANDLE, name.as_ptr() as *const _) };
            if fn_ptr == std::ptr::null_mut() {
                return CUresult::CUDA_ERROR_UNKNOWN;
            }
            println!("{}", stringify!($fn_name));
            let typed_fn = unsafe { std::mem::transmute::<_, fn( $( $arg_id : $arg_type),* ) -> $ret_type>(fn_ptr) };
            typed_fn($( $arg_id ),*)
        }
    };
}

macro_rules! extern_redirect_with {
    (
        pub fn $fn_name:ident ( $($arg_id:ident: $arg_type:ty),* $(,)? ) -> $ret_type:ty ;
        $receiver:path ;
    ) => {
        #[no_mangle]
        pub fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
            unsafe { $crate::init_libcuda_handle() };
            let continuation = |$( $arg_id : $arg_type),* | {
                let name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
                let fn_ptr = unsafe { libc::dlsym($crate::LIBCUDA_HANDLE, name.as_ptr() as *const _) };
                if fn_ptr == std::ptr::null_mut() {
                    return CUresult::CUDA_ERROR_UNKNOWN;
                }
                println!("{}", stringify!($fn_name));
                let typed_fn = unsafe { std::mem::transmute::<_, fn( $( $arg_id : $arg_type),* ) -> $ret_type>(fn_ptr) };
                typed_fn($( $arg_id ),*)
            };
            unsafe { $receiver($( $arg_id ),* , continuation) }
        }
    };
}

#[allow(warnings)]
mod cuda;

pub static mut LIBCUDA_HANDLE: *mut c_void = ptr::null_mut();
pub static mut BUFFERS: Vec<(usize, usize)> = Vec::new();

// We are doing dlopen here instead of just using LD_PRELOAD,
// it's because CUDA Runtime API does dlopen to open libcuda.so, which ingores LD_PRELOAD
pub unsafe fn init_libcuda_handle() {
    if LIBCUDA_HANDLE == ptr::null_mut() {
        let libcuda_handle = libc::dlopen(
            "/usr/lib/x86_64-linux-gnu/libcuda.so.1".as_ptr() as *const _,
            libc::RTLD_LOCAL | libc::RTLD_NOW,
        );
        assert_ne!(libcuda_handle, ptr::null_mut());
        LIBCUDA_HANDLE = libcuda_handle;
    }
}

pub unsafe fn cuMemAlloc_v2(
    dptr: *mut CUdeviceptr,
    bytesize: usize,
    cont: impl FnOnce(*mut CUdeviceptr, usize) -> CUresult,
) -> CUresult {
    let result = cont(dptr, bytesize);
    assert_eq!(result, CUresult::CUDA_SUCCESS);
    let start = (*dptr).0 as usize;
    BUFFERS.push((start, bytesize));
    CUresult::CUDA_SUCCESS
}

pub unsafe fn cuLaunchKernel(
    f: CUfunction,
    gridDimX: ::std::os::raw::c_uint,
    gridDimY: ::std::os::raw::c_uint,
    gridDimZ: ::std::os::raw::c_uint,
    blockDimX: ::std::os::raw::c_uint,
    blockDimY: ::std::os::raw::c_uint,
    blockDimZ: ::std::os::raw::c_uint,
    sharedMemBytes: ::std::os::raw::c_uint,
    hStream: CUstream,
    kernelParams: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
    cont: impl FnOnce(
        CUfunction,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        CUstream,
        *mut *mut ::std::os::raw::c_void,
        *mut *mut ::std::os::raw::c_void,
    ) -> CUresult,
) -> CUresult {
    let mut error = CUresult::CUDA_SUCCESS;
    for i in 0..3 {
        let dev_ptr = *(*kernelParams.add(i) as *mut usize);
        let (start, len) = BUFFERS
            .iter()
            .find(|(start, _)| *start == dev_ptr as usize)
            .unwrap();
        let mut output = vec![0u8; *len];
        error = cuda::cuMemcpyDtoH_v2(
            output.as_mut_ptr() as *mut _,
            CUdeviceptr(*start as u64),
            *len,
        );
        let mut file = File::create(format!("vectoradd_arg_in_{}.bin", i)).unwrap();
        file.write_all(&mut output).unwrap();
        assert_eq!(error, CUresult::CUDA_SUCCESS);
    }
    error = cont(
        f,
        gridDimX,
        gridDimY,
        gridDimZ,
        blockDimX,
        blockDimY,
        blockDimZ,
        sharedMemBytes,
        hStream,
        kernelParams,
        extra,
    );
    assert_eq!(error, CUresult::CUDA_SUCCESS);
    error = cuda::cuStreamSynchronize(hStream);
    assert_eq!(error, CUresult::CUDA_SUCCESS);
    for i in 0..3 {
        let dev_ptr = *(*kernelParams.add(i) as *mut usize);
        let (start, len) = BUFFERS
            .iter()
            .find(|(start, _)| *start == dev_ptr as usize)
            .unwrap();
        let mut output = vec![0u8; *len];
        error = cuda::cuMemcpyDtoH_v2(
            output.as_mut_ptr() as *mut _,
            CUdeviceptr(*start as u64),
            *len,
        );
        let mut file = File::create(format!("vectoradd_arg_out_{}.bin", i)).unwrap();
        file.write_all(&mut output).unwrap();
        assert_eq!(error, CUresult::CUDA_SUCCESS);
    }
    CUresult::CUDA_SUCCESS
}
