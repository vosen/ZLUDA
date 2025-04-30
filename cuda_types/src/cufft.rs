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
pub const CUFFT_VER_MAJOR: u32 = 11;
pub const CUFFT_VER_MINOR: u32 = 3;
pub const CUFFT_VER_PATCH: u32 = 3;
pub const CUFFT_VER_BUILD: u32 = 83;
pub const CUFFT_VERSION: u32 = 11303;
pub const CUFFT_FORWARD: i32 = -1;
pub const CUFFT_INVERSE: u32 = 1;
impl libFormat_t {
    pub const LIB_FORMAT_CUFFT: libFormat_t = libFormat_t(0);
}
impl libFormat_t {
    pub const LIB_FORMAT_UNDEFINED: libFormat_t = libFormat_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct libFormat_t(pub ::core::ffi::c_uint);
pub use self::libFormat_t as libFormat;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudaXtDesc_t {
    pub version: ::core::ffi::c_int,
    pub nGPUs: ::core::ffi::c_int,
    pub GPUs: [::core::ffi::c_int; 64usize],
    pub data: [*mut ::core::ffi::c_void; 64usize],
    pub size: [usize; 64usize],
    pub cudaXtState: *mut ::core::ffi::c_void,
}
pub type cudaXtDesc = cudaXtDesc_t;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudaLibXtDesc_t {
    pub version: ::core::ffi::c_int,
    pub descriptor: *mut cudaXtDesc,
    pub library: libFormat,
    pub subFormat: ::core::ffi::c_int,
    pub libDescriptor: *mut ::core::ffi::c_void,
}
pub type cudaLibXtDesc = cudaLibXtDesc_t;
impl cufftResult_t {
    pub const CUFFT_SUCCESS: cufftResult_t = cufftResult_t(0);
}
impl cufftResult_t {
    pub const CUFFT_INVALID_PLAN: cufftResult_t = cufftResult_t(1);
}
impl cufftResult_t {
    pub const CUFFT_ALLOC_FAILED: cufftResult_t = cufftResult_t(2);
}
impl cufftResult_t {
    pub const CUFFT_INVALID_TYPE: cufftResult_t = cufftResult_t(3);
}
impl cufftResult_t {
    pub const CUFFT_INVALID_VALUE: cufftResult_t = cufftResult_t(4);
}
impl cufftResult_t {
    pub const CUFFT_INTERNAL_ERROR: cufftResult_t = cufftResult_t(5);
}
impl cufftResult_t {
    pub const CUFFT_EXEC_FAILED: cufftResult_t = cufftResult_t(6);
}
impl cufftResult_t {
    pub const CUFFT_SETUP_FAILED: cufftResult_t = cufftResult_t(7);
}
impl cufftResult_t {
    pub const CUFFT_INVALID_SIZE: cufftResult_t = cufftResult_t(8);
}
impl cufftResult_t {
    pub const CUFFT_UNALIGNED_DATA: cufftResult_t = cufftResult_t(9);
}
impl cufftResult_t {
    pub const CUFFT_INCOMPLETE_PARAMETER_LIST: cufftResult_t = cufftResult_t(10);
}
impl cufftResult_t {
    pub const CUFFT_INVALID_DEVICE: cufftResult_t = cufftResult_t(11);
}
impl cufftResult_t {
    pub const CUFFT_PARSE_ERROR: cufftResult_t = cufftResult_t(12);
}
impl cufftResult_t {
    pub const CUFFT_NO_WORKSPACE: cufftResult_t = cufftResult_t(13);
}
impl cufftResult_t {
    pub const CUFFT_NOT_IMPLEMENTED: cufftResult_t = cufftResult_t(14);
}
impl cufftResult_t {
    pub const CUFFT_LICENSE_ERROR: cufftResult_t = cufftResult_t(15);
}
impl cufftResult_t {
    pub const CUFFT_NOT_SUPPORTED: cufftResult_t = cufftResult_t(16);
}
#[repr(transparent)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftResult_t(pub ::core::ffi::c_uint);
pub use self::cufftResult_t as cufftResult;
pub type cufftReal = f32;
pub type cufftDoubleReal = f64;
pub type cufftComplex = super::cuda::cuComplex;
pub type cufftDoubleComplex = super::cuda::cuDoubleComplex;
impl cufftType_t {
    pub const CUFFT_R2C: cufftType_t = cufftType_t(42);
}
impl cufftType_t {
    pub const CUFFT_C2R: cufftType_t = cufftType_t(44);
}
impl cufftType_t {
    pub const CUFFT_C2C: cufftType_t = cufftType_t(41);
}
impl cufftType_t {
    pub const CUFFT_D2Z: cufftType_t = cufftType_t(106);
}
impl cufftType_t {
    pub const CUFFT_Z2D: cufftType_t = cufftType_t(108);
}
impl cufftType_t {
    pub const CUFFT_Z2Z: cufftType_t = cufftType_t(105);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftType_t(pub ::core::ffi::c_uint);
pub use self::cufftType_t as cufftType;
impl cufftCompatibility_t {
    pub const CUFFT_COMPATIBILITY_FFTW_PADDING: cufftCompatibility_t = cufftCompatibility_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftCompatibility_t(pub ::core::ffi::c_uint);
pub use self::cufftCompatibility_t as cufftCompatibility;
pub type cufftHandle = ::core::ffi::c_int;
impl cufftProperty_t {
    pub const NVFFT_PLAN_PROPERTY_INT64_PATIENT_JIT: cufftProperty_t = cufftProperty_t(
        1,
    );
}
impl cufftProperty_t {
    pub const NVFFT_PLAN_PROPERTY_INT64_MAX_NUM_HOST_THREADS: cufftProperty_t = cufftProperty_t(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftProperty_t(pub ::core::ffi::c_uint);
pub use self::cufftProperty_t as cufftProperty;
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_INPUT: cufftXtSubFormat_t = cufftXtSubFormat_t(0);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_OUTPUT: cufftXtSubFormat_t = cufftXtSubFormat_t(1);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_INPLACE: cufftXtSubFormat_t = cufftXtSubFormat_t(2);
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_INPLACE_SHUFFLED: cufftXtSubFormat_t = cufftXtSubFormat_t(
        3,
    );
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_1D_INPUT_SHUFFLED: cufftXtSubFormat_t = cufftXtSubFormat_t(
        4,
    );
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_DISTRIBUTED_INPUT: cufftXtSubFormat_t = cufftXtSubFormat_t(
        5,
    );
}
impl cufftXtSubFormat_t {
    pub const CUFFT_XT_FORMAT_DISTRIBUTED_OUTPUT: cufftXtSubFormat_t = cufftXtSubFormat_t(
        6,
    );
}
impl cufftXtSubFormat_t {
    pub const CUFFT_FORMAT_UNDEFINED: cufftXtSubFormat_t = cufftXtSubFormat_t(7);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtSubFormat_t(pub ::core::ffi::c_uint);
pub use self::cufftXtSubFormat_t as cufftXtSubFormat;
impl cufftXtCopyType_t {
    pub const CUFFT_COPY_HOST_TO_DEVICE: cufftXtCopyType_t = cufftXtCopyType_t(0);
}
impl cufftXtCopyType_t {
    pub const CUFFT_COPY_DEVICE_TO_HOST: cufftXtCopyType_t = cufftXtCopyType_t(1);
}
impl cufftXtCopyType_t {
    pub const CUFFT_COPY_DEVICE_TO_DEVICE: cufftXtCopyType_t = cufftXtCopyType_t(2);
}
impl cufftXtCopyType_t {
    pub const CUFFT_COPY_UNDEFINED: cufftXtCopyType_t = cufftXtCopyType_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtCopyType_t(pub ::core::ffi::c_uint);
pub use self::cufftXtCopyType_t as cufftXtCopyType;
impl cufftXtQueryType_t {
    pub const CUFFT_QUERY_1D_FACTORS: cufftXtQueryType_t = cufftXtQueryType_t(0);
}
impl cufftXtQueryType_t {
    pub const CUFFT_QUERY_UNDEFINED: cufftXtQueryType_t = cufftXtQueryType_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtQueryType_t(pub ::core::ffi::c_uint);
pub use self::cufftXtQueryType_t as cufftXtQueryType;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXt1dFactors_t {
    pub size: ::core::ffi::c_longlong,
    pub stringCount: ::core::ffi::c_longlong,
    pub stringLength: ::core::ffi::c_longlong,
    pub substringLength: ::core::ffi::c_longlong,
    pub factor1: ::core::ffi::c_longlong,
    pub factor2: ::core::ffi::c_longlong,
    pub stringMask: ::core::ffi::c_longlong,
    pub substringMask: ::core::ffi::c_longlong,
    pub factor1Mask: ::core::ffi::c_longlong,
    pub factor2Mask: ::core::ffi::c_longlong,
    pub stringShift: ::core::ffi::c_int,
    pub substringShift: ::core::ffi::c_int,
    pub factor1Shift: ::core::ffi::c_int,
    pub factor2Shift: ::core::ffi::c_int,
}
pub type cufftXt1dFactors = cufftXt1dFactors_t;
impl cufftXtWorkAreaPolicy_t {
    pub const CUFFT_WORKAREA_MINIMAL: cufftXtWorkAreaPolicy_t = cufftXtWorkAreaPolicy_t(
        0,
    );
}
impl cufftXtWorkAreaPolicy_t {
    pub const CUFFT_WORKAREA_USER: cufftXtWorkAreaPolicy_t = cufftXtWorkAreaPolicy_t(1);
}
impl cufftXtWorkAreaPolicy_t {
    pub const CUFFT_WORKAREA_PERFORMANCE: cufftXtWorkAreaPolicy_t = cufftXtWorkAreaPolicy_t(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtWorkAreaPolicy_t(pub ::core::ffi::c_uint);
pub use self::cufftXtWorkAreaPolicy_t as cufftXtWorkAreaPolicy;
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_LD_COMPLEX: cufftXtCallbackType_t = cufftXtCallbackType_t(0);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_LD_COMPLEX_DOUBLE: cufftXtCallbackType_t = cufftXtCallbackType_t(
        1,
    );
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_LD_REAL: cufftXtCallbackType_t = cufftXtCallbackType_t(2);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_LD_REAL_DOUBLE: cufftXtCallbackType_t = cufftXtCallbackType_t(3);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_ST_COMPLEX: cufftXtCallbackType_t = cufftXtCallbackType_t(4);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_ST_COMPLEX_DOUBLE: cufftXtCallbackType_t = cufftXtCallbackType_t(
        5,
    );
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_ST_REAL: cufftXtCallbackType_t = cufftXtCallbackType_t(6);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_ST_REAL_DOUBLE: cufftXtCallbackType_t = cufftXtCallbackType_t(7);
}
impl cufftXtCallbackType_t {
    pub const CUFFT_CB_UNDEFINED: cufftXtCallbackType_t = cufftXtCallbackType_t(8);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cufftXtCallbackType_t(pub ::core::ffi::c_uint);
pub use self::cufftXtCallbackType_t as cufftXtCallbackType;
pub type cufftCallbackLoadC = ::core::option::Option<
    unsafe extern "C" fn(
        dataIn: *mut ::core::ffi::c_void,
        offset: usize,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ) -> cufftComplex,
>;
pub type cufftCallbackLoadZ = ::core::option::Option<
    unsafe extern "C" fn(
        dataIn: *mut ::core::ffi::c_void,
        offset: usize,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ) -> cufftDoubleComplex,
>;
pub type cufftCallbackLoadR = ::core::option::Option<
    unsafe extern "C" fn(
        dataIn: *mut ::core::ffi::c_void,
        offset: usize,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ) -> cufftReal,
>;
pub type cufftCallbackLoadD = ::core::option::Option<
    unsafe extern "C" fn(
        dataIn: *mut ::core::ffi::c_void,
        offset: usize,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ) -> cufftDoubleReal,
>;
pub type cufftCallbackStoreC = ::core::option::Option<
    unsafe extern "C" fn(
        dataOut: *mut ::core::ffi::c_void,
        offset: usize,
        element: cufftComplex,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ),
>;
pub type cufftCallbackStoreZ = ::core::option::Option<
    unsafe extern "C" fn(
        dataOut: *mut ::core::ffi::c_void,
        offset: usize,
        element: cufftDoubleComplex,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ),
>;
pub type cufftCallbackStoreR = ::core::option::Option<
    unsafe extern "C" fn(
        dataOut: *mut ::core::ffi::c_void,
        offset: usize,
        element: cufftReal,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ),
>;
pub type cufftCallbackStoreD = ::core::option::Option<
    unsafe extern "C" fn(
        dataOut: *mut ::core::ffi::c_void,
        offset: usize,
        element: cufftDoubleReal,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ),
>;
pub type cufftJITCallbackLoadC = ::core::option::Option<
    unsafe extern "C" fn(
        dataIn: *mut ::core::ffi::c_void,
        offset: ::core::ffi::c_ulonglong,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ) -> cufftComplex,
>;
pub type cufftJITCallbackLoadZ = ::core::option::Option<
    unsafe extern "C" fn(
        dataIn: *mut ::core::ffi::c_void,
        offset: ::core::ffi::c_ulonglong,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ) -> cufftDoubleComplex,
>;
pub type cufftJITCallbackLoadR = ::core::option::Option<
    unsafe extern "C" fn(
        dataIn: *mut ::core::ffi::c_void,
        offset: ::core::ffi::c_ulonglong,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ) -> cufftReal,
>;
pub type cufftJITCallbackLoadD = ::core::option::Option<
    unsafe extern "C" fn(
        dataIn: *mut ::core::ffi::c_void,
        offset: ::core::ffi::c_ulonglong,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ) -> cufftDoubleReal,
>;
pub type cufftJITCallbackStoreC = ::core::option::Option<
    unsafe extern "C" fn(
        dataOut: *mut ::core::ffi::c_void,
        offset: ::core::ffi::c_ulonglong,
        element: cufftComplex,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ),
>;
pub type cufftJITCallbackStoreZ = ::core::option::Option<
    unsafe extern "C" fn(
        dataOut: *mut ::core::ffi::c_void,
        offset: ::core::ffi::c_ulonglong,
        element: cufftDoubleComplex,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ),
>;
pub type cufftJITCallbackStoreR = ::core::option::Option<
    unsafe extern "C" fn(
        dataOut: *mut ::core::ffi::c_void,
        offset: ::core::ffi::c_ulonglong,
        element: cufftReal,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ),
>;
pub type cufftJITCallbackStoreD = ::core::option::Option<
    unsafe extern "C" fn(
        dataOut: *mut ::core::ffi::c_void,
        offset: ::core::ffi::c_ulonglong,
        element: cufftDoubleReal,
        callerInfo: *mut ::core::ffi::c_void,
        sharedPointer: *mut ::core::ffi::c_void,
    ),
>;
