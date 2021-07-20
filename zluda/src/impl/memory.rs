use super::{stream, CUresult, GlobalState};
use std::{ffi::c_void, mem};

pub fn alloc_v2(dptr: *mut *mut c_void, bytesize: usize) -> Result<(), CUresult> {
    let ptr = GlobalState::lock_current_context(|ctx| {
        let dev = unsafe { &mut *ctx.device };
        Ok::<_, CUresult>(dev.ocl_context.mem_alloc_device(bytesize, 0, dev.base)?)
    })??;
    unsafe { *dptr = ptr };
    Ok(())
}

pub fn copy_v2(dst: *mut c_void, src: *const c_void, bytesize: usize) -> Result<(), CUresult> {
    GlobalState::lock_enqueue(stream::CU_STREAM_LEGACY, |cmd_list, signal, wait| {
        unsafe { cmd_list.append_memory_copy_raw(dst, src, bytesize, Some(signal), wait)? };
        Ok(())
    })
}

pub fn free_v2(ptr: *mut c_void) -> Result<(), CUresult> {
    GlobalState::lock_current_context(|ctx| {
        let dev = unsafe { &mut *ctx.device };
        Ok::<_, CUresult>(dev.ocl_context.mem_free(ptr)?)
    })
    .map_err(|_| CUresult::CUDA_ERROR_INVALID_VALUE)?
}

pub(crate) fn set_d32_v2(dst: *mut c_void, mut ui: u32, n: usize) -> Result<(), CUresult> {
    GlobalState::lock_enqueue(stream::CU_STREAM_LEGACY, |cmd_list, signal, wait| {
        unsafe {
            cmd_list.append_memory_fill_raw(
                dst,
                &mut ui as *mut _ as *mut _,
                mem::size_of::<u32>(),
                mem::size_of::<u32>() * n,
                Some(signal),
                wait,
            )
        }?;
        Ok(())
    })
}

pub(crate) fn set_d8_v2(dst: *mut c_void, mut uc: u8, n: usize) -> Result<(), CUresult> {
    GlobalState::lock_enqueue(stream::CU_STREAM_LEGACY, |cmd_list, signal, wait| {
        unsafe {
            cmd_list.append_memory_fill_raw(
                dst,
                &mut uc as *mut _ as *mut _,
                mem::size_of::<u8>(),
                mem::size_of::<u8>() * n,
                Some(signal),
                wait,
            )
        }?;
        Ok(())
    })
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
