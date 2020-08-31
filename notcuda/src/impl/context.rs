use super::CUresult;
use super::{device, HasLivenessCookie, LiveCheck};
use crate::{cuda::CUcontext, cuda_impl};
use l0::sys::ze_result_t;
use std::mem::{self, ManuallyDrop};
use std::{
    cell::RefCell,
    num::NonZeroU32,
    os::raw::c_uint,
    ptr,
    sync::{atomic::AtomicU32, Mutex},
};

thread_local! {
    pub static CONTEXT_STACK: RefCell<Vec<*const Context>> = RefCell::new(Vec::new());
}

pub type Context = LiveCheck<ContextData>;

impl HasLivenessCookie for ContextData {
    #[cfg(target_pointer_width = "64")]
    const COOKIE: usize = 0x5f0119560b643ffb;

    #[cfg(target_pointer_width = "32")]
    const COOKIE: usize = 0x0b643ffb;
}

enum ContextRefCount {
    Primary,
    NonPrimary(NonZeroU32),
}

impl ContextRefCount {
    fn new(is_primary: bool) -> Self {
        if is_primary {
            ContextRefCount::Primary
        } else {
            ContextRefCount::NonPrimary(unsafe { NonZeroU32::new_unchecked(1) })
        }
    }

    fn incr(&mut self) -> Result<(), CUresult> {
        match self {
            ContextRefCount::Primary => Ok(()),
            ContextRefCount::NonPrimary(c) => {
                let (new_count, overflow) = c.get().overflowing_add(1);
                if overflow {
                    Err(CUresult::CUDA_ERROR_INVALID_VALUE)
                } else {
                    *c = unsafe { NonZeroU32::new_unchecked(new_count) };
                    Ok(())
                }
            }
        }
    }

    #[must_use]
    fn decr(&mut self) -> bool {
        match self {
            ContextRefCount::Primary => false,
            ContextRefCount::NonPrimary(c) => {
                if c.get() == 1 {
                    return true;
                }
                *c = unsafe { NonZeroU32::new_unchecked(c.get() - 1) };
                false
            }
        }
    }

    fn is_primary(&self) -> bool {
        match self {
            ContextRefCount::Primary => true,
            ContextRefCount::NonPrimary(_) => false,
        }
    }
}

pub struct ContextData {
    pub flags: AtomicU32,
    pub device_index: device::Index,
    // This pointer is null only for a moment when constructing primary context
    pub device: *const Mutex<device::Device>,
    // The split between mutable / non-mutable is mainly to avoid recursive locking in cuDevicePrimaryCtxGetState
    pub mutable: Mutex<ContextDataMutable>,
}

pub struct ContextDataMutable {
    ref_count: ContextRefCount,
    pub cuda_manager: *mut cuda_impl::rt::ContextStateManager,
    pub cuda_state: *mut cuda_impl::rt::ContextState,
    pub cuda_dtor_cb: Option<
        extern "C" fn(
            CUcontext,
            *mut cuda_impl::rt::ContextStateManager,
            *mut cuda_impl::rt::ContextState,
        ),
    >,
}

impl ContextData {
    pub fn new(
        flags: c_uint,
        is_primary: bool,
        dev_index: device::Index,
        dev: *const Mutex<device::Device>,
    ) -> Self {
        ContextData {
            flags: AtomicU32::new(flags),
            device_index: dev_index,
            device: dev,
            mutable: Mutex::new(ContextDataMutable {
                ref_count: ContextRefCount::new(is_primary),
                cuda_manager: ptr::null_mut(),
                cuda_state: ptr::null_mut(),
                cuda_dtor_cb: None,
            }),
        }
    }
}

pub fn create_v2(pctx: *mut *mut Context, flags: u32, dev_idx: device::Index) -> CUresult {
    if pctx == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let dev = device::get_device_ref(dev_idx);
    let dev = match dev {
        Ok(d) => d,
        Err(e) => return e,
    };
    let mut ctx = Box::new(LiveCheck::new(ContextData::new(flags, false, dev_idx, dev)));
    let ctx_ref = ctx.as_mut() as *mut Context;
    unsafe { *pctx = ctx_ref };
    mem::forget(ctx);
    CONTEXT_STACK.with(|stack| stack.borrow_mut().push(ctx_ref));
    CUresult::CUDA_SUCCESS
}

pub fn destroy_v2(ctx: *mut Context) -> CUresult {
    if ctx == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    CONTEXT_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        let should_pop = match stack.last() {
            Some(active_ctx) => *active_ctx == (ctx as *const _),
            None => false,
        };
        if should_pop {
            stack.pop();
        }
    });
    let mut ctx_box = ManuallyDrop::new(unsafe { Box::from_raw(ctx) });
    if !ctx_box.try_drop() {
        CUresult::CUDA_ERROR_INVALID_CONTEXT
    } else {
        unsafe { ManuallyDrop::drop(&mut ctx_box) };
        CUresult::CUDA_SUCCESS
    }
}

pub fn pop_current_v2(pctx: *mut *mut Context) -> CUresult {
    if pctx == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let mut ctx = CONTEXT_STACK.with(|stack| stack.borrow_mut().pop());
    let ctx_ptr = match &mut ctx {
        Some(ctx) => *ctx as *mut _,
        None => return CUresult::CUDA_ERROR_INVALID_CONTEXT,
    };
    unsafe { *pctx = ctx_ptr };
    CUresult::CUDA_SUCCESS
}

