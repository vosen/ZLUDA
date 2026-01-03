use std::sync::OnceLock;

use cuda_types::{cublas::*, cublaslt::cublasLtHandle_t};
use hipblaslt_sys::hipblasLtHandle_t;
use rocblas_sys::rocblas_handle;

use crate::{from_cuda_object, ZludaObject};

pub struct BlasHandles {
    pub rocblas: OnceLock<rocblas_handle>,
    pub hipblas_lt: OnceLock<Result<hipblasLtHandle_t, cublasError_t>>,
}

impl BlasHandles {
    pub fn new() -> Self {
        Self {
            rocblas: OnceLock::new(),
            hipblas_lt: OnceLock::new(),
        }
    }

    pub fn drop_checked(&mut self) -> cublasStatus_t {
        if let Some(rocblas) = self.rocblas.get() {
            unsafe { rocblas_sys::rocblas_destroy_handle(*rocblas) }?;
        }

        if let Some(Ok(hipblas_lt)) = self.hipblas_lt.get() {
            unsafe { hipblaslt_sys::hipblasLtDestroy(*hipblas_lt) }?;
        }

        Ok(())
    }
}

// This is perhaps a bit of a hack, but zluda_blaslt needs to be able to take both a BlasHandle and
// a BlasLtHandle. We make both of them newtypes for BlasHandles, using #[repr(transparent)] to
// ensure that they have the same layout, and then give their ZludaObjects the same COOKIE value so
// that they are interchangeable.

const BLAS_HANDLES_COOKIE: usize = 0x57c3fdb0fd72b08e;

macro_rules! define_blas_handle {
    ($name:ident, $cuda_handle:ty) => {
        #[repr(transparent)]
        pub struct $name(pub BlasHandles);

        impl $name {
            pub fn new() -> Self {
                Self(BlasHandles::new())
            }
        }

        impl ZludaObject for $name {
            const COOKIE: usize = BLAS_HANDLES_COOKIE;

            type Error = cublasError_t;
            type CudaHandle = $cuda_handle;

            fn drop_checked(&mut self) -> cublasStatus_t {
                self.0.drop_checked()
            }
        }

        from_cuda_object!($name);
    };
}

define_blas_handle!(BlasHandle, cublasHandle_t);
define_blas_handle!(BlasLtHandle, cublasLtHandle_t);
