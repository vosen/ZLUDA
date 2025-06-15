// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnContext {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnRuntimeTag_t {
    _unused: [u8; 0],
}
impl cudnnErrQueryMode_t {
    pub const CUDNN_ERRQUERY_RAWCODE: cudnnErrQueryMode_t = cudnnErrQueryMode_t(0);
}
impl cudnnErrQueryMode_t {
    pub const CUDNN_ERRQUERY_NONBLOCKING: cudnnErrQueryMode_t = cudnnErrQueryMode_t(1);
}
impl cudnnErrQueryMode_t {
    pub const CUDNN_ERRQUERY_BLOCKING: cudnnErrQueryMode_t = cudnnErrQueryMode_t(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnErrQueryMode_t(pub ::core::ffi::c_uint);
impl cudnnMathType_t {
    pub const CUDNN_DEFAULT_MATH: cudnnMathType_t = cudnnMathType_t(0);
}
impl cudnnMathType_t {
    pub const CUDNN_TENSOR_OP_MATH: cudnnMathType_t = cudnnMathType_t(1);
}
impl cudnnMathType_t {
    pub const CUDNN_TENSOR_OP_MATH_ALLOW_CONVERSION: cudnnMathType_t = cudnnMathType_t(
        2,
    );
}
impl cudnnMathType_t {
    pub const CUDNN_FMA_MATH: cudnnMathType_t = cudnnMathType_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnMathType_t(pub ::core::ffi::c_uint);
impl cudnnNanPropagation_t {
    pub const CUDNN_NOT_PROPAGATE_NAN: cudnnNanPropagation_t = cudnnNanPropagation_t(0);
}
impl cudnnNanPropagation_t {
    pub const CUDNN_PROPAGATE_NAN: cudnnNanPropagation_t = cudnnNanPropagation_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnNanPropagation_t(pub ::core::ffi::c_uint);
impl cudnnTensorFormat_t {
    pub const CUDNN_TENSOR_NCHW: cudnnTensorFormat_t = cudnnTensorFormat_t(0);
}
impl cudnnTensorFormat_t {
    pub const CUDNN_TENSOR_NHWC: cudnnTensorFormat_t = cudnnTensorFormat_t(1);
}
impl cudnnTensorFormat_t {
    pub const CUDNN_TENSOR_NCHW_VECT_C: cudnnTensorFormat_t = cudnnTensorFormat_t(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnTensorFormat_t(pub ::core::ffi::c_uint);
impl cudnnReduceTensorOp_t {
    pub const CUDNN_REDUCE_TENSOR_ADD: cudnnReduceTensorOp_t = cudnnReduceTensorOp_t(0);
}
impl cudnnReduceTensorOp_t {
    pub const CUDNN_REDUCE_TENSOR_MUL: cudnnReduceTensorOp_t = cudnnReduceTensorOp_t(1);
}
impl cudnnReduceTensorOp_t {
    pub const CUDNN_REDUCE_TENSOR_MIN: cudnnReduceTensorOp_t = cudnnReduceTensorOp_t(2);
}
impl cudnnReduceTensorOp_t {
    pub const CUDNN_REDUCE_TENSOR_MAX: cudnnReduceTensorOp_t = cudnnReduceTensorOp_t(3);
}
impl cudnnReduceTensorOp_t {
    pub const CUDNN_REDUCE_TENSOR_AMAX: cudnnReduceTensorOp_t = cudnnReduceTensorOp_t(4);
}
impl cudnnReduceTensorOp_t {
    pub const CUDNN_REDUCE_TENSOR_AVG: cudnnReduceTensorOp_t = cudnnReduceTensorOp_t(5);
}
impl cudnnReduceTensorOp_t {
    pub const CUDNN_REDUCE_TENSOR_NORM1: cudnnReduceTensorOp_t = cudnnReduceTensorOp_t(
        6,
    );
}
impl cudnnReduceTensorOp_t {
    pub const CUDNN_REDUCE_TENSOR_NORM2: cudnnReduceTensorOp_t = cudnnReduceTensorOp_t(
        7,
    );
}
impl cudnnReduceTensorOp_t {
    pub const CUDNN_REDUCE_TENSOR_MUL_NO_ZEROS: cudnnReduceTensorOp_t = cudnnReduceTensorOp_t(
        8,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnReduceTensorOp_t(pub ::core::ffi::c_uint);
impl cudnnActivationMode_t {
    pub const CUDNN_ACTIVATION_SIGMOID: cudnnActivationMode_t = cudnnActivationMode_t(0);
}
impl cudnnActivationMode_t {
    pub const CUDNN_ACTIVATION_RELU: cudnnActivationMode_t = cudnnActivationMode_t(1);
}
impl cudnnActivationMode_t {
    pub const CUDNN_ACTIVATION_TANH: cudnnActivationMode_t = cudnnActivationMode_t(2);
}
impl cudnnActivationMode_t {
    pub const CUDNN_ACTIVATION_CLIPPED_RELU: cudnnActivationMode_t = cudnnActivationMode_t(
        3,
    );
}
impl cudnnActivationMode_t {
    pub const CUDNN_ACTIVATION_ELU: cudnnActivationMode_t = cudnnActivationMode_t(4);
}
impl cudnnActivationMode_t {
    pub const CUDNN_ACTIVATION_IDENTITY: cudnnActivationMode_t = cudnnActivationMode_t(
        5,
    );
}
impl cudnnActivationMode_t {
    pub const CUDNN_ACTIVATION_SWISH: cudnnActivationMode_t = cudnnActivationMode_t(6);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnActivationMode_t(pub ::core::ffi::c_uint);
impl cudnnSeverity_t {
    pub const CUDNN_SEV_FATAL: cudnnSeverity_t = cudnnSeverity_t(0);
}
impl cudnnSeverity_t {
    pub const CUDNN_SEV_ERROR: cudnnSeverity_t = cudnnSeverity_t(1);
}
impl cudnnSeverity_t {
    pub const CUDNN_SEV_WARNING: cudnnSeverity_t = cudnnSeverity_t(2);
}
impl cudnnSeverity_t {
    pub const CUDNN_SEV_INFO: cudnnSeverity_t = cudnnSeverity_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnSeverity_t(pub ::core::ffi::c_uint);
impl cudnnConvolutionMode_t {
    pub const CUDNN_CONVOLUTION: cudnnConvolutionMode_t = cudnnConvolutionMode_t(0);
}
impl cudnnConvolutionMode_t {
    pub const CUDNN_CROSS_CORRELATION: cudnnConvolutionMode_t = cudnnConvolutionMode_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnConvolutionMode_t(pub ::core::ffi::c_uint);
impl cudnnReorderType_t {
    pub const CUDNN_DEFAULT_REORDER: cudnnReorderType_t = cudnnReorderType_t(0);
}
impl cudnnReorderType_t {
    pub const CUDNN_NO_REORDER: cudnnReorderType_t = cudnnReorderType_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnReorderType_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnFractionStruct {
    pub numerator: i64,
    pub denominator: i64,
}
impl cudnnResampleMode_t {
    pub const CUDNN_RESAMPLE_NEAREST: cudnnResampleMode_t = cudnnResampleMode_t(0);
}
impl cudnnResampleMode_t {
    pub const CUDNN_RESAMPLE_BILINEAR: cudnnResampleMode_t = cudnnResampleMode_t(1);
}
impl cudnnResampleMode_t {
    pub const CUDNN_RESAMPLE_AVGPOOL: cudnnResampleMode_t = cudnnResampleMode_t(2);
}
impl cudnnResampleMode_t {
    pub const CUDNN_RESAMPLE_AVGPOOL_INCLUDE_PADDING: cudnnResampleMode_t = cudnnResampleMode_t(
        2,
    );
}
impl cudnnResampleMode_t {
    pub const CUDNN_RESAMPLE_AVGPOOL_EXCLUDE_PADDING: cudnnResampleMode_t = cudnnResampleMode_t(
        4,
    );
}
impl cudnnResampleMode_t {
    pub const CUDNN_RESAMPLE_MAXPOOL: cudnnResampleMode_t = cudnnResampleMode_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnResampleMode_t(pub ::core::ffi::c_uint);
impl cudnnSignalMode_t {
    pub const CUDNN_SIGNAL_SET: cudnnSignalMode_t = cudnnSignalMode_t(0);
}
impl cudnnSignalMode_t {
    pub const CUDNN_SIGNAL_WAIT: cudnnSignalMode_t = cudnnSignalMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnSignalMode_t(pub ::core::ffi::c_uint);
impl cudnnGenStatsMode_t {
    pub const CUDNN_GENSTATS_SUM_SQSUM: cudnnGenStatsMode_t = cudnnGenStatsMode_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnGenStatsMode_t(pub ::core::ffi::c_uint);
impl cudnnBnFinalizeStatsMode_t {
    pub const CUDNN_BN_FINALIZE_STATISTICS_TRAINING: cudnnBnFinalizeStatsMode_t = cudnnBnFinalizeStatsMode_t(
        0,
    );
}
impl cudnnBnFinalizeStatsMode_t {
    pub const CUDNN_BN_FINALIZE_STATISTICS_INFERENCE: cudnnBnFinalizeStatsMode_t = cudnnBnFinalizeStatsMode_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBnFinalizeStatsMode_t(pub ::core::ffi::c_uint);
impl cudnnRngDistribution_t {
    pub const CUDNN_RNG_DISTRIBUTION_BERNOULLI: cudnnRngDistribution_t = cudnnRngDistribution_t(
        0,
    );
}
impl cudnnRngDistribution_t {
    pub const CUDNN_RNG_DISTRIBUTION_UNIFORM: cudnnRngDistribution_t = cudnnRngDistribution_t(
        1,
    );
}
impl cudnnRngDistribution_t {
    pub const CUDNN_RNG_DISTRIBUTION_NORMAL: cudnnRngDistribution_t = cudnnRngDistribution_t(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnRngDistribution_t(pub ::core::ffi::c_uint);
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_HANDLE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        0,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_DATA_TYPE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        1,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_BOOLEAN: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        2,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_INT64: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        3,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_FLOAT: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        4,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_DOUBLE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        5,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_VOID_PTR: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        6,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_CONVOLUTION_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        7,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_HEUR_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        8,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_KNOB_TYPE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        9,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_NAN_PROPOGATION: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        10,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_NUMERICAL_NOTE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        11,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_LAYOUT_TYPE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        12,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_ATTRIB_NAME: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        13,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_POINTWISE_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        14,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_BACKEND_DESCRIPTOR: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        15,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_GENSTATS_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        16,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_BN_FINALIZE_STATS_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        17,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_REDUCTION_OPERATOR_TYPE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        18,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_BEHAVIOR_NOTE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        19,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_TENSOR_REORDERING_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        20,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_RESAMPLE_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        21,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_PADDING_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        22,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_INT32: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        23,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_CHAR: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        24,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_SIGNAL_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        25,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_FRACTION: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        26,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_NORM_MODE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        27,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_NORM_FWD_PHASE: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        28,
    );
}
impl cudnnBackendAttributeType_t {
    pub const CUDNN_TYPE_RNG_DISTRIBUTION: cudnnBackendAttributeType_t = cudnnBackendAttributeType_t(
        29,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendAttributeType_t(pub ::core::ffi::c_uint);
impl cudnnBackendLayoutType_t {
    pub const CUDNN_LAYOUT_TYPE_PREFERRED_NCHW: cudnnBackendLayoutType_t = cudnnBackendLayoutType_t(
        0,
    );
}
impl cudnnBackendLayoutType_t {
    pub const CUDNN_LAYOUT_TYPE_PREFERRED_NHWC: cudnnBackendLayoutType_t = cudnnBackendLayoutType_t(
        1,
    );
}
impl cudnnBackendLayoutType_t {
    pub const CUDNN_LAYOUT_TYPE_PREFERRED_PAD4CK: cudnnBackendLayoutType_t = cudnnBackendLayoutType_t(
        2,
    );
}
impl cudnnBackendLayoutType_t {
    pub const CUDNN_LAYOUT_TYPE_PREFERRED_PAD8CK: cudnnBackendLayoutType_t = cudnnBackendLayoutType_t(
        3,
    );
}
impl cudnnBackendLayoutType_t {
    pub const CUDNN_LAYOUT_TYPE_COUNT: cudnnBackendLayoutType_t = cudnnBackendLayoutType_t(
        4,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendLayoutType_t(pub ::core::ffi::c_uint);
impl cudnnBackendHeurMode_t {
    pub const CUDNN_HEUR_MODE_INSTANT: cudnnBackendHeurMode_t = cudnnBackendHeurMode_t(
        0,
    );
}
impl cudnnBackendHeurMode_t {
    pub const CUDNN_HEUR_MODE_B: cudnnBackendHeurMode_t = cudnnBackendHeurMode_t(1);
}
impl cudnnBackendHeurMode_t {
    pub const CUDNN_HEUR_MODE_FALLBACK: cudnnBackendHeurMode_t = cudnnBackendHeurMode_t(
        2,
    );
}
impl cudnnBackendHeurMode_t {
    pub const CUDNN_HEUR_MODE_A: cudnnBackendHeurMode_t = cudnnBackendHeurMode_t(3);
}
impl cudnnBackendHeurMode_t {
    pub const CUDNN_HEUR_MODES_COUNT: cudnnBackendHeurMode_t = cudnnBackendHeurMode_t(4);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendHeurMode_t(pub ::core::ffi::c_uint);
impl cudnnPaddingMode_t {
    pub const CUDNN_ZERO_PAD: cudnnPaddingMode_t = cudnnPaddingMode_t(0);
}
impl cudnnPaddingMode_t {
    pub const CUDNN_NEG_INF_PAD: cudnnPaddingMode_t = cudnnPaddingMode_t(1);
}
impl cudnnPaddingMode_t {
    pub const CUDNN_EDGE_VAL_PAD: cudnnPaddingMode_t = cudnnPaddingMode_t(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnPaddingMode_t(pub ::core::ffi::c_uint);
impl cudnnBackendNormFwdPhase_t {
    pub const CUDNN_NORM_FWD_INFERENCE: cudnnBackendNormFwdPhase_t = cudnnBackendNormFwdPhase_t(
        0,
    );
}
impl cudnnBackendNormFwdPhase_t {
    pub const CUDNN_NORM_FWD_TRAINING: cudnnBackendNormFwdPhase_t = cudnnBackendNormFwdPhase_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendNormFwdPhase_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnTensorStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnPoolingStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnFilterStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnLRNStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnActivationStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnSpatialTransformerStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnOpTensorStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnReduceTensorStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnCTCLossStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnTensorTransformStruct {
    _unused: [u8; 0],
}
impl cudnnDeterminism_t {
    pub const CUDNN_NON_DETERMINISTIC: cudnnDeterminism_t = cudnnDeterminism_t(0);
}
impl cudnnDeterminism_t {
    pub const CUDNN_DETERMINISTIC: cudnnDeterminism_t = cudnnDeterminism_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnDeterminism_t(pub ::core::ffi::c_uint);
impl cudnnFoldingDirection_t {
    pub const CUDNN_TRANSFORM_FOLD: cudnnFoldingDirection_t = cudnnFoldingDirection_t(0);
}
impl cudnnFoldingDirection_t {
    pub const CUDNN_TRANSFORM_UNFOLD: cudnnFoldingDirection_t = cudnnFoldingDirection_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnFoldingDirection_t(pub ::core::ffi::c_uint);
impl cudnnOpTensorOp_t {
    pub const CUDNN_OP_TENSOR_ADD: cudnnOpTensorOp_t = cudnnOpTensorOp_t(0);
}
impl cudnnOpTensorOp_t {
    pub const CUDNN_OP_TENSOR_MUL: cudnnOpTensorOp_t = cudnnOpTensorOp_t(1);
}
impl cudnnOpTensorOp_t {
    pub const CUDNN_OP_TENSOR_MIN: cudnnOpTensorOp_t = cudnnOpTensorOp_t(2);
}
impl cudnnOpTensorOp_t {
    pub const CUDNN_OP_TENSOR_MAX: cudnnOpTensorOp_t = cudnnOpTensorOp_t(3);
}
impl cudnnOpTensorOp_t {
    pub const CUDNN_OP_TENSOR_SQRT: cudnnOpTensorOp_t = cudnnOpTensorOp_t(4);
}
impl cudnnOpTensorOp_t {
    pub const CUDNN_OP_TENSOR_NOT: cudnnOpTensorOp_t = cudnnOpTensorOp_t(5);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnOpTensorOp_t(pub ::core::ffi::c_uint);
impl cudnnReduceTensorIndices_t {
    pub const CUDNN_REDUCE_TENSOR_NO_INDICES: cudnnReduceTensorIndices_t = cudnnReduceTensorIndices_t(
        0,
    );
}
impl cudnnReduceTensorIndices_t {
    pub const CUDNN_REDUCE_TENSOR_FLATTENED_INDICES: cudnnReduceTensorIndices_t = cudnnReduceTensorIndices_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnReduceTensorIndices_t(pub ::core::ffi::c_uint);
impl cudnnIndicesType_t {
    pub const CUDNN_32BIT_INDICES: cudnnIndicesType_t = cudnnIndicesType_t(0);
}
impl cudnnIndicesType_t {
    pub const CUDNN_64BIT_INDICES: cudnnIndicesType_t = cudnnIndicesType_t(1);
}
impl cudnnIndicesType_t {
    pub const CUDNN_16BIT_INDICES: cudnnIndicesType_t = cudnnIndicesType_t(2);
}
impl cudnnIndicesType_t {
    pub const CUDNN_8BIT_INDICES: cudnnIndicesType_t = cudnnIndicesType_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnIndicesType_t(pub ::core::ffi::c_uint);
impl cudnnSoftmaxAlgorithm_t {
    pub const CUDNN_SOFTMAX_FAST: cudnnSoftmaxAlgorithm_t = cudnnSoftmaxAlgorithm_t(0);
}
impl cudnnSoftmaxAlgorithm_t {
    pub const CUDNN_SOFTMAX_ACCURATE: cudnnSoftmaxAlgorithm_t = cudnnSoftmaxAlgorithm_t(
        1,
    );
}
impl cudnnSoftmaxAlgorithm_t {
    pub const CUDNN_SOFTMAX_LOG: cudnnSoftmaxAlgorithm_t = cudnnSoftmaxAlgorithm_t(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnSoftmaxAlgorithm_t(pub ::core::ffi::c_uint);
impl cudnnSoftmaxMode_t {
    pub const CUDNN_SOFTMAX_MODE_INSTANCE: cudnnSoftmaxMode_t = cudnnSoftmaxMode_t(0);
}
impl cudnnSoftmaxMode_t {
    pub const CUDNN_SOFTMAX_MODE_CHANNEL: cudnnSoftmaxMode_t = cudnnSoftmaxMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnSoftmaxMode_t(pub ::core::ffi::c_uint);
impl cudnnPoolingMode_t {
    pub const CUDNN_POOLING_MAX: cudnnPoolingMode_t = cudnnPoolingMode_t(0);
}
impl cudnnPoolingMode_t {
    pub const CUDNN_POOLING_AVERAGE_COUNT_INCLUDE_PADDING: cudnnPoolingMode_t = cudnnPoolingMode_t(
        1,
    );
}
impl cudnnPoolingMode_t {
    pub const CUDNN_POOLING_AVERAGE_COUNT_EXCLUDE_PADDING: cudnnPoolingMode_t = cudnnPoolingMode_t(
        2,
    );
}
impl cudnnPoolingMode_t {
    pub const CUDNN_POOLING_MAX_DETERMINISTIC: cudnnPoolingMode_t = cudnnPoolingMode_t(
        3,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnPoolingMode_t(pub ::core::ffi::c_uint);
impl cudnnLRNMode_t {
    pub const CUDNN_LRN_CROSS_CHANNEL_DIM1: cudnnLRNMode_t = cudnnLRNMode_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnLRNMode_t(pub ::core::ffi::c_uint);
impl cudnnDivNormMode_t {
    pub const CUDNN_DIVNORM_PRECOMPUTED_MEANS: cudnnDivNormMode_t = cudnnDivNormMode_t(
        0,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnDivNormMode_t(pub ::core::ffi::c_uint);
impl cudnnBatchNormMode_t {
    pub const CUDNN_BATCHNORM_PER_ACTIVATION: cudnnBatchNormMode_t = cudnnBatchNormMode_t(
        0,
    );
}
impl cudnnBatchNormMode_t {
    pub const CUDNN_BATCHNORM_SPATIAL: cudnnBatchNormMode_t = cudnnBatchNormMode_t(1);
}
impl cudnnBatchNormMode_t {
    pub const CUDNN_BATCHNORM_SPATIAL_PERSISTENT: cudnnBatchNormMode_t = cudnnBatchNormMode_t(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBatchNormMode_t(pub ::core::ffi::c_uint);
impl cudnnBatchNormOps_t {
    pub const CUDNN_BATCHNORM_OPS_BN: cudnnBatchNormOps_t = cudnnBatchNormOps_t(0);
}
impl cudnnBatchNormOps_t {
    pub const CUDNN_BATCHNORM_OPS_BN_ACTIVATION: cudnnBatchNormOps_t = cudnnBatchNormOps_t(
        1,
    );
}
impl cudnnBatchNormOps_t {
    pub const CUDNN_BATCHNORM_OPS_BN_ADD_ACTIVATION: cudnnBatchNormOps_t = cudnnBatchNormOps_t(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBatchNormOps_t(pub ::core::ffi::c_uint);
impl cudnnNormMode_t {
    pub const CUDNN_NORM_PER_ACTIVATION: cudnnNormMode_t = cudnnNormMode_t(0);
}
impl cudnnNormMode_t {
    pub const CUDNN_NORM_PER_CHANNEL: cudnnNormMode_t = cudnnNormMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnNormMode_t(pub ::core::ffi::c_uint);
impl cudnnNormAlgo_t {
    pub const CUDNN_NORM_ALGO_STANDARD: cudnnNormAlgo_t = cudnnNormAlgo_t(0);
}
impl cudnnNormAlgo_t {
    pub const CUDNN_NORM_ALGO_PERSIST: cudnnNormAlgo_t = cudnnNormAlgo_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnNormAlgo_t(pub ::core::ffi::c_uint);
impl cudnnNormOps_t {
    pub const CUDNN_NORM_OPS_NORM: cudnnNormOps_t = cudnnNormOps_t(0);
}
impl cudnnNormOps_t {
    pub const CUDNN_NORM_OPS_NORM_ACTIVATION: cudnnNormOps_t = cudnnNormOps_t(1);
}
impl cudnnNormOps_t {
    pub const CUDNN_NORM_OPS_NORM_ADD_ACTIVATION: cudnnNormOps_t = cudnnNormOps_t(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnNormOps_t(pub ::core::ffi::c_uint);
impl cudnnSamplerType_t {
    pub const CUDNN_SAMPLER_BILINEAR: cudnnSamplerType_t = cudnnSamplerType_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnSamplerType_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnDropoutStruct {
    _unused: [u8; 0],
}
impl cudnnConvolutionFwdAlgo_t {
    pub const CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM: cudnnConvolutionFwdAlgo_t = cudnnConvolutionFwdAlgo_t(
        0,
    );
}
impl cudnnConvolutionFwdAlgo_t {
    pub const CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM: cudnnConvolutionFwdAlgo_t = cudnnConvolutionFwdAlgo_t(
        1,
    );
}
impl cudnnConvolutionFwdAlgo_t {
    pub const CUDNN_CONVOLUTION_FWD_ALGO_GEMM: cudnnConvolutionFwdAlgo_t = cudnnConvolutionFwdAlgo_t(
        2,
    );
}
impl cudnnConvolutionFwdAlgo_t {
    pub const CUDNN_CONVOLUTION_FWD_ALGO_DIRECT: cudnnConvolutionFwdAlgo_t = cudnnConvolutionFwdAlgo_t(
        3,
    );
}
impl cudnnConvolutionFwdAlgo_t {
    pub const CUDNN_CONVOLUTION_FWD_ALGO_FFT: cudnnConvolutionFwdAlgo_t = cudnnConvolutionFwdAlgo_t(
        4,
    );
}
impl cudnnConvolutionFwdAlgo_t {
    pub const CUDNN_CONVOLUTION_FWD_ALGO_FFT_TILING: cudnnConvolutionFwdAlgo_t = cudnnConvolutionFwdAlgo_t(
        5,
    );
}
impl cudnnConvolutionFwdAlgo_t {
    pub const CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD: cudnnConvolutionFwdAlgo_t = cudnnConvolutionFwdAlgo_t(
        6,
    );
}
impl cudnnConvolutionFwdAlgo_t {
    pub const CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD_NONFUSED: cudnnConvolutionFwdAlgo_t = cudnnConvolutionFwdAlgo_t(
        7,
    );
}
impl cudnnConvolutionFwdAlgo_t {
    pub const CUDNN_CONVOLUTION_FWD_ALGO_COUNT: cudnnConvolutionFwdAlgo_t = cudnnConvolutionFwdAlgo_t(
        8,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnConvolutionFwdAlgo_t(pub ::core::ffi::c_uint);
impl cudnnConvolutionBwdFilterAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_FILTER_ALGO_0: cudnnConvolutionBwdFilterAlgo_t = cudnnConvolutionBwdFilterAlgo_t(
        0,
    );
}
impl cudnnConvolutionBwdFilterAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_FILTER_ALGO_1: cudnnConvolutionBwdFilterAlgo_t = cudnnConvolutionBwdFilterAlgo_t(
        1,
    );
}
impl cudnnConvolutionBwdFilterAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT: cudnnConvolutionBwdFilterAlgo_t = cudnnConvolutionBwdFilterAlgo_t(
        2,
    );
}
impl cudnnConvolutionBwdFilterAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_FILTER_ALGO_3: cudnnConvolutionBwdFilterAlgo_t = cudnnConvolutionBwdFilterAlgo_t(
        3,
    );
}
impl cudnnConvolutionBwdFilterAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD: cudnnConvolutionBwdFilterAlgo_t = cudnnConvolutionBwdFilterAlgo_t(
        4,
    );
}
impl cudnnConvolutionBwdFilterAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD_NONFUSED: cudnnConvolutionBwdFilterAlgo_t = cudnnConvolutionBwdFilterAlgo_t(
        5,
    );
}
impl cudnnConvolutionBwdFilterAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT_TILING: cudnnConvolutionBwdFilterAlgo_t = cudnnConvolutionBwdFilterAlgo_t(
        6,
    );
}
impl cudnnConvolutionBwdFilterAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_FILTER_ALGO_COUNT: cudnnConvolutionBwdFilterAlgo_t = cudnnConvolutionBwdFilterAlgo_t(
        7,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnConvolutionBwdFilterAlgo_t(pub ::core::ffi::c_uint);
impl cudnnConvolutionBwdDataAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_DATA_ALGO_0: cudnnConvolutionBwdDataAlgo_t = cudnnConvolutionBwdDataAlgo_t(
        0,
    );
}
impl cudnnConvolutionBwdDataAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_DATA_ALGO_1: cudnnConvolutionBwdDataAlgo_t = cudnnConvolutionBwdDataAlgo_t(
        1,
    );
}
impl cudnnConvolutionBwdDataAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT: cudnnConvolutionBwdDataAlgo_t = cudnnConvolutionBwdDataAlgo_t(
        2,
    );
}
impl cudnnConvolutionBwdDataAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT_TILING: cudnnConvolutionBwdDataAlgo_t = cudnnConvolutionBwdDataAlgo_t(
        3,
    );
}
impl cudnnConvolutionBwdDataAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD: cudnnConvolutionBwdDataAlgo_t = cudnnConvolutionBwdDataAlgo_t(
        4,
    );
}
impl cudnnConvolutionBwdDataAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD_NONFUSED: cudnnConvolutionBwdDataAlgo_t = cudnnConvolutionBwdDataAlgo_t(
        5,
    );
}
impl cudnnConvolutionBwdDataAlgo_t {
    pub const CUDNN_CONVOLUTION_BWD_DATA_ALGO_COUNT: cudnnConvolutionBwdDataAlgo_t = cudnnConvolutionBwdDataAlgo_t(
        6,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnConvolutionBwdDataAlgo_t(pub ::core::ffi::c_uint);
impl cudnnCTCLossAlgo_t {
    pub const CUDNN_CTC_LOSS_ALGO_DETERMINISTIC: cudnnCTCLossAlgo_t = cudnnCTCLossAlgo_t(
        0,
    );
}
impl cudnnCTCLossAlgo_t {
    pub const CUDNN_CTC_LOSS_ALGO_NON_DETERMINISTIC: cudnnCTCLossAlgo_t = cudnnCTCLossAlgo_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnCTCLossAlgo_t(pub ::core::ffi::c_uint);
impl cudnnRNNAlgo_t {
    pub const CUDNN_RNN_ALGO_STANDARD: cudnnRNNAlgo_t = cudnnRNNAlgo_t(0);
}
impl cudnnRNNAlgo_t {
    pub const CUDNN_RNN_ALGO_PERSIST_STATIC: cudnnRNNAlgo_t = cudnnRNNAlgo_t(1);
}
impl cudnnRNNAlgo_t {
    pub const CUDNN_RNN_ALGO_PERSIST_DYNAMIC: cudnnRNNAlgo_t = cudnnRNNAlgo_t(2);
}
impl cudnnRNNAlgo_t {
    pub const CUDNN_RNN_ALGO_PERSIST_STATIC_SMALL_H: cudnnRNNAlgo_t = cudnnRNNAlgo_t(3);
}
impl cudnnRNNAlgo_t {
    pub const CUDNN_RNN_ALGO_COUNT: cudnnRNNAlgo_t = cudnnRNNAlgo_t(4);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnRNNAlgo_t(pub ::core::ffi::c_uint);
impl cudnnForwardMode_t {
    pub const CUDNN_FWD_MODE_INFERENCE: cudnnForwardMode_t = cudnnForwardMode_t(0);
}
impl cudnnForwardMode_t {
    pub const CUDNN_FWD_MODE_TRAINING: cudnnForwardMode_t = cudnnForwardMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnForwardMode_t(pub ::core::ffi::c_uint);
impl cudnnRNNMode_t {
    pub const CUDNN_RNN_RELU: cudnnRNNMode_t = cudnnRNNMode_t(0);
}
impl cudnnRNNMode_t {
    pub const CUDNN_RNN_TANH: cudnnRNNMode_t = cudnnRNNMode_t(1);
}
impl cudnnRNNMode_t {
    pub const CUDNN_LSTM: cudnnRNNMode_t = cudnnRNNMode_t(2);
}
impl cudnnRNNMode_t {
    pub const CUDNN_GRU: cudnnRNNMode_t = cudnnRNNMode_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnRNNMode_t(pub ::core::ffi::c_uint);
impl cudnnRNNBiasMode_t {
    pub const CUDNN_RNN_NO_BIAS: cudnnRNNBiasMode_t = cudnnRNNBiasMode_t(0);
}
impl cudnnRNNBiasMode_t {
    pub const CUDNN_RNN_SINGLE_INP_BIAS: cudnnRNNBiasMode_t = cudnnRNNBiasMode_t(1);
}
impl cudnnRNNBiasMode_t {
    pub const CUDNN_RNN_DOUBLE_BIAS: cudnnRNNBiasMode_t = cudnnRNNBiasMode_t(2);
}
impl cudnnRNNBiasMode_t {
    pub const CUDNN_RNN_SINGLE_REC_BIAS: cudnnRNNBiasMode_t = cudnnRNNBiasMode_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnRNNBiasMode_t(pub ::core::ffi::c_uint);
impl cudnnDirectionMode_t {
    pub const CUDNN_UNIDIRECTIONAL: cudnnDirectionMode_t = cudnnDirectionMode_t(0);
}
impl cudnnDirectionMode_t {
    pub const CUDNN_BIDIRECTIONAL: cudnnDirectionMode_t = cudnnDirectionMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnDirectionMode_t(pub ::core::ffi::c_uint);
impl cudnnRNNInputMode_t {
    pub const CUDNN_LINEAR_INPUT: cudnnRNNInputMode_t = cudnnRNNInputMode_t(0);
}
impl cudnnRNNInputMode_t {
    pub const CUDNN_SKIP_INPUT: cudnnRNNInputMode_t = cudnnRNNInputMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnRNNInputMode_t(pub ::core::ffi::c_uint);
impl cudnnRNNClipMode_t {
    pub const CUDNN_RNN_CLIP_NONE: cudnnRNNClipMode_t = cudnnRNNClipMode_t(0);
}
impl cudnnRNNClipMode_t {
    pub const CUDNN_RNN_CLIP_MINMAX: cudnnRNNClipMode_t = cudnnRNNClipMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnRNNClipMode_t(pub ::core::ffi::c_uint);
impl cudnnRNNDataLayout_t {
    pub const CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_UNPACKED: cudnnRNNDataLayout_t = cudnnRNNDataLayout_t(
        0,
    );
}
impl cudnnRNNDataLayout_t {
    pub const CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_PACKED: cudnnRNNDataLayout_t = cudnnRNNDataLayout_t(
        1,
    );
}
impl cudnnRNNDataLayout_t {
    pub const CUDNN_RNN_DATA_LAYOUT_BATCH_MAJOR_UNPACKED: cudnnRNNDataLayout_t = cudnnRNNDataLayout_t(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnRNNDataLayout_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnRNNStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnRNNDataStruct {
    _unused: [u8; 0],
}
impl cudnnSeqDataAxis_t {
    pub const CUDNN_SEQDATA_TIME_DIM: cudnnSeqDataAxis_t = cudnnSeqDataAxis_t(0);
}
impl cudnnSeqDataAxis_t {
    pub const CUDNN_SEQDATA_BATCH_DIM: cudnnSeqDataAxis_t = cudnnSeqDataAxis_t(1);
}
impl cudnnSeqDataAxis_t {
    pub const CUDNN_SEQDATA_BEAM_DIM: cudnnSeqDataAxis_t = cudnnSeqDataAxis_t(2);
}
impl cudnnSeqDataAxis_t {
    pub const CUDNN_SEQDATA_VECT_DIM: cudnnSeqDataAxis_t = cudnnSeqDataAxis_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnSeqDataAxis_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnSeqDataStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnAttnStruct {
    _unused: [u8; 0],
}
impl cudnnMultiHeadAttnWeightKind_t {
    pub const CUDNN_MH_ATTN_Q_WEIGHTS: cudnnMultiHeadAttnWeightKind_t = cudnnMultiHeadAttnWeightKind_t(
        0,
    );
}
impl cudnnMultiHeadAttnWeightKind_t {
    pub const CUDNN_MH_ATTN_K_WEIGHTS: cudnnMultiHeadAttnWeightKind_t = cudnnMultiHeadAttnWeightKind_t(
        1,
    );
}
impl cudnnMultiHeadAttnWeightKind_t {
    pub const CUDNN_MH_ATTN_V_WEIGHTS: cudnnMultiHeadAttnWeightKind_t = cudnnMultiHeadAttnWeightKind_t(
        2,
    );
}
impl cudnnMultiHeadAttnWeightKind_t {
    pub const CUDNN_MH_ATTN_O_WEIGHTS: cudnnMultiHeadAttnWeightKind_t = cudnnMultiHeadAttnWeightKind_t(
        3,
    );
}
impl cudnnMultiHeadAttnWeightKind_t {
    pub const CUDNN_MH_ATTN_Q_BIASES: cudnnMultiHeadAttnWeightKind_t = cudnnMultiHeadAttnWeightKind_t(
        4,
    );
}
impl cudnnMultiHeadAttnWeightKind_t {
    pub const CUDNN_MH_ATTN_K_BIASES: cudnnMultiHeadAttnWeightKind_t = cudnnMultiHeadAttnWeightKind_t(
        5,
    );
}
impl cudnnMultiHeadAttnWeightKind_t {
    pub const CUDNN_MH_ATTN_V_BIASES: cudnnMultiHeadAttnWeightKind_t = cudnnMultiHeadAttnWeightKind_t(
        6,
    );
}
impl cudnnMultiHeadAttnWeightKind_t {
    pub const CUDNN_MH_ATTN_O_BIASES: cudnnMultiHeadAttnWeightKind_t = cudnnMultiHeadAttnWeightKind_t(
        7,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnMultiHeadAttnWeightKind_t(pub ::core::ffi::c_uint);
impl cudnnWgradMode_t {
    pub const CUDNN_WGRAD_MODE_ADD: cudnnWgradMode_t = cudnnWgradMode_t(0);
}
impl cudnnWgradMode_t {
    pub const CUDNN_WGRAD_MODE_SET: cudnnWgradMode_t = cudnnWgradMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnWgradMode_t(pub ::core::ffi::c_uint);
impl cudnnLossNormalizationMode_t {
    pub const CUDNN_LOSS_NORMALIZATION_NONE: cudnnLossNormalizationMode_t = cudnnLossNormalizationMode_t(
        0,
    );
}
impl cudnnLossNormalizationMode_t {
    pub const CUDNN_LOSS_NORMALIZATION_SOFTMAX: cudnnLossNormalizationMode_t = cudnnLossNormalizationMode_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnLossNormalizationMode_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnConvolutionStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnFusedOpsConstParamStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnFusedOpsVariantParamStruct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnFusedOpsPlanStruct {
    _unused: [u8; 0],
}
impl cudnnFusedOps_t {
    pub const CUDNN_FUSED_SCALE_BIAS_ACTIVATION_CONV_BNSTATS: cudnnFusedOps_t = cudnnFusedOps_t(
        0,
    );
}
impl cudnnFusedOps_t {
    pub const CUDNN_FUSED_SCALE_BIAS_ACTIVATION_WGRAD: cudnnFusedOps_t = cudnnFusedOps_t(
        1,
    );
}
impl cudnnFusedOps_t {
    pub const CUDNN_FUSED_BN_FINALIZE_STATISTICS_TRAINING: cudnnFusedOps_t = cudnnFusedOps_t(
        2,
    );
}
impl cudnnFusedOps_t {
    pub const CUDNN_FUSED_BN_FINALIZE_STATISTICS_INFERENCE: cudnnFusedOps_t = cudnnFusedOps_t(
        3,
    );
}
impl cudnnFusedOps_t {
    pub const CUDNN_FUSED_CONV_SCALE_BIAS_ADD_ACTIVATION: cudnnFusedOps_t = cudnnFusedOps_t(
        4,
    );
}
impl cudnnFusedOps_t {
    pub const CUDNN_FUSED_SCALE_BIAS_ADD_ACTIVATION_GEN_BITMASK: cudnnFusedOps_t = cudnnFusedOps_t(
        5,
    );
}
impl cudnnFusedOps_t {
    pub const CUDNN_FUSED_DACTIVATION_FORK_DBATCHNORM: cudnnFusedOps_t = cudnnFusedOps_t(
        6,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnFusedOps_t(pub ::core::ffi::c_uint);
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_XDESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        0,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_XDATA_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        1,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_MODE: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        2,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_EQSCALEBIAS_DESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        3,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_EQSCALE_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        4,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_EQBIAS_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        5,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_ACTIVATION_DESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        6,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_CONV_DESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        7,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_WDESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        8,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_WDATA_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        9,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_DWDESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        10,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_DWDATA_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        11,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_YDESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        12,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_YDATA_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        13,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_DYDESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        14,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_DYDATA_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        15,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_YSTATS_DESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        16,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_YSUM_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        17,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_YSQSUM_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        18,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_SCALEBIAS_MEANVAR_DESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        19,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_SCALE_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        20,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_BIAS_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        21,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_SAVED_MEAN_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        22,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_SAVED_INVSTD_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        23,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_RUNNING_MEAN_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        24,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_RUNNING_VAR_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        25,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_ZDESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        26,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_ZDATA_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        27,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_Z_EQSCALEBIAS_DESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        28,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_Z_EQSCALE_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        29,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_Z_EQBIAS_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        30,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_ACTIVATION_BITMASK_DESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        31,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_ACTIVATION_BITMASK_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        32,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_DXDESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        33,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_DXDATA_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        34,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_DZDESC: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        35,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_DZDATA_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        36,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_DSCALE_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        37,
    );
}
impl cudnnFusedOpsConstParamLabel_t {
    pub const CUDNN_PARAM_BN_DBIAS_PLACEHOLDER: cudnnFusedOpsConstParamLabel_t = cudnnFusedOpsConstParamLabel_t(
        38,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnFusedOpsConstParamLabel_t(pub ::core::ffi::c_uint);
impl cudnnFusedOpsPointerPlaceHolder_t {
    pub const CUDNN_PTR_NULL: cudnnFusedOpsPointerPlaceHolder_t = cudnnFusedOpsPointerPlaceHolder_t(
        0,
    );
}
impl cudnnFusedOpsPointerPlaceHolder_t {
    pub const CUDNN_PTR_ELEM_ALIGNED: cudnnFusedOpsPointerPlaceHolder_t = cudnnFusedOpsPointerPlaceHolder_t(
        1,
    );
}
impl cudnnFusedOpsPointerPlaceHolder_t {
    pub const CUDNN_PTR_16B_ALIGNED: cudnnFusedOpsPointerPlaceHolder_t = cudnnFusedOpsPointerPlaceHolder_t(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnFusedOpsPointerPlaceHolder_t(pub ::core::ffi::c_uint);
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_XDATA: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        0,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_EQSCALE: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        1,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_EQBIAS: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        2,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_WDATA: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        3,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_DWDATA: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        4,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_YDATA: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        5,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_DYDATA: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        6,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_YSUM: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        7,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_YSQSUM: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        8,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_WORKSPACE: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        9,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_SCALE: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        10,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_BIAS: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        11,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_SAVED_MEAN: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        12,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_SAVED_INVSTD: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        13,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_RUNNING_MEAN: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        14,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_RUNNING_VAR: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        15,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_ZDATA: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        16,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_Z_EQSCALE: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        17,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_Z_EQBIAS: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        18,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_ACTIVATION_BITMASK: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        19,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_DXDATA: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        20,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_DZDATA: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        21,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_DSCALE: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        22,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_PTR_BN_DBIAS: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        23,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_SCALAR_SIZE_T_WORKSPACE_SIZE_IN_BYTES: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        100,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_SCALAR_INT64_T_BN_ACCUMULATION_COUNT: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        101,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_SCALAR_DOUBLE_BN_EXP_AVG_FACTOR: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        102,
    );
}
impl cudnnFusedOpsVariantParamLabel_t {
    pub const CUDNN_SCALAR_DOUBLE_BN_EPSILON: cudnnFusedOpsVariantParamLabel_t = cudnnFusedOpsVariantParamLabel_t(
        103,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnFusedOpsVariantParamLabel_t(pub ::core::ffi::c_uint);
