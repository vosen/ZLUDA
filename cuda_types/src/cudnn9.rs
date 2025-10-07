// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub type __half = u16;
pub type __nv_bfloat16 = u16;
pub use super::cuda::cuComplex;
pub use super::cuda::cuDoubleComplex;
pub use super::cuda::cudaDataType;
pub use super::cuda::cudaDataType_t;
pub type cudaStream_t = super::cuda::CUstream;
pub use super::cuda::libraryPropertyType;
pub type cudaGraphExecUpdateResultInfo_st = super::cuda::CUgraphExecUpdateResultInfo_st;
pub type cudaAsyncNotificationType = super::cuda::CUasyncNotificationType_enum;
pub type cudaGraph_t = super::cuda::CUgraph;
pub const CUDNN_MAJOR: u32 = 9;
pub const CUDNN_MINOR: u32 = 13;
pub const CUDNN_PATCHLEVEL: u32 = 1;
pub const CUDNN_VERSION: u32 = 91301;
pub const CUDNN_MAX_SM_MAJOR_NUMBER: u32 = 12;
pub const CUDNN_MAX_SM_MINOR_NUMBER: u32 = 1;
pub const CUDNN_MAX_DEVICE_VERSION: u32 = 1210;
pub const CUDNN_GRAPH_MAJOR: u32 = 9;
pub const CUDNN_GRAPH_MINOR: u32 = 13;
pub const CUDNN_GRAPH_PATCH: u32 = 1;
pub const CUDNN_DIM_MAX: u32 = 8;
pub const CUDNN_OPS_MAJOR: u32 = 9;
pub const CUDNN_OPS_MINOR: u32 = 13;
pub const CUDNN_OPS_PATCH: u32 = 1;
pub const CUDNN_LRN_MIN_N: u32 = 1;
pub const CUDNN_LRN_MAX_N: u32 = 16;
pub const CUDNN_LRN_MIN_K: f64 = 0.00001;
pub const CUDNN_LRN_MIN_BETA: f64 = 0.01;
pub const CUDNN_BN_MIN_EPSILON: f64 = 0.0;
pub const CUDNN_ADV_MAJOR: u32 = 9;
pub const CUDNN_ADV_MINOR: u32 = 13;
pub const CUDNN_ADV_PATCH: u32 = 1;
pub const CUDNN_RNN_PADDED_IO_DISABLED: u32 = 0;
pub const CUDNN_RNN_PADDED_IO_ENABLED: u32 = 1;
pub const CUDNN_SEQDATA_DIM_COUNT: u32 = 4;
pub const CUDNN_ATTN_QUERYMAP_ALL_TO_ONE: u32 = 0;
pub const CUDNN_ATTN_QUERYMAP_ONE_TO_ONE: u32 = 1;
pub const CUDNN_ATTN_DISABLE_PROJ_BIASES: u32 = 0;
pub const CUDNN_ATTN_ENABLE_PROJ_BIASES: u32 = 2;
pub const CUDNN_ATTN_WKIND_COUNT: u32 = 8;
pub const CUDNN_CNN_MAJOR: u32 = 9;
pub const CUDNN_CNN_MINOR: u32 = 13;
pub const CUDNN_CNN_PATCH: u32 = 1;
pub use super::cudnn::cudnnContext;
pub type cudnnHandle_t = *mut cudnnContext;
pub use super::cudnn::cudnnRuntimeTag_t;
pub use super::cudnn::cudnnErrQueryMode_t;
impl cudnnDataType_t {
    pub const CUDNN_DATA_FLOAT: cudnnDataType_t = cudnnDataType_t(0);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_DOUBLE: cudnnDataType_t = cudnnDataType_t(1);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_HALF: cudnnDataType_t = cudnnDataType_t(2);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_INT8: cudnnDataType_t = cudnnDataType_t(3);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_INT32: cudnnDataType_t = cudnnDataType_t(4);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_INT8x4: cudnnDataType_t = cudnnDataType_t(5);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_UINT8: cudnnDataType_t = cudnnDataType_t(6);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_UINT8x4: cudnnDataType_t = cudnnDataType_t(7);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_INT8x32: cudnnDataType_t = cudnnDataType_t(8);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_BFLOAT16: cudnnDataType_t = cudnnDataType_t(9);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_INT64: cudnnDataType_t = cudnnDataType_t(10);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_BOOLEAN: cudnnDataType_t = cudnnDataType_t(11);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_FP8_E4M3: cudnnDataType_t = cudnnDataType_t(12);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_FP8_E5M2: cudnnDataType_t = cudnnDataType_t(13);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_FAST_FLOAT_FOR_FP8: cudnnDataType_t = cudnnDataType_t(14);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_FP8_E8M0: cudnnDataType_t = cudnnDataType_t(15);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_FP4_E2M1: cudnnDataType_t = cudnnDataType_t(16);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_INT4: cudnnDataType_t = cudnnDataType_t(17);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_UINT4: cudnnDataType_t = cudnnDataType_t(18);
}
impl cudnnDataType_t {
    pub const CUDNN_DATA_UINT32: cudnnDataType_t = cudnnDataType_t(19);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnDataType_t(pub ::core::ffi::c_uint);
pub use super::cudnn::cudnnMathType_t;
pub use super::cudnn::cudnnNanPropagation_t;
impl cudnnCTCGradMode_t {
    pub const CUDNN_CTC_ZERO_OOB_GRADIENTS: cudnnCTCGradMode_t = cudnnCTCGradMode_t(0);
}
impl cudnnCTCGradMode_t {
    pub const CUDNN_CTC_SKIP_OOB_GRADIENTS: cudnnCTCGradMode_t = cudnnCTCGradMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnCTCGradMode_t(pub ::core::ffi::c_uint);
pub use super::cudnn::cudnnTensorFormat_t;
pub use super::cudnn::cudnnReduceTensorOp_t;
pub use super::cudnn::cudnnActivationMode_t;
pub use super::cudnn::cudnnSeverity_t;
#[repr(C)]
pub struct cudnnDebugStruct {
    pub cudnn_version: ::core::ffi::c_uint,
    pub cudnnStatus: cudnnStatus_t,
    pub time_sec: ::core::ffi::c_uint,
    pub time_usec: ::core::ffi::c_uint,
    pub time_delta: ::core::ffi::c_uint,
    pub handle: cudnnHandle_t,
    pub stream: cudaStream_t,
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
pub use super::cudnn::cudnnConvolutionMode_t;
pub use super::cudnn::cudnnReorderType_t;
pub type cudnnBackendDescriptor_t = *mut ::core::ffi::c_void;
pub use super::cudnn::cudnnFractionStruct;
pub type cudnnFraction_t = cudnnFractionStruct;
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_ADD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(0);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_ADD_SQUARE: cudnnPointwiseMode_t = cudnnPointwiseMode_t(5);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_DIV: cudnnPointwiseMode_t = cudnnPointwiseMode_t(6);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_MAX: cudnnPointwiseMode_t = cudnnPointwiseMode_t(3);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_MIN: cudnnPointwiseMode_t = cudnnPointwiseMode_t(2);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_MOD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(7);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_MUL: cudnnPointwiseMode_t = cudnnPointwiseMode_t(1);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_POW: cudnnPointwiseMode_t = cudnnPointwiseMode_t(8);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_SUB: cudnnPointwiseMode_t = cudnnPointwiseMode_t(9);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_ABS: cudnnPointwiseMode_t = cudnnPointwiseMode_t(10);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_CEIL: cudnnPointwiseMode_t = cudnnPointwiseMode_t(11);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_COS: cudnnPointwiseMode_t = cudnnPointwiseMode_t(12);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_EXP: cudnnPointwiseMode_t = cudnnPointwiseMode_t(13);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_FLOOR: cudnnPointwiseMode_t = cudnnPointwiseMode_t(14);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_LOG: cudnnPointwiseMode_t = cudnnPointwiseMode_t(15);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_NEG: cudnnPointwiseMode_t = cudnnPointwiseMode_t(16);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_RSQRT: cudnnPointwiseMode_t = cudnnPointwiseMode_t(17);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_SIN: cudnnPointwiseMode_t = cudnnPointwiseMode_t(18);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_SQRT: cudnnPointwiseMode_t = cudnnPointwiseMode_t(4);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_TAN: cudnnPointwiseMode_t = cudnnPointwiseMode_t(19);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_ERF: cudnnPointwiseMode_t = cudnnPointwiseMode_t(20);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_IDENTITY: cudnnPointwiseMode_t = cudnnPointwiseMode_t(21);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_RECIPROCAL: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        22,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_ATAN2: cudnnPointwiseMode_t = cudnnPointwiseMode_t(23);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_RELU_FWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(100);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_TANH_FWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(101);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_SIGMOID_FWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        102,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_ELU_FWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(103);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_GELU_FWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(104);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_SOFTPLUS_FWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        105,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_SWISH_FWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        106,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_GELU_APPROX_TANH_FWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        107,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_RELU_BWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(200);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_TANH_BWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(201);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_SIGMOID_BWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        202,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_ELU_BWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(203);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_GELU_BWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(204);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_SOFTPLUS_BWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        205,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_SWISH_BWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        206,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_GELU_APPROX_TANH_BWD: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        207,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_CMP_EQ: cudnnPointwiseMode_t = cudnnPointwiseMode_t(300);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_CMP_NEQ: cudnnPointwiseMode_t = cudnnPointwiseMode_t(301);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_CMP_GT: cudnnPointwiseMode_t = cudnnPointwiseMode_t(302);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_CMP_GE: cudnnPointwiseMode_t = cudnnPointwiseMode_t(303);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_CMP_LT: cudnnPointwiseMode_t = cudnnPointwiseMode_t(304);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_CMP_LE: cudnnPointwiseMode_t = cudnnPointwiseMode_t(305);
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_LOGICAL_AND: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        400,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_LOGICAL_OR: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        401,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_LOGICAL_NOT: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        402,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_GEN_INDEX: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        501,
    );
}
impl cudnnPointwiseMode_t {
    pub const CUDNN_POINTWISE_BINARY_SELECT: cudnnPointwiseMode_t = cudnnPointwiseMode_t(
        601,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnPointwiseMode_t(pub ::core::ffi::c_uint);
pub use super::cudnn::cudnnResampleMode_t;
pub use super::cudnn::cudnnSignalMode_t;
pub use super::cudnn::cudnnGenStatsMode_t;
pub use super::cudnn::cudnnBnFinalizeStatsMode_t;
pub use super::cudnn::cudnnRngDistribution_t;
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        0,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_MATH_PREC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_NAN_PROPAGATION: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_RELU_LOWER_CLIP: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        3,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_RELU_UPPER_CLIP: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        4,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_RELU_LOWER_CLIP_SLOPE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        5,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_ELU_ALPHA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        6,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_SOFTPLUS_BETA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        7,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_SWISH_BETA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        8,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_POINTWISE_AXIS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        9,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_CONVOLUTION_COMP_TYPE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        100,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_CONVOLUTION_CONV_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        101,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_CONVOLUTION_DILATIONS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        102,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_CONVOLUTION_FILTER_STRIDES: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        103,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_CONVOLUTION_POST_PADDINGS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        104,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_CONVOLUTION_PRE_PADDINGS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        105,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_CONVOLUTION_SPATIAL_DIMS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        106,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINEHEUR_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        200,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINEHEUR_OPERATION_GRAPH: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        201,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINEHEUR_RESULTS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        202,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINEHEUR_SM_COUNT_TARGET: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        203,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINEHEUR_DEVICEPROP: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        204,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINECFG_ENGINE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        300,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINECFG_INTERMEDIATE_INFO: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        301,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINECFG_KNOB_CHOICES: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        302,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINECFG_WORKSPACE_SIZE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        303,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINECFG_SHARED_MEMORY_USED: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        304,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_EXECUTION_PLAN_HANDLE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        400,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_EXECUTION_PLAN_ENGINE_CONFIG: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        401,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_EXECUTION_PLAN_WORKSPACE_SIZE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        402,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_EXECUTION_PLAN_COMPUTED_INTERMEDIATE_UIDS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        403,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_EXECUTION_PLAN_RUN_ONLY_INTERMEDIATE_UIDS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        404,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_EXECUTION_PLAN_JSON_REPRESENTATION: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        405,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_EXECUTION_PLAN_KERNEL_CACHE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        406,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_EXECUTION_PLAN_DEVICEPROP: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        407,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_INTERMEDIATE_INFO_UNIQUE_ID: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        500,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_INTERMEDIATE_INFO_SIZE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        501,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_INTERMEDIATE_INFO_DEPENDENT_DATA_UIDS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        502,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_INTERMEDIATE_INFO_DEPENDENT_ATTRIBUTES: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        503,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_KNOB_CHOICE_KNOB_TYPE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        600,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_KNOB_CHOICE_KNOB_VALUE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        601,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_ALPHA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        700,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_BETA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        701,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_CONV_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        702,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_W: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        703,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_X: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        704,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_Y: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        705,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_ALPHA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        706,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_BETA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        707,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_CONV_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        708,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_W: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        709,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DX: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        710,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DY: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        711,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_ALPHA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        712,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_BETA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        713,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_CONV_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        714,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DW: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        715,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_X: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        716,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DY: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        717,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_POINTWISE_PW_DESCRIPTOR: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        750,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_POINTWISE_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        751,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_POINTWISE_BDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        752,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_POINTWISE_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        753,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_POINTWISE_ALPHA1: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        754,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_POINTWISE_ALPHA2: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        755,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_POINTWISE_DXDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        756,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_POINTWISE_DYDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        757,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_POINTWISE_TDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        758,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_GENSTATS_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        770,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_GENSTATS_MATH_PREC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        771,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_GENSTATS_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        772,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_GENSTATS_SUMDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        773,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_GENSTATS_SQSUMDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        774,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_STATS_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        780,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_MATH_PREC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        781,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_Y_SUM_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        782,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_Y_SQ_SUM_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        783,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        784,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_BIAS_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        785,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_PREV_RUNNING_MEAN_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        786,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_PREV_RUNNING_VAR_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        787,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_UPDATED_RUNNING_MEAN_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        788,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_UPDATED_RUNNING_VAR_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        789,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_SAVED_MEAN_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        790,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_SAVED_INV_STD_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        791,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_EQ_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        792,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_EQ_BIAS_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        793,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_ACCUM_COUNT_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        794,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_EPSILON_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        795,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_FINALIZE_EXP_AVERATE_FACTOR_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        796,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATIONGRAPH_HANDLE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        800,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATIONGRAPH_OPS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        801,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATIONGRAPH_ENGINE_GLOBAL_COUNT: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        802,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATIONGRAPH_IS_DYNAMIC_SHAPE_ENABLED: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        803,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATIONGRAPH_IS_SAME_TOPOLOGY: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        804,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_BYTE_ALIGNMENT: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        900,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_DATA_TYPE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        901,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_DIMENSIONS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        902,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_STRIDES: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        903,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_VECTOR_COUNT: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        904,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_VECTORIZED_DIMENSION: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        905,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_UNIQUE_ID: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        906,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_IS_VIRTUAL: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        907,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_IS_BY_VALUE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        908,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_REORDERING_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        909,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_TENSOR_RAGGED_OFFSET_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        913,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_VARIANT_PACK_UNIQUE_IDS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1000,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_VARIANT_PACK_DATA_POINTERS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1001,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_VARIANT_PACK_INTERMEDIATES: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1002,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_VARIANT_PACK_WORKSPACE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1003,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_LAYOUT_INFO_TENSOR_UID: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1100,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_LAYOUT_INFO_TYPES: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1101,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_KNOB_INFO_TYPE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1200,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_KNOB_INFO_MAXIMUM_VALUE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1201,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_KNOB_INFO_MINIMUM_VALUE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1202,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_KNOB_INFO_STRIDE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1203,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINE_OPERATION_GRAPH: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1300,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINE_GLOBAL_INDEX: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1301,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINE_KNOB_INFO: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1302,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINE_NUMERICAL_NOTE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1303,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINE_LAYOUT_INFO: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1304,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINE_BEHAVIOR_NOTE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1305,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINE_SM_COUNT_TARGET: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1306,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_ENGINE_DEVICEPROP: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1307,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_MATMUL_COMP_TYPE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1500,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_MATMUL_PADDING_VALUE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1503,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_MATMUL_ADESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1520,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_MATMUL_BDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1521,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_MATMUL_CDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1522,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_MATMUL_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1523,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_MATMUL_IRREGULARLY_STRIDED_BATCH_COUNT: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1524,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_MATMUL_GEMM_M_OVERRIDE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1525,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_MATMUL_GEMM_N_OVERRIDE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1526,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_MATMUL_GEMM_K_OVERRIDE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1527,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_REDUCTION_OPERATOR: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1600,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_REDUCTION_COMP_TYPE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1601,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_REDUCTION_IS_DETERMINISTIC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1602,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_REDUCTION_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1610,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_REDUCTION_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1611,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_REDUCTION_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1612,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_MATH_PREC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1620,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_MEAN_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1621,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_INVSTD_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1622,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_BN_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1623,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_X_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1624,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DY_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1625,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DBN_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1626,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DBN_BIAS_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1627,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_DY_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1628,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_X_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1629,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_BIAS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1630,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RESAMPLE_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1700,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RESAMPLE_COMP_TYPE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1701,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RESAMPLE_SPATIAL_DIMS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1702,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RESAMPLE_POST_PADDINGS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1703,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RESAMPLE_PRE_PADDINGS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1704,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RESAMPLE_STRIDES: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1705,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RESAMPLE_WINDOW_DIMS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1706,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RESAMPLE_NAN_PROPAGATION: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1707,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RESAMPLE_PADDING_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1708,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_FWD_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1710,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_FWD_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1711,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_FWD_IDXDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1712,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_FWD_ALPHA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1713,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_FWD_BETA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1714,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_FWD_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1716,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DXDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1720,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DYDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1721,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_BWD_IDXDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1722,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_BWD_ALPHA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1723,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_BWD_BETA: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1724,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1725,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_BWD_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1726,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESAMPLE_BWD_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1727,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONCAT_AXIS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1800,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONCAT_INPUT_DESCS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1801,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONCAT_INPLACE_INDEX: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1802,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONCAT_OUTPUT_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1803,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SIGNAL_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1900,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SIGNAL_FLAGDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1901,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SIGNAL_VALUE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1902,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SIGNAL_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1903,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SIGNAL_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1904,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_CONTAINER_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1950,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1951,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_SEQUENCE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1952,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_PAGE_TABLE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        1953,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2000,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_PHASE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2001,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2002,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_MEAN_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2003,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_INV_VARIANCE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2004,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2005,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_BIAS_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2006,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_EPSILON_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2007,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_EXP_AVG_FACTOR_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2008,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_INPUT_RUNNING_MEAN_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2009,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_INPUT_RUNNING_VAR_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2010,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_OUTPUT_RUNNING_MEAN_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2011,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_OUTPUT_RUNNING_VAR_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2012,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2013,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_FWD_PEER_STAT_DESCS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2014,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_MODE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2100,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2101,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_MEAN_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2102,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_INV_VARIANCE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2103,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_DYDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2104,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2105,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_EPSILON_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2106,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_DSCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2107,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_DBIAS_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2108,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_DXDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2109,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_NORM_BWD_PEER_STAT_DESCS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2110,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESHAPE_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2200,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RESHAPE_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2201,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2250,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2251,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_LOWER_BANDWIDTH: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2252,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_UPPER_BANDWIDTH: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2253,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_AXIS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2254,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_PAD_VALUE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2255,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_KV_TOKEN_OFFSET_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2256,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_SPECULATIVE_MASK_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2257,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2270,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2271,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_LOWER_BANDWIDTH: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2272,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_UPPER_BANDWIDTH: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2273,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_AXIS: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2274,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_PAD_VALUE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2275,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_CONTRACT_BAND_MAX_TOKEN_VALUE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2276,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RNG_DISTRIBUTION: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2300,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RNG_NORMAL_DIST_MEAN: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2301,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RNG_NORMAL_DIST_STANDARD_DEVIATION: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2302,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RNG_UNIFORM_DIST_MAXIMUM: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2303,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RNG_UNIFORM_DIST_MINIMUM: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2304,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_RNG_BERNOULLI_DIST_PROBABILITY: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2305,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RNG_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2310,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RNG_SEED: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2311,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RNG_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2312,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_RNG_OFFSET_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2313,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_KERNEL_CACHE_OPERATION_GRAPH: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2400,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_KERNEL_CACHE_IS_ENGINECFG_KERNEL_CACHED: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2401,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_KERNEL_CACHE_JSON_REPRESENTATION: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2402,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2500,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2501,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2502,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_MATH_PREC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2503,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_BLOCK_SIZE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2504,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_XDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2600,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_SCALE_DESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2601,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_YDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2602,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_MATH_PREC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2603,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_BLOCK_SIZE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2604,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_NEG_SCALE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2605,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_DEVICEPROP_DEVICE_ID: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2700,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_DEVICEPROP_HANDLE: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2701,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_DEVICEPROP_JSON_REPRESENTATION: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2702,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SDPA_FWD_QDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2800,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SDPA_FWD_KDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2801,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SDPA_FWD_VDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2802,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SDPA_FWD_ODESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2803,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SDPA_FWD_STATSDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2804,
    );
}
impl cudnnBackendAttributeName_t {
    pub const CUDNN_ATTR_OPERATION_SDPA_FWD_SCALEDESC: cudnnBackendAttributeName_t = cudnnBackendAttributeName_t(
        2805,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendAttributeName_t(pub ::core::ffi::c_uint);
pub use super::cudnn::cudnnBackendAttributeType_t;
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_POINTWISE_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        0,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_CONVOLUTION_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        1,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_ENGINE_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        2,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_ENGINECFG_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        3,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_ENGINEHEUR_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        4,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_EXECUTION_PLAN_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        5,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_INTERMEDIATE_INFO_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        6,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_KNOB_CHOICE_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        7,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_KNOB_INFO_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        8,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_LAYOUT_INFO_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        9,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_CONVOLUTION_FORWARD_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        10,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_FILTER_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        11,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_DATA_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        12,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_POINTWISE_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        13,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_GEN_STATS_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        14,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATIONGRAPH_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        15,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_VARIANT_PACK_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        16,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_TENSOR_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        17,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_MATMUL_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        18,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_MATMUL_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        19,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_BN_FINALIZE_STATISTICS_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        20,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_REDUCTION_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        21,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_REDUCTION_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        22,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_BN_BWD_WEIGHTS_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        23,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_RESAMPLE_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        24,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_RESAMPLE_FWD_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        25,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_RESAMPLE_BWD_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        26,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_CONCAT_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        27,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_SIGNAL_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        28,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_NORM_FORWARD_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        29,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_NORM_BACKWARD_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        30,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_RESHAPE_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        31,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_RNG_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        32,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_RNG_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        33,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_KERNEL_CACHE_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        34,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_PAGED_CACHE_LOAD_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        35,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_BLOCK_SCALE_QUANTIZE_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        36,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_BLOCK_SCALE_DEQUANTIZE_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        37,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_DEVICEPROP_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        38,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_EXPAND_BAND_MATRIX_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        39,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_CONTRACT_BAND_MATRIX_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        40,
    );
}
impl cudnnBackendDescriptorType_t {
    pub const CUDNN_BACKEND_OPERATION_SDPA_FWD_DESCRIPTOR: cudnnBackendDescriptorType_t = cudnnBackendDescriptorType_t(
        41,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendDescriptorType_t(pub ::core::ffi::c_uint);
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
    pub const CUDNN_NUMERICAL_NOTE_STRICT_NAN_PROP: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        9,
    );
}
impl cudnnBackendNumericalNote_t {
    pub const CUDNN_NUMERICAL_NOTE_TYPE_COUNT: cudnnBackendNumericalNote_t = cudnnBackendNumericalNote_t(
        10,
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
    pub const CUDNN_BEHAVIOR_NOTE_SUPPORTS_CUDA_GRAPH_NATIVE_API: cudnnBackendBehaviorNote_t = cudnnBackendBehaviorNote_t(
        3,
    );
}
impl cudnnBackendBehaviorNote_t {
    pub const CUDNN_BEHAVIOR_NOTE_TYPE_COUNT: cudnnBackendBehaviorNote_t = cudnnBackendBehaviorNote_t(
        4,
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
    pub const CUDNN_KNOB_TYPE_CTA_COUNT: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        37,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_STREAM_K: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        38,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_SPLIT_P_SLC: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        39,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_TILE_M: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        40,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_TILE_N: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        41,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_WARP_SPEC_CFG: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        42,
    );
}
impl cudnnBackendKnobType_t {
    pub const CUDNN_KNOB_TYPE_COUNTS: cudnnBackendKnobType_t = cudnnBackendKnobType_t(
        43,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendKnobType_t(pub ::core::ffi::c_uint);
pub use super::cudnn::cudnnBackendLayoutType_t;
pub use super::cudnn::cudnnBackendHeurMode_t;
impl cudnnBackendTensorReordering_t {
    pub const CUDNN_TENSOR_REORDERING_NONE: cudnnBackendTensorReordering_t = cudnnBackendTensorReordering_t(
        0,
    );
}
impl cudnnBackendTensorReordering_t {
    pub const CUDNN_TENSOR_REORDERING_INT8x32: cudnnBackendTensorReordering_t = cudnnBackendTensorReordering_t(
        1,
    );
}
impl cudnnBackendTensorReordering_t {
    pub const CUDNN_TENSOR_REORDERING_F16x16: cudnnBackendTensorReordering_t = cudnnBackendTensorReordering_t(
        2,
    );
}
impl cudnnBackendTensorReordering_t {
    pub const CUDNN_TENSOR_REORDERING_F8_128x4: cudnnBackendTensorReordering_t = cudnnBackendTensorReordering_t(
        3,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendTensorReordering_t(pub ::core::ffi::c_uint);
pub use super::cudnn::cudnnPaddingMode_t;
impl cudnnBackendNormMode_t {
    pub const CUDNN_LAYER_NORM: cudnnBackendNormMode_t = cudnnBackendNormMode_t(0);
}
impl cudnnBackendNormMode_t {
    pub const CUDNN_INSTANCE_NORM: cudnnBackendNormMode_t = cudnnBackendNormMode_t(1);
}
impl cudnnBackendNormMode_t {
    pub const CUDNN_BATCH_NORM: cudnnBackendNormMode_t = cudnnBackendNormMode_t(2);
}
impl cudnnBackendNormMode_t {
    pub const CUDNN_GROUP_NORM: cudnnBackendNormMode_t = cudnnBackendNormMode_t(3);
}
impl cudnnBackendNormMode_t {
    pub const CUDNN_RMS_NORM: cudnnBackendNormMode_t = cudnnBackendNormMode_t(4);
}
impl cudnnBackendNormMode_t {
    pub const CUDNN_ADA_LAYER_NORM: cudnnBackendNormMode_t = cudnnBackendNormMode_t(5);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudnnBackendNormMode_t(pub ::core::ffi::c_uint);
pub use super::cudnn::cudnnBackendNormFwdPhase_t;
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
pub use super::cudnn::cudnnDeterminism_t;
pub use super::cudnn::cudnnFoldingDirection_t;
pub use super::cudnn::cudnnOpTensorOp_t;
pub use super::cudnn::cudnnReduceTensorIndices_t;
pub use super::cudnn::cudnnIndicesType_t;
pub use super::cudnn::cudnnSoftmaxAlgorithm_t;
pub use super::cudnn::cudnnSoftmaxMode_t;
pub use super::cudnn::cudnnPoolingMode_t;
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
pub use super::cudnn::cudnnConvolutionFwdAlgo_t;
pub use super::cudnn::cudnnConvolutionBwdFilterAlgo_t;
pub use super::cudnn::cudnnConvolutionBwdDataAlgo_t;
pub use super::cudnn::cudnnCTCLossAlgo_t;
pub use super::cudnn::cudnnRNNAlgo_t;
pub use super::cudnn::cudnnForwardMode_t;
pub use super::cudnn::cudnnRNNMode_t;
pub use super::cudnn::cudnnRNNBiasMode_t;
pub use super::cudnn::cudnnDirectionMode_t;
pub use super::cudnn::cudnnRNNInputMode_t;
pub use super::cudnn::cudnnRNNClipMode_t;
pub use super::cudnn::cudnnRNNDataLayout_t;
pub use super::cudnn::cudnnRNNStruct;
pub type cudnnRNNDescriptor_t = *mut cudnnRNNStruct;
pub use super::cudnn::cudnnRNNDataStruct;
pub type cudnnRNNDataDescriptor_t = *mut cudnnRNNDataStruct;
pub use super::cudnn::cudnnSeqDataAxis_t;
pub use super::cudnn::cudnnSeqDataStruct;
pub type cudnnSeqDataDescriptor_t = *mut cudnnSeqDataStruct;
pub use super::cudnn::cudnnAttnStruct;
pub type cudnnAttnDescriptor_t = *mut cudnnAttnStruct;
pub use super::cudnn::cudnnMultiHeadAttnWeightKind_t;
pub use super::cudnn::cudnnWgradMode_t;
pub use super::cudnn::cudnnLossNormalizationMode_t;
pub use super::cudnn::cudnnConvolutionStruct;
pub type cudnnConvolutionDescriptor_t = *mut cudnnConvolutionStruct;
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
impl cudnnError_t {
    pub const r#NOT_INITIALIZED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1001)
    });
    pub const r#SUBLIBRARY_VERSION_MISMATCH: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1002)
    });
    pub const r#SERIALIZATION_VERSION_MISMATCH: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1003)
    });
    pub const r#DEPRECATED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1004)
    });
    pub const r#LICENSE_ERROR: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1005)
    });
    pub const r#RUNTIME_IN_PROGRESS: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1006)
    });
    pub const r#RUNTIME_FP_OVERFLOW: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1007)
    });
    pub const r#SUBLIBRARY_LOADING_FAILED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1008)
    });
    pub const r#BAD_PARAM: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2000)
    });
    pub const r#BAD_PARAM_NULL_POINTER: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2002)
    });
    pub const r#BAD_PARAM_MISALIGNED_POINTER: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2003)
    });
    pub const r#BAD_PARAM_NOT_FINALIZED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2004)
    });
    pub const r#BAD_PARAM_OUT_OF_BOUND: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2005)
    });
    pub const r#BAD_PARAM_SIZE_INSUFFICIENT: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2006)
    });
    pub const r#BAD_PARAM_STREAM_MISMATCH: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2007)
    });
    pub const r#BAD_PARAM_SHAPE_MISMATCH: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2008)
    });
    pub const r#BAD_PARAM_DUPLICATED_ENTRIES: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2009)
    });
    pub const r#BAD_PARAM_ATTRIBUTE_TYPE: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2010)
    });
    pub const r#BAD_PARAM_CUDA_GRAPH_MISMATCH: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2011)
    });
    pub const r#BAD_PARAM_DESCRIPTOR_TYPE: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2012)
    });
    pub const r#NOT_SUPPORTED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3000)
    });
    pub const r#NOT_SUPPORTED_GRAPH_PATTERN: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3001)
    });
    pub const r#NOT_SUPPORTED_SHAPE: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3002)
    });
    pub const r#NOT_SUPPORTED_DATA_TYPE: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3003)
    });
    pub const r#NOT_SUPPORTED_LAYOUT: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3004)
    });
    pub const r#NOT_SUPPORTED_INCOMPATIBLE_CUDA_DRIVER: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3005)
    });
    pub const r#NOT_SUPPORTED_INCOMPATIBLE_CUDART: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3006)
    });
    pub const r#NOT_SUPPORTED_ARCH_MISMATCH: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3007)
    });
    pub const r#NOT_SUPPORTED_RUNTIME_PREREQUISITE_MISSING: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3008)
    });
    pub const r#NOT_SUPPORTED_SUBLIBRARY_UNAVAILABLE: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3009)
    });
    pub const r#NOT_SUPPORTED_SHARED_MEMORY_INSUFFICIENT: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3010)
    });
    pub const r#NOT_SUPPORTED_PADDING: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3011)
    });
    pub const r#NOT_SUPPORTED_BAD_LAUNCH_PARAM: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3012)
    });
    pub const r#NOT_SUPPORTED_CUDA_GRAPH_NATIVE_API: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3013)
    });
    pub const r#INTERNAL_ERROR: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4000)
    });
    pub const r#INTERNAL_ERROR_COMPILATION_FAILED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4001)
    });
    pub const r#INTERNAL_ERROR_UNEXPECTED_VALUE: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4002)
    });
    pub const r#INTERNAL_ERROR_HOST_ALLOCATION_FAILED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4003)
    });
    pub const r#INTERNAL_ERROR_DEVICE_ALLOCATION_FAILED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4004)
    });
    pub const r#INTERNAL_ERROR_BAD_LAUNCH_PARAM: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4005)
    });
    pub const r#INTERNAL_ERROR_TEXTURE_CREATION_FAILED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4006)
    });
    pub const r#EXECUTION_FAILED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5000)
    });
    pub const r#EXECUTION_FAILED_CUDA_DRIVER: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5001)
    });
    pub const r#EXECUTION_FAILED_CUBLAS: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5002)
    });
    pub const r#EXECUTION_FAILED_CUDART: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5003)
    });
    pub const r#EXECUTION_FAILED_CURAND: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5004)
    });
    pub const r#ALLOC_FAILED: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4003)
    });
    pub const r#INVALID_VALUE: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2001)
    });
    pub const r#ARCH_MISMATCH: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3007)
    });
    pub const r#MAPPING_ERROR: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4006)
    });
    pub const r#RUNTIME_PREREQUISITE_MISSING: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3008)
    });
    pub const r#VERSION_MISMATCH: cudnnError_t = cudnnError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1002)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct cudnnError_t(pub ::core::num::NonZeroU32);
