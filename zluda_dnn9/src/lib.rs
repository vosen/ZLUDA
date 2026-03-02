macro_rules! export_dnn9_unmangled {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                zluda_dnn::dnn9::$fn_name( $( $arg_id ),* )
            }
        )*
    };
}

cuda_macros::cudnn9_function_declarations! {
    export_dnn9_unmangled
}

#[cfg(windows)]
mod windows {
    use zluda_windows;

    #[no_mangle]
    static __pfnDliNotifyHook2: zluda_windows::PfnDliHook =
        zluda_windows::open_already_loaded_amdhip;
}

#[cfg(test)]
mod tests_impl {
    use crate::tests::CudnnApi;
    use cuda_macros::test_cuda;
    use cuda_types::cudnn9::*;
    use cuda_types::{cuda::CUstream, cudnn9::cudnnDataType_t};
    use half::f16;
    use rand::Rng;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::{mem, ptr};

    #[test_cuda]
    fn create_destroy(api: impl CudnnApi) {
        let mut handle = unsafe { std::mem::zeroed() };
        api.cudnnCreate(&mut handle);
        api.cudnnDestroy(handle);
    }

    #[test_cuda]
    fn pytorch_repro(api: impl CudnnApi) {
        let mut handle = unsafe { std::mem::zeroed() };
        api.cudnnCreate(&mut handle);
        api.cudnnSetStream(handle, CUstream(ptr::null_mut()));
        let mut tensor1 = unsafe { std::mem::zeroed() };
        api.cudnnCreateTensorDescriptor(&mut tensor1);
        api.cudnnSetTensorNdDescriptor(
            tensor1,
            cudnnDataType_t::CUDNN_DATA_HALF,
            4,
            [1, 256, 1, 7824].as_ptr(),
            [2002944, 7824, 7824, 1].as_ptr(),
        );
        let mut filter = unsafe { std::mem::zeroed() };
        api.cudnnCreateFilterDescriptor(&mut filter);
        api.cudnnSetFilterNdDescriptor(
            filter,
            cudnnDataType_t::CUDNN_DATA_HALF,
            cudnnTensorFormat_t::CUDNN_TENSOR_NCHW,
            4,
            [256, 256, 1, 7].as_ptr(),
        );
        let mut tensor2 = unsafe { std::mem::zeroed() };
        api.cudnnCreateTensorDescriptor(&mut tensor2);
        api.cudnnSetTensorNdDescriptor(
            tensor2,
            cudnnDataType_t::CUDNN_DATA_HALF,
            4,
            [1, 256, 1, 7824].as_ptr(),
            [2002944, 7824, 7824, 1].as_ptr(),
        );
        let mut conv_desc = unsafe { std::mem::zeroed() };
        api.cudnnCreateConvolutionDescriptor(&mut conv_desc);
        api.cudnnSetConvolutionNdDescriptor(
            conv_desc,
            2,
            [0, 3].as_ptr(),
            [1, 1].as_ptr(),
            [1, 1].as_ptr(),
            cudnnConvolutionMode_t::CUDNN_CROSS_CORRELATION,
            cudnnDataType_t::CUDNN_DATA_FLOAT,
        );
        api.cudnnSetConvolutionGroupCount(conv_desc, 1);
        let mut algo_count = 0;
        let mut perf_results = unsafe { mem::zeroed() };
        api.cudnnGetConvolutionBackwardDataAlgorithm_v7(
            handle,
            filter,
            tensor2,
            conv_desc,
            tensor1,
            8,
            &mut algo_count,
            &mut perf_results,
        );
        let mut rng = ChaCha8Rng::from_seed([
            0xca, 0xb9, 0x3c, 0xf4, 0xa7, 0xc7, 0xa8, 0xa5, 0x47, 0x6c, 0x86, 0xa0, 0x62, 0x6e,
            0x4c, 0xb7, 0x95, 0x5f, 0x39, 0x19, 0x2f, 0xb8, 0x69, 0xd1, 0xce, 0xc4, 0xfc, 0xc7,
            0xe3, 0x2c, 0xe4, 0x1d,
        ]);
        let filter_mem_dev = api.alloc(256 * 256 * 1 * 7 * mem::size_of::<f16>());
        let filter_mem_host = fill_buffer::<f16>(&api, &mut rng, filter_mem_dev, 256 * 256 * 1 * 7);
        let tensor1_mem_dev = api.alloc(1 * 256 * 1 * 7824 * mem::size_of::<f16>());
        let tensor1_mem_host =
            fill_buffer::<f16>(&api, &mut rng, tensor1_mem_dev, 1 * 256 * 1 * 7824);
        let tensor2_mem_dev = api.alloc(1 * 256 * 1 * 7824 * mem::size_of::<f16>());
        let tensor2_mem_host =
            fill_buffer::<f16>(&api, &mut rng, tensor2_mem_dev, 1 * 256 * 1 * 7824);
        let workspace_mem = if perf_results.memory > 0 {
            api.alloc(perf_results.memory)
        } else {
            ptr::null_mut()
        };
        api.cudnnConvolutionBackwardData(
            handle,
            std::ptr::from_ref(&1.0f32).cast(),
            filter,
            filter_mem_dev,
            tensor2,
            tensor2_mem_dev,
            conv_desc,
            perf_results.algo,
            workspace_mem,
            perf_results.memory,
            std::ptr::from_ref(&0.0f32).cast(),
            tensor1,
            tensor1_mem_dev,
        );
        
    }

    fn fill_buffer<T>(
        api: &impl CudnnApi,
        rng: &mut impl Rng,
        filter_mem_device: *mut std::ffi::c_void,
        size: usize,
    ) -> Vec<T>
    where
        rand::distr::StandardUniform: rand::distr::Distribution<T>,
    {
        let filter_mem_host = (0..size).map(|_| rng.random::<T>()).collect::<Vec<_>>();
        api.copy_to_device(
            filter_mem_device,
            filter_mem_host.as_ptr().cast(),
            size * mem::size_of::<T>(),
        );
        filter_mem_host
    }

    // fn allocate<T>(api: impl CudnnApi, size: usize) -> *mut std::ffi::c_void {
    //     let mut ptr = std::ptr::null_mut();
    //     unsafe {
    //         api.cudnnAlloc(&mut ptr, 1024);
    //     }
    //     ptr
    // }
}

#[cfg(test)]
mod tests;
