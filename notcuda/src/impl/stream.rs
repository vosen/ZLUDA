use std::cell::RefCell;

use device::Device;

use super::device;

pub struct Stream {
    dev: *mut Device,
}

pub struct DefaultStream {
    streams: Vec<Option<Stream>>,
}

impl DefaultStream {
    fn new() -> Self {
        DefaultStream {
            streams: Vec::new(),
        }
    }
}

thread_local! {
    pub static DEFAULT_STREAM: RefCell<DefaultStream> = RefCell::new(DefaultStream::new());
}

#[cfg(test)]
mod tests {
    use crate::cuda::CUstream;

    use super::super::test::CudaDriverFns;
    use super::super::CUresult;
    use std::{ffi::c_void, ptr};

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
    }
}
