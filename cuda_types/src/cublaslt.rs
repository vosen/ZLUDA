// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_FMA: u32 = 1;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_HMMA: u32 = 2;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_IMMA: u32 = 4;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_DMMA: u32 = 8;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_TENSOR_OP_MASK: u32 = 254;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_OP_TYPE_MASK: u32 = 255;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_16F: u32 = 256;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_32F: u32 = 512;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_64F: u32 = 1024;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_32I: u32 = 2048;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_ACCUMULATOR_TYPE_MASK: u32 = 65280;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_16F: u32 = 65536;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_16BF: u32 = 131072;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_TF32: u32 = 262144;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_32F: u32 = 524288;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_64F: u32 = 1048576;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_8I: u32 = 2097152;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_8F_E4M3: u32 = 4194304;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_INPUT_8F_E5M2: u32 = 8388608;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_OP_INPUT_TYPE_MASK: u32 = 16711680;
pub const CUBLASLT_NUMERICAL_IMPL_FLAGS_GAUSSIAN: u64 = 4294967296;
impl cublasStatus_t {
    pub const CUBLAS_STATUS_SUCCESS: cublasStatus_t = cublasStatus_t(0);
}
impl cublasStatus_t {
    pub const CUBLAS_STATUS_NOT_INITIALIZED: cublasStatus_t = cublasStatus_t(1);
}
impl cublasStatus_t {
    pub const CUBLAS_STATUS_ALLOC_FAILED: cublasStatus_t = cublasStatus_t(3);
}
impl cublasStatus_t {
    pub const CUBLAS_STATUS_INVALID_VALUE: cublasStatus_t = cublasStatus_t(7);
}
impl cublasStatus_t {
    pub const CUBLAS_STATUS_ARCH_MISMATCH: cublasStatus_t = cublasStatus_t(8);
}
impl cublasStatus_t {
    pub const CUBLAS_STATUS_MAPPING_ERROR: cublasStatus_t = cublasStatus_t(11);
}
impl cublasStatus_t {
    pub const CUBLAS_STATUS_EXECUTION_FAILED: cublasStatus_t = cublasStatus_t(13);
}
impl cublasStatus_t {
    pub const CUBLAS_STATUS_INTERNAL_ERROR: cublasStatus_t = cublasStatus_t(14);
}
impl cublasStatus_t {
    pub const CUBLAS_STATUS_NOT_SUPPORTED: cublasStatus_t = cublasStatus_t(15);
}
impl cublasStatus_t {
    pub const CUBLAS_STATUS_LICENSE_ERROR: cublasStatus_t = cublasStatus_t(16);
}
#[repr(transparent)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasStatus_t(pub ::core::ffi::c_uint);
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cublasLtContext {
    _unused: [u8; 0],
}
/// Opaque structure holding CUBLASLT context
pub type cublasLtHandle_t = *mut cublasLtContext;
/// Semi-opaque descriptor for matrix memory layout
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatrixLayoutOpaque_t {
    pub data: [u64; 8usize],
}
/// Opaque descriptor for matrix memory layout
pub type cublasLtMatrixLayout_t = *mut cublasLtMatrixLayoutOpaque_t;
/** Semi-opaque algorithm descriptor (to avoid complicated alloc/free schemes)

 This structure can be trivially serialized and later restored for use with the same version of cuBLAS library to save
 on selecting the right configuration again.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulAlgo_t {
    pub data: [u64; 8usize],
}
/// Semi-opaque descriptor for cublasLtMatmul() operation details
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulDescOpaque_t {
    pub data: [u64; 32usize],
}
/// Opaque descriptor for cublasLtMatmul() operation details
pub type cublasLtMatmulDesc_t = *mut cublasLtMatmulDescOpaque_t;
/// Semi-opaque descriptor for cublasLtMatrixTransform() operation details
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatrixTransformDescOpaque_t {
    pub data: [u64; 8usize],
}
/// Opaque descriptor for cublasLtMatrixTransform() operation details
pub type cublasLtMatrixTransformDesc_t = *mut cublasLtMatrixTransformDescOpaque_t;
/// Semi-opaque descriptor for cublasLtMatmulPreference() operation details
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulPreferenceOpaque_t {
    pub data: [u64; 8usize],
}
/// Opaque descriptor for cublasLtMatmulAlgoGetHeuristic() configuration
pub type cublasLtMatmulPreference_t = *mut cublasLtMatmulPreferenceOpaque_t;
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_UNDEFINED: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        0,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(1);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(2);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(3);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(4);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(5);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(6);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(7);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(8);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(9);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(10);
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        11,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        12,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        13,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        14,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        15,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        16,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        17,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        18,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        19,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        20,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        21,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        22,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        23,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        24,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        25,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        26,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_96x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        27,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_96x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        28,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x160: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        29,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_160x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        30,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        31,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        32,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        33,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        34,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        35,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_END: cublasLtMatmulTile_t = cublasLtMatmulTile_t(36);
}
#[repr(transparent)]
/** Tile size (in C/D matrix Rows x Cols)

 General order of tile IDs is sorted by size first and by first dimension second.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulTile_t(pub ::core::ffi::c_uint);
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_UNDEFINED: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        0,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_16x1: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        1,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_16x2: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        2,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_16x3: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        3,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_16x4: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        4,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_16x5: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        5,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_16x6: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        6,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_32x1: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        7,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_32x2: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        8,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_32x3: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        9,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_32x4: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        10,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_32x5: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        11,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_32x6: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        12,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_64x1: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        13,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_64x2: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        14,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_64x3: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        15,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_64x4: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        16,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_64x5: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        17,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_64x6: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        18,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_128x1: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        19,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_128x2: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        20,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_128x3: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        21,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_128x4: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        22,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_128x5: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        23,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_128x6: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        24,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_32x10: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        25,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_8x4: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        26,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_16x10: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        27,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_8x5: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        28,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_8x3: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        31,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_8xAUTO: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        32,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_16xAUTO: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        33,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_32xAUTO: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        34,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_64xAUTO: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        35,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_128xAUTO: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        36,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_END: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        37,
    );
}
#[repr(transparent)]
/** Size and number of stages in which elements are read into shared memory

 General order of stages IDs is sorted by stage size first and by number of stages second.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulStages_t(pub ::core::ffi::c_uint);
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_AUTO: cublasLtClusterShape_t = cublasLtClusterShape_t(
        0,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        2,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_2x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        3,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_4x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        4,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x2x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        5,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_2x2x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        6,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_4x2x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        7,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x4x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        8,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_2x4x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        9,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_4x4x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        10,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_8x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        11,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x8x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        12,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_8x2x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        13,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_2x8x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        14,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_16x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        15,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x16x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        16,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_3x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        17,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_5x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        18,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_6x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        19,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_7x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        20,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_9x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        21,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_10x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        22,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_11x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        23,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_12x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        24,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_13x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        25,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_14x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        26,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_15x1x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        27,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_3x2x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        28,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_5x2x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        29,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_6x2x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        30,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_7x2x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        31,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x3x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        32,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_2x3x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        33,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_3x3x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        34,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_4x3x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        35,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_5x3x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        36,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_3x4x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        37,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x5x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        38,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_2x5x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        39,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_3x5x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        40,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x6x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        41,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_2x6x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        42,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x7x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        43,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_2x7x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        44,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x9x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        45,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x10x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        46,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x11x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        47,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x12x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        48,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x13x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        49,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x14x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        50,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_1x15x1: cublasLtClusterShape_t = cublasLtClusterShape_t(
        51,
    );
}
impl cublasLtClusterShape_t {
    /// Let library pick cluster shape automatically
    pub const CUBLASLT_CLUSTER_SHAPE_END: cublasLtClusterShape_t = cublasLtClusterShape_t(
        52,
    );
}
#[repr(transparent)]
/** Thread Block Cluster size

 Typically dimensioned similar to cublasLtMatmulTile_t, with the third coordinate unused at this time.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtClusterShape_t(pub ::core::ffi::c_uint);
impl cublasLtMatmulInnerShape_t {
    pub const CUBLASLT_MATMUL_INNER_SHAPE_UNDEFINED: cublasLtMatmulInnerShape_t = cublasLtMatmulInnerShape_t(
        0,
    );
}
impl cublasLtMatmulInnerShape_t {
    pub const CUBLASLT_MATMUL_INNER_SHAPE_MMA884: cublasLtMatmulInnerShape_t = cublasLtMatmulInnerShape_t(
        1,
    );
}
impl cublasLtMatmulInnerShape_t {
    pub const CUBLASLT_MATMUL_INNER_SHAPE_MMA1684: cublasLtMatmulInnerShape_t = cublasLtMatmulInnerShape_t(
        2,
    );
}
impl cublasLtMatmulInnerShape_t {
    pub const CUBLASLT_MATMUL_INNER_SHAPE_MMA1688: cublasLtMatmulInnerShape_t = cublasLtMatmulInnerShape_t(
        3,
    );
}
impl cublasLtMatmulInnerShape_t {
    pub const CUBLASLT_MATMUL_INNER_SHAPE_MMA16816: cublasLtMatmulInnerShape_t = cublasLtMatmulInnerShape_t(
        4,
    );
}
impl cublasLtMatmulInnerShape_t {
    pub const CUBLASLT_MATMUL_INNER_SHAPE_END: cublasLtMatmulInnerShape_t = cublasLtMatmulInnerShape_t(
        5,
    );
}
#[repr(transparent)]
/** Inner size of the kernel

 Represents various aspects of internal kernel design, that don't impact CUDA grid size but may have other more subtle
 effects.
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulInnerShape_t(pub ::core::ffi::c_uint);
impl cublasLtPointerMode_t {
    /// matches CUBLAS_POINTER_MODE_HOST, pointer targets a single value host memory
    pub const CUBLASLT_POINTER_MODE_HOST: cublasLtPointerMode_t = cublasLtPointerMode_t(
        0,
    );
}
impl cublasLtPointerMode_t {
    /// matches CUBLAS_POINTER_MODE_DEVICE, pointer targets a single value device memory
    pub const CUBLASLT_POINTER_MODE_DEVICE: cublasLtPointerMode_t = cublasLtPointerMode_t(
        1,
    );
}
impl cublasLtPointerMode_t {
    /// pointer targets an array in device memory
    pub const CUBLASLT_POINTER_MODE_DEVICE_VECTOR: cublasLtPointerMode_t = cublasLtPointerMode_t(
        2,
    );
}
impl cublasLtPointerMode_t {
    /** alpha pointer targets an array in device memory, beta is zero. Note:
CUBLASLT_MATMUL_DESC_ALPHA_VECTOR_BATCH_STRIDE is not supported, must be 0.*/
    pub const CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_ZERO: cublasLtPointerMode_t = cublasLtPointerMode_t(
        3,
    );
}
impl cublasLtPointerMode_t {
    /// alpha pointer targets an array in device memory, beta is a single value in host memory.
    pub const CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_HOST: cublasLtPointerMode_t = cublasLtPointerMode_t(
        4,
    );
}
#[repr(transparent)]
/// Pointer mode to use for alpha/beta
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtPointerMode_t(pub ::core::ffi::c_uint);
impl cublasLtPointerModeMask_t {
    /// see CUBLASLT_POINTER_MODE_HOST
    pub const CUBLASLT_POINTER_MODE_MASK_HOST: cublasLtPointerModeMask_t = cublasLtPointerModeMask_t(
        1,
    );
}
impl cublasLtPointerModeMask_t {
    /// see CUBLASLT_POINTER_MODE_DEVICE
    pub const CUBLASLT_POINTER_MODE_MASK_DEVICE: cublasLtPointerModeMask_t = cublasLtPointerModeMask_t(
        2,
    );
}
impl cublasLtPointerModeMask_t {
    /// see CUBLASLT_POINTER_MODE_DEVICE_VECTOR
    pub const CUBLASLT_POINTER_MODE_MASK_DEVICE_VECTOR: cublasLtPointerModeMask_t = cublasLtPointerModeMask_t(
        4,
    );
}
impl cublasLtPointerModeMask_t {
    /// see CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_ZERO
    pub const CUBLASLT_POINTER_MODE_MASK_ALPHA_DEVICE_VECTOR_BETA_ZERO: cublasLtPointerModeMask_t = cublasLtPointerModeMask_t(
        8,
    );
}
impl cublasLtPointerModeMask_t {
    /// see CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_HOST
    pub const CUBLASLT_POINTER_MODE_MASK_ALPHA_DEVICE_VECTOR_BETA_HOST: cublasLtPointerModeMask_t = cublasLtPointerModeMask_t(
        16,
    );
}
#[repr(transparent)]
/// Mask to define pointer mode capability
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtPointerModeMask_t(pub ::core::ffi::c_uint);
pub type cublasLtNumericalImplFlags_t = u64;
impl cublasLtOrder_t {
    /** Column-major

 Leading dimension is the stride (in elements) to the beginning of next column in memory.*/
    pub const CUBLASLT_ORDER_COL: cublasLtOrder_t = cublasLtOrder_t(0);
}
impl cublasLtOrder_t {
    /** Row major

 Leading dimension is the stride (in elements) to the beginning of next row in memory.*/
    pub const CUBLASLT_ORDER_ROW: cublasLtOrder_t = cublasLtOrder_t(1);
}
impl cublasLtOrder_t {
    /** Column-major ordered tiles of 32 columns.

 Leading dimension is the stride (in elements) to the beginning of next group of 32-columns. E.g. if matrix has 33
 columns and 2 rows, ld must be at least (32) * 2 = 64.*/
    pub const CUBLASLT_ORDER_COL32: cublasLtOrder_t = cublasLtOrder_t(2);
}
impl cublasLtOrder_t {
    /** Column-major ordered tiles of composite tiles with total 32 columns and 8 rows, tile composed of interleaved
 inner tiles of 4 columns within 4 even or odd rows in an alternating pattern.

 Leading dimension is the stride (in elements) to the beginning of the first 32 column x 8 row tile for the next
 32-wide group of columns. E.g. if matrix has 33 columns and 1 row, ld must be at least (32 * 8) * 1 = 256.*/
    pub const CUBLASLT_ORDER_COL4_4R2_8C: cublasLtOrder_t = cublasLtOrder_t(3);
}
impl cublasLtOrder_t {
    /** Column-major ordered tiles of composite tiles with total 32 columns ands 32 rows.
 Element offset within the tile is calculated as (((row%8)/2*4+row/8)*2+row%2)*32+col.

 Leading dimension is the stride (in elements) to the beginning of the first 32 column x 32 row tile for the next
 32-wide group of columns. E.g. if matrix has 33 columns and 1 row, ld must be at least (32*32)*1 = 1024.*/
    pub const CUBLASLT_ORDER_COL32_2R_4R4: cublasLtOrder_t = cublasLtOrder_t(4);
}
#[repr(transparent)]
/// Enum for data ordering
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtOrder_t(pub ::core::ffi::c_uint);
impl cublasLtMatrixLayoutAttribute_t {
    /** Data type, see cudaDataType.

 uint32_t*/
    pub const CUBLASLT_MATRIX_LAYOUT_TYPE: cublasLtMatrixLayoutAttribute_t = cublasLtMatrixLayoutAttribute_t(
        0,
    );
}
impl cublasLtMatrixLayoutAttribute_t {
    /** Memory order of the data, see cublasLtOrder_t.

 int32_t, default: CUBLASLT_ORDER_COL*/
    pub const CUBLASLT_MATRIX_LAYOUT_ORDER: cublasLtMatrixLayoutAttribute_t = cublasLtMatrixLayoutAttribute_t(
        1,
    );
}
impl cublasLtMatrixLayoutAttribute_t {
    /** Number of rows.

 Usually only values that can be expressed as int32_t are supported.

 uint64_t*/
    pub const CUBLASLT_MATRIX_LAYOUT_ROWS: cublasLtMatrixLayoutAttribute_t = cublasLtMatrixLayoutAttribute_t(
        2,
    );
}
impl cublasLtMatrixLayoutAttribute_t {
    /** Number of columns.

 Usually only values that can be expressed as int32_t are supported.

 uint64_t*/
    pub const CUBLASLT_MATRIX_LAYOUT_COLS: cublasLtMatrixLayoutAttribute_t = cublasLtMatrixLayoutAttribute_t(
        3,
    );
}
impl cublasLtMatrixLayoutAttribute_t {
    /** Matrix leading dimension.

 For CUBLASLT_ORDER_COL this is stride (in elements) of matrix column, for more details and documentation for
 other memory orders see documentation for cublasLtOrder_t values.

 Currently only non-negative values are supported, must be large enough so that matrix memory locations are not
 overlapping (e.g. greater or equal to CUBLASLT_MATRIX_LAYOUT_ROWS in case of CUBLASLT_ORDER_COL).

 int64_t;*/
    pub const CUBLASLT_MATRIX_LAYOUT_LD: cublasLtMatrixLayoutAttribute_t = cublasLtMatrixLayoutAttribute_t(
        4,
    );
}
impl cublasLtMatrixLayoutAttribute_t {
    /** Number of matmul operations to perform in the batch.

 See also CUBLASLT_ALGO_CAP_STRIDED_BATCH_SUPPORT

 int32_t, default: 1*/
    pub const CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT: cublasLtMatrixLayoutAttribute_t = cublasLtMatrixLayoutAttribute_t(
        5,
    );
}
impl cublasLtMatrixLayoutAttribute_t {
    /** Stride (in elements) to the next matrix for strided batch operation.

 When matrix type is planar-complex (CUBLASLT_MATRIX_LAYOUT_PLANE_OFFSET != 0), batch stride
 is interpreted by cublasLtMatmul() in number of real valued sub-elements. E.g. for data of type CUDA_C_16F,
 offset of 1024B is encoded as a stride of value 512 (since each element of the real and imaginary matrices
 is a 2B (16bit) floating point type).

 NOTE: A bug in cublasLtMatrixTransform() causes it to interpret the batch stride for a planar-complex matrix
 as if it was specified in number of complex elements. Therefore an offset of 1024B must be encoded as stride
 value 256 when calling cublasLtMatrixTransform() (each complex element is 4B with real and imaginary values 2B
 each). This behavior is expected to be corrected in the next major cuBLAS version.

 int64_t, default: 0*/
    pub const CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET: cublasLtMatrixLayoutAttribute_t = cublasLtMatrixLayoutAttribute_t(
        6,
    );
}
impl cublasLtMatrixLayoutAttribute_t {
    /** Stride (in bytes) to the imaginary plane for planar complex layout.

 int64_t, default: 0 - 0 means that layout is regular (real and imaginary parts of complex numbers are interleaved
 in memory in each element)*/
    pub const CUBLASLT_MATRIX_LAYOUT_PLANE_OFFSET: cublasLtMatrixLayoutAttribute_t = cublasLtMatrixLayoutAttribute_t(
        7,
    );
}
#[repr(transparent)]
/// Attributes of memory layout
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatrixLayoutAttribute_t(pub ::core::ffi::c_uint);
impl cublasLtMatmulDescAttributes_t {
    /** Compute type, see cudaDataType. Defines data type used for multiply and accumulate operations and the
 accumulator during matrix multiplication.

 int32_t*/
    pub const CUBLASLT_MATMUL_DESC_COMPUTE_TYPE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        0,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Scale type, see cudaDataType. Defines data type of alpha and beta. Accumulator and value from matrix C are
 typically converted to scale type before final scaling. Value is then converted from scale type to type of matrix
 D before being stored in memory.

