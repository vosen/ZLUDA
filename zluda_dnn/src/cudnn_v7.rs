use crate::types::*;

/* automatically generated by rust-bindgen 0.66.1 */

#[no_mangle]
pub unsafe extern "system" fn cudnnGetConvolutionForwardAlgorithm(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    wDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    preference: cudnnConvolutionFwdPreference_t,
    memoryLimitInBytes: usize,
    algo: *mut cudnnConvolutionFwdAlgo_t,
) -> cudnnStatus_t {
    crate::get_convolution_forward_algorithm(
        handle,
        xDesc,
        wDesc,
        convDesc,
        yDesc,
        memoryLimitInBytes,
        algo,
    )
}

#[no_mangle]
pub unsafe extern "system" fn cudnnGetConvolutionBackwardFilterAlgorithm(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dwDesc: cudnnFilterDescriptor_t,
    preference: cudnnConvolutionBwdFilterPreference_t,
    memoryLimitInBytes: usize,
    algo: *mut cudnnConvolutionBwdFilterAlgo_t,
) -> cudnnStatus_t {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cudnnGetConvolutionBackwardDataAlgorithm(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    preference: cudnnConvolutionBwdDataPreference_t,
    memoryLimitInBytes: usize,
    algo: *mut cudnnConvolutionBwdDataAlgo_t,
) -> cudnnStatus_t {
    crate::get_convolution_backward_data_algorithm(
        handle,
        wDesc,
        dyDesc,
        convDesc,
        dxDesc,
        memoryLimitInBytes,
        algo,
    )
}

#[no_mangle]
pub unsafe extern "system" fn cudnnSetRNNDescriptor(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    hiddenSize: ::std::os::raw::c_int,
    numLayers: ::std::os::raw::c_int,
    dropoutDesc: cudnnDropoutDescriptor_t,
    inputMode: cudnnRNNInputMode_t,
    direction: cudnnDirectionMode_t,
    mode: cudnnRNNMode_t,
    algo: cudnnRNNAlgo_t,
    mathPrec: cudnnDataType_t,
) -> cudnnStatus_t {
    crate::unsupported()
}

#[no_mangle]
pub unsafe extern "system" fn cudnnSetRNNDescriptor_v5(
    rnnDesc: cudnnRNNDescriptor_t,
    hiddenSize: ::std::os::raw::c_int,
    numLayers: ::std::os::raw::c_int,
    dropoutDesc: cudnnDropoutDescriptor_t,
    inputMode: cudnnRNNInputMode_t,
    direction: cudnnDirectionMode_t,
    mode: cudnnRNNMode_t,
    mathPrec: cudnnDataType_t,
) -> cudnnStatus_t {
    crate::unsupported()
}
