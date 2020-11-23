use super::{
    context::{Context, ContextData},
    CUresult, GlobalState,
};
use std::{mem, ptr};

use super::{HasLivenessCookie, LiveCheck};

pub type Stream = LiveCheck<StreamData>;

pub const CU_STREAM_LEGACY: *mut Stream = 1 as *mut _;
pub const CU_STREAM_PER_THREAD: *mut Stream = 2 as *mut _;

impl HasLivenessCookie for StreamData {
    #[cfg(target_pointer_width = "64")]
    const COOKIE: usize = 0x512097354de18d35;

    #[cfg(target_pointer_width = "32")]
    const COOKIE: usize = 0x77d5cc0b;

    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_HANDLE;

    fn try_drop(&mut self) -> Result<(), CUresult> {
        if self.context != ptr::null_mut() {
            let context = unsafe { &mut *self.context };
            if !context.streams.remove(&(self as *mut _)) {
                return Err(CUresult::CUDA_ERROR_UNKNOWN);
            }
        }
        Ok(())
    }
}

pub struct StreamData {
    pub context: *mut ContextData,
    pub queue: l0::CommandQueue,
}

impl StreamData {
    pub fn new_unitialized(ctx: &mut l0::Context, dev: &l0::Device) -> Result<Self, CUresult> {
        Ok(StreamData {
            context: ptr::null_mut(),
            queue: l0::CommandQueue::new(ctx, dev)?,
        })
    }
    pub fn new(ctx: &mut ContextData) -> Result<Self, CUresult> {
        let l0_ctx = &mut unsafe { &mut *ctx.device }.l0_context;
        let l0_dev = &unsafe { &*ctx.device }.base;
        Ok(StreamData {
            context: ctx as *mut _,
            queue: l0::CommandQueue::new(l0_ctx, l0_dev)?,
        })
    }

    pub fn command_list(&self) -> Result<l0::CommandList, l0::sys::_ze_result_t> {
        let ctx = unsafe { &mut *self.context };
        let dev = unsafe { &mut *ctx.device };
        l0::CommandList::new(&mut dev.l0_context, &dev.base)
    }
}

impl Drop for StreamData {
    fn drop(&mut self) {
        if self.context == ptr::null_mut() {
            return;
        }
        unsafe { (&mut *self.context).streams.remove(&(&mut *self as *mut _)) };
    }
}

pub(crate) fn get_ctx(hstream: *mut Stream, pctx: *mut *mut Context) -> Result<(), CUresult> {
    if pctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let ctx_ptr = GlobalState::lock_stream(hstream, |stream| stream.context)?;
    if ctx_ptr == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED);
    }
    unsafe { *pctx = Context::ptr_from_inner(ctx_ptr) };
    Ok(())
}

pub(crate) fn create(phstream: *mut *mut Stream, _flags: u32) -> Result<(), CUresult> {
    let stream_ptr = GlobalState::lock_current_context(|ctx| {
        let mut stream_box = Box::new(Stream::new(StreamData::new(ctx)?));
        let stream_ptr = stream_box.as_mut().as_option_mut().unwrap() as *mut _;
        if !ctx.streams.insert(stream_ptr) {
            return Err(CUresult::CUDA_ERROR_UNKNOWN);
        }
        mem::forget(stream_box);
        Ok::<_, CUresult>(stream_ptr)
    })??;
    unsafe { *phstream = Stream::ptr_from_inner(stream_ptr) };
    Ok(())
}

