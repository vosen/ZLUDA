#[allow(warnings)]
mod cudnn_types_v7;
#[allow(warnings)]
mod cudnn_types_v8;

pub mod types {
    pub use super::cudnn_types_v7::*;
    pub use super::cudnn_types_v8::*;
}

#[allow(warnings)]
mod cudnn_v7;
pub use cudnn_v7::*;

#[allow(warnings)]
mod cudnn_v8;
pub use cudnn_v8::*;

use types::*;

use hip_runtime_sys::*;
use miopen_sys::*;
use std::{mem, ptr};

macro_rules! call {
    ($expr:expr) => {{
        let result = $expr;
        if result != miopen_sys::miopenStatus_t::miopenStatusSuccess {
            return to_cudnn(result);
        }
    }};
}

#[cfg(debug_assertions)]
fn unsupported() -> cudnnStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
fn unsupported() -> cudnnStatus_t {
    cudnnStatus_t::CUDNN_STATUS_NOT_SUPPORTED
}

fn to_cudnn(status: miopen_sys::miopenStatus_t) -> cudnnStatus_t {
    match status {
        miopen_sys::miopenStatus_t::miopenStatusSuccess => cudnnStatus_t::CUDNN_STATUS_SUCCESS,
        err => panic!("{}", err.0), //cudnnStatus_t::CUDNN_STATUS_INTERNAL_ERROR,
    }
}

unsafe fn create(handle: *mut cudnnHandle_t) -> cudnnStatus_t {
    to_cudnn(miopen_sys::miopenCreate(handle as _))
}

unsafe fn cudnn_create_tensor_descriptor(
    tensor_desc: *mut cudnnTensorDescriptor_t,
) -> cudnnStatus_t {
    to_cudnn(miopen_sys::miopenCreateTensorDescriptor(tensor_desc as _))
}

unsafe fn cudnn_create_activation_descriptor(
    activation_desc: *mut cudnnActivationDescriptor_t,
) -> cudnnStatus_t {
    to_cudnn(miopen_sys::miopenCreateActivationDescriptor(
        activation_desc as _,
    ))
}

unsafe fn cudnn_create_convolution_descriptor(
    conv_desc: *mut cudnnConvolutionDescriptor_t,
) -> cudnnStatus_t {
    to_cudnn(miopen_sys::miopenCreateConvolutionDescriptor(
        conv_desc as _,
    ))
}

unsafe fn cudnn_create_filter_descriptor(
    filter_desc: *mut cudnnFilterDescriptor_t,
) -> cudnnStatus_t {
    to_cudnn(miopen_sys::miopenCreateTensorDescriptor(filter_desc as _))
}

unsafe fn cudnn_create_lrn_descriptor(norm_desc: *mut cudnnLRNDescriptor_t) -> cudnnStatus_t {
    to_cudnn(miopen_sys::miopenCreateLRNDescriptor(norm_desc as _))
}

unsafe fn cudnn_create_pooling_descriptor(
    pooling_desc: *mut cudnnPoolingDescriptor_t,
) -> cudnnStatus_t {
    to_cudnn(miopen_sys::miopenCreatePoolingDescriptor(pooling_desc as _))
}

unsafe fn set_tensor_nd_decriptor(
    tensor_desc: *mut cudnnTensorStruct,
    data_type: cudnnDataType_t,
    nb_dims: i32,
    dim_a: *const i32,
    stride_a: *const i32,
) -> cudnnStatus_t {
    to_cudnn(miopen_sys::miopenSetTensorDescriptor(
        tensor_desc as _,
        from_data_type(data_type),
        nb_dims,
        dim_a as _,
        stride_a as _,
    ))
}

fn from_data_type(type_: cudnnDataType_t) -> miopenDataType_t {
    match type_ {
        cudnnDataType_t::CUDNN_DATA_FLOAT => miopenDataType_t::miopenFloat,
        cudnnDataType_t::CUDNN_DATA_DOUBLE => miopenDataType_t::miopenDouble,
        cudnnDataType_t::CUDNN_DATA_HALF => miopenDataType_t::miopenHalf,
        _ => todo!(),
    }
}

unsafe fn set_filter_nd_descriptor(
    filter_desc: cudnnFilterDescriptor_t,
    data_type: cudnnDataType_t,
    _format: cudnnTensorFormat_t,
    nb_dims: i32,
    filter_dim_a: *const i32,
) -> cudnnStatus_t {
    let data_type = from_data_type(data_type);
    to_cudnn(miopenSetTensorDescriptor(
        filter_desc as _,
        data_type,
        nb_dims,
        filter_dim_a as _,
        ptr::null_mut(),
    ))
}

