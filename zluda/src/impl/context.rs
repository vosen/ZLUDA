use super::{module, FromCuda, ZludaObject};
use cuda_types::cuda::*;
use hip_runtime_sys::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{cell::RefCell, ffi::c_void, ptr, sync::Mutex};

thread_local! {
    pub(crate) static STACK: RefCell<Vec<(CUcontext, hipDevice_t)>> = RefCell::new(Vec::new());
}

pub(crate) struct Context {
    pub(crate) device: hipDevice_t,
    pub(crate) state: Mutex<ContextState>,
}

pub(crate) struct ContextState {
    pub(crate) ref_count: u32,
    pub(crate) flags: u32,
    pub(crate) modules: FxHashSet<CUmodule>,
    pub(crate) storage: FxHashMap<usize, StorageData>,
}

pub(crate) struct StorageData {
    pub(crate) value: usize,
    pub(crate) reset_cb: Option<extern "system" fn(CUcontext, *mut c_void, *mut c_void)>,
    pub(crate) handle: CUcontext,
}

impl ContextState {
    pub(crate) fn new() -> Self {
        ContextState {
            ref_count: 0,
            flags: 0,
            modules: FxHashSet::default(),
            storage: FxHashMap::default(),
        }
    }

    pub(crate) fn reset(&mut self) -> CUresult {
        for (key, data) in self.storage.iter_mut() {
            if let Some(_cb) = data.reset_cb {
                // TODO: check that these callbacks do not call into the CUDA driver
                // since this could result in a recursive mutex lock.
                _cb(data.handle, *key as *mut c_void, data.value as *mut c_void);
            }
        }
        self.ref_count = 0;
        self.flags = 0;
        // drop all modules and return first error if any
        let result = self.modules.drain().fold(Ok(()), |res: CUresult, hmod| {
            match (res, super::drop_checked::<module::Module>(hmod)) {
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
                _ => Ok(()),
            }
        });
        self.storage.clear();
        result
    }
}

impl Context {
    pub(crate) fn new(device: hipDevice_t) -> Self {
        Self {
            device: device,
            state: Mutex::new(ContextState::new()),
        }
    }

    pub(crate) fn with_state(&self, fn_: impl FnOnce(&ContextState) -> CUresult) -> CUresult {
        match self.state.lock() {
            Ok(guard) => fn_(&*guard),
            Err(_) => CUresult::ERROR_UNKNOWN,
        }
    }

    pub(crate) fn with_state_mut(
        &self,
        fn_: impl FnOnce(&mut ContextState) -> CUresult,
    ) -> CUresult {
        match self.state.lock() {
            Ok(mut guard) => fn_(&mut *guard),
            Err(_) => CUresult::ERROR_UNKNOWN,
        }
    }
}

impl ZludaObject for Context {
    const COOKIE: usize = 0x5f867c6d9cb73315;

    type CudaHandle = CUcontext;

    fn drop_checked(&mut self) -> CUresult {
        Ok(())
    }
}

pub(crate) fn get_current_context() -> Result<CUcontext, CUerror> {
    if let Some(ctx) = STACK.with(|stack| stack.borrow().last().copied().map(|(ctx, _)| ctx)) {
        return Ok(ctx);
    }
    Err(CUerror::INVALID_CONTEXT)
}

pub(crate) unsafe fn get_limit(pvalue: *mut usize, limit: hipLimit_t) -> hipError_t {
    unsafe { hipDeviceGetLimit(pvalue, limit) }
}

pub(crate) fn set_limit(limit: hipLimit_t, value: usize) -> hipError_t {
    unsafe { hipDeviceSetLimit(limit, value) }
}

pub(crate) fn synchronize() -> hipError_t {
    unsafe { hipDeviceSynchronize() }
}

pub(crate) fn set_current(raw_ctx: CUcontext) -> CUresult {
    let new_device = if raw_ctx.0 == ptr::null_mut() {
        STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            if let Some((_, old_device)) = stack.pop() {
                if let Some((_, new_device)) = stack.last() {
                    if old_device != *new_device {
                        return Some(*new_device);
                    }
                }
            }
            None
        })
    } else {
        let ctx: &Context = FromCuda::from_cuda(&raw_ctx)?;
        let device = ctx.device;
        STACK.with(move |stack| {
            let mut stack = stack.borrow_mut();
            let last_device = stack.last().map(|(_, dev)| *dev);
            stack.push((raw_ctx, device));
            match last_device {
                None => Some(device),
                Some(last_device) if last_device != device => Some(device),
                _ => None,
            }
        })
    };
    if let Some(dev) = new_device {
        unsafe { hipSetDevice(dev)? };
    }
    Ok(())
}

pub(crate) fn get_current(pctx: &mut CUcontext) -> CUresult {
    match get_current_context() {
        Ok(ctx) => *pctx = ctx,
        Err(_) => *pctx = CUcontext(ptr::null_mut()),
    }
    CUresult::SUCCESS
}

pub(crate) fn get_device(dev: &mut hipDevice_t) -> CUresult {
    let cu_ctx = get_current_context()?;
    let ctx: &Context = FromCuda::from_cuda(&cu_ctx)?;
    *dev = ctx.device;
    Ok(())
}

pub(crate) unsafe fn push_current(ctx: CUcontext) -> CUresult {
    if ctx == CUcontext(ptr::null_mut()) {
        return CUresult::ERROR_INVALID_VALUE;
    }
    set_current(ctx)
}

pub(crate) unsafe fn push_current_v2(ctx: CUcontext) -> CUresult {
    push_current(ctx)
}

pub(crate) unsafe fn pop_current(ctx: &mut CUcontext) -> CUresult {
    STACK.with(|stack| {
        if let Some((_ctx, _)) = stack.borrow_mut().pop() {
            *ctx = _ctx;
        }
    });
    Ok(())
}

pub(crate) unsafe fn create_v2(
    ctx: &mut CUcontext,
    _flags: ::core::ffi::c_uint,
    dev: cuda_types::cuda::CUdevice,
) -> CUresult {
    let handle = Context::wrap(Context::new(dev));
    // TODO: optimize
    set_current(handle)?;
    *ctx = handle;
    Ok(())
}

pub(crate) unsafe fn destroy_v2(ctx: CUcontext) -> CUresult {
    super::drop_checked::<Context>(ctx)
}

pub(crate) unsafe fn pop_current_v2(ctx: &mut CUcontext) -> CUresult {
    pop_current(ctx)
}
