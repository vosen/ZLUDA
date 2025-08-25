use cuda_types::{cublas::*, cuda::*};
use hip_runtime_sys::*;
use rocblas_sys::*;
use std::{
    ffi::CStr,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr,
};

pub trait CudaErrorType {
    const INVALID_VALUE: Self;
    const NOT_SUPPORTED: Self;
}

impl CudaErrorType for CUerror {
    const INVALID_VALUE: Self = Self::INVALID_VALUE;
    const NOT_SUPPORTED: Self = Self::NOT_SUPPORTED;
}

impl CudaErrorType for cublasError_t {
    const INVALID_VALUE: Self = Self::INVALID_VALUE;
    const NOT_SUPPORTED: Self = Self::NOT_SUPPORTED;
}

/// Used to try to convert CUDA API values into our internal representation.
///
/// Similar to [`TryFrom`], but we can implement this for primitive types. We also provide conversions from pointers to references.
pub trait FromCuda<'a, T, E: CudaErrorType>: Sized {
    /// Tries to convert to this type.
    fn from_cuda(t: &'a T) -> Result<Self, E>;
}

macro_rules! from_cuda_nop {
    ($($type_:ty),*) => {
        $(
            impl<'a, E: CudaErrorType> FromCuda<'a, $type_, E> for $type_ {
                fn from_cuda(x: &'a $type_) -> Result<Self, E> {
                    Ok(*x)
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut $type_, E> for &'a mut $type_ {
                fn from_cuda(x: &'a *mut $type_) -> Result<Self, E> {
                    match unsafe { x.as_mut() } {
                        Some(x) => Ok(x),
                        None => Err(E::INVALID_VALUE),
                    }
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *const $type_, E> for &'a $type_ {
                fn from_cuda(x: &'a *const $type_) -> Result<Self, E> {
                    match unsafe { x.as_ref() } {
                        Some(x) => Ok(x),
                        None => Err(E::INVALID_VALUE),
                    }
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut $type_, E> for Option<&'a mut $type_> {
                fn from_cuda(x: &'a *mut $type_) -> Result<Self, E> {
                    Ok(unsafe { x.as_mut() })
                }
            }
        )*
    };
}

macro_rules! from_cuda_transmute {
    ($($from:ty => $to:ty),*) => {
        $(
            impl<'a, E: CudaErrorType> FromCuda<'a, $from, E> for $to {
                fn from_cuda(x: &'a $from) -> Result<Self, E> {
                    Ok(unsafe { std::mem::transmute(*x) })
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut $from, E> for &'a mut $to {
                fn from_cuda(x: &'a *mut $from) -> Result<Self, E> {
                    match unsafe { x.cast::<$to>().as_mut() } {
                        Some(x) => Ok(x),
                        None => Err(E::INVALID_VALUE),
                    }
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut $from, E> for * mut $to {
                fn from_cuda(x: &'a *mut $from) -> Result<Self, E> {
                    Ok(x.cast::<$to>())
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut *const $from, E> for *mut *const $to {
                fn from_cuda(x: &'a *mut *const $from) -> Result<Self, E> {
                    Ok(x.cast::<*const $to>())
                }
            }
        )*
    };
}

/// Implement the [`FromCuda`] trait for a [`ZludaObject`].
#[macro_export]
macro_rules! from_cuda_object {
    ($($type_:ty),*) => {
        $(
            impl<'a> zluda_common::FromCuda<'a, <$type_ as zluda_common::ZludaObject>::CudaHandle, <$type_ as zluda_common::ZludaObject>::Error> for &'a $type_ {
                fn from_cuda(handle: &'a <$type_ as zluda_common::ZludaObject>::CudaHandle) -> Result<&'a $type_, <$type_ as zluda_common::ZludaObject>::Error> {
                    Ok(zluda_common::as_ref(handle).as_result()?)
                }
            }
        )*
    };
}

from_cuda_nop!(
    *mut i8,
    *mut i32,
    *mut u64,
    *mut usize,
    *const f32,
    *mut f32,
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_char,
    *mut ::core::ffi::c_void,
    *mut *mut ::core::ffi::c_void,
    u8,
    i32,
    u32,
    u64,
    i64,
    usize,
    cuda_types::cuda::CUdevprop,
    CUdevice_attribute,
    CUdriverProcAddressQueryResult,
    CUjit_option,
    CUlibraryOption,
    CUmoduleLoadingMode,
    CUuuid,
    CUlibrary,
    CUmodule,
    CUcontext,
    cublasHandle_t,
    cublasStatus_t
);
from_cuda_transmute!(
    CUuuid => hipUUID,
    CUfunction => hipFunction_t,
    CUfunction_attribute => hipFunction_attribute,
    CUstream => hipStream_t,
    CUpointer_attribute => hipPointer_attribute,
    CUdeviceptr_v2 => hipDeviceptr_t,
    CUevent => hipEvent_t,
    // This is safe because HIP's enum is the subset of CUDA's enum and
    // this type is used purely as a function result
    CUstreamCaptureStatus => hipStreamCaptureStatus,
    CUgraph => hipGraph_t,
    CUstreamCaptureMode => hipStreamCaptureMode,
    CUgraphNode => hipGraphNode_t
);

impl<'a, E: CudaErrorType> FromCuda<'a, CUlimit, E> for hipLimit_t {
    fn from_cuda(limit: &'a CUlimit) -> Result<Self, E> {
        Ok(match *limit {
            CUlimit::CU_LIMIT_STACK_SIZE => hipLimit_t::hipLimitStackSize,
            CUlimit::CU_LIMIT_PRINTF_FIFO_SIZE => hipLimit_t::hipLimitPrintfFifoSize,
            CUlimit::CU_LIMIT_MALLOC_HEAP_SIZE => hipLimit_t::hipLimitMallocHeapSize,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, *const ::core::ffi::c_char, E> for &CStr {
    fn from_cuda(s: &'a *const ::core::ffi::c_char) -> Result<Self, E> {
        if *s != ptr::null() {
            Ok(unsafe { CStr::from_ptr(*s) })
        } else {
            Err(E::INVALID_VALUE)
        }
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, *const ::core::ffi::c_void, E> for &'a ::core::ffi::c_void {
    fn from_cuda(x: &'a *const ::core::ffi::c_void) -> Result<Self, E> {
        match unsafe { x.as_ref() } {
            Some(x) => Ok(x),
            None => Err(E::INVALID_VALUE),
        }
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasOperation_t, E> for rocblas_operation {
    fn from_cuda(t: &'a cublasOperation_t) -> Result<Self, E> {
        Ok(match *t {
            cublasOperation_t::CUBLAS_OP_N => rocblas_operation::rocblas_operation_none,
            cublasOperation_t::CUBLAS_OP_T => rocblas_operation::rocblas_operation_transpose,
            cublasOperation_t::CUBLAS_OP_C => {
                rocblas_operation::rocblas_operation_conjugate_transpose
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasMath_t, E> for rocblas_math_mode {
    fn from_cuda(mode: &'a cublasMath_t) -> Result<Self, E> {
        Ok(match *mode {
            cublasMath_t::CUBLAS_DEFAULT_MATH => rocblas_math_mode_::rocblas_default_math,
            cublasMath_t::CUBLAS_TF32_TENSOR_OP_MATH => rocblas_math_mode::rocblas_xf32_xdl_math_op,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

/// Represents an object that can be sent across the API boundary.
///
/// Some CUDA calls operate on an opaque handle. For example, `cuModuleLoadData` will load a
/// module's data and set the `module` output argument to a new `CUmodule`. Then, other functions
/// like `cuModuleGetFunction` can take that `CUmodule` as an argument.
pub trait ZludaObject: Sized + Send + Sync {
    /// This is a unique identifier used by [`LiveCheck`] for runtime type and lifetime checking.
    ///
    /// You can generate a new one with Python:
    ///
    /// ```python
    /// import random
    /// hex(random.getrandbits(64))
    /// ```
    const COOKIE: usize;

    /// The value of [`Self::Error`] used to represent a type check failure or use after free.
    const LIVENESS_FAIL: Self::Error = Self::Error::INVALID_VALUE;

    /// The error type that should be used. This is generally specific to the library this trait
    /// is being implemented in – for example, a [`ZludaObject`] in `zluda` should use the
    /// [`CUerror`] type, and a [`ZludaObject`] in `zluda_blas` should use the [`cublasStatus_t`]
    /// type.
    type Error: CudaErrorType;
    /// The handle type that an object of this trait should should be wrapped as.
    type CudaHandle: Sized;

    /// Executes the destructor for this type.
    fn drop_checked(&mut self) -> Result<(), Self::Error>;

    /// Wraps an object of this trait in a [`LiveCheck`] that can be used for runtime type and
    /// lifetime checking, and returns it as an opaque [`Self::CudaHandle`] that can be passed to
    /// the API caller.
    fn wrap(self) -> Self::CudaHandle {
        unsafe { mem::transmute_copy(&LiveCheck::wrap(self)) }
    }
}

/// Wraps a [`ZludaObject`] and provides runtime type and lifetime checking.
///
/// Arbitrary memory can be cast to a value of this type, and then [`LiveCheck::as_result`] can be
/// used to get the wrapped [`ZludaObject`] value, if it is valid.
#[repr(C)]
pub struct LiveCheck<T: ZludaObject> {
    cookie: usize,
    /// The wrapped [`ZludaObject`].
    pub data: MaybeUninit<T>,
}

impl<T: ZludaObject> LiveCheck<T> {
    /// Wraps `data` as a valid, initialized `LiveCheck`.
    pub fn new(data: T) -> Self {
        LiveCheck {
            cookie: T::COOKIE,
            data: MaybeUninit::new(data),
        }
    }

    /// Returns this value as an opaque `T::CudaHandle`.
    pub fn as_handle(&self) -> T::CudaHandle {
        unsafe { mem::transmute_copy(&self) }
    }

    fn wrap(data: T) -> *mut Self {
        Box::into_raw(Box::new(Self::new(data)))
    }

    /// Checks if this value represents a valid and initialized value of `T` and returns it.
    /// Returns `T::LIVENESS_FAIL` if it does not.
    pub fn as_result(&self) -> Result<&T, T::Error> {
        if self.cookie == T::COOKIE {
            Ok(unsafe { self.data.assume_init_ref() })
        } else {
            Err(T::LIVENESS_FAIL)
        }
    }

    // This looks like nonsense, but it's not. There are two cases:
    // Err(CUerror) -> meaning that the object is invalid, this pointer does not point into valid memory
    // Ok(maybe_error) -> meaning that the object is valid, we dropped everything, but there *might*
    //                    an error in the underlying runtime that we want to propagate
    #[must_use]
    fn drop_checked(&mut self) -> Result<Result<(), T::Error>, T::Error> {
        if self.cookie == T::COOKIE {
            self.cookie = 0;
            let result = unsafe { self.data.assume_init_mut().drop_checked() };
            unsafe { MaybeUninit::assume_init_drop(&mut self.data) };
            Ok(result)
        } else {
            Err(T::LIVENESS_FAIL)
        }
    }
}

/// Cast a `T::CudaHandle` reference to a [`LiveCheck`] reference, preserving the lifetime.
pub fn as_ref<'a, T: ZludaObject>(
    handle: &'a T::CudaHandle,
) -> &'a ManuallyDrop<Box<LiveCheck<T>>> {
    unsafe { mem::transmute(handle) }
}

/// Try to drop `handle`.
///
/// Returns an error if `handle` is not initialized, not a value of `T`, or if `T::drop_checked`
/// returns an error.
pub fn drop_checked<T: ZludaObject>(handle: T::CudaHandle) -> Result<(), T::Error> {
    let mut wrapped_object: ManuallyDrop<Box<LiveCheck<T>>> =
        unsafe { mem::transmute_copy(&handle) };
    let underlying_error = LiveCheck::drop_checked(&mut wrapped_object)?;
    unsafe { ManuallyDrop::drop(&mut wrapped_object) };
    underlying_error
}
