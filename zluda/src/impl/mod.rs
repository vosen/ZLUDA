use crate::{
    cuda::{CUctx_st, CUdevice, CUdeviceptr, CUfunc_st, CUmod_st, CUresult, CUstream_st},
    r#impl::device::Device,
};
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
pub mod context;
pub mod device;
pub mod export_table;
pub mod function;
pub mod memory;
pub mod module;
pub mod stream;

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

impl<T> From<TryLockError<T>> for CUresult {
    fn from(_: TryLockError<T>) -> Self {
        CUresult::CUDA_ERROR_ILLEGAL_STATE
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

lazy_static! {
    static ref GLOBAL_STATE: Mutex<Option<GlobalState>> = Mutex::new(None);
}

struct GlobalState {
    devices: Vec<Device>,
}

unsafe impl Send for GlobalState {}

impl GlobalState {
    fn lock<T>(f: impl FnOnce(&mut GlobalState) -> T) -> Result<T, CUresult> {
        let mut mutex = GLOBAL_STATE
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        let global_state = mutex.as_mut().ok_or(CUresult::CUDA_ERROR_ILLEGAL_STATE)?;
        Ok(f(global_state))
    }

    fn lock_device<T>(
        device::Index(dev_idx): device::Index,
        f: impl FnOnce(&'static mut device::Device) -> T,
    ) -> Result<T, CUresult> {
        if dev_idx < 0 {
            return Err(CUresult::CUDA_ERROR_INVALID_DEVICE);
        }
        Self::lock(|global_state| {
            if dev_idx >= global_state.devices.len() as c_int {
                Err(CUresult::CUDA_ERROR_INVALID_DEVICE)
            } else {
                Ok(f(unsafe {
                    transmute_lifetime_mut(&mut global_state.devices[dev_idx as usize])
                }))
            }
        })?
    }

    fn lock_current_context<F: FnOnce(&mut context::ContextData) -> R, R>(
        f: F,
    ) -> Result<R, CUresult> {
        Self::lock_current_context_unchecked(|ctx| Ok(f(ctx.as_result_mut()?)))?
    }

    fn lock_current_context_unchecked<F: FnOnce(&mut context::Context) -> R, R>(
        f: F,
    ) -> Result<R, CUresult> {
        context::CONTEXT_STACK.with(|stack| {
            stack
                .borrow_mut()
                .last_mut()
                .ok_or(CUresult::CUDA_ERROR_INVALID_CONTEXT)
                .map(|ctx| GlobalState::lock(|_| f(unsafe { &mut **ctx })))?
        })
    }

    fn lock_stream<T>(
        stream: *mut stream::Stream,
        f: impl FnOnce(&mut stream::StreamData) -> T,
    ) -> Result<T, CUresult> {
        if stream == ptr::null_mut()
            || stream == stream::CU_STREAM_LEGACY
            || stream == stream::CU_STREAM_PER_THREAD
        {
            Self::lock_current_context(|ctx| Ok(f(&mut ctx.default_stream)))?
        } else {
            Self::lock(|_| {
                let stream = unsafe { &mut *stream }.as_result_mut()?;
                Ok(f(stream))
            })?
        }
    }

    fn lock_function<T>(
        func: *mut function::Function,
        f: impl FnOnce(&mut function::FunctionData) -> T,
    ) -> Result<T, CUresult> {
        if func == ptr::null_mut() {
            return Err(CUresult::CUDA_ERROR_INVALID_HANDLE);
        }
        Self::lock(|_| {
            let func = unsafe { &mut *func }.as_result_mut()?;
            Ok(f(func))
        })?
    }
}

// TODO: implement
fn is_intel_gpu_driver(_: &l0::Driver) -> bool {
    true
}

pub fn init() -> Result<(), CUresult> {
    let mut global_state = GLOBAL_STATE
        .lock()
        .map_err(|_| CUresult::CUDA_ERROR_UNKNOWN)?;
    if global_state.is_some() {
        return Ok(());
    }
    l0::init()?;
    let drivers = l0::Driver::get()?;
    let devices = match drivers.into_iter().find(is_intel_gpu_driver) {
        None => return Err(CUresult::CUDA_ERROR_UNKNOWN),
        Some(driver) => device::init(&driver)?,
    };
    *global_state = Some(GlobalState { devices });
    drop(global_state);
    Ok(())
}

unsafe fn transmute_lifetime_mut<'a, 'b, T: ?Sized>(t: &'a mut T) -> &'b mut T {
    mem::transmute(t)
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

impl<'a> CudaRepr for CUmod_st {
    type Impl = module::Module;
}

impl<'a> CudaRepr for CUfunc_st {
    type Impl = function::Function;
}

impl<'a> CudaRepr for CUstream_st {
    type Impl = stream::Stream;
}
