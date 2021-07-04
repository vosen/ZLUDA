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
#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
pub(crate) mod os;
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
            l0_sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ENUMERATION
            | l0_sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT
            | l0_sys::ze_result_t::ZE_RESULT_ERROR_INVALID_GROUP_SIZE_DIMENSION
            | l0_sys::ze_result_t::ZE_RESULT_ERROR_INVALID_GLOBAL_WIDTH_DIMENSION => {
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
    global_heap: *mut c_void,
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

    fn lock_enqueue(
        stream: *mut stream::Stream,
        f: impl FnOnce(&l0::CommandList, &l0::Event<'static>, &[&l0::Event<'static>]) -> Result<(), CUresult>,
    ) -> Result<(), CUresult> {
        Self::lock_stream(stream, |stream_data| {
            let l0_dev = unsafe { (*(*stream_data.context).device).base };
            let l0_ctx = unsafe { &mut (*(*stream_data.context).device).l0_context };
            let event_pool = unsafe { &mut (*(*stream_data.context).device).event_pool };
            let cmd_list = unsafe { mem::transmute(stream_data.command_list()?) };
            stream_data
                .process_finished_events(&mut |(_, marker)| event_pool.mark_as_free(marker))?;
            let prev_event = stream_data.get_last_event();
            let prev_event_array = prev_event.map(|e| [e]);
            let empty = [];
            let prev_event_slice = prev_event_array.as_ref().map_or(&empty[..], |arr| &arr[..]);
            let (new_event, new_marker) = event_pool.get(l0_dev, l0_ctx)?;
            f(&cmd_list, &new_event, prev_event_slice)?;
            cmd_list.close()?;
            unsafe { stream_data.queue.execute(&cmd_list, None)? };
            stream_data.push_event((new_event, new_marker));
            Ok(())
        })?
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
    let global_heap = unsafe { os::heap_create() };
    if global_heap == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_OUT_OF_MEMORY);
    }
    *global_state = Some(GlobalState {
        devices,
        global_heap,
    });
    drop(global_state);
    Ok(())
}

macro_rules! stringify_curesult {
    ($x:ident => [ $($variant:ident),+ ]) => {
        match $x {
            $(
                CUresult::$variant => Some(concat!(stringify!($variant), "\0")),
            )+
            _ => None
        }
    }
}

pub(crate) fn get_error_string(error: CUresult, str: *mut *const i8) -> CUresult {
    if str == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let text = stringify_curesult!(
        error => [
            CUDA_SUCCESS,
            CUDA_ERROR_INVALID_VALUE,
            CUDA_ERROR_OUT_OF_MEMORY,
            CUDA_ERROR_NOT_INITIALIZED,
            CUDA_ERROR_DEINITIALIZED,
            CUDA_ERROR_PROFILER_DISABLED,
            CUDA_ERROR_PROFILER_NOT_INITIALIZED,
            CUDA_ERROR_PROFILER_ALREADY_STARTED,
            CUDA_ERROR_PROFILER_ALREADY_STOPPED,
            CUDA_ERROR_NO_DEVICE,
            CUDA_ERROR_INVALID_DEVICE,
            CUDA_ERROR_INVALID_IMAGE,
            CUDA_ERROR_INVALID_CONTEXT,
            CUDA_ERROR_CONTEXT_ALREADY_CURRENT,
            CUDA_ERROR_MAP_FAILED,
            CUDA_ERROR_UNMAP_FAILED,
            CUDA_ERROR_ARRAY_IS_MAPPED,
            CUDA_ERROR_ALREADY_MAPPED,
            CUDA_ERROR_NO_BINARY_FOR_GPU,
            CUDA_ERROR_ALREADY_ACQUIRED,
            CUDA_ERROR_NOT_MAPPED,
            CUDA_ERROR_NOT_MAPPED_AS_ARRAY,
            CUDA_ERROR_NOT_MAPPED_AS_POINTER,
            CUDA_ERROR_ECC_UNCORRECTABLE,
            CUDA_ERROR_UNSUPPORTED_LIMIT,
            CUDA_ERROR_CONTEXT_ALREADY_IN_USE,
            CUDA_ERROR_PEER_ACCESS_UNSUPPORTED,
            CUDA_ERROR_INVALID_PTX,
            CUDA_ERROR_INVALID_GRAPHICS_CONTEXT,
            CUDA_ERROR_NVLINK_UNCORRECTABLE,
            CUDA_ERROR_JIT_COMPILER_NOT_FOUND,
            CUDA_ERROR_INVALID_SOURCE,
            CUDA_ERROR_FILE_NOT_FOUND,
            CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND,
            CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
            CUDA_ERROR_OPERATING_SYSTEM,
            CUDA_ERROR_INVALID_HANDLE,
            CUDA_ERROR_ILLEGAL_STATE,
            CUDA_ERROR_NOT_FOUND,
            CUDA_ERROR_NOT_READY,
            CUDA_ERROR_ILLEGAL_ADDRESS,
            CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES,
            CUDA_ERROR_LAUNCH_TIMEOUT,
            CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING,
            CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED,
            CUDA_ERROR_PEER_ACCESS_NOT_ENABLED,
            CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE,
            CUDA_ERROR_CONTEXT_IS_DESTROYED,
            CUDA_ERROR_ASSERT,
            CUDA_ERROR_TOO_MANY_PEERS,
            CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED,
            CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED,
            CUDA_ERROR_HARDWARE_STACK_ERROR,
            CUDA_ERROR_ILLEGAL_INSTRUCTION,
            CUDA_ERROR_MISALIGNED_ADDRESS,
            CUDA_ERROR_INVALID_ADDRESS_SPACE,
            CUDA_ERROR_INVALID_PC,
            CUDA_ERROR_LAUNCH_FAILED,
            CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE,
            CUDA_ERROR_NOT_PERMITTED,
            CUDA_ERROR_NOT_SUPPORTED,
            CUDA_ERROR_SYSTEM_NOT_READY,
            CUDA_ERROR_SYSTEM_DRIVER_MISMATCH,
            CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE,
            CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED,
            CUDA_ERROR_STREAM_CAPTURE_INVALIDATED,
            CUDA_ERROR_STREAM_CAPTURE_MERGE,
            CUDA_ERROR_STREAM_CAPTURE_UNMATCHED,
            CUDA_ERROR_STREAM_CAPTURE_UNJOINED,
            CUDA_ERROR_STREAM_CAPTURE_ISOLATION,
            CUDA_ERROR_STREAM_CAPTURE_IMPLICIT,
            CUDA_ERROR_CAPTURED_EVENT,
            CUDA_ERROR_STREAM_CAPTURE_WRONG_THREAD,
            CUDA_ERROR_TIMEOUT,
            CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE,
            CUDA_ERROR_UNKNOWN
        ]
    );
    match text {
        Some(text) => {
            unsafe { *str = text.as_ptr() as *const _ };
            CUresult::CUDA_SUCCESS
        }
        None => CUresult::CUDA_ERROR_INVALID_VALUE,
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
