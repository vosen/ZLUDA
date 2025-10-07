// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub const MIOPEN_BACKEND_OPENCL: u32 = 0;
pub const MIOPEN_BACKEND_HIP: u32 = 1;
pub const MIOPEN_MODE_NOGPU: u32 = 0;
pub const MIOPEN_USE_ROCBLAS: u32 = 1;
pub const MIOPEN_USE_HIPBLASLT: u32 = 1;
pub const MIOPEN_USE_ROCTRACER: u32 = 1;
pub const MIOPEN_BUILD_DEV: u32 = 0;
pub const MIOPEN_GPU_SYNC: u32 = 0;
pub const MIOPEN_ENABLE_SQLITE: u32 = 1;
pub const MIOPEN_ENABLE_SQLITE_KERN_CACHE: u32 = 1;
pub const MIOPEN_DEBUG_FIND_DB_CACHING: u32 = 1;
pub const MIOPEN_USE_COMGR: u32 = 1;
pub const MIOPEN_USE_HIPRTC: u32 = 1;
pub const MIOPEN_USE_HIP_KERNELS: u32 = 1;
pub const MIOPEN_DISABLE_USERDB: u32 = 0;
pub const MIOPEN_EMBED_DB: u32 = 0;
pub const MIOPEN_DISABLE_SYSDB: u32 = 0;
pub const MIOPEN_LOG_FUNC_TIME_ENABLE: u32 = 0;
pub const MIOPEN_ENABLE_SQLITE_BACKOFF: u32 = 1;
pub const MIOPEN_USE_MLIR: u32 = 1;
pub const MIOPEN_USE_COMPOSABLEKERNEL: u32 = 1;
pub const MIOPEN_BUILD_DRIVER: u32 = 1;
pub const MIOPEN_ENABLE_AI_IMMED_MODE_FALLBACK: u32 = 1;
pub const MIOPEN_ENABLE_AI_KERNEL_TUNING: u32 = 1;
pub const MIOPEN_HIP_COMPILER_HAS_OPTION_OFFLOAD_UNIFORM_BLOCK: u32 = 1;
pub const MIOPEN_WORKAROUND_USE_BOOST_FILESYSTEM: u32 = 0;
pub const MIOPEN_ENABLE_FIN_INTERFACE: u32 = 0;
pub const MIOPEN_AMD_COMGR_VERSION_MAJOR: u32 = 3;
pub const MIOPEN_AMD_COMGR_VERSION_MINOR: u32 = 0;
pub const MIOPEN_AMD_COMGR_VERSION_PATCH: u32 = 0;
pub const MIOPEN_USE_RNE_BFLOAT16: u32 = 1;
pub const MIOPEN_FP8_IEEE_EXPONENT_BIAS: u32 = 0;
pub const MIOPEN_FP8_CLIPPING: u32 = 1;
pub const MIOPEN_OFFLINE_COMPILER_PATHS_V2: u32 = 1;
pub const MIOPEN_CACHE_DIR: &[u8; 17] = b"~/.cache/miopen/\0";
pub const MIOPEN_USE_SQLITE_PERFDB: u32 = 0;
pub const MIOPEN_NDEBUG: u32 = 0;
pub const MIOPEN_ALLOC_BUFFERS: u32 = 0;
pub const MIOPEN_ROCBLAS_VERSION_MAJOR: u32 = 4;
pub const MIOPEN_ROCBLAS_VERSION_MINOR: u32 = 4;
pub const MIOPEN_ROCBLAS_VERSION_PATCH: u32 = 0;
pub const MIOPEN_ROCBLAS_VERSION_FLAT: u32 = 4004000;
pub const MIOPEN_HIPBLASLT_VERSION_MAJOR: u32 = 0;
pub const MIOPEN_HIPBLASLT_VERSION_MINOR: u32 = 12;
pub const MIOPEN_HIPBLASLT_VERSION_PATCH: u32 = 0;
pub const MIOPEN_HIPBLASLT_VERSION_FLAT: u32 = 12000;
pub const MIOPEN_GOLDEN_DB_VERSION: u32 = 21;
pub const MIOPEN_API_VERSION_REDUCE_TENSOR: u32 = 1;
pub type miopenAcceleratorQueue_t = hip_runtime_sys::hipStream_t;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenHandle {
    pub _address: u8,
}
pub type miopenHandle_t = *mut miopenHandle;
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    /** @brief Get character string for an error code.

 A function which returns a NULL terminated character string of the error code.

 @param error  miopenStatus_t type error status (input)
 @return       errorString*/
    pub fn miopenGetErrorString(error: miopenStatus_t) -> *const ::core::ffi::c_char;
}
/** @brief Custom allocator function

 This function allow for user-defined custom allocator

 @param context     A pointer a context (input)
 @param sizeBytes   Number of bytes to allocate (input)
*/
pub type miopenAllocatorFunction = ::core::option::Option<
    unsafe extern "C" fn(
        context: *mut ::core::ffi::c_void,
        sizeBytes: usize,
    ) -> *mut ::core::ffi::c_void,
>;
/** @brief Custom deallocator function

 This function allow for user-defined custom deallocation function

 @param context     A pointer context (input)
 @param memory      A pointer allocated memory (input)
*/
pub type miopenDeallocatorFunction = ::core::option::Option<
    unsafe extern "C" fn(
        context: *mut ::core::ffi::c_void,
        memory: *mut ::core::ffi::c_void,
    ),
