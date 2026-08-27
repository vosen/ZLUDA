// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rocfft_plan_t {
    _unused: [u8; 0],
}
/** @brief Pointer type to plan structure
  @details This type is used to declare a plan handle that can be initialized
 with ::rocfft_plan_create.*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocfft_plan(pub *mut rocfft_plan_t);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rocfft_plan_description_t {
    _unused: [u8; 0],
}
/** @brief Pointer type to plan description structure
  @details This type is used to declare a plan description handle that can be
 initialized with ::rocfft_plan_description_create.*/
pub type rocfft_plan_description = *mut rocfft_plan_description_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rocfft_execution_info_t {
    _unused: [u8; 0],
}
/** @brief Pointer type to execution info structure
  @details This type is used to declare an execution info handle that can be
 initialized with ::rocfft_execution_info_create.*/
pub type rocfft_execution_info = *mut rocfft_execution_info_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rocfft_field_t {
    _unused: [u8; 0],
}
/** @brief Pointer type to a rocFFT field structure.

  @details rocFFT fields are used to hold data decomposition information which is then passed to a
  \ref rocfft_plan via a \ref rocfft_plan_description

  @warning Experimental!  This feature is part of an experimental API preview.*/
pub type rocfft_field = *mut rocfft_field_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rocfft_brick_t {
    _unused: [u8; 0],
}
/** @brief Pointer type to a rocFFT brick structure.

  @details rocFFT bricks are used to describe the data decomposition of fields.

  @warning Experimental!  This feature is part of an experimental API preview.*/
pub type rocfft_brick = *mut rocfft_brick_t;
impl rocfft_transform_type_e {
    pub const rocfft_transform_type_complex_forward: rocfft_transform_type_e = rocfft_transform_type_e(
        0,
    );
}
impl rocfft_transform_type_e {
    pub const rocfft_transform_type_complex_inverse: rocfft_transform_type_e = rocfft_transform_type_e(
        1,
    );
}
impl rocfft_transform_type_e {
    pub const rocfft_transform_type_real_forward: rocfft_transform_type_e = rocfft_transform_type_e(
        2,
    );
}
impl rocfft_transform_type_e {
    pub const rocfft_transform_type_real_inverse: rocfft_transform_type_e = rocfft_transform_type_e(
        3,
    );
}
#[repr(transparent)]
/// @brief Type of transform
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocfft_transform_type_e(pub ::core::ffi::c_uint);
/// @brief Type of transform
pub use self::rocfft_transform_type_e as rocfft_transform_type;
impl rocfft_precision_e {
    pub const rocfft_precision_single: rocfft_precision_e = rocfft_precision_e(0);
}
impl rocfft_precision_e {
    pub const rocfft_precision_double: rocfft_precision_e = rocfft_precision_e(1);
}
impl rocfft_precision_e {
    pub const rocfft_precision_half: rocfft_precision_e = rocfft_precision_e(2);
}
#[repr(transparent)]
/// @brief Precision
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocfft_precision_e(pub ::core::ffi::c_uint);
/// @brief Precision
pub use self::rocfft_precision_e as rocfft_precision;
impl rocfft_result_placement_e {
    pub const rocfft_placement_inplace: rocfft_result_placement_e = rocfft_result_placement_e(
        0,
    );
}
impl rocfft_result_placement_e {
    pub const rocfft_placement_notinplace: rocfft_result_placement_e = rocfft_result_placement_e(
        1,
    );
}
#[repr(transparent)]
/** @brief Result placement
  @details Declares where the output of the transform should be
  placed.  Note that input buffers may still be overwritten
  during execution of a transform, even if the transform is not
  in-place.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocfft_result_placement_e(pub ::core::ffi::c_uint);
/** @brief Result placement
  @details Declares where the output of the transform should be
  placed.  Note that input buffers may still be overwritten
  during execution of a transform, even if the transform is not
  in-place.*/
