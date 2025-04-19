// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub const CUDNN_MAJOR: u32 = 8;
pub const CUDNN_MINOR: u32 = 9;
pub const CUDNN_PATCHLEVEL: u32 = 7;
pub const CUDNN_VERSION: u32 = 8907;
pub const CUDNN_MAX_SM_MAJOR_NUMBER: u32 = 9;
pub const CUDNN_MAX_SM_MINOR_NUMBER: u32 = 0;
pub const CUDNN_MAX_DEVICE_VERSION: u32 = 900;
pub const CUDNN_SM_50: u32 = 500;
pub const CUDNN_SM_52: u32 = 520;
pub const CUDNN_SM_53: u32 = 530;
pub const CUDNN_SM_60: u32 = 600;
pub const CUDNN_SM_61: u32 = 610;
pub const CUDNN_SM_62: u32 = 620;
pub const CUDNN_SM_70: u32 = 700;
pub const CUDNN_SM_72: u32 = 720;
pub const CUDNN_SM_75: u32 = 750;
pub const CUDNN_SM_80: u32 = 800;
pub const CUDNN_SM_86: u32 = 860;
pub const CUDNN_SM_87: u32 = 870;
pub const CUDNN_SM_89: u32 = 890;
pub const CUDNN_SM_90: u32 = 900;
pub const CUDNN_SM_9X_END: u32 = 999;
pub const CUDNN_MIN_DEVICE_VERSION: u32 = 500;
pub const CUDNN_OPS_INFER_MAJOR: u32 = 8;
pub const CUDNN_OPS_INFER_MINOR: u32 = 9;
pub const CUDNN_OPS_INFER_PATCH: u32 = 7;
pub const CUDNN_DIM_MAX: u32 = 8;
pub const CUDNN_LRN_MIN_N: u32 = 1;
pub const CUDNN_LRN_MAX_N: u32 = 16;
pub const CUDNN_LRN_MIN_K: f64 = 0.00001;
pub const CUDNN_LRN_MIN_BETA: f64 = 0.01;
pub const CUDNN_BN_MIN_EPSILON: f64 = 0.0;
pub const CUDNN_OPS_TRAIN_MAJOR: u32 = 8;
pub const CUDNN_OPS_TRAIN_MINOR: u32 = 9;
pub const CUDNN_OPS_TRAIN_PATCH: u32 = 7;
pub const CUDNN_ADV_INFER_MAJOR: u32 = 8;
pub const CUDNN_ADV_INFER_MINOR: u32 = 9;
pub const CUDNN_ADV_INFER_PATCH: u32 = 7;
pub const CUDNN_RNN_PADDED_IO_DISABLED: u32 = 0;
pub const CUDNN_RNN_PADDED_IO_ENABLED: u32 = 1;
pub const CUDNN_SEQDATA_DIM_COUNT: u32 = 4;
pub const CUDNN_ATTN_QUERYMAP_ALL_TO_ONE: u32 = 0;
pub const CUDNN_ATTN_QUERYMAP_ONE_TO_ONE: u32 = 1;
pub const CUDNN_ATTN_DISABLE_PROJ_BIASES: u32 = 0;
pub const CUDNN_ATTN_ENABLE_PROJ_BIASES: u32 = 2;
pub const CUDNN_ATTN_WKIND_COUNT: u32 = 8;
pub const CUDNN_ADV_TRAIN_MAJOR: u32 = 8;
pub const CUDNN_ADV_TRAIN_MINOR: u32 = 9;
pub const CUDNN_ADV_TRAIN_PATCH: u32 = 7;
pub const CUDNN_CNN_INFER_MAJOR: u32 = 8;
pub const CUDNN_CNN_INFER_MINOR: u32 = 9;
pub const CUDNN_CNN_INFER_PATCH: u32 = 7;
pub const CUDNN_CNN_TRAIN_MAJOR: u32 = 8;
pub const CUDNN_CNN_TRAIN_MINOR: u32 = 9;
pub const CUDNN_CNN_TRAIN_PATCH: u32 = 7;
pub use super::cudnn::cudnnContext;
pub type cudnnHandle_t = *mut cudnnContext;
impl cudnnStatus_t {
    pub const CUDNN_STATUS_SUCCESS: cudnnStatus_t = cudnnStatus_t(0);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_NOT_INITIALIZED: cudnnStatus_t = cudnnStatus_t(1);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_ALLOC_FAILED: cudnnStatus_t = cudnnStatus_t(2);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_BAD_PARAM: cudnnStatus_t = cudnnStatus_t(3);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_INTERNAL_ERROR: cudnnStatus_t = cudnnStatus_t(4);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_INVALID_VALUE: cudnnStatus_t = cudnnStatus_t(5);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_ARCH_MISMATCH: cudnnStatus_t = cudnnStatus_t(6);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_MAPPING_ERROR: cudnnStatus_t = cudnnStatus_t(7);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_EXECUTION_FAILED: cudnnStatus_t = cudnnStatus_t(8);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_NOT_SUPPORTED: cudnnStatus_t = cudnnStatus_t(9);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_LICENSE_ERROR: cudnnStatus_t = cudnnStatus_t(10);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_RUNTIME_PREREQUISITE_MISSING: cudnnStatus_t = cudnnStatus_t(
        11,
    );
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_RUNTIME_IN_PROGRESS: cudnnStatus_t = cudnnStatus_t(12);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_RUNTIME_FP_OVERFLOW: cudnnStatus_t = cudnnStatus_t(13);
}
impl cudnnStatus_t {
    pub const CUDNN_STATUS_VERSION_MISMATCH: cudnnStatus_t = cudnnStatus_t(14);
}
#[repr(transparent)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnStatus_t(pub ::core::ffi::c_uint);
pub use super::cudnn::cudnnRuntimeTag_t;
pub use super::cudnn::cudnnErrQueryMode_t;
pub use super::cudnn::cudnnTensorStruct;
pub type cudnnTensorDescriptor_t = *mut cudnnTensorStruct;
pub use super::cudnn::cudnnPoolingStruct;
pub type cudnnPoolingDescriptor_t = *mut cudnnPoolingStruct;
pub use super::cudnn::cudnnFilterStruct;
pub type cudnnFilterDescriptor_t = *mut cudnnFilterStruct;
pub use super::cudnn::cudnnLRNStruct;
pub type cudnnLRNDescriptor_t = *mut cudnnLRNStruct;
pub use super::cudnn::cudnnActivationStruct;
pub type cudnnActivationDescriptor_t = *mut cudnnActivationStruct;
pub use super::cudnn::cudnnSpatialTransformerStruct;
pub type cudnnSpatialTransformerDescriptor_t = *mut cudnnSpatialTransformerStruct;
pub use super::cudnn::cudnnOpTensorStruct;
pub type cudnnOpTensorDescriptor_t = *mut cudnnOpTensorStruct;
pub use super::cudnn::cudnnReduceTensorStruct;
pub type cudnnReduceTensorDescriptor_t = *mut cudnnReduceTensorStruct;
pub use super::cudnn::cudnnCTCLossStruct;
pub type cudnnCTCLossDescriptor_t = *mut cudnnCTCLossStruct;
pub use super::cudnn::cudnnTensorTransformStruct;
pub type cudnnTensorTransformDescriptor_t = *mut cudnnTensorTransformStruct;
pub use super::cudnn9::cudnnDataType_t;
pub use super::cudnn::cudnnMathType_t;
pub use super::cudnn::cudnnNanPropagation_t;
pub use super::cudnn::cudnnDeterminism_t;
pub use super::cudnn::cudnnTensorFormat_t;
pub use super::cudnn::cudnnFoldingDirection_t;
pub use super::cudnn::cudnnOpTensorOp_t;
pub use super::cudnn::cudnnReduceTensorOp_t;
pub use super::cudnn::cudnnReduceTensorIndices_t;
pub use super::cudnn::cudnnIndicesType_t;
pub use super::cudnn::cudnnSoftmaxAlgorithm_t;
pub use super::cudnn::cudnnSoftmaxMode_t;
pub use super::cudnn::cudnnPoolingMode_t;
pub use super::cudnn::cudnnActivationMode_t;
pub use super::cudnn::cudnnLRNMode_t;
pub use super::cudnn::cudnnDivNormMode_t;
pub use super::cudnn::cudnnBatchNormMode_t;
pub use super::cudnn::cudnnBatchNormOps_t;
pub use super::cudnn::cudnnNormMode_t;
pub use super::cudnn::cudnnNormAlgo_t;
pub use super::cudnn::cudnnNormOps_t;
pub use super::cudnn::cudnnSamplerType_t;
pub use super::cudnn::cudnnDropoutStruct;
pub type cudnnDropoutDescriptor_t = *mut cudnnDropoutStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnAlgorithmStruct {
    _unused: [u8; 0],
}
pub type cudnnAlgorithmDescriptor_t = *mut cudnnAlgorithmStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnAlgorithmPerformanceStruct {
    _unused: [u8; 0],
}
pub type cudnnAlgorithmPerformance_t = *mut cudnnAlgorithmPerformanceStruct;
pub use super::cudnn::cudnnConvolutionFwdAlgo_t;
pub use super::cudnn::cudnnConvolutionBwdFilterAlgo_t;
pub use super::cudnn::cudnnConvolutionBwdDataAlgo_t;
pub use super::cudnn::cudnnRNNAlgo_t;
pub use super::cudnn::cudnnCTCLossAlgo_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudnnAlgorithmUnionStruct {
    pub algo: cudnnAlgorithmUnionStruct_Algorithm,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudnnAlgorithmUnionStruct_Algorithm {
    pub convFwdAlgo: cudnnConvolutionFwdAlgo_t,
    pub convBwdFilterAlgo: cudnnConvolutionBwdFilterAlgo_t,
    pub convBwdDataAlgo: cudnnConvolutionBwdDataAlgo_t,
    pub RNNAlgo: cudnnRNNAlgo_t,
    pub CTCLossAlgo: cudnnCTCLossAlgo_t,
}
pub type cudnnAlgorithm_t = cudnnAlgorithmUnionStruct;
pub use super::cudnn::cudnnSeverity_t;
#[repr(C)]
pub struct cudnnDebugStruct {
    pub cudnn_version: ::core::ffi::c_uint,
    pub cudnnStatus: cudnnStatus_t,
    pub time_sec: ::core::ffi::c_uint,
    pub time_usec: ::core::ffi::c_uint,
    pub time_delta: ::core::ffi::c_uint,
    pub handle: cudnnHandle_t,
    pub stream: super::cuda::CUstream,
    pub pid: ::core::ffi::c_ulonglong,
    pub tid: ::core::ffi::c_ulonglong,
    pub cudaDeviceId: ::core::ffi::c_int,
    pub reserved: [::core::ffi::c_int; 15usize],
}
pub type cudnnDebug_t = cudnnDebugStruct;
pub type cudnnCallback_t = ::core::option::Option<
    unsafe extern "C" fn(
        sev: cudnnSeverity_t,
        udata: *mut ::core::ffi::c_void,
        dbg: *const cudnnDebug_t,
        msg: *const ::core::ffi::c_char,
    ),
>;
pub use super::cudnn::cudnnForwardMode_t;
pub use super::cudnn::cudnnRNNMode_t;
pub use super::cudnn::cudnnRNNBiasMode_t;
pub use super::cudnn::cudnnDirectionMode_t;
pub use super::cudnn::cudnnRNNInputMode_t;
pub use super::cudnn::cudnnRNNClipMode_t;
pub use super::cudnn::cudnnRNNDataLayout_t;
pub type cudnnRNNPaddingMode_t = ::core::ffi::c_uint;
pub use super::cudnn::cudnnRNNStruct;
pub type cudnnRNNDescriptor_t = *mut cudnnRNNStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnPersistentRNNPlan {
    _unused: [u8; 0],
}
pub type cudnnPersistentRNNPlan_t = *mut cudnnPersistentRNNPlan;
pub use super::cudnn::cudnnRNNDataStruct;
pub type cudnnRNNDataDescriptor_t = *mut cudnnRNNDataStruct;
pub use super::cudnn::cudnnSeqDataAxis_t;
pub use super::cudnn::cudnnSeqDataStruct;
pub type cudnnSeqDataDescriptor_t = *mut cudnnSeqDataStruct;
pub type cudnnAttnQueryMap_t = ::core::ffi::c_uint;
pub use super::cudnn::cudnnAttnStruct;
pub type cudnnAttnDescriptor_t = *mut cudnnAttnStruct;
pub use super::cudnn::cudnnMultiHeadAttnWeightKind_t;
pub use super::cudnn::cudnnWgradMode_t;
pub use super::cudnn::cudnnLossNormalizationMode_t;
pub use super::cudnn::cudnnConvolutionStruct;
pub type cudnnConvolutionDescriptor_t = *mut cudnnConvolutionStruct;
pub use super::cudnn::cudnnConvolutionMode_t;
pub use super::cudnn::cudnnReorderType_t;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct cudnnConvolutionFwdAlgoPerfStruct {
    pub algo: cudnnConvolutionFwdAlgo_t,
    pub status: cudnnStatus_t,
    pub time: f32,
    pub memory: usize,
    pub determinism: cudnnDeterminism_t,
    pub mathType: cudnnMathType_t,
    pub reserved: [::core::ffi::c_int; 3usize],
}
pub type cudnnConvolutionFwdAlgoPerf_t = cudnnConvolutionFwdAlgoPerfStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct cudnnConvolutionBwdDataAlgoPerfStruct {
    pub algo: cudnnConvolutionBwdDataAlgo_t,
    pub status: cudnnStatus_t,
    pub time: f32,
    pub memory: usize,
    pub determinism: cudnnDeterminism_t,
    pub mathType: cudnnMathType_t,
    pub reserved: [::core::ffi::c_int; 3usize],
}
pub type cudnnConvolutionBwdDataAlgoPerf_t = cudnnConvolutionBwdDataAlgoPerfStruct;
pub use super::cudnn::cudnnFusedOpsConstParamStruct;
pub type cudnnFusedOpsConstParamPack_t = *mut cudnnFusedOpsConstParamStruct;
pub use super::cudnn::cudnnFusedOpsVariantParamStruct;
pub type cudnnFusedOpsVariantParamPack_t = *mut cudnnFusedOpsVariantParamStruct;
pub use super::cudnn::cudnnFusedOpsPlanStruct;
pub type cudnnFusedOpsPlan_t = *mut cudnnFusedOpsPlanStruct;
pub use super::cudnn::cudnnFusedOps_t;
pub use super::cudnn::cudnnFusedOpsConstParamLabel_t;
pub use super::cudnn::cudnnFusedOpsPointerPlaceHolder_t;
pub use super::cudnn::cudnnFusedOpsVariantParamLabel_t;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct cudnnConvolutionBwdFilterAlgoPerfStruct {
    pub algo: cudnnConvolutionBwdFilterAlgo_t,
    pub status: cudnnStatus_t,
    pub time: f32,
    pub memory: usize,
    pub determinism: cudnnDeterminism_t,
    pub mathType: cudnnMathType_t,
    pub reserved: [::core::ffi::c_int; 3usize],
}
pub type cudnnConvolutionBwdFilterAlgoPerf_t = cudnnConvolutionBwdFilterAlgoPerfStruct;
pub type cudnnBackendDescriptor_t = *mut ::core::ffi::c_void;
pub use super::cudnn::cudnnFractionStruct;
pub type cudnnFraction_t = cudnnFractionStruct;
pub use super::cudnn9::cudnnPointwiseMode_t;
pub use super::cudnn::cudnnResampleMode_t;
pub use super::cudnn::cudnnSignalMode_t;
pub use super::cudnn::cudnnGenStatsMode_t;
pub use super::cudnn::cudnnBnFinalizeStatsMode_t;
pub use super::cudnn::cudnnRngDistribution_t;
pub use super::cudnn9::cudnnBackendAttributeName_t;
pub use super::cudnn::cudnnBackendAttributeType_t;
pub use super::cudnn9::cudnnBackendDescriptorType_t;
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_TENSOR_CORE: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        0,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_DOWN_CONVERT_INPUTS: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        1,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_REDUCED_PRECISION_REDUCTION: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        2,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_FFT: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        3,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_NONDETERMINISTIC: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        4,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_WINOGRAD: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        5,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_4x4: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        6,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_6x6: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        7,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_13x13: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        8,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_TYPE_COUNT: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        9,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendNumericalNote_t(pub ::core::ffi::c_uint);
impl cudnnBackendBehaviorNote_t {
    pub const CUDNN_BEHAVIOR_NOTE_RUNTIME_COMPILATION: cudnnBackendBehaviorNote_t = cudnnBackendBehaviorNote_t(
        0,
    );
}
impl cudnnBackendBehaviorNote_t {
    pub const CUDNN_BEHAVIOR_NOTE_REQUIRES_FILTER_INT8x32_REORDER: cudnnBackendBehaviorNote_t = cudnnBackendBehaviorNote_t(
        1,
    );
}
impl cudnnBackendBehaviorNote_t {
    pub const CUDNN_BEHAVIOR_NOTE_REQUIRES_BIAS_INT8x32_REORDER: cudnnBackendBehaviorNote_t = cudnnBackendBehaviorNote_t(
        2,
    );
}
impl cudnnBackendBehaviorNote_t {
    pub const CUDNN_BEHAVIOR_NOTE_TYPE_COUNT: cudnnBackendBehaviorNote_t = cudnnBackendBehaviorNote_t(
        3,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendBehaviorNote_t(pub ::core::ffi::c_uint);
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SPLIT_K: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        0,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SWIZZLE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        1,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_TILE_SIZE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        2,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_USE_TEX: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        3,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_EDGE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(4);
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_KBLOCK: cudnnBackendKnobType_t = cudnnBackendKnobType_t(5);
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_LDGA: cudnnBackendKnobType_t = cudnnBackendKnobType_t(6);
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_LDGB: cudnnBackendKnobType_t = cudnnBackendKnobType_t(7);
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_CHUNK_K: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        8,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SPLIT_H: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        9,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_WINO_TILE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        10,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_MULTIPLY: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        11,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SPLIT_K_BUF: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        12,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_TILEK: cudnnBackendKnobType_t = cudnnBackendKnobType_t(13);
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_STAGES: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        14,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_REDUCTION_MODE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        15,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_CTA_SPLIT_K_MODE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        16,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SPLIT_K_SLC: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        17,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_IDX_MODE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        18,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SLICED: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        19,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SPLIT_RS: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        20,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SINGLEBUFFER: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        21,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_LDGC: cudnnBackendKnobType_t = cudnnBackendKnobType_t(22);
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SPECFILT: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        23,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_KERNEL_CFG: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        24,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_WORKSPACE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        25,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_TILE_CGA: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        26,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_TILE_CGA_M: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        27,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_TILE_CGA_N: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        28,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_BLOCK_SIZE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        29,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_OCCUPANCY: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        30,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_ARRAY_SIZE_PER_THREAD: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        31,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_NUM_C_PER_BLOCK: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        32,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SPLIT_COLS: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        33,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_TILE_ROWS: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        34,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_TILE_COLS: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        35,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_LOAD_SIZE: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        36,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_COUNTS: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        37,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendKnobType_t(pub ::core::ffi::c_uint);
pub use super::cudnn::cudnnBackendLayoutType_t;
pub use super::cudnn::cudnnBackendHeurMode_t;
pub use super::cudnn9::cudnnBackendTensorReordering_t;
pub use super::cudnn::cudnnPaddingMode_t;
pub use super::cudnn9::cudnnBackendNormMode_t;
pub use super::cudnn::cudnnBackendNormFwdPhase_t;
