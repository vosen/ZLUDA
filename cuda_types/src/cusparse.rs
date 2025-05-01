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
pub const CUSPARSE_VER_MAJOR: u32 = 12;
pub const CUSPARSE_VER_MINOR: u32 = 5;
pub const CUSPARSE_VER_PATCH: u32 = 8;
pub const CUSPARSE_VER_BUILD: u32 = 93;
pub const CUSPARSE_VERSION: u32 = 12508;
/// Result information returned by cudaGraphExecUpdate
pub type cudaGraphExecUpdateResultInfo = cudaGraphExecUpdateResultInfo_st;
/// Information describing an async notification event
#[repr(C)]
pub struct cudaAsyncNotificationInfo {
    pub type_: cudaAsyncNotificationType,
    pub info: cudaAsyncNotificationInfo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaAsyncNotificationInfo__bindgen_ty_1 {
    pub overBudget: cudaAsyncNotificationInfo__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cudaAsyncNotificationInfo__bindgen_ty_1__bindgen_ty_1 {
    pub bytesOverBudget: ::core::ffi::c_ulonglong,
}
/// Information describing an async notification event
pub type cudaAsyncNotificationInfo_t = cudaAsyncNotificationInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseContext {
    _unused: [u8; 0],
}
pub type cusparseHandle_t = *mut cusparseContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseMatDescr {
    _unused: [u8; 0],
}
pub type cusparseMatDescr_t = *mut cusparseMatDescr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsrsv2Info {
    _unused: [u8; 0],
}
pub type bsrsv2Info_t = *mut bsrsv2Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsrsm2Info {
    _unused: [u8; 0],
}
pub type bsrsm2Info_t = *mut bsrsm2Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csric02Info {
    _unused: [u8; 0],
}
pub type csric02Info_t = *mut csric02Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsric02Info {
    _unused: [u8; 0],
}
pub type bsric02Info_t = *mut bsric02Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csrilu02Info {
    _unused: [u8; 0],
}
pub type csrilu02Info_t = *mut csrilu02Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsrilu02Info {
    _unused: [u8; 0],
}
pub type bsrilu02Info_t = *mut bsrilu02Info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csru2csrInfo {
    _unused: [u8; 0],
}
pub type csru2csrInfo_t = *mut csru2csrInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseColorInfo {
    _unused: [u8; 0],
}
pub type cusparseColorInfo_t = *mut cusparseColorInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pruneInfo {
    _unused: [u8; 0],
}
pub type pruneInfo_t = *mut pruneInfo;
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_SUCCESS: cusparseStatus_t = cusparseStatus_t(0);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_NOT_INITIALIZED: cusparseStatus_t = cusparseStatus_t(1);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_ALLOC_FAILED: cusparseStatus_t = cusparseStatus_t(2);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_INVALID_VALUE: cusparseStatus_t = cusparseStatus_t(3);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_ARCH_MISMATCH: cusparseStatus_t = cusparseStatus_t(4);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_MAPPING_ERROR: cusparseStatus_t = cusparseStatus_t(5);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_EXECUTION_FAILED: cusparseStatus_t = cusparseStatus_t(6);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_INTERNAL_ERROR: cusparseStatus_t = cusparseStatus_t(7);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_MATRIX_TYPE_NOT_SUPPORTED: cusparseStatus_t = cusparseStatus_t(
        8,
    );
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_ZERO_PIVOT: cusparseStatus_t = cusparseStatus_t(9);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_NOT_SUPPORTED: cusparseStatus_t = cusparseStatus_t(10);
}
impl cusparseStatus_t {
    pub const CUSPARSE_STATUS_INSUFFICIENT_RESOURCES: cusparseStatus_t = cusparseStatus_t(
        11,
    );
}
#[repr(transparent)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseStatus_t(pub ::core::ffi::c_uint);
impl cusparsePointerMode_t {
    pub const CUSPARSE_POINTER_MODE_HOST: cusparsePointerMode_t = cusparsePointerMode_t(
        0,
    );
}
impl cusparsePointerMode_t {
    pub const CUSPARSE_POINTER_MODE_DEVICE: cusparsePointerMode_t = cusparsePointerMode_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparsePointerMode_t(pub ::core::ffi::c_uint);
impl cusparseAction_t {
    pub const CUSPARSE_ACTION_SYMBOLIC: cusparseAction_t = cusparseAction_t(0);
}
impl cusparseAction_t {
    pub const CUSPARSE_ACTION_NUMERIC: cusparseAction_t = cusparseAction_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseAction_t(pub ::core::ffi::c_uint);
impl cusparseMatrixType_t {
    pub const CUSPARSE_MATRIX_TYPE_GENERAL: cusparseMatrixType_t = cusparseMatrixType_t(
        0,
    );
}
impl cusparseMatrixType_t {
    pub const CUSPARSE_MATRIX_TYPE_SYMMETRIC: cusparseMatrixType_t = cusparseMatrixType_t(
        1,
    );
}
impl cusparseMatrixType_t {
    pub const CUSPARSE_MATRIX_TYPE_HERMITIAN: cusparseMatrixType_t = cusparseMatrixType_t(
        2,
    );
}
impl cusparseMatrixType_t {
    pub const CUSPARSE_MATRIX_TYPE_TRIANGULAR: cusparseMatrixType_t = cusparseMatrixType_t(
        3,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseMatrixType_t(pub ::core::ffi::c_uint);
impl cusparseFillMode_t {
    pub const CUSPARSE_FILL_MODE_LOWER: cusparseFillMode_t = cusparseFillMode_t(0);
}
impl cusparseFillMode_t {
    pub const CUSPARSE_FILL_MODE_UPPER: cusparseFillMode_t = cusparseFillMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseFillMode_t(pub ::core::ffi::c_uint);
impl cusparseDiagType_t {
    pub const CUSPARSE_DIAG_TYPE_NON_UNIT: cusparseDiagType_t = cusparseDiagType_t(0);
}
impl cusparseDiagType_t {
    pub const CUSPARSE_DIAG_TYPE_UNIT: cusparseDiagType_t = cusparseDiagType_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseDiagType_t(pub ::core::ffi::c_uint);
impl cusparseIndexBase_t {
    pub const CUSPARSE_INDEX_BASE_ZERO: cusparseIndexBase_t = cusparseIndexBase_t(0);
}
impl cusparseIndexBase_t {
    pub const CUSPARSE_INDEX_BASE_ONE: cusparseIndexBase_t = cusparseIndexBase_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseIndexBase_t(pub ::core::ffi::c_uint);
impl cusparseOperation_t {
    pub const CUSPARSE_OPERATION_NON_TRANSPOSE: cusparseOperation_t = cusparseOperation_t(
        0,
    );
}
impl cusparseOperation_t {
    pub const CUSPARSE_OPERATION_TRANSPOSE: cusparseOperation_t = cusparseOperation_t(1);
}
impl cusparseOperation_t {
    pub const CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE: cusparseOperation_t = cusparseOperation_t(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseOperation_t(pub ::core::ffi::c_uint);
impl cusparseDirection_t {
    pub const CUSPARSE_DIRECTION_ROW: cusparseDirection_t = cusparseDirection_t(0);
}
impl cusparseDirection_t {
    pub const CUSPARSE_DIRECTION_COLUMN: cusparseDirection_t = cusparseDirection_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseDirection_t(pub ::core::ffi::c_uint);
impl cusparseSolvePolicy_t {
    pub const CUSPARSE_SOLVE_POLICY_NO_LEVEL: cusparseSolvePolicy_t = cusparseSolvePolicy_t(
        0,
    );
}
impl cusparseSolvePolicy_t {
    pub const CUSPARSE_SOLVE_POLICY_USE_LEVEL: cusparseSolvePolicy_t = cusparseSolvePolicy_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSolvePolicy_t(pub ::core::ffi::c_uint);
impl cusparseColorAlg_t {
    pub const CUSPARSE_COLOR_ALG0: cusparseColorAlg_t = cusparseColorAlg_t(0);
}
impl cusparseColorAlg_t {
    pub const CUSPARSE_COLOR_ALG1: cusparseColorAlg_t = cusparseColorAlg_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseColorAlg_t(pub ::core::ffi::c_uint);
pub type cusparseLoggerCallback_t = ::core::option::Option<
    unsafe extern "C" fn(
        logLevel: ::core::ffi::c_int,
        functionName: *const ::core::ffi::c_char,
        message: *const ::core::ffi::c_char,
    ),
>;
impl cusparseCsr2CscAlg_t {
    pub const CUSPARSE_CSR2CSC_ALG_DEFAULT: cusparseCsr2CscAlg_t = cusparseCsr2CscAlg_t(
        1,
    );
}
impl cusparseCsr2CscAlg_t {
    pub const CUSPARSE_CSR2CSC_ALG1: cusparseCsr2CscAlg_t = cusparseCsr2CscAlg_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseCsr2CscAlg_t(pub ::core::ffi::c_uint);
impl cusparseFormat_t {
    ///< Compressed Sparse Row (CSR)
    pub const CUSPARSE_FORMAT_CSR: cusparseFormat_t = cusparseFormat_t(1);
}
impl cusparseFormat_t {
    ///< Compressed Sparse Column (CSC)
    pub const CUSPARSE_FORMAT_CSC: cusparseFormat_t = cusparseFormat_t(2);
}
impl cusparseFormat_t {
    ///< Coordinate (COO) - Structure of Arrays
    pub const CUSPARSE_FORMAT_COO: cusparseFormat_t = cusparseFormat_t(3);
}
impl cusparseFormat_t {
    ///< Blocked ELL
    pub const CUSPARSE_FORMAT_BLOCKED_ELL: cusparseFormat_t = cusparseFormat_t(5);
}
impl cusparseFormat_t {
    ///< Blocked Compressed Sparse Row (BSR)
    pub const CUSPARSE_FORMAT_BSR: cusparseFormat_t = cusparseFormat_t(6);
}
impl cusparseFormat_t {
    ///< Sliced ELL
    pub const CUSPARSE_FORMAT_SLICED_ELLPACK: cusparseFormat_t = cusparseFormat_t(7);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseFormat_t(pub ::core::ffi::c_uint);
impl cusparseOrder_t {
    ///< Column-Major Order - Matrix memory layout
    pub const CUSPARSE_ORDER_COL: cusparseOrder_t = cusparseOrder_t(1);
}
impl cusparseOrder_t {
    ///< Row-Major Order - Matrix memory layout
    pub const CUSPARSE_ORDER_ROW: cusparseOrder_t = cusparseOrder_t(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseOrder_t(pub ::core::ffi::c_uint);
impl cusparseIndexType_t {
    /**< 16-bit unsigned integer for matrix/vector
< indices*/
    pub const CUSPARSE_INDEX_16U: cusparseIndexType_t = cusparseIndexType_t(1);
}
impl cusparseIndexType_t {
    ///< 32-bit signed integer for matrix/vector indices
    pub const CUSPARSE_INDEX_32I: cusparseIndexType_t = cusparseIndexType_t(2);
}
impl cusparseIndexType_t {
    ///< 64-bit signed integer for matrix/vector indices
    pub const CUSPARSE_INDEX_64I: cusparseIndexType_t = cusparseIndexType_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseIndexType_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpVecDescr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseDnVecDescr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpMatDescr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseDnMatDescr {
    _unused: [u8; 0],
}
pub type cusparseSpVecDescr_t = *mut cusparseSpVecDescr;
pub type cusparseDnVecDescr_t = *mut cusparseDnVecDescr;
pub type cusparseSpMatDescr_t = *mut cusparseSpMatDescr;
pub type cusparseDnMatDescr_t = *mut cusparseDnMatDescr;
pub type cusparseConstSpVecDescr_t = *const cusparseSpVecDescr;
pub type cusparseConstDnVecDescr_t = *const cusparseDnVecDescr;
pub type cusparseConstSpMatDescr_t = *const cusparseSpMatDescr;
pub type cusparseConstDnMatDescr_t = *const cusparseDnMatDescr;
impl cusparseSpMatAttribute_t {
    pub const CUSPARSE_SPMAT_FILL_MODE: cusparseSpMatAttribute_t = cusparseSpMatAttribute_t(
        0,
    );
}
impl cusparseSpMatAttribute_t {
    pub const CUSPARSE_SPMAT_DIAG_TYPE: cusparseSpMatAttribute_t = cusparseSpMatAttribute_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSpMatAttribute_t(pub ::core::ffi::c_uint);
impl cusparseSparseToDenseAlg_t {
    pub const CUSPARSE_SPARSETODENSE_ALG_DEFAULT: cusparseSparseToDenseAlg_t = cusparseSparseToDenseAlg_t(
        0,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSparseToDenseAlg_t(pub ::core::ffi::c_uint);
impl cusparseDenseToSparseAlg_t {
    pub const CUSPARSE_DENSETOSPARSE_ALG_DEFAULT: cusparseDenseToSparseAlg_t = cusparseDenseToSparseAlg_t(
        0,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseDenseToSparseAlg_t(pub ::core::ffi::c_uint);
impl cusparseSpMVAlg_t {
    pub const CUSPARSE_SPMV_ALG_DEFAULT: cusparseSpMVAlg_t = cusparseSpMVAlg_t(0);
}
impl cusparseSpMVAlg_t {
    pub const CUSPARSE_SPMV_CSR_ALG1: cusparseSpMVAlg_t = cusparseSpMVAlg_t(2);
}
impl cusparseSpMVAlg_t {
    pub const CUSPARSE_SPMV_CSR_ALG2: cusparseSpMVAlg_t = cusparseSpMVAlg_t(3);
}
impl cusparseSpMVAlg_t {
    pub const CUSPARSE_SPMV_COO_ALG1: cusparseSpMVAlg_t = cusparseSpMVAlg_t(1);
}
impl cusparseSpMVAlg_t {
    pub const CUSPARSE_SPMV_COO_ALG2: cusparseSpMVAlg_t = cusparseSpMVAlg_t(4);
}
impl cusparseSpMVAlg_t {
    pub const CUSPARSE_SPMV_SELL_ALG1: cusparseSpMVAlg_t = cusparseSpMVAlg_t(5);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSpMVAlg_t(pub ::core::ffi::c_uint);
impl cusparseSpSVAlg_t {
    pub const CUSPARSE_SPSV_ALG_DEFAULT: cusparseSpSVAlg_t = cusparseSpSVAlg_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSpSVAlg_t(pub ::core::ffi::c_uint);
impl cusparseSpSVUpdate_t {
    pub const CUSPARSE_SPSV_UPDATE_GENERAL: cusparseSpSVUpdate_t = cusparseSpSVUpdate_t(
        0,
    );
}
impl cusparseSpSVUpdate_t {
    pub const CUSPARSE_SPSV_UPDATE_DIAGONAL: cusparseSpSVUpdate_t = cusparseSpSVUpdate_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSpSVUpdate_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpSVDescr {
    _unused: [u8; 0],
}
pub type cusparseSpSVDescr_t = *mut cusparseSpSVDescr;
impl cusparseSpSMAlg_t {
    pub const CUSPARSE_SPSM_ALG_DEFAULT: cusparseSpSMAlg_t = cusparseSpSMAlg_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSpSMAlg_t(pub ::core::ffi::c_uint);
impl cusparseSpSMUpdate_t {
    pub const CUSPARSE_SPSM_UPDATE_GENERAL: cusparseSpSMUpdate_t = cusparseSpSMUpdate_t(
        0,
    );
}
impl cusparseSpSMUpdate_t {
    pub const CUSPARSE_SPSM_UPDATE_DIAGONAL: cusparseSpSMUpdate_t = cusparseSpSMUpdate_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSpSMUpdate_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpSMDescr {
    _unused: [u8; 0],
}
pub type cusparseSpSMDescr_t = *mut cusparseSpSMDescr;
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_ALG_DEFAULT: cusparseSpMMAlg_t = cusparseSpMMAlg_t(0);
}
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_COO_ALG1: cusparseSpMMAlg_t = cusparseSpMMAlg_t(1);
}
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_COO_ALG2: cusparseSpMMAlg_t = cusparseSpMMAlg_t(2);
}
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_COO_ALG3: cusparseSpMMAlg_t = cusparseSpMMAlg_t(3);
}
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_COO_ALG4: cusparseSpMMAlg_t = cusparseSpMMAlg_t(5);
}
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_CSR_ALG1: cusparseSpMMAlg_t = cusparseSpMMAlg_t(4);
}
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_CSR_ALG2: cusparseSpMMAlg_t = cusparseSpMMAlg_t(6);
}
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_CSR_ALG3: cusparseSpMMAlg_t = cusparseSpMMAlg_t(12);
}
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_BLOCKED_ELL_ALG1: cusparseSpMMAlg_t = cusparseSpMMAlg_t(13);
}
impl cusparseSpMMAlg_t {
    pub const CUSPARSE_SPMM_BSR_ALG1: cusparseSpMMAlg_t = cusparseSpMMAlg_t(14);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSpMMAlg_t(pub ::core::ffi::c_uint);
impl cusparseSpGEMMAlg_t {
    pub const CUSPARSE_SPGEMM_DEFAULT: cusparseSpGEMMAlg_t = cusparseSpGEMMAlg_t(0);
}
impl cusparseSpGEMMAlg_t {
    pub const CUSPARSE_SPGEMM_CSR_ALG_DETERMINITIC: cusparseSpGEMMAlg_t = cusparseSpGEMMAlg_t(
        1,
    );
}
impl cusparseSpGEMMAlg_t {
    pub const CUSPARSE_SPGEMM_CSR_ALG_NONDETERMINITIC: cusparseSpGEMMAlg_t = cusparseSpGEMMAlg_t(
        2,
    );
}
impl cusparseSpGEMMAlg_t {
    pub const CUSPARSE_SPGEMM_ALG1: cusparseSpGEMMAlg_t = cusparseSpGEMMAlg_t(3);
}
impl cusparseSpGEMMAlg_t {
    pub const CUSPARSE_SPGEMM_ALG2: cusparseSpGEMMAlg_t = cusparseSpGEMMAlg_t(4);
}
impl cusparseSpGEMMAlg_t {
    pub const CUSPARSE_SPGEMM_ALG3: cusparseSpGEMMAlg_t = cusparseSpGEMMAlg_t(5);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSpGEMMAlg_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpGEMMDescr {
    _unused: [u8; 0],
}
pub type cusparseSpGEMMDescr_t = *mut cusparseSpGEMMDescr;
impl cusparseSDDMMAlg_t {
    pub const CUSPARSE_SDDMM_ALG_DEFAULT: cusparseSDDMMAlg_t = cusparseSDDMMAlg_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSDDMMAlg_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cusparseSpMMOpPlan {
    _unused: [u8; 0],
}
pub type cusparseSpMMOpPlan_t = *mut cusparseSpMMOpPlan;
impl cusparseSpMMOpAlg_t {
    pub const CUSPARSE_SPMM_OP_ALG_DEFAULT: cusparseSpMMOpAlg_t = cusparseSpMMOpAlg_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cusparseSpMMOpAlg_t(pub ::core::ffi::c_uint);
