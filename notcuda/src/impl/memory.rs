use super::CUresult;
use std::ffi::c_void;

pub fn alloc_v2(dptr: *mut *mut c_void, bytesize: usize) -> CUresult {
    let alloc_result = super::device::with_current_exclusive(|dev| unsafe {
        dev.base.mem_alloc_device(&mut dev.l0_context, bytesize, 0)
    });
    match alloc_result {
        Ok(Ok(alloc)) => {
            unsafe { *dptr = alloc };
            CUresult::CUDA_SUCCESS
        }
        Ok(Err(e)) => e.into(),
        Err(e) => e,
    }
}

pub fn copy_v2(
    dst: *mut c_void,
    src: *const c_void,
    bytesize: usize,
) -> Result<Result<(), l0::sys::ze_result_t>, CUresult> {
    super::device::with_current_exclusive(|dev| unsafe {
        memcpy_impl(
            &mut dev.l0_context,
            dst,
            src,
            bytesize,
            &dev.base,
            &mut dev.default_queue,
        )
    })
}

unsafe fn memcpy_impl(
    ctx: &mut l0::Context,
    dst: *mut c_void,
    src: *const c_void,
    bytes_count: usize,
    dev: &l0::Device,
    queue: &mut l0::CommandQueue,
) -> l0::Result<()> {
    let mut cmd_list = l0::CommandList::new(ctx, &dev)?;
    cmd_list.append_memory_copy_unsafe(dst, src, bytes_count, None, &mut [])?;
    queue.execute(cmd_list)?;
    Ok(())
}

#[cfg(test)]
mod tests {
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
}
