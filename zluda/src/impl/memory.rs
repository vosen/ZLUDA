use super::{
    stream::{self, CU_STREAM_LEGACY},
    CUresult, GlobalState,
};
use std::{
    ffi::c_void,
    mem::{self, size_of},
    ptr,
};

pub fn alloc_v2(dptr: *mut *mut c_void, bytesize: usize) -> Result<(), CUresult> {
    let ptr = GlobalState::lock_stream(CU_STREAM_LEGACY, |stream_data| {
        let dev = unsafe { &mut *(*stream_data.context).device };
        let queue = stream_data.cmd_list.as_ref().unwrap();
        let ptr = unsafe {
            ocl_core::ffi::clSVMAlloc(
                dev.ocl_context.as_ptr(),
                ocl_core::ffi::CL_MEM_READ_WRITE,
                bytesize,
                0,
            )
        };
        // CUDA does the same thing and e.g. GeekBench relies on this behavior
        let mut event = ptr::null_mut();
        let err = unsafe {
            ocl_core::ffi::clEnqueueSVMMemFill(
                queue.as_ptr(),
                ptr,
                &0u8 as *const u8 as *const c_void,
                1,
                bytesize,
                0,
                ptr::null(),
                &mut event,
            )
        };
        assert_eq!(err, 0);
        let err = unsafe { ocl_core::ffi::clWaitForEvents(1, &mut event) };
        assert_eq!(err, 0);
        dev.allocations.insert(ptr);
        Ok::<_, CUresult>(ptr)
    })??;
    unsafe { *dptr = ptr };
    Ok(())
}

pub fn copy_v2(dst: *mut c_void, src: *const c_void, bytesize: usize) -> Result<(), CUresult> {
    GlobalState::lock_stream(stream::CU_STREAM_LEGACY, |stream_data| {
        let dev = unsafe { &*(*stream_data.context).device };
        let queue = stream_data.cmd_list.as_ref().unwrap();
        let err = unsafe {
            ocl_core::ffi::clEnqueueSVMMemcpy(
                queue.as_ptr(),
                1,
                dst,
                src,
                bytesize,
                0,
                ptr::null(),
                ptr::null_mut(),
            )
        };
        assert_eq!(err, 0);
        Ok(())
    })?
}

pub fn free_v2(ptr: *mut c_void) -> Result<(), CUresult> {
    GlobalState::lock_current_context(|ctx| {
        let dev = unsafe { &mut *ctx.device };
        unsafe { ocl_core::ffi::clSVMFree(dev.ocl_context.as_ptr(), ptr) };
        dev.allocations.remove(&ptr);
        Ok(())
    })?
}

pub(crate) fn set_d32_v2(dst: *mut c_void, mut ui: u32, n: usize) -> Result<(), CUresult> {
    GlobalState::lock_stream(stream::CU_STREAM_LEGACY, move |stream_data| {
        let dev = unsafe { &*(*stream_data.context).device };
        let queue = stream_data.cmd_list.as_ref().unwrap();
        let pattern_size = mem::size_of_val(&ui);
        let mut event = ptr::null_mut();
        let err = unsafe {
            ocl_core::ffi::clEnqueueSVMMemFill(
                queue.as_ptr(),
                dst,
                &ui as *const _ as *const _,
                pattern_size,
                pattern_size * n,
                0,
                ptr::null(),
                &mut event,
            )
        };
        assert_eq!(err, 0);
        let err = unsafe { ocl_core::ffi::clWaitForEvents(1, &mut event) };
        assert_eq!(err, 0);
        Ok(())
    })?
}

pub(crate) fn set_d8_v2(dst: *mut c_void, mut uc: u8, n: usize) -> Result<(), CUresult> {
    GlobalState::lock_stream(stream::CU_STREAM_LEGACY, move |stream_data| {
        let dev = unsafe { &*(*stream_data.context).device };
        let queue = stream_data.cmd_list.as_ref().unwrap();
        let pattern_size = mem::size_of_val(&uc);
        let mut event = ptr::null_mut();
        let err = unsafe {
            ocl_core::ffi::clEnqueueSVMMemFill(
                queue.as_ptr(),
                dst,
                &uc as *const _ as *const _,
                pattern_size,
                pattern_size * n,
                0,
                ptr::null(),
                &mut event,
            )
        };
        assert_eq!(err, 0);
        let err = unsafe { ocl_core::ffi::clWaitForEvents(1, &mut event) };
        assert_eq!(err, 0);
        Ok(())
    })?
}

#[cfg(test)]
mod test {
    use super::super::test::CudaDriverFns;
    use super::super::CUresult;
    use std::ptr;

    cuda_driver_test!(alloc_without_ctx);

    fn alloc_without_ctx<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut mem = ptr::null_mut();
        assert_eq!(
            T::cuMemAlloc_v2(&mut mem, std::mem::size_of::<usize>()),
            CUresult::CUDA_ERROR_INVALID_CONTEXT
        );
        assert_eq!(mem, ptr::null_mut());
    }

    cuda_driver_test!(alloc_with_ctx);

    fn alloc_with_ctx<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx, 0, 0), CUresult::CUDA_SUCCESS);
        let mut mem = ptr::null_mut();
        assert_eq!(
            T::cuMemAlloc_v2(&mut mem, std::mem::size_of::<usize>()),
            CUresult::CUDA_SUCCESS
        );
        assert_ne!(mem, ptr::null_mut());
        assert_eq!(T::cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
    }

    cuda_driver_test!(free_without_ctx);

    fn free_without_ctx<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut ctx = ptr::null_mut();
        assert_eq!(T::cuCtxCreate_v2(&mut ctx, 0, 0), CUresult::CUDA_SUCCESS);
        let mut mem = ptr::null_mut();
        assert_eq!(
            T::cuMemAlloc_v2(&mut mem, std::mem::size_of::<usize>()),
            CUresult::CUDA_SUCCESS
        );
        assert_ne!(mem, ptr::null_mut());
        assert_eq!(T::cuCtxDestroy_v2(ctx), CUresult::CUDA_SUCCESS);
        assert_eq!(T::cuMemFree_v2(mem), CUresult::CUDA_ERROR_INVALID_VALUE);
    }
}
