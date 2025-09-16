// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
impl hipblasOperation_t {
    ///<  Operate with the matrix.
    pub const HIPBLAS_OP_N: hipblasOperation_t = hipblasOperation_t(111);
}
impl hipblasOperation_t {
    ///<  Operate with the transpose of the matrix.
    pub const HIPBLAS_OP_T: hipblasOperation_t = hipblasOperation_t(112);
}
impl hipblasOperation_t {
    ///< Operate with the conjugate transpose of the matrix.
    pub const HIPBLAS_OP_C: hipblasOperation_t = hipblasOperation_t(113);
}
#[repr(transparent)]
/// \brief Used to specify whether the matrix is to be transposed or not.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasOperation_t(pub ::core::ffi::c_uint);
impl hipblasComputeType_t {
    ///< compute will be at least 16-bit precision
    pub const HIPBLAS_COMPUTE_16F: hipblasComputeType_t = hipblasComputeType_t(0);
}
impl hipblasComputeType_t {
    ///< compute will be exactly 16-bit precision
    pub const HIPBLAS_COMPUTE_16F_PEDANTIC: hipblasComputeType_t = hipblasComputeType_t(
        1,
    );
}
impl hipblasComputeType_t {
    ///< compute will be at least 32-bit precision
    pub const HIPBLAS_COMPUTE_32F: hipblasComputeType_t = hipblasComputeType_t(2);
}
impl hipblasComputeType_t {
    ///< compute will be exactly 32-bit precision
    pub const HIPBLAS_COMPUTE_32F_PEDANTIC: hipblasComputeType_t = hipblasComputeType_t(
        3,
    );
}
impl hipblasComputeType_t {
    ///< 32-bit input can use 16-bit compute
    pub const HIPBLAS_COMPUTE_32F_FAST_16F: hipblasComputeType_t = hipblasComputeType_t(
        4,
    );
}
impl hipblasComputeType_t {
    ///< 32-bit input can is bf16 compute
    pub const HIPBLAS_COMPUTE_32F_FAST_16BF: hipblasComputeType_t = hipblasComputeType_t(
        5,
    );
}
impl hipblasComputeType_t {
    pub const HIPBLAS_COMPUTE_32F_FAST_TF32: hipblasComputeType_t = hipblasComputeType_t(
        6,
    );
}
impl hipblasComputeType_t {
    ///< compute will be at least 64-bit precision
    pub const HIPBLAS_COMPUTE_64F: hipblasComputeType_t = hipblasComputeType_t(7);
}
impl hipblasComputeType_t {
    ///< compute will be exactly 64-bit precision
    pub const HIPBLAS_COMPUTE_64F_PEDANTIC: hipblasComputeType_t = hipblasComputeType_t(
        8,
    );
}
impl hipblasComputeType_t {
    ///< compute will be at least 32-bit integer precision
    pub const HIPBLAS_COMPUTE_32I: hipblasComputeType_t = hipblasComputeType_t(9);
}
impl hipblasComputeType_t {
    ///< compute will be exactly 32-bit integer precision
    pub const HIPBLAS_COMPUTE_32I_PEDANTIC: hipblasComputeType_t = hipblasComputeType_t(
        10,
    );
}
#[repr(transparent)]
/** \brief The compute type to be used. Currently only used with GemmEx with the HIPBLAS_V2 interface.
         Note that support for compute types is largely dependent on backend.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasComputeType_t(pub ::core::ffi::c_uint);
/// \brief Struct to represent a 16 bit brain floating point number.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hip_bfloat16 {
    pub data: u16,
}
impl hipDataType {
    pub const HIP_R_32F: hipDataType = hipDataType(0);
}
impl hipDataType {
    pub const HIP_R_64F: hipDataType = hipDataType(1);
}
impl hipDataType {
    pub const HIP_R_16F: hipDataType = hipDataType(2);
}
impl hipDataType {
    pub const HIP_R_8I: hipDataType = hipDataType(3);
}
impl hipDataType {
    pub const HIP_C_32F: hipDataType = hipDataType(4);
}
impl hipDataType {
    pub const HIP_C_64F: hipDataType = hipDataType(5);
}
impl hipDataType {
    pub const HIP_C_16F: hipDataType = hipDataType(6);
}
impl hipDataType {
    pub const HIP_C_8I: hipDataType = hipDataType(7);
}
impl hipDataType {
    pub const HIP_R_8U: hipDataType = hipDataType(8);
}
impl hipDataType {
    pub const HIP_C_8U: hipDataType = hipDataType(9);
}
impl hipDataType {
    pub const HIP_R_32I: hipDataType = hipDataType(10);
}
impl hipDataType {
    pub const HIP_C_32I: hipDataType = hipDataType(11);
}
impl hipDataType {
    pub const HIP_R_32U: hipDataType = hipDataType(12);
}
impl hipDataType {
    pub const HIP_C_32U: hipDataType = hipDataType(13);
}
impl hipDataType {
    pub const HIP_R_16BF: hipDataType = hipDataType(14);
}
impl hipDataType {
    pub const HIP_C_16BF: hipDataType = hipDataType(15);
}
impl hipDataType {
    pub const HIP_R_4I: hipDataType = hipDataType(16);
}
impl hipDataType {
    pub const HIP_C_4I: hipDataType = hipDataType(17);
}
impl hipDataType {
    pub const HIP_R_4U: hipDataType = hipDataType(18);
}
impl hipDataType {
    pub const HIP_C_4U: hipDataType = hipDataType(19);
}
impl hipDataType {
    pub const HIP_R_16I: hipDataType = hipDataType(20);
}
impl hipDataType {
    pub const HIP_C_16I: hipDataType = hipDataType(21);
}
impl hipDataType {
    pub const HIP_R_16U: hipDataType = hipDataType(22);
}
impl hipDataType {
    pub const HIP_C_16U: hipDataType = hipDataType(23);
}
impl hipDataType {
    pub const HIP_R_64I: hipDataType = hipDataType(24);
}
impl hipDataType {
    pub const HIP_C_64I: hipDataType = hipDataType(25);
}
impl hipDataType {
    pub const HIP_R_64U: hipDataType = hipDataType(26);
}
impl hipDataType {
    pub const HIP_C_64U: hipDataType = hipDataType(27);
}
impl hipDataType {
    pub const HIP_R_8F_E4M3: hipDataType = hipDataType(28);
}
impl hipDataType {
    pub const HIP_R_8F_E5M2: hipDataType = hipDataType(29);
}
impl hipDataType {
    pub const HIP_R_8F_E4M3_FNUZ: hipDataType = hipDataType(1000);
}
impl hipDataType {
    pub const HIP_R_8F_E5M2_FNUZ: hipDataType = hipDataType(1001);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipDataType(pub ::core::ffi::c_uint);
/// \brief Single precision floating point type
pub type hipblasLtFloat = f32;
/// \brief Structure definition for hipblasLtHalf
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct _hipblasLtHalf {
    pub data: u16,
}
/// \brief Structure definition for hipblasLtHalf
pub type hipblasLtHalf = _hipblasLtHalf;
/// \brief Struct to represent a 16 bit brain floating point number.
pub type hipblasLtBfloat16 = hip_bfloat16;
pub type hipblasLtInt8 = i8;
pub type hipblasLtInt32 = i32;
impl hipblasLtEpilogue_t {
    ///<No special postprocessing, just scale and quantize the results if necessary.
    pub const HIPBLASLT_EPILOGUE_DEFAULT: hipblasLtEpilogue_t = hipblasLtEpilogue_t(1);
}
impl hipblasLtEpilogue_t {
    ///<Apply ReLU point-wise transform to the results:(x:=max(x, 0))
    pub const HIPBLASLT_EPILOGUE_RELU: hipblasLtEpilogue_t = hipblasLtEpilogue_t(2);
}
impl hipblasLtEpilogue_t {
    ///<Apply (broadcast) bias from the bias vector. Bias vector length must match matrix D rows, and it must be packed (such as stride between vector elements is 1). Bias vector is broadcast to all columns and added before applying the final postprocessing.
    pub const HIPBLASLT_EPILOGUE_BIAS: hipblasLtEpilogue_t = hipblasLtEpilogue_t(4);
}
impl hipblasLtEpilogue_t {
    ///<Apply bias and then ReLU transform.
    pub const HIPBLASLT_EPILOGUE_RELU_BIAS: hipblasLtEpilogue_t = hipblasLtEpilogue_t(6);
}
impl hipblasLtEpilogue_t {
    ///<Apply GELU point-wise transform to the results (x:=GELU(x)).
    pub const HIPBLASLT_EPILOGUE_GELU: hipblasLtEpilogue_t = hipblasLtEpilogue_t(32);
}
impl hipblasLtEpilogue_t {
    ///<Apply Bias and then GELU transform.
    pub const HIPBLASLT_EPILOGUE_GELU_BIAS: hipblasLtEpilogue_t = hipblasLtEpilogue_t(
        36,
    );
}
impl hipblasLtEpilogue_t {
    ///<Output GEMM results before applying GELU transform.
    pub const HIPBLASLT_EPILOGUE_GELU_AUX: hipblasLtEpilogue_t = hipblasLtEpilogue_t(
        160,
    );
}
impl hipblasLtEpilogue_t {
    ///<Output GEMM results after applying bias but before applying GELU transform.
    pub const HIPBLASLT_EPILOGUE_GELU_AUX_BIAS: hipblasLtEpilogue_t = hipblasLtEpilogue_t(
        164,
    );
}
impl hipblasLtEpilogue_t {
    ///<Apply gradient GELU transform. Requires additional aux input.
    pub const HIPBLASLT_EPILOGUE_DGELU: hipblasLtEpilogue_t = hipblasLtEpilogue_t(192);
}
impl hipblasLtEpilogue_t {
    ///<Apply gradient GELU transform and bias gradient to the results. Requires additional aux input.
    pub const HIPBLASLT_EPILOGUE_DGELU_BGRAD: hipblasLtEpilogue_t = hipblasLtEpilogue_t(
        208,
    );
}
impl hipblasLtEpilogue_t {
    ///<Apply bias gradient to A and output gemm result.
    pub const HIPBLASLT_EPILOGUE_BGRADA: hipblasLtEpilogue_t = hipblasLtEpilogue_t(256);
}
impl hipblasLtEpilogue_t {
    ///<Apply bias gradient to B and output gemm result.
    pub const HIPBLASLT_EPILOGUE_BGRADB: hipblasLtEpilogue_t = hipblasLtEpilogue_t(512);
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify the enum type to set the postprocessing options for the epilogue.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtEpilogue_t(pub ::core::ffi::c_uint);
impl hipblasLtMatrixLayoutAttribute_t {
    ///<Number of batch of this matrix. Default value is 1. Data Type: int32_t
    pub const HIPBLASLT_MATRIX_LAYOUT_BATCH_COUNT: hipblasLtMatrixLayoutAttribute_t = hipblasLtMatrixLayoutAttribute_t(
        0,
    );
}
impl hipblasLtMatrixLayoutAttribute_t {
    ///<Stride (in elements) to the next matrix for the strided batch operation. Default value is 0. Data Type: int64_t
    pub const HIPBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET: hipblasLtMatrixLayoutAttribute_t = hipblasLtMatrixLayoutAttribute_t(
        1,
    );
}
impl hipblasLtMatrixLayoutAttribute_t {
    /** Data type, see hipDataType.

 uint32_t*/
    pub const HIPBLASLT_MATRIX_LAYOUT_TYPE: hipblasLtMatrixLayoutAttribute_t = hipblasLtMatrixLayoutAttribute_t(
        2,
    );
}
impl hipblasLtMatrixLayoutAttribute_t {
    /** Memory order of the data, see hipblasLtOrder_t.

 int32_t, default: HIPBLASLT_ORDER_COL*/
    pub const HIPBLASLT_MATRIX_LAYOUT_ORDER: hipblasLtMatrixLayoutAttribute_t = hipblasLtMatrixLayoutAttribute_t(
        3,
    );
}
impl hipblasLtMatrixLayoutAttribute_t {
    /** Number of rows.

 Usually only values that can be expressed as int32_t are supported.

 uint64_t*/
    pub const HIPBLASLT_MATRIX_LAYOUT_ROWS: hipblasLtMatrixLayoutAttribute_t = hipblasLtMatrixLayoutAttribute_t(
        4,
    );
}
impl hipblasLtMatrixLayoutAttribute_t {
    /** Number of columns.

 Usually only values that can be expressed as int32_t are supported.

 uint64_t*/
    pub const HIPBLASLT_MATRIX_LAYOUT_COLS: hipblasLtMatrixLayoutAttribute_t = hipblasLtMatrixLayoutAttribute_t(
        5,
    );
}
impl hipblasLtMatrixLayoutAttribute_t {
    /** Matrix leading dimension.

 For HIPBLASLT_ORDER_COL this is stride (in elements) of matrix column, for more details and documentation for
 other memory orders see documentation for hipblasLtOrder_t values.

 Currently only non-negative values are supported, must be large enough so that matrix memory locations are not
 overlapping (e.g. greater or equal to HIPBLASLT_MATRIX_LAYOUT_ROWS in case of HIPBLASLT_ORDER_COL).

 int64_t;*/
    pub const HIPBLASLT_MATRIX_LAYOUT_LD: hipblasLtMatrixLayoutAttribute_t = hipblasLtMatrixLayoutAttribute_t(
        6,
    );
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify the attributes that define the details of the matrix.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatrixLayoutAttribute_t(pub ::core::ffi::c_uint);
impl hipblasLtPointerMode_t {
    pub const HIPBLASLT_POINTER_MODE_HOST: hipblasLtPointerMode_t = hipblasLtPointerMode_t(
        0,
    );
}
impl hipblasLtPointerMode_t {
    /// targets host memory
    pub const HIPBLASLT_POINTER_MODE_DEVICE: hipblasLtPointerMode_t = hipblasLtPointerMode_t(
        1,
    );
}
impl hipblasLtPointerMode_t {
    /// targets device memory
    pub const HIPBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_HOST: hipblasLtPointerMode_t = hipblasLtPointerMode_t(
        4,
    );
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Pointer mode to use for alpha.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtPointerMode_t(pub ::core::ffi::c_uint);
impl hipblasLtMatmulDescAttributes_t {
    ///<Specifies the type of transformation operation that should be performed on matrix A. Default value is HIPBLAS_OP_N (for example, non-transpose operation). See hipblasOperation_t. Data Type:int32_t
    pub const HIPBLASLT_MATMUL_DESC_TRANSA: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        0,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Specifies the type of transformation operation that should be performed on matrix B. Default value is HIPBLAS_OP_N (for example, non-transpose operation). See hipblasOperation_t. Data Type:int32_t
    pub const HIPBLASLT_MATMUL_DESC_TRANSB: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        1,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Epilogue function. See hipblasLtEpilogue_t. Default value is: HIPBLASLT_EPILOGUE_DEFAULT. Data Type: uint32_t
    pub const HIPBLASLT_MATMUL_DESC_EPILOGUE: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        2,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Bias or Bias gradient vector pointer in the device memory. Data Type:void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_BIAS_POINTER: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        3,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Type of the bias vector in the device memory. Can be set same as D matrix type or Scale type. Bias case: see HIPBLASLT_EPILOGUE_BIAS. Data Type:int32_t based on hipDataType
    pub const HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        4,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Device pointer to the scale factor value that converts data in matrix A to the compute data type range. The scaling factor must have the same type as the compute type. If not specified, or set to NULL, the scaling factor is assumed to be 1. If set for an unsupported matrix data, scale, and compute type combination, calling hipblasLtMatmul() will return HIPBLAS_INVALID_VALUE. Default value: NULL Data Type: void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        5,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Equivalent to HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER for matrix B. Default value: NULL Type: void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        6,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Equivalent to HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER for matrix C. Default value: NULL Type: void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_C_SCALE_POINTER: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        7,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Equivalent to HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER for matrix D. Default value: NULL Type: void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_D_SCALE_POINTER: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        8,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Equivalent to HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER for matrix AUX. Default value: NULL Type: void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        9,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Epilogue auxiliary buffer pointer in the device memory. Data Type:void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        10,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<The leading dimension of the epilogue auxiliary buffer pointer in the device memory. Data Type:int64_t
    pub const HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        11,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<The batch stride of the epilogue auxiliary buffer pointer in the device memory. Data Type:int64_t
    pub const HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        12,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Specifies alpha and beta are passed by reference, whether they are scalars on the host or on the device, or device vectors. Default value is: HIPBLASLT_POINTER_MODE_HOST (i.e., on the host). Data Type: int32_t based on hipblasLtPointerMode_t
    pub const HIPBLASLT_MATMUL_DESC_POINTER_MODE: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        13,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Device pointer to the memory location that on completion will be set to the maximum of absolute values in the output matrix. Data Type:void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_AMAX_D_POINTER: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        14,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Compute input A types. Defines the data type used for the input A of matrix multiply.
    pub const HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_A_EXT: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        100,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Compute input B types. Defines the data type used for the input B of matrix multiply.
    pub const HIPBLASLT_MATMUL_DESC_COMPUTE_INPUT_TYPE_B_EXT: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        101,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Equivalent to HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER but in vector. Default value: NULL Type: void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER_VEC_EXT: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        102,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    ///<Equivalent to HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER but in vector. Default value: NULL Type: void* /const void*
    pub const HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER_VEC_EXT: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        103,
    );
}
impl hipblasLtMatmulDescAttributes_t {
    pub const HIPBLASLT_MATMUL_DESC_MAX: hipblasLtMatmulDescAttributes_t = hipblasLtMatmulDescAttributes_t(
        104,
    );
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify the attributes that define the specifics of the matrix multiply operation.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatmulDescAttributes_t(pub ::core::ffi::c_uint);
impl hipblasLtMatmulPreferenceAttributes_t {
    ///<Search mode. Data Type: uint32_t
    pub const HIPBLASLT_MATMUL_PREF_SEARCH_MODE: hipblasLtMatmulPreferenceAttributes_t = hipblasLtMatmulPreferenceAttributes_t(
        0,
    );
}
impl hipblasLtMatmulPreferenceAttributes_t {
    ///<Maximum allowed workspace memory. Default is 0 (no workspace memory allowed). Data Type: uint64_t
    pub const HIPBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES: hipblasLtMatmulPreferenceAttributes_t = hipblasLtMatmulPreferenceAttributes_t(
        1,
    );
}
impl hipblasLtMatmulPreferenceAttributes_t {
    pub const HIPBLASLT_MATMUL_PREF_MAX: hipblasLtMatmulPreferenceAttributes_t = hipblasLtMatmulPreferenceAttributes_t(
        2,
    );
}
#[repr(transparent)]
/** \ingroup types_module
  \brief It is an enumerated type used to apply algorithm search preferences while fine-tuning the heuristic function.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatmulPreferenceAttributes_t(pub ::core::ffi::c_uint);
impl hipblasLtOrder_t {
    /** Column-major

 Leading dimension is the stride (in elements) to the beginning of next column in memory.*/
    pub const HIPBLASLT_ORDER_COL: hipblasLtOrder_t = hipblasLtOrder_t(0);
}
impl hipblasLtOrder_t {
    /** Row major

 Leading dimension is the stride (in elements) to the beginning of next row in memory.*/
    pub const HIPBLASLT_ORDER_ROW: hipblasLtOrder_t = hipblasLtOrder_t(1);
}
#[repr(transparent)]
/// Enum for data ordering
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtOrder_t(pub ::core::ffi::c_uint);
impl hipblasLtMatrixTransformDescAttributes_t {
    /** Scale type, see hipDataType. Inputs are converted to scale type for scaling and summation and results are then
 converted to output type to store in memory.

 int32_t*/
    pub const HIPBLASLT_MATRIX_TRANSFORM_DESC_SCALE_TYPE: hipblasLtMatrixTransformDescAttributes_t = hipblasLtMatrixTransformDescAttributes_t(
        0,
    );
}
impl hipblasLtMatrixTransformDescAttributes_t {
    /** Pointer mode of alpha and beta, see hipblasLtPointerMode_t.

 int32_t, default: HIPBLASLT_POINTER_MODE_HOST*/
    pub const HIPBLASLT_MATRIX_TRANSFORM_DESC_POINTER_MODE: hipblasLtMatrixTransformDescAttributes_t = hipblasLtMatrixTransformDescAttributes_t(
        1,
    );
}
impl hipblasLtMatrixTransformDescAttributes_t {
    /** Transform of matrix A, see hipblasOperation_t.

 int32_t, default: HIPBLAS_OP_N*/
    pub const HIPBLASLT_MATRIX_TRANSFORM_DESC_TRANSA: hipblasLtMatrixTransformDescAttributes_t = hipblasLtMatrixTransformDescAttributes_t(
        2,
    );
}
impl hipblasLtMatrixTransformDescAttributes_t {
    /** Transform of matrix B, see hipblasOperation_t.

 int32_t, default: HIPBLAS_OP_N*/
    pub const HIPBLASLT_MATRIX_TRANSFORM_DESC_TRANSB: hipblasLtMatrixTransformDescAttributes_t = hipblasLtMatrixTransformDescAttributes_t(
        3,
    );
}
#[repr(transparent)]
/// Matrix transform descriptor attributes to define details of the operation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatrixTransformDescAttributes_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatmulDescOpaque_t {
    pub data: [u64; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatrixLayoutOpaque_t {
    pub data: [u64; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatmulPreferenceOpaque_t {
    pub data: [u64; 5usize],
}
/// Semi-opaque descriptor for hipblasLtMatrixTransform() operation details
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatrixTransformDescOpaque_t {
    pub data: [u64; 8usize],
}
/** \ingroup types_module
  \brief Opaque descriptor for hipblasLtMatrixTransform() operation details

  \details
  The hipblasLtMatrixTransformDesc_t is a pointer to an opaque structure holding the description of a matrix transformation operation.
  \ref hipblasLtMatrixTransformDescCreate():
  To create one instance of the descriptor.
  \ref hipblasLtMatrixTransformDescDestroy():
  To destroy a previously created descriptor and release the resources.*/
pub type hipblasLtMatrixTransformDesc_t = *mut hipblasLtMatrixTransformDescOpaque_t;
/** \ingroup types_module
  \brief Handle to the hipBLASLt library context queue

  \details
  The hipblasLtHandle_t type is a pointer type to an opaque structure holding the hipBLASLt library context. Use the following functions to manipulate this library context:

  \ref hipblasLtCreate():
  To initialize the hipBLASLt library context and return a handle to an opaque structure holding the hipBLASLt library context.
  \ref hipblasLtDestroy():
  To destroy a previously created hipBLASLt library context descriptor and release the resources.*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtHandle_t(pub *mut ::core::ffi::c_void);
/** \ingroup types_module
  \brief Descriptor of the matrix multiplication operation

  \details
  This is a pointer to an opaque structure holding the description of the matrix multiplication operation \ref hipblasLtMatmul().
  Use the following functions to manipulate this descriptor:
  \ref hipblasLtMatmulDescCreate(): To create one instance of the descriptor.
  \ref hipblasLtMatmulDescDestroy(): To destroy a previously created descriptor and release the resources.*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatmulDesc_t(pub *mut hipblasLtMatmulDescOpaque_t);
/** \ingroup types_module
  \brief Descriptor of the matrix layout

  \details
  This is a pointer to an opaque structure holding the description of a matrix layout.
  Use the following functions to manipulate this descriptor:
  \ref hipblasLtMatrixLayoutCreate(): To create one instance of the descriptor.
  \ref hipblasLtMatrixLayoutDestroy(): To destroy a previously created descriptor and release the resources.*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatrixLayout_t(pub *mut hipblasLtMatrixLayoutOpaque_t);
/** \ingroup types_module
  \brief Descriptor of the matrix multiplication preference

  \details
  This is a pointer to an opaque structure holding the description of the preferences for \ref hipblasLtMatmulAlgoGetHeuristic() configuration.
  Use the following functions to manipulate this descriptor:
  \ref hipblasLtMatmulPreferenceCreate(): To create one instance of the descriptor.
  \ref hipblasLtMatmulPreferenceDestroy(): To destroy a previously created descriptor and release the resources.*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatmulPreference_t(pub *mut hipblasLtMatmulPreferenceOpaque_t);
/** \ingroup types_module
  \struct hipblasLtMatmulAlgo_t
  \brief Description of the matrix multiplication algorithm

  \details
  This is an opaque structure holding the description of the matrix multiplication algorithm.
  This structure can be trivially serialized and later restored for use with the same version of hipBLASLt library to save on selecting the right configuration again.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct _hipblasLtMatmulAlgo_t {
    pub data: [u8; 16usize],
    pub max_workspace_bytes: usize,
}
/** \ingroup types_module
  \struct hipblasLtMatmulAlgo_t
  \brief Description of the matrix multiplication algorithm

  \details
  This is an opaque structure holding the description of the matrix multiplication algorithm.
  This structure can be trivially serialized and later restored for use with the same version of hipBLASLt library to save on selecting the right configuration again.*/
pub type hipblasLtMatmulAlgo_t = _hipblasLtMatmulAlgo_t;
/** \ingroup types_module
  \struct hipblasLtMatmulHeuristicResult_t
  \brief Description of the matrix multiplication algorithm

  \details
  This is a descriptor that holds the configured matrix multiplication algorithm descriptor and its runtime properties.
  This structure can be trivially serialized and later restored for use with the same version of hipBLASLt library to save on selecting the right configuration again.
  @param algo \ref hipblasLtMatmulAlgo_t struct
  @param workspaceSize Actual size of workspace memory required
  @param state Result status. Other fields are valid only if, after call to hipblasLtMatmulAlgoGetHeuristic(), this member is set to HIPBLAS_STATUS_SUCCESS*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct _hipblasLtMatmulHeuristicResult_t {
    ///<Algo struct
    pub algo: hipblasLtMatmulAlgo_t,
    ///<Actual size of workspace memory required.
    pub workspaceSize: usize,
    ///<Result status. Other fields are valid only if, after call to hipblasLtMatmulAlgoGetHeuristic(), this member is set to HIPBLAS_STATUS_SUCCESS..
    pub state: hipblasStatus_t,
    ///<Waves count is a device utilization metric. A wavesCount value of 1.0f suggests that when the kernel is launched it will fully occupy the GPU.
    pub wavesCount: f32,
    ///<Reserved.
    pub reserved: [::core::ffi::c_int; 4usize],
}
/** \ingroup types_module
  \struct hipblasLtMatmulHeuristicResult_t
  \brief Description of the matrix multiplication algorithm

  \details
  This is a descriptor that holds the configured matrix multiplication algorithm descriptor and its runtime properties.
  This structure can be trivially serialized and later restored for use with the same version of hipBLASLt library to save on selecting the right configuration again.
  @param algo \ref hipblasLtMatmulAlgo_t struct
  @param workspaceSize Actual size of workspace memory required
  @param state Result status. Other fields are valid only if, after call to hipblasLtMatmulAlgoGetHeuristic(), this member is set to HIPBLAS_STATUS_SUCCESS*/
pub type hipblasLtMatmulHeuristicResult_t = _hipblasLtMatmulHeuristicResult_t;
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipblasLtGetVersion(
        handle: hipblasLtHandle_t,
        version: *mut ::core::ffi::c_int,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipblasLtGetGitRevision(
        handle: hipblasLtHandle_t,
        rev: *mut ::core::ffi::c_char,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipblasLtGetArchName(
        archName: *mut *mut ::core::ffi::c_char,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Create a hipblaslt handle

  \details
  This function initializes the hipBLASLt library and creates a handle to an
 opaque structure holding the hipBLASLt library context. It allocates light
 hardware resources on the host and device, and must be called prior to making
 any other hipBLASLt library calls. The hipBLASLt library context is tied to
 the current ROCm device. To use the library on multiple devices, one
 hipBLASLt handle should be created for each device.

  @param[out]
  handle  Pointer to the allocated hipBLASLt handle for the created hipBLASLt
 context.

  \retval HIPBLAS_STATUS_SUCCESS The allocation completed successfully.
  \retval HIPBLAS_STATUS_INVALID_VALUE \p handle == NULL.*/
    pub fn hipblasLtCreate(handle: *mut hipblasLtHandle_t) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Destory a hipblaslt handle

  \details
  This function releases hardware resources used by the hipBLASLt library.
  This function is usually the last call with a particular handle to the
 hipBLASLt library. Because hipblasLtCreate() allocates some internal
 resources and the release of those resources by calling hipblasLtDestroy()
 will implicitly call hipDeviceSynchronize(), it is recommended to minimize
 the number of hipblasLtCreate()/hipblasLtDestroy() occurrences.

  @param[in]
  handle  Pointer to the hipBLASLt handle to be destroyed.

  \retval HIPBLAS_STATUS_SUCCESS The hipBLASLt context was successfully
 destroyed. \retval HIPBLAS_STATUS_NOT_INITIALIZED The hipBLASLt library was
 not initialized. \retval HIPBLAS_STATUS_INVALID_VALUE \p handle == NULL.*/
    pub fn hipblasLtDestroy(handle: hipblasLtHandle_t) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Create a matrix layout descriptor

  \details
  This function creates a matrix layout descriptor by allocating the memory
 needed to hold its opaque structure.

  @param[out]
  matLayout Pointer to the structure holding the matrix layout descriptor
 created by this function. see \ref hipblasLtMatrixLayout_t .
  @param[in]
  type Enumerant that specifies the data precision for the matrix layout
 descriptor this function creates. See hipDataType.
  @param[in]
  rows Number of rows of the matrix.
  @param[in]
  cols Number of columns of the matrix.
  @param[in]
  ld The leading dimension of the matrix. In column major layout, this is the
 number of elements to jump to reach the next column. Thus ld >= m (number of
 rows).

  \retval HIPBLAS_STATUS_SUCCESS If the descriptor was created successfully.
  \retval HIPBLAS_STATUS_ALLOC_FAILED If the memory could not be allocated.*/
    pub fn hipblasLtMatrixLayoutCreate(
        matLayout: *mut hipblasLtMatrixLayout_t,
        type_: hipDataType,
        rows: u64,
        cols: u64,
        ld: i64,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Destory a matrix layout descriptor

  \details
  This function destroys a previously created matrix layout descriptor object.

  @param[in]
  matLayout Pointer to the structure holding the matrix layout descriptor that
 should be destroyed by this function. see \ref hipblasLtMatrixLayout_t .

  \retval HIPBLAS_STATUS_SUCCESS If the operation was successful.*/
    pub fn hipblasLtMatrixLayoutDestroy(
        matLayout: hipblasLtMatrixLayout_t,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief  Set attribute to a matrix descriptor

  \details
  This function sets the value of the specified attribute belonging to a
 previously created matrix descriptor.

  @param[in]
  matLayout  Pointer to the previously created structure holding the matrix
 mdescriptor queried by this function. See \ref hipblasLtMatrixLayout_t.
  @param[in]
  attr  	The attribute that will be set by this function. See \ref
 hipblasLtMatrixLayoutAttribute_t.
  @param[in]
  buf  The value to which the specified attribute should be set.
  @param[in]
  sizeInBytes Size of buf buffer (in bytes) for verification.

  \retval HIPBLAS_STATUS_SUCCESS If the attribute was set successfully..
  \retval HIPBLAS_STATUS_INVALID_VALUE If \p buf is NULL or \p sizeInBytes
 doesn't match the size of the internal storage for the selected attribute.*/
    pub fn hipblasLtMatrixLayoutSetAttribute(
        matLayout: hipblasLtMatrixLayout_t,
        attr: hipblasLtMatrixLayoutAttribute_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Query attribute from a matrix descriptor

  \details
  This function returns the value of the queried attribute belonging to a
 previously created matrix descriptor.

  @param[in]
  matLayout  Pointer to the previously created structure holding the matrix
 descriptor queried by this function. See \ref hipblasLtMatrixLayout_t.
  @param[in]
  attr  	    The attribute that will be retrieved by this function. See
 \ref hipblasLtMatrixLayoutAttribute_t.
  @param[out]
  buf         Memory address containing the attribute value retrieved by this
 function.
  @param[in]
  sizeInBytes Size of \p buf buffer (in bytes) for verification.
  @param[out]
  sizeWritten Valid only when the return value is HIPBLAS_STATUS_SUCCESS. If
 sizeInBytes is non-zero: then sizeWritten is the number of bytes actually
 written; if sizeInBytes is 0: then sizeWritten is the number of bytes needed
 to write full contents.

  \retval HIPBLAS_STATUS_SUCCESS       If attribute's value was successfully
 written to user memory. \retval HIPBLAS_STATUS_INVALID_VALUE If \p
 sizeInBytes is 0 and \p sizeWritten is NULL, or if \p sizeInBytes is non-zero
 and \p buf is NULL, or \p sizeInBytes doesn't match size of internal storage
 for the selected attribute.*/
    pub fn hipblasLtMatrixLayoutGetAttribute(
        matLayout: hipblasLtMatrixLayout_t,
        attr: hipblasLtMatrixLayoutAttribute_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Create a matrix multiply descriptor

  \details
  This function creates a matrix multiply descriptor by allocating the memory
 needed to hold its opaque structure.

  @param[out]
  matmulDesc  Pointer to the structure holding the matrix multiply descriptor
 created by this function. See \ref hipblasLtMatmulDesc_t .
  @param[in]
  computeType  Enumerant that specifies the data precision for the matrix
 multiply descriptor this function creates. See hipblasComputeType_t .
  @param[in]
  scaleType  Enumerant that specifies the data precision for the matrix
 transform descriptor this function creates. See hipDataType.

  \retval HIPBLAS_STATUS_SUCCESS If the descriptor was created successfully.
  \retval HIPBLAS_STATUS_ALLOC_FAILED If the memory could not be allocated.*/
    pub fn hipblasLtMatmulDescCreate(
        matmulDesc: *mut hipblasLtMatmulDesc_t,
        computeType: hipblasComputeType_t,
        scaleType: hipDataType,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Destory a matrix multiply descriptor

  \details
  This function destroys a previously created matrix multiply descriptor
 object.

  @param[in]
  matmulDesc  Pointer to the structure holding the matrix multiply descriptor
 that should be destroyed by this function. See \ref hipblasLtMatmulDesc_t .

  \retval HIPBLAS_STATUS_SUCCESS If operation was successful.*/
    pub fn hipblasLtMatmulDescDestroy(
        matmulDesc: hipblasLtMatmulDesc_t,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief  Set attribute to a matrix multiply descriptor

  \details
  This function sets the value of the specified attribute belonging to a
 previously created matrix multiply descriptor.

  @param[in]
  matmulDesc  Pointer to the previously created structure holding the matrix
 multiply descriptor queried by this function. See \ref hipblasLtMatmulDesc_t.
  @param[in]
  attr  	The attribute that will be set by this function. See \ref
 hipblasLtMatmulDescAttributes_t.
  @param[in]
  buf  The value to which the specified attribute should be set.
  @param[in]
  sizeInBytes Size of buf buffer (in bytes) for verification.

  \retval HIPBLAS_STATUS_SUCCESS If the attribute was set successfully..
  \retval HIPBLAS_STATUS_INVALID_VALUE If \p buf is NULL or \p sizeInBytes
 doesn't match the size of the internal storage for the selected attribute.*/
    pub fn hipblasLtMatmulDescSetAttribute(
        matmulDesc: hipblasLtMatmulDesc_t,
        attr: hipblasLtMatmulDescAttributes_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Query attribute from a matrix multiply descriptor

  \details
  This function returns the value of the queried attribute belonging to a
 previously created matrix multiply descriptor.

  @param[in]
  matmulDesc  Pointer to the previously created structure holding the matrix
 multiply descriptor queried by this function. See \ref hipblasLtMatmulDesc_t.
  @param[in]
  attr  	    The attribute that will be retrieved by this function. See
 \ref hipblasLtMatmulDescAttributes_t.
  @param[out]
  buf         Memory address containing the attribute value retrieved by this
 function.
  @param[in]
  sizeInBytes Size of \p buf buffer (in bytes) for verification.
  @param[out]
  sizeWritten Valid only when the return value is HIPBLAS_STATUS_SUCCESS. If
 sizeInBytes is non-zero: then sizeWritten is the number of bytes actually
 written; if sizeInBytes is 0: then sizeWritten is the number of bytes needed
 to write full contents.

  \retval HIPBLAS_STATUS_SUCCESS       If attribute's value was successfully
 written to user memory. \retval HIPBLAS_STATUS_INVALID_VALUE If \p
 sizeInBytes is 0 and \p sizeWritten is NULL, or if \p sizeInBytes is non-zero
 and \p buf is NULL, or \p sizeInBytes doesn't match size of internal storage
 for the selected attribute.*/
    pub fn hipblasLtMatmulDescGetAttribute(
        matmulDesc: hipblasLtMatmulDesc_t,
        attr: hipblasLtMatmulDescAttributes_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Create a preference descriptor

  \details
  This function creates a matrix multiply heuristic search preferences
 descriptor by allocating the memory needed to hold its opaque structure.

  @param[out]
  pref  Pointer to the structure holding the matrix multiply preferences
 descriptor created by this function. see \ref hipblasLtMatmulPreference_t .

  \retval HIPBLAS_STATUS_SUCCESS         If the descriptor was created
 successfully. \retval HIPBLAS_STATUS_ALLOC_FAILED    If memory could not be
 allocated.*/
    pub fn hipblasLtMatmulPreferenceCreate(
        pref: *mut hipblasLtMatmulPreference_t,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Destory a preferences descriptor

  \details
  This function destroys a previously created matrix multiply preferences
 descriptor object.

  @param[in]
  pref  Pointer to the structure holding the matrix multiply preferences
 descriptor that should be destroyed by this function. See \ref
 hipblasLtMatmulPreference_t .

  \retval HIPBLAS_STATUS_SUCCESS If operation was successful.*/
    pub fn hipblasLtMatmulPreferenceDestroy(
        pref: hipblasLtMatmulPreference_t,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Set attribute to a preference descriptor

  \details
  This function sets the value of the specified attribute belonging to a
 previously created matrix multiply preferences descriptor.

  @param[in]
  pref        Pointer to the previously created structure holding the matrix
 multiply preferences descriptor queried by this function. See \ref
 hipblasLtMatmulPreference_t
  @param[in]
  attr  	    The attribute that will be set by this function. See \ref
 hipblasLtMatmulPreferenceAttributes_t.
  @param[in]
  buf         The value to which the specified attribute should be set.
  @param[in]
  sizeInBytes Size of \p buf buffer (in bytes) for verification.

  \retval HIPBLAS_STATUS_SUCCESS If the attribute was set successfully..
  \retval HIPBLAS_STATUS_INVALID_VALUE If \p buf is NULL or \p sizeInBytes
 doesn't match the size of the internal storage for the selected attribute.*/
    pub fn hipblasLtMatmulPreferenceSetAttribute(
        pref: hipblasLtMatmulPreference_t,
        attr: hipblasLtMatmulPreferenceAttributes_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Query attribute from a preference descriptor

  \details
  This function returns the value of the queried attribute belonging to a
 previously created matrix multiply heuristic search preferences descriptor.

  @param[in]
  pref        Pointer to the previously created structure holding the matrix
 multiply heuristic search preferences descriptor queried by this function.
 See \ref hipblasLtMatmulPreference_t.
  @param[in]
  attr  	    The attribute that will be retrieved by this function. See
 \ref hipblasLtMatmulPreferenceAttributes_t.
  @param[out]
  buf         Memory address containing the attribute value retrieved by this
 function.
  @param[in]
  sizeInBytes Size of \p buf buffer (in bytes) for verification.
  @param[out]
  sizeWritten Valid only when the return value is HIPBLAS_STATUS_SUCCESS. If
 sizeInBytes is non-zero: then sizeWritten is the number of bytes actually
 written; if sizeInBytes is 0: then sizeWritten is the number of bytes needed
 to write full contents.

  \retval HIPBLAS_STATUS_SUCCESS       If attribute's value was successfully
 written to user memory. \retval HIPBLAS_STATUS_INVALID_VALUE If \p
 sizeInBytes is 0 and \p sizeWritten is NULL, or if \p sizeInBytes is non-zero
 and \p buf is NULL, or \p sizeInBytes doesn't match size of internal storage
 for the selected attribute.*/
    pub fn hipblasLtMatmulPreferenceGetAttribute(
        pref: hipblasLtMatmulPreference_t,
        attr: hipblasLtMatmulPreferenceAttributes_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Retrieve the possible algorithms

  \details
  This function retrieves the possible algorithms for the matrix multiply
 operation hipblasLtMatmul() function with the given input matrices A, B and
 C, and the output matrix D. The output is placed in heuristicResultsArray[]
 in the order of increasing estimated compute time. Note that the wall duration
 increases if the requestedAlgoCount increases.

  @param[in]
  handle                  Pointer to the allocated hipBLASLt handle for the
 hipBLASLt context. See \ref hipblasLtHandle_t .
  @param[in]
  matmulDesc              Handle to a previously created matrix multiplication
 descriptor of type \ref hipblasLtMatmulDesc_t .
  @param[in]
  Adesc,Bdesc,Cdesc,Ddesc Handles to the previously created matrix layout
 descriptors of the type \ref hipblasLtMatrixLayout_t .
  @param[in]
  pref                    Pointer to the structure holding the heuristic
 search preferences descriptor. See \ref hipblasLtMatmulPreference_t .
  @param[in]
  requestedAlgoCount      Size of the \p heuristicResultsArray (in elements).
 This is the requested maximum number of algorithms to return.
  @param[out]
  heuristicResultsArray[] Array containing the algorithm heuristics and
 associated runtime characteristics, returned by this function, in the order
 of increasing estimated compute time.
  @param[out]
  returnAlgoCount         Number of algorithms returned by this function. This
 is the number of \p heuristicResultsArray elements written.

  \retval HIPBLAS_STATUS_SUCCESS           If query was successful. Inspect
 heuristicResultsArray[0 to (returnAlgoCount -1)].state for the status of the
 results. \retval HIPBLAS_STATUS_NOT_SUPPORTED     If no heuristic function
 available for current configuration. \retval HIPBLAS_STATUS_INVALID_VALUE If
 \p requestedAlgoCount is less or equal to zero.*/
    pub fn hipblasLtMatmulAlgoGetHeuristic(
        handle: hipblasLtHandle_t,
        matmulDesc: hipblasLtMatmulDesc_t,
        Adesc: hipblasLtMatrixLayout_t,
        Bdesc: hipblasLtMatrixLayout_t,
        Cdesc: hipblasLtMatrixLayout_t,
        Ddesc: hipblasLtMatrixLayout_t,
        pref: hipblasLtMatmulPreference_t,
        requestedAlgoCount: ::core::ffi::c_int,
        heuristicResultsArray: *mut hipblasLtMatmulHeuristicResult_t,
        returnAlgoCount: *mut ::core::ffi::c_int,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Retrieve the possible algorithms

  \details
  This function computes the matrix multiplication of matrices A and B to
 produce the output matrix D, according to the following operation: \p D = \p
 alpha*( \p A *\p B) + \p beta*( \p C ), where \p A, \p B, and \p C are input
 matrices, and \p alpha and \p beta are input scalars. Note: This function
 supports both in-place matrix multiplication (C == D and Cdesc == Ddesc) and
 out-of-place matrix multiplication (C != D, both matrices must have the same
 data type, number of rows, number of columns, batch size, and memory order).
 In the out-of-place case, the leading dimension of C can be different from
 the leading dimension of D. Specifically the leading dimension of C can be 0
 to achieve row or column broadcast. If Cdesc is omitted, this function
 assumes it to be equal to Ddesc.

  @param[in]
  handle                  Pointer to the allocated hipBLASLt handle for the
 hipBLASLt context. See \ref hipblasLtHandle_t .
  @param[in]
  matmulDesc              Handle to a previously created matrix multiplication
 descriptor of type \ref hipblasLtMatmulDesc_t .
  @param[in]
  alpha,beta              Pointers to the scalars used in the multiplication.
  @param[in]
  Adesc,Bdesc,Cdesc,Ddesc Handles to the previously created matrix layout
 descriptors of the type \ref hipblasLtMatrixLayout_t .
  @param[in]
  A,B,C                   Pointers to the GPU memory associated with the
 corresponding descriptors \p Adesc, \p Bdesc and \p Cdesc .
  @param[out]
  D                       Pointer to the GPU memory associated with the
 descriptor \p Ddesc .
  @param[in]
  algo                    Handle for matrix multiplication algorithm to be
 used. See \ref hipblasLtMatmulAlgo_t . When NULL, an implicit heuristics query
 with default search preferences will be performed to determine actual
 algorithm to use.
  @param[in]
  workspace               Pointer to the workspace buffer allocated in the GPU
 memory. Pointer must be 16B aligned (that is, lowest 4 bits of address must
 be 0).
  @param[in]
  workspaceSizeInBytes    Size of the workspace.
  @param[in]
  stream                  The HIP stream where all the GPU work will be
 submitted.

  \retval HIPBLAS_STATUS_SUCCESS           If the operation completed
 successfully. \retval HIPBLAS_STATUS_EXECUTION_FAILED  If HIP reported an
 execution error from the device. \retval HIPBLAS_STATUS_ARCH_MISMATCH     If
 the configured operation cannot be run using the selected device. \retval
 HIPBLAS_STATUS_NOT_SUPPORTED     If the current implementation on the
 selected device doesn't support the configured operation. \retval
 HIPBLAS_STATUS_INVALID_VALUE     If the parameters are unexpectedly NULL, in
 conflict or in an impossible configuration. For example, when
 workspaceSizeInBytes is less than workspace required by the configured algo.
  \retval HIBLAS_STATUS_NOT_INITIALIZED    If hipBLASLt handle has not been
 initialized.*/
    pub fn hipblasLtMatmul(
        handle: hipblasLtHandle_t,
        matmulDesc: hipblasLtMatmulDesc_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        Adesc: hipblasLtMatrixLayout_t,
        B: *const ::core::ffi::c_void,
        Bdesc: hipblasLtMatrixLayout_t,
        beta: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        Cdesc: hipblasLtMatrixLayout_t,
        D: *mut ::core::ffi::c_void,
        Ddesc: hipblasLtMatrixLayout_t,
        algo: *const hipblasLtMatmulAlgo_t,
        workspace: *mut ::core::ffi::c_void,
        workspaceSizeInBytes: usize,
        stream: hip_runtime_sys::hipStream_t,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** Create new matrix transform operation descriptor.

 \retval     HIPBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated
 \retval     HIPBLAS_STATUS_SUCCESS       if desciptor was created successfully*/
    pub fn hipblasLtMatrixTransformDescCreate(
        transformDesc: *mut hipblasLtMatrixTransformDesc_t,
        scaleType: hipDataType,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** Destroy matrix transform operation descriptor.

 \retval     HIPBLAS_STATUS_SUCCESS  if operation was successful*/
    pub fn hipblasLtMatrixTransformDescDestroy(
        transformDesc: hipblasLtMatrixTransformDesc_t,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** Set matrix transform operation descriptor attribute.

 \param[in]  transformDesc  The descriptor
 \param[in]  attr           The attribute
 \param[in]  buf            memory address containing the new value
 \param[in]  sizeInBytes    size of buf buffer for verification (in bytes)

 \retval     HIPBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     HIPBLAS_STATUS_SUCCESS        if attribute was set successfully*/
    pub fn hipblasLtMatrixTransformDescSetAttribute(
        transformDesc: hipblasLtMatrixTransformDesc_t,
        attr: hipblasLtMatrixTransformDescAttributes_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Matrix transform operation getter
  \details Get matrix transform operation descriptor attribute.

 @param[in]  transformDesc  The descriptor
 @param[in]  attr           The attribute
 @param[out] buf            memory address containing the new value
 @param[in]  sizeInBytes    size of buf buffer for verification (in bytes)
 @param[out] sizeWritten    only valid when return value is HIPBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number
 of bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents

 \retval HIPBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero
                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval HIPBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory*/
    pub fn hipblasLtMatrixTransformDescGetAttribute(
        transformDesc: hipblasLtMatrixTransformDesc_t,
        attr: hipblasLtMatrixTransformDescAttributes_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> hipblasStatus_t;
}
#[cfg_attr(windows, link(name = "hipblaslt", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup library_module
  \brief Matrix layout conversion helper
  \details
   Matrix layout conversion helper (C = alpha * op(A) + beta * op(B)),
 can be used to change memory order of data or to scale and shift the values.
 @param[in]  lightHandle   Pointer to the allocated hipBLASLt handle for the
 hipBLASLt context. See \ref hipblasLtHandle_t .
 @param[in]  transformDesc Pointer to allocated matrix transform descriptor.
 @param[in]  alpha         Pointer to scalar alpha, either pointer to host or device address.
 @param[in]  A             Pointer to matrix A, must be pointer to device address.
 @param[in]  Adesc         Pointer to layout for input matrix A.
 @param[in]  beta          Pointer to scalar beta, either pointer to host or device address.
 @param[in]  B             Pointer to layout for matrix B, must be pointer to device address
 @param[in]  Bdesc         Pointer to layout for inputmatrix B.
 @param[in]  C             Pointer to matrix C, must be pointer to device address
 @param[out] Cdesc         Pointer to layout for output matrix C.
 @param[in] stream         The HIP stream where all the GPU work will be submitted.

 \retval HIPBLAS_STATUS_NOT_INITIALIZED   if hipBLASLt handle has not been initialized
 \retval HIPBLAS_STATUS_INVALID_VALUE     if parameters are in conflict or in an impossible configuration; e.g.
                                              when A is not NULL, but Adesc is NULL
 \retval HIPBLAS_STATUS_NOT_SUPPORTED     if current implementation on selected device doesn't support configured
                                              operation
 \retval HIPBLAS_STATUS_ARCH_MISMATCH     if configured operation cannot be run using selected device
 \retval HIPBLAS_STATUS_EXECUTION_FAILED  if HIP reported execution error from the device
 \retval HIPBLAS_STATUS_SUCCESS           if the operation completed successfully*/
    pub fn hipblasLtMatrixTransform(
        lightHandle: hipblasLtHandle_t,
        transformDesc: hipblasLtMatrixTransformDesc_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        Adesc: hipblasLtMatrixLayout_t,
        beta: *const ::core::ffi::c_void,
        B: *const ::core::ffi::c_void,
        Bdesc: hipblasLtMatrixLayout_t,
        C: *mut ::core::ffi::c_void,
        Cdesc: hipblasLtMatrixLayout_t,
        stream: hip_runtime_sys::hipStream_t,
    ) -> hipblasStatus_t;
}
impl hipblasLtError {
    pub const r#NOT_INITIALIZED: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const r#ALLOC_FAILED: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const r#INVALID_VALUE: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const r#MAPPING_ERROR: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const r#EXECUTION_FAILED: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const r#INTERNAL_ERROR: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const r#NOT_SUPPORTED: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const r#ARCH_MISMATCH: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const r#HANDLE_IS_NULLPTR: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(9)
    });
    pub const r#INVALID_ENUM: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(10)
    });
    pub const r#UNKNOWN: hipblasLtError = hipblasLtError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(11)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct hipblasLtError(pub ::core::num::NonZeroU32);
pub trait hipblasStatus_tConsts {
    const SUCCESS: hipblasStatus_t = hipblasStatus_t::Ok(());
    const ERROR_NOT_INITIALIZED: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#NOT_INITIALIZED,
    );
    const ERROR_ALLOC_FAILED: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#ALLOC_FAILED,
    );
    const ERROR_INVALID_VALUE: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#INVALID_VALUE,
    );
    const ERROR_MAPPING_ERROR: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#MAPPING_ERROR,
    );
    const ERROR_EXECUTION_FAILED: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#EXECUTION_FAILED,
    );
    const ERROR_INTERNAL_ERROR: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#INTERNAL_ERROR,
    );
    const ERROR_NOT_SUPPORTED: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#NOT_SUPPORTED,
    );
    const ERROR_ARCH_MISMATCH: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#ARCH_MISMATCH,
    );
    const ERROR_HANDLE_IS_NULLPTR: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#HANDLE_IS_NULLPTR,
    );
    const ERROR_INVALID_ENUM: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#INVALID_ENUM,
    );
    const ERROR_UNKNOWN: hipblasStatus_t = hipblasStatus_t::Err(
        hipblasLtError::r#UNKNOWN,
    );
}
impl hipblasStatus_tConsts for hipblasStatus_t {}
#[must_use]
pub type hipblasStatus_t = ::core::result::Result<(), hipblasLtError>;
const _: fn() = || {
    let _ = std::mem::transmute::<hipblasStatus_t, u32>;
};
unsafe impl Send for hipblasLtHandle_t {}
unsafe impl Sync for hipblasLtHandle_t {}
unsafe impl Send for hipblasLtMatmulDesc_t {}
unsafe impl Sync for hipblasLtMatmulDesc_t {}
unsafe impl Send for hipblasLtMatmulPreference_t {}
unsafe impl Sync for hipblasLtMatmulPreference_t {}
unsafe impl Send for hipblasLtMatrixLayout_t {}
unsafe impl Sync for hipblasLtMatrixLayout_t {}