 int32_t, default: same as CUBLASLT_MATMUL_DESC_COMPUTE_TYPE*/
    pub const CUBLASLT_MATMUL_DESC_SCALE_TYPE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        1,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Pointer mode of alpha and beta, see cublasLtPointerMode_t. When CUBLASLT_POINTER_MODE_DEVICE_VECTOR is in use,
 alpha/beta vector lenghts must match number of output matrix rows.

 int32_t, default: CUBLASLT_POINTER_MODE_HOST*/
    pub const CUBLASLT_MATMUL_DESC_POINTER_MODE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        2,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Transform of matrix A, see cublasOperation_t.

 int32_t, default: CUBLAS_OP_N*/
    pub const CUBLASLT_MATMUL_DESC_TRANSA: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        3,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Transform of matrix B, see cublasOperation_t.

 int32_t, default: CUBLAS_OP_N*/
    pub const CUBLASLT_MATMUL_DESC_TRANSB: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        4,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Transform of matrix C, see cublasOperation_t.

 Currently only CUBLAS_OP_N is supported.

 int32_t, default: CUBLAS_OP_N*/
    pub const CUBLASLT_MATMUL_DESC_TRANSC: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        5,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Matrix fill mode, see cublasFillMode_t.

 int32_t, default: CUBLAS_FILL_MODE_FULL*/
    pub const CUBLASLT_MATMUL_DESC_FILL_MODE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        6,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Epilogue function, see cublasLtEpilogue_t.