pub trait cudnnStatus_tConsts {
    const SUCCESS: cudnnStatus_t = cudnnStatus_t::Ok(());
    const ERROR_NOT_INITIALIZED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_INITIALIZED,
    );
    const ERROR_SUBLIBRARY_VERSION_MISMATCH: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#SUBLIBRARY_VERSION_MISMATCH,
    );
    const ERROR_SERIALIZATION_VERSION_MISMATCH: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#SERIALIZATION_VERSION_MISMATCH,
    );
    const ERROR_DEPRECATED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#DEPRECATED,
    );
    const ERROR_LICENSE_ERROR: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#LICENSE_ERROR,
    );
    const ERROR_RUNTIME_IN_PROGRESS: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#RUNTIME_IN_PROGRESS,
    );
    const ERROR_RUNTIME_FP_OVERFLOW: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#RUNTIME_FP_OVERFLOW,
    );
    const ERROR_SUBLIBRARY_LOADING_FAILED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#SUBLIBRARY_LOADING_FAILED,
    );
    const ERROR_BAD_PARAM: cudnnStatus_t = cudnnStatus_t::Err(cudnnError_t::r#BAD_PARAM);
    const ERROR_BAD_PARAM_NULL_POINTER: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_NULL_POINTER,
    );
    const ERROR_BAD_PARAM_MISALIGNED_POINTER: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_MISALIGNED_POINTER,
    );
    const ERROR_BAD_PARAM_NOT_FINALIZED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_NOT_FINALIZED,
    );
    const ERROR_BAD_PARAM_OUT_OF_BOUND: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_OUT_OF_BOUND,
    );
    const ERROR_BAD_PARAM_SIZE_INSUFFICIENT: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_SIZE_INSUFFICIENT,
    );
    const ERROR_BAD_PARAM_STREAM_MISMATCH: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_STREAM_MISMATCH,
    );
    const ERROR_BAD_PARAM_SHAPE_MISMATCH: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_SHAPE_MISMATCH,
    );
    const ERROR_BAD_PARAM_DUPLICATED_ENTRIES: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_DUPLICATED_ENTRIES,
    );
    const ERROR_BAD_PARAM_ATTRIBUTE_TYPE: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_ATTRIBUTE_TYPE,
    );
    const ERROR_BAD_PARAM_CUDA_GRAPH_MISMATCH: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_CUDA_GRAPH_MISMATCH,
    );
    const ERROR_BAD_PARAM_DESCRIPTOR_TYPE: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#BAD_PARAM_DESCRIPTOR_TYPE,
    );
    const ERROR_NOT_SUPPORTED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED,
    );
    const ERROR_NOT_SUPPORTED_GRAPH_PATTERN: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_GRAPH_PATTERN,
    );
    const ERROR_NOT_SUPPORTED_SHAPE: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_SHAPE,
    );
    const ERROR_NOT_SUPPORTED_DATA_TYPE: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_DATA_TYPE,
    );
    const ERROR_NOT_SUPPORTED_LAYOUT: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_LAYOUT,
    );
    const ERROR_NOT_SUPPORTED_INCOMPATIBLE_CUDA_DRIVER: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_INCOMPATIBLE_CUDA_DRIVER,
    );
    const ERROR_NOT_SUPPORTED_INCOMPATIBLE_CUDART: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_INCOMPATIBLE_CUDART,
    );
    const ERROR_NOT_SUPPORTED_ARCH_MISMATCH: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_ARCH_MISMATCH,
    );
    const ERROR_NOT_SUPPORTED_RUNTIME_PREREQUISITE_MISSING: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_RUNTIME_PREREQUISITE_MISSING,
    );
    const ERROR_NOT_SUPPORTED_SUBLIBRARY_UNAVAILABLE: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_SUBLIBRARY_UNAVAILABLE,
    );
    const ERROR_NOT_SUPPORTED_SHARED_MEMORY_INSUFFICIENT: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_SHARED_MEMORY_INSUFFICIENT,
    );
    const ERROR_NOT_SUPPORTED_PADDING: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_PADDING,
    );
    const ERROR_NOT_SUPPORTED_BAD_LAUNCH_PARAM: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_BAD_LAUNCH_PARAM,
    );
    const ERROR_NOT_SUPPORTED_CUDA_GRAPH_NATIVE_API: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#NOT_SUPPORTED_CUDA_GRAPH_NATIVE_API,
    );
    const ERROR_INTERNAL_ERROR: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#INTERNAL_ERROR,
    );
    const ERROR_INTERNAL_ERROR_COMPILATION_FAILED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#INTERNAL_ERROR_COMPILATION_FAILED,
    );
    const ERROR_INTERNAL_ERROR_UNEXPECTED_VALUE: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#INTERNAL_ERROR_UNEXPECTED_VALUE,
    );
    const ERROR_INTERNAL_ERROR_HOST_ALLOCATION_FAILED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#INTERNAL_ERROR_HOST_ALLOCATION_FAILED,
    );
    const ERROR_INTERNAL_ERROR_DEVICE_ALLOCATION_FAILED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#INTERNAL_ERROR_DEVICE_ALLOCATION_FAILED,
    );
    const ERROR_INTERNAL_ERROR_BAD_LAUNCH_PARAM: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#INTERNAL_ERROR_BAD_LAUNCH_PARAM,
    );
    const ERROR_INTERNAL_ERROR_TEXTURE_CREATION_FAILED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#INTERNAL_ERROR_TEXTURE_CREATION_FAILED,
    );
    const ERROR_EXECUTION_FAILED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#EXECUTION_FAILED,
    );
    const ERROR_EXECUTION_FAILED_CUDA_DRIVER: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#EXECUTION_FAILED_CUDA_DRIVER,
    );
    const ERROR_EXECUTION_FAILED_CUBLAS: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#EXECUTION_FAILED_CUBLAS,
    );
    const ERROR_EXECUTION_FAILED_CUDART: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#EXECUTION_FAILED_CUDART,
    );
    const ERROR_EXECUTION_FAILED_CURAND: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#EXECUTION_FAILED_CURAND,
    );
    const ERROR_ALLOC_FAILED: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#ALLOC_FAILED,
    );
    const ERROR_INVALID_VALUE: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#INVALID_VALUE,
    );
    const ERROR_ARCH_MISMATCH: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#ARCH_MISMATCH,
    );
    const ERROR_MAPPING_ERROR: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#MAPPING_ERROR,
    );
    const ERROR_RUNTIME_PREREQUISITE_MISSING: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#RUNTIME_PREREQUISITE_MISSING,
    );
    const ERROR_VERSION_MISMATCH: cudnnStatus_t = cudnnStatus_t::Err(
        cudnnError_t::r#VERSION_MISMATCH,
    );
}
impl cudnnStatus_tConsts for cudnnStatus_t {}
#[must_use]
pub type cudnnStatus_t = ::core::result::Result<(), cudnnError_t>;
const _: fn() = || {
    let _ = std::mem::transmute::<cudnnStatus_t, u32>;
};
impl From<miopen_sys::miopenError_t> for cudnnError_t {
    fn from(error: miopen_sys::miopenError_t) -> Self {
        match error {
            miopen_sys::miopenError_t::NotInitialized => cudnnError_t::NOT_INITIALIZED,
            miopen_sys::miopenError_t::InvalidValue => cudnnError_t::INVALID_VALUE,
            miopen_sys::miopenError_t::BadParm => cudnnError_t::BAD_PARAM,
            miopen_sys::miopenError_t::AllocFailed => cudnnError_t::ALLOC_FAILED,
            miopen_sys::miopenError_t::InternalError => cudnnError_t::INTERNAL_ERROR,
            miopen_sys::miopenError_t::NotImplemented | miopen_sys::miopenError_t::UnsupportedOp => cudnnError_t::NOT_SUPPORTED,
            miopen_sys::miopenError_t::VersionMismatch => cudnnError_t::VERSION_MISMATCH,
            _ => cudnnError_t::INTERNAL_ERROR,
        }
    }
}