pub fn with_current<F: FnOnce(&ContextData) -> R, R>(f: F) -> Result<R, CUresult> {
    CONTEXT_STACK.with(|stack| {
        stack
            .borrow()
            .last()
            .and_then(|c| unsafe { &**c }.as_ref())
            .ok_or(CUresult::CUDA_ERROR_INVALID_CONTEXT)
            .map(f)
    })
}

pub fn get_current(pctx: *mut *mut Context) -> l0::Result<()> {
    if pctx == ptr::null_mut() {
        return Err(ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT);
    }
    let ctx = CONTEXT_STACK.with(|stack| match stack.borrow().last() {
        Some(ctx) => *ctx as *mut _,
        None => ptr::null_mut(),
    });
    unsafe { *pctx = ctx };
    Ok(())
}

pub fn set_current(ctx: *mut Context) -> CUresult {
    if ctx == ptr::null_mut() {
        CONTEXT_STACK.with(|stack| stack.borrow_mut().pop());
        CUresult::CUDA_SUCCESS
    } else {
        CONTEXT_STACK.with(|stack| stack.borrow_mut().push(ctx));
        CUresult::CUDA_SUCCESS
    }
}

pub fn get_api_version(ctx: *mut Context, version: *mut u32) -> CUresult {
    let _ctx = match unsafe { ctx.as_mut() } {
        None => return CUresult::CUDA_ERROR_INVALID_VALUE,
        Some(ctx) => match ctx.as_mut() {
            None => return CUresult::CUDA_ERROR_INVALID_CONTEXT,
            Some(ctx) => ctx,
        },
    };
    //TODO: query device for properties roughly matching CUDA API version
    unsafe { *version = 1100 };
    CUresult::CUDA_SUCCESS
}

pub fn get_device(dev: *mut device::Index) -> CUresult {
    let dev_idx = with_current(|ctx| ctx.device_index);
    match dev_idx {
        Ok(idx) => {
            unsafe { *dev = idx }
            CUresult::CUDA_SUCCESS
        }
        Err(err) => err,
    }
}

#[cfg(test)]
pub fn is_context_stack_empty() -> bool {
    CONTEXT_STACK.with(|stack| stack.borrow().is_empty())
}

#[cfg(test)]
mod tests {
    use super::super::test::CudaDriverFns;
    use super::super::CUresult;
    use std::{ffi::c_void, ptr};

    cuda_driver_test!(destroy_leaves_zombie_context);

    fn destroy_leaves_zombie_context<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx1 = ptr::null_mut();
        let mut ctx2 = ptr::null_mut();
        let mut ctx3 = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx1, 0, 0), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuCtxCreate_v2(&mut ctx2, 0, 0), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuCtxCreate_v2(&mut ctx3, 0, 0), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuCtxDestroy_v2(ctx2), CUresult::CUDA_SUCCESS);
        let mut popped_ctx1 = ptr::null_mut();
        assert_eq!(
            T::cuCtxPopCurrent_v2(&mut popped_ctx1),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(popped_ctx1, ctx3);
        let mut popped_ctx2 = ptr::null_mut();
        assert_eq!(
            T::cuCtxPopCurrent_v2(&mut popped_ctx2),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(popped_ctx2, ctx2);
        let mut popped_ctx3 = ptr::null_mut();
        assert_eq!(
            T::cuCtxPopCurrent_v2(&mut popped_ctx3),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(popped_ctx3, ctx1);
        let mut temp = 0;
        assert_eq!(
            T::cuCtxGetApiVersion(ctx2, &mut temp),
            CUresult::CUDA_ERROR_INVALID_CONTEXT
        );
        assert_eq!(
            T::cuCtxPopCurrent_v2(&mut ptr::null_mut()),
            CUresult::CUDA_ERROR_INVALID_CONTEXT
        );
    }

    cuda_driver_test!(empty_pop_fails);

    fn empty_pop_fails<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx = ptr::null_mut();
        assert_eq!(
            T::cuCtxPopCurrent_v2(&mut ctx),
            CUresult::CUDA_ERROR_INVALID_CONTEXT
        );
    }

    cuda_driver_test!(destroy_pops_top_of_stack);

    fn destroy_pops_top_of_stack<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx1 = ptr::null_mut();
        let mut ctx2 = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx1, 0, 0), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuCtxCreate_v2(&mut ctx2, 0, 0), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuCtxDestroy_v2(ctx2), CUresult::CUDA_SUCCESS);
        let mut popped_ctx1 = ptr::null_mut();
        assert_eq!(
            T::cuCtxPopCurrent_v2(&mut popped_ctx1),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(popped_ctx1, ctx1);
        let mut popped_ctx2 = ptr::null_mut();
        assert_eq!(
            T::cuCtxPopCurrent_v2(&mut popped_ctx2),
            CUresult::CUDA_ERROR_INVALID_CONTEXT
        );
    }

    cuda_driver_test!(double_destroy_fails);

    fn double_destroy_fails<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx, 0, 0), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
        let destroy_result = T::cuCtxDestroy_v2(ctx);
        // original CUDA impl returns randomly one or the other
        assert!(
            destroy_result == CUresult::CUDA_ERROR_INVALID_CONTEXT
                || destroy_result == CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
        );
    }

    cuda_driver_test!(no_current_on_init);

    fn no_current_on_init<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx = 1 as *mut c_void;
        assert_eq!(T::cuCtxGetCurrent(&mut ctx), CUresult::CUDA_SUCCESS);
        assert_eq!(ctx, ptr::null_mut());
    }
}