 uint32_t, default: CUBLASLT_EPILOGUE_DEFAULT*/
    pub const CUBLASLT_MATMUL_DESC_EPILOGUE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        7,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Bias or bias gradient vector pointer in the device memory.

 Bias case. See CUBLASLT_EPILOGUE_BIAS.
 For bias data type see CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE.

 Bias vector length must match matrix D rows count.

 Bias gradient case. See CUBLASLT_EPILOGUE_DRELU_BGRAD and CUBLASLT_EPILOGUE_DGELU_BGRAD.
 Bias gradient vector elements are the same type as the output elements
 (Ctype) with the exception of IMMA kernels (see above).

 Routines that don't dereference this pointer, like cublasLtMatmulAlgoGetHeuristic()
 depend on its value to determine expected pointer alignment.

 Bias case: const void *, default: NULL
 Bias gradient case: void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_BIAS_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        8,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Batch stride for bias or bias gradient vector.

 Used together with CUBLASLT_MATMUL_DESC_BIAS_POINTER when matrix D's CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT > 1.

 int64_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_BIAS_BATCH_STRIDE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        10,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Pointer for epilogue auxiliary buffer.

 - Output vector for ReLu bit-mask in forward pass when CUBLASLT_EPILOGUE_RELU_AUX
   or CUBLASLT_EPILOGUE_RELU_AUX_BIAS epilogue is used.
 - Input vector for ReLu bit-mask in backward pass when
   CUBLASLT_EPILOGUE_DRELU_BGRAD epilogue is used.