unsafe fn set_convolution_nd_descriptor(
    conv_desc: cudnnConvolutionDescriptor_t,
    array_length: i32,
    pad_a: *const i32,
    filter_stride_a: *const i32,
    dilation_a: *const i32,
    mode: cudnnConvolutionMode_t,
    _compute_type: cudnnDataType_t,
) -> cudnnStatus_t {
    if array_length != 2 {
        todo!()
    }
    let pad_h = *pad_a.add(0);
    let pad_w = *pad_a.add(1);
    let u = *filter_stride_a.add(0);
    let v = *filter_stride_a.add(1);
    let d_h = *dilation_a.add(0);
    let d_w = *dilation_a.add(1);
    let mode = conv_mode_to_cudnn(mode);
    to_cudnn(miopen_sys::miopenInitConvolutionDescriptor(
        conv_desc as _,
        mode,
        pad_h,
        pad_w,
        u,
        v,
        d_h,
        d_w,
    ))
}

fn conv_mode_to_cudnn(mode: cudnnConvolutionMode_t) -> miopenConvolutionMode_t {
    match mode {
        cudnnConvolutionMode_t::CUDNN_CONVOLUTION => miopenConvolutionMode_t::miopenTranspose,
        cudnnConvolutionMode_t::CUDNN_CROSS_CORRELATION => {
            miopenConvolutionMode_t::miopenConvolution
        }
        _ => panic!(),
    }
}

unsafe fn get_convolution_nd_forward_output_dim(
    conv_desc: cudnnConvolutionDescriptor_t,
    input_tensor_desc: cudnnTensorDescriptor_t,
    filter_desc: cudnnFilterDescriptor_t,
    mut nb_dims: i32,
    tensor_ouput_dim_a: *mut i32,
) -> cudnnStatus_t {
    to_cudnn(miopen_sys::miopenGetConvolutionNdForwardOutputDim(
        conv_desc as _,
        input_tensor_desc as _,
        filter_desc as _,
        &mut nb_dims as *mut _,
        tensor_ouput_dim_a,
    ))
}

