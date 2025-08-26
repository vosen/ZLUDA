use cuda_types::{cublas::*, cublaslt::cublasLtHandle_t};
use zluda_common::{from_cuda_object, ZludaObject};

pub struct Handle {
    _handle: usize,
}

impl ZludaObject for Handle {
    const COOKIE: usize = 0x49dec801578301ee;

    type Error = cublasError_t;
    type CudaHandle = cublasLtHandle_t;

    fn drop_checked(&mut self) -> cublasStatus_t {
        Ok(())
    }
}

from_cuda_object!(Handle);

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cublasStatus_t {
    cublasStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn get_status_name(_status: cublasStatus_t) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_status_string(_status: cublasStatus_t) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_version() -> usize {
    todo!()
}

pub(crate) fn get_cudart_version() -> usize {
    todo!()
}

pub(crate) fn disable_cpu_instructions_set_mask(_mask: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    todo!()
}

pub(crate) fn create(handle: &mut cuda_types::cublaslt::cublasLtHandle_t) -> cublasStatus_t {
    *handle = Handle { _handle: 0 }.wrap();
    Ok(())
}

pub(crate) fn destroy(handle: cuda_types::cublaslt::cublasLtHandle_t) -> cublasStatus_t {
    zluda_common::drop_checked::<Handle>(handle)
}