 - Output of GELU input matrix in forward pass when
   CUBLASLT_EPILOGUE_GELU_AUX_BIAS epilogue is used.
 - Input of GELU input matrix for backward pass when
   CUBLASLT_EPILOGUE_DGELU_BGRAD epilogue is used.

 For aux data type see CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_DATA_TYPE.

 Routines that don't dereference this pointer, like cublasLtMatmulAlgoGetHeuristic()
 depend on its value to determine expected pointer alignment.

 Requires setting CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD attribute.

 Forward pass: void *, default: NULL
 Backward pass: const void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        11,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Leading dimension for epilogue auxiliary buffer.

 - ReLu bit-mask matrix leading dimension in elements (i.e. bits)
   when CUBLASLT_EPILOGUE_RELU_AUX, CUBLASLT_EPILOGUE_RELU_AUX_BIAS or CUBLASLT_EPILOGUE_DRELU_BGRAD epilogue is
 used. Must be divisible by 128 and be no less than the number of rows in the output matrix.

 - GELU input matrix leading dimension in elements
   when CUBLASLT_EPILOGUE_GELU_AUX_BIAS or CUBLASLT_EPILOGUE_DGELU_BGRAD epilogue used.
   Must be divisible by 8 and be no less than the number of rows in the output matrix.

 int64_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        12,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Batch stride for epilogue auxiliary buffer.

 - ReLu bit-mask matrix batch stride in elements (i.e. bits)
   when CUBLASLT_EPILOGUE_RELU_AUX, CUBLASLT_EPILOGUE_RELU_AUX_BIAS or CUBLASLT_EPILOGUE_DRELU_BGRAD epilogue is
 used. Must be divisible by 128.

 - GELU input matrix batch stride in elements
   when CUBLASLT_EPILOGUE_GELU_AUX_BIAS or CUBLASLT_EPILOGUE_DGELU_BGRAD epilogue used.
   Must be divisible by 8.

 int64_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        13,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Batch stride for alpha vector.

 Used together with CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_HOST when matrix D's
 CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT > 1. If CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_ZERO is set then
 CUBLASLT_MATMUL_DESC_ALPHA_VECTOR_BATCH_STRIDE must be set to 0 as this mode doesnt supported batched alpha vector.

 int64_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_ALPHA_VECTOR_BATCH_STRIDE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        14,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Number of SMs to target for parallel execution. Optimizes heuristics for execution on a different number of SMs
  when user expects a concurrent stream to be using some of the device resources.

  int32_t, default: 0 - use the number reported by the device.*/
    pub const CUBLASLT_MATMUL_DESC_SM_COUNT_TARGET: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        15,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Device pointer to the scale factor value that converts data in matrix A to the compute data type range.

  The scaling factor value must have the same type as the compute type.

  If not specified, or set to NULL, the scaling factor is assumed to be 1.

  If set for an unsupported matrix data, scale, and compute type combination, calling cublasLtMatmul()
  will return CUBLAS_INVALID_VALUE.

  const void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_A_SCALE_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        17,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Device pointer to the scale factor value to convert data in matrix B to compute data type range.

  The scaling factor value must have the same type as the compute type.

  If not specified, or set to NULL, the scaling factor is assumed to be 1.

  If set for an unsupported matrix data, scale, and compute type combination, calling cublasLtMatmul()
  will return CUBLAS_INVALID_VALUE.

  const void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_B_SCALE_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        18,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Device pointer to the scale factor value to convert data in matrix C to compute data type range.

  The scaling factor value must have the same type as the compute type.

  If not specified, or set to NULL, the scaling factor is assumed to be 1.

  If set for an unsupported matrix data, scale, and compute type combination, calling cublasLtMatmul()
  will return CUBLAS_INVALID_VALUE.

  const void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_C_SCALE_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        19,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Device pointer to the scale factor value to convert data in matrix D to compute data type range.

  The scaling factor value must have the same type as the compute type.

  If not specified, or set to NULL, the scaling factor is assumed to be 1.

  If set for an unsupported matrix data, scale, and compute type combination, calling cublasLtMatmul()
  will return CUBLAS_INVALID_VALUE.

  const void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_D_SCALE_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        20,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Device pointer to the memory location that on completion will be set to the maximum of absolute values in the
  output matrix.

  The computed value has the same type as the compute type.

  If not specified or set to NULL, the maximum absolute value is not computed. If set for an unsupported matrix
  data, scale, and compute type combination, calling cublasLtMatmul() will return CUBLAS_INVALID_VALUE.

  void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_AMAX_D_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        21,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Type of the data to be stored to the memory pointed to by CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.

  If unset, the data type defaults to the type of elements of the output matrix with some exceptions, see details
 below.

  ReLu uses a bit-mask.

  GELU input matrix elements type is the same as the type of elements of
  the output matrix with some exceptions, see details below.

  For fp8 kernels with output type CUDA_R_8F_E4M3 the aux data type can be CUDA_R_8F_E4M3 or CUDA_R_16F with some
  restrictions.  See https://docs.nvidia.com/cuda/cublas/index.html#cublasLtMatmulDescAttributes_t for more details.

  If set for an unsupported matrix data, scale, and compute type combination, calling cublasLtMatmul()
  will return CUBLAS_INVALID_VALUE.

  int32_t based on cudaDataType, default: -1*/
    pub const CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_DATA_TYPE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        22,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Device pointer to the scaling factor value to convert results from compute type data range to storage
  data range in the auxiliary matrix that is set via CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.

