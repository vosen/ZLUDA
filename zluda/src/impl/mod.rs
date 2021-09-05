use crate::cuda::{CUctx_st, CUdevice, CUdeviceptr, CUfunc_st, CUmod_st, CUresult, CUstream_st};
use std::{
    ffi::c_void,
    mem::{self, ManuallyDrop},
    os::raw::c_int,
    ptr,
    sync::Mutex,
    sync::TryLockError,
};

#[cfg(test)]
#[macro_use]
pub mod test;
pub mod device;
pub mod export_table;
pub mod function;
#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
pub(crate) mod os;

#[cfg(debug_assertions)]
pub fn unimplemented() -> CUresult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub fn unimplemented() -> CUresult {
    CUresult::CUDA_ERROR_NOT_SUPPORTED
}

pub trait HasLivenessCookie: Sized {
    const COOKIE: usize;
    const LIVENESS_FAIL: CUresult;

    fn try_drop(&mut self) -> Result<(), CUresult>;
}

// This struct is a best-effort check if wrapped value has been dropped,
// while it's inherently safe, its use coming from FFI is very unsafe
#[repr(C)]
pub struct LiveCheck<T: HasLivenessCookie> {
    cookie: usize,
    data: ManuallyDrop<T>,
}

impl<T: HasLivenessCookie> LiveCheck<T> {
    pub fn new(data: T) -> Self {
        LiveCheck {
            cookie: T::COOKIE,
            data: ManuallyDrop::new(data),
        }
    }

    fn destroy_impl(this: *mut Self) -> Result<(), CUresult> {
        let mut ctx_box = ManuallyDrop::new(unsafe { Box::from_raw(this) });
        ctx_box.try_drop()?;
        unsafe { ManuallyDrop::drop(&mut ctx_box) };
        Ok(())
    }

    unsafe fn ptr_from_inner(this: *mut T) -> *mut Self {
        let outer_ptr = (this as *mut u8).sub(mem::size_of::<usize>());
        outer_ptr as *mut Self
    }

    pub unsafe fn as_ref_unchecked(&self) -> &T {
        &self.data
    }

    pub fn as_option_mut(&mut self) -> Option<&mut T> {
        if self.cookie == T::COOKIE {
            Some(&mut self.data)
        } else {
            None
        }
    }

    pub fn as_result(&self) -> Result<&T, CUresult> {
        if self.cookie == T::COOKIE {
            Ok(&self.data)
        } else {
            Err(T::LIVENESS_FAIL)
        }
    }

    pub fn as_result_mut(&mut self) -> Result<&mut T, CUresult> {
        if self.cookie == T::COOKIE {
            Ok(&mut self.data)
        } else {
            Err(T::LIVENESS_FAIL)
        }
    }

    #[must_use]
    pub fn try_drop(&mut self) -> Result<(), CUresult> {
        if self.cookie == T::COOKIE {
            self.cookie = 0;
            self.data.try_drop()?;
            unsafe { ManuallyDrop::drop(&mut self.data) };
            return Ok(());
        }
        Err(T::LIVENESS_FAIL)
    }
}

impl<T: HasLivenessCookie> Drop for LiveCheck<T> {
    fn drop(&mut self) {
        self.cookie = 0;
    }
}

pub trait CudaRepr: Sized {
    type Impl: Sized;
}

impl<T: CudaRepr> CudaRepr for *mut T {
    type Impl = *mut T::Impl;
}

pub trait Decuda<To> {
    fn decuda(self: Self) -> To;
}

impl<T: CudaRepr> Decuda<*mut T::Impl> for *mut T {
    fn decuda(self: Self) -> *mut T::Impl {
        self as *mut _
    }
}

impl<T> From<TryLockError<T>> for CUresult {
    fn from(_: TryLockError<T>) -> Self {
        CUresult::CUDA_ERROR_ILLEGAL_STATE
    }
}

impl From<ocl_core::Error> for CUresult {
    fn from(result: ocl_core::Error) -> Self {
        match result {
            _ => CUresult::CUDA_ERROR_UNKNOWN,
        }
    }
}

impl From<hip_runtime_sys::hipError_t> for CUresult {
    fn from(result: hip_runtime_sys::hipError_t) -> Self {
        match result {
            hip_runtime_sys::hipError_t::hipErrorRuntimeMemory
            | hip_runtime_sys::hipError_t::hipErrorRuntimeOther => CUresult::CUDA_ERROR_UNKNOWN,
            hip_runtime_sys::hipError_t(e) => CUresult(e),
        }
    }
}

pub trait Encuda {
    type To: Sized;
    fn encuda(self: Self) -> Self::To;
}

impl Encuda for CUresult {
    type To = CUresult;
    fn encuda(self: Self) -> Self::To {
        self
    }
}

impl Encuda for () {
    type To = CUresult;
    fn encuda(self: Self) -> Self::To {
        CUresult::CUDA_SUCCESS
    }
}

impl<T1: Encuda<To = CUresult>, T2: Encuda<To = CUresult>> Encuda for Result<T1, T2> {
    type To = CUresult;
    fn encuda(self: Self) -> Self::To {
        match self {
            Ok(e) => e.encuda(),
            Err(e) => e.encuda(),
        }
    }
}

unsafe fn transmute_lifetime<'a, 'b, T: ?Sized>(t: &'a T) -> &'b T {
    mem::transmute(t)
}

unsafe fn transmute_lifetime_mut<'a, 'b, T: ?Sized>(t: &'a mut T) -> &'b mut T {
    mem::transmute(t)
}

pub fn driver_get_version() -> c_int {
    i32::max_value()
}

impl<'a> CudaRepr for CUdeviceptr {
    type Impl = *mut c_void;
}

impl Decuda<*mut c_void> for CUdeviceptr {
    fn decuda(self) -> *mut c_void {
        self.0 as *mut _
    }
}
