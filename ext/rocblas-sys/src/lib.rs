// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
/// \brief Struct to represent a 16 bit Brain floating-point number.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_bfloat16 {
    pub data: u16,
}
/// \brief Struct to represent a 8 bit floating-point number.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_f8 {
    pub data: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_bf8 {
    pub data: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _rocblas_handle {
    _unused: [u8; 0],
}
/** \brief rocblas_handle is a structure holding the rocblas library context.
 It must be initialized using rocblas_create_handle(),
 and the returned handle must be passed
 to all subsequent library function calls.
 It should be destroyed at the end using rocblas_destroy_handle().*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_handle(pub *mut _rocblas_handle);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipStream_t {
    _unused: [u8; 0],
}
/// \brief Forward declaration of hipStream_t
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipStream_t(pub *mut ihipStream_t);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipEvent_t {
    _unused: [u8; 0],
}
/// \brief Forward declaration of hipEvent_t
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipEvent_t(pub *mut ihipEvent_t);
/// \brief Opaque base class for device memory allocation
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rocblas_device_malloc_base {
    _unused: [u8; 0],
}
pub type rocblas_int = i32;
/// \brief Stride between matrices or vectors in strided_batched functions
pub type rocblas_stride = i64;
/// \brief Single precision floating point type
pub type rocblas_float = f32;
/// \brief Double precision floating point type
pub type rocblas_double = f64;
/// \brief Structure definition for rocblas_half
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_half {
    pub data: u16,
}
/// \brief Struct to represent a complex number with single precision real and imaginary parts.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rocblas_float_complex {
    pub x: f32,
    pub y: f32,
}
/// \brief Struct to represent a complex number with double precision real and imaginary parts.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rocblas_double_complex {
    pub x: f64,
    pub y: f64,
}
impl rocblas_operation_ {
    ///< Operate with the matrix.
    pub const rocblas_operation_none: rocblas_operation_ = rocblas_operation_(111);
}
impl rocblas_operation_ {
    ///< Operate with the transpose of the matrix.
    pub const rocblas_operation_transpose: rocblas_operation_ = rocblas_operation_(112);
}
impl rocblas_operation_ {
    pub const rocblas_operation_conjugate_transpose: rocblas_operation_ = rocblas_operation_(
        113,
    );
}
#[repr(transparent)]
/// \brief Used to specify whether the matrix is to be transposed or not.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_operation_(pub ::core::ffi::c_uint);
/// \brief Used to specify whether the matrix is to be transposed or not.
pub use self::rocblas_operation_ as rocblas_operation;
impl rocblas_fill_ {
    ///< Upper triangle.
    pub const rocblas_fill_upper: rocblas_fill_ = rocblas_fill_(121);
}
impl rocblas_fill_ {
    ///< Lower triangle.
    pub const rocblas_fill_lower: rocblas_fill_ = rocblas_fill_(122);
}
impl rocblas_fill_ {
    pub const rocblas_fill_full: rocblas_fill_ = rocblas_fill_(123);
}
#[repr(transparent)]
/** \brief Used by the Hermitian, symmetric and triangular matrix
 routines to specify whether the upper, or lower triangle is being referenced.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_fill_(pub ::core::ffi::c_uint);
/** \brief Used by the Hermitian, symmetric and triangular matrix
 routines to specify whether the upper, or lower triangle is being referenced.*/
pub use self::rocblas_fill_ as rocblas_fill;
impl rocblas_diagonal_ {
    ///< Non-unit triangular.
    pub const rocblas_diagonal_non_unit: rocblas_diagonal_ = rocblas_diagonal_(131);
}
impl rocblas_diagonal_ {
    ///< Unit triangular.
    pub const rocblas_diagonal_unit: rocblas_diagonal_ = rocblas_diagonal_(132);
}
#[repr(transparent)]
/** \brief It is used by the triangular matrix routines to specify whether the
 matrix is unit triangular.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_diagonal_(pub ::core::ffi::c_uint);
/** \brief It is used by the triangular matrix routines to specify whether the
 matrix is unit triangular.*/
pub use self::rocblas_diagonal_ as rocblas_diagonal;
impl rocblas_side_ {
    /**< Multiply general matrix by symmetric,
Hermitian, or triangular matrix on the left.*/
    pub const rocblas_side_left: rocblas_side_ = rocblas_side_(141);
}
impl rocblas_side_ {
    /**< Multiply general matrix by symmetric,
Hermitian, or triangular matrix on the right.*/
    pub const rocblas_side_right: rocblas_side_ = rocblas_side_(142);
}
impl rocblas_side_ {
    pub const rocblas_side_both: rocblas_side_ = rocblas_side_(143);
}
#[repr(transparent)]
/// \brief Indicates the side matrix A is located relative to matrix B during multiplication.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_side_(pub ::core::ffi::c_uint);
/// \brief Indicates the side matrix A is located relative to matrix B during multiplication.
pub use self::rocblas_side_ as rocblas_side;
impl rocblas_datatype_ {
    ///< 16-bit floating point, real
    pub const rocblas_datatype_f16_r: rocblas_datatype_ = rocblas_datatype_(150);
}
impl rocblas_datatype_ {
    ///< 32-bit floating point, real
    pub const rocblas_datatype_f32_r: rocblas_datatype_ = rocblas_datatype_(151);
}
impl rocblas_datatype_ {
    ///< 64-bit floating point, real
    pub const rocblas_datatype_f64_r: rocblas_datatype_ = rocblas_datatype_(152);
}
impl rocblas_datatype_ {
    ///< 16-bit floating point, complex
    pub const rocblas_datatype_f16_c: rocblas_datatype_ = rocblas_datatype_(153);
}
impl rocblas_datatype_ {
    ///< 32-bit floating point, complex
    pub const rocblas_datatype_f32_c: rocblas_datatype_ = rocblas_datatype_(154);
}
impl rocblas_datatype_ {
    ///< 64-bit floating point, complex
    pub const rocblas_datatype_f64_c: rocblas_datatype_ = rocblas_datatype_(155);
}
impl rocblas_datatype_ {
    ///<  8-bit signed integer, real
    pub const rocblas_datatype_i8_r: rocblas_datatype_ = rocblas_datatype_(160);
}
impl rocblas_datatype_ {
    ///<  8-bit unsigned integer, real
    pub const rocblas_datatype_u8_r: rocblas_datatype_ = rocblas_datatype_(161);
}
impl rocblas_datatype_ {
    ///< 32-bit signed integer, real
    pub const rocblas_datatype_i32_r: rocblas_datatype_ = rocblas_datatype_(162);
}
impl rocblas_datatype_ {
    ///< 32-bit unsigned integer, real
    pub const rocblas_datatype_u32_r: rocblas_datatype_ = rocblas_datatype_(163);
}
impl rocblas_datatype_ {
    ///<  8-bit signed integer, complex
    pub const rocblas_datatype_i8_c: rocblas_datatype_ = rocblas_datatype_(164);
}
impl rocblas_datatype_ {
    ///<  8-bit unsigned integer, complex
    pub const rocblas_datatype_u8_c: rocblas_datatype_ = rocblas_datatype_(165);
}
impl rocblas_datatype_ {
    ///< 32-bit signed integer, complex
    pub const rocblas_datatype_i32_c: rocblas_datatype_ = rocblas_datatype_(166);
}
impl rocblas_datatype_ {
    ///< 32-bit unsigned integer, complex
    pub const rocblas_datatype_u32_c: rocblas_datatype_ = rocblas_datatype_(167);
}
impl rocblas_datatype_ {
    ///< 16-bit bfloat, real
    pub const rocblas_datatype_bf16_r: rocblas_datatype_ = rocblas_datatype_(168);
}
impl rocblas_datatype_ {
    ///< 16-bit bfloat, complex
    pub const rocblas_datatype_bf16_c: rocblas_datatype_ = rocblas_datatype_(169);
}
impl rocblas_datatype_ {
    ///< 8 bit floating point, real
    pub const rocblas_datatype_f8_r: rocblas_datatype_ = rocblas_datatype_(170);
}
impl rocblas_datatype_ {
    ///< 8 bit bfloat, real
    pub const rocblas_datatype_bf8_r: rocblas_datatype_ = rocblas_datatype_(171);
}
impl rocblas_datatype_ {
    ///< Invalid datatype value, do not use
    pub const rocblas_datatype_invalid: rocblas_datatype_ = rocblas_datatype_(255);
}
#[repr(transparent)]
/// \brief Indicates the precision width of data stored in a blas type.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_datatype_(pub ::core::ffi::c_uint);
/// \brief Indicates the precision width of data stored in a blas type.
pub use self::rocblas_datatype_ as rocblas_datatype;
impl rocblas_computetype_ {
    pub const rocblas_compute_type_f32: rocblas_computetype_ = rocblas_computetype_(300);
}
impl rocblas_computetype_ {
    pub const rocblas_compute_type_f8_f8_f32: rocblas_computetype_ = rocblas_computetype_(
        301,
    );
}
impl rocblas_computetype_ {
    pub const rocblas_compute_type_f8_bf8_f32: rocblas_computetype_ = rocblas_computetype_(
        302,
    );
}
impl rocblas_computetype_ {
    pub const rocblas_compute_type_bf8_f8_f32: rocblas_computetype_ = rocblas_computetype_(
        303,
    );
}
impl rocblas_computetype_ {
    pub const rocblas_compute_type_bf8_bf8_f32: rocblas_computetype_ = rocblas_computetype_(
        304,
    );
}
impl rocblas_computetype_ {
    ///< Invalid datatype value, do not use
    pub const rocblas_compute_type_invalid: rocblas_computetype_ = rocblas_computetype_(
        455,
    );
}
#[repr(transparent)]
/// \brief Indicates the compute precision mode.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_computetype_(pub ::core::ffi::c_uint);
/// \brief Indicates the compute precision mode.
pub use self::rocblas_computetype_ as rocblas_computetype;
///   @brief rocblas status codes definition
pub type rocblas_status_ = ::core::ffi::c_uint;
impl rocblas_pointer_mode_ {
    /// \brief Scalar values affected by this variable are located on the host.
    pub const rocblas_pointer_mode_host: rocblas_pointer_mode_ = rocblas_pointer_mode_(
        0,
    );
}
impl rocblas_pointer_mode_ {
    /// \brief Scalar values affected by this variable are located on the device.
    pub const rocblas_pointer_mode_device: rocblas_pointer_mode_ = rocblas_pointer_mode_(
        1,
    );
}
#[repr(transparent)]
/** \brief Indicates if scalar pointers are on host or device. This is used for
    scalars alpha and beta and for scalar function return values.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_pointer_mode_(pub ::core::ffi::c_uint);
/** \brief Indicates if scalar pointers are on host or device. This is used for
    scalars alpha and beta and for scalar function return values.*/
pub use self::rocblas_pointer_mode_ as rocblas_pointer_mode;
impl rocblas_atomics_mode_ {
    /// \brief Algorithms will refrain from atomics where applicable
    pub const rocblas_atomics_not_allowed: rocblas_atomics_mode_ = rocblas_atomics_mode_(
        0,
    );
}
impl rocblas_atomics_mode_ {
    /// \brief Algorithms will take advantage of atomics where applicable
    pub const rocblas_atomics_allowed: rocblas_atomics_mode_ = rocblas_atomics_mode_(1);
}
#[repr(transparent)]
/** \brief Indicates if atomics operations are allowed. Not allowing atomic operations
    may generally improve determinism and repeatability of results at a cost of performance.
    Defaults to rocblas_atomics_allowed.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_atomics_mode_(pub ::core::ffi::c_uint);
/** \brief Indicates if atomics operations are allowed. Not allowing atomic operations
    may generally improve determinism and repeatability of results at a cost of performance.
    Defaults to rocblas_atomics_allowed.*/
pub use self::rocblas_atomics_mode_ as rocblas_atomics_mode;
impl rocblas_performance_metric_ {
    /// \brief Use Tensile's default performance metric for solution selection
    pub const rocblas_default_performance_metric: rocblas_performance_metric_ = rocblas_performance_metric_(
        0,
    );
}
impl rocblas_performance_metric_ {
    /// \brief Select the solution with the highest GFlops across all compute units
    pub const rocblas_device_efficiency_performance_metric: rocblas_performance_metric_ = rocblas_performance_metric_(
        1,
    );
}
impl rocblas_performance_metric_ {
    /** \brief Select the solution with the highest GFlops per compute unit it uses. This
 may be useful when running multiple small gemm problems simultaneously*/
    pub const rocblas_cu_efficiency_performance_metric: rocblas_performance_metric_ = rocblas_performance_metric_(
        2,
    );
}
#[repr(transparent)]
/** \brief Indicates which performance metric Tensile uses when selecting the optimal
    solution for gemm problems.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_performance_metric_(pub ::core::ffi::c_uint);
/** \brief Indicates which performance metric Tensile uses when selecting the optimal
    solution for gemm problems.*/
pub use self::rocblas_performance_metric_ as rocblas_performance_metric;
impl rocblas_layer_mode_ {
    /// \brief No logging will take place.
    pub const rocblas_layer_mode_none: rocblas_layer_mode_ = rocblas_layer_mode_(0);
}
impl rocblas_layer_mode_ {
    /// \brief A line containing the function name and value of arguments passed will be printed with each rocBLAS function call.
    pub const rocblas_layer_mode_log_trace: rocblas_layer_mode_ = rocblas_layer_mode_(1);
}
impl rocblas_layer_mode_ {
    /// \brief Outputs a line each time a rocBLAS function is called, this line can be used with rocblas-bench to make the same call again.
    pub const rocblas_layer_mode_log_bench: rocblas_layer_mode_ = rocblas_layer_mode_(2);
}
impl rocblas_layer_mode_ {
    /// \brief Outputs a YAML description of each rocBLAS function called, along with its arguments and number of times it was called.
    pub const rocblas_layer_mode_log_profile: rocblas_layer_mode_ = rocblas_layer_mode_(
        4,
    );
}
#[repr(transparent)]
/// \brief Indicates if layer is active with bitmask
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_layer_mode_(pub ::core::ffi::c_uint);
/// \brief Indicates if layer is active with bitmask
pub use self::rocblas_layer_mode_ as rocblas_layer_mode;
impl rocblas_gemm_algo_ {
    pub const rocblas_gemm_algo_standard: rocblas_gemm_algo_ = rocblas_gemm_algo_(0);
}
impl rocblas_gemm_algo_ {
    pub const rocblas_gemm_algo_solution_index: rocblas_gemm_algo_ = rocblas_gemm_algo_(
        1,
    );
}
#[repr(transparent)]
/// \brief Indicates if layer is active with bitmask
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_gemm_algo_(pub ::core::ffi::c_uint);
/// \brief Indicates if layer is active with bitmask
pub use self::rocblas_gemm_algo_ as rocblas_gemm_algo;
impl rocblas_geam_ex_operation_ {
    pub const rocblas_geam_ex_operation_min_plus: rocblas_geam_ex_operation_ = rocblas_geam_ex_operation_(
        0,
    );
}
impl rocblas_geam_ex_operation_ {
    pub const rocblas_geam_ex_operation_plus_min: rocblas_geam_ex_operation_ = rocblas_geam_ex_operation_(
        1,
    );
}
#[repr(transparent)]
/// \brief Which mathematical geam-like operation to perform for geam_ex
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_geam_ex_operation_(pub ::core::ffi::c_uint);
/// \brief Which mathematical geam-like operation to perform for geam_ex
pub use self::rocblas_geam_ex_operation_ as rocblas_geam_ex_operation;
impl rocblas_gemm_flags_ {
    /// \brief Default empty flags
    pub const rocblas_gemm_flags_none: rocblas_gemm_flags_ = rocblas_gemm_flags_(0);
}
impl rocblas_gemm_flags_ {
    #[doc = " \\brief Before ROCm 6.0 rocblas_gemm_flags_pack_int8x4 = 0x1, as has now been removed so is available for future use */\n/*! \\brief Select the gemm problem with the highest efficiency per compute unit used. Useful for running multiple smaller problems\n simultaneously. This takes precedence over the performance metric set in rocblas_handle and currently only works for\n gemm_*_ex problems."]
    pub const rocblas_gemm_flags_use_cu_efficiency: rocblas_gemm_flags_ = rocblas_gemm_flags_(
        2,
    );
}
impl rocblas_gemm_flags_ {
    /** \brief Select an alternate implementation for the MI200 FP16 HPA
 (High Precision Accumulate) GEMM kernel utilizing the BF16 matrix
 instructions with reduced accuracy in cases where computation cannot
 tolerate the FP16 matrix instructions flushing subnormal FP16
 input/output data to zero. See the "MI200 (gfx90a) Considerations"
 section for more details.*/
    pub const rocblas_gemm_flags_fp16_alt_impl: rocblas_gemm_flags_ = rocblas_gemm_flags_(
        4,
    );
}
impl rocblas_gemm_flags_ {
    /** \brief Select an alternate implementation for the MI200 FP16 HPA
 (High Precision Accumulate) GEMM kernel utilizing the BF16 matrix
 instructions with reduced accuracy in cases where computation cannot
 tolerate the FP16 matrix instructions flushing subnormal FP16
 input/output data to zero. See the "MI200 (gfx90a) Considerations"
 section for more details.*/
    pub const rocblas_gemm_flags_check_solution_index: rocblas_gemm_flags_ = rocblas_gemm_flags_(
        8,
    );
}
impl rocblas_gemm_flags_ {
    /** \brief Select an alternate implementation for the MI200 FP16 HPA
 (High Precision Accumulate) GEMM kernel utilizing the BF16 matrix
 instructions with reduced accuracy in cases where computation cannot
 tolerate the FP16 matrix instructions flushing subnormal FP16
 input/output data to zero. See the "MI200 (gfx90a) Considerations"
 section for more details.*/
    pub const rocblas_gemm_flags_fp16_alt_impl_rnz: rocblas_gemm_flags_ = rocblas_gemm_flags_(
        16,
    );
}
impl rocblas_gemm_flags_ {
    /** \brief Select an alternate implementation for the MI200 FP16 HPA
 (High Precision Accumulate) GEMM kernel utilizing the BF16 matrix
 instructions with reduced accuracy in cases where computation cannot
 tolerate the FP16 matrix instructions flushing subnormal FP16
 input/output data to zero. See the "MI200 (gfx90a) Considerations"
 section for more details.*/
    pub const rocblas_gemm_flags_stochastic_rounding: rocblas_gemm_flags_ = rocblas_gemm_flags_(
        32,
    );
}
#[repr(transparent)]
/// \brief Control flags passed into gemm algorithms invoked by Tensile Host
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_gemm_flags_(pub ::core::ffi::c_uint);
/// \brief Control flags passed into gemm algorithms invoked by Tensile Host
pub use self::rocblas_gemm_flags_ as rocblas_gemm_flags;
/// \brief Union for representing scalar values
#[repr(C)]
#[derive(Copy, Clone)]
pub union rocblas_union_u {
    pub h: rocblas_half,
    pub s: f32,
    pub d: f64,
    pub i: i32,
    pub c: rocblas_float_complex,
    pub z: rocblas_double_complex,
}
/// \brief Union for representing scalar values
pub type rocblas_union_t = rocblas_union_u;
impl rocblas_check_numerics_mode_ {
    pub const rocblas_check_numerics_mode_no_check: rocblas_check_numerics_mode_ = rocblas_check_numerics_mode_(
        0,
    );
}
impl rocblas_check_numerics_mode_ {
    pub const rocblas_check_numerics_mode_info: rocblas_check_numerics_mode_ = rocblas_check_numerics_mode_(
        1,
    );
}
impl rocblas_check_numerics_mode_ {
    pub const rocblas_check_numerics_mode_warn: rocblas_check_numerics_mode_ = rocblas_check_numerics_mode_(
        2,
    );
}
impl rocblas_check_numerics_mode_ {
    pub const rocblas_check_numerics_mode_fail: rocblas_check_numerics_mode_ = rocblas_check_numerics_mode_(
        4,
    );
}
impl rocblas_check_numerics_mode_ {
    pub const rocblas_check_numerics_mode_only_nan_inf: rocblas_check_numerics_mode_ = rocblas_check_numerics_mode_(
        8,
    );
}
#[repr(transparent)]
/// \brief Numerical checking for verifying the Input and Output vector/matrix of the rocBLAS functions for a NaN, zero, infinity and denormal value
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_check_numerics_mode_(pub ::core::ffi::c_uint);
/// \brief Numerical checking for verifying the Input and Output vector/matrix of the rocBLAS functions for a NaN, zero, infinity and denormal value
pub use self::rocblas_check_numerics_mode_ as rocblas_check_numerics_mode;
impl rocblas_math_mode_ {
    pub const rocblas_default_math: rocblas_math_mode_ = rocblas_math_mode_(0);
}
impl rocblas_math_mode_ {
    pub const rocblas_xf32_xdl_math_op: rocblas_math_mode_ = rocblas_math_mode_(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocblas_math_mode_(pub ::core::ffi::c_uint);
pub use self::rocblas_math_mode_ as rocblas_math_mode;
extern "C" {
    #[must_use]
    /// \brief Create handle
    pub fn rocblas_create_handle(handle: *mut rocblas_handle) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /// \brief Destroy handle
    pub fn rocblas_destroy_handle(handle: rocblas_handle) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /// \brief Set stream for handle
    pub fn rocblas_set_stream(
        handle: rocblas_handle,
        stream: hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /// \brief Get stream [0] from handle
    pub fn rocblas_get_stream(
        handle: rocblas_handle,
        stream: *mut hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /// \brief Set rocblas_pointer_mode
    pub fn rocblas_set_pointer_mode(
        handle: rocblas_handle,
        pointer_mode: rocblas_pointer_mode,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /// \brief Get rocblas_pointer_mode
    pub fn rocblas_get_pointer_mode(
        handle: rocblas_handle,
        pointer_mode: *mut rocblas_pointer_mode,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief Set rocblas_atomics_mode
  \details
  Some rocBLAS functions may have implementations which use atomic operations to increase performance.
  By using atomic operations, results are not guaranteed to be identical between multiple runs.
  Results will be accurate with or without atomic operations, but if it is required to
  have bit-wise reproducible results, atomic operations should not be used.

  Atomic operations can be turned on or off for a handle by calling rocblas_set_atomics_mode.
  By default, this is set to `rocblas_atomics_allowed`.*/
    pub fn rocblas_set_atomics_mode(
        handle: rocblas_handle,
        atomics_mode: rocblas_atomics_mode,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /// \brief Get rocblas_atomics_mode
    pub fn rocblas_get_atomics_mode(
        handle: rocblas_handle,
        atomics_mode: *mut rocblas_atomics_mode,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /// \brief Set rocblas_math_mode
    pub fn rocblas_set_math_mode(
        handle: rocblas_handle,
        math_mode: rocblas_math_mode,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /// \brief Get rocblas_math_mode
    pub fn rocblas_get_math_mode(
        handle: rocblas_handle,
        math_mode: *mut rocblas_math_mode,
    ) -> rocblas_status;
}
extern "C" {
    /// \brief  Indicates whether the pointer is on the host or device.
    pub fn rocblas_pointer_to_mode(
        ptr: *mut ::core::ffi::c_void,
    ) -> rocblas_pointer_mode;
}
extern "C" {
    #[must_use]
    /** \brief Copy vector from host to device
@param[in]
n           [rocblas_int]
number of elements in the vector
@param[in]
elem_size   [rocblas_int]
number of bytes per element in the matrix
@param[in]
x           pointer to vector on the host
@param[in]
incx        [rocblas_int]
specifies the increment for the elements of the vector
@param[out]
y           pointer to vector on the device
@param[in]
incy        [rocblas_int]
specifies the increment for the elements of the vector*/
    pub fn rocblas_set_vector(
        n: rocblas_int,
        elem_size: rocblas_int,
        x: *const ::core::ffi::c_void,
        incx: rocblas_int,
        y: *mut ::core::ffi::c_void,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_set_vector_64(
        n: i64,
        elem_size: i64,
        x: *const ::core::ffi::c_void,
        incx: i64,
        y: *mut ::core::ffi::c_void,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief Copy vector from device to host
@param[in]
n           [rocblas_int]
number of elements in the vector
@param[in]
elem_size   [rocblas_int]
number of bytes per element in the matrix
@param[in]
x           pointer to vector on the device
@param[in]
incx        [rocblas_int]
specifies the increment for the elements of the vector
@param[out]
y           pointer to vector on the host
@param[in]
incy        [rocblas_int]
specifies the increment for the elements of the vector*/
    pub fn rocblas_get_vector(
        n: rocblas_int,
        elem_size: rocblas_int,
        x: *const ::core::ffi::c_void,
        incx: rocblas_int,
        y: *mut ::core::ffi::c_void,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_get_vector_64(
        n: i64,
        elem_size: i64,
        x: *const ::core::ffi::c_void,
        incx: i64,
        y: *mut ::core::ffi::c_void,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief Copy matrix from host to device
@param[in]
rows        [rocblas_int]
number of rows in matrices
@param[in]
cols        [rocblas_int]
number of columns in matrices
@param[in]
elem_size   [rocblas_int]
number of bytes per element in the matrix
@param[in]
a           pointer to matrix on the host
@param[in]
lda         [rocblas_int]
specifies the leading dimension of A, lda >= rows
@param[out]
b           pointer to matrix on the GPU
@param[in]
ldb         [rocblas_int]
specifies the leading dimension of B, ldb >= rows*/
    pub fn rocblas_set_matrix(
        rows: rocblas_int,
        cols: rocblas_int,
        elem_size: rocblas_int,
        a: *const ::core::ffi::c_void,
        lda: rocblas_int,
        b: *mut ::core::ffi::c_void,
        ldb: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_set_matrix_64(
        rows: i64,
        cols: i64,
        elem_size: i64,
        a: *const ::core::ffi::c_void,
        lda: i64,
        b: *mut ::core::ffi::c_void,
        ldb: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief Copy matrix from device to host
@param[in]
rows        [rocblas_int]
number of rows in matrices
@param[in]
cols        [rocblas_int]
number of columns in matrices
@param[in]
elem_size   [rocblas_int]
number of bytes per element in the matrix
@param[in]
a           pointer to matrix on the GPU
@param[in]
lda         [rocblas_int]
specifies the leading dimension of A, lda >= rows
@param[out]
b           pointer to matrix on the host
@param[in]
ldb         [rocblas_int]
specifies the leading dimension of B, ldb >= rows*/
    pub fn rocblas_get_matrix(
        rows: rocblas_int,
        cols: rocblas_int,
        elem_size: rocblas_int,
        a: *const ::core::ffi::c_void,
        lda: rocblas_int,
        b: *mut ::core::ffi::c_void,
        ldb: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_get_matrix_64(
        rows: i64,
        cols: i64,
        elem_size: i64,
        a: *const ::core::ffi::c_void,
        lda: i64,
        b: *mut ::core::ffi::c_void,
        ldb: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief Asynchronously copy vector from host to device
\details
rocblas_set_vector_async copies a vector from pinned host memory to device memory asynchronously.
Memory on the host must be allocated with hipHostMalloc or the transfer will be synchronous.
@param[in]
n           [rocblas_int]
number of elements in the vector
@param[in]
elem_size   [rocblas_int]
number of bytes per element in the matrix
@param[in]
x           pointer to vector on the host
@param[in]
incx        [rocblas_int]
specifies the increment for the elements of the vector
@param[out]
y           pointer to vector on the device
@param[in]
incy        [rocblas_int]
specifies the increment for the elements of the vector
@param[in]
stream      specifies the stream into which this transfer request is queued*/
    pub fn rocblas_set_vector_async(
        n: rocblas_int,
        elem_size: rocblas_int,
        x: *const ::core::ffi::c_void,
        incx: rocblas_int,
        y: *mut ::core::ffi::c_void,
        incy: rocblas_int,
        stream: hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_set_vector_async_64(
        n: i64,
        elem_size: i64,
        x: *const ::core::ffi::c_void,
        incx: i64,
        y: *mut ::core::ffi::c_void,
        incy: i64,
        stream: hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief Asynchronously copy vector from device to host
\details
rocblas_get_vector_async copies a vector from pinned host memory to device memory asynchronously.
Memory on the host must be allocated with hipHostMalloc or the transfer will be synchronous.
@param[in]
n           [rocblas_int]
number of elements in the vector
@param[in]
elem_size   [rocblas_int]
number of bytes per element in the matrix
@param[in]
x           pointer to vector on the device
@param[in]
incx        [rocblas_int]
specifies the increment for the elements of the vector
@param[out]
y           pointer to vector on the host
@param[in]
incy        [rocblas_int]
specifies the increment for the elements of the vector
@param[in]
stream      specifies the stream into which this transfer request is queued*/
    pub fn rocblas_get_vector_async(
        n: rocblas_int,
        elem_size: rocblas_int,
        x: *const ::core::ffi::c_void,
        incx: rocblas_int,
        y: *mut ::core::ffi::c_void,
        incy: rocblas_int,
        stream: hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_get_vector_async_64(
        n: i64,
        elem_size: i64,
        x: *const ::core::ffi::c_void,
        incx: i64,
        y: *mut ::core::ffi::c_void,
        incy: i64,
        stream: hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief Asynchronously copy matrix from host to device
\details
rocblas_set_matrix_async copies a matrix from pinned host memory to device memory asynchronously.
Memory on the host must be allocated with hipHostMalloc or the transfer will be synchronous.
@param[in]
rows        [rocblas_int]
number of rows in matrices
@param[in]
cols        [rocblas_int]
number of columns in matrices
@param[in]
elem_size   [rocblas_int]
number of bytes per element in the matrix
@param[in]
a           pointer to matrix on the host
@param[in]
lda         [rocblas_int]
specifies the leading dimension of A, lda >= rows
@param[out]
b           pointer to matrix on the GPU
@param[in]
ldb         [rocblas_int]
specifies the leading dimension of B, ldb >= rows
@param[in]
stream      specifies the stream into which this transfer request is queued*/
    pub fn rocblas_set_matrix_async(
        rows: rocblas_int,
        cols: rocblas_int,
        elem_size: rocblas_int,
        a: *const ::core::ffi::c_void,
        lda: rocblas_int,
        b: *mut ::core::ffi::c_void,
        ldb: rocblas_int,
        stream: hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_set_matrix_async_64(
        rows: i64,
        cols: i64,
        elem_size: i64,
        a: *const ::core::ffi::c_void,
        lda: i64,
        b: *mut ::core::ffi::c_void,
        ldb: i64,
        stream: hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief asynchronously copy matrix from device to host
\details
rocblas_get_matrix_async copies a matrix from device memory to pinned host memory asynchronously.
Memory on the host must be allocated with hipHostMalloc or the transfer will be synchronous.
@param[in]
rows        [rocblas_int]
number of rows in matrices
@param[in]
cols        [rocblas_int]
number of columns in matrices
@param[in]
elem_size   [rocblas_int]
number of bytes per element in the matrix
@param[in]
a           pointer to matrix on the GPU
@param[in]
lda         [rocblas_int]
specifies the leading dimension of A, lda >= rows
@param[out]
b           pointer to matrix on the host
@param[in]
ldb         [rocblas_int]
specifies the leading dimension of B, ldb >= rows
@param[in]
stream      specifies the stream into which this transfer request is queued*/
    pub fn rocblas_get_matrix_async(
        rows: rocblas_int,
        cols: rocblas_int,
        elem_size: rocblas_int,
        a: *const ::core::ffi::c_void,
        lda: rocblas_int,
        b: *mut ::core::ffi::c_void,
        ldb: rocblas_int,
        stream: hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_get_matrix_async_64(
        rows: i64,
        cols: i64,
        elem_size: i64,
        a: *const ::core::ffi::c_void,
        lda: i64,
        b: *mut ::core::ffi::c_void,
        ldb: i64,
        stream: hipStream_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /// Function to set start/stop event handlers (for internal use only)
    pub fn rocblas_set_start_stop_events(
        handle: rocblas_handle,
        startEvent: hipEvent_t,
        stopEvent: hipEvent_t,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_set_solution_fitness_query(
        handle: rocblas_handle,
        fitness: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief specifies the performance metric that solution selection uses
\details
Determines which performance metric will be used by Tensile when selecting the optimal solution
for gemm problems. If a valid solution benchmarked for this performance metric does not exist
for a problem, Tensile will default to a solution benchmarked for overall performance instead.
@param[in]
handle      [rocblas_handle]
the handle of device
@param[in]
metric      [rocblas_performance_metric]
the performance metric to be used*/
    pub fn rocblas_set_performance_metric(
        handle: rocblas_handle,
        metric: rocblas_performance_metric,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief returns the performance metric being used for solution selection
\details
Returns the performance metric used by Tensile to select the optimal solution for gemm problems.
@param[in]
handle      [rocblas_handle]
the handle of device
@param[out]
metric      [rocblas_performance_metric*]
pointer to where the metric will be stored*/
    pub fn rocblas_get_performance_metric(
        handle: rocblas_handle,
        metric: *mut rocblas_performance_metric,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
scal  scales each element of vector x with scalar alpha:

x := alpha * x

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x.
@param[in]
alpha     device pointer or host pointer for the scalar alpha.
@param[in, out]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
*/
    pub fn rocblas_sscal(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f32,
        x: *mut f32,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dscal(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f64,
        x: *mut f64,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cscal(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zscal(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csscal(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f32,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdscal(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f64,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sscal_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f32,
        x: *mut f32,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dscal_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f64,
        x: *mut f64,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cscal_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *mut rocblas_float_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zscal_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *mut rocblas_double_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csscal_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f32,
        x: *mut rocblas_float_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdscal_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f64,
        x: *mut rocblas_double_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
scal_batched  scales each element of vector x_i with scalar alpha, for i = 1, ... , batch_count:

x_i := alpha * x_i,
where (x_i) is the i-th instance of the batch.

@param[in]
handle      [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n           [rocblas_int]
the number of elements in each x_i.
@param[in]
alpha       host pointer or device pointer for the scalar alpha.
@param[in, out]
x           device array of device pointers storing each vector x_i.
@param[in]
incx        [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
batch_count [rocblas_int]
specifies the number of batches in x.
*/
    pub fn rocblas_sscal_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *mut f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dscal_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *mut f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cscal_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zscal_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csscal_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdscal_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sscal_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f32,
        x: *const *mut f32,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dscal_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f64,
        x: *const *mut f64,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cscal_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zscal_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csscal_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f32,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdscal_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f64,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
scal_strided_batched  scales each element of vector x_i with scalar alpha, for i = 1, ... , batch_count:

x_i := alpha * x_i,
where (x_i) is the i-th instance of the batch.

@param[in]
handle      [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n           [rocblas_int]
the number of elements in each x_i.
@param[in]
alpha       host pointer or device pointer for the scalar alpha.
@param[in, out]
x           device pointer to the first vector (x_1) in the batch.
@param[in]
incx        [rocblas_int]
specifies the increment for the elements of x.
@param[in]
stride_x    [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stride_x. However, ensure that stride_x is of appropriate size, for a typical
case this means stride_x >= n * incx.
@param[in]
batch_count [rocblas_int]
specifies the number of batches in x.
*/
    pub fn rocblas_sscal_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f32,
        x: *mut f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dscal_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f64,
        x: *mut f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cscal_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zscal_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csscal_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f32,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdscal_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f64,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sscal_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f32,
        x: *mut f32,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dscal_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f64,
        x: *mut f64,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cscal_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zscal_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csscal_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f32,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdscal_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f64,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
copy  copies each element x[i] into y[i], for  i = 1 , ... , n:

y := x

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x to be copied to y.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[out]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_scopy(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        y: *mut f32,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dcopy(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        y: *mut f64,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ccopy(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zcopy(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scopy_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        y: *mut f32,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dcopy_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        y: *mut f64,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ccopy_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        y: *mut rocblas_float_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zcopy_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        y: *mut rocblas_double_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
copy_batched copies each element x_i[j] into y_i[j], for  j = 1 , ... , n; i = 1 , ... , batch_count:

y_i := x_i,
where (x_i, y_i) is the i-th instance of the batch.
x_i and y_i are vectors.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i to be copied to y_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each vector x_i.
@param[out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_scopy_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dcopy_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        y: *const *mut f64,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ccopy_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zcopy_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scopy_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f32,
        incx: i64,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dcopy_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f64,
        incx: i64,
        y: *const *mut f64,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ccopy_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zcopy_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
copy_strided_batched copies each element x_i[j] into y_i[j], for  j = 1 , ... , n; i = 1 , ... , batch_count:

y_i := x_i,
where (x_i, y_i) is the i-th instance of the batch.
x_i and y_i are vectors.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i to be copied to y_i.
@param[in]
x         device pointer to the first vector (x_1) in the batch.
@param[in]
incx      [rocblas_int]
specifies the increments for the elements of vectors x_i.
@param[in]
stridex     [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stride_x. However, the user should
take care to ensure that stride_x is of appropriate size. For a typical
case, this means stride_x >= n * incx.
@param[out]
y         device pointer to the first vector (y_1) in the batch.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of vectors y_i.
@param[in]
stridey     [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
There are no restrictions placed on stride_y, However, ensure that stride_y is of appropriate size, for a typical
case this means stride_y >= n * incy. stridey should be non zero.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_scopy_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dcopy_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ccopy_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zcopy_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scopy_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dcopy_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut f64,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ccopy_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zcopy_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
dot(u)  performs the dot product of vectors x and y:

result = x * y;

dotc  performs the dot product of the conjugate of complex vector x and complex vector y.

result = conjugate (x) * y;

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x and y.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
result
device pointer or host pointer to store the dot product.
return is 0.0 if n <= 0.
*/
    pub fn rocblas_sdot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        y: *const f32,
        incy: rocblas_int,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        y: *const f64,
        incy: rocblas_int,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hdot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_half,
        incx: rocblas_int,
        y: *const rocblas_half,
        incy: rocblas_int,
        result: *mut rocblas_half,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_bfdot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_bfloat16,
        incx: rocblas_int,
        y: *const rocblas_bfloat16,
        incy: rocblas_int,
        result: *mut rocblas_bfloat16,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotu(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotu(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotc(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotc(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sdot_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        y: *const f32,
        incy: i64,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddot_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        y: *const f64,
        incy: i64,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hdot_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_half,
        incx: i64,
        y: *const rocblas_half,
        incy: i64,
        result: *mut rocblas_half,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_bfdot_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_bfloat16,
        incx: i64,
        y: *const rocblas_bfloat16,
        incy: i64,
        result: *mut rocblas_bfloat16,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotu_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        y: *const rocblas_float_complex,
        incy: i64,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotu_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        y: *const rocblas_double_complex,
        incy: i64,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotc_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        y: *const rocblas_float_complex,
        incy: i64,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotc_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        y: *const rocblas_double_complex,
        incy: i64,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
dot_batched(u) performs a batch of dot products of vectors x and y:

result_i = x_i * y_i;

dotc_batched  performs a batch of dot products of the conjugate of complex vector x and complex vector y

result_i = conjugate (x_i) * y_i;
where (x_i, y_i) is the i-th instance of the batch.
x_i and y_i are vectors, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i and y_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[in, out]
result
device array or host array of batch_count size to store the dot products of each batch.
return 0.0 for each element if n <= 0.
*/
    pub fn rocblas_sdot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        y: *const *const f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        y: *const *const f64,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hdot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_half,
        incx: rocblas_int,
        y: *const *const rocblas_half,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_half,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_bfdot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_bfloat16,
        incx: rocblas_int,
        y: *const *const rocblas_bfloat16,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_bfloat16,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotu_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const *const rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotu_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const *const rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotc_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const *const rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotc_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const *const rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sdot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f32,
        incx: i64,
        y: *const *const f32,
        incy: i64,
        batch_count: i64,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f64,
        incx: i64,
        y: *const *const f64,
        incy: i64,
        batch_count: i64,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hdot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_half,
        incx: i64,
        y: *const *const rocblas_half,
        incy: i64,
        batch_count: i64,
        result: *mut rocblas_half,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_bfdot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_bfloat16,
        incx: i64,
        y: *const *const rocblas_bfloat16,
        incy: i64,
        batch_count: i64,
        result: *mut rocblas_bfloat16,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotu_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        y: *const *const rocblas_float_complex,
        incy: i64,
        batch_count: i64,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotu_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        y: *const *const rocblas_double_complex,
        incy: i64,
        batch_count: i64,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotc_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        y: *const *const rocblas_float_complex,
        incy: i64,
        batch_count: i64,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotc_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        y: *const *const rocblas_double_complex,
        incy: i64,
        batch_count: i64,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
dot_strided_batched(u)  performs a batch of dot products of vectors x and y:

result_i = x_i * y_i;

dotc_strided_batched  performs a batch of dot products of the conjugate of complex vector x and complex vector y

result_i = conjugate (x_i) * y_i;
where (x_i, y_i) is the i-th instance of the batch.
x_i and y_i are vectors, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i and y_i.
@param[in]
x         device pointer to the first vector (x_1) in the batch.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stridex     [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in]
y         device pointer to the first vector (y_1) in the batch.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
stridey     [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[in, out]
result
device array or host array of batch_count size to store the dot products of each batch.
return 0.0 for each element if n <= 0.
*/
    pub fn rocblas_sdot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hdot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_half,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_half,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_half,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_bfdot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_bfloat16,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_bfloat16,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_bfloat16,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotu_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotu_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotc_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotc_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sdot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        y: *const f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        y: *const f64,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hdot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_half,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_half,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
        result: *mut rocblas_half,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_bfdot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_bfloat16,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_bfloat16,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
        result: *mut rocblas_bfloat16,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotu_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotu_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdotc_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
        result: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdotc_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
        result: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
swap  interchanges vectors x and y:

y := x;
x := y

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x and y.
@param[in, out]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in, out]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_sswap(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
        y: *mut f32,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dswap(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
        y: *mut f64,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cswap(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zswap(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sswap_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f32,
        incx: i64,
        y: *mut f32,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dswap_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f64,
        incx: i64,
        y: *mut f64,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cswap_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
        y: *mut rocblas_float_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zswap_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
        y: *mut rocblas_double_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
swap_batched interchanges vectors x_i and y_i, for i = 1 , ... , batch_count:

y_i := x_i;
x_i := y_i

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i and y_i.
@param[in, out]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in, out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sswap_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut f32,
        incx: rocblas_int,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dswap_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut f64,
        incx: rocblas_int,
        y: *const *mut f64,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cswap_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zswap_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sswap_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut f32,
        incx: i64,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dswap_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut f64,
        incx: i64,
        y: *const *mut f64,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cswap_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zswap_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
swap_strided_batched interchanges vectors x_i and y_i, for i = 1 , ... , batch_count:

y_i := x_i;
x_i := y_i

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i and y_i.
@param[in, out]
x         device pointer to the first vector x_1.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
stridex   [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stride_x. However, ensure that stride_x is of appropriate size. For a typical
case this means stride_x >= n * incx.
@param[in, out]
y         device pointer to the first vector y_1.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
stridey   [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
There are no restrictions placed on stride_x. However, ensure that stride_y is of appropriate size. For a typical
case this means stride_y >= n * incy. stridey should be non zero.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sswap_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dswap_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cswap_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zswap_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sswap_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f32,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dswap_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f64,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut f64,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cswap_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zswap_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
axpy   computes constant alpha multiplied by vector x, plus vector y:

y := alpha * x + y

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x and y.
@param[in]
alpha     device pointer or host pointer to specify the scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[out]
y         device pointer storing vector y.
@param[in, out]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_haxpy(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_half,
        x: *const rocblas_half,
        incx: rocblas_int,
        y: *mut rocblas_half,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_saxpy(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        y: *mut f32,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_daxpy(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        y: *mut f64,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_caxpy(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zaxpy(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_haxpy_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_half,
        x: *const rocblas_half,
        incx: i64,
        y: *mut rocblas_half,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_saxpy_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        y: *mut f32,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_daxpy_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        y: *mut f64,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_caxpy_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        y: *mut rocblas_float_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zaxpy_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        y: *mut rocblas_double_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
axpy_batched compute y := alpha * x + y over a set of batched vectors.

@param[in]
handle    rocblas_handle
handle to the rocblas library context queue.
@param[in]
n         rocblas_int
@param[in]
alpha     specifies the scalar alpha.
@param[in]
x         pointer storing vector x on the GPU.
@param[in]
incx      rocblas_int
specifies the increment for the elements of x.
@param[out]
y         pointer storing vector y on the GPU.
@param[in, out]
incy      rocblas_int
specifies the increment for the elements of y.

@param[in]
batch_count rocblas_int
number of instances in the batch.
*/
    pub fn rocblas_haxpy_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_half,
        x: *const *const rocblas_half,
        incx: rocblas_int,
        y: *const *mut rocblas_half,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_saxpy_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *const f32,
        incx: rocblas_int,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_daxpy_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *const f64,
        incx: rocblas_int,
        y: *const *mut f64,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_caxpy_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zaxpy_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_haxpy_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_half,
        x: *const *const rocblas_half,
        incx: i64,
        y: *const *mut rocblas_half,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_saxpy_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f32,
        x: *const *const f32,
        incx: i64,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_daxpy_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f64,
        x: *const *const f64,
        incx: i64,
        y: *const *mut f64,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_caxpy_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: i64,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zaxpy_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: i64,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
axpy_strided_batched compute y := alpha * x + y over a set of strided batched vectors.

@param[in]
handle    rocblas_handle
handle to the rocblas library context queue.
@param[in]
n         rocblas_int.
@param[in]
alpha     specifies the scalar alpha.
@param[in]
x         pointer storing vector x on the GPU.
@param[in]
incx      rocblas_int
specifies the increment for the elements of x.
@param[in]
stridex   rocblas_stride
specifies the increment between vectors of x.
@param[out]
y         pointer storing vector y on the GPU.
@param[in, out]
incy      rocblas_int
specifies the increment for the elements of y.
@param[in]
stridey   rocblas_stride
specifies the increment between vectors of y.

@param[in]
batch_count rocblas_int
number of instances in the batch.
*/
    pub fn rocblas_haxpy_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_half,
        x: *const rocblas_half,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut rocblas_half,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_saxpy_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_daxpy_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_caxpy_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zaxpy_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_haxpy_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_half,
        x: *const rocblas_half,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut rocblas_half,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_saxpy_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_daxpy_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut f64,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_caxpy_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zaxpy_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
asum computes the sum of the magnitudes of elements of a real vector x,
or the sum of magnitudes of the real and imaginary parts of elements if x is a complex vector.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x and y.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x. incx must be > 0.
@param[in, out]
result
device pointer or host pointer to store the asum product.
return is 0.0 if n <= 0.
*/
    pub fn rocblas_sasum(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dasum(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scasum(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dzasum(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sasum_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dasum_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scasum_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dzasum_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
asum_batched computes the sum of the magnitudes of the elements in a batch of real vectors x_i,
or the sum of magnitudes of the real and imaginary parts of elements if x_i is a complex
vector, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each vector x_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[out]
results
device array or host array of batch_count size for results.
return is 0.0 if n, incx<=0.
*/
    pub fn rocblas_sasum_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dasum_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scasum_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dzasum_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sasum_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f32,
        incx: i64,
        batch_count: i64,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dasum_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f64,
        incx: i64,
        batch_count: i64,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scasum_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        batch_count: i64,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dzasum_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        batch_count: i64,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
asum_strided_batched computes the sum of the magnitudes of elements of a real vectors x_i,
or the sum of magnitudes of the real and imaginary parts of elements if x_i is a complex
vector, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each vector x_i.
@param[in]
x         device pointer to the first vector x_1.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
stridex   [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stride_x. However, ensure that stride_x is of appropriate size. For a typical
case this means stride_x >= n * incx.
@param[out]
results
device pointer or host pointer to array for storing contiguous batch_count results.
return is 0.0 if n, incx<=0.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sasum_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dasum_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scasum_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dzasum_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sasum_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dasum_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scasum_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dzasum_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
nrm2 computes the euclidean norm of a real or complex vector:

result := sqrt( x'*x ) for real vectors
result := sqrt( x**H*x ) for complex vectors

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
result
device pointer or host pointer to store the nrm2 product.
return is 0.0 if n, incx<=0.
*/
    pub fn rocblas_snrm2(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dnrm2(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scnrm2(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dznrm2(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_snrm2_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dnrm2_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scnrm2_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        result: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dznrm2_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        result: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
nrm2_batched computes the euclidean norm over a batch of real or complex vectors:

result := sqrt( x_i'*x_i ) for real vectors x, for i = 1, ..., batch_count
result := sqrt( x_i**H*x_i ) for complex vectors x, for i = 1, ..., batch_count

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each x_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[out]
results
device pointer or host pointer to array of batch_count size for nrm2 results.
return is 0.0 for each element if n <= 0, incx<=0.
*/
    pub fn rocblas_snrm2_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dnrm2_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scnrm2_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dznrm2_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_snrm2_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f32,
        incx: i64,
        batch_count: i64,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dnrm2_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f64,
        incx: i64,
        batch_count: i64,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scnrm2_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        batch_count: i64,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dznrm2_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        batch_count: i64,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
nrm2_strided_batched computes the euclidean norm over a batch of real or complex vectors:

result := sqrt( x_i'*x_i ) for real vectors x, for i = 1, ..., batch_count
result := sqrt( x_i**H*x_i ) for complex vectors, for i = 1, ..., batch_count

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each x_i.
@param[in]
x         device pointer to the first vector x_1.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
stridex   [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stride_x. However, ensure that stride_x is of appropriate size. For a typical
case this means stride_x >= n * incx.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[out]
results
device pointer or host pointer to array for storing contiguous batch_count results.
return is 0.0 for each element if n <= 0, incx<=0.
*/
    pub fn rocblas_snrm2_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dnrm2_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scnrm2_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dznrm2_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_snrm2_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dnrm2_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scnrm2_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        results: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dznrm2_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        results: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
amax finds the first index of the element of maximum magnitude of a vector x.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
result
device pointer or host pointer to store the amax index.
return is 0.0 if n, incx<=0.
*/
    pub fn rocblas_isamax(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamax(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamax(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamax(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_isamax_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamax_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamax_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamax_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
amax_batched finds the first index of the element of maximum magnitude of each vector x_i in a batch, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each vector x_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
batch_count [rocblas_int]
number of instances in the batch. Must be > 0.
@param[out]
result
device or host array of pointers of batch_count size for results.
return is 0 if n, incx<=0.
*/
    pub fn rocblas_isamax_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamax_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamax_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamax_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_isamax_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f32,
        incx: i64,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamax_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f64,
        incx: i64,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamax_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamax_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
amax_strided_batched finds the first index of the element of maximum magnitude of each vector x_i in a batch, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each vector x_i.
@param[in]
x         device pointer to the first vector x_1.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
stridex   [rocblas_stride]
specifies the pointer increment between one x_i and the next x_(i + 1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[out]
result
device or host pointer for storing contiguous batch_count results.
return is 0 if n <= 0, incx<=0.
*/
    pub fn rocblas_isamax_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamax_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamax_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamax_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_isamax_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamax_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamax_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamax_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
amin finds the first index of the element of minimum magnitude of a vector x.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
result
device pointer or host pointer to store the amin index.
return is 0.0 if n, incx<=0.
*/
    pub fn rocblas_isamin(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamin(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamin(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamin(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_isamin_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamin_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamin_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamin_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
amin_batched finds the first index of the element of minimum magnitude of each vector x_i in a batch, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each vector x_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
batch_count [rocblas_int]
number of instances in the batch. Must be > 0.
@param[out]
result
device or host pointers to array of batch_count size for results.
return is 0 if n, incx<=0.
*/
    pub fn rocblas_isamin_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamin_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamin_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamin_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_isamin_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f32,
        incx: i64,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamin_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const f64,
        incx: i64,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamin_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamin_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
amin_strided_batched finds the first index of the element of minimum magnitude of each vector x_i in a batch, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each vector x_i.
@param[in]
x         device pointer to the first vector x_1.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
stridex   [rocblas_stride]
specifies the pointer increment between one x_i and the next x_(i + 1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[out]
result
device or host pointer to array for storing contiguous batch_count results.
return is 0 if n <= 0, incx<=0.
*/
    pub fn rocblas_isamin_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamin_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamin_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamin_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_isamin_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_idamin_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_icamin_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_izamin_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        result: *mut i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rot applies the Givens rotation matrix defined by c=cos(alpha) and s=sin(alpha) to vectors x and y.
Scalars c and s may be stored in either host or device memory. Location is specified by calling rocblas_set_pointer_mode.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n       [rocblas_int]
number of elements in the x and y vectors.
@param[in, out]
x       device pointer storing vector x.
@param[in]
incx    [rocblas_int]
specifies the increment between elements of x.
@param[in, out]
y       device pointer storing vector y.
@param[in]
incy    [rocblas_int]
specifies the increment between elements of y.
@param[in]
c       device pointer or host pointer storing scalar cosine component of the rotation matrix.
@param[in]
s       device pointer or host pointer storing scalar sine component of the rotation matrix.
*/
    pub fn rocblas_srot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
        y: *mut f32,
        incy: rocblas_int,
        c: *const f32,
        s: *const f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
        y: *mut f64,
        incy: rocblas_int,
        c: *const f64,
        s: *const f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        c: *const f32,
        s: *const rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csrot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        c: *const f32,
        s: *const f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        c: *const f64,
        s: *const rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdrot(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        c: *const f64,
        s: *const f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srot_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f32,
        incx: i64,
        y: *mut f32,
        incy: i64,
        c: *const f32,
        s: *const f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drot_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f64,
        incx: i64,
        y: *mut f64,
        incy: i64,
        c: *const f64,
        s: *const f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crot_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
        y: *mut rocblas_float_complex,
        incy: i64,
        c: *const f32,
        s: *const rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csrot_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
        y: *mut rocblas_float_complex,
        incy: i64,
        c: *const f32,
        s: *const f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrot_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
        y: *mut rocblas_double_complex,
        incy: i64,
        c: *const f64,
        s: *const rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdrot_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
        y: *mut rocblas_double_complex,
        incy: i64,
        c: *const f64,
        s: *const f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rot_batched applies the Givens rotation matrix defined by c=cos(alpha) and s=sin(alpha) to batched vectors x_i and y_i, for i = 1, ..., batch_count.
Scalars c and s may be stored in either host or device memory. Location is specified by calling rocblas_set_pointer_mode.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n       [rocblas_int]
number of elements in each x_i and y_i vectors.
@param[in, out]
x       device array of deivce pointers storing each vector x_i.
@param[in]
incx    [rocblas_int]
specifies the increment between elements of each x_i.
@param[in, out]
y       device array of device pointers storing each vector y_i.
@param[in]
incy    [rocblas_int]
specifies the increment between elements of each y_i.
@param[in]
c       device pointer or host pointer to scalar cosine component of the rotation matrix.
@param[in]
s       device pointer or host pointer to scalar sine component of the rotation matrix.
@param[in]
batch_count [rocblas_int]
the number of x and y arrays, i.e. the number of batches.
*/
    pub fn rocblas_srot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut f32,
        incx: rocblas_int,
        y: *const *mut f32,
        incy: rocblas_int,
        c: *const f32,
        s: *const f32,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut f64,
        incx: rocblas_int,
        y: *const *mut f64,
        incy: rocblas_int,
        c: *const f64,
        s: *const f64,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        c: *const f32,
        s: *const rocblas_float_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csrot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        c: *const f32,
        s: *const f32,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        c: *const f64,
        s: *const rocblas_double_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdrot_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        c: *const f64,
        s: *const f64,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut f32,
        incx: i64,
        y: *const *mut f32,
        incy: i64,
        c: *const f32,
        s: *const f32,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut f64,
        incx: i64,
        y: *const *mut f64,
        incy: i64,
        c: *const f64,
        s: *const f64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        c: *const f32,
        s: *const rocblas_float_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csrot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        c: *const f32,
        s: *const f32,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        c: *const f64,
        s: *const rocblas_double_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdrot_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        c: *const f64,
        s: *const f64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rot_strided_batched applies the Givens rotation matrix defined by c=cos(alpha) and s=sin(alpha) to strided batched vectors x_i and y_i, for i = 1, ..., batch_count.
Scalars c and s may be stored in either host or device memory, location is specified by calling rocblas_set_pointer_mode.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n       [rocblas_int]
number of elements in each x_i and y_i vectors.
@param[in, out]
x       device pointer to the first vector x_1.
@param[in]
incx    [rocblas_int]
specifies the increment between elements of each x_i.
@param[in]
stride_x [rocblas_stride]
specifies the increment from the beginning of x_i to the beginning of x_(i+1).
@param[in, out]
y       device pointer to the first vector y_1.
@param[in]
incy    [rocblas_int]
specifies the increment between elements of each y_i.
@param[in]
stride_y [rocblas_stride]
specifies the increment from the beginning of y_i to the beginning of y_(i+1)
@param[in]
c       device pointer or host pointer to scalar cosine component of the rotation matrix.
@param[in]
s       device pointer or host pointer to scalar sine component of the rotation matrix.
@param[in]
batch_count [rocblas_int]
the number of x and y arrays, i.e. the number of batches.
*/
    pub fn rocblas_srot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *mut f32,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        c: *const f32,
        s: *const f32,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *mut f64,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        c: *const f64,
        s: *const f64,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        c: *const f32,
        s: *const rocblas_float_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csrot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        c: *const f32,
        s: *const f32,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        c: *const f64,
        s: *const rocblas_double_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdrot_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        c: *const f64,
        s: *const f64,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f32,
        incx: i64,
        stride_x: rocblas_stride,
        y: *mut f32,
        incy: i64,
        stride_y: rocblas_stride,
        c: *const f32,
        s: *const f32,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f64,
        incx: i64,
        stride_x: rocblas_stride,
        y: *mut f64,
        incy: i64,
        stride_y: rocblas_stride,
        c: *const f64,
        s: *const f64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: i64,
        stride_y: rocblas_stride,
        c: *const f32,
        s: *const rocblas_float_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csrot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        y: *mut rocblas_float_complex,
        incy: i64,
        stride_y: rocblas_stride,
        c: *const f32,
        s: *const f32,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: i64,
        stride_y: rocblas_stride,
        c: *const f64,
        s: *const rocblas_double_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdrot_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        y: *mut rocblas_double_complex,
        incy: i64,
        stride_y: rocblas_stride,
        c: *const f64,
        s: *const f64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rotg creates the Givens rotation matrix for the vector (a b).
Scalars a, b, c, and s may be stored in either host or device memory, location is specified by
calling rocblas_set_pointer_mode. The computation uses the formulas

sigma = sgn(a)    if |a| >  |b|
= sgn(b)    if |b| >= |a|
r = sigma*sqrt( a**2 + b**2 )
c = 1; s = 0      if r = 0
c = a/r; s = b/r  if r != 0

The subroutine also computes

z = s    if |a| > |b|,
= 1/c  if |b| >= |a| and c != 0
= 1    if c = 0

This allows c and s to be reconstructed from z as follows:

If z = 1, set c = 0, s = 1.
If |z| < 1, set c = sqrt(1 - z**2) and s = z.
If |z| > 1, set c = 1/z and s = sqrt( 1 - c**2).

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in, out]
a       pointer to a, an element in vector (a,b), overwritten with r.
@param[in, out]
b       pointer to b, an element in vector (a,b), overwritten with z.
@param[out]
c       pointer to c, cosine element of Givens rotation.
@param[out]
s       pointer to s, sine element of Givens rotation.
*/
    pub fn rocblas_srotg(
        handle: rocblas_handle,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        s: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotg(
        handle: rocblas_handle,
        a: *mut f64,
        b: *mut f64,
        c: *mut f64,
        s: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crotg(
        handle: rocblas_handle,
        a: *mut rocblas_float_complex,
        b: *mut rocblas_float_complex,
        c: *mut f32,
        s: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrotg(
        handle: rocblas_handle,
        a: *mut rocblas_double_complex,
        b: *mut rocblas_double_complex,
        c: *mut f64,
        s: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srotg_64(
        handle: rocblas_handle,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        s: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotg_64(
        handle: rocblas_handle,
        a: *mut f64,
        b: *mut f64,
        c: *mut f64,
        s: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crotg_64(
        handle: rocblas_handle,
        a: *mut rocblas_float_complex,
        b: *mut rocblas_float_complex,
        c: *mut f32,
        s: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrotg_64(
        handle: rocblas_handle,
        a: *mut rocblas_double_complex,
        b: *mut rocblas_double_complex,
        c: *mut f64,
        s: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rotg_batched creates the Givens rotation matrix for the batched vectors (a_i b_i), for i = 1, ..., batch_count.
a, b, c, and s are host pointers to an array of device pointers on the device, where each device pointer points
to a scalar value of a_i, b_i, c_i, or s_i.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in, out]
a       a, overwritten with r.
@param[in, out]
b       b overwritten with z.
@param[out]
c       cosine element of Givens rotation for the batch.
@param[out]
s       sine element of Givens rotation for the batch.
@param[in]
batch_count [rocblas_int]
number of batches (length of arrays a, b, c, and s).
*/
    pub fn rocblas_srotg_batched(
        handle: rocblas_handle,
        a: *const *mut f32,
        b: *const *mut f32,
        c: *const *mut f32,
        s: *const *mut f32,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotg_batched(
        handle: rocblas_handle,
        a: *const *mut f64,
        b: *const *mut f64,
        c: *const *mut f64,
        s: *const *mut f64,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crotg_batched(
        handle: rocblas_handle,
        a: *const *mut rocblas_float_complex,
        b: *const *mut rocblas_float_complex,
        c: *const *mut f32,
        s: *const *mut rocblas_float_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrotg_batched(
        handle: rocblas_handle,
        a: *const *mut rocblas_double_complex,
        b: *const *mut rocblas_double_complex,
        c: *const *mut f64,
        s: *const *mut rocblas_double_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srotg_batched_64(
        handle: rocblas_handle,
        a: *const *mut f32,
        b: *const *mut f32,
        c: *const *mut f32,
        s: *const *mut f32,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotg_batched_64(
        handle: rocblas_handle,
        a: *const *mut f64,
        b: *const *mut f64,
        c: *const *mut f64,
        s: *const *mut f64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crotg_batched_64(
        handle: rocblas_handle,
        a: *const *mut rocblas_float_complex,
        b: *const *mut rocblas_float_complex,
        c: *const *mut f32,
        s: *const *mut rocblas_float_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrotg_batched_64(
        handle: rocblas_handle,
        a: *const *mut rocblas_double_complex,
        b: *const *mut rocblas_double_complex,
        c: *const *mut f64,
        s: *const *mut rocblas_double_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rotg_strided_batched creates the Givens rotation matrix for the strided batched vectors (a_i b_i), for i = 1, ..., batch_count.
a, b, c, and s are host pointers to arrays a, b, c, s on the device.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in, out]
a       host pointer to first single input vector element a_1 on the device, overwritten with r.
@param[in]
stride_a [rocblas_stride]
distance between elements of a in batch (distance between a_i and a_(i + 1)).
@param[in, out]
b       host pointer to first single input vector element b_1 on the device, overwritten with z.
@param[in]
stride_b [rocblas_stride]
distance between elements of b in batch (distance between b_i and b_(i + 1)).
@param[out]
c       host pointer to first single cosine element of Givens rotations c_1 on the device.
@param[in]
stride_c [rocblas_stride]
distance between elements of c in batch (distance between c_i and c_(i + 1)).
@param[out]
s       host pointer to first single sine element of Givens rotations s_1 on the device.
@param[in]
stride_s [rocblas_stride]
distance between elements of s in batch (distance between s_i and s_(i + 1)).
@param[in]
batch_count [rocblas_int]
number of batches (length of arrays a, b, c, and s).
*/
    pub fn rocblas_srotg_strided_batched(
        handle: rocblas_handle,
        a: *mut f32,
        stride_a: rocblas_stride,
        b: *mut f32,
        stride_b: rocblas_stride,
        c: *mut f32,
        stride_c: rocblas_stride,
        s: *mut f32,
        stride_s: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotg_strided_batched(
        handle: rocblas_handle,
        a: *mut f64,
        stride_a: rocblas_stride,
        b: *mut f64,
        stride_b: rocblas_stride,
        c: *mut f64,
        stride_c: rocblas_stride,
        s: *mut f64,
        stride_s: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crotg_strided_batched(
        handle: rocblas_handle,
        a: *mut rocblas_float_complex,
        stride_a: rocblas_stride,
        b: *mut rocblas_float_complex,
        stride_b: rocblas_stride,
        c: *mut f32,
        stride_c: rocblas_stride,
        s: *mut rocblas_float_complex,
        stride_s: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrotg_strided_batched(
        handle: rocblas_handle,
        a: *mut rocblas_double_complex,
        stride_a: rocblas_stride,
        b: *mut rocblas_double_complex,
        stride_b: rocblas_stride,
        c: *mut f64,
        stride_c: rocblas_stride,
        s: *mut rocblas_double_complex,
        stride_s: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srotg_strided_batched_64(
        handle: rocblas_handle,
        a: *mut f32,
        stride_a: rocblas_stride,
        b: *mut f32,
        stride_b: rocblas_stride,
        c: *mut f32,
        stride_c: rocblas_stride,
        s: *mut f32,
        stride_s: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotg_strided_batched_64(
        handle: rocblas_handle,
        a: *mut f64,
        stride_a: rocblas_stride,
        b: *mut f64,
        stride_b: rocblas_stride,
        c: *mut f64,
        stride_c: rocblas_stride,
        s: *mut f64,
        stride_s: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_crotg_strided_batched_64(
        handle: rocblas_handle,
        a: *mut rocblas_float_complex,
        stride_a: rocblas_stride,
        b: *mut rocblas_float_complex,
        stride_b: rocblas_stride,
        c: *mut f32,
        stride_c: rocblas_stride,
        s: *mut rocblas_float_complex,
        stride_s: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zrotg_strided_batched_64(
        handle: rocblas_handle,
        a: *mut rocblas_double_complex,
        stride_a: rocblas_stride,
        b: *mut rocblas_double_complex,
        stride_b: rocblas_stride,
        c: *mut f64,
        stride_c: rocblas_stride,
        s: *mut rocblas_double_complex,
        stride_s: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rotm applies the modified Givens rotation matrix defined by param to vectors x and y.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n       [rocblas_int]
number of elements in the x and y vectors.
@param[in, out]
x       device pointer storing vector x.
@param[in]
incx    [rocblas_int]
specifies the increment between elements of x.
@param[in, out]
y       device pointer storing vector y.
@param[in]
incy    [rocblas_int]
specifies the increment between elements of y.
@param[in]
param   device vector or host vector of 5 elements defining the rotation.

param[0] = flag
param[1] = H11
param[2] = H21
param[3] = H12
param[4] = H22

The flag parameter defines the form of H:

flag = -1 => H = ( H11 H12 H21 H22 )
flag =  0 => H = ( 1.0 H12 H21 1.0 )
flag =  1 => H = ( H11 1.0 -1.0 H22 )
flag = -2 => H = ( 1.0 0.0 0.0 1.0 )

param may be stored in either host or device memory,
location is specified by calling rocblas_set_pointer_mode.
*/
    pub fn rocblas_srotm(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
        y: *mut f32,
        incy: rocblas_int,
        param: *const f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotm(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
        y: *mut f64,
        incy: rocblas_int,
        param: *const f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srotm_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f32,
        incx: i64,
        y: *mut f32,
        incy: i64,
        param: *const f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotm_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f64,
        incx: i64,
        y: *mut f64,
        incy: i64,
        param: *const f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rotm_batched applies the modified Givens rotation matrix defined by param_i to batched vectors x_i and y_i, for i = 1, ..., batch_count.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n       [rocblas_int]
number of elements in the x and y vectors.
@param[in, out]
x       device array of device pointers storing each vector x_i.
@param[in]
incx    [rocblas_int]
specifies the increment between elements of each x_i.
@param[in, out]
y       device array of device pointers storing each vector y_1.
@param[in]
incy    [rocblas_int]
specifies the increment between elements of each y_i.
@param[in]
param   device array of device vectors of 5 elements defining the rotation.

param[0] = flag
param[1] = H11
param[2] = H21
param[3] = H12
param[4] = H22

The flag parameter defines the form of H:

flag = -1 => H = ( H11 H12 H21 H22 )
flag =  0 => H = ( 1.0 H12 H21 1.0 )
flag =  1 => H = ( H11 1.0 -1.0 H22 )
flag = -2 => H = ( 1.0 0.0 0.0 1.0 )

param may ONLY be stored on the device for the batched version of this function.

@param[in]
batch_count [rocblas_int]
the number of x and y arrays, i.e. the number of batches.
*/
    pub fn rocblas_srotm_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut f32,
        incx: rocblas_int,
        y: *const *mut f32,
        incy: rocblas_int,
        param: *const *const f32,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotm_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const *mut f64,
        incx: rocblas_int,
        y: *const *mut f64,
        incy: rocblas_int,
        param: *const *const f64,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srotm_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut f32,
        incx: i64,
        y: *const *mut f32,
        incy: i64,
        param: *const *const f32,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotm_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *const *mut f64,
        incx: i64,
        y: *const *mut f64,
        incy: i64,
        param: *const *const f64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rotm_strided_batched applies the modified Givens rotation matrix defined by param_i to strided batched vectors x_i and y_i, for i = 1, ..., batch_count

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n       [rocblas_int]
number of elements in the x and y vectors.
@param[in, out]
x       device pointer pointing to first strided batched vector x_1.
@param[in]
incx    [rocblas_int]
specifies the increment between elements of each x_i.
@param[in]
stride_x [rocblas_stride]
specifies the increment between the beginning of x_i and x_(i + 1)
@param[in, out]
y       device pointer pointing to first strided batched vector y_1.
@param[in]
incy    [rocblas_int]
specifies the increment between elements of each y_i.
@param[in]
stride_y [rocblas_stride]
specifies the increment between the beginning of y_i and y_(i + 1).
@param[in]
param   device pointer pointing to first array of 5 elements defining the rotation (param_1).

param[0] = flag
param[1] = H11
param[2] = H21
param[3] = H12
param[4] = H22

The flag parameter defines the form of H:

flag = -1 => H = ( H11 H12 H21 H22 )
flag =  0 => H = ( 1.0 H12 H21 1.0 )
flag =  1 => H = ( H11 1.0 -1.0 H22 )
flag = -2 => H = ( 1.0 0.0 0.0 1.0 )

param may ONLY be stored on the device for the strided_batched
version of this function.

@param[in]
stride_param [rocblas_stride]
specifies the increment between the beginning of param_i and param_(i + 1).
@param[in]
batch_count [rocblas_int]
the number of x and y arrays, i.e. the number of batches.
*/
    pub fn rocblas_srotm_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *mut f32,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        param: *const f32,
        stride_param: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotm_strided_batched(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *mut f64,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        param: *const f64,
        stride_param: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srotm_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f32,
        incx: i64,
        stride_x: rocblas_stride,
        y: *mut f32,
        incy: i64,
        stride_y: rocblas_stride,
        param: *const f32,
        stride_param: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotm_strided_batched_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut f64,
        incx: i64,
        stride_x: rocblas_stride,
        y: *mut f64,
        incy: i64,
        stride_y: rocblas_stride,
        param: *const f64,
        stride_param: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rotmg creates the modified Givens rotation matrix for the vector (d1 * x1, d2 * y1).
Parameters may be stored in either host or device memory. Location is specified by calling rocblas_set_pointer_mode:

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in, out]
d1      device pointer or host pointer to input scalar that is overwritten.
@param[in, out]
d2      device pointer or host pointer to input scalar that is overwritten.
@param[in, out]
x1      device pointer or host pointer to input scalar that is overwritten.
@param[in]
y1      device pointer or host pointer to input scalar.
@param[out]
param   device vector or host vector of five elements defining the rotation.

param[0] = flag
param[1] = H11
param[2] = H21
param[3] = H12
param[4] = H22

The flag parameter defines the form of H:

flag = -1 => H = ( H11 H12 H21 H22 )
flag =  0 => H = ( 1.0 H12 H21 1.0 )
flag =  1 => H = ( H11 1.0 -1.0 H22 )
flag = -2 => H = ( 1.0 0.0 0.0 1.0 )

param may be stored in either host or device memory.
Location is specified by calling rocblas_set_pointer_mode.
*/
    pub fn rocblas_srotmg(
        handle: rocblas_handle,
        d1: *mut f32,
        d2: *mut f32,
        x1: *mut f32,
        y1: *const f32,
        param: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotmg(
        handle: rocblas_handle,
        d1: *mut f64,
        d2: *mut f64,
        x1: *mut f64,
        y1: *const f64,
        param: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srotmg_64(
        handle: rocblas_handle,
        d1: *mut f32,
        d2: *mut f32,
        x1: *mut f32,
        y1: *const f32,
        param: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotmg_64(
        handle: rocblas_handle,
        d1: *mut f64,
        d2: *mut f64,
        x1: *mut f64,
        y1: *const f64,
        param: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rotmg_batched creates the modified Givens rotation matrix for the batched vectors (d1_i * x1_i, d2_i * y1_i), for i = 1, ..., batch_count.
Parameters may be stored in either host or device memory. Location is specified by calling rocblas_set_pointer_mode:

- If the pointer mode is set to rocblas_pointer_mode_host, then this function blocks the CPU until the GPU has finished and the results are available in host memory.
- If the pointer mode is set to rocblas_pointer_mode_device, then this function returns immediately and synchronization is required to read the results.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in, out]
d1      device batched array or host batched array of input scalars that is overwritten.
@param[in, out]
d2      device batched array or host batched array of input scalars that is overwritten.
@param[in, out]
x1      device batched array or host batched array of input scalars that is overwritten.
@param[in]
y1      device batched array or host batched array of input scalars.
@param[out]
param   device batched array or host batched array of vectors of 5 elements defining the rotation.

param[0] = flag
param[1] = H11
param[2] = H21
param[3] = H12
param[4] = H22

The flag parameter defines the form of H:

flag = -1 => H = ( H11 H12 H21 H22 )
flag =  0 => H = ( 1.0 H12 H21 1.0 )
flag =  1 => H = ( H11 1.0 -1.0 H22 )
flag = -2 => H = ( 1.0 0.0 0.0 1.0 )

param may be stored in either host or device memory.
Location is specified by calling rocblas_set_pointer_mode.

@param[in]
batch_count [rocblas_int]
the number of instances in the batch.
*/
    pub fn rocblas_srotmg_batched(
        handle: rocblas_handle,
        d1: *const *mut f32,
        d2: *const *mut f32,
        x1: *const *mut f32,
        y1: *const *const f32,
        param: *const *mut f32,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotmg_batched(
        handle: rocblas_handle,
        d1: *const *mut f64,
        d2: *const *mut f64,
        x1: *const *mut f64,
        y1: *const *const f64,
        param: *const *mut f64,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srotmg_batched_64(
        handle: rocblas_handle,
        d1: *const *mut f32,
        d2: *const *mut f32,
        x1: *const *mut f32,
        y1: *const *const f32,
        param: *const *mut f32,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotmg_batched_64(
        handle: rocblas_handle,
        d1: *const *mut f64,
        d2: *const *mut f64,
        x1: *const *mut f64,
        y1: *const *const f64,
        param: *const *mut f64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rotmg_strided_batched creates the modified Givens rotation matrix for the strided batched vectors (d1_i * x1_i, d2_i * y1_i), for i = 1, ..., batch_count.
Parameters may be stored in either host or device memory. Location is specified by calling rocblas_set_pointer_mode:

- If the pointer mode is set to rocblas_pointer_mode_host, then this function blocks the CPU until the GPU has finished and the results are available in host memory.
- If the pointer mode is set to rocblas_pointer_mode_device, then this function returns immediately and synchronization is required to read the results.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in, out]
d1      device strided_batched array or host strided_batched array of input scalars that is overwritten.
@param[in]
stride_d1 [rocblas_stride]
specifies the increment between the beginning of d1_i and d1_(i+1).
@param[in, out]
d2      device strided_batched array or host strided_batched array of input scalars that is overwritten.
@param[in]
stride_d2 [rocblas_stride]
specifies the increment between the beginning of d2_i and d2_(i+1).
@param[in, out]
x1      device strided_batched array or host strided_batched array of input scalars that is overwritten.
@param[in]
stride_x1 [rocblas_stride]
specifies the increment between the beginning of x1_i and x1_(i+1).
@param[in]
y1      device strided_batched array or host strided_batched array of input scalars.
@param[in]
stride_y1 [rocblas_stride]
specifies the increment between the beginning of y1_i and y1_(i+1).
@param[out]
param   device strided_batched array or host strided_batched array of vectors of 5 elements defining the rotation.

param[0] = flag
param[1] = H11
param[2] = H21
param[3] = H12
param[4] = H22
The flag parameter defines the form of H:

flag = -1 => H = ( H11 H12 H21 H22 )
flag =  0 => H = ( 1.0 H12 H21 1.0 )
flag =  1 => H = ( H11 1.0 -1.0 H22 )
flag = -2 => H = ( 1.0 0.0 0.0 1.0 )

param may be stored in either host or device memory.
Location is specified by calling rocblas_set_pointer_mode.

@param[in]
stride_param [rocblas_stride]
specifies the increment between the beginning of param_i and param_(i + 1).
@param[in]
batch_count [rocblas_int]
the number of instances in the batch.
*/
    pub fn rocblas_srotmg_strided_batched(
        handle: rocblas_handle,
        d1: *mut f32,
        stride_d1: rocblas_stride,
        d2: *mut f32,
        stride_d2: rocblas_stride,
        x1: *mut f32,
        stride_x1: rocblas_stride,
        y1: *const f32,
        stride_y1: rocblas_stride,
        param: *mut f32,
        stride_param: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotmg_strided_batched(
        handle: rocblas_handle,
        d1: *mut f64,
        stride_d1: rocblas_stride,
        d2: *mut f64,
        stride_d2: rocblas_stride,
        x1: *mut f64,
        stride_x1: rocblas_stride,
        y1: *const f64,
        stride_y1: rocblas_stride,
        param: *mut f64,
        stride_param: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_srotmg_strided_batched_64(
        handle: rocblas_handle,
        d1: *mut f32,
        stride_d1: rocblas_stride,
        d2: *mut f32,
        stride_d2: rocblas_stride,
        x1: *mut f32,
        stride_x1: rocblas_stride,
        y1: *const f32,
        stride_y1: rocblas_stride,
        param: *mut f32,
        stride_param: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_drotmg_strided_batched_64(
        handle: rocblas_handle,
        d1: *mut f64,
        stride_d1: rocblas_stride,
        d2: *mut f64,
        stride_d2: rocblas_stride,
        x1: *mut f64,
        stride_x1: rocblas_stride,
        y1: *const f64,
        stride_y1: rocblas_stride,
        param: *mut f64,
        stride_param: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
gbmv performs one of the matrix-vector operations:

y := alpha*A*x    + beta*y,   or
y := alpha*A**T*x + beta*y,   or
y := alpha*A**H*x + beta*y,
where alpha and beta are scalars, x and y are vectors and A is an
m by n banded matrix with kl sub-diagonals and ku super-diagonals.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
trans     [rocblas_operation]
indicates whether matrix A is tranposed (conjugated) or not.
@param[in]
m         [rocblas_int]
number of rows of matrix A.
@param[in]
n         [rocblas_int]
number of columns of matrix A.
@param[in]
kl        [rocblas_int]
number of sub-diagonals of A.
@param[in]
ku        [rocblas_int]
number of super-diagonals of A.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A     device pointer storing banded matrix A.
Leading (kl + ku + 1) by n part of the matrix contains the coefficients
of the banded matrix. The leading diagonal resides in row (ku + 1) with
the first super-diagonal above on the RHS of row ku. The first sub-diagonal
resides below on the LHS of row ku + 2. This propagates up and down across
sub/super-diagonals.

Ex: (m = n = 7; ku = 2, kl = 2)
1 2 3 0 0 0 0             0 0 3 3 3 3 3
4 1 2 3 0 0 0             0 2 2 2 2 2 2
5 4 1 2 3 0 0    ---->    1 1 1 1 1 1 1
0 5 4 1 2 3 0             4 4 4 4 4 4 0
0 0 5 4 1 2 3             5 5 5 5 5 0 0
0 0 0 5 4 1 2             0 0 0 0 0 0 0
0 0 0 0 5 4 1             0 0 0 0 0 0 0

Note that the empty elements which do not correspond to data will not
be referenced.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. Must be >= (kl + ku + 1).
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_sgbmv(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgbmv(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgbmv(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgbmv(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgbmv_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        x: *const f32,
        incx: i64,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgbmv_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        x: *const f64,
        incx: i64,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgbmv_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgbmv_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
gbmv_batched performs one of the matrix-vector operations:

y_i := alpha*A_i*x_i    + beta*y_i,   or
y_i := alpha*A_i**T*x_i + beta*y_i,   or
y_i := alpha*A_i**H*x_i + beta*y_i,
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
m by n banded matrix with kl sub-diagonals and ku super-diagonals,
for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
trans     [rocblas_operation]
indicates whether matrix A is tranposed (conjugated) or not.
@param[in]
m         [rocblas_int]
number of rows of each matrix A_i.
@param[in]
n         [rocblas_int]
number of columns of each matrix A_i.
@param[in]
kl        [rocblas_int]
number of sub-diagonals of each A_i.
@param[in]
ku        [rocblas_int]
number of super-diagonals of each A_i.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A     device array of device pointers storing each banded matrix A_i.
Leading (kl + ku + 1) by n part of the matrix contains the coefficients
of the banded matrix. The leading diagonal resides in row (ku + 1) with
the first super-diagonal above on the RHS of row ku. The first sub-diagonal
resides below on the LHS of row ku + 2. This propagates up and down across
sub/super-diagonals.

Ex: (m = n = 7; ku = 2, kl = 2)
1 2 3 0 0 0 0             0 0 3 3 3 3 3
4 1 2 3 0 0 0             0 2 2 2 2 2 2
5 4 1 2 3 0 0    ---->    1 1 1 1 1 1 1
0 5 4 1 2 3 0             4 4 4 4 4 4 0
0 0 5 4 1 2 3             5 5 5 5 5 0 0
0 0 0 5 4 1 2             0 0 0 0 0 0 0
0 0 0 0 5 4 1             0 0 0 0 0 0 0

Note that the empty elements which do not correspond to data will not
be referenced.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. Must be >= (kl + ku + 1)
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
batch_count [rocblas_int]
specifies the number of instances in the batch.
*/
    pub fn rocblas_sgbmv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgbmv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *const *mut f64,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgbmv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgbmv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgbmv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        x: *const *const f32,
        incx: i64,
        beta: *const f32,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgbmv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        x: *const *const f64,
        incx: i64,
        beta: *const f64,
        y: *const *mut f64,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgbmv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgbmv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
gbmv_strided_batched performs one of the matrix-vector operations:

y_i := alpha*A_i*x_i    + beta*y_i,   or
y_i := alpha*A_i**T*x_i + beta*y_i,   or
y_i := alpha*A_i**H*x_i + beta*y_i,
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
m by n banded matrix with kl sub-diagonals and ku super-diagonals,
for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
trans     [rocblas_operation]
indicates whether matrix A is tranposed (conjugated) or not.
@param[in]
m         [rocblas_int]
number of rows of matrix A.
@param[in]
n         [rocblas_int]
number of columns of matrix A.
@param[in]
kl        [rocblas_int]
number of sub-diagonals of A.
@param[in]
ku        [rocblas_int]
number of super-diagonals of A.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A     device pointer to first banded matrix (A_1).
Leading (kl + ku + 1) by n part of the matrix contains the coefficients
of the banded matrix. The leading diagonal resides in row (ku + 1) with
the first super-diagonal above on the RHS of row ku. The first sub-diagonal
resides below on the LHS of row ku + 2. This propagates up and down across
sub/super-diagonals.

Ex: (m = n = 7; ku = 2, kl = 2)
1 2 3 0 0 0 0             0 0 3 3 3 3 3
4 1 2 3 0 0 0             0 2 2 2 2 2 2
5 4 1 2 3 0 0    ---->    1 1 1 1 1 1 1
0 5 4 1 2 3 0             4 4 4 4 4 4 0
0 0 5 4 1 2 3             5 5 5 5 5 0 0
0 0 0 5 4 1 2             0 0 0 0 0 0 0
0 0 0 0 5 4 1             0 0 0 0 0 0 0

Note that the empty elements which do not correspond to data will not
be referenced.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. Must be >= (kl + ku + 1).
@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).
@param[in]
x         device pointer to first vector (x_1).
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device pointer to first vector (y_1).
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
stride_y  [rocblas_stride]
stride from the start of one vector (y_i) and the next one (x_i+1).
@param[in]
batch_count [rocblas_int]
specifies the number of instances in the batch.
*/
    pub fn rocblas_sgbmv_strided_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgbmv_strided_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgbmv_strided_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgbmv_strided_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        kl: rocblas_int,
        ku: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgbmv_strided_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const f32,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgbmv_strided_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const f64,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgbmv_strided_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgbmv_strided_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        kl: i64,
        ku: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
gemv performs one of the matrix-vector operations:

y := alpha*A*x    + beta*y,   or
y := alpha*A**T*x + beta*y,   or
y := alpha*A**H*x + beta*y,
where alpha and beta are scalars, x and y are vectors and A is an
m by n matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
trans     [rocblas_operation]
indicates whether matrix A is tranposed (conjugated) or not.
@param[in]
m         [rocblas_int]
number of rows of matrix A.
@param[in]
n         [rocblas_int]
number of columns of matrix A.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A         device pointer storing matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_sgemv(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemv(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemv(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemv(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemv_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        x: *const f32,
        incx: i64,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemv_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        x: *const f64,
        incx: i64,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemv_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemv_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
gemv_batched performs a batch of matrix-vector operations:

y_i := alpha*A_i*x_i    + beta*y_i,   or
y_i := alpha*A_i**T*x_i + beta*y_i,   or
y_i := alpha*A_i**H*x_i + beta*y_i,
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
m by n matrix, for i = 1, ..., batch_count.

@param[in]
handle      [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
trans       [rocblas_operation]
indicates whether matrices A_i are tranposed (conjugated) or not.
@param[in]
m           [rocblas_int]
number of rows of each matrix A_i.
@param[in]
n           [rocblas_int]
number of columns of each matrix A_i.
@param[in]
alpha       device pointer or host pointer to scalar alpha.
@param[in]
A           device array of device pointers storing each matrix A_i.
@param[in]
lda         [rocblas_int]
specifies the leading dimension of each matrix A_i.
@param[in]
x           device array of device pointers storing each vector x_i.
@param[in]
incx        [rocblas_int]
specifies the increment for the elements of each vector x_i.
@param[in]
beta        device pointer or host pointer to scalar beta.
@param[in, out]
y           device array of device pointers storing each vector y_i.
@param[in]
incy        [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sgemv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *const *mut f64,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hshgemv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const rocblas_half,
        lda: rocblas_int,
        x: *const *const rocblas_half,
        incx: rocblas_int,
        beta: *const f32,
        y: *const *mut rocblas_half,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hssgemv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const rocblas_half,
        lda: rocblas_int,
        x: *const *const rocblas_half,
        incx: rocblas_int,
        beta: *const f32,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_tstgemv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const rocblas_bfloat16,
        lda: rocblas_int,
        x: *const *const rocblas_bfloat16,
        incx: rocblas_int,
        beta: *const f32,
        y: *const *mut rocblas_bfloat16,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_tssgemv_batched(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const rocblas_bfloat16,
        lda: rocblas_int,
        x: *const *const rocblas_bfloat16,
        incx: rocblas_int,
        beta: *const f32,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        x: *const *const f32,
        incx: i64,
        beta: *const f32,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        x: *const *const f64,
        incx: i64,
        beta: *const f64,
        y: *const *mut f64,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hshgemv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const *const rocblas_half,
        lda: i64,
        x: *const *const rocblas_half,
        incx: i64,
        beta: *const f32,
        y: *const *mut rocblas_half,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hssgemv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const *const rocblas_half,
        lda: i64,
        x: *const *const rocblas_half,
        incx: i64,
        beta: *const f32,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_tstgemv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const *const rocblas_bfloat16,
        lda: i64,
        x: *const *const rocblas_bfloat16,
        incx: i64,
        beta: *const f32,
        y: *const *mut rocblas_bfloat16,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_tssgemv_batched_64(
        handle: rocblas_handle,
        trans: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const *const rocblas_bfloat16,
        lda: i64,
        x: *const *const rocblas_bfloat16,
        incx: i64,
        beta: *const f32,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
gemv_strided_batched performs a batch of matrix-vector operations:

y_i := alpha*A_i*x_i    + beta*y_i,   or
y_i := alpha*A_i**T*x_i + beta*y_i,   or
y_i := alpha*A_i**H*x_i + beta*y_i,
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
m by n matrix, for i = 1, ..., batch_count.

@param[in]
handle      [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
transA      [rocblas_operation]
indicates whether matrices A_i are tranposed (conjugated) or not.
@param[in]
m           [rocblas_int]
number of rows of matrices A_i.
@param[in]
n           [rocblas_int]
number of columns of matrices A_i.
@param[in]
alpha       device pointer or host pointer to scalar alpha.
@param[in]
A           device pointer to the first matrix (A_1) in the batch.
@param[in]
lda         [rocblas_int]
specifies the leading dimension of matrices A_i.
@param[in]
strideA     [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).
@param[in]
x           device pointer to the first vector (x_1) in the batch.
@param[in]
incx        [rocblas_int]
specifies the increment for the elements of vectors x_i.
@param[in]
stridex     [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stride_x. However, ensure that stride_x is of appropriate size. When trans equals rocblas_operation_none
this typically means stride_x >= n * incx, otherwise stride_x >= m * incx.
@param[in]
beta        device pointer or host pointer to scalar beta.
@param[in, out]
y           device pointer to the first vector (y_1) in the batch.
@param[in]
incy        [rocblas_int]
specifies the increment for the elements of vectors y_i.
@param[in]
stridey     [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
There are no restrictions placed on stride_y. However, ensure that stride_y is of appropriate size. When trans equals rocblas_operation_none
this typically means stride_y >= m * incy, otherwise stride_y >= n * incy. stridey should be non zero.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sgemv_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemv_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemv_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemv_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hshgemv_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const rocblas_half,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const rocblas_half,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut rocblas_half,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hssgemv_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const rocblas_half,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const rocblas_half,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_tstgemv_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const rocblas_bfloat16,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const rocblas_bfloat16,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut rocblas_bfloat16,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_tssgemv_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const rocblas_bfloat16,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const rocblas_bfloat16,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemv_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        strideA: rocblas_stride,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemv_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        strideA: rocblas_stride,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemv_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        strideA: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemv_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        strideA: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hshgemv_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const rocblas_half,
        lda: i64,
        strideA: rocblas_stride,
        x: *const rocblas_half,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut rocblas_half,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hssgemv_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const rocblas_half,
        lda: i64,
        strideA: rocblas_stride,
        x: *const rocblas_half,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_tstgemv_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const rocblas_bfloat16,
        lda: i64,
        strideA: rocblas_stride,
        x: *const rocblas_bfloat16,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut rocblas_bfloat16,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_tssgemv_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const rocblas_bfloat16,
        lda: i64,
        strideA: rocblas_stride,
        x: *const rocblas_bfloat16,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hbmv performs the matrix-vector operations:

y := alpha*A*x + beta*y
where alpha and beta are scalars, x and y are n element vectors and A is an
n by n Hermitian band matrix, with k super-diagonals.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: The upper triangular part of A is being supplied.
- rocblas_fill_lower: The lower triangular part of A is being supplied.
@param[in]
n         [rocblas_int]
the order of the matrix A.
@param[in]
k         [rocblas_int]
the number of super-diagonals of the matrix A. Must be >= 0.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A         device pointer storing matrix A. Of dimension (lda, n).

if uplo == rocblas_fill_upper:
The leading (k + 1) by n part of A must contain the upper
triangular band part of the Hermitian matrix, with the leading
diagonal in row (k + 1), the first super-diagonal on the RHS
of row k, etc.
The top left k by x triangle of A will not be referenced.
Ex (upper, lda = n = 4, k = 1):
A                             Represented matrix
(0,0) (5,9) (6,8) (7,7)       (1, 0) (5, 9) (0, 0) (0, 0)
(1,0) (2,0) (3,0) (4,0)       (5,-9) (2, 0) (6, 8) (0, 0)
(0,0) (0,0) (0,0) (0,0)       (0, 0) (6,-8) (3, 0) (7, 7)
(0,0) (0,0) (0,0) (0,0)       (0, 0) (0, 0) (7,-7) (4, 0)

if uplo == rocblas_fill_lower:
The leading (k + 1) by n part of A must contain the lower
triangular band part of the Hermitian matrix, with the leading
diagonal in row (1), the first sub-diagonal on the LHS of
row 2, etc.
The bottom right k by k triangle of A will not be referenced.
Ex (lower, lda = 2, n = 4, k = 1):
A                               Represented matrix
(1,0) (2,0) (3,0) (4,0)         (1, 0) (5,-9) (0, 0) (0, 0)
(5,9) (6,8) (7,7) (0,0)         (5, 9) (2, 0) (6,-8) (0, 0)
(0, 0) (6, 8) (3, 0) (7,-7)
(0, 0) (0, 0) (7, 7) (4, 0)

As a Hermitian matrix, the imaginary part of the main diagonal
of A will not be referenced and is assumed to be == 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. must be >= k + 1.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_chbmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhbmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chbmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhbmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hbmv_batched performs one of the matrix-vector operations:

y_i := alpha*A_i*x_i + beta*y_i
where alpha and beta are scalars, x_i and y_i are n element vectors and A_i is an
n by n Hermitian band matrix with k super-diagonals, for each batch in i = [1, batch_count].

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: The upper triangular part of each A_i is being supplied.
- rocblas_fill_lower: The lower triangular part of each A_i is being supplied.
@param[in]
n         [rocblas_int]
the order of each matrix A_i.
@param[in]
k         [rocblas_int]
the number of super-diagonals of each matrix A_i. Must be >= 0.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A         device array of device pointers storing each matrix_i A of dimension (lda, n).

if uplo == rocblas_fill_upper:
The leading (k + 1) by n part of each A_i must contain the upper
triangular band part of the Hermitian matrix, with the leading
diagonal in row (k + 1), the first super-diagonal on the RHS
of row k, etc.
The top left k by x triangle of each A_i will not be referenced.
Ex (upper, lda = n = 4, k = 1):
A                             Represented matrix
(0,0) (5,9) (6,8) (7,7)       (1, 0) (5, 9) (0, 0) (0, 0)
(1,0) (2,0) (3,0) (4,0)       (5,-9) (2, 0) (6, 8) (0, 0)
(0,0) (0,0) (0,0) (0,0)       (0, 0) (6,-8) (3, 0) (7, 7)
(0,0) (0,0) (0,0) (0,0)       (0, 0) (0, 0) (7,-7) (4, 0)

if uplo == rocblas_fill_lower:
The leading (k + 1) by n part of each A_i must contain the lower
triangular band part of the Hermitian matrix, with the leading
diagonal in row (1), the first sub-diagonal on the LHS of
row 2, etc.
The bottom right k by k triangle of each A_i will not be referenced.
Ex (lower, lda = 2, n = 4, k = 1):
A                               Represented matrix
(1,0) (2,0) (3,0) (4,0)         (1, 0) (5,-9) (0, 0) (0, 0)
(5,9) (6,8) (7,7) (0,0)         (5, 9) (2, 0) (6,-8) (0, 0)
(0, 0) (6, 8) (3, 0) (7,-7)
(0, 0) (0, 0) (7, 7) (4, 0)

As a Hermitian matrix, the imaginary part of the main diagonal
of each A_i will not be referenced and is assumed to be == 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. must be >= max(1, n).
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_chbmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhbmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chbmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhbmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hbmv_strided_batched performs one of the matrix-vector operations:

y_i := alpha*A_i*x_i + beta*y_i
where alpha and beta are scalars, x_i and y_i are n element vectors and A_i is an
n by n Hermitian band matrix with k super-diagonals, for each batch in i = [1, batch_count].

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: The upper triangular part of each A_i is being supplied.
- rocblas_fill_lower: The lower triangular part of each A_i is being supplied.
@param[in]
n         [rocblas_int]
the order of each matrix A_i.
@param[in]
k         [rocblas_int]
the number of super-diagonals of each matrix A_i. Must be >= 0.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A         device array pointing to the first matrix A_1. Each A_i is of dimension (lda, n).

if uplo == rocblas_fill_upper:
The leading (k + 1) by n part of each A_i must contain the upper
triangular band part of the Hermitian matrix, with the leading
diagonal in row (k + 1), the first super-diagonal on the RHS
of row k, etc.
The top left k by x triangle of each A_i will not be referenced.
Ex (upper, lda = n = 4, k = 1):
A                             Represented matrix
(0,0) (5,9) (6,8) (7,7)       (1, 0) (5, 9) (0, 0) (0, 0)
(1,0) (2,0) (3,0) (4,0)       (5,-9) (2, 0) (6, 8) (0, 0)
(0,0) (0,0) (0,0) (0,0)       (0, 0) (6,-8) (3, 0) (7, 7)
(0,0) (0,0) (0,0) (0,0)       (0, 0) (0, 0) (7,-7) (4, 0)

if uplo == rocblas_fill_lower:
The leading (k + 1) by n part of each A_i must contain the lower
triangular band part of the Hermitian matrix, with the leading
diagonal in row (1), the first sub-diagonal on the LHS of
row 2, etc.
The bottom right k by k triangle of each A_i will not be referenced.
Ex (lower, lda = 2, n = 4, k = 1):
A                               Represented matrix
(1,0) (2,0) (3,0) (4,0)         (1, 0) (5,-9) (0, 0) (0, 0)
(5,9) (6,8) (7,7) (0,0)         (5, 9) (2, 0) (6,-8) (0, 0)
(0, 0) (6, 8) (3, 0) (7,-7)
(0, 0) (0, 0) (7, 7) (4, 0)

As a Hermitian matrix, the imaginary part of the main diagonal
of each A_i will not be referenced and is assumed to be == 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. must be >= max(1, n).
@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).
@param[in]
x         device array pointing to the first vector y_1.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device array pointing to the first vector y_1.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
stride_y  [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_chbmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhbmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chbmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhbmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hemv performs one of the matrix-vector operations:

y := alpha*A*x + beta*y
where alpha and beta are scalars, x and y are n element vectors and A is an
n by n Hermitian matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: the upper triangular part of the Hermitian matrix A is supplied.
- rocblas_fill_lower: the lower triangular part of the Hermitian matrix A is supplied.
@param[in]
n         [rocblas_int]
the order of the matrix A.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A         device pointer storing matrix A. Of dimension (lda, n).

if uplo == rocblas_fill_upper:
The upper triangular part of A must contain
the upper triangular part of a Hermitian matrix. The lower
triangular part of A will not be referenced.

if uplo == rocblas_fill_lower:
The lower triangular part of A must contain
the lower triangular part of a Hermitian matrix. The upper
triangular part of A will not be referenced.
As a Hermitian matrix, the imaginary part of the main diagonal
of A will not be referenced and is assumed to be == 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. must be >= max(1, n).
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_chemv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chemv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hemv_batched performs one of the matrix-vector operations:

y_i := alpha*A_i*x_i + beta*y_i
where alpha and beta are scalars, x_i and y_i are n element vectors and A_i is an
n by n Hermitian matrix, for each batch in i = [1, batch_count].

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: the upper triangular part of the Hermitian matrix A is supplied.
- rocblas_fill_lower: the lower triangular part of the Hermitian matrix A is supplied.
@param[in]
n         [rocblas_int]
the order of each matrix A_i.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A         device array of device pointers storing each matrix A_i of dimension (lda, n).

if uplo == rocblas_fill_upper:
The upper triangular part of each A_i must contain
the upper triangular part of a Hermitian matrix. The lower
triangular part of each A_i will not be referenced.

if uplo == rocblas_fill_lower:
The lower triangular part of each A_i must contain
the lower triangular part of a Hermitian matrix. The upper
triangular part of each A_i will not be referenced.
As a Hermitian matrix, the imaginary part of the main diagonal
of each A_i will not be referenced and is assumed to be == 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. must be >= max(1, n).
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_chemv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chemv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hemv_strided_batched performs one of the matrix-vector operations:

y_i := alpha*A_i*x_i + beta*y_i
where alpha and beta are scalars, x_i and y_i are n element vectors and A_i is an
n by n Hermitian matrix, for each batch in i = [1, batch_count].

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: the upper triangular part of the Hermitian matrix A is supplied.
- rocblas_fill_lower: the lower triangular part of the Hermitian matrix A is supplied.
@param[in]
n         [rocblas_int]
the order of each matrix A_i.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
A         device array of device pointers storing each matrix A_i of dimension (lda, n).

if uplo == rocblas_fill_upper:
The upper triangular part of each A_i must contain
the upper triangular part of a Hermitian matrix. The lower
triangular part of each A_i will not be referenced.

if uplo == rocblas_fill_lower:
The lower triangular part of each A_i must contain
the lower triangular part of a Hermitian matrix. The upper
triangular part of each A_i will not be referenced.
As a Hermitian matrix, the imaginary part of the main diagonal
of each A_i will not be referenced and is assumed to be == 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. must be >= max(1, n).
@param[in]
stride_A    [rocblas_stride]
stride from the start of one (A_i) to the next (A_i+1).
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
stride_y  [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_chemv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chemv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
her performs the matrix-vector operations:

A := A + alpha*x*x**H
where alpha is a real scalar, x is a vector, and A is an
n by n Hermitian matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of A is supplied in A.
- rocblas_fill_lower: The lower triangular part of A is supplied in A.
@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in, out]
A         device pointer storing the specified triangular portion of the Hermitian matrix A. Of size (lda * n).

if uplo == rocblas_fill_upper:
The upper triangular portion of the Hermitian matrix A is supplied.
The lower triangluar portion will not be touched.

if uplo == rocblas_fill_lower:
The lower triangular portion of the Hermitian matrix A is supplied.
The upper triangular portion will not be touched.
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. Must be at least max(1, n).*/
    pub fn rocblas_cher(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cher_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const rocblas_float_complex,
        incx: i64,
        A: *mut rocblas_float_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const rocblas_double_complex,
        incx: i64,
        A: *mut rocblas_double_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
her_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*x_i**H
where alpha is a real scalar, x_i is a vector, and A_i is an
n by n symmetric matrix, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in A.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in A.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in, out]
A         device array of device pointers storing the specified triangular portion of
each Hermitian matrix A_i of at least size ((n * (n + 1)) / 2). Array is of at least size batch_count.

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The lower triangular portion of each A_i will not be touched.
if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The upper triangular portion of each A_i will not be touched.
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. Must be at least max(1, n).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_cher_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        A: *const *mut rocblas_float_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        A: *const *mut rocblas_double_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cher_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const *const rocblas_float_complex,
        incx: i64,
        A: *const *mut rocblas_float_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        A: *const *mut rocblas_double_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
her_strided_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*x_i**H
where alpha is a real scalar, x_i is a vector, and A_i is an
n by n Hermitian matrix, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in A.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in A.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer pointing to the first vector (x_1).
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in, out]
A         device array of device pointers storing the specified triangular portion of
each Hermitian matrix A_i. Points to the first matrix (A_1).

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The lower triangular portion of each A_i will not be touched.

if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The upper triangular portion of each A_i will not be touched.
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
stride_A    [rocblas_stride]
stride from the start of one (A_i) and the next (A_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_cher_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cher_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
her2 performs the matrix-vector operations:

A := A + alpha*x*y**H + conj(alpha)*y*x**H
where alpha is a complex scalar, x and y are vectors, and A is an
n by n Hermitian matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of A is supplied.
- rocblas_fill_lower: The lower triangular part of A is supplied.
@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
A         device pointer storing the specified triangular portion of
the Hermitian matrix A. Of size (lda, n).

if uplo == rocblas_fill_upper:
The upper triangular portion of the Hermitian matrix A is supplied.
The lower triangular portion of A will not be touched.

if uplo == rocblas_fill_lower:
The lower triangular portion of the Hermitian matrix A is supplied.
The upper triangular portion of A will not be touched.
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. Must be at least max(lda, 1).*/
    pub fn rocblas_cher2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cher2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        y: *const rocblas_float_complex,
        incy: i64,
        A: *mut rocblas_float_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        y: *const rocblas_double_complex,
        incy: i64,
        A: *mut rocblas_double_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
her2_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*y_i**H + conj(alpha)*y_i*x_i**H
where alpha is a complex scalar, x_i and y_i are vectors, and A_i is an
n by n Hermitian matrix for each batch in i = [1, batch_count].

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in, out]
A         device array of device pointers storing the specified triangular portion of
each Hermitian matrix A_i of size (lda, n).

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The lower triangular portion of each A_i will not be touched.

if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The upper triangular portion of each A_i will not be touched.
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. Must be at least max(lda, 1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_cher2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const *const rocblas_float_complex,
        incy: rocblas_int,
        A: *const *mut rocblas_float_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const *const rocblas_double_complex,
        incy: rocblas_int,
        A: *const *mut rocblas_double_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cher2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: i64,
        y: *const *const rocblas_float_complex,
        incy: i64,
        A: *const *mut rocblas_float_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: i64,
        y: *const *const rocblas_double_complex,
        incy: i64,
        A: *const *mut rocblas_double_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
her2_strided_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*y_i**H + conj(alpha)*y_i*x_i**H
where alpha is a complex scalar, x_i and y_i are vectors, and A_i is an
n by n Hermitian matrix for each batch in i = [1, batch_count].

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer pointing to the first vector x_1.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
specifies the stride between the beginning of one vector (x_i) and the next (x_i+1).
@param[in]
y         device pointer pointing to the first vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
stride_y  [rocblas_stride]
specifies the stride between the beginning of one vector (y_i) and the next (y_i+1).
@param[in, out]
A         device pointer pointing to the first matrix (A_1). Stores the specified triangular portion of
each Hermitian matrix A_i.

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The lower triangular portion of each A_i will not be touched.

if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The upper triangular portion of each A_i will not be touched.
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. Must be at least max(lda, 1).
@param[in]
stride_A  [rocblas_stride]
specifies the stride between the beginning of one matrix (A_i) and the next (A_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_cher2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cher2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: i64,
        stride_y: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: i64,
        stride_y: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hpmv performs the matrix-vector operation:

y := alpha*A*x + beta*y
where alpha and beta are scalars, x and y are n element vectors and A is an
n by n Hermitian matrix, supplied in packed form (see description below).

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: the upper triangular part of the Hermitian matrix A is supplied in AP.
- rocblas_fill_lower: the lower triangular part of the Hermitian matrix A is supplied in AP.
@param[in]
n         [rocblas_int]
the order of the matrix A. Must be >= 0.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
AP        device pointer storing the packed version of the specified triangular portion of
the Hermitian matrix A. Of at least size ((n * (n + 1)) / 2).

if uplo == rocblas_fill_upper:
The upper triangular portion of the Hermitian matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 3)
(1, 0) (2, 1) (3, 2)
(2,-1) (4, 0) (5,-1) ---> [(1,0),(2,1),(4,0),(3,2),(5,-1),(6,0)]
(3,-2) (5, 1) (6, 0)

if uplo == rocblas_fill_lower:
The lower triangular portion of the Hermitian matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 3)
(1, 0) (2, 1) (3, 2)
(2,-1) (4, 0) (5,-1) ---> [(1,0),(2,-1),(3,-2),(4,0),(5,1),(6,0)]
(3,-2) (5, 1) (6, 0)
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_chpmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        AP: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        AP: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chpmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        AP: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        AP: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hpmv_batched performs the matrix-vector operation:

y_i := alpha*A_i*x_i + beta*y_i
where alpha and beta are scalars, x_i and y_i are n element vectors and A_i is an
n by n Hermitian matrix, supplied in packed form (see description below),
for each batch in i = [1, batch_count].

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: the upper triangular part of each Hermitian matrix A_i is supplied in AP.
- rocblas_fill_lower: the lower triangular part of each Hermitian matrix A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the order of each matrix A_i.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
AP      device pointer of device pointers storing the packed version of the specified triangular
portion of each Hermitian matrix A_i. Each A_i is of at least size ((n * (n + 1)) / 2).

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that each AP_i contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 3)
(1, 0) (2, 1) (3, 2)
(2,-1) (4, 0) (5,-1) ---> [(1,0),(2,1),(4,0),(3,2),(5,-1),(6,0)]
(3,-2) (5, 1) (6, 0)

if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that each AP_i contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 3)
(1, 0) (2, 1) (3, 2)
(2,-1) (4, 0) (5,-1) ---> [(1,0),(2,-1),(3,-2),(4,0),(5,1),(6,0)]
(3,-2) (5, 1) (6, 0)
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_chpmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        AP: *const *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        AP: *const *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chpmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        AP: *const *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        AP: *const *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hpmv_strided_batched performs the matrix-vector operation:

y_i := alpha*A_i*x_i + beta*y_i
where alpha and beta are scalars, x_i and y_i are n element vectors and A_i is an
n by n Hermitian matrix, supplied in packed form (see description below),
for each batch in i = [1, batch_count].

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: the upper triangular part of each Hermitian matrix A_i is supplied in AP.
- rocblas_fill_lower: the lower triangular part of each Hermitian matrix A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the order of each matrix A_i.
@param[in]
alpha     device pointer or host pointer to scalar alpha.
@param[in]
AP        device pointer pointing to the beginning of the first matrix (AP_1). Stores the packed
version of the specified triangular portion of each Hermitian matrix AP_i of size ((n * (n + 1)) / 2).

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that each AP_i contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 3)
(1, 0) (2, 1) (3, 2)
(2,-1) (4, 0) (5,-1) ---> [(1,0),(2,1),(4,0),(3,2),(5,-1),(6,0)]
(3,-2) (5, 1) (6, 0)

if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that each AP_i contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 3)
(1, 0) (2, 1) (3, 2)
(2,-1) (4, 0) (5,-1) ---> [(1,0),(2,-1),(3,-2),(4,0),(5,1),(6,0)]
(3,-2) (5, 1) (6, 0)
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (AP_i) and the next one (AP_i+1).
@param[in]
x         device array pointing to the beginning of the first vector (x_1).
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[in, out]
y         device array pointing to the beginning of the first vector (y_1).
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
stride_y  [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_chpmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        AP: *const rocblas_float_complex,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        AP: *const rocblas_double_complex,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chpmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        AP: *const rocblas_float_complex,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        AP: *const rocblas_double_complex,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hpr performs the matrix-vector operations:

A := A + alpha*x*x**H
where alpha is a real scalar, x is a vector, and A is an
n by n Hermitian matrix, supplied in packed form.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of A is supplied in AP.
- rocblas_fill_lower: The lower triangular part of A is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in, out]
AP        device pointer storing the packed version of the specified triangular portion of
the Hermitian matrix A. Of at least size ((n * (n + 1)) / 2).

if uplo == rocblas_fill_upper:
The upper triangular portion of the Hermitian matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,1),(3,0),(4,9),(5,3),(6,0)]
(4,-9) (5,-3) (6,0)

if uplo == rocblas_fill_lower:
The lower triangular portion of the Hermitian matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,-1),(4,-9),(3,0),(5,-3),(6,0)]
(4,-9) (5,-3) (6,0)
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.*/
    pub fn rocblas_chpr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        AP: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        AP: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chpr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const rocblas_float_complex,
        incx: i64,
        AP: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const rocblas_double_complex,
        incx: i64,
        AP: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hpr_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*x_i**H
where alpha is a real scalar, x_i is a vector, and A_i is an
n by n symmetric matrix, supplied in packed form, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in AP.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in, out]
AP        device array of device pointers storing the packed version of the specified triangular portion of
each Hermitian matrix A_i of at least size ((n * (n + 1)) / 2). Array is of at least size batch_count.

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,1),(3,0),(4,9),(5,3),(6,0)]
(4,-9) (5,-3) (6,0)

if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,-1),(4,-9),(3,0),(5,-3),(6,0)]
(4,-9) (5,-3) (6,0)
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_chpr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        AP: *const *mut rocblas_float_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        AP: *const *mut rocblas_double_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chpr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const *const rocblas_float_complex,
        incx: i64,
        AP: *const *mut rocblas_float_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        AP: *const *mut rocblas_double_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hpr_strided_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*x_i**H
where alpha is a real scalar, x_i is a vector, and A_i is an
n by n symmetric matrix, supplied in packed form, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in AP.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer pointing to the first vector (x_1).
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in, out]
AP        device array of device pointers storing the packed version of the specified triangular portion of
each Hermitian matrix A_i. Points to the first matrix (A_1).

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,1),(3,0),(4,9),(5,3),(6,0)]
(4,-9) (5,-3) (6,0)

if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,-1),(4,-9),(3,0),(5,-3),(6,0)]
(4,-9) (5,-3) (6,0)
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
stride_A    [rocblas_stride]
stride from the start of one (A_i) and the next (A_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_chpr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        AP: *mut rocblas_float_complex,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        AP: *mut rocblas_double_complex,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chpr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        AP: *mut rocblas_float_complex,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        AP: *mut rocblas_double_complex,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hpr2 performs the matrix-vector operations:

A := A + alpha*x*y**H + conj(alpha)*y*x**H
where alpha is a complex scalar, x and y are vectors, and A is an
n by n Hermitian matrix, supplied in packed form.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of A is supplied in AP.
- rocblas_fill_lower: The lower triangular part of A is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
AP        device pointer storing the packed version of the specified triangular portion of
the Hermitian matrix A. Of at least size ((n * (n + 1)) / 2).

if uplo == rocblas_fill_upper:
The upper triangular portion of the Hermitian matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,1),(3,0),(4,9),(5,3),(6,0)]
(4,-9) (5,-3) (6,0)

if uplo == rocblas_fill_lower:
The lower triangular portion of the Hermitian matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,-1),(4,-9),(3,0),(5,-3),(6,0)]
(4,-9) (5,-3) (6,0)
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.*/
    pub fn rocblas_chpr2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        AP: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        AP: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chpr2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        y: *const rocblas_float_complex,
        incy: i64,
        AP: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        y: *const rocblas_double_complex,
        incy: i64,
        AP: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hpr2_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*y_i**H + conj(alpha)*y_i*x_i**H
where alpha is a complex scalar, x_i and y_i are vectors, and A_i is an
n by n symmetric matrix, supplied in packed form, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in AP.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in, out]
AP        device array of device pointers storing the packed version of the specified triangular portion of
each Hermitian matrix A_i of at least size ((n * (n + 1)) / 2). Array is of at least size batch_count.

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,1),(3,0),(4,9),(5,3),(6,0)]
(4,-9) (5,-3) (6,0)

if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) --> [(1,0),(2,-1),(4,-9),(3,0),(5,-3),(6,0)]
(4,-9) (5,-3) (6,0)
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_chpr2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const *const rocblas_float_complex,
        incy: rocblas_int,
        AP: *const *mut rocblas_float_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const *const rocblas_double_complex,
        incy: rocblas_int,
        AP: *const *mut rocblas_double_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chpr2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: i64,
        y: *const *const rocblas_float_complex,
        incy: i64,
        AP: *const *mut rocblas_float_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: i64,
        y: *const *const rocblas_double_complex,
        incy: i64,
        AP: *const *mut rocblas_double_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
hpr2_strided_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*y_i**H + conj(alpha)*y_i*x_i**H
where alpha is a complex scalar, x_i and y_i are vectors, and A_i is an
n by n symmetric matrix, supplied in packed form, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in AP.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer pointing to the first vector (x_1).
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in]
y         device pointer pointing to the first vector (y_1).
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
stride_y  [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
@param[in, out]
AP        device array of device pointers storing the packed version of the specified triangular portion of
each Hermitian matrix A_i. Points to the first matrix (A_1).

if uplo == rocblas_fill_upper:
The upper triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,1),(3,0),(4,9),(5,3),(6,0)]
(4,-9) (5,-3) (6,0)

if uplo == rocblas_fill_lower:
The lower triangular portion of each Hermitian matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 3)
(1, 0) (2, 1) (4,9)
(2,-1) (3, 0) (5,3) ---> [(1,0),(2,-1),(4,-9),(3,0),(5,-3),(6,0)]
(4,-9) (5,-3) (6,0)
Note that the imaginary part of the diagonal elements are not accessed
and are assumed to be 0.
@param[in]
stride_A    [rocblas_stride]
stride from the start of one (A_i) and the next (A_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_chpr2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        AP: *mut rocblas_float_complex,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        AP: *mut rocblas_double_complex,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chpr2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: i64,
        stride_y: rocblas_stride,
        AP: *mut rocblas_float_complex,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhpr2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: i64,
        stride_y: rocblas_stride,
        AP: *mut rocblas_double_complex,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
trmv performs one of the matrix-vector operations:

x = A*x or
x = A**T*x or
x = A**H*x
where x is an n element vector and A is an n by n unit, or non-unit, upper or lower triangular matrix.
The vector x is overwritten.
@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:   op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of A. n >= 0.

@param[in]
A         device pointer storing matrix A, of dimension ( lda, n ). If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains the matrix A, otherwise the lower triangular part of the leading n-by-n array contains the matrix A.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. lda must be at least max( 1, n ).

@param[in, out]
x         device pointer storing vector x. On exit, x is overwritten with the transformed vector x.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
*/
    pub fn rocblas_strmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f32,
        lda: i64,
        x: *mut f32,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f64,
        lda: i64,
        x: *mut f64,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
trmv_batched performs one of the matrix-vector operations:

x_i = A_i*x_i or
x_i = A_i**T*x_i or
x_i = A_i**H*x_i, 0 < i < batch_count
where x_i is an n element vector and A_i is an n by n (unit, or non-unit, upper or lower triangular matrix)
The vectors x_i are overwritten.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix.
- rocblas_fill_lower:  A_i is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:   op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A_i is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of matrices A_i. n >= 0.

@param[in]
A         device pointer to an array of device pointers to the A_i matrices, of dimension ( lda, n ). If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains the matrix A_i, otherwise the lower triangular part of the leading n-by-n array contains the matrix A_i.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of A_i. lda must be at least max( 1, n ).

@param[in, out]
x         device pointer to an array of device pointers to the x_i vectors. On exit, each x_i is overwritten with the transformed vector x_i.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of vectors x_i.

@param[in]
batch_count [rocblas_int]
The number of batched matrices/vectors.

*/
    pub fn rocblas_strmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const f32,
        lda: rocblas_int,
        x: *const *mut f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const f64,
        lda: rocblas_int,
        x: *const *mut f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const f32,
        lda: i64,
        x: *const *mut f32,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const f64,
        lda: i64,
        x: *const *mut f64,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
trmv_strided_batched performs one of the matrix-vector operations:

x_i = A_i*x_i or
x_i = A_i**T*x_i, or
x_i = A_i**H*x_i, 0 < i < batch_count
where x_i is an n element vector and A_i is an n by n (unit, or non-unit, upper or lower triangular matrix)
with strides specifying how to retrieve $x_i$ (resp. $A_i$) from $x_{i-1}$ (resp. $A_i$).

The vectors x_i are overwritten.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix.
- rocblas_fill_lower:  A_i is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:   op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A_i is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of matrices A_i. n >= 0.

@param[in]
A         device pointer to the matrix A_1 of the batch, of dimension ( lda, n ). If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains the matrix A_i, otherwise the lower triangular part of the leading n-by-n array contains the matrix A_i.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of A_i. lda must be at least max( 1, n ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one A_i matrix to the next A_{i + 1}.

@param[in, out]
x         device pointer to the vector x_1 of the batch. On exit, each x_i is overwritten with the transformed vector x_i.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of one vector x.

@param[in]
stride_x  [rocblas_stride]
stride from the start of one x_i vector to the next x_{i + 1}.

@param[in]
batch_count [rocblas_int]
The number of batched matrices/vectors.

*/
    pub fn rocblas_strmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tpmv performs one of the matrix-vector operations:

x = A*x or
x = A**T*x or
x = A**H*x
where x is an n element vector and A is an n by n unit, or non-unit,
upper or lower triangular matrix, supplied in the pack form.
The vector x is overwritten.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:   op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A is not assumed to be unit triangular.

@param[in]
n       [rocblas_int]
n specifies the number of rows of A. n >= 0.

@param[in]
A       device pointer storing matrix A,
of dimension at leat ( n * ( n + 1 ) / 2 ).
- Before entry with uplo = rocblas_fill_upper, the array A
must contain the upper triangular matrix packed sequentially,
column by column, so that
A[0] contains a_{0,0}, A[1] and A[2] contain
a_{0,1} and a_{1, 1}, respectively, and so on.

- Before entry with uplo = rocblas_fill_lower, the array A
must contain the lower triangular matrix packed sequentially,
column by column, so that
A[0] contains a_{0,0}, A[1] and A[2] contain
a_{1,0} and a_{2,0}, respectively, and so on.

Note that when DIAG = rocblas_diagonal_unit, the diagonal elements of A are
not referenced, but are assumed to be unity.

@param[in, out]
x      device pointer storing vector x. On exit, x is overwritten with the transformed vector x.

@param[in]
incx    [rocblas_int]
specifies the increment for the elements of x. incx must not be zero.
*/
    pub fn rocblas_stpmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f32,
        x: *mut f32,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f64,
        x: *mut f64,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stpmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f32,
        x: *mut f32,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f64,
        x: *mut f64,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_float_complex,
        x: *mut rocblas_float_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_double_complex,
        x: *mut rocblas_double_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tpmv_batched performs one of the matrix-vector operations:

x_i = A_i*x_i or
x_i = A_i**T*x_i or
x_i = A_i**H*x_i, 0 < i < batch_count
where x_i is an n element vector and A_i is an n by n (unit, or non-unit, upper or lower triangular matrix)
The vectors x_i are overwritten.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix.
- rocblas_fill_lower:  A_i is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:   op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A_i is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of matrices A_i. n >= 0.

@param[in]
A         device pointer to an array of device pointers to the A_i matrices, of dimension ( lda, n ). If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains the matrix A_i, otherwise the lower triangular part of the leading n-by-n array contains the matrix A_i.

@param[in, out]
x         device pointer to an array of device pointers to the x_i vectors. On exit, each x_i is overwritten with the transformed vector x_i.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of vectors x_i.

@param[in]
batch_count [rocblas_int]
The number of batched matrices/vectors.

*/
    pub fn rocblas_stpmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const f32,
        x: *const *mut f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const f64,
        x: *const *mut f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const rocblas_float_complex,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const rocblas_double_complex,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stpmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const f32,
        x: *const *mut f32,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const f64,
        x: *const *mut f64,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const rocblas_float_complex,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const rocblas_double_complex,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tpmv_strided_batched performs one of the matrix-vector operations:

x_i = A_i*x_i or
x_i = A_i**T*x_i or
x_i = A_i**H*x_i, 0 < i < batch_count
where x_i is an n element vector and A_i is an n by n (unit, or non-unit, upper or lower triangular matrix)
with strides specifying how to retrieve $x_i$ (resp. $A_i$) from $x_{i-1}$ (resp. $A_i$).
The vectors x_i are overwritten.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix.
- rocblas_fill_lower:  A_i is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:   op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A_i is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of matrices A_i. n >= 0.

@param[in]
A       device pointer to the matrix A_1 of the batch, of dimension ( lda, n ). If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains the matrix A_i, otherwise the lower triangular part of the leading n-by-n array contains the matrix A_i.

@param[in]
stride_A  [rocblas_stride]
stride from the start of one A_i matrix to the next A_{i + 1}.

@param[in, out]
x       device pointer to the vector x_1 of the batch. On exit, each x_i is overwritten with the transformed vector x_i.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of one vector x.

@param[in]
stride_x  [rocblas_stride]
stride from the start of one x_i vector to the next x_{i + 1}.

@param[in]
batch_count [rocblas_int]
The number of batched matrices/vectors.

*/
    pub fn rocblas_stpmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f32,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f64,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stpmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f32,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f64,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_float_complex,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_double_complex,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tbmv performs one of the matrix-vector operations:

x := A*x      or
x := A**T*x   or
x := A**H*x,
x is a vectors and A is a banded n by n matrix (see description below).

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: A is an upper banded triangular matrix.
- rocblas_fill_lower: A is a  lower banded triangular matrix.
@param[in]
trans     [rocblas_operation]
indicates whether matrix A is tranposed (conjugated) or not.
@param[in]
diag      [rocblas_diagonal]
- rocblas_diagonal_unit: The main diagonal of A is assumed to consist of only
1's and is not referenced.
- rocblas_diagonal_non_unit: No assumptions are made of A's main diagonal.
@param[in]
n         [rocblas_int]
the number of rows and columns of the matrix represented by A.
@param[in]
k         [rocblas_int]

if uplo == rocblas_fill_upper, k specifies the number of super-diagonals
of the matrix A.

if uplo == rocblas_fill_lower, k specifies the number of sub-diagonals
of the matrix A.
k must satisfy k > 0 && k < lda.
@param[in]
A         device pointer storing banded triangular matrix A.

if uplo == rocblas_fill_upper:
The matrix represented is an upper banded triangular matrix
with the main diagonal and k super-diagonals, everything
else can be assumed to be 0.
The matrix is compacted so that the main diagonal resides on the k'th
row, the first super diagonal resides on the RHS of the k-1'th row, etc,
with the k'th diagonal on the RHS of the 0'th row.
Ex: (rocblas_fill_upper; n = 5; k = 2)
1 6 9 0 0              0 0 9 8 7
0 2 7 8 0              0 6 7 8 9
0 0 3 8 7     ---->    1 2 3 4 5
0 0 0 4 9              0 0 0 0 0
0 0 0 0 5              0 0 0 0 0

if uplo == rocblas_fill_lower:
The matrix represnted is a lower banded triangular matrix
with the main diagonal and k sub-diagonals, everything else can be
assumed to be 0.
The matrix is compacted so that the main diagonal resides on the 0'th row,
working up to the k'th diagonal residing on the LHS of the k'th row.
Ex: (rocblas_fill_lower; n = 5; k = 2)
1 0 0 0 0              1 2 3 4 5
6 2 0 0 0              6 7 8 9 0
9 7 3 0 0     ---->    9 8 7 0 0
0 8 8 4 0              0 0 0 0 0
0 0 7 9 5              0 0 0 0 0
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. lda must satisfy lda > k.
@param[in, out]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
*/
    pub fn rocblas_stbmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stbmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const f32,
        lda: i64,
        x: *mut f32,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const f64,
        lda: i64,
        x: *mut f64,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tbmv_batched performs one of the matrix-vector operations:

x_i := A_i*x_i      or
x_i := A_i**T*x_i   or
x_i := A_i**H*x_i,
where (A_i, x_i) is the i-th instance of the batch.
x_i is a vector and A_i is an n by n matrix, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: each A_i is an upper banded triangular matrix.
- rocblas_fill_lower: each A_i is a  lower banded triangular matrix.
@param[in]
trans     [rocblas_operation]
indicates whether each matrix A_i is tranposed (conjugated) or not.
@param[in]
diag      [rocblas_diagonal]
- rocblas_diagonal_unit: The main diagonal of each A_i is assumed to consist of only
1's and is not referenced.
- rocblas_diagonal_non_unit: No assumptions are made of each A_i's main diagonal.
@param[in]
n         [rocblas_int]
the number of rows and columns of the matrix represented by each A_i.
@param[in]
k         [rocblas_int]

if uplo == rocblas_fill_upper, k specifies the number of super-diagonals
of each matrix A_i.

if uplo == rocblas_fill_lower, k specifies the number of sub-diagonals
of each matrix A_i.
k must satisfy k > 0 && k < lda.
@param[in]
A         device array of device pointers storing each banded triangular matrix A_i.

if uplo == rocblas_fill_upper:
The matrix represented is an upper banded triangular matrix
with the main diagonal and k super-diagonals, everything
else can be assumed to be 0.
The matrix is compacted so that the main diagonal resides on the k'th
row, the first super diagonal resides on the RHS of the k-1'th row, etc,
with the k'th diagonal on the RHS of the 0'th row.
Ex: (rocblas_fill_upper; n = 5; k = 2)
1 6 9 0 0              0 0 9 8 7
0 2 7 8 0              0 6 7 8 9
0 0 3 8 7     ---->    1 2 3 4 5
0 0 0 4 9              0 0 0 0 0
0 0 0 0 5              0 0 0 0 0

if uplo == rocblas_fill_lower:
The matrix represnted is a lower banded triangular matrix
with the main diagonal and k sub-diagonals, everything else can be
assumed to be 0.
The matrix is compacted so that the main diagonal resides on the 0'th row,
working up to the k'th diagonal residing on the LHS of the k'th row.
Ex: (rocblas_fill_lower; n = 5; k = 2)
1 0 0 0 0              1 2 3 4 5
6 2 0 0 0              6 7 8 9 0
9 7 3 0 0     ---->    9 8 7 0 0
0 8 8 4 0              0 0 0 0 0
0 0 7 9 5              0 0 0 0 0
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. lda must satisfy lda > k.
@param[in, out]
x         device array of device pointer storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_stbmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const *const f32,
        lda: rocblas_int,
        x: *const *mut f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const *const f64,
        lda: rocblas_int,
        x: *const *mut f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stbmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const *const f32,
        lda: i64,
        x: *const *mut f32,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const *const f64,
        lda: i64,
        x: *const *mut f64,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tbmv_strided_batched performs one of the matrix-vector operations:

x_i := A_i*x_i      or
x_i := A_i**T*x_i   or
x_i := A_i**H*x_i,
where (A_i, x_i) is the i-th instance of the batch.
x_i is a vector and A_i is an n by n matrix, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
- rocblas_fill_upper: each A_i is an upper banded triangular matrix.
- rocblas_fill_lower: each A_i is a  lower banded triangular matrix.
@param[in]
trans     [rocblas_operation]
indicates whether each matrix A_i is tranposed (conjugated) or not.
@param[in]
diag      [rocblas_diagonal]
- rocblas_diagonal_unit: The main diagonal of each A_i is assumed to consist of only
1's and is not referenced.
- rocblas_diagonal_non_unit: No assumptions are made of each A_i's main diagonal.
@param[in]
n         [rocblas_int]
the number of rows and columns of the matrix represented by each A_i.
@param[in]
k         [rocblas_int]

if uplo == rocblas_fill_upper, k specifies the number of super-diagonals
of each matrix A_i.

if uplo == rocblas_fill_lower, k specifies the number of sub-diagonals
of each matrix A_i.
k must satisfy k > 0 && k < lda.
@param[in]
A         device array to the first matrix A_i of the batch. Stores each banded triangular matrix A_i.

if uplo == rocblas_fill_upper:
The matrix represented is an upper banded triangular matrix
with the main diagonal and k super-diagonals, everything
else can be assumed to be 0.
The matrix is compacted so that the main diagonal resides on the k'th
row, the first super diagonal resides on the RHS of the k-1'th row, etc,
with the k'th diagonal on the RHS of the 0'th row.
Ex: (rocblas_fill_upper; n = 5; k = 2)
1 6 9 0 0              0 0 9 8 7
0 2 7 8 0              0 6 7 8 9
0 0 3 8 7     ---->    1 2 3 4 5
0 0 0 4 9              0 0 0 0 0
0 0 0 0 5              0 0 0 0 0

if uplo == rocblas_fill_lower:
The matrix represnted is a lower banded triangular matrix
with the main diagonal and k sub-diagonals, everything else can be
assumed to be 0.
The matrix is compacted so that the main diagonal resides on the 0'th row,
working up to the k'th diagonal residing on the LHS of the k'th row.
Ex: (rocblas_fill_lower; n = 5; k = 2)
1 0 0 0 0              1 2 3 4 5
6 2 0 0 0              6 7 8 9 0
9 7 3 0 0     ---->    9 8 7 0 0
0 8 8 4 0              0 0 0 0 0
0 0 7 9 5              0 0 0 0 0
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. lda must satisfy lda > k.
@param[in]
stride_A  [rocblas_stride]
stride from the start of one A_i matrix to the next A_(i + 1).
@param[in, out]
x         device array to the first vector x_i of the batch.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one x_i matrix to the next x_(i + 1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_stbmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stbmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tbsv solves:

A*x = b or
A**T*x = b or
A**H*x = b
where x and b are vectors and A is a banded triangular matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none: Solves A*x = b
- rocblas_operation_transpose: Solves A**T*x = b
- rocblas_operation_conjugate_transpose: Solves A**H*x = b

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit: A is assumed to be unit triangular (i.e. the diagonal elements
of A are not used in computations).
- rocblas_diagonal_non_unit: A is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of b. n >= 0.
@param[in]
k         [rocblas_int]

if(uplo == rocblas_fill_upper)
k specifies the number of super-diagonals of A.
if(uplo == rocblas_fill_lower)
k specifies the number of sub-diagonals of A.
k >= 0.

@param[in]
A         device pointer storing the matrix A in banded format.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
lda >= (k + 1).

@param[in, out]
x         device pointer storing input vector b. Overwritten by the output vector x.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
*/
    pub fn rocblas_stbsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stbsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const f32,
        lda: i64,
        x: *mut f32,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const f64,
        lda: i64,
        x: *mut f64,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tbsv_batched solves:

A_i*x_i = b_i or
A_i**T*x_i = b_i or
A_i**H*x_i = b_i
where x_i and b_i are vectors and A_i is a banded triangular matrix,
for i = [1, batch_count].

The input vectors b_i are overwritten by the output vectors x_i.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix.
- rocblas_fill_lower:  A_i is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none: Solves A_i*x_i = b_i
- rocblas_operation_transpose: Solves A_i**T*x_i = b_i
- rocblas_operation_conjugate_transpose: Solves A_i**H*x_i = b_i

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     each A_i is assumed to be unit triangular (i.e. the diagonal elements
of each A_i are not used in computations).
- rocblas_diagonal_non_unit: each A_i is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of each b_i. n >= 0.
@param[in]
k         [rocblas_int]

if(uplo == rocblas_fill_upper)
k specifies the number of super-diagonals of each A_i.
if(uplo == rocblas_fill_lower)
k specifies the number of sub-diagonals of each A_i.
k >= 0.

@param[in]
A         device vector of device pointers storing each matrix A_i in banded format.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
lda >= (k + 1).

@param[in, out]
x         device vector of device pointers storing each input vector b_i. Overwritten by each output
vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_stbsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const *const f32,
        lda: rocblas_int,
        x: *const *mut f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const *const f64,
        lda: rocblas_int,
        x: *const *mut f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stbsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const *const f32,
        lda: i64,
        x: *const *mut f32,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const *const f64,
        lda: i64,
        x: *const *mut f64,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tbsv_strided_batched solves:

A_i*x_i = b_i or
A_i**T*x_i = b_i or
A_i**H*x_i = b_i
where x_i and b_i are vectors and A_i is a banded triangular matrix,
for i = [1, batch_count].

The input vectors b_i are overwritten by the output vectors x_i.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix.
- rocblas_fill_lower:  A_i is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none: Solves A_i*x_i = b_i
- rocblas_operation_transpose: Solves A_i**T*x_i = b_i
- rocblas_operation_conjugate_transpose: Solves A_i**H*x_i = b_i

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     each A_i is assumed to be unit triangular (i.e. the diagonal elements
of each A_i are not used in computations).
- rocblas_diagonal_non_unit: each A_i is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of each b_i. n >= 0.
@param[in]
k         [rocblas_int]

if(uplo == rocblas_fill_upper)
k specifies the number of super-diagonals of each A_i.
if(uplo == rocblas_fill_lower)
k specifies the number of sub-diagonals of each A_i.
k >= 0.

@param[in]
A         device pointer pointing to the first banded matrix A_1.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
lda >= (k + 1).
@param[in]
stride_A  [rocblas_stride]
specifies the distance between the start of one matrix (A_i) and the next (A_i+1).

@param[in, out]
x         device pointer pointing to the first input vector b_1. Overwritten by output vectors x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
specifies the distance between the start of one vector (x_i) and the next (x_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_stbsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        k: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stbsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtbsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctbsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztbsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        k: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
trsv solves:

A*x = b or
A**T*x = b or
A**H*x = b,
where x and b are vectors and A is a triangular matrix.
The vector x is overwritten on b.

Although not widespread, some gemm kernels used by trsv may use atomic operations.
See Atomic Operations in the API Reference Guide for more information.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:   op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of b. n >= 0.

@param[in]
A         device pointer storing matrix A, of dimension ( lda, n ). If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains the matrix A, otherwise the lower triangular part of the leading n-by-n array contains the matrix A.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. lda must be at least max( 1, n ).

@param[in, out]
x         device pointer storing vector x. On exit, x is overwritten with the transformed vector x.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
*/
    pub fn rocblas_strsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        x: *mut f32,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        x: *mut f64,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f32,
        lda: i64,
        x: *mut f32,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f64,
        lda: i64,
        x: *mut f64,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *mut rocblas_float_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *mut rocblas_double_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
trsv_batched solves:

A_i*x_i = b_i or
A_i**T*x_i = b_i or
A_i**H*x_i = b_i,
where (A_i, x_i, b_i) is the i-th instance of the batch.
x_i and b_i are vectors and A_i is an
n by n triangular matrix.

The vector x is overwritten on b.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:   op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of b. n >= 0.

@param[in]
A         device pointer to an array of device pointers to the A_i matrices, of dimension ( lda, n ). If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains the matrix A_i, otherwise the lower triangular part of the leading n-by-n array contains the matrix A_i.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of A_i. lda must be at least max( 1, n ).

@param[in, out]
x         device pointer to an array of device pointers to the x_i vectors. On exit, each x_i is overwritten with the transformed vector x_i.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_strsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const f32,
        lda: rocblas_int,
        x: *const *mut f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const f64,
        lda: rocblas_int,
        x: *const *mut f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const f32,
        lda: i64,
        x: *const *mut f32,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const f64,
        lda: i64,
        x: *const *mut f64,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
trsv_strided_batched solves:

A_i*x_i = b_i or
A_i**T*x_i = b_i or
A_i**H*x_i = b_i,
where (A_i, x_i, b_i) is the i-th instance of the batch.
x_i and b_i are vectors and A_i is an n by n triangular matrix, for i = 1, ..., batch_count.

The vector x is overwritten on b.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA     [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:   op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of each b_i. n >= 0.

@param[in]
A         device pointer to the matrix A_1 of the batch, of dimension ( lda, n ). If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains the matrix A_i, otherwise the lower triangular part of the leading n-by-n array contains the matrix A_i.

@param[in]
stride_A  [rocblas_stride]
stride from the start of one A_i matrix to the next A_(i + 1).

@param[in]
lda       [rocblas_int]
specifies the leading dimension of A_i. lda must be at least max( 1, n ).

@param[in, out]
x         device pointer to the vector x_1 of the batch. On exit, each x_i is overwritten with the transformed vector x_i.

@param[in]
stride_x [rocblas_stride]
stride from the start of one x_i vector to the next x_(i + 1)

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_strsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tpsv solves:

A*x = b or
A**T*x = b or
A**H*x = b
where x and b are vectors and A is a triangular matrix stored in the packed format.

The input vector b is overwritten by the output vector x.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA  [rocblas_operation]
- rocblas_operation_none: Solves A*x = b
- rocblas_operation_transpose: Solves A**T*x = b
- rocblas_operation_conjugate_transpose: Solves A**H*x = b

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:  A is assumed to be unit triangular (i.e. the diagonal elements
of A are not used in computations).
- rocblas_diagonal_non_unit: A is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of b. n >= 0.

@param[in]
AP        device pointer storing the packed version of matrix A,
of dimension >= (n * (n + 1) / 2).

@param[in, out]
x         device pointer storing vector b on input, overwritten by x on output.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
*/
    pub fn rocblas_stpsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const f32,
        x: *mut f32,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const f64,
        x: *mut f64,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const rocblas_float_complex,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpsv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const rocblas_double_complex,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stpsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const f32,
        x: *mut f32,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const f64,
        x: *mut f64,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const rocblas_float_complex,
        x: *mut rocblas_float_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpsv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const rocblas_double_complex,
        x: *mut rocblas_double_complex,
        incx: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tpsv_batched solves:

A_i*x_i = b_i or
A_i**T*x_i = b_i or
A_i**H*x_i = b_i
where x_i and b_i are vectors and A_i is a triangular matrix stored in the packed format,
for i in [1, batch_count].

The input vectors b_i are overwritten by the output vectors x_i.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  each A_i is an upper triangular matrix.
- rocblas_fill_lower:  each A_i is a  lower triangular matrix.

@param[in]
transA  [rocblas_operation]
- rocblas_operation_none: Solves A*x = b
- rocblas_operation_transpose: Solves A**T*x = b
- rocblas_operation_conjugate_transpose: Solves A**H*x = b

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit: Each A_i is assumed to be unit triangular (i.e. the diagonal elements
of each A_i are not used in computations).
- rocblas_diagonal_non_unit: each A_i is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of each b_i. n >= 0.

@param[in]
AP        device array of device pointers storing the packed versions of each matrix A_i,
of dimension >= (n * (n + 1) / 2).

@param[in, out]
x         device array of device pointers storing each input vector b_i, overwritten by x_i on output.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
batch_count [rocblas_int]
specifies the number of instances in the batch.
*/
    pub fn rocblas_stpsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const *const f32,
        x: *const *mut f32,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const *const f64,
        x: *const *mut f64,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const *const rocblas_float_complex,
        x: *const *mut rocblas_float_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpsv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const *const rocblas_double_complex,
        x: *const *mut rocblas_double_complex,
        incx: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stpsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const *const f32,
        x: *const *mut f32,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const *const f64,
        x: *const *mut f64,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const *const rocblas_float_complex,
        x: *const *mut rocblas_float_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpsv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const *const rocblas_double_complex,
        x: *const *mut rocblas_double_complex,
        incx: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
tpsv_strided_batched solves:

A_i*x_i = b_i or
A_i**T*x_i = b_i or
A_i**H*x_i = b_i
where x_i and b_i are vectors and A_i is a triangular matrix stored in the packed format,
for i in [1, batch_count].

The input vectors b_i are overwritten by the output vectors x_i.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  each A_i is an upper triangular matrix.
- rocblas_fill_lower:  each A_i is a  lower triangular matrix.

@param[in]
transA  [rocblas_operation]
- rocblas_operation_none: Solves A*x = b
- rocblas_operation_transpose: Solves A**T*x = b
- rocblas_operation_conjugate_transpose: Solves A**H*x = b

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     each A_i is assumed to be unit triangular (i.e. the diagonal elements
of each A_i are not used in computations).
- rocblas_diagonal_non_unit: each A_i is not assumed to be unit triangular.

@param[in]
n         [rocblas_int]
n specifies the number of rows of each b_i. n >= 0.

@param[in]
AP        device pointer pointing to the first packed matrix A_1,
of dimension >= (n * (n + 1) / 2).

@param[in]
stride_A  [rocblas_stride]
stride from the beginning of one packed matrix (AP_i) and the next (AP_i+1).

@param[in, out]
x         device pointer pointing to the first input vector b_1. Overwritten by each x_i on output.

@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the beginning of one vector (x_i) and the next (x_i+1).
@param[in]
batch_count [rocblas_int]
specifies the number of instances in the batch.
*/
    pub fn rocblas_stpsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const f32,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const f64,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const rocblas_float_complex,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpsv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: rocblas_int,
        AP: *const rocblas_double_complex,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_stpsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const f32,
        stride_A: rocblas_stride,
        x: *mut f32,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtpsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const f64,
        stride_A: rocblas_stride,
        x: *mut f64,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctpsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const rocblas_float_complex,
        stride_A: rocblas_stride,
        x: *mut rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztpsv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        n: i64,
        AP: *const rocblas_double_complex,
        stride_A: rocblas_stride,
        x: *mut rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
symv performs the matrix-vector operation:

y := alpha*A*x + beta*y
where alpha and beta are scalars, x and y are n element vectors and
A should contain an upper or lower triangular n by n symmetric matrix.

symv has an implementation which uses atomic operations. See Atomic Operations
in the API Reference Guide for more information.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo     [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced.
- if rocblas_fill_lower, the upper part of A is not referenced.
@param[in]
n         [rocblas_int]
@param[in]
alpha
specifies the scalar alpha.
@param[in]
A         pointer storing matrix A on the GPU
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
@param[in]
x         pointer storing vector x on the GPU.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
beta      specifies the scalar beta
@param[out]
y         pointer storing vector y on the GPU.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_ssymv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssymv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        x: *const f32,
        incx: i64,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        x: *const f64,
        incx: i64,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
symv_batched performs the matrix-vector operation:

y_i := alpha*A_i*x_i + beta*y_i
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
n by n symmetric matrix, for i = 1, ..., batch_count.
A a should contain an upper or lower triangular symmetric matrix
and the opposing triangular part of A is not referenced.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced.
- if rocblas_fill_lower, the upper part of A is not referenced.
@param[in]
n         [rocblas_int]
number of rows and columns of each matrix A_i.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
A         device array of device pointers storing each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each matrix A_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each vector x_i.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssymv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *const *mut f64,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssymv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        x: *const *const f32,
        incx: i64,
        beta: *const f32,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        x: *const *const f64,
        incx: i64,
        beta: *const f64,
        y: *const *mut f64,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        beta: *const rocblas_float_complex,
        y: *const *mut rocblas_float_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        beta: *const rocblas_double_complex,
        y: *const *mut rocblas_double_complex,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
symv_strided_batched performs the matrix-vector operation:

y_i := alpha*A_i*x_i + beta*y_i
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
n by n symmetric matrix, for i = 1, ..., batch_count.
A a should contain an upper or lower triangular symmetric matrix
and the opposing triangular part of A is not referenced.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
number of rows and columns of each matrix A_i.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
A         Device pointer to the first matrix A_1 on the GPU.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each matrix A_i.
@param[in]
strideA     [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).
@param[in]
x         Device pointer to the first vector x_1 on the GPU.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each vector x_i.
@param[in]
stridex     [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stride_x. However, ensure that stridex is of appropriate size.
This typically means stridex >= n * incx. stridex should be non zero.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[out]
y         Device pointer to the first vector y_1 on the GPU.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in]
stridey     [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
There are no restrictions placed on stride_y. However, ensure that stridey is of appropriate size.
This typically means stridey >= n * incy. stridey should be non zero.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssymv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssymv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        strideA: rocblas_stride,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        strideA: rocblas_stride,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        strideA: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const rocblas_float_complex,
        y: *mut rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        strideA: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const rocblas_double_complex,
        y: *mut rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
spmv performs the matrix-vector operation:

y := alpha*A*x + beta*y
where alpha and beta are scalars, x and y are n element vectors and
A should contain an upper or lower triangular n by n packed symmetric matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      rocblas_fill
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
@param[in]
alpha
specifies the scalar alpha.
@param[in]
A         pointer storing matrix A on the GPU.
@param[in]
x         pointer storing vector x on the GPU.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
beta      specifies the scalar beta.
@param[out]
y         pointer storing vector y on the GPU.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_sspmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        x: *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        x: *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sspmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        x: *const f32,
        incx: i64,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        x: *const f64,
        incx: i64,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
spmv_batched performs the matrix-vector operation:

y_i := alpha*A_i*x_i + beta*y_i
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
n by n symmetric matrix, for i = 1, ..., batch_count.
A should contain an upper or lower triangular n by n packed symmetric matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
number of rows and columns of each matrix A_i.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
A         device array of device pointers storing each matrix A_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each vector x_i.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sspmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        x: *const *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        x: *const *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *const *mut f64,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sspmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        A: *const *const f32,
        x: *const *const f32,
        incx: i64,
        beta: *const f32,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        A: *const *const f64,
        x: *const *const f64,
        incx: i64,
        beta: *const f64,
        y: *const *mut f64,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
spmv_strided_batched performs the matrix-vector operation:

y_i := alpha*A_i*x_i + beta*y_i
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
n by n symmetric matrix, for i = 1, ..., batch_count.
A should contain an upper or lower triangular n by n packed symmetric matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
number of rows and columns of each matrix A_i.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
A         Device pointer to the first matrix A_1 on the GPU.
@param[in]
strideA     [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).
@param[in]
x         Device pointer to the first vector x_1 on the GPU.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each vector x_i.
@param[in]
stridex     [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stridex. However, ensure that stridex is of appropriate size.
This typically means stridex >= n * incx. stridex should be non zero.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[out]
y         Device pointer to the first vector y_1 on the GPU.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in]
stridey     [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
There are no restrictions placed on stridey. However, ensure that stridey is of appropriate size.
This typically means stridey >= n * incy. stridey should be non zero.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sspmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        strideA: rocblas_stride,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        strideA: rocblas_stride,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sspmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        strideA: rocblas_stride,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        strideA: rocblas_stride,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
sbmv performs the matrix-vector operation:

y := alpha*A*x + beta*y
where alpha and beta are scalars, x and y are n element vectors and
A should contain an upper or lower triangular n by n symmetric banded matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      rocblas_fill
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
@param[in]
k         [rocblas_int]
specifies the number of sub- and super-diagonals.
@param[in]
alpha
specifies the scalar alpha.
@param[in]
A         pointer storing matrix A on the GPU.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of matrix A.
@param[in]
x         pointer storing vector x on the GPU.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
beta      specifies the scalar beta.
@param[out]
y         pointer storing vector y on the GPU.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
*/
    pub fn rocblas_ssbmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsbmv(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssbmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        x: *const f32,
        incx: i64,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsbmv_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        x: *const f64,
        incx: i64,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
sbmv_batched performs the matrix-vector operation:

y_i := alpha*A_i*x_i + beta*y_i
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
n by n symmetric banded matrix, for i = 1, ..., batch_count.
A should contain an upper or lower triangular n by n symmetric banded matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
number of rows and columns of each matrix A_i.
@param[in]
k         [rocblas_int]
specifies the number of sub- and super-diagonals.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
A         device array of device pointers storing each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each matrix A_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each vector x_i.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[out]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssbmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        beta: *const f32,
        y: *const *mut f32,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsbmv_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        beta: *const f64,
        y: *const *mut f64,
        incy: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssbmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        x: *const *const f32,
        incx: i64,
        beta: *const f32,
        y: *const *mut f32,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsbmv_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        x: *const *const f64,
        incx: i64,
        beta: *const f64,
        y: *const *mut f64,
        incy: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
sbmv_strided_batched performs the matrix-vector operation:

y_i := alpha*A_i*x_i + beta*y_i
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha and beta are scalars, x_i and y_i are vectors and A_i is an
n by n symmetric banded matrix, for i = 1, ..., batch_count.
A should contain an upper or lower triangular n by n symmetric banded matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
number of rows and columns of each matrix A_i.
@param[in]
k         [rocblas_int]
specifies the number of sub- and super-diagonals.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
A         Device pointer to the first matrix A_1 on the GPU.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each matrix A_i.
@param[in]
strideA     [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).
@param[in]
x         Device pointer to the first vector x_1 on the GPU.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each vector x_i.
@param[in]
stridex     [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stridex. However, ensure that stridex is of appropriate size.
This typically means stridex >= n * incx. stridex should be non zero.
@param[in]
beta      device pointer or host pointer to scalar beta.
@param[out]
y         Device pointer to the first vector y_1 on the GPU.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in]
stridey     [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
There are no restrictions placed on stridey. However, ensure that stridey is of appropriate size.
This typically means stridey >= n * incy. stridey should be non zero.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssbmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsbmv_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        strideA: rocblas_stride,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssbmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        strideA: rocblas_stride,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f32,
        y: *mut f32,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsbmv_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        strideA: rocblas_stride,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        beta: *const f64,
        y: *mut f64,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
ger,geru,gerc performs the matrix-vector operations:

A := A + alpha*x*y**T , OR
A := A + alpha*x*y**H for gerc
where alpha is a scalar, x and y are vectors, and A is an
m by n matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
m         [rocblas_int]
the number of rows of the matrix A.
@param[in]
n         [rocblas_int]
the number of columns of the matrix A.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
A         device pointer storing matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
*/
    pub fn rocblas_sger(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        y: *const f32,
        incy: rocblas_int,
        A: *mut f32,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dger(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        y: *const f64,
        incy: rocblas_int,
        A: *mut f64,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeru(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeru(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgerc(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgerc(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sger_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        y: *const f32,
        incy: i64,
        A: *mut f32,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dger_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        y: *const f64,
        incy: i64,
        A: *mut f64,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeru_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        y: *const rocblas_float_complex,
        incy: i64,
        A: *mut rocblas_float_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeru_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        y: *const rocblas_double_complex,
        incy: i64,
        A: *mut rocblas_double_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgerc_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        y: *const rocblas_float_complex,
        incy: i64,
        A: *mut rocblas_float_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgerc_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        y: *const rocblas_double_complex,
        incy: i64,
        A: *mut rocblas_double_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
ger_batched,geru_batched,gerc_batched perform a batch of the matrix-vector operations:

A := A + alpha*x*y**T , OR
A := A + alpha*x*y**H for gerc
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha is a scalar, x_i and y_i are vectors and A_i is an
m by n matrix, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
m         [rocblas_int]
the number of rows of each matrix A_i.
@param[in]
n         [rocblas_int]
the number of columns of each matrix A_i.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each vector x_i.
@param[in]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in, out]
A         device array of device pointers storing each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sger_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *const f32,
        incx: rocblas_int,
        y: *const *const f32,
        incy: rocblas_int,
        A: *const *mut f32,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dger_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *const f64,
        incx: rocblas_int,
        y: *const *const f64,
        incy: rocblas_int,
        A: *const *mut f64,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeru_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const *const rocblas_float_complex,
        incy: rocblas_int,
        A: *const *mut rocblas_float_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeru_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const *const rocblas_double_complex,
        incy: rocblas_int,
        A: *const *mut rocblas_double_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgerc_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const *const rocblas_float_complex,
        incy: rocblas_int,
        A: *const *mut rocblas_float_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgerc_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const *const rocblas_double_complex,
        incy: rocblas_int,
        A: *const *mut rocblas_double_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sger_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const f32,
        x: *const *const f32,
        incx: i64,
        y: *const *const f32,
        incy: i64,
        A: *const *mut f32,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dger_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const f64,
        x: *const *const f64,
        incx: i64,
        y: *const *const f64,
        incy: i64,
        A: *const *mut f64,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeru_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: i64,
        y: *const *const rocblas_float_complex,
        incy: i64,
        A: *const *mut rocblas_float_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeru_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: i64,
        y: *const *const rocblas_double_complex,
        incy: i64,
        A: *const *mut rocblas_double_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgerc_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: i64,
        y: *const *const rocblas_float_complex,
        incy: i64,
        A: *const *mut rocblas_float_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgerc_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: i64,
        y: *const *const rocblas_double_complex,
        incy: i64,
        A: *const *mut rocblas_double_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
ger_strided_batched,geru_strided_batched,gerc_strided_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*y_i**T, OR
A_i := A_i + alpha*x_i*y_i**H  for gerc
where (A_i, x_i, y_i) is the i-th instance of the batch.
alpha is a scalar, x_i and y_i are vectors and A_i is an
m by n matrix, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
m         [rocblas_int]
the number of rows of each matrix A_i.
@param[in]
n         [rocblas_int]
the number of columns of each matrix A_i.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer to the first vector (x_1) in the batch.
@param[in]
incx      [rocblas_int]
specifies the increments for the elements of each vector x_i.
@param[in]
stridex   [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stride_x. However, ensure that stride_x is of appropriate size. For a typical
case this means stride_x >= m * incx.
@param[in, out]
y         device pointer to the first vector (y_1) in the batch.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each vector y_i.
@param[in]
stridey   [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
There are no restrictions placed on stride_y. However, ensure that stride_y is of appropriate size. For a typical
case this means stride_y >= n * incy.
@param[in, out]
A         device pointer to the first matrix (A_1) in the batch.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
strideA     [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1)
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sger_strided_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut f32,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dger_strided_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut f64,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeru_strided_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeru_strided_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgerc_strided_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgerc_strided_batched(
        handle: rocblas_handle,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sger_strided_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        y: *const f32,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut f32,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dger_strided_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        y: *const f64,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut f64,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeru_strided_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeru_strided_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgerc_strided_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgerc_strided_batched_64(
        handle: rocblas_handle,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
spr performs the matrix-vector operations:

A := A + alpha*x*x**T
where alpha is a scalar, x is a vector, and A is an
n by n symmetric matrix, supplied in packed form.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of A is supplied in AP.
- rocblas_fill_lower: The lower triangular part of A is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in, out]
AP        device pointer storing the packed version of the specified triangular portion of
the symmetric matrix A. Of at least size ((n * (n + 1)) / 2).

if uplo == rocblas_fill_upper:
The upper triangular portion of the symmetric matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 4)
1 2 4 7
2 3 5 8   -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
4 5 6 9
7 8 9 0

if uplo == rocblas_fill_lower:
The lower triangular portion of the symmetric matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 4)
1 2 3 4
2 5 6 7    -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
3 6 8 9
4 7 9 0*/
    pub fn rocblas_sspr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        AP: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        AP: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cspr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        AP: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zspr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        AP: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sspr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        AP: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        AP: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cspr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        AP: *mut rocblas_float_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zspr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        AP: *mut rocblas_double_complex,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
spr_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*x_i**T
where alpha is a scalar, x_i is a vector, and A_i is an
n by n symmetric matrix, supplied in packed form, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in AP.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in, out]
AP        device array of device pointers storing the packed version of the specified triangular portion of
each symmetric matrix A_i of at least size ((n * (n + 1)) / 2). Array is of at least size batch_count.

if uplo == rocblas_fill_upper:
The upper triangular portion of each symmetric matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 4)
1 2 4 7
2 3 5 8   -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
4 5 6 9
7 8 9 0

if uplo == rocblas_fill_lower:
The lower triangular portion of each symmetric matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 4)
1 2 3 4
2 5 6 7    -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
3 6 8 9
4 7 9 0
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_sspr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *const f32,
        incx: rocblas_int,
        AP: *const *mut f32,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *const f64,
        incx: rocblas_int,
        AP: *const *mut f64,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cspr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        AP: *const *mut rocblas_float_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zspr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        AP: *const *mut rocblas_double_complex,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sspr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const *const f32,
        incx: i64,
        AP: *const *mut f32,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const *const f64,
        incx: i64,
        AP: *const *mut f64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cspr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: i64,
        AP: *const *mut rocblas_float_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zspr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: i64,
        AP: *const *mut rocblas_double_complex,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
spr_strided_batched performs the matrix-vector operations:

A_i := A_i + alpha*x_i*x_i**T
where alpha is a scalar, x_i is a vector, and A_i is an
n by n symmetric matrix, supplied in packed form, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in AP.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer pointing to the first vector (x_1).
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in, out]
AP        device pointer storing the packed version of the specified triangular portion of
each symmetric matrix A_i. Points to the first A_1.

if uplo == rocblas_fill_upper:
The upper triangular portion of each symmetric matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 4)
1 2 4 7
2 3 5 8   -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
4 5 6 9
7 8 9 0

if uplo == rocblas_fill_lower:
The lower triangular portion of each symmetric matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(2) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 4)
1 2 3 4
2 5 6 7    -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
3 6 8 9
4 7 9 0
@param[in]
stride_A    [rocblas_stride]
stride from the start of one (A_i) and the next (A_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_sspr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        AP: *mut f32,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        AP: *mut f64,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cspr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        AP: *mut rocblas_float_complex,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zspr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        AP: *mut rocblas_double_complex,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sspr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        stride_x: rocblas_stride,
        AP: *mut f32,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        stride_x: rocblas_stride,
        AP: *mut f64,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cspr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        AP: *mut rocblas_float_complex,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zspr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        AP: *mut rocblas_double_complex,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
spr2 performs the matrix-vector operation:

A := A + alpha*x*y**T + alpha*y*x**T
where alpha is a scalar, x and y are vectors, and A is an
n by n symmetric matrix, supplied in packed form.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of A is supplied in AP.
- rocblas_fill_lower: The lower triangular part of A is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
AP        device pointer storing the packed version of the specified triangular portion of
the symmetric matrix A. Of at least size ((n * (n + 1)) / 2).

if uplo == rocblas_fill_upper:
The upper triangular portion of the symmetric matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 4)
1 2 4 7
2 3 5 8   -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
4 5 6 9
7 8 9 0

if uplo == rocblas_fill_lower:
The lower triangular portion of the symmetric matrix A is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(n) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 4)
1 2 3 4
2 5 6 7    -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
3 6 8 9
4 7 9 0*/
    pub fn rocblas_sspr2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        y: *const f32,
        incy: rocblas_int,
        AP: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        y: *const f64,
        incy: rocblas_int,
        AP: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sspr2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        y: *const f32,
        incy: i64,
        AP: *mut f32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        y: *const f64,
        incy: i64,
        AP: *mut f64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
spr2_batched performs the matrix-vector operation:

A_i := A_i + alpha*x_i*y_i**T + alpha*y_i*x_i**T
where alpha is a scalar, x_i and y_i are vectors, and A_i is an
n by n symmetric matrix, supplied in packed form, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in AP.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in, out]
AP        device array of device pointers storing the packed version of the specified triangular portion of
each symmetric matrix A_i of at least size ((n * (n + 1)) / 2). Array is of at least size batch_count.

if uplo == rocblas_fill_upper:
The upper triangular portion of each symmetric matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 4)
1 2 4 7
2 3 5 8   -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
4 5 6 9
7 8 9 0

if uplo == rocblas_fill_lower:
The lower triangular portion of each symmetric matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(n) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 4)
1 2 3 4
2 5 6 7    -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
3 6 8 9
4 7 9 0
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_sspr2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *const f32,
        incx: rocblas_int,
        y: *const *const f32,
        incy: rocblas_int,
        AP: *const *mut f32,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *const f64,
        incx: rocblas_int,
        y: *const *const f64,
        incy: rocblas_int,
        AP: *const *mut f64,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sspr2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const *const f32,
        incx: i64,
        y: *const *const f32,
        incy: i64,
        AP: *const *mut f32,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const *const f64,
        incx: i64,
        y: *const *const f64,
        incy: i64,
        AP: *const *mut f64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
spr2_strided_batched performs the matrix-vector operation:

A_i := A_i + alpha*x_i*y_i**T + alpha*y_i*x_i**T
where alpha is a scalar, x_i and y_i are vectors, and A_i is an
n by n symmetric matrix, supplied in packed form, for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- rocblas_fill_upper: The upper triangular part of each A_i is supplied in AP.
- rocblas_fill_lower: The lower triangular part of each A_i is supplied in AP.
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A_i. Must be at least 0.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer pointing to the first vector (x_1).
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
@param[in]
y         device pointer pointing to the first vector (y_1).
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
stride_y  [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1).
@param[in, out]
AP        device pointer storing the packed version of the specified triangular portion of
each symmetric matrix A_i. Points to the first A_1.

if uplo == rocblas_fill_upper:
The upper triangular portion of each symmetric matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(0,1)
AP(2) = A(1,1), etc.
Ex: (rocblas_fill_upper; n = 4)
1 2 4 7
2 3 5 8   -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
4 5 6 9
7 8 9 0

if uplo == rocblas_fill_lower:
The lower triangular portion of each symmetric matrix A_i is supplied.
The matrix is compacted so that AP contains the triangular portion
column-by-column
so that:
AP(0) = A(0,0)
AP(1) = A(1,0)
AP(n) = A(2,1), etc.
Ex: (rocblas_fill_lower; n = 4)
1 2 3 4
2 5 6 7    -----> [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
3 6 8 9
4 7 9 0
@param[in]
stride_A    [rocblas_stride]
stride from the start of one (A_i) and the next (A_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.*/
    pub fn rocblas_sspr2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *const f32,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        AP: *mut f32,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *const f64,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        AP: *mut f64,
        stride_A: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sspr2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        stride_x: rocblas_stride,
        y: *const f32,
        incy: i64,
        stride_y: rocblas_stride,
        AP: *mut f32,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dspr2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        stride_x: rocblas_stride,
        y: *const f64,
        incy: i64,
        stride_y: rocblas_stride,
        AP: *mut f64,
        stride_A: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
syr performs the matrix-vector operations:

A := A + alpha*x*x**T
where alpha is a scalar, x is a vector, and A is an
n by n symmetric matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced

@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in, out]
A         device pointer storing matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
*/
    pub fn rocblas_ssyr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        A: *mut f32,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        A: *mut f64,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        A: *mut f32,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        A: *mut f64,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        A: *mut rocblas_float_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        A: *mut rocblas_double_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
syr_batched performs a batch of matrix-vector operations:

A[i] := A[i] + alpha*x[i]*x[i]**T
where alpha is a scalar, x is an array of vectors, and A is an array of
n by n symmetric matrices, for i = 1 , ... , batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in, out]
A         device array of device pointers storing each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *const f32,
        incx: rocblas_int,
        A: *const *mut f32,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *const f64,
        incx: rocblas_int,
        A: *const *mut f64,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        A: *const *mut rocblas_float_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        A: *const *mut rocblas_double_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const *const f32,
        incx: i64,
        A: *const *mut f32,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const *const f64,
        incx: i64,
        A: *const *mut f64,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: i64,
        A: *const *mut rocblas_float_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: i64,
        A: *const *mut rocblas_double_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
syr_strided_batched performs the matrix-vector operations:

A[i] := A[i] + alpha*x[i]*x[i]**T
where alpha is a scalar, vectors, and A is an array of
n by n symmetric matrices, for i = 1 , ... , batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer to the first vector x_1.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stridex   [rocblas_stride]
specifies the pointer increment between vectors (x_i) and (x_i+1).
@param[in, out]
A         device pointer to the first matrix A_1.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
strideA   [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        A: *mut f32,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        A: *mut f64,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        A: *mut f32,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        A: *mut f64,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
syr2 performs the matrix-vector operations:

A := A + alpha*x*y**T + alpha*y*x**T
where alpha is a scalar, x and y are vectors, and A is an
n by n symmetric matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced

@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
y         device pointer storing vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
A         device pointer storing matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
*/
    pub fn rocblas_ssyr2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        y: *const f32,
        incy: rocblas_int,
        A: *mut f32,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        y: *const f64,
        incy: rocblas_int,
        A: *mut f64,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyr2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        y: *const f32,
        incy: i64,
        A: *mut f32,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        y: *const f64,
        incy: i64,
        A: *mut f64,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        y: *const rocblas_float_complex,
        incy: i64,
        A: *mut rocblas_float_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        y: *const rocblas_double_complex,
        incy: i64,
        A: *mut rocblas_double_complex,
        lda: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
syr2_batched performs a batch of matrix-vector operations:

A[i] := A[i] + alpha*x[i]*y[i]**T + alpha*y[i]*x[i]**T
where alpha is a scalar, x[i] and y[i] are vectors, and A[i] is a
n by n symmetric matrix, for i = 1 , ... , batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
the number of rows and columns of matrix A.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
y         device array of device pointers storing each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in, out]
A         device array of device pointers storing each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyr2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const *const f32,
        incx: rocblas_int,
        y: *const *const f32,
        incy: rocblas_int,
        A: *const *mut f32,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const *const f64,
        incx: rocblas_int,
        y: *const *const f64,
        incy: rocblas_int,
        A: *const *mut f64,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        y: *const *const rocblas_float_complex,
        incy: rocblas_int,
        A: *const *mut rocblas_float_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        y: *const *const rocblas_double_complex,
        incy: rocblas_int,
        A: *const *mut rocblas_double_complex,
        lda: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyr2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const *const f32,
        incx: i64,
        y: *const *const f32,
        incy: i64,
        A: *const *mut f32,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const *const f64,
        incx: i64,
        y: *const *const f64,
        incy: i64,
        A: *const *mut f64,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const *const rocblas_float_complex,
        incx: i64,
        y: *const *const rocblas_float_complex,
        incy: i64,
        A: *const *mut rocblas_float_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const *const rocblas_double_complex,
        incx: i64,
        y: *const *const rocblas_double_complex,
        incy: i64,
        A: *const *mut rocblas_double_complex,
        lda: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 2 API </b>

\details
syr2_strided_batched the matrix-vector operations:

A[i] := A[i] + alpha*x[i]*y[i]**T + alpha*y[i]*x[i]**T
where alpha is a scalar, x[i] and y[i] are vectors, and A[i] is a
n by n symmetric matrices, for i = 1 , ... , batch_count

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
- if rocblas_fill_upper, the lower part of A is not referenced
- if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
n         [rocblas_int]
the number of rows and columns of each matrix A.
@param[in]
alpha
device pointer or host pointer to scalar alpha.
@param[in]
x         device pointer to the first vector x_1.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stridex   [rocblas_stride]
specifies the pointer increment between vectors (x_i) and (x_i+1).
@param[in]
y         device pointer to the first vector y_1.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
stridey   [rocblas_stride]
specifies the pointer increment between vectors (y_i) and (y_i+1).
@param[in, out]
A         device pointer to the first matrix A_1.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
strideA   [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyr2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f32,
        x: *const f32,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const f32,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut f32,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const f64,
        x: *const f64,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const f64,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut f64,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: rocblas_int,
        stridey: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: rocblas_int,
        strideA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyr2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f32,
        x: *const f32,
        incx: i64,
        stridex: rocblas_stride,
        y: *const f32,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut f32,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const f64,
        x: *const f64,
        incx: i64,
        stridex: rocblas_stride,
        y: *const f64,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut f64,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_float_complex,
        x: *const rocblas_float_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_float_complex,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut rocblas_float_complex,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        n: i64,
        alpha: *const rocblas_double_complex,
        x: *const rocblas_double_complex,
        incx: i64,
        stridex: rocblas_stride,
        y: *const rocblas_double_complex,
        incy: i64,
        stridey: rocblas_stride,
        A: *mut rocblas_double_complex,
        lda: i64,
        strideA: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
hemm performs one of the matrix-matrix operations:

C := alpha*A*B + beta*C if side == rocblas_side_left,
C := alpha*B*A + beta*C if side == rocblas_side_right,

where alpha and beta are scalars, B and C are m by n matrices, and
A is a Hermitian matrix stored as either upper or lower.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side  [rocblas_side]
- rocblas_side_left:      C := alpha*A*B + beta*C
- rocblas_side_right:     C := alpha*B*A + beta*C

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix
- rocblas_fill_lower:  A is a  lower triangular matrix

@param[in]
m       [rocblas_int]
m specifies the number of rows of B and C. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B and C. n >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A and B are not referenced.

@param[in]
A       pointer storing matrix A on the GPU.
- A is m by m if side == rocblas_side_left
- A is n by n if side == rocblas_side_right
Only the upper/lower triangular part is accessed.
The imaginary component of the diagonal elements is not used.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if side = rocblas_side_left,  lda >= max( 1, m ),
otherwise lda >= max( 1, n ).

@param[in]
B       pointer storing matrix B on the GPU.
Matrix dimension is m by n

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B. ldb >= max( 1, m ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       pointer storing matrix C on the GPU.
Matrix dimension is m by n

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, m ).
*/
    pub fn rocblas_chemm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chemm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
hemm_batched performs a batch of the matrix-matrix operations:

C_i := alpha*A_i*B_i + beta*C_i if side == rocblas_side_left,
C_i := alpha*B_i*A_i + beta*C_i if side == rocblas_side_right,

where alpha and beta are scalars, B_i and C_i are m by n matrices, and
A_i is a Hermitian matrix stored as either upper or lower.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side  [rocblas_side]
- rocblas_side_left:      C_i := alpha*A_i*B_i + beta*C_i
- rocblas_side_right:     C_i := alpha*B_i*A_i + beta*C_i

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix
- rocblas_fill_lower:  A_i is a  lower triangular matrix

@param[in]
m       [rocblas_int]
m specifies the number of rows of B_i and C_i. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B_i and C_i. n >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A_i and B_i are not referenced.

@param[in]
A       device array of device pointers storing each matrix A_i on the GPU.
- A_i is m by m if side == rocblas_side_left
- A_i is n by n if side == rocblas_side_right
Only the upper/lower triangular part is accessed.
The imaginary component of the diagonal elements is not used.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if side = rocblas_side_left,  lda >= max( 1, m ),
otherwise lda >= max( 1, n ).

@param[in]
B       device array of device pointers storing each matrix B_i on the GPU.
Matrix dimension is m by n

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i. ldb >= max( 1, m ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C_i need not be set before entry.

@param[in]
C       device array of device pointers storing each matrix C_i on the GPU.
Matrix dimension is m by n

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C_i. ldc >= max( 1, m ).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_chemm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chemm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
hemm_strided_batched performs a batch of the matrix-matrix operations:

C_i := alpha*A_i*B_i + beta*C_i if side == rocblas_side_left,
C_i := alpha*B_i*A_i + beta*C_i if side == rocblas_side_right,

where alpha and beta are scalars, B_i and C_i are m by n matrices, and
A_i is a Hermitian matrix stored as either upper or lower.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side  [rocblas_side]
- rocblas_side_left:      C_i := alpha*A_i*B_i + beta*C_i
- rocblas_side_right:     C_i := alpha*B_i*A_i + beta*C_i

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix
- rocblas_fill_lower:  A_i is a  lower triangular matrix

@param[in]
m       [rocblas_int]
m specifies the number of rows of B_i and C_i. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B_i and C_i. n >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A_i and B_i are not referenced.

@param[in]
A       device pointer to first matrix A_1
- A_i is m by m if side == rocblas_side_left
- A_i is n by n if side == rocblas_side_right
Only the upper/lower triangular part is accessed.
The imaginary component of the diagonal elements is not used.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if side = rocblas_side_left,  lda >= max( 1, m ),
otherwise lda >= max( 1, n ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).

@param[in]
B       device pointer to first matrix B_1 of dimension (ldb, n) on the GPU

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i.

if side = rocblas_operation_none,  ldb >= max( 1, m ),
otherwise ldb >= max( 1, n ).

@param[in]
stride_B  [rocblas_stride]
stride from the start of one matrix (B_i) and the next one (B_i+1).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C        device pointer to first matrix C_1 of dimension (ldc, n) on the GPU.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, m ).

@param[in, out]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_chemm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_chemm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zhemm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
herk performs one of the matrix-matrix operations for a Hermitian rank-k update:

C := alpha*op( A )*op( A )^H + beta*C,

where  alpha and beta are scalars, op(A) is an n by k matrix, and
C is a n x n Hermitian matrix stored as either upper or lower.

op( A ) = A, and A is n by k if transA == rocblas_operation_none
op( A ) = A^H and A is k by n if transA == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C is an upper triangular matrix
- rocblas_fill_lower:  C is a  lower triangular matrix

@param[in]
transA  [rocblas_operation]
- rocblas_operation_conjugate_transpose:  op(A) = A^H
- rocblas_operation_none:                 op(A) = A

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       pointer storing matrix A on the GPU.
Matrix dimension is ( lda, k ) when if transA = rocblas_operation_none, otherwise (lda, n)

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if transA = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       pointer storing matrix C on the GPU.
The imaginary component of the diagonal elements are not used but are set to zero unless quick return.
only the upper/lower triangular part is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
*/
    pub fn rocblas_cherk(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherk(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cherk_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const rocblas_float_complex,
        lda: i64,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherk_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const rocblas_double_complex,
        lda: i64,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
herk_batched performs a batch of the matrix-matrix operations for a Hermitian rank-k update:

C_i := alpha*op( A_i )*op( A_i )^H + beta*C_i,

where  alpha and beta are scalars, op(A) is an n by k matrix, and
C_i is a n x n Hermitian matrix stored as either upper or lower.

op( A_i ) = A_i, and A_i is n by k if transA == rocblas_operation_none
op( A_i ) = A_i^H and A_i is k by n if transA == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
transA  [rocblas_operation]
- rocblas_operation_conjugate_transpose: op(A) = A^H
- rocblas_operation_none:                op(A) = A

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       device array of device pointers storing each matrix_i A of dimension (lda, k)
when transA is rocblas_operation_none, otherwise of dimension (lda, n).

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if transA = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       device array of device pointers storing each matrix C_i on the GPU.
The imaginary component of the diagonal elements are not used but are set to zero unless quick return.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_cherk_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        beta: *const f32,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherk_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        beta: *const f64,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cherk_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const *const rocblas_float_complex,
        lda: i64,
        beta: *const f32,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherk_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const *const rocblas_double_complex,
        lda: i64,
        beta: *const f64,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
herk_strided_batched performs a batch of the matrix-matrix operations for a Hermitian rank-k update:

C_i := alpha*op( A_i )*op( A_i )^H + beta*C_i,

where  alpha and beta are scalars, op(A) is an n by k matrix, and
C_i is a n x n Hermitian matrix stored as either upper or lower.

op( A_i ) = A_i, and A_i is n by k if transA == rocblas_operation_none
op( A_i ) = A_i^H and A_i is k by n if transA == rocblas_operation_conjugate_transpose


@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
transA  [rocblas_operation]
- rocblas_operation_conjugate_transpose: op(A) = A^H
- rocblas_operation_none:                op(A) = A

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       Device pointer to the first matrix A_1 on the GPU of dimension (lda, k)
when transA is rocblas_operation_none, otherwise of dimension (lda, n)

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if transA = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       Device pointer to the first matrix C_1 on the GPU.
The imaginary component of the diagonal elements are not used but are set to zero unless quick return.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).

@param[in, out]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_cherk_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherk_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cherk_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherk_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
her2k performs one of the matrix-matrix operations for a Hermitian rank-2k update:

C := alpha*op( A )*op( B )^H + conj(alpha)*op( B )*op( A )^H + beta*C,

where  alpha and beta are scalars, op(A) and op(B) are n by k matrices, and
C is a n x n Hermitian matrix stored as either upper or lower.

op( A ) = A, op( B ) = B, and A and B are n by k if trans == rocblas_operation_none
op( A ) = A^H, op( B ) = B^H,  and A and B are k by n if trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C is an upper triangular matrix
- rocblas_fill_lower:  C is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_conjugate_transpose:  op( A ) = A^H, op( B ) = B^H
- rocblas_operation_none:                 op( A ) = A, op( B ) = B

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       pointer storing matrix A on the GPU.
Matrix dimension is ( lda, k ) when if trans = rocblas_operation_none, otherwise (lda, n)

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
B       pointer storing matrix B on the GPU.
Matrix dimension is ( ldb, k ) when if trans = rocblas_operation_none, otherwise (ldb, n)

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       pointer storing matrix C on the GPU.
The imaginary component of the diagonal elements are not used but are set to zero unless quick return.
only the upper/lower triangular part is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
*/
    pub fn rocblas_cher2k(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2k(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cher2k_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *const rocblas_float_complex,
        ldb: i64,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2k_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *const rocblas_double_complex,
        ldb: i64,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
her2k_batched performs a batch of the matrix-matrix operations for a Hermitian rank-2k update:

C_i := alpha*op( A_i )*op( B_i )^H + conj(alpha)*op( B_i )*op( A_i )^H + beta*C_i,

where  alpha and beta are scalars, op(A_i) and op(B_i) are n by k matrices, and
C_i is a n x n Hermitian matrix stored as either upper or lower.

op( A_i ) = A_i, op( B_i ) = B_i, and A_i and B_i are n by k if trans == rocblas_operation_none
op( A_i ) = A_i^H, op( B_i ) = B_i^H,  and A_i and B_i are k by n if trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_conjugate_transpose: op(A) = A^H
- rocblas_operation_none:                op(A) = A

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       device array of device pointers storing each matrix_i A of dimension (lda, k)
when trans is rocblas_operation_none, otherwise of dimension (lda, n).

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).
@param[in]
B       device array of device pointers storing each matrix_i B of dimension (ldb, k)
when trans is rocblas_operation_none, otherwise of dimension (ldb, n).

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).
@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       device array of device pointers storing each matrix C_i on the GPU.
The imaginary component of the diagonal elements are not used but are set to zero unless quick return.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_cher2k_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const f32,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2k_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const f64,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cher2k_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        beta: *const f32,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2k_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        beta: *const f64,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
her2k_strided_batched performs a batch of the matrix-matrix operations for a Hermitian rank-2k update:

C_i := alpha*op( A_i )*op( B_i )^H + conj(alpha)*op( B_i )*op( A_i )^H + beta*C_i,

where  alpha and beta are scalars, op(A_i) and op(B_i) are n by k matrices, and
C_i is a n x n Hermitian matrix stored as either upper or lower.

op( A_i ) = A_i, op( B_i ) = B_i, and A_i and B_i are n by k if trans == rocblas_operation_none
op( A_i ) = A_i^H, op( B_i ) = B_i^H,  and A_i and B_i are k by n if trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_conjugate_transpose: op( A_i ) = A_i^H, op( B_i ) = B_i^H
- rocblas_operation_none:                op( A_i ) = A_i, op( B_i ) = B_i

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       Device pointer to the first matrix A_1 on the GPU of dimension (lda, k)
when trans is rocblas_operation_none, otherwise of dimension (lda, n).

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).

@param[in]
B       Device pointer to the first matrix B_1 on the GPU of dimension (ldb, k)
when trans is rocblas_operation_none, otherwise of dimension (ldb, n).

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).

@param[in]
stride_B  [rocblas_stride]
stride from the start of one matrix (B_i) and the next one (B_i+1).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       Device pointer to the first matrix C_1 on the GPU.
The imaginary component of the diagonal elements are not used but are set to zero unless quick return.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).

@param[in, out]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_cher2k_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2k_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cher2k_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zher2k_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
herkx performs one of the matrix-matrix operations for a Hermitian rank-k update:

C := alpha*op( A )*op( B )^H + beta*C,

where  alpha and beta are scalars, op(A) and op(B) are n by k matrices, and
C is a n x n Hermitian matrix stored as either upper or lower.

This routine should only be used when the caller can guarantee that the result of op( A )*op( B )^T will be Hermitian.

op( A ) = A, op( B ) = B, and A and B are n by k if trans == rocblas_operation_none
op( A ) = A^H, op( B ) = B^H,  and A and B are k by n if trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C is an upper triangular matrix
- rocblas_fill_lower:  C is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_conjugate_transpose:  op( A ) = A^H, op( B ) = B^H
- rocblas_operation_none:                 op( A ) = A, op( B ) = B

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       pointer storing matrix A on the GPU.
Matrix dimension is ( lda, k ) when if trans = rocblas_operation_none, otherwise (lda, n)

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).
@param[in]
B       pointer storing matrix B on the GPU.
Matrix dimension is ( ldb, k ) when if trans = rocblas_operation_none, otherwise (ldb, n)

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       pointer storing matrix C on the GPU.
The imaginary component of the diagonal elements are not used but are set to zero unless quick return.
only the upper/lower triangular part is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
*/
    pub fn rocblas_cherkx(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherkx(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cherkx_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *const rocblas_float_complex,
        ldb: i64,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherkx_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *const rocblas_double_complex,
        ldb: i64,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
herkx_batched performs a batch of the matrix-matrix operations for a Hermitian rank-k update:

C_i := alpha*op( A_i )*op( B_i )^H + beta*C_i,

where  alpha and beta are scalars, op(A_i) and op(B_i) are n by k matrices, and
C_i is a n x n Hermitian matrix stored as either upper or lower.

This routine should only be used when the caller can guarantee that the result of op( A )*op( B )^T will be Hermitian.

op( A_i ) = A_i, op( B_i ) = B_i, and A_i and B_i are n by k if trans == rocblas_operation_none
op( A_i ) = A_i^H, op( B_i ) = B_i^H,  and A_i and B_i are k by n if trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_conjugate_transpose: op(A) = A^H
- rocblas_operation_none:                op(A) = A

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       device array of device pointers storing each matrix_i A of dimension (lda, k)
when trans is rocblas_operation_none, otherwise of dimension (lda, n)

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
B       device array of device pointers storing each matrix_i B of dimension (ldb, k)
when trans is rocblas_operation_none, otherwise of dimension (ldb, n)

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       device array of device pointers storing each matrix C_i on the GPU.
The imaginary component of the diagonal elements are not used but are set to zero unless quick return.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_cherkx_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const f32,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherkx_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const f64,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cherkx_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        beta: *const f32,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherkx_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        beta: *const f64,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
herkx_strided_batched performs a batch of the matrix-matrix operations for a Hermitian rank-k update:

C_i := alpha*op( A_i )*op( B_i )^H + beta*C_i,

where  alpha and beta are scalars, op(A_i) and op(B_i) are n by k matrices, and
C_i is a n x n Hermitian matrix stored as either upper or lower.

This routine should only be used when the caller can guarantee that the result of op( A )*op( B )^T will be Hermitian.

op( A_i ) = A_i, op( B_i ) = B_i, and A_i and B_i are n by k if trans == rocblas_operation_none
op( A_i ) = A_i^H, op( B_i ) = B_i^H,  and A_i and B_i are k by n if trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_conjugate_transpose: op( A_i ) = A_i^H, op( B_i ) = B_i^H
- rocblas_operation_none:                op( A_i ) = A_i, op( B_i ) = B_i

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       Device pointer to the first matrix A_1 on the GPU of dimension (lda, k)
when trans is rocblas_operation_none, otherwise of dimension (lda, n).

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1)

@param[in]
B       Device pointer to the first matrix B_1 on the GPU of dimension (ldb, k)
when trans is rocblas_operation_none, otherwise of dimension (ldb, n).

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).

@param[in]
stride_B  [rocblas_stride]
stride from the start of one matrix (B_i) and the next one (B_i+1)

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       Device pointer to the first matrix C_1 on the GPU.
The imaginary component of the diagonal elements are not used but are set to zero unless quick return.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).

@param[in, out]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_cherkx_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherkx_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cherkx_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zherkx_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
symm performs one of the matrix-matrix operations:

C := alpha*A*B + beta*C if side == rocblas_side_left,
C := alpha*B*A + beta*C if side == rocblas_side_right,

where alpha and beta are scalars, B and C are m by n matrices, and
A is a symmetric matrix stored as either upper or lower.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side  [rocblas_side]
- rocblas_side_left:      C := alpha*A*B + beta*C
- rocblas_side_right:     C := alpha*B*A + beta*C

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix
- rocblas_fill_lower:  A is a  lower triangular matrix

@param[in]
m       [rocblas_int]
m specifies the number of rows of B and C. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B and C. n >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A and B are not referenced.

@param[in]
A       pointer storing matrix A on the GPU.
- A is m by m if side == rocblas_side_left
- A is n by n if side == rocblas_side_right
only the upper/lower triangular part is accessed.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if side = rocblas_side_left,  lda >= max( 1, m ),
otherwise lda >= max( 1, n ).

@param[in]
B       pointer storing matrix B on the GPU.
Matrix dimension is m by n

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B. ldb >= max( 1, m ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       pointer storing matrix C on the GPU.
Matrix dimension is m by n

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, m ).
*/
    pub fn rocblas_ssymm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        B: *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        B: *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssymm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        B: *const f32,
        ldb: i64,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        B: *const f64,
        ldb: i64,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
symm_batched performs a batch of the matrix-matrix operations:

C_i := alpha*A_i*B_i + beta*C_i if side == rocblas_side_left,
C_i := alpha*B_i*A_i + beta*C_i if side == rocblas_side_right,

where alpha and beta are scalars, B_i and C_i are m by n matrices, and
A_i is a symmetric matrix stored as either upper or lower.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side  [rocblas_side]
- rocblas_side_left:      C_i := alpha*A_i*B_i + beta*C_i
- rocblas_side_right:     C_i := alpha*B_i*A_i + beta*C_i

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix
- rocblas_fill_lower:  A_i is a  lower triangular matrix

@param[in]
m       [rocblas_int]
m specifies the number of rows of B_i and C_i. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B_i and C_i. n >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A_i and B_i are not referenced.

@param[in]
A       device array of device pointers storing each matrix A_i on the GPU.
- A_i is m by m if side == rocblas_side_left
- A_i is n by n if side == rocblas_side_right
only the upper/lower triangular part is accessed.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if side = rocblas_side_left,  lda >= max( 1, m ),
otherwise lda >= max( 1, n ).

@param[in]
B       device array of device pointers storing each matrix B_i on the GPU.
Matrix dimension is m by n

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i. ldb >= max( 1, m ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C_i need not be set before entry.

@param[in]
C       device array of device pointers storing each matrix C_i on the GPU.
Matrix dimension is m by n.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C_i. ldc >= max( 1, m ).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssymm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        B: *const *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *const *mut f32,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        B: *const *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *const *mut f64,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssymm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        B: *const *const f32,
        ldb: i64,
        beta: *const f32,
        C: *const *mut f32,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        B: *const *const f64,
        ldb: i64,
        beta: *const f64,
        C: *const *mut f64,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
symm_strided_batched performs a batch of the matrix-matrix operations:

C_i := alpha*A_i*B_i + beta*C_i if side == rocblas_side_left,
C_i := alpha*B_i*A_i + beta*C_i if side == rocblas_side_right,

where alpha and beta are scalars, B_i and C_i are m by n matrices, and
A_i is a symmetric matrix stored as either upper or lower.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side  [rocblas_side]
- rocblas_side_left:      C_i := alpha*A_i*B_i + beta*C_i
- rocblas_side_right:     C_i := alpha*B_i*A_i + beta*C_i

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A_i is an upper triangular matrix
- rocblas_fill_lower:  A_i is a  lower triangular matrix

@param[in]
m       [rocblas_int]
m specifies the number of rows of B_i and C_i. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B_i and C_i. n >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A_i and B_i are not referenced.

@param[in]
A       device pointer to first matrix A_1
- A_i is m by m if side == rocblas_side_left
- A_i is n by n if side == rocblas_side_right
only the upper/lower triangular part is accessed.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if side = rocblas_side_left,  lda >= max( 1, m ),
otherwise lda >= max( 1, n ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).

@param[in]
B       device pointer to first matrix B_1 of dimension (ldb, n) on the GPU.

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i. ldb >= max( 1, m ).

@param[in]
stride_B  [rocblas_stride]
stride from the start of one matrix (B_i) and the next one (B_i+1).
@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C        device pointer to first matrix C_1 of dimension (ldc, n) on the GPU.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, m ).

@param[in, out]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssymm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const f32,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const f64,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssymm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const f32,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsymm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const f64,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csymm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsymm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
syrk performs one of the matrix-matrix operations for a symmetric rank-k update:

C := alpha*op( A )*op( A )^T + beta*C,

where  alpha and beta are scalars, op(A) is an n by k matrix, and
C is a symmetric n x n matrix stored as either upper or lower.

op( A ) = A, and A is n by k if transA == rocblas_operation_none
op( A ) = A^T and A is k by n if transA == rocblas_operation_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C is an upper triangular matrix
- rocblas_fill_lower:  C is a  lower triangular matrix

@param[in]
transA  [rocblas_operation]
- rocblas_operation_transpose:           op(A) = A^T
- rocblas_operation_none:                op(A) = A
- rocblas_operation_conjugate_transpose: op(A) = A^T

rocblas_operation_conjugate_transpose is not supported for complex types. See cherk
and zherk.

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       pointer storing matrix A on the GPU.
Matrix dimension is ( lda, k ) when if transA = rocblas_operation_none, otherwise (lda, n)

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if transA = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       pointer storing matrix C on the GPU.
only the upper/lower triangular part is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
*/
    pub fn rocblas_ssyrk(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrk(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrk(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrk(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyrk_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrk_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrk_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrk_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
syrk_batched performs a batch of the matrix-matrix operations for a symmetric rank-k update:

C_i := alpha*op( A_i )*op( A_i )^T + beta*C_i,

where  alpha and beta are scalars, op(A_i) is an n by k matrix, and
C_i is a symmetric n x n matrix stored as either upper or lower.

op( A_i ) = A_i, and A_i is n by k if transA == rocblas_operation_none
op( A_i ) = A_i^T and A_i is k by n if transA == rocblas_operation_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
transA  [rocblas_operation]
- rocblas_operation_transpose:           op(A) = A^T
- rocblas_operation_none:                op(A) = A
- rocblas_operation_conjugate_transpose: op(A) = A^T

rocblas_operation_conjugate_transpose is not supported for complex types. See cherk
and zherk.

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       device array of device pointers storing each matrix_i A of dimension (lda, k)
when transA is rocblas_operation_none, otherwise of dimension (lda, n).

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if transA = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       device array of device pointers storing each matrix C_i on the GPU.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyrk_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        beta: *const f32,
        C: *const *mut f32,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrk_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        beta: *const f64,
        C: *const *mut f64,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrk_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrk_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyrk_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        beta: *const f32,
        C: *const *mut f32,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrk_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        beta: *const f64,
        C: *const *mut f64,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrk_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrk_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
syrk_strided_batched performs a batch of the matrix-matrix operations for a symmetric rank-k update:

C_i := alpha*op( A_i )*op( A_i )^T + beta*C_i,

where  alpha and beta are scalars, op(A_i) is an n by k matrix, and
C_i is a symmetric n x n matrix stored as either upper or lower.

op( A_i ) = A_i, and A_i is n by k if transA == rocblas_operation_none
op( A_i ) = A_i^T and A_i is k by n if transA == rocblas_operation_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
transA  [rocblas_operation]
- rocblas_operation_transpose:           op(A) = A^T
- rocblas_operation_none:                op(A) = A
- rocblas_operation_conjugate_transpose: op(A) = A^T

rocblas_operation_conjugate_transpose is not supported for complex types. See cherk
and zherk.

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       Device pointer to the first matrix A_1 on the GPU of dimension (lda, k)
when transA is rocblas_operation_none, otherwise of dimension (lda, n).

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if transA = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       Device pointer to the first matrix C_1 on the GPU. on the GPU.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).

@param[in, out]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1)

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyrk_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrk_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrk_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrk_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyrk_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrk_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrk_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrk_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
syr2k performs one of the matrix-matrix operations for a symmetric rank-2k update:

C := alpha*(op( A )*op( B )^T + op( B )*op( A )^T) + beta*C,

where  alpha and beta are scalars, op(A) and op(B) are n by k matrix, and
C is a symmetric n x n matrix stored as either upper or lower.

op( A ) = A, op( B ) = B, and A and B are n by k if trans == rocblas_operation_none
op( A ) = A^T, op( B ) = B^T, and A and B are k by n if trans == rocblas_operation_transpose
or for ssyr2k and dsyr2k when trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C is an upper triangular matrix
- rocblas_fill_lower:  C is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_transpose:           op( A ) = A^T, op( B ) = B^T
- rocblas_operation_none:                op( A ) = A, op( B ) = B
- rocblas_operation_conjugate_transpose: op( A ) = A^T, op( B ) = B^T

rocblas_operation_conjugate_transpose is not supported for complex types in csyr2k and zsyr2k.

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A) and op(B). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       pointer storing matrix A on the GPU.
Matrix dimension is ( lda, k ) when if trans = rocblas_operation_none, otherwise (lda, n)
only the upper/lower triangular part is accessed.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
B       pointer storing matrix B on the GPU.
Matrix dimension is ( ldb, k ) when if trans = rocblas_operation_none, otherwise (ldb, n)
only the upper/lower triangular part is accessed.

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B.
if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).
@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       pointer storing matrix C on the GPU.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
*/
    pub fn rocblas_ssyr2k(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        B: *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2k(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        B: *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2k(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2k(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyr2k_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        B: *const f32,
        ldb: i64,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2k_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        B: *const f64,
        ldb: i64,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2k_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2k_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
syr2k_batched performs a batch of the matrix-matrix operations for a symmetric rank-2k update:

C_i := alpha*(op( A_i )*op( B_i )^T + op( B_i )*op( A_i )^T) + beta*C_i,

where  alpha and beta are scalars, op(A_i) and op(B_i) are n by k matrix, and
C_i is a symmetric n x n matrix stored as either upper or lower.

op( A_i ) = A_i, op( B_i ) = B_i, and A_i and B_i are n by k if trans == rocblas_operation_none
op( A_i ) = A_i^T, op( B_i ) = B_i^T, and A_i and B_i are k by n if trans == rocblas_operation_transpose
or for ssyr2k_batched and dsyr2k_batched when trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_transpose:           op( A_i ) = A_i^T, op( B_i ) = B_i^T
- rocblas_operation_none:                op( A_i ) = A_i, op( B_i ) = B_i
- rocblas_operation_conjugate_transpose: op( A_i ) = A_i^T, op( B_i ) = B_i^T

rocblas_operation_conjugate_transpose is not supported for complex types in csyr2k_batched and zsyr2k_batched.

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       device array of device pointers storing each matrix_i A of dimension (lda, k)
when trans is rocblas_operation_none, otherwise of dimension (lda, n).

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.
if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).
@param[in]
B       device array of device pointers storing each matrix_i B of dimension (ldb, k)
when trans is rocblas_operation_none, otherwise of dimension (ldb, n).
@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).
@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       device array of device pointers storing each matrix C_i on the GPU.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyr2k_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        B: *const *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *const *mut f32,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2k_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        B: *const *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *const *mut f64,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2k_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2k_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyr2k_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        B: *const *const f32,
        ldb: i64,
        beta: *const f32,
        C: *const *mut f32,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2k_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        B: *const *const f64,
        ldb: i64,
        beta: *const f64,
        C: *const *mut f64,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2k_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2k_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
syr2k_strided_batched performs a batch of the matrix-matrix operations for a symmetric rank-2k update:

C_i := alpha*(op( A_i )*op( B_i )^T + op( B_i )*op( A_i )^T) + beta*C_i,

where  alpha and beta are scalars, op(A_i) and op(B_i) are n by k matrix, and
C_i is a symmetric n x n matrix stored as either upper or lower.

op( A_i ) = A_i, op( B_i ) = B_i, and A_i and B_i are n by k if trans == rocblas_operation_none
op( A_i ) = A_i^T, op( B_i ) = B_i^T, and A_i and B_i are k by n if trans == rocblas_operation_transpose
or for ssyr2k_strided_batched and dsyr2k_strided_batched when trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_transpose:           op( A_i ) = A_i^T, op( B_i ) = B_i^T
- rocblas_operation_none:                op( A_i ) = A_i, op( B_i ) = B_i
- rocblas_operation_conjugate_transpose: op( A_i ) = A_i^T, op( B_i ) = B_i^T

rocblas_operation_conjugate_transpose is not supported for complex types in csyr2k_strided_batched and zsyr2k_strided_batched.

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       Device pointer to the first matrix A_1 on the GPU of dimension (lda, k)
when trans is rocblas_operation_none, otherwise of dimension (lda, n).

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1)

@param[in]
B       Device pointer to the first matrix B_1 on the GPU of dimension (ldb, k)
when trans is rocblas_operation_none, otherwise of dimension (ldb, n)

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).

@param[in]
stride_B  [rocblas_stride]
stride from the start of one matrix (B_i) and the next one (B_i+1)

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       Device pointer to the first matrix C_1 on the GPU.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).

@param[in, out]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyr2k_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const f32,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2k_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const f64,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2k_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2k_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyr2k_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const f32,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyr2k_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const f64,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyr2k_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyr2k_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
syrkx performs one of the matrix-matrix operations for a symmetric rank-k update:

C := alpha*op( A )*op( B )^T + beta*C,

where  alpha and beta are scalars, op(A) and op(B) are n by k matrix, and
C is a symmetric n x n matrix stored as either upper or lower.

This routine should only be used when the caller can guarantee that the result of op( A )*op( B )^T will be symmetric.

op( A ) = A, op( B ) = B, and A and B are n by k if trans == rocblas_operation_none
op( A ) = A^T, op( B ) = B^T,  and A and B are k by n if trans == rocblas_operation_transpose
or for ssyrkx and dsyrkx when trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C is an upper triangular matrix
- rocblas_fill_lower:  C is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_transpose:           op( A ) = A^T, op( B ) = B^T
- rocblas_operation_none:                op( A ) = A, op( B ) = B
- rocblas_operation_conjugate_transpose: op( A ) = A^T, op( B ) = B^T

rocblas_operation_conjugate_transpose is not supported for complex types in csyrkx and zsyrkx.

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A) and op(B). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       pointer storing matrix A on the GPU.
Matrix dimension is ( lda, k ) when if trans = rocblas_operation_none, otherwise (lda, n)

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
B       pointer storing matrix B on the GPU.
Matrix dimension is ( ldb, k ) when if trans = rocblas_operation_none, otherwise (ldb, n)

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       pointer storing matrix C on the GPU.
only the upper/lower triangular part is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).
*/
    pub fn rocblas_ssyrkx(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        B: *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrkx(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        B: *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrkx(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrkx(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyrkx_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        B: *const f32,
        ldb: i64,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrkx_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        B: *const f64,
        ldb: i64,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrkx_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrkx_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
syrkx_batched performs a batch of the matrix-matrix operations for a symmetric rank-k update:

C_i := alpha*op( A_i )*op( B_i )^T + beta*C_i,

where  alpha and beta are scalars, op(A_i) and op(B_i) are n by k matrix, and
C_i is a symmetric n x n matrix stored as either upper or lower.

This routine should only be used when the caller can guarantee that the result of op( A_i )*op( B_i )^T will be symmetric.

op( A_i ) = A_i, op( B_i ) = B_i, and A_i and B_i are n by k if trans == rocblas_operation_none
op( A_i ) = A_i^T, op( B_i ) = B_i^T,  and A_i and B_i are k by n if trans == rocblas_operation_transpose
or for ssyrkx_batched and dsyrkx_batched when trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_transpose:           op( A_i ) = A_i^T, op( B_i ) = B_i^T
- rocblas_operation_none:                op( A_i ) = A_i, op( B_i ) = B_i
- rocblas_operation_conjugate_transpose: op( A_i ) = A_i^T, op( B_i ) = B_i^T

rocblas_operation_conjugate_transpose is not supported for complex types in csyrkx_batched and zsyrkx_batched.

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       device array of device pointers storing each matrix_i A of dimension (lda, k)
when trans is rocblas_operation_none, otherwise of dimension (lda, n)

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
B       device array of device pointers storing each matrix_i B of dimension (ldb, k)
when trans is rocblas_operation_none, otherwise of dimension (ldb, n)

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       device array of device pointers storing each matrix C_i on the GPU.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyrkx_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        B: *const *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *const *mut f32,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrkx_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        B: *const *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *const *mut f64,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrkx_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrkx_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyrkx_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        B: *const *const f32,
        ldb: i64,
        beta: *const f32,
        C: *const *mut f32,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrkx_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        B: *const *const f64,
        ldb: i64,
        beta: *const f64,
        C: *const *mut f64,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrkx_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrkx_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
syrkx_strided_batched performs a batch of the matrix-matrix operations for a symmetric rank-k update:

C_i := alpha*op( A_i )*op( B_i )^T + beta*C_i,

where  alpha and beta are scalars, op(A_i) and op(B_i) are n by k matrix, and
C_i is a symmetric n x n matrix stored as either upper or lower.

This routine should only be used when the caller can guarantee that the result of op( A_i )*op( B_i )^T will be symmetric.

op( A_i ) = A_i, op( B_i ) = B_i, and A_i and B_i are n by k if trans == rocblas_operation_none
op( A_i ) = A_i^T, op( B_i ) = B_i^T,  and A_i and B_i are k by n if trans == rocblas_operation_transpose
or for ssyrkx_strided_batched and dsyrkx_strided_batched when trans == rocblas_operation_conjugate_transpose

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C_i is an upper triangular matrix
- rocblas_fill_lower:  C_i is a  lower triangular matrix

@param[in]
trans  [rocblas_operation]
- rocblas_operation_transpose:           op( A_i ) = A_i^T, op( B_i ) = B_i^T
- rocblas_operation_none:                op( A_i ) = A_i, op( B_i ) = B_i
- rocblas_operation_conjugate_transpose: op( A_i ) = A_i^T, op( B_i ) = B_i^T

rocblas_operation_conjugate_transpose is not supported for complex types in csyrkx_strided_batched and zsyrkx_strided_batched.

@param[in]
n       [rocblas_int]
n specifies the number of rows and columns of C_i. n >= 0.

@param[in]
k       [rocblas_int]
k specifies the number of columns of op(A). k >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and A need not be set before
entry.

@param[in]
A       Device pointer to the first matrix A_1 on the GPU of dimension (lda, k)
when trans is rocblas_operation_none, otherwise of dimension (lda, n)

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A_i.

if trans = rocblas_operation_none,  lda >= max( 1, n ),
otherwise lda >= max( 1, k ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).

@param[in]
B       Device pointer to the first matrix B_1 on the GPU of dimension (ldb, k)
when trans is rocblas_operation_none, otherwise of dimension (ldb, n).

@param[in]
ldb     [rocblas_int]
ldb specifies the first dimension of B_i.

if trans = rocblas_operation_none,  ldb >= max( 1, n ),
otherwise ldb >= max( 1, k ).

@param[in]
stride_B  [rocblas_stride]
stride from the start of one matrix (B_i) and the next one (B_i+1).

@param[in]
beta
beta specifies the scalar beta. When beta is
zero then C need not be set before entry.

@param[in]
C       Device pointer to the first matrix C_1 on the GPU.
only the upper/lower triangular part of each C_i is accessed.

@param[in]
ldc    [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, n ).

@param[in, out]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).

@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_ssyrkx_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const f32,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrkx_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const f64,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrkx_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrkx_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ssyrkx_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const f32,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dsyrkx_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const f64,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_csyrkx_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zsyrkx_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        trans: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
trmm performs one of the matrix-matrix operations:

C := alpha*op( A )*B,   or
C := alpha*B*op( A ),

The Legacy BLAS in-place trmm functionality,

B := alpha*op( A )*B,   or
B := alpha*B*op( A ),

is available by setting pointer C equal to pointer B, and ldc equal to ldb.

alpha  is a scalar,  B  is an m by n matrix, C  is an m by n matrix,  A  is a unit, or
non-unit,  upper or lower triangular matrix  and  op( A )  is one  of

op( A ) = A     or
op( A ) = A^T   or
op( A ) = A^H.

When uplo == rocblas_fill_upper the  leading  k by k
upper triangular part of the array  A must contain the upper
triangular matrix and the strictly lower triangular part of
A is not referenced. Here k is m when side == rocblas_side_left
and is n when side == rocblas_side_right.

When uplo == rocblas_fill_lower the  leading  k by k
lower triangular part of the array  A must contain the lower
triangular matrix  and the strictly upper triangular part of
A is not referenced. Here k is m when  side == rocblas_side_left
and is n when side == rocblas_side_right.

Note that when  diag == rocblas_diagonal_unit  the diagonal elements of
A  are not referenced either,  but are assumed to be  unity.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side    [rocblas_side]
Specifies whether op(A) multiplies B from the left or right as follows:
- rocblas_side_left:       C := alpha*op( A )*B
- rocblas_side_right:      C := alpha*B*op( A )

@param[in]
uplo    [rocblas_fill]
Specifies whether the matrix A is an upper or lower triangular matrix as follows:
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA  [rocblas_operation]
Specifies the form of op(A) to be used in the matrix multiplication as follows:
- rocblas_operation_none:    op(A) = A
- rocblas_operation_transpose:      op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
Specifies whether or not A is unit triangular as follows:
- rocblas_diagonal_unit:      A is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A is not assumed to be unit triangular.

@param[in]
m       [rocblas_int]
m specifies the number of rows of B. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B. n >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A is not referenced and B need not be set before
entry.

@param[in]
A       Device pointer to matrix A on the GPU.
A has dimension ( lda, k ), where k is m
when  side == rocblas_side_left  and
is  n  when  side == rocblas_side_right.

When uplo == rocblas_fill_upper the  leading  k by k
upper triangular part of the array  A must contain the upper
triangular matrix  and the strictly lower triangular part of
A is not referenced.

When uplo == rocblas_fill_lower the  leading  k by k
lower triangular part of the array  A must contain the lower
triangular matrix  and the strictly upper triangular part of
A is not referenced.

Note that when  diag == rocblas_diagonal_unit  the diagonal elements of
A  are not referenced either,  but are assumed to be  unity.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if side == rocblas_side_left,  lda >= max( 1, m ),
if side == rocblas_side_right, lda >= max( 1, n ).

@param[in]
B       Device pointer to the matrix B on the GPU.

@param[in]
ldb    [rocblas_int]
ldb specifies the first dimension of B. ldb >= max( 1, m ).

@param[out]
C      Device pointer to the matrix C on the GPU.

@param[in]
ldc   [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, m).
If B and C are pointers to the same matrix then ldc must equal ldb or
rocblas_status_invalid_value will be returned.
*/
    pub fn rocblas_strmm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        B: *const f32,
        ldb: rocblas_int,
        C: *mut f32,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        B: *const f64,
        ldb: rocblas_int,
        C: *mut f64,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strmm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        B: *const f32,
        ldb: i64,
        C: *mut f32,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        B: *const f64,
        ldb: i64,
        C: *mut f64,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *const rocblas_float_complex,
        ldb: i64,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *const rocblas_double_complex,
        ldb: i64,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
trmm_batched performs one of the matrix-matrix operations:

C_i := alpha*op( A_i )*B_i,   or
C_i := alpha*B_i*op( A_i )  for i = 0, 1, ... batch_count -1,

The Legacy BLAS in-place trmm_batched functionality,

B_i := alpha*op( A_i )*B_i,   or
B_i := alpha*B_i*op( A_i )  for i = 0, 1, ... batch_count -1,

is available by setting pointer C equal to pointer B and ldc equal to ldb.

alpha  is a scalar,  B_i  is an m by n matrix, C_i  is an m by n matrix,  A_i  is a unit, or
non-unit,  upper or lower triangular matrix  and  op( A_i )  is one  of

op( A_i ) = A_i     or
op( A_i ) = A_i^T   or
op( A_i ) = A_i^H.

When uplo == rocblas_fill_upper the  leading  k by k
upper triangular part of the array  A must contain the upper
triangular matrix and the strictly lower triangular part of
A is not referenced. Here k is m when side == rocblas_side_left
and is n when side == rocblas_side_right.

When uplo == rocblas_fill_lower the  leading  k by k
lower triangular part of the array  A must contain the lower
triangular matrix  and the strictly upper triangular part of
A is not referenced. Here k is m when  side == rocblas_side_left
and is n when side == rocblas_side_right.

Note that when  diag == rocblas_diagonal_unit  the diagonal elements of
A  are not referenced either,  but are assumed to be  unity.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side    [rocblas_side]
Specifies whether op(A_i) multiplies B_i from the left or right as follows:
- rocblas_side_left:       C_i := alpha*op( A_i )*B_i
- rocblas_side_right:      C_i := alpha*B_i*op( A_i )

@param[in]
uplo    [rocblas_fill]
Specifies whether the matrix A is an upper or lower triangular matrix as follows:
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA  [rocblas_operation]
Specifies the form of op(A_i) to be used in the matrix multiplication as follows:
- rocblas_operation_none:    op(A_i) = A_i
- rocblas_operation_transpose:      op(A_i) = A_i^T
- rocblas_operation_conjugate_transpose:  op(A_i) = A_i^H

@param[in]
diag    [rocblas_diagonal]
Specifies whether or not A_i is unit triangular as follows:
- rocblas_diagonal_unit:      A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A_i is not assumed to be unit triangular.

@param[in]
m       [rocblas_int]
m specifies the number of rows of B_i. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B_i. n >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A_i is not referenced and B_i need not be set before
entry.

@param[in]
A       Device array of device pointers storing each matrix A_i on the GPU.
Each A_i is of dimension ( lda, k ), where k is m
when  side == rocblas_side_left  and
is  n  when  side == rocblas_side_right.

When uplo == rocblas_fill_upper the  leading  k by k
upper triangular part of the array  A must contain the upper
triangular matrix  and the strictly lower triangular part of
A is not referenced.

When uplo == rocblas_fill_lower the  leading  k by k
lower triangular part of the array  A must contain the lower
triangular matrix  and the strictly upper triangular part of
A is not referenced.

Note that when  diag == rocblas_diagonal_unit  the diagonal elements of
A_i  are not referenced either,  but are assumed to be  unity.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if side == rocblas_side_left,  lda >= max( 1, m ),
if side == rocblas_side_right, lda >= max( 1, n ).

@param[in]
B       device array of device pointers storing each matrix B_i on the GPU.

@param[in]
ldb    [rocblas_int]
ldb specifies the first dimension of B_i. ldb >= max( 1, m ).

@param[out]
C      device array of device pointers storing each matrix C_i on the GPU.

@param[in]
ldc   [rocblas_int]
ldc specifies the first dimension of C. ldc >= max( 1, m).
If B and C are pointers to the same array of pointers then ldc must
equal ldb or rocblas_status_invalid_value will be returned.

@param[in]
batch_count [rocblas_int]
number of instances i in the batch.*/
    pub fn rocblas_strmm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        B: *const *const f32,
        ldb: rocblas_int,
        C: *const *mut f32,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        B: *const *const f64,
        ldb: rocblas_int,
        C: *const *mut f64,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strmm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        B: *const *const f32,
        ldb: i64,
        C: *const *mut f32,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        B: *const *const f64,
        ldb: i64,
        C: *const *mut f64,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
trmm_strided_batched performs one of the matrix-matrix operations:

C_i := alpha*op( A_i )*B_i,   or
C_i := alpha*B_i*op( A_i )  for i = 0, 1, ... batch_count -1,

The Legacy BLAS in-place trmm_strided_batched functionality,

B_i := alpha*op( A_i )*B_i,   or
B_i := alpha*B_i*op( A_i )  for i = 0, 1, ... batch_count -1,

is available by setting pointer C equal to pointer B, ldc equal to ldb, and stride_C equal to stride_B.

alpha  is a scalar,  B_i  is an m by n matrix, C_i  is an m by n matrix,  A_i  is a unit, or
non-unit,  upper or lower triangular matrix  and  op( A_i )  is one  of

op( A_i ) = A_i   or
op( A_i ) = A_i^T   or
op( A_i ) = A_i^H.

When uplo == rocblas_fill_upper the  leading  k by k
upper triangular part of the array  A must contain the upper
triangular matrix and the strictly lower triangular part of
A is not referenced. Here k is m when side == rocblas_side_left
and is n when side == rocblas_side_right.

When uplo == rocblas_fill_lower the  leading  k by k
lower triangular part of the array  A must contain the lower
triangular matrix  and the strictly upper triangular part of
A is not referenced. Here k is m when  side == rocblas_side_left
and is n when side == rocblas_side_right.

Note that when  diag == rocblas_diagonal_unit  the diagonal elements of
A  are not referenced either,  but are assumed to be  unity.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side    [rocblas_side]
Specifies whether op(A_i) multiplies B_i from the left or right as follows:
- rocblas_side_left:       C_i := alpha*op( A_i )*B_i
- rocblas_side_right:      C_i := alpha*B_i*op( A_i )

@param[in]
uplo    [rocblas_fill]
Specifies whether the matrix A is an upper or lower triangular matrix as follows:
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA  [rocblas_operation]
Specifies the form of op(A_i) to be used in the matrix multiplication as follows:
- rocblas_operation_none:    op(A_i) = A_i
- rocblas_operation_transpose:      op(A_i) = A_i^T
- rocblas_operation_conjugate_transpose:  op(A_i) = A_i^H

@param[in]
diag    [rocblas_diagonal]
Specifies whether or not A_i is unit triangular as follows:
- rocblas_diagonal_unit:      A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A_i is not assumed to be unit triangular.

@param[in]
m       [rocblas_int]
m specifies the number of rows of B_i. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B_i. n >= 0.

@param[in]
alpha
alpha specifies the scalar alpha. When alpha is
zero then A_i is not referenced and B_i need not be set before
entry.

@param[in]
A       Device pointer to the first matrix A_0 on the GPU.
Each A_i is of dimension ( lda, k ), where k is m
when  side == rocblas_side_left  and
is  n  when  side == rocblas_side_right.

When uplo == rocblas_fill_upper the  leading  k by k
upper triangular part of the array  A must contain the upper
triangular matrix  and the strictly lower triangular part of
A is not referenced.

When uplo == rocblas_fill_lower the  leading  k by k
lower triangular part of the array  A must contain the lower
triangular matrix  and the strictly upper triangular part of
A is not referenced.

Note that when  diag == rocblas_diagonal_unit  the diagonal elements of
A_i  are not referenced either,  but are assumed to be  unity.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if side == rocblas_side_left,  lda >= max( 1, m ),
if side == rocblas_side_right, lda >= max( 1, n ).

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).

@param[in]
B       Device pointer to the first matrix B_0 on the GPU.

@param[in]
ldb    [rocblas_int]
ldb specifies the first dimension of B_i. ldb >= max( 1, m ).

@param[in]
stride_B  [rocblas_stride]
stride from the start of one matrix (B_i) and the next one (B_i+1).

@param[out]
C      Device pointer to the first matrix C_0 on the GPU.

@param[in]
ldc   [rocblas_int]
ldc specifies the first dimension of C_i. ldc >= max( 1, m).
If B and C pointers are to the same matrix then ldc must equal ldb or
rocblas_status_invalid_size will be returned.

@param[in]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).
If B == C and ldb == ldc then stride_C should equal stride_B or
behavior is undefined.

@param[in]
batch_count [rocblas_int]
number of instances i in the batch.*/
    pub fn rocblas_strmm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const f32,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        C: *mut f32,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const f64,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        C: *mut f64,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strmm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const f32,
        ldb: i64,
        stride_B: rocblas_stride,
        C: *mut f32,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrmm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const f64,
        ldb: i64,
        stride_B: rocblas_stride,
        C: *mut f64,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrmm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrmm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
trtri  compute the inverse of a matrix A, namely, invA
and write the result into invA;

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'

if rocblas_fill_upper, the lower part of A is not referenced
if rocblas_fill_lower, the upper part of A is not referenced
@param[in]
diag      [rocblas_diagonal]
- 'rocblas_diagonal_non_unit', A is non-unit triangular;
- 'rocblas_diagonal_unit', A is unit triangular;
@param[in]
n         [rocblas_int]
size of matrix A and invA.
@param[in]
A         device pointer storing matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
@param[out]
invA      device pointer storing matrix invA.
Partial inplace operation is supported. See below:
-If UPLO = 'U', the leading N-by-N upper triangular part of the invA will store
the inverse of the upper triangular matrix, and the strictly lower
triangular part of invA may be cleared.
- If UPLO = 'L', the leading N-by-N lower triangular part of the invA will store
the inverse of the lower triangular matrix, and the strictly upper
triangular part of invA may be cleared.
@param[in]
ldinvA    [rocblas_int]
specifies the leading dimension of invA.*/
    pub fn rocblas_strtri(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        invA: *mut f32,
        ldinvA: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrtri(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        invA: *mut f64,
        ldinvA: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrtri(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        invA: *mut rocblas_float_complex,
        ldinvA: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrtri(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        invA: *mut rocblas_double_complex,
        ldinvA: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
trtri_batched  compute the inverse of A_i and write into invA_i where
A_i and invA_i are the i-th matrices in the batch,
for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
@param[in]
diag      [rocblas_diagonal]
- 'rocblas_diagonal_non_unit', A is non-unit triangular;
- 'rocblas_diagonal_unit', A is unit triangular;
@param[in]
n         [rocblas_int]
@param[in]
A         device array of device pointers storing each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[out]
invA      device array of device pointers storing the inverse of each matrix A_i.
Partial inplace operation is supported. See below:
-If UPLO = 'U', the leading N-by-N upper triangular part of the invA will store
the inverse of the upper triangular matrix, and the strictly lower
triangular part of invA may be cleared.
- If UPLO = 'L', the leading N-by-N lower triangular part of the invA will store
the inverse of the lower triangular matrix, and the strictly upper
triangular part of invA may be cleared.
@param[in]
ldinvA    [rocblas_int]
specifies the leading dimension of each invA_i.
@param[in]
batch_count [rocblas_int]
numbers of matrices in the batch.*/
    pub fn rocblas_strtri_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const f32,
        lda: rocblas_int,
        invA: *const *mut f32,
        ldinvA: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrtri_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const f64,
        lda: rocblas_int,
        invA: *const *mut f64,
        ldinvA: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrtri_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        invA: *const *mut rocblas_float_complex,
        ldinvA: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrtri_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        invA: *const *mut rocblas_double_complex,
        ldinvA: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
trtri_strided_batched compute the inverse of A_i and write into invA_i where
A_i and invA_i are the i-th matrices in the batch,
for i = 1, ..., batch_count.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo      [rocblas_fill]
specifies whether the upper 'rocblas_fill_upper' or lower 'rocblas_fill_lower'
@param[in]
diag      [rocblas_diagonal]
- 'rocblas_diagonal_non_unit', A is non-unit triangular;
- 'rocblas_diagonal_unit', A is unit triangular;
@param[in]
n         [rocblas_int]
@param[in]
A         device pointer pointing to address of first matrix A_1.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A.
@param[in]
stride_a  [rocblas_stride]
"batch stride a": stride from the start of one A_i matrix to the next A_(i + 1).
@param[out]
invA      device pointer storing the inverses of each matrix A_i.
Partial inplace operation is supported. See below:

- If UPLO = 'U', the leading N-by-N upper triangular part of the invA will store
the inverse of the upper triangular matrix, and the strictly lower
triangular part of invA may be cleared.

- If UPLO = 'L', the leading N-by-N lower triangular part of the invA will store
the inverse of the lower triangular matrix, and the strictly upper
triangular part of invA may be cleared.
@param[in]
ldinvA    [rocblas_int]
specifies the leading dimension of each invA_i.
@param[in]
stride_invA  [rocblas_stride]
"batch stride invA": stride from the start of one invA_i matrix to the next invA_(i + 1).
@param[in]
batch_count  [rocblas_int]
numbers of matrices in the batch.*/
    pub fn rocblas_strtri_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        invA: *mut f32,
        ldinvA: rocblas_int,
        stride_invA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrtri_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        invA: *mut f64,
        ldinvA: rocblas_int,
        stride_invA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrtri_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        invA: *mut rocblas_float_complex,
        ldinvA: rocblas_int,
        stride_invA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrtri_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        diag: rocblas_diagonal,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        invA: *mut rocblas_double_complex,
        ldinvA: rocblas_int,
        stride_invA: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
trsm solves:

op(A)*X = alpha*B or  X*op(A) = alpha*B,

where alpha is a scalar, X and B are m by n matrices,

A is triangular matrix and op(A) is one of

op( A ) = A   or   op( A ) = A^T   or   op( A ) = A^H.

The matrix X is overwritten on B.

Note about memory allocation:
When trsm is launched with a k evenly divisible by the internal block size of 128,
and is no larger than 10 of these blocks, the API takes advantage of utilizing pre-allocated
memory found in the handle to increase overall performance. This memory can be managed by using
the environment variable WORKBUF_TRSM_B_CHNK. When this variable is not set the device memory
used for temporary storage will default to 1 MB and may result in chunking, which in turn may
reduce performance. Under these circumstances it is recommended that WORKBUF_TRSM_B_CHNK be set
to the desired chunk of right hand sides to be used at a time
(where k is m when rocblas_side_left and is n when rocblas_side_right).

Although not widespread, some gemm kernels used by trsm may use atomic operations.
See Atomic Operations in the API Reference Guide for more information.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side    [rocblas_side]
- rocblas_side_left:       op(A)*X = alpha*B
- rocblas_side_right:      X*op(A) = alpha*B

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a  lower triangular matrix.

@param[in]
transA  [rocblas_operation]
- transB:    op(A) = A.
- rocblas_operation_transpose:      op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A is not assumed to be unit triangular.

@param[in]
m       [rocblas_int]
m specifies the number of rows of B. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B. n >= 0.

@param[in]
alpha
device pointer or host pointer specifying the scalar alpha. When alpha is
&zero then A is not referenced and B need not be set before
entry.

@param[in]
A       device pointer storing matrix A.
of dimension ( lda, k ), where k is m
when  rocblas_side_left  and
is  n  when  rocblas_side_right
only the upper/lower triangular part is accessed.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if side = rocblas_side_left,  lda >= max( 1, m ),
if side = rocblas_side_right, lda >= max( 1, n ).

@param[in,out]
B       device pointer storing matrix B.

@param[in]
ldb    [rocblas_int]
ldb specifies the first dimension of B. ldb >= max( 1, m ).
*/
    pub fn rocblas_strsm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        B: *mut f32,
        ldb: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        B: *mut f64,
        ldb: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *mut rocblas_float_complex,
        ldb: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsm(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *mut rocblas_double_complex,
        ldb: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strsm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        B: *mut f32,
        ldb: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        B: *mut f64,
        ldb: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *mut rocblas_float_complex,
        ldb: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *mut rocblas_double_complex,
        ldb: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
trsm_batched performs the following batched operation:

op(A_i)*X_i = alpha*B_i or
X_i*op(A_i) = alpha*B_i, for i = 1, ..., batch_count,

where alpha is a scalar, X and B are batched m by n matrices,

A is triangular batched matrix and op(A) is one of

op( A ) = A   or
op( A ) = A^T   or
op( A ) = A^H.

Each matrix X_i is overwritten on B_i for i = 1, ..., batch_count.

Note about memory allocation:
When trsm is launched with a k evenly divisible by the internal block size of 128,
and is no larger than 10 of these blocks, the API takes advantage of utilizing pre-allocated
memory found in the handle to increase overall performance. This memory can be managed by using
the environment variable WORKBUF_TRSM_B_CHNK. When this variable is not set the device memory
used for temporary storage will default to 1 MB and may result in chunking, which in turn may
reduce performance. Under these circumstances it is recommended that WORKBUF_TRSM_B_CHNK be set
to the desired chunk of right hand sides to be used at a time
(where k is m when rocblas_side_left and is n when rocblas_side_right).

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
side    [rocblas_side]
- rocblas_side_left:       op(A)*X = alpha*B
- rocblas_side_right:      X*op(A) = alpha*B
@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  each A_i is an upper triangular matrix.
- rocblas_fill_lower:  each A_i is a  lower triangular matrix.
@param[in]
transA  [rocblas_operation]
- transB:    op(A) = A
- rocblas_operation_transpose:      op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H
@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     each A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  each A_i is not assumed to be unit triangular.
@param[in]
m       [rocblas_int]
m specifies the number of rows of each B_i. m >= 0.
@param[in]
n       [rocblas_int]
n specifies the number of columns of each B_i. n >= 0.
@param[in]
alpha
device pointer or host pointer specifying the scalar alpha. When alpha is
&zero then A is not referenced and B need not be set before
entry.
@param[in]
A       device array of device pointers storing each matrix A_i on the GPU.
Matricies are of dimension ( lda, k ), where k is m
when  rocblas_side_left  and is  n  when  rocblas_side_right
only the upper/lower triangular part is accessed.
@param[in]
lda     [rocblas_int]
lda specifies the first dimension of each A_i.

if side = rocblas_side_left,  lda >= max( 1, m ),
if side = rocblas_side_right, lda >= max( 1, n ).
@param[in,out]
B       device array of device pointers storing each matrix B_i on the GPU.
@param[in]
ldb    [rocblas_int]
ldb specifies the first dimension of each B_i. ldb >= max( 1, m ).
@param[in]
batch_count [rocblas_int]
number of trsm operatons in the batch.*/
    pub fn rocblas_strsm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        B: *const *mut f32,
        ldb: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        B: *const *mut f64,
        ldb: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *mut rocblas_float_complex,
        ldb: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *mut rocblas_double_complex,
        ldb: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strsm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        B: *const *mut f32,
        ldb: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        B: *const *mut f64,
        ldb: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *mut rocblas_float_complex,
        ldb: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *mut rocblas_double_complex,
        ldb: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
trsm_srided_batched performs the following strided batched operation:

op(A_i)*X_i = alpha*B_i or
X_i*op(A_i) = alpha*B_i, for i = 1, ..., batch_count,

where alpha is a scalar, X and B are strided batched m by n matrices,

A is triangular strided batched matrix and op(A) is one of

op( A ) = A   or
op( A ) = A^T   or
op( A ) = A^H.

Each matrix X_i is overwritten on B_i for i = 1, ..., batch_count.

Note about memory allocation:
When trsm is launched with a k evenly divisible by the internal block size of 128,
and is no larger than 10 of these blocks, the API takes advantage of utilizing pre-allocated
memory found in the handle to increase overall performance. This memory can be managed by using
the environment variable WORKBUF_TRSM_B_CHNK. When this variable is not set the device memory
used for temporary storage will default to 1 MB and may result in chunking, which in turn may
reduce performance. Under these circumstances it is recommended that WORKBUF_TRSM_B_CHNK be set
to the desired chunk of right hand sides to be used at a time
(where k is m when rocblas_side_left and is n when rocblas_side_right).
@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
side    [rocblas_side]
- rocblas_side_left:       op(A)*X = alpha*B.
- rocblas_side_right:      X*op(A) = alpha*B.
@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  each A_i is an upper triangular matrix.
- rocblas_fill_lower:  each A_i is a  lower triangular matrix.
@param[in]
transA  [rocblas_operation]
- transB:    op(A) = A.
- rocblas_operation_transpose:      op(A) = A^T.
- rocblas_operation_conjugate_transpose:  op(A) = A^H.
@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     each A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  each A_i is not assumed to be unit triangular.
@param[in]
m       [rocblas_int]
m specifies the number of rows of each B_i. m >= 0.
@param[in]
n       [rocblas_int]
n specifies the number of columns of each B_i. n >= 0.
@param[in]
alpha
device pointer or host pointer specifying the scalar alpha. When alpha is
&zero then A is not referenced and B need not be set before
entry.
@param[in]
A       device pointer pointing to the first matrix A_1.
of dimension ( lda, k ), where k is m
when  rocblas_side_left  and
is  n  when  rocblas_side_right
only the upper/lower triangular part is accessed.
@param[in]
lda     [rocblas_int]
lda specifies the first dimension of each A_i.

if side = rocblas_side_left,  lda >= max( 1, m ).
if side = rocblas_side_right, lda >= max( 1, n ).
@param[in]
stride_a [rocblas_stride]
stride from the start of one A_i matrix to the next A_(i + 1).
@param[in,out]
B       device pointer pointing to the first matrix B_1.
@param[in]
ldb    [rocblas_int]
ldb specifies the first dimension of each B_i. ldb >= max( 1, m ).
@param[in]
stride_b [rocblas_stride]
stride from the start of one B_i matrix to the next B_(i + 1).
@param[in]
batch_count [rocblas_int]
number of trsm operatons in the batch.*/
    pub fn rocblas_strsm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *mut f32,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *mut f64,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *mut rocblas_float_complex,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *mut rocblas_double_complex,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_strsm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_a: rocblas_stride,
        B: *mut f32,
        ldb: i64,
        stride_b: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dtrsm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_a: rocblas_stride,
        B: *mut f64,
        ldb: i64,
        stride_b: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ctrsm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_a: rocblas_stride,
        B: *mut rocblas_float_complex,
        ldb: i64,
        stride_b: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ztrsm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_a: rocblas_stride,
        B: *mut rocblas_double_complex,
        ldb: i64,
        stride_b: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
gemm_kernel_name functions were never fully implemented and are deprecated for removal in a future release.

Returns rocblas_status_not_implemented.*/
    pub fn rocblas_hgemm_kernel_name(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_half,
        A: *const rocblas_half,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const rocblas_half,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const rocblas_half,
        C: *mut rocblas_half,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemm_kernel_name(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const f32,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemm_kernel_name(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const f64,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
gemm performs one of the matrix-matrix operations:

C = alpha*op( A )*op( B ) + beta*C,

where op( X ) is one of

op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,

alpha and beta are scalars, and A, B and C are matrices, with
op( A ) an m by k matrix, op( B ) a k by n matrix and C an m by n matrix.

Although not widespread, some gemm kernels may use atomic operations. See Atomic Operations
in the API Reference Guide for more information.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).
@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).
@param[in]
m         [rocblas_int]
number or rows of matrices op( A ) and C.
@param[in]
n         [rocblas_int]
number of columns of matrices op( B ) and C.
@param[in]
k         [rocblas_int]
number of columns of matrix op( A ) and number of rows of matrix op( B ).
@param[in]
alpha     device pointer or host pointer specifying the scalar alpha.
@param[in]
A         device pointer storing matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
@param[in]
B         device pointer storing matrix B.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of B.
@param[in]
beta      device pointer or host pointer specifying the scalar beta.
@param[in, out]
C         device pointer storing matrix C on the GPU.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C.
*/
    pub fn rocblas_sgemm(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        B: *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemm(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        B: *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hgemm(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_half,
        A: *const rocblas_half,
        lda: rocblas_int,
        B: *const rocblas_half,
        ldb: rocblas_int,
        beta: *const rocblas_half,
        C: *mut rocblas_half,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemm(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemm(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemm_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        B: *const f32,
        ldb: i64,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemm_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        B: *const f64,
        ldb: i64,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hgemm_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const rocblas_half,
        A: *const rocblas_half,
        lda: i64,
        B: *const rocblas_half,
        ldb: i64,
        beta: *const rocblas_half,
        C: *mut rocblas_half,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemm_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemm_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
gemm_batched performs one of the batched matrix-matrix operations:

C_i = alpha*op( A_i )*op( B_i ) + beta*C_i, for i = 1, ..., batch_count,

where op( X ) is one of

op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,

alpha and beta are scalars, and A, B and C are strided batched matrices, with

op( A ) an m by k by batch_count matrices,
op( B ) an k by n by batch_count matrices and
C an m by n by batch_count matrices.

@param[in]
handle    [rocblas_handle
handle to the rocblas library context queue.
@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).
@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).
@param[in]
m         [rocblas_int]
matrix dimention m.
@param[in]
n         [rocblas_int]
matrix dimention n.
@param[in]
k         [rocblas_int]
matrix dimention k.
@param[in]
alpha     device pointer or host pointer specifying the scalar alpha.
@param[in]
A         device array of device pointers storing each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
B         device array of device pointers storing each matrix B_i.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of each B_i.
@param[in]
beta      device pointer or host pointer specifying the scalar beta.
@param[in, out]
C         device array of device pointers storing each matrix C_i.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of each C_i.
@param[in]
batch_count
[rocblas_int]
number of gemm operations in the batch.*/
    pub fn rocblas_sgemm_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        B: *const *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *const *mut f32,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemm_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        B: *const *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *const *mut f64,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hgemm_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_half,
        A: *const *const rocblas_half,
        lda: rocblas_int,
        B: *const *const rocblas_half,
        ldb: rocblas_int,
        beta: *const rocblas_half,
        C: *const *mut rocblas_half,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemm_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemm_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemm_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        B: *const *const f32,
        ldb: i64,
        beta: *const f32,
        C: *const *mut f32,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemm_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        B: *const *const f64,
        ldb: i64,
        beta: *const f64,
        C: *const *mut f64,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hgemm_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const rocblas_half,
        A: *const *const rocblas_half,
        lda: i64,
        B: *const *const rocblas_half,
        ldb: i64,
        beta: *const rocblas_half,
        C: *const *mut rocblas_half,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemm_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemm_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
gemm_strided_batched performs one of the strided batched matrix-matrix operations:

C_i = alpha*op( A_i )*op( B_i ) + beta*C_i, for i = 1, ..., batch_count,

where op( X ) is one of

op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,

alpha and beta are scalars, and A, B and C are strided batched matrices, with
op( A ) an m by k by batch_count strided_batched matrix,
op( B ) an k by n by batch_count strided_batched matrix and
C an m by n by batch_count strided_batched matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).
@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).
@param[in]
m         [rocblas_int]
matrix dimention m.
@param[in]
n         [rocblas_int]
matrix dimention n.
@param[in]
k         [rocblas_int]
matrix dimention k.
@param[in]
alpha     device pointer or host pointer specifying the scalar alpha.
@param[in]
A         device pointer pointing to the first matrix A_1.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
stride_a  [rocblas_stride]
stride from the start of one A_i matrix to the next A_(i + 1).
@param[in]
B         device pointer pointing to the first matrix B_1.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of each B_i.
@param[in]
stride_b  [rocblas_stride]
stride from the start of one B_i matrix to the next B_(i + 1).
@param[in]
beta      device pointer or host pointer specifying the scalar beta.
@param[in, out]
C         device pointer pointing to the first matrix C_1.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of each C_i.
@param[in]
stride_c  [rocblas_stride]
stride from the start of one C_i matrix to the next C_(i + 1).
@param[in]
batch_count
[rocblas_int]
number of gemm operatons in the batch.
*/
    pub fn rocblas_sgemm_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const f32,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemm_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const f64,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hgemm_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_half,
        A: *const rocblas_half,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const rocblas_half,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const rocblas_half,
        C: *mut rocblas_half,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemm_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemm_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemm_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_a: rocblas_stride,
        B: *const f32,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
        stride_c: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemm_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_a: rocblas_stride,
        B: *const f64,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
        stride_c: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_hgemm_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const rocblas_half,
        A: *const rocblas_half,
        lda: i64,
        stride_a: rocblas_stride,
        B: *const rocblas_half,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const rocblas_half,
        C: *mut rocblas_half,
        ldc: i64,
        stride_c: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemm_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_a: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_c: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemm_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_a: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_c: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
dgmm performs one of the matrix-matrix operations:

C = A * diag(x) if side == rocblas_side_right
C = diag(x) * A if side == rocblas_side_left

where C and A are m by n dimensional matrices. diag( x ) is a diagonal matrix
and x is vector of dimension n if side == rocblas_side_right and dimension m
if side == rocblas_side_left.


@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
side      [rocblas_side]
specifies the side of diag(x).
@param[in]
m         [rocblas_int]
matrix dimension m.
@param[in]
n         [rocblas_int]
matrix dimension n.
@param[in]
A         device pointer storing matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
@param[in]
x         device pointer storing vector x.
@param[in]
incx      [rocblas_int]
specifies the increment between values of x
@param[in, out]
C         device pointer storing matrix C.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C.
*/
    pub fn rocblas_sdgmm(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        x: *const f32,
        incx: rocblas_int,
        C: *mut f32,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddgmm(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        x: *const f64,
        incx: rocblas_int,
        C: *mut f64,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdgmm(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdgmm(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sdgmm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const f32,
        lda: i64,
        x: *const f32,
        incx: i64,
        C: *mut f32,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddgmm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const f64,
        lda: i64,
        x: *const f64,
        incx: i64,
        C: *mut f64,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdgmm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        x: *const rocblas_float_complex,
        incx: i64,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdgmm_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        x: *const rocblas_double_complex,
        incx: i64,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
dgmm_batched performs one of the batched matrix-matrix operations:

C_i = A_i * diag(x_i) for i = 0, 1, ... batch_count-1 if side == rocblas_side_right
C_i = diag(x_i) * A_i for i = 0, 1, ... batch_count-1 if side == rocblas_side_left,

where C_i and A_i are m by n dimensional matrices. diag(x_i) is a diagonal matrix
and x_i is vector of dimension n if side == rocblas_side_right and dimension m
if side == rocblas_side_left.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
side      [rocblas_side]
specifies the side of diag(x).
@param[in]
m         [rocblas_int]
matrix dimension m.
@param[in]
n         [rocblas_int]
matrix dimension n.
@param[in]
A         device array of device pointers storing each matrix A_i on the GPU.
Each A_i is of dimension ( lda, n ).
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A_i.
@param[in]
x         device array of device pointers storing each vector x_i on the GPU.
Each x_i is of dimension n if side == rocblas_side_right and dimension
m if side == rocblas_side_left.
@param[in]
incx      [rocblas_int]
specifies the increment between values of x_i.
@param[in, out]
C         device array of device pointers storing each matrix C_i on the GPU.
Each C_i is of dimension ( ldc, n ).
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
*/
    pub fn rocblas_sdgmm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const *const f32,
        lda: rocblas_int,
        x: *const *const f32,
        incx: rocblas_int,
        C: *const *mut f32,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddgmm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const *const f64,
        lda: rocblas_int,
        x: *const *const f64,
        incx: rocblas_int,
        C: *const *mut f64,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdgmm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        x: *const *const rocblas_float_complex,
        incx: rocblas_int,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdgmm_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        x: *const *const rocblas_double_complex,
        incx: rocblas_int,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sdgmm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const *const f32,
        lda: i64,
        x: *const *const f32,
        incx: i64,
        C: *const *mut f32,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddgmm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const *const f64,
        lda: i64,
        x: *const *const f64,
        incx: i64,
        C: *const *mut f64,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdgmm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const *const rocblas_float_complex,
        lda: i64,
        x: *const *const rocblas_float_complex,
        incx: i64,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdgmm_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const *const rocblas_double_complex,
        lda: i64,
        x: *const *const rocblas_double_complex,
        incx: i64,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
dgmm_strided_batched performs one of the batched matrix-matrix operations:

C_i = A_i * diag(x_i)   if side == rocblas_side_right   for i = 0, 1, ... batch_count-1
C_i = diag(x_i) * A_i   if side == rocblas_side_left    for i = 0, 1, ... batch_count-1,

where C_i and A_i are m by n dimensional matrices. diag(x_i) is a diagonal matrix
and x_i is vector of dimension n if side == rocblas_side_right and dimension m
if side == rocblas_side_left.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
side      [rocblas_side]
specifies the side of diag(x).
@param[in]
m         [rocblas_int]
matrix dimension m.
@param[in]
n         [rocblas_int]
matrix dimension n.
@param[in]
A         device pointer to the first matrix A_0 on the GPU.
Each A_i is of dimension ( lda, n ).
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).
@param[in]
x         pointer to the first vector x_0 on the GPU.
Each x_i is of dimension n if side == rocblas_side_right and dimension
m if side == rocblas_side_left.
@param[in]
incx      [rocblas_int]
specifies the increment between values of x.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector(x_i) and the next one (x_i+1).
@param[in, out]
C         device pointer to the first matrix C_0 on the GPU.
Each C_i is of dimension ( ldc, n ).
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C.
@param[in]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).
@param[in]
batch_count [rocblas_int]
number of instances i in the batch.
*/
    pub fn rocblas_sdgmm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const f32,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        C: *mut f32,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddgmm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const f64,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        C: *mut f64,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdgmm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdgmm_strided_batched(
        handle: rocblas_handle,
        side: rocblas_side,
        m: rocblas_int,
        n: rocblas_int,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sdgmm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const f32,
        incx: i64,
        stride_x: rocblas_stride,
        C: *mut f32,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_ddgmm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const f64,
        incx: i64,
        stride_x: rocblas_stride,
        C: *mut f64,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cdgmm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const rocblas_float_complex,
        incx: i64,
        stride_x: rocblas_stride,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zdgmm_strided_batched_64(
        handle: rocblas_handle,
        side: rocblas_side,
        m: i64,
        n: i64,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        x: *const rocblas_double_complex,
        incx: i64,
        stride_x: rocblas_stride,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
geam performs one of the matrix-matrix operations:

C = alpha*op( A ) + beta*op( B ),

where op( X ) is one of

op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,

alpha and beta are scalars, and A, B and C are matrices, with
op( A ) an m by n matrix, op( B ) an m by n matrix, and C an m by n matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).
@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).
@param[in]
m         [rocblas_int]
matrix dimension m.
@param[in]
n         [rocblas_int]
matrix dimension n.
@param[in]
alpha     device pointer or host pointer specifying the scalar alpha.
@param[in]
A         device pointer storing matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
@param[in]
beta      device pointer or host pointer specifying the scalar beta.
@param[in]
B         device pointer storing matrix B.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of B.
@param[in, out]
C         device pointer storing matrix C.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C.
*/
    pub fn rocblas_sgeam(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        beta: *const f32,
        B: *const f32,
        ldb: rocblas_int,
        C: *mut f32,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgeam(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        beta: *const f64,
        B: *const f64,
        ldb: rocblas_int,
        C: *mut f64,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeam(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        beta: *const rocblas_float_complex,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeam(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        beta: *const rocblas_double_complex,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgeam_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        beta: *const f32,
        B: *const f32,
        ldb: i64,
        C: *mut f32,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgeam_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        beta: *const f64,
        B: *const f64,
        ldb: i64,
        C: *mut f64,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeam_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        beta: *const rocblas_float_complex,
        B: *const rocblas_float_complex,
        ldb: i64,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeam_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        beta: *const rocblas_double_complex,
        B: *const rocblas_double_complex,
        ldb: i64,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
geam_batched performs one of the batched matrix-matrix operations:

C_i = alpha*op( A_i ) + beta*op( B_i )  for i = 0, 1, ... batch_count - 1,

where alpha and beta are scalars, and op(A_i), op(B_i) and C_i are m by n matrices
and op( X ) is one of

op( X ) = X      or
op( X ) = X**T

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).
@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).
@param[in]
m         [rocblas_int]
matrix dimension m.
@param[in]
n         [rocblas_int]
matrix dimension n.
@param[in]
alpha     device pointer or host pointer specifying the scalar alpha.
@param[in]
A         device array of device pointers storing each matrix A_i on the GPU.
Each A_i is of dimension ( lda, k ), where k is m
when  transA == rocblas_operation_none and
is  n  when  transA == rocblas_operation_transpose.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
@param[in]
beta      device pointer or host pointer specifying the scalar beta.
@param[in]
B         device array of device pointers storing each matrix B_i on the GPU.
Each B_i is of dimension ( ldb, k ), where k is m
when  transB == rocblas_operation_none and
is  n  when  transB == rocblas_operation_transpose.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of B.
@param[in, out]
C         device array of device pointers storing each matrix C_i on the GPU.
Each C_i is of dimension ( ldc, n ).
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C.

@param[in]
batch_count [rocblas_int]
number of instances i in the batch.
*/
    pub fn rocblas_sgeam_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        beta: *const f32,
        B: *const *const f32,
        ldb: rocblas_int,
        C: *const *mut f32,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgeam_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        beta: *const f64,
        B: *const *const f64,
        ldb: rocblas_int,
        C: *const *mut f64,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeam_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        beta: *const rocblas_float_complex,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeam_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        beta: *const rocblas_double_complex,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgeam_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        beta: *const f32,
        B: *const *const f32,
        ldb: i64,
        C: *const *mut f32,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgeam_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        beta: *const f64,
        B: *const *const f64,
        ldb: i64,
        C: *const *mut f64,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeam_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        beta: *const rocblas_float_complex,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeam_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        beta: *const rocblas_double_complex,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
geam_strided_batched performs one of the batched matrix-matrix operations:

C_i = alpha*op( A_i ) + beta*op( B_i )  for i = 0, 1, ... batch_count - 1,

where alpha and beta are scalars, and op(A_i), op(B_i) and C_i are m by n matrices
and op( X ) is one of

op( X ) = X      or
op( X ) = X**T

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).

@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).

@param[in]
m         [rocblas_int]
matrix dimension m.

@param[in]
n         [rocblas_int]
matrix dimension n.

@param[in]
alpha     device pointer or host pointer specifying the scalar alpha.

@param[in]
A         device pointer to the first matrix A_0 on the GPU.
Each A_i is of dimension ( lda, k ), where k is m
when  transA == rocblas_operation_none and
is  n  when  transA == rocblas_operation_transpose.

@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.

@param[in]
stride_A  [rocblas_stride]
stride from the start of one matrix (A_i) and the next one (A_i+1).

@param[in]
beta      device pointer or host pointer specifying the scalar beta.

@param[in]
B         pointer to the first matrix B_0 on the GPU.
Each B_i is of dimension ( ldb, k ), where k is m
when  transB == rocblas_operation_none and
is  n  when  transB == rocblas_operation_transpose.

@param[in]
ldb       [rocblas_int]
specifies the leading dimension of B.

@param[in]
stride_B  [rocblas_stride]
stride from the start of one matrix (B_i) and the next one (B_i+1)

@param[in, out]
C         pointer to the first matrix C_0 on the GPU.
Each C_i is of dimension ( ldc, n ).

@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C.

@param[in]
stride_C  [rocblas_stride]
stride from the start of one matrix (C_i) and the next one (C_i+1).

@param[in]
batch_count [rocblas_int]
number of instances i in the batch.
*/
    pub fn rocblas_sgeam_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const f32,
        B: *const f32,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        C: *mut f32,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgeam_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const f64,
        B: *const f64,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        C: *mut f64,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeam_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const rocblas_float_complex,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeam_strided_batched(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        beta: *const rocblas_double_complex,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_C: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgeam_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const f32,
        B: *const f32,
        ldb: i64,
        stride_B: rocblas_stride,
        C: *mut f32,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgeam_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const f64,
        B: *const f64,
        ldb: i64,
        stride_B: rocblas_stride,
        C: *mut f64,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgeam_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const rocblas_float_complex,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgeam_strided_batched_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_A: rocblas_stride,
        beta: *const rocblas_double_complex,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_B: rocblas_stride,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_C: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
gemm_ex performs one of the matrix-matrix operations:

D = alpha*op( A )*op( B ) + beta*C,

where op( X ) is one of

op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,

alpha and beta are scalars, and A, B, C, and D are matrices, with
op( A ) an m by k matrix, op( B ) a k by n matrix and C and D are m by n matrices.
C and D may point to the same matrix if their parameters are identical.

Supported types are as follows:
- rocblas_datatype_f64_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f32_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f16_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f16_r = a_type = b_type = c_type = d_type; rocblas_datatype_f32_r =
compute_type
- rocblas_datatype_f16_r = a_type = b_type; rocblas_datatype_f32_r = c_type = d_type =
compute_type
- rocblas_datatype_bf16_r = a_type = b_type = c_type = d_type; rocblas_datatype_f32_r =
compute_type
- rocblas_datatype_bf16_r = a_type = b_type; rocblas_datatype_f32_r = c_type = d_type =
compute_type
- rocblas_datatype_i8_r = a_type = b_type; rocblas_datatype_i32_r = c_type = d_type =
compute_type
- rocblas_datatype_f32_c  = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f64_c  = a_type = b_type = c_type = d_type = compute_type

Although not widespread, some gemm kernels used by gemm_ex may use atomic operations.
See Atomic Operations in the API Reference Guide for more information.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).
@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).
@param[in]
m         [rocblas_int]
matrix dimension m.
@param[in]
n         [rocblas_int]
matrix dimension n.
@param[in]
k         [rocblas_int]
matrix dimension k.
@param[in]
alpha     [const void *]
device pointer or host pointer specifying the scalar alpha. Same datatype as compute_type.
@param[in]
a         [void *]
device pointer storing matrix A.
@param[in]
a_type    [rocblas_datatype]
specifies the datatype of matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A.
@param[in]
b         [void *]
device pointer storing matrix B.
@param[in]
b_type    [rocblas_datatype]
specifies the datatype of matrix B.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of B.
@param[in]
beta      [const void *]
device pointer or host pointer specifying the scalar beta. Same datatype as compute_type.
@param[in]
c         [void *]
device pointer storing matrix C.
@param[in]
c_type    [rocblas_datatype]
specifies the datatype of matrix C.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C.
@param[out]
d         [void *]
device pointer storing matrix D.
If d and c pointers are to the same matrix then d_type must equal c_type and ldd must equal ldc
or the respective invalid status will be returned.
@param[in]
d_type    [rocblas_datatype]
specifies the datatype of matrix D.
@param[in]
ldd       [rocblas_int]
specifies the leading dimension of D.
@param[in]
compute_type
[rocblas_datatype]
specifies the datatype of computation.
@param[in]
algo      [rocblas_gemm_algo]
enumerant specifying the algorithm type.
@param[in]
solution_index
[int32_t]
if algo is rocblas_gemm_algo_solution_index, this controls which solution is used.
When algo is not rocblas_gemm_algo_solution_index, or if solution_index <= 0, the default solution is used.
This parameter was unused in previous releases and instead always used the default solution
@param[in]
flags     [uint32_t]
optional gemm flags.
*/
    pub fn rocblas_gemm_ex(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        a: *const ::core::ffi::c_void,
        a_type: rocblas_datatype,
        lda: rocblas_int,
        b: *const ::core::ffi::c_void,
        b_type: rocblas_datatype,
        ldb: rocblas_int,
        beta: *const ::core::ffi::c_void,
        c: *const ::core::ffi::c_void,
        c_type: rocblas_datatype,
        ldc: rocblas_int,
        d: *mut ::core::ffi::c_void,
        d_type: rocblas_datatype,
        ldd: rocblas_int,
        compute_type: rocblas_datatype,
        algo: rocblas_gemm_algo,
        solution_index: i32,
        flags: u32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_gemm_ex_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const ::core::ffi::c_void,
        a: *const ::core::ffi::c_void,
        a_type: rocblas_datatype,
        lda: i64,
        b: *const ::core::ffi::c_void,
        b_type: rocblas_datatype,
        ldb: i64,
        beta: *const ::core::ffi::c_void,
        c: *const ::core::ffi::c_void,
        c_type: rocblas_datatype,
        ldc: i64,
        d: *mut ::core::ffi::c_void,
        d_type: rocblas_datatype,
        ldd: i64,
        compute_type: rocblas_datatype,
        algo: rocblas_gemm_algo,
        solution_index: i32,
        flags: u32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
gemm_batched_ex performs one of the batched matrix-matrix operations:
D_i = alpha*op(A_i)*op(B_i) + beta*C_i, for i = 1, ..., batch_count.
where op( X ) is one of
op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,
alpha and beta are scalars, and A, B, C, and D are batched pointers to matrices, with
op( A ) an m by k by batch_count batched matrix,
op( B ) a k by n by batch_count batched matrix and
C and D are m by n by batch_count batched matrices.
The batched matrices are an array of pointers to matrices.
The number of pointers to matrices is batch_count.
C and D may point to the same matrices if their parameters are identical.

Supported types are as follows:
- rocblas_datatype_f64_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f32_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f16_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f16_r = a_type = b_type = c_type = d_type; rocblas_datatype_f32_r =
compute_type
- rocblas_datatype_bf16_r = a_type = b_type = c_type = d_type; rocblas_datatype_f32_r =
compute_type
- rocblas_datatype_i8_r = a_type = b_type; rocblas_datatype_i32_r = c_type = d_type =
compute_type
- rocblas_datatype_f32_c  = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f64_c  = a_type = b_type = c_type = d_type = compute_type

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).
@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).
@param[in]
m         [rocblas_int]
matrix dimension m.
@param[in]
n         [rocblas_int]
matrix dimension n.
@param[in]
k         [rocblas_int]
matrix dimension k.
@param[in]
alpha     [const void *]
device pointer or host pointer specifying the scalar alpha. Same datatype as compute_type.
@param[in]
a         [void *]
device pointer storing array of pointers to each matrix A_i.
@param[in]
a_type    [rocblas_datatype]
specifies the datatype of each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
b         [void *]
device pointer storing array of pointers to each matrix B_i.
@param[in]
b_type    [rocblas_datatype]
specifies the datatype of each matrix B_i.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of each B_i.
@param[in]
beta      [const void *]
device pointer or host pointer specifying the scalar beta. Same datatype as compute_type.
@param[in]
c         [void *]
device array of device pointers to each matrix C_i.
@param[in]
c_type    [rocblas_datatype]
specifies the datatype of each matrix C_i.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of each C_i.
@param[out]
d         [void *]
device array of device pointers to each matrix D_i.
If d and c are the same array of matrix pointers then d_type must equal c_type and ldd must equal ldc
or the respective invalid status will be returned.
@param[in]
d_type    [rocblas_datatype]
specifies the datatype of each matrix D_i.
@param[in]
ldd       [rocblas_int]
specifies the leading dimension of each D_i.
@param[in]
batch_count
[rocblas_int]
number of gemm operations in the batch.
@param[in]
compute_type
[rocblas_datatype]
specifies the datatype of computation.
@param[in]
algo      [rocblas_gemm_algo]
enumerant specifying the algorithm type.
@param[in]
solution_index
[int32_t]
if algo is rocblas_gemm_algo_solution_index, this controls which solution is used.
When algo is not rocblas_gemm_algo_solution_index, or if solution_index <= 0, the default solution is used.
This parameter was unused in previous releases and instead always used the default solution
@param[in]
flags     [uint32_t]
optional gemm flags.
*/
    pub fn rocblas_gemm_batched_ex(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        a: *const ::core::ffi::c_void,
        a_type: rocblas_datatype,
        lda: rocblas_int,
        b: *const ::core::ffi::c_void,
        b_type: rocblas_datatype,
        ldb: rocblas_int,
        beta: *const ::core::ffi::c_void,
        c: *const ::core::ffi::c_void,
        c_type: rocblas_datatype,
        ldc: rocblas_int,
        d: *mut ::core::ffi::c_void,
        d_type: rocblas_datatype,
        ldd: rocblas_int,
        batch_count: rocblas_int,
        compute_type: rocblas_datatype,
        algo: rocblas_gemm_algo,
        solution_index: i32,
        flags: u32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_gemm_batched_ex_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const ::core::ffi::c_void,
        a: *const ::core::ffi::c_void,
        a_type: rocblas_datatype,
        lda: i64,
        b: *const ::core::ffi::c_void,
        b_type: rocblas_datatype,
        ldb: i64,
        beta: *const ::core::ffi::c_void,
        c: *const ::core::ffi::c_void,
        c_type: rocblas_datatype,
        ldc: i64,
        d: *mut ::core::ffi::c_void,
        d_type: rocblas_datatype,
        ldd: i64,
        batch_count: i64,
        compute_type: rocblas_datatype,
        algo: rocblas_gemm_algo,
        solution_index: i32,
        flags: u32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
gemm_strided_batched_ex performs one of the strided_batched matrix-matrix operations:

D_i = alpha*op(A_i)*op(B_i) + beta*C_i, for i = 1, ..., batch_count

where op( X ) is one of

op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,

alpha and beta are scalars, and A, B, C, and D are strided_batched matrices, with
op( A ) an m by k by batch_count strided_batched matrix,
op( B ) a k by n by batch_count strided_batched matrix and
C and D are m by n by batch_count strided_batched matrices.
C and D may point to the same matrices if their parameters are identical.

The strided_batched matrices are multiple matrices separated by a constant stride.
The number of matrices is batch_count.

Supported types are as follows:
- rocblas_datatype_f64_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f32_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f16_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f16_r = a_type = b_type = c_type = d_type; rocblas_datatype_f32_r =
compute_type
- rocblas_datatype_bf16_r = a_type = b_type = c_type = d_type; rocblas_datatype_f32_r =
compute_type
- rocblas_datatype_i8_r = a_type = b_type; rocblas_datatype_i32_r = c_type = d_type =
compute_type
- rocblas_datatype_f32_c  = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f64_c  = a_type = b_type = c_type = d_type = compute_type

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).
@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).
@param[in]
m         [rocblas_int]
matrix dimension m.
@param[in]
n         [rocblas_int]
matrix dimension n.
@param[in]
k         [rocblas_int]
matrix dimension k.
@param[in]
alpha     [const void *]
device pointer or host pointer specifying the scalar alpha. Same datatype as compute_type.
@param[in]
a         [void *]
device pointer pointing to first matrix A_1.
@param[in]
a_type    [rocblas_datatype]
specifies the datatype of each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i.
@param[in]
stride_a  [rocblas_stride]
specifies stride from start of one A_i matrix to the next A_(i + 1).
@param[in]
b         [void *]
device pointer pointing to first matrix B_1.
@param[in]
b_type    [rocblas_datatype]
specifies the datatype of each matrix B_i.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of each B_i.
@param[in]
stride_b  [rocblas_stride]
specifies stride from start of one B_i matrix to the next B_(i + 1).
@param[in]
beta      [const void *]
device pointer or host pointer specifying the scalar beta. Same datatype as compute_type.
@param[in]
c         [void *]
device pointer pointing to first matrix C_1.
@param[in]
c_type    [rocblas_datatype]
specifies the datatype of each matrix C_i.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of each C_i.
@param[in]
stride_c  [rocblas_stride]
specifies stride from start of one C_i matrix to the next C_(i + 1).
@param[out]
d         [void *]
device pointer storing each matrix D_i.
If d and c pointers are to the same matrix then d_type must equal c_type and ldd must equal ldc
and stride_d must equal stride_c or the respective invalid status will be returned.
@param[in]
d_type    [rocblas_datatype]
specifies the datatype of each matrix D_i.
@param[in]
ldd       [rocblas_int]
specifies the leading dimension of each D_i.
@param[in]
stride_d  [rocblas_stride]
specifies stride from start of one D_i matrix to the next D_(i + 1).
@param[in]
batch_count
[rocblas_int]
number of gemm operations in the batch.
@param[in]
compute_type
[rocblas_datatype]
specifies the datatype of computation.
@param[in]
algo      [rocblas_gemm_algo]
enumerant specifying the algorithm type.
@param[in]
solution_index
[int32_t]
if algo is rocblas_gemm_algo_solution_index, this controls which solution is used.
When algo is not rocblas_gemm_algo_solution_index, or if solution_index <= 0, the default solution is used.
This parameter was unused in previous releases and instead always used the default solution
@param[in]
flags     [uint32_t]
optional gemm flags.
*/
    pub fn rocblas_gemm_strided_batched_ex(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        a: *const ::core::ffi::c_void,
        a_type: rocblas_datatype,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        b: *const ::core::ffi::c_void,
        b_type: rocblas_datatype,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const ::core::ffi::c_void,
        c: *const ::core::ffi::c_void,
        c_type: rocblas_datatype,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        d: *mut ::core::ffi::c_void,
        d_type: rocblas_datatype,
        ldd: rocblas_int,
        stride_d: rocblas_stride,
        batch_count: rocblas_int,
        compute_type: rocblas_datatype,
        algo: rocblas_gemm_algo,
        solution_index: i32,
        flags: u32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_gemm_strided_batched_ex_64(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: i64,
        n: i64,
        k: i64,
        alpha: *const ::core::ffi::c_void,
        a: *const ::core::ffi::c_void,
        a_type: rocblas_datatype,
        lda: i64,
        stride_a: rocblas_stride,
        b: *const ::core::ffi::c_void,
        b_type: rocblas_datatype,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const ::core::ffi::c_void,
        c: *const ::core::ffi::c_void,
        c_type: rocblas_datatype,
        ldc: i64,
        stride_c: rocblas_stride,
        d: *mut ::core::ffi::c_void,
        d_type: rocblas_datatype,
        ldd: i64,
        stride_d: rocblas_stride,
        batch_count: i64,
        compute_type: rocblas_datatype,
        algo: rocblas_gemm_algo,
        solution_index: i32,
        flags: u32,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
gemmt performs matrix-matrix operations and updates the upper or lower triangular part of the result matrix:

C = alpha*op( A )*op( B ) + beta*C,

where op( X ) is one of

op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,

alpha and beta are scalars. A, B  are general matrices and C is either an upper or lower triangular matrix, with
op( A ) an n by k matrix, op( B ) a k by n matrix and C an n by n matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C is an upper triangular matrix
- rocblas_fill_lower:  C is a  lower triangular matrix
@param[in]
transA    [rocblas_operation]
- rocblas_operation_none:    op(A) = A.
- rocblas_operation_transpose:      op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H
@param[in]
transB    [rocblas_operation]
- rocblas_operation_none:    op(B) = B.
- rocblas_operation_transpose:      op(B) = B^T
- rocblas_operation_conjugate_transpose:  op(B) = B^H
@param[in]
n         [rocblas_int]
number or rows of matrices op( A ), columns of op( B ), and (rows, columns) of C.
@param[in]
k         [rocblas_int]
number of rows of matrices op( B ) and columns of op( A ).
@param[in]
alpha     device pointer or host pointer specifying the scalar alpha.
@param[in]
A         device pointer storing matrix A. If transa = rocblas_operation_none, then, the leading n-by-k part of the array contains the matrix A, otherwise the leading k-by-n part of the array contains the matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A. If transA == rocblas_operation_none, must have lda >= max(1, n), otherwise, must have lda >= max(1, k).
@param[in]
B         device pointer storing matrix B. If transB = rocblas_operation_none, then, the leading k-by-n part of the array contains the matrix B, otherwise the leading n-by-k part of the array contains the matrix B.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of B. If transB == rocblas_operation_none, must have ldb >= max(1, k), otherwise, must have ldb >= max(1, n)
@param[in]
beta      device pointer or host pointer specifying the scalar beta.
@param[in, out]
C         device pointer storing matrix C on the GPU. If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains the matrix C, otherwise the lower triangular part of the leading n-by-n array contains the matrix C.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C. Must have ldc >= max(1, n).
*/
    pub fn rocblas_sgemmt(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        B: *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemmt(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        B: *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemmt(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemmt(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemmt_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        B: *const f32,
        ldb: i64,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemmt_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        B: *const f64,
        ldb: i64,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemmt_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        B: *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemmt_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        B: *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
gemmt_batched performs matrix-matrix operations and updates the upper or lower triangular part of the result matrix:

C_i = alpha*op( A_i )*op( B_i ) + beta*C_i, for i = 1, ..., batch_count,

where op( X ) is one of

op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,

alpha and beta are scalars. A, B  are general matrices and C is either an upper or lower triangular matrix, with

op( A ) an n by k by batch_count matrices,
op( B ) an k by n by batch_count matrices and
C an n by n by batch_count matrices.

@param[in]
handle    [rocblas_handle
handle to the rocblas library context queue.
@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C is an upper triangular matrix
- rocblas_fill_lower:  C is a  lower triangular matrix
@param[in]
transA    [rocblas_operation]
- rocblas_operation_none:    op(A_i) = A_i.
- rocblas_operation_transpose:      op(A_i) = A_i^T
- rocblas_operation_conjugate_transpose:  op(A_i) = A_i^H
@param[in]
transB    [rocblas_operation]
- rocblas_operation_none:    op(B_i) = B_i.
- rocblas_operation_transpose:      op(B_i) = B_i^T
- rocblas_operation_conjugate_transpose:  op(B_i) = B_i^H
@param[in]
n         [rocblas_int]
number or rows of matrices op( A_i ), columns of op( B_i ), and (rows, columns) of C_i.
@param[in]
k         [rocblas_int]
number of rows of matrices op( B_i ) and columns of op( A_i ).
@param[in]
alpha     device pointer or host pointer specifying the scalar alpha.
@param[in]
A         device array of device pointers storing each matrix A_i. If transa = rocblas_operation_none, then, the leading n-by-k part of the array contains each matrix A_i, otherwise the leading k-by-n part of the array contains each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. If transA == rocblas_operation_none, must have lda >= max(1, n), otherwise, must have lda >= max(1, k).
@param[in]
B         device array of device pointers storing each matrix B_i. If transB = rocblas_operation_none, then, the leading k-by-n part of the array contains each matrix B_i, otherwise the leading n-by-k part of the array contains each matrix B_i.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of each B_i. If transB == rocblas_operation_none, must have ldb >= max(1, k), otherwise, must have ldb >= max(1, n).
@param[in]
beta      device pointer or host pointer specifying the scalar beta.
@param[in, out]
C         device array of device pointers storing each matrix C_i. If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains each matrix C_i, otherwise the lower triangular part of the leading n-by-n array contains each matrix C_i.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of each C_i. Must have ldc >= max(1, n).
@param[in]
batch_count
[rocblas_int]
number of gemm operations in the batch.*/
    pub fn rocblas_sgemmt_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: rocblas_int,
        B: *const *const f32,
        ldb: rocblas_int,
        beta: *const f32,
        C: *const *mut f32,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemmt_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: rocblas_int,
        B: *const *const f64,
        ldb: rocblas_int,
        beta: *const f64,
        C: *const *mut f64,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemmt_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: rocblas_int,
        B: *const *const rocblas_float_complex,
        ldb: rocblas_int,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemmt_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: rocblas_int,
        B: *const *const rocblas_double_complex,
        ldb: rocblas_int,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: rocblas_int,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemmt_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: i64,
        B: *const *const f32,
        ldb: i64,
        beta: *const f32,
        C: *const *mut f32,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemmt_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: i64,
        B: *const *const f64,
        ldb: i64,
        beta: *const f64,
        C: *const *mut f64,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemmt_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const *const rocblas_float_complex,
        lda: i64,
        B: *const *const rocblas_float_complex,
        ldb: i64,
        beta: *const rocblas_float_complex,
        C: *const *mut rocblas_float_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemmt_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const *const rocblas_double_complex,
        lda: i64,
        B: *const *const rocblas_double_complex,
        ldb: i64,
        beta: *const rocblas_double_complex,
        C: *const *mut rocblas_double_complex,
        ldc: i64,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 3 API </b>

\details
gemmt_strided_batched performs matrix-matrix operations and updates the upper or lower triangular part of the result matrix:

C_i = alpha*op( A_i )*op( B_i ) + beta*C_i, for i = 1, ..., batch_count,

where op( X ) is one of

op( X ) = X      or
op( X ) = X**T   or
op( X ) = X**H,

alpha and beta are scalars. A, B  are general matrices and C is either an upper or lower triangular matrix, with
op( A ) an n by k by batch_count strided_batched matrix,
op( B ) an k by n by batch_count strided_batched matrix and
C an n by n by batch_count strided_batched matrix.

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  C is an upper triangular matrix
- rocblas_fill_lower:  C is a  lower triangular matrix
@param[in]
transA    [rocblas_operation]
- rocblas_operation_none:    op(A_i) = A_i.
- rocblas_operation_transpose:      op(A_i) = A_i^T
- rocblas_operation_conjugate_transpose:  op(A_i) = A_i^H
@param[in]
transB    [rocblas_operation]
- rocblas_operation_none:    op(B_i) = B_i.
- rocblas_operation_transpose:      op(B_i) = B_i^T
- rocblas_operation_conjugate_transpose:  op(B_i) = B_i^H
@param[in]
n         [rocblas_int]
number or rows of matrices op( A_i ), columns of op( B_i ), and (rows, columns) of C_i.
@param[in]
k         [rocblas_int]
number of rows of matrices op( B_i ) and columns of op( A_i ).
@param[in]
alpha     device pointer or host pointer specifying the scalar alpha.
@param[in]
A         device array of device pointers storing each matrix A_i. If transa = rocblas_operation_none, then, the leading n-by-k part of the array contains each matrix A_i, otherwise the leading k-by-n part of the array contains each matrix A_i.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of each A_i. If transA == rocblas_operation_none, must have lda >= max(1, n), otherwise, must have lda >= max(1, k).
@param[in]
stride_a  [rocblas_stride]
stride from the start of one A_i matrix to the next A_(i + 1).
@param[in]
B         device array of device pointers storing each matrix B_i. If transB = rocblas_operation_none, then, the leading k-by-n part of the array contains each matrix B_i, otherwise the leading n-by-k part of the array contains each matrix B_i.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of each B_i. If transB == rocblas_operation_none, must have ldb >= max(1, k), otherwise, must have ldb >= max(1, n).
@param[in]
stride_b  [rocblas_stride]
stride from the start of one B_i matrix to the next B_(i + 1).
@param[in]
beta      device pointer or host pointer specifying the scalar beta.
@param[in, out]
C         device array of device pointers storing each matrix C_i. If uplo == rocblas_fill_upper, the upper triangular part of the leading n-by-n array contains each matrix C_i, otherwise the lower triangular part of the leading n-by-n array contains each matrix C_i.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of each C_i. Must have ldc >= max(1, n).
@param[in]
stride_c  [rocblas_stride]
stride from the start of one C_i matrix to the next C_(i + 1).
@param[in]
batch_count
[rocblas_int]
number of gemm operatons in the batch.
*/
    pub fn rocblas_sgemmt_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f32,
        A: *const f32,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const f32,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemmt_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const f64,
        A: *const f64,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const f64,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemmt_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemmt_strided_batched(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: rocblas_int,
        stride_a: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: rocblas_int,
        stride_b: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: rocblas_int,
        stride_c: rocblas_stride,
        batch_count: rocblas_int,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_sgemmt_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f32,
        A: *const f32,
        lda: i64,
        stride_a: rocblas_stride,
        B: *const f32,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const f32,
        C: *mut f32,
        ldc: i64,
        stride_c: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dgemmt_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const f64,
        A: *const f64,
        lda: i64,
        stride_a: rocblas_stride,
        B: *const f64,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const f64,
        C: *mut f64,
        ldc: i64,
        stride_c: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_cgemmt_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_float_complex,
        A: *const rocblas_float_complex,
        lda: i64,
        stride_a: rocblas_stride,
        B: *const rocblas_float_complex,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const rocblas_float_complex,
        C: *mut rocblas_float_complex,
        ldc: i64,
        stride_c: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_zgemmt_strided_batched_64(
        handle: rocblas_handle,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        transB: rocblas_operation,
        n: i64,
        k: i64,
        alpha: *const rocblas_double_complex,
        A: *const rocblas_double_complex,
        lda: i64,
        stride_a: rocblas_stride,
        B: *const rocblas_double_complex,
        ldb: i64,
        stride_b: rocblas_stride,
        beta: *const rocblas_double_complex,
        C: *mut rocblas_double_complex,
        ldc: i64,
        stride_c: rocblas_stride,
        batch_count: i64,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
geam_ex performs one of the matrix-matrix operations:

Dij = min(alpha * (Aik + Bkj), beta * Cij)
Dij = min(alpha * Aik, alpha * Bkj) + beta * Cij

alpha and beta are scalars, and A, B, C, and D are matrices, with
op( A ) an m by k matrix, op( B ) a k by n matrix and C and D are m by n matrices.
C and D may point to the same matrix if their type and leading dimensions are identical.

Aik refers to the element at the i-th row and k-th column of op( A ), Bkj refers to
the element at the k-th row and j-th column of op( B ), and Cij/Dij refers to the element
at the i-th row and j-th column of C/D.

Supported types are as follows:
- rocblas_datatype_f64_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f32_r = a_type = b_type = c_type = d_type = compute_type
- rocblas_datatype_f16_r = a_type = b_type = c_type = d_type = compute_type

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
transA    [rocblas_operation]
specifies the form of op( A ).
@param[in]
transB    [rocblas_operation]
specifies the form of op( B ).
@param[in]
m         [rocblas_int]
matrix dimension m.
@param[in]
n         [rocblas_int]
matrix dimension n.
@param[in]
k         [rocblas_int]
matrix dimension k.
@param[in]
alpha     [const void *]
device pointer or host pointer specifying the scalar alpha. Same datatype as compute_type.
@param[in]
A         [void *]
device pointer storing matrix A.
@param[in]
a_type    [rocblas_datatype]
specifies the datatype of matrix A.
@param[in]
lda       [rocblas_int]
specifies the leading dimension of A

if transA == N, must have lda >= max(1, m)
otherwise, must have lda >= max(1, k)
@param[in]
B         [void *]
device pointer storing matrix B.
@param[in]
b_type    [rocblas_datatype]
specifies the datatype of matrix B.
@param[in]
ldb       [rocblas_int]
specifies the leading dimension of B

if transB == N, must have ldb >= max(1, k)
otherwise, must have ldb >= max(1, n)
@param[in]
beta      [const void *]
device pointer or host pointer specifying the scalar beta. Same datatype as compute_type.
@param[in]
C         [void *]
device pointer storing matrix C.
@param[in]
c_type    [rocblas_datatype]
specifies the datatype of matrix C.
@param[in]
ldc       [rocblas_int]
specifies the leading dimension of C, must have ldc >= max(1, m).
@param[out]
D         [void *]
device pointer storing matrix D.
If D and C pointers are to the same matrix then d_type must equal c_type and ldd must equal ldc
or the respective invalid status will be returned.
@param[in]
d_type    [rocblas_datatype]
specifies the datatype of matrix D.
@param[in]
ldd       [rocblas_int]
specifies the leading dimension of D, must have ldd >= max(1, m).
@param[in]
compute_type
[rocblas_datatype]
specifies the datatype of computation.
@param[in]
geam_ex_op [rocblas_geam_ex_operation]
enumerant specifying the operation type, support for rocblas_geam_ex_operation_min_plus and rocblas_geam_ex_operation_plus_min.
*/
    pub fn rocblas_geam_ex(
        handle: rocblas_handle,
        transA: rocblas_operation,
        transB: rocblas_operation,
        m: rocblas_int,
        n: rocblas_int,
        k: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        a_type: rocblas_datatype,
        lda: rocblas_int,
        B: *const ::core::ffi::c_void,
        b_type: rocblas_datatype,
        ldb: rocblas_int,
        beta: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        c_type: rocblas_datatype,
        ldc: rocblas_int,
        D: *mut ::core::ffi::c_void,
        d_type: rocblas_datatype,
        ldd: rocblas_int,
        compute_type: rocblas_datatype,
        geam_ex_op: rocblas_geam_ex_operation,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
trsm_ex solves:

op(A)*X = alpha*B or X*op(A) = alpha*B,

where alpha is a scalar, X and B are m by n matrices,
A is triangular matrix and op(A) is one of

op( A ) = A   or   op( A ) = A^T   or   op( A ) = A^H.

The matrix X is overwritten on B.

This function gives the user the ability to reuse the invA matrix between runs.
If invA == NULL, rocblas_trsm_ex will automatically calculate invA on every run.

Setting up invA:
The accepted invA matrix consists of the packed 128x128 inverses of the diagonal blocks of
matrix A, followed by any smaller diagonal block that remains.
To set up invA it is recommended that rocblas_trtri_batched be used with matrix A as the input.

Device memory of size 128 x k should be allocated for invA ahead of time, where k is m when
rocblas_side_left and is n when rocblas_side_right. The actual number of elements in invA
should be passed as invA_size.

To begin, rocblas_trtri_batched must be called on the full 128x128-sized diagonal blocks of
matrix A. Below are the restricted parameters:
- n = 128
- ldinvA = 128
- stride_invA = 128x128
- batch_count = k / 128,

Then any remaining block may be added:
- n = k % 128
- invA = invA + stride_invA * previous_batch_count
- ldinvA = 128
- batch_count = 1

Although not widespread, some gemm kernels used by trsm_ex may use atomic operations.
See Atomic Operations in the API Reference Guide for more information.

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side    [rocblas_side]
- rocblas_side_left:       op(A)*X = alpha*B
- rocblas_side_right:      X*op(A) = alpha*B

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  A is an upper triangular matrix.
- rocblas_fill_lower:  A is a lower triangular matrix.

@param[in]
transA  [rocblas_operation]
- transB:    op(A) = A.
- rocblas_operation_transpose:      op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     A is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  A is not assumed to be unit triangular.

@param[in]
m       [rocblas_int]
m specifies the number of rows of B. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of B. n >= 0.

@param[in]
alpha   [void *]
device pointer or host pointer specifying the scalar alpha. When alpha is
&zero then A is not referenced, and B need not be set before
entry.

@param[in]
A       [void *]
device pointer storing matrix A.
of dimension ( lda, k ), where k is m
when rocblas_side_left and
is n when rocblas_side_right
only the upper/lower triangular part is accessed.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if side = rocblas_side_left,  lda >= max( 1, m ),
if side = rocblas_side_right, lda >= max( 1, n ).

@param[in, out]
B       [void *]
device pointer storing matrix B.
B is of dimension ( ldb, n ).
Before entry, the leading m by n part of the array B must
contain the right-hand side matrix B, and on exit is
overwritten by the solution matrix X.

@param[in]
ldb    [rocblas_int]
ldb specifies the first dimension of B. ldb >= max( 1, m ).

@param[in]
invA    [void *]
device pointer storing the inverse diagonal blocks of A.
invA is of dimension ( ld_invA, k ), where k is m
when rocblas_side_left and
is n when rocblas_side_right.
ld_invA must be equal to 128.

@param[in]
invA_size [rocblas_int]
invA_size specifies the number of elements of device memory in invA.

@param[in]
compute_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_trsm_ex(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: rocblas_int,
        B: *mut ::core::ffi::c_void,
        ldb: rocblas_int,
        invA: *const ::core::ffi::c_void,
        invA_size: rocblas_int,
        compute_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
trsm_batched_ex solves:

op(A_i)*X_i = alpha*B_i or X_i*op(A_i) = alpha*B_i,

for i = 1, ..., batch_count; and where alpha is a scalar, X and B are arrays of m by n matrices,
A is an array of triangular matrix and each op(A_i) is one of

op( A_i ) = A_i   or   op( A_i ) = A_i^T   or   op( A_i ) = A_i^H.

Each matrix X_i is overwritten on B_i.

This function gives the user the ability to reuse the invA matrix between runs.
If invA == NULL, rocblas_trsm_batched_ex will automatically calculate each invA_i on every run.

Setting up invA:
Each accepted invA_i matrix consists of the packed 128x128 inverses of the diagonal blocks of
matrix A_i, followed by any smaller diagonal block that remains.
To set up each invA_i it is recommended that rocblas_trtri_batched be used with matrix A_i as the input.
invA is an array of pointers of batch_count length holding each invA_i.

Device memory of size 128 x k should be allocated for each invA_i ahead of time, where k is m when
rocblas_side_left and is n when rocblas_side_right. The actual number of elements in each invA_i
should be passed as invA_size.

To begin, rocblas_trtri_batched must be called on the full 128x128-sized diagonal blocks of each
matrix A_i. Below are the restricted parameters:
- n = 128
- ldinvA = 128
- stride_invA = 128x128
- batch_count = k / 128,

Then any remaining block may be added:
- n = k % 128
- invA = invA + stride_invA * previous_batch_count
- ldinvA = 128
- batch_count = 1

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side    [rocblas_side]
- rocblas_side_left:       op(A)*X = alpha*B
- rocblas_side_right:      X*op(A) = alpha*B

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  each A_i is an upper triangular matrix.
- rocblas_fill_lower:  each A_i is a lower triangular matrix.

@param[in]
transA  [rocblas_operation]
- transB:    op(A) = A.
- rocblas_operation_transpose:      op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     each A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  each A_i is not assumed to be unit triangular.

@param[in]
m       [rocblas_int]
m specifies the number of rows of each B_i. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of each B_i. n >= 0.

@param[in]
alpha   [void *]
device pointer or host pointer alpha specifying the scalar alpha. When alpha is
&zero then A is not referenced, and B need not be set before
entry.

@param[in]
A       [void *]
device array of device pointers storing each matrix A_i.
each A_i is of dimension ( lda, k ), where k is m
when rocblas_side_left and
is n when rocblas_side_right
only the upper/lower triangular part is accessed.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of each A_i.

if side = rocblas_side_left,  lda >= max( 1, m ),
if side = rocblas_side_right, lda >= max( 1, n ).

@param[in, out]
B       [void *]
device array of device pointers storing each matrix B_i.
each B_i is of dimension ( ldb, n ).
Before entry, the leading m by n part of the array B_i must
contain the right-hand side matrix B_i, and on exit is
overwritten by the solution matrix X_i

@param[in]
ldb    [rocblas_int]
ldb specifies the first dimension of each B_i. ldb >= max( 1, m ).

@param[in]
batch_count [rocblas_int]
specifies how many batches.

@param[in]
invA    [void *]
device array of device pointers storing the inverse diagonal blocks of each A_i.
each invA_i is of dimension ( ld_invA, k ), where k is m
when rocblas_side_left and
is n when rocblas_side_right.
ld_invA must be equal to 128.

@param[in]
invA_size [rocblas_int]
invA_size specifies the number of elements of device memory in each invA_i.

@param[in]
compute_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_trsm_batched_ex(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: rocblas_int,
        B: *mut ::core::ffi::c_void,
        ldb: rocblas_int,
        batch_count: rocblas_int,
        invA: *const ::core::ffi::c_void,
        invA_size: rocblas_int,
        compute_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
trsm_strided_batched_ex solves:

op(A_i)*X_i = alpha*B_i or X_i*op(A_i) = alpha*B_i,

for i = 1, ..., batch_count; and where alpha is a scalar, X and B are strided batched m by n matrices,
A is a strided batched triangular matrix and op(A_i) is one of

op( A_i ) = A_i   or   op( A_i ) = A_i^T   or   op( A_i ) = A_i^H.

Each matrix X_i is overwritten on B_i.

This function gives the user the ability to reuse each invA_i matrix between runs.
If invA == NULL, rocblas_trsm_batched_ex will automatically calculate each invA_i on every run.

Setting up invA:
Each accepted invA_i matrix consists of the packed 128x128 inverses of the diagonal blocks of
matrix A_i, followed by any smaller diagonal block that remains.
To set up invA_i it is recommended that rocblas_trtri_batched be used with matrix A_i as the input.
invA is a contiguous piece of memory holding each invA_i.

Device memory of size 128 x k should be allocated for each invA_i ahead of time, where k is m when
rocblas_side_left and is n when rocblas_side_right. The actual number of elements in each invA_i
should be passed as invA_size.

To begin, rocblas_trtri_batched must be called on the full 128x128-sized diagonal blocks of each
matrix A_i. Below are the restricted parameters:
- n = 128
- ldinvA = 128
- stride_invA = 128x128
- batch_count = k / 128

Then any remaining block may be added:
- n = k % 128
- invA = invA + stride_invA * previous_batch_count
- ldinvA = 128
- batch_count = 1

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.

@param[in]
side    [rocblas_side]
- rocblas_side_left:       op(A)*X = alpha*B
- rocblas_side_right:      X*op(A) = alpha*B

@param[in]
uplo    [rocblas_fill]
- rocblas_fill_upper:  each A_i is an upper triangular matrix.
- rocblas_fill_lower:  each A_i is a lower triangular matrix.

@param[in]
transA  [rocblas_operation]
- transB:    op(A) = A.
- rocblas_operation_transpose:      op(A) = A^T
- rocblas_operation_conjugate_transpose:  op(A) = A^H

@param[in]
diag    [rocblas_diagonal]
- rocblas_diagonal_unit:     each A_i is assumed to be unit triangular.
- rocblas_diagonal_non_unit:  each A_i is not assumed to be unit triangular.

@param[in]
m       [rocblas_int]
m specifies the number of rows of each B_i. m >= 0.

@param[in]
n       [rocblas_int]
n specifies the number of columns of each B_i. n >= 0.

@param[in]
alpha   [void *]
device pointer or host pointer specifying the scalar alpha. When alpha is
&zero then A is not referenced, and B need not be set before
entry.

@param[in]
A       [void *]
device pointer storing matrix A.
of dimension ( lda, k ), where k is m
when rocblas_side_left and
is n when rocblas_side_right
only the upper/lower triangular part is accessed.

@param[in]
lda     [rocblas_int]
lda specifies the first dimension of A.

if side = rocblas_side_left,  lda >= max( 1, m ),
if side = rocblas_side_right, lda >= max( 1, n ).

@param[in]
stride_A [rocblas_stride]
The stride between each A matrix.

@param[in, out]
B       [void *]
device pointer pointing to first matrix B_i.
each B_i is of dimension ( ldb, n ).
Before entry, the leading m by n part of each array B_i must
contain the right-hand side of matrix B_i, and on exit is
overwritten by the solution matrix X_i.

@param[in]
ldb    [rocblas_int]
ldb specifies the first dimension of each B_i. ldb >= max( 1, m ).

@param[in]
stride_B [rocblas_stride]
The stride between each B_i matrix.

@param[in]
batch_count [rocblas_int]
specifies how many batches.

@param[in]
invA    [void *]
device pointer storing the inverse diagonal blocks of each A_i.
invA points to the first invA_1.
each invA_i is of dimension ( ld_invA, k ), where k is m
when rocblas_side_left and
is n when rocblas_side_right.
ld_invA must be equal to 128.

@param[in]
invA_size [rocblas_int]
invA_size specifies the number of elements of device memory in each invA_i.

@param[in]
stride_invA [rocblas_stride]
The stride between each invA matrix.

@param[in]
compute_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_trsm_strided_batched_ex(
        handle: rocblas_handle,
        side: rocblas_side,
        uplo: rocblas_fill,
        transA: rocblas_operation,
        diag: rocblas_diagonal,
        m: rocblas_int,
        n: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: rocblas_int,
        stride_A: rocblas_stride,
        B: *mut ::core::ffi::c_void,
        ldb: rocblas_int,
        stride_B: rocblas_stride,
        batch_count: rocblas_int,
        invA: *const ::core::ffi::c_void,
        invA_size: rocblas_int,
        stride_invA: rocblas_stride,
        compute_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
axpy_ex   computes constant alpha multiplied by vector x, plus vector y.

y := alpha * x + y

Currently supported datatypes are as follows:

-------------------------------------------------
| alpha_type | x_type | y_type | execution_type |
|------------|--------|--------|----------------|
|  bf16_r    | bf16_r |  bf16_r|      f32_r     |
|  f32_r     | bf16_r |  bf16_r|      f32_r     |
|  f16_r     | f16_r  |  f16_r |      f16_r     |
|  f16_r     | f16_r  |  f16_r |      f32_r     |
|  f32_r     | f16_r  |  f16_r |      f32_r     |
|  f32_r     | f32_r  |  f32_r |      f32_r     |
|  f64_r     | f64_r  |  f64_r |      f64_r     |
|  f32_c     | f32_c  |  f32_c |      f32_c     |
|  f64_c     | f64_c  |  f64_c |      f64_c     |
-------------------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x and y.
@param[in]
alpha     device pointer or host pointer to specify the scalar alpha.
@param[in]
alpha_type [rocblas_datatype]
specifies the datatype of alpha.
@param[in]
x         device pointer storing vector x.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in, out]
y         device pointer storing vector y.
@param[in]
y_type [rocblas_datatype]
specifies the datatype of vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_axpy_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_axpy_ex_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
axpy_batched_ex   computes constant alpha multiplied by vector x, plus vector y over
a set of batched vectors.

y := alpha * x + y

Currently supported datatypes are as follows:

-------------------------------------------------
| alpha_type | x_type | y_type | execution_type |
|------------|--------|--------|----------------|
|  bf16_r    | bf16_r |  bf16_r|      f32_r     |
|  f32_r     | bf16_r |  bf16_r|      f32_r     |
|  f16_r     | f16_r  |  f16_r |      f16_r     |
|  f16_r     | f16_r  |  f16_r |      f32_r     |
|  f32_r     | f16_r  |  f16_r |      f32_r     |
|  f32_r     | f32_r  |  f32_r |      f32_r     |
|  f64_r     | f64_r  |  f64_r |      f64_r     |
|  f32_c     | f32_c  |  f32_c |      f32_c     |
|  f64_c     | f64_c  |  f64_c |      f64_c     |
-------------------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i and y_i.
@param[in]
alpha     device pointer or host pointer to specify the scalar alpha.
@param[in]
alpha_type [rocblas_datatype]
specifies the datatype of alpha.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in, out]
y         device array of device pointers storing each vector y_i.
@param[in]
y_type [rocblas_datatype]
specifies the datatype of each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_axpy_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        batch_count: rocblas_int,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_axpy_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        batch_count: i64,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
axpy_strided_batched_ex   computes constant alpha multiplied by vector x, plus vector y over
a set of strided batched vectors.

y := alpha * x + y

Currently supported datatypes are as follows:

-------------------------------------------------
| alpha_type | x_type | y_type | execution_type |
|------------|--------|--------|----------------|
|  bf16_r    | bf16_r |  bf16_r|      f32_r     |
|  f32_r     | bf16_r |  bf16_r|      f32_r     |
|  f16_r     | f16_r  |  f16_r |      f16_r     |
|  f16_r     | f16_r  |  f16_r |      f32_r     |
|  f32_r     | f16_r  |  f16_r |      f32_r     |
|  f32_r     | f32_r  |  f32_r |      f32_r     |
|  f64_r     | f64_r  |  f64_r |      f64_r     |
|  f32_c     | f32_c  |  f32_c |      f32_c     |
|  f64_c     | f64_c  |  f64_c |      f64_c     |
-------------------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i and y_i.
@param[in]
alpha     device pointer or host pointer to specify the scalar alpha.
@param[in]
alpha_type [rocblas_datatype]
specifies the datatype of alpha.
@param[in]
x         device pointer to the first vector x_1.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stridex   [rocblas_stride]
stride from the start of one vector (x_i) to the next one (x_i+1).
There are no restrictions placed on stridex. However, ensure that stridex is of appropriate size. For a typical
case this means stridex >= n * incx.
@param[in, out]
y         device pointer to the first vector y_1.
@param[in]
y_type [rocblas_datatype]
specifies the datatype of each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
stridey   [rocblas_stride]
stride from the start of one vector (y_i) to the next one (y_i+1).
There are no restrictions placed on stridey. However, ensure that stridey is of appropriate size. For a typical
case this means stridey >= n * incy.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_axpy_strided_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        stridex: rocblas_stride,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        stridey: rocblas_stride,
        batch_count: rocblas_int,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_axpy_strided_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        stridex: rocblas_stride,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        stridey: rocblas_stride,
        batch_count: i64,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
dot_ex  performs the dot product of vectors x and y.

result = x * y;

dotc_ex  performs the dot product of the conjugate of complex vector x and complex vector y

result = conjugate (x) * y;

Currently supported datatypes are as follows:

--------------------------------------------------
| x_type | y_type | result_type | execution_type |
|--------|--------|-------------|----------------|
| f16_r  | f16_r  |    f16_r    |     f16_r      |
| f16_r  | f16_r  |    f16_r    |     f32_r      |
| bf16_r | bf16_r |    bf16_r   |     f32_r      |
| f32_r  | f32_r  |    f32_r    |     f32_r      |
| f32_r  | f32_r  |    f64_r    |     f64_r      |
| f64_r  | f64_r  |    f64_r    |     f64_r      |
| f32_c  | f32_c  |    f32_c    |     f32_c      |
| f64_c  | f64_c  |    f64_c    |     f64_c      |
--------------------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x and y.
@param[in]
x         device pointer storing vector x.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of y.
@param[in]
y         device pointer storing vector y.
@param[in]
y_type [rocblas_datatype]
specifies the datatype of vector y.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
result
device pointer or host pointer to store the dot product.
return is 0.0 if n <= 0.
@param[in]
result_type [rocblas_datatype]
specifies the datatype of the result.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_dot_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dotc_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dot_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dotc_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
dot_batched_ex performs a batch of dot products of vectors x and y.

result_i = x_i * y_i;

dotc_batched_ex  performs a batch of dot products of the conjugate of complex vector x and complex vector y

result_i = conjugate (x_i) * y_i;

where (x_i, y_i) is the i-th instance of the batch.
x_i and y_i are vectors, for i = 1, ..., batch_count

Currently supported datatypes are as follows:

--------------------------------------------------
| x_type | y_type | result_type | execution_type |
|--------|--------|-------------|----------------|
| f16_r  | f16_r  |    f16_r    |     f16_r      |
| f16_r  | f16_r  |    f16_r    |     f32_r      |
| bf16_r | bf16_r |    bf16_r   |     f32_r      |
| f32_r  | f32_r  |    f32_r    |     f32_r      |
| f32_r  | f32_r  |    f64_r    |     f64_r      |
| f64_r  | f64_r  |    f64_r    |     f64_r      |
| f32_c  | f32_c  |    f32_c    |     f32_c      |
| f64_c  | f64_c  |    f64_c    |     f64_c      |
--------------------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i and y_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
y         device array of device pointers storing each vector y_i.
@param[in]
y_type [rocblas_datatype]
specifies the datatype of each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[in, out]
result
device array or host array of batch_count size to store the dot products of each batch.
return 0.0 for each element if n <= 0.
@param[in]
result_type [rocblas_datatype]
specifies the datatype of the result.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_dot_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dotc_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        batch_count: rocblas_int,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dot_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        batch_count: i64,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dotc_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        batch_count: i64,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
dot_strided_batched_ex  performs a batch of dot products of vectors x and y.

result_i = x_i * y_i;

dotc_strided_batched_ex  performs a batch of dot products of the conjugate of complex vector x and complex vector y

result_i = conjugate (x_i) * y_i;

where (x_i, y_i) is the i-th instance of the batch.
x_i and y_i are vectors, for i = 1, ..., batch_count

Currently supported datatypes are as follows:

--------------------------------------------------
| x_type | y_type | result_type | execution_type |
|--------|--------|-------------|----------------|
| f16_r  | f16_r  |    f16_r    |     f16_r      |
| f16_r  | f16_r  |    f16_r    |     f32_r      |
| bf16_r | bf16_r |    bf16_r   |     f32_r      |
| f32_r  | f32_r  |    f32_r    |     f32_r      |
| f32_r  | f32_r  |    f64_r    |     f64_r      |
| f64_r  | f64_r  |    f64_r    |     f64_r      |
| f32_c  | f32_c  |    f32_c    |     f32_c      |
| f64_c  | f64_c  |    f64_c    |     f64_c      |
--------------------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in each x_i and y_i.
@param[in]
x         device pointer to the first vector (x_1) in the batch.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stride_x    [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1)
@param[in]
y         device pointer to the first vector (y_1) in the batch.
@param[in]
y_type [rocblas_datatype]
specifies the datatype of each vector y_i.
@param[in]
incy      [rocblas_int]
specifies the increment for the elements of each y_i.
@param[in]
stride_y    [rocblas_stride]
stride from the start of one vector (y_i) and the next one (y_i+1)
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[in, out]
result
device array or host array of batch_count size to store the dot products of each batch.
return 0.0 for each element if n <= 0.
@param[in]
result_type [rocblas_datatype]
specifies the datatype of the result.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_dot_strided_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dotc_strided_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        batch_count: rocblas_int,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dot_strided_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        stride_x: rocblas_stride,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_dotc_strided_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        stride_x: rocblas_stride,
        y: *const ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        stride_y: rocblas_stride,
        batch_count: i64,
        result: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief BLAS_EX API

\details
nrm2_ex computes the euclidean norm of a real or complex vector.

result := sqrt( x'*x ) for real vectors
result := sqrt( x**H*x ) for complex vectors

Currently supported datatypes are as follows:

-------------------------------------
|  x_type | result | execution_type |
|---------|--------|----------------|
|  bf16_r |  bf16_r|     f32_r      |
|  f16_r  |  f16_r |     f32_r      |
|  f32_r  |  f32_r |     f32_r      |
|  f64_r  |  f64_r |     f64_r      |
|  f32_c  |  f32_r |     f32_r      |
|  f64_c  |  f64_r |     f64_r      |
-------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x.
@param[in]
x         device pointer storing vector x.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of the vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of y.
@param[in, out]
results
device pointer or host pointer to store the nrm2 product.
return is 0.0 if n, incx<=0.
@param[in]
result_type [rocblas_datatype]
specifies the datatype of the result.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_nrm2_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        results: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_nrm2_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        results: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief BLAS_EX API

\details
nrm2_batched_ex computes the euclidean norm over a batch of real or complex vectors.

result := sqrt( x_i'*x_i ) for real vectors x, for i = 1, ..., batch_count
result := sqrt( x_i**H*x_i ) for complex vectors x, for i = 1, ..., batch_count

Currently supported datatypes are as follows:

-------------------------------------
|  x_type | result | execution_type |
|---------|--------|----------------|
|  bf16_r |  bf16_r|     f32_r      |
|  f16_r  |  f16_r |     f32_r      |
|  f32_r  |  f32_r |     f32_r      |
|  f64_r  |  f64_r |     f64_r      |
|  f32_c  |  f32_r |     f32_r      |
|  f64_c  |  f64_r |     f64_r      |
-------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each x_i.
@param[in]
x         device array of device pointers storing each vector x_i.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[out]
results
device pointer or host pointer to array of batch_count size for nrm2 results.
return is 0.0 for each element if n <= 0, incx<=0.
@param[in]
result_type [rocblas_datatype]
specifies the datatype of the result.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_nrm2_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        batch_count: rocblas_int,
        results: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_nrm2_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        batch_count: i64,
        results: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief BLAS_EX API

\details
nrm2_strided_batched_ex computes the euclidean norm over a batch of real or complex vectors.

result := sqrt( x_i'*x_i ) for real vectors x, for i = 1, ..., batch_count
result := sqrt( x_i**H*x_i ) for complex vectors, for i = 1, ..., batch_count

Currently supported datatypes are as follows:

-------------------------------------
|  x_type | result | execution_type |
|---------|--------|----------------|
|  bf16_r |  bf16_r|     f32_r      |
|  f16_r  |  f16_r |     f32_r      |
|  f32_r  |  f32_r |     f32_r      |
|  f64_r  |  f64_r |     f64_r      |
|  f32_c  |  f32_r |     f32_r      |
|  f64_c  |  f64_r |     f64_r      |
-------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
number of elements in each x_i.
@param[in]
x         device pointer to the first vector x_1.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i. incx must be > 0.
@param[in]
stride_x  [rocblas_stride]
stride from the start of one vector (x_i) and the next one (x_i+1).
There are no restrictions placed on stride_x. However, ensure that stride_x is of appropriate size. For a typical
case this means stride_x >= n * incx.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[out]
results
device pointer or host pointer to array for storing contiguous batch_count results.
return is 0.0 for each element if n <= 0, incx<=0.
@param[in]
result_type [rocblas_datatype]
specifies the datatype of the result.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_nrm2_strided_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        batch_count: rocblas_int,
        results: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_nrm2_strided_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *const ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        stride_x: rocblas_stride,
        batch_count: i64,
        results: *mut ::core::ffi::c_void,
        result_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
rot_ex applies the Givens rotation matrix defined by c=cos(alpha) and s=sin(alpha) to vectors x and y.
Scalars c and s may be stored in either host or device memory. Location is specified by calling rocblas_set_pointer_mode.

In the case where cs_type is real:

x := c * x + s * y
y := c * y - s * x

In the case where cs_type is complex, the imaginary part of c is ignored:

x := real(c) * x + s * y
y := real(c) * y - conj(s) * x

Currently supported datatypes are as follows:

------------------------------------------------
|  x_type | y_type  | cs_type | execution_type |
|---------|---------|---------|----------------|
|  bf16_r |  bf16_r | bf16_r  |  f32_r         |
|  f16_r  |  f16_r  | f16_r   |  f32_r         |
|  f32_r  |  f32_r  | f32_r   |  f32_r         |
|  f64_r  |  f64_r  | f64_r   |  f64_r         |
|  f32_c  |  f32_c  | f32_c   |  f32_c         |
|  f32_c  |  f32_c  | f32_r   |  f32_c         |
|  f64_c  |  f64_c  | f64_c   |  f64_c         |
|  f64_c  |  f64_c  | f64_r   |  f64_c         |
------------------------------------------------

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n       [rocblas_int]
number of elements in the x and y vectors.
@param[in, out]
x       device pointer storing vector x.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of vector x.
@param[in]
incx    [rocblas_int]
specifies the increment between elements of x.
@param[in, out]
y       device pointer storing vector y.
@param[in]
y_type [rocblas_datatype]
specifies the datatype of vector y.
@param[in]
incy    [rocblas_int]
specifies the increment between elements of y.
@param[in]
c       device pointer or host pointer storing scalar cosine component of the rotation matrix.
@param[in]
s       device pointer or host pointer storing scalar sine component of the rotation matrix.
@param[in]
cs_type [rocblas_datatype]
specifies the datatype of c and s.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_rot_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        c: *const ::core::ffi::c_void,
        s: *const ::core::ffi::c_void,
        cs_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_rot_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        c: *const ::core::ffi::c_void,
        s: *const ::core::ffi::c_void,
        cs_type: rocblas_datatype,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
rot_batched_ex applies the Givens rotation matrix defined by c=cos(alpha) and s=sin(alpha) to batched vectors x_i and y_i, for i = 1, ..., batch_count.
Scalars c and s may be stored in either host or device memory. Location is specified by calling rocblas_set_pointer_mode.

In the case where cs_type is real:

x := c * x + s * y
y := c * y - s * x

In the case where cs_type is complex, the imaginary part of c is ignored:

x := real(c) * x + s * y
y := real(c) * y - conj(s) * x

Currently supported datatypes are as follows:

------------------------------------------------
|  x_type | y_type  | cs_type | execution_type |
|---------|---------|---------|----------------|
|  bf16_r |  bf16_r | bf16_r  |  f32_r         |
|  f16_r  |  f16_r  | f16_r   |  f32_r         |
|  f32_r  |  f32_r  | f32_r   |  f32_r         |
|  f64_r  |  f64_r  | f64_r   |  f64_r         |
|  f32_c  |  f32_c  | f32_c   |  f32_c         |
|  f32_c  |  f32_c  | f32_r   |  f32_c         |
|  f64_c  |  f64_c  | f64_c   |  f64_c         |
|  f64_c  |  f64_c  | f64_r   |  f64_c         |
------------------------------------------------

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n       [rocblas_int]
number of elements in each x_i and y_i vectors.
@param[in, out]
x       device array of deivce pointers storing each vector x_i.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx    [rocblas_int]
specifies the increment between elements of each x_i.
@param[in, out]
y       device array of device pointers storing each vector y_i.
@param[in]
y_type [rocblas_datatype]
specifies the datatype of each vector y_i.
@param[in]
incy    [rocblas_int]
specifies the increment between elements of each y_i.
@param[in]
c       device pointer or host pointer to scalar cosine component of the rotation matrix.
@param[in]
s       device pointer or host pointer to scalar sine component of the rotation matrix.
@param[in]
cs_type [rocblas_datatype]
specifies the datatype of c and s.
@param[in]
batch_count [rocblas_int]
the number of x and y arrays, the number of batches.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_rot_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        c: *const ::core::ffi::c_void,
        s: *const ::core::ffi::c_void,
        cs_type: rocblas_datatype,
        batch_count: rocblas_int,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_rot_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        c: *const ::core::ffi::c_void,
        s: *const ::core::ffi::c_void,
        cs_type: rocblas_datatype,
        batch_count: i64,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS Level 1 API </b>

\details
rot_strided_batched_ex applies the Givens rotation matrix defined by c=cos(alpha) and s=sin(alpha) to strided batched vectors x_i and y_i, for i = 1, ..., batch_count.
Scalars c and s may be stored in either host or device memory. Location is specified by calling rocblas_set_pointer_mode.

In the case where cs_type is real:

x := c * x + s * y
y := c * y - s * x

In the case where cs_type is complex, the imaginary part of c is ignored:

x := real(c) * x + s * y
y := real(c) * y - conj(s) * x

Currently supported datatypes are as follows:

------------------------------------------------
|  x_type | y_type  | cs_type | execution_type |
|---------|---------|---------|----------------|
|  bf16_r |  bf16_r | bf16_r  |  f32_r         |
|  f16_r  |  f16_r  | f16_r   |  f32_r         |
|  f32_r  |  f32_r  | f32_r   |  f32_r         |
|  f64_r  |  f64_r  | f64_r   |  f64_r         |
|  f32_c  |  f32_c  | f32_c   |  f32_c         |
|  f32_c  |  f32_c  | f32_r   |  f32_c         |
|  f64_c  |  f64_c  | f64_c   |  f64_c         |
|  f64_c  |  f64_c  | f64_r   |  f64_c         |
------------------------------------------------

@param[in]
handle  [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n       [rocblas_int]
number of elements in each x_i and y_i vectors.
@param[in, out]
x       device pointer to the first vector x_1.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx    [rocblas_int]
specifies the increment between elements of each x_i.
@param[in]
stride_x [rocblas_stride]
specifies the increment from the beginning of x_i to the beginning of x_(i+1)
@param[in, out]
y       device pointer to the first vector y_1.
@param[in]
y_type [rocblas_datatype]
specifies the datatype of each vector y_i.
@param[in]
incy    [rocblas_int]
specifies the increment between elements of each y_i.
@param[in]
stride_y [rocblas_stride]
specifies the increment from the beginning of y_i to the beginning of y_(i+1)
@param[in]
c       device pointer or host pointer to scalar cosine component of the rotation matrix.
@param[in]
s       device pointer or host pointer to scalar sine component of the rotation matrix.
@param[in]
cs_type [rocblas_datatype]
specifies the datatype of c and s.
@param[in]
batch_count [rocblas_int]
the number of x and y arrays, the number of batches.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_rot_strided_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        stride_x: rocblas_stride,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: rocblas_int,
        stride_y: rocblas_stride,
        c: *const ::core::ffi::c_void,
        s: *const ::core::ffi::c_void,
        cs_type: rocblas_datatype,
        batch_count: rocblas_int,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_rot_strided_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        stride_x: rocblas_stride,
        y: *mut ::core::ffi::c_void,
        y_type: rocblas_datatype,
        incy: i64,
        stride_y: rocblas_stride,
        c: *const ::core::ffi::c_void,
        s: *const ::core::ffi::c_void,
        cs_type: rocblas_datatype,
        batch_count: i64,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
scal_ex  scales each element of vector x with scalar alpha.

x := alpha * x

Currently supported datatypes are as follows:

----------------------------------------
| alpha_type | x_type | execution_type |
|------------|--------|----------------|
|  f32_r     | bf16_r |     f32_r      |
|  bf16_r    | bf16_r |     f32_r      |
|  f16_r     | f16_r  |     f16_r      |
|  f16_r     | f16_r  |     f32_r      |
|  f32_r     | f16_r  |     f32_r      |
|  f32_r     | f32_r  |     f32_r      |
|  f64_r     | f64_r  |     f64_r      |
|  f32_c     | f32_c  |     f32_c      |
|  f64_c     | f64_c  |     f64_c      |
|  f32_r     | f32_c  |     f32_c      |
|  f64_r     | f64_c  |     f64_c      |
----------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x.
@param[in]
alpha     device pointer or host pointer for the scalar alpha.
@param[in]
alpha_type [rocblas_datatype]
specifies the datatype of alpha.
@param[in, out]
x         device pointer storing vector x.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of vector x.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of x.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_scal_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scal_ex_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
scal_batched_ex  scales each element of each vector x_i with scalar alpha.

x_i := alpha * x_i

Currently supported datatypes are as follows:

----------------------------------------
| alpha_type | x_type | execution_type |
|------------|--------|----------------|
|  f32_r     | bf16_r |     f32_r      |
|  bf16_r    | bf16_r |     f32_r      |
|  f16_r     | f16_r  |     f16_r      |
|  f16_r     | f16_r  |     f32_r      |
|  f32_r     | f16_r  |     f32_r      |
|  f32_r     | f32_r  |     f32_r      |
|  f64_r     | f64_r  |     f64_r      |
|  f32_c     | f32_c  |     f32_c      |
|  f64_c     | f64_c  |     f64_c      |
|  f32_r     | f32_c  |     f32_c      |
|  f64_r     | f64_c  |     f64_c      |
----------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x.
@param[in]
alpha     device pointer or host pointer for the scalar alpha.
@param[in]
alpha_type [rocblas_datatype]
specifies the datatype of alpha.
@param[in, out]
x         device array of device pointers storing each vector x_i.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.
*/
    pub fn rocblas_scal_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        batch_count: rocblas_int,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scal_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        batch_count: i64,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** @{
\brief <b> BLAS EX API </b>

\details
scal_strided_batched_ex  scales each element of vector x with scalar alpha over a set
of strided batched vectors.

x := alpha * x

Currently supported datatypes are as follows:

----------------------------------------
| alpha_type | x_type | execution_type |
|------------|--------|----------------|
|  f32_r     | bf16_r |     f32_r      |
|  bf16_r    | bf16_r |     f32_r      |
|  f16_r     | f16_r  |     f16_r      |
|  f16_r     | f16_r  |     f32_r      |
|  f32_r     | f16_r  |     f32_r      |
|  f32_r     | f32_r  |     f32_r      |
|  f64_r     | f64_r  |     f64_r      |
|  f32_c     | f32_c  |     f32_c      |
|  f64_c     | f64_c  |     f64_c      |
|  f32_r     | f32_c  |     f32_c      |
|  f64_r     | f64_c  |     f64_c      |
----------------------------------------

@param[in]
handle    [rocblas_handle]
handle to the rocblas library context queue.
@param[in]
n         [rocblas_int]
the number of elements in x.
@param[in]
alpha     device pointer or host pointer for the scalar alpha.
@param[in]
alpha_type [rocblas_datatype]
specifies the datatype of alpha.
@param[in, out]
x         device pointer to the first vector x_1.
@param[in]
x_type [rocblas_datatype]
specifies the datatype of each vector x_i.
@param[in]
incx      [rocblas_int]
specifies the increment for the elements of each x_i.
@param[in]
stridex   [rocblas_stride]
stride from the start of one vector (x_i) to the next one (x_i+1).
There are no restrictions placed on stridex. However, ensure that stridex is of appropriate size. For a typical
case this means stridex >= n * incx.
@param[in]
batch_count [rocblas_int]
number of instances in the batch.
@param[in]
execution_type [rocblas_datatype]
specifies the datatype of computation.

*/
    pub fn rocblas_scal_strided_batched_ex(
        handle: rocblas_handle,
        n: rocblas_int,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: rocblas_int,
        stridex: rocblas_stride,
        batch_count: rocblas_int,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_scal_strided_batched_ex_64(
        handle: rocblas_handle,
        n: i64,
        alpha: *const ::core::ffi::c_void,
        alpha_type: rocblas_datatype,
        x: *mut ::core::ffi::c_void,
        x_type: rocblas_datatype,
        incx: i64,
        stridex: rocblas_stride,
        batch_count: i64,
        execution_type: rocblas_datatype,
    ) -> rocblas_status;
}
extern "C" {
    /** BLAS Auxiliary API

\details
rocblas_status_to_string

Returns string representing rocblas_status value

@param[in]
status  [rocblas_status]
rocBLAS status to convert to string*/
    pub fn rocblas_status_to_string(
        status: rocblas_status,
    ) -> *const ::core::ffi::c_char;
}
extern "C" {
    /** \brief Initialize rocBLAS on the current HIP device, to avoid costly startup time at the first call on that device.
\details

Calling `rocblas_initialize()` allows upfront initialization including device specific kernel setup.
Otherwise this function is automatically called on the first function call that requires these initializations (mainly GEMM).
*/
    pub fn rocblas_initialize();
}
extern "C" {
    #[must_use]
    /** \brief   Loads char* buf with the rocblas library version. size_t len
is the maximum length of char* buf.
\details

@param[in, out]
buf             pointer to buffer for version string

@param[in]
len             length of buf
*/
    pub fn rocblas_get_version_string(
        buf: *mut ::core::ffi::c_char,
        len: usize,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief   Queries the minimum buffer size for a successful call to
\ref rocblas_get_version_string
\details

@param[out]
len             pointer to size_t for storing the length
*/
    pub fn rocblas_get_version_string_size(len: *mut usize) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief
\details
Indicates that subsequent rocBLAS kernel calls should collect the optimal device memory size in bytes for their given kernel arguments
and keep track of the maximum.
Each kernel call can reuse temporary device memory on the same stream so the maximum is collected.
Returns rocblas_status_size_query_mismatch if another size query is already in progress; returns rocblas_status_success otherwise
@param[in]
handle          rocblas handle*/
    pub fn rocblas_start_device_memory_size_query(
        handle: rocblas_handle,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief
\details
Stops collecting optimal device memory size information.
Returns rocblas_status_size_query_mismatch if a collection is not underway; rocblas_status_invalid_handle if handle is nullptr;
rocblas_status_invalid_pointer if size is nullptr; rocblas_status_success otherwise
@param[in]
handle          rocblas handle
@param[out]
size            maximum of the optimal sizes collected*/
    pub fn rocblas_stop_device_memory_size_query(
        handle: rocblas_handle,
        size: *mut usize,
    ) -> rocblas_status;
}
extern "C" {
    pub fn rocblas_is_device_memory_size_query(handle: rocblas_handle) -> bool;
}
extern "C" {
    #[must_use]
    pub fn rocblas_set_optimal_device_memory_size_impl(
        handle: rocblas_handle,
        count: usize,
        ...
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_device_malloc_alloc(
        handle: rocblas_handle,
        res: *mut *mut rocblas_device_malloc_base,
        count: usize,
        ...
    ) -> rocblas_status;
}
extern "C" {
    pub fn rocblas_device_malloc_success(ptr: *mut rocblas_device_malloc_base) -> bool;
}
extern "C" {
    #[must_use]
    pub fn rocblas_device_malloc_ptr(
        ptr: *mut rocblas_device_malloc_base,
        res: *mut *mut ::core::ffi::c_void,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_device_malloc_get(
        ptr: *mut rocblas_device_malloc_base,
        index: usize,
        res: *mut *mut ::core::ffi::c_void,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    pub fn rocblas_device_malloc_free(
        ptr: *mut rocblas_device_malloc_base,
    ) -> rocblas_status;
}
extern "C" {
    pub fn rocblas_device_malloc_set_default_memory_size(size: usize);
}
extern "C" {
    #[must_use]
    /** \brief
\details
Gets the current device memory size for the handle.
Returns rocblas_status_invalid_handle if handle is nullptr; rocblas_status_invalid_pointer if size is nullptr; rocblas_status_success otherwise
@param[in]
handle          rocblas handle
@param[out]
size            current device memory size for the handle*/
    pub fn rocblas_get_device_memory_size(
        handle: rocblas_handle,
        size: *mut usize,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief
\details
Changes the size of allocated device memory at runtime.

Any previously allocated device memory managed by the handle is freed.

If size > 0 sets the device memory size to the specified size (in bytes).
If size == 0, frees the memory allocated so far, and lets rocBLAS manage device memory in the future, expanding it when necessary.
Returns rocblas_status_invalid_handle if handle is nullptr; rocblas_status_invalid_pointer if size is nullptr; rocblas_status_success otherwise
@param[in]
handle          rocblas handle
@param[in]
size            size of allocated device memory*/
    pub fn rocblas_set_device_memory_size(
        handle: rocblas_handle,
        size: usize,
    ) -> rocblas_status;
}
extern "C" {
    #[must_use]
    /** \brief
\details
Sets the device workspace for the handle to use.

Any previously allocated device memory managed by the handle is freed.

Returns rocblas_status_invalid_handle if handle is nullptr; rocblas_status_success otherwise
@param[in]
handle          rocblas handle
@param[in]
addr            address of workspace memory
@param[in]
size            size of workspace memory
*/
    pub fn rocblas_set_workspace(
        handle: rocblas_handle,
        addr: *mut ::core::ffi::c_void,
        size: usize,
    ) -> rocblas_status;
}
extern "C" {
    /** \brief
\details
Returns true when device memory in handle is managed by rocBLAS
@param[in]
handle          rocblas handle*/
    pub fn rocblas_is_managing_device_memory(handle: rocblas_handle) -> bool;
}
extern "C" {
    /** \brief
\details
Returns true when device memory in handle is managed by the user
@param[in]
handle          rocblas handle*/
    pub fn rocblas_is_user_managing_device_memory(handle: rocblas_handle) -> bool;
}
extern "C" {
    pub fn rocblas_abort() -> !;
}
impl rocblas_error {
    pub const r#invalid_handle: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const r#not_implemented: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const r#invalid_pointer: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const r#invalid_size: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const r#memory_error: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const r#internal_error: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const r#perf_degraded: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const r#size_query_mismatch: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const r#size_increased: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(9)
    });
    pub const r#size_unchanged: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(10)
    });
    pub const r#invalid_value: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(11)
    });
    pub const r#continue: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(12)
    });
    pub const r#check_numerics_fail: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(13)
    });
    pub const r#excluded_from_build: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(14)
    });
    pub const r#arch_mismatch: rocblas_error = rocblas_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(15)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct rocblas_error(pub ::core::num::NonZeroU32);
pub trait rocblas_statusConsts {
    const success: rocblas_status = rocblas_status::Ok(());
    const error_invalid_handle: rocblas_status = rocblas_status::Err(
        rocblas_error::r#invalid_handle,
    );
    const error_not_implemented: rocblas_status = rocblas_status::Err(
        rocblas_error::r#not_implemented,
    );
    const error_invalid_pointer: rocblas_status = rocblas_status::Err(
        rocblas_error::r#invalid_pointer,
    );
    const error_invalid_size: rocblas_status = rocblas_status::Err(
        rocblas_error::r#invalid_size,
    );
    const error_memory_error: rocblas_status = rocblas_status::Err(
        rocblas_error::r#memory_error,
    );
    const error_internal_error: rocblas_status = rocblas_status::Err(
        rocblas_error::r#internal_error,
    );
    const error_perf_degraded: rocblas_status = rocblas_status::Err(
        rocblas_error::r#perf_degraded,
    );
    const error_size_query_mismatch: rocblas_status = rocblas_status::Err(
        rocblas_error::r#size_query_mismatch,
    );
    const error_size_increased: rocblas_status = rocblas_status::Err(
        rocblas_error::r#size_increased,
    );
    const error_size_unchanged: rocblas_status = rocblas_status::Err(
        rocblas_error::r#size_unchanged,
    );
    const error_invalid_value: rocblas_status = rocblas_status::Err(
        rocblas_error::r#invalid_value,
    );
    const error_continue: rocblas_status = rocblas_status::Err(
        rocblas_error::r#continue,
    );
    const error_check_numerics_fail: rocblas_status = rocblas_status::Err(
        rocblas_error::r#check_numerics_fail,
    );
    const error_excluded_from_build: rocblas_status = rocblas_status::Err(
        rocblas_error::r#excluded_from_build,
    );
    const error_arch_mismatch: rocblas_status = rocblas_status::Err(
        rocblas_error::r#arch_mismatch,
    );
}
impl rocblas_statusConsts for rocblas_status {}
#[must_use]
pub type rocblas_status = ::core::result::Result<(), rocblas_error>;
const _: fn() = || {
    let _ = std::mem::transmute::<rocblas_status, u32>;
};
unsafe impl Send for rocblas_handle {}
unsafe impl Sync for rocblas_handle {}