pub use self::rocfft_result_placement_e as rocfft_result_placement;
impl rocfft_array_type_e {
    pub const rocfft_array_type_complex_interleaved: rocfft_array_type_e = rocfft_array_type_e(
        0,
    );
}
impl rocfft_array_type_e {
    pub const rocfft_array_type_complex_planar: rocfft_array_type_e = rocfft_array_type_e(
        1,
    );
}
impl rocfft_array_type_e {
    pub const rocfft_array_type_real: rocfft_array_type_e = rocfft_array_type_e(2);
}
impl rocfft_array_type_e {
    pub const rocfft_array_type_hermitian_interleaved: rocfft_array_type_e = rocfft_array_type_e(
        3,
    );
}
impl rocfft_array_type_e {
    pub const rocfft_array_type_hermitian_planar: rocfft_array_type_e = rocfft_array_type_e(
        4,
    );
}
impl rocfft_array_type_e {
    pub const rocfft_array_type_unset: rocfft_array_type_e = rocfft_array_type_e(5);
}
#[repr(transparent)]
/// @brief Array type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocfft_array_type_e(pub ::core::ffi::c_uint);
/// @brief Array type
pub use self::rocfft_array_type_e as rocfft_array_type;
impl rocfft_comm_type_e {
    pub const rocfft_comm_none: rocfft_comm_type_e = rocfft_comm_type_e(0);
}
impl rocfft_comm_type_e {
    pub const rocfft_comm_mpi: rocfft_comm_type_e = rocfft_comm_type_e(1);
}
#[repr(transparent)]
/// @brief Communicator type for distributed transforms
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rocfft_comm_type_e(pub ::core::ffi::c_uint);
/// @brief Communicator type for distributed transforms
pub use self::rocfft_comm_type_e as rocfft_comm_type;
#[cfg(not(windows))]
extern "C" {
    /** @brief Library setup function, called once in program before start of
 library use*/
    pub fn rocfft_setup() -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Library cleanup function, called once in program after end of library
 use*/
    pub fn rocfft_cleanup() -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Create an FFT plan

  @details This API creates a plan, which the user can execute
  subsequently.  This function takes many of the fundamental
  parameters needed to specify a transform.

  The dimensions parameter can take a value of 1, 2, or 3. The
  'lengths' array specifies the size of data in each dimension. Note
  that lengths[0] is the size of the innermost dimension, lengths[1]
  is the next higher dimension and so on (column-major ordering).

  The 'number_of_transforms' parameter specifies how many
  transforms (of the same kind) needs to be computed. By specifying
  a value greater than 1, a batch of transforms can be computed
  with a single API call.

  Additionally, a handle to a plan description can be passed for
  more detailed transforms. For simple transforms, this parameter
  can be set to NULL.

  The plan must be destroyed with a call to ::rocfft_plan_destroy.

  @param[out] plan plan handle
  @param[in] placement placement of result
  @param[in] transform_type type of transform
  @param[in] precision precision
  @param[in] dimensions dimensions
  @param[in] lengths dimensions-sized array of transform lengths
  @param[in] number_of_transforms number of transforms
  @param[in] description description handle created by
 rocfft_plan_description_create; can be
  NULL for simple transforms*/
    pub fn rocfft_plan_create(
        plan: *mut rocfft_plan,
        placement: rocfft_result_placement,
        transform_type: rocfft_transform_type,
        precision: rocfft_precision,
        dimensions: usize,
        lengths: *const usize,
        number_of_transforms: usize,
        description: rocfft_plan_description,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Execute an FFT plan

  @details This API executes an FFT plan on buffers given by the user.

  If the transform is in-place, only the input buffer is needed and
  the output buffer parameter can be set to NULL. For not in-place
  transforms, output buffers have to be specified.

  Input and output buffers are arrays of pointers.  Interleaved
  array formats are the default, and require just one pointer per
  input or output buffer.  Planar array formats require two
  pointers per input or output buffer - real and imaginary
  pointers, in that order.

  If fields have been set for transform input or output, these
  arrays have one pointer per brick in the input or output field,
  provided in the order that the bricks were added to the field.

  Note that input buffers may still be overwritten during execution
  of a transform, even if the transform is not in-place.

  The final parameter in this function is a rocfft_execution_info
  handle. This optional parameter serves as a way for the user to control
  execution streams and work buffers.

  @param[in] plan plan handle
  @param[in,out] in_buffer array (of size 1 for interleaved data, of size 2
 for planar data, or one per brick if an input field is set) of input buffers
  @param[in,out] out_buffer array (of size 1 for interleaved data, of size 2
 for planar data, or one per brick if an output field is set) of output buffers,
 ignored for in-place transforms
  @param[in] info execution info handle created by
 rocfft_execution_info_create*/
    pub fn rocfft_execute(
        plan: rocfft_plan,
        in_buffer: *mut *mut ::core::ffi::c_void,
        out_buffer: *mut *mut ::core::ffi::c_void,
        info: rocfft_execution_info,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Destroy an FFT plan
  @details This API frees the plan after it is no longer needed.
  @param[in] plan plan handle*/
    pub fn rocfft_plan_destroy(plan: rocfft_plan) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set scaling factor.
  @details rocFFT multiplies each element of the result by the given factor at the end of the transform.

  The supplied factor must be a finite number.  That is, it must neither be infinity nor NaN.

  @param[in] description description handle
  @param[in] scale_factor scaling factor*/
    pub fn rocfft_plan_description_set_scale_factor(
        description: rocfft_plan_description,
        scale_factor: f64,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /**  @brief Set advanced data layout parameters on a plan description

  @details This API specifies advanced layout of input/output
  buffers for a plan description.

  The following parameters are supported for inputs and outputs:

  * Array type (real, hermitian, or complex data, in either
    interleaved or planar format).
      * Real forward transforms require real input and hermitian output.
      * Real inverse transforms require hermitian input and real output.
      * Complex transforms require complex input and output.
      * Hermitian and complex data defaults to interleaved if a specific
format is not specified.
  * Offset of first data element in the data buffer.  Defaults to 0 if unspecified.
  * Stride between consecutive elements in each dimension.  Defaults
to contiguous data in all dimensions if unspecified.
  * Distance between consecutive batches.  Defaults to contiguous
batches if unspecified.

  Not all combinations of array types are supported and error codes
  will be returned for unsupported cases.

  Offset, stride, and distance for either input or output provided
  here is ignored if a field is set for the corresponding input or
  output.

  @param[in, out] description description handle
  @param[in] in_array_type array type of input buffer
  @param[in] out_array_type array type of output buffer
  @param[in] in_offsets offsets, in element units, to start of data in input buffer
  @param[in] out_offsets offsets, in element units, to start of data in output buffer
  @param[in] in_strides_size size of in_strides array (must be equal to transform dimensions)
  @param[in] in_strides array of strides, in each dimension, of
   input buffer; if set to null ptr library chooses defaults
  @param[in] in_distance distance between start of each data instance in input buffer
  @param[in] out_strides_size size of out_strides array (must be
  equal to transform dimensions)
  @param[in] out_strides array of strides, in each dimension, of
   output buffer; if set to null ptr library chooses defaults
  @param[in] out_distance distance between start of each data instance in output buffer*/
    pub fn rocfft_plan_description_set_data_layout(
        description: rocfft_plan_description,
        in_array_type: rocfft_array_type,
        out_array_type: rocfft_array_type,
        in_offsets: *const usize,
        out_offsets: *const usize,
        in_strides_size: usize,
        in_strides: *const usize,
        in_distance: usize,
        out_strides_size: usize,
        out_strides: *const usize,
        out_distance: usize,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Create a rocfft field struct.

  @warning Experimental!  This feature is part of an experimental API preview.*/
    pub fn rocfft_field_create(field: *mut rocfft_field) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Destroy a rocfft field struct

 The field struct can be destroyed after being added to the plan description; it is not used for
 plan execution.

  @warning Experimental!  This feature is part of an experimental API preview.*/
    pub fn rocfft_field_destroy(field: rocfft_field) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Get library version string

 @param[in, out] buf buffer that receives the version string
 @param[in] len length of buf, minimum 30 characters*/
    pub fn rocfft_get_version_string(
        buf: *mut ::core::ffi::c_char,
        len: usize,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set the communication library for distributed transforms.

  @details Set the multi-processing communication library for a plan.

  Multi-processing communication libraries require library-specific
  handle to also be specified.  For MPI libraries, this is a
  pointer to an MPI communicator.

  @param[in] description description handle
  @param[in] comm_type communicator type
  @param[in] comm_handle handle to communication-library-specific state
*/
    pub fn rocfft_plan_description_set_comm(
        description: rocfft_plan_description,
        comm_type: rocfft_comm_type,
        comm_handle: *mut ::core::ffi::c_void,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Define a brick as part of a decomposition of a field.

 Fields can contain a full-dimensional data distribution.  The
 decomposition is specified by providing a lower coordinate and an
 upper coordinate in the field's index space.  The lower coordinate
 is inclusive (contained within the brick) and the upper coordinate
 is exclusive (first index past the end of the brick).

 One must also provide a stride for the brick data which specifies
 how the brick's data is arranged in memory.

 All coordinates and strides must include batch dimensions, and are in
 column-major order (fastest-moving dimension first).

 A HIP device ID is also provided - each brick may reside on a
 different device.

 All arrays may be re-used or freed immediately after the function returns.

 @param[out] brick: brick structure
 @param[in] field_lower: array of length dim specifying the lower index (inclusive) for the brick in the
 field's index space.
 @param[in] field_upper: array of length dim specifying the upper index (exclusive) for the brick in the
 field's index space.
 @param[in] brick_stride: array of length dim specifying the brick's stride in memory
 @param[in] dim_with_batch length of the arrays; this must match the dimension of
 the FFT plus one for the batch dimension.
 @param[in] deviceID: HIP device ID for the device on which the brick's data is resident.

  @warning Experimental!  This feature is part of an experimental API preview.*/
    pub fn rocfft_brick_create(
        brick: *mut rocfft_brick,
        field_lower: *const usize,
        field_upper: *const usize,
        brick_stride: *const usize,
        dim_with_batch: usize,
        deviceID: ::core::ffi::c_int,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Deallocate a brick created with rocfft_brick_create.

  @warning Experimental!  This feature is part of an experimental API preview.*/
    pub fn rocfft_brick_destroy(brick: rocfft_brick) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Add a brick to a field.

 Note that the order in which the bricks are added is significant;
 the pointers provided for each brick to ::rocfft_execute are in
 the same order that the bricks were added to the field.

 The brick may be added to another field or destroyed any time
 after this function returns.

 @param[in, out] field: \ref rocfft_field struct which holds the brick decomposition.
 @param[in] brick: \ref rocfft_brick struct to add to the field.

  @warning Experimental!  This feature is part of an experimental API preview.*/
    pub fn rocfft_field_add_brick(
        field: rocfft_field,
        brick: rocfft_brick,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Add a \ref rocfft_field to a \ref rocfft_plan_description as an input.

 The field may be reused or freed immediately after the function returns.

 @param[in, out] description: \ref rocfft_plan_description that will pass the field information to plan creation
 @param[in] field: \ref rocfft_field struct added as an input field

  @warning Experimental!  This feature is part of an experimental API preview.*/
    pub fn rocfft_plan_description_add_infield(
        description: rocfft_plan_description,
        field: rocfft_field,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Add a \ref rocfft_field to a \ref rocfft_plan_description as an output.

 The field may be reused or freed immediately after the function returns.

 @param[in, out] description: \ref rocfft_plan_description  that will pass the field information to plan creation
 @param[in] field: \ref rocfft_field struct added as an output field

  @warning Experimental!  This feature is part of an experimental API preview.*/
    pub fn rocfft_plan_description_add_outfield(
        description: rocfft_plan_description,
        field: rocfft_field,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Get work buffer size
  @details Get the work buffer size required for a plan.
  @param[in] plan plan handle
  @param[out] size_in_bytes size of needed work buffer in bytes*/
    pub fn rocfft_plan_get_work_buffer_size(
        plan: rocfft_plan,
        size_in_bytes: *mut usize,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Print all plan information
  @details Prints plan details to stdout, to aid debugging
  @param[in] plan plan handle*/
    pub fn rocfft_plan_get_print(plan: rocfft_plan) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Create plan description
  @details This API creates a plan description with which the user
 can set extra plan properties.  The plan description must be freed
 with a call to ::rocfft_plan_description_destroy.
  @param[out] description plan description handle*/
    pub fn rocfft_plan_description_create(
        description: *mut rocfft_plan_description,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Destroy a plan description
  @details This API frees the plan description.  A plan description
  can be freed any time after it is passed to ::rocfft_plan_create.
  @param[in] description plan description handle*/
    pub fn rocfft_plan_description_destroy(
        description: rocfft_plan_description,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Create execution info
  @details This API creates an execution info with which the user
 can control plan execution and work buffers.  The execution info must be freed
 with a call to ::rocfft_execution_info_destroy.
  @param[out] info execution info handle*/
    pub fn rocfft_execution_info_create(
        info: *mut rocfft_execution_info,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Destroy an execution info
  @details This API frees the execution info.  An execution info
  object can be freed any time after it is passed to
  ::rocfft_execute.
  @param[in] info execution info handle*/
    pub fn rocfft_execution_info_destroy(info: rocfft_execution_info) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set work buffer in execution info

  @details This is one of the execution info functions to specify
  optional additional information to control execution.  This API
  provides a work buffer for the transform. It must be called
  before ::rocfft_execute.

  When a non-zero value is obtained from
  ::rocfft_plan_get_work_buffer_size, that means the library needs a
  work buffer to compute the transform. In this case, the user
  should allocate the work buffer and pass it to the library via
  this API.

  If a work buffer is required for the transform but is not
  specified using this function, ::rocfft_execute will automatically
  allocate the required buffer and free it when execution is
  finished.

  Users should allocate their own work buffers if they need precise
  control over the lifetimes of those buffers, or if multiple plans
  need to share the same buffer.

  @param[in] info execution info handle
  @param[in] work_buffer work buffer
  @param[in] size_in_bytes size of work buffer in bytes*/
    pub fn rocfft_execution_info_set_work_buffer(
        info: rocfft_execution_info,
        work_buffer: *mut ::core::ffi::c_void,
        size_in_bytes: usize,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set stream in execution info
  @details Associates an existing compute stream to a plan.  This
 must be called before the call to ::rocfft_execute.

  Once the association is made, execution of the FFT will run the
  computation through the specified stream.

  The stream must be of type hip_runtime_sys::hipStream_t. It is an error to pass
  the address of a hip_runtime_sys::hipStream_t object.

  @param[in] info execution info handle
  @param[in] stream underlying compute stream*/
    pub fn rocfft_execution_info_set_stream(
        info: rocfft_execution_info,
        stream: *mut ::core::ffi::c_void,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set a load callback for a plan execution (experimental)
  @details This function specifies a user-defined callback function
  that is run to load input from global memory at the start of the
  transform.  Callbacks are an experimental feature in rocFFT.

  Callback function pointers/data are given as arrays, with one
  function/data pointer per brick in the input field of the plan.
  A plan with no input field specified is considered to have one
  brick.

  All functions in the array must perform the same logical
  operation.  That is, any function in the array must be
  substitutable for any other function in the array if the data
  being loaded were moved to another brick.

  The provided function pointers replace any previously-specified
  load callback for this execution info handle.

  Load callbacks have the following signature:

  @code
  Tdata load_cb(Tdata* data, size_t offset, void* cbdata, void* sharedMem);
  @endcode

  'Tdata' is the type of a single element of the input buffer.  It is
  the caller's responsibility to ensure that the function type is
  appropriate for the plan (for example, a single-precision
  real-to-complex transform would load single-precision real
  elements).

  A null value for 'cb_functions' may be specified to clear any
  previously registered load callback.  'cb_data' may be null if
  the functions require no additional pointer to be passed to them.

  Currently, 'shared_mem_bytes' must be 0.  Callbacks are not
  supported on transforms that use planar formats for either input
  or output.

  @param[in] info execution info handle
  @param[in] cb_functions callback function pointers
  @param[in] cb_data callback function data, passed to the function pointer when it is called
  @param[in] shared_mem_bytes amount of shared memory to allocate for the callback function to use*/
    pub fn rocfft_execution_info_set_load_callback(
        info: rocfft_execution_info,
        cb_functions: *mut *mut ::core::ffi::c_void,
        cb_data: *mut *mut ::core::ffi::c_void,
        shared_mem_bytes: usize,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Set a store callback for a plan execution (experimental)
  @details This function specifies a user-defined callback function
  that is run to store output to global memory at the end of the
  transform.  Callbacks are an experimental feature in rocFFT.

  Callback function pointers/data are given as arrays, with one
  function/data pointer per device executing this plan.  Currently,
  plans can only use one device.

  Callback function pointers/data are given as arrays, with one
  function/data pointer per brick in the output field of the plan.
  A plan with no output field specified is considered to have one
  brick.

  All functions in the array must perform the same logical
  operation.  That is, any function in the array must be
  substitutable for any other function in the array if the data
  being stored were moved to another brick.

  The provided function pointers replace any previously-specified
  store callback for this execution info handle.

  Store callbacks have the following signature:

  @code
  void store_cb(Tdata* data, size_t offset, Tdata element, void* cbdata, void* sharedMem);
  @endcode

  'Tdata' is the type of a single element of the output buffer.  It is
  the caller's responsibility to ensure that the function type is
  appropriate for the plan (for example, a single-precision
  real-to-complex transform would store single-precision complex
  elements).

  A null value for 'cb_functions' may be specified to clear any
  previously registered load callback.  'cb_data' may be null if
  the functions require no additional pointer to be passed to them.

  Currently, 'shared_mem_bytes' must be 0.  Callbacks are not
  supported on transforms that use planar formats for either input
  or output.

  @param[in] info execution info handle
  @param[in] cb_functions callback function pointers
  @param[in] cb_data callback function data, passed to the function pointer when it is called
  @param[in] shared_mem_bytes amount of shared memory to allocate for the callback function to use*/
    pub fn rocfft_execution_info_set_store_callback(
        info: rocfft_execution_info,
        cb_functions: *mut *mut ::core::ffi::c_void,
        cb_data: *mut *mut ::core::ffi::c_void,
        shared_mem_bytes: usize,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Serialize compiled kernel cache

  @details Serialize rocFFT's cache of compiled kernels into a
  buffer.  This buffer is allocated by rocFFT and must be freed
  with a call to ::rocfft_cache_buffer_free.  The length of the
  buffer in bytes is written to 'buffer_len_bytes'.*/
    pub fn rocfft_cache_serialize(
        buffer: *mut *mut ::core::ffi::c_void,
        buffer_len_bytes: *mut usize,
    ) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Free cache serialization buffer

  @details Deallocate a buffer allocated by ::rocfft_cache_serialize.*/
    pub fn rocfft_cache_buffer_free(buffer: *mut ::core::ffi::c_void) -> rocfft_status;
}
#[cfg(not(windows))]
extern "C" {
    /** @brief Deserialize a buffer into the compiled kernel cache.

  @details Kernels in the buffer that match already-cached kernels
  will replace those kernels that are in the cache.  Already-cached
  kernels that do not match those in the buffer are unmodified by
  this operation.  The cache is unmodified if either a null buffer
  pointer or a zero length is passed.*/
    pub fn rocfft_cache_deserialize(
        buffer: *const ::core::ffi::c_void,
        buffer_len_bytes: usize,
    ) -> rocfft_status;
}
impl rocfft_error {
    pub const r#failure: rocfft_error = rocfft_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const r#invalid_arg_value: rocfft_error = rocfft_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const r#invalid_dimensions: rocfft_error = rocfft_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const r#invalid_array_type: rocfft_error = rocfft_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const r#invalid_strides: rocfft_error = rocfft_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const r#invalid_distance: rocfft_error = rocfft_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const r#invalid_offset: rocfft_error = rocfft_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const r#invalid_work_buffer: rocfft_error = rocfft_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct rocfft_error(pub ::core::num::NonZeroU32);
pub trait rocfft_statusConsts {
    const success: rocfft_status = rocfft_status::Ok(());
    const error_failure: rocfft_status = rocfft_status::Err(rocfft_error::r#failure);
    const error_invalid_arg_value: rocfft_status = rocfft_status::Err(
        rocfft_error::r#invalid_arg_value,
    );
    const error_invalid_dimensions: rocfft_status = rocfft_status::Err(
        rocfft_error::r#invalid_dimensions,
    );
    const error_invalid_array_type: rocfft_status = rocfft_status::Err(
        rocfft_error::r#invalid_array_type,
    );
    const error_invalid_strides: rocfft_status = rocfft_status::Err(
        rocfft_error::r#invalid_strides,
    );
    const error_invalid_distance: rocfft_status = rocfft_status::Err(
        rocfft_error::r#invalid_distance,
    );
    const error_invalid_offset: rocfft_status = rocfft_status::Err(
        rocfft_error::r#invalid_offset,
    );
    const error_invalid_work_buffer: rocfft_status = rocfft_status::Err(
        rocfft_error::r#invalid_work_buffer,
    );
}
impl rocfft_statusConsts for rocfft_status {}
#[must_use]
pub type rocfft_status = ::core::result::Result<(), rocfft_error>;
const _: fn() = || {
    let _ = std::mem::transmute::<rocfft_status, u32>;
};
unsafe impl Send for rocfft_plan {}
unsafe impl Sync for rocfft_plan {}
