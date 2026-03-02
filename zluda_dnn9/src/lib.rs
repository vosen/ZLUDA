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
        // Copy result (dx) back from device
        let dx_count = 1 * 256 * 1 * 7824;
        let mut dx_result = vec![f16::ZERO; dx_count];
        api.copy_to_host(
            dx_result.as_mut_ptr().cast(),
            tensor1_mem_dev as *const _,
            dx_count * mem::size_of::<f16>(),
        );
        // Compute reference with candle on CPU
        // cudnnConvolutionBackwardData with cross-correlation computes a transposed convolution:
        //   dx = conv_transpose(filter, dy)
        // Since H=1 and pad_h=0, this is effectively a 1D transposed convolution.
        // filter shape: [256, 256, 1, 7] -> reshape to [256, 256, 7] for conv_transpose1d
        // dy (tensor2) shape: [1, 256, 1, 7824] -> reshape to [1, 256, 7824]
        // dx (tensor1) shape: [1, 256, 1, 7824] -> reshape to [1, 256, 7824]
        use candle_core::{Device, Tensor};
        let dev = &Device::Cpu;
        let filter_f32: Vec<f32> = filter_mem_host.iter().map(|v| v.to_f32()).collect();
        let dy_f32: Vec<f32> = tensor2_mem_host.iter().map(|v| v.to_f32()).collect();
        let filter_t = Tensor::from_vec(filter_f32, &[256, 256, 1, 7], dev)
            .unwrap()
            .reshape(&[256, 256, 7])
            .unwrap();
        let dy_t = Tensor::from_vec(dy_f32, &[1, 256, 1, 7824], dev)
            .unwrap()
            .reshape(&[1, 256, 7824])
            .unwrap();
        // padding=3, output_padding=0, stride=1, dilation=1, groups=1
        let dx_ref = dy_t
            .conv_transpose1d(
                &filter_t, /*padding=*/ 3, /*output_padding=*/ 0, /*stride=*/ 1,
                /*dilation=*/ 1, /*groups=*/ 1,
            )
            .unwrap();
        let dx_ref_f32 = dx_ref.flatten_all().unwrap().to_vec1::<f32>().unwrap();
        // Compare GPU result against CPU reference
        assert_eq!(
            dx_result.len(),
            dx_ref_f32.len(),
            "output size mismatch: GPU={} vs CPU={}",
            dx_result.len(),
            dx_ref_f32.len()
        );
        let mut max_abs_err: f32 = 0.0;
        let mut max_rel_err: f32 = 0.0;
        for i in 0..dx_result.len() {
            let gpu = dx_result[i].to_f32();
            let cpu = dx_ref_f32[i];
            let abs_err = (gpu - cpu).abs();
            let rel_err = if cpu.abs() > 1e-6 {
                abs_err / cpu.abs()
            } else {
                abs_err
            };
            max_abs_err = max_abs_err.max(abs_err);
            max_rel_err = max_rel_err.max(rel_err);
        }
        // f16 has ~3 decimal digits of precision. With accumulated sums over 256*7=1792 terms,
        // the absolute error can be significant, so use a generous tolerance.
        assert!(
            max_rel_err < 0.05,
            "relative error too large: max_abs_err={max_abs_err}, max_rel_err={max_rel_err}"
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
}

#[cfg(test)]
mod tests;