>;
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Method to return version of MIOpen

 The output values of this call follow from the versioning
 format major.minor.patch

 Pointers that are NULL will be ignored.

 @param major     Major version number (output)
 @param minor     Minor version number (output)
 @param patch     Patch version number (output)

 @return          miopenStatus_t*/
    pub fn miopenGetVersion(
        major: *mut usize,
        minor: *mut usize,
        patch: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Method to create the MIOpen handle object.

 This function creates a MIOpen handle. This is called at the very start to initialize the MIOpen
 environment.
 @param handle     A pointer to a MIOpen handle type (output)

 @return           miopenStatus_t*/
    pub fn miopenCreate(handle: *mut miopenHandle_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create a MIOpen handle with an accelerator stream.

 The HIP side uses a hip_runtime_sys::hipStream_t type for the stream, while OpenCL will use a
 cl_command_queue.

 Create a handle with a previously created accelerator command queue.
 @param handle     A pointer to a MIOpen handle type (output)
 @param stream      An accelerator queue type (input)

 @return           miopenStatus_t*/
    pub fn miopenCreateWithStream(
        handle: *mut miopenHandle_t,
        stream: miopenAcceleratorQueue_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the MIOpen handle.

 This is called when breaking down the MIOpen environment.
 @param handle     MIOpen handle (input)
 @return           miopenStatus_t*/
    pub fn miopenDestroy(handle: miopenHandle_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set accelerator command queue previously created

 Set a command queue for an accelerator device
 @param handle     MIOpen handle (input)
 @param streamID   An accelerator queue type (input)
 @return           miopenStatus_t*/
    pub fn miopenSetStream(
        handle: miopenHandle_t,
        streamID: miopenAcceleratorQueue_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the previously created accelerator command queue

 Creates a command queue for an accelerator device
 @param handle     MIOpen handle (input)
 @param streamID   Pointer to a accelerator queue type (output)
 @return           miopenStatus_t*/
    pub fn miopenGetStream(
        handle: miopenHandle_t,
        streamID: *mut miopenAcceleratorQueue_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set allocator for previously created miopenHandle

 Set a command queue for an accelerator device
 @param handle     MIOpen handle
 @param allocator  A callback function MIOpen will use for internal memory allocations.
      The provided callback function should allocate device memory with requested size
      and return a pointer to this memory.
      Passing 0 will restore the default MIOpen allocator and deallocator.
 @param deallocator  A callback function MIOpen will use to for internal memory deallocation.
      The provided callback function should free the specified memory pointer
 @param allocatorContext  User-specified pointer which is passed to \p allocator and \p
 deallocator
      This allows the callback function to access state set by the caller to this function,
      for example a stateful heap allocator or a c++ class.
 @return           miopenStatus_t*/
    pub fn miopenSetAllocator(
        handle: miopenHandle_t,
        allocator: miopenAllocatorFunction,
        deallocator: miopenDeallocatorFunction,
        allocatorContext: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get time for last kernel launched

 This function is used only when profiling mode has been enabled.
 Kernel timings are based on the MIOpen handle and is not thread-safe.
 In order to use multi-threaded profiling, create an MIOpen handle for each
 concurrent thread.

 @param handle     MIOpen handle (input)
 @param time       Pointer to a float type to contain kernel time in milliseconds (output)
 @return           miopenStatus_t*/
    pub fn miopenGetKernelTime(handle: miopenHandle_t, time: *mut f32) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enable profiling to retrieve kernel time

 Enable or disable kernel profiling. This profiling is only for kernel time.
 @param handle     MIOpen handle (input)
 @param enable     Boolean to toggle profiling (input)
 @return           miopenStatus_t*/
    pub fn miopenEnableProfiling(handle: miopenHandle_t, enable: bool) -> miopenStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenFusionOpDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenFusionOpDescriptor_t(pub *mut miopenFusionOpDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenTensorDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenTensorDescriptor_t(pub *mut miopenTensorDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenSeqTensorDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenSeqTensorDescriptor_t(pub *mut miopenSeqTensorDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenConvolutionDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenConvolutionDescriptor_t(pub *mut miopenConvolutionDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenPoolingDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenPoolingDescriptor_t(pub *mut miopenPoolingDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenLRNDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenLRNDescriptor_t(pub *mut miopenLRNDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenActivationDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenActivationDescriptor_t(pub *mut miopenActivationDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNDescriptor_t(pub *mut miopenRNNDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenCTCLossDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenCTCLossDescriptor_t(pub *mut miopenCTCLossDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenDropoutDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenDropoutDescriptor_t(pub *mut miopenDropoutDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenReduceTensorDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenReduceTensorDescriptor_t(pub *mut miopenReduceTensorDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenMhaDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenMhaDescriptor_t(pub *mut miopenMhaDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenSoftmaxDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenSoftmaxDescriptor_t(pub *mut miopenSoftmaxDescriptor);
impl miopenDataType_t {
    ///< 16-bit floating point (Fully supported)
    pub const miopenHalf: miopenDataType_t = miopenDataType_t(0);
}
impl miopenDataType_t {
    ///< 32-bit floating point (Fully supported)
    pub const miopenFloat: miopenDataType_t = miopenDataType_t(1);
}
impl miopenDataType_t {
    ///< 32-bit integer (Partially supported)
    pub const miopenInt32: miopenDataType_t = miopenDataType_t(2);
}
impl miopenDataType_t {
    ///< 8-bit integer (Partially supported)
    pub const miopenInt8: miopenDataType_t = miopenDataType_t(3);
}
impl miopenDataType_t {
    /**< 16-bit binary floating point (8-bit exponent, 7-bit fraction)
(Partially supported)*/
    pub const miopenBFloat16: miopenDataType_t = miopenDataType_t(5);
}
impl miopenDataType_t {
    ///< 64-bit floating point (Partially supported)
    pub const miopenDouble: miopenDataType_t = miopenDataType_t(6);
}
impl miopenDataType_t {
    pub const miopenInt64: miopenDataType_t = miopenDataType_t(9);
}
#[repr(transparent)]
/** @ingroup tensor
 @enum miopenDataType_t
 MIOpen floating point datatypes. Both 32-bit and 16-bit floats are supported in MIOpen.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenDataType_t(pub ::core::ffi::c_uint);
impl miopenTensorLayout_t {
    ///< NCHW memory layout (Fully supported)
    pub const miopenTensorNCHW: miopenTensorLayout_t = miopenTensorLayout_t(0);
}
impl miopenTensorLayout_t {
    ///< NHWC memory layout (Fully supported)
    pub const miopenTensorNHWC: miopenTensorLayout_t = miopenTensorLayout_t(1);
}
impl miopenTensorLayout_t {
    ///< CHWN memory layout (Not supported)
    pub const miopenTensorCHWN: miopenTensorLayout_t = miopenTensorLayout_t(2);
}
impl miopenTensorLayout_t {
    ///< NCHWc4 memory layout (Partially supported)
    pub const miopenTensorNCHWc4: miopenTensorLayout_t = miopenTensorLayout_t(3);
}
impl miopenTensorLayout_t {
    ///< NCHWc8 memory layout (Partially supported)
    pub const miopenTensorNCHWc8: miopenTensorLayout_t = miopenTensorLayout_t(4);
}
impl miopenTensorLayout_t {
    ///< CHWNc4 memory layout (Partially supported)
    pub const miopenTensorCHWNc4: miopenTensorLayout_t = miopenTensorLayout_t(5);
}
impl miopenTensorLayout_t {
    ///< CHWNc8 memory layout (Partially supported)
    pub const miopenTensorCHWNc8: miopenTensorLayout_t = miopenTensorLayout_t(6);
}
impl miopenTensorLayout_t {
    ///< NCDHW memory layout (Fully supported)
    pub const miopenTensorNCDHW: miopenTensorLayout_t = miopenTensorLayout_t(7);
}
impl miopenTensorLayout_t {
    ///< NCDHW memory layout (Fully supported)
    pub const miopenTensorNDHWC: miopenTensorLayout_t = miopenTensorLayout_t(8);
}
#[repr(transparent)]
/** @ingroup tensor
 @enum miopenTensorLayout_t
 Tensor layouts supported by MIOpen.
 miopenTensorCHWNc4 and miopenTensorCHWNc8 layout only support weight tensor.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenTensorLayout_t(pub ::core::ffi::c_uint);
impl miopenIndexType_t {
    ///<  8-bit unsigned
    pub const miopenIndexUint8: miopenIndexType_t = miopenIndexType_t(0);
}
impl miopenIndexType_t {
    ///< 16-bit unsigned
    pub const miopenIndexUint16: miopenIndexType_t = miopenIndexType_t(1);
}
impl miopenIndexType_t {
    ///< 32-bit unsigned
    pub const miopenIndexUint32: miopenIndexType_t = miopenIndexType_t(2);
}
impl miopenIndexType_t {
    ///< 64-bit unsigned
    pub const miopenIndexUint64: miopenIndexType_t = miopenIndexType_t(3);
}
#[repr(transparent)]
/** @ingroup pooling
 @enum miopenIndexType_t
 MIOpen index datatypes.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenIndexType_t(pub ::core::ffi::c_uint);
impl miopenTensorOp_t {
    ///< Add tensors element-wise
    pub const miopenTensorOpAdd: miopenTensorOp_t = miopenTensorOp_t(0);
}
impl miopenTensorOp_t {
    ///< Multiply two tensors element-wise
    pub const miopenTensorOpMul: miopenTensorOp_t = miopenTensorOp_t(1);
}
impl miopenTensorOp_t {
    ///< Minimum of tensor element pairs
    pub const miopenTensorOpMin: miopenTensorOp_t = miopenTensorOp_t(2);
}
impl miopenTensorOp_t {
    ///< Maximum of tensor element pairs
    pub const miopenTensorOpMax: miopenTensorOp_t = miopenTensorOp_t(3);
}
#[repr(transparent)]
/** @ingroup tensor
 @enum miopenTensorOp_t
 Element-wise tensor operation modes*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenTensorOp_t(pub ::core::ffi::c_uint);
impl miopenConvolutionMode_t {
    ///< Cross-Correlation convolution
    pub const miopenConvolution: miopenConvolutionMode_t = miopenConvolutionMode_t(0);
}
impl miopenConvolutionMode_t {
    ///< Transpose convolutions -- deconvolution
    pub const miopenTranspose: miopenConvolutionMode_t = miopenConvolutionMode_t(1);
}
impl miopenConvolutionMode_t {
    ///< Deprecated Group convolution legacy, ToBe Removed
    pub const miopenGroupConv: miopenConvolutionMode_t = miopenConvolutionMode_t(2);
}
impl miopenConvolutionMode_t {
    ///< Deprecated Depthwise convolution legacy, ToBe Removed
    pub const miopenDepthwise: miopenConvolutionMode_t = miopenConvolutionMode_t(3);
}
#[repr(transparent)]
/** @ingroup convolutions
  @enum miopenConvolutionMode_t
 Convolution mode selection for convolution layer preference.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenConvolutionMode_t(pub ::core::ffi::c_uint);
impl miopenPaddingMode_t {
    ///< MIOPEN Default Padding
    pub const miopenPaddingDefault: miopenPaddingMode_t = miopenPaddingMode_t(0);
}
impl miopenPaddingMode_t {
    ///< Tensorflow SAME Padding
    pub const miopenPaddingSame: miopenPaddingMode_t = miopenPaddingMode_t(1);
}
impl miopenPaddingMode_t {
    ///< Tensorflow VALID Padding
    pub const miopenPaddingValid: miopenPaddingMode_t = miopenPaddingMode_t(2);
}
#[repr(transparent)]
/** @ingroup padding
  @enum miopenPaddingMode_t
 Padding mode selection for convolution/Pooling layer preference*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenPaddingMode_t(pub ::core::ffi::c_uint);
impl miopenPoolingMode_t {
    ///< Maximum pooling
    pub const miopenPoolingMax: miopenPoolingMode_t = miopenPoolingMode_t(0);
}
impl miopenPoolingMode_t {
    ///< Average pooling
    pub const miopenPoolingAverage: miopenPoolingMode_t = miopenPoolingMode_t(1);
}
impl miopenPoolingMode_t {
    ///< Inclusive Average pooling
    pub const miopenPoolingAverageInclusive: miopenPoolingMode_t = miopenPoolingMode_t(
        2,
    );
}
#[repr(transparent)]
/** @ingroup pooling
 @enum miopenPoolingMode_t
 Pooling layer mode*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenPoolingMode_t(pub ::core::ffi::c_uint);
impl miopenPoolingWorkspaceIndexMode_t {
    ///< Use mask indices, 2D pooling only
    pub const miopenPoolingWorkspaceIndexMask: miopenPoolingWorkspaceIndexMode_t = miopenPoolingWorkspaceIndexMode_t(
        0,
    );
}
impl miopenPoolingWorkspaceIndexMode_t {
    ///< Use image indices
    pub const miopenPoolingWorkspaceIndexImage: miopenPoolingWorkspaceIndexMode_t = miopenPoolingWorkspaceIndexMode_t(
        1,
    );
}
#[repr(transparent)]
/** @ingroup pooling
 @enum miopenPoolingWorkspaceIndexMode_t
 Pooling layer workspace index mode. miopenPoolingWorkspaceIndexMask mode records indices
 indicating the max values' positions in the filter/mask. miopenPoolingWorkspaceIndexImage mode
 records indices indicating the max values' positions in the image.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenPoolingWorkspaceIndexMode_t(pub ::core::ffi::c_uint);
impl miopenLRNMode_t {
    ///< Channel independent
    pub const miopenLRNWithinChannel: miopenLRNMode_t = miopenLRNMode_t(0);
}
impl miopenLRNMode_t {
    ///< Cross Channel
    pub const miopenLRNCrossChannel: miopenLRNMode_t = miopenLRNMode_t(1);
}
#[repr(transparent)]
/** @ingroup LRN
 @enum miopenLRNMode_t
 Local Response Normalization layer mode*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenLRNMode_t(pub ::core::ffi::c_uint);
impl miopenBatchNormMode_t {
    ///< Element-wise normalization for fully connected layer
    pub const miopenBNPerActivation: miopenBatchNormMode_t = miopenBatchNormMode_t(0);
}
impl miopenBatchNormMode_t {
    ///< Mini-batch spatial normalization for convolutional layers
    pub const miopenBNSpatial: miopenBatchNormMode_t = miopenBatchNormMode_t(1);
}
#[repr(transparent)]
/** @ingroup batchnorm
 @enum miopenBatchNormMode_t
 Batch Normalization layer mode*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenBatchNormMode_t(pub ::core::ffi::c_uint);
impl miopenActivationMode_t {
    ///< No activation, pass through the data
    pub const miopenActivationPASTHRU: miopenActivationMode_t = miopenActivationMode_t(
        0,
    );
}
impl miopenActivationMode_t {
    ///< Sigmoid function: \f$1 / (1 + e^{-x})\f$
    pub const miopenActivationLOGISTIC: miopenActivationMode_t = miopenActivationMode_t(
        1,
    );
}
impl miopenActivationMode_t {
    ///< Tanh activation \f$ \beta * tanh( \alpha * x) \f$
    pub const miopenActivationTANH: miopenActivationMode_t = miopenActivationMode_t(2);
}
impl miopenActivationMode_t {
    ///< Rectified Linear Unit \f$ max(0, x) \f$
    pub const miopenActivationRELU: miopenActivationMode_t = miopenActivationMode_t(3);
}
impl miopenActivationMode_t {
    ///< \f$log(1 + e^x)\f$
    pub const miopenActivationSOFTRELU: miopenActivationMode_t = miopenActivationMode_t(
        4,
    );
}
impl miopenActivationMode_t {
    ///< Absolute value \f$abs(x)\f$
    pub const miopenActivationABS: miopenActivationMode_t = miopenActivationMode_t(5);
}
impl miopenActivationMode_t {
    ///< Scaled and shifted power \f$(\alpha + \beta * x)^{gamma}\f$
    pub const miopenActivationPOWER: miopenActivationMode_t = miopenActivationMode_t(6);
}
impl miopenActivationMode_t {
    pub const miopenActivationCLIPPEDRELU: miopenActivationMode_t = miopenActivationMode_t(
        7,
    );
}
impl miopenActivationMode_t {
    pub const miopenActivationLEAKYRELU: miopenActivationMode_t = miopenActivationMode_t(
        8,
    );
}
impl miopenActivationMode_t {
    pub const miopenActivationELU: miopenActivationMode_t = miopenActivationMode_t(9);
}
#[repr(transparent)]
/** @ingroup activation
 @enum miopenActivationMode_t
 Activation layer modes*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenActivationMode_t(pub ::core::ffi::c_uint);
impl miopenSoftmaxAlgorithm_t {
    ///< straightforward softmax
    pub const MIOPEN_SOFTMAX_FAST: miopenSoftmaxAlgorithm_t = miopenSoftmaxAlgorithm_t(
        0,
    );
}
impl miopenSoftmaxAlgorithm_t {
    ///< scaled softmax by maximum value in input domain
    pub const MIOPEN_SOFTMAX_ACCURATE: miopenSoftmaxAlgorithm_t = miopenSoftmaxAlgorithm_t(
        1,
    );
}
impl miopenSoftmaxAlgorithm_t {
    ///< log softmax
    pub const MIOPEN_SOFTMAX_LOG: miopenSoftmaxAlgorithm_t = miopenSoftmaxAlgorithm_t(2);
}
#[repr(transparent)]
/** @ingroup softmax
 @enum miopenSoftmaxAlgorithm_t
 Softmax implementation algorithms*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenSoftmaxAlgorithm_t(pub ::core::ffi::c_uint);
impl miopenSoftmaxMode_t {
    ///< compute per image (N) across C, H, W
    pub const MIOPEN_SOFTMAX_MODE_INSTANCE: miopenSoftmaxMode_t = miopenSoftmaxMode_t(0);
}
impl miopenSoftmaxMode_t {
    pub const MIOPEN_SOFTMAX_MODE_CHANNEL: miopenSoftmaxMode_t = miopenSoftmaxMode_t(1);
}
#[repr(transparent)]
/** @ingroup softmax
 @enum miopenSoftmaxMode_t
 Softmax modes*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenSoftmaxMode_t(pub ::core::ffi::c_uint);
impl miopenReduceTensorOp_t {
    ///< the operation is adding the values of the reduced elements
    pub const MIOPEN_REDUCE_TENSOR_ADD: miopenReduceTensorOp_t = miopenReduceTensorOp_t(
        0,
    );
}
impl miopenReduceTensorOp_t {
    pub const MIOPEN_REDUCE_TENSOR_MUL: miopenReduceTensorOp_t = miopenReduceTensorOp_t(
        1,
    );
}
impl miopenReduceTensorOp_t {
    pub const MIOPEN_REDUCE_TENSOR_MIN: miopenReduceTensorOp_t = miopenReduceTensorOp_t(
        2,
    );
}
impl miopenReduceTensorOp_t {
    pub const MIOPEN_REDUCE_TENSOR_MAX: miopenReduceTensorOp_t = miopenReduceTensorOp_t(
        3,
    );
}
impl miopenReduceTensorOp_t {
    pub const MIOPEN_REDUCE_TENSOR_AMAX: miopenReduceTensorOp_t = miopenReduceTensorOp_t(
        4,
    );
}
impl miopenReduceTensorOp_t {
    pub const MIOPEN_REDUCE_TENSOR_AVG: miopenReduceTensorOp_t = miopenReduceTensorOp_t(
        5,
    );
}
impl miopenReduceTensorOp_t {
    pub const MIOPEN_REDUCE_TENSOR_NORM1: miopenReduceTensorOp_t = miopenReduceTensorOp_t(
        6,
    );
}
impl miopenReduceTensorOp_t {
    /**< the operation is getting the square root of the sum of
squares of the reduced elements*/
    pub const MIOPEN_REDUCE_TENSOR_NORM2: miopenReduceTensorOp_t = miopenReduceTensorOp_t(
        7,
    );
}
#[repr(transparent)]
/** @ingroup TensorReduce
 @enum miopenReduceTensorOp_t
 Tensor Reduction operation types*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenReduceTensorOp_t(pub ::core::ffi::c_uint);
impl miopenNanPropagation_t {
    ///< does not propagate Nan number
    pub const MIOPEN_NOT_PROPAGATE_NAN: miopenNanPropagation_t = miopenNanPropagation_t(
        0,
    );
}
impl miopenNanPropagation_t {
    ///< propagate the Nan number by the Reduction operation
    pub const MIOPEN_PROPAGATE_NAN: miopenNanPropagation_t = miopenNanPropagation_t(1);
}
#[repr(transparent)]
/** @ingroup TensorReduce
 @enum miopenReduceTensorOp_t
 Nan numbers propagation modes*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenNanPropagation_t(pub ::core::ffi::c_uint);
impl miopenReduceTensorIndices_t {
    ///< Does not compuate indices
    pub const MIOPEN_REDUCE_TENSOR_NO_INDICES: miopenReduceTensorIndices_t = miopenReduceTensorIndices_t(
        0,
    );
}
impl miopenReduceTensorIndices_t {
    ///< Compute the relative, flatted indices
    pub const MIOPEN_REDUCE_TENSOR_FLATTENED_INDICES: miopenReduceTensorIndices_t = miopenReduceTensorIndices_t(
        1,
    );
}
#[repr(transparent)]
/** @ingroup TensorReduce
 @enum miopenReduceTensorIndices_t
 Reduction Indices computation modes*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenReduceTensorIndices_t(pub ::core::ffi::c_uint);
impl miopenIndicesType_t {
    ///< 32-bit unsigned integer indices
    pub const MIOPEN_32BIT_INDICES: miopenIndicesType_t = miopenIndicesType_t(0);
}
impl miopenIndicesType_t {
    ///< 64-bit unsigned integer indices
    pub const MIOPEN_64BIT_INDICES: miopenIndicesType_t = miopenIndicesType_t(1);
}
impl miopenIndicesType_t {
    ///< 16-bit unsigned integer indices
    pub const MIOPEN_16BIT_INDICES: miopenIndicesType_t = miopenIndicesType_t(2);
}
impl miopenIndicesType_t {
    ///< 8-bit unsigned integer indices
    pub const MIOPEN_8BIT_INDICES: miopenIndicesType_t = miopenIndicesType_t(3);
}
#[repr(transparent)]
/** @ingroup TensorReduce
 @enum miopenIndicesType_t
 Reduction Indices types*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenIndicesType_t(pub ::core::ffi::c_uint);
impl miopenConvolutionAttrib_t {
    pub const MIOPEN_CONVOLUTION_ATTRIB_FP16_ALT_IMPL: miopenConvolutionAttrib_t = miopenConvolutionAttrib_t(
        0,
    );
}
impl miopenConvolutionAttrib_t {
    pub const MIOPEN_CONVOLUTION_ATTRIB_DETERMINISTIC: miopenConvolutionAttrib_t = miopenConvolutionAttrib_t(
        1,
    );
}
#[repr(transparent)]
/** @ingroup convolutions
  @enum miopenConvolutionAttrib_t
 Attribute for convolution descriptor, used for alternating the convolution behavior*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenConvolutionAttrib_t(pub ::core::ffi::c_uint);
impl miopenConvolutionFindMode_t {
    pub const miopenConvolutionFindModeNormal: miopenConvolutionFindMode_t = miopenConvolutionFindMode_t(
        1,
    );
}
impl miopenConvolutionFindMode_t {
    pub const miopenConvolutionFindModeFast: miopenConvolutionFindMode_t = miopenConvolutionFindMode_t(
        2,
    );
}
impl miopenConvolutionFindMode_t {
    pub const miopenConvolutionFindModeHybrid: miopenConvolutionFindMode_t = miopenConvolutionFindMode_t(
        3,
    );
}
impl miopenConvolutionFindMode_t {
    pub const miopenConvolutionFindModeDynamicHybrid: miopenConvolutionFindMode_t = miopenConvolutionFindMode_t(
        5,
    );
}
impl miopenConvolutionFindMode_t {
    pub const miopenConvolutionFindModeDefault: miopenConvolutionFindMode_t = miopenConvolutionFindMode_t(
        5,
    );
}
#[repr(transparent)]
/** @ingroup convolutions
  @enum miopenConvolutionFindMode_t
 Findmode for convolution descriptor, used for changing the find behavior when calling
 miopenFindConvolutionForwardAlgorithm(), miopenFindConvolutionBackwardDataAlgorithm(), or
 miopenFindConvolutionBackwardWeightsAlgorithm().*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenConvolutionFindMode_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create a Tensor Descriptor

 API for creating an uninitialized tensor descriptor.
 @param tensorDesc Pointer to a tensor descriptor type (output)
 @return           miopenStatus_t*/
    pub fn miopenCreateTensorDescriptor(
        tensorDesc: *mut miopenTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set shape of 4D tensor

 Interface for setting 4-D tensor shape. MIOpen currently implements NCHW and NHWC layout.

 @param tensorDesc Tensor descriptor (input/output)
 @param dataType   MIOpen datatype (input)
 @param n          Mini-batch size (input)
 @param c          Number of channels (input)
 @param h          Data height dimension size (input)
 @param w          Data width dimension size (input)
 @return           miopenStatus_t*/
    pub fn miopenSet4dTensorDescriptor(
        tensorDesc: miopenTensorDescriptor_t,
        dataType: miopenDataType_t,
        n: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set shape of ND tensor with specific layout

 Interface for setting N-D packed tensor shape. This interface support NHWC, NCHW, NCHWc*, CHWNc*
 @param tensorDesc   Tensor descriptor (input/output)
 @param dataType     MIOpen datatype (input)
 @param tensorLayout Tensor layout (input)
 @param lens         Tensor dimensions (input)
 @param num_lens     Tensor dimension size (input)
 @return             miopenStatus_t*/
    pub fn miopenSetNdTensorDescriptorWithLayout(
        tensorDesc: miopenTensorDescriptor_t,
        dataType: miopenDataType_t,
        tensorLayout: miopenTensorLayout_t,
        lens: *const ::core::ffi::c_int,
        num_lens: ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set shape and stride of 4D tensor

 Interface for setting 4-D tensor shape and stride. It allows to create the non-packed tensor.
 A non-packed tensor refers to the tensor where the elements are not compressed or packed in any
 specific way. Each element in the tensor is stored individually, and there is no special
 compression applied to the storage.

 @param tensorDesc Tensor descriptor (input/output)
 @param dataType   MIOpen datatype (input)
 @param n          Mini-batch size (input)
 @param c          Number of channels (input)
 @param h          Data height dimension size (input)
 @param w          Data width dimension size (input)
 @param nStride    Mini-batch dimension stride (input)
 @param cStride    Channel dimension stride (input)
 @param hStride    Height dimension stride (input)
 @param wStride    Width dimension stride (input)
 @return           miopenStatus_t*/
    pub fn miopenSet4dTensorDescriptorEx(
        tensorDesc: miopenTensorDescriptor_t,
        dataType: miopenDataType_t,
        n: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        nStride: ::core::ffi::c_int,
        cStride: ::core::ffi::c_int,
        hStride: ::core::ffi::c_int,
        wStride: ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the details of the tensor descriptor

 Interface to query the 4-D tensor shape.

 @param tensorDesc Tensor descriptor (input)
 @param dataType   MIOpen datatype (output)
 @param n          Mini-batch size (output)
 @param c          Number of channels (output)
 @param h          Data height dimension size (output)
 @param w          Data width dimension size (output)
 @param nStride    Mini-batch dimension stride (output)
 @param cStride    Channel dimension stride (output)
 @param hStride    Height dimension stride (output)
 @param wStride    Width dimension stride (output)
 @return           miopenStatus_t*/
    pub fn miopenGet4dTensorDescriptor(
        tensorDesc: miopenTensorDescriptor_t,
        dataType: *mut miopenDataType_t,
        n: *mut ::core::ffi::c_int,
        c: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
        w: *mut ::core::ffi::c_int,
        nStride: *mut ::core::ffi::c_int,
        cStride: *mut ::core::ffi::c_int,
        hStride: *mut ::core::ffi::c_int,
        wStride: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set shape of N-dimensional tensor

 Interface for setting non-packed tensor shape.
 @param tensorDesc   Tensor descriptor (input/output)
 @param dataType     MIOpen datatype (input)
 @param nbDims       Number of dimensions in the dimsA array (input)
 @param dimsA        Array containing the size of dimensions (input)
 @param stridesA     Array containing the size of stride (input)
 @return             miopenStatus_t*/
    pub fn miopenSetTensorDescriptor(
        tensorDesc: miopenTensorDescriptor_t,
        dataType: miopenDataType_t,
        nbDims: ::core::ffi::c_int,
        dimsA: *const ::core::ffi::c_int,
        stridesA: *const ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set shape of N-dimensional tensor

 Interface for querying tensor size. MIOpen has support for 1, 2, 3, 4, 5 dimensional tensor of
 layout.
 @param tensorDesc   Tensor descriptor (input)
 @param size         number of elements in tensor described by the descriptor (output)
 @return             miopenStatus_t*/
    pub fn miopenGetTensorDescriptorSize(
        tensorDesc: miopenTensorDescriptor_t,
        size: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the details of the N-dimensional tensor descriptor.

 @param tensorDesc Tensor descriptor (input)
 @param dataType   MIOpen datatype (output)
 @param dimsA      Array containing the size of dimensions (output)
 @param stridesA   Array containing the size of stride (output)
 @return           miopenStatus_t*/
    pub fn miopenGetTensorDescriptor(
        tensorDesc: miopenTensorDescriptor_t,
        dataType: *mut miopenDataType_t,
        dimsA: *mut ::core::ffi::c_int,
        stridesA: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the tensor descriptor

 @param tensorDesc Tensor descriptor (input)
 @return           miopenStatus_t*/
    pub fn miopenDestroyTensorDescriptor(
        tensorDesc: miopenTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create a Tensor Descriptor for sequence data

 API for creating an uninitialized sequence data tensor descriptor.
 @param tensorDesc Pointer to a sequence data tensor descriptor type (output)
 @return           miopenStatus_t*/
    pub fn miopenCreateSeqTensorDescriptor(
        tensorDesc: *mut miopenSeqTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the sequence data tensor descriptor

 @param tensorDesc Tensor descriptor (input)
 @return           miopenStatus_t*/
    pub fn miopenDestroySeqTensorDescriptor(
        tensorDesc: miopenSeqTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute element-wise tensor operations

 This function implements: \f$ C = op ( alpha1[0] * A, alpha2[0] * B ) + beta[0] * C \f$

 For Forward Bias one can also use, miopenConvolutionForwardBias()

 @param handle     MIOpen handle (input)
 @param tensorOp   Operation from miopenTensorOp_t (input)
 @param alpha1     Tensor A's floating point scaling factor, allocated on the host (input)
 @param aDesc      Tensor descriptor for tensor A (input)
 @param A          Tensor A (input)
 @param alpha2     Tensor B's floating point scaling factor, allocated on the host (input)
 @param bDesc      Tensor descriptor for tensor B (input)
 @param B          Tensor B (input)
 @param beta       Tensor C's floating point scaling factor, allocated on the host (input)
 @param cDesc      Tensor descriptor for tensor C (input)
 @param C          Tensor C (input and output)
 @return           miopenStatus_t*/
    pub fn miopenOpTensor(
        handle: miopenHandle_t,
        tensorOp: miopenTensorOp_t,
        alpha1: *const ::core::ffi::c_void,
        aDesc: miopenTensorDescriptor_t,
        A: *const ::core::ffi::c_void,
        alpha2: *const ::core::ffi::c_void,
        bDesc: miopenTensorDescriptor_t,
        B: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        cDesc: miopenTensorDescriptor_t,
        C: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Fills a tensor with a single value.

 Supported datatypes are fp32, fp16, and bfp16

 @param handle     MIOpen handle (input)
 @param yDesc      Tensor descriptor for tensor y (input)
 @param y          Tensor y (input)
 @param alpha      Pointer to fill value (input)
 @return           miopenStatus_t*/
    pub fn miopenSetTensor(
        handle: miopenHandle_t,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        alpha: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Scales all elements in a tensor by a single value.

 Supported datatypes are fp32 and fp16

 @param handle     MIOpen handle (input)
 @param yDesc      Tensor descriptor for tensor y (input)
 @param y          Tensor y (input and output)
 @param alpha      Floating point scaling factor, allocated on the host (input)
 @return           miopenStatus_t*/
    pub fn miopenScaleTensor(
        handle: miopenHandle_t,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        alpha: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns number of bytes associated with tensor descriptor

 @param tensorDesc Tensor descriptor (input)
 @param numBytes   Number of bytes associated with tensor descriptor (output)
 @return           miopenStatus_t*/
    pub fn miopenGetTensorNumBytes(
        tensorDesc: miopenTensorDescriptor_t,
        numBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Copies one tensor to another tensor with a different layout/scale.

 This function implements:
 1. \f$ Y = alpha * X + beta * Y \f$ for fp32 and fp16 datatype
 2. Vectorize/de-vectorize along channel dimension C for int8 datatype

 Currently this is used for transforming from int8 to int8x4 vector datatypes

 @param handle     MIOpen handle (input)
 @param alpha      Floating point scaling factor, allocated on the host (input)
 @param xDesc      Source Tensor descriptor for tensor x (input)
 @param x          Source Tensor x (input)
 @param beta       Floating point scaling factor, allocated on the host (input)
 @param yDesc      Destination Tensor descriptor for tensor y (input)
 @param y          Destination Tensor y (output)
 @return           miopenStatus_t*/
    pub fn miopenTransformTensor(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a convolution layer descriptor

 @param convDesc   Convolution layer descriptor
 @return           miopenStatus_t*/
    pub fn miopenCreateConvolutionDescriptor(
        convDesc: *mut miopenConvolutionDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a 2-D convolution layer descriptor

 For group/depthwise convolution dilation height and width, only a dilation value of 1 is
 supported.

 @param convDesc   Convolution layer descriptor (output)
 @param c_mode     Convolutional mode (input)
 @param pad_h      Height input data padding (input)
 @param pad_w      Width input data padding (input)
 @param stride_h   Stride for the height of input data (input)
 @param stride_w   Stride for the width of input data (input)
 @param dilation_h Dilation height (input)
 @param dilation_w Dilation width (input)
 @return           miopenStatus_t*/
    pub fn miopenInitConvolutionDescriptor(
        convDesc: miopenConvolutionDescriptor_t,
        c_mode: miopenConvolutionMode_t,
        pad_h: ::core::ffi::c_int,
        pad_w: ::core::ffi::c_int,
        stride_h: ::core::ffi::c_int,
        stride_w: ::core::ffi::c_int,
        dilation_h: ::core::ffi::c_int,
        dilation_w: ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a N-dimensional convolution layer descriptor

 @param convDesc      Convolution layer descriptor (output)
 @param spatialDim    Convolutional spatial dimension (input)
 @param padA          Array of input data padding (input)
 @param strideA       Array of convolution stride (input)
 @param dilationA     Array of convolution dilation (input)
 @param c_mode        Convolutional mode (input)
 @return              miopenStatus_t*/
    pub fn miopenInitConvolutionNdDescriptor(
        convDesc: miopenConvolutionDescriptor_t,
        spatialDim: ::core::ffi::c_int,
        padA: *const ::core::ffi::c_int,
        strideA: *const ::core::ffi::c_int,
        dilationA: *const ::core::ffi::c_int,
        c_mode: miopenConvolutionMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retrieves the spatial dimension of a convolution layer descriptor

 @param convDesc              Convolution layer descriptor (input)
 @param spatialDim            Spatial dimension of convolution descriptor (output)
 @return                      miopenStatus_t*/
    pub fn miopenGetConvolutionSpatialDim(
        convDesc: miopenConvolutionDescriptor_t,
        spatialDim: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retrieves a 2-D convolution layer descriptor's details

 For group/depthwise convolution dilation height and width, only a dilation value of 1 is
 supported.

 @param convDesc   Convolution layer descriptor (input)
 @param c_mode     Convolutional mode (output)
 @param pad_h      Height input data padding (output)
 @param pad_w      Width input data padding (output)
 @param stride_h   Stride for the height of input data (output)
 @param stride_w   Stride for the width of input data (output)
 @param dilation_h Dilation height (output)
 @param dilation_w Dilation width (output)
 @return           miopenStatus_t*/
    pub fn miopenGetConvolutionDescriptor(
        convDesc: miopenConvolutionDescriptor_t,
        c_mode: *mut miopenConvolutionMode_t,
        pad_h: *mut ::core::ffi::c_int,
        pad_w: *mut ::core::ffi::c_int,
        stride_h: *mut ::core::ffi::c_int,
        stride_w: *mut ::core::ffi::c_int,
        dilation_h: *mut ::core::ffi::c_int,
        dilation_w: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retrieves a N-dimensional convolution layer descriptor's details

 @param convDesc               Convolution layer descriptor (input)
 @param requestedSpatialDim    Expected convolution spatial dimension (intput)
 @param spatialDim             Convolutional spatial dimension (output)
 @param padA                   Array of input data padding (output)
 @param strideA                Array of convolution stride (output)
 @param dilationA              Array of convolution dilation (output)
 @param c_mode                 Convolutional mode (output)
 @return                       miopenStatus_t*/
    pub fn miopenGetConvolutionNdDescriptor(
        convDesc: miopenConvolutionDescriptor_t,
        requestedSpatialDim: ::core::ffi::c_int,
        spatialDim: *mut ::core::ffi::c_int,
        padA: *mut ::core::ffi::c_int,
        strideA: *mut ::core::ffi::c_int,
        dilationA: *mut ::core::ffi::c_int,
        c_mode: *mut miopenConvolutionMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the number of groups to be used in Group/Depthwise convolution

 @param convDesc   Convolution layer descriptor (input)
 @param groupCount Pointer to number of groups in group/depthwise convolution (output)
 @return           miopenStatus_t*/
    pub fn miopenGetConvolutionGroupCount(
        convDesc: miopenConvolutionDescriptor_t,
        groupCount: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the number of groups to be used in Group/Depthwise convolution

 Must be called before all computational APIs of group/depthwise convolution, it is preferable to
 call miopenInitConvolutionDescriptor() first, then miopenSetConvolutionGroupCount() to fully
 initialize group convolutions. Both Convolution Mode and Transpose Convolution Mode support
 group/depthwise convolution. To run depthwise convolution, set groupCount value equal to number
 of channels.

 @param convDesc   Convolution layer descriptor (output)
 @param groupCount      number of groups, in depthwise conv using filter_number/channel_multiplier
 (input)
 @return           miopenStatus_t*/
    pub fn miopenSetConvolutionGroupCount(
        convDesc: miopenConvolutionDescriptor_t,
        groupCount: ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the output padding to be used in 2-D Transpose convolution

 This function is optional for initialization of Transpose convolution. If applicable, it must be
 called before all computational APIs of Transpose convolution. It is preferable to call
 miopenInitConvolutionDescriptor() first, then miopenSetTransposeConvOutputPadding() to fully
 initialize transpose convolutions.

 @param convDesc   Convolution layer descriptor (output)
 @param adj_h      output padding for the height of output data (input)
 @param adj_w      output padding for the width of output data (input)
 @return           miopenStatus_t*/
    pub fn miopenSetTransposeConvOutputPadding(
        convDesc: miopenConvolutionDescriptor_t,
        adj_h: ::core::ffi::c_int,
        adj_w: ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the output padding to be used in N-dimensional Transpose convolution

 This function is optional for initialization of Transpose convolution. If applicable, it must be
 called before all computational APIs of Transpose convolution. It is preferable to call
 miopenInitConvolutionNdDescriptor() first, then miopenSetTransposeConvNdOutputPadding() to fully
 initialize transpose convolutions. Currently, 2-D and 3-D convolutions are supported.

 @param convDesc      Convolution layer descriptor (output)
 @param spatialDim    Convolutional spatial dimension (input)
 @param adjA          array of output padding for output data (input)
 @return              miopenStatus_t*/
    pub fn miopenSetTransposeConvNdOutputPadding(
        convDesc: miopenConvolutionDescriptor_t,
        spatialDim: ::core::ffi::c_int,
        adjA: *const ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the shape of a resulting 4-D tensor from a 2-D convolution

 This function returns the dimensions of the resulting 4D tensor of a 2D
 convolution, given the convolution descriptor, the input tensor descriptor
 and the filter descriptor. This function can help to setup the output tensor
 and allocate the proper amount of memory prior to launch the actual
 convolution.

 @param convDesc          Convolution layer descriptor (input)
 @param inputTensorDesc   Input data tensor descriptor (input)
 @param filterDesc        Weight descriptor (input)
 @param n                 Mini-batch size (output)
 @param c                 Number of channels (output)
 @param h                 Data height dimension size (output)
 @param w                 Data width dimension size (output)
 @return                  miopenStatus_t*/
    pub fn miopenGetConvolutionForwardOutputDim(
        convDesc: miopenConvolutionDescriptor_t,
        inputTensorDesc: miopenTensorDescriptor_t,
        filterDesc: miopenTensorDescriptor_t,
        n: *mut ::core::ffi::c_int,
        c: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
        w: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the shape of a resulting N-dimensional tensor from a (N-2)-dimensional convolution

 This function returns the dimensions of the resulting N-dimensional tensor of a (N-2)-dimensional
 convolution, given the convolution descriptor, the input tensor descriptor
 and the filter descriptor. It is used to setup the output tensor descriptor prior to executing
 the convolution layer.

 @param convDesc          Convolution layer descriptor (input)
 @param inputTensorDesc   Input data tensor descriptor (input)
 @param filterDesc        Weight descriptor (input)
 @param nDim              Pointer to Output data tensor dimension (output)
 @param outputTensorDimA  Array of Output data tensor length (output)*/
    pub fn miopenGetConvolutionNdForwardOutputDim(
        convDesc: miopenConvolutionDescriptor_t,
        inputTensorDesc: miopenTensorDescriptor_t,
        filterDesc: miopenTensorDescriptor_t,
        nDim: *mut ::core::ffi::c_int,
        outputTensorDimA: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the tensor descriptor object

 @param convDesc Convolution tensor descriptor type (input)
 @return           miopenStatus_t*/
    pub fn miopenDestroyConvolutionDescriptor(
        convDesc: miopenConvolutionDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the attribute of the convolution descriptor

 @param convDesc          Convolution layer descriptor (input)
 @param attr              Attribute of this convolution to set (input)
 @param value             Value of this attribute (input)*/
    pub fn miopenSetConvolutionAttribute(
        convDesc: miopenConvolutionDescriptor_t,
        attr: miopenConvolutionAttrib_t,
        value: ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the attribute of the convolution descriptor

 @param convDesc          Convolution layer descriptor (input)
 @param attr              Attribute of this convolution to get (input)
 @param value             Value of this attribute (output)*/
    pub fn miopenGetConvolutionAttribute(
        convDesc: miopenConvolutionDescriptor_t,
        attr: miopenConvolutionAttrib_t,
        value: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the Find Mode attribute in the convolution descriptor.

 The subsequent calls of miopenFindConvolutionForwardAlgorithm(),
 miopenFindConvolutionBackwardDataAlgorithm(), or miopenFindConvolutionBackwardWeightsAlgorithm()
 invoked with convDesc, will follow the findMode set by this call.

 Note that the default Find Mode is overriden by the MIOPEN_FIND_MODE environment variable,
 if it is set. If unset, the default is as specified by miopenConvolutionFindModeDefault.

 @param convDesc   Convolution layer descriptor (input)
 @param findMode   Find Mode of convDesc (input)
 @return           miopenStatus_t*/
    pub fn miopenSetConvolutionFindMode(
        convDesc: miopenConvolutionDescriptor_t,
        findMode: miopenConvolutionFindMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Reads the Find Mode attribute from the convolution descriptor.

 @param convDesc   Convolution layer descriptor (input)
 @param findMode   Find Mode of convDesc (output)
 @return           miopenStatus_t*/
    pub fn miopenGetConvolutionFindMode(
        convDesc: miopenConvolutionDescriptor_t,
        findMode: *mut miopenConvolutionFindMode_t,
    ) -> miopenStatus_t;
}
impl miopenConvFwdAlgorithm_t {
    ///< GEMM variant
    pub const miopenConvolutionFwdAlgoGEMM: miopenConvFwdAlgorithm_t = miopenConvFwdAlgorithm_t(
        0,
    );
}
impl miopenConvFwdAlgorithm_t {
    ///< Direct convolutions
    pub const miopenConvolutionFwdAlgoDirect: miopenConvFwdAlgorithm_t = miopenConvFwdAlgorithm_t(
        1,
    );
}
impl miopenConvFwdAlgorithm_t {
    ///< Fast Fourier Transform indirect convolutions
    pub const miopenConvolutionFwdAlgoFFT: miopenConvFwdAlgorithm_t = miopenConvFwdAlgorithm_t(
        2,
    );
}
impl miopenConvFwdAlgorithm_t {
    ///< Winograd indirect convolutions
    pub const miopenConvolutionFwdAlgoWinograd: miopenConvFwdAlgorithm_t = miopenConvFwdAlgorithm_t(
        3,
    );
}
impl miopenConvFwdAlgorithm_t {
    ///< Implicit GEMM convolutions
    pub const miopenConvolutionFwdAlgoImplicitGEMM: miopenConvFwdAlgorithm_t = miopenConvFwdAlgorithm_t(
        5,
    );
}
#[repr(transparent)]
/** @enum miopenConvFwdAlgorithm_t
 Convolutional algorithm mode for forward propagation. MIOpen use cross-correlation for its
 convolution implementation.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenConvFwdAlgorithm_t(pub ::core::ffi::c_uint);
impl miopenConvBwdWeightsAlgorithm_t {
    ///< GEMM variant
    pub const miopenConvolutionBwdWeightsAlgoGEMM: miopenConvBwdWeightsAlgorithm_t = miopenConvBwdWeightsAlgorithm_t(
        0,
    );
}
impl miopenConvBwdWeightsAlgorithm_t {
    ///< Direct convolution algorithm
    pub const miopenConvolutionBwdWeightsAlgoDirect: miopenConvBwdWeightsAlgorithm_t = miopenConvBwdWeightsAlgorithm_t(
        1,
    );
}
impl miopenConvBwdWeightsAlgorithm_t {
    ///< Winograd convolutions
    pub const miopenConvolutionBwdWeightsAlgoWinograd: miopenConvBwdWeightsAlgorithm_t = miopenConvBwdWeightsAlgorithm_t(
        3,
    );
}
impl miopenConvBwdWeightsAlgorithm_t {
    ///< Implicit GEMM convolutions
    pub const miopenConvolutionBwdWeightsAlgoImplicitGEMM: miopenConvBwdWeightsAlgorithm_t = miopenConvBwdWeightsAlgorithm_t(
        5,
    );
}
#[repr(transparent)]
/** @enum miopenConvBwdWeightsAlgorithm_t
 Convolutional algorithm mode for back propagation on weights.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenConvBwdWeightsAlgorithm_t(pub ::core::ffi::c_uint);
impl miopenConvBwdDataAlgorithm_t {
    ///< GEMM variant
    pub const miopenConvolutionBwdDataAlgoGEMM: miopenConvBwdDataAlgorithm_t = miopenConvBwdDataAlgorithm_t(
        0,
    );
}
impl miopenConvBwdDataAlgorithm_t {
    ///< Direct convolutions
    pub const miopenConvolutionBwdDataAlgoDirect: miopenConvBwdDataAlgorithm_t = miopenConvBwdDataAlgorithm_t(
        1,
    );
}
impl miopenConvBwdDataAlgorithm_t {
    ///< Fast Fourier Transform indirect convolutions
    pub const miopenConvolutionBwdDataAlgoFFT: miopenConvBwdDataAlgorithm_t = miopenConvBwdDataAlgorithm_t(
        2,
    );
}
impl miopenConvBwdDataAlgorithm_t {
    ///< Winograd indirect convolutions
    pub const miopenConvolutionBwdDataAlgoWinograd: miopenConvBwdDataAlgorithm_t = miopenConvBwdDataAlgorithm_t(
        3,
    );
}
impl miopenConvBwdDataAlgorithm_t {
    pub const miopenTransposeBwdDataAlgoGEMM: miopenConvBwdDataAlgorithm_t = miopenConvBwdDataAlgorithm_t(
        4,
    );
}
impl miopenConvBwdDataAlgorithm_t {
    ///< Implicit GEMM convolutions
    pub const miopenConvolutionBwdDataAlgoImplicitGEMM: miopenConvBwdDataAlgorithm_t = miopenConvBwdDataAlgorithm_t(
        5,
    );
}
#[repr(transparent)]
/** @enum miopenConvBwdDataAlgorithm_t
 Convolutional algorithm mode for back propagation on data.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenConvBwdDataAlgorithm_t(pub ::core::ffi::c_uint);
impl miopenConvAlgorithm_t {
    ///< GEMM variant
    pub const miopenConvolutionAlgoGEMM: miopenConvAlgorithm_t = miopenConvAlgorithm_t(
        0,
    );
}
impl miopenConvAlgorithm_t {
    ///< Direct convolutions
    pub const miopenConvolutionAlgoDirect: miopenConvAlgorithm_t = miopenConvAlgorithm_t(
        1,
    );
}
impl miopenConvAlgorithm_t {
    ///< Fast Fourier Transform indirect convolutions
    pub const miopenConvolutionAlgoFFT: miopenConvAlgorithm_t = miopenConvAlgorithm_t(2);
}
impl miopenConvAlgorithm_t {
    ///< Winograd indirect convolutions
    pub const miopenConvolutionAlgoWinograd: miopenConvAlgorithm_t = miopenConvAlgorithm_t(
        3,
    );
}
impl miopenConvAlgorithm_t {
    ///< Implicit GEMM convolutions
    pub const miopenConvolutionAlgoImplicitGEMM: miopenConvAlgorithm_t = miopenConvAlgorithm_t(
        5,
    );
}
#[repr(transparent)]
/** @enum miopenConvAlgorithm_t
 Top-level convolutional algorithm mode*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenConvAlgorithm_t(pub ::core::ffi::c_uint);
/** @brief Perf struct for forward, backward filter, or backward data algorithms

 Contains the union to hold the selected convolution algorithm for forward, or backwards layers,
 and also contains the time it took to run the algorithm and the workspace required to run the
 algorithm. The workspace in this structure can be used when executing the convolution layer.*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct miopenConvAlgoPerf_t {
    pub __bindgen_anon_1: miopenConvAlgoPerf_t__bindgen_ty_1,
    ///< Time to exectued the selected algorithm represented in the union
    pub time: f32,
    ///< Workspace required to run the selected algorithm represented in the union
    pub memory: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union miopenConvAlgoPerf_t__bindgen_ty_1 {
    ///< Forward convolution algorithm enum selection
    pub fwd_algo: miopenConvFwdAlgorithm_t,
    /**< Back propagation on weights
convolution algorithm enum selection*/
    pub bwd_weights_algo: miopenConvBwdWeightsAlgorithm_t,
    ///< Back propagation on data convolution algorithm enum selection
    pub bwd_data_algo: miopenConvBwdDataAlgorithm_t,
}
/** @brief Performance struct for forward, backward filter, or backward data algorithms in
 immediate mode

 Contains a 64-bit integer identifying the solution and the algorithm for the solution,
 as well as the runtime, workspace size and a boolean flag indicating whether the returned
 solution is a heuristic or resulting from an actual run
*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct miopenConvSolution_t {
    /**< Represents the approximate time required to execute this solution on the GPU,
in milliseconds. This value may either be based on an acutal kernel run or an
estimate based on a heuristic.*/
    pub time: f32,
    /**< Workspace required to run the selected algorithm represented in the
union*/
    pub workspace_size: usize,
    ///< Identifier for the returned solution
    pub solution_id: u64,
    ///< The algorithm used to compute the solution
    pub algorithm: miopenConvAlgorithm_t,
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the maximum number of solutions applicable for the given input/output and weights
  tensor descriptor for Convolution in the Forward direction.

 This call returns the maximum number of applicable solutions for a forward convolution problem.
 The \c solutionCount returned may be used to allocate the memory required for the
 \c miopenConvAlgoPerf_t which is required by miopenConvolutionGetSolution API calls.

 @param handle         MIOpen handle (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param xDesc          Tensor descriptor for input data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param solutionCount  Pointer to memory to return number of applicable solutions (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionForwardGetSolutionCount(
        handle: miopenHandle_t,
        wDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        yDesc: miopenTensorDescriptor_t,
        solutionCount: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the applicable solutions for a convolution configuration described by
  input, output and convolution descriptors.

  The returned solutions array is sorted in the order of decreasing performance. The returned
 solutions
 might be based
  on heuristics and for more consistent performance results the user the advised to run the Find
 step.
  The maximum length of the solutions array may be queried using
 miopenConvolutionForwardGetSolutionCount

 @param handle         MIOpen handle (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param xDesc          Tensor descriptor for input data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param maxSolutionCount The size of the solutions array passed in below (input)
 @param solutionCount The size of the solutions array returned (output)
 @param solutions      A pointer to an array of type miopenConvSolution_t allocated by the user,
                      filled in by MIOpen with applicable solutions. (output)
 @return               miopenStatus_t
*/
    pub fn miopenConvolutionForwardGetSolution(
        handle: miopenHandle_t,
        wDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        yDesc: miopenTensorDescriptor_t,
        maxSolutionCount: usize,
        solutionCount: *mut usize,
        solutions: *mut miopenConvSolution_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the workspace size required for a particular solution id.

 This is an optional call for users who may have serialized the solution id and just need the
 workspace
 size for it. The same information is returned by the miopenConvolutionForwardGetSolution as part
 of the
 miopenConvSolution_t struct.

 @param handle         MIOpen handle (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param xDesc          Tensor descriptor for input data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param solution_id      ID of the solution for which workspace size is required (input)
 @param workSpaceSize  The size of the workspace (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionForwardGetSolutionWorkspaceSize(
        handle: miopenHandle_t,
        wDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        yDesc: miopenTensorDescriptor_t,
        solution_id: u64,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Compiles the solution provided by the user, this solution may be acquired by the
 miopenConvolutionForwardGetSolution API call above.
   Compiling the solution ensures that the first API call to miopenConvolutionForwardImmediate
 does
 not cause a compile.

   This is an optional step and may be skipped if a slow first miopenConvolutionForwardImmediate
 invocation is acceptable.

 @param handle         MIOpen handle (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param xDesc          Tensor descriptor for input data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param solution_id      ID of the solution to be compiled, as chosen by the user
 @return               miopenStatus_t*/
    pub fn miopenConvolutionForwardCompileSolution(
        handle: miopenHandle_t,
        wDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        yDesc: miopenTensorDescriptor_t,
        solution_id: u64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Executes the Forward convolution operation based on the provided solution ID.

 Supported datatypes are fp32, fp16, bfp16, and int8

 @param handle         MIOpen handle (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param w              Weights tensor w (input)
 @param xDesc          Tensor descriptor for input data tensor x (input)
 @param x              Data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param y              Data tensor y (output)
 @param workSpace      Workspace tensor (input)
 @param workSpaceSize  Size of the memory in bytes pointed to by workSpace above
 @param solution_id      ID of the solution to be compiled, as chosen by the user
 @return               miopenStatus_t*/
    pub fn miopenConvolutionForwardImmediate(
        handle: miopenHandle_t,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
        solution_id: u64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the maximum number of solutions applicable for the given input/output and weights
  tensor descriptor for backward Convolution w-r-t Data.

  This call returns the maximum number of applicable solutions for a the convolution problem, the
 number
  returned may be used to allocate the memory required for the miopenConvAlgoPert2_t which is
 required
  by miopenConvolutionBackwardDataGetSolution API calls.

 @param handle         MIOpen handle (input)
 @param dyDesc         Tensor descriptor for data input tensor dy (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param convDesc       Convolution layer descriptor (input)
 @param dxDesc         Tensor descriptor for output data tensor dx (input)
 @param solutionCount  Pointer to memory to return number of applicable solutions (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardDataGetSolutionCount(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dxDesc: miopenTensorDescriptor_t,
        solutionCount: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the applicable solutions for a backward convolution w-r-t data as described by
  input, output and convolution descriptors.

  The returned solutions array is sorted in the order of decreasing performance. The returned
 solutions
  ns
 might be based
  on heuristics and for more consistent performance results the user the advised to run the Find
 step.
  The maximum length of the solutions array may be queried using
 miopenConvolutionBackwardDataGetSolutionCount

 @param handle           MIOpen handle (input)
 @param dyDesc           Tensor descriptor for data input tensor dy (input)
 @param wDesc            Tensor descriptor for weight tensor w (input)
 @param convDesc         Convolution layer descriptor (input)
 @param dxDesc           Tensor descriptor for output data tensor dx (input)
 @param maxSolutionCount The size of the solutions array passed in below (input)
 @param solutionCount    The size of the solutions array returned (output)
 @param solutions        A pointer to an array of type miopenConvSolution_t allocated by the user,
                         filled in by MIOpen with applicable solutions. (output)
 @return                 miopenStatus_t
*/
    pub fn miopenConvolutionBackwardDataGetSolution(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dxDesc: miopenTensorDescriptor_t,
        maxSolutionCount: usize,
        solutionCount: *mut usize,
        solutions: *mut miopenConvSolution_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the workspace size required for a particular solution id.

 This is an optional call for users who may have serialized the solution id and just need the
 workspace
 size for it. The same information is returned by the miopenConvolutionBackwardDataGetSolution as
 part of the
 miopenConvSolution_t struct.

 @param handle         MIOpen handle (input)
 @param dyDesc           Tensor descriptor for data input tensor dy (input)
 @param wDesc            Tensor descriptor for weight tensor w (input)
 @param convDesc         Convolution layer descriptor (input)
 @param dxDesc           Tensor descriptor for output data tensor dx (input)
 @param solution_id      ID of the solution for which workspace size is required (input)
 @param workSpaceSize  The size of the workspace (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardDataGetSolutionWorkspaceSize(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dxDesc: miopenTensorDescriptor_t,
        solution_id: u64,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Compiles the solution provided by the user, this solution may be acquired by the
 miopenConvolutionBackwardDataGetSolution API call above.
   Compiling the solution ensures that the first API call to
 miopenConvolutionBackwardDataImmediate
 does not cause a compile.

   This is an optional step and may be skipped if a slow first
 miopenConvolutionBackwardDataImmediate
 invocation is acceptable.

 @param handle         MIOpen handle (input)
 @param dyDesc         Tensor descriptor for data input tensor dy (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param convDesc       Convolution layer descriptor (input)
 @param dxDesc         Tensor descriptor for output data tensor dx (input)
 @param solution_id      ID of the solution to be compiled, as chosen by the user
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardDataCompileSolution(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dxDesc: miopenTensorDescriptor_t,
        solution_id: u64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Executes the Backward convolution w-r-t data  operation based on the provided solution
 ID.


 @param handle         MIOpen handle (input)
 @param dyDesc         Tensor descriptor for data input tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param w              Weights tensor w (input)
 @param convDesc       Convolution layer descriptor (input)
 @param dxDesc         Tensor descriptor for output data tensor dx (input)
 @param dx             Data delta tensor dx (output)
 @param workSpace      Workspace tensor (input)
 @param workSpaceSize  Size in bytes of the workspace memory pointed to by workSpace
 @param solution_id      ID of the solution to be compiled, as chosen by the user
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardDataImmediate(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
        solution_id: u64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the maximum number of solutions applicable for the given input/output and weights
  tensor descriptor for backward Convolution w-r-t Weights.

  This call returns the maximum number of applicable solutions for a the convolution problem, the
 number
  returned may be used to allocate the memory required for the miopenConvAlgoPert2_t which is
 required
  by miopenConvolutionBackwardWeightsGetSolution API calls.

 @param handle         MIOpen handle (input)
 @param dyDesc         Tensor descriptor for data tensor dy (input)
 @param xDesc          Tensor descriptor for data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param dwDesc         Tensor descriptor for weight tensor dw (input)
 @param solutionCount  Pointer to memory to return number of applicable solutions (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardWeightsGetSolutionCount(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dwDesc: miopenTensorDescriptor_t,
        solutionCount: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the applicable solutions for a backward convolution w-r-t weights as described by
  input, output and convolution descriptors.

  The returned solutions array is sorted in the order of decreasing performance. The returned
 solutions
 might be based
  on heuristics and for more consistent performance results the user the advised to run the Find
 step.
  The maximum length of the solutions array may be queried using
 miopenConvolutionBackwardWeightsGetSolutionCount

 @param handle           MIOpen handle (input)
 @param dyDesc           Tensor descriptor for data tensor dy (input)
 @param xDesc            Tensor descriptor for data tensor x (input)
 @param convDesc         Convolution layer descriptor (input)
 @param dwDesc           Tensor descriptor for weight tensor dw (input)
 @param maxSolutionCount The size of the solutions array passed in below (input)
 @param solutionCount    The size of the solutions array returned (output)
 @param solutions        A pointer to an array of type miopenConvSolution_t allocated by the user,
                         filled in by MIOpen with applicable solutions. (output)
 @return                 miopenStatus_t
*/
    pub fn miopenConvolutionBackwardWeightsGetSolution(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dwDesc: miopenTensorDescriptor_t,
        maxSolutionCount: usize,
        solutionCount: *mut usize,
        solutions: *mut miopenConvSolution_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the workspace size required for a particular solution id.

 This is an optional call for users who may have serialized the solution id and just need the
 workspace
 size for it. The same information is returned by the miopenConvolutionBackwardWeightsGetSolution
 as part of the
 miopenConvSolution_t struct.

 @param handle         MIOpen handle (input)
 @param dyDesc         Tensor descriptor for data tensor dy (input)
 @param xDesc          Tensor descriptor for data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param dwDesc         Tensor descriptor for weight tensor dw (input)
 @param solution_id      ID of the solution for which workspace size is required (input)
 @param workSpaceSize  The size of the workspace (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardWeightsGetSolutionWorkspaceSize(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dwDesc: miopenTensorDescriptor_t,
        solution_id: u64,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Compiles the solution provided by the user, this solution may be acquired by the
 miopenConvolutionBackwardWeightsGetSolution API call above.
   Compiling the solution ensures that the first API call to
 miopenConvolutionBackwardWeightsImmediate
 does not cause a compile.

   This is an optional step and may be skipped if a slow first
 miopenConvolutionBackwardWeightsImmediate invocation is acceptable.

 @param handle         MIOpen handle (input)
 @param dyDesc         Tensor descriptor for data tensor dy (input)
 @param xDesc          Tensor descriptor for data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param dwDesc         Tensor descriptor for weight tensor dw (input)
 @param solution_id      ID of the solution to be compiled, as chosen by the user
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardWeightsCompileSolution(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dwDesc: miopenTensorDescriptor_t,
        solution_id: u64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Executes the Backward convolution w-r-t weights  operation based on the provided solution
 ID.


 @param handle         MIOpen handle (input)
 @param dyDesc         Tensor descriptor for data tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param xDesc          Tensor descriptor for data tensor x (input)
 @param x              Data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param dwDesc         Tensor descriptor for weight tensor dw (input)
 @param dw             Weights delta tensor dw (output)
 @param workSpace      Workspace tensor (input)
 @param workSpaceSize  Size in bytes of the memory passed in, pointed to by workSpace pointer
 above
 @param solution_id      ID of the solution to be compiled, as chosen by the user
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardWeightsImmediate(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        dwDesc: miopenTensorDescriptor_t,
        dw: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
        solution_id: u64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the workspace size required for a forward convolution algorithm.

 For given tensor and convolution descriptors, this function calculates and returns the minimum
 size of the workspace that must be provided to miopenFindConvolutionForwardAlgorithm() in order
 for the latter to find the best candidate from the available forward data convolution algorithms.

 WARNING: Providing smaller workspace may result in the selection of a slow convolution
 algorithm, and therefore affect library performance.

 It should be assumed that the required workspace size is different for each convolution
 configuration. Therefore, typically this function should be called at least once for each
 convolution configuration used.

 Since the convolution configuration is determined by tensor and convolution descriptors, the user
 should ensure that all descriptors contain complete information. For example, if Group/Depthwise
 convolution mode is used, then miopenSetConvolutionGroupCount() should be called before running
 this, and so on.

 @param handle         MIOpen handle (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param xDesc          Tensor descriptor for input data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param workSpaceSize  Pointer to memory to return size in bytes (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionForwardGetWorkSpaceSize(
        handle: miopenHandle_t,
        wDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        yDesc: miopenTensorDescriptor_t,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Search and run the forward convolutional algorithms and return a list of kernel times.

 This function attempts all MIOpen forward convolution algorithms based on
 the input configuration, and outputs performance metrics to a
 user-allocated array of type miopenConvAlgoPerf_t. These metrics are written
 in a sorted fashion where the first element has the lowest compute time.
 Users can chose the top-most algorithm if they only care about the fastest
 algorithm.

 This function is mandatory before using miopenConvolutionForward(). In order
 to execute this function, miopenConvolutionForwardGetWorkSpaceSize() must be
 run to determine the required memory for this search.

 * If exhaustiveSearch == 0, MIOpen will look for the first kernel with a configuration match. If
 a configuration match is not found, a default configuration will be returned.

 * If exhaustiveSearch == 1, MIOpen will look for the best kernel for the provided configuration.
 If a match is not found, an exhaustive search is performed by running individual algorithms.

 If using Group/Depthwise convolution mode, call miopenSetConvolutionGroupCount() before running
 this.

 @param handle             MIOpen handle (input)
 @param xDesc              Tensor descriptor for data input tensor x (input)
 @param x                  Data tensor x (input)
 @param wDesc              Tensor descriptor for weight tensor w (input)
 @param w                  Weights tensor w (input)
 @param convDesc           Convolution layer descriptor (input)
 @param yDesc              Tensor descriptor for output data tensor y (input)
 @param y                  Data tensor y (output)
 @param requestAlgoCount   Number of algorithms to return kernel times (input)
 @param returnedAlgoCount  Pointer to number of algorithms returned (output)
 @param perfResults        Pointer to union of best algorithm for forward and backwards (input)
 @param workSpace          Pointer to workspace buffer (input).
 @param workSpaceSize      Size in bytes of the workspace buffer (input).
                           The buffer must be allocated on the device by the caller.
                           The size of the buffer should be determined by calling
                           miopenConvolutionForwardGetWorkSpaceSize(), see its
                           documentation for details.
 @param exhaustiveSearch   A boolean to toggle a full search of all algorithms
                           and configurations (input)
 @return                   miopenStatus_t*/
    pub fn miopenFindConvolutionForwardAlgorithm(
        handle: miopenHandle_t,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        requestAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut miopenConvAlgoPerf_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
        exhaustiveSearch: bool,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a forward convolution layer

 Runs the forward convolution layer based on the selected algorithm. The function
 miopenFindConvolutionForwardAlgorithm() must have been executed previously to
 determine the required memory needed for the workspace and the best convolutional algorithm.
 The scaling parameter alpha (float) and shift parameter beta (float) are only supported for
 alpha = 1 and beta = 0 in 2D. In 3D, these parameters can take other values.

 The forward convolution is designed to accommodate both packed and non-packed tensor strides for
 multiple data types and dimensions across various platforms. This flexibility ensures optimal
 performance in handling diverse computational scenarios. To configure tensor parameters,
 including strides, users can utilize the APIs miopenSetTensorDescriptor() and
 miopenGetTensorDescriptor(). These APIs empower developers to seamlessly set and retrieve tensor
 information, facilitating a more intuitive and efficient workflow. The tensor strides are
 non-packed by default.

 If using Group/Depthwise convolution mode, call miopenSetConvolutionGroupCount() before running
 this.

 @param handle         MIOpen handle (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param xDesc          Tensor descriptor for data input tensor x (input)
 @param x              Data tensor x (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param w              Weights tensor w (inputs)
 @param convDesc       Convolution layer descriptor (inputs)
 @param algo           Algorithm selected (inputs)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param y              Data tensor y (output)
 @param workSpace      Pointer to workspace required (input)
 @param workSpaceSize  Size in bytes of the memory determined by the find step (input)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionForward(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        algo: miopenConvFwdAlgorithm_t,
        beta: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Calculate element-wise scale and shift of a tensor via a bias tensor

  This function applies an element-wise bias to a data tensor from an input bias tensor.
  The scaling parameter alpha (float) and shift parameter beta (float) are only supported for
  alpha = 1 and beta = 0.

 @param handle         MIOpen handle (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param bDesc          Tensor descriptor for bias tensor b (input)
 @param b              Bias tensor b (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for data tensor y (input)
 @param y              Data tensor y (input and output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionForwardBias(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        bDesc: miopenTensorDescriptor_t,
        b: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the workspace size required for a backward data convolution algorithm.

 For given tensor and convolution descriptors, this function calculates and returns the minimum
 size of the workspace that must be provided to miopenFindConvolutionBackwardDataAlgorithm() in
 order for the latter to find the best candidate from the available backward data convolution
 algorithms.

 WARNING: Providing smaller workspace may result in the selection of a slow convolution
 algorithm, and therefore affect library performance.

 It should be assumed that the required workspace size is different for each convolution
 configuration. Therefore, typically this function should be called at least once for each
 convolution configuration used.

 Since the convolution configuration is determined by tensor and convolution descriptors, the user
 should ensure that all descriptors contain complete information. For example, if Group/Depthwise
 convolution mode is used, then miopenSetConvolutionGroupCount() should be called before running
 this, and so on.

 @param handle         MIOpen handle (input)
 @param dyDesc         Tensor descriptor for data input tensor dy (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param convDesc       Convolution layer descriptor (input)
 @param dxDesc         Tensor descriptor for output data tensor dx (input)
 @param workSpaceSize  Size in bytes of the memory required (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardDataGetWorkSpaceSize(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dxDesc: miopenTensorDescriptor_t,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Search and run the backwards data convolution algorithms and return a list of kernel
 times.

 This function attempts all MIOpen backward data convolution algorithms, and outputs the
 performance metrics to a user-allocated array of type miopenConvAlgoPerf_t.
 These metrics are written in sorted fashion where the first element has the lowest compute time.
 This function is mandatory before using backwards convolutions. Users can chose the top-most
 algorithm if they only care about the fastest algorithm.

 This function is mandatory before using miopenConvolutionBackwardData(). In order to
 execute this function, miopenConvolutionBackwardsDataGetWorkSpaceSize() must be run to determine
 the required memory for this search.

 * If exhaustiveSearch == 0, MIOpen will look for the first kernel with a configuration match. If
 a configuration match is not found, a default configuration will be returned.

 * If exhaustiveSearch == 1, MIOpen will look for the best kernel for the provided configuration.
 If a match is not found, an exhaustive search is performed by running individual algorithms.

 If using Group/Depthwise convolution mode, call miopenSetConvolutionGroupCount() before running
 this.

 @param handle             MIOpen handle (input)
 @param dyDesc             Tensor descriptor for data input tensor dy (input)
 @param dy                 Data delta tensor dy (input)
 @param wDesc              Tensor descriptor for weight tensor w (input)
 @param w                  Weights tensor w (input)
 @param convDesc           Convolution layer descriptor (input)
 @param dxDesc             Tensor descriptor for output data tensor dx (input)
 @param dx                 Data delta tensor dx (input)
 @param requestAlgoCount   Number of algorithms to return kernel times (input)
 @param returnedAlgoCount  Pointer to number of algorithms returned (output)
 @param perfResults        Pointer to union of best algorithm for forward and backwards (output)
 @param workSpace          Pointer to workspace buffer (input).
 @param workSpaceSize      Size in bytes of the workspace buffer (input).
                           The buffer must be allocated on the device by the caller.
                           The size of the buffer should be determined by calling
                           miopenConvolutionBackwardDataGetWorkSpaceSize(), see its
                           documentation for details.
 @param exhaustiveSearch   A boolean to toggle a full search of all algorithms
                           and configurations (input)
 @return                   miopenStatus_t*/
    pub fn miopenFindConvolutionBackwardDataAlgorithm(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        requestAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut miopenConvAlgoPerf_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
        exhaustiveSearch: bool,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a backward data convolution layer

 Runs the backward data convolution layer based on the selected algorithm. The function
 miopenFindConvolutionBackwardDataAlgorithm() must have been executed previously to
 determine the required memory needed for the workspace and the best convolutional
 algorithm.

 The backward data convolution is designed to accommodate both packed and non-packed tensor
 strides for multiple data types and dimensions across various platforms. This flexibility ensures
 optimal performance in handling diverse computational scenarios. To configure tensor parameters,
 including strides, users can utilize the APIs miopenSetTensorDescriptor() and
 miopenGetTensorDescriptor(). These APIs empower developers to seamlessly set and retrieve tensor
 information, facilitating a more intuitive and efficient workflow. The tensor strides are
 non-packed by default.

 If using Group/Depthwise convolution mode, call miopenSetConvolutionGroupCount() before running
 this.

 @param handle         MIOpen handle (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param dyDesc         Tensor descriptor for data input tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param wDesc          Tensor descriptor for weight tensor w (input)
 @param w              Weights tensor w (input)
 @param convDesc       Convolution layer descriptor (input)
 @param algo           Algorithm selected (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param dxDesc         Tensor descriptor for output data tensor dx (input)
 @param dx             Data delta tensor dx (output)
 @param workSpace      Pointer to workspace required for the search (input)
 @param workSpaceSize  Size in bytes of the memory needed for find (input)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardData(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        algo: miopenConvBwdDataAlgorithm_t,
        beta: *const ::core::ffi::c_void,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the GPU memory required for the backward weights convolution algorithm.

 For given tensor and convolution descriptors, this function calculates and returns the minimum
 size of the workspace that must be provided to miopenFindConvolutionBackwardWeightsAlgorithm() in
 order for the latter to find the best candidate from the available backward weights convolution
 algorithms.

 WARNING: Providing smaller workspace may result in the selection of a slow convolution
 algorithm, and therefore affect library performance.

 It should be assumed that the required workspace size is different for each convolution
 configuration. Therefore, typically this function should be called at least once for each
 convolution configuration used.

 Since the convolution configuration is determined by tensor and convolution descriptors, the user
 should ensure that all descriptors contain complete information. For example, if Group/Depthwise
 convolution mode is used, then miopenSetConvolutionGroupCount() should be called before running
 this, and so on.

 @param handle         MIOpen handle (input)
 @param dyDesc         Tensor descriptor for data input tensor dy (input)
 @param xDesc          Tensor descriptor for data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param dwDesc         Tensor descriptor for output weights tensor dw (input)
 @param workSpaceSize  Size in bytes of the memory required (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardWeightsGetWorkSpaceSize(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        dwDesc: miopenTensorDescriptor_t,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Search and run the backwards weights convolutional algorithms and return a list of kernel
 times.

 This function attempts all MIOpen backward weights convolution algorithms, and outputs
 the performance metrics to a user-allocated array of type miopenConvAlgoPerf_t. These metrics are
 written in sorted fashion where the first element has the lowest compute time.
 This function is mandatory before using backwards weight convolutions. Users can chose the
 top-most algorithm if they only care about the fastest algorithm.

 This function is mandatory before using miopenConvolutionBackwardWeights(). In order to
 execute this function, miopenConvolutionBackwardsWeightsGetWorkSpaceSize() must be run to
 determine the required memory for this search.

 * If exhaustiveSearch == 0, MIOpen will look for the first kernel with a configuration match. If
 a configuration match is not found, a default configuration will be returned.

 * If exhaustiveSearch == 1, MIOpen will look for the best kernel for the provided configuration.
 If a match is not found, an exhaustive search is performed by running individual algorithms.

 If using Group/Depthwise convolution mode, call miopenSetConvolutionGroupCount() before running
 this.

 @param handle             MIOpen handle (input)
 @param dyDesc             Tensor descriptor for data input tensor dy (input)
 @param dy                 Data delta tensor dy (input)
 @param xDesc              Tensor descriptor for output data tensor x (input)
 @param x                  Data delta tensor dx (input)
 @param convDesc           Convolution layer descriptor (input)
 @param dwDesc             Tensor descriptor for weight tensor dw (input)
 @param dw                 Weights delta tensor dw (input)
 @param requestAlgoCount   Number of algorithms to return kernel times (input)
 @param returnedAlgoCount  Pointer to number of algorithms returned (output)
 @param perfResults        Pointer to union of best algorithm for forward and backwards (output)
 @param workSpace          Pointer to workspace buffer (input).
 @param workSpaceSize      Size in bytes of the workspace buffer (input).
                           The buffer must be allocated on the device by the caller.
                           The size of the buffer should be determined by calling
                           miopenConvolutionBackwardWeightsGetWorkSpaceSize(), see its
                           documentation for details.
 @param exhaustiveSearch   A boolean to toggle a full search of all algorithms
                           and configurations (input)
 @return                   miopenStatus_t*/
    pub fn miopenFindConvolutionBackwardWeightsAlgorithm(
        handle: miopenHandle_t,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        dwDesc: miopenTensorDescriptor_t,
        dw: *mut ::core::ffi::c_void,
        requestAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut miopenConvAlgoPerf_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
        exhaustiveSearch: bool,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a backward weights convolution layer

 Runs the backward weights convolution layer based on the selected algorithm. The function
 miopenFindConvolutionBackwardWeightsAlgorithm() must have
 been executed previously to determine the required memory needed for the workspace and the
 best convolutional algorithm.

 The backward weights convolution is designed to accommodate both packed and non-packed tensor
 strides for multiple data types and dimensions across various platforms. This flexibility ensures
 optimal performance in handling diverse computational scenarios. To configure tensor parameters,
 including strides, users can utilize the APIs miopenSetTensorDescriptor() and
 miopenGetTensorDescriptor(). These APIs empower developers to seamlessly set and retrieve tensor
 information, facilitating a more intuitive and efficient workflow. The tensor strides are
 non-packed by default.

 If using Group/Depthwise convolution mode, call miopenSetConvolutionGroupCount() before running
 this.

 @param handle         MIOpen handle (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param dyDesc         Tensor descriptor for data tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param xDesc          Tensor descriptor for data tensor x (input)
 @param x              Data tensor x (input)
 @param convDesc       Convolution layer descriptor (input)
 @param algo           Algorithm selected (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param dwDesc         Tensor descriptor for weight tensor dw (input)
 @param dw             Weights delta tensor dw (output)
 @param workSpace      Pointer to workspace required for the search (input)
 @param workSpaceSize  Size in bytes of the memory needed for find (input)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardWeights(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        algo: miopenConvBwdWeightsAlgorithm_t,
        beta: *const ::core::ffi::c_void,
        dwDesc: miopenTensorDescriptor_t,
        dw: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Calculates the gradient with respect to the bias.

 Compute the convolution backwards gradient with respect to the bias tensor.
 The scaling parameter alpha (float) and shift parameter beta (float) are only supported for
 alpha = 1 and beta = 0.

 @param handle         MIOpen handle (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param dyDesc         Tensor descriptor for data input tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param dbDesc         Tensor descriptor for input bias tensor db (input)
 @param db             Bias delta tensor db (output)
 @return               miopenStatus_t*/
    pub fn miopenConvolutionBackwardBias(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dbDesc: miopenTensorDescriptor_t,
        db: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a pooling layer descriptor

 @param poolDesc   Pointer to a pooling layer descriptor (output)
 @return           miopenStatus_t*/
    pub fn miopenCreatePoolingDescriptor(
        poolDesc: *mut miopenPoolingDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set index data type for pooling layer. The default indexing type is uint8_t.
 Users can set the index type to any of the miopenIndexType_t sizes; 8, 16, 32, or 64 bit
 unsigned integers.

 @param poolDesc     Pointer to a pooling layer descriptor (input)
 @param index_type   Index type (input)
 @return             miopenStatus_t*/
    pub fn miopenSetPoolingIndexType(
        poolDesc: miopenPoolingDescriptor_t,
        index_type: miopenIndexType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the index data type for pooling layer. The index type to any of the
 miopenIndexType_t sizes; 8, 16, 32, or 64 bit unsigned integers.

 @param poolDesc     Pointer to a pooling layer descriptor (input)
 @param index_type   Index type (output)
 @return             miopenStatus_t*/
    pub fn miopenGetPoolingIndexType(
        poolDesc: miopenPoolingDescriptor_t,
        index_type: *mut miopenIndexType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set workspace index mode for pooling layer. The default mode is
 miopenPoolingWorkSpaceIndexMask.

 @param poolDesc         Pointer to a pooling layer descriptor (input/output)
 @param workspace_index  Workspace index mode (input)
 @return                 miopenStatus_t*/
    pub fn miopenSetPoolingWorkSpaceIndexMode(
        poolDesc: miopenPoolingDescriptor_t,
        workspace_index: miopenPoolingWorkspaceIndexMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get workspace index mode for pooling layer.

 @param poolDesc         Pointer to a pooling layer descriptor (input)
 @param workspace_index  Workspace index mode (output)
 @return                 miopenStatus_t*/
    pub fn miopenGetPoolingWorkSpaceIndexMode(
        poolDesc: miopenPoolingDescriptor_t,
        workspace_index: *mut miopenPoolingWorkspaceIndexMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a 2-D pooling layer descriptor details.

 Sets the window shape, padding, and stride for a previously created 2-D pooling descriptor.

 @param poolDesc       Pointer to a pooling layer descriptor (output)
 @param mode           Pooling mode enum (input)
 @param windowHeight   Input window height dimension (input)
 @param windowWidth    Input window width dimension (input)
 @param pad_h          Number of elements to pad height (input)
 @param pad_w          Number of elements to pad width (input)
 @param stride_h       Vertical stride (input)
 @param stride_w       Horizontal stride (input)
 @return               miopenStatus_t*/
    pub fn miopenSet2dPoolingDescriptor(
        poolDesc: miopenPoolingDescriptor_t,
        mode: miopenPoolingMode_t,
        windowHeight: ::core::ffi::c_int,
        windowWidth: ::core::ffi::c_int,
        pad_h: ::core::ffi::c_int,
        pad_w: ::core::ffi::c_int,
        stride_h: ::core::ffi::c_int,
        stride_w: ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a 2-D pooling layer descriptor details

 Gets the window shape, padding, and stride for a previously created 2-D pooling descriptor.

 @param poolDesc       Pointer to a pooling layer descriptor (input)
 @param mode           Pooling mode enum (output)
 @param windowHeight   Input window height dimension (output)
 @param windowWidth    Input window width dimension (output)
 @param pad_h          Number of elements to pad height (output)
 @param pad_w          Number of elements to pad width (output)
 @param stride_h       Vertical stride (output)
 @param stride_w       Horizontal stride (output)
 @return               miopenStatus_t*/
    pub fn miopenGet2dPoolingDescriptor(
        poolDesc: miopenPoolingDescriptor_t,
        mode: *mut miopenPoolingMode_t,
        windowHeight: *mut ::core::ffi::c_int,
        windowWidth: *mut ::core::ffi::c_int,
        pad_h: *mut ::core::ffi::c_int,
        pad_w: *mut ::core::ffi::c_int,
        stride_h: *mut ::core::ffi::c_int,
        stride_w: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the shape of the output tensor for 2-D pooling

 Retrieve the tensor dimensions for the forward 2-D pooling. This call is required for
 the forward if the output dimensions are different than the input tensor
 dimensions.

 @param poolDesc   Pointer to a pooling layer descriptor (input)
 @param tensorDesc Input tensor descriptor (input)
 @param n	         Mini-batch dim (output)
 @param c	         Number of channels (output)
 @param h          Heights of input map (output)
 @param w          Width of input map (output)
 @return           miopenStatus_t*/
    pub fn miopenGetPoolingForwardOutputDim(
        poolDesc: miopenPoolingDescriptor_t,
        tensorDesc: miopenTensorDescriptor_t,
        n: *mut ::core::ffi::c_int,
        c: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
        w: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set details of a N-D pooling layer descriptor

 Set the window shape, padding, and stride for a previously created N-D pooling descriptor.

 @param poolDesc     Pointer to a pooling layer descriptor (input/output)
 @param mode         Pooling mode enum (input)
 @param nbDims       Dimension of the pooling (input)
 @param windowDimA   Array of input window dimensions with length equal to or larger than
 dimsRequested (input)
 @param padA         Array of number of elements to padding with length equal to or larger than
 dimsRequested (input)
 @param stridesA     Array of stride parameter with length equal to or larger than dimsRequested
 (input)
 @return               miopenStatus_t*/
    pub fn miopenSetNdPoolingDescriptor(
        poolDesc: miopenPoolingDescriptor_t,
        mode: miopenPoolingMode_t,
        nbDims: ::core::ffi::c_int,
        windowDimA: *const ::core::ffi::c_int,
        padA: *const ::core::ffi::c_int,
        stridesA: *const ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get details of a N-D pooling layer descriptor

 Get the window shape, padding, and stride for a previously created N-D pooling descriptor.

 @param poolDesc         Pointer to a pooling layer descriptor (input)
 @param nbDimsRequested  Dimension of the expected pooling descriptor (input)
 @param mode             Pooling mode enum (output)
 @param nbDims           Actual dimension of the pooling descriptor (output)
 @param windowDimA       Array of input window dimensions with length equal to or larger than
 dimsRequested (output)
 @param padA             Array of number of elements to padding with length equal to or larger
 than dimsRequested (output)
 @param stridesA         Array of stride parameter with length equal to or larger than
 dimsRequested (output)
 @return                 miopenStatus_t*/
    pub fn miopenGetNdPoolingDescriptor(
        poolDesc: miopenPoolingDescriptor_t,
        nbDimsRequested: ::core::ffi::c_int,
        mode: *mut miopenPoolingMode_t,
        nbDims: *mut ::core::ffi::c_int,
        windowDimA: *mut ::core::ffi::c_int,
        padA: *mut ::core::ffi::c_int,
        stridesA: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the shape of the output tensor for N-D pooling

 Retrieve the tensor dimensions for the forward N-D pooling. This call is required for
 the forward if the output dimensions are different than the input tensor
 dimensions.

 @param poolDesc      Pointer to a pooling layer descriptor (input)
 @param tensorDesc    Input tensor descriptor (input)
 @param dims          Dimension of the pooling (input)
 @param tensorDimArr  Array of tensor dimension (output)
 @return           miopenStatus_t*/
    pub fn miopenGetPoolingNdForwardOutputDim(
        poolDesc: miopenPoolingDescriptor_t,
        tensorDesc: miopenTensorDescriptor_t,
        dims: ::core::ffi::c_int,
        tensorDimArr: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the amount of GPU memory required for pooling

 Retrieves the amount of workspace in bytes require for pooling. This call is required to
 determine the amount of GPU memory needed for the backwards pooling algorithms. For max-
 pooling, an assumption is that index data type is uint8_t, therefore the returned
 workspace size will be based on this assumption even if the user sets the index type with
 miopenSetPoolingIndexType().

 @param yDesc          Descriptor for pooling layer (input)
 @param workSpaceSize  Pointer to workSpaceSize (output)
 @return               miopenStatus_t*/
    pub fn miopenPoolingGetWorkSpaceSize(
        yDesc: miopenTensorDescriptor_t,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the amount of GPU memory required for pooling

 Retrieves the amount of workspace in bytes require for pooling. This call is required to
 determine the amount of GPU memory needed for the backwards pooling algorithms. For max-
 pooling, there is no assumption on index data type. As the user can set the index datatype
 size using miopenSetPoolingIndexType().

 @param poolDesc       Pointer to a pooling layer descriptor (input)
 @param yDesc          Descriptor for pooling layer (input)
 @param workSpaceSize  Pointer to workSpaceSize (output)
 @return               miopenStatus_t*/
    pub fn miopenPoolingGetWorkSpaceSizeV2(
        poolDesc: miopenPoolingDescriptor_t,
        yDesc: miopenTensorDescriptor_t,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a forward pooling layer

 Runs forward pooling. miopenGetPoolingForwardOutputDim() should be called before
 miopenPoolingForward().
 If the parameter do_backward == 0, then set workSpace = nullptr and workSpaceSize = 0. However,
 for back-propagation do_backwards must be set to 1 in miopenPoolingForward().

 @param handle         MIOpen handle (input)
 @param poolDesc       Descriptor for pooling layer (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param xDesc          Tensor descriptor for data input tensor x (input)
 @param x              Data tensor x (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param y              Data tensor y (output)
 @param do_backward    Boolean to toggle save data in workspace for backwards pass (input)
 @param workSpace      Pointer user allocated memory (input)
 @param workSpaceSize  Size in bytes of the memory needed (input)
 @return               miopenStatus_t*/
    pub fn miopenPoolingForward(
        handle: miopenHandle_t,
        poolDesc: miopenPoolingDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        do_backward: bool,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a backward pooling layer

 Runs backward pooling. miopenPoolingGetWorkSpaceSize() must be called before
 miopenPoolingBackward() to determine the amount of workSpace to be allocated.

 @param handle         MIOpen handle (input)
 @param poolDesc       Descriptor for pooling layer (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param y              Data tensor y (input)
 @param dyDesc         Tensor descriptor for data input tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param xDesc          Tensor descriptor for output data tensor x (input)
 @param x              Data tensor x (output)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param dxDesc         Tensor descriptor for tensor dx (input)
 @param dx             Weights delta tensor dx (output)
 @param workSpace      Pointer to user allocated workspace (input)
 @return               miopenStatus_t*/
    pub fn miopenPoolingBackward(
        handle: miopenHandle_t,
        poolDesc: miopenPoolingDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the pooling descriptor object

 @param poolDesc Pooling tensor descriptor type (input)
 @return           miopenStatus_t*/
    pub fn miopenDestroyPoolingDescriptor(
        poolDesc: miopenPoolingDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @addtogroup LRN\n\n  @{\n/\n/*! @brief Creates a local response normalization (LRN) layer descriptor\n\n @param lrnDesc    Pointer to a local response normalization layer descriptor type\n @return           miopenStatus_t"]
    pub fn miopenCreateLRNDescriptor(
        lrnDesc: *mut miopenLRNDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a LRN layer descriptor details

 Sets all of the descriptor details for the LRN layer. The number of window elements lrnN is
 a diameter and always odd.

 @param lrnDesc      Pointer to a LRN layer descriptor (output)
 @param mode         LRN mode enum (input)
 @param lrnN         Number of normalization window elements (input)
 @param lrnAlpha     Scaling factor (input)
 @param lrnBeta      Shift factor (input)
 @param lrnK         K factor (input)
 @return             miopenStatus_t*/
    pub fn miopenSetLRNDescriptor(
        lrnDesc: miopenLRNDescriptor_t,
        mode: miopenLRNMode_t,
        lrnN: ::core::ffi::c_uint,
        lrnAlpha: f64,
        lrnBeta: f64,
        lrnK: f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a LRN layer descriptor details

 Retrieve the LRN descriptor details.

 @param lrnDesc      Pointer to a LRN layer descriptor (input)
 @param mode         LRN mode enum (output)
 @param lrnN         Number of normalization window elements (output)
 @param lrnAlpha     Scaling factor (output)
 @param lrnBeta      Shift factor (output)
 @param lrnK         K factor (output)
 @return             miopenStatus_t*/
    pub fn miopenGetLRNDescriptor(
        lrnDesc: miopenLRNDescriptor_t,
        mode: *mut miopenLRNMode_t,
        lrnN: *mut ::core::ffi::c_uint,
        lrnAlpha: *mut f64,
        lrnBeta: *mut f64,
        lrnK: *mut f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Determine the workspace requirements.

 This function determines the GPU memory allocation required to execute the LRN layer based on the
 LRN descriptor.

 @param yDesc           Pointer to a LRN layer descriptor (input)
 @param workSpaceSize   Output variable for workspace size (output)
 @return                miopenStatus_t*/
    pub fn miopenLRNGetWorkSpaceSize(
        yDesc: miopenTensorDescriptor_t,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a LRN forward layer

 Runs the forward layer normalization in the forward direction. If do_backward == 0, then
 set workSpace = nullptr and workSpaceSize = 0. However, if the user wishes to execute backwards,
 then they must set do_backwards = 1 in miopenLRNForward().

 @param handle         MIOpen handle (input)
 @param lrnDesc        Descriptor for LRN layer (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param xDesc          Tensor descriptor for data input tensor x (input)
 @param x              Data tensor x (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param y              Data tensor y (output)
 @param do_backward    Boolean to toggle save data in workspace for backwards pass (input)
 @param workSpace      Pointer user allocated memory (input)
 @return               miopenStatus_t*/
    pub fn miopenLRNForward(
        handle: miopenHandle_t,
        lrnDesc: miopenLRNDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        do_backward: bool,
        workSpace: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a LRN backward layer

 @param handle         MIOpen handle (input)
 @param lrnDesc        Descriptor for LRN layer (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for data input tensor y (input)
 @param y              Data tensor y (input)
 @param dyDesc         Tensor descriptor for data input tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param xDesc          Tensor descriptor for input data tensor x (input)
 @param x              Data tensor x (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param dxDesc         Tensor descriptor for output data tensor dx(input)
 @param dx             Data delta tensor x (output)
 @param workSpace      Pointer user allocated memory (input)
 @return               miopenStatus_t*/
    pub fn miopenLRNBackward(
        handle: miopenHandle_t,
        lrnDesc: miopenLRNDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        workSpace: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the LRN descriptor object

 @param lrnDesc   LRN tensor descriptor type (input)
 @return          miopenStatus_t*/
    pub fn miopenDestroyLRNDescriptor(lrnDesc: miopenLRNDescriptor_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Derive tensor for gamma and beta from input tensor descriptor

 This function takes the input tensor descriptor and outputs a derived tensor for the
 normalization scale (gamma) and shift (beta) tensors.

 For an input tensor NCHW and spatial mode, the output derived tensor is 1C11, while for
 per-activation the derived tensor is 1CHW.

 For an input tensor NCDHW and spatial mode, the output derived tensor is 1C111, while for
 per-activation the derived tensor is 1CDHW.

 @param derivedBnDesc   Output derived tensor descriptor (output)
 @param xDesc           Input tensor descriptor (input)
 @param bn_mode         Batch Normalization mode (input)
 @return                miopenStatus_t*/
    pub fn miopenDeriveBNTensorDescriptor(
        derivedBnDesc: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        bn_mode: miopenBatchNormMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute forward training layer for batch normalization

 Batch normalization pass for forward training pass.
 Takes in batch normalization mode bn_mode and input tensor x, output tensor y, bnBias and bnScale
 with their descriptor.

 If either resultSaveMean, or resultSaveInvVariance are null pointers then the values for the mean
 and inverse variance will not be used.

 Likewise, if either resultRunningMean, or resultRunningVariance are null pointers then the values
 for the running mean and variance will not be saved.
 Running averages and variances are scaled using an exponential averaging factor: \f[
 \mu_{old} = \mu_{new}*factor + \mu_{old}*(1-factor)
 \f]
 where \f[
 factor=1/(1+iteration)
 \f]

 @param handle                    MIOpen handle (input)
 @param bn_mode                   Batch normalization mode (input)
 @param alpha                     Floating point scaling factor, allocated on the host (input)
 @param beta                      Floating point shift factor, allocated on the host (input)
 @param xDesc                     Tensor descriptor for data input tensor x (input)
 @param x                         Data tensor x (input)
 @param yDesc                     Tensor descriptor for output data tensor y (input)
 @param y                         Data tensor y (output)
 @param bnScaleBiasMeanVarDesc    Tensor descriptor for BN scaling, shifting, saved variance and
 mean (input)
 @param bnScale                   Batch norm scaling, gamma, tensor (input)
 @param bnBias                    Batch norm bias, beta, tensor (input)
 @param expAvgFactor              Exponential averaging factor (input)
 @param resultRunningMean         Running average saved for inference (output)
 @param resultRunningVariance     Running variance saved for inference (output)
 @param epsilon                   Value to stablize inverse variance calculation (input)
 @param resultSaveMean            Saved mini-batch mean for backwards pass (output)
 @param resultSaveInvVariance     Saved mini-batch inverse variance for backwards pass (output)
 @return                          miopenStatus_t*/
    pub fn miopenBatchNormalizationForwardTraining(
        handle: miopenHandle_t,
        bn_mode: miopenBatchNormMode_t,
        alpha: *mut ::core::ffi::c_void,
        beta: *mut ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        bnScaleBiasMeanVarDesc: miopenTensorDescriptor_t,
        bnScale: *mut ::core::ffi::c_void,
        bnBias: *mut ::core::ffi::c_void,
        expAvgFactor: f64,
        resultRunningMean: *mut ::core::ffi::c_void,
        resultRunningVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::core::ffi::c_void,
        resultSaveInvVariance: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute forward training layer for batch normalization

 Batch normalization pass for forward training pass.
 Takes in batch normalization mode bn_mode and input tensor x, output tensor y, bnBias and bnScale
 with their descriptor.

 If either resultSaveMean, or resultSaveInvVariance are null pointers then the values for the mean
 and inverse variance will not be used.

 Likewise, if either resultRunningMean, or resultRunningVariance are null pointers then the values
 for the running mean and variance will not be saved.
 Running averages and variances are scaled using an exponential averaging factor: \f[
 \mu_{old} = \mu_{new}*factor + \mu_{old}*(1-factor)
 \f]
 where \f[
 factor=1/(1+iteration)
 \f]

 @param handle                    MIOpen handle (input)
 @param bn_mode                   Batch normalization mode (input)
 @param alpha                     Floating point scaling factor, allocated on the host (input)
 @param beta                      Floating point shift factor, allocated on the host (input)
 @param xDesc                     Tensor descriptor for data input tensor x (input)
 @param x                         Data tensor x (input)
 @param yDesc                     Tensor descriptor for output data tensor y (input)
 @param y                         Data tensor y (output)
 @param ScaleDesc                 Tensor descriptor for BN scaling
 @param biasVarDesc               Tensor descriptor for BN bias
 @param savedMeanDesc             Tensor descriptor for BN saved Mean
 @param savedVarDesc              Tensor descriptor for BN saved Variance
 @param bnScale                   Batch norm scaling, gamma, tensor (input)
 @param bnBias                    Batch norm bias, beta, tensor (input)
 @param expAvgFactor              Exponential averaging factor (input)
 @param resultRunningMean         Running average saved for inference (output)
 @param resultRunningVariance     Running variance saved for inference (output)
 @param epsilon                   Value to stablize inverse variance calculation (input)
 @param resultSaveMean            Saved mini-batch mean for backwards pass (output)
 @param resultSaveInvVariance     Saved mini-batch inverse variance for backwards pass (output)
 @return                          miopenStatus_t*/
    pub fn miopenBatchNormalizationForwardTraining_V2(
        handle: miopenHandle_t,
        bn_mode: miopenBatchNormMode_t,
        alpha: *mut ::core::ffi::c_void,
        beta: *mut ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        scaleDesc: miopenTensorDescriptor_t,
        biasVarDesc: miopenTensorDescriptor_t,
        savedMeanDesc: miopenTensorDescriptor_t,
        savedVarDesc: miopenTensorDescriptor_t,
        bnScale: *mut ::core::ffi::c_void,
        bnBias: *mut ::core::ffi::c_void,
        expAvgFactor: f64,
        resultRunningMean: *mut ::core::ffi::c_void,
        resultRunningVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::core::ffi::c_void,
        resultSaveInvVariance: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute forward inference layer for batch normalization

 Batch normalization pass for forward inference pass.
 Takes in batch normalization mode bn_mode and input tensor x, output tensor y, bnBias and bnScale
 with their descriptor.

 If either estimatedMean, or estimatedVariance are null pointers then the values for the mean and
 variance will be calculated from input data and this calculated mean and variance will be used
 to update input values.
 If variance is zero and epsilon is also zero, this function outputs NAN values.  Input espilon
 value should always be non zero positive value.

 @param handle                    MIOpen handle (input)
 @param bn_mode                   Batch normalization mode (input)
 @param alpha                     Floating point scaling factor, allocated on the host (input)
 @param beta                      Floating point shift factor, allocated on the host (input)
 @param xDesc                     Tensor descriptor for data input tensor x (input)
 @param x                         Data tensor x (input)
 @param yDesc                     Tensor descriptor for output data tensor y (input)
 @param y                         Data tensor y (output)
 @param bnScaleBiasMeanVarDesc    Tensor descriptor for BN scaling, shifting, saved variance and
 mean (input)
 @param bnScale                   Batch norm scaling, gamma, tensor (input)
 @param bnBias                    Batch norm bias, beta, tensor (input)
 @param estimatedMean             Running average saved during forward training (input)
 @param estimatedVariance         Running variance saved during forward training (input)
 @param epsilon                   Value to stabilize inverse variance calculation (input)
 @return                          miopenStatus_t*/
    pub fn miopenBatchNormalizationForwardInference(
        handle: miopenHandle_t,
        bn_mode: miopenBatchNormMode_t,
        alpha: *mut ::core::ffi::c_void,
        beta: *mut ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        bnScaleBiasMeanVarDesc: miopenTensorDescriptor_t,
        bnScale: *mut ::core::ffi::c_void,
        bnBias: *mut ::core::ffi::c_void,
        estimatedMean: *mut ::core::ffi::c_void,
        estimatedVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute forward inference layer for batch normalization

 Batch normalization pass for forward inference pass.
 Takes in batch normalization mode bn_mode and input tensor x, output tensor y, bnBias and bnScale
 with their descriptor.

 If either estimatedMean, or estimatedVariance are null pointers then the values for the mean and
 variance will be calculated from input data and this calculated mean and variance will be used
 to update input values.
 If variance is zero and epsilon is also zero, this function outputs NAN values.  Input espilon
 value should always be non zero positive value.

 @param handle                    MIOpen handle (input)
 @param bn_mode                   Batch normalization mode (input)
 @param alpha                     Floating point scaling factor, allocated on the host (input)
 @param beta                      Floating point shift factor, allocated on the host (input)
 @param xDesc                     Tensor descriptor for data input tensor x (input)
 @param x                         Data tensor x (input)
 @param yDesc                     Tensor descriptor for output data tensor y (input)
 @param y                         Data tensor y (output)
 @param ScaleDesc                 Tensor descriptor for BN scaling
 @param biasVarDesc               Tensor descriptor for BN bias
 @param estMeanDesc               Tensor descriptor for BN estimated Mean
 @param estVarianceDesc           Tensor descriptor for BN estimated Variance
 @param bnScale                   Batch norm scaling, gamma, tensor (input)
 @param bnBias                    Batch norm bias, beta, tensor (input)
 @param estimatedMean             Running average saved during forward training (input)
 @param estimatedVariance         Running variance saved during forward training (input)
 @param epsilon                   Value to stabilize inverse variance calculation (input)
 @return                          miopenStatus_t*/
    pub fn miopenBatchNormalizationForwardInference_V2(
        handle: miopenHandle_t,
        bn_mode: miopenBatchNormMode_t,
        alpha: *mut ::core::ffi::c_void,
        beta: *mut ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        scaleDesc: miopenTensorDescriptor_t,
        biasDesc: miopenTensorDescriptor_t,
        estMeanDesc: miopenTensorDescriptor_t,
        estVarianceDesc: miopenTensorDescriptor_t,
        bnScale: *mut ::core::ffi::c_void,
        bnBias: *mut ::core::ffi::c_void,
        estimatedMean: *mut ::core::ffi::c_void,
        estimatedVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute backwards propagation layer for batch normalization

 Batch normalization pass for backwards propagation training pass.
 The method for backwards propagation batch normalization.

 Takes in batch normalization mode bn_mode and input tensor data x, input activation tensor dy,
 output tensor dx, the learned tensors resultBNBiasDiff and resultBNScaleDiff with their
 descriptor.

 If BOTH savedMean, and savedVariance are not null pointers then the method will use the saved
 mean and variance calculated by the forward training phase.

 @param handle                    MIOpen handle (input)
 @param bn_mode                   Batch normalization mode (input)
 @param alphaDataDiff             Floating point scaling factor, allocated on the host (input)
 @param betaDataDiff              Floating point shift factor, allocated on the host (input)
 @param alphaParamDiff            Floating point scaling factor, allocated on the host (input)
 @param betaParamDiff             Floating point shift factor, allocated on the host (input)
 @param xDesc                     Tensor descriptor for data input tensor x (input)
 @param x                         Data tensor x (input)
 @param dyDesc                    Tensor descriptor for output data tensor y (input)
 @param dy                        Data tensor y (input)
 @param dxDesc                    Tensor descriptor for output data tensor dx (input)
 @param dx                        Data delta tensor dx (output)
 @param bnScaleBiasDiffDesc       Tensor descriptor for BN scaling, shifting, saved variance and
 mean (input)
 @param bnScale                   Batch norm scaling, gamma, tensor (input)
 @param resultBnScaleDiff         Tensor for dscale (output)
 @param resultBnBiasDiff          Tensor for dbias (output)
 @param epsilon                   Value to stabilize inverse variance calculation (input)
 @param savedMean                 Saved mini-batch mean for backwards pass (input)
 @param savedInvVariance          Saved mini-bathc inverse variance for backwards pass (input)
 @return                          miopenStatus_t*/
    pub fn miopenBatchNormalizationBackward(
        handle: miopenHandle_t,
        bn_mode: miopenBatchNormMode_t,
        alphaDataDiff: *const ::core::ffi::c_void,
        betaDataDiff: *const ::core::ffi::c_void,
        alphaParamDiff: *const ::core::ffi::c_void,
        betaParamDiff: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        bnScaleBiasDiffDesc: miopenTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        resultBnScaleDiff: *mut ::core::ffi::c_void,
        resultBnBiasDiff: *mut ::core::ffi::c_void,
        epsilon: f64,
        savedMean: *const ::core::ffi::c_void,
        savedInvVariance: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute backwards propagation layer for batch normalization

 Batch normalization pass for backwards propagation training pass.
 The method for backwards propagation batch normalization.

 Takes in batch normalization mode bn_mode and input tensor data x, input activation tensor dy,
 output tensor dx, the learned tensors resultBNBiasDiff and resultBNScaleDiff with their
 descriptor.

 If BOTH savedMean, and savedVariance are not null pointers then the method will use the saved
 mean and variance calculated by the forward training phase.

 @param handle                    MIOpen handle (input)
 @param bn_mode                   Batch normalization mode (input)
 @param alphaDataDiff             Floating point scaling factor, allocated on the host (input)
 @param betaDataDiff              Floating point shift factor, allocated on the host (input)
 @param alphaParamDiff            Floating point scaling factor, allocated on the host (input)
 @param betaParamDiff             Floating point shift factor, allocated on the host (input)
 @param xDesc                     Tensor descriptor for data input tensor x (input)
 @param x                         Data tensor x (input)
 @param dyDesc                    Tensor descriptor for output data tensor y (input)
 @param dy                        Data tensor y (input)
 @param dxDesc                    Tensor descriptor for output data tensor dx (input)
 @param dx                        Data delta tensor dx (output)
 @param scaleDesc                 Tensor descriptor for scaling descriptor (input)
 @param biasDesc                  Tensor descriptor for bias/shift descriptor (input)
 @param savedMeanDesc             Tensor descriptor for saved Mean  descriptor (input)
 @param savedVarDesc              Tensor descriptor for saved Variance descriptor (input)
 , shifting, saved variance and
 mean (input)
 @param bnScale                   Batch norm scaling, gamma, tensor (input)
 @param resultBnScaleDiff         Tensor for dscale (output)
 @param resultBnBiasDiff          Tensor for dbias (output)
 @param epsilon                   Value to stabilize inverse variance calculation (input)
 @param savedMean                 Saved mini-batch mean for backwards pass (input)
 @param savedInvVariance          Saved mini-bathc inverse variance for backwards pass (input)
 @return                          miopenStatus_t*/
    pub fn miopenBatchNormalizationBackward_V2(
        handle: miopenHandle_t,
        bn_mode: miopenBatchNormMode_t,
        alphaDataDiff: *const ::core::ffi::c_void,
        betaDataDiff: *const ::core::ffi::c_void,
        alphaParamDiff: *const ::core::ffi::c_void,
        betaParamDiff: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        scaleDesc: miopenTensorDescriptor_t,
        biasDesc: miopenTensorDescriptor_t,
        savedMeanDesc: miopenTensorDescriptor_t,
        savedVarDesc: miopenTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        resultBnScaleDiff: *mut ::core::ffi::c_void,
        resultBnBiasDiff: *mut ::core::ffi::c_void,
        epsilon: f64,
        savedMean: *const ::core::ffi::c_void,
        savedInvVariance: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @addtogroup activation\n\n  @{\n/\n/*! @brief Creates the Activation descriptor object\n\n @param activDesc Pointer to an activation tensor descriptor type\n @return          miopenStatus_t"]
    pub fn miopenCreateActivationDescriptor(
        activDesc: *mut miopenActivationDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the activation layer descriptor details

 Sets all of the descriptor details for the activation layer

 @param activDesc    Pointer to a activation layer descriptor (output)
 @param mode         Activation mode enum (input)
 @param activAlpha   Alpha value for some activation modes (input)
 @param activBeta    Beta value for some activation modes (input)
 @param activGamma   Gamma value for some activation modes (input)
 @return             miopenStatus_t*/
    pub fn miopenSetActivationDescriptor(
        activDesc: miopenActivationDescriptor_t,
        mode: miopenActivationMode_t,
        activAlpha: f64,
        activBeta: f64,
        activGamma: f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the activation layer descriptor details

 Retrieves all of the descriptor details for the activation layer.

 @param activDesc    Pointer to a activation layer descriptor (input)
 @param mode         Activation mode enum (output)
 @param activAlpha   Alpha value for some activation modes (output)
 @param activBeta    Beta value for some activation modes (output)
 @param activGamma   Gamma value for some activation modes (output)
 @return             miopenStatus_t*/
    pub fn miopenGetActivationDescriptor(
        activDesc: miopenActivationDescriptor_t,
        mode: *mut miopenActivationMode_t,
        activAlpha: *mut f64,
        activBeta: *mut f64,
        activGamma: *mut f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute an activation forward layer

 @param handle         MIOpen handle (input)
 @param activDesc      Descriptor for activation layer (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param xDesc          Tensor descriptor for data input tensor x (input)
 @param x              Data tensor x (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param y              Data tensor y (output)
 @return               miopenStatus_t*/
    pub fn miopenActivationForward(
        handle: miopenHandle_t,
        activDesc: miopenActivationDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a activation backwards layer

 @param handle         MIOpen handle (input)
 @param activDesc      Descriptor for activation layer (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for input data tensor y (input)
 @param y              Data tensor y (input)
 @param dyDesc         Tensor descriptor for input data tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param xDesc          Tensor descriptor for data input tensor x (input)
 @param x              Data tensor x (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param dxDesc         Tensor descriptor for data output tensor dx (input)
 @param dx             Output data delta tensor dx (output)
 @return               miopenStatus_t*/
    pub fn miopenActivationBackward(
        handle: miopenHandle_t,
        activDesc: miopenActivationDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the activation descriptor object

 @param activDesc   Activation tensor descriptor type (input)
 @return            miopenStatus_t*/
    pub fn miopenDestroyActivationDescriptor(
        activDesc: miopenActivationDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @addtogroup softmax\n\n  @{\n/\n/*! @brief Execute a softmax forward layer\n\n This API only implements the SOFTMAX_MODE_CHANNEL in SOFTMAX_ACCURATE path.\n\n @param handle         MIOpen handle (input)\n @param alpha          Floating point scaling factor, allocated on the host (input)\n @param xDesc          Tensor descriptor for data input tensor x (input)\n @param x              Data tensor x (input)\n @param beta           Floating point shift factor, allocated on the host (input)\n @param yDesc          Tensor descriptor for output data tensor y (input)\n @param y              Data tensor y (output)\n @return               miopenStatus_t"]
    pub fn miopenSoftmaxForward(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a softmax backwards layer

 This API only implements the SOFTMAX_MODE_CHANNEL in SOFTMAX_ACCURATE path.

 @param handle         MIOpen handle (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for input data tensor y (input)
 @param y              Data tensor y (input)
 @param dyDesc         Tensor descriptor for input data tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param dxDesc         Tensor descriptor for data output tensor dx (input)
 @param dx             Output data delta tensor dx (output)
 @return               miopenStatus_t*/
    pub fn miopenSoftmaxBackward(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a softmax forward layer with expanded modes and algorithms

 @param handle         MIOpen handle (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param xDesc          Tensor descriptor for data input tensor x (input)
 @param x              Data tensor x (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for output data tensor y (input)
 @param y              Data tensor y (output)
 @param algorithm      Softmax implementation algorithm (input)
 @param mode           Softmax mode (input)
 @return               miopenStatus_t*/
    pub fn miopenSoftmaxForward_V2(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        algorithm: miopenSoftmaxAlgorithm_t,
        mode: miopenSoftmaxMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute a softmax backwards layer with expanded modes and algorithms

 @param handle         MIOpen handle (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param yDesc          Tensor descriptor for input data tensor y (input)
 @param y              Data tensor y (input)
 @param dyDesc         Tensor descriptor for input data tensor dy (input)
 @param dy             Data delta tensor dy (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param dxDesc         Tensor descriptor for data output tensor dx (input)
 @param dx             Output data delta tensor dx (output)
 @param algorithm      Softmax implementation algorithm (input)
 @param mode           Softmax mode (input)
 @return               miopenStatus_t*/
    pub fn miopenSoftmaxBackward_V2(
        handle: miopenHandle_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        algorithm: miopenSoftmaxAlgorithm_t,
        mode: miopenSoftmaxMode_t,
    ) -> miopenStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenFusionPlanDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenFusionPlanDescriptor_t(pub *mut miopenFusionPlanDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenOperatorDescriptor {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenOperatorDescriptor_t(pub *mut miopenOperatorDescriptor);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenOperatorArgs {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenOperatorArgs_t(pub *mut miopenOperatorArgs);
impl miopenFusionDirection_t {
    ///< fuses layers vertically, current the only supported mode
    pub const miopenVerticalFusion: miopenFusionDirection_t = miopenFusionDirection_t(0);
}
impl miopenFusionDirection_t {
    ///< fuses layers horizontally, this is unimplemented
    pub const miopenHorizontalFusion: miopenFusionDirection_t = miopenFusionDirection_t(
        1,
    );
}
#[repr(transparent)]
/** @enum miopenFusionDirection_t
 @brief Kernel fusion direction in the network*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenFusionDirection_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates the kenrel fusion plan descriptor object

 @param fusePlanDesc  Pointer to a fusion plan (output)
 @param fuseDirection Horizontal or Vertical fusion (input)
 @param inputDesc     Descriptor to tensor for the input (input)
 @return              miopenStatus_t*/
    pub fn miopenCreateFusionPlan(
        fusePlanDesc: *mut miopenFusionPlanDescriptor_t,
        fuseDirection: miopenFusionDirection_t,
        inputDesc: miopenTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroy the fusion plan descriptor object

 @param fusePlanDesc  A fusion plan descriptor type
 @return              miopenStatus_t*/
    pub fn miopenDestroyFusionPlan(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Compiles the fusion plan

 @param handle           MIOpen handle (input)
 @param fusePlanDesc A fusion plan descriptor (input)
 @return             miopenStatus_t*/
    pub fn miopenCompileFusionPlan(
        handle: miopenHandle_t,
        fusePlanDesc: miopenFusionPlanDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Allows access to the operators in a fusion plan
 @details This api call does bounds checking on the supplied op_idx and would
          return miopenStatusError if the index is out of bounds

 @param fusePlanDesc A fusion plan descriptor (input)
 @param op_idx Index of the required operator in the fusion plan, in the order of insertion
 @param op returned pointer to the operator
 @return miopenStatus_t*/
    pub fn miopenFusionPlanGetOp(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        op_idx: ::core::ffi::c_int,
        op: *mut miopenFusionOpDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the workspace size required for the fusion plan
 @param handle         MIOpen handle (input)
 @param fusePlanDesc   A fusion plan descriptor (input)
 @param workSpaceSize  Pointer to memory to return size in bytes (output)
 @param algo           Algorithm selected (inputs)
 @return               miopenStatus_t*/
    pub fn miopenFusionPlanGetWorkSpaceSize(
        handle: miopenHandle_t,
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        workSpaceSize: *mut usize,
        algo: miopenConvFwdAlgorithm_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the supported algorithms for the convolution operator in the Fusion Plan

 @details A Convolution operator in a fusion plan may be implemented by different algorithms
 representing different tradeoffs of memory and performance. The returned list of algorithms
 is sorted in decreasing order of priority. Therefore, if the user does not request an
 algorithm to be set using the miopenFusionPlanConvolutionSetAlgo call, the first algorithm
 in the list would be used to execute the convolution in the fusion plan. Moreover this call
 must be immediately preceded by the miopenCreateOpConvForward call for the op in question.

 @param fusePlanDesc A fusion plan descriptor (input)
 @param requestAlgoCount Number of algorithms to return (input)
 @param returnedAlgoCount The actual number of returned algorithms; always be less than
 equal to requestAlgoCount (output)
 @param returnedAlgos Pointer to the list of supported algorithms
 @return miopenStatus_t*/
    pub fn miopenFusionPlanConvolutionGetAlgo(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        requestAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        returnedAlgos: *mut miopenConvFwdAlgorithm_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Requests the fusion runtime to choose a particular algorithm for the added convolution
 operation

 @details Please see the description for miopenFusionPlanConvolutionGetAlgo

 @param fusePlanDesc A fusion plan descriptor (input)
 @param algo Requested algorithm for the convolution operator (input)
 @return miopenStatus_t*/
    pub fn miopenFusionPlanConvolutionSetAlgo(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        algo: miopenConvFwdAlgorithm_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates forward convolution operator.

 @param fusePlanDesc   A fusion plan descriptor (input)
 @param convOp         Pointer to an operator type (output)
 @param convDesc       Convolution layer descriptor (input)
 @param wDesc          Descriptor for the weights tensor (input)
 @return               miopenStatus_t*/
    pub fn miopenCreateOpConvForward(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        convOp: *mut miopenFusionOpDescriptor_t,
        convDesc: miopenConvolutionDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a forward activation operator.

 @param fusePlanDesc    A fusion plan descriptor (input)
 @param activFwdOp         Pointer to an operator type (output)
 @param mode            Activation version (input)
 @return                miopenStatus_t*/
    pub fn miopenCreateOpActivationForward(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        activFwdOp: *mut miopenFusionOpDescriptor_t,
        mode: miopenActivationMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a backward activation operator.

 @param fusePlanDesc    A fusion plan descriptor (input)
 @param activBwdOp         Pointer to an operator type (output)
 @param mode            Activation version (input)
 @return                miopenStatus_t*/
    pub fn miopenCreateOpActivationBackward(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        activBwdOp: *mut miopenFusionOpDescriptor_t,
        mode: miopenActivationMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a forward bias operator.

 @param fusePlanDesc   A fusion plan descriptor (input)
 @param biasOp         Pointer to an operator type (output)
 @param bDesc          bias tensor descriptor (input)
 @return               miopenStatus_t*/
    pub fn miopenCreateOpBiasForward(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        biasOp: *mut miopenFusionOpDescriptor_t,
        bDesc: miopenTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a forward inference batch normalization operator.

 @param fusePlanDesc           A fusion plan descriptor (input)
 @param bnOp                   Pointer to an operator type (output)
 @param bn_mode                Batch normalization layer mode (input)
 @param bnScaleBiasMeanVarDesc Gamma, beta, mean, variance tensor descriptor (input)
 @return                       miopenStatus_t*/
    pub fn miopenCreateOpBatchNormInference(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        bnOp: *mut miopenFusionOpDescriptor_t,
        bn_mode: miopenBatchNormMode_t,
        bnScaleBiasMeanVarDesc: miopenTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a forward training batch normalization operator.

 @param fusePlanDesc           A fusion plan descriptor (input)
 @param bnFwdOp                   Pointer to an operator type (output)
 @param bn_mode                Batch normalization layer mode (input)
 @param runningMeanVariance    Toggles whether or not to save population statistics for inference;
 batch statistic are required (input)
 @return                       miopenStatus_t*/
    pub fn miopenCreateOpBatchNormForward(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        bnFwdOp: *mut miopenFusionOpDescriptor_t,
        bn_mode: miopenBatchNormMode_t,
        runningMeanVariance: bool,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a back propagation batch normalization operator.

 @param fusePlanDesc           A fusion plan descriptor (input)
 @param bnBwdOp                   Pointer to an operator type (output)
 @param bn_mode                Batch normalization layer mode (input)
 @return                       miopenStatus_t*/
    pub fn miopenCreateOpBatchNormBackward(
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        bnBwdOp: *mut miopenFusionOpDescriptor_t,
        bn_mode: miopenBatchNormMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates an operator argument object

 @param args        Pointer to an operator argument type (output)
 @return            miopenStatus_t*/
    pub fn miopenCreateOperatorArgs(args: *mut miopenOperatorArgs_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys an operator argument object

 @param args        An operator argument type (output)
 @return            miopenStatus_t*/
    pub fn miopenDestroyOperatorArgs(args: miopenOperatorArgs_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the arguments for forward convolution op

 @param args    An arguments object type (output)
 @param convOp  Forward convolution operator (input)
 @param alpha   Floating point scaling factor, allocated on the host (input)
 @param beta    Floating point shift factor, allocated on the host (input)
 @param w       Pointer to tensor memory  (input)
 @return        miopenStatus_t*/
    pub fn miopenSetOpArgsConvForward(
        args: miopenOperatorArgs_t,
        convOp: miopenFusionOpDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        w: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the arguments for forward activation op

 @param args    An arguments object type (output)
 @param activFwdOp   Activation backwards operator (input)
 @param alpha   Floating point scaling factor, allocated on the host (input)
 @param beta    Floating point shift factor, allocated on the host (input)
 @param activAlpha  Double precision activation parameter which depends on activation mode (input)
 @param activBeta   Double precision activation parameter which depends on activation mode (input)
 @param activGamma  Double precision activation parameter which depends on activation mode (input)
 @return        miopenStatus_t*/
    pub fn miopenSetOpArgsActivForward(
        args: miopenOperatorArgs_t,
        activFwdOp: miopenFusionOpDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        activAlpha: f64,
        activBeta: f64,
        activGamma: f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the arguments for backward activation op

 @param args    An arguments object type (output)
 @param activBwdOp   Activation backwards operator (input)
 @param alpha   Floating point scaling factor, allocated on the host (input)
 @param beta    Floating point shift factor, allocated on the host (input)
 @param y        Data tensor y, output of activations in the forward direction (input)
 @param reserved    Data tensor reserved memory space; currently should be nullptr (input)
 @param activAlpha  Double precision activation parameter which depends on activation mode (input)
 @param activBeta   Double precision activation parameter which depends on activation mode (input)
 @param activGamma  Double precision activation parameter which depends on activation mode (input)
 @return        miopenStatus_t*/
    pub fn miopenSetOpArgsActivBackward(
        args: miopenOperatorArgs_t,
        activBwdOp: miopenFusionOpDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        y: *const ::core::ffi::c_void,
        reserved: *const ::core::ffi::c_void,
        activAlpha: f64,
        activBeta: f64,
        activGamma: f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the arguments for inference batch normalization op

 @param args               An arguments object type (output)
 @param bnOp               Batch normalization inference operator (input)
 @param alpha              Floating point scaling factor, allocated on the host (input)
 @param beta               Floating point shift factor, allocated on the host (input)
 @param bnScale            Pointer to the gamma tensor memory  (input)
 @param bnBias             Pointer to the beta tensor memory  (input)
 @param estimatedMean      Pointer to population mean memory  (input)
 @param estimatedVariance  Pointer to population variance memory  (input)
 @param epsilon            Scalar value for numerical stability (input)
 @return                   miopenStatus_t*/
    pub fn miopenSetOpArgsBatchNormInference(
        args: miopenOperatorArgs_t,
        bnOp: miopenFusionOpDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        bnScale: *const ::core::ffi::c_void,
        bnBias: *const ::core::ffi::c_void,
        estimatedMean: *const ::core::ffi::c_void,
        estimatedVariance: *const ::core::ffi::c_void,
        epsilon: f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the arguments for forward batch normalization op

 @param args               An arguments object type (output)
 @param bnOp               Batch normalization forward operator (input)
 @param alpha              Floating point scaling factor, allocated on the host (input)
 @param beta               Floating point shift factor, allocated on the host (input)
 @param bnScale            Pointer to the gamma tensor memory  (input)
 @param bnBias             Pointer to the beta tensor memory  (input)
 @param savedMean          Pointer to batch mean memory  (input)
 @param savedInvVariance   Pointer to batch inverse variance memory  (input)
 @param runningMean        Pointer to population mean memory  (input)
 @param runningVariance    Pointer to population variance memory  (input)
 @param expAvgFactor       Scalar value for control of population statistics (input)
 @param epsilon            Scalar value for numerical stability (input)
 @return                   miopenStatus_t*/
    pub fn miopenSetOpArgsBatchNormForward(
        args: miopenOperatorArgs_t,
        bnOp: miopenFusionOpDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        bnScale: *const ::core::ffi::c_void,
        bnBias: *const ::core::ffi::c_void,
        savedMean: *mut ::core::ffi::c_void,
        savedInvVariance: *mut ::core::ffi::c_void,
        runningMean: *mut ::core::ffi::c_void,
        runningVariance: *mut ::core::ffi::c_void,
        expAvgFactor: f64,
        epsilon: f64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the arguments for backward batch normalization op

 @param args               An arguments object type (output)
 @param bnOp               Batch normalization forward operator (input)
 @param alpha              Floating point scaling factor, allocated on the host (input)
 @param beta               Floating point shift factor, allocated on the host (input)
 @param x                  Pointer to the forward input tensor memory  (input)
 @param bnScale            Pointer to the gamma tensor memory  (input)
 @param bnBias             Pointer to the beta tensor memory  (input)
 @param resultBnScaleDiff  Pointer to the gamma gradient tensor memory  (output)
 @param resultBnBiasDiff   Pointer to the beta gradient tensor memory  (output)
 @param savedMean          Pointer to batch mean memory  (input)
 @param savedInvVariance   Pointer to batch inverse variance memory  (input)
 @return                   miopenStatus_t*/
    pub fn miopenSetOpArgsBatchNormBackward(
        args: miopenOperatorArgs_t,
        bnOp: miopenFusionOpDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        x: *const ::core::ffi::c_void,
        bnScale: *const ::core::ffi::c_void,
        bnBias: *const ::core::ffi::c_void,
        resultBnScaleDiff: *mut ::core::ffi::c_void,
        resultBnBiasDiff: *mut ::core::ffi::c_void,
        savedMean: *const ::core::ffi::c_void,
        savedInvVariance: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the arguments for forward bias op

 @param args           An arguments object type (output)
 @param biasOp         Forward bias operator (input)
 @param alpha          Floating point scaling factor, allocated on the host (input)
 @param beta           Floating point shift factor, allocated on the host (input)
 @param bias           Pointer to the forward bias input tensor memory  (input)
 @return               miopenStatus_t*/
    pub fn miopenSetOpArgsBiasForward(
        args: miopenOperatorArgs_t,
        biasOp: miopenFusionOpDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        bias: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Executes the fusion plan


 @param handle           MIOpen handle (input)
 @param fusePlanDesc     fused plan descriptor (input)
 @param inputDesc        Descriptor of the input tensor (input)
 @param input            Source data tensor  (input)
 @param outputDesc       Decriptor of the output tensor (input)
 @param output           Destination data tensor  (output)
 @param args             An argument object of the fused kernel (input)
 @return           miopenStatus_t*/
    pub fn miopenExecuteFusionPlan(
        handle: miopenHandle_t,
        fusePlanDesc: miopenFusionPlanDescriptor_t,
        inputDesc: miopenTensorDescriptor_t,
        input: *const ::core::ffi::c_void,
        outputDesc: miopenTensorDescriptor_t,
        output: *mut ::core::ffi::c_void,
        args: miopenOperatorArgs_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Prepares and executes the Convlution+Bias+Activation Fusion.


 @param handle               MIOpen handle (input)
 @param alpha1               floating point scaling factor, allocated on the host (input)
 @param xDesc                Tensor descriptor for input data tensor x (input)
 @param x                    Data tensor x (input)
 @param wDesc                Tensor descriptor for weight tensor w (input)
 @param w                    Weights tensor w (input)
 @param convDesc             Convolution layer descriptor (input)
 @param algo                 Algorithm selected (inputs)
 @param workspace            Pointer to workspace required (input)
 @param workspaceSizeInBytes Size of the memory in bytes pointed to by workSpace above
 @param alpha2               floating point scaling factor, allocated on the host (input)
 @param zDesc                Tensor descriptor for tensor z (input)
 @param z                    Data tensor z (input)
 @param biasDesc             Tensor descriptor for input data tensor x (input)
 @param bias                 Data tensor bias (input)
 @param activationDesc       Activation descriptor that specifies the activation mode
 @param yDesc                Tensor descriptor for output data tensor y (input)
 @param y                    Output data tensor*/
    pub fn miopenConvolutionBiasActivationForward(
        handle: miopenHandle_t,
        alpha1: *const ::core::ffi::c_void,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: miopenConvolutionDescriptor_t,
        algo: miopenConvFwdAlgorithm_t,
        workspace: *mut ::core::ffi::c_void,
        workspaceSizeInBytes: usize,
        alpha2: *const ::core::ffi::c_void,
        zDesc: miopenTensorDescriptor_t,
        z: *const ::core::ffi::c_void,
        biasDesc: miopenTensorDescriptor_t,
        bias: *const ::core::ffi::c_void,
        activationDesc: miopenActivationDescriptor_t,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
impl miopenRNNMode_t {
    ///< RNN with ReLU activation
    pub const miopenRNNRELU: miopenRNNMode_t = miopenRNNMode_t(0);
}
impl miopenRNNMode_t {
    ///< RNN with tanh activation
    pub const miopenRNNTANH: miopenRNNMode_t = miopenRNNMode_t(1);
}
impl miopenRNNMode_t {
    ///< LSTM
    pub const miopenLSTM: miopenRNNMode_t = miopenRNNMode_t(2);
}
impl miopenRNNMode_t {
    ///< GRU
    pub const miopenGRU: miopenRNNMode_t = miopenRNNMode_t(3);
}
#[repr(transparent)]
/**  @enum miopenRNNMode_t
 RNN mode selection for rnn layer preference*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNMode_t(pub ::core::ffi::c_uint);
impl miopenRNNInputMode_t {
    ///< Matrix multiplication at the input of the first layer
    pub const miopenRNNlinear: miopenRNNInputMode_t = miopenRNNInputMode_t(0);
}
impl miopenRNNInputMode_t {
    ///< No operation is performed at the input of the first layer.
    pub const miopenRNNskip: miopenRNNInputMode_t = miopenRNNInputMode_t(1);
}
#[repr(transparent)]
/** @enum miopenRNNInputMode_t
 Recurrent Neural Network layer initial input mode*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNInputMode_t(pub ::core::ffi::c_uint);
impl miopenRNNAlgo_t {
    /**< Use dedicated gate-operation kernel for LSTM and fundamental
algorithm for vanilla RNN & GRU*/
    pub const miopenRNNdefault: miopenRNNAlgo_t = miopenRNNAlgo_t(0);
}
impl miopenRNNAlgo_t {
    /**< Deprecated, low performance. Function by basic tesnsor
operations, supported for vanilla RNN, LSTM, GRU*/
    pub const miopenRNNfundamental: miopenRNNAlgo_t = miopenRNNAlgo_t(1);
}
impl miopenRNNAlgo_t {
    /**< The algorithm rounds some RNN parametrs upwards
to utilize the most optimal GEMM kernel in the computation.*/
    pub const miopenRNNroundedDynamic: miopenRNNAlgo_t = miopenRNNAlgo_t(2);
}
#[repr(transparent)]
/** @enum miopenRNNAlgo_t
 Recurrent Neural Network algorithm mode*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNAlgo_t(pub ::core::ffi::c_uint);
impl miopenRNNDirectionMode_t {
    ///< Forward in time only.
    pub const miopenRNNunidirection: miopenRNNDirectionMode_t = miopenRNNDirectionMode_t(
        0,
    );
}
impl miopenRNNDirectionMode_t {
    ///< Forward and backwards in time.
    pub const miopenRNNbidirection: miopenRNNDirectionMode_t = miopenRNNDirectionMode_t(
        1,
    );
}
#[repr(transparent)]
/** @enum miopenRNNDirectionMode_t
 Recurrent Neural Network bi-directional behavior*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNDirectionMode_t(pub ::core::ffi::c_uint);
impl miopenRNNBiasMode_t {
    ///< No Biases will be applied to GEMM operations
    pub const miopenRNNNoBias: miopenRNNBiasMode_t = miopenRNNBiasMode_t(0);
}
impl miopenRNNBiasMode_t {
    ///< Biases will be applied to GEMM operations
    pub const miopenRNNwithBias: miopenRNNBiasMode_t = miopenRNNBiasMode_t(1);
}
#[repr(transparent)]
/** @enum miopenRNNBiasMode_t
 Recurrent Neural Network add on bias*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNBiasMode_t(pub ::core::ffi::c_uint);
impl miopenRNNGEMMalgoMode_t {
    pub const miopenRNNAlgoGEMM: miopenRNNGEMMalgoMode_t = miopenRNNGEMMalgoMode_t(0);
}
#[repr(transparent)]
/** @enum miopenRNNGEMMalgoMode_t
 Recurrent Neural Network add on bias*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNGEMMalgoMode_t(pub ::core::ffi::c_uint);
impl miopenRNNPaddingMode_t {
    ///< Not padded data at RNN input/output
    pub const miopenRNNIONotPadded: miopenRNNPaddingMode_t = miopenRNNPaddingMode_t(0);
}
impl miopenRNNPaddingMode_t {
    ///< Padded data at RNN input/output
    pub const miopenRNNIOWithPadding: miopenRNNPaddingMode_t = miopenRNNPaddingMode_t(1);
}
#[repr(transparent)]
/** @enum miopenRNNPaddingMode_t
 Recurrent Neural Network input/output data padding mode*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNPaddingMode_t(pub ::core::ffi::c_uint);
impl miopenRNNFWDMode_t {
    ///< FWD, BWD, WRW
    pub const miopenRNNTraining: miopenRNNFWDMode_t = miopenRNNFWDMode_t(0);
}
impl miopenRNNFWDMode_t {
    ///< Only FWD-inference no back-propagation
    pub const miopenRNNInference: miopenRNNFWDMode_t = miopenRNNFWDMode_t(1);
}
#[repr(transparent)]
/** @enum miopenRNNFWDMode_t
 Recurrent Neural Network Training/Inference mode*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNFWDMode_t(pub ::core::ffi::c_uint);
impl miopenRNNBaseLayout_t {
    pub const miopenRNNDataUnknownLayout: miopenRNNBaseLayout_t = miopenRNNBaseLayout_t(
        0,
    );
}
impl miopenRNNBaseLayout_t {
    pub const miopenRNNDataSeqMajorNotPadded: miopenRNNBaseLayout_t = miopenRNNBaseLayout_t(
        1,
    );
}
impl miopenRNNBaseLayout_t {
    pub const miopenRNNDataSeqMajorPadded: miopenRNNBaseLayout_t = miopenRNNBaseLayout_t(
        2,
    );
}
impl miopenRNNBaseLayout_t {
    pub const miopenRNNDataBatchMajorPadded: miopenRNNBaseLayout_t = miopenRNNBaseLayout_t(
        3,
    );
}
#[repr(transparent)]
/** @enum miopenRNNBaseLayout_t
 Data layouts for RNN operations*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNNBaseLayout_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create a RNN layer Descriptor

 API for creating an uninitialized RNN layer descriptor.
 @param rnnDesc    Pointer to a tensor descriptor type
 @return           miopenStatus_t*/
    pub fn miopenCreateRNNDescriptor(
        rnnDesc: *mut miopenRNNDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retrieves a RNN layer descriptor's details

 @param rnnDesc    RNN layer descriptor (input)
 @param rnnMode    RNN mode (output)
 @param algoMode   RNN algorithm mode (output)
 @param inputMode  RNN data input mode (output)
 @param dirMode    Uni or bi direction mode (output)
 @param biasMode   Bias used (output)
 @param hiddenSize Size of hidden state (output)
 @param layer      Number of stacked layers (output)
 @return           miopenStatus_t*/
    pub fn miopenGetRNNDescriptor(
        rnnDesc: miopenRNNDescriptor_t,
        rnnMode: *mut miopenRNNMode_t,
        algoMode: *mut miopenRNNAlgo_t,
        inputMode: *mut miopenRNNInputMode_t,
        dirMode: *mut miopenRNNDirectionMode_t,
        biasMode: *mut miopenRNNBiasMode_t,
        hiddenSize: *mut ::core::ffi::c_int,
        layer: *mut ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retrieves a RNN layer descriptor's details version 2. This version enables retrieving
 information of the dropout descriptor of the rnn descriptor.

 @param rnnDesc     RNN layer descriptor (input)
 @param hiddenSize  Size of hidden state (output)
 @param layer       Number of stacked layers (output)
 @param dropoutDesc Pre-configured dropout descriptor for dropout layer in between RNN layers
 (output)
 @param inputMode   RNN data input mode (output)
 @param dirMode     Uni or bi direction mode (output)
 @param rnnMode     RNN mode (output)
 @param biasMode    Bias used (output)
 @param algoMode    RNN algorithm mode (output)
 @param dataType    Data type of RNN (output)
 @return            miopenStatus_t*/
    pub fn miopenGetRNNDescriptor_V2(
        rnnDesc: miopenRNNDescriptor_t,
        hiddenSize: *mut ::core::ffi::c_int,
        layer: *mut ::core::ffi::c_int,
        dropoutDesc: *mut miopenDropoutDescriptor_t,
        inputMode: *mut miopenRNNInputMode_t,
        dirMode: *mut miopenRNNDirectionMode_t,
        rnnMode: *mut miopenRNNMode_t,
        biasMode: *mut miopenRNNBiasMode_t,
        algoMode: *mut miopenRNNAlgo_t,
        dataType: *mut miopenDataType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the tensor descriptor object

 @param rnnDesc RNN tensor descriptor type (input)
 @return           miopenStatus_t*/
    pub fn miopenDestroyRNNDescriptor(rnnDesc: miopenRNNDescriptor_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the details of the RNN descriptor

 Interface for setting the values of the RNN descriptor object. This function requires specific
 algorithm selection.
 @param rnnDesc      RNN layer descriptor type (input)
 @param hsize        Hidden layer size (input)
 @param nlayers      Number of layers (input)
 @param inMode       RNN first layer input mode (input)
 @param direction    RNN direction (input)
 @param rnnMode      RNN model type (input)
 @param biasMode     RNN bias included (input)
 @param algo         RNN algorithm selected (input)
 @param dataType     MIOpen datatype (input)
 @return             miopenStatus_t*/
    pub fn miopenSetRNNDescriptor(
        rnnDesc: miopenRNNDescriptor_t,
        hsize: ::core::ffi::c_int,
        nlayers: ::core::ffi::c_int,
        inMode: miopenRNNInputMode_t,
        direction: miopenRNNDirectionMode_t,
        rnnMode: miopenRNNMode_t,
        biasMode: miopenRNNBiasMode_t,
        algo: miopenRNNAlgo_t,
        dataType: miopenDataType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the details of the RNN descriptor version 2. This version enables the use of dropout
 in rnn.

 Interface for setting the values of the RNN descriptor object. This function requires specific
 algorithm selection.
 @param rnnDesc      RNN layer descriptor type (input/output)
 @param hsize        Hidden layer size (input)
 @param nlayers      Number of layers (input)
 @param dropoutDesc  Pre-initialized dropout descriptor for dropout layer in between RNN layers
 (input)
 @param inMode       RNN first layer input mode (input)
 @param direction    RNN direction (input)
 @param rnnMode      RNN model type (input)
 @param biasMode     RNN bias included (input)
 @param algo         RNN algorithm selected (input)
 @param dataType     MIOpen datatype (input)
 @return             miopenStatus_t*/
    pub fn miopenSetRNNDescriptor_V2(
        rnnDesc: miopenRNNDescriptor_t,
        hsize: ::core::ffi::c_int,
        nlayers: ::core::ffi::c_int,
        dropoutDesc: miopenDropoutDescriptor_t,
        inMode: miopenRNNInputMode_t,
        direction: miopenRNNDirectionMode_t,
        rnnMode: miopenRNNMode_t,
        biasMode: miopenRNNBiasMode_t,
        algo: miopenRNNAlgo_t,
        dataType: miopenDataType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set shape of RNN seqData tensor

 Interface for setting tensor shape to be used as RNN input data

 @param seqTensorDesc     Tensor descriptor (input/output)
 @param dataType          MIOpen datatype (input)
 @param layout            One of the main supported layouts for RNN data(input)
 @param maxSequenceLen      Sequence length limit within this SeqTensor(input)
 @param batchSize         Number of sequences within this SeqTensor (input)
 @param vectorSize        Vector size (input)
 @param sequenceLenArray  Array containing the length of each sequence in the SeqTensor(input)
 @param paddingMarker     Not used, should be NULL (input)
 @return                  miopenStatus_t*/
    pub fn miopenSetRNNDataSeqTensorDescriptor(
        seqTensorDesc: miopenSeqTensorDescriptor_t,
        dataType: miopenDataType_t,
        layout: miopenRNNBaseLayout_t,
        maxSequenceLen: ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
        vectorSize: ::core::ffi::c_int,
        sequenceLenArray: *const ::core::ffi::c_int,
        paddingMarker: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get shape of RNN seqData tensor

 Interface for setting tensor shape to be used as RNN input data

 @param seqTensorDesc             Tensor descriptor (input)
 @param dataType                  MIOpen datatype (output)
 @param layout                    One of the main supported layouts for RNN data(output)
 @param maxSequenceLen              Sequence length limit within this SeqTensor(output)
 @param batchSize                 Number of sequences within this SeqTensor (output)
 @param vectorSize                Vector size (output)
 @param sequenceLenArrayLimit  Limit for number of elements that can be returned to user
 by sequenceLenArray (input)
 @param sequenceLenArray          Array containing the length of each sequence in the
 SeqTensor. This is allowed to be a NULL pointer if sequenceLenArrayLimit is 0 (output)
 @param paddingMarker             Not used, should be NULL (input)
 @return                          miopenStatus_t*/
    pub fn miopenGetRNNDataSeqTensorDescriptor(
        seqTensorDesc: miopenSeqTensorDescriptor_t,
        dataType: *mut miopenDataType_t,
        layout: *mut miopenRNNBaseLayout_t,
        maxSequenceLen: *mut ::core::ffi::c_int,
        batchSize: *mut ::core::ffi::c_int,
        vectorSize: *mut ::core::ffi::c_int,
        sequenceLenArrayLimit: ::core::ffi::c_int,
        sequenceLenArray: *mut ::core::ffi::c_int,
        paddingMarker: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the amount of memory required to execute the RNN layer

 This function calculates the amount of memory required to run the RNN layer given an RNN
 descriptor and a tensor descriptor.

 @param handle          MIOpen handle (input)
 @param rnnDesc         RNN layer descriptor type (input)
 @param sequenceLen     Number of iteration unrolls (input)
 @param xDesc           An array of tensor descriptors. These are the
 input descriptors to each time step. The first dimension of each descriptor is the
 batch size and may decrease from element n to element n+1 and not increase in size.
 The second dimension is the same for all descriptors in the array and is the input
 vector length. (input)
 @param numBytes        Number of bytes required for RNN layer execution (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNWorkspaceSize(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        sequenceLen: ::core::ffi::c_int,
        xDesc: *const miopenTensorDescriptor_t,
        numBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the amount of memory required for RNN training

 This function calculates the amount of memory required to train the RNN layer given an
 RNN descriptor and a tensor descriptor.

 @param handle          MIOpen handle (input)
 @param rnnDesc         RNN layer descriptor type (input)
 @param sequenceLen     Number of iteration unrolls (input)
 @param xDesc           An array of tensor descriptors. These are the
 input descriptors to each time step. The first dimension of each descriptor is the
 batch size and may decrease from element n to element n+1 and not increase in size.
 The second dimension is the same for all descriptors in the array and is the input
 vector length. (input)
 @param numBytes        Number of bytes required for RNN layer execution (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNTrainingReserveSize(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        sequenceLen: ::core::ffi::c_int,
        xDesc: *const miopenTensorDescriptor_t,
        numBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the amount of additional memory required for this RNN layer execution.

 This function calculates the size of extra buffers, depending on the layer configuration, which
 is determined by: RNN descriptor, isInference, and data descriptor. If isInference is True,
 reserve_space_size is always zero, because the reserve_space buffer is not used in Inference
 computation.

 @param handle           MIOpen handle (input)
 @param rnnDesc          RNN layer descriptor type (input)
 @param xDesc            Sequence data tensor descriptor (input)
 @param fwdMode          Specifies in which mode the buffers will be used.
 @param workSpaceSize    Minimum WorkSpace buffer size required for RNN layer execution (output)
 @param reserveSpaceSize Minimum ReserveSpaceSize buffer size required for RNN layer execution
 (output)
 @return                 miopenStatus_t*/
    pub fn miopenGetRNNTempSpaceSizes(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        xDesc: miopenSeqTensorDescriptor_t,
        fwdMode: miopenRNNFWDMode_t,
        workSpaceSize: *mut usize,
        reserveSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the amount of parameter memory required for RNN training

 This function calculates the amount of parameter memory required to train the RNN layer given an
 RNN descriptor and a tensor descriptor.

 @param handle          MIOpen handle (input)
 @param rnnDesc         RNN layer descriptor type (input)
 @param xDesc           A tensor descriptor (input)
 @param numBytes        Number of bytes required for RNN layer execution (output)
 @param dtype           MIOpen data type enum (input)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNParamsSize(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        numBytes: *mut usize,
        dtype: miopenDataType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Obtain a weight tensor descriptor for RNNs

 This function populates a weight descriptor that describes the memory layout of the
 weight matrix.

 @param handle          MIOpen handle (input)
 @param rnnDesc         Fully populated RNN layer descriptor type (input)
 @param xDesc           A previously populated tensor descriptor (input)
 @param wDesc           A previously allocated tensor descriptor (output)
 @param dtype           MIOpen data type enum (input)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNParamsDescriptor(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        dtype: miopenDataType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Obtain a the size in bytes of the RNN input tensor

 This function determines the size in bytes of the allocation needed for the input data
 tensor for an RNN layer. The number of bytes is derived from the array of
 tensor descriptors.

 @param handle          MIOpen handle (input)
 @param rnnDesc         Fully populated RNN layer descriptor (input)
 @param seqLen          Number of iteration unrolls (input)
 @param xDesc           An array of tensor descriptors. These are the
 input descriptors to each time step. The first dimension of each descriptor is the
 batch size and may decrease from element n to element n+1 and not increase in size.
 The second dimension is the same for all descriptors in the array and is the input
 vector length. (input)
 @param numBytes        Number of bytes required for input tensor (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNInputTensorSize(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        seqLen: ::core::ffi::c_int,
        xDesc: *mut miopenTensorDescriptor_t,
        numBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Obtain a the size in bytes of the RNN hidden tensor

 This function determines the size in bytes of the allocation needed for the
 hidden tensor over all layers

 @param handle          MIOpen handle (input)
 @param rnnDesc         Fully populated RNN layer descriptor type (input)
 @param seqLen          Number of iteration unrolls (input)
 @param xDesc           An array of previously populated tensor descriptors (input)
 @param numBytes        Number of bytes required for input tensor (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNHiddenTensorSize(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        seqLen: ::core::ffi::c_int,
        xDesc: *mut miopenTensorDescriptor_t,
        numBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the number of bytes of a parameter matrix


 For RNN vanilla miopenRNNRELU and miopenRNNTANH, paramID == 0 retrieves the
 weight matrix associated with the in input GEMM, while paramID == 1 retrieves
 the weight matrix associated with the hidden state GEMM.

 For miopenLSTM paramID 0 to 3 refer to the weight matrices associated
 with the input GEMM, 4-7 are associated with matrices associated with the
 hidden state GEMM.

 * paramID 0 and 4 are for the input gate.

 * paramID 1 and 5 are for the forget gate.

 * paramID 2 and 6 are for the output gate.

 * paramID 3 and 7 are for the new memory gate.

 For miopenGRU paramID 0 to 2 refer to the weight matrix offset associated
 with the input GEMM, while 3 through 5 are associated with the hidden state
 GEMM.

 * paramID 0 and 3 are for the update gate.

 * paramID 1 and 4 are for the reset gate.

 * paramID 2 and 5 are for the new memory gate.

 For bi-directional RNNs the backwards in time direction is numbered as the layer
 directly after the forward in time direction.

 @param handle          MIOpen handle (input)
 @param rnnDesc         RNN layer descriptor type (input)
 @param layer           The layer number in the RNN stack (input)
 @param xDesc           A tensor descriptor to input (input)
 @param paramID         ID of the internal parameter tensor (input)
 @param numBytes        The number of bytes of the layer's parameter matrix (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNLayerParamSize(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        layer: ::core::ffi::c_int,
        xDesc: miopenTensorDescriptor_t,
        paramID: ::core::ffi::c_int,
        numBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the number of bytes of a bias

 For RNN vanilla miopenRNNRELU and miopenRNNTANH, biasID == 0 retrieves the
 weight matrix associated with the in input GEMM, while biasID == 1 retrieves
 the bias associated with the hidden state GEMM.

 For miopenLSTM biasID 0 to 3 refer to the biases associated
 with the input GEMM, 4-7 are associated with biases associated with the
 hidden state GEMM.

 * biasID 0 and 4 are for the input gate.

 * biasID 1 and 5 are for the forget gate.

 * biasID 2 and 6 are for the output gate.

 * biasID 3 and 7 are for the new memory gate.

 For miopenGRU biasID 0 to 2 refer to the biases associated with the input GEMM,
 while 3 through 5 are associated with the hidden state GEMM.

 * biasID 0 and 3 are for the update gate.

 * biasID 1 and 4 are for the reset gate.

 * biasID 2 and 5 are for the new memory gate.

 For bi-directional RNNs the backwards in time direction is numbered as the layer
 directly after the forward in time direction.

 @param handle          MIOpen handle (input)
 @param rnnDesc         RNN layer descriptor type (input)
 @param layer           The layer number in the RNN stack (input)
 @param biasID          ID of the internal parameter tensor (input)
 @param numBytes        The number of bytes of the layer's bias (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNLayerBiasSize(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        layer: ::core::ffi::c_int,
        biasID: ::core::ffi::c_int,
        numBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a weight matrix for a specific layer in an RNN stack

 This function retrieves the weight matrix data for a specific layer and parameter ID
 and copies the data into previously allocated device memory.

 For RNN vanilla miopenRNNRELU and miopenRNNTANH, paramID == 0 retrieves the
 weight matrix associated with the in input GEMM, while paramID == 1 retrieves
 the weight matrix associated with the hidden state GEMM.

 For miopenLSTM paramID 0 to 3 refer to the weight matrices associated
 with the input GEMM, 4-7 are associated with matrices associated with the
 hidden state GEMM.

 * paramID 0 and 4 are for the input gate.

 * paramID 1 and 5 are for the forget gate.

 * paramID 2 and 6 are for the output gate.

 * paramID 3 and 7 are for the new memory gate.

 For miopenGRU paramID 0 to 2 refer to the weight matrix offset associated
 with the input GEMM, while 3 through 5 are associated with the hidden state
 GEMM.

 * paramID 0 and 3 are for the update gate.

 * paramID 1 and 4 are for the reset gate.

 * paramID 2 and 5 are for the new memory gate.

 For bi-directional RNNs the backwards in time direction is numbered as the layer
 directly after the forward in time direction.

 The output argument paramDesc is a previously created tensor descriptor that is populated
 to describe the memory layout of the parameter matrix. It is full packed and is used when
 calling to miopenSetRNNLayerParam()

 The argument layerParam should either be nullptr, or have device memory allocated
 to allow copying of the entire layer parameter matrix into it. If layerParam is
 nullptr then only the paramDesc is populated and returned. The size in bytes of the
 layer parameter matrix can be determined by using miopenGetRNNLayerParamSize().

 Note: When inputSkip mode is selected there is no input layer matrix operation,
 and therefore no associated memory. In this case miopenGetRNNLayerParam() will return
 a error status miopenStatusBadParm for input paramID associated with the input GEMM.

 @param handle          MIOpen handle (input)
 @param rnnDesc         RNN layer descriptor type (input)
 @param layer           The layer number in the RNN stack (input)
 @param xDesc           A tensor descriptor to input (input)
 @param wDesc           A tensor descriptor to the parameter tensor (input)
 @param w               Pointer to memory containing parameter tensor (input)
 @param paramID         ID of the internal parameter tensor (input)
 @param paramDesc       Tensor descriptor for the fully packed output parameter tensor (output)
 @param layerParam      Pointer to the memory location of the parameter tensor (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNLayerParam(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        layer: ::core::ffi::c_int,
        xDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        paramID: ::core::ffi::c_int,
        paramDesc: miopenTensorDescriptor_t,
        layerParam: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a bias for a specific layer in an RNN stack

 This function retrieves the bias data for a specific layer and bias ID and copies
 the data into previously allocated device memory.

 For RNN vanilla miopenRNNRELU and miopenRNNTANH, biasID == 0 retrieves the
 bias associated with the in input GEMM, while biasID == 1 retrieves
 the bias associated with the hidden state GEMM.

 For miopenLSTM biasID 0 to 3 refer to the biases associated
 with the input GEMM, 4-7 are associated with biases associated with the
 hidden state GEMM.

 * biasID 0 and 4 are for the input gate.

 * biasID 1 and 5 are for the forget gate.

 * biasID 2 and 6 are for the output gate.

 * biasID 3 and 7 are for the new memory gate.

 For miopenGRU biasID 0 to 2 refer to the biases associated with the input GEMM,
 while 3 through 5 are associated with the hidden state GEMM.

 * biasID 0 and 3 are for the update gate.

 * biasID 1 and 4 are for the reset gate.

 * biasID 2 and 5 are for the new memory gate.

 For bi-directional RNNs the backwards in time direction is numbered as the layer
 directly after the forward in time direction.

 The output argument biasDesc is a previously created tensor descriptor that is populated
 to describe the memory layout of the bias. It is full packed and is used when
 calling to miopenSetRNNLayerBias()

 The argument layerBias should either be nullptr, or have device memory allocated
 to allow copying of the entire layer bias into it. If layerBias is
 nullptr then only the biasDesc is populated and returned. The size in bytes of the
 layer bias can be determined by using miopenGetRNNLayerBiasSize().

 Note: When inputSkip mode is selected there is no input layer matrix operation,
 and therefore no associated memory. In this case miopenGetRNNLayerBias() will return
 a error status miopenStatusBadParm for input biasID associated with the input GEMM.

 @param handle          MIOpen handle (input)
 @param rnnDesc         RNN layer descriptor type (input)
 @param layer           The layer number in the RNN stack (input)
 @param xDesc           A tensor descriptor to input (input)
 @param wDesc           A tensor descriptor to the parameter tensor (input)
 @param w               Pointer to memory containing parameter tensor (input)
 @param biasID          ID of the internal parameter tensor (input)
 @param biasDesc        Descriptor of the parameter tensor (output)
 @param layerBias       Pointer to the memory location of the bias tensor (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNLayerBias(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        layer: ::core::ffi::c_int,
        xDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        biasID: ::core::ffi::c_int,
        biasDesc: miopenTensorDescriptor_t,
        layerBias: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets an index offset for a specific weight matrix for a layer in the
  RNN stack

 This function retrieves the index offset for a weight matrix in a layer.

 For RNN vanilla miopenRNNRELU and miopenRNNTANH, paramID == 0 retrieves the
 weight matrix offset associated with the in input GEMM, while paramID == 1
 retrieves the weight matrix offset associated with the hidden state GEMM.

 For miopenLSTM paramID 0 to 3 refer to the weight matrix offsets associated
 with the input GEMM, 4-7 are associated with matrix offset associated with the
 hidden state GEMM.

 * paramID 0 and 4 are for the input gate.

 * paramID 1 and 5 are for the forget gate.

 * paramID 2 and 6 are for the output gate.

 * paramID 3 and 7 are for the new memory gate.

 For miopenGRU paramID 0 to 2 refer to the weight matrix offset associated
 with the input GEMM, while 3 through 5 are associated with the hidden state
 GEMM.

 * paramID 0 and 3 are for the update gate.

 * paramID 1 and 4 are for the reset gate.

 * paramID 2 and 5 are for the new memory gate.

 For bi-directional RNNs the backwards in time direction is numbered as the layer
 directly after the forward in time direction.

 The output argument paramDesc is a previously created tensor descriptor that is populated
 to describe the memory layout of the parameter matrix. It is full packed and is used when
 calling to miopenSetRNNLayerParam().

 The argument layerParamOffset should either be nullptr, or an address to place the
 offset. If layerParamOffset is nullptr then only the paramDesc is populated and returned.

 Note: When inputSkip mode is selected there is no input layer matrix operation,
 and therefore no associated memory. In this case miopenGetRNNLayerParamOffset() will return
 a error status miopenStatusBadParm for input paramID associated with the input GEMM.


 @param rnnDesc           RNN layer descriptor type (input)
 @param layer             The layer number in the RNN stack (input)
 @param xDesc             A tensor descriptor to input (input)
 @param paramID           ID of the internal parameter tensor (input)
 @param paramDesc         Tensor descriptor for the fully packed output parameter tensor (output)
 @param layerParamOffset  Location for the parameter offset (output)
 @return                  miopenStatus_t*/
    pub fn miopenGetRNNLayerParamOffset(
        rnnDesc: miopenRNNDescriptor_t,
        layer: ::core::ffi::c_int,
        xDesc: miopenTensorDescriptor_t,
        paramID: ::core::ffi::c_int,
        paramDesc: miopenTensorDescriptor_t,
        layerParamOffset: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a bias index offset for a specific layer in an RNN stack

 This function retrieves the bias index offset for a specific layer and bias ID.

 For RNN vanilla miopenRNNRELU and miopenRNNTANH, biasID == 0 retrieves the
 bias associated with the in input GEMM, while biasID == 1 retrieves
 the weight matrix associated with the hidden state GEMM.

 For miopenLSTM biasID 0 to 3 refer to the bias offset associated
 with the input GEMM, 4-7 are the bias offsets associated with the hidden state GEMM.

 * biasID 0 and 4 are for the input gate.

 * biasID 1 and 5 are for the forget gate.

 * biasID 2 and 6 are for the output gate.

 * biasID 3 and 7 are for the new memory gate.

 For miopenGRU biasID 0 to 2 refer to the biases associated with the input GEMM,
 while 3 through 5 are associated with the hidden state GEMM.

 * biasID 0 and 3 are for the update gate.

 * biasID 1 and 4 are for the reset gate.

 * biasID 2 and 5 are for the new memory gate.

 For bi-directional RNNs the backwards in time direction is numbered as the layer
 directly after the forward in time direction.

 The output argument biasDesc is a previously created tensor descriptor that is populated
 to describe the memory layout of the bias. It is full packed and is used when
 calling to miopenSetRNNLayerBias()

 The argument layerBiasOffset should either be nullptr, or point to an output address.
 If layerBias is nullptr then only the biasDesc is populated and returned.

 Note: When inputSkip mode is selected there is no input layer matrix operation,
 and therefore no associated memory. In this case miopenGetRNNLayerBiasOffset() will return
 a error status miopenStatusBadParm for input biasID associated with the input GEMM.

 @param rnnDesc         RNN layer descriptor type (input)
 @param layer           The layer number in the RNN stack (input)
 @param xDesc           A tensor descriptor to input (input)
 @param biasID          ID of the internal parameter tensor (input)
 @param biasDesc        Descriptor of the parameter tensor (output)
 @param layerBiasOffset Pointer to the memory location of the bias tensor (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNLayerBiasOffset(
        rnnDesc: miopenRNNDescriptor_t,
        layer: ::core::ffi::c_int,
        xDesc: miopenTensorDescriptor_t,
        biasID: ::core::ffi::c_int,
        biasDesc: miopenTensorDescriptor_t,
        layerBiasOffset: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a weight matrix for a specific layer in an RNN stack

 This function sets the weight matrix data for a specific layer and parameter ID.

 For RNN vanilla miopenRNNRELU and miopenRNNTANH, paramID == 0 sets the
 weight matrix associated with the in input GEMM, while paramID == 1 sets
 the weight matrix associated with the hidden state GEMM.


 For miopenLSTM paramID 0 to 3 refer to the weight matrices associated
 with the input GEMM, 4-7 are associated with matrices associated with the
 hidden state GEMM.

 * paramID 0 and 4 are for the input gate.

 * paramID 1 and 5 are for the forget gate.

 * paramID 2 and 6 are for the output gate.

 * paramID 3 and 7 are for the new memory gate.

 For miopenGRU paramID 0 to 2 refer to the weight matrix offset associated
 with the input GEMM, while 3 through 5 are associated with the hidden state
 GEMM.

 * paramID 0 and 3 are for the update gate.

 * paramID 1 and 4 are for the reset gate.

 * paramID 2 and 5 are for the new memory gate.

 For bi-directional RNNs the backwards in time direction is numbered as the layer
 directly after the forward in time direction.

 The input argument paramDesc is a previously populated tensor descriptor typically
 by first calling miopenGetRNNLayerParam().

 Note: When inputSkip mode is selected there is no input layer matrix operation,
 and therefore no associated memory. In this case miopenSetRNNLayerParam() will return
 a error status miopenStatusBadParm for input paramID associated with the input GEMM.

 @param handle          MIOpen handle (input)
 @param rnnDesc         RNN layer descriptor type (input)
 @param layer           The layer number in the RNN stack (input)
 @param xDesc           A tensor descriptor to input (input)
 @param wDesc           A tensor descriptor to the parameter tensor (input)
 @param w               Pointer to memory containing parameter tensor (input)
 @param paramID         ID of the internal parameter tensor (input)
 @param paramDesc       Descriptor of the parameter tensor (input)
 @param layerParam      Pointer to the memory location of the parameter tensor (input)
 @return                miopenStatus_t*/
    pub fn miopenSetRNNLayerParam(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        layer: ::core::ffi::c_int,
        xDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        w: *mut ::core::ffi::c_void,
        paramID: ::core::ffi::c_int,
        paramDesc: miopenTensorDescriptor_t,
        layerParam: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a bias for a specific layer in an RNN stack

 This function sets the bias data for a specific layer and bias ID.

 For RNN vanilla miopenRNNRELU and miopenRNNTANH, biasID == 0 retrieves the
 weight matrix associated with the in input GEMM, while biasID == 1 retrieves
 the bias associated with the hidden state GEMM.

 For miopenLSTM biasID 0 to 3 refer to the biases associated
 with the input GEMM, 4-7 are associated with the biases associated with the
 hidden state GEMM.

 * biasID 0 and 4 are for the input gate.

 * biasID 1 and 5 are for the forget gate.

 * biasID 2 and 6 are for the output gate.

 * biasID 3 and 7 are for the new memory gate.

 For miopenGRU biasID 0 to 2 refer to the biases associated with the input GEMM,
 while 3 through 5 are associated with the hidden state GEMM.

 * biasID 0 and 3 are for the update gate.

 * biasID 1 and 4 are for the reset gate.

 * biasID 2 and 5 are for the new new memory gate.

 For bi-directional RNNs the backwards in time direction is numbered as the layer
 directly after the forward in time direction.

 The input argument biasDesc is a previously populated tensor descriptor typically
 by first calling miopenGetRNNLayeBias().

 Note: When inputSkip mode is selected there is no input layer matrix operation,
 and therefore no associated memory. In this case miopenSetRNNLayerBias will return
 a error status miopenStatusBadParm for input biasID associated with the input GEMM.

 @param handle          MIOpen handle (input)
 @param rnnDesc         RNN layer descriptor type (input)
 @param layer           The layer number in the RNN stack (input)
 @param xDesc           A tensor descriptor to input (input)
 @param wDesc           A tensor descriptor to the bias tensor (input)
 @param w               Pointer to memory containing bias tensor (input)
 @param biasID          ID of the internal bias tensor (input)
 @param biasDesc        Descriptor of the bias tensor (output)
 @param layerBias       Pointer to the memory location of the bias tensor (output)
 @return                miopenStatus_t*/
    pub fn miopenSetRNNLayerBias(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        layer: ::core::ffi::c_int,
        xDesc: miopenTensorDescriptor_t,
        wDesc: miopenTensorDescriptor_t,
        w: *mut ::core::ffi::c_void,
        biasID: ::core::ffi::c_int,
        biasDesc: miopenTensorDescriptor_t,
        layerBias: *const ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a bias for a specific layer in an RNN stack

 This function changes padidng mode at previously created and initialized RNN descriptor.
 This function must be called before using miopenGetRNNWorkspaceSize()
 and miopenGetRNNTrainingReserveSize() functions.
 By default, not padded data is expected at the RNN input/output.

 @param rnnDesc         RNN layer descriptor type (input/output)
 @param paddingMode     RNN input/output data padding mode (input)
 @return                miopenStatus_t*/
    pub fn miopenSetRNNPaddingMode(
        rnnDesc: miopenRNNDescriptor_t,
        paddingMode: miopenRNNPaddingMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief This function retrieves the RNN padding mode from the RNN descriptor.

 @param rnnDesc         RNN layer descriptor type (input)
 @param paddingMode     Pointer to the RNN padding mode (output)
 @return                miopenStatus_t*/
    pub fn miopenGetRNNPaddingMode(
        rnnDesc: miopenRNNDescriptor_t,
        paddingMode: *mut miopenRNNPaddingMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute forward training for recurrent layer.

 Interface for executing the forward training / inference pass on a RNN.

 @param handle                MIOpen handle (input)
 @param rnnDesc               RNN layer descriptor type (input)
 @param fwdMode          Specifies in which mode the buffers will be used.
 @param xDesc                 An input tensor descriptor for sequenced RNN data. This
 miopenSeqTensorDescriptor_t should be initialyzed by `miopenSetRNNDataSeqTensorDescriptor`
 function.(input)
 @param x                     Pointer to input tensor (input)

 @param hDesc                A hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param hx                    Pointer to the hidden layer input tensor. If hx is NULL,
 then the initial hidden state will be zero initialized. (input)
 @param hy                    Pointer to the hidden layer output tensor. If hy is NULL,
 then the final hidden state will not be saved. (output)

 @param cDesc                A cell tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param cx                    Pointer to the cell layer input tensor. If cx is NULL,
 then the initial cell state will be zero initialized. (input)
 @param cy                    Pointer to the cell layer output tensor. If hy is NULL,
 then the final cell state will not be saved. (output)

 @param yDesc                 An array of fully packed tensor descriptors associated
 with the output from each time step. The first dimension of the tensor descriptors
 must equal the first dimension of the first descriptor (batch size) in the xDesc
 tensor array. The second dimension of the element of the descriptor array
 depends on the direction mode selected. If the direction mode is unidirectional,
 the second dimension is the hiddenSize. If direction mode is bidirectional
 the second dimension is twice the hiddenSize. (input)
 @param y                     Pointer to output tensor (output)

 @param w                     Pointer to input weights tensor (input)
 @param weightSpaceSize       Number of allocated bytes in memory for the weights tensor
 @param workSpace             Pointer to memory allocated for forward (input / output)
 @param workSpaceNumBytes     Number of allocated bytes in memory for the workspace (input)
 @param reserveSpace          Pointer to memory allocated for hidden states used durning training
 (input / output)
 @param reserveSpaceNumBytes  Number of allocated bytes in memory for use in the forward  (input)
 @return                      miopenStatus_t*/
    pub fn miopenRNNForward(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        fwdMode: miopenRNNFWDMode_t,
        xDesc: miopenSeqTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hDesc: miopenTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        hy: *mut ::core::ffi::c_void,
        cDesc: miopenTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        cy: *mut ::core::ffi::c_void,
        yDesc: miopenSeqTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        w: *const ::core::ffi::c_void,
        weightSpaceSize: usize,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceNumBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceNumBytes: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute backward data for recurrent layer

 Interface for executing the backward data pass on a RNN.

 @param handle                MIOpen handle (input)
 @param rnnDesc               RNN layer descriptor type (input)

 @param yDesc                 An output tensor descriptor for sequenced RNN data. This
 miopenSeqTensorDescriptor_t should be initialyzed by `miopenSetRNNDataSeqTensorDescriptor`
function.(input)
 @param y                     Pointer to input tensor (input)
 @param dy                    Pointer to the hidden layer input tensor (input)

 @param hDesc                An input hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param hx                    Pointer to the hidden layer input tensor. If hx is NULL,
 then the initial hidden state will be zero initialized. (input)
 @param dhy                   Pointer to the cell layer input tensor (input)
 @param dhx                   Pointer to the delta hidden layer output tensor. If dhx is NULL
 the hidden gradient will not ouput. (output)

 @param cDesc                A input cell tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param cx                    Pointer to the hidden layer input tensor. If cx is NULL,
 then the initial cell state will be zero initialized. (input)
 @param dcy                   Pointer to the cell layer input tensor. If dcy is NULL,
 then the initial delta cell state will be zero initialized. (input)
 @param dcx                   Pointer to the cell layer output tensor. If dcx is NULL
 the cell gradient will not ouput. (output)

 @param xDesc                 An input tensor descriptor for sequenced RNN data. This
 miopenSeqTensorDescriptor_t should be initialyzed by `miopenSetRNNDataSeqTensorDescriptor`
function.(input)
 @param dx                    Pointer to the cell layer output tensor (output)

 @param w                     Pointer to input weights tensor (input)
 @param weightSpaceSize       Number of allocated bytes in memory for the weights tensor
 @param workSpace             Pointer to memory allocated for forward training (input)
 @param workSpaceNumBytes     Number of allocated bytes in memory for the workspace (input)
 @param reserveSpace          Pointer to memory allocated for random states (input / output)
 @param reserveSpaceNumBytes  Number of allocated bytes in memory for use in the forward (input)
 @return                      miopenStatus_t*/
    pub fn miopenRNNBackwardSeqData(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        yDesc: miopenSeqTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dy: *const ::core::ffi::c_void,
        hDesc: miopenTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        dhy: *const ::core::ffi::c_void,
        dhx: *mut ::core::ffi::c_void,
        cDesc: miopenTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        dcy: *const ::core::ffi::c_void,
        dcx: *mut ::core::ffi::c_void,
        xDesc: miopenSeqTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        w: *const ::core::ffi::c_void,
        weightSpaceSize: usize,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceNumBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceNumBytes: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute backward weights for recurrent layer

 Interface for executing the backward weights pass on a RNN.

 @param handle                MIOpen handle (input)
 @param rnnDesc               RNN layer descriptor type (input)

 @param xDesc                 An input tensor descriptor for sequenced RNN data. This
 miopenSeqTensorDescriptor_t should be initialyzed by `miopenSetRNNDataSeqTensorDescriptor`
function.(input)
 @param x                     Pointer to input tensor (input)

 @param hDesc                A hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param hx                    Pointer to the hidden layer input tensor. If hx is NULL,
 then the initial hidden state will be zero initialized. (input)

 @param yDesc                 An output tensor descriptor for sequenced RNN data. This
 miopenSeqTensorDescriptor_t should be initialyzed by `miopenSetRNNDataSeqTensorDescriptor`
function.(input)
 @param y                     Pointer to the output tensor (input)

 @param dw                    Pointer to input weights tensor (input / output)
 @param weightSpaceSize       Number of allocated bytes in memory for the weights tensor
 @param workSpace             Pointer to memory allocated for forward training (input)
 @param workSpaceNumBytes     Number of allocated bytes in memory for the workspace (input)
 @param reserveSpace          Pointer to memory allocated for random states (input)
 @param reserveSpaceNumBytes  Number of allocated bytes in memory for use in the forward (input)
 @return                      miopenStatus_t*/
    pub fn miopenRNNBackwardWeightsSeqTensor(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        xDesc: miopenSeqTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hDesc: miopenTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        yDesc: miopenSeqTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dw: *mut ::core::ffi::c_void,
        weightSpaceSize: usize,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceNumBytes: usize,
        reserveSpace: *const ::core::ffi::c_void,
        reserveSpaceNumBytes: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute forward training for recurrent layer

 Interface for executing the forward training pass on a RNN.

 @param handle                MIOpen handle (input)
 @param rnnDesc               RNN layer descriptor type (input)
 @param sequenceLen           Temporal iterations to unroll (input)
 @param xDesc                 An array of tensor descriptors. These are the
 input descriptors to each time step. The first dimension of each descriptor is the
 batch size and may decrease from element n to element n+1 and not increase in size.
 The second dimension is the same for all descriptors in the array and is the input
 vector length. (input)
 @param x                     Pointer to input tensor (input)
 @param hxDesc                A hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param hx                    Pointer to the hidden layer input tensor. If hx is NULL,
 then the initial hidden state will be zero initialized. (input)
 @param cxDesc                A cell tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param cx                    Pointer to the cell layer input tensor. If cx is NULL,
 then the initial cell state will be zero initialized. (input)
 @param wDesc                 A weights tensor descriptor (input)
 @param w                     Pointer to input weights tensor (input)
 @param yDesc                 An array of fully packed tensor descriptors associated
 with the output from each time step. The first dimension of the tensor descriptors
 must equal the first dimension of the first descriptor (batch size) in the xDesc
 tensor array. The second dimension of the element of the descriptor array
 depends on the direction mode selected. If the direction mode is unidirectional,
 the second dimension is the hiddenSize. If direction mode is bidirectional
 the second dimension is twice the hiddenSize. (input)
 @param y                     Pointer to output tensor (output)
 @param hyDesc                A hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param hy                    Pointer to the hidden layer output tensor. If hy is NULL,
 then the final hidden state will not be saved. (output)
 @param cyDesc                A cell tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param cy                    Pointer to the cell layer output tensor. If hy is NULL,
 then the final cell state will not be saved. (output)
 @param workSpace             Pointer to memory allocated for forward training (input)
 @param workSpaceNumBytes     Number of allocated bytes in memory for the workspace (input)
 @param reserveSpace          Pointer to memory allocated for random states (input / output)
 @param reserveSpaceNumBytes  Number of allocated bytes in memory for use in the forward  (input)
 @return                      miopenStatus_t*/
    pub fn miopenRNNForwardTraining(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        sequenceLen: ::core::ffi::c_int,
        xDesc: *const miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: miopenTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: miopenTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        yDesc: *const miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hyDesc: miopenTensorDescriptor_t,
        hy: *mut ::core::ffi::c_void,
        cyDesc: miopenTensorDescriptor_t,
        cy: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceNumBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceNumBytes: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute backward data for recurrent layer

 Interface for executing the backward data pass on a RNN.

 @param handle                MIOpen handle (input)
 @param rnnDesc               RNN layer descriptor type (input)
 @param sequenceLen           Temporal iterations to unroll (input)
 @param yDesc                 An array of tensor descriptors (input)
 @param y                     Pointer to input tensor (input)
 @param dyDesc                An array of fully packed tensor descriptors associated
 with the output from each time step. The first dimension of the tensor descriptors
 must equal the first dimension of the first descriptor (batch size) in the xDesc
 tensor array. The second dimension of the element of the descriptor array
 depends on the direction mode selected. If the direction mode is unidirectional,
 the second dimension is the hiddenSize. If direction mode is bidirectional
 the second dimension is twice the hiddenSize. (input)
 @param dy                    Pointer to the hidden layer input tensor (input)
 @param dhyDesc               A hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param dhy                   Pointer to the cell layer input tensor (input)
 @param dcyDesc               A cell tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param dcy                   Pointer to the cell layer input tensor. If dcy is NULL,
 then the initial delta cell state will be zero initialized. (input)
 @param wDesc                 A weights tensor descriptor (input)
 @param w                     Pointer to input weights tensor (input)
 @param hxDesc                An input hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param hx                    Pointer to the hidden layer input tensor. If hx is NULL,
 then the initial hidden state will be zero initialized. (input)
 @param cxDesc                A input cell tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param cx                    Pointer to the hidden layer input tensor. If cx is NULL,
 then the initial cell state will be zero initialized. (input)
 @param dxDesc                An array of tensor descriptors. These are the
 input descriptors to each time step. The first dimension of each descriptor is the
 batch size and may decrease from element n to element n+1 and not increase in size.
 The second dimension is the same for all descriptors in the array and is the input
 vector length. (input)
 @param dx                    Pointer to the cell layer output tensor (output)
 @param dhxDesc               A hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param dhx                   Pointer to the delta hidden layer output tensor. If dhx is NULL
 the hidden gradient will not ouput. (output)
 @param dcxDesc               A tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param dcx                   Pointer to the cell layer output tensor. If dcx is NULL
 the cell gradient will not ouput. (output)
 @param workSpace             Pointer to memory allocated for forward training (input)
 @param workSpaceNumBytes     Number of allocated bytes in memory for the workspace (input)
 @param reserveSpace          Pointer to memory allocated for random states (input / output)
 @param reserveSpaceNumBytes  Number of allocated bytes in memory for use in the forward (input)
 @return                      miopenStatus_t*/
    pub fn miopenRNNBackwardData(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        sequenceLen: ::core::ffi::c_int,
        yDesc: *const miopenTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: *const miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dhyDesc: miopenTensorDescriptor_t,
        dhy: *const ::core::ffi::c_void,
        dcyDesc: miopenTensorDescriptor_t,
        dcy: *const ::core::ffi::c_void,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        hxDesc: miopenTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: miopenTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        dxDesc: *const miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        dhxDesc: miopenTensorDescriptor_t,
        dhx: *mut ::core::ffi::c_void,
        dcxDesc: miopenTensorDescriptor_t,
        dcx: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceNumBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceNumBytes: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute backward weights for recurrent layer

 Interface for executing the backward weights pass on a RNN.

 @param handle                MIOpen handle (input)
 @param rnnDesc               RNN layer descriptor type (input)
 @param sequenceLen           Temporal iterations to unroll (input)
 @param xDesc                 An array of tensor descriptors. These are the
 input descriptors to each time step. The first dimension of each descriptor is the
 batch size and may decrease from element n to element n+1 and not increase in size.
 The second dimension is the same for all descriptors in the array and is the input
 vector length. (input)
 @param x                     Pointer to input tensor (input)
 @param hxDesc                A hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param hx                    Pointer to the hidden layer input tensor. If hx is NULL,
 then the initial hidden state will be zero initialized. (input)
 @param yDesc                 An array of fully packed tensor descriptors associated
 with the output from each time step. The first dimension of the tensor descriptors
 must equal the first dimension of the first descriptor (batch size) in the xDesc
 tensor array. The second dimension of the element of the descriptor array
 depends on the direction mode selected. If the direction mode is unidirectional,
 the second dimension is the hiddenSize. If direction mode is bidirectional
 the second dimension is twice the hiddenSize. (input)
 @param y                     Pointer to the output tensor (input)
 @param dwDesc                A weights tensor descriptor (input)
 @param dw                    Pointer to input weights tensor (input / output)
 @param workSpace             Pointer to memory allocated for forward training (input)
 @param workSpaceNumBytes     Number of allocated bytes in memory for the workspace (input)
 @param reserveSpace          Pointer to memory allocated for random states (input)
 @param reserveSpaceNumBytes  Number of allocated bytes in memory for use in the forward (input)
 @return                      miopenStatus_t*/
    pub fn miopenRNNBackwardWeights(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        sequenceLen: ::core::ffi::c_int,
        xDesc: *const miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: miopenTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        yDesc: *const miopenTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dwDesc: miopenTensorDescriptor_t,
        dw: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceNumBytes: usize,
        reserveSpace: *const ::core::ffi::c_void,
        reserveSpaceNumBytes: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute forward inference for RNN layer

 Interface for executing the forward inference pass on a RNN.

 @param handle                MIOpen handle (input)
 @param rnnDesc               RNN layer descriptor type (input)
 @param sequenceLen           Temporal iterations to unroll (input)
 @param xDesc                 An array of tensor descriptors. These are the
 input descriptors to each time step. The first dimension of each descriptor is the
 batch size and may decrease from element n to element n+1 and not increase in size.
 The second dimension is the same for all descriptors in the array and is the input
 vector length. (input)
 @param x                     Pointer to input tensor (input)
 @param hxDesc                A hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param hx                    Pointer to the hidden layer input tensor. If hx is NULL,
 then the initial hidden state will be zero initialized. (input)
 @param cxDesc                A cell tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param cx                    Pointer to the cell layer input tensor. If cx is NULL,
 then the initial cell state will be zero initialized. (input)
 @param wDesc                 A weights tensor descriptor (input)
 @param w                     Pointer to input weights tensor (input)
 @param yDesc                 An array of fully packed tensor descriptors associated
 with the output from each time step. The first dimension of the tensor descriptors
 must equal the first dimension of the first descriptor (batch size) in the xDesc
 tensor array. The second dimension of the element of the descriptor array
 depends on the direction mode selected. If the direction mode is unidirectional,
 the second dimension is the hiddenSize. If direction mode is bidirectional
 the second dimension is twice the hiddenSize. (input)
 @param y                     Pointer to output tensor (output)
 @param hyDesc                A hidden tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param hy                    Pointer to the hidden layer output tensor. If hy is NULL,
 then the final hidden state will not be saved. (output)
 @param cyDesc                A output cell tensor descriptor that has as its first dimension
 of the number of layers if the direction mode is unidirectional and twice the
 number of layers if the direction mode is bidirectional. The second dimension of
 the descriptor must equal the largest first dimension of the xDesc tensor descriptor
 array. The third dimension equals the hiddenSize. (input)
 @param cy                    Pointer to the cell layer output tensor. If cy is NULL,
 then the final cell state will not be saved. (output)
 @param workSpace             Pointer to memory allocated for forward training (input)
 @param workSpaceNumBytes     Number of allocated bytes in memory for the workspace (input)
 @return                      miopenStatus_t*/
    pub fn miopenRNNForwardInference(
        handle: miopenHandle_t,
        rnnDesc: miopenRNNDescriptor_t,
        sequenceLen: ::core::ffi::c_int,
        xDesc: *const miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: miopenTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: miopenTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        wDesc: miopenTensorDescriptor_t,
        w: *const ::core::ffi::c_void,
        yDesc: *const miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hyDesc: miopenTensorDescriptor_t,
        hy: *mut ::core::ffi::c_void,
        cyDesc: miopenTensorDescriptor_t,
        cy: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceNumBytes: usize,
    ) -> miopenStatus_t;
}
impl miopenCTCLossAlgo_t {
    ///< Results are guaranteed to be reproducible
    pub const MIOPEN_CTC_LOSS_ALGO_DETERMINISTIC: miopenCTCLossAlgo_t = miopenCTCLossAlgo_t(
        0,
    );
}
#[repr(transparent)]
/** @enum miopenCTCLossAlgo_t
 Algorithms available to execute the CTC loss operation*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenCTCLossAlgo_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create a CTC loss function Descriptor

 API for creating an uninitialized CTC loss function descriptor.
 @param ctcLossDesc  Pointer to the CTC loss function descriptor type (output)
 @return             miopenStatus_t*/
    pub fn miopenCreateCTCLossDescriptor(
        ctcLossDesc: *mut miopenCTCLossDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retrieves a CTC loss function descriptor's details

 @param ctcLossDesc          CTC loss function descriptor (input)
 @param dataType             Data type used in this CTC loss operation, only fp32 currently
 supported (output)
 @param blank_label_id       User defined index for blank label (output)
 @param apply_softmax_layer  Boolean to toggle input layer property (output)
 @return                     miopenStatus_t*/
    pub fn miopenGetCTCLossDescriptor(
        ctcLossDesc: miopenCTCLossDescriptor_t,
        dataType: *mut miopenDataType_t,
        blank_label_id: *mut ::core::ffi::c_int,
        apply_softmax_layer: *mut bool,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys a CTC loss function descriptor object

 @param ctcLossDesc  CTC loss function descriptor type (input)
 @return             miopenStatus_t*/
    pub fn miopenDestroyCTCLossDescriptor(
        ctcLossDesc: miopenCTCLossDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the details of a CTC loss function descriptor

 @param ctcLossDesc          CTC loss function descriptor type (input)
 @param dataType             Data type used in this CTC loss operation, only fp32 currently
 supported (input)
 @param blank_label_id       User defined index for blank label, default 0 (input)
 @param apply_softmax_layer  Boolean to toggle input layer property (input)
 @return             miopenStatus_t*/
    pub fn miopenSetCTCLossDescriptor(
        ctcLossDesc: miopenCTCLossDescriptor_t,
        dataType: miopenDataType_t,
        blank_label_id: ::core::ffi::c_int,
        apply_softmax_layer: bool,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the amount of memory required to execute miopenCTCLoss

 This function calculates the amount of memory required to run the CTC loss function given a CTC
 loss function descriptor with the specified algorithm.
 @param handle         MIOpen handle (input)
 @param probsDesc      Tensor descriptor for probabilities (input)
 @param gradientsDesc  Tensor descriptor for gradients (input)
 @param labels         Pointer to the flattened labels list (input)
 @param labelLengths   Pointer to the lengths list for "labels" (input)
 @param inputLengths   Pointer to the list of the time steps in each batch (input)
 @param algo           CTC loss algorithm selected (input)
 @param ctcLossDesc    CTC loss function descriptor type (input)
 @param workSpaceSize  Number of bytes of workspace required for CTC loss operation with selected
 algorithm (output)
 @return               miopenStatus_t*/
    pub fn miopenGetCTCLossWorkspaceSize(
        handle: miopenHandle_t,
        probsDesc: miopenTensorDescriptor_t,
        gradientsDesc: miopenTensorDescriptor_t,
        labels: *const ::core::ffi::c_int,
        labelLengths: *const ::core::ffi::c_int,
        inputLengths: *const ::core::ffi::c_int,
        algo: miopenCTCLossAlgo_t,
        ctcLossDesc: miopenCTCLossDescriptor_t,
        workSpaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute forward inference for CTCLoss layer

 Interface for executing the forward inference pass on a CTCLoss.
 @param handle         MIOpen handle (input)
 @param probsDesc      Tensor descriptor for probabilities (input)
 @param probs          Pointer to the probabilities tensor (input)
 @param labels         Pointer to the flattened labels list (input)
 @param labelLengths   Pointer to the lengths list for "labels" (input)
 @param inputLengths   Pointer to the list of the time steps in each batch (input)
 @param losses         Pointer to the computed losses of CTC (Output)
 @param gradientsDesc  Tensor descriptor for gradients (input)
 @param gradients      Pointer to the computed gradients of CTC (Output)
 @param algo           CTC loss algorithm selected (input)
 @param ctcLossDesc    CTC loss function descriptor type (input)
 @param workSpace      Pointer to memory allocated for execute CTC loss operation (input)
 @param workSpaceSize  Number of bytes of workspace required for CTC loss operation with selected
 algorithm (input)
 @return               miopenStatus_t*/
    pub fn miopenCTCLoss(
        handle: miopenHandle_t,
        probsDesc: miopenTensorDescriptor_t,
        probs: *const ::core::ffi::c_void,
        labels: *const ::core::ffi::c_int,
        labelLengths: *const ::core::ffi::c_int,
        inputLengths: *const ::core::ffi::c_int,
        losses: *mut ::core::ffi::c_void,
        gradientsDesc: miopenTensorDescriptor_t,
        gradients: *mut ::core::ffi::c_void,
        algo: miopenCTCLossAlgo_t,
        ctcLossDesc: miopenCTCLossDescriptor_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
    ) -> miopenStatus_t;
}
impl miopenRNGType_t {
    ///< XORWOW pseudorandom generator
    pub const MIOPEN_RNG_PSEUDO_XORWOW: miopenRNGType_t = miopenRNGType_t(0);
}
#[repr(transparent)]
/**  @enum miopenRNGType_t
 random number generator type*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenRNGType_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates the dropout descriptor object

 @param dropoutDesc Pointer to a dropout descriptor type
 @return            miopenStatus_t*/
    pub fn miopenCreateDropoutDescriptor(
        dropoutDesc: *mut miopenDropoutDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the dropout descriptor object

 @param dropoutDesc Dropout descriptor type (input)
 @return            miopenStatus_t*/
    pub fn miopenDestroyDropoutDescriptor(
        dropoutDesc: miopenDropoutDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the amount of memory required to run dropout

 This function calculates the amount of memory required to run dropout.
 @param xDesc                    Tensor descriptor for data tensor x (input)
 @param reserveSpaceSizeInBytes  Number of bytes of reservespace required for executing dropout
 (Output)
 @return                         miopenStatus_t*/
    pub fn miopenDropoutGetReserveSpaceSize(
        xDesc: miopenTensorDescriptor_t,
        reserveSpaceSizeInBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the amount of memory required to store the states of the random number generators

 This function calculates the amount of memory required to store the states of the random number
 generators used by miopenDropoutForward.
 @param handle            MIOpen handle (input)
 @param stateSizeInBytes  Number of bytes required to store random generator states (Output)
 @return                  miopenStatus_t*/
    pub fn miopenDropoutGetStatesSize(
        handle: miopenHandle_t,
        stateSizeInBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the details of the dropout descriptor

 Interface for querying the dropout descriptor
 @param dropoutDesc  Dropout layer descriptor (input)
 @param handle       MIOpen handle (input)
 @param dropout      The probability by which the input is set to 0 in the dropout layer (Output)
 @param states       Pointer to memory that holds random number generator states (Output)
 @param seed         Seed used to initialize random number generator states (Output)
 @param use_mask     Boolean flag indicating whether to use a saved mask (an existing or
 user-defined dropout layout) in reserveSpace (Output)
 @param state_evo    Boolean flag indicating whether to adopt state evolution strategy to update
 the PRNG states by the end of each implementation (Output placeholder, currently not enabled)
 @param rng_mode     Random number generator used to generate parallel random number sequences
 (Output)
 @return             miopenStatus_t*/
    pub fn miopenGetDropoutDescriptor(
        dropoutDesc: miopenDropoutDescriptor_t,
        handle: miopenHandle_t,
        dropout: *mut f32,
        states: *mut *mut ::core::ffi::c_void,
        seed: *mut ::core::ffi::c_ulonglong,
        use_mask: *mut bool,
        state_evo: *mut bool,
        rng_mode: *mut miopenRNGType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Restore the dropout descriptor to a saved state

 This function restores the state of dropout descriptor using the address of a state buffer with
 previously saved PRNG state pattern, without launching the expensive PRNG initialization process.

 Interface for restoring the dropout descriptor
 @param dropoutDesc       Dropout layer descriptor (input/Output)
 @param handle            MIOpen handle (input)
 @param dropout           The probability by which the input is set to 0 in the dropout layer
 (input)
 @param states            Pointer to memory that holds random number generator states (input)
 @param stateSizeInBytes  Number of bytes holding random generator states (input)
 @param seed              Seed used to initialize random number generator states (input)
 @param use_mask          Boolean flag indicating whether to use a saved mask (an existing or
 user-defined dropout layout) in reserveSpace (input)
 @param state_evo         Boolean flag indicating whether to adopt state evolution strategy to
 update the PRNG states by the end of each implementation (input placeholder, currently not
 enabled)
 @param rng_mode          Random number generator used to generate parallel random number
 sequences (input)
 @return                  miopenStatus_t*/
    pub fn miopenRestoreDropoutDescriptor(
        dropoutDesc: miopenDropoutDescriptor_t,
        handle: miopenHandle_t,
        dropout: f32,
        states: *mut ::core::ffi::c_void,
        stateSizeInBytes: usize,
        seed: ::core::ffi::c_ulonglong,
        use_mask: bool,
        state_evo: bool,
        rng_mode: miopenRNGType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Initialize the dropout descriptor

 Interface for setting up the dropout descriptor
 @param dropoutDesc       Dropout layer descriptor (input/Output)
 @param handle            MIOpen handle (input)
 @param dropout           The probability by which the input is set to 0 in the dropout layer
 (input)
 @param states            Pointer to memory that holds random number generator states (input)
 @param stateSizeInBytes  Number of bytes provided for random generator states (input)
 @param seed              Seed used to initialize random number generator states (input)
 @param use_mask          Boolean flag indicating whether to use a saved mask (an existing or
 user-defined dropout layout) in reserveSpace (input)
 @param state_evo         Boolean flag indicating whether to adopt state evolution strategy to
 update the PRNG states by the end of each implementation (input placeholder, currently not
 enabled)
 @param rng_mode          Random number generator used to generate parallel random number
 sequences (input)
 @return                  miopenStatus_t*/
    pub fn miopenSetDropoutDescriptor(
        dropoutDesc: miopenDropoutDescriptor_t,
        handle: miopenHandle_t,
        dropout: f32,
        states: *mut ::core::ffi::c_void,
        stateSizeInBytes: usize,
        seed: ::core::ffi::c_ulonglong,
        use_mask: bool,
        state_evo: bool,
        rng_mode: miopenRNGType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute forward dropout operation

 Interface for executing the forward pass on a Dropout.
 @param handle                   MIOpen handle (input)
 @param dropoutDesc              Dropout layer descriptor (input)
 @param noise_shape              Tensor descriptor for noise shape (input placeholder, currently
 not enabled)
 @param xDesc                    Tensor descriptor for data tensor x (input)
 @param x                        Data tensor x (input)
 @param yDesc                    Tensor descriptor for data tensor y (input)
 @param y                        Data tensor y (Output)
 @param reserveSpace             Pointer to memory allocated for executing forward dropout,
 expecting reserveSpace unchanged before next call of miopenDropoutBackward (Output)
 @param reserveSpaceSizeInBytes  Number of bytes of reservespace required for executing forward
 dropout (input)
 @return                         miopenStatus_t*/
    pub fn miopenDropoutForward(
        handle: miopenHandle_t,
        dropoutDesc: miopenDropoutDescriptor_t,
        noise_shape: miopenTensorDescriptor_t,
        xDesc: miopenTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: miopenTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Execute backward dropout operation

 Interface for executing the backward pass on a Dropout.
 @param handle                   MIOpen handle (input)
 @param dropoutDesc              Dropout layer descriptor (input)
 @param noise_shape              Tensor descriptor for noise shape (input placeholder, currently
 not enabled)
 @param dyDesc                   Tensor descriptor for data delta tensor dy (input)
 @param dy                       Data delta tensor dy (input)
 @param dxDesc                   Tensor descriptor for data delta tensor dx (input)
 @param dx                       Data delta tensor dx (Output)
 @param reserveSpace             Pointer to memory allocated for executing backward dropout,
 expecting reserveSpace unchanged after previous call of miopenDropoutForward (input)
 @param reserveSpaceSizeInBytes  Number of bytes of reservespace required for executing backward
 dropout (input)
 @return                         miopenStatus_t*/
    pub fn miopenDropoutBackward(
        handle: miopenHandle_t,
        dropoutDesc: miopenDropoutDescriptor_t,
        noise_shape: miopenTensorDescriptor_t,
        dyDesc: miopenTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dxDesc: miopenTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates the ReduceTensor descriptor object

 @param reduceTensorDesc Pointer to a ReduceTensor descriptor type
 @return            miopenStatus_t*/
    pub fn miopenCreateReduceTensorDescriptor(
        reduceTensorDesc: *mut miopenReduceTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroy the ReduceTensor descriptor object

 @param reduceTensorDesc  ReduceTensor descriptor type (input)
 @return            miopenStatus_t*/
    pub fn miopenDestroyReduceTensorDescriptor(
        reduceTensorDesc: miopenReduceTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Initialize a ReduceTensor descriptor object

 @param reduceTensorDesc         Pointer to the ReduceTensor descriptor object (output)
 @param reduceTensorOp           Enumerant specifying the operation used by ReduceTensor (input)
 @param reduceTensorCompType     Enumerant specifying the data type used with ReduceTensor
 operation (input)
 @param reduceTensorNanOpt       Enumerant specifying the Nan number propagation mode (input)
 @param reduceTensorIndices      Enumerant specifying the indices modes used by ReduceTensor
 (input)
 @param reduceTensorIndicesType  Enumerant specifying the data type of the indices (input)
 @return           miopenStatus_t*/
    pub fn miopenSetReduceTensorDescriptor(
        reduceTensorDesc: miopenReduceTensorDescriptor_t,
        reduceTensorOp: miopenReduceTensorOp_t,
        reduceTensorCompType: miopenDataType_t,
        reduceTensorNanOpt: miopenNanPropagation_t,
        reduceTensorIndices: miopenReduceTensorIndices_t,
        reduceTensorIndicesType: miopenIndicesType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query a ReduceTensor descriptor object

 @param reduceTensorDesc         Pointer to the ReduceTensor descriptor object (input)
 @param reduceTensorOp           Pointer to enumerant specifying the operation used by
 ReduceTensor (output)
 @param reduceTensorCompType     Pointer to enumerant specifying the data type used with
 ReduceTensor operation (output)
 @param reduceTensorNanOpt       Pointer to enumerant specifying the Nan number propagation mode
 (output)
 @param reduceTensorIndices      Pointer to enumerant specifying the indices modes used by
 ReduceTensor (output)
 @param reduceTensorIndicesType  Pointer to enumerant specifying the data type of the indices
 (output)
 @return           miopenStatus_t*/
    pub fn miopenGetReduceTensorDescriptor(
        reduceTensorDesc: miopenReduceTensorDescriptor_t,
        reduceTensorOp: *mut miopenReduceTensorOp_t,
        reduceTensorCompType: *mut miopenDataType_t,
        reduceTensorNanOpt: *mut miopenNanPropagation_t,
        reduceTensorIndices: *mut miopenReduceTensorIndices_t,
        reduceTensorIndicesType: *mut miopenIndicesType_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Helper function to query the minimum index space size required by the ReduceTensor call

 @param handle                   MIOpen Handle (input)
 @param reduceTensorDesc         Pointer to the ReduceTensor descriptor object (input)
 @param aDesc                    Pointer to the input tensor descriptor (input)
 @param cDesc                    Pointer to the output tensor descriptor (input)
 @param sizeInBytes              Pointer to data to return the minimum index space size
 @return           miopenStatus_t*/
    pub fn miopenGetReductionIndicesSize(
        handle: miopenHandle_t,
        reduceTensorDesc: miopenReduceTensorDescriptor_t,
        aDesc: miopenTensorDescriptor_t,
        cDesc: miopenTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Helper function to query the minimum workspace size required by the ReduceTensor call

 @param handle                   MIOpen Handle (input)
 @param reduceTensorDesc         Pointer to the ReduceTensor descriptor object (input)
 @param aDesc                    Pointer to the input tensor descriptor (input)
 @param cDesc                    Pointer to the output tensor descriptor (input)
 @param sizeInBytes              Pointer to data to return the minimum workspace size
 @return           miopenStatus_t*/
    pub fn miopenGetReductionWorkspaceSize(
        handle: miopenHandle_t,
        reduceTensorDesc: miopenReduceTensorDescriptor_t,
        aDesc: miopenTensorDescriptor_t,
        cDesc: miopenTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief TensorReduce function doing reduction on tensor A by implementing C = alpha * reduceOp(A)
 + beta * C

 The length of each dimension of output tensor C must match the length of the corresponding
 dimension of
 input tensor A or must be equal to 1. The dimensions with length equal to 1 indicate the
 dimensions
 of A to be reduced.

 @param handle                   MIOpen Handle (input)
 @param reduceTensorDesc         Pointer to the ReduceTensor descriptor object (input)
 @param indices                  Address of the allocated indices data space (output)
 @param indicesSizeInBytes       Size in bytes of the allocated indices data space (input)
 @param workspace                Address of the allocated workspace data (input)
 @param workspaceSizeInBytes     Size in bytes of the allocated workspace data (input)
 @param alpha                    Pointer to scale factor for data in input tensor A (input)
 @param aDesc                    Pointer to the tensor descriptor for input tensor A (input)
 @param A                        Pointer to the data of input tensor A (input)
 @param beta                     Pointer to scale factor for data in output tensor C (input)
 @param cDesc                    Pointer to the tensor descriptor for output tensor C (input)
 @param C                        Pointer to the data of output tensor C (output)
 @return           miopenStatus_t*/
    pub fn miopenReduceTensor(
        handle: miopenHandle_t,
        reduceTensorDesc: miopenReduceTensorDescriptor_t,
        indices: *mut ::core::ffi::c_void,
        indicesSizeInBytes: usize,
        workspace: *mut ::core::ffi::c_void,
        workspaceSizeInBytes: usize,
        alpha: *const ::core::ffi::c_void,
        aDesc: miopenTensorDescriptor_t,
        A: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        cDesc: miopenTensorDescriptor_t,
        C: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenProblem {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenProblem_t(pub *mut miopenProblem);
impl miopenProblemDirection_t {
    pub const miopenProblemDirectionForward: miopenProblemDirection_t = miopenProblemDirection_t(
        0,
    );
}
impl miopenProblemDirection_t {
    pub const miopenProblemDirectionBackward: miopenProblemDirection_t = miopenProblemDirection_t(
        1,
    );
}
impl miopenProblemDirection_t {
    pub const miopenProblemDirectionBackwardWeights: miopenProblemDirection_t = miopenProblemDirection_t(
        2,
    );
}
#[repr(transparent)]
/** @enum miopenProblemDirection_t
 Directions of miopen operation.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenProblemDirection_t(pub ::core::ffi::c_uint);
impl miopenTensorArgumentId_t {
    pub const miopenTensorArgumentIdInvalid: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        0,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorConvolutionX: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        1,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorConvolutionW: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        2,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorConvolutionY: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        3,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaK: miopenTensorArgumentId_t = miopenTensorArgumentId_t(4);
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaQ: miopenTensorArgumentId_t = miopenTensorArgumentId_t(5);
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaV: miopenTensorArgumentId_t = miopenTensorArgumentId_t(6);
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDescaleK: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        7,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDescaleQ: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        8,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDescaleV: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        9,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDescaleS: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        10,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaScaleS: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        11,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaScaleO: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        12,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDropoutProbability: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        13,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDropoutSeed: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        14,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDropoutOffset: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        15,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaO: miopenTensorArgumentId_t = miopenTensorArgumentId_t(16);
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaAmaxO: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        17,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaAmaxS: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        18,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaM: miopenTensorArgumentId_t = miopenTensorArgumentId_t(19);
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaZInv: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        20,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDO: miopenTensorArgumentId_t = miopenTensorArgumentId_t(21);
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDescaleO: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        22,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDescaleDO: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        23,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDescaleDS: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        24,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaScaleDS: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        25,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaScaleDQ: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        26,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaScaleDK: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        27,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaScaleDV: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        28,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDQ: miopenTensorArgumentId_t = miopenTensorArgumentId_t(29);
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDK: miopenTensorArgumentId_t = miopenTensorArgumentId_t(30);
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaDV: miopenTensorArgumentId_t = miopenTensorArgumentId_t(31);
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaAmaxDQ: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        32,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaAmaxDK: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        33,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaAmaxDV: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        34,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaAmaxDS: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        35,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaBias: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        36,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorArgumentIsScalar: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        2147483648,
    );
}
impl miopenTensorArgumentId_t {
    pub const miopenTensorMhaMask: miopenTensorArgumentId_t = miopenTensorArgumentId_t(
        2147483649,
    );
}
#[repr(transparent)]
/** @enum miopenTensorArgumentId_t
 Identifiers for tensor arguments of problems and operations.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenTensorArgumentId_t(pub ::core::ffi::c_uint);
impl miopenFindResultsOrder_t {
    pub const miopenFindResultsOrderByTime: miopenFindResultsOrder_t = miopenFindResultsOrder_t(
        0,
    );
}
impl miopenFindResultsOrder_t {
    pub const miopenFindResultsOrderByWorkspaceSize: miopenFindResultsOrder_t = miopenFindResultsOrder_t(
        1,
    );
}
#[repr(transparent)]
/** @enum miopenTensorArgumentId_t
 Different ways to sort results of the find call.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenFindResultsOrder_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Initializes a problem object describing a convolution operation.

 @param problem      Pointer to the problem to initialize
 @param operatorDesc Descriptor of the operator to be used
 @param direction    Direction of the operation
 @return             miopenStatus_t*/
    pub fn miopenCreateConvProblem(
        problem: *mut miopenProblem_t,
        operatorDesc: miopenConvolutionDescriptor_t,
        direction: miopenProblemDirection_t,
    ) -> miopenStatus_t;
}
impl miopenMhaMask_t {
    pub const miopenMhaMaskNone: miopenMhaMask_t = miopenMhaMask_t(0);
}
impl miopenMhaMask_t {
    pub const miopenMhaMaskCausal: miopenMhaMask_t = miopenMhaMask_t(1);
}
#[repr(transparent)]
/** @enum miopenMhaMask_t
 Different masks for Mha.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenMhaMask_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn miopenCreateMhaProblem(
        problem: *mut miopenProblem_t,
        operatorDesc: miopenMhaDescriptor_t,
        direction: miopenProblemDirection_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates the mha descriptor object

 @param mhaDesc     Pointer to a mha descriptor type
 @return            miopenStatus_t*/
    pub fn miopenCreateMhaDescriptor(
        mhaDesc: *mut miopenMhaDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the Mha descriptor details

 Sets all of the descriptor details for the Mha

 @param mhaDesc               Pointer to a Mha descriptor
 @param scale                 Scale
 @return                      miopenStatus_t*/
    pub fn miopenSetMhaDescriptor(
        mhaDesc: miopenMhaDescriptor_t,
        scale: f32,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the Mha descriptor details

 Retrieves all of the descriptor details for the Mha.

 @param mhaDesc       Pointer to a Mha descriptor
 @param scale         Scale (output)
 @return              miopenStatus_t*/
    pub fn miopenGetMhaDescriptor(
        mhaDesc: miopenMhaDescriptor_t,
        scale: *mut f32,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates the Softmax descriptor object

 @param softmaxDesc Pointer to an softmax descriptor type
 @return            miopenStatus_t*/
    pub fn miopenCreateSoftmaxDescriptor(
        softmaxDesc: *mut miopenSoftmaxDescriptor_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the softmax descriptor details

 Sets all of the descriptor details for the softmax layer

 @param softmaxDesc  Pointer to a softmax layer descriptor
 @param alpha        Softmax alpha parameter
 @param beta         Softmax beta parameter
 @param algorithm    Softmax algorithm
 @param mode         Softmax mode
 @return             miopenStatus_t*/
    pub fn miopenSetSoftmaxDescriptor(
        softmaxDesc: miopenSoftmaxDescriptor_t,
        alpha: f32,
        beta: f32,
        algorithm: miopenSoftmaxAlgorithm_t,
        mode: miopenSoftmaxMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the softmax layer descriptor details

 Retrieves all of the descriptor details for the softmax layer.

 @param softmaxDesc   Pointer to a softmax layer descriptor (input)
 @param alpha         Softmax alpha parameter (output)
 @param beta          Softmax beta parameter (output)
 @param algorithm     Softmax algorithm (output)
 @param mode          Softmax mode (output)
 @return              miopenStatus_t*/
    pub fn miopenGetSoftmaxDescriptor(
        softmaxDesc: miopenSoftmaxDescriptor_t,
        alpha: *mut f32,
        beta: *mut f32,
        algorithm: *mut miopenSoftmaxAlgorithm_t,
        mode: *mut miopenSoftmaxMode_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys a problem object.

 @param problem Problem to destroy
 @return        miopenStatus_t*/
    pub fn miopenDestroyProblem(problem: miopenProblem_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a tensor descriptor for the specified argument.

 @param problem    Problem to update
 @param id         Id of the argument for the descriptor
 @param descriptor Tensor descriptor to set
 @return           miopenStatus_t*/
    pub fn miopenSetProblemTensorDescriptor(
        problem: miopenProblem_t,
        id: miopenTensorArgumentId_t,
        descriptor: miopenTensorDescriptor_t,
    ) -> miopenStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenFindOptions {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenFindOptions_t(pub *mut miopenFindOptions);
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Initializes miopenFindOptions object.

 @param options    Pointer to options object to initialze
 @return           miopenStatus_t*/
    pub fn miopenCreateFindOptions(options: *mut miopenFindOptions_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys miopenFindOptions object.

 @param options    Options object to destroy
 @return           miopenStatus_t*/
    pub fn miopenDestroyFindOptions(options: miopenFindOptions_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the tuning find option. Default value is zero.

 @param options    Options object to update
 @param value      Value of zero means no tuning, value of one means tuning enabled
 @return           miopenStatus_t*/
    pub fn miopenSetFindOptionTuning(
        options: miopenFindOptions_t,
        value: ::core::ffi::c_int,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the results order find option. Default value is miopenFindResultsOrderByTime.

 @param options    Options object to update
 @param value      Specifies what order should find results have
 @return           miopenStatus_t*/
    pub fn miopenSetFindOptionResultsOrder(
        options: miopenFindOptions_t,
        value: miopenFindResultsOrder_t,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the workspace limit find option. Default value is maximum of size_t

 @param options    Options object to update
 @param value      Specifies the workspace limit for find call. All solvers exceeding the limit
 would be ignored.
 @return           miopenStatus_t*/
    pub fn miopenSetFindOptionWorkspaceLimit(
        options: miopenFindOptions_t,
        value: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Attaches the preallocated workspace to find options. Allocated by the library by default.

 @param options    Options object to update
 @param buffer     Specifies the workspace for find call
 @param size       Specifies the size of the buffer passed
 @return           miopenStatus_t*/
    pub fn miopenSetFindOptionPreallocatedWorkspace(
        options: miopenFindOptions_t,
        buffer: *mut ::core::ffi::c_void,
        size: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Attaches a preallocated tensor to find options. If not used, buffers are allocated by
 MIOpen internally, which is not recommended.

 @param options    Options object to update
 @param id         Specifies the id of the tensor passed
 @param buffer     Specifies the tensor for find call
 @return           miopenStatus_t*/
    pub fn miopenSetFindOptionPreallocatedTensor(
        options: miopenFindOptions_t,
        id: miopenTensorArgumentId_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Forces library to attach kernel binaries to solutions for later saving. This allows zero
 lookup miopenRunSolution calls after miopenLoadSolution. Default value is 0.

 @param options    Options object to update
 @param attach     1 means attaching, 0 - skipping, any other value - reserved for future use
 @return           miopenStatus_t*/
    pub fn miopenSetFindOptionAttachBinaries(
        options: miopenFindOptions_t,
        attach: ::core::ffi::c_uint,
    ) -> miopenStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenSolution {
    pub _address: u8,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenSolution_t(pub *mut miopenSolution);
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Finds solutions to a problem by running different applicable solutions. Memory is
 automatically allocated.

 @param handle       Handle to execute the kernels
 @param problem      Problem to solve
 @param options      Find options. When null default values would be used
 @param solutions    Pointer to the first result. Must not be null
 @param numSolutions Pointer to the amount of results. Ignored if null
 @param maxSolutions Limits the amount of results
 @return             miopenStatus_t*/
    pub fn miopenFindSolutions(
        handle: miopenHandle_t,
        problem: miopenProblem_t,
        options: miopenFindOptions_t,
        solutions: *mut miopenSolution_t,
        numSolutions: *mut usize,
        maxSolutions: usize,
    ) -> miopenStatus_t;
}
/// @brief Values of a tensor or scalar argument for the miopenRunSolution function.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct miopenTensorArgument_t {
    pub id: miopenTensorArgumentId_t,
    pub descriptor: *mut miopenTensorDescriptor_t,
    pub buffer: *mut ::core::ffi::c_void,
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Runs the solution using the passed in buffers.

 @param handle        Handle to execute the kernels
 @param solution      Solution to execute
 @param nInputs       Amount to inputs for the solution
 @param tensors       Tensor arguments described by miopenTensorArgument_t
 @param workspace     Pointer to device buffer used as workspace. May be null when not required.
 Should not be less than expected
 @param workspaceSize Size of the workspace buffer
 @return              miopenStatus_t*/
    pub fn miopenRunSolution(
        handle: miopenHandle_t,
        solution: miopenSolution_t,
        nInputs: usize,
        tensors: *const miopenTensorArgument_t,
        workspace: *mut ::core::ffi::c_void,
        workspaceSize: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys solution object.

 @param solution   Solution to destroy
 @return           miopenStatus_t*/
    pub fn miopenDestroySolution(solution: miopenSolution_t) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Loads solution object from binary data.

 @param solution   Pointer to the solution to load
 @param data       Data to load the solution from
 @param size       Size of the solution blob
 @return           miopenStatus_t*/
    pub fn miopenLoadSolution(
        solution: *mut miopenSolution_t,
        data: *const ::core::ffi::c_char,
        size: usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Saves a solution object as binary data.

 @param solution   Solution to save
 @param data       Pointer to a buffer to save soltuion to
 @return           miopenStatus_t*/
    pub fn miopenSaveSolution(
        solution: miopenSolution_t,
        data: *mut ::core::ffi::c_char,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Reads the expected size of a solution.

 @param solution   Solution to get size
 @param size       Pointer to a location where to write the size of the solution blob
 @return           miopenStatus_t*/
    pub fn miopenGetSolutionSize(
        solution: miopenSolution_t,
        size: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Reads the amount of workspace required to exectute the solution.

 @param solution      Solution to get required workspace size
 @param workspaceSize Pointer to a location where to write the workspace size
 @return              miopenStatus_t*/
    pub fn miopenGetSolutionWorkspaceSize(
        solution: miopenSolution_t,
        workspaceSize: *mut usize,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Reads the time spent to execute the solution the last it was run.

 @param solution Solution to get exection time
 @param time     Pointer to a location where to write the execution time
 @return         miopenStatus_t*/
    pub fn miopenGetSolutionTime(
        solution: miopenSolution_t,
        time: *mut f32,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Reads id of the solver referred by the solution.

 @param solution Solution to get solver id from
 @param solverId Pointer to a location where to write the solver id
 @return         miopenStatus_t*/
    pub fn miopenGetSolutionSolverId(
        solution: miopenSolution_t,
        solverId: *mut u64,
    ) -> miopenStatus_t;
}
#[cfg_attr(windows, link(name = "MIOpen", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the convolution algorithm implemented by a solver.

 @param solverId Solver id to get convolution algorithm of
 @param result   Pointer to a location where to write the algorithm
 @return         miopenStatus_t*/
    pub fn miopenGetSolverIdConvAlgorithm(
        solverId: u64,
        result: *mut miopenConvAlgorithm_t,
    ) -> miopenStatus_t;
}
impl miopenError_t {
    pub const r#NotInitialized: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const r#InvalidValue: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const r#BadParm: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const r#AllocFailed: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const r#InternalError: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const r#NotImplemented: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const r#UnknownError: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const r#UnsupportedOp: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const r#GpuOperationsSkipped: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(9)
    });
    pub const r#VersionMismatch: miopenError_t = miopenError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(10)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct miopenError_t(pub ::core::num::NonZeroU32);
pub trait miopenStatus_tConsts {
    const Success: miopenStatus_t = miopenStatus_t::Ok(());
    const ErrorNotInitialized: miopenStatus_t = miopenStatus_t::Err(
        miopenError_t::r#NotInitialized,
    );
    const ErrorInvalidValue: miopenStatus_t = miopenStatus_t::Err(
        miopenError_t::r#InvalidValue,
    );
    const ErrorBadParm: miopenStatus_t = miopenStatus_t::Err(miopenError_t::r#BadParm);
    const ErrorAllocFailed: miopenStatus_t = miopenStatus_t::Err(
        miopenError_t::r#AllocFailed,
    );
    const ErrorInternalError: miopenStatus_t = miopenStatus_t::Err(
        miopenError_t::r#InternalError,
    );
    const ErrorNotImplemented: miopenStatus_t = miopenStatus_t::Err(
        miopenError_t::r#NotImplemented,
    );
    const ErrorUnknownError: miopenStatus_t = miopenStatus_t::Err(
        miopenError_t::r#UnknownError,
    );
    const ErrorUnsupportedOp: miopenStatus_t = miopenStatus_t::Err(
        miopenError_t::r#UnsupportedOp,
    );
    const ErrorGpuOperationsSkipped: miopenStatus_t = miopenStatus_t::Err(
        miopenError_t::r#GpuOperationsSkipped,
    );
    const ErrorVersionMismatch: miopenStatus_t = miopenStatus_t::Err(
        miopenError_t::r#VersionMismatch,
    );
}
impl miopenStatus_tConsts for miopenStatus_t {}
#[must_use]
pub type miopenStatus_t = ::core::result::Result<(), miopenError_t>;
const _: fn() = || {
    let _ = std::mem::transmute::<miopenStatus_t, u32>;
};
unsafe impl Send for miopenHandle {}
unsafe impl Sync for miopenHandle {}
unsafe impl Send for miopenFusionOpDescriptor_t {}
unsafe impl Sync for miopenFusionOpDescriptor_t {}
unsafe impl Send for miopenTensorDescriptor_t {}
unsafe impl Sync for miopenTensorDescriptor_t {}
unsafe impl Send for miopenSeqTensorDescriptor_t {}
unsafe impl Sync for miopenSeqTensorDescriptor_t {}
unsafe impl Send for miopenConvolutionDescriptor_t {}
unsafe impl Sync for miopenConvolutionDescriptor_t {}
unsafe impl Send for miopenPoolingDescriptor_t {}
unsafe impl Sync for miopenPoolingDescriptor_t {}
unsafe impl Send for miopenLRNDescriptor_t {}
unsafe impl Sync for miopenLRNDescriptor_t {}
unsafe impl Send for miopenActivationDescriptor_t {}
unsafe impl Sync for miopenActivationDescriptor_t {}
unsafe impl Send for miopenRNNDescriptor_t {}
unsafe impl Sync for miopenRNNDescriptor_t {}
unsafe impl Send for miopenCTCLossDescriptor_t {}
unsafe impl Sync for miopenCTCLossDescriptor_t {}
unsafe impl Send for miopenDropoutDescriptor_t {}
unsafe impl Sync for miopenDropoutDescriptor_t {}
unsafe impl Send for miopenReduceTensorDescriptor_t {}
unsafe impl Sync for miopenReduceTensorDescriptor_t {}
unsafe impl Send for miopenMhaDescriptor_t {}
unsafe impl Sync for miopenMhaDescriptor_t {}
unsafe impl Send for miopenSoftmaxDescriptor_t {}
unsafe impl Sync for miopenSoftmaxDescriptor_t {}
unsafe impl Send for miopenFusionPlanDescriptor_t {}
unsafe impl Sync for miopenFusionPlanDescriptor_t {}
unsafe impl Send for miopenOperatorDescriptor_t {}
unsafe impl Sync for miopenOperatorDescriptor_t {}
unsafe impl Send for miopenOperatorArgs_t {}
unsafe impl Sync for miopenOperatorArgs_t {}
unsafe impl Send for miopenProblem_t {}
unsafe impl Sync for miopenProblem_t {}
unsafe impl Send for miopenFindOptions_t {}
unsafe impl Sync for miopenFindOptions_t {}
unsafe impl Send for miopenSolution_t {}
unsafe impl Sync for miopenSolution_t {}