  The scaling factor value must have the same type as the compute type.

  If not specified, or set to NULL, the scaling factor is assumed to be 1. If set for an unsupported matrix data,
  scale, and compute type combination, calling cublasLtMatmul() will return CUBLAS_INVALID_VALUE.

  void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        23,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Device pointer to the memory location that on completion will be set to the maximum of absolute values in the
  buffer that is set via CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.

  The computed value has the same type as the compute type.

  If not specified or set to NULL, the maximum absolute value is not computed. If set for an unsupported matrix
  data, scale, and compute type combination, calling cublasLtMatmul() will return CUBLAS_INVALID_VALUE.

  void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_AMAX_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        24,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Flag for managing fp8 fast accumulation mode.
  When enabled, problem execution might be faster but at the cost of lower accuracy because intermediate results
  will not periodically be promoted to a higher precision.

  int8_t, default: 0 - fast accumulation mode is disabled.*/
    pub const CUBLASLT_MATMUL_DESC_FAST_ACCUM: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        25,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Type of bias or bias gradient vector in the device memory.

 Bias case: see CUBLASLT_EPILOGUE_BIAS.

 Bias vector elements are the same type as the elements of output matrix (Dtype) with the following exceptions:
 - IMMA kernels with computeType=CUDA_R_32I and Ctype=CUDA_R_8I where the bias vector elements
   are the same type as alpha, beta (CUBLASLT_MATMUL_DESC_SCALE_TYPE=CUDA_R_32F)
 - fp8 kernels with an output type of CUDA_R_32F, CUDA_R_8F_E4M3 or CUDA_R_8F_E5M2, See
   https://docs.nvidia.com/cuda/cublas/index.html#cublasLtMatmul for details.

 int32_t based on cudaDataType, default: -1*/
    pub const CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        26,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** EXPERIMENTAL: Number of atomic synchronization chunks in the row dimension of the output matrix D.

 int32_t, default 0 (atomic synchronization disabled)*/
    pub const CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_NUM_CHUNKS_D_ROWS: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        27,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** EXPERIMENTAL: Number of atomic synchronization chunks in the column dimension of the output matrix D.

 int32_t, default 0 (atomic synchronization disabled)*/
    pub const CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_NUM_CHUNKS_D_COLS: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        28,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** EXPERIMENTAL: Pointer to a device array of input atomic counters consumed by a matmul.

 int32_t *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_IN_COUNTERS_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        29,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** EXPERIMENTAL: Pointer to a device array of output atomic counters produced by a matmul.

