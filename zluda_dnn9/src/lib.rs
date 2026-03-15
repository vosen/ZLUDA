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
    fn pytorch_conv_backward(api: impl CudnnApi) {
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
        let mut perf_results =
            unsafe { [mem::zeroed::<cudnnConvolutionBwdDataAlgoPerfStruct>(); 8] };
        api.cudnnGetConvolutionBackwardDataAlgorithm_v7(
            handle,
            filter,
            tensor2,
            conv_desc,
            tensor1,
            perf_results.len() as i32,
            &mut algo_count,
            perf_results.as_mut_ptr(),
        );
        for i in 0..algo_count as usize {
            if perf_results[i].status != Ok(()) {
                continue;
            }
            let mut memory = 0;
            api.cudnnGetConvolutionBackwardDataWorkspaceSize(
                handle,
                filter,
                tensor2,
                conv_desc,
                tensor1,
                perf_results[i].algo,
                &mut memory,
            );
            assert_eq!(memory, perf_results[i].memory);
        }
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
        let workspace_mem = if perf_results[0].memory > 0 {
            api.alloc(perf_results[0].memory)
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
            perf_results[0].algo,
            workspace_mem,
            perf_results[0].memory,
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

    #[test_cuda]
    fn pytorch_conv_backward_f32(api: impl CudnnApi) {
        let mut handle = unsafe { std::mem::zeroed() };
        api.cudnnCreate(&mut handle);
        api.cudnnSetStream(handle, CUstream(ptr::null_mut()));
        let mut tensor1 = unsafe { std::mem::zeroed() };
        api.cudnnCreateTensorDescriptor(&mut tensor1);
        api.cudnnSetTensorNdDescriptor(
            tensor1,
            cudnnDataType_t::CUDNN_DATA_FLOAT,
            4,
            [1, 256, 1, 320].as_ptr(),
            [81920, 320, 320, 1].as_ptr(),
        );
        let mut filter = unsafe { std::mem::zeroed() };
        api.cudnnCreateFilterDescriptor(&mut filter);
        api.cudnnSetFilterNdDescriptor(
            filter,
            cudnnDataType_t::CUDNN_DATA_FLOAT,
            cudnnTensorFormat_t::CUDNN_TENSOR_NCHW,
            4,
            [512, 256, 1, 16].as_ptr(),
        );
        let mut tensor2 = unsafe { std::mem::zeroed() };
        api.cudnnCreateTensorDescriptor(&mut tensor2);
        api.cudnnSetTensorNdDescriptor(
            tensor2,
            cudnnDataType_t::CUDNN_DATA_FLOAT,
            4,
            [1, 512, 1, 40].as_ptr(),
            [20480, 40, 40, 1].as_ptr(),
        );
        let mut conv_desc = unsafe { std::mem::zeroed() };
        api.cudnnCreateConvolutionDescriptor(&mut conv_desc);
        api.cudnnSetConvolutionNdDescriptor(
            conv_desc,
            2,
            [0, 4].as_ptr(),
            [1, 8].as_ptr(),
            [1, 1].as_ptr(),
            cudnnConvolutionMode_t::CUDNN_CROSS_CORRELATION,
            cudnnDataType_t::CUDNN_DATA_FLOAT,
        );
        api.cudnnSetConvolutionGroupCount(conv_desc, 1);
        let mut algo_count = 0;
        let mut perf_results =
            unsafe { [mem::zeroed::<cudnnConvolutionBwdDataAlgoPerfStruct>(); 8] };
        api.cudnnGetConvolutionBackwardDataAlgorithm_v7(
            handle,
            filter,
            tensor2,
            conv_desc,
            tensor1,
            perf_results.len() as i32,
            &mut algo_count,
            perf_results.as_mut_ptr(),
        );
        for i in 0..algo_count as usize {
            if perf_results[i].status != Ok(()) {
                continue;
            }
            let mut memory = 0;
            api.cudnnGetConvolutionBackwardDataWorkspaceSize(
                handle,
                filter,
                tensor2,
                conv_desc,
                tensor1,
                perf_results[i].algo,
                &mut memory,
            );
            assert_eq!(memory, perf_results[i].memory);
        }
        let mut rng = ChaCha8Rng::from_seed([
            0xca, 0xb9, 0x3c, 0xf4, 0xa7, 0xc7, 0xa8, 0xa5, 0x47, 0x6c, 0x86, 0xa0, 0x62, 0x6e,
            0x4c, 0xb7, 0x95, 0x5f, 0x39, 0x19, 0x2f, 0xb8, 0x69, 0xd1, 0xce, 0xc4, 0xfc, 0xc7,
            0xe3, 0x2c, 0xe4, 0x1d,
        ]);
        let filter_mem_dev = api.alloc(512 * 256 * 1 * 16 * mem::size_of::<f32>());
        let filter_mem_host =
            fill_buffer::<f32>(&api, &mut rng, filter_mem_dev, 512 * 256 * 1 * 16);
        let tensor1_mem_dev = api.alloc(1 * 256 * 1 * 320 * mem::size_of::<f32>());
        let tensor2_mem_dev = api.alloc(1 * 512 * 1 * 40 * mem::size_of::<f32>());
        let tensor2_mem_host =
            fill_buffer::<f32>(&api, &mut rng, tensor2_mem_dev, 1 * 512 * 1 * 40);
        let workspace_mem = if perf_results[0].memory > 0 {
            api.alloc(perf_results[0].memory)
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
            perf_results[0].algo,
            workspace_mem,
            perf_results[0].memory,
            std::ptr::from_ref(&0.0f32).cast(),
            tensor1,
            tensor1_mem_dev,
        );
        // Copy result (dx) back from device
        let dx_count = 1 * 256 * 1 * 320;
        let mut dx_result = vec![0f32; dx_count];
        api.copy_to_host(
            dx_result.as_mut_ptr().cast(),
            tensor1_mem_dev as *const _,
            dx_count * mem::size_of::<f32>(),
        );
        // Compute reference with candle on CPU
        // cudnnConvolutionBackwardData with cross-correlation computes a transposed convolution:
        //   dx = conv_transpose(filter, dy)
        // Since H=1 and pad_h=0, this is effectively a 1D transposed convolution.
        // filter shape: [512, 256, 1, 16] -> reshape to [512, 256, 16] for conv_transpose1d
        // dy (tensor2) shape: [1, 512, 1, 40] -> reshape to [1, 512, 40]
        // dx (tensor1) shape: [1, 256, 1, 320] -> reshape to [1, 256, 320]
        use candle_core::{Device, Tensor};
        let dev = &Device::Cpu;
        let filter_f32: Vec<f32> = filter_mem_host.iter().copied().collect();
        let dy_f32: Vec<f32> = tensor2_mem_host.iter().copied().collect();
        let filter_t = Tensor::from_vec(filter_f32, &[512, 256, 1, 16], dev)
            .unwrap()
            .reshape(&[512, 256, 16])
            .unwrap();
        let dy_t = Tensor::from_vec(dy_f32, &[1, 512, 1, 40], dev)
            .unwrap()
            .reshape(&[1, 512, 40])
            .unwrap();
        // padding=4, output_padding=0, stride=8, dilation=1, groups=1
        let dx_ref = dy_t
            .conv_transpose1d(
                &filter_t, /*padding=*/ 4, /*output_padding=*/ 0, /*stride=*/ 8,
                /*dilation=*/ 1, /*groups=*/ 1,
            )
            .unwrap();
        let dx_ref_f32 = dx_ref.flatten_all().unwrap().to_vec1::<f32>().unwrap();
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
            let gpu = dx_result[i];
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
        assert!(
            max_rel_err < 0.05,
            "relative error too large: max_abs_err={max_abs_err}, max_rel_err={max_rel_err}"
        );
    }

    #[test_cuda]
    fn pytorch_conv_backward_filter(api: impl CudnnApi) {
        let mut handle = unsafe { std::mem::zeroed() };
        api.cudnnCreate(&mut handle);
        api.cudnnSetStream(handle, CUstream(ptr::null_mut()));
        // x (input) descriptor: [1, 256, 1, 7824]
        let mut x_desc = unsafe { std::mem::zeroed() };
        api.cudnnCreateTensorDescriptor(&mut x_desc);
        api.cudnnSetTensorNdDescriptor(
            x_desc,
            cudnnDataType_t::CUDNN_DATA_HALF,
            4,
            [1, 256, 1, 7824].as_ptr(),
            [2002944, 7824, 7824, 1].as_ptr(),
        );
        // filter (dw) descriptor: [256, 256, 1, 7]
        let mut filter_desc = unsafe { std::mem::zeroed() };
        api.cudnnCreateFilterDescriptor(&mut filter_desc);
        api.cudnnSetFilterNdDescriptor(
            filter_desc,
            cudnnDataType_t::CUDNN_DATA_HALF,
            cudnnTensorFormat_t::CUDNN_TENSOR_NCHW,
            4,
            [256, 256, 1, 7].as_ptr(),
        );
        // dy (output gradient) descriptor: [1, 256, 1, 7824]
        let mut dy_desc = unsafe { std::mem::zeroed() };
        api.cudnnCreateTensorDescriptor(&mut dy_desc);
        api.cudnnSetTensorNdDescriptor(
            dy_desc,
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
        let mut perf_results = unsafe { [mem::zeroed(); 8] };
        api.cudnnGetConvolutionBackwardFilterAlgorithm_v7(
            handle,
            x_desc,
            dy_desc,
            conv_desc,
            filter_desc,
            perf_results.len() as i32,
            &mut algo_count,
            perf_results.as_mut_ptr(),
        );
        for i in 0..algo_count as usize {
            if perf_results[i].status != Ok(()) {
                continue;
            }
            let mut memory = 0;
            api.cudnnGetConvolutionBackwardFilterWorkspaceSize(
                handle,
                x_desc,
                dy_desc,
                conv_desc,
                filter_desc,
                perf_results[i].algo,
                &mut memory,
            );
            assert_eq!(memory, perf_results[i].memory);
        }
        let mut rng = ChaCha8Rng::from_seed([
            0xca, 0xb9, 0x3c, 0xf4, 0xa7, 0xc7, 0xa8, 0xa5, 0x47, 0x6c, 0x86, 0xa0, 0x62, 0x6e,
            0x4c, 0xb7, 0x95, 0x5f, 0x39, 0x19, 0x2f, 0xb8, 0x69, 0xd1, 0xce, 0xc4, 0xfc, 0xc7,
            0xe3, 0x2c, 0xe4, 0x1d,
        ]);
        let x_mem_dev = api.alloc(1 * 256 * 1 * 7824 * mem::size_of::<f16>());
        let x_mem_host = fill_buffer::<f16>(&api, &mut rng, x_mem_dev, 1 * 256 * 1 * 7824);
        let dy_mem_dev = api.alloc(1 * 256 * 1 * 7824 * mem::size_of::<f16>());
        let dy_mem_host = fill_buffer::<f16>(&api, &mut rng, dy_mem_dev, 1 * 256 * 1 * 7824);
        let dw_mem_dev = api.alloc(256 * 256 * 1 * 7 * mem::size_of::<f16>());
        let workspace_mem = if perf_results[0].memory > 0 {
            api.alloc(perf_results[0].memory)
        } else {
            ptr::null_mut()
        };
        api.cudnnConvolutionBackwardFilter(
            handle,
            std::ptr::from_ref(&1.0f32).cast(),
            x_desc,
            x_mem_dev,
            dy_desc,
            dy_mem_dev,
            conv_desc,
            perf_results[0].algo,
            workspace_mem,
            perf_results[0].memory,
            std::ptr::from_ref(&0.0f32).cast(),
            filter_desc,
            dw_mem_dev,
        );
        // Copy result (dw) back from device
        let dw_count = 256 * 256 * 1 * 7;
        let mut dw_result = vec![f16::ZERO; dw_count];
        api.copy_to_host(
            dw_result.as_mut_ptr().cast(),
            dw_mem_dev as *const _,
            dw_count * mem::size_of::<f16>(),
        );
        // Compute reference on CPU
        // cudnnConvolutionBackwardFilter with cross-correlation computes:
        //   dw[k, c, 0, r] = sum_n sum_p dy[n, k, 0, p] * x[n, c, 0, p + r - pad]
        // With N=1, H=1, pad_w=3, stride=1, dilation=1.
        let c_out = 256usize;
        let c_in = 256usize;
        let kw = 7usize;
        let w = 7824usize;
        let pad = 3usize;
        let x_f32: Vec<f32> = x_mem_host.iter().map(|v| v.to_f32()).collect();
        let dy_f32: Vec<f32> = dy_mem_host.iter().map(|v| v.to_f32()).collect();
        let mut dw_ref = vec![0.0f32; c_out * c_in * kw];
        for k in 0..c_out {
            for c in 0..c_in {
                for r in 0..kw {
                    let mut sum = 0.0f32;
                    for p in 0..w {
                        let x_idx = p + r;
                        if x_idx >= pad && x_idx < w + pad {
                            let x_val = x_f32[c * w + (x_idx - pad)];
                            let dy_val = dy_f32[k * w + p];
                            sum += x_val * dy_val;
                        }
                    }
                    dw_ref[k * (c_in * kw) + c * kw + r] = sum;
                }
            }
        }
        // Compare GPU result against CPU reference
        assert_eq!(
            dw_result.len(),
            dw_ref.len(),
            "output size mismatch: GPU={} vs CPU={}",
            dw_result.len(),
            dw_ref.len()
        );
        let mut max_abs_err: f32 = 0.0;
        let mut max_rel_err: f32 = 0.0;
        for i in 0..dw_result.len() {
            let gpu = dw_result[i].to_f32();
            let cpu = dw_ref[i];
            let abs_err = (gpu - cpu).abs();
            let rel_err = if cpu.abs() > 1e-6 {
                abs_err / cpu.abs()
            } else {
                abs_err
            };
            max_abs_err = max_abs_err.max(abs_err);
            max_rel_err = max_rel_err.max(rel_err);
        }
        // The backward filter accumulates over W=7824 spatial positions per (k,c,r),
        // with f16 inputs. This is a large reduction so expect notable numerical error.
        assert!(
            max_rel_err < 0.05,
            "relative error too large: max_abs_err={max_abs_err}, max_rel_err={max_rel_err}"
        );
    }

    #[test_cuda]
    fn pytorch_conv_forward(api: impl CudnnApi) {
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
        let mut perf_results = unsafe { [mem::zeroed(); 8] };
        api.cudnnGetConvolutionForwardAlgorithm_v7(
            handle,
            tensor1,
            filter,
            conv_desc,
            tensor2,
            perf_results.len() as i32,
            &mut algo_count,
            perf_results.as_mut_ptr(),
        );
        for i in 0..algo_count as usize {
            if perf_results[i].status != Ok(()) {
                continue;
            }
            let mut memory = 0;
            api.cudnnGetConvolutionForwardWorkspaceSize(
                handle,
                tensor1,
                filter,
                conv_desc,
                tensor2,
                perf_results[i].algo,
                &mut memory,
            );
            assert_eq!(memory, perf_results[i].memory);
        }
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
        let workspace_mem = if perf_results[0].memory > 0 {
            api.alloc(perf_results[0].memory)
        } else {
            ptr::null_mut()
        };
        api.cudnnConvolutionForward(
            handle,
            std::ptr::from_ref(&1.0f32).cast(),
            tensor1,
            tensor1_mem_dev,
            filter,
            filter_mem_dev,
            conv_desc,
            perf_results[0].algo,
            workspace_mem,
            perf_results[0].memory,
            std::ptr::from_ref(&0.0f32).cast(),
            tensor2,
            tensor2_mem_dev,
        );
        // Copy result (y) back from device
        let y_count = 1 * 256 * 1 * 7824;
        let mut y_result = vec![f16::ZERO; y_count];
        api.copy_to_host(
            y_result.as_mut_ptr().cast(),
            tensor2_mem_dev as *const _,
            y_count * mem::size_of::<f16>(),
        );
        // Compute reference with candle on CPU
        // cudnnConvolutionForward with cross-correlation computes:
        //   y = conv(x, filter)
        // Since H=1 and pad_h=0, this is effectively a 1D convolution.
        // filter shape: [256, 256, 1, 7] -> reshape to [256, 256, 7] for conv1d
        // x (tensor1) shape: [1, 256, 1, 7824] -> reshape to [1, 256, 7824]
        // y (tensor2) shape: [1, 256, 1, 7824] -> reshape to [1, 256, 7824]
        use candle_core::{Device, Tensor};
        let dev = &Device::Cpu;
        let filter_f32: Vec<f32> = filter_mem_host.iter().map(|v| v.to_f32()).collect();
        let x_f32: Vec<f32> = tensor1_mem_host.iter().map(|v| v.to_f32()).collect();
        let filter_t = Tensor::from_vec(filter_f32, &[256, 256, 1, 7], dev)
            .unwrap()
            .reshape(&[256, 256, 7])
            .unwrap();
        let x_t = Tensor::from_vec(x_f32, &[1, 256, 1, 7824], dev)
            .unwrap()
            .reshape(&[1, 256, 7824])
            .unwrap();
        // padding=3, stride=1, dilation=1, groups=1
        let y_ref = x_t
            .conv1d(
                &filter_t, /*padding=*/ 3, /*stride=*/ 1, /*dilation=*/ 1,
                /*groups=*/ 1,
            )
            .unwrap();
        let y_ref_f32 = y_ref.flatten_all().unwrap().to_vec1::<f32>().unwrap();
        // Compare GPU result against CPU reference
        assert_eq!(
            y_result.len(),
            y_ref_f32.len(),
            "output size mismatch: GPU={} vs CPU={}",
            y_result.len(),
            y_ref_f32.len()
        );
        let mut max_abs_err: f32 = 0.0;
        let mut max_rel_err: f32 = 0.0;
        for i in 0..y_result.len() {
            let gpu = y_result[i].to_f32();
            let cpu = y_ref_f32[i];
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

    #[test_cuda]
    fn conv_forward_nhwc(api: impl CudnnApi) {
        let mut handle = unsafe { std::mem::zeroed() };
        api.cudnnCreate(&mut handle);
        api.cudnnSetStream(handle, CUstream(ptr::null_mut()));

        // Input tensor: [1,1,1,25600] with stride [25600,1,25600,1]
        let mut x_desc = unsafe { std::mem::zeroed() };
        api.cudnnCreateTensorDescriptor(&mut x_desc);
        api.cudnnSetTensorNdDescriptor(
            x_desc,
            cudnnDataType_t::CUDNN_DATA_FLOAT,
            4,
            [1, 1, 1, 25600].as_ptr(),
            [25600, 1, 25600, 1].as_ptr(),
        );

        // Filter tensor: [256,1,1,80] NHWC
        let mut w_desc = unsafe { std::mem::zeroed() };
        api.cudnnCreateFilterDescriptor(&mut w_desc);
        api.cudnnSetFilterNdDescriptor(
            w_desc,
            cudnnDataType_t::CUDNN_DATA_FLOAT,
            cudnnTensorFormat_t::CUDNN_TENSOR_NHWC,
            4,
            [256, 1, 1, 80].as_ptr(),
        );

        // Output tensor: [1,256,1,640] with stride [163840,1,163840,256]
        let mut y_desc = unsafe { std::mem::zeroed() };
        api.cudnnCreateTensorDescriptor(&mut y_desc);
        api.cudnnSetTensorNdDescriptor(
            y_desc,
            cudnnDataType_t::CUDNN_DATA_FLOAT,
            4,
            [1, 256, 1, 640].as_ptr(),
            [163840, 1, 163840, 256].as_ptr(),
        );

        // Convolution descriptor
        let mut conv_desc = unsafe { std::mem::zeroed() };
        api.cudnnCreateConvolutionDescriptor(&mut conv_desc);
        api.cudnnSetConvolutionNdDescriptor(
            conv_desc,
            2,
            [0, 20].as_ptr(),
            [1, 40].as_ptr(),
            [1, 1].as_ptr(),
            cudnnConvolutionMode_t::CUDNN_CROSS_CORRELATION,
            cudnnDataType_t::CUDNN_DATA_FLOAT,
        );
        api.cudnnSetConvolutionGroupCount(conv_desc, 1);
        api.cudnnSetConvolutionMathType(conv_desc, cudnnMathType_t::CUDNN_DEFAULT_MATH);

        // Select algorithm
        let mut algo_count = 0;
        let mut perf_results = unsafe { mem::zeroed() };
        api.cudnnGetConvolutionForwardAlgorithm_v7(
            handle,
            x_desc,
            w_desc,
            conv_desc,
            y_desc,
            1,
            &mut algo_count,
            &mut perf_results,
        );
        assert_eq!(algo_count, 1);

        let x_size = 1 * 1 * 1 * 25600 * mem::size_of::<f32>();
        let w_size = 256 * 1 * 1 * 80 * mem::size_of::<f32>();
        let y_size = 1 * 256 * 1 * 640 * mem::size_of::<f32>();
        let x_mem = api.alloc(x_size);
        let w_mem = api.alloc(w_size);
        let y_mem = api.alloc(y_size);
        let workspace_mem = if perf_results.memory > 0 {
            api.alloc(perf_results.memory)
        } else {
            std::ptr::null_mut()
        };

        // Prepare random data
        let mut rng = ChaCha8Rng::from_seed([
            0xca, 0xb9, 0x3c, 0xf4, 0xa7, 0xc7, 0xa8, 0xa5, 0x47, 0x6c, 0x86, 0xa0, 0x62, 0x6e,
            0x4c, 0xb7, 0xca, 0xb9, 0x3c, 0xf4, 0xa7, 0xc7, 0xa8, 0xa5, 0x47, 0x6c, 0x86, 0xa0,
            0x62, 0x6e, 0x4c, 0xb7,
        ]);
        let x_host: Vec<f32> = (0..(1 * 1 * 1 * 25600)).map(|_| rng.random()).collect();
        let w_host: Vec<f32> = (0..(256 * 1 * 1 * 80)).map(|_| rng.random()).collect();
        api.copy_to_device(x_mem, x_host.as_ptr().cast(), x_size);
        api.copy_to_device(w_mem, w_host.as_ptr().cast(), w_size);

        api.cudnnConvolutionForward(
            handle,
            std::ptr::from_ref(&1.0f32).cast(),
            x_desc,
            x_mem,
            w_desc,
            w_mem,
            conv_desc,
            perf_results.algo,
            workspace_mem,
            perf_results.memory,
            std::ptr::from_ref(&0.0f32).cast(),
            y_desc,
            y_mem,
        );

        // Copy GPU output back
        let mut y_host = vec![0.0f32; 1 * 256 * 1 * 640];
        api.copy_to_host(y_host.as_mut_ptr().cast(), y_mem as *const _, y_size);

        // Compute reference using candle
        use candle_core::{Device, Tensor};
        let dev = &Device::Cpu;
        let x_t = Tensor::from_vec(x_host.clone(), &[1, 1, 1, 25600], dev)
            .unwrap()
            .reshape(&[1, 1, 25600])
            .unwrap();
        let w_t = Tensor::from_vec(w_host.clone(), &[256, 1, 1, 80], dev)
            .unwrap()
            .reshape(&[256, 1, 80])
            .unwrap();
        let y_ref = x_t
            .conv1d(
                &w_t, /*padding=*/ 20, /*stride=*/ 40, /*dilation=*/ 1,
                /*groups=*/ 1,
            )
            .unwrap();
        let y_ref_f32 = y_ref.flatten_all().unwrap().to_vec1::<f32>().unwrap();

        // y_ref is in N,C,L order; cuDNN output is NHWC (N,L,C). Convert.
        let mut y_ref_nhwc = vec![0.0f32; y_ref_f32.len()];
        let c = 256;
        let l = 640;
        for out_c in 0..c {
            for out_l in 0..l {
                y_ref_nhwc[out_l * c + out_c] = y_ref_f32[out_c * l + out_l];
            }
        }

        assert_eq!(y_host.len(), y_ref_nhwc.len());
        let mut max_abs_err = 0.0f32;
        let mut max_rel_err = 0.0f32;
        for i in 0..y_host.len() {
            let gpu = y_host[i];
            let cpu = y_ref_nhwc[i];
            let abs_err = (gpu - cpu).abs();
            let rel_err = if cpu.abs() > 1e-6 {
                abs_err / cpu.abs()
            } else {
                abs_err
            };
            max_abs_err = max_abs_err.max(abs_err);
            max_rel_err = max_rel_err.max(rel_err);
        }
        assert!(
            max_rel_err < 0.05,
            "relative error too large: max_abs_err={max_abs_err}, max_rel_err={max_rel_err}"
        );

        api.cudnnDestroyConvolutionDescriptor(conv_desc);
        api.cudnnDestroyFilterDescriptor(w_desc);
        api.cudnnDestroyTensorDescriptor(y_desc);
        api.cudnnDestroyTensorDescriptor(x_desc);
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
