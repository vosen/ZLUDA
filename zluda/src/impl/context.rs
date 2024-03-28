// HIP does not implement context APIs:
// https://rocmdocs.amd.com/en/latest/Programming_Guides/HIP_API_Guide.html#hip-context-management-apis

use super::{fold_cuda_errors, module, stream, LiveCheck, ZludaObject};
use crate::hip_call_cuda;
use cuda_types::*;
use hip_runtime_sys::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::ptr;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;
use std::{cell::RefCell, ffi::c_void};

// We store device separately to avoid accessing context fields when popping
// a context from the stack. It's perfectly ok to destroy a context and remove
// it from the stack later
thread_local! {
    pub(crate) static CONTEXT_STACK: RefCell<Vec<(*mut Context, hipDevice_t)>> = RefCell::new(Vec::new());
}

pub(crate) type Context = LiveCheck<ContextData>;

impl ZludaObject for ContextData {
    #[cfg(target_pointer_width = "64")]
    const LIVENESS_COOKIE: usize = 0x5f0119560b643ffb;
    #[cfg(target_pointer_width = "32")]
    const LIVENESS_COOKIE: usize = 0x0b643ffb;
    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_CONTEXT;

    fn drop_with_result(&mut self, _: bool) -> Result<(), CUresult> {
        self.with_inner_mut(|mutable| {
            fold_cuda_errors(
                mutable
                    .streams
                    .iter()
                    .copied()
                    .map(|s| unsafe { LiveCheck::drop_box_with_result(s, true) }),
            )
        })?
    }
}

pub(crate) struct ContextData {
    pub(crate) device: hipDevice_t,
    pub(crate) variant: ContextVariant,
}

pub(crate) enum ContextVariant {
    NonPrimary(NonPrimaryContextData),
    Primary(Mutex<PrimaryContextData>),
}

pub(crate) struct PrimaryContextData {
    pub(crate) ref_count: u32,
    pub(crate) flags: u32,
    pub(crate) mutable: ContextInnerMutable,
}

pub(crate) struct NonPrimaryContextData {
    flags: AtomicU32,
    mutable: Mutex<ContextInnerMutable>,
}

impl ContextData {
    pub(crate) fn new_non_primary(flags: u32, device: hipDevice_t) -> Self {
        Self {
            device,
            variant: ContextVariant::NonPrimary(NonPrimaryContextData {
                flags: AtomicU32::new(flags),
                mutable: Mutex::new(ContextInnerMutable::new()),
            }),
        }
    }

    pub(crate) fn new_primary(device: hipDevice_t) -> Self {
        Self {
            device,
            variant: ContextVariant::Primary(Mutex::new(PrimaryContextData {
                ref_count: 0,
                flags: 0,
                mutable: ContextInnerMutable::new(),
            })),
        }
    }

    pub(crate) fn with_inner_mut<T>(
        &self,
        fn_: impl FnOnce(&mut ContextInnerMutable) -> T,
    ) -> Result<T, CUresult> {
        Ok(match self.variant {
            ContextVariant::Primary(ref mutex_over_primary_ctx_data) => {
                let mut primary_ctx_data = mutex_over_primary_ctx_data
                    .lock()
                    .map_err(|_| CUresult::CUDA_ERROR_UNKNOWN)?;
                fn_(&mut primary_ctx_data.mutable)
            }
            ContextVariant::NonPrimary(NonPrimaryContextData { ref mutable, .. }) => {
                let mut ctx_data_mutable =
                    mutable.lock().map_err(|_| CUresult::CUDA_ERROR_UNKNOWN)?;
                fn_(&mut ctx_data_mutable)
            }
        })
    }
}

pub(crate) struct ContextInnerMutable {
    pub(crate) streams: FxHashSet<*mut stream::Stream>,
    pub(crate) modules: FxHashSet<*mut module::Module>,
    // Field below is here to support CUDA Driver Dark API
    pub(crate) local_storage: FxHashMap<*mut c_void, LocalStorageValue>,
}

impl ContextInnerMutable {
    pub(crate) fn new() -> Self {
        ContextInnerMutable {
            streams: FxHashSet::default(),
            modules: FxHashSet::default(),
            local_storage: FxHashMap::default(),
        }
    }
    pub(crate) fn drop_with_result(&mut self) -> Result<(), CUresult> {
        fold_cuda_errors(
            self.streams
                .iter()
                .copied()
                .map(|s| unsafe { LiveCheck::drop_box_with_result(s, true) }),
        )
    }
}

pub(crate) struct LocalStorageValue {
    pub(crate) value: *mut c_void,
    pub(crate) _dtor_callback: Option<extern "system" fn(CUcontext, *mut c_void, *mut c_void)>,
}

pub(crate) unsafe fn create(
    pctx: *mut *mut Context,
    flags: u32,
    dev: hipDevice_t,
) -> Result<(), CUresult> {
    if pctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let context_box = Box::new(LiveCheck::new(ContextData::new_non_primary(flags, dev)));
    let context_ptr = Box::into_raw(context_box);
    *pctx = context_ptr;
    push_context_stack(context_ptr)
}