unsafe fn find_convolution_forward_algorithm(
    handle: cudnnHandle_t,
    x_desc: cudnnTensorDescriptor_t,
    w_desc: cudnnFilterDescriptor_t,
    conv_desc: cudnnConvolutionDescriptor_t,
    y_desc: cudnnTensorDescriptor_t,
    requested_algo_count: i32,
    returned_algo_count: *mut i32,
    perf_results: *mut cudnnConvolutionFwdAlgoPerf_t,
) -> cudnnStatus_t {
    let mut result = vec![mem::zeroed(); requested_algo_count as usize];
    let mut x_size = 0;
    call! { miopenGetTensorNumBytes(x_desc as _, &mut x_size) };
    let mut x = mem::zeroed();
    let error = hipMalloc(&mut x, x_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let mut w_size = 0;
    call! { miopenGetTensorNumBytes(w_desc as _, &mut w_size) };
    let mut w = mem::zeroed();
    let error = hipMalloc(&mut w, w_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let mut y_size = 0;
    call! { miopenGetTensorNumBytes(y_desc as _, &mut y_size) };
    let mut y = mem::zeroed();
    let error = hipMalloc(&mut y, y_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let mut workspace_size = 0;
    call! { miopenConvolutionForwardGetWorkSpaceSize(handle as _, w_desc as _, x_desc as _, conv_desc as _, y_desc as _, &mut workspace_size) };
    let mut workspace = mem::zeroed();
    let error = hipMalloc(&mut workspace, workspace_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let error = to_cudnn(miopenFindConvolutionForwardAlgorithm(
        handle as _,
        x_desc as _,
        x,
        w_desc as _,
        w,
        conv_desc as _,
        y_desc as _,
        y,
        requested_algo_count,
        returned_algo_count,
        result.as_mut_ptr(),
        workspace,
        workspace_size,
        true,
    ));
    // TODO: propagaate error codes
    drop(hipFree(x));
    drop(hipFree(w));
    drop(hipFree(y));
    drop(hipFree(workspace));
    for i in 0..result.len() {
        let result = result[i];
        *perf_results.add(i) = algoperf_to_cudnn(result);
    }
    error
}

unsafe fn find_convolution_forward_algorithm_ex(
    handle: *mut cudnnContext,
    x_desc: *mut cudnnTensorStruct,
    x: *const std::ffi::c_void,
    w_desc: *mut cudnnFilterStruct,
    w: *const std::ffi::c_void,
    conv_desc: *mut cudnnConvolutionStruct,
    y_desc: *mut cudnnTensorStruct,
    y: *mut std::ffi::c_void,
    requested_algo_count: i32,
    returned_algo_count: *mut i32,
    perf_results: *mut cudnnConvolutionFwdAlgoPerfStruct,
    work_space: *mut std::ffi::c_void,
    work_space_size_in_bytes: usize,
) -> cudnnStatus_t {
    let mut result = vec![mem::zeroed(); requested_algo_count as usize];
    let error = to_cudnn(miopenFindConvolutionForwardAlgorithm(
        handle as _,
        x_desc as _,
        x,
        w_desc as _,
        w,
        conv_desc as _,
        y_desc as _,
        y,
        requested_algo_count,
        returned_algo_count,
        result.as_mut_ptr(),
        work_space,
        work_space_size_in_bytes,
        true,
    ));
    for i in 0..result.len() {
        let result = result[i];
        *perf_results.add(i) = algoperf_to_cudnn(result);
    }
    error
}

unsafe fn algoperf_to_cudnn(result: miopenConvAlgoPerf_t) -> cudnnConvolutionFwdAlgoPerf_t {
    let algo = algo_to_cudnn(result);
    cudnnConvolutionFwdAlgoPerf_t {
        algo,
        status: cudnnStatus_t::CUDNN_STATUS_SUCCESS,
        time: result.time,
        memory: result.memory,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: mem::zeroed(),
    }
}

unsafe fn algo_to_cudnn(result: miopenConvAlgoPerf_t) -> cudnnConvolutionFwdAlgo_t {
    match result.__bindgen_anon_1.fwd_algo {
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoGEMM => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoDirect => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_DIRECT
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoFFT => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoWinograd => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoImplicitGEMM => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM
        }
        _ => cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM,
    }
}

unsafe fn get_convolution_forward_algorithm(
    handle: cudnnHandle_t,
    x_desc: cudnnTensorDescriptor_t,
    w_desc: cudnnFilterDescriptor_t,
    conv_desc: cudnnConvolutionDescriptor_t,
    y_desc: cudnnTensorDescriptor_t,
    _memory_limit_in_bytes: usize,
    algo: *mut cudnnConvolutionFwdAlgo_t,
) -> cudnnStatus_t {
    let mut algo_count = 0;
    let mut result = mem::zeroed();
    let mut x_size = 0;
    call! { miopenGetTensorNumBytes(x_desc as _, &mut x_size) };
    let mut x = mem::zeroed();
    let error = hipMalloc(&mut x, x_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let mut w_size = 0;
    call! { miopenGetTensorNumBytes(w_desc as _, &mut w_size) };
    let mut w = mem::zeroed();
    let error = hipMalloc(&mut w, w_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let mut y_size = 0;
    call! { miopenGetTensorNumBytes(y_desc as _, &mut y_size) };
    let mut y = mem::zeroed();
    let error = hipMalloc(&mut y, y_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let mut workspace_size = 0;
    call! { miopenConvolutionForwardGetWorkSpaceSize(handle as _, w_desc as _, x_desc as _, conv_desc as _, y_desc as _, &mut workspace_size) };
    let mut workspace = mem::zeroed();
    let error = hipMalloc(&mut workspace, workspace_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let error = to_cudnn(miopenFindConvolutionForwardAlgorithm(
        handle as _,
        x_desc as _,
        x,
        w_desc as _,
        w,
        conv_desc as _,
        y_desc as _,
        y,
        1,
        &mut algo_count,
        &mut result,
        workspace,
        workspace_size,
        true,
    ));
    // TODO: propagate error codes
    drop(hipFree(x));
    drop(hipFree(w));
    drop(hipFree(y));
    drop(hipFree(workspace));
    if algo_count > 0 {
        *algo = algo_to_cudnn(result);
    }
    error
}

pub unsafe fn get_convolution_forward_workspace_size(
    handle: *mut cudnnContext,
    x_desc: *mut cudnnTensorStruct,
    w_desc: *mut cudnnFilterStruct,
    conv_desc: *mut cudnnConvolutionStruct,
    y_desc: *mut cudnnTensorStruct,
    _algo: cudnnConvolutionFwdAlgo_t,
    size_in_bytes: *mut usize,
) -> cudnnStatus_t {
    to_cudnn(miopenConvolutionForwardGetWorkSpaceSize(
        handle as _,
        w_desc as _,
        x_desc as _,
        conv_desc as _,
        y_desc as _,
        size_in_bytes,
    ))
}

unsafe fn convolution_forward(
    handle: *mut cudnnContext,
    alpha: *const std::ffi::c_void,
    x_desc: *mut cudnnTensorStruct,
    x: *const std::ffi::c_void,
    w_desc: *mut cudnnFilterStruct,
    w: *const std::ffi::c_void,
    conv_desc: *mut cudnnConvolutionStruct,
    algo: cudnnConvolutionFwdAlgo_t,
    work_space: *mut std::ffi::c_void,
    work_space_size_in_bytes: usize,
    beta: *const std::ffi::c_void,
    y_desc: *mut cudnnTensorStruct,
    y: *mut std::ffi::c_void,
) -> cudnnStatus_t {
    let mut algo = algo_from_cudnn(algo);
    // In cuDNN it is possible to find algorithm for sizes X and then pass the algo
    // for sizes Y. On miOpen this fails
    let mut perf_results = vec![mem::zeroed(); 32];
    let mut algo_count = 0;
    call!(miopenFindConvolutionForwardAlgorithm(
        handle as _,
        x_desc as _,
        x,
        w_desc as _,
        w,
        conv_desc as _,
        y_desc as _,
        y,
        32,
        &mut algo_count,
        perf_results.as_mut_ptr(),
        work_space,
        work_space_size_in_bytes,
        true,
    ));
    if algo_count == 0 {
        panic!()
    }
    if let None = perf_results[..algo_count as usize]
        .iter()
        .find(|result| result.__bindgen_anon_1.fwd_algo == algo)
    {
        algo = perf_results[0].__bindgen_anon_1.fwd_algo;
    }
    to_cudnn(miopenConvolutionForward(
        handle as _,
        alpha,
        x_desc as _,
        x,
        w_desc as _,
        w,
        conv_desc as _,
        algo,
        beta,
        y_desc as _,
        y,
        work_space,
        work_space_size_in_bytes,
    ))
}

fn algo_from_cudnn(algo: cudnnConvolutionFwdAlgo_t) -> miopenConvFwdAlgorithm_t {
    match algo {
        cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM => {
            miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoImplicitGEMM
        }
        cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM => {
            miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoGEMM
        }
        cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM => {
            miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoGEMM
        }
        cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_DIRECT => {
            miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoDirect
        }
        cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT => {
            miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoFFT
        }
        cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT_TILING => {
            miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoFFT
        }
        cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD => {
            miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoWinograd
        }
        cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD_NONFUSED => {
            miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoWinograd
        }
        _ => miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoGEMM,
    }
}

unsafe fn add_tensor(
    handle: *mut cudnnContext,
    alpha: *const std::ffi::c_void,
    a_desc: *mut cudnnTensorStruct,
    a: *const std::ffi::c_void,
    beta: *const std::ffi::c_void,
    c_desc: *mut cudnnTensorStruct,
    c: *mut std::ffi::c_void,
) -> cudnnStatus_t {
    // CUDA tensor A might be 1 in some dimensions
    // MIOpen tensors A and C must be the same
    let zero = 0f64;
    to_cudnn(miopenOpTensor(
        handle as _,
        miopenTensorOp_t::miopenTensorOpAdd,
        alpha,
        c_desc as _,
        c,
        beta,
        a_desc as _,
        a,
        &zero as *const _ as _,
        c_desc as _,
        c,
    ))
}

unsafe fn set_pooling_nd_descriptor(
    pooling_desc: *mut cudnnPoolingStruct,
    mode: cudnnPoolingMode_t,
    _maxpooling_nan_opt: cudnnNanPropagation_t,
    nb_dims: i32,
    window_dim_a: *const i32,
    padding_a: *const i32,
    stride_a: *const i32,
) -> cudnnStatus_t {
    let mode = pooling_from_cudnn(mode);
    to_cudnn(miopenSetNdPoolingDescriptor(
        pooling_desc as _,
        mode,
        nb_dims,
        window_dim_a as _,
        padding_a as _,
        stride_a as _,
    ))
}

fn pooling_from_cudnn(mode: cudnnPoolingMode_t) -> miopenPoolingMode_t {
    match mode {
        cudnnPoolingMode_t::CUDNN_POOLING_MAX => miopenPoolingMode_t::miopenPoolingMax,
        cudnnPoolingMode_t::CUDNN_POOLING_AVERAGE_COUNT_INCLUDE_PADDING => {
            miopenPoolingMode_t::miopenPoolingAverageInclusive
        }
        cudnnPoolingMode_t::CUDNN_POOLING_AVERAGE_COUNT_EXCLUDE_PADDING => {
            miopenPoolingMode_t::miopenPoolingAverage
        }
        _ => todo!(),
    }
}

unsafe fn get_pooling_nd_forward_output_dim(
    pooling_desc: *mut cudnnPoolingStruct,
    input_tensor_desc: *mut cudnnTensorStruct,
    nb_dims: i32,
    output_tensor_dim_a: *mut i32,
) -> cudnnStatus_t {
    if nb_dims != 4 {
        todo!()
    }
    to_cudnn(miopenGetPoolingForwardOutputDim(
        pooling_desc as _,
        input_tensor_desc as _,
        output_tensor_dim_a.add(0),
        output_tensor_dim_a.add(1),
        output_tensor_dim_a.add(2),
        output_tensor_dim_a.add(3),
    ))
}

unsafe fn pooling_forward(
    handle: *mut cudnnContext,
    pooling_desc: *mut cudnnPoolingStruct,
    alpha: *const std::ffi::c_void,
    x_desc: *mut cudnnTensorStruct,
    x: *const std::ffi::c_void,
    beta: *const std::ffi::c_void,
    y_desc: *mut cudnnTensorStruct,
    y: *mut std::ffi::c_void,
) -> cudnnStatus_t {
    let mut workspace_size = 0;
    call! { miopenPoolingGetWorkSpaceSize(y_desc as _, &mut workspace_size) };
    let mut workspace = mem::zeroed();
    let error = hipMalloc(&mut workspace, workspace_size);
    if error != hipError_t::hipSuccess {
        return cudnnStatus_t::CUDNN_STATUS_INTERNAL_ERROR;
    }
    // TODO: Only alpha=1 and beta=0 is supported
    let error = to_cudnn(miopenPoolingForward(
        handle as _,
        pooling_desc as _,
        alpha,
        x_desc as _,
        x,
        beta,
        y_desc as _,
        y,
        false,
        workspace,
        workspace_size,
    ));
    // TODO: propagate error codes
    drop(hipFree(workspace));
    error
}

unsafe fn set_activation_descriptor(
    activation_desc: *mut cudnnActivationStruct,
    mode: cudnnActivationMode_t,
    _relu_nan_opt: cudnnNanPropagation_t,
    coef: f64,
) -> cudnnStatus_t {
    let mode = activation_mode(mode);
    to_cudnn(miopenSetActivationDescriptor(
        activation_desc as _,
        mode,
        coef,
        0.0,
        0.0,
    ))
}

fn activation_mode(mode: cudnnActivationMode_t) -> miopenActivationMode_t {
    match mode {
        cudnnActivationMode_t::CUDNN_ACTIVATION_SIGMOID => {
            miopenActivationMode_t::miopenActivationLOGISTIC
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_RELU => {
            miopenActivationMode_t::miopenActivationRELU
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_TANH => {
            miopenActivationMode_t::miopenActivationTANH
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_CLIPPED_RELU => {
            miopenActivationMode_t::miopenActivationCLIPPEDRELU
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_ELU => miopenActivationMode_t::miopenActivationELU,
        cudnnActivationMode_t::CUDNN_ACTIVATION_IDENTITY => {
            miopenActivationMode_t::miopenActivationPASTHRU
        }
        _ => panic!(),
    }
}

unsafe fn activation_forward(
    handle: *mut cudnnContext,
    activation_desc: *mut cudnnActivationStruct,
    alpha: *const std::ffi::c_void,
    x_desc: *mut cudnnTensorStruct,
    x: *const std::ffi::c_void,
    beta: *const std::ffi::c_void,
    y_desc: *mut cudnnTensorStruct,
    y: *mut std::ffi::c_void,
) -> cudnnStatus_t {
    to_cudnn(miopenActivationForward(
        handle as _,
        activation_desc as _,
        alpha,
        x_desc as _,
        x,
        beta,
        y_desc as _,
        y,
    ))
}

unsafe fn set_lrn_descriptor(
    norm_desc: *mut cudnnLRNStruct,
    lrn_n: u32,
    lrn_alpha: f64,
    lrn_beta: f64,
    lrn_k: f64,
) -> cudnnStatus_t {
    to_cudnn(miopenSetLRNDescriptor(
        norm_desc as _,
        miopenLRNMode_t::miopenLRNCrossChannel, // ???
        lrn_n,
        lrn_alpha,
        lrn_beta,
        lrn_k,
    ))
}

unsafe fn lrn_cross_channel_forward(
    handle: *mut cudnnContext,
    norm_desc: *mut cudnnLRNStruct,
    _lrn_mode: cudnnLRNMode_t,
    alpha: *const std::ffi::c_void,
    x_desc: *mut cudnnTensorStruct,
    x: *const std::ffi::c_void,
    beta: *const std::ffi::c_void,
    y_desc: *mut cudnnTensorStruct,
    y: *mut std::ffi::c_void,
) -> cudnnStatus_t {
    to_cudnn(miopenLRNForward(
        handle as _,
        norm_desc as _,
        alpha,
        x_desc as _,
        x,
        beta,
        y_desc as _,
        y,
        false,
        ptr::null_mut(),
    ))
}

unsafe fn softmax_forward(
    handle: *mut cudnnContext,
    algo: cudnnSoftmaxAlgorithm_t,
    mode: cudnnSoftmaxMode_t,
    alpha: *const std::ffi::c_void,
    x_desc: *mut cudnnTensorStruct,
    x: *const std::ffi::c_void,
    beta: *const std::ffi::c_void,
    y_desc: *mut cudnnTensorStruct,
    y: *mut std::ffi::c_void,
) -> cudnnStatus_t {
    let algo = softmax_algo(algo);
    let mode = softmax_mode(mode);
    to_cudnn(miopenSoftmaxForward_V2(
        handle as _,
        alpha,
        x_desc as _,
        x,
        beta,
        y_desc as _,
        y,
        algo,
        mode,
    ))
}

fn softmax_algo(algo: cudnnSoftmaxAlgorithm_t) -> miopenSoftmaxAlgorithm_t {
    match algo {
        cudnnSoftmaxAlgorithm_t::CUDNN_SOFTMAX_ACCURATE => {
            miopenSoftmaxAlgorithm_t::MIOPEN_SOFTMAX_ACCURATE
        }
        cudnnSoftmaxAlgorithm_t::CUDNN_SOFTMAX_FAST => {
            miopenSoftmaxAlgorithm_t::MIOPEN_SOFTMAX_FAST
        }
        cudnnSoftmaxAlgorithm_t::CUDNN_SOFTMAX_LOG => miopenSoftmaxAlgorithm_t::MIOPEN_SOFTMAX_LOG,
        _ => panic!(),
    }
}

fn softmax_mode(mode: cudnnSoftmaxMode_t) -> miopenSoftmaxMode_t {
    match mode {
        cudnnSoftmaxMode_t::CUDNN_SOFTMAX_MODE_CHANNEL => {
            miopenSoftmaxMode_t::MIOPEN_SOFTMAX_MODE_CHANNEL
        }
        cudnnSoftmaxMode_t::CUDNN_SOFTMAX_MODE_INSTANCE => {
            miopenSoftmaxMode_t::MIOPEN_SOFTMAX_MODE_INSTANCE
        }
        _ => panic!(),
    }
}

unsafe fn destroy(handle: *mut cudnnContext) -> cudnnStatus_t {
    to_cudnn(miopenDestroy(handle as _))
}

unsafe fn destroy_activation_descriptor(
    activation_desc: *mut cudnnActivationStruct,
) -> cudnnStatus_t {
    to_cudnn(miopenDestroyActivationDescriptor(activation_desc as _))
}

unsafe fn destroy_convolution_descriptor(conv_desc: *mut cudnnConvolutionStruct) -> cudnnStatus_t {
    to_cudnn(miopenDestroyConvolutionDescriptor(conv_desc as _))
}

unsafe fn destroy_filter_descriptor(filter_desc: *mut cudnnFilterStruct) -> cudnnStatus_t {
    to_cudnn(miopenDestroyTensorDescriptor(filter_desc as _))
}

unsafe fn destroy_lrn_descriptor(lrn_desc: *mut cudnnLRNStruct) -> cudnnStatus_t {
    to_cudnn(miopenDestroyLRNDescriptor(lrn_desc as _))
}

unsafe fn destroy_pooling_descriptor(pooling_desc: *mut cudnnPoolingStruct) -> cudnnStatus_t {
    to_cudnn(miopenDestroyPoolingDescriptor(pooling_desc as _))
}

unsafe fn destroy_tensor_descriptor(tensor_desc: *mut cudnnTensorStruct) -> cudnnStatus_t {
    to_cudnn(miopenDestroyTensorDescriptor(tensor_desc as _))
}

unsafe fn set_tensor_4d_descriptor_ex(
    tensor_desc: *mut cudnnTensorStruct,
    data_type: cudnnDataType_t,
    n: i32,
    c: i32,
    h: i32,
    w: i32,
    n_stride: i32,
    c_stride: i32,
    h_stride: i32,
    w_stride: i32,
) -> cudnnStatus_t {
    let data_type = from_data_type(data_type);
    to_cudnn(miopenSet4dTensorDescriptorEx(
        tensor_desc as _,
        data_type,
        n,
        c,
        h,
        w,
        n_stride,
        c_stride,
        h_stride,
        w_stride,
    ))
}

unsafe fn transform_tensor(
    handle: *mut cudnnContext,
    alpha: *const std::ffi::c_void,
    x_desc: *mut cudnnTensorStruct,
    x: *const std::ffi::c_void,
    beta: *const std::ffi::c_void,
    y_desc: *mut cudnnTensorStruct,
    y: *mut std::ffi::c_void,
) -> cudnnStatus_t {
    to_cudnn(miopenTransformTensor(
        handle as _,
        alpha,
        x_desc as _,
        x,
        beta,
        y_desc as _,
        y,
    ))
}

unsafe fn set_stream(stream_id: *mut CUstream_st) -> cudnnStatus_t {
    if stream_id != ptr::null_mut() {
        todo!()
    }
    cudnnStatus_t::CUDNN_STATUS_SUCCESS
}

fn set_convolution_math_type(
    _conv_desc: cudnnConvolutionDescriptor_t,
    _math_type: cudnnMathType_t,
) -> cudnnStatus_t {
    //TODO: implement
    cudnnStatus_t::CUDNN_STATUS_SUCCESS
}

unsafe fn set_convolution_group_count(
    conv_desc: *mut cudnnConvolutionStruct,
    group_count: i32,
) -> cudnnStatus_t {
    //TODO: implement
    to_cudnn(miopenSetConvolutionGroupCount(conv_desc as _, group_count))
}

unsafe fn get_convolution_backward_data_algorithm_max_count(
    _handle: *mut cudnnContext,
    count: *mut i32,
) -> cudnnStatus_t {
    *count = 1;
    cudnnStatus_t::CUDNN_STATUS_SUCCESS
}

unsafe fn get_convolution_backward_data_algorithm_v7(
    handle: *mut cudnnContext,
    w_desc: *mut cudnnFilterStruct,
    dy_desc: *mut cudnnTensorStruct,
    conv_desc: *mut cudnnConvolutionStruct,
    dx_desc: *mut cudnnTensorStruct,
    requested_algo_count: i32,
    returned_algo_count: *mut i32,
    perf_results: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    memory_limit_in_bytes: usize,
) -> cudnnStatus_t {
    let mut work_space_size = 0;
    let mut dy_size = 0;
    call! { miopenGetTensorNumBytes(dy_desc as _, &mut dy_size) };
    let mut dy = mem::zeroed();
    let error = hipMalloc(&mut dy, dy_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let mut w_size = 0;
    call! { miopenGetTensorNumBytes(w_desc as _, &mut w_size) };
    let mut w = mem::zeroed();
    let error = hipMalloc(&mut w, w_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let mut dx_size = 0;
    call! { miopenGetTensorNumBytes(dx_desc as _, &mut dx_size) };
    let mut dx = mem::zeroed();
    let error = hipMalloc(&mut dx, dx_size);
    if error != hipError_t::hipSuccess {
        panic!("{:?}", error);
    }
    let error = to_cudnn(miopenConvolutionBackwardDataGetWorkSpaceSize(
        handle as _,
        dy_desc as _,
        w_desc as _,
        conv_desc as _,
        dx_desc as _,
        &mut work_space_size,
    ));
    work_space_size = work_space_size.min(memory_limit_in_bytes);
    if error != cudnnStatus_t::CUDNN_STATUS_SUCCESS {
        panic!("")
    }
    let mut work_space = mem::zeroed();
    if hipMalloc(&mut work_space, work_space_size) != hipError_t::hipSuccess {
        panic!("")
    }
    let mut miopen_perf_results = vec![mem::zeroed(); requested_algo_count as usize];
    let result = to_cudnn(miopenFindConvolutionBackwardDataAlgorithm(
        handle as _,
        dy_desc as _,
        dy,
        w_desc as _,
        w,
        conv_desc as _,
        dx_desc as _,
        dx,
        requested_algo_count,
        returned_algo_count,
        miopen_perf_results.as_mut_ptr(),
        work_space,
        work_space_size,
        true,
    ));
    drop(hipFree(dy));
    drop(hipFree(w));
    drop(hipFree(dx));
    drop(hipFree(work_space));
    for i in 0..*returned_algo_count {
        *perf_results.add(i as usize) = convert_bwd_algo(miopen_perf_results[i as usize]);
    }
    result
}

unsafe fn convert_bwd_algo(result: miopenConvAlgoPerf_t) -> cudnnConvolutionBwdDataAlgoPerf_t {
    let algo = bwd_data_algo_to_cudnn(result.__bindgen_anon_1.bwd_data_algo);
    cudnnConvolutionBwdDataAlgoPerf_t {
        algo,
        status: cudnnStatus_t::CUDNN_STATUS_SUCCESS,
        time: result.time,
        memory: result.memory,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: mem::zeroed(),
    }
}

fn bwd_data_algo_to_cudnn(algo: miopenConvBwdDataAlgorithm_t) -> cudnnConvolutionBwdDataAlgo_t {
    match algo {
        miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoGEMM => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0
        }
        miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoDirect => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_1
        }
        miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoWinograd => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD
        }
        miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoFFT => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT
        }
        miopenConvBwdDataAlgorithm_t::miopenTransposeBwdDataAlgoGEMM => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0
        }
        miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoImplicitGEMM => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0
        }
        _ => panic!(),
    }
}

fn bwd_data_algo_from_cudnn(algo: cudnnConvolutionBwdDataAlgo_t) -> miopenConvBwdDataAlgorithm_t {
    match algo {
        cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0 => {
            miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoGEMM
        }
        cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_1 => {
            miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoDirect
        }
        cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT => {
            miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoFFT
        }
        cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT_TILING => {
            miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoFFT
        }
        cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD => {
            miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoWinograd
        }
        cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD_NONFUSED => {
            miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoWinograd
        }
        _ => panic!(),
    }
}

unsafe fn get_convolution_backward_data_algorithm(
    handle: *mut cudnnContext,
    w_desc: *mut cudnnFilterStruct,
    dy_desc: *mut cudnnTensorStruct,
    conv_desc: *mut cudnnConvolutionStruct,
    dx_desc: *mut cudnnTensorStruct,
    memory_limit_in_bytes: usize,
    algo: *mut cudnnConvolutionBwdDataAlgo_t,
) -> cudnnStatus_t {
    let mut algo_count = 0;
    let mut perf_result = mem::zeroed::<cudnnConvolutionBwdDataAlgoPerf_t>();
    let error = get_convolution_backward_data_algorithm_v7(
        handle,
        w_desc,
        dy_desc,
        conv_desc,
        dx_desc,
        1,
        &mut algo_count,
        &mut perf_result as *mut _,
        memory_limit_in_bytes,
    );
    if error != cudnnStatus_t::CUDNN_STATUS_SUCCESS || algo_count == 0 {
        panic!("")
    }
    *algo = perf_result.algo;
    cudnnStatus_t::CUDNN_STATUS_SUCCESS
}

unsafe fn get_convolution_backward_data_workspace_size(
    handle: *mut cudnnContext,
    w_desc: *mut cudnnFilterStruct,
    dy_desc: *mut cudnnTensorStruct,
    conv_desc: *mut cudnnConvolutionStruct,
    dx_desc: *mut cudnnTensorStruct,
    _algo: cudnnConvolutionBwdDataAlgo_t,
    size_in_bytes: *mut usize,
) -> cudnnStatus_t {
    to_cudnn(miopenConvolutionBackwardDataGetWorkSpaceSize(
        handle as _,
        dy_desc as _,
        w_desc as _,
        conv_desc as _,
        dx_desc as _,
        size_in_bytes,
    ))
}

unsafe fn convolution_backward_data(
    handle: *mut cudnnContext,
    alpha: *const std::ffi::c_void,
    w_desc: *mut cudnnFilterStruct,
    w: *const std::ffi::c_void,
    dy_desc: *mut cudnnTensorStruct,
    dy: *const std::ffi::c_void,
    conv_desc: *mut cudnnConvolutionStruct,
    algo: cudnnConvolutionBwdDataAlgo_t,
    work_space: *mut std::ffi::c_void,
    work_space_size_in_bytes: usize,
    beta: *const std::ffi::c_void,
    dx_desc: *mut cudnnTensorStruct,
    dx: *mut std::ffi::c_void,
) -> cudnnStatus_t {
    let algo = bwd_data_algo_from_cudnn(algo);
    to_cudnn(miopenConvolutionBackwardData(
        handle as _,
        alpha,
        dy_desc as _,
        dy,
        w_desc as _,
        w,
        conv_desc as _,
        algo,
        beta,
        dx_desc as _,
        dx,
        work_space,
        work_space_size_in_bytes,
    ))
}