 int32_t *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_OUT_COUNTERS_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        30,
    );
}
#[repr(transparent)]
/// Matmul descriptor attributes to define details of the operation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulDescAttributes_t(pub ::core::ffi::c_uint);
impl cublasLtMatrixTransformDescAttributes_t {
    /** Scale type, see cudaDataType. Inputs are converted to scale type for scaling and summation and results are then
 converted to output type to store in memory.

 int32_t*/
    pub const CUBLASLT_MATRIX_TRANSFORM_DESC_SCALE_TYPE: cublasLtMatrixTransformDescAttributes_t = cublasLtMatrixTransformDescAttributes_t(
        0,
    );
}
impl cublasLtMatrixTransformDescAttributes_t {
    /** Pointer mode of alpha and beta, see cublasLtPointerMode_t.

 int32_t, default: CUBLASLT_POINTER_MODE_HOST*/
    pub const CUBLASLT_MATRIX_TRANSFORM_DESC_POINTER_MODE: cublasLtMatrixTransformDescAttributes_t = cublasLtMatrixTransformDescAttributes_t(
        1,
    );
}
impl cublasLtMatrixTransformDescAttributes_t {
    /** Transform of matrix A, see cublasOperation_t.

 int32_t, default: CUBLAS_OP_N*/
    pub const CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSA: cublasLtMatrixTransformDescAttributes_t = cublasLtMatrixTransformDescAttributes_t(
        2,
    );
}
impl cublasLtMatrixTransformDescAttributes_t {
    /** Transform of matrix B, see cublasOperation_t.

 int32_t, default: CUBLAS_OP_N*/
    pub const CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSB: cublasLtMatrixTransformDescAttributes_t = cublasLtMatrixTransformDescAttributes_t(
        3,
    );
}
#[repr(transparent)]
/// Matrix transform descriptor attributes to define details of the operation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatrixTransformDescAttributes_t(pub ::core::ffi::c_uint);
impl cublasLtReductionScheme_t {
    /// No reduction scheme, dot-product shall be performed in one sequence.
    pub const CUBLASLT_REDUCTION_SCHEME_NONE: cublasLtReductionScheme_t = cublasLtReductionScheme_t(
        0,
    );
}
impl cublasLtReductionScheme_t {
    /** Reduction is performed "in place" - using the output buffer (and output data type) and counters (in workspace) to
 guarantee the sequentiality.*/
    pub const CUBLASLT_REDUCTION_SCHEME_INPLACE: cublasLtReductionScheme_t = cublasLtReductionScheme_t(
        1,
    );
}
impl cublasLtReductionScheme_t {
    /// Intermediate results are stored in compute type in the workspace and reduced in a separate step.
    pub const CUBLASLT_REDUCTION_SCHEME_COMPUTE_TYPE: cublasLtReductionScheme_t = cublasLtReductionScheme_t(
        2,
    );
}
impl cublasLtReductionScheme_t {
    /// Intermediate results are stored in output type in the workspace and reduced in a separate step.
    pub const CUBLASLT_REDUCTION_SCHEME_OUTPUT_TYPE: cublasLtReductionScheme_t = cublasLtReductionScheme_t(
        4,
    );
}
impl cublasLtReductionScheme_t {
    /// Intermediate results are stored in output type in the workspace and reduced in a separate step.
    pub const CUBLASLT_REDUCTION_SCHEME_MASK: cublasLtReductionScheme_t = cublasLtReductionScheme_t(
        7,
    );
}
#[repr(transparent)]
/// Reduction scheme for portions of the dot-product calculated in parallel (a. k. a. "split - K").
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtReductionScheme_t(pub ::core::ffi::c_uint);
impl cublasLtEpilogue_t {
    /// No special postprocessing, just scale and quantize results if necessary.
    pub const CUBLASLT_EPILOGUE_DEFAULT: cublasLtEpilogue_t = cublasLtEpilogue_t(1);
}
impl cublasLtEpilogue_t {
    /// ReLu, apply ReLu point-wise transform to the results (x:=max(x, 0)).
    pub const CUBLASLT_EPILOGUE_RELU: cublasLtEpilogue_t = cublasLtEpilogue_t(2);
}
impl cublasLtEpilogue_t {
    /** ReLu, apply ReLu point-wise transform to the results (x:=max(x, 0)).

 This epilogue mode produces an extra output, a ReLu bit-mask matrix,
 see CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.*/
    pub const CUBLASLT_EPILOGUE_RELU_AUX: cublasLtEpilogue_t = cublasLtEpilogue_t(130);
}
impl cublasLtEpilogue_t {
    /** Bias, apply (broadcasted) Bias from bias vector. Bias vector length must match matrix D rows, it must be packed
 (stride between vector elements is 1). Bias vector is broadcasted to all columns and added before applying final
 postprocessing.*/
    pub const CUBLASLT_EPILOGUE_BIAS: cublasLtEpilogue_t = cublasLtEpilogue_t(4);
}
impl cublasLtEpilogue_t {
    /// ReLu and Bias, apply Bias and then ReLu transform
    pub const CUBLASLT_EPILOGUE_RELU_BIAS: cublasLtEpilogue_t = cublasLtEpilogue_t(6);
}
impl cublasLtEpilogue_t {
    /** ReLu and Bias, apply Bias and then ReLu transform

 This epilogue mode produces an extra output, a ReLu bit-mask matrix,
 see CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.*/
    pub const CUBLASLT_EPILOGUE_RELU_AUX_BIAS: cublasLtEpilogue_t = cublasLtEpilogue_t(
        134,
    );
}
impl cublasLtEpilogue_t {
    /** ReLu and Bias, apply Bias and then ReLu transform

 This epilogue mode produces an extra output, a ReLu bit-mask matrix,
 see CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.*/
    pub const CUBLASLT_EPILOGUE_DRELU: cublasLtEpilogue_t = cublasLtEpilogue_t(136);
}
impl cublasLtEpilogue_t {
    /** ReLu and Bias, apply Bias and then ReLu transform

 This epilogue mode produces an extra output, a ReLu bit-mask matrix,
 see CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.*/
    pub const CUBLASLT_EPILOGUE_DRELU_BGRAD: cublasLtEpilogue_t = cublasLtEpilogue_t(
        152,
    );
}
impl cublasLtEpilogue_t {
    /// GELU, apply GELU point-wise transform to the results (x:=GELU(x)).
    pub const CUBLASLT_EPILOGUE_GELU: cublasLtEpilogue_t = cublasLtEpilogue_t(32);
}
impl cublasLtEpilogue_t {
    /** GELU, apply GELU point-wise transform to the results (x:=GELU(x)).

 This epilogue mode outputs GELU input as a separate matrix (useful for training).
 See CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.*/
    pub const CUBLASLT_EPILOGUE_GELU_AUX: cublasLtEpilogue_t = cublasLtEpilogue_t(160);
}
impl cublasLtEpilogue_t {
    /// GELU and Bias, apply Bias and then GELU transform
    pub const CUBLASLT_EPILOGUE_GELU_BIAS: cublasLtEpilogue_t = cublasLtEpilogue_t(36);
}
impl cublasLtEpilogue_t {
    /** GELU and Bias, apply Bias and then GELU transform

 This epilogue mode outputs GELU input as a separate matrix (useful for training).
 See CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.*/
    pub const CUBLASLT_EPILOGUE_GELU_AUX_BIAS: cublasLtEpilogue_t = cublasLtEpilogue_t(
        164,
    );
}
impl cublasLtEpilogue_t {
    /** GELU and Bias, apply Bias and then GELU transform

 This epilogue mode outputs GELU input as a separate matrix (useful for training).
 See CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.*/
    pub const CUBLASLT_EPILOGUE_DGELU: cublasLtEpilogue_t = cublasLtEpilogue_t(192);
}
impl cublasLtEpilogue_t {
    /** GELU and Bias, apply Bias and then GELU transform

 This epilogue mode outputs GELU input as a separate matrix (useful for training).
 See CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER.*/
    pub const CUBLASLT_EPILOGUE_DGELU_BGRAD: cublasLtEpilogue_t = cublasLtEpilogue_t(
        208,
    );
}
impl cublasLtEpilogue_t {
    /** Bias gradient based on the input matrix A.

 The bias size corresponds to the number of rows of the matrix D.
 The reduction happens over the GEMM's "k" dimension.

 Stores Bias gradient in the auxiliary output
 (see CUBLASLT_MATMUL_DESC_BIAS_POINTER).*/
    pub const CUBLASLT_EPILOGUE_BGRADA: cublasLtEpilogue_t = cublasLtEpilogue_t(256);
}
impl cublasLtEpilogue_t {
    /** Bias gradient based on the input matrix B.

 The bias size corresponds to the number of columns of the matrix D.
 The reduction happens over the GEMM's "k" dimension.

 Stores Bias gradient in the auxiliary output
 (see CUBLASLT_MATMUL_DESC_BIAS_POINTER).*/
    pub const CUBLASLT_EPILOGUE_BGRADB: cublasLtEpilogue_t = cublasLtEpilogue_t(512);
}
#[repr(transparent)]
/// Postprocessing options for the epilogue
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtEpilogue_t(pub ::core::ffi::c_uint);
impl cublasLtMatmulSearch_t {
    /// ask heuristics for best algo for given usecase
    pub const CUBLASLT_SEARCH_BEST_FIT: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        0,
    );
}
impl cublasLtMatmulSearch_t {
    /// only try to find best config for preconfigured algo id
    pub const CUBLASLT_SEARCH_LIMITED_BY_ALGO_ID: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        1,
    );
}
impl cublasLtMatmulSearch_t {
    /// reserved for future use
    pub const CUBLASLT_SEARCH_RESERVED_02: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        2,
    );
}
impl cublasLtMatmulSearch_t {
    /// reserved for future use
    pub const CUBLASLT_SEARCH_RESERVED_03: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        3,
    );
}
impl cublasLtMatmulSearch_t {
    /// reserved for future use
    pub const CUBLASLT_SEARCH_RESERVED_04: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        4,
    );
}
impl cublasLtMatmulSearch_t {
    /// reserved for future use
    pub const CUBLASLT_SEARCH_RESERVED_05: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        5,
    );
}
#[repr(transparent)]
/// Matmul heuristic search mode
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulSearch_t(pub ::core::ffi::c_uint);
impl cublasLtMatmulPreferenceAttributes_t {
    /** Search mode, see cublasLtMatmulSearch_t.

 uint32_t, default: CUBLASLT_SEARCH_BEST_FIT*/
    pub const CUBLASLT_MATMUL_PREF_SEARCH_MODE: cublasLtMatmulPreferenceAttributes_t = cublasLtMatmulPreferenceAttributes_t(
        0,
    );
}
impl cublasLtMatmulPreferenceAttributes_t {
    /** Maximum allowed workspace size in bytes.

 uint64_t, default: 0 - no workspace allowed*/
    pub const CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES: cublasLtMatmulPreferenceAttributes_t = cublasLtMatmulPreferenceAttributes_t(
        1,
    );
}
impl cublasLtMatmulPreferenceAttributes_t {
    /** Reduction scheme mask, see cublasLtReductionScheme_t. Filters heuristic result to only include algo configs that
 use one of the required modes.

 E.g. mask value of 0x03 will allow only INPLACE and COMPUTE_TYPE reduction schemes.

 uint32_t, default: CUBLASLT_REDUCTION_SCHEME_MASK (allows all reduction schemes)*/
    pub const CUBLASLT_MATMUL_PREF_REDUCTION_SCHEME_MASK: cublasLtMatmulPreferenceAttributes_t = cublasLtMatmulPreferenceAttributes_t(
        3,
    );
}
impl cublasLtMatmulPreferenceAttributes_t {
    /** Minimum buffer alignment for matrix A (in bytes).

 Selecting a smaller value will exclude algorithms that can not work with matrix A that is not as strictly aligned
 as they need.

 uint32_t, default: 256*/
    pub const CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_A_BYTES: cublasLtMatmulPreferenceAttributes_t = cublasLtMatmulPreferenceAttributes_t(
        5,
    );
}
impl cublasLtMatmulPreferenceAttributes_t {
    /** Minimum buffer alignment for matrix B (in bytes).

 Selecting a smaller value will exclude algorithms that can not work with matrix B that is not as strictly aligned
 as they need.

 uint32_t, default: 256*/
    pub const CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_B_BYTES: cublasLtMatmulPreferenceAttributes_t = cublasLtMatmulPreferenceAttributes_t(
        6,
    );
}
impl cublasLtMatmulPreferenceAttributes_t {
    /** Minimum buffer alignment for matrix C (in bytes).

 Selecting a smaller value will exclude algorithms that can not work with matrix C that is not as strictly aligned
 as they need.

 uint32_t, default: 256*/
    pub const CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_C_BYTES: cublasLtMatmulPreferenceAttributes_t = cublasLtMatmulPreferenceAttributes_t(
        7,
    );
}
impl cublasLtMatmulPreferenceAttributes_t {
    /** Minimum buffer alignment for matrix D (in bytes).

 Selecting a smaller value will exclude algorithms that can not work with matrix D that is not as strictly aligned
 as they need.

 uint32_t, default: 256*/
    pub const CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_D_BYTES: cublasLtMatmulPreferenceAttributes_t = cublasLtMatmulPreferenceAttributes_t(
        8,
    );
}
impl cublasLtMatmulPreferenceAttributes_t {
    /** Maximum wave count.

 See cublasLtMatmulHeuristicResult_t::wavesCount.

 Selecting a non-zero value will exclude algorithms that report device utilization higher than specified.

 float, default: 0.0f*/
    pub const CUBLASLT_MATMUL_PREF_MAX_WAVES_COUNT: cublasLtMatmulPreferenceAttributes_t = cublasLtMatmulPreferenceAttributes_t(
        9,
    );
}
impl cublasLtMatmulPreferenceAttributes_t {
    /** Numerical implementation details mask, see cublasLtNumericalImplFlags_t. Filters heuristic result to only include
 algorithms that use the allowed implementations.

 uint64_t, default: uint64_t(-1) (allow everything)*/
    pub const CUBLASLT_MATMUL_PREF_IMPL_MASK: cublasLtMatmulPreferenceAttributes_t = cublasLtMatmulPreferenceAttributes_t(
        12,
    );
}
#[repr(transparent)]
/// Algo search preference to fine tune the heuristic function.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulPreferenceAttributes_t(pub ::core::ffi::c_uint);
/** Results structure used by cublasLtMatmulGetAlgo.

 Holds returned configured algo descriptor and its runtime properties.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct cublasLtMatmulHeuristicResult_t {
    /** Matmul algorithm descriptor.

 Must be initialized with cublasLtMatmulAlgoInit() if preferences' CUBLASLT_MATMUL_PERF_SEARCH_MODE is set to
 CUBLASLT_SEARCH_LIMITED_BY_ALGO_ID*/
    pub algo: cublasLtMatmulAlgo_t,
    /// Actual size of workspace memory required.
    pub workspaceSize: usize,
    /** Result status, other fields are only valid if after call to cublasLtMatmulAlgoGetHeuristic() this member is set to
 CUBLAS_STATUS_SUCCESS.*/
    pub state: cublasStatus_t,
    /** Waves count - a device utilization metric.

 wavesCount value of 1.0f suggests that when kernel is launched it will fully occupy the GPU.*/
    pub wavesCount: f32,
    pub reserved: [::core::ffi::c_int; 4usize],
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** support for split K, see CUBLASLT_ALGO_CONFIG_SPLITK_NUM

 int32_t, 0 means no support, supported otherwise*/
    pub const CUBLASLT_ALGO_CAP_SPLITK_SUPPORT: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        0,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** reduction scheme mask, see cublasLtReductionScheme_t; shows supported reduction schemes, if reduction scheme is
 not masked out it is supported.

 e.g. int isReductionSchemeComputeTypeSupported ? (reductionSchemeMask & CUBLASLT_REDUCTION_SCHEME_COMPUTE_TYPE) ==
 CUBLASLT_REDUCTION_SCHEME_COMPUTE_TYPE ? 1 : 0;

 uint32_t*/
    pub const CUBLASLT_ALGO_CAP_REDUCTION_SCHEME_MASK: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        1,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** support for cta swizzling, see CUBLASLT_ALGO_CONFIG_CTA_SWIZZLING

 uint32_t, 0 means no support, 1 means supported value of 1, other values are reserved*/
    pub const CUBLASLT_ALGO_CAP_CTA_SWIZZLING_SUPPORT: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        2,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** support strided batch

 int32_t, 0 means no support, supported otherwise*/
    pub const CUBLASLT_ALGO_CAP_STRIDED_BATCH_SUPPORT: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        3,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** support results out of place (D != C in D = alpha.A.B + beta.C)

 int32_t, 0 means no support, supported otherwise*/
    pub const CUBLASLT_ALGO_CAP_OUT_OF_PLACE_RESULT_SUPPORT: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        4,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** syrk/herk support (on top of regular gemm)

 int32_t, 0 means no support, supported otherwise*/
    pub const CUBLASLT_ALGO_CAP_UPLO_SUPPORT: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        5,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** tile ids possible to use, see cublasLtMatmulTile_t; if no tile ids are supported use
 CUBLASLT_MATMUL_TILE_UNDEFINED

 use cublasLtMatmulAlgoCapGetAttribute() with sizeInBytes=0 to query actual count

 array of uint32_t*/
    pub const CUBLASLT_ALGO_CAP_TILE_IDS: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        6,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** custom option range is from 0 to CUBLASLT_ALGO_CAP_CUSTOM_OPTION_MAX (inclusive), see
 CUBLASLT_ALGO_CONFIG_CUSTOM_OPTION

 int32_t*/
    pub const CUBLASLT_ALGO_CAP_CUSTOM_OPTION_MAX: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        7,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** whether algorithm supports custom (not COL or ROW memory order), see cublasLtOrder_t

 int32_t 0 means only COL and ROW memory order is allowed, non-zero means that algo might have different
 requirements;*/
    pub const CUBLASLT_ALGO_CAP_CUSTOM_MEMORY_ORDER: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        10,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** bitmask enumerating pointer modes algorithm supports

 uint32_t, see cublasLtPointerModeMask_t*/
    pub const CUBLASLT_ALGO_CAP_POINTER_MODE_MASK: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        11,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** bitmask enumerating kinds of postprocessing algorithm supports in the epilogue

 uint32_t, see cublasLtEpilogue_t*/
    pub const CUBLASLT_ALGO_CAP_EPILOGUE_MASK: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        12,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** stages ids possible to use, see cublasLtMatmulStages_t; if no stages ids are supported use
 CUBLASLT_MATMUL_STAGES_UNDEFINED

 use cublasLtMatmulAlgoCapGetAttribute() with sizeInBytes=0 to query actual count

 array of uint32_t*/
    pub const CUBLASLT_ALGO_CAP_STAGES_IDS: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        13,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** support for nagative ld for all of the matrices

 int32_t 0 means no support, supported otherwise*/
    pub const CUBLASLT_ALGO_CAP_LD_NEGATIVE: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        14,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** details about algorithm's implementation that affect it's numerical behavior

 uint64_t, see cublasLtNumericalImplFlags_t*/
    pub const CUBLASLT_ALGO_CAP_NUMERICAL_IMPL_FLAGS: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        15,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** minimum alignment required for A matrix in bytes
  (required for buffer pointer, leading dimension, and possibly other strides defined for matrix memory order)

 uint32_t*/
    pub const CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_A_BYTES: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        16,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** minimum alignment required for B matrix in bytes
  (required for buffer pointer, leading dimension, and possibly other strides defined for matrix memory order)

 uint32_t*/
    pub const CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_B_BYTES: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        17,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** minimum alignment required for C matrix in bytes
  (required for buffer pointer, leading dimension, and possibly other strides defined for matrix memory order)

 uint32_t*/
    pub const CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_C_BYTES: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        18,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** minimum alignment required for D matrix in bytes
  (required for buffer pointer, leading dimension, and possibly other strides defined for matrix memory order)

 uint32_t*/
    pub const CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_D_BYTES: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        19,
    );
}
impl cublasLtMatmulAlgoCapAttributes_t {
    /** EXPERIMENTAL: support for synchronization via atomic counters

 int32_t*/
    pub const CUBLASLT_ALGO_CAP_ATOMIC_SYNC: cublasLtMatmulAlgoCapAttributes_t = cublasLtMatmulAlgoCapAttributes_t(
        20,
    );
}
#[repr(transparent)]
/// Capabilities Attributes that can be retrieved from an initialized Algo structure
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulAlgoCapAttributes_t(pub ::core::ffi::c_uint);
impl cublasLtMatmulAlgoConfigAttributes_t {
    /** algorithm index, see cublasLtMatmulAlgoGetIds()

 readonly, set by cublasLtMatmulAlgoInit()
 int32_t*/
    pub const CUBLASLT_ALGO_CONFIG_ID: cublasLtMatmulAlgoConfigAttributes_t = cublasLtMatmulAlgoConfigAttributes_t(
        0,
    );
}
impl cublasLtMatmulAlgoConfigAttributes_t {
    /** tile id, see cublasLtMatmulTile_t

 uint32_t, default: CUBLASLT_MATMUL_TILE_UNDEFINED*/
    pub const CUBLASLT_ALGO_CONFIG_TILE_ID: cublasLtMatmulAlgoConfigAttributes_t = cublasLtMatmulAlgoConfigAttributes_t(
        1,
    );
}
impl cublasLtMatmulAlgoConfigAttributes_t {
    /** Number of K splits. If the number of K splits is greater than one, SPLITK_NUM parts
 of matrix multiplication will be computed in parallel. The results will be accumulated
 according to CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME

 int32_t, default: 1*/
    pub const CUBLASLT_ALGO_CONFIG_SPLITK_NUM: cublasLtMatmulAlgoConfigAttributes_t = cublasLtMatmulAlgoConfigAttributes_t(
        2,
    );
}
impl cublasLtMatmulAlgoConfigAttributes_t {
    /** reduction scheme, see cublasLtReductionScheme_t

 uint32_t, default: CUBLASLT_REDUCTION_SCHEME_NONE*/
    pub const CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME: cublasLtMatmulAlgoConfigAttributes_t = cublasLtMatmulAlgoConfigAttributes_t(
        3,
    );
}
impl cublasLtMatmulAlgoConfigAttributes_t {
    /** cta swizzling, change mapping from CUDA grid coordinates to parts of the matrices

 possible values: 0, 1, other values reserved

 uint32_t, default: 0*/
    pub const CUBLASLT_ALGO_CONFIG_CTA_SWIZZLING: cublasLtMatmulAlgoConfigAttributes_t = cublasLtMatmulAlgoConfigAttributes_t(
        4,
    );
}
impl cublasLtMatmulAlgoConfigAttributes_t {
    /** custom option, each algorithm can support some custom options that don't fit description of the other config
 attributes, see CUBLASLT_ALGO_CAP_CUSTOM_OPTION_MAX to get accepted range for any specific case

 uint32_t, default: 0*/
    pub const CUBLASLT_ALGO_CONFIG_CUSTOM_OPTION: cublasLtMatmulAlgoConfigAttributes_t = cublasLtMatmulAlgoConfigAttributes_t(
        5,
    );
}
impl cublasLtMatmulAlgoConfigAttributes_t {
    /** stages id, see cublasLtMatmulStages_t

 uint32_t, default: CUBLASLT_MATMUL_STAGES_UNDEFINED*/
    pub const CUBLASLT_ALGO_CONFIG_STAGES_ID: cublasLtMatmulAlgoConfigAttributes_t = cublasLtMatmulAlgoConfigAttributes_t(
        6,
    );
}
impl cublasLtMatmulAlgoConfigAttributes_t {
    /** inner shape id, see cublasLtMatmulInnerShape_t

 uint16_t, default: 0 (CUBLASLT_MATMUL_INNER_SHAPE_UNDEFINED)*/
    pub const CUBLASLT_ALGO_CONFIG_INNER_SHAPE_ID: cublasLtMatmulAlgoConfigAttributes_t = cublasLtMatmulAlgoConfigAttributes_t(
        7,
    );
}
impl cublasLtMatmulAlgoConfigAttributes_t {
    /** Thread Block Cluster shape id, see cublasLtClusterShape_t. Defines cluster size to use.

 uint16_t, default: 0 (CUBLASLT_CLUSTER_SHAPE_AUTO)*/
    pub const CUBLASLT_ALGO_CONFIG_CLUSTER_SHAPE_ID: cublasLtMatmulAlgoConfigAttributes_t = cublasLtMatmulAlgoConfigAttributes_t(
        8,
    );
}
#[repr(transparent)]
/// Algo Configuration Attributes that can be set according to the Algo capabilities
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulAlgoConfigAttributes_t(pub ::core::ffi::c_uint);
/// Experimental: Logger callback type.
pub type cublasLtLoggerCallback_t = ::core::option::Option<
    unsafe extern "C" fn(
        logLevel: ::core::ffi::c_int,
        functionName: *const ::core::ffi::c_char,
        message: *const ::core::ffi::c_char,
    ),
>;
