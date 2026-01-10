use cuda_types::{cublas::*, cublaslt::cublasLtHandle_t};
use hipblaslt_sys::hipblasLtHandle_t;
use rocblas_sys::rocblas_handle;

use crate::{from_cuda_object, ZludaObject};

#[repr(C)]
pub struct BlasHandle {
    pub hipblas_lt: hipblasLtHandle_t,
    pub rocblas: rocblas_handle,
}

#[repr(C)]
pub struct BlasLtHandle {
    pub hipblas_lt: hipblasLtHandle_t,
    pub _rocblas_padding: rocblas_handle,
}

// This is perhaps a bit of a hack, but zluda_blaslt needs to be able to take both a BlasHandle and
// a BlasLtHandle. We make both of them newtypes for BlasHandles, using #[repr(C)] to ensure that
// they have the same layout, and then give their ZludaObjects the same COOKIE value so that they
// are interchangeable.

const BLAS_HANDLES_COOKIE: usize = 0x57c3fdb0fd72b08e;

impl BlasHandle {
    pub fn new(hipblas_lt: hipblasLtHandle_t, rocblas: rocblas_handle) -> Self {
        Self {
            hipblas_lt,
            rocblas,
        }
    }
}
impl ZludaObject for BlasHandle {
    const COOKIE: usize = BLAS_HANDLES_COOKIE;
    type Error = cublasError_t;
    type CudaHandle = cublasHandle_t;
    fn drop_checked(&mut self) -> cublasStatus_t {
        unsafe { rocblas_sys::rocblas_destroy_handle(self.rocblas) }?;
        unsafe { hipblaslt_sys::hipblasLtDestroy(self.hipblas_lt) }?;

        Ok(())
    }
}
from_cuda_object!(BlasHandle);

impl BlasLtHandle {
    pub fn new(hipblas_lt: hipblasLtHandle_t) -> Self {
        Self {
            hipblas_lt,
            _rocblas_padding: unsafe { std::mem::zeroed() },
        }
    }
}
impl ZludaObject for BlasLtHandle {
    const COOKIE: usize = BLAS_HANDLES_COOKIE;
    type Error = cublasError_t;
    type CudaHandle = cublasLtHandle_t;
    fn drop_checked(&mut self) -> cublasStatus_t {
        unsafe { hipblaslt_sys::hipblasLtDestroy(self.hipblas_lt) }?;
        Ok(())
    }
}
from_cuda_object!(BlasLtHandle);

#[cfg(test)]
mod tests {
    use rocblas_sys::_rocblas_handle;

    use super::*;

    #[test]
    fn blas_handle_as_blaslt_handle() {
        let hipblas_lt = hipblasLtHandle_t(1234 as *mut std::ffi::c_void);
        let rocblas = rocblas_handle(5678 as *mut _rocblas_handle);
        let blas_handle = BlasHandle::new(hipblas_lt, rocblas);

        let cublas_handle = BlasHandle::wrap(blas_handle);
        let cublaslt_handle: cublasLtHandle_t = unsafe { std::mem::transmute(cublas_handle) };

        let blaslt_handle: &BlasLtHandle = crate::as_ref(&cublaslt_handle)
            .expect("cublaslt_handle should not be null")
            .as_result()
            .expect("cublaslt_handle should not fail liveness check");

        assert_eq!(blaslt_handle.hipblas_lt.0 as usize, 1234);
    }

    #[test]
    fn blaslt_handle_as_blas_handle() {
        // cublasLtHandle_t cannot be used as a cublasHandle_t, so we treat passing one in as a
        // null handle
        let hipblas_lt = hipblasLtHandle_t(1234 as *mut std::ffi::c_void);
        let blaslt_handle = BlasLtHandle::new(hipblas_lt);

        let cublaslt_handle = BlasLtHandle::wrap(blaslt_handle);
        let cublas_handle: cublasHandle_t = unsafe { std::mem::transmute(cublaslt_handle) };

        let blas_handle: &BlasHandle = crate::as_ref(&cublas_handle)
            .expect("cublas_handle should not be null")
            .as_result()
            .expect("cublas_handle should not fail liveness check");

        assert_eq!(blas_handle.rocblas.0 as usize, 0);
    }
}
