// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub const hipfftVersionMajor: u32 = 1;
pub const hipfftVersionMinor: u32 = 0;
pub const hipfftVersionPatch: u32 = 22;
pub type float2 = u64;
pub type double2 = u128;
pub type hipFloatComplex = float2;
pub type hipDoubleComplex = double2;
pub type hipComplex = hipFloatComplex;
impl hipfftType_t {
    /// Real to complex (interleaved)
    pub const HIPFFT_R2C: hipfftType_t = hipfftType_t(42);
}
impl hipfftType_t {
    /// Complex (interleaved) to real
    pub const HIPFFT_C2R: hipfftType_t = hipfftType_t(44);
}
impl hipfftType_t {
    /// Complex to complex (interleaved)
    pub const HIPFFT_C2C: hipfftType_t = hipfftType_t(41);
}
impl hipfftType_t {
    /// Double to double-complex (interleaved)
    pub const HIPFFT_D2Z: hipfftType_t = hipfftType_t(106);
}
impl hipfftType_t {
    /// Double-complex (interleaved) to double
    pub const HIPFFT_Z2D: hipfftType_t = hipfftType_t(108);
}
impl hipfftType_t {
    /// Double-complex to double-complex (interleaved)
    pub const HIPFFT_Z2Z: hipfftType_t = hipfftType_t(105);
}
#[repr(transparent)]
/** @brief Transform type
  @details This type is used to declare the Fourier transform type that will be executed.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipfftType_t(pub ::core::ffi::c_uint);
/** @brief Transform type
  @details This type is used to declare the Fourier transform type that will be executed.*/
