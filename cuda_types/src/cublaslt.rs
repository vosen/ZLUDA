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
    pub const CUBLASLT_MATMUL_TILE_8x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        36,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        37,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        38,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        39,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        40,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        41,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        42,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        43,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x640: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        44,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x704: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        45,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_8x768: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        46,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        47,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        48,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        49,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        50,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        51,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        52,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        53,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        54,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        55,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x640: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        56,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x704: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        57,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_16x768: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        58,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        59,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        60,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        61,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        62,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        63,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        64,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        65,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        66,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        67,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x640: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        68,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x704: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        69,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_24x768: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        70,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        71,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        72,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        73,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        74,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        75,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        76,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x640: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        77,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x704: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        78,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_32x768: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        79,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        80,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        81,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        82,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        83,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        84,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        85,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        86,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        87,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        88,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x640: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        89,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x704: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        90,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_40x768: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        91,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        92,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        93,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        94,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        95,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        96,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        97,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        98,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        99,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        100,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x640: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        101,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x704: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        102,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_48x768: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        103,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        104,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        105,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        106,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        107,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        108,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        109,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        110,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        111,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        112,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x640: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        113,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x704: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        114,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_56x768: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        115,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        116,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        117,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        118,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        119,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        120,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x640: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        121,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x704: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        122,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x768: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        123,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        124,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        125,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        126,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        127,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        128,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        129,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        130,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        131,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        132,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_72x640: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        133,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_80x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        134,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_80x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        135,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_80x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        136,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_80x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        137,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_80x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        138,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_80x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        139,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_80x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        140,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_80x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        141,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_80x576: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        142,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_88x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        143,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_88x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        144,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_88x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        145,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_88x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        146,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_88x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        147,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_88x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        148,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_88x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        149,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_88x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        150,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_96x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        151,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_96x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        152,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_96x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        153,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_96x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        154,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_96x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        155,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_96x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        156,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_104x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        157,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_104x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        158,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_104x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        159,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_104x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        160,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_104x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        161,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_104x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        162,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_104x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        163,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_112x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        164,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_112x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        165,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_112x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        166,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_112x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        167,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_112x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        168,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_112x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        169,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_120x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        170,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_120x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        171,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_120x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        172,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_120x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        173,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_120x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        174,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_120x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        175,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        176,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x384: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        177,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_136x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        178,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_136x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        179,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_136x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        180,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_136x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        181,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_136x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        182,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_144x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        183,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_144x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        184,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_144x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        185,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_144x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        186,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_144x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        187,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_152x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        188,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_152x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        189,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_152x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        190,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_152x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        191,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_152x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        192,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_160x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        193,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_160x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        194,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_160x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        195,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_168x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        196,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_168x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        197,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_168x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        198,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_168x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        199,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_176x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        200,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_176x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        201,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_176x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        202,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_176x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        203,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_184x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        204,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_184x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        205,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_184x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        206,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_184x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        207,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        208,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        209,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        210,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_200x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        211,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_200x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        212,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_200x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        213,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_208x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        214,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_208x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        215,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_208x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        216,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_216x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        217,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_216x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        218,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_216x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        219,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_224x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        220,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_224x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        221,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_224x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        222,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_232x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        223,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_232x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        224,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_232x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        225,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_240x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        226,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_240x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        227,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_240x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        228,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_248x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        229,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_248x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        230,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_248x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        231,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        232,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_264x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        233,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_264x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        234,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_272x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        235,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_272x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        236,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_280x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        237,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_280x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        238,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_288x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        239,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_288x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        240,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_296x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        241,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_296x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        242,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_304x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        243,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_304x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        244,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_312x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        245,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_312x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        246,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        247,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        248,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_328x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        249,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_328x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        250,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_336x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        251,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_336x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        252,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_344x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        253,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_344x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        254,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_352x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        255,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_352x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        256,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_360x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        257,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_360x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        258,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_368x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        259,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_368x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        260,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_376x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        261,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_376x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        262,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        263,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        264,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_392x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        265,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_400x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        266,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_408x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        267,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_416x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        268,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_424x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        269,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_432x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        270,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_440x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        271,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        272,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_456x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        273,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_464x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        274,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_472x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        275,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_480x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        276,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_488x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        277,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_496x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        278,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_504x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        279,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_520x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        280,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_528x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        281,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_536x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        282,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_544x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        283,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_552x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        284,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_560x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        285,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_568x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        286,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        287,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_584x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        288,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_592x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        289,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_600x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        290,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_608x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        291,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_616x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        292,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_624x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        293,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_632x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        294,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        295,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_648x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        296,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_656x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        297,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_664x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        298,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_672x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        299,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_680x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        300,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_688x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        301,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_696x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        302,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        303,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_712x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        304,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_720x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        305,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_728x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        306,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_736x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        307,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_744x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        308,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_752x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        309,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_760x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        310,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x64: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        311,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        312,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        313,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        314,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        315,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        316,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        317,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        318,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        319,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x104: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        320,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x112: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        321,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x120: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        322,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x136: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        323,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x144: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        324,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x152: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        325,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x160: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        326,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x168: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        327,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x176: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        328,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x184: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        329,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x200: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        330,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x208: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        331,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x216: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        332,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x224: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        333,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x232: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        334,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x240: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        335,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x248: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        336,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x264: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        337,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x272: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        338,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x280: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        339,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x288: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        340,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x296: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        341,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x304: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        342,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x312: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        343,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x328: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        344,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x336: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        345,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x344: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        346,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x352: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        347,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x360: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        348,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x368: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        349,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x376: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        350,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x392: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        351,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x400: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        352,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x408: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        353,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x416: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        354,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x424: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        355,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x432: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        356,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x440: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        357,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x456: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        358,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x464: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        359,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x472: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        360,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x480: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        361,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x488: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        362,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x496: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        363,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x504: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        364,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x520: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        365,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x528: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        366,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x536: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        367,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x544: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        368,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x552: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        369,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x560: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        370,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x568: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        371,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x584: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        372,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x592: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        373,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x600: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        374,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x608: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        375,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x616: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        376,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x624: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        377,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x632: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        378,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x648: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        379,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x656: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        380,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x664: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        381,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x672: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        382,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x680: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        383,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x688: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        384,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x696: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        385,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x712: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        386,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x720: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        387,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x728: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        388,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x736: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        389,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x744: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        390,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x752: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        391,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_64x760: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        392,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        393,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        394,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        395,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        396,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        397,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        398,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        399,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        400,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        401,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x104: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        402,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x112: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        403,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x120: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        404,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x136: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        405,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x144: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        406,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x152: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        407,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x168: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        408,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x176: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        409,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x184: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        410,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x200: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        411,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x208: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        412,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x216: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        413,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x224: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        414,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x232: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        415,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x240: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        416,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x248: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        417,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x264: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        418,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x272: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        419,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x280: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        420,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x288: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        421,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x296: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        422,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x304: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        423,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x312: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        424,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x328: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        425,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x336: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        426,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x344: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        427,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x352: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        428,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x360: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        429,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x368: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        430,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x376: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        431,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x392: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        432,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x400: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        433,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x408: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        434,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x416: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        435,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x424: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        436,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x432: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        437,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x440: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        438,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x448: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        439,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x456: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        440,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x464: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        441,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x472: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        442,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x480: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        443,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x488: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        444,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x496: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        445,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x504: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        446,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_128x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        447,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        448,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        449,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        450,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        451,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        452,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        453,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        454,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        455,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        456,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        457,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        458,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x104: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        459,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x112: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        460,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x120: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        461,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x136: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        462,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x144: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        463,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x152: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        464,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x160: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        465,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x168: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        466,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x176: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        467,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x184: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        468,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x200: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        469,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x208: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        470,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x216: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        471,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x224: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        472,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x232: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        473,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x240: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        474,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x248: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        475,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x264: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        476,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x272: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        477,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x280: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        478,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x288: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        479,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x296: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        480,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x304: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        481,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x312: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        482,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x320: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        483,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x328: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        484,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_192x336: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        485,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        486,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        487,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        488,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        489,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        490,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        491,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        492,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        493,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        494,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        495,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x104: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        496,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x112: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        497,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x120: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        498,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x136: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        499,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x144: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        500,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x152: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        501,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x160: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        502,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x168: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        503,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x176: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        504,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x184: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        505,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x200: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        506,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x208: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        507,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x216: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        508,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x224: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        509,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x232: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        510,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x240: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        511,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x248: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        512,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x256: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        513,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        514,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        515,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        516,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        517,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        518,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        519,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        520,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        521,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        522,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        523,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        524,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x104: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        525,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x112: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        526,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x120: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        527,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x136: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        528,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x144: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        529,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x152: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        530,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x160: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        531,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x168: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        532,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x176: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        533,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x184: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        534,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x192: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        535,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_320x200: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        536,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        537,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        538,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        539,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        540,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        541,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        542,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        543,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        544,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        545,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        546,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        547,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x104: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        548,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x112: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        549,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x120: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        550,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x136: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        551,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x144: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        552,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x152: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        553,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x160: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        554,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_384x168: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        555,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        556,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        557,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        558,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        559,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        560,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        561,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        562,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        563,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        564,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        565,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        566,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x104: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        567,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x112: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        568,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x120: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        569,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        570,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x136: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        571,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_448x144: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        572,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        573,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        574,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        575,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        576,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        577,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        578,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        579,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        580,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        581,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        582,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        583,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x104: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        584,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x112: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        585,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x120: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        586,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x128: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        587,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        588,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        589,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        590,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        591,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        592,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        593,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        594,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        595,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        596,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        597,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        598,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x104: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        599,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_576x112: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        600,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        601,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        602,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        603,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        604,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        605,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        606,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        607,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        608,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        609,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        610,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_640x96: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        611,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        612,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        613,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        614,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        615,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        616,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        617,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        618,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        619,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        620,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_704x88: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        621,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x8: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        622,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x16: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        623,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x24: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        624,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x32: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        625,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x40: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        626,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x48: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        627,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x56: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        628,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x72: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        629,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_768x80: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        630,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        631,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_256x1024: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        632,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x512: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        633,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_512x1024: cublasLtMatmulTile_t = cublasLtMatmulTile_t(
        634,
    );
}
impl cublasLtMatmulTile_t {
    pub const CUBLASLT_MATMUL_TILE_END: cublasLtMatmulTile_t = cublasLtMatmulTile_t(635);
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
    pub const CUBLASLT_MATMUL_STAGES_256xAUTO: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        37,
    );
}
impl cublasLtMatmulStages_t {
    pub const CUBLASLT_MATMUL_STAGES_END: cublasLtMatmulStages_t = cublasLtMatmulStages_t(
        38,
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
impl cublasLtMatmulMatrixScale_t {
    /// Scaling factors are single precision scalars applied to the whole tensor
    pub const CUBLASLT_MATMUL_MATRIX_SCALE_SCALAR_32F: cublasLtMatmulMatrixScale_t = cublasLtMatmulMatrixScale_t(
        0,
    );
}
impl cublasLtMatmulMatrixScale_t {
    /** Scaling factors are tensors that contain a dedicated scaling factor stored as an 8-bit CUDA_R_8F_UE4M3 value for
each 16-element block in the innermost dimension of the corresponding data tensor*/
    pub const CUBLASLT_MATMUL_MATRIX_SCALE_VEC16_UE4M3: cublasLtMatmulMatrixScale_t = cublasLtMatmulMatrixScale_t(
        1,
    );
}
impl cublasLtMatmulMatrixScale_t {
    /** Same as above, except that scaling factor tensor elements have type CUDA_R_8F_UE8M0 and the block size is 32
elements*/
    pub const CUBLASLT_MATMUL_MATRIX_SCALE_VEC32_UE8M0: cublasLtMatmulMatrixScale_t = cublasLtMatmulMatrixScale_t(
        2,
    );
}
impl cublasLtMatmulMatrixScale_t {
    /** Same as above, except that scaling factor tensor elements have type CUDA_R_8F_UE8M0 and the block size is 32
elements*/
    pub const CUBLASLT_MATMUL_MATRIX_SCALE_END: cublasLtMatmulMatrixScale_t = cublasLtMatmulMatrixScale_t(
        3,
    );
}
#[repr(transparent)]
/// Scaling mode for per-matrix scaling
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct cublasLtMatmulMatrixScale_t(pub ::core::ffi::c_uint);
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
    /** EXPERIMENTAL, DEPRECATED: Number of atomic synchronization chunks in the row dimension of the output matrix D.

 int32_t, default 0 (atomic synchronization disabled)*/
    pub const CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_NUM_CHUNKS_D_ROWS: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        27,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** EXPERIMENTAL, DEPRECATED: Number of atomic synchronization chunks in the column dimension of the output matrix D.

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
impl cublasLtMatmulDescAttributes_t {
    /** Scaling mode that defines how the matrix scaling factor for matrix A is interpreted

 int32_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_A_SCALE_MODE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        31,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Scaling mode that defines how the matrix scaling factor for matrix B is interpreted

 int32_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_B_SCALE_MODE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        32,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Scaling mode that defines how the matrix scaling factor for matrix C is interpreted

 int32_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_C_SCALE_MODE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        33,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Scaling mode that defines how the matrix scaling factor for matrix D is interpreted

 int32_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_D_SCALE_MODE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        34,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Scaling mode that defines how the matrix scaling factor for the auxiliary matrix is interpreted

 int32_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_MODE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        35,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Device pointer to the scale factors that are used to convert data in matrix D to the compute data type range.

  The scaling factor value type is defined by the scaling mode (see CUBLASLT_MATMUL_DESC_D_OUT_SCALE_MODE)

  If set for an unsupported matrix data, scale, scale mode, and compute type combination, calling cublasLtMatmul()
  will return CUBLAS_INVALID_VALUE.

  void *, default: NULL*/
    pub const CUBLASLT_MATMUL_DESC_D_OUT_SCALE_POINTER: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        36,
    );
}
impl cublasLtMatmulDescAttributes_t {
    /** Scaling mode that defines how the output matrix scaling factor for matrix D is interpreted

 int32_t, default: 0*/
    pub const CUBLASLT_MATMUL_DESC_D_OUT_SCALE_MODE: cublasLtMatmulDescAttributes_t = cublasLtMatmulDescAttributes_t(
        37,
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
impl cublasLtMatmulSearch_t {
    /// reserved for future use
    pub const CUBLASLT_SEARCH_RESERVED_06: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        6,
    );
}
impl cublasLtMatmulSearch_t {
    /// reserved for future use
    pub const CUBLASLT_SEARCH_RESERVED_07: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        7,
    );
}
impl cublasLtMatmulSearch_t {
    /// reserved for future use
    pub const CUBLASLT_SEARCH_RESERVED_08: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        8,
    );
}
impl cublasLtMatmulSearch_t {
    /// reserved for future use
    pub const CUBLASLT_SEARCH_RESERVED_09: cublasLtMatmulSearch_t = cublasLtMatmulSearch_t(
        9,
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
/** Results structure used by cublasLtMatmulAlgoGetHeuristic

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
