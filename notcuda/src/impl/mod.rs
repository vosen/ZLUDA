use crate::cuda::{CUctx_st, CUdevice, CUdeviceptr, CUresult, CUmodule};
use std::{ffi::c_void, mem::ManuallyDrop, os::raw::c_int, sync::Mutex};

#[cfg(test)]
#[macro_use]
pub mod test;
pub mod context;
pub mod device;
pub mod export_table;
pub mod memory;
pub mod module;

#[cfg(debug_assertions)]
pub fn unimplemented() -> CUresult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub fn unimplemented() -> CUresult {
    CUresult::CUDA_ERROR_NOT_SUPPORTED
}

pub trait HasLivenessCookie {
    const COOKIE: usize;
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

    pub unsafe fn as_ref_unchecked(&self) -> &T {
        &self.data
    }

    pub fn as_ref(&self) -> Option<&T> {
        if self.cookie == T::COOKIE {
            Some(&self.data)
        } else {
            None
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.cookie == T::COOKIE {
            Some(&mut self.data)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_drop(&mut self) -> bool {
        if self.cookie == T::COOKIE {
            self.cookie = 0;
            unsafe { ManuallyDrop::drop(&mut self.data) };
            return true;
        }
        false
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

impl From<l0::sys::ze_result_t> for CUresult {
    fn from(result: l0::sys::ze_result_t) -> Self {
        match result {
            l0::sys::ze_result_t::ZE_RESULT_SUCCESS => CUresult::CUDA_SUCCESS,
            l0_sys::ze_result_t::ZE_RESULT_ERROR_UNINITIALIZED => {
                CUresult::CUDA_ERROR_NOT_INITIALIZED
            }
            l0_sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ENUMERATION => {
                CUresult::CUDA_ERROR_INVALID_VALUE
            }
            l0_sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT => {
                CUresult::CUDA_ERROR_INVALID_VALUE
            }
            l0_sys::ze_result_t::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY => {
                CUresult::CUDA_ERROR_OUT_OF_MEMORY
            }
            l0_sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_FEATURE => {
                CUresult::CUDA_ERROR_NOT_SUPPORTED
            }
            _ => CUresult::CUDA_ERROR_UNKNOWN,
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

impl Encuda for l0::sys::ze_result_t {
    type To = CUresult;
    fn encuda(self: Self) -> Self::To {
        self.into()
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

pub enum Error {
    L0(l0::sys::ze_result_t),
    Cuda(CUresult),
}

impl Encuda for Error {
    type To = CUresult;
    fn encuda(self: Self) -> Self::To {
        match self {
            Error::L0(e) => e.into(),
            Error::Cuda(e) => e,
        }
    }
}

lazy_static! {
    static ref GLOBAL_STATE: Mutex<Option<GlobalState>> = Mutex::new(None);
}

struct GlobalState {
    driver: l0::Driver,
}

unsafe impl Send for GlobalState {}

// TODO: implement
fn is_intel_gpu_driver(_: &l0::Driver) -> bool {
    true
}

pub fn init() -> l0::Result<()> {
    let mut global_state = GLOBAL_STATE
        .lock()
        .map_err(|_| l0::sys::ze_result_t::ZE_RESULT_ERROR_UNKNOWN)?;
    if global_state.is_some() {
        return Ok(());
    }
    l0::init()?;
    let drivers = l0::Driver::get()?;
    let driver = match drivers.into_iter().find(is_intel_gpu_driver) {
        None => return Err(l0::sys::ze_result_t::ZE_RESULT_ERROR_UNKNOWN),
        Some(driver) => {
            device::init(&driver)?;
            driver
        }
    };
    *global_state = Some(GlobalState { driver });
    drop(global_state);
    Ok(())
}

pub fn driver_get_version() -> c_int {
    i32::max_value()
}

impl<'a> CudaRepr for CUctx_st {
    type Impl = context::Context;
}

impl<'a> CudaRepr for CUdevice {
    type Impl = device::Index;
}

impl Decuda<device::Index> for CUdevice {
    fn decuda(self) -> device::Index {
        device::Index(self.0)
    }
}

impl<'a> CudaRepr for CUdeviceptr {
    type Impl = *mut c_void;
}

impl Decuda<*mut c_void> for CUdeviceptr {
    fn decuda(self) -> *mut c_void {
        self.0 as *mut _
    }
}

impl<'a> CudaRepr for CUmodule {
    type Impl = *mut module::Module;
}

