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
pub const CUBLAS_VER_MAJOR: u32 = 12;
pub const CUBLAS_VER_MINOR: u32 = 8;
pub const CUBLAS_VER_PATCH: u32 = 4;
pub const CUBLAS_VER_BUILD: u32 = 1;
pub const CUBLAS_VERSION: u32 = 120804;
impl cublasFillMode_t {
    pub const CUBLAS_FILL_MODE_LOWER: cublasFillMode_t = cublasFillMode_t(0);
}
impl cublasFillMode_t {
    pub const CUBLAS_FILL_MODE_UPPER: cublasFillMode_t = cublasFillMode_t(1);
}
impl cublasFillMode_t {
    pub const CUBLAS_FILL_MODE_FULL: cublasFillMode_t = cublasFillMode_t(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasFillMode_t(pub ::core::ffi::c_uint);
impl cublasDiagType_t {
    pub const CUBLAS_DIAG_NON_UNIT: cublasDiagType_t = cublasDiagType_t(0);
}
impl cublasDiagType_t {
    pub const CUBLAS_DIAG_UNIT: cublasDiagType_t = cublasDiagType_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasDiagType_t(pub ::core::ffi::c_uint);
impl cublasSideMode_t {
    pub const CUBLAS_SIDE_LEFT: cublasSideMode_t = cublasSideMode_t(0);
}
impl cublasSideMode_t {
    pub const CUBLAS_SIDE_RIGHT: cublasSideMode_t = cublasSideMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasSideMode_t(pub ::core::ffi::c_uint);
impl cublasOperation_t {
    pub const CUBLAS_OP_N: cublasOperation_t = cublasOperation_t(0);
}
impl cublasOperation_t {
    pub const CUBLAS_OP_T: cublasOperation_t = cublasOperation_t(1);
}
impl cublasOperation_t {
    pub const CUBLAS_OP_C: cublasOperation_t = cublasOperation_t(2);
}
impl cublasOperation_t {
    pub const CUBLAS_OP_HERMITAN: cublasOperation_t = cublasOperation_t(2);
}
impl cublasOperation_t {
    pub const CUBLAS_OP_CONJG: cublasOperation_t = cublasOperation_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasOperation_t(pub ::core::ffi::c_uint);
impl cublasPointerMode_t {
    pub const CUBLAS_POINTER_MODE_HOST: cublasPointerMode_t = cublasPointerMode_t(0);
}
impl cublasPointerMode_t {
    pub const CUBLAS_POINTER_MODE_DEVICE: cublasPointerMode_t = cublasPointerMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasPointerMode_t(pub ::core::ffi::c_uint);
impl cublasAtomicsMode_t {
    pub const CUBLAS_ATOMICS_NOT_ALLOWED: cublasAtomicsMode_t = cublasAtomicsMode_t(0);
}
impl cublasAtomicsMode_t {
    pub const CUBLAS_ATOMICS_ALLOWED: cublasAtomicsMode_t = cublasAtomicsMode_t(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasAtomicsMode_t(pub ::core::ffi::c_uint);
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_DFALT: cublasGemmAlgo_t = cublasGemmAlgo_t(-1);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_DEFAULT: cublasGemmAlgo_t = cublasGemmAlgo_t(-1);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO0: cublasGemmAlgo_t = cublasGemmAlgo_t(0);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO1: cublasGemmAlgo_t = cublasGemmAlgo_t(1);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO2: cublasGemmAlgo_t = cublasGemmAlgo_t(2);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO3: cublasGemmAlgo_t = cublasGemmAlgo_t(3);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO4: cublasGemmAlgo_t = cublasGemmAlgo_t(4);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO5: cublasGemmAlgo_t = cublasGemmAlgo_t(5);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO6: cublasGemmAlgo_t = cublasGemmAlgo_t(6);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO7: cublasGemmAlgo_t = cublasGemmAlgo_t(7);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO8: cublasGemmAlgo_t = cublasGemmAlgo_t(8);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO9: cublasGemmAlgo_t = cublasGemmAlgo_t(9);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO10: cublasGemmAlgo_t = cublasGemmAlgo_t(10);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO11: cublasGemmAlgo_t = cublasGemmAlgo_t(11);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO12: cublasGemmAlgo_t = cublasGemmAlgo_t(12);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO13: cublasGemmAlgo_t = cublasGemmAlgo_t(13);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO14: cublasGemmAlgo_t = cublasGemmAlgo_t(14);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO15: cublasGemmAlgo_t = cublasGemmAlgo_t(15);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO16: cublasGemmAlgo_t = cublasGemmAlgo_t(16);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO17: cublasGemmAlgo_t = cublasGemmAlgo_t(17);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO18: cublasGemmAlgo_t = cublasGemmAlgo_t(18);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO19: cublasGemmAlgo_t = cublasGemmAlgo_t(19);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO20: cublasGemmAlgo_t = cublasGemmAlgo_t(20);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO21: cublasGemmAlgo_t = cublasGemmAlgo_t(21);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO22: cublasGemmAlgo_t = cublasGemmAlgo_t(22);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO23: cublasGemmAlgo_t = cublasGemmAlgo_t(23);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_DEFAULT_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(99);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_DFALT_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(99);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO0_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(100);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO1_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(101);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO2_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(102);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO3_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(103);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO4_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(104);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO5_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(105);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO6_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(106);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO7_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(107);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO8_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(108);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO9_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(109);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO10_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(110);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO11_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(111);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO12_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(112);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO13_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(113);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO14_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(114);
}
impl cublasGemmAlgo_t {
    pub const CUBLAS_GEMM_ALGO15_TENSOR_OP: cublasGemmAlgo_t = cublasGemmAlgo_t(115);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasGemmAlgo_t(pub ::core::ffi::c_int);
impl cublasMath_t {
    pub const CUBLAS_DEFAULT_MATH: cublasMath_t = cublasMath_t(0);
}
impl cublasMath_t {
    pub const CUBLAS_TENSOR_OP_MATH: cublasMath_t = cublasMath_t(1);
}
impl cublasMath_t {
    pub const CUBLAS_PEDANTIC_MATH: cublasMath_t = cublasMath_t(2);
}
impl cublasMath_t {
    pub const CUBLAS_TF32_TENSOR_OP_MATH: cublasMath_t = cublasMath_t(3);
}
impl cublasMath_t {
    pub const CUBLAS_MATH_DISALLOW_REDUCED_PRECISION_REDUCTION: cublasMath_t = cublasMath_t(
        16,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasMath_t(pub ::core::ffi::c_uint);
pub use super::cuda::cudaDataType as cublasDataType_t;
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_16F: cublasComputeType_t = cublasComputeType_t(64);
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_16F_PEDANTIC: cublasComputeType_t = cublasComputeType_t(65);
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_32F: cublasComputeType_t = cublasComputeType_t(68);
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_32F_PEDANTIC: cublasComputeType_t = cublasComputeType_t(69);
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_32F_FAST_16F: cublasComputeType_t = cublasComputeType_t(74);
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_32F_FAST_16BF: cublasComputeType_t = cublasComputeType_t(
        75,
    );
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_32F_FAST_TF32: cublasComputeType_t = cublasComputeType_t(
        77,
    );
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_64F: cublasComputeType_t = cublasComputeType_t(70);
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_64F_PEDANTIC: cublasComputeType_t = cublasComputeType_t(71);
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_32I: cublasComputeType_t = cublasComputeType_t(72);
}
impl cublasComputeType_t {
    pub const CUBLAS_COMPUTE_32I_PEDANTIC: cublasComputeType_t = cublasComputeType_t(73);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasComputeType_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cublasContext {
    _unused: [u8; 0],
}
pub type cublasHandle_t = *mut cublasContext;
pub type cublasLogCallback = ::core::option::Option<
    unsafe extern "C" fn(msg: *const ::core::ffi::c_char),
>;
impl cublasError_t {
    pub const NOT_INITIALIZED: cublasError_t = cublasError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const ALLOC_FAILED: cublasError_t = cublasError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const INVALID_VALUE: cublasError_t = cublasError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const ARCH_MISMATCH: cublasError_t = cublasError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const MAPPING_ERROR: cublasError_t = cublasError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(11)
    });
    pub const EXECUTION_FAILED: cublasError_t = cublasError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(13)
    });
    pub const INTERNAL_ERROR: cublasError_t = cublasError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(14)
    });
    pub const NOT_SUPPORTED: cublasError_t = cublasError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(15)
    });
    pub const LICENSE_ERROR: cublasError_t = cublasError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(16)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct cublasError_t(pub ::core::num::NonZeroU32);
pub trait cublasStatus_tConsts {
    const SUCCESS: cublasStatus_t = cublasStatus_t::Ok(());
    const ERROR_NOT_INITIALIZED: cublasStatus_t = cublasStatus_t::Err(
        cublasError_t::NOT_INITIALIZED,
    );
    const ERROR_ALLOC_FAILED: cublasStatus_t = cublasStatus_t::Err(
        cublasError_t::ALLOC_FAILED,
    );
    const ERROR_INVALID_VALUE: cublasStatus_t = cublasStatus_t::Err(
        cublasError_t::INVALID_VALUE,
    );
    const ERROR_ARCH_MISMATCH: cublasStatus_t = cublasStatus_t::Err(
        cublasError_t::ARCH_MISMATCH,
    );
    const ERROR_MAPPING_ERROR: cublasStatus_t = cublasStatus_t::Err(
        cublasError_t::MAPPING_ERROR,
    );
    const ERROR_EXECUTION_FAILED: cublasStatus_t = cublasStatus_t::Err(
        cublasError_t::EXECUTION_FAILED,
    );
    const ERROR_INTERNAL_ERROR: cublasStatus_t = cublasStatus_t::Err(
        cublasError_t::INTERNAL_ERROR,
    );
    const ERROR_NOT_SUPPORTED: cublasStatus_t = cublasStatus_t::Err(
        cublasError_t::NOT_SUPPORTED,
    );
    const ERROR_LICENSE_ERROR: cublasStatus_t = cublasStatus_t::Err(
        cublasError_t::LICENSE_ERROR,
    );
}
impl cublasStatus_tConsts for cublasStatus_t {}
#[must_use]
pub type cublasStatus_t = ::core::result::Result<(), cublasError_t>;
const _: fn() = || {
    let _ = std::mem::transmute::<cublasStatus_t, u32>;
};
