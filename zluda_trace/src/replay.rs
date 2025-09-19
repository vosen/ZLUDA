use crate::{
    log::ErrorEntry,
    trace::{self, ParsedModule, SavedKernel},
    CudaDynamicFns, FnCallLog,
};
use cuda_types::cuda::*;
use zluda_trace_common::replay::KernelParameter;

pub(crate) fn pre_kernel_launch(
    libcuda: &mut CudaDynamicFns,
    state: &mut trace::StateTracker,
    fn_logger: &mut FnCallLog,
    f: CUfunction,
    args: *mut *mut std::ffi::c_void,
) -> Option<Vec<KernelParameter>> {
    let SavedKernel { name, owner } = fn_logger.try_return(|| {
        state
            .kernels
            .get(&f)
            .ok_or(ErrorEntry::UnknownFunctionHandle(f))
    })?;
    let ParsedModule { kernels } = fn_logger.try_return(|| {
        state
            .parsed_libraries
            .get(owner)
            .ok_or(ErrorEntry::UnknownLibrary(f, *owner))
    })?;
    let kernel_params = fn_logger.try_return(|| {
        kernels
            .get(name)
            .ok_or_else(|| ErrorEntry::UnknownFunction(f, *owner, name.clone()))
    })?;
    let raw_args = unsafe { std::slice::from_raw_parts(args, kernel_params.len()) };
    let mut all_params = Vec::new();
    for (raw_arg, layout) in raw_args.iter().zip(kernel_params.iter()) {
        let mut offset = 0;
        let mut ptr_overrides = Vec::new();
        while offset + std::mem::size_of::<usize>() <= layout.size() {
            let maybe_ptr = unsafe { raw_arg.cast::<u8>().add(offset) };
            let maybe_ptr = unsafe { maybe_ptr.cast::<usize>().read_unaligned() };
            let attrs = &mut [
                CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_RANGE_START_ADDR,
                CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_RANGE_SIZE,
            ];
            let mut start = 0usize;
            let mut size = 0usize;
            let mut data = [
                (&mut start as *mut usize).cast::<std::ffi::c_void>(),
                (&mut size as *mut usize).cast::<std::ffi::c_void>(),
            ];
            if let Some(Ok(())) = libcuda.cuPointerGetAttributes(
                2,
                attrs.as_mut_ptr(),
                data.as_mut_ptr(),
                CUdeviceptr_v2(maybe_ptr as _),
            ) {
                let mut pre_buffer = vec![0u8; size];
                let post_buffer = vec![0u8; size];
                fn_logger.try_cuda(|| {
                    libcuda.cuMemcpyDtoH_v2(
                        pre_buffer.as_mut_ptr().cast(),
                        CUdeviceptr_v2(start as _),
                        size,
                    )
                })?;
                let buffer_offset = maybe_ptr - start;
                ptr_overrides.push((offset, buffer_offset, pre_buffer, post_buffer));
            }
            offset += std::mem::size_of::<usize>();
        }
        all_params.push(KernelParameter {
            data: unsafe { std::slice::from_raw_parts(raw_arg.cast::<u8>(), layout.size()) }
                .to_vec(),
            device_ptrs: ptr_overrides,
        });
    }
    Some(all_params)
}

pub(crate) fn post_kernel_launch(
    libcuda: &mut CudaDynamicFns,
    fn_logger: &mut FnCallLog,
    args: *mut *mut std::ffi::c_void,
    mut kernel_params: Vec<KernelParameter>,
    enqueue_counter: usize,
    kernel_name: String,
) -> Option<()> {
    let raw_args = unsafe { std::slice::from_raw_parts(args, kernel_params.len()) };
    for (raw_arg, param) in raw_args.iter().zip(kernel_params.iter_mut()) {
        for (offset_in_param, offset_in_buffer, _, data_after) in param.device_ptrs.iter_mut() {
            let dev_ptr_param = unsafe { raw_arg.cast::<u8>().add(*offset_in_param) };
            let mut dev_ptr = unsafe { dev_ptr_param.cast::<usize>().read_unaligned() };
            dev_ptr -= *offset_in_buffer;
            fn_logger.try_cuda(|| {
                libcuda.cuMemcpyDtoH_v2(
                    data_after.as_mut_ptr().cast(),
                    CUdeviceptr_v2(dev_ptr as _),
                    data_after.len(),
                )
            })?;
        }
    }
    let path = format!("kernel_{enqueue_counter}_.tar.zst");
    let file =
        fn_logger.try_return(|| std::fs::File::create_new(path).map_err(ErrorEntry::IoError))?;
    fn_logger.try_return(|| {
        zluda_trace_common::replay::save(file, kernel_name, kernel_params)
            .map_err(ErrorEntry::IoError)
    })
}
