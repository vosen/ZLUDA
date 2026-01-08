use std::{
    ops::Deref,
    sync::{RwLock, RwLockReadGuard},
};

use cuda_types::{cublas::*, cublaslt::cublasLtHandle_t};
use hipblaslt_sys::hipblasLtHandle_t;
use rocblas_sys::rocblas_handle;

use crate::{from_cuda_object, ZludaObject};

pub struct BlasHandle {
    pub blas_lt: RwLock<Option<BlasLtHandle>>,
    pub rocblas: rocblas_handle,
}

pub struct BlasLtHandle {
    pub hipblas_lt: hipblasLtHandle_t,
}

impl BlasHandle {
    pub fn new(hipblas_lt: hipblasLtHandle_t, rocblas: rocblas_handle) -> Self {
        let blas_lt_handle = BlasLtHandle::new(hipblas_lt);
        Self {
            blas_lt: RwLock::new(Some(blas_lt_handle)),
            rocblas,
        }
    }
}
impl ZludaObject for BlasHandle {
    const COOKIE: usize = 0x57c3fdb0fd72b08e;
    type Error = cublasError_t;
    type CudaHandle = cublasHandle_t;
    fn drop_checked(&mut self) -> cublasStatus_t {
        let mut write_guard = self
            .blas_lt
            .write()
            .map_err(|_| cublasError_t::INTERNAL_ERROR)?;
        if let Some(mut blas_lt) = write_guard.take() {
            blas_lt.drop_checked()?;
        }
        unsafe { rocblas_sys::rocblas_destroy_handle(self.rocblas) }?;

        Ok(())
    }
}
from_cuda_object!(BlasHandle);

impl BlasLtHandle {
    pub fn new(hipblas_lt: hipblasLtHandle_t) -> Self {
        Self { hipblas_lt }
    }
}
impl ZludaObject for BlasLtHandle {
    const COOKIE: usize = 0xd1d9cb43416c9620;
    type Error = cublasError_t;
    type CudaHandle = cublasLtHandle_t;
    fn drop_checked(&mut self) -> cublasStatus_t {
        unsafe { hipblaslt_sys::hipblasLtDestroy(self.hipblas_lt) }?;
        Ok(())
    }
}

pub enum BlasLtHandleGuard<'a> {
    Direct(&'a BlasLtHandle),
    RwLock(RwLockReadGuard<'a, Option<BlasLtHandle>>),
}

impl<'a> Deref for BlasLtHandleGuard<'a> {
    type Target = BlasLtHandle;

    fn deref(&self) -> &Self::Target {
        match self {
            BlasLtHandleGuard::Direct(handle) => handle,
            BlasLtHandleGuard::RwLock(guard) => guard
                .as_ref()
                .expect("BlasLtHandleGuard::RwLock invariant: always contains Some"),
        }
    }
}

fn as_blas_lt_handle<'a>(handle: &'a cublasLtHandle_t) -> Option<BlasLtHandleGuard<'a>> {
    // Try interpreting as a direct BlasLtHandle
    let live_check = crate::as_ref(handle)?;
    if let Ok(blas_lt) = live_check.as_result() {
        return Some(BlasLtHandleGuard::Direct(blas_lt));
    }

    // Fallback: handle may be a BlasHandle with an embedded BlasLtHandle
    let live_check = crate::as_ref::<BlasHandle>(unsafe { std::mem::transmute(handle) })?;
    let blas_handle = live_check.as_result().ok()?;
    let guard = blas_handle.blas_lt.read().ok()?;
    // Verify the Option is Some before constructing
    if guard.is_none() {
        return None;
    }
    Some(BlasLtHandleGuard::RwLock(guard))
}

impl<'a> crate::FromCuda<'a, cublasLtHandle_t, cublasError_t> for BlasLtHandleGuard<'a> {
    fn from_cuda(handle: &'a cublasLtHandle_t) -> Result<BlasLtHandleGuard<'a>, cublasError_t> {
        as_blas_lt_handle(handle).ok_or(cublasError_t::INVALID_VALUE)
    }
}