pub use self::hipfftType_t as hipfftType;
impl hipfftLibraryPropertyType_t {
    pub const HIPFFT_MAJOR_VERSION: hipfftLibraryPropertyType_t = hipfftLibraryPropertyType_t(
        0,
    );
}
impl hipfftLibraryPropertyType_t {
    pub const HIPFFT_MINOR_VERSION: hipfftLibraryPropertyType_t = hipfftLibraryPropertyType_t(
        1,
    );
}
impl hipfftLibraryPropertyType_t {
    pub const HIPFFT_PATCH_LEVEL: hipfftLibraryPropertyType_t = hipfftLibraryPropertyType_t(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipfftLibraryPropertyType_t(pub ::core::ffi::c_uint);
pub use self::hipfftLibraryPropertyType_t as hipfftLibraryPropertyType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipfftHandle_t {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipfftHandle(pub *mut hipfftHandle_t);
/// @brief Single-precision floating point complex type
pub type hipfftComplex = hipComplex;
/// @brief Double-precision floating point complex type
pub type hipfftDoubleComplex = hipDoubleComplex;
/// @brief Single-precision floating point type
pub type hipfftReal = f32;
/// @brief Double-precision floating point type
pub type hipfftDoubleReal = f64;
#[cfg(not(windows))]
extern "C" {
    /** @brief Create a new one-dimensional FFT plan.

  @details Allocate and initialize a new one-dimensional FFT plan.

  @param[out] plan Pointer to the FFT plan handle.
  @param[in] nx FFT length.
  @param[in] type FFT type.
  @param[in] batch Number of batched transforms to compute.*/
    pub fn hipfftPlan1d(
        plan: *mut hipfftHandle,
        nx: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Create a new two-dimensional FFT plan.

  @details Allocate and initialize a new two-dimensional FFT plan.
  Two-dimensional data should be stored in C ordering (row-major
  format), so that indexes in y-direction (j index) vary the
  fastest.

  @param[out] plan Pointer to the FFT plan handle.
  @param[in] nx Number of elements in the x-direction (slow index).
  @param[in] ny Number of elements in the y-direction (fast index).
  @param[in] type FFT type.*/
    pub fn hipfftPlan2d(
        plan: *mut hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: hipfftType,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Create a new three-dimensional FFT plan.

  @details Allocate and initialize a new three-dimensional FFT plan.
  Three-dimensional data should be stored in C ordering (row-major
  format), so that indexes in z-direction (k index) vary the
  fastest.

  @param[out] plan Pointer to the FFT plan handle.
  @param[in] nx Number of elements in the x-direction (slowest index).
  @param[in] ny Number of elements in the y-direction.
  @param[in] nz Number of elements in the z-direction (fastest index).
  @param[in] type FFT type.*/
    pub fn hipfftPlan3d(
        plan: *mut hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: hipfftType,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Create a new batched rank-dimensional FFT plan with advanced data layout.

 @details Allocate and initialize a new batched rank-dimensional
  FFT plan. The number of elements to transform in each direction of
  the input data is specified in n.

  The batch parameter tells hipFFT how many transforms to perform.
  The distance between the first elements of two consecutive batches
  of the input and output data are specified with the idist and odist
  parameters.

  The inembed and onembed parameters define the input and output data
  layouts. The number of elements in the data is assumed to be larger
  than the number of elements in the transform. Strided data layouts
  are also supported. Strides along the fastest direction in the input
  and output data are specified via the istride and ostride parameters.

  If both inembed and onembed parameters are set to NULL, all the
  advanced data layout parameters are ignored and reverted to default
  values, i.e., the batched transform is performed with non-strided data
  access and the number of data/transform elements are assumed to be
  equivalent.

  @param[out] plan Pointer to the FFT plan handle.
  @param[in] rank Dimension of transform (1, 2, or 3).
  @param[in] n Number of elements to transform in the x/y/z directions.
  @param[in] inembed Number of elements in the input data in the x/y/z directions.
  @param[in] istride Distance between two successive elements in the input data.
  @param[in] idist Distance between input batches.
  @param[in] onembed Number of elements in the output data in the x/y/z directions.
  @param[in] ostride Distance between two successive elements in the output data.
  @param[in] odist Distance between output batches.
  @param[in] type FFT type.
  @param[in] batch Number of batched transforms to perform.*/
    pub fn hipfftPlanMany(
        plan: *mut hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Allocate a new plan.

  @param[out] plan Pointer to the FFT plan handle to be allocated.*/
    pub fn hipfftCreate(plan: *mut hipfftHandle) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set scaling factor.

  @details hipFFT multiplies each element of the result by the given factor at the end of the transform.

  The supplied factor must be a finite number.  That is, it must neither be infinity nor NaN.

  This function must be called after the plan is allocated using
  ::hipfftCreate, but before the plan is initialized by any of the
  "MakePlan" functions.  Therefore, API functions that combine
  creation and initialization (::hipfftPlan1d, ::hipfftPlan2d,
  ::hipfftPlan3d, and ::hipfftPlanMany) cannot set a scale factor.

  Note that the scale factor applies to both forward and
  backward transforms executed with the specified plan handle.*/
    pub fn hipfftExtPlanScaleFactor(
        plan: hipfftHandle,
        scalefactor: f64,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Initialize a new one-dimensional FFT plan.

  @details Assumes that the plan has been created already, and
  modifies the plan associated with the plan handle.

  @param[in] plan Handle of the FFT plan.
  @param[in] nx FFT length.
  @param[in] type FFT type.
  @param[in] batch Number of batched transforms to compute.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftMakePlan1d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Initialize a new two-dimensional FFT plan.

  @details Assumes that the plan has been created already, and
  modifies the plan associated with the plan handle.
  Two-dimensional data should be stored in C ordering (row-major
  format), so that indexes in y-direction (j index) vary the
  fastest.

  @param[in] plan Handle of the FFT plan.
  @param[in] nx Number of elements in the x-direction (slow index).
  @param[in] ny Number of elements in the y-direction (fast index).
  @param[in] type FFT type.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftMakePlan2d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: hipfftType,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Initialize a new two-dimensional FFT plan.

  @details Assumes that the plan has been created already, and
  modifies the plan associated with the plan handle.
  Three-dimensional data should be stored in C ordering (row-major
  format), so that indexes in z-direction (k index) vary the
  fastest.

  @param[in] plan Handle of the FFT plan.
  @param[in] nx Number of elements in the x-direction (slowest index).
  @param[in] ny Number of elements in the y-direction.
  @param[in] nz Number of elements in the z-direction (fastest index).
  @param[in] type FFT type.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftMakePlan3d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: hipfftType,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Initialize a new batched rank-dimensional FFT plan with advanced data layout.

  @details Assumes that the plan has been created already, and
  modifies the plan associated with the plan handle. The number
  of elements to transform in each direction of the input data
  in the FFT plan is specified in n.

  The batch parameter tells hipFFT how many transforms to perform.
  The distance between the first elements of two consecutive batches
  of the input and output data are specified with the idist and odist
  parameters.

  The inembed and onembed parameters define the input and output data
  layouts. The number of elements in the data is assumed to be larger
  than the number of elements in the transform. Strided data layouts
  are also supported. Strides along the fastest direction in the input
  and output data are specified via the istride and ostride parameters.

  If both inembed and onembed parameters are set to NULL, all the
  advanced data layout parameters are ignored and reverted to default
  values, i.e., the batched transform is performed with non-strided data
  access and the number of data/transform elements are assumed to be
  equivalent.

  @param[out] plan Pointer to the FFT plan handle.
  @param[in] rank Dimension of transform (1, 2, or 3).
  @param[in] n Number of elements to transform in the x/y/z directions.
  @param[in] inembed Number of elements in the input data in the x/y/z directions.
  @param[in] istride Distance between two successive elements in the input data.
  @param[in] idist Distance between input batches.
  @param[in] onembed Number of elements in the output data in the x/y/z directions.
  @param[in] ostride Distance between two successive elements in the output data.
  @param[in] odist Distance between output batches.
  @param[in] type FFT type.
  @param[in] batch Number of batched transforms to perform.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftMakePlanMany(
        plan: hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    pub fn hipfftMakePlanMany64(
        plan: hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_longlong,
        inembed: *mut ::core::ffi::c_longlong,
        istride: ::core::ffi::c_longlong,
        idist: ::core::ffi::c_longlong,
        onembed: *mut ::core::ffi::c_longlong,
        ostride: ::core::ffi::c_longlong,
        odist: ::core::ffi::c_longlong,
        type_: hipfftType,
        batch: ::core::ffi::c_longlong,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Return an estimate of the work area size required for a 1D plan.

  @param[in] nx Number of elements in the x-direction.
  @param[in] type FFT type.
  @param[in] batch Number of batched transforms to perform.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftEstimate1d(
        nx: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Return an estimate of the work area size required for a 2D plan.

  @param[in] nx Number of elements in the x-direction.
  @param[in] ny Number of elements in the y-direction.
  @param[in] type FFT type.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftEstimate2d(
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: hipfftType,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Return an estimate of the work area size required for a 3D plan.

  @param[in] nx Number of elements in the x-direction.
  @param[in] ny Number of elements in the y-direction.
  @param[in] nz Number of elements in the z-direction.
  @param[in] type FFT type.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftEstimate3d(
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: hipfftType,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Return an estimate of the work area size required for a rank-dimensional plan.

  @param[in] rank Dimension of FFT transform (1, 2, or 3).
  @param[in] n Number of elements in the x/y/z directions.
  @param[in] inembed
  @param[in] istride
  @param[in] idist Distance between input batches.
  @param[in] onembed
  @param[in] ostride
  @param[in] odist Distance between output batches.
  @param[in] type FFT type.
  @param[in] batch Number of batched transforms to perform.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftEstimateMany(
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Return size of the work area size required for a 1D plan.

  @param[in] plan Pointer to the FFT plan.
  @param[in] nx Number of elements in the x-direction.
  @param[in] type FFT type.
  @param[in] batch Number of batched transforms to perform.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftGetSize1d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Return size of the work area size required for a 2D plan.

  @param[in] plan Pointer to the FFT plan.
  @param[in] nx Number of elements in the x-direction.
  @param[in] ny Number of elements in the y-direction.
  @param[in] type FFT type.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftGetSize2d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: hipfftType,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Return size of the work area size required for a 3D plan.

  @param[in] plan Pointer to the FFT plan.
  @param[in] nx Number of elements in the x-direction.
  @param[in] ny Number of elements in the y-direction.
  @param[in] nz Number of elements in the z-direction.
  @param[in] type FFT type.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftGetSize3d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: hipfftType,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Return size of the work area size required for a rank-dimensional plan.

  @param[in] plan Pointer to the FFT plan.
  @param[in] rank Dimension of FFT transform (1, 2, or 3).
  @param[in] n Number of elements in the x/y/z directions.
  @param[in] inembed
  @param[in] istride
  @param[in] idist Distance between input batches.
  @param[in] onembed
  @param[in] ostride
  @param[in] odist Distance between output batches.
  @param[in] type FFT type.
  @param[in] batch Number of batched transforms to perform.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftGetSizeMany(
        plan: hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    pub fn hipfftGetSizeMany64(
        plan: hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_longlong,
        inembed: *mut ::core::ffi::c_longlong,
        istride: ::core::ffi::c_longlong,
        idist: ::core::ffi::c_longlong,
        onembed: *mut ::core::ffi::c_longlong,
        ostride: ::core::ffi::c_longlong,
        odist: ::core::ffi::c_longlong,
        type_: hipfftType,
        batch: ::core::ffi::c_longlong,
        workSize: *mut usize,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Return size of the work area size required for a rank-dimensional plan.

  @param[in] plan Pointer to the FFT plan.
  @param[out] workSize Pointer to work area size (returned value).*/
    pub fn hipfftGetSize(plan: hipfftHandle, workSize: *mut usize) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set the plan's auto-allocation flag.  The plan will allocate its own workarea.

  @param[in] plan Pointer to the FFT plan.
  @param[in] autoAllocate 0 to disable auto-allocation, non-zero to enable.*/
    pub fn hipfftSetAutoAllocation(
        plan: hipfftHandle,
        autoAllocate: ::core::ffi::c_int,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set the plan's work area.

  @param[in] plan Pointer to the FFT plan.
  @param[in] workArea Pointer to the work area (on device).*/
    pub fn hipfftSetWorkArea(
        plan: hipfftHandle,
        workArea: *mut ::core::ffi::c_void,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Execute a (float) complex-to-complex FFT.

  @details If the input and output buffers are equal, an in-place
  transform is performed.

  @param[in] plan The FFT plan.
  @param[in] idata Input data (on device).
  @param[out] odata Output data (on device).
  @param[in] direction Either `HIPFFT_FORWARD` or `HIPFFT_BACKWARD`.*/
    pub fn hipfftExecC2C(
        plan: hipfftHandle,
        idata: *mut hipfftComplex,
        odata: *mut hipfftComplex,
        direction: ::core::ffi::c_int,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Execute a (float) real-to-complex FFT.

  @details If the input and output buffers are equal, an in-place
  transform is performed.

  @param[in] plan The FFT plan.
  @param[in] idata Input data (on device).
  @param[out] odata Output data (on device).*/
    pub fn hipfftExecR2C(
        plan: hipfftHandle,
        idata: *mut hipfftReal,
        odata: *mut hipfftComplex,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Execute a (float) complex-to-real FFT.

  @details If the input and output buffers are equal, an in-place
  transform is performed.

  @param[in] plan The FFT plan.
  @param[in] idata Input data (on device).
  @param[out] odata Output data (on device).*/
    pub fn hipfftExecC2R(
        plan: hipfftHandle,
        idata: *mut hipfftComplex,
        odata: *mut hipfftReal,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Execute a (double) complex-to-complex FFT.

  @details If the input and output buffers are equal, an in-place
  transform is performed.

  @param[in] plan The FFT plan.
  @param[in] idata Input data (on device).
  @param[out] odata Output data (on device).
  @param[in] direction Either `HIPFFT_FORWARD` or `HIPFFT_BACKWARD`.*/
    pub fn hipfftExecZ2Z(
        plan: hipfftHandle,
        idata: *mut hipfftDoubleComplex,
        odata: *mut hipfftDoubleComplex,
        direction: ::core::ffi::c_int,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Execute a (double) real-to-complex FFT.

  @details If the input and output buffers are equal, an in-place
  transform is performed.

  @param[in] plan The FFT plan.
  @param[in] idata Input data (on device).
  @param[out] odata Output data (on device).*/
    pub fn hipfftExecD2Z(
        plan: hipfftHandle,
        idata: *mut hipfftDoubleReal,
        odata: *mut hipfftDoubleComplex,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Execute a (double) complex-to-real FFT.

  @details If the input and output buffers are equal, an in-place
  transform is performed.

  @param[in] plan The FFT plan.
  @param[in] idata Input data (on device).
  @param[out] odata Output data (on device).*/
    pub fn hipfftExecZ2D(
        plan: hipfftHandle,
        idata: *mut hipfftDoubleComplex,
        odata: *mut hipfftDoubleReal,
    ) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set HIP stream to execute plan on.

 @details Associates a HIP stream with a hipFFT plan.  All kernels
 launched by this plan are associated with the provided stream.

 @param[in] plan The FFT plan.
 @param[in] stream The HIP stream.*/
    pub fn hipfftSetStream(plan: hipfftHandle, stream: hip_runtime_sys::hipStream_t) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Destroy and deallocate an existing plan.

  @param[in] plan Handle of the FFT plan to be destroyed.*/
    pub fn hipfftDestroy(plan: hipfftHandle) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Get rocFFT/cuFFT version.

  @param[out] version cuFFT/rocFFT version (returned value).*/
    pub fn hipfftGetVersion(version: *mut ::core::ffi::c_int) -> hipfftResult;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Get library property.

  @param[in] type Property type.
  @param[out] value Returned value.*/
    pub fn hipfftGetProperty(
        type_: hipfftLibraryPropertyType,
        value: *mut ::core::ffi::c_int,
    ) -> hipfftResult;
}
impl hipfftError {
    pub const r#INVALID_PLAN: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const r#ALLOC_FAILED: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const r#INVALID_TYPE: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const r#INVALID_VALUE: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const r#INTERNAL_ERROR: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const r#EXEC_FAILED: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const r#SETUP_FAILED: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const r#INVALID_SIZE: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const r#UNALIGNED_DATA: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(9)
    });
    pub const r#INCOMPLETE_PARAMETER_LIST: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(10)
    });
    pub const r#INVALID_DEVICE: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(11)
    });
    pub const r#PARSE_ERROR: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(12)
    });
    pub const r#NO_WORKSPACE: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(13)
    });
    pub const r#NOT_IMPLEMENTED: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(14)
    });
    pub const r#NOT_SUPPORTED: hipfftError = hipfftError(unsafe {
        ::core::num::NonZeroU32::new_unchecked(16)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct hipfftError(pub ::core::num::NonZeroU32);
pub trait hipfftResultConsts {
    const SUCCESS: hipfftResult = hipfftResult::Ok(());
    const ERROR_INVALID_PLAN: hipfftResult = hipfftResult::Err(
        hipfftError::r#INVALID_PLAN,
    );
    const ERROR_ALLOC_FAILED: hipfftResult = hipfftResult::Err(
        hipfftError::r#ALLOC_FAILED,
    );
    const ERROR_INVALID_TYPE: hipfftResult = hipfftResult::Err(
        hipfftError::r#INVALID_TYPE,
    );
    const ERROR_INVALID_VALUE: hipfftResult = hipfftResult::Err(
        hipfftError::r#INVALID_VALUE,
    );
    const ERROR_INTERNAL_ERROR: hipfftResult = hipfftResult::Err(
        hipfftError::r#INTERNAL_ERROR,
    );
    const ERROR_EXEC_FAILED: hipfftResult = hipfftResult::Err(
        hipfftError::r#EXEC_FAILED,
    );
    const ERROR_SETUP_FAILED: hipfftResult = hipfftResult::Err(
        hipfftError::r#SETUP_FAILED,
    );
    const ERROR_INVALID_SIZE: hipfftResult = hipfftResult::Err(
        hipfftError::r#INVALID_SIZE,
    );
    const ERROR_UNALIGNED_DATA: hipfftResult = hipfftResult::Err(
        hipfftError::r#UNALIGNED_DATA,
    );
    const ERROR_INCOMPLETE_PARAMETER_LIST: hipfftResult = hipfftResult::Err(
        hipfftError::r#INCOMPLETE_PARAMETER_LIST,
    );
    const ERROR_INVALID_DEVICE: hipfftResult = hipfftResult::Err(
        hipfftError::r#INVALID_DEVICE,
    );
    const ERROR_PARSE_ERROR: hipfftResult = hipfftResult::Err(
        hipfftError::r#PARSE_ERROR,
    );
    const ERROR_NO_WORKSPACE: hipfftResult = hipfftResult::Err(
        hipfftError::r#NO_WORKSPACE,
    );
    const ERROR_NOT_IMPLEMENTED: hipfftResult = hipfftResult::Err(
        hipfftError::r#NOT_IMPLEMENTED,
    );
    const ERROR_NOT_SUPPORTED: hipfftResult = hipfftResult::Err(
        hipfftError::r#NOT_SUPPORTED,
    );
}
impl hipfftResultConsts for hipfftResult {}
#[must_use]
pub type hipfftResult = ::core::result::Result<(), hipfftError>;
const _: fn() = || {
    let _ = std::mem::transmute::<hipfftResult, u32>;
};
unsafe impl Send for hipfftHandle {}
unsafe impl Sync for hipfftHandle {}