pub(crate) fn destroy_v2(pstream: *mut Stream) -> Result<(), CUresult> {
    if pstream == ptr::null_mut() || pstream == CU_STREAM_LEGACY || pstream == CU_STREAM_PER_THREAD
    {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    GlobalState::lock(|_| Stream::destroy_impl(pstream))?
}

#[cfg(test)]
mod test {
    use crate::cuda::CUstream;

    use super::super::test::CudaDriverFns;
    use super::super::CUresult;
    use std::{ptr, thread};

    const CU_STREAM_LEGACY: CUstream = 1 as *mut _;
    const CU_STREAM_PER_THREAD: CUstream = 2 as *mut _;

    cuda_driver_test!(default_stream_uses_current_ctx_legacy);
    cuda_driver_test!(default_stream_uses_current_ctx_ptsd);

    fn default_stream_uses_current_ctx_legacy<T: CudaDriverFns>() {
        default_stream_uses_current_ctx_impl::<T>(CU_STREAM_LEGACY);
    }

    fn default_stream_uses_current_ctx_ptsd<T: CudaDriverFns>() {
        default_stream_uses_current_ctx_impl::<T>(CU_STREAM_PER_THREAD);
    }

    fn default_stream_uses_current_ctx_impl<T: CudaDriverFns>(stream: CUstream) {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx1 = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx1, 0, 0), CUresult::CUDA_SUCCESS);
        let mut stream_ctx1 = ptr::null_mut();
        assert_eq!(
            T::cuStreamGetCtx(stream, &mut stream_ctx1),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(ctx1, stream_ctx1);
        let mut ctx2 = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx2, 0, 0), CUresult::CUDA_SUCCESS);
        assert_ne!(ctx1, ctx2);
        let mut stream_ctx2 = ptr::null_mut();
        assert_eq!(
            T::cuStreamGetCtx(stream, &mut stream_ctx2),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(ctx2, stream_ctx2);
        //  Cleanup
        assert_eq!(T::cuCtxDestroy_v2(ctx1), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuCtxDestroy_v2(ctx2), CUresult::CUDA_SUCCESS);
    }

    cuda_driver_test!(stream_context_destroyed);

    fn stream_context_destroyed<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx, 0, 0), CUresult::CUDA_SUCCESS);
        let mut stream = ptr::null_mut();
        assert_eq!(T::cuStreamCreate(&mut stream, 0), CUresult::CUDA_SUCCESS);
        let mut stream_ctx1 = ptr::null_mut();
        assert_eq!(
            T::cuStreamGetCtx(stream, &mut stream_ctx1),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(stream_ctx1, ctx);
        assert_eq!(T::cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
        let mut stream_ctx2 = ptr::null_mut();
        // When a context gets destroyed, its streams are also destroyed
        let cuda_result = T::cuStreamGetCtx(stream, &mut stream_ctx2);
        assert!(
            cuda_result == CUresult::CUDA_ERROR_INVALID_HANDLE
                || cuda_result == CUresult::CUDA_ERROR_INVALID_CONTEXT
                || cuda_result == CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
        );
        assert_eq!(
            T::cuStreamDestroy_v2(stream),
            CUresult::CUDA_ERROR_INVALID_HANDLE
        );
        // Check if creating another context is possible
        let mut ctx2 = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx2, 0, 0), CUresult::CUDA_SUCCESS);
        //  Cleanup
        assert_eq!(T::cuCtxDestroy_v2(ctx2), CUresult::CUDA_SUCCESS);
    }

    cuda_driver_test!(stream_moves_context_to_another_thread);

    fn stream_moves_context_to_another_thread<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx, 0, 0), CUresult::CUDA_SUCCESS);
        let mut stream = ptr::null_mut();
        assert_eq!(T::cuStreamCreate(&mut stream, 0), CUresult::CUDA_SUCCESS);
        let mut stream_ctx1 = ptr::null_mut();
        assert_eq!(
            T::cuStreamGetCtx(stream, &mut stream_ctx1),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(stream_ctx1, ctx);
        let stream_ptr = stream as usize;
        let stream_ctx_on_thread = thread::spawn(move || {
            let mut stream_ctx2 = ptr::null_mut();
            assert_eq!(
                T::cuStreamGetCtx(stream_ptr as *mut _, &mut stream_ctx2),
                CUresult::CUDA_SUCCESS
            );
            stream_ctx2 as usize
        })
        .join()
        .unwrap();
        assert_eq!(stream_ctx1, stream_ctx_on_thread as *mut _);
        //  Cleanup
        assert_eq!(T::cuStreamDestroy_v2(stream), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
    }

    cuda_driver_test!(can_destroy_stream);

    fn can_destroy_stream<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx, 0, 0), CUresult::CUDA_SUCCESS);
        let mut stream = ptr::null_mut();
        assert_eq!(T::cuStreamCreate(&mut stream, 0), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuStreamDestroy_v2(stream), CUresult::CUDA_SUCCESS);
        // Cleanup
        assert_eq!(T::cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
    }

    cuda_driver_test!(cant_destroy_default_stream);

    fn cant_destroy_default_stream<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx, 0, 0), CUresult::CUDA_SUCCESS);
        assert_ne!(
            T::cuStreamDestroy_v2(super::CU_STREAM_LEGACY as *mut _),
            CUresult::CUDA_SUCCESS
        );
        // Cleanup
        assert_eq!(T::cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
    }
}
