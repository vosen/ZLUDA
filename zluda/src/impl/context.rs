use super::{driver, FromCuda, ZludaObject};
use cuda_types::cuda::*;
use hip_runtime_sys::*;
use rustc_hash::FxHashSet;
use std::{cell::RefCell, ptr, sync::Mutex};

thread_local! {
    pub(crate) static CONTEXT_STACK: RefCell<Vec<(CUcontext, hipDevice_t)>> = RefCell::new(Vec::new());
}

pub(crate) struct Context {
    pub(crate) device: hipDevice_t,
    pub(crate) mutable: Mutex<OwnedByContext>,
}

pub(crate) struct OwnedByContext {
    pub(crate) ref_count: usize, // only used by primary context
    pub(crate) _memory: FxHashSet<hipDeviceptr_t>,
    pub(crate) _streams: FxHashSet<hipStream_t>,
    pub(crate) _modules: FxHashSet<CUmodule>,
}

impl ZludaObject for Context {
    const COOKIE: usize = 0x5f867c6d9cb73315;

    type CudaHandle = CUcontext;

    fn drop_checked(&mut self) -> CUresult {
        Ok(())
    }
}

pub(crate) fn new(device: hipDevice_t) -> Context {
    Context {
        device,
        mutable: Mutex::new(OwnedByContext {
            ref_count: 0,
            _memory: FxHashSet::default(),
            _streams: FxHashSet::default(),
            _modules: FxHashSet::default(),
        }),
    }
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

pub(crate) fn get_primary(hip_dev: hipDevice_t) -> Result<(&'static Context, CUcontext), CUerror> {
    let dev = driver::device(hip_dev)?;
    Ok(dev.primary_context())
}

pub(crate) fn set_current(raw_ctx: CUcontext) -> CUresult {
    let new_device = if raw_ctx.0 == ptr::null_mut() {
        CONTEXT_STACK.with(|stack| {
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
        CONTEXT_STACK.with(move |stack| {
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
