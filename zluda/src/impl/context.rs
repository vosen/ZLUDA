use super::{device, stream::Stream, stream::StreamData, HasLivenessCookie, LiveCheck};
use super::{CUresult, GlobalState};
use crate::{cuda::CUcontext, cuda_impl};
use l0::sys::ze_result_t;
use std::{cell::RefCell, num::NonZeroU32, os::raw::c_uint, ptr, sync::atomic::AtomicU32};
use std::{
    collections::HashSet,
    mem::{self},
};

thread_local! {
    pub static CONTEXT_STACK: RefCell<Vec<*mut Context>> = RefCell::new(Vec::new());
}

pub type Context = LiveCheck<ContextData>;

impl HasLivenessCookie for ContextData {
    #[cfg(target_pointer_width = "64")]
    const COOKIE: usize = 0x5f0119560b643ffb;

    #[cfg(target_pointer_width = "32")]
    const COOKIE: usize = 0x0b643ffb;

    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_CONTEXT;

    fn try_drop(&mut self) -> Result<(), CUresult> {
        for stream in self.streams.iter() {
            let stream = unsafe { &mut **stream };
            stream.context = ptr::null_mut();
            Stream::destroy_impl(unsafe { Stream::ptr_from_inner(stream) })?;
        }
        Ok(())
    }
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
}

pub struct ContextData {
    pub flags: AtomicU32,
    // This pointer is null only for a moment when constructing primary context
    pub device: *mut device::Device,
    ref_count: ContextRefCount,
    pub default_stream: StreamData,
    pub streams: HashSet<*mut StreamData>,
    // All the fields below are here to support internal CUDA driver API
    pub cuda_manager: *mut cuda_impl::rt::ContextStateManager,
    pub cuda_state: *mut cuda_impl::rt::ContextState,
    pub cuda_dtor_cb: Option<
        extern "system" fn(
            CUcontext,
            *mut cuda_impl::rt::ContextStateManager,
            *mut cuda_impl::rt::ContextState,
        ),
    >,
}

impl ContextData {
    pub fn new(
        l0_ctx: &'static l0::Context,
        l0_dev: l0::Device,
        flags: c_uint,
        is_primary: bool,
        dev: *mut device::Device,
    ) -> Result<Self, CUresult> {
        let default_stream = StreamData::new_unitialized(l0_ctx, l0_dev)?;
        Ok(ContextData {
            flags: AtomicU32::new(flags),
            device: dev,
            ref_count: ContextRefCount::new(is_primary),
            default_stream,
            streams: HashSet::new(),
            cuda_manager: ptr::null_mut(),
            cuda_state: ptr::null_mut(),
            cuda_dtor_cb: None,
        })
    }
}

impl Context {
    pub fn late_init(&mut self) {
        let ctx_data = self.as_option_mut().unwrap();
        ctx_data.default_stream.context = ctx_data as *mut _;
    }
}

pub fn create_v2(
    pctx: *mut *mut Context,
    flags: u32,
    dev_idx: device::Index,
) -> Result<(), CUresult> {
    if pctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let mut ctx_box = GlobalState::lock_device(dev_idx, |dev| {
        let dev_ptr = dev as *mut _;
        let mut ctx_box = Box::new(LiveCheck::new(ContextData::new(
            &mut dev.l0_context,
            dev.base,
            flags,
            false,
            dev_ptr as *mut _,
        )?));
        ctx_box.late_init();
        Ok::<_, CUresult>(ctx_box)
    })??;
    let ctx_ref = ctx_box.as_mut() as *mut Context;
    unsafe { *pctx = ctx_ref };
    mem::forget(ctx_box);
    CONTEXT_STACK.with(|stack| stack.borrow_mut().push(ctx_ref));
    Ok(())
}

pub fn destroy_v2(ctx: *mut Context) -> Result<(), CUresult> {
    if ctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    CONTEXT_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        let should_pop = match stack.last() {
            Some(active_ctx) => *active_ctx == (ctx as *mut _),
            None => false,
        };
        if should_pop {
            stack.pop();
        }
    });
    GlobalState::lock(|_| Context::destroy_impl(ctx))?
}

pub(crate) fn push_current_v2(pctx: *mut Context) -> CUresult {
    if pctx == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    CONTEXT_STACK.with(|stack| stack.borrow_mut().push(pctx));
    CUresult::CUDA_SUCCESS
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

pub fn get_api_version(ctx: *mut Context, version: *mut u32) -> Result<(), CUresult> {
    if ctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    GlobalState::lock(|_| {
        unsafe { &*ctx }.as_result()?;
        Ok::<_, CUresult>(())
    })??;
    //TODO: query device for properties roughly matching CUDA API version
    unsafe { *version = 1100 };
    Ok(())
}

pub fn get_device(dev: *mut device::Index) -> Result<(), CUresult> {
    let dev_idx = GlobalState::lock_current_context(|ctx| unsafe { &*ctx.device }.index)?;
    unsafe { *dev = dev_idx };
    Ok(())
}

pub fn attach(pctx: *mut *mut Context, _flags: c_uint) -> Result<(), CUresult> {
    if pctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let ctx = GlobalState::lock_current_context_unchecked(|unchecked_ctx| {
        let ctx = unchecked_ctx.as_result_mut()?;
        ctx.ref_count.incr()?;
        Ok::<_, CUresult>(unchecked_ctx as *mut _)
    })??;
    unsafe { *pctx = ctx };
    Ok(())
}

pub fn detach(pctx: *mut Context) -> Result<(), CUresult> {
    if pctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    GlobalState::lock_current_context_unchecked(|unchecked_ctx| {
        let ctx = unchecked_ctx.as_result_mut()?;
        if ctx.ref_count.decr() {
            Context::destroy_impl(unchecked_ctx)?;
        }
        Ok::<_, CUresult>(())
    })?
}

pub(crate) fn synchronize() -> CUresult {
    // TODO: change the implementation once we do async stream operations
    CUresult::CUDA_SUCCESS
}

#[cfg(test)]
mod test {
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