pub(crate) unsafe fn destroy(ctx: *mut Context) -> Result<(), CUresult> {
    if ctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let ctx_ref = LiveCheck::as_result(ctx)?;
    if let ContextVariant::Primary { .. } = ctx_ref.variant {
        return Err(CUresult::CUDA_ERROR_INVALID_CONTEXT);
    }
    CONTEXT_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        let should_pop = match stack.last() {
            Some((active_ctx, _)) => *active_ctx == ctx,
            None => false,
        };
        if should_pop {
            pop_context_stack_impl(&mut stack)?;
        }
        Ok(())
    })?;
    LiveCheck::drop_box_with_result(ctx, false)
}

pub(crate) unsafe fn push_current(pctx: *mut Context) -> Result<(), CUresult> {
    if pctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    push_context_stack(pctx)
}

pub(crate) unsafe fn pop_current(pctx: *mut *mut Context) -> Result<(), CUresult> {
    let mut ctx = pop_context_stack()?;
    let ctx_ptr = match &mut ctx {
        Some(ctx) => *ctx as *mut _,
        None => return Err(CUresult::CUDA_ERROR_INVALID_CONTEXT),
    };
    if pctx != ptr::null_mut() {
        *pctx = ctx_ptr;
    }
    Ok(())
}

pub(crate) unsafe fn set_current(ctx: *mut Context) -> Result<(), CUresult> {
    if ctx == ptr::null_mut() {
        pop_context_stack()?;
    } else {
        push_context_stack(ctx)?;
    }
    Ok(())
}

pub(crate) fn get_current(pctx: *mut *mut Context) -> CUresult {
    if pctx == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let ctx = get_current_from_stack().unwrap_or(ptr::null_mut());
    unsafe { *pctx = ctx };
    CUresult::CUDA_SUCCESS
}

pub fn get_device(dev: *mut hipDevice_t) -> Result<(), CUresult> {
    let dev_idx = with_current(|ctx| ctx.device)?;
    unsafe { *dev = dev_idx };
    Ok(())
}

pub(crate) fn get_limit(pvalue: *mut usize, limit: hipLimit_t) -> Result<(), CUresult> {
    hip_call_cuda! { hipDeviceGetLimit(pvalue, limit) };
    Ok(())
}

pub(crate) fn set_limit(limit: hipLimit_t, value: usize) -> Result<(), CUresult> {
    hip_call_cuda! { hipDeviceSetLimit(limit, value) };
    Ok(())
}

pub(crate) fn set_flags(flags: u32) -> Result<(), CUresult> {
    with_current(|ctx| match ctx.variant {
        ContextVariant::NonPrimary(ref context) => {
            context
                .flags
                .store(flags, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        }
        // This looks stupid, but this is an actual CUDA behavior,
        // see primary_context.rs test
        ContextVariant::Primary(_) => Ok(()),
    })?
}

pub(crate) unsafe fn get_api_version(ctx: *mut Context, version: *mut u32) -> Result<(), CUresult> {
    if ctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_CONTEXT);
    }
    //let ctx = LiveCheck::as_result(ctx)?;
    //TODO: query device for properties roughly matching CUDA API version
    *version = 3020;
    Ok(())
}

pub(crate) unsafe fn synchronize() -> Result<(), CUresult> {
    // TODO
    // We currently do this to sync with default stream which syncs whole device anyway,
    // figure out if we can do something smarter here
    hip_call_cuda!(hipDeviceSynchronize());
    Ok(())
}

pub(crate) fn with_current<T>(f: impl FnOnce(&ContextData) -> T) -> Result<T, CUresult> {
    CONTEXT_STACK.with(|stack| {
        stack
            .borrow()
            .last()
            .ok_or(CUresult::CUDA_ERROR_INVALID_CONTEXT)
            .and_then(|(ctx, _)| Ok(f(unsafe { LiveCheck::as_result(*ctx)? })))
    })
}

fn get_current_from_stack() -> Option<*mut Context> {
    CONTEXT_STACK.with(|stack| stack.borrow().last().copied().map(|(ctx, _)| ctx))
}

fn pop_context_stack() -> Result<Option<*mut Context>, CUresult> {
    CONTEXT_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        pop_context_stack_impl(&mut stack)
    })
}

fn pop_context_stack_impl(
    stack: &mut Vec<(*mut Context, hipDevice_t)>,
) -> Result<Option<*mut Context>, CUresult> {
    let ctx = stack.pop();
    if let Some((_, device)) = stack.last() {
        hip_call_cuda!(hipSetDevice(*device));
    }
    Ok(ctx.map(|(ctx, _)| ctx))
}

unsafe fn push_context_stack(ctx: *mut Context) -> Result<(), CUresult> {
    let device = { LiveCheck::as_result(ctx)?.device };
    CONTEXT_STACK.with(|stack| stack.borrow_mut().push((ctx, device)));
    hip_call_cuda!(hipSetDevice(device));
    Ok(())
}

pub(crate) unsafe fn get_stream_priority_range(
    least_priority: *mut ::std::os::raw::c_int,
    greatest_priority: *mut ::std::os::raw::c_int,
) -> Result<(), CUresult> {
    hip_call_cuda!(hipDeviceGetStreamPriorityRange(
        least_priority,
        greatest_priority
    ));
    Ok(())
}
