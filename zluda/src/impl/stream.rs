use super::{context, LiveCheck, ZludaObject};
use crate::{hip_call_cuda, r#impl::hipfix};
use cuda_types::{CUhostFn, CUresult};
use hip_runtime_sys::*;
use std::{ffi::c_void, ptr};

pub(crate) const CU_STREAM_NULL: *mut Stream = 0 as *mut _;
pub(crate) const CU_STREAM_LEGACY: *mut Stream = 1 as *mut _;
pub(crate) const CU_STREAM_PER_THREAD: *mut Stream = 2 as *mut _;

pub(crate) type Stream = LiveCheck<StreamData>;

impl ZludaObject for StreamData {
    #[cfg(target_pointer_width = "64")]
    const LIVENESS_COOKIE: usize = 0x512097354de18d35;
    #[cfg(target_pointer_width = "32")]
    const LIVENESS_COOKIE: usize = 0x77d5cc0b;
    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_HANDLE;

    fn drop_with_result(&mut self, by_owner: bool) -> Result<(), CUresult> {
        if !by_owner {
            let ctx = unsafe { LiveCheck::as_result(self.ctx)? };
            {
                ctx.with_inner_mut(|ctx_mutable| {
                    ctx_mutable
                        .streams
                        .remove(&unsafe { LiveCheck::from_raw(&mut *self) });
                })?;
            }
        }
        hip_call_cuda!(hipStreamDestroy(self.base));
        Ok(())
    }
}

pub(crate) struct StreamData {
    pub(crate) base: hipStream_t,
    pub(crate) ctx: *mut context::Context,
}

pub(crate) unsafe fn create_with_priority(
    p_stream: *mut *mut Stream,
    flags: ::std::os::raw::c_uint,
    priority: ::std::os::raw::c_int,
) -> Result<(), CUresult> {
    if p_stream == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let mut hip_stream = ptr::null_mut();
    hip_call_cuda!(hipStreamCreateWithPriority(
        &mut hip_stream,
        flags,
        priority
    ));
    let stream = Box::into_raw(Box::new(LiveCheck::new(StreamData {
        base: hip_stream,
        ctx: ptr::null_mut(),
    })));
    let ctx = context::with_current(|ctx| {
        ctx.with_inner_mut(|ctx_mutable| {
            ctx_mutable.streams.insert(stream);
        })?;
        Ok(LiveCheck::from_raw(ctx as *const _ as _))
    })??;
    (*stream).as_mut_unchecked().ctx = ctx;
    *p_stream = stream;
    Ok(())
}

pub(crate) unsafe fn get_ctx(
    stream: *mut Stream,
    pctx: *mut *mut context::Context,
) -> Result<(), CUresult> {
    let ctx = if as_default_stream(stream).is_some() {
        context::with_current(|ctx| LiveCheck::from_raw(ctx as *const _ as _))?
    } else {
        let stream = LiveCheck::as_result(stream)?;
        stream.ctx
    };
    *pctx = ctx;
    Ok(())
}

pub(crate) unsafe fn synchronize(
    stream: *mut Stream,
    default_stream_per_thread: bool,
) -> Result<(), CUresult> {
    let hip_stream = hipfix::as_hip_stream_per_thread(stream, default_stream_per_thread)?;
    hip_call_cuda!(hipStreamSynchronize(hip_stream));
    Ok(())
}

pub(crate) unsafe fn destroy(stream: *mut Stream) -> Result<(), CUresult> {
    if as_default_stream(stream).is_some() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    LiveCheck::drop_box_with_result(stream, false)
}

pub(crate) fn as_default_stream(stream: *mut Stream) -> Option<hipStream_t> {
    match stream {
        CU_STREAM_NULL | CU_STREAM_LEGACY => Some(hipStreamNull),
        CU_STREAM_PER_THREAD => Some(hipStreamPerThread),
        _ => None,
    }
}

pub(crate) unsafe fn as_hip_stream(stream: *mut Stream) -> Result<hipStream_t, CUresult> {
    Ok(match as_default_stream(stream) {
        Some(s) => s,
        None => LiveCheck::as_result(stream)?.base,
    })
}

pub(crate) unsafe fn launch_host_func(
    stream: *mut Stream,
    fn_: CUhostFn,
    user_data: *mut ::std::os::raw::c_void,
) -> Result<(), CUresult> {
    let fn_ = *fn_.as_ref().ok_or(CUresult::CUDA_ERROR_INVALID_VALUE)?;
    let hip_stream = as_hip_stream(stream)?;
    // TODO: use hipLaunchHostFunc when it comes to Windows
    //hip_call_cuda!(hipLaunchHostFunc(hip_stream, fn_, user_data));
    let callback = Box::new(HostCallback { fn_, user_data });
    hip_call_cuda!(hipStreamAddCallback(
        hip_stream,
        Some(steam_callback_to_host_func),
        Box::into_raw(callback) as _,
        0
    ));
    Ok(())
}

pub(crate) unsafe fn wait_event(
    stream: *mut Stream,
    h_event: hipEvent_t,
    flags: ::std::os::raw::c_uint,
    default_stream_per_thread: bool,
) -> Result<(), CUresult> {
    let hip_stream = hipfix::as_hip_stream_per_thread(stream, default_stream_per_thread)?;
    hip_call_cuda! { hipStreamWaitEvent(hip_stream, h_event, flags) };
    Ok(())
}

unsafe extern "C" fn steam_callback_to_host_func(
    _stream: hipStream_t,
    result: hipError_t,
    callback_ptr: *mut c_void,
) {
    if result != hipError_t::hipSuccess {
        return;
    }
    let callback_ptr = &*(callback_ptr as *const HostCallback);
    (callback_ptr.fn_)(callback_ptr.user_data);
}

struct HostCallback {
    fn_: unsafe extern "system" fn(userData: *mut ::std::os::raw::c_void),
    user_data: *mut ::std::os::raw::c_void,
}

pub(crate) unsafe fn query(stream: *mut Stream) -> Result<(), CUresult> {
    let hip_stream = as_hip_stream(stream)?;
    hip_call_cuda! { hipStreamQuery(hip_stream) };
    Ok(())
}

pub(crate) unsafe fn get_capture_info(
    stream: *mut Stream,
    capture_status_out: *mut hipStreamCaptureStatus,
    id_out: *mut u64,
) -> Result<(), CUresult> {
    let hip_stream = as_hip_stream(stream)?;
    hip_call_cuda! { hipStreamGetCaptureInfo(hip_stream, capture_status_out, id_out) };
    Ok(())
}

pub(crate) unsafe fn get_flags(stream: *mut Stream, flags: *mut u32) -> Result<(), CUresult> {
    let hip_stream = as_hip_stream(stream)?;
    hip_call_cuda! { hipStreamGetFlags(hip_stream, flags) };
    Ok(())
}

pub(crate) unsafe fn is_capturing(
    stream: *mut Stream,
    capture_status: *mut hipStreamCaptureStatus,
) -> Result<(), CUresult> {
    let hip_stream = as_hip_stream(stream)?;
    hip_call_cuda! { hipStreamIsCapturing(hip_stream, capture_status) };
    Ok(())
}
