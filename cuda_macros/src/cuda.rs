// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
extern "system" {
    /** \brief Gets the string description of an error code

 Sets \p *pStr to the address of a NULL-terminated string description
 of the error code \p error.
 If the error code is not recognized, ::CUDA_ERROR_INVALID_VALUE
 will be returned and \p *pStr will be set to the NULL address.

 \param error - Error code to convert to string
 \param pStr - Address of the string pointer.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::CUresult,
 ::cudaGetErrorString*/
    fn cuGetErrorString(
        error: cuda_types::cuda::CUresult,
        pStr: *mut *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the string representation of an error code enum name

 Sets \p *pStr to the address of a NULL-terminated string representation
 of the name of the enum error code \p error.
 If the error code is not recognized, ::CUDA_ERROR_INVALID_VALUE
 will be returned and \p *pStr will be set to the NULL address.

 \param error - Error code to convert to string
 \param pStr - Address of the string pointer.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::CUresult,
 ::cudaGetErrorName*/
    fn cuGetErrorName(
        error: cuda_types::cuda::CUresult,
        pStr: *mut *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Initialize the CUDA driver API
 Initializes the driver API and must be called before any other function from
 the driver API in the current process. Currently, the \p Flags parameter must be 0. If ::cuInit()
 has not been called, any function from the driver API will return
 ::CUDA_ERROR_NOT_INITIALIZED.

 \param Flags - Initialization flag for CUDA.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_SYSTEM_DRIVER_MISMATCH,
 ::CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE
 \notefnerr*/
    fn cuInit(Flags: ::core::ffi::c_uint) -> cuda_types::cuda::CUresult;
    /** \brief Returns the latest CUDA version supported by driver

 Returns in \p *driverVersion the version of CUDA supported by
 the driver.  The version is returned as
 (1000 * major + 10 * minor). For example, CUDA 9.2
 would be represented by 9020.

 This function automatically returns ::CUDA_ERROR_INVALID_VALUE if
 \p driverVersion is NULL.

 \param driverVersion - Returns the CUDA driver version

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::cudaDriverGetVersion,
 ::cudaRuntimeGetVersion*/
    fn cuDriverGetVersion(
        driverVersion: *mut ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a handle to a compute device

 Returns in \p *device a device handle given an ordinal in the range <b>[0,
 ::cuDeviceGetCount()-1]</b>.

 \param device  - Returned device handle
 \param ordinal - Device number to get handle for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGetUuid,
 ::cuDeviceGetLuid,
 ::cuDeviceTotalMem,
 ::cuDeviceGetExecAffinitySupport*/
    fn cuDeviceGet(
        device: *mut cuda_types::cuda::CUdevice,
        ordinal: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the number of compute-capable devices

 Returns in \p *count the number of devices with compute capability greater
 than or equal to 2.0 that are available for execution. If there is no such
 device, ::cuDeviceGetCount() returns 0.

 \param count - Returned number of compute-capable devices

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetName,
 ::cuDeviceGetUuid,
 ::cuDeviceGetLuid,
 ::cuDeviceGet,
 ::cuDeviceTotalMem,
 ::cuDeviceGetExecAffinitySupport,
 ::cudaGetDeviceCount*/
    fn cuDeviceGetCount(count: *mut ::core::ffi::c_int) -> cuda_types::cuda::CUresult;
    /** \brief Returns an identifier string for the device

 Returns an ASCII string identifying the device \p dev in the NULL-terminated
 string pointed to by \p name. \p len specifies the maximum length of the
 string that may be returned. \p name is shortened to the specified \p len, if \p len is less than the device name

 \param name - Returned identifier string for the device
 \param len  - Maximum length of string to store in \p name
 \param dev  - Device to get identifier string for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetUuid,
 ::cuDeviceGetLuid,
 ::cuDeviceGetCount,
 ::cuDeviceGet,
 ::cuDeviceTotalMem,
 ::cuDeviceGetExecAffinitySupport,
 ::cudaGetDeviceProperties*/
    fn cuDeviceGetName(
        name: *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Return an UUID for the device

 Note there is a later version of this API, ::cuDeviceGetUuid_v2. It will
 supplant this version in 12.0, which is retained for minor version compatibility.

 Returns 16-octets identifying the device \p dev in the structure
 pointed by the \p uuid.

 \param uuid - Returned UUID
 \param dev  - Device to get identifier string for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetUuid_v2
 ::cuDeviceGetAttribute,
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGetLuid,
 ::cuDeviceGet,
 ::cuDeviceTotalMem,
 ::cuDeviceGetExecAffinitySupport,
 ::cudaGetDeviceProperties*/
    fn cuDeviceGetUuid(
        uuid: *mut cuda_types::cuda::CUuuid,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Return an UUID for the device (11.4+)

 Returns 16-octets identifying the device \p dev in the structure
 pointed by the \p uuid. If the device is in MIG mode, returns its
 MIG UUID which uniquely identifies the subscribed MIG compute instance.

 \param uuid - Returned UUID
 \param dev  - Device to get identifier string for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGetLuid,
 ::cuDeviceGet,
 ::cuDeviceTotalMem,
 ::cudaGetDeviceProperties*/
    fn cuDeviceGetUuid_v2(
        uuid: *mut cuda_types::cuda::CUuuid,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Return an LUID and device node mask for the device

 Return identifying information (\p luid and \p deviceNodeMask) to allow
 matching device with graphics APIs.

 \param luid - Returned LUID
 \param deviceNodeMask - Returned device node mask
 \param dev  - Device to get identifier string for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGet,
 ::cuDeviceTotalMem,
 ::cuDeviceGetExecAffinitySupport,
 ::cudaGetDeviceProperties*/
    fn cuDeviceGetLuid(
        luid: *mut ::core::ffi::c_char,
        deviceNodeMask: *mut ::core::ffi::c_uint,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the total amount of memory on the device

 Returns in \p *bytes the total amount of memory available on the device
 \p dev in bytes.

 \param bytes - Returned memory available on device in bytes
 \param dev   - Device handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGetUuid,
 ::cuDeviceGet,
 ::cuDeviceGetExecAffinitySupport,
 ::cudaMemGetInfo*/
    fn cuDeviceTotalMem_v2(
        bytes: *mut usize,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the maximum number of elements allocatable in a 1D linear texture for a given texture element size.

 Returns in \p maxWidthInElements the maximum number of texture elements allocatable in a 1D linear texture
 for given \p format and \p numChannels.

 \param maxWidthInElements    - Returned maximum number of texture elements allocatable for given \p format and \p numChannels.
 \param format                - Texture format.
 \param numChannels           - Number of channels per texture element.
 \param dev                   - Device handle.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGetUuid,
 ::cuDeviceGet,
 ::cudaMemGetInfo,
 ::cuDeviceTotalMem*/
    fn cuDeviceGetTexture1DLinearMaxWidth(
        maxWidthInElements: *mut usize,
        format: cuda_types::cuda::CUarray_format,
        numChannels: ::core::ffi::c_uint,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns information about the device

 Returns in \p *pi the integer value of the attribute \p attrib on device
 \p dev. The supported attributes are:
 - ::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK: Maximum number of threads per
   block;
 - ::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X: Maximum x-dimension of a block
 - ::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y: Maximum y-dimension of a block
 - ::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z: Maximum z-dimension of a block
 - ::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X: Maximum x-dimension of a grid
 - ::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y: Maximum y-dimension of a grid
 - ::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z: Maximum z-dimension of a grid
 - ::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK: Maximum amount of
   shared memory available to a thread block in bytes
 - ::CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY: Memory available on device for
   __constant__ variables in a CUDA C kernel in bytes
 - ::CU_DEVICE_ATTRIBUTE_WARP_SIZE: Warp size in threads
 - ::CU_DEVICE_ATTRIBUTE_MAX_PITCH: Maximum pitch in bytes allowed by the
   memory copy functions that involve memory regions allocated through
   ::cuMemAllocPitch()
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH: Maximum 1D
  texture width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH: Maximum width
  for a 1D texture bound to linear memory
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH: Maximum
  mipmapped 1D texture width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_WIDTH: Maximum 2D
  texture width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_HEIGHT: Maximum 2D
  texture height
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH: Maximum width
  for a 2D texture bound to linear memory
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT: Maximum height
  for a 2D texture bound to linear memory
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH: Maximum pitch
  in bytes for a 2D texture bound to linear memory
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_WIDTH: Maximum
  mipmapped 2D texture width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_HEIGHT: Maximum
  mipmapped 2D texture height
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH: Maximum 3D
  texture width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT: Maximum 3D
  texture height
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH: Maximum 3D
  texture depth
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH_ALTERNATE:
  Alternate maximum 3D texture width, 0 if no alternate
  maximum 3D texture size is supported
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT_ALTERNATE:
  Alternate maximum 3D texture height, 0 if no alternate
  maximum 3D texture size is supported
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH_ALTERNATE:
  Alternate maximum 3D texture depth, 0 if no alternate
  maximum 3D texture size is supported
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_WIDTH:
  Maximum cubemap texture width or height
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_WIDTH:
  Maximum 1D layered texture width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_LAYERS:
   Maximum layers in a 1D layered texture
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_WIDTH:
  Maximum 2D layered texture width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_HEIGHT:
   Maximum 2D layered texture height
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_LAYERS:
   Maximum layers in a 2D layered texture
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_WIDTH:
   Maximum cubemap layered texture width or height
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_LAYERS:
   Maximum layers in a cubemap layered texture
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_WIDTH:
   Maximum 1D surface width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_WIDTH:
   Maximum 2D surface width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_HEIGHT:
   Maximum 2D surface height
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_WIDTH:
   Maximum 3D surface width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_HEIGHT:
   Maximum 3D surface height
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_DEPTH:
   Maximum 3D surface depth
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_WIDTH:
   Maximum 1D layered surface width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_LAYERS:
   Maximum layers in a 1D layered surface
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_WIDTH:
   Maximum 2D layered surface width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_HEIGHT:
   Maximum 2D layered surface height
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_LAYERS:
   Maximum layers in a 2D layered surface
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_WIDTH:
   Maximum cubemap surface width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_WIDTH:
   Maximum cubemap layered surface width
 - ::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_LAYERS:
   Maximum layers in a cubemap layered surface
 - ::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK: Maximum number of 32-bit
   registers available to a thread block
 - ::CU_DEVICE_ATTRIBUTE_CLOCK_RATE: The typical clock frequency in kilohertz
 - ::CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT: Alignment requirement; texture
   base addresses aligned to ::textureAlign bytes do not need an offset
   applied to texture fetches
 - ::CU_DEVICE_ATTRIBUTE_TEXTURE_PITCH_ALIGNMENT: Pitch alignment requirement
   for 2D texture references bound to pitched memory
 - ::CU_DEVICE_ATTRIBUTE_GPU_OVERLAP: 1 if the device can concurrently copy
   memory between host and device while executing a kernel, or 0 if not
 - ::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT: Number of multiprocessors on
   the device
 - ::CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT: 1 if there is a run time limit
   for kernels executed on the device, or 0 if not
 - ::CU_DEVICE_ATTRIBUTE_INTEGRATED: 1 if the device is integrated with the
   memory subsystem, or 0 if not
 - ::CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY: 1 if the device can map host
   memory into the CUDA address space, or 0 if not
 - ::CU_DEVICE_ATTRIBUTE_COMPUTE_MODE: Compute mode that device is currently
   in. Available modes are as follows:
   - ::CU_COMPUTEMODE_DEFAULT: Default mode - Device is not restricted and
     can have multiple CUDA contexts present at a single time.
   - ::CU_COMPUTEMODE_PROHIBITED: Compute-prohibited mode - Device is
     prohibited from creating new CUDA contexts.
   - ::CU_COMPUTEMODE_EXCLUSIVE_PROCESS:  Compute-exclusive-process mode - Device
     can have only one context used by a single process at a time.
 - ::CU_DEVICE_ATTRIBUTE_CONCURRENT_KERNELS: 1 if the device supports
   executing multiple kernels within the same context simultaneously, or 0 if
   not. It is not guaranteed that multiple kernels will be resident
   on the device concurrently so this feature should not be relied upon for
   correctness.
 - ::CU_DEVICE_ATTRIBUTE_ECC_ENABLED: 1 if error correction is enabled on the
    device, 0 if error correction is disabled or not supported by the device
 - ::CU_DEVICE_ATTRIBUTE_PCI_BUS_ID: PCI bus identifier of the device
 - ::CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID: PCI device (also known as slot) identifier
   of the device
 - ::CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID: PCI domain identifier of the device
 - ::CU_DEVICE_ATTRIBUTE_TCC_DRIVER: 1 if the device is using a TCC driver. TCC
    is only available on Tesla hardware running Windows Vista or later
 - ::CU_DEVICE_ATTRIBUTE_MEMORY_CLOCK_RATE: Peak memory clock frequency in kilohertz
 - ::CU_DEVICE_ATTRIBUTE_GLOBAL_MEMORY_BUS_WIDTH: Global memory bus width in bits
 - ::CU_DEVICE_ATTRIBUTE_L2_CACHE_SIZE: Size of L2 cache in bytes. 0 if the device doesn't have L2 cache
 - ::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR: Maximum resident threads per multiprocessor
 - ::CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING: 1 if the device shares a unified address space with
   the host, or 0 if not
 - ::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR: Major compute capability version number
 - ::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR: Minor compute capability version number
 - ::CU_DEVICE_ATTRIBUTE_GLOBAL_L1_CACHE_SUPPORTED: 1 if device supports caching globals
    in L1 cache, 0 if caching globals in L1 cache is not supported by the device
 - ::CU_DEVICE_ATTRIBUTE_LOCAL_L1_CACHE_SUPPORTED: 1 if device supports caching locals
    in L1 cache, 0 if caching locals in L1 cache is not supported by the device
 - ::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR: Maximum amount of
   shared memory available to a multiprocessor in bytes; this amount is shared
   by all thread blocks simultaneously resident on a multiprocessor
 - ::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR: Maximum number of 32-bit
   registers available to a multiprocessor; this number is shared by all thread
   blocks simultaneously resident on a multiprocessor
 - ::CU_DEVICE_ATTRIBUTE_MANAGED_MEMORY: 1 if device supports allocating managed memory
   on this system, 0 if allocating managed memory is not supported by the device on this system.
 - ::CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD: 1 if device is on a multi-GPU board, 0 if not.
 - ::CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD_GROUP_ID: Unique identifier for a group of devices
   associated with the same board. Devices on the same multi-GPU board will share the same identifier.
 - ::CU_DEVICE_ATTRIBUTE_HOST_NATIVE_ATOMIC_SUPPORTED: 1 if Link between the device and the host
   supports native atomic operations.
 - ::CU_DEVICE_ATTRIBUTE_SINGLE_TO_DOUBLE_PRECISION_PERF_RATIO: Ratio of single precision performance
   (in floating-point operations per second) to double precision performance.
 - ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS: Device supports coherently accessing
   pageable memory without calling cudaHostRegister on it.
 - ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS: Device can coherently access managed memory
   concurrently with the CPU.
 - ::CU_DEVICE_ATTRIBUTE_COMPUTE_PREEMPTION_SUPPORTED: Device supports Compute Preemption.
 - ::CU_DEVICE_ATTRIBUTE_CAN_USE_HOST_POINTER_FOR_REGISTERED_MEM: Device can access host registered
   memory at the same virtual address as the CPU.
 -  ::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN: The maximum per block shared memory size
    supported on this device. This is the maximum value that can be opted into when using the cuFuncSetAttribute() or cuKernelSetAttribute() call.
    For more details see ::CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES
 - ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES: Device accesses pageable memory via the host's
   page tables.
 - ::CU_DEVICE_ATTRIBUTE_DIRECT_MANAGED_MEM_ACCESS_FROM_HOST: The host can directly access managed memory on the device without migration.
 - ::CU_DEVICE_ATTRIBUTE_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED:  Device supports virtual memory management APIs like ::cuMemAddressReserve, ::cuMemCreate, ::cuMemMap and related APIs
 - ::CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR_SUPPORTED: Device supports exporting memory to a posix file descriptor with ::cuMemExportToShareableHandle, if requested via ::cuMemCreate
 - ::CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_HANDLE_SUPPORTED:  Device supports exporting memory to a Win32 NT handle with ::cuMemExportToShareableHandle, if requested via ::cuMemCreate
 - ::CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_KMT_HANDLE_SUPPORTED: Device supports exporting memory to a Win32 KMT handle with ::cuMemExportToShareableHandle, if requested via ::cuMemCreate
 - ::CU_DEVICE_ATTRIBUTE_MAX_BLOCKS_PER_MULTIPROCESSOR: Maximum number of thread blocks that can reside on a multiprocessor
 - ::CU_DEVICE_ATTRIBUTE_GENERIC_COMPRESSION_SUPPORTED: Device supports compressible memory allocation via ::cuMemCreate
 - ::CU_DEVICE_ATTRIBUTE_MAX_PERSISTING_L2_CACHE_SIZE: Maximum L2 persisting lines capacity setting in bytes
 - ::CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE: Maximum value of CUaccessPolicyWindow::num_bytes
 - ::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WITH_CUDA_VMM_SUPPORTED: Device supports specifying the GPUDirect RDMA flag with ::cuMemCreate.
 - ::CU_DEVICE_ATTRIBUTE_RESERVED_SHARED_MEMORY_PER_BLOCK: Amount of shared memory per block reserved by CUDA driver in bytes
 - ::CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED: Device supports sparse CUDA arrays and sparse CUDA mipmapped arrays.
 - ::CU_DEVICE_ATTRIBUTE_READ_ONLY_HOST_REGISTER_SUPPORTED: Device supports using the ::cuMemHostRegister flag ::CU_MEMHOSTERGISTER_READ_ONLY to register memory that must be mapped as read-only to the GPU
 - ::CU_DEVICE_ATTRIBUTE_MEMORY_POOLS_SUPPORTED: Device supports using the ::cuMemAllocAsync and ::cuMemPool family of APIs
 - ::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_SUPPORTED: Device supports GPUDirect RDMA APIs, like nvidia_p2p_get_pages (see https://docs.nvidia.com/cuda/gpudirect-rdma for more information)
 - ::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_FLUSH_WRITES_OPTIONS: The returned attribute shall be interpreted as a bitmask, where the individual bits are described by the ::CUflushGPUDirectRDMAWritesOptions enum
 - ::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WRITES_ORDERING: GPUDirect RDMA writes to the device do not need to be flushed for consumers within the scope indicated by the returned attribute. See ::CUGPUDirectRDMAWritesOrdering for the numerical values returned here.
 - ::CU_DEVICE_ATTRIBUTE_MEMPOOL_SUPPORTED_HANDLE_TYPES: Bitmask of handle types supported with mempool based IPC
 - ::CU_DEVICE_ATTRIBUTE_DEFERRED_MAPPING_CUDA_ARRAY_SUPPORTED: Device supports deferred mapping CUDA arrays and CUDA mipmapped arrays.
 - ::CU_DEVICE_ATTRIBUTE_NUMA_CONFIG: NUMA configuration of a device: value is of type ::CUdeviceNumaConfig enum
 - ::CU_DEVICE_ATTRIBUTE_NUMA_ID: NUMA node ID of the GPU memory
 - ::CU_DEVICE_ATTRIBUTE_MULTICAST_SUPPORTED: Device supports switch multicast and reduction operations.
 - ::CU_DEVICE_ATTRIBUTE_GPU_PCI_DEVICE_ID: The combined 16-bit PCI device ID and 16-bit PCI vendor ID.
 - ::CU_DEVICE_ATTRIBUTE_GPU_PCI_SUBSYSTEM_ID: The combined 16-bit PCI subsystem ID and 16-bit PCI subsystem vendor ID.
ID.
 - ::CU_DEVICE_ATTRIBUTE_HOST_NUMA_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED: Device supports HOST_NUMA location with the virtual memory management APIs like ::cuMemCreate, ::cuMemMap and related APIs
 - ::CU_DEVICE_ATTRIBUTE_HOST_NUMA_MEMORY_POOLS_SUPPORTED: Device supports HOST_NUMA location with the ::cuMemAllocAsync and ::cuMemPool family of APIs

 \param pi     - Returned device attribute value
 \param attrib - Device attribute to query
 \param dev    - Device handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGetUuid,
 ::cuDeviceGet,
 ::cuDeviceTotalMem,
 ::cuDeviceGetExecAffinitySupport,
 ::cudaDeviceGetAttribute,
 ::cudaGetDeviceProperties*/
    fn cuDeviceGetAttribute(
        pi: *mut ::core::ffi::c_int,
        attrib: cuda_types::cuda::CUdevice_attribute,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Return NvSciSync attributes that this device can support.

 Returns in \p nvSciSyncAttrList, the properties of NvSciSync that
 this CUDA device, \p dev can support. The returned \p nvSciSyncAttrList
 can be used to create an NvSciSync object that matches this device's capabilities.

 If NvSciSyncAttrKey_RequiredPerm field in \p nvSciSyncAttrList is
 already set this API will return ::CUDA_ERROR_INVALID_VALUE.

 The applications should set \p nvSciSyncAttrList to a valid
 NvSciSyncAttrList failing which this API will return
 ::CUDA_ERROR_INVALID_HANDLE.

 The \p flags controls how applications intends to use
 the NvSciSync created from the \p nvSciSyncAttrList. The valid flags are:
 - ::CUDA_NVSCISYNC_ATTR_SIGNAL, specifies that the applications intends to
 signal an NvSciSync on this CUDA device.
 - ::CUDA_NVSCISYNC_ATTR_WAIT, specifies that the applications intends to
 wait on an NvSciSync on this CUDA device.

 At least one of these flags must be set, failing which the API
 returns ::CUDA_ERROR_INVALID_VALUE. Both the flags are orthogonal
 to one another: a developer may set both these flags that allows to
 set both wait and signal specific attributes in the same \p nvSciSyncAttrList.

 Note that this API updates the input \p nvSciSyncAttrList with values equivalent
 to the following public attribute key-values:
 NvSciSyncAttrKey_RequiredPerm is set to
 - NvSciSyncAccessPerm_SignalOnly if ::CUDA_NVSCISYNC_ATTR_SIGNAL is set in \p flags.
 - NvSciSyncAccessPerm_WaitOnly if ::CUDA_NVSCISYNC_ATTR_WAIT is set in \p flags.
 - NvSciSyncAccessPerm_WaitSignal if both ::CUDA_NVSCISYNC_ATTR_WAIT and
 ::CUDA_NVSCISYNC_ATTR_SIGNAL are set in \p flags.
 NvSciSyncAttrKey_PrimitiveInfo is set to
 - NvSciSyncAttrValPrimitiveType_SysmemSemaphore on any valid \p device.
 - NvSciSyncAttrValPrimitiveType_Syncpoint if \p device is a Tegra device.
 - NvSciSyncAttrValPrimitiveType_SysmemSemaphorePayload64b if \p device is GA10X+.
 NvSciSyncAttrKey_GpuId is set to the same UUID that is returned for this
 \p device from ::cuDeviceGetUuid.

 \param nvSciSyncAttrList     - Return NvSciSync attributes supported.
 \param dev                   - Valid Cuda Device to get NvSciSync attributes for.
 \param flags                 - flags describing NvSciSync usage.

 \return

 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa
 ::cuImportExternalSemaphore,
 ::cuDestroyExternalSemaphore,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync*/
    fn cuDeviceGetNvSciSyncAttributes(
        nvSciSyncAttrList: *mut ::core::ffi::c_void,
        dev: cuda_types::cuda::CUdevice,
        flags: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the current memory pool of a device

 The memory pool must be local to the specified device.
 ::cuMemAllocAsync allocates from the current mempool of the provided stream's device.
 By default, a device's current memory pool is its default memory pool.

 \note Use ::cuMemAllocFromPoolAsync to specify asynchronous allocations from a device different
 than the one the stream runs on.

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuDeviceGetDefaultMemPool, ::cuDeviceGetMemPool, ::cuMemPoolCreate, ::cuMemPoolDestroy, ::cuMemAllocFromPoolAsync*/
    fn cuDeviceSetMemPool(
        dev: cuda_types::cuda::CUdevice,
        pool: cuda_types::cuda::CUmemoryPool,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the current mempool for a device

 Returns the last pool provided to ::cuDeviceSetMemPool for this device
 or the device's default memory pool if ::cuDeviceSetMemPool has never been called.
 By default the current mempool is the default mempool for a device.
 Otherwise the returned pool must have been set with ::cuDeviceSetMemPool.

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuDeviceGetDefaultMemPool, ::cuMemPoolCreate, ::cuDeviceSetMemPool*/
    fn cuDeviceGetMemPool(
        pool: *mut cuda_types::cuda::CUmemoryPool,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the default mempool of a device

 The default mempool of a device contains device memory from that device.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuMemAllocAsync, ::cuMemPoolTrimTo, ::cuMemPoolGetAttribute, ::cuMemPoolSetAttribute, cuMemPoolSetAccess, ::cuDeviceGetMemPool, ::cuMemPoolCreate*/
    fn cuDeviceGetDefaultMemPool(
        pool_out: *mut cuda_types::cuda::CUmemoryPool,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns information about the execution affinity support of the device.

 Returns in \p *pi whether execution affinity type \p type is supported by device \p dev.
 The supported types are:
 - ::CU_EXEC_AFFINITY_TYPE_SM_COUNT: 1 if context with limited SMs is supported by the device,
   or 0 if not;

 \param pi   - 1 if the execution affinity type \p type is supported by the device, or 0 if not
 \param type - Execution affinity type to query
 \param dev  - Device handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGetUuid,
 ::cuDeviceGet,
 ::cuDeviceTotalMem*/
    fn cuDeviceGetExecAffinitySupport(
        pi: *mut ::core::ffi::c_int,
        type_: cuda_types::cuda::CUexecAffinityType,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Blocks until remote writes are visible to the specified scope

 Blocks until GPUDirect RDMA writes to the target context via mappings
 created through APIs like nvidia_p2p_get_pages (see
 https://docs.nvidia.com/cuda/gpudirect-rdma for more information), are
 visible to the specified scope.

 If the scope equals or lies within the scope indicated by
 ::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WRITES_ORDERING, the call
 will be a no-op and can be safely omitted for performance. This can be
 determined by comparing the numerical values between the two enums, with
 smaller scopes having smaller values.

 On platforms that support GPUDirect RDMA writes via more than one path in
 hardware (see ::CU_MEM_RANGE_FLAG_DMA_BUF_MAPPING_TYPE_PCIE), the user should
 consider those paths as belonging to separate ordering domains. Note that in
 such cases CUDA driver will report both RDMA writes ordering and RDMA write
 scope as ALL_DEVICES and a call to cuFlushGPUDirectRDMA will be a no-op,
 but when these multiple paths are used simultaneously, it is the user's
 responsibility to ensure ordering by using mechanisms outside the scope of
 CUDA.

 Users may query support for this API via
 ::CU_DEVICE_ATTRIBUTE_FLUSH_FLUSH_GPU_DIRECT_RDMA_OPTIONS.

 \param target - The target of the operation, see ::CUflushGPUDirectRDMAWritesTarget
 \param scope  - The scope of the operation, see ::CUflushGPUDirectRDMAWritesScope

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr
*/
    fn cuFlushGPUDirectRDMAWrites(
        target: cuda_types::cuda::CUflushGPUDirectRDMAWritesTarget,
        scope: cuda_types::cuda::CUflushGPUDirectRDMAWritesScope,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns properties for a selected device

 \deprecated

 This function was deprecated as of CUDA 5.0 and replaced by ::cuDeviceGetAttribute().

 Returns in \p *prop the properties of device \p dev. The ::CUdevprop
 structure is defined as:

 \code
typedef struct CUdevprop_st {
int maxThreadsPerBlock;
int maxThreadsDim[3];
int maxGridSize[3];
int sharedMemPerBlock;
int totalConstantMemory;
int SIMDWidth;
int memPitch;
int regsPerBlock;
int clockRate;
int textureAlign
} CUdevprop;
 \endcode
 where:

 - ::maxThreadsPerBlock is the maximum number of threads per block;
 - ::maxThreadsDim[3] is the maximum sizes of each dimension of a block;
 - ::maxGridSize[3] is the maximum sizes of each dimension of a grid;
 - ::sharedMemPerBlock is the total amount of shared memory available per
   block in bytes;
 - ::totalConstantMemory is the total amount of constant memory available on
   the device in bytes;
 - ::SIMDWidth is the warp size;
 - ::memPitch is the maximum pitch allowed by the memory copy functions that
   involve memory regions allocated through ::cuMemAllocPitch();
 - ::regsPerBlock is the total number of registers available per block;
 - ::clockRate is the clock frequency in kilohertz;
 - ::textureAlign is the alignment requirement; texture base addresses that
   are aligned to ::textureAlign bytes do not need an offset applied to
   texture fetches.

 \param prop - Returned properties of device
 \param dev  - Device to get properties for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGetUuid,
 ::cuDeviceGet,
 ::cuDeviceTotalMem*/
    fn cuDeviceGetProperties(
        prop: *mut cuda_types::cuda::CUdevprop,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the compute capability of the device

 \deprecated

 This function was deprecated as of CUDA 5.0 and its functionality superseded
 by ::cuDeviceGetAttribute().

 Returns in \p *major and \p *minor the major and minor revision numbers that
 define the compute capability of the device \p dev.

 \param major - Major revision number
 \param minor - Minor revision number
 \param dev   - Device handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGetAttribute,
 ::cuDeviceGetCount,
 ::cuDeviceGetName,
 ::cuDeviceGetUuid,
 ::cuDeviceGet,
 ::cuDeviceTotalMem*/
    fn cuDeviceComputeCapability(
        major: *mut ::core::ffi::c_int,
        minor: *mut ::core::ffi::c_int,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Retain the primary context on the GPU

 Retains the primary context on the device.
 Once the user successfully retains the primary context, the primary context
 will be active and available to the user until the user releases it
 with ::cuDevicePrimaryCtxRelease() or resets it with ::cuDevicePrimaryCtxReset().
 Unlike ::cuCtxCreate() the newly retained context is not pushed onto the stack.

 Retaining the primary context for the first time will fail with ::CUDA_ERROR_UNKNOWN
 if the compute mode of the device is ::CU_COMPUTEMODE_PROHIBITED. The function
 ::cuDeviceGetAttribute() can be used with ::CU_DEVICE_ATTRIBUTE_COMPUTE_MODE to
 determine the compute mode  of the device.
 The <i>nvidia-smi</i> tool can be used to set the compute mode for
 devices. Documentation for <i>nvidia-smi</i> can be obtained by passing a
 -h option to it.

 Please note that the primary context always supports pinned allocations. Other
 flags can be specified by ::cuDevicePrimaryCtxSetFlags().

 \param pctx  - Returned context handle of the new context
 \param dev   - Device for which primary context is requested

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuDevicePrimaryCtxRelease,
 ::cuDevicePrimaryCtxSetFlags,
 ::cuCtxCreate,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize*/
    fn cuDevicePrimaryCtxRetain(
        pctx: *mut cuda_types::cuda::CUcontext,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Release the primary context on the GPU

 Releases the primary context interop on the device.
 A retained context should always be released once the user is done using
 it. The context is automatically reset once the last reference to it is
 released. This behavior is different when the primary context was retained
 by the CUDA runtime from CUDA 4.0 and earlier. In this case, the primary
 context remains always active.

 Releasing a primary context that has not been previously retained will
 fail with ::CUDA_ERROR_INVALID_CONTEXT.

 Please note that unlike ::cuCtxDestroy() this method does not pop the context
 from stack in any circumstances.

 \param dev - Device which primary context is released

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa ::cuDevicePrimaryCtxRetain,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize*/
    fn cuDevicePrimaryCtxRelease_v2(
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Set flags for the primary context

 Sets the flags for the primary context on the device overwriting perviously
 set ones.

 The three LSBs of the \p flags parameter can be used to control how the OS
 thread, which owns the CUDA context at the time of an API call, interacts
 with the OS scheduler when waiting for results from the GPU. Only one of
 the scheduling flags can be set when creating a context.

 - ::CU_CTX_SCHED_SPIN: Instruct CUDA to actively spin when waiting for
 results from the GPU. This can decrease latency when waiting for the GPU,
 but may lower the performance of CPU threads if they are performing work in
 parallel with the CUDA thread.

 - ::CU_CTX_SCHED_YIELD: Instruct CUDA to yield its thread when waiting for
 results from the GPU. This can increase latency when waiting for the GPU,
 but can increase the performance of CPU threads performing work in parallel
 with the GPU.

 - ::CU_CTX_SCHED_BLOCKING_SYNC: Instruct CUDA to block the CPU thread on a
 synchronization primitive when waiting for the GPU to finish work.

 - ::CU_CTX_BLOCKING_SYNC: Instruct CUDA to block the CPU thread on a
 synchronization primitive when waiting for the GPU to finish work. <br>
 <b>Deprecated:</b> This flag was deprecated as of CUDA 4.0 and was
 replaced with ::CU_CTX_SCHED_BLOCKING_SYNC.

 - ::CU_CTX_SCHED_AUTO: The default value if the \p flags parameter is zero,
 uses a heuristic based on the number of active CUDA contexts in the
 process \e C and the number of logical processors in the system \e P. If
 \e C > \e P, then CUDA will yield to other OS threads when waiting for
 the GPU (::CU_CTX_SCHED_YIELD), otherwise CUDA will not yield while
 waiting for results and actively spin on the processor (::CU_CTX_SCHED_SPIN).
 Additionally, on Tegra devices, ::CU_CTX_SCHED_AUTO uses a heuristic based on
 the power profile of the platform and may choose ::CU_CTX_SCHED_BLOCKING_SYNC
 for low-powered devices.

 - ::CU_CTX_LMEM_RESIZE_TO_MAX: Instruct CUDA to not reduce local memory
 after resizing local memory for a kernel. This can prevent thrashing by
 local memory allocations when launching many kernels with high local
 memory usage at the cost of potentially increased memory usage. <br>
 <b>Deprecated:</b> This flag is deprecated and the behavior enabled
 by this flag is now the default and cannot be disabled.

 - ::CU_CTX_COREDUMP_ENABLE: If GPU coredumps have not been enabled globally
 with ::cuCoredumpSetAttributeGlobal or environment variables, this flag can
 be set during context creation to instruct CUDA to create a coredump if
 this context raises an exception during execution. These environment variables
 are described in the CUDA-GDB user guide under the "GPU core dump support"
 section.
 The initial settings will be taken from the global settings at the time of
 context creation. The other settings that control coredump output can be
 modified by calling ::cuCoredumpSetAttribute from the created context after
 it becomes current.

 - ::CU_CTX_USER_COREDUMP_ENABLE: If user-triggered GPU coredumps have not
 been enabled globally with ::cuCoredumpSetAttributeGlobal or environment
 variables, this flag can be set during context creation to instruct CUDA to
 create a coredump if data is written to a certain pipe that is present in the
 OS space. These environment variables are described in the CUDA-GDB user
 guide under the "GPU core dump support" section.
 It is important to note that the pipe name *must* be set with
 ::cuCoredumpSetAttributeGlobal before creating the context if this flag is
 used. Setting this flag implies that ::CU_CTX_COREDUMP_ENABLE is set.
 The initial settings will be taken from the global settings at the time of
 context creation. The other settings that control coredump output can be
 modified by calling ::cuCoredumpSetAttribute from the created context after
 it becomes current.

 - ::CU_CTX_SYNC_MEMOPS: Ensures that synchronous memory operations initiated
 on this context will always synchronize. See further documentation in the
 section titled "API Synchronization behavior" to learn more about cases when
 synchronous memory operations can exhibit asynchronous behavior.

 \param dev   - Device for which the primary context flags are set
 \param flags - New flags for the device

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr

 \sa ::cuDevicePrimaryCtxRetain,
 ::cuDevicePrimaryCtxGetState,
 ::cuCtxCreate,
 ::cuCtxGetFlags,
 ::cuCtxSetFlags,
 ::cudaSetDeviceFlags*/
    fn cuDevicePrimaryCtxSetFlags_v2(
        dev: cuda_types::cuda::CUdevice,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get the state of the primary context

 Returns in \p *flags the flags for the primary context of \p dev, and in
 \p *active whether it is active.  See ::cuDevicePrimaryCtxSetFlags for flag
 values.

 \param dev    - Device to get primary context flags for
 \param flags  - Pointer to store flags
 \param active - Pointer to store context state; 0 = inactive, 1 = active

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr

 \sa
 ::cuDevicePrimaryCtxSetFlags,
 ::cuCtxGetFlags,
 ::cuCtxSetFlags,
 ::cudaGetDeviceFlags*/
    fn cuDevicePrimaryCtxGetState(
        dev: cuda_types::cuda::CUdevice,
        flags: *mut ::core::ffi::c_uint,
        active: *mut ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroy all allocations and reset all state on the primary context

 Explicitly destroys and cleans up all resources associated with the current
 device in the current process.

 Note that it is responsibility of the calling function to ensure that no
 other module in the process is using the device any more. For that reason
 it is recommended to use ::cuDevicePrimaryCtxRelease() in most cases.
 However it is safe for other modules to call ::cuDevicePrimaryCtxRelease()
 even after resetting the device.
 Resetting the primary context does not release it, an application that has
 retained the primary context should explicitly release its usage.

 \param dev - Device for which primary context is destroyed

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE
 \notefnerr

 \sa ::cuDevicePrimaryCtxRetain,
 ::cuDevicePrimaryCtxRelease,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize,
 ::cudaDeviceReset*/
    fn cuDevicePrimaryCtxReset_v2(
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a CUDA context

 \note In most cases it is recommended to use ::cuDevicePrimaryCtxRetain.

 Creates a new CUDA context and associates it with the calling thread. The
 \p flags parameter is described below. The context is created with a usage
 count of 1 and the caller of ::cuCtxCreate() must call ::cuCtxDestroy()
 when done using the context. If a context is already current to the thread,
 it is supplanted by the newly created context and may be restored by a subsequent
 call to ::cuCtxPopCurrent().

 The three LSBs of the \p flags parameter can be used to control how the OS
 thread, which owns the CUDA context at the time of an API call, interacts
 with the OS scheduler when waiting for results from the GPU. Only one of
 the scheduling flags can be set when creating a context.

 - ::CU_CTX_SCHED_SPIN: Instruct CUDA to actively spin when waiting for
 results from the GPU. This can decrease latency when waiting for the GPU,
 but may lower the performance of CPU threads if they are performing work in
 parallel with the CUDA thread.

 - ::CU_CTX_SCHED_YIELD: Instruct CUDA to yield its thread when waiting for
 results from the GPU. This can increase latency when waiting for the GPU,
 but can increase the performance of CPU threads performing work in parallel
 with the GPU.

 - ::CU_CTX_SCHED_BLOCKING_SYNC: Instruct CUDA to block the CPU thread on a
 synchronization primitive when waiting for the GPU to finish work.

 - ::CU_CTX_BLOCKING_SYNC: Instruct CUDA to block the CPU thread on a
 synchronization primitive when waiting for the GPU to finish work. <br>
 <b>Deprecated:</b> This flag was deprecated as of CUDA 4.0 and was
 replaced with ::CU_CTX_SCHED_BLOCKING_SYNC.

 - ::CU_CTX_SCHED_AUTO: The default value if the \p flags parameter is zero,
 uses a heuristic based on the number of active CUDA contexts in the
 process \e C and the number of logical processors in the system \e P. If
 \e C > \e P, then CUDA will yield to other OS threads when waiting for
 the GPU (::CU_CTX_SCHED_YIELD), otherwise CUDA will not yield while
 waiting for results and actively spin on the processor (::CU_CTX_SCHED_SPIN).
 Additionally, on Tegra devices, ::CU_CTX_SCHED_AUTO uses a heuristic based on
 the power profile of the platform and may choose ::CU_CTX_SCHED_BLOCKING_SYNC
 for low-powered devices.

 - ::CU_CTX_MAP_HOST: Instruct CUDA to support mapped pinned allocations.
 This flag must be set in order to allocate pinned host memory that is
 accessible to the GPU.

 - ::CU_CTX_LMEM_RESIZE_TO_MAX: Instruct CUDA to not reduce local memory
 after resizing local memory for a kernel. This can prevent thrashing by
 local memory allocations when launching many kernels with high local
 memory usage at the cost of potentially increased memory usage. <br>
 <b>Deprecated:</b> This flag is deprecated and the behavior enabled
 by this flag is now the default and cannot be disabled.
 Instead, the per-thread stack size can be controlled with ::cuCtxSetLimit().

 - ::CU_CTX_COREDUMP_ENABLE: If GPU coredumps have not been enabled globally
 with ::cuCoredumpSetAttributeGlobal or environment variables, this flag can
 be set during context creation to instruct CUDA to create a coredump if
 this context raises an exception during execution. These environment variables
 are described in the CUDA-GDB user guide under the "GPU core dump support"
 section.
 The initial attributes will be taken from the global attributes at the time of
 context creation. The other attributes that control coredump output can be
 modified by calling ::cuCoredumpSetAttribute from the created context after
 it becomes current.

 - ::CU_CTX_USER_COREDUMP_ENABLE: If user-triggered GPU coredumps have not
 been enabled globally with ::cuCoredumpSetAttributeGlobal or environment
 variables, this flag can be set during context creation to instruct CUDA to
 create a coredump if data is written to a certain pipe that is present in the
 OS space. These environment variables are described in the CUDA-GDB user
 guide under the "GPU core dump support" section.
 It is important to note that the pipe name *must* be set with
 ::cuCoredumpSetAttributeGlobal before creating the context if this flag is
 used. Setting this flag implies that ::CU_CTX_COREDUMP_ENABLE is set.
 The initial attributes will be taken from the global attributes at the time of
 context creation. The other attributes that control coredump output can be
 modified by calling ::cuCoredumpSetAttribute from the created context after
 it becomes current.
 Setting this flag on any context creation is equivalent to setting the
 ::CU_COREDUMP_ENABLE_USER_TRIGGER attribute to \p true globally.

 - ::CU_CTX_SYNC_MEMOPS: Ensures that synchronous memory operations initiated
 on this context will always synchronize. See further documentation in the
 section titled "API Synchronization behavior" to learn more about cases when
 synchronous memory operations can exhibit asynchronous behavior.

 Context creation will fail with ::CUDA_ERROR_UNKNOWN if the compute mode of
 the device is ::CU_COMPUTEMODE_PROHIBITED. The function ::cuDeviceGetAttribute()
 can be used with ::CU_DEVICE_ATTRIBUTE_COMPUTE_MODE to determine the
 compute mode of the device. The <i>nvidia-smi</i> tool can be used to set
 the compute mode for * devices.
 Documentation for <i>nvidia-smi</i> can be obtained by passing a
 -h option to it.

 \param pctx  - Returned context handle of the new context
 \param flags - Context creation flags
 \param dev   - Device to create context on

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCoredumpSetAttributeGlobal,
 ::cuCoredumpSetAttribute,
 ::cuCtxSynchronize*/
    fn cuCtxCreate_v2(
        pctx: *mut cuda_types::cuda::CUcontext,
        flags: ::core::ffi::c_uint,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a CUDA context with execution affinity

 Creates a new CUDA context with execution affinity and associates it with
 the calling thread. The \p paramsArray and \p flags parameter are described below.
 The context is created with a usage count of 1 and the caller of ::cuCtxCreate() must
 call ::cuCtxDestroy() when done using the context. If a context is already
 current to the thread, it is supplanted by the newly created context and may
 be restored by a subsequent call to ::cuCtxPopCurrent().

 The type and the amount of execution resource the context can use is limited by \p paramsArray
 and \p numParams. The \p paramsArray is an array of \p CUexecAffinityParam and the \p numParams
 describes the size of the array. If two \p CUexecAffinityParam in the array have the same type,
 the latter execution affinity parameter overrides the former execution affinity parameter.
 The supported execution affinity types are:
 - ::CU_EXEC_AFFINITY_TYPE_SM_COUNT limits the portion of SMs that the context can use. The portion
   of SMs is specified as the number of SMs via \p CUexecAffinitySmCount. This limit will be internally
   rounded up to the next hardware-supported amount. Hence, it is imperative to query the actual execution
   affinity of the context via \p cuCtxGetExecAffinity after context creation. Currently, this attribute
   is only supported under Volta+ MPS.

 The three LSBs of the \p flags parameter can be used to control how the OS
 thread, which owns the CUDA context at the time of an API call, interacts
 with the OS scheduler when waiting for results from the GPU. Only one of
 the scheduling flags can be set when creating a context.

 - ::CU_CTX_SCHED_SPIN: Instruct CUDA to actively spin when waiting for
 results from the GPU. This can decrease latency when waiting for the GPU,
 but may lower the performance of CPU threads if they are performing work in
 parallel with the CUDA thread.

 - ::CU_CTX_SCHED_YIELD: Instruct CUDA to yield its thread when waiting for
 results from the GPU. This can increase latency when waiting for the GPU,
 but can increase the performance of CPU threads performing work in parallel
 with the GPU.

 - ::CU_CTX_SCHED_BLOCKING_SYNC: Instruct CUDA to block the CPU thread on a
 synchronization primitive when waiting for the GPU to finish work.

 - ::CU_CTX_BLOCKING_SYNC: Instruct CUDA to block the CPU thread on a
 synchronization primitive when waiting for the GPU to finish work. <br>
 <b>Deprecated:</b> This flag was deprecated as of CUDA 4.0 and was
 replaced with ::CU_CTX_SCHED_BLOCKING_SYNC.

 - ::CU_CTX_SCHED_AUTO: The default value if the \p flags parameter is zero,
 uses a heuristic based on the number of active CUDA contexts in the
 process \e C and the number of logical processors in the system \e P. If
 \e C > \e P, then CUDA will yield to other OS threads when waiting for
 the GPU (::CU_CTX_SCHED_YIELD), otherwise CUDA will not yield while
 waiting for results and actively spin on the processor (::CU_CTX_SCHED_SPIN).
 Additionally, on Tegra devices, ::CU_CTX_SCHED_AUTO uses a heuristic based on
 the power profile of the platform and may choose ::CU_CTX_SCHED_BLOCKING_SYNC
 for low-powered devices.

 - ::CU_CTX_MAP_HOST: Instruct CUDA to support mapped pinned allocations.
 This flag must be set in order to allocate pinned host memory that is
 accessible to the GPU.

 - ::CU_CTX_LMEM_RESIZE_TO_MAX: Instruct CUDA to not reduce local memory
 after resizing local memory for a kernel. This can prevent thrashing by
 local memory allocations when launching many kernels with high local
 memory usage at the cost of potentially increased memory usage. <br>
 <b>Deprecated:</b> This flag is deprecated and the behavior enabled
 by this flag is now the default and cannot be disabled.
 Instead, the per-thread stack size can be controlled with ::cuCtxSetLimit().

 - ::CU_CTX_COREDUMP_ENABLE: If GPU coredumps have not been enabled globally
 with ::cuCoredumpSetAttributeGlobal or environment variables, this flag can
 be set during context creation to instruct CUDA to create a coredump if
 this context raises an exception during execution. These environment variables
 are described in the CUDA-GDB user guide under the "GPU core dump support"
 section.
 The initial attributes will be taken from the global attributes at the time of
 context creation. The other attributes that control coredump output can be
 modified by calling ::cuCoredumpSetAttribute from the created context after
 it becomes current.

 - ::CU_CTX_USER_COREDUMP_ENABLE: If user-triggered GPU coredumps have not
 been enabled globally with ::cuCoredumpSetAttributeGlobal or environment
 variables, this flag can be set during context creation to instruct CUDA to
 create a coredump if data is written to a certain pipe that is present in the
 OS space. These environment variables are described in the CUDA-GDB user
 guide under the "GPU core dump support" section.
 It is important to note that the pipe name *must* be set with
 ::cuCoredumpSetAttributeGlobal before creating the context if this flag is
 used. Setting this flag implies that ::CU_CTX_COREDUMP_ENABLE is set.
 The initial attributes will be taken from the global attributes at the time of
 context creation. The other attributes that control coredump output can be
 modified by calling ::cuCoredumpSetAttribute from the created context after
 it becomes current.
 Setting this flag on any context creation is equivalent to setting the
 ::CU_COREDUMP_ENABLE_USER_TRIGGER attribute to \p true globally.

 Context creation will fail with ::CUDA_ERROR_UNKNOWN if the compute mode of
 the device is ::CU_COMPUTEMODE_PROHIBITED. The function ::cuDeviceGetAttribute()
 can be used with ::CU_DEVICE_ATTRIBUTE_COMPUTE_MODE to determine the
 compute mode of the device. The <i>nvidia-smi</i> tool can be used to set
 the compute mode for * devices.
 Documentation for <i>nvidia-smi</i> can be obtained by passing a
 -h option to it.

 \param pctx        - Returned context handle of the new context
 \param paramsArray - Execution affinity parameters
 \param numParams   - Number of execution affinity parameters
 \param flags       - Context creation flags
 \param dev         - Device to create context on

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_UNSUPPORTED_EXEC_AFFINITY,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize,
 ::cuCoredumpSetAttributeGlobal,
 ::cuCoredumpSetAttribute,
 ::CUexecAffinityParam*/
    fn cuCtxCreate_v3(
        pctx: *mut cuda_types::cuda::CUcontext,
        paramsArray: *mut cuda_types::cuda::CUexecAffinityParam,
        numParams: ::core::ffi::c_int,
        flags: ::core::ffi::c_uint,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a CUDA context

 Creates a new CUDA context and associates it with the calling thread. The
 \p flags parameter is described below. The context is created with a usage
 count of 1 and the caller of ::cuCtxCreate() must call ::cuCtxDestroy()
 when done using the context. If a context is already current to the thread,
 it is supplanted by the newly created context and may be restored by a subsequent
 call to ::cuCtxPopCurrent().

 CUDA context can be created with execution affinity. The type and the amount of
execution resource the context can use is limited by \p paramsArray and \p numExecAffinityParams
in \p execAffinity. The \p paramsArray is an array of \p CUexecAffinityParam and the \p numExecAffinityParams
 describes the size of the paramsArray. If two \p CUexecAffinityParam in the array have the same type,
 the latter execution affinity parameter overrides the former execution affinity parameter.
 The supported execution affinity types are:
 - ::CU_EXEC_AFFINITY_TYPE_SM_COUNT limits the portion of SMs that the context can use. The portion
   of SMs is specified as the number of SMs via \p CUexecAffinitySmCount. This limit will be internally
   rounded up to the next hardware-supported amount. Hence, it is imperative to query the actual execution
   affinity of the context via \p cuCtxGetExecAffinity after context creation. Currently, this attribute
   is only supported under Volta+ MPS.

 CUDA context can be created in CIG(CUDA in Graphics) mode by setting \p cigParams.
 Data from graphics client is shared with CUDA via the \p sharedData in \p cigParams.
 Support for D3D12 graphics client can be determined using ::cuDeviceGetAttribute() with
 ::CU_DEVICE_ATTRIBUTE_D3D12_CIG_SUPPORTED. \p sharedData is a ID3D12CommandQueue handle.
 Support for Vulkan graphics client can be determined using ::cuDeviceGetAttribute() with
 ::CU_DEVICE_ATTRIBUTE_VULKAN_CIG_SUPPORTED. \p sharedData is a Nvidia specific data blob
 populated by calling vkGetExternalComputeQueueDataNV().
 Either \p execAffinityParams or \p cigParams can be set to a non-null value. Setting both to a
 non-null value will result in an undefined behavior.

 The three LSBs of the \p flags parameter can be used to control how the OS
 thread, which owns the CUDA context at the time of an API call, interacts
 with the OS scheduler when waiting for results from the GPU. Only one of
 the scheduling flags can be set when creating a context.

 - ::CU_CTX_SCHED_SPIN: Instruct CUDA to actively spin when waiting for
 results from the GPU. This can decrease latency when waiting for the GPU,
 but may lower the performance of CPU threads if they are performing work in
 parallel with the CUDA thread.

 - ::CU_CTX_SCHED_YIELD: Instruct CUDA to yield its thread when waiting for
 results from the GPU. This can increase latency when waiting for the GPU,
 but can increase the performance of CPU threads performing work in parallel
 with the GPU.

 - ::CU_CTX_SCHED_BLOCKING_SYNC: Instruct CUDA to block the CPU thread on a
 synchronization primitive when waiting for the GPU to finish work.

 - ::CU_CTX_BLOCKING_SYNC: Instruct CUDA to block the CPU thread on a
 synchronization primitive when waiting for the GPU to finish work. <br>
 <b>Deprecated:</b> This flag was deprecated as of CUDA 4.0 and was
 replaced with ::CU_CTX_SCHED_BLOCKING_SYNC.

 - ::CU_CTX_SCHED_AUTO: The default value if the \p flags parameter is zero,
 uses a heuristic based on the number of active CUDA contexts in the
 process \e C and the number of logical processors in the system \e P. If
 \e C > \e P, then CUDA will yield to other OS threads when waiting for
 the GPU (::CU_CTX_SCHED_YIELD), otherwise CUDA will not yield while
 waiting for results and actively spin on the processor (::CU_CTX_SCHED_SPIN).
 Additionally, on Tegra devices, ::CU_CTX_SCHED_AUTO uses a heuristic based on
 the power profile of the platform and may choose ::CU_CTX_SCHED_BLOCKING_SYNC
 for low-powered devices.

 - ::CU_CTX_MAP_HOST: Instruct CUDA to support mapped pinned allocations.
 This flag must be set in order to allocate pinned host memory that is
 accessible to the GPU.

 - ::CU_CTX_LMEM_RESIZE_TO_MAX: Instruct CUDA to not reduce local memory
 after resizing local memory for a kernel. This can prevent thrashing by
 local memory allocations when launching many kernels with high local
 memory usage at the cost of potentially increased memory usage. <br>
 <b>Deprecated:</b> This flag is deprecated and the behavior enabled
 by this flag is now the default and cannot be disabled.
 Instead, the per-thread stack size can be controlled with ::cuCtxSetLimit().

 - ::CU_CTX_COREDUMP_ENABLE: If GPU coredumps have not been enabled globally
 with ::cuCoredumpSetAttributeGlobal or environment variables, this flag can
 be set during context creation to instruct CUDA to create a coredump if
 this context raises an exception during execution. These environment variables
 are described in the CUDA-GDB user guide under the "GPU core dump support"
 section.
 The initial attributes will be taken from the global attributes at the time of
 context creation. The other attributes that control coredump output can be
 modified by calling ::cuCoredumpSetAttribute from the created context after
 it becomes current. This flag is not supported when CUDA context is created in
 CIG(CUDA in Graphics) mode.

 - ::CU_CTX_USER_COREDUMP_ENABLE: If user-triggered GPU coredumps have not
 been enabled globally with ::cuCoredumpSetAttributeGlobal or environment
 variables, this flag can be set during context creation to instruct CUDA to
 create a coredump if data is written to a certain pipe that is present in the
 OS space. These environment variables are described in the CUDA-GDB user
 guide under the "GPU core dump support" section.
 It is important to note that the pipe name *must* be set with
 ::cuCoredumpSetAttributeGlobal before creating the context if this flag is
 used. Setting this flag implies that ::CU_CTX_COREDUMP_ENABLE is set.
 The initial attributes will be taken from the global attributes at the time of
 context creation. The other attributes that control coredump output can be
 modified by calling ::cuCoredumpSetAttribute from the created context after
 it becomes current.
 Setting this flag on any context creation is equivalent to setting the
 ::CU_COREDUMP_ENABLE_USER_TRIGGER attribute to \p true globally.
 This flag is not supported when CUDA context is created in
 CIG(CUDA in Graphics) mode.

 - ::CU_CTX_SYNC_MEMOPS: Ensures that synchronous memory operations initiated
 on this context will always synchronize. See further documentation in the
 section titled "API Synchronization behavior" to learn more about cases when
 synchronous memory operations can exhibit asynchronous behavior.

 Context creation will fail with ::CUDA_ERROR_UNKNOWN if the compute mode of
 the device is ::CU_COMPUTEMODE_PROHIBITED. The function ::cuDeviceGetAttribute()
 can be used with ::CU_DEVICE_ATTRIBUTE_COMPUTE_MODE to determine the
 compute mode of the device. The <i>nvidia-smi</i> tool can be used to set
 the compute mode for * devices.
 Documentation for <i>nvidia-smi</i> can be obtained by passing a
 -h option to it.

 Context creation will fail with :: CUDA_ERROR_INVALID_VALUE if invalid parameter was
 passed by client to create the CUDA context.

 Context creation in CIG mode will fail with ::CUDA_ERROR_NOT_SUPPORTED if CIG is not supported
 by the device or the driver.
 \param pctx              - Returned context handle of the new context
 \param ctxCreateParams   - Context creation parameters
 \param flags             - Context creation flags
 \param dev               - Device to create context on

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCoredumpSetAttributeGlobal,
 ::cuCoredumpSetAttribute,
 ::cuCtxSynchronize*/
    fn cuCtxCreate_v4(
        pctx: *mut cuda_types::cuda::CUcontext,
        ctxCreateParams: *mut cuda_types::cuda::CUctxCreateParams,
        flags: ::core::ffi::c_uint,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroy a CUDA context

 Destroys the CUDA context specified by \p ctx.  The context \p ctx will be
 destroyed regardless of how many threads it is current to.
 It is the responsibility of the calling function to ensure that no API
 call issues using \p ctx while ::cuCtxDestroy() is executing.

 Destroys and cleans up all resources associated with the context.
 It is the caller's responsibility to ensure that the context or its resources
 are not accessed or passed in subsequent API calls and doing so will result in undefined behavior.
 These resources include CUDA types ::CUmodule, ::CUfunction, ::CUstream, ::CUevent,
 ::CUarray, ::CUmipmappedArray, ::CUtexObject, ::CUsurfObject, ::CUtexref, ::CUsurfref,
 ::CUgraphicsResource, ::CUlinkState, ::CUexternalMemory and ::CUexternalSemaphore.
 These resources also include memory allocations by ::cuMemAlloc(), ::cuMemAllocHost(),
 ::cuMemAllocManaged() and ::cuMemAllocPitch().

 If \p ctx is current to the calling thread then \p ctx will also be
 popped from the current thread's context stack (as though ::cuCtxPopCurrent()
 were called).  If \p ctx is current to other threads, then \p ctx will
 remain current to those threads, and attempting to access \p ctx from
 those threads will result in the error ::CUDA_ERROR_CONTEXT_IS_DESTROYED.

 \note ::cuCtxDestroy() will not destroy memory allocations by ::cuMemCreate(), ::cuMemAllocAsync() and
 ::cuMemAllocFromPoolAsync(). These memory allocations are not associated with any CUDA context and need to
 be destroyed explicitly.

 \param ctx - Context to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize*/
    fn cuCtxDestroy_v2(ctx: cuda_types::cuda::CUcontext) -> cuda_types::cuda::CUresult;
    /** \brief Pushes a context on the current CPU thread

 Pushes the given context \p ctx onto the CPU thread's stack of current
 contexts. The specified context becomes the CPU thread's current context, so
 all CUDA functions that operate on the current context are affected.

 The previous current context may be made current again by calling
 ::cuCtxDestroy() or ::cuCtxPopCurrent().

 \param ctx - Context to push

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize*/
    fn cuCtxPushCurrent_v2(
        ctx: cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Pops the current CUDA context from the current CPU thread.

 Pops the current CUDA context from the CPU thread and passes back the
 old context handle in \p *pctx. That context may then be made current
 to a different CPU thread by calling ::cuCtxPushCurrent().

 If a context was current to the CPU thread before ::cuCtxCreate() or
 ::cuCtxPushCurrent() was called, this function makes that context current to
 the CPU thread again.

 \param pctx - Returned popped context handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize*/
    fn cuCtxPopCurrent_v2(
        pctx: *mut cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Binds the specified CUDA context to the calling CPU thread

 Binds the specified CUDA context to the calling CPU thread.
 If \p ctx is NULL then the CUDA context previously bound to the
 calling CPU thread is unbound and ::CUDA_SUCCESS is returned.

 If there exists a CUDA context stack on the calling CPU thread, this
 will replace the top of that stack with \p ctx.
 If \p ctx is NULL then this will be equivalent to popping the top
 of the calling CPU thread's CUDA context stack (or a no-op if the
 calling CPU thread's CUDA context stack is empty).

 \param ctx - Context to bind to the calling CPU thread

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa
 ::cuCtxGetCurrent,
 ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cudaSetDevice*/
    fn cuCtxSetCurrent(ctx: cuda_types::cuda::CUcontext) -> cuda_types::cuda::CUresult;
    /** \brief Returns the CUDA context bound to the calling CPU thread.

 Returns in \p *pctx the CUDA context bound to the calling CPU thread.
 If no context is bound to the calling CPU thread then \p *pctx is
 set to NULL and ::CUDA_SUCCESS is returned.

 \param pctx - Returned context handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 \notefnerr

 \sa
 ::cuCtxSetCurrent,
 ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cudaGetDevice*/
    fn cuCtxGetCurrent(
        pctx: *mut cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the device handle for the current context

 Returns in \p *device the handle of the current context's device.

 \param device - Returned device handle for the current context

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize,
 ::cudaGetDevice*/
    fn cuCtxGetDevice(
        device: *mut cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the flags for the current context

 Returns in \p *flags the flags of the current context. See ::cuCtxCreate
 for flag values.

 \param flags - Pointer to store flags of current context

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetCurrent,
 ::cuCtxGetDevice,
 ::cuCtxGetLimit,
 ::cuCtxGetSharedMemConfig,
 ::cuCtxGetStreamPriorityRange,
 ::cuCtxSetFlags,
 ::cudaGetDeviceFlags*/
    fn cuCtxGetFlags(flags: *mut ::core::ffi::c_uint) -> cuda_types::cuda::CUresult;
    /** \brief Sets the flags for the current context

 Sets the flags for the current context overwriting previously set ones. See
 ::cuDevicePrimaryCtxSetFlags for flag values.

 \param flags - Flags to set on the current context

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetCurrent,
 ::cuCtxGetDevice,
 ::cuCtxGetLimit,
 ::cuCtxGetSharedMemConfig,
 ::cuCtxGetStreamPriorityRange,
 ::cuCtxGetFlags,
 ::cudaGetDeviceFlags,
 ::cuDevicePrimaryCtxSetFlags,*/
    fn cuCtxSetFlags(flags: ::core::ffi::c_uint) -> cuda_types::cuda::CUresult;
    /** \brief Returns the unique Id associated with the context supplied

 Returns in \p ctxId the unique Id which is associated with a given context.
 The Id is unique for the life of the program for this instance of CUDA.
 If context is supplied as NULL and there is one current, the Id of the
 current context is returned.

 \param ctx - Context for which to obtain the Id
 \param ctxId - Pointer to store the Id of the context

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPushCurrent*/
    fn cuCtxGetId(
        ctx: cuda_types::cuda::CUcontext,
        ctxId: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Block for the current context's tasks to complete

 Blocks until the current context has completed all preceding requested tasks.
 If the current context is the primary context, green contexts that have been
 created will also be synchronized.
 ::cuCtxSynchronize() returns an error if one of the preceding tasks failed.
 If the context was created with the ::CU_CTX_SCHED_BLOCKING_SYNC flag, the
 CPU thread will block until the GPU context has finished its work.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cudaDeviceSynchronize*/
    fn cuCtxSynchronize() -> cuda_types::cuda::CUresult;
    /** \brief Set resource limits

 Setting \p limit to \p value is a request by the application to update
 the current limit maintained by the context. The driver is free to
 modify the requested value to meet h/w requirements (this could be
 clamping to minimum or maximum values, rounding up to nearest element
 size, etc). The application can use ::cuCtxGetLimit() to find out exactly
 what the limit has been set to.

 Setting each ::CUlimit has its own specific restrictions, so each is
 discussed here.

 - ::CU_LIMIT_STACK_SIZE controls the stack size in bytes of each GPU thread.
   The driver automatically increases the per-thread stack size
   for each kernel launch as needed. This size isn't reset back to the
   original value after each launch. Setting this value will take effect
   immediately, and if necessary, the device will block until all preceding
   requested tasks are complete.

 - ::CU_LIMIT_PRINTF_FIFO_SIZE controls the size in bytes of the FIFO used
   by the ::printf() device system call. Setting ::CU_LIMIT_PRINTF_FIFO_SIZE
   must be performed before launching any kernel that uses the ::printf()
   device system call, otherwise ::CUDA_ERROR_INVALID_VALUE will be returned.

 - ::CU_LIMIT_MALLOC_HEAP_SIZE controls the size in bytes of the heap used
   by the ::malloc() and ::free() device system calls. Setting
   ::CU_LIMIT_MALLOC_HEAP_SIZE must be performed before launching any kernel
   that uses the ::malloc() or ::free() device system calls, otherwise
   ::CUDA_ERROR_INVALID_VALUE will be returned.

 - ::CU_LIMIT_DEV_RUNTIME_SYNC_DEPTH controls the maximum nesting depth of
   a grid at which a thread can safely call ::cudaDeviceSynchronize(). Setting
   this limit must be performed before any launch of a kernel that uses the
   device runtime and calls ::cudaDeviceSynchronize() above the default sync
   depth, two levels of grids. Calls to ::cudaDeviceSynchronize() will fail
   with error code ::cudaErrorSyncDepthExceeded if the limitation is
   violated. This limit can be set smaller than the default or up the maximum
   launch depth of 24. When setting this limit, keep in mind that additional
   levels of sync depth require the driver to reserve large amounts of device
   memory which can no longer be used for user allocations. If these
   reservations of device memory fail, ::cuCtxSetLimit() will return
   ::CUDA_ERROR_OUT_OF_MEMORY, and the limit can be reset to a lower value.
   This limit is only applicable to devices of compute capability < 9.0.
   Attempting to set this limit on devices of other compute capability
   versions will result in the error ::CUDA_ERROR_UNSUPPORTED_LIMIT being
   returned.

 - ::CU_LIMIT_DEV_RUNTIME_PENDING_LAUNCH_COUNT controls the maximum number of
   outstanding device runtime launches that can be made from the current
   context. A grid is outstanding from the point of launch up until the grid
   is known to have been completed. Device runtime launches which violate
   this limitation fail and return ::cudaErrorLaunchPendingCountExceeded when
   ::cudaGetLastError() is called after launch. If more pending launches than
   the default (2048 launches) are needed for a module using the device
   runtime, this limit can be increased. Keep in mind that being able to
   sustain additional pending launches will require the driver to reserve
   larger amounts of device memory upfront which can no longer be used for
   allocations. If these reservations fail, ::cuCtxSetLimit() will return
   ::CUDA_ERROR_OUT_OF_MEMORY, and the limit can be reset to a lower value.
   This limit is only applicable to devices of compute capability 3.5 and
   higher. Attempting to set this limit on devices of compute capability less
   than 3.5 will result in the error ::CUDA_ERROR_UNSUPPORTED_LIMIT being
   returned.

 - ::CU_LIMIT_MAX_L2_FETCH_GRANULARITY controls the L2 cache fetch granularity.
   Values can range from 0B to 128B. This is purely a performance hint and
   it can be ignored or clamped depending on the platform.

 - ::CU_LIMIT_PERSISTING_L2_CACHE_SIZE controls size in bytes available for
   persisting L2 cache. This is purely a performance hint and it can be
   ignored or clamped depending on the platform.

 \param limit - Limit to set
 \param value - Size of limit

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNSUPPORTED_LIMIT,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSynchronize,
 ::cudaDeviceSetLimit*/
    fn cuCtxSetLimit(
        limit: cuda_types::cuda::CUlimit,
        value: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns resource limits

 Returns in \p *pvalue the current size of \p limit.  The supported
 ::CUlimit values are:
 - ::CU_LIMIT_STACK_SIZE: stack size in bytes of each GPU thread.
 - ::CU_LIMIT_PRINTF_FIFO_SIZE: size in bytes of the FIFO used by the
   ::printf() device system call.
 - ::CU_LIMIT_MALLOC_HEAP_SIZE: size in bytes of the heap used by the
   ::malloc() and ::free() device system calls.
 - ::CU_LIMIT_DEV_RUNTIME_SYNC_DEPTH: maximum grid depth at which a thread
   can issue the device runtime call ::cudaDeviceSynchronize() to wait on
   child grid launches to complete.
 - ::CU_LIMIT_DEV_RUNTIME_PENDING_LAUNCH_COUNT: maximum number of outstanding
   device runtime launches that can be made from this context.
 - ::CU_LIMIT_MAX_L2_FETCH_GRANULARITY: L2 cache fetch granularity.
 - ::CU_LIMIT_PERSISTING_L2_CACHE_SIZE: Persisting L2 cache size in bytes

 \param limit  - Limit to query
 \param pvalue - Returned size of limit

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNSUPPORTED_LIMIT
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize,
 ::cudaDeviceGetLimit*/
    fn cuCtxGetLimit(
        pvalue: *mut usize,
        limit: cuda_types::cuda::CUlimit,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the preferred cache configuration for the current context.

 On devices where the L1 cache and shared memory use the same hardware
 resources, this function returns through \p pconfig the preferred cache configuration
 for the current context. This is only a preference. The driver will use
 the requested configuration if possible, but it is free to choose a different
 configuration if required to execute functions.

 This will return a \p pconfig of ::CU_FUNC_CACHE_PREFER_NONE on devices
 where the size of the L1 cache and shared memory are fixed.

 The supported cache configurations are:
 - ::CU_FUNC_CACHE_PREFER_NONE: no preference for shared memory or L1 (default)
 - ::CU_FUNC_CACHE_PREFER_SHARED: prefer larger shared memory and smaller L1 cache
 - ::CU_FUNC_CACHE_PREFER_L1: prefer larger L1 cache and smaller shared memory
 - ::CU_FUNC_CACHE_PREFER_EQUAL: prefer equal sized L1 cache and shared memory

 \param pconfig - Returned cache configuration

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize,
 ::cuFuncSetCacheConfig,
 ::cudaDeviceGetCacheConfig*/
    fn cuCtxGetCacheConfig(
        pconfig: *mut cuda_types::cuda::CUfunc_cache,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the preferred cache configuration for the current context.

 On devices where the L1 cache and shared memory use the same hardware
 resources, this sets through \p config the preferred cache configuration for
 the current context. This is only a preference. The driver will use
 the requested configuration if possible, but it is free to choose a different
 configuration if required to execute the function. Any function preference
 set via ::cuFuncSetCacheConfig() or ::cuKernelSetCacheConfig() will be preferred over this context-wide
 setting. Setting the context-wide cache configuration to
 ::CU_FUNC_CACHE_PREFER_NONE will cause subsequent kernel launches to prefer
 to not change the cache configuration unless required to launch the kernel.

 This setting does nothing on devices where the size of the L1 cache and
 shared memory are fixed.

 Launching a kernel with a different preference than the most recent
 preference setting may insert a device-side synchronization point.

 The supported cache configurations are:
 - ::CU_FUNC_CACHE_PREFER_NONE: no preference for shared memory or L1 (default)
 - ::CU_FUNC_CACHE_PREFER_SHARED: prefer larger shared memory and smaller L1 cache
 - ::CU_FUNC_CACHE_PREFER_L1: prefer larger L1 cache and smaller shared memory
 - ::CU_FUNC_CACHE_PREFER_EQUAL: prefer equal sized L1 cache and shared memory

 \param config - Requested cache configuration

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize,
 ::cuFuncSetCacheConfig,
 ::cudaDeviceSetCacheConfig,
 ::cuKernelSetCacheConfig*/
    fn cuCtxSetCacheConfig(
        config: cuda_types::cuda::CUfunc_cache,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the context's API version.

 Returns a version number in \p version corresponding to the capabilities of
 the context (e.g. 3010 or 3020), which library developers can use to direct
 callers to a specific API version. If \p ctx is NULL, returns the API version
 used to create the currently bound context.

 Note that new API versions are only introduced when context capabilities are
 changed that break binary compatibility, so the API version and driver version
 may be different. For example, it is valid for the API version to be 3020 while
 the driver version is 4020.

 \param ctx     - Context to check
 \param version - Pointer to version

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize*/
    fn cuCtxGetApiVersion(
        ctx: cuda_types::cuda::CUcontext,
        version: *mut ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns numerical values that correspond to the least and
 greatest stream priorities.

 Returns in \p *leastPriority and \p *greatestPriority the numerical values that correspond
 to the least and greatest stream priorities respectively. Stream priorities
 follow a convention where lower numbers imply greater priorities. The range of
 meaningful stream priorities is given by [\p *greatestPriority, \p *leastPriority].
 If the user attempts to create a stream with a priority value that is
 outside the meaningful range as specified by this API, the priority is
 automatically clamped down or up to either \p *leastPriority or \p *greatestPriority
 respectively. See ::cuStreamCreateWithPriority for details on creating a
 priority stream.
 A NULL may be passed in for \p *leastPriority or \p *greatestPriority if the value
 is not desired.

 This function will return '0' in both \p *leastPriority and \p *greatestPriority if
 the current context's device does not support stream priorities
 (see ::cuDeviceGetAttribute).

 \param leastPriority    - Pointer to an int in which the numerical value for least
                           stream priority is returned
 \param greatestPriority - Pointer to an int in which the numerical value for greatest
                           stream priority is returned

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr

 \sa ::cuStreamCreateWithPriority,
 ::cuStreamGetPriority,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize,
 ::cudaDeviceGetStreamPriorityRange*/
    fn cuCtxGetStreamPriorityRange(
        leastPriority: *mut ::core::ffi::c_int,
        greatestPriority: *mut ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Resets all persisting lines in cache to normal status.

 ::cuCtxResetPersistingL2Cache Resets all persisting lines in cache to normal
 status. Takes effect on function return.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa
 ::CUaccessPolicyWindow*/
    fn cuCtxResetPersistingL2Cache() -> cuda_types::cuda::CUresult;
    /** \brief Returns the execution affinity setting for the current context.

 Returns in \p *pExecAffinity the current value of \p type. The supported
 ::CUexecAffinityType values are:
 - ::CU_EXEC_AFFINITY_TYPE_SM_COUNT: number of SMs the context is limited to use.

 \param type          - Execution affinity type to query
 \param pExecAffinity - Returned execution affinity

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNSUPPORTED_EXEC_AFFINITY
 \notefnerr

 \sa
 ::CUexecAffinityParam*/
    fn cuCtxGetExecAffinity(
        pExecAffinity: *mut cuda_types::cuda::CUexecAffinityParam,
        type_: cuda_types::cuda::CUexecAffinityType,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Records an event.

 Captures in \p hEvent all the activities of the context \p hCtx
 at the time of this call. \p hEvent and \p hCtx must be from the same
 CUDA context, otherwise ::CUDA_ERROR_INVALID_HANDLE will be returned.
 Calls such as ::cuEventQuery() or ::cuCtxWaitEvent() will then examine
 or wait for completion of the work that was captured.
 Uses of \p hCtx after this call do not modify \p hEvent.
 If the context passed to \p hCtx is the primary context, \p hEvent will
 capture all the activities of the primary context and its green contexts.
 If the context passed to \p hCtx is a context converted from green context
 via ::cuCtxFromGreenCtx(), \p hEvent will capture only the activities of the green context.

 \note The API will return ::CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED if the
 specified context \p hCtx has a stream in the capture mode. In such a case,
 the call will invalidate all the conflicting captures.

 \param hCtx - Context to record event for
 \param hEvent - Event to record

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED

 \sa
 ::cuCtxWaitEvent,
 ::cuGreenCtxRecordEvent,
 ::cuGreenCtxWaitEvent,
 ::cuEventRecord*/
    fn cuCtxRecordEvent(
        hCtx: cuda_types::cuda::CUcontext,
        hEvent: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Make a context wait on an event

 Makes all future work submitted to context \p hCtx wait for all work
 captured in \p hEvent. The synchronization will be performed on the device
 and will not block the calling CPU thread. See ::cuCtxRecordEvent()
 for details on what is captured by an event.
 If the context passed to \p hCtx is the primary context, the primary context
 and its green contexts will wait for \p hEvent.
 If the context passed to \p hCtx is a context converted from green context
 via ::cuCtxFromGreenCtx(), the green context will wait for \p hEvent.

 \note \p hEvent may be from a different context or device than \p hCtx.

 \note The API will return ::CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED and
 invalidate the capture if the specified event \p hEvent is part of an ongoing
 capture sequence or if the specified context \p hCtx has a stream in the capture mode.

 \param hCtx    - Context to wait
 \param hEvent  - Event to wait on

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED

 \sa
 ::cuCtxRecordEvent,
 ::cuGreenCtxRecordEvent,
 ::cuGreenCtxWaitEvent,
 ::cuStreamWaitEvent*/
    fn cuCtxWaitEvent(
        hCtx: cuda_types::cuda::CUcontext,
        hEvent: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Increment a context's usage-count

 \deprecated

 Note that this function is deprecated and should not be used.

 Increments the usage count of the context and passes back a context handle
 in \p *pctx that must be passed to ::cuCtxDetach() when the application is
 done with the context. ::cuCtxAttach() fails if there is no context current
 to the thread.

 Currently, the \p flags parameter must be 0.

 \param pctx  - Returned context handle of the current context
 \param flags - Context attach flags (must be 0)

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxDetach,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize*/
    fn cuCtxAttach(
        pctx: *mut cuda_types::cuda::CUcontext,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Decrement a context's usage-count

 \deprecated

 Note that this function is deprecated and should not be used.

 Decrements the usage count of the context \p ctx, and destroys the context
 if the usage count goes to 0. The context must be a handle that was passed
 back by ::cuCtxCreate() or ::cuCtxAttach(), and must be current to the
 calling thread.

 \param ctx - Context to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetCacheConfig,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize*/
    fn cuCtxDetach(ctx: cuda_types::cuda::CUcontext) -> cuda_types::cuda::CUresult;
    /** \brief Returns the current shared memory configuration for the current context.

 \deprecated

 This function will return in \p pConfig the current size of shared memory banks
 in the current context. On devices with configurable shared memory banks,
 ::cuCtxSetSharedMemConfig can be used to change this setting, so that all
 subsequent kernel launches will by default use the new bank size. When
 ::cuCtxGetSharedMemConfig is called on devices without configurable shared
 memory, it will return the fixed bank size of the hardware.

 The returned bank configurations can be either:
 - ::CU_SHARED_MEM_CONFIG_FOUR_BYTE_BANK_SIZE:  shared memory bank width is
   four bytes.
 - ::CU_SHARED_MEM_CONFIG_EIGHT_BYTE_BANK_SIZE: shared memory bank width will
   eight bytes.

 \param pConfig - returned shared memory configuration
 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize,
 ::cuCtxGetSharedMemConfig,
 ::cuFuncSetCacheConfig,
 ::cudaDeviceGetSharedMemConfig*/
    fn cuCtxGetSharedMemConfig(
        pConfig: *mut cuda_types::cuda::CUsharedconfig,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the shared memory configuration for the current context.

 \deprecated

 On devices with configurable shared memory banks, this function will set
 the context's shared memory bank size which is used for subsequent kernel
 launches.

 Changed the shared memory configuration between launches may insert a device
 side synchronization point between those launches.

 Changing the shared memory bank size will not increase shared memory usage
 or affect occupancy of kernels, but may have major effects on performance.
 Larger bank sizes will allow for greater potential bandwidth to shared memory,
 but will change what kinds of accesses to shared memory will result in bank
 conflicts.

 This function will do nothing on devices with fixed shared memory bank size.

 The supported bank configurations are:
 - ::CU_SHARED_MEM_CONFIG_DEFAULT_BANK_SIZE: set bank width to the default initial
   setting (currently, four bytes).
 - ::CU_SHARED_MEM_CONFIG_FOUR_BYTE_BANK_SIZE: set shared memory bank width to
   be natively four bytes.
 - ::CU_SHARED_MEM_CONFIG_EIGHT_BYTE_BANK_SIZE: set shared memory bank width to
   be natively eight bytes.

 \param config - requested shared memory configuration

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxCreate,
 ::cuCtxDestroy,
 ::cuCtxGetApiVersion,
 ::cuCtxGetCacheConfig,
 ::cuCtxGetDevice,
 ::cuCtxGetFlags,
 ::cuCtxGetLimit,
 ::cuCtxPopCurrent,
 ::cuCtxPushCurrent,
 ::cuCtxSetLimit,
 ::cuCtxSynchronize,
 ::cuCtxGetSharedMemConfig,
 ::cuFuncSetCacheConfig,
 ::cudaDeviceSetSharedMemConfig*/
    fn cuCtxSetSharedMemConfig(
        config: cuda_types::cuda::CUsharedconfig,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Loads a compute module

 Takes a filename \p fname and loads the corresponding module \p module into
 the current context. The CUDA driver API does not attempt to lazily
 allocate the resources needed by a module; if the memory for functions and
 data (constant and global) needed by the module cannot be allocated,
 ::cuModuleLoad() fails. The file should be a \e cubin file as output by
 \b nvcc, or a \e PTX file either as output by \b nvcc or handwritten, or
 a \e fatbin file as output by \b nvcc from toolchain 4.0 or later.

 \param module - Returned module
 \param fname  - Filename of module to load

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_PTX,
 ::CUDA_ERROR_UNSUPPORTED_PTX_VERSION,
 ::CUDA_ERROR_NOT_FOUND,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_FILE_NOT_FOUND,
 ::CUDA_ERROR_NO_BINARY_FOR_GPU,
 ::CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
 ::CUDA_ERROR_JIT_COMPILER_NOT_FOUND
 \notefnerr

 \sa ::cuModuleGetFunction,
 ::cuModuleGetGlobal,
 ::cuModuleGetTexRef,
 ::cuModuleLoadData,
 ::cuModuleLoadDataEx,
 ::cuModuleLoadFatBinary,
 ::cuModuleUnload*/
    fn cuModuleLoad(
        module: *mut cuda_types::cuda::CUmodule,
        fname: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Load a module's data

 Takes a pointer \p image and loads the corresponding module \p module into
 the current context. The \p image may be a \e cubin or \e fatbin
 as output by \b nvcc, or a NULL-terminated \e PTX, either as output by \b nvcc
 or hand-written.

 \param module - Returned module
 \param image  - Module data to load

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_PTX,
 ::CUDA_ERROR_UNSUPPORTED_PTX_VERSION,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NO_BINARY_FOR_GPU,
 ::CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
 ::CUDA_ERROR_JIT_COMPILER_NOT_FOUND
 \notefnerr

 \sa ::cuModuleGetFunction,
 ::cuModuleGetGlobal,
 ::cuModuleGetTexRef,
 ::cuModuleLoad,
 ::cuModuleLoadDataEx,
 ::cuModuleLoadFatBinary,
 ::cuModuleUnload*/
    fn cuModuleLoadData(
        module: *mut cuda_types::cuda::CUmodule,
        image: *const ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Load a module's data with options

 Takes a pointer \p image and loads the corresponding module \p module into
 the current context. The \p image may be a \e cubin or \e fatbin
 as output by \b nvcc, or a NULL-terminated \e PTX, either as output by \b nvcc
 or hand-written.

 \param module       - Returned module
 \param image        - Module data to load
 \param numOptions   - Number of options
 \param options      - Options for JIT
 \param optionValues - Option values for JIT

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_PTX,
 ::CUDA_ERROR_UNSUPPORTED_PTX_VERSION,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NO_BINARY_FOR_GPU,
 ::CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
 ::CUDA_ERROR_JIT_COMPILER_NOT_FOUND
 \notefnerr

 \sa ::cuModuleGetFunction,
 ::cuModuleGetGlobal,
 ::cuModuleGetTexRef,
 ::cuModuleLoad,
 ::cuModuleLoadData,
 ::cuModuleLoadFatBinary,
 ::cuModuleUnload*/
    fn cuModuleLoadDataEx(
        module: *mut cuda_types::cuda::CUmodule,
        image: *const ::core::ffi::c_void,
        numOptions: ::core::ffi::c_uint,
        options: *mut cuda_types::cuda::CUjit_option,
        optionValues: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Load a module's data

 Takes a pointer \p fatCubin and loads the corresponding module \p module
 into the current context. The pointer represents a <i>fat binary</i> object,
 which is a collection of different \e cubin and/or \e PTX files, all
 representing the same device code, but compiled and optimized for different
 architectures.

 Prior to CUDA 4.0, there was no documented API for constructing and using
 fat binary objects by programmers.  Starting with CUDA 4.0, fat binary
 objects can be constructed by providing the <i>-fatbin option</i> to \b nvcc.
 More information can be found in the \b nvcc document.

 \param module   - Returned module
 \param fatCubin - Fat binary to load

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_PTX,
 ::CUDA_ERROR_UNSUPPORTED_PTX_VERSION,
 ::CUDA_ERROR_NOT_FOUND,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NO_BINARY_FOR_GPU,
 ::CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
 ::CUDA_ERROR_JIT_COMPILER_NOT_FOUND
 \notefnerr

 \sa ::cuModuleGetFunction,
 ::cuModuleGetGlobal,
 ::cuModuleGetTexRef,
 ::cuModuleLoad,
 ::cuModuleLoadData,
 ::cuModuleLoadDataEx,
 ::cuModuleUnload*/
    fn cuModuleLoadFatBinary(
        module: *mut cuda_types::cuda::CUmodule,
        fatCubin: *const ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unloads a module

 Unloads a module \p hmod from the current context. Attempting to unload
 a module which was obtained from the Library Management API such as
 ::cuLibraryGetModule will return ::CUDA_ERROR_NOT_PERMITTED.

 \param hmod - Module to unload

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_PERMITTED
 \notefnerr
 \note_destroy_ub

 \sa ::cuModuleGetFunction,
 ::cuModuleGetGlobal,
 ::cuModuleGetTexRef,
 ::cuModuleLoad,
 ::cuModuleLoadData,
 ::cuModuleLoadDataEx,
 ::cuModuleLoadFatBinary*/
    fn cuModuleUnload(hmod: cuda_types::cuda::CUmodule) -> cuda_types::cuda::CUresult;
    /** \brief Query lazy loading mode

 Returns lazy loading mode
 Module loading mode is controlled by CUDA_MODULE_LOADING env variable

 \param mode      - Returns the lazy loading mode

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr

 \sa
 ::cuModuleLoad,*/
    fn cuModuleGetLoadingMode(
        mode: *mut cuda_types::cuda::CUmoduleLoadingMode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a function handle

 Returns in \p *hfunc the handle of the function of name \p name located in
 module \p hmod. If no function of that name exists, ::cuModuleGetFunction()
 returns ::CUDA_ERROR_NOT_FOUND.

 \param hfunc - Returned function handle
 \param hmod  - Module to retrieve function from
 \param name  - Name of function to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_FOUND
 \notefnerr

 \sa ::cuModuleGetGlobal,
 ::cuModuleGetTexRef,
 ::cuModuleLoad,
 ::cuModuleLoadData,
 ::cuModuleLoadDataEx,
 ::cuModuleLoadFatBinary,
 ::cuModuleUnload*/
    fn cuModuleGetFunction(
        hfunc: *mut cuda_types::cuda::CUfunction,
        hmod: cuda_types::cuda::CUmodule,
        name: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the number of functions within a module

 Returns in \p count the number of functions in \p mod.

 \param count - Number of functions found within the module
 \param mod - Module to query

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE*/
    fn cuModuleGetFunctionCount(
        count: *mut ::core::ffi::c_uint,
        mod_: cuda_types::cuda::CUmodule,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the function handles within a module.

 Returns in \p functions a maximum number of \p numFunctions function handles within \p mod. When
 function loading mode is set to LAZY the function retrieved may be partially loaded. The loading
 state of a function can be queried using ::cuFunctionIsLoaded. CUDA APIs may load the function
 automatically when called with partially loaded function handle which may incur additional
 latency. Alternatively, ::cuFunctionLoad can be used to explicitly load a function. The returned
 function handles become invalid when the module is unloaded.

 \param functions - Buffer where the function handles are returned to
 \param numFunctions - Maximum number of function handles may be returned to the buffer
 \param mod - Module to query from

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuModuleGetFunction,
 ::cuModuleGetFunctionCount,
 ::cuFuncIsLoaded,
 ::cuFuncLoad*/
    fn cuModuleEnumerateFunctions(
        functions: *mut cuda_types::cuda::CUfunction,
        numFunctions: ::core::ffi::c_uint,
        mod_: cuda_types::cuda::CUmodule,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a global pointer from a module

 Returns in \p *dptr and \p *bytes the base pointer and size of the
 global of name \p name located in module \p hmod. If no variable of that name
 exists, ::cuModuleGetGlobal() returns ::CUDA_ERROR_NOT_FOUND.
 One of the parameters \p dptr or \p bytes (not both) can be NULL in which
 case it is ignored.

 \param dptr  - Returned global device pointer
 \param bytes - Returned global size in bytes
 \param hmod  - Module to retrieve global from
 \param name  - Name of global to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_FOUND
 \notefnerr

 \sa ::cuModuleGetFunction,
 ::cuModuleGetTexRef,
 ::cuModuleLoad,
 ::cuModuleLoadData,
 ::cuModuleLoadDataEx,
 ::cuModuleLoadFatBinary,
 ::cuModuleUnload,
 ::cudaGetSymbolAddress,
 ::cudaGetSymbolSize*/
    fn cuModuleGetGlobal_v2(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        bytes: *mut usize,
        hmod: cuda_types::cuda::CUmodule,
        name: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a pending JIT linker invocation.

 If the call is successful, the caller owns the returned CUlinkState, which
 should eventually be destroyed with ::cuLinkDestroy.  The
 device code machine size (32 or 64 bit) will match the calling application.

 Both linker and compiler options may be specified.  Compiler options will
 be applied to inputs to this linker action which must be compiled from PTX.
 The options ::CU_JIT_WALL_TIME,
 ::CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES, and ::CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES
 will accumulate data until the CUlinkState is destroyed.

 The data passed in via ::cuLinkAddData and ::cuLinkAddFile will be treated
 as relocatable (-rdc=true to nvcc) when linking the final cubin during
 ::cuLinkComplete and will have similar consequences as offline relocatable
 device code linking.

 \p optionValues must remain valid for the life of the CUlinkState if output
 options are used.  No other references to inputs are maintained after this
 call returns.

 \note For LTO-IR input, only LTO-IR compiled with toolkits prior to CUDA 12.0 will be accepted

 \param numOptions   Size of options arrays
 \param options      Array of linker and compiler options
 \param optionValues Array of option values, each cast to void *
 \param stateOut     On success, this will contain a CUlinkState to specify
                     and complete this action

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_JIT_COMPILER_NOT_FOUND
 \notefnerr

 \sa ::cuLinkAddData,
 ::cuLinkAddFile,
 ::cuLinkComplete,
 ::cuLinkDestroy*/
    fn cuLinkCreate_v2(
        numOptions: ::core::ffi::c_uint,
        options: *mut cuda_types::cuda::CUjit_option,
        optionValues: *mut *mut ::core::ffi::c_void,
        stateOut: *mut cuda_types::cuda::CUlinkState,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Add an input to a pending linker invocation

 Ownership of \p data is retained by the caller.  No reference is retained to any
 inputs after this call returns.

 This method accepts only compiler options, which are used if the data must
 be compiled from PTX, and does not accept any of
 ::CU_JIT_WALL_TIME, ::CU_JIT_INFO_LOG_BUFFER, ::CU_JIT_ERROR_LOG_BUFFER,
 ::CU_JIT_TARGET_FROM_CUCONTEXT, or ::CU_JIT_TARGET.

 \note For LTO-IR input, only LTO-IR compiled with toolkits prior to CUDA 12.0 will be accepted

 \param state        A pending linker action.
 \param type         The type of the input data.
 \param data         The input data.  PTX must be NULL-terminated.
 \param size         The length of the input data.
 \param name         An optional name for this input in log messages.
 \param numOptions   Size of options.
 \param options      Options to be applied only for this input (overrides options from ::cuLinkCreate).
 \param optionValues Array of option values, each cast to void *.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_IMAGE,
 ::CUDA_ERROR_INVALID_PTX,
 ::CUDA_ERROR_UNSUPPORTED_PTX_VERSION,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NO_BINARY_FOR_GPU

 \sa ::cuLinkCreate,
 ::cuLinkAddFile,
 ::cuLinkComplete,
 ::cuLinkDestroy*/
    fn cuLinkAddData_v2(
        state: cuda_types::cuda::CUlinkState,
        type_: cuda_types::cuda::CUjitInputType,
        data: *mut ::core::ffi::c_void,
        size: usize,
        name: *const ::core::ffi::c_char,
        numOptions: ::core::ffi::c_uint,
        options: *mut cuda_types::cuda::CUjit_option,
        optionValues: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Add a file input to a pending linker invocation

 No reference is retained to any inputs after this call returns.

 This method accepts only compiler options, which are used if the input
 must be compiled from PTX, and does not accept any of
 ::CU_JIT_WALL_TIME, ::CU_JIT_INFO_LOG_BUFFER, ::CU_JIT_ERROR_LOG_BUFFER,
 ::CU_JIT_TARGET_FROM_CUCONTEXT, or ::CU_JIT_TARGET.

 This method is equivalent to invoking ::cuLinkAddData on the contents
 of the file.

 \note For LTO-IR input, only LTO-IR compiled with toolkits prior to CUDA 12.0 will be accepted

 \param state        A pending linker action
 \param type         The type of the input data
 \param path         Path to the input file
 \param numOptions   Size of options
 \param options      Options to be applied only for this input (overrides options from ::cuLinkCreate)
 \param optionValues Array of option values, each cast to void *

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_FILE_NOT_FOUND
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_IMAGE,
 ::CUDA_ERROR_INVALID_PTX,
 ::CUDA_ERROR_UNSUPPORTED_PTX_VERSION,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NO_BINARY_FOR_GPU

 \sa ::cuLinkCreate,
 ::cuLinkAddData,
 ::cuLinkComplete,
 ::cuLinkDestroy*/
    fn cuLinkAddFile_v2(
        state: cuda_types::cuda::CUlinkState,
        type_: cuda_types::cuda::CUjitInputType,
        path: *const ::core::ffi::c_char,
        numOptions: ::core::ffi::c_uint,
        options: *mut cuda_types::cuda::CUjit_option,
        optionValues: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Complete a pending linker invocation

 Completes the pending linker action and returns the cubin image for the linked
 device code, which can be used with ::cuModuleLoadData.  The cubin is owned by
 \p state, so it should be loaded before \p state is destroyed via ::cuLinkDestroy.
 This call does not destroy \p state.

 \param state    A pending linker invocation
 \param cubinOut On success, this will point to the output image
 \param sizeOut  Optional parameter to receive the size of the generated image

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa ::cuLinkCreate,
 ::cuLinkAddData,
 ::cuLinkAddFile,
 ::cuLinkDestroy,
 ::cuModuleLoadData*/
    fn cuLinkComplete(
        state: cuda_types::cuda::CUlinkState,
        cubinOut: *mut *mut ::core::ffi::c_void,
        sizeOut: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys state for a JIT linker invocation.

 \param state State object for the linker invocation

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE

 \sa ::cuLinkCreate*/
    fn cuLinkDestroy(state: cuda_types::cuda::CUlinkState) -> cuda_types::cuda::CUresult;
    /** \brief Returns a handle to a texture reference

 \deprecated

 Returns in \p *pTexRef the handle of the texture reference of name \p name
 in the module \p hmod. If no texture reference of that name exists,
 ::cuModuleGetTexRef() returns ::CUDA_ERROR_NOT_FOUND. This texture reference
 handle should not be destroyed, since it will be destroyed when the module
 is unloaded.

 \param pTexRef  - Returned texture reference
 \param hmod     - Module to retrieve texture reference from
 \param name     - Name of texture reference to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_FOUND
 \notefnerr

 \sa
 ::cuModuleGetFunction,
 ::cuModuleGetGlobal,
 ::cuModuleGetSurfRef,
 ::cuModuleLoad,
 ::cuModuleLoadData,
 ::cuModuleLoadDataEx,
 ::cuModuleLoadFatBinary,
 ::cuModuleUnload*/
    fn cuModuleGetTexRef(
        pTexRef: *mut cuda_types::cuda::CUtexref,
        hmod: cuda_types::cuda::CUmodule,
        name: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a handle to a surface reference

 \deprecated

 Returns in \p *pSurfRef the handle of the surface reference of name \p name
 in the module \p hmod. If no surface reference of that name exists,
 ::cuModuleGetSurfRef() returns ::CUDA_ERROR_NOT_FOUND.

 \param pSurfRef  - Returned surface reference
 \param hmod     - Module to retrieve surface reference from
 \param name     - Name of surface reference to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_FOUND
 \notefnerr

 \sa
 ::cuModuleGetFunction,
 ::cuModuleGetGlobal,
 ::cuModuleGetTexRef,
 ::cuModuleLoad,
 ::cuModuleLoadData,
 ::cuModuleLoadDataEx,
 ::cuModuleLoadFatBinary,
 ::cuModuleUnload*/
    fn cuModuleGetSurfRef(
        pSurfRef: *mut cuda_types::cuda::CUsurfref,
        hmod: cuda_types::cuda::CUmodule,
        name: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Load a library with specified code and options

 Takes a pointer \p code and loads the corresponding library \p library based on
 the application defined library loading mode:
 - If module loading is set to EAGER, via the environment variables described in "Module loading",
   \p library is loaded eagerly into all contexts at the time of the call and future contexts
   at the time of creation until the library is unloaded with ::cuLibraryUnload().
 - If the environment variables are set to LAZY, \p library
   is not immediately loaded onto all existent contexts and will only be
   loaded when a function is needed for that context, such as a kernel launch.

 These environment variables are described in the CUDA programming guide under the
 "CUDA environment variables" section.

 The \p code may be a \e cubin or \e fatbin as output by \b nvcc,
 or a NULL-terminated \e PTX, either as output by \b nvcc or hand-written.
 A fatbin should also contain relocatable code when doing separate compilation.

 Options are passed as an array via \p jitOptions and any corresponding parameters are passed in
 \p jitOptionsValues. The number of total JIT options is supplied via \p numJitOptions.
 Any outputs will be returned via \p jitOptionsValues.

 Library load options are passed as an array via \p libraryOptions and any corresponding parameters are passed in
 \p libraryOptionValues. The number of total library load options is supplied via \p numLibraryOptions.

 \note If the library contains managed variables and no device in the system
 supports managed variables this call is expected to return ::CUDA_ERROR_NOT_SUPPORTED

 \param library             - Returned library
 \param code                - Code to load
 \param jitOptions          - Options for JIT
 \param jitOptionsValues    - Option values for JIT
 \param numJitOptions       - Number of options
 \param libraryOptions      - Options for loading
 \param libraryOptionValues - Option values for loading
 \param numLibraryOptions   - Number of options for loading

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_PTX,
 ::CUDA_ERROR_UNSUPPORTED_PTX_VERSION,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NO_BINARY_FOR_GPU,
 ::CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
 ::CUDA_ERROR_JIT_COMPILER_NOT_FOUND,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuLibraryLoadFromFile,
 ::cuLibraryUnload,
 ::cuModuleLoad,
 ::cuModuleLoadData,
 ::cuModuleLoadDataEx*/
    fn cuLibraryLoadData(
        library: *mut cuda_types::cuda::CUlibrary,
        code: *const ::core::ffi::c_void,
        jitOptions: *mut cuda_types::cuda::CUjit_option,
        jitOptionsValues: *mut *mut ::core::ffi::c_void,
        numJitOptions: ::core::ffi::c_uint,
        libraryOptions: *mut cuda_types::cuda::CUlibraryOption,
        libraryOptionValues: *mut *mut ::core::ffi::c_void,
        numLibraryOptions: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Load a library with specified file and options

 Takes a pointer \p code and loads the corresponding library \p library based on
 the application defined library loading mode:
 - If module loading is set to EAGER, via the environment variables described in "Module loading",
   \p library is loaded eagerly into all contexts at the time of the call and future contexts
   at the time of creation until the library is unloaded with ::cuLibraryUnload().
 - If the environment variables are set to LAZY, \p library
   is not immediately loaded onto all existent contexts and will only be
   loaded when a function is needed for that context, such as a kernel launch.

 These environment variables are described in the CUDA programming guide under the
 "CUDA environment variables" section.

 The file should be a \e cubin file as output by \b nvcc, or a \e PTX file either
 as output by \b nvcc or handwritten, or a \e fatbin file as output by \b nvcc.
 A fatbin should also contain relocatable code when doing separate compilation.

 Options are passed as an array via \p jitOptions and any corresponding parameters are
 passed in \p jitOptionsValues. The number of total options is supplied via \p numJitOptions.
 Any outputs will be returned via \p jitOptionsValues.

 Library load options are passed as an array via \p libraryOptions and any corresponding parameters are passed in
 \p libraryOptionValues. The number of total library load options is supplied via \p numLibraryOptions.

 \note If the library contains managed variables and no device in the system
 supports managed variables this call is expected to return ::CUDA_ERROR_NOT_SUPPORTED

 \param library             - Returned library
 \param fileName            - File to load from
 \param jitOptions          - Options for JIT
 \param jitOptionsValues    - Option values for JIT
 \param numJitOptions       - Number of options
 \param libraryOptions      - Options for loading
 \param libraryOptionValues - Option values for loading
 \param numLibraryOptions   - Number of options for loading

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_PTX,
 ::CUDA_ERROR_UNSUPPORTED_PTX_VERSION,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NO_BINARY_FOR_GPU,
 ::CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
 ::CUDA_ERROR_JIT_COMPILER_NOT_FOUND,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuLibraryLoadData,
 ::cuLibraryUnload,
 ::cuModuleLoad,
 ::cuModuleLoadData,
 ::cuModuleLoadDataEx*/
    fn cuLibraryLoadFromFile(
        library: *mut cuda_types::cuda::CUlibrary,
        fileName: *const ::core::ffi::c_char,
        jitOptions: *mut cuda_types::cuda::CUjit_option,
        jitOptionsValues: *mut *mut ::core::ffi::c_void,
        numJitOptions: ::core::ffi::c_uint,
        libraryOptions: *mut cuda_types::cuda::CUlibraryOption,
        libraryOptionValues: *mut *mut ::core::ffi::c_void,
        numLibraryOptions: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unloads a library

 Unloads the library specified with \p library

 \param library - Library to unload

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuModuleUnload*/
    fn cuLibraryUnload(
        library: cuda_types::cuda::CUlibrary,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a kernel handle

 Returns in \p pKernel the handle of the kernel with name \p name located in library \p library.
 If kernel handle is not found, the call returns ::CUDA_ERROR_NOT_FOUND.

 \param pKernel - Returned kernel handle
 \param library - Library to retrieve kernel from
 \param name - Name of kernel to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_FOUND

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload,
 ::cuKernelGetFunction,
 ::cuLibraryGetModule,
 ::cuModuleGetFunction*/
    fn cuLibraryGetKernel(
        pKernel: *mut cuda_types::cuda::CUkernel,
        library: cuda_types::cuda::CUlibrary,
        name: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the number of kernels within a library

 Returns in \p count the number of kernels in \p lib.

 \param count - Number of kernels found within the library
 \param lib - Library to query

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE*/
    fn cuLibraryGetKernelCount(
        count: *mut ::core::ffi::c_uint,
        lib: cuda_types::cuda::CUlibrary,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Retrieve the kernel handles within a library.

 Returns in \p kernels a maximum number of \p numKernels kernel handles within \p lib.
 The returned kernel handle becomes invalid when the library is unloaded.

 \param kernels - Buffer where the kernel handles are returned to
 \param numKernels - Maximum number of kernel handles may be returned to the buffer
 \param lib - Library to query from

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuLibraryGetKernelCount*/
    fn cuLibraryEnumerateKernels(
        kernels: *mut cuda_types::cuda::CUkernel,
        numKernels: ::core::ffi::c_uint,
        lib: cuda_types::cuda::CUlibrary,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a module handle

 Returns in \p pMod the module handle associated with the current context located in
 library \p library. If module handle is not found, the call returns ::CUDA_ERROR_NOT_FOUND.

 \param pMod - Returned module handle
 \param library - Library to retrieve module from

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_FOUND,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload,
 ::cuModuleGetFunction*/
    fn cuLibraryGetModule(
        pMod: *mut cuda_types::cuda::CUmodule,
        library: cuda_types::cuda::CUlibrary,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a function handle

 Returns in \p pFunc the handle of the function for the requested kernel \p kernel and
 the current context. If function handle is not found, the call returns ::CUDA_ERROR_NOT_FOUND.

 \param pFunc - Returned function handle
 \param kernel - Kernel to retrieve function for the requested context

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_FOUND,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload,
 ::cuLibraryGetKernel,
 ::cuLibraryGetModule,
 ::cuModuleGetFunction*/
    fn cuKernelGetFunction(
        pFunc: *mut cuda_types::cuda::CUfunction,
        kernel: cuda_types::cuda::CUkernel,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a library handle

 Returns in \p pLib the handle of the library for the requested kernel \p kernel

 \param pLib - Returned library handle
 \param kernel - Kernel to retrieve library handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_FOUND

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload,
 ::cuLibraryGetKernel*/
    fn cuKernelGetLibrary(
        pLib: *mut cuda_types::cuda::CUlibrary,
        kernel: cuda_types::cuda::CUkernel,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a global device pointer

 Returns in \p *dptr and \p *bytes the base pointer and size of the global with
 name \p name for the requested library \p library and the current context.
 If no global for the requested name \p name exists, the call returns ::CUDA_ERROR_NOT_FOUND.
 One of the parameters \p dptr or \p bytes (not both) can be NULL in which
 case it is ignored.

 \param dptr - Returned global device pointer for the requested context
 \param bytes - Returned global size in bytes
 \param library - Library to retrieve global from
 \param name - Name of global to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_FOUND,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload,
 ::cuLibraryGetModule,
 cuModuleGetGlobal*/
    fn cuLibraryGetGlobal(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        bytes: *mut usize,
        library: cuda_types::cuda::CUlibrary,
        name: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a pointer to managed memory

 Returns in \p *dptr and \p *bytes the base pointer and size of the managed memory with
 name \p name for the requested library \p library. If no managed memory with the
 requested name \p name exists, the call returns ::CUDA_ERROR_NOT_FOUND. One of the parameters
 \p dptr or \p bytes (not both) can be NULL in which case it is ignored.
 Note that managed memory for library \p library is shared across devices and is registered
 when the library is loaded into atleast one context.

 \param dptr - Returned pointer to the managed memory
 \param bytes - Returned memory size in bytes
 \param library - Library to retrieve managed memory from
 \param name - Name of managed memory to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_FOUND

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload*/
    fn cuLibraryGetManaged(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        bytes: *mut usize,
        library: cuda_types::cuda::CUlibrary,
        name: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a pointer to a unified function

 Returns in \p *fptr the function pointer to a unified function denoted by \p symbol.
 If no unified function with name \p symbol exists, the call returns ::CUDA_ERROR_NOT_FOUND.
 If there is no device with attribute ::CU_DEVICE_ATTRIBUTE_UNIFIED_FUNCTION_POINTERS present in the system,
 the call may return ::CUDA_ERROR_NOT_FOUND.

 \param fptr - Returned pointer to a unified function
 \param library - Library to retrieve function pointer memory from
 \param symbol - Name of function pointer to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_FOUND

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload*/
    fn cuLibraryGetUnifiedFunction(
        fptr: *mut *mut ::core::ffi::c_void,
        library: cuda_types::cuda::CUlibrary,
        symbol: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns information about a kernel

 Returns in \p *pi the integer value of the attribute \p attrib for the kernel
 \p kernel for the requested device \p dev. The supported attributes are:
 - ::CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK: The maximum number of threads
   per block, beyond which a launch of the kernel would fail. This number
   depends on both the kernel and the requested device.
 - ::CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES: The size in bytes of
   statically-allocated shared memory per block required by this kernel.
   This does not include dynamically-allocated shared memory requested by
   the user at runtime.
 - ::CU_FUNC_ATTRIBUTE_CONST_SIZE_BYTES: The size in bytes of user-allocated
   constant memory required by this kernel.
 - ::CU_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES: The size in bytes of local memory
   used by each thread of this kernel.
 - ::CU_FUNC_ATTRIBUTE_NUM_REGS: The number of registers used by each thread
   of this kernel.
 - ::CU_FUNC_ATTRIBUTE_PTX_VERSION: The PTX virtual architecture version for
   which the kernel was compiled. This value is the major PTX version * 10
   + the minor PTX version, so a PTX version 1.3 function would return the
   value 13. Note that this may return the undefined value of 0 for cubins
   compiled prior to CUDA 3.0.
 - ::CU_FUNC_ATTRIBUTE_BINARY_VERSION: The binary architecture version for
   which the kernel was compiled. This value is the major binary
   version * 10 + the minor binary version, so a binary version 1.3 function
   would return the value 13. Note that this will return a value of 10 for
   legacy cubins that do not have a properly-encoded binary architecture
   version.
 - ::CU_FUNC_CACHE_MODE_CA: The attribute to indicate whether the kernel has
   been compiled with user specified option "-Xptxas --dlcm=ca" set.
 - ::CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES: The maximum size in bytes of
   dynamically-allocated shared memory.
 - ::CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT: Preferred shared memory-L1
   cache split ratio in percent of total shared memory.
 - ::CU_FUNC_ATTRIBUTE_CLUSTER_SIZE_MUST_BE_SET: If this attribute is set, the
   kernel must launch with a valid cluster size specified.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_WIDTH: The required cluster width in
   blocks.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_HEIGHT: The required cluster height in
   blocks.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_DEPTH: The required cluster depth in
   blocks.
 - ::CU_FUNC_ATTRIBUTE_NON_PORTABLE_CLUSTER_SIZE_ALLOWED: Indicates whether
   the function can be launched with non-portable cluster size. 1 is allowed,
   0 is disallowed. A non-portable cluster size may only function on the
   specific SKUs the program is tested on. The launch might fail if the
   program is run on a different hardware platform. CUDA API provides
   cudaOccupancyMaxActiveClusters to assist with checking whether the desired
   size can be launched on the current device. A portable cluster size is
   guaranteed to be functional on all compute capabilities higher than the
   target compute capability. The portable cluster size for sm_90 is 8 blocks
   per cluster. This value may increase for future compute capabilities. The
   specific hardware unit may support higher cluster sizes that’s not
   guaranteed to be portable.
 - ::CU_FUNC_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE: The block
   scheduling policy of a function. The value type is CUclusterSchedulingPolicy.

 \note If another thread is trying to set the same attribute on the same device using
 ::cuKernelSetAttribute() simultaneously, the attribute query will give the old or new
 value depending on the interleavings chosen by the OS scheduler and memory consistency.

 \param pi     - Returned attribute value
 \param attrib - Attribute requested
 \param kernel  - Kernel to query attribute of
 \param dev - Device to query attribute of

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload,
 ::cuKernelSetAttribute,
 ::cuLibraryGetKernel,
 ::cuLaunchKernel,
 ::cuKernelGetFunction,
 ::cuLibraryGetModule,
 ::cuModuleGetFunction,
 ::cuFuncGetAttribute*/
    fn cuKernelGetAttribute(
        pi: *mut ::core::ffi::c_int,
        attrib: cuda_types::cuda::CUfunction_attribute,
        kernel: cuda_types::cuda::CUkernel,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets information about a kernel

 This call sets the value of a specified attribute \p attrib on the kernel \p kernel
 for the requested device \p dev to an integer value specified by \p val.
 This function returns CUDA_SUCCESS if the new value of the attribute could be
 successfully set. If the set fails, this call will return an error.
 Not all attributes can have values set. Attempting to set a value on a read-only
 attribute will result in an error (CUDA_ERROR_INVALID_VALUE)

 Note that attributes set using ::cuFuncSetAttribute() will override the attribute
 set by this API irrespective of whether the call to ::cuFuncSetAttribute() is made
 before or after this API call. However, ::cuKernelGetAttribute() will always
 return the attribute value set by this API.

 Supported attributes are:
 - ::CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES: This is the maximum size in bytes of
   dynamically-allocated shared memory. The value should contain the requested
   maximum size of dynamically-allocated shared memory. The sum of this value and
   the function attribute ::CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES cannot exceed the
   device attribute ::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN.
   The maximal size of requestable dynamic shared memory may differ by GPU
   architecture.
 - ::CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT: On devices where the L1
   cache and shared memory use the same hardware resources, this sets the shared memory
   carveout preference, in percent of the total shared memory.
   See ::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR
   This is only a hint, and the driver can choose a different ratio if required to execute the function.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_WIDTH: The required cluster width in
   blocks. The width, height, and depth values must either all be 0 or all be
   positive. The validity of the cluster dimensions is checked at launch time.
   If the value is set during compile time, it cannot be set at runtime.
   Setting it at runtime will return CUDA_ERROR_NOT_PERMITTED.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_HEIGHT: The required cluster height in
   blocks. The width, height, and depth values must either all be 0 or all be
   positive. The validity of the cluster dimensions is checked at launch time.
   If the value is set during compile time, it cannot be set at runtime.
   Setting it at runtime will return CUDA_ERROR_NOT_PERMITTED.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_DEPTH: The required cluster depth in
   blocks. The width, height, and depth values must either all be 0 or all be
   positive. The validity of the cluster dimensions is checked at launch time.
   If the value is set during compile time, it cannot be set at runtime.
   Setting it at runtime will return CUDA_ERROR_NOT_PERMITTED.
 - ::CU_FUNC_ATTRIBUTE_NON_PORTABLE_CLUSTER_SIZE_ALLOWED: Indicates whether
   the function can be launched with non-portable cluster size. 1 is allowed,
   0 is disallowed.
 - ::CU_FUNC_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE: The block
   scheduling policy of a function. The value type is CUclusterSchedulingPolicy.

 \note The API has stricter locking requirements in comparison to its legacy counterpart
 ::cuFuncSetAttribute() due to device-wide semantics. If multiple threads are trying to
 set the same attribute on the same device simultaneously, the attribute setting will depend
 on the interleavings chosen by the OS scheduler and memory consistency.

 \param attrib - Attribute requested
 \param val - Value to set
 \param kernel  - Kernel to set attribute of
 \param dev - Device to set attribute of

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload,
 ::cuKernelGetAttribute,
 ::cuLibraryGetKernel,
 ::cuLaunchKernel,
 ::cuKernelGetFunction,
 ::cuLibraryGetModule,
 ::cuModuleGetFunction,
 ::cuFuncSetAttribute*/
    fn cuKernelSetAttribute(
        attrib: cuda_types::cuda::CUfunction_attribute,
        val: ::core::ffi::c_int,
        kernel: cuda_types::cuda::CUkernel,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the preferred cache configuration for a device kernel.

 On devices where the L1 cache and shared memory use the same hardware
 resources, this sets through \p config the preferred cache configuration for
 the device kernel \p kernel on the requested device \p dev. This is only a preference.
 The driver will use the requested configuration if possible, but it is free to choose a different
 configuration if required to execute \p kernel.  Any context-wide preference
 set via ::cuCtxSetCacheConfig() will be overridden by this per-kernel
 setting.

 Note that attributes set using ::cuFuncSetCacheConfig() will override the attribute
 set by this API irrespective of whether the call to ::cuFuncSetCacheConfig() is made
 before or after this API call.

 This setting does nothing on devices where the size of the L1 cache and
 shared memory are fixed.

 Launching a kernel with a different preference than the most recent
 preference setting may insert a device-side synchronization point.


 The supported cache configurations are:
 - ::CU_FUNC_CACHE_PREFER_NONE: no preference for shared memory or L1 (default)
 - ::CU_FUNC_CACHE_PREFER_SHARED: prefer larger shared memory and smaller L1 cache
 - ::CU_FUNC_CACHE_PREFER_L1: prefer larger L1 cache and smaller shared memory
 - ::CU_FUNC_CACHE_PREFER_EQUAL: prefer equal sized L1 cache and shared memory

 \note The API has stricter locking requirements in comparison to its legacy counterpart
 ::cuFuncSetCacheConfig() due to device-wide semantics. If multiple threads are trying to
 set a config on the same device simultaneously, the cache config setting will depend
 on the interleavings chosen by the OS scheduler and memory consistency.

 \param kernel  - Kernel to configure cache for
 \param config - Requested cache configuration
 \param dev - Device to set attribute of

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa ::cuLibraryLoadData,
 ::cuLibraryLoadFromFile,
 ::cuLibraryUnload,
 ::cuLibraryGetKernel,
 ::cuKernelGetFunction,
 ::cuLibraryGetModule,
 ::cuModuleGetFunction,
 ::cuFuncSetCacheConfig,
 ::cuCtxSetCacheConfig,
 ::cuLaunchKernel*/
    fn cuKernelSetCacheConfig(
        kernel: cuda_types::cuda::CUkernel,
        config: cuda_types::cuda::CUfunc_cache,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the function name for a ::CUkernel handle

 Returns in \p **name the function name associated with the kernel handle \p hfunc .
 The function name is returned as a null-terminated string. The returned name is only
 valid when the kernel handle is valid. If the library is unloaded or reloaded, one
 must call the API again to get the updated name. This API may return a mangled name if
 the function is not declared as having C linkage. If either \p **name or \p hfunc
 is NULL, ::CUDA_ERROR_INVALID_VALUE is returned.

 \param name - The returned name of the function
 \param hfunc - The function handle to retrieve the name for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
*/
    fn cuKernelGetName(
        name: *mut *const ::core::ffi::c_char,
        hfunc: cuda_types::cuda::CUkernel,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the offset and size of a kernel parameter in the device-side parameter layout

 Queries the kernel parameter at \p paramIndex into \p kernel's list of parameters, and returns
 in \p paramOffset and \p paramSize the offset and size, respectively, where the parameter
 will reside in the device-side parameter layout. This information can be used to update kernel
 node parameters from the device via ::cudaGraphKernelNodeSetParam() and
 ::cudaGraphKernelNodeUpdatesApply(). \p paramIndex must be less than the number of parameters
 that \p kernel takes. \p paramSize can be set to NULL if only the parameter offset is desired.

 \param kernel      - The kernel to query
 \param paramIndex  - The parameter index to query
 \param paramOffset - Returns the offset into the device-side parameter layout at which the parameter resides
 \param paramSize   - Optionally returns the size of the parameter in the device-side parameter layout

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr

 \sa ::cuFuncGetParamInfo*/
    fn cuKernelGetParamInfo(
        kernel: cuda_types::cuda::CUkernel,
        paramIndex: usize,
        paramOffset: *mut usize,
        paramSize: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets free and total memory

 Returns in \p *total the total amount of memory available to the the current context.
 Returns in \p *free the amount of memory on the device that is free according to the OS.
 CUDA is not guaranteed to be able to allocate all of the memory that the OS reports as free.
 In a multi-tenet situation, free estimate returned is prone to race condition where
 a new allocation/free done by a different process or a different thread in the same
 process between the time when free memory was estimated and reported, will result in
 deviation in free value reported and actual free memory.

 The integrated GPU on Tegra shares memory with CPU and other component
 of the SoC. The free and total values returned by the API excludes
 the SWAP memory space maintained by the OS on some platforms.
 The OS may move some of the memory pages into swap area as the GPU or
 CPU allocate or access memory. See Tegra app note on how to calculate
 total and free memory on Tegra.

 \param free  - Returned free memory in bytes
 \param total - Returned total memory in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemGetInfo*/
    fn cuMemGetInfo_v2(
        free: *mut usize,
        total: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allocates device memory

 Allocates \p bytesize bytes of linear memory on the device and returns in
 \p *dptr a pointer to the allocated memory. The allocated memory is suitably
 aligned for any kind of variable. The memory is not cleared. If \p bytesize
 is 0, ::cuMemAlloc() returns ::CUDA_ERROR_INVALID_VALUE.

 \param dptr     - Returned device pointer
 \param bytesize - Requested allocation size in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMalloc*/
    fn cuMemAlloc_v2(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        bytesize: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allocates pitched device memory

 Allocates at least \p WidthInBytes * \p Height bytes of linear memory on
 the device and returns in \p *dptr a pointer to the allocated memory. The
 function may pad the allocation to ensure that corresponding pointers in
 any given row will continue to meet the alignment requirements for
 coalescing as the address is updated from row to row. \p ElementSizeBytes
 specifies the size of the largest reads and writes that will be performed
 on the memory range. \p ElementSizeBytes may be 4, 8 or 16 (since coalesced
 memory transactions are not possible on other data sizes). If
 \p ElementSizeBytes is smaller than the actual read/write size of a kernel,
 the kernel will run correctly, but possibly at reduced speed. The pitch
 returned in \p *pPitch by ::cuMemAllocPitch() is the width in bytes of the
 allocation. The intended usage of pitch is as a separate parameter of the
 allocation, used to compute addresses within the 2D array. Given the row
 and column of an array element of type \b T, the address is computed as:
 \code
T* pElement = (T*)((char*)BaseAddress + Row * Pitch) + Column;
 \endcode

 The pitch returned by ::cuMemAllocPitch() is guaranteed to work with
 ::cuMemcpy2D() under all circumstances. For allocations of 2D arrays, it is
 recommended that programmers consider performing pitch allocations using
 ::cuMemAllocPitch(). Due to alignment restrictions in the hardware, this is
 especially true if the application will be performing 2D memory copies
 between different regions of device memory (whether linear memory or CUDA
 arrays).

 The byte alignment of the pitch returned by ::cuMemAllocPitch() is guaranteed
 to match or exceed the alignment requirement for texture binding with
 ::cuTexRefSetAddress2D().

 \param dptr             - Returned device pointer
 \param pPitch           - Returned pitch of allocation in bytes
 \param WidthInBytes     - Requested allocation width in bytes
 \param Height           - Requested allocation height in rows
 \param ElementSizeBytes - Size of largest reads/writes for range

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMallocPitch*/
    fn cuMemAllocPitch_v2(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        pPitch: *mut usize,
        WidthInBytes: usize,
        Height: usize,
        ElementSizeBytes: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Frees device memory

 Frees the memory space pointed to by \p dptr, which must have been returned
 by a previous call to one of the following memory allocation APIs - ::cuMemAlloc(),
 ::cuMemAllocPitch(), ::cuMemAllocManaged(), ::cuMemAllocAsync(), ::cuMemAllocFromPoolAsync()

 Note - This API will not perform any implict synchronization when the pointer was allocated with
 ::cuMemAllocAsync or ::cuMemAllocFromPoolAsync. Callers must ensure that all accesses to these
 pointer have completed before invoking ::cuMemFree. For best performance and memory reuse, users
 should use ::cuMemFreeAsync to free memory allocated via the stream ordered memory allocator.
 For all other pointers, this API may perform implicit synchronization.

 \param dptr - Pointer to memory to free

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemAllocManaged, ::cuMemAllocAsync, ::cuMemAllocFromPoolAsync,
 ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned, ::cuMemcpy3D, ::cuMemcpy3DAsync,
 ::cuMemcpyAtoA, ::cuMemcpyAtoD, ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA,
 ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync, ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA,
 ::cuMemcpyHtoAAsync, ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc, ::cuMemFreeAsync,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaFree*/
    fn cuMemFree_v2(dptr: cuda_types::cuda::CUdeviceptr) -> cuda_types::cuda::CUresult;
    /** \brief Get information on memory allocations

 Returns the base address in \p *pbase and size in \p *psize of the
 allocation by ::cuMemAlloc() or ::cuMemAllocPitch() that contains the input
 pointer \p dptr. Both parameters \p pbase and \p psize are optional. If one
 of them is NULL, it is ignored.

 \param pbase - Returned base address
 \param psize - Returned size of device memory allocation
 \param dptr  - Device pointer to query

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_NOT_FOUND,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32*/
    fn cuMemGetAddressRange_v2(
        pbase: *mut cuda_types::cuda::CUdeviceptr,
        psize: *mut usize,
        dptr: cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allocates page-locked host memory

 Allocates \p bytesize bytes of host memory that is page-locked and
 accessible to the device. The driver tracks the virtual memory ranges
 allocated with this function and automatically accelerates calls to
 functions such as ::cuMemcpy(). Since the memory can be accessed directly by
 the device, it can be read or written with much higher bandwidth than
 pageable memory obtained with functions such as ::malloc().

 On systems where ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES
 is true, ::cuMemAllocHost may not page-lock the allocated memory.

 Page-locking excessive amounts of memory with ::cuMemAllocHost() may degrade system
 performance, since it reduces the amount of memory available to the system
 for paging. As a result, this function is best used sparingly to allocate
 staging areas for data exchange between host and device.

 Note all host memory allocated using ::cuMemAllocHost() will automatically
 be immediately accessible to all contexts on all devices which support unified
 addressing (as may be queried using ::CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING).
 The device pointer that may be used to access this host memory from those
 contexts is always equal to the returned host pointer \p *pp.
 See \ref CUDA_UNIFIED for additional details.

 \param pp       - Returned pointer to host memory
 \param bytesize - Requested allocation size in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMallocHost*/
    fn cuMemAllocHost_v2(
        pp: *mut *mut ::core::ffi::c_void,
        bytesize: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Frees page-locked host memory

 Frees the memory space pointed to by \p p, which must have been returned by
 a previous call to ::cuMemAllocHost().

 \param p - Pointer to memory to free

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaFreeHost*/
    fn cuMemFreeHost(p: *mut ::core::ffi::c_void) -> cuda_types::cuda::CUresult;
    /** \brief Allocates page-locked host memory

 Allocates \p bytesize bytes of host memory that is page-locked and accessible
 to the device. The driver tracks the virtual memory ranges allocated with
 this function and automatically accelerates calls to functions such as
 ::cuMemcpyHtoD(). Since the memory can be accessed directly by the device,
 it can be read or written with much higher bandwidth than pageable memory
 obtained with functions such as ::malloc().

 On systems where ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES
 is true, ::cuMemHostAlloc may not page-lock the allocated memory.

 Page-locking excessive amounts of memory may degrade system performance,
 since it reduces the amount of memory available to the system for paging.
 As a result, this function is best used sparingly to allocate staging areas
 for data exchange between host and device.

 The \p Flags parameter enables different options to be specified that
 affect the allocation, as follows.

 - ::CU_MEMHOSTALLOC_PORTABLE: The memory returned by this call will be
   considered as pinned memory by all CUDA contexts, not just the one that
   performed the allocation.

 - ::CU_MEMHOSTALLOC_DEVICEMAP: Maps the allocation into the CUDA address
   space. The device pointer to the memory may be obtained by calling
   ::cuMemHostGetDevicePointer().

 - ::CU_MEMHOSTALLOC_WRITECOMBINED: Allocates the memory as write-combined
   (WC). WC memory can be transferred across the PCI Express bus more
   quickly on some system configurations, but cannot be read efficiently by
   most CPUs. WC memory is a good option for buffers that will be written by
   the CPU and read by the GPU via mapped pinned memory or host->device
   transfers.

 All of these flags are orthogonal to one another: a developer may allocate
 memory that is portable, mapped and/or write-combined with no restrictions.

 The ::CU_MEMHOSTALLOC_DEVICEMAP flag may be specified on CUDA contexts for
 devices that do not support mapped pinned memory. The failure is deferred
 to ::cuMemHostGetDevicePointer() because the memory may be mapped into
 other CUDA contexts via the ::CU_MEMHOSTALLOC_PORTABLE flag.

 The memory allocated by this function must be freed with ::cuMemFreeHost().

 Note all host memory allocated using ::cuMemHostAlloc() will automatically
 be immediately accessible to all contexts on all devices which support unified
 addressing (as may be queried using ::CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING).
 Unless the flag ::CU_MEMHOSTALLOC_WRITECOMBINED is specified, the device pointer
 that may be used to access this host memory from those contexts is always equal
 to the returned host pointer \p *pp.  If the flag ::CU_MEMHOSTALLOC_WRITECOMBINED
 is specified, then the function ::cuMemHostGetDevicePointer() must be used
 to query the device pointer, even if the context supports unified addressing.
 See \ref CUDA_UNIFIED for additional details.

 \param pp       - Returned pointer to host memory
 \param bytesize - Requested allocation size in bytes
 \param Flags    - Flags for allocation request

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaHostAlloc*/
    fn cuMemHostAlloc(
        pp: *mut *mut ::core::ffi::c_void,
        bytesize: usize,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Passes back device pointer of mapped pinned memory

 Passes back the device pointer \p pdptr corresponding to the mapped, pinned
 host buffer \p p allocated by ::cuMemHostAlloc.

 ::cuMemHostGetDevicePointer() will fail if the ::CU_MEMHOSTALLOC_DEVICEMAP
 flag was not specified at the time the memory was allocated, or if the
 function is called on a GPU that does not support mapped pinned memory.

 For devices that have a non-zero value for the device attribute
 ::CU_DEVICE_ATTRIBUTE_CAN_USE_HOST_POINTER_FOR_REGISTERED_MEM, the memory
 can also be accessed from the device using the host pointer \p p.
 The device pointer returned by ::cuMemHostGetDevicePointer() may or may not
 match the original host pointer \p p and depends on the devices visible to the
 application. If all devices visible to the application have a non-zero value for the
 device attribute, the device pointer returned by ::cuMemHostGetDevicePointer()
 will match the original pointer \p p. If any device visible to the application
 has a zero value for the device attribute, the device pointer returned by
 ::cuMemHostGetDevicePointer() will not match the original host pointer \p p,
 but it will be suitable for use on all devices provided Unified Virtual Addressing
 is enabled. In such systems, it is valid to access the memory using either pointer
 on devices that have a non-zero value for the device attribute. Note however that
 such devices should access the memory using only one of the two pointers and not both.

 \p Flags provides for future releases. For now, it must be set to 0.

 \param pdptr - Returned device pointer
 \param p     - Host pointer
 \param Flags - Options (must be 0)

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaHostGetDevicePointer*/
    fn cuMemHostGetDevicePointer_v2(
        pdptr: *mut cuda_types::cuda::CUdeviceptr,
        p: *mut ::core::ffi::c_void,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Passes back flags that were used for a pinned allocation

 Passes back the flags \p pFlags that were specified when allocating
 the pinned host buffer \p p allocated by ::cuMemHostAlloc.

 ::cuMemHostGetFlags() will fail if the pointer does not reside in
 an allocation performed by ::cuMemAllocHost() or ::cuMemHostAlloc().

 \param pFlags - Returned flags word
 \param p     - Host pointer

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::cuMemAllocHost,
 ::cuMemHostAlloc,
 ::cudaHostGetFlags*/
    fn cuMemHostGetFlags(
        pFlags: *mut ::core::ffi::c_uint,
        p: *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allocates memory that will be automatically managed by the Unified Memory system

 Allocates \p bytesize bytes of managed memory on the device and returns in
 \p *dptr a pointer to the allocated memory. If the device doesn't support
 allocating managed memory, ::CUDA_ERROR_NOT_SUPPORTED is returned. Support
 for managed memory can be queried using the device attribute
 ::CU_DEVICE_ATTRIBUTE_MANAGED_MEMORY. The allocated memory is suitably
 aligned for any kind of variable. The memory is not cleared. If \p bytesize
 is 0, ::cuMemAllocManaged returns ::CUDA_ERROR_INVALID_VALUE. The pointer
 is valid on the CPU and on all GPUs in the system that support managed memory.
 All accesses to this pointer must obey the Unified Memory programming model.

 \p flags specifies the default stream association for this allocation.
 \p flags must be one of ::CU_MEM_ATTACH_GLOBAL or ::CU_MEM_ATTACH_HOST. If
 ::CU_MEM_ATTACH_GLOBAL is specified, then this memory is accessible from
 any stream on any device. If ::CU_MEM_ATTACH_HOST is specified, then the
 allocation should not be accessed from devices that have a zero value for the
 device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS; an explicit call to
 ::cuStreamAttachMemAsync will be required to enable access on such devices.

 If the association is later changed via ::cuStreamAttachMemAsync to
 a single stream, the default association as specified during ::cuMemAllocManaged
 is restored when that stream is destroyed. For __managed__ variables, the
 default association is always ::CU_MEM_ATTACH_GLOBAL. Note that destroying a
 stream is an asynchronous operation, and as a result, the change to default
 association won't happen until all work in the stream has completed.

 Memory allocated with ::cuMemAllocManaged should be released with ::cuMemFree.

 Device memory oversubscription is possible for GPUs that have a non-zero value for the
 device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS. Managed memory on
 such GPUs may be evicted from device memory to host memory at any time by the Unified
 Memory driver in order to make room for other allocations.

 In a system where all GPUs have a non-zero value for the device attribute
 ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS, managed memory may not be populated when this
 API returns and instead may be populated on access. In such systems, managed memory can
 migrate to any processor's memory at any time. The Unified Memory driver will employ heuristics to
 maintain data locality and prevent excessive page faults to the extent possible. The application
 can also guide the driver about memory usage patterns via ::cuMemAdvise. The application
 can also explicitly migrate memory to a desired processor's memory via
 ::cuMemPrefetchAsync.

 In a multi-GPU system where all of the GPUs have a zero value for the device attribute
 ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS and all the GPUs have peer-to-peer support
 with each other, the physical storage for managed memory is created on the GPU which is active
 at the time ::cuMemAllocManaged is called. All other GPUs will reference the data at reduced
 bandwidth via peer mappings over the PCIe bus. The Unified Memory driver does not migrate
 memory among such GPUs.

 In a multi-GPU system where not all GPUs have peer-to-peer support with each other and
 where the value of the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS
 is zero for at least one of those GPUs, the location chosen for physical storage of managed
 memory is system-dependent.
 - On Linux, the location chosen will be device memory as long as the current set of active
 contexts are on devices that either have peer-to-peer support with each other or have a
 non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS.
 If there is an active context on a GPU that does not have a non-zero value for that device
 attribute and it does not have peer-to-peer support with the other devices that have active
 contexts on them, then the location for physical storage will be 'zero-copy' or host memory.
 Note that this means that managed memory that is located in device memory is migrated to
 host memory if a new context is created on a GPU that doesn't have a non-zero value for
 the device attribute and does not support peer-to-peer with at least one of the other devices
 that has an active context. This in turn implies that context creation may fail if there is
 insufficient host memory to migrate all managed allocations.
 - On Windows, the physical storage is always created in 'zero-copy' or host memory.
 All GPUs will reference the data at reduced bandwidth over the PCIe bus. In these
 circumstances, use of the environment variable CUDA_VISIBLE_DEVICES is recommended to
 restrict CUDA to only use those GPUs that have peer-to-peer support.
 Alternatively, users can also set CUDA_MANAGED_FORCE_DEVICE_ALLOC to a
 non-zero value to force the driver to always use device memory for physical storage.
 When this environment variable is set to a non-zero value, all contexts created in
 that process on devices that support managed memory have to be peer-to-peer compatible
 with each other. Context creation will fail if a context is created on a device that
 supports managed memory and is not peer-to-peer compatible with any of the other
 managed memory supporting devices on which contexts were previously created, even if
 those contexts have been destroyed. These environment variables are described
 in the CUDA programming guide under the "CUDA environment variables" section.
 - On ARM, managed memory is not available on discrete gpu with Drive PX-2.

 \param dptr     - Returned device pointer
 \param bytesize - Requested allocation size in bytes
 \param flags    - Must be one of ::CU_MEM_ATTACH_GLOBAL or ::CU_MEM_ATTACH_HOST

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cuDeviceGetAttribute, ::cuStreamAttachMemAsync,
 ::cudaMallocManaged*/
    fn cuMemAllocManaged(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        bytesize: usize,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Registers a callback function to receive async notifications

 Registers \p callbackFunc to receive async notifications.

 The \p userData parameter is passed to the callback function at async notification time.
 Likewise, \p callback is also passed to the callback function to distinguish between
 multiple registered callbacks.

 The callback function being registered should be designed to return quickly (~10ms).
 Any long running tasks should be queued for execution on an application thread.

 Callbacks may not call cuDeviceRegisterAsyncNotification or cuDeviceUnregisterAsyncNotification.
 Doing so will result in ::CUDA_ERROR_NOT_PERMITTED. Async notification callbacks execute
 in an undefined order and may be serialized.

 Returns in \p *callback a handle representing the registered callback instance.

 \param device - The device on which to register the callback
 \param callbackFunc - The function to register as a callback
 \param userData - A generic pointer to user data. This is passed into the callback function.
 \param callback - A handle representing the registered callback instance

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cuDeviceUnregisterAsyncNotification*/
    fn cuDeviceRegisterAsyncNotification(
        device: cuda_types::cuda::CUdevice,
        callbackFunc: cuda_types::cuda::CUasyncCallback,
        userData: *mut ::core::ffi::c_void,
        callback: *mut cuda_types::cuda::CUasyncCallbackHandle,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unregisters an async notification callback

 Unregisters \p callback so that the corresponding callback function will stop receiving
 async notifications.

 \param device - The device from which to remove \p callback.
 \param callback - The callback instance to unregister from receiving async notifications.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cuDeviceRegisterAsyncNotification*/
    fn cuDeviceUnregisterAsyncNotification(
        device: cuda_types::cuda::CUdevice,
        callback: cuda_types::cuda::CUasyncCallbackHandle,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a handle to a compute device

 Returns in \p *device a device handle given a PCI bus ID string.

 \param dev      - Returned device handle

 \param pciBusId - String in one of the following forms:
 [domain]:[bus]:[device].[function]
 [domain]:[bus]:[device]
 [bus]:[device].[function]
 where \p domain, \p bus, \p device, and \p function are all hexadecimal values

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGet,
 ::cuDeviceGetAttribute,
 ::cuDeviceGetPCIBusId,
 ::cudaDeviceGetByPCIBusId*/
    fn cuDeviceGetByPCIBusId(
        dev: *mut cuda_types::cuda::CUdevice,
        pciBusId: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a PCI Bus Id string for the device

 Returns an ASCII string identifying the device \p dev in the NULL-terminated
 string pointed to by \p pciBusId. \p len specifies the maximum length of the
 string that may be returned.

 \param pciBusId - Returned identifier string for the device in the following format
 [domain]:[bus]:[device].[function]
 where \p domain, \p bus, \p device, and \p function are all hexadecimal values.
 pciBusId should be large enough to store 13 characters including the NULL-terminator.

 \param len      - Maximum length of string to store in \p name

 \param dev      - Device to get identifier string for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuDeviceGet,
 ::cuDeviceGetAttribute,
 ::cuDeviceGetByPCIBusId,
 ::cudaDeviceGetPCIBusId*/
    fn cuDeviceGetPCIBusId(
        pciBusId: *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets an interprocess handle for a previously allocated event

 Takes as input a previously allocated event. This event must have been
 created with the ::CU_EVENT_INTERPROCESS and ::CU_EVENT_DISABLE_TIMING
 flags set. This opaque handle may be copied into other processes and
 opened with ::cuIpcOpenEventHandle to allow efficient hardware
 synchronization between GPU work in different processes.

 After the event has been opened in the importing process,
 ::cuEventRecord, ::cuEventSynchronize, ::cuStreamWaitEvent and
 ::cuEventQuery may be used in either process. Performing operations
 on the imported event after the exported event has been freed
 with ::cuEventDestroy will result in undefined behavior.

 IPC functionality is restricted to devices with support for unified
 addressing on Linux and Windows operating systems.
 IPC functionality on Windows is supported for compatibility purposes
 but not recommended as it comes with performance cost.
 Users can test their device for IPC functionality by calling
 ::cuDeviceGetAttribute with ::CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED

 \param pHandle - Pointer to a user allocated CUipcEventHandle
                    in which to return the opaque event handle
 \param event   - Event allocated with ::CU_EVENT_INTERPROCESS and
                    ::CU_EVENT_DISABLE_TIMING flags.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_MAP_FAILED,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuEventCreate,
 ::cuEventDestroy,
 ::cuEventSynchronize,
 ::cuEventQuery,
 ::cuStreamWaitEvent,
 ::cuIpcOpenEventHandle,
 ::cuIpcGetMemHandle,
 ::cuIpcOpenMemHandle,
 ::cuIpcCloseMemHandle,
 ::cudaIpcGetEventHandle*/
    fn cuIpcGetEventHandle(
        pHandle: *mut cuda_types::cuda::CUipcEventHandle,
        event: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Opens an interprocess event handle for use in the current process

 Opens an interprocess event handle exported from another process with
 ::cuIpcGetEventHandle. This function returns a ::CUevent that behaves like
 a locally created event with the ::CU_EVENT_DISABLE_TIMING flag specified.
 This event must be freed with ::cuEventDestroy.

 Performing operations on the imported event after the exported event has
 been freed with ::cuEventDestroy will result in undefined behavior.

 IPC functionality is restricted to devices with support for unified
 addressing on Linux and Windows operating systems.
 IPC functionality on Windows is supported for compatibility purposes
 but not recommended as it comes with performance cost.
 Users can test their device for IPC functionality by calling
 ::cuapiDeviceGetAttribute with ::CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED

 \param phEvent - Returns the imported event
 \param handle  - Interprocess handle to open

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_MAP_FAILED,
 ::CUDA_ERROR_PEER_ACCESS_UNSUPPORTED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuEventCreate,
 ::cuEventDestroy,
 ::cuEventSynchronize,
 ::cuEventQuery,
 ::cuStreamWaitEvent,
 ::cuIpcGetEventHandle,
 ::cuIpcGetMemHandle,
 ::cuIpcOpenMemHandle,
 ::cuIpcCloseMemHandle,
 ::cudaIpcOpenEventHandle*/
    fn cuIpcOpenEventHandle(
        phEvent: *mut cuda_types::cuda::CUevent,
        handle: cuda_types::cuda::CUipcEventHandle,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets an interprocess memory handle for an existing device memory
 allocation

 Takes a pointer to the base of an existing device memory allocation created
 with ::cuMemAlloc and exports it for use in another process. This is a
 lightweight operation and may be called multiple times on an allocation
 without adverse effects.

 If a region of memory is freed with ::cuMemFree and a subsequent call
 to ::cuMemAlloc returns memory with the same device address,
 ::cuIpcGetMemHandle will return a unique handle for the
 new memory.

 IPC functionality is restricted to devices with support for unified
 addressing on Linux and Windows operating systems.
 IPC functionality on Windows is supported for compatibility purposes
 but not recommended as it comes with performance cost.
 Users can test their device for IPC functionality by calling
 ::cuapiDeviceGetAttribute with ::CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED

 \param pHandle - Pointer to user allocated ::CUipcMemHandle to return
                    the handle in.
 \param dptr    - Base pointer to previously allocated device memory

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_MAP_FAILED,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuMemAlloc,
 ::cuMemFree,
 ::cuIpcGetEventHandle,
 ::cuIpcOpenEventHandle,
 ::cuIpcOpenMemHandle,
 ::cuIpcCloseMemHandle,
 ::cudaIpcGetMemHandle*/
    fn cuIpcGetMemHandle(
        pHandle: *mut cuda_types::cuda::CUipcMemHandle,
        dptr: cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Opens an interprocess memory handle exported from another process
 and returns a device pointer usable in the local process.

 Maps memory exported from another process with ::cuIpcGetMemHandle into
 the current device address space. For contexts on different devices
 ::cuIpcOpenMemHandle can attempt to enable peer access between the
 devices as if the user called ::cuCtxEnablePeerAccess. This behavior is
 controlled by the ::CU_IPC_MEM_LAZY_ENABLE_PEER_ACCESS flag.
 ::cuDeviceCanAccessPeer can determine if a mapping is possible.

 Contexts that may open ::CUipcMemHandles are restricted in the following way.
 ::CUipcMemHandles from each ::CUdevice in a given process may only be opened
 by one ::CUcontext per ::CUdevice per other process.

 If the memory handle has already been opened by the current context, the
 reference count on the handle is incremented by 1 and the existing device pointer
 is returned.

 Memory returned from ::cuIpcOpenMemHandle must be freed with
 ::cuIpcCloseMemHandle.

 Calling ::cuMemFree on an exported memory region before calling
 ::cuIpcCloseMemHandle in the importing context will result in undefined
 behavior.

 IPC functionality is restricted to devices with support for unified
 addressing on Linux and Windows operating systems.
 IPC functionality on Windows is supported for compatibility purposes
 but not recommended as it comes with performance cost.
 Users can test their device for IPC functionality by calling
 ::cuapiDeviceGetAttribute with ::CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED

 \param pdptr  - Returned device pointer
 \param handle - ::CUipcMemHandle to open
 \param Flags  - Flags for this operation. Must be specified as ::CU_IPC_MEM_LAZY_ENABLE_PEER_ACCESS

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_MAP_FAILED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_TOO_MANY_PEERS,
 ::CUDA_ERROR_INVALID_VALUE

 \note No guarantees are made about the address returned in \p *pdptr.
 In particular, multiple processes may not receive the same address for the same \p handle.

 \sa
 ::cuMemAlloc,
 ::cuMemFree,
 ::cuIpcGetEventHandle,
 ::cuIpcOpenEventHandle,
 ::cuIpcGetMemHandle,
 ::cuIpcCloseMemHandle,
 ::cuCtxEnablePeerAccess,
 ::cuDeviceCanAccessPeer,
 ::cudaIpcOpenMemHandle*/
    fn cuIpcOpenMemHandle_v2(
        pdptr: *mut cuda_types::cuda::CUdeviceptr,
        handle: cuda_types::cuda::CUipcMemHandle,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Attempts to close memory mapped with ::cuIpcOpenMemHandle

 Decrements the reference count of the memory returned by ::cuIpcOpenMemHandle by 1.
 When the reference count reaches 0, this API unmaps the memory. The original allocation
 in the exporting process as well as imported mappings in other processes
 will be unaffected.

 Any resources used to enable peer access will be freed if this is the
 last mapping using them.

 IPC functionality is restricted to devices with support for unified
 addressing on Linux and Windows operating systems.
 IPC functionality on Windows is supported for compatibility purposes
 but not recommended as it comes with performance cost.
 Users can test their device for IPC functionality by calling
 ::cuapiDeviceGetAttribute with ::CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED

 \param dptr - Device pointer returned by ::cuIpcOpenMemHandle

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_MAP_FAILED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE
 \sa
 ::cuMemAlloc,
 ::cuMemFree,
 ::cuIpcGetEventHandle,
 ::cuIpcOpenEventHandle,
 ::cuIpcGetMemHandle,
 ::cuIpcOpenMemHandle,
 ::cudaIpcCloseMemHandle*/
    fn cuIpcCloseMemHandle(
        dptr: cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Registers an existing host memory range for use by CUDA

 Page-locks the memory range specified by \p p and \p bytesize and maps it
 for the device(s) as specified by \p Flags. This memory range also is added
 to the same tracking mechanism as ::cuMemHostAlloc to automatically accelerate
 calls to functions such as ::cuMemcpyHtoD(). Since the memory can be accessed
 directly by the device, it can be read or written with much higher bandwidth
 than pageable memory that has not been registered.  Page-locking excessive
 amounts of memory may degrade system performance, since it reduces the amount
 of memory available to the system for paging. As a result, this function is
 best used sparingly to register staging areas for data exchange between
 host and device.

 On systems where ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES
 is true, ::cuMemHostRegister will not page-lock the memory range specified
 by \p ptr but only populate unpopulated pages.

 The \p Flags parameter enables different options to be specified that
 affect the allocation, as follows.

 - ::CU_MEMHOSTREGISTER_PORTABLE: The memory returned by this call will be
   considered as pinned memory by all CUDA contexts, not just the one that
   performed the allocation.

 - ::CU_MEMHOSTREGISTER_DEVICEMAP: Maps the allocation into the CUDA address
   space. The device pointer to the memory may be obtained by calling
   ::cuMemHostGetDevicePointer().

 - ::CU_MEMHOSTREGISTER_IOMEMORY: The pointer is treated as pointing to some
   I/O memory space, e.g. the PCI Express resource of a 3rd party device.

 - ::CU_MEMHOSTREGISTER_READ_ONLY: The pointer is treated as pointing to memory
   that is considered read-only by the device.  On platforms without
   ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES, this flag is
   required in order to register memory mapped to the CPU as read-only.  Support
   for the use of this flag can be queried from the device attribute
   ::CU_DEVICE_ATTRIBUTE_READ_ONLY_HOST_REGISTER_SUPPORTED.  Using this flag with
   a current context associated with a device that does not have this attribute
   set will cause ::cuMemHostRegister to error with CUDA_ERROR_NOT_SUPPORTED.

 All of these flags are orthogonal to one another: a developer may page-lock
 memory that is portable or mapped with no restrictions.

 The ::CU_MEMHOSTREGISTER_DEVICEMAP flag may be specified on CUDA contexts for
 devices that do not support mapped pinned memory. The failure is deferred
 to ::cuMemHostGetDevicePointer() because the memory may be mapped into
 other CUDA contexts via the ::CU_MEMHOSTREGISTER_PORTABLE flag.

 For devices that have a non-zero value for the device attribute
 ::CU_DEVICE_ATTRIBUTE_CAN_USE_HOST_POINTER_FOR_REGISTERED_MEM, the memory
 can also be accessed from the device using the host pointer \p p.
 The device pointer returned by ::cuMemHostGetDevicePointer() may or may not
 match the original host pointer \p ptr and depends on the devices visible to the
 application. If all devices visible to the application have a non-zero value for the
 device attribute, the device pointer returned by ::cuMemHostGetDevicePointer()
 will match the original pointer \p ptr. If any device visible to the application
 has a zero value for the device attribute, the device pointer returned by
 ::cuMemHostGetDevicePointer() will not match the original host pointer \p ptr,
 but it will be suitable for use on all devices provided Unified Virtual Addressing
 is enabled. In such systems, it is valid to access the memory using either pointer
 on devices that have a non-zero value for the device attribute. Note however that
 such devices should access the memory using only of the two pointers and not both.

 The memory page-locked by this function must be unregistered with
 ::cuMemHostUnregister().

 \param p        - Host pointer to memory to page-lock
 \param bytesize - Size in bytes of the address range to page-lock
 \param Flags    - Flags for allocation request

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa
 ::cuMemHostUnregister,
 ::cuMemHostGetFlags,
 ::cuMemHostGetDevicePointer,
 ::cudaHostRegister*/
    fn cuMemHostRegister_v2(
        p: *mut ::core::ffi::c_void,
        bytesize: usize,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unregisters a memory range that was registered with cuMemHostRegister.

 Unmaps the memory range whose base address is specified by \p p, and makes
 it pageable again.

 The base address must be the same one specified to ::cuMemHostRegister().

 \param p - Host pointer to memory to unregister

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED,
 \notefnerr

 \sa
 ::cuMemHostRegister,
 ::cudaHostUnregister*/
    fn cuMemHostUnregister(p: *mut ::core::ffi::c_void) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory

 Copies data between two pointers.
 \p dst and \p src are base pointers of the destination and source, respectively.
 \p ByteCount specifies the number of bytes to copy.
 Note that this function infers the type of the transfer (host to host, host to
   device, device to device, or device to host) from the pointer values.  This
   function is only allowed in contexts which support unified addressing.

 \param dst - Destination unified virtual address space pointer
 \param src - Source unified virtual address space pointer
 \param ByteCount - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpy,
 ::cudaMemcpyToSymbol,
 ::cudaMemcpyFromSymbol*/
    fn cuMemcpy_ptds(
        dst: cuda_types::cuda::CUdeviceptr,
        src: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies device memory between two contexts

 Copies from device memory in one context to device memory in another
 context. \p dstDevice is the base device pointer of the destination memory
 and \p dstContext is the destination context.  \p srcDevice is the base
 device pointer of the source memory and \p srcContext is the source pointer.
 \p ByteCount specifies the number of bytes to copy.

 \param dstDevice  - Destination device pointer
 \param dstContext - Destination context
 \param srcDevice  - Source device pointer
 \param srcContext - Source context
 \param ByteCount  - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync

 \sa ::cuMemcpyDtoD, ::cuMemcpy3DPeer, ::cuMemcpyDtoDAsync, ::cuMemcpyPeerAsync,
 ::cuMemcpy3DPeerAsync,
 ::cudaMemcpyPeer*/
    fn cuMemcpyPeer_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstContext: cuda_types::cuda::CUcontext,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        srcContext: cuda_types::cuda::CUcontext,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Host to Device

 Copies from host memory to device memory. \p dstDevice and \p srcHost are
 the base addresses of the destination and source, respectively. \p ByteCount
 specifies the number of bytes to copy.

 \param dstDevice - Destination device pointer
 \param srcHost   - Source host pointer
 \param ByteCount - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpy,
 ::cudaMemcpyToSymbol*/
    fn cuMemcpyHtoD_v2_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Device to Host

 Copies from device to host memory. \p dstHost and \p srcDevice specify the
 base pointers of the destination and source, respectively. \p ByteCount
 specifies the number of bytes to copy.

 \param dstHost   - Destination host pointer
 \param srcDevice - Source device pointer
 \param ByteCount - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpy,
 ::cudaMemcpyFromSymbol*/
    fn cuMemcpyDtoH_v2_ptds(
        dstHost: *mut ::core::ffi::c_void,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Device to Device

 Copies from device memory to device memory. \p dstDevice and \p srcDevice
 are the base pointers of the destination and source, respectively.
 \p ByteCount specifies the number of bytes to copy.

 \param dstDevice - Destination device pointer
 \param srcDevice - Source device pointer
 \param ByteCount - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpy,
 ::cudaMemcpyToSymbol,
 ::cudaMemcpyFromSymbol*/
    fn cuMemcpyDtoD_v2_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Device to Array

 Copies from device memory to a 1D CUDA array. \p dstArray and \p dstOffset
 specify the CUDA array handle and starting index of the destination data.
 \p srcDevice specifies the base pointer of the source. \p ByteCount
 specifies the number of bytes to copy.

 \param dstArray  - Destination array
 \param dstOffset - Offset in bytes of destination array
 \param srcDevice - Source device pointer
 \param ByteCount - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpyToArray*/
    fn cuMemcpyDtoA_v2_ptds(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: usize,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Array to Device

 Copies from one 1D CUDA array to device memory. \p dstDevice specifies the
 base pointer of the destination and must be naturally aligned with the CUDA
 array elements. \p srcArray and \p srcOffset specify the CUDA array handle
 and the offset in bytes into the array where the copy is to begin.
 \p ByteCount specifies the number of bytes to copy and must be evenly
 divisible by the array element size.

 \param dstDevice - Destination device pointer
 \param srcArray  - Source array
 \param srcOffset - Offset in bytes of source array
 \param ByteCount - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpyFromArray*/
    fn cuMemcpyAtoD_v2_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: usize,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Host to Array

 Copies from host memory to a 1D CUDA array. \p dstArray and \p dstOffset
 specify the CUDA array handle and starting offset in bytes of the destination
 data.  \p pSrc specifies the base address of the source. \p ByteCount specifies
 the number of bytes to copy.

 \param dstArray  - Destination array
 \param dstOffset - Offset in bytes of destination array
 \param srcHost   - Source host pointer
 \param ByteCount - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpyToArray*/
    fn cuMemcpyHtoA_v2_ptds(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: usize,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Array to Host

 Copies from one 1D CUDA array to host memory. \p dstHost specifies the base
 pointer of the destination. \p srcArray and \p srcOffset specify the CUDA
 array handle and starting offset in bytes of the source data.
 \p ByteCount specifies the number of bytes to copy.

 \param dstHost   - Destination device pointer
 \param srcArray  - Source array
 \param srcOffset - Offset in bytes of source array
 \param ByteCount - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpyFromArray*/
    fn cuMemcpyAtoH_v2_ptds(
        dstHost: *mut ::core::ffi::c_void,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: usize,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Array to Array

 Copies from one 1D CUDA array to another. \p dstArray and \p srcArray
 specify the handles of the destination and source CUDA arrays for the copy,
 respectively. \p dstOffset and \p srcOffset specify the destination and
 source offsets in bytes into the CUDA arrays. \p ByteCount is the number of
 bytes to be copied. The size of the elements in the CUDA arrays need not be
 the same format, but the elements must be the same size; and count must be
 evenly divisible by that size.

 \param dstArray  - Destination array
 \param dstOffset - Offset in bytes of destination array
 \param srcArray  - Source array
 \param srcOffset - Offset in bytes of source array
 \param ByteCount - Size of memory copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpyArrayToArray*/
    fn cuMemcpyAtoA_v2_ptds(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: usize,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: usize,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory for 2D arrays

 Perform a 2D memory copy according to the parameters specified in \p pCopy.
 The ::CUDA_MEMCPY2D structure is defined as:

 \code
typedef struct CUDA_MEMCPY2D_st {
unsigned int srcXInBytes, srcY;
CUmemorytype srcMemoryType;
const void *srcHost;
CUdeviceptr srcDevice;
CUarray srcArray;
unsigned int srcPitch;

unsigned int dstXInBytes, dstY;
CUmemorytype dstMemoryType;
void *dstHost;
CUdeviceptr dstDevice;
CUarray dstArray;
unsigned int dstPitch;

unsigned int WidthInBytes;
unsigned int Height;
} CUDA_MEMCPY2D;
 \endcode
 where:
 - ::srcMemoryType and ::dstMemoryType specify the type of memory of the
   source and destination, respectively; ::CUmemorytype_enum is defined as:

 \code
typedef enum CUmemorytype_enum {
CU_MEMORYTYPE_HOST = 0x01,
CU_MEMORYTYPE_DEVICE = 0x02,
CU_MEMORYTYPE_ARRAY = 0x03,
CU_MEMORYTYPE_UNIFIED = 0x04
} CUmemorytype;
 \endcode

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::srcDevice and ::srcPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::srcArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_HOST, ::srcHost and ::srcPitch
 specify the (host) base address of the source data and the bytes per row to
 apply. ::srcArray is ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_DEVICE, ::srcDevice and ::srcPitch
 specify the (device) base address of the source data and the bytes per row
 to apply. ::srcArray is ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_ARRAY, ::srcArray specifies the
 handle of the source data. ::srcHost, ::srcDevice and ::srcPitch are
 ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_HOST, ::dstHost and ::dstPitch
 specify the (host) base address of the destination data and the bytes per
 row to apply. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::dstDevice and ::dstPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::dstArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_DEVICE, ::dstDevice and ::dstPitch
 specify the (device) base address of the destination data and the bytes per
 row to apply. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_ARRAY, ::dstArray specifies the
 handle of the destination data. ::dstHost, ::dstDevice and ::dstPitch are
 ignored.

 - ::srcXInBytes and ::srcY specify the base address of the source data for
   the copy.

 \par
 For host pointers, the starting address is
 \code
void* Start = (void*)((char*)srcHost+srcY*srcPitch + srcXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr Start = srcDevice+srcY*srcPitch+srcXInBytes;
 \endcode

 \par
 For CUDA arrays, ::srcXInBytes must be evenly divisible by the array
 element size.

 - ::dstXInBytes and ::dstY specify the base address of the destination data
   for the copy.

 \par
 For host pointers, the base address is
 \code
void* dstStart = (void*)((char*)dstHost+dstY*dstPitch + dstXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr dstStart = dstDevice+dstY*dstPitch+dstXInBytes;
 \endcode

 \par
 For CUDA arrays, ::dstXInBytes must be evenly divisible by the array
 element size.

 - ::WidthInBytes and ::Height specify the width (in bytes) and height of
   the 2D copy being performed.
 - If specified, ::srcPitch must be greater than or equal to ::WidthInBytes +
   ::srcXInBytes, and ::dstPitch must be greater than or equal to
   ::WidthInBytes + dstXInBytes.

 \par
 ::cuMemcpy2D() returns an error if any pitch is greater than the maximum
 allowed (::CU_DEVICE_ATTRIBUTE_MAX_PITCH). ::cuMemAllocPitch() passes back
 pitches that always work with ::cuMemcpy2D(). On intra-device memory copies
 (device to device, CUDA array to device, CUDA array to CUDA array),
 ::cuMemcpy2D() may fail for pitches not computed by ::cuMemAllocPitch().
 ::cuMemcpy2DUnaligned() does not have this restriction, but may run
 significantly slower in the cases where ::cuMemcpy2D() would have returned
 an error code.

 \param pCopy - Parameters for the memory copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpy2D,
 ::cudaMemcpy2DToArray,
 ::cudaMemcpy2DFromArray*/
    fn cuMemcpy2D_v2_ptds(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory for 2D arrays

 Perform a 2D memory copy according to the parameters specified in \p pCopy.
 The ::CUDA_MEMCPY2D structure is defined as:

 \code
typedef struct CUDA_MEMCPY2D_st {
unsigned int srcXInBytes, srcY;
CUmemorytype srcMemoryType;
const void *srcHost;
CUdeviceptr srcDevice;
CUarray srcArray;
unsigned int srcPitch;
unsigned int dstXInBytes, dstY;
CUmemorytype dstMemoryType;
void *dstHost;
CUdeviceptr dstDevice;
CUarray dstArray;
unsigned int dstPitch;
unsigned int WidthInBytes;
unsigned int Height;
} CUDA_MEMCPY2D;
 \endcode
 where:
 - ::srcMemoryType and ::dstMemoryType specify the type of memory of the
   source and destination, respectively; ::CUmemorytype_enum is defined as:

 \code
typedef enum CUmemorytype_enum {
CU_MEMORYTYPE_HOST = 0x01,
CU_MEMORYTYPE_DEVICE = 0x02,
CU_MEMORYTYPE_ARRAY = 0x03,
CU_MEMORYTYPE_UNIFIED = 0x04
} CUmemorytype;
 \endcode

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::srcDevice and ::srcPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::srcArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_HOST, ::srcHost and ::srcPitch
 specify the (host) base address of the source data and the bytes per row to
 apply. ::srcArray is ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_DEVICE, ::srcDevice and ::srcPitch
 specify the (device) base address of the source data and the bytes per row
 to apply. ::srcArray is ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_ARRAY, ::srcArray specifies the
 handle of the source data. ::srcHost, ::srcDevice and ::srcPitch are
 ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::dstDevice and ::dstPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::dstArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_HOST, ::dstHost and ::dstPitch
 specify the (host) base address of the destination data and the bytes per
 row to apply. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_DEVICE, ::dstDevice and ::dstPitch
 specify the (device) base address of the destination data and the bytes per
 row to apply. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_ARRAY, ::dstArray specifies the
 handle of the destination data. ::dstHost, ::dstDevice and ::dstPitch are
 ignored.

 - ::srcXInBytes and ::srcY specify the base address of the source data for
   the copy.

 \par
 For host pointers, the starting address is
 \code
void* Start = (void*)((char*)srcHost+srcY*srcPitch + srcXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr Start = srcDevice+srcY*srcPitch+srcXInBytes;
 \endcode

 \par
 For CUDA arrays, ::srcXInBytes must be evenly divisible by the array
 element size.

 - ::dstXInBytes and ::dstY specify the base address of the destination data
   for the copy.

 \par
 For host pointers, the base address is
 \code
void* dstStart = (void*)((char*)dstHost+dstY*dstPitch + dstXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr dstStart = dstDevice+dstY*dstPitch+dstXInBytes;
 \endcode

 \par
 For CUDA arrays, ::dstXInBytes must be evenly divisible by the array
 element size.

 - ::WidthInBytes and ::Height specify the width (in bytes) and height of
   the 2D copy being performed.
 - If specified, ::srcPitch must be greater than or equal to ::WidthInBytes +
   ::srcXInBytes, and ::dstPitch must be greater than or equal to
   ::WidthInBytes + dstXInBytes.

 \par
 ::cuMemcpy2D() returns an error if any pitch is greater than the maximum
 allowed (::CU_DEVICE_ATTRIBUTE_MAX_PITCH). ::cuMemAllocPitch() passes back
 pitches that always work with ::cuMemcpy2D(). On intra-device memory copies
 (device to device, CUDA array to device, CUDA array to CUDA array),
 ::cuMemcpy2D() may fail for pitches not computed by ::cuMemAllocPitch().
 ::cuMemcpy2DUnaligned() does not have this restriction, but may run
 significantly slower in the cases where ::cuMemcpy2D() would have returned
 an error code.

 \param pCopy - Parameters for the memory copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpy2D,
 ::cudaMemcpy2DToArray,
 ::cudaMemcpy2DFromArray*/
    fn cuMemcpy2DUnaligned_v2_ptds(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory for 3D arrays

 Perform a 3D memory copy according to the parameters specified in
 \p pCopy. The ::CUDA_MEMCPY3D structure is defined as:

 \code
typedef struct CUDA_MEMCPY3D_st {

unsigned int srcXInBytes, srcY, srcZ;
unsigned int srcLOD;
CUmemorytype srcMemoryType;
const void *srcHost;
CUdeviceptr srcDevice;
CUarray srcArray;
unsigned int srcPitch;  // ignored when src is array
unsigned int srcHeight; // ignored when src is array; may be 0 if Depth==1

unsigned int dstXInBytes, dstY, dstZ;
unsigned int dstLOD;
CUmemorytype dstMemoryType;
void *dstHost;
CUdeviceptr dstDevice;
CUarray dstArray;
unsigned int dstPitch;  // ignored when dst is array
unsigned int dstHeight; // ignored when dst is array; may be 0 if Depth==1

unsigned int WidthInBytes;
unsigned int Height;
unsigned int Depth;
} CUDA_MEMCPY3D;
 \endcode
 where:
 - ::srcMemoryType and ::dstMemoryType specify the type of memory of the
   source and destination, respectively; ::CUmemorytype_enum is defined as:

 \code
typedef enum CUmemorytype_enum {
CU_MEMORYTYPE_HOST = 0x01,
CU_MEMORYTYPE_DEVICE = 0x02,
CU_MEMORYTYPE_ARRAY = 0x03,
CU_MEMORYTYPE_UNIFIED = 0x04
} CUmemorytype;
 \endcode

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::srcDevice and ::srcPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::srcArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_HOST, ::srcHost, ::srcPitch and
 ::srcHeight specify the (host) base address of the source data, the bytes
 per row, and the height of each 2D slice of the 3D array. ::srcArray is
 ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_DEVICE, ::srcDevice, ::srcPitch and
 ::srcHeight specify the (device) base address of the source data, the bytes
 per row, and the height of each 2D slice of the 3D array. ::srcArray is
 ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_ARRAY, ::srcArray specifies the
 handle of the source data. ::srcHost, ::srcDevice, ::srcPitch and
 ::srcHeight are ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::dstDevice and ::dstPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::dstArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_HOST, ::dstHost and ::dstPitch
 specify the (host) base address of the destination data, the bytes per row,
 and the height of each 2D slice of the 3D array. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_DEVICE, ::dstDevice and ::dstPitch
 specify the (device) base address of the destination data, the bytes per
 row, and the height of each 2D slice of the 3D array. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_ARRAY, ::dstArray specifies the
 handle of the destination data. ::dstHost, ::dstDevice, ::dstPitch and
 ::dstHeight are ignored.

 - ::srcXInBytes, ::srcY and ::srcZ specify the base address of the source
   data for the copy.

 \par
 For host pointers, the starting address is
 \code
void* Start = (void*)((char*)srcHost+(srcZ*srcHeight+srcY)*srcPitch + srcXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr Start = srcDevice+(srcZ*srcHeight+srcY)*srcPitch+srcXInBytes;
 \endcode

 \par
 For CUDA arrays, ::srcXInBytes must be evenly divisible by the array
 element size.

 - dstXInBytes, ::dstY and ::dstZ specify the base address of the
   destination data for the copy.

 \par
 For host pointers, the base address is
 \code
void* dstStart = (void*)((char*)dstHost+(dstZ*dstHeight+dstY)*dstPitch + dstXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr dstStart = dstDevice+(dstZ*dstHeight+dstY)*dstPitch+dstXInBytes;
 \endcode

 \par
 For CUDA arrays, ::dstXInBytes must be evenly divisible by the array
 element size.

 - ::WidthInBytes, ::Height and ::Depth specify the width (in bytes), height
   and depth of the 3D copy being performed.
 - If specified, ::srcPitch must be greater than or equal to ::WidthInBytes +
   ::srcXInBytes, and ::dstPitch must be greater than or equal to
   ::WidthInBytes + dstXInBytes.
 - If specified, ::srcHeight must be greater than or equal to ::Height +
   ::srcY, and ::dstHeight must be greater than or equal to ::Height + ::dstY.

 \par
 ::cuMemcpy3D() returns an error if any pitch is greater than the maximum
 allowed (::CU_DEVICE_ATTRIBUTE_MAX_PITCH).

 The ::srcLOD and ::dstLOD members of the ::CUDA_MEMCPY3D structure must be
 set to 0.

 \param pCopy - Parameters for the memory copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMemcpy3D*/
    fn cuMemcpy3D_v2_ptds(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory between contexts

 Perform a 3D memory copy according to the parameters specified in
 \p pCopy.  See the definition of the ::CUDA_MEMCPY3D_PEER structure
 for documentation of its parameters.

 \param pCopy - Parameters for the memory copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_sync

 \sa ::cuMemcpyDtoD, ::cuMemcpyPeer, ::cuMemcpyDtoDAsync, ::cuMemcpyPeerAsync,
 ::cuMemcpy3DPeerAsync,
 ::cudaMemcpy3DPeer*/
    fn cuMemcpy3DPeer_ptds(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_PEER,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory asynchronously

 Copies data between two pointers.
 \p dst and \p src are base pointers of the destination and source, respectively.
 \p ByteCount specifies the number of bytes to copy.
 Note that this function infers the type of the transfer (host to host, host to
   device, device to device, or device to host) from the pointer values.  This
   function is only allowed in contexts which support unified addressing.

 \param dst       - Destination unified virtual address space pointer
 \param src       - Source unified virtual address space pointer
 \param ByteCount - Size of memory copy in bytes
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemcpyAsync,
 ::cudaMemcpyToSymbolAsync,
 ::cudaMemcpyFromSymbolAsync*/
    fn cuMemcpyAsync_ptsz(
        dst: cuda_types::cuda::CUdeviceptr,
        src: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies device memory between two contexts asynchronously.

 Copies from device memory in one context to device memory in another
 context. \p dstDevice is the base device pointer of the destination memory
 and \p dstContext is the destination context.  \p srcDevice is the base
 device pointer of the source memory and \p srcContext is the source pointer.
 \p ByteCount specifies the number of bytes to copy.

 \param dstDevice  - Destination device pointer
 \param dstContext - Destination context
 \param srcDevice  - Source device pointer
 \param srcContext - Source context
 \param ByteCount  - Size of memory copy in bytes
 \param hStream    - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuMemcpyDtoD, ::cuMemcpyPeer, ::cuMemcpy3DPeer, ::cuMemcpyDtoDAsync,
 ::cuMemcpy3DPeerAsync,
 ::cudaMemcpyPeerAsync*/
    fn cuMemcpyPeerAsync_ptsz(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstContext: cuda_types::cuda::CUcontext,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        srcContext: cuda_types::cuda::CUcontext,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Host to Device

 Copies from host memory to device memory. \p dstDevice and \p srcHost are
 the base addresses of the destination and source, respectively. \p ByteCount
 specifies the number of bytes to copy.

 \param dstDevice - Destination device pointer
 \param srcHost   - Source host pointer
 \param ByteCount - Size of memory copy in bytes
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemcpyAsync,
 ::cudaMemcpyToSymbolAsync*/
    fn cuMemcpyHtoDAsync_v2_ptsz(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Device to Host

 Copies from device to host memory. \p dstHost and \p srcDevice specify the
 base pointers of the destination and source, respectively. \p ByteCount
 specifies the number of bytes to copy.

 \param dstHost   - Destination host pointer
 \param srcDevice - Source device pointer
 \param ByteCount - Size of memory copy in bytes
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemcpyAsync,
 ::cudaMemcpyFromSymbolAsync*/
    fn cuMemcpyDtoHAsync_v2_ptsz(
        dstHost: *mut ::core::ffi::c_void,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Device to Device

 Copies from device memory to device memory. \p dstDevice and \p srcDevice
 are the base pointers of the destination and source, respectively.
 \p ByteCount specifies the number of bytes to copy.

 \param dstDevice - Destination device pointer
 \param srcDevice - Source device pointer
 \param ByteCount - Size of memory copy in bytes
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemcpyAsync,
 ::cudaMemcpyToSymbolAsync,
 ::cudaMemcpyFromSymbolAsync*/
    fn cuMemcpyDtoDAsync_v2_ptsz(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Host to Array

 Copies from host memory to a 1D CUDA array. \p dstArray and \p dstOffset
 specify the CUDA array handle and starting offset in bytes of the
 destination data. \p srcHost specifies the base address of the source.
 \p ByteCount specifies the number of bytes to copy.

 \param dstArray  - Destination array
 \param dstOffset - Offset in bytes of destination array
 \param srcHost   - Source host pointer
 \param ByteCount - Size of memory copy in bytes
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemcpyToArrayAsync*/
    fn cuMemcpyHtoAAsync_v2_ptsz(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: usize,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory from Array to Host

 Copies from one 1D CUDA array to host memory. \p dstHost specifies the base
 pointer of the destination. \p srcArray and \p srcOffset specify the CUDA
 array handle and starting offset in bytes of the source data.
 \p ByteCount specifies the number of bytes to copy.

 \param dstHost   - Destination pointer
 \param srcArray  - Source array
 \param srcOffset - Offset in bytes of source array
 \param ByteCount - Size of memory copy in bytes
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream
 \note_memcpy

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemcpyFromArrayAsync*/
    fn cuMemcpyAtoHAsync_v2_ptsz(
        dstHost: *mut ::core::ffi::c_void,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: usize,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory for 2D arrays

 Perform a 2D memory copy according to the parameters specified in \p pCopy.
 The ::CUDA_MEMCPY2D structure is defined as:

 \code
typedef struct CUDA_MEMCPY2D_st {
unsigned int srcXInBytes, srcY;
CUmemorytype srcMemoryType;
const void *srcHost;
CUdeviceptr srcDevice;
CUarray srcArray;
unsigned int srcPitch;
unsigned int dstXInBytes, dstY;
CUmemorytype dstMemoryType;
void *dstHost;
CUdeviceptr dstDevice;
CUarray dstArray;
unsigned int dstPitch;
unsigned int WidthInBytes;
unsigned int Height;
} CUDA_MEMCPY2D;
 \endcode
 where:
 - ::srcMemoryType and ::dstMemoryType specify the type of memory of the
   source and destination, respectively; ::CUmemorytype_enum is defined as:

 \code
typedef enum CUmemorytype_enum {
CU_MEMORYTYPE_HOST = 0x01,
CU_MEMORYTYPE_DEVICE = 0x02,
CU_MEMORYTYPE_ARRAY = 0x03,
CU_MEMORYTYPE_UNIFIED = 0x04
} CUmemorytype;
 \endcode

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_HOST, ::srcHost and ::srcPitch
 specify the (host) base address of the source data and the bytes per row to
 apply. ::srcArray is ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::srcDevice and ::srcPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::srcArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_DEVICE, ::srcDevice and ::srcPitch
 specify the (device) base address of the source data and the bytes per row
 to apply. ::srcArray is ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_ARRAY, ::srcArray specifies the
 handle of the source data. ::srcHost, ::srcDevice and ::srcPitch are
 ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::dstDevice and ::dstPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::dstArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_HOST, ::dstHost and ::dstPitch
 specify the (host) base address of the destination data and the bytes per
 row to apply. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_DEVICE, ::dstDevice and ::dstPitch
 specify the (device) base address of the destination data and the bytes per
 row to apply. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_ARRAY, ::dstArray specifies the
 handle of the destination data. ::dstHost, ::dstDevice and ::dstPitch are
 ignored.

 - ::srcXInBytes and ::srcY specify the base address of the source data for
   the copy.

 \par
 For host pointers, the starting address is
 \code
void* Start = (void*)((char*)srcHost+srcY*srcPitch + srcXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr Start = srcDevice+srcY*srcPitch+srcXInBytes;
 \endcode

 \par
 For CUDA arrays, ::srcXInBytes must be evenly divisible by the array
 element size.

 - ::dstXInBytes and ::dstY specify the base address of the destination data
   for the copy.

 \par
 For host pointers, the base address is
 \code
void* dstStart = (void*)((char*)dstHost+dstY*dstPitch + dstXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr dstStart = dstDevice+dstY*dstPitch+dstXInBytes;
 \endcode

 \par
 For CUDA arrays, ::dstXInBytes must be evenly divisible by the array
 element size.

 - ::WidthInBytes and ::Height specify the width (in bytes) and height of
   the 2D copy being performed.
 - If specified, ::srcPitch must be greater than or equal to ::WidthInBytes +
   ::srcXInBytes, and ::dstPitch must be greater than or equal to
   ::WidthInBytes + dstXInBytes.
 - If specified, ::srcPitch must be greater than or equal to ::WidthInBytes +
   ::srcXInBytes, and ::dstPitch must be greater than or equal to
   ::WidthInBytes + dstXInBytes.
 - If specified, ::srcHeight must be greater than or equal to ::Height +
   ::srcY, and ::dstHeight must be greater than or equal to ::Height + ::dstY.

 \par
 ::cuMemcpy2DAsync() returns an error if any pitch is greater than the maximum
 allowed (::CU_DEVICE_ATTRIBUTE_MAX_PITCH). ::cuMemAllocPitch() passes back
 pitches that always work with ::cuMemcpy2D(). On intra-device memory copies
 (device to device, CUDA array to device, CUDA array to CUDA array),
 ::cuMemcpy2DAsync() may fail for pitches not computed by ::cuMemAllocPitch().

 \param pCopy   - Parameters for the memory copy
 \param hStream - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemcpy2DAsync,
 ::cudaMemcpy2DToArrayAsync,
 ::cudaMemcpy2DFromArrayAsync*/
    fn cuMemcpy2DAsync_v2_ptsz(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory for 3D arrays

 Perform a 3D memory copy according to the parameters specified in
 \p pCopy. The ::CUDA_MEMCPY3D structure is defined as:

 \code
typedef struct CUDA_MEMCPY3D_st {

unsigned int srcXInBytes, srcY, srcZ;
unsigned int srcLOD;
CUmemorytype srcMemoryType;
const void *srcHost;
CUdeviceptr srcDevice;
CUarray srcArray;
unsigned int srcPitch;  // ignored when src is array
unsigned int srcHeight; // ignored when src is array; may be 0 if Depth==1

unsigned int dstXInBytes, dstY, dstZ;
unsigned int dstLOD;
CUmemorytype dstMemoryType;
void *dstHost;
CUdeviceptr dstDevice;
CUarray dstArray;
unsigned int dstPitch;  // ignored when dst is array
unsigned int dstHeight; // ignored when dst is array; may be 0 if Depth==1

unsigned int WidthInBytes;
unsigned int Height;
unsigned int Depth;
} CUDA_MEMCPY3D;
 \endcode
 where:
 - ::srcMemoryType and ::dstMemoryType specify the type of memory of the
   source and destination, respectively; ::CUmemorytype_enum is defined as:

 \code
typedef enum CUmemorytype_enum {
CU_MEMORYTYPE_HOST = 0x01,
CU_MEMORYTYPE_DEVICE = 0x02,
CU_MEMORYTYPE_ARRAY = 0x03,
CU_MEMORYTYPE_UNIFIED = 0x04
} CUmemorytype;
 \endcode

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::srcDevice and ::srcPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::srcArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_HOST, ::srcHost, ::srcPitch and
 ::srcHeight specify the (host) base address of the source data, the bytes
 per row, and the height of each 2D slice of the 3D array. ::srcArray is
 ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_DEVICE, ::srcDevice, ::srcPitch and
 ::srcHeight specify the (device) base address of the source data, the bytes
 per row, and the height of each 2D slice of the 3D array. ::srcArray is
 ignored.

 \par
 If ::srcMemoryType is ::CU_MEMORYTYPE_ARRAY, ::srcArray specifies the
 handle of the source data. ::srcHost, ::srcDevice, ::srcPitch and
 ::srcHeight are ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_UNIFIED, ::dstDevice and ::dstPitch
   specify the (unified virtual address space) base address of the source data
   and the bytes per row to apply.  ::dstArray is ignored.
 This value may be used only if unified addressing is supported in the calling
   context.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_HOST, ::dstHost and ::dstPitch
 specify the (host) base address of the destination data, the bytes per row,
 and the height of each 2D slice of the 3D array. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_DEVICE, ::dstDevice and ::dstPitch
 specify the (device) base address of the destination data, the bytes per
 row, and the height of each 2D slice of the 3D array. ::dstArray is ignored.

 \par
 If ::dstMemoryType is ::CU_MEMORYTYPE_ARRAY, ::dstArray specifies the
 handle of the destination data. ::dstHost, ::dstDevice, ::dstPitch and
 ::dstHeight are ignored.

 - ::srcXInBytes, ::srcY and ::srcZ specify the base address of the source
   data for the copy.

 \par
 For host pointers, the starting address is
 \code
void* Start = (void*)((char*)srcHost+(srcZ*srcHeight+srcY)*srcPitch + srcXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr Start = srcDevice+(srcZ*srcHeight+srcY)*srcPitch+srcXInBytes;
 \endcode

 \par
 For CUDA arrays, ::srcXInBytes must be evenly divisible by the array
 element size.

 - dstXInBytes, ::dstY and ::dstZ specify the base address of the
   destination data for the copy.

 \par
 For host pointers, the base address is
 \code
void* dstStart = (void*)((char*)dstHost+(dstZ*dstHeight+dstY)*dstPitch + dstXInBytes);
 \endcode

 \par
 For device pointers, the starting address is
 \code
CUdeviceptr dstStart = dstDevice+(dstZ*dstHeight+dstY)*dstPitch+dstXInBytes;
 \endcode

 \par
 For CUDA arrays, ::dstXInBytes must be evenly divisible by the array
 element size.

 - ::WidthInBytes, ::Height and ::Depth specify the width (in bytes), height
   and depth of the 3D copy being performed.
 - If specified, ::srcPitch must be greater than or equal to ::WidthInBytes +
   ::srcXInBytes, and ::dstPitch must be greater than or equal to
   ::WidthInBytes + dstXInBytes.
 - If specified, ::srcHeight must be greater than or equal to ::Height +
   ::srcY, and ::dstHeight must be greater than or equal to ::Height + ::dstY.

 \par
 ::cuMemcpy3DAsync() returns an error if any pitch is greater than the maximum
 allowed (::CU_DEVICE_ATTRIBUTE_MAX_PITCH).

 The ::srcLOD and ::dstLOD members of the ::CUDA_MEMCPY3D structure must be
 set to 0.

 \param pCopy - Parameters for the memory copy
 \param hStream - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemcpy3DAsync*/
    fn cuMemcpy3DAsync_v2_ptsz(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies memory between contexts asynchronously.

 Perform a 3D memory copy according to the parameters specified in
 \p pCopy.  See the definition of the ::CUDA_MEMCPY3D_PEER structure
 for documentation of its parameters.

 \param pCopy - Parameters for the memory copy
 \param hStream - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuMemcpyDtoD, ::cuMemcpyPeer, ::cuMemcpyDtoDAsync, ::cuMemcpyPeerAsync,
 ::cuMemcpy3DPeerAsync,
 ::cudaMemcpy3DPeerAsync*/
    fn cuMemcpy3DPeerAsync_ptsz(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_PEER,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Performs a batch of memory copies asynchronously.

 Performs a batch of memory copies. The batch as a whole executes in stream order but copies within a
 batch are not guaranteed to execute in any specific order. This API only supports pointer-to-pointer copies.
 For copies involving CUDA arrays, please see ::cuMemcpy3DBatchAsync.

 Performs memory copies from source buffers specified in \p srcs to destination buffers specified in \p dsts.
 The size of each copy is specified in \p sizes. All three arrays must be of the same length as specified
 by \p count. Since there are no ordering guarantees for copies within a batch, specifying any dependent copies
 within a batch will result in undefined behavior.

 Every copy in the batch has to be associated with a set of attributes specified in the \p attrs array.
 Each entry in this array can apply to more than one copy. This can be done by specifying in the \p attrsIdxs array,
 the index of the first copy that the corresponding entry in the \p attrs array applies to. Both \p attrs and
 \p attrsIdxs must be of the same length as specified by \p numAttrs. For example, if a batch has 10 copies listed
 in dst/src/sizes, the first 6 of which have one set of attributes and the remaining 4 another, then \p numAttrs
 will be 2, \p attrsIdxs will be {0, 6} and \p attrs will contains the two sets of attributes. Note that the first entry
 in \p attrsIdxs must always be 0. Also, each entry must be greater than the previous entry and the last entry should be
 less than \p count. Furthermore, \p numAttrs must be lesser than or equal to \p count.

 The ::CUmemcpyAttributes::srcAccessOrder indicates the source access ordering to be observed for copies associated
 with the attribute. If the source access order is set to ::CU_MEMCPY_SRC_ACCESS_ORDER_STREAM, then the source will
 be accessed in stream order. If the source access order is set to ::CU_MEMCPY_SRC_ACCESS_ORDER_DURING_API_CALL then
 it indicates that access to the source pointer can be out of stream order and all accesses must be complete before
 the API call returns. This flag is suited for ephemeral sources (ex., stack variables) when it's known that no prior
 operations in the stream can be accessing the memory and also that the lifetime of the memory is limited to the scope
 that the source variable was declared in. Specifying this flag allows the driver to optimize the copy and removes the
 need for the user to synchronize the stream after the API call. If the source access order is set to
 ::CU_MEMCPY_SRC_ACCESS_ORDER_ANY then it indicates that access to the source pointer can be out of stream order and the
 accesses can happen even after the API call returns. This flag is suited for host pointers allocated
 outside CUDA (ex., via malloc) when it's known that no prior operations in the stream can be accessing the memory.
 Specifying this flag allows the driver to optimize the copy on certain platforms. Each memcpy operation in the batch must
 have a valid ::CUmemcpyAttributes corresponding to it including the appropriate srcAccessOrder setting, otherwise the API
 will return ::CUDA_ERROR_INVALID_VALUE.

 The ::CUmemcpyAttributes::srcLocHint and ::CUmemcpyAttributes::dstLocHint allows applications to specify hint locations
 for operands of a copy when the operand doesn't have a fixed location. That is, these hints are
 only applicable for managed memory pointers on devices where ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS is true or
 system-allocated pageable memory on devices where ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS is true.
 For other cases, these hints are ignored.

 The ::CUmemcpyAttributes::flags field can be used to specify certain flags for copies. Setting the
 ::CU_MEMCPY_FLAG_PREFER_OVERLAP_WITH_COMPUTE flag indicates that the associated copies should preferably overlap with
 any compute work. Note that this flag is a hint and can be ignored depending on the platform and other parameters of the copy.

 If any error is encountered while parsing the batch, the index within the batch where the error was encountered
 will be returned in \p failIdx.

 \param dsts          - Array of destination pointers.
 \param srcs          - Array of memcpy source pointers.
 \param sizes         - Array of sizes for memcpy operations.
 \param count         - Size of \p dsts, \p srcs and \p sizes arrays
 \param attrs         - Array of memcpy attributes.
 \param attrsIdxs     - Array of indices to specify which copies each entry in the \p attrs array applies to.
The attributes specified in attrs[k] will be applied to copies starting from attrsIdxs[k]
through attrsIdxs[k+1] - 1. Also attrs[numAttrs-1] will apply to copies starting from
attrsIdxs[numAttrs-1] through count - 1.
 \param numAttrs      - Size of \p attrs and \p attrsIdxs arrays.
 \param failIdx       - Pointer to a location to return the index of the copy where a failure was encountered.
The value will be SIZE_MAX if the error doesn't pertain to any specific copy.
 \param hStream       - The stream to enqueue the operations in. Must not be legacy NULL stream.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_async
 \note_memcpy*/
    fn cuMemcpyBatchAsync_ptsz(
        dsts: *mut cuda_types::cuda::CUdeviceptr,
        srcs: *mut cuda_types::cuda::CUdeviceptr,
        sizes: *mut usize,
        count: usize,
        attrs: *mut cuda_types::cuda::CUmemcpyAttributes,
        attrsIdxs: *mut usize,
        numAttrs: usize,
        failIdx: *mut usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Performs a batch of 3D memory copies asynchronously.

 Performs a batch of memory copies. The batch as a whole executes in stream order but copies within a
 batch are not guaranteed to execute in any specific order. Note that this means specifying any dependent
 copies within a batch will result in undefined behavior.

 Performs memory copies as specified in the \p opList array. The length of this array is specified in \p numOps.
 Each entry in this array describes a copy operation. This includes among other things, the source and destination
 operands for the copy as specified in ::CUDA_MEMCPY3D_BATCH_OP::src and ::CUDA_MEMCPY3D_BATCH_OP::dst respectively.
 The source and destination operands of a copy can either be a pointer or a CUDA array. The width, height and depth
 of a copy is specified in ::CUDA_MEMCPY3D_BATCH_OP::extent. The width, height and depth of a copy are specified in
 elements and must not be zero. For pointer-to-pointer copies, the element size is considered to be 1. For pointer
 to CUDA array or vice versa copies, the element size is determined by the CUDA array. For CUDA array to CUDA array copies,
 the element size of the two CUDA arrays must match.

 For a given operand, if ::CUmemcpy3DOperand::type is specified as ::CU_MEMCPY_OPERAND_TYPE_POINTER, then
 ::CUmemcpy3DOperand::op::ptr will be used. The ::CUmemcpy3DOperand::op::ptr::ptr field must contain the pointer where
 the copy should begin. The ::CUmemcpy3DOperand::op::ptr::rowLength field specifies the length of each row in elements and
 must either be zero or be greater than or equal to the width of the copy specified in ::CUDA_MEMCPY3D_BATCH_OP::extent::width.
 The ::CUmemcpy3DOperand::op::ptr::layerHeight field specifies the height of each layer and must either be zero or be greater than
 or equal to the height of the copy specified in ::CUDA_MEMCPY3D_BATCH_OP::extent::height. When either of these values is zero,
 that aspect of the operand is considered to be tightly packed according to the copy extent. For managed memory pointers on devices where
 ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS is true or system-allocated pageable memory on devices where
 ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS is true, the ::CUmemcpy3DOperand::op::ptr::locHint field can be used to hint
 the location of the operand.

 If an operand's type is specified as ::CU_MEMCPY_OPERAND_TYPE_ARRAY, then ::CUmemcpy3DOperand::op::array will be used.
 The ::CUmemcpy3DOperand::op::array::array field specifies the CUDA array and ::CUmemcpy3DOperand::op::array::offset specifies
 the 3D offset into that array where the copy begins.

 The ::CUmemcpyAttributes::srcAccessOrder indicates the source access ordering to be observed for copies associated
 with the attribute. If the source access order is set to ::CU_MEMCPY_SRC_ACCESS_ORDER_STREAM, then the source will
 be accessed in stream order. If the source access order is set to ::CU_MEMCPY_SRC_ACCESS_ORDER_DURING_API_CALL then
 it indicates that access to the source pointer can be out of stream order and all accesses must be complete before
 the API call returns. This flag is suited for ephemeral sources (ex., stack variables) when it's known that no prior
 operations in the stream can be accessing the memory and also that the lifetime of the memory is limited to the scope
 that the source variable was declared in. Specifying this flag allows the driver to optimize the copy and removes the
 need for the user to synchronize the stream after the API call. If the source access order is set to
 ::CU_MEMCPY_SRC_ACCESS_ORDER_ANY then it indicates that access to the source pointer can be out of stream order and the
 accesses can happen even after the API call returns. This flag is suited for host pointers allocated
 outside CUDA (ex., via malloc) when it's known that no prior operations in the stream can be accessing the memory.
 Specifying this flag allows the driver to optimize the copy on certain platforms. Each memcopy operation in \p opList must
 have a valid srcAccessOrder setting, otherwise this API will return ::CUDA_ERROR_INVALID_VALUE.

 The ::CUmemcpyAttributes::flags field can be used to specify certain flags for copies. Setting the
 ::CU_MEMCPY_FLAG_PREFER_OVERLAP_WITH_COMPUTE flag indicates that the associated copies should preferably overlap with
 any compute work. Note that this flag is a hint and can be ignored depending on the platform and other parameters of the copy.

 If any error is encountered while parsing the batch, the index within the batch where the error was encountered
 will be returned in \p failIdx.

 \param numOps     - Total number of memcpy operations.
 \param opList     - Array of size \p numOps containing the actual memcpy operations.
 \param failIdx    - Pointer to a location to return the index of the copy where a failure was encountered.
                     The value will be SIZE_MAX if the error doesn't pertain to any specific copy.
 \param flags      - Flags for future use, must be zero now.
 \param hStream    - The stream to enqueue the operations in. Must not be default NULL stream.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_async
 \note_memcpy*/
    fn cuMemcpy3DBatchAsync_ptsz(
        numOps: usize,
        opList: *mut cuda_types::cuda::CUDA_MEMCPY3D_BATCH_OP,
        failIdx: *mut usize,
        flags: ::core::ffi::c_ulonglong,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Initializes device memory

 Sets the memory range of \p N 8-bit values to the specified value
 \p uc.

 \param dstDevice - Destination device pointer
 \param uc        - Value to set
 \param N         - Number of elements

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemset*/
    fn cuMemsetD8_v2_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        uc: ::core::ffi::c_uchar,
        N: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Initializes device memory

 Sets the memory range of \p N 16-bit values to the specified value
 \p us. The \p dstDevice pointer must be two byte aligned.

 \param dstDevice - Destination device pointer
 \param us        - Value to set
 \param N         - Number of elements

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemset*/
    fn cuMemsetD16_v2_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        us: ::core::ffi::c_ushort,
        N: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Initializes device memory

 Sets the memory range of \p N 32-bit values to the specified value
 \p ui. The \p dstDevice pointer must be four byte aligned.

 \param dstDevice - Destination device pointer
 \param ui        - Value to set
 \param N         - Number of elements

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32Async,
 ::cudaMemset*/
    fn cuMemsetD32_v2_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        ui: ::core::ffi::c_uint,
        N: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Initializes device memory

 Sets the 2D memory range of \p Width 8-bit values to the specified value
 \p uc. \p Height specifies the number of rows to set, and \p dstPitch
 specifies the number of bytes between each row. This function performs
 fastest when the pitch is one that has been passed back by
 ::cuMemAllocPitch().

 \param dstDevice - Destination device pointer
 \param dstPitch  - Pitch of destination device pointer(Unused if \p Height is 1)
 \param uc        - Value to set
 \param Width     - Width of row
 \param Height    - Number of rows

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemset2D*/
    fn cuMemsetD2D8_v2_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        uc: ::core::ffi::c_uchar,
        Width: usize,
        Height: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Initializes device memory

 Sets the 2D memory range of \p Width 16-bit values to the specified value
 \p us. \p Height specifies the number of rows to set, and \p dstPitch
 specifies the number of bytes between each row. The \p dstDevice pointer
 and \p dstPitch offset must be two byte aligned. This function performs
 fastest when the pitch is one that has been passed back by
 ::cuMemAllocPitch().

 \param dstDevice - Destination device pointer
 \param dstPitch  - Pitch of destination device pointer(Unused if \p Height is 1)
 \param us        - Value to set
 \param Width     - Width of row
 \param Height    - Number of rows

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemset2D*/
    fn cuMemsetD2D16_v2_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        us: ::core::ffi::c_ushort,
        Width: usize,
        Height: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Initializes device memory

 Sets the 2D memory range of \p Width 32-bit values to the specified value
 \p ui. \p Height specifies the number of rows to set, and \p dstPitch
 specifies the number of bytes between each row. The \p dstDevice pointer
 and \p dstPitch offset must be four byte aligned. This function performs
 fastest when the pitch is one that has been passed back by
 ::cuMemAllocPitch().

 \param dstDevice - Destination device pointer
 \param dstPitch  - Pitch of destination device pointer(Unused if \p Height is 1)
 \param ui        - Value to set
 \param Width     - Width of row
 \param Height    - Number of rows

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemset2D*/
    fn cuMemsetD2D32_v2_ptds(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        ui: ::core::ffi::c_uint,
        Width: usize,
        Height: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets device memory

 Sets the memory range of \p N 8-bit values to the specified value
 \p uc.

 \param dstDevice - Destination device pointer
 \param uc        - Value to set
 \param N         - Number of elements
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset
 \note_null_stream

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemsetAsync*/
    fn cuMemsetD8Async_ptsz(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        uc: ::core::ffi::c_uchar,
        N: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets device memory

 Sets the memory range of \p N 16-bit values to the specified value
 \p us. The \p dstDevice pointer must be two byte aligned.

 \param dstDevice - Destination device pointer
 \param us        - Value to set
 \param N         - Number of elements
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset
 \note_null_stream

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemsetAsync*/
    fn cuMemsetD16Async_ptsz(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        us: ::core::ffi::c_ushort,
        N: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets device memory

 Sets the memory range of \p N 32-bit values to the specified value
 \p ui. The \p dstDevice pointer must be four byte aligned.

 \param dstDevice - Destination device pointer
 \param ui        - Value to set
 \param N         - Number of elements
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset
 \note_null_stream

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async, ::cuMemsetD32,
 ::cudaMemsetAsync*/
    fn cuMemsetD32Async_ptsz(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        ui: ::core::ffi::c_uint,
        N: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets device memory

 Sets the 2D memory range of \p Width 8-bit values to the specified value
 \p uc. \p Height specifies the number of rows to set, and \p dstPitch
 specifies the number of bytes between each row. This function performs
 fastest when the pitch is one that has been passed back by
 ::cuMemAllocPitch().

 \param dstDevice - Destination device pointer
 \param dstPitch  - Pitch of destination device pointer(Unused if \p Height is 1)
 \param uc        - Value to set
 \param Width     - Width of row
 \param Height    - Number of rows
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset
 \note_null_stream

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemset2DAsync*/
    fn cuMemsetD2D8Async_ptsz(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        uc: ::core::ffi::c_uchar,
        Width: usize,
        Height: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets device memory

 Sets the 2D memory range of \p Width 16-bit values to the specified value
 \p us. \p Height specifies the number of rows to set, and \p dstPitch
 specifies the number of bytes between each row. The \p dstDevice pointer
 and \p dstPitch offset must be two byte aligned. This function performs
 fastest when the pitch is one that has been passed back by
 ::cuMemAllocPitch().

 \param dstDevice - Destination device pointer
 \param dstPitch  - Pitch of destination device pointer(Unused if \p Height is 1)
 \param us        - Value to set
 \param Width     - Width of row
 \param Height    - Number of rows
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset
 \note_null_stream

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D32, ::cuMemsetD2D32Async,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemset2DAsync*/
    fn cuMemsetD2D16Async_ptsz(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        us: ::core::ffi::c_ushort,
        Width: usize,
        Height: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets device memory

 Sets the 2D memory range of \p Width 32-bit values to the specified value
 \p ui. \p Height specifies the number of rows to set, and \p dstPitch
 specifies the number of bytes between each row. The \p dstDevice pointer
 and \p dstPitch offset must be four byte aligned. This function performs
 fastest when the pitch is one that has been passed back by
 ::cuMemAllocPitch().

 \param dstDevice - Destination device pointer
 \param dstPitch  - Pitch of destination device pointer(Unused if \p Height is 1)
 \param ui        - Value to set
 \param Width     - Width of row
 \param Height    - Number of rows
 \param hStream   - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr
 \note_memset
 \note_null_stream

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D8Async,
 ::cuMemsetD2D16, ::cuMemsetD2D16Async, ::cuMemsetD2D32,
 ::cuMemsetD8, ::cuMemsetD8Async, ::cuMemsetD16, ::cuMemsetD16Async,
 ::cuMemsetD32, ::cuMemsetD32Async,
 ::cudaMemset2DAsync*/
    fn cuMemsetD2D32Async_ptsz(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        ui: ::core::ffi::c_uint,
        Width: usize,
        Height: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a 1D or 2D CUDA array

 Creates a CUDA array according to the ::CUDA_ARRAY_DESCRIPTOR structure
 \p pAllocateArray and returns a handle to the new CUDA array in \p *pHandle.
 The ::CUDA_ARRAY_DESCRIPTOR is defined as:

 \code
typedef struct {
unsigned int Width;
unsigned int Height;
CUarray_format Format;
unsigned int NumChannels;
} CUDA_ARRAY_DESCRIPTOR;
 \endcode
 where:

 - \p Width, and \p Height are the width, and height of the CUDA array (in
 elements); the CUDA array is one-dimensional if height is 0, two-dimensional
 otherwise;
 - ::Format specifies the format of the elements; ::CUarray_format is
 defined as:
 \code
typedef enum CUarray_format_enum {
CU_AD_FORMAT_UNSIGNED_INT8 = 0x01,
CU_AD_FORMAT_UNSIGNED_INT16 = 0x02,
CU_AD_FORMAT_UNSIGNED_INT32 = 0x03,
CU_AD_FORMAT_SIGNED_INT8 = 0x08,
CU_AD_FORMAT_SIGNED_INT16 = 0x09,
CU_AD_FORMAT_SIGNED_INT32 = 0x0a,
CU_AD_FORMAT_HALF = 0x10,
CU_AD_FORMAT_FLOAT = 0x20,
CU_AD_FORMAT_NV12 = 0xb0,
CU_AD_FORMAT_UNORM_INT8X1 = 0xc0,
CU_AD_FORMAT_UNORM_INT8X2 = 0xc1,
CU_AD_FORMAT_UNORM_INT8X4 = 0xc2,
CU_AD_FORMAT_UNORM_INT16X1 = 0xc3,
CU_AD_FORMAT_UNORM_INT16X2 = 0xc4,
CU_AD_FORMAT_UNORM_INT16X4 = 0xc5,
CU_AD_FORMAT_SNORM_INT8X1 = 0xc6,
CU_AD_FORMAT_SNORM_INT8X2 = 0xc7,
CU_AD_FORMAT_SNORM_INT8X4 = 0xc8,
CU_AD_FORMAT_SNORM_INT16X1 = 0xc9,
CU_AD_FORMAT_SNORM_INT16X2 = 0xca,
CU_AD_FORMAT_SNORM_INT16X4 = 0xcb,
CU_AD_FORMAT_BC1_UNORM = 0x91,
CU_AD_FORMAT_BC1_UNORM_SRGB = 0x92,
CU_AD_FORMAT_BC2_UNORM = 0x93,
CU_AD_FORMAT_BC2_UNORM_SRGB = 0x94,
CU_AD_FORMAT_BC3_UNORM = 0x95,
CU_AD_FORMAT_BC3_UNORM_SRGB = 0x96,
CU_AD_FORMAT_BC4_UNORM = 0x97,
CU_AD_FORMAT_BC4_SNORM = 0x98,
CU_AD_FORMAT_BC5_UNORM = 0x99,
CU_AD_FORMAT_BC5_SNORM = 0x9a,
CU_AD_FORMAT_BC6H_UF16 = 0x9b,
CU_AD_FORMAT_BC6H_SF16 = 0x9c,
CU_AD_FORMAT_BC7_UNORM = 0x9d,
CU_AD_FORMAT_BC7_UNORM_SRGB = 0x9e,
CU_AD_FORMAT_P010 = 0x9f,
CU_AD_FORMAT_P016 = 0xa1,
CU_AD_FORMAT_NV16 = 0xa2,
CU_AD_FORMAT_P210 = 0xa3,
CU_AD_FORMAT_P216 = 0xa4,
CU_AD_FORMAT_YUY2 = 0xa5,
CU_AD_FORMAT_Y210 = 0xa6,
CU_AD_FORMAT_Y216 = 0xa7,
CU_AD_FORMAT_AYUV = 0xa8,
CU_AD_FORMAT_Y410 = 0xa9,
CU_AD_FORMAT_Y416 = 0xb1,
CU_AD_FORMAT_Y444_PLANAR8 = 0xb2,
CU_AD_FORMAT_Y444_PLANAR10 = 0xb3,
CU_AD_FORMAT_YUV444_8bit_SemiPlanar = 0xb4,
CU_AD_FORMAT_YUV444_16bit_SemiPlanar = 0xb5,
CU_AD_FORMAT_UNORM_INT_101010_2 = 0x50,
} CUarray_format;
  \endcode
 - \p NumChannels specifies the number of packed components per CUDA array
 element; it may be 1, 2, or 4;

 Here are examples of CUDA array descriptions:

 Description for a CUDA array of 2048 floats:
 \code
CUDA_ARRAY_DESCRIPTOR desc;
desc.Format = CU_AD_FORMAT_FLOAT;
desc.NumChannels = 1;
desc.Width = 2048;
desc.Height = 1;
 \endcode

 Description for a 64 x 64 CUDA array of floats:
 \code
CUDA_ARRAY_DESCRIPTOR desc;
desc.Format = CU_AD_FORMAT_FLOAT;
desc.NumChannels = 1;
desc.Width = 64;
desc.Height = 64;
 \endcode

 Description for a \p width x \p height CUDA array of 64-bit, 4x16-bit
 float16's:
 \code
CUDA_ARRAY_DESCRIPTOR desc;
desc.Format = CU_AD_FORMAT_HALF;
desc.NumChannels = 4;
desc.Width = width;
desc.Height = height;
 \endcode

 Description for a \p width x \p height CUDA array of 16-bit elements, each
 of which is two 8-bit unsigned chars:
 \code
CUDA_ARRAY_DESCRIPTOR arrayDesc;
desc.Format = CU_AD_FORMAT_UNSIGNED_INT8;
desc.NumChannels = 2;
desc.Width = width;
desc.Height = height;
 \endcode

 \param pHandle        - Returned array
 \param pAllocateArray - Array descriptor

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMallocArray*/
    fn cuArrayCreate_v2(
        pHandle: *mut cuda_types::cuda::CUarray,
        pAllocateArray: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get a 1D or 2D CUDA array descriptor

 Returns in \p *pArrayDescriptor a descriptor containing information on the
 format and dimensions of the CUDA array \p hArray. It is useful for
 subroutines that have been passed a CUDA array, but need to know the CUDA
 array parameters for validation or other purposes.

 \param pArrayDescriptor - Returned array descriptor
 \param hArray           - Array to get descriptor of

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaArrayGetInfo*/
    fn cuArrayGetDescriptor_v2(
        pArrayDescriptor: *mut cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR,
        hArray: cuda_types::cuda::CUarray,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the layout properties of a sparse CUDA array

 Returns the layout properties of a sparse CUDA array in \p sparseProperties
 If the CUDA array is not allocated with flag ::CUDA_ARRAY3D_SPARSE
 ::CUDA_ERROR_INVALID_VALUE will be returned.

 If the returned value in ::CUDA_ARRAY_SPARSE_PROPERTIES::flags contains ::CU_ARRAY_SPARSE_PROPERTIES_SINGLE_MIPTAIL,
 then ::CUDA_ARRAY_SPARSE_PROPERTIES::miptailSize represents the total size of the array. Otherwise, it will be zero.
 Also, the returned value in ::CUDA_ARRAY_SPARSE_PROPERTIES::miptailFirstLevel is always zero.
 Note that the \p array must have been allocated using ::cuArrayCreate or ::cuArray3DCreate. For CUDA arrays obtained
 using ::cuMipmappedArrayGetLevel, ::CUDA_ERROR_INVALID_VALUE will be returned. Instead, ::cuMipmappedArrayGetSparseProperties
 must be used to obtain the sparse properties of the entire CUDA mipmapped array to which \p array belongs to.

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE

 \param[out] sparseProperties - Pointer to ::CUDA_ARRAY_SPARSE_PROPERTIES
 \param[in] array - CUDA array to get the sparse properties of
 \sa ::cuMipmappedArrayGetSparseProperties, ::cuMemMapArrayAsync*/
    fn cuArrayGetSparseProperties(
        sparseProperties: *mut cuda_types::cuda::CUDA_ARRAY_SPARSE_PROPERTIES,
        array: cuda_types::cuda::CUarray,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the layout properties of a sparse CUDA mipmapped array

 Returns the sparse array layout properties in \p sparseProperties
 If the CUDA mipmapped array is not allocated with flag ::CUDA_ARRAY3D_SPARSE
 ::CUDA_ERROR_INVALID_VALUE will be returned.

 For non-layered CUDA mipmapped arrays, ::CUDA_ARRAY_SPARSE_PROPERTIES::miptailSize returns the
 size of the mip tail region. The mip tail region includes all mip levels whose width, height or depth
 is less than that of the tile.
 For layered CUDA mipmapped arrays, if ::CUDA_ARRAY_SPARSE_PROPERTIES::flags contains ::CU_ARRAY_SPARSE_PROPERTIES_SINGLE_MIPTAIL,
 then ::CUDA_ARRAY_SPARSE_PROPERTIES::miptailSize specifies the size of the mip tail of all layers combined.
 Otherwise, ::CUDA_ARRAY_SPARSE_PROPERTIES::miptailSize specifies mip tail size per layer.
 The returned value of ::CUDA_ARRAY_SPARSE_PROPERTIES::miptailFirstLevel is valid only if ::CUDA_ARRAY_SPARSE_PROPERTIES::miptailSize is non-zero.

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE

 \param[out] sparseProperties - Pointer to ::CUDA_ARRAY_SPARSE_PROPERTIES
 \param[in] mipmap - CUDA mipmapped array to get the sparse properties of
 \sa ::cuArrayGetSparseProperties, ::cuMemMapArrayAsync*/
    fn cuMipmappedArrayGetSparseProperties(
        sparseProperties: *mut cuda_types::cuda::CUDA_ARRAY_SPARSE_PROPERTIES,
        mipmap: cuda_types::cuda::CUmipmappedArray,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the memory requirements of a CUDA array

 Returns the memory requirements of a CUDA array in \p memoryRequirements
 If the CUDA array is not allocated with flag ::CUDA_ARRAY3D_DEFERRED_MAPPING
 ::CUDA_ERROR_INVALID_VALUE will be returned.

 The returned value in ::CUDA_ARRAY_MEMORY_REQUIREMENTS::size
 represents the total size of the CUDA array.
 The returned value in ::CUDA_ARRAY_MEMORY_REQUIREMENTS::alignment
 represents the alignment necessary for mapping the CUDA array.

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE

 \param[out] memoryRequirements - Pointer to ::CUDA_ARRAY_MEMORY_REQUIREMENTS
 \param[in] array - CUDA array to get the memory requirements of
 \param[in] device - Device to get the memory requirements for
 \sa ::cuMipmappedArrayGetMemoryRequirements, ::cuMemMapArrayAsync*/
    fn cuArrayGetMemoryRequirements(
        memoryRequirements: *mut cuda_types::cuda::CUDA_ARRAY_MEMORY_REQUIREMENTS,
        array: cuda_types::cuda::CUarray,
        device: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the memory requirements of a CUDA mipmapped array

 Returns the memory requirements of a CUDA mipmapped array in \p memoryRequirements
 If the CUDA mipmapped array is not allocated with flag ::CUDA_ARRAY3D_DEFERRED_MAPPING
 ::CUDA_ERROR_INVALID_VALUE will be returned.

 The returned value in ::CUDA_ARRAY_MEMORY_REQUIREMENTS::size
 represents the total size of the CUDA mipmapped array.
 The returned value in ::CUDA_ARRAY_MEMORY_REQUIREMENTS::alignment
 represents the alignment necessary for mapping the CUDA mipmapped
 array.

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE

 \param[out] memoryRequirements - Pointer to ::CUDA_ARRAY_MEMORY_REQUIREMENTS
 \param[in] mipmap - CUDA mipmapped array to get the memory requirements of
 \param[in] device - Device to get the memory requirements for
 \sa ::cuArrayGetMemoryRequirements, ::cuMemMapArrayAsync*/
    fn cuMipmappedArrayGetMemoryRequirements(
        memoryRequirements: *mut cuda_types::cuda::CUDA_ARRAY_MEMORY_REQUIREMENTS,
        mipmap: cuda_types::cuda::CUmipmappedArray,
        device: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets a CUDA array plane from a CUDA array

 Returns in \p pPlaneArray a CUDA array that represents a single format plane
 of the CUDA array \p hArray.

 If \p planeIdx is greater than the maximum number of planes in this array or if the array does
 not have a multi-planar format e.g: ::CU_AD_FORMAT_NV12, then ::CUDA_ERROR_INVALID_VALUE is returned.

 Note that if the \p hArray has format ::CU_AD_FORMAT_NV12, then passing in 0 for \p planeIdx returns
 a CUDA array of the same size as \p hArray but with one channel and ::CU_AD_FORMAT_UNSIGNED_INT8 as its format.
 If 1 is passed for \p planeIdx, then the returned CUDA array has half the height and width
 of \p hArray with two channels and ::CU_AD_FORMAT_UNSIGNED_INT8 as its format.

 \param pPlaneArray   - Returned CUDA array referenced by the \p planeIdx
 \param hArray        - Multiplanar CUDA array
 \param planeIdx      - Plane index

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa
 ::cuArrayCreate,
 ::cudaArrayGetPlane*/
    fn cuArrayGetPlane(
        pPlaneArray: *mut cuda_types::cuda::CUarray,
        hArray: cuda_types::cuda::CUarray,
        planeIdx: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys a CUDA array

 Destroys the CUDA array \p hArray.

 \param hArray - Array to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ARRAY_IS_MAPPED,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaFreeArray*/
    fn cuArrayDestroy(hArray: cuda_types::cuda::CUarray) -> cuda_types::cuda::CUresult;
    /** \brief Creates a 3D CUDA array

 Creates a CUDA array according to the ::CUDA_ARRAY3D_DESCRIPTOR structure
 \p pAllocateArray and returns a handle to the new CUDA array in \p *pHandle.
 The ::CUDA_ARRAY3D_DESCRIPTOR is defined as:

 \code
typedef struct {
unsigned int Width;
unsigned int Height;
unsigned int Depth;
CUarray_format Format;
unsigned int NumChannels;
unsigned int Flags;
} CUDA_ARRAY3D_DESCRIPTOR;
 \endcode
 where:

 - \p Width, \p Height, and \p Depth are the width, height, and depth of the
 CUDA array (in elements); the following types of CUDA arrays can be allocated:
     - A 1D array is allocated if \p Height and \p Depth extents are both zero.
     - A 2D array is allocated if only \p Depth extent is zero.
     - A 3D array is allocated if all three extents are non-zero.
     - A 1D layered CUDA array is allocated if only \p Height is zero and the
       ::CUDA_ARRAY3D_LAYERED flag is set. Each layer is a 1D array. The number
       of layers is determined by the depth extent.
     - A 2D layered CUDA array is allocated if all three extents are non-zero and
       the ::CUDA_ARRAY3D_LAYERED flag is set. Each layer is a 2D array. The number
       of layers is determined by the depth extent.
     - A cubemap CUDA array is allocated if all three extents are non-zero and the
       ::CUDA_ARRAY3D_CUBEMAP flag is set. \p Width must be equal to \p Height, and
       \p Depth must be six. A cubemap is a special type of 2D layered CUDA array,
       where the six layers represent the six faces of a cube. The order of the six
       layers in memory is the same as that listed in ::CUarray_cubemap_face.
     - A cubemap layered CUDA array is allocated if all three extents are non-zero,
       and both, ::CUDA_ARRAY3D_CUBEMAP and ::CUDA_ARRAY3D_LAYERED flags are set.
       \p Width must be equal to \p Height, and \p Depth must be a multiple of six.
       A cubemap layered CUDA array is a special type of 2D layered CUDA array that
       consists of a collection of cubemaps. The first six layers represent the first
       cubemap, the next six layers form the second cubemap, and so on.

 - ::Format specifies the format of the elements; ::CUarray_format is
 defined as:
 \code
typedef enum CUarray_format_enum {
CU_AD_FORMAT_UNSIGNED_INT8 = 0x01,
CU_AD_FORMAT_UNSIGNED_INT16 = 0x02,
CU_AD_FORMAT_UNSIGNED_INT32 = 0x03,
CU_AD_FORMAT_SIGNED_INT8 = 0x08,
CU_AD_FORMAT_SIGNED_INT16 = 0x09,
CU_AD_FORMAT_SIGNED_INT32 = 0x0a,
CU_AD_FORMAT_HALF = 0x10,
CU_AD_FORMAT_FLOAT = 0x20,
CU_AD_FORMAT_NV12 = 0xb0,
CU_AD_FORMAT_UNORM_INT8X1 = 0xc0,
CU_AD_FORMAT_UNORM_INT8X2 = 0xc1,
CU_AD_FORMAT_UNORM_INT8X4 = 0xc2,
CU_AD_FORMAT_UNORM_INT16X1 = 0xc3,
CU_AD_FORMAT_UNORM_INT16X2 = 0xc4,
CU_AD_FORMAT_UNORM_INT16X4 = 0xc5,
CU_AD_FORMAT_SNORM_INT8X1 = 0xc6,
CU_AD_FORMAT_SNORM_INT8X2 = 0xc7,
CU_AD_FORMAT_SNORM_INT8X4 = 0xc8,
CU_AD_FORMAT_SNORM_INT16X1 = 0xc9,
CU_AD_FORMAT_SNORM_INT16X2 = 0xca,
CU_AD_FORMAT_SNORM_INT16X4 = 0xcb,
CU_AD_FORMAT_BC1_UNORM = 0x91,
CU_AD_FORMAT_BC1_UNORM_SRGB = 0x92,
CU_AD_FORMAT_BC2_UNORM = 0x93,
CU_AD_FORMAT_BC2_UNORM_SRGB = 0x94,
CU_AD_FORMAT_BC3_UNORM = 0x95,
CU_AD_FORMAT_BC3_UNORM_SRGB = 0x96,
CU_AD_FORMAT_BC4_UNORM = 0x97,
CU_AD_FORMAT_BC4_SNORM = 0x98,
CU_AD_FORMAT_BC5_UNORM = 0x99,
CU_AD_FORMAT_BC5_SNORM = 0x9a,
CU_AD_FORMAT_BC6H_UF16 = 0x9b,
CU_AD_FORMAT_BC6H_SF16 = 0x9c,
CU_AD_FORMAT_BC7_UNORM = 0x9d,
CU_AD_FORMAT_BC7_UNORM_SRGB = 0x9e,
CU_AD_FORMAT_P010 = 0x9f,
CU_AD_FORMAT_P016 = 0xa1,
CU_AD_FORMAT_NV16 = 0xa2,
CU_AD_FORMAT_P210 = 0xa3,
CU_AD_FORMAT_P216 = 0xa4,
CU_AD_FORMAT_YUY2 = 0xa5,
CU_AD_FORMAT_Y210 = 0xa6,
CU_AD_FORMAT_Y216 = 0xa7,
CU_AD_FORMAT_AYUV = 0xa8,
CU_AD_FORMAT_Y410 = 0xa9,
CU_AD_FORMAT_Y416 = 0xb1,
CU_AD_FORMAT_Y444_PLANAR8 = 0xb2,
CU_AD_FORMAT_Y444_PLANAR10 = 0xb3,
CU_AD_FORMAT_YUV444_8bit_SemiPlanar = 0xb4,
CU_AD_FORMAT_YUV444_16bit_SemiPlanar = 0xb5,
CU_AD_FORMAT_UNORM_INT_101010_2 = 0x50,
} CUarray_format;
  \endcode

 - \p NumChannels specifies the number of packed components per CUDA array
 element; it may be 1, 2, or 4;

 - ::Flags may be set to
   - ::CUDA_ARRAY3D_LAYERED to enable creation of layered CUDA arrays. If this flag is set,
     \p Depth specifies the number of layers, not the depth of a 3D array.
   - ::CUDA_ARRAY3D_SURFACE_LDST to enable surface references to be bound to the CUDA array.
     If this flag is not set, ::cuSurfRefSetArray will fail when attempting to bind the CUDA array
     to a surface reference.
   - ::CUDA_ARRAY3D_CUBEMAP to enable creation of cubemaps. If this flag is set, \p Width must be
     equal to \p Height, and \p Depth must be six. If the ::CUDA_ARRAY3D_LAYERED flag is also set,
     then \p Depth must be a multiple of six.
   - ::CUDA_ARRAY3D_TEXTURE_GATHER to indicate that the CUDA array will be used for texture gather.
     Texture gather can only be performed on 2D CUDA arrays.

 \p Width, \p Height and \p Depth must meet certain size requirements as listed in the following table.
 All values are specified in elements. Note that for brevity's sake, the full name of the device attribute
 is not specified. For ex., TEXTURE1D_WIDTH refers to the device attribute
 ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH.

 Note that 2D CUDA arrays have different size requirements if the ::CUDA_ARRAY3D_TEXTURE_GATHER flag
 is set. \p Width and \p Height must not be greater than ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_WIDTH
 and ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_HEIGHT respectively, in that case.

 <table>
 <tr><td><b>CUDA array type</b></td>
 <td><b>Valid extents that must always be met<br>{(width range in elements), (height range),
 (depth range)}</b></td>
 <td><b>Valid extents with CUDA_ARRAY3D_SURFACE_LDST set<br>
 {(width range in elements), (height range), (depth range)}</b></td></tr>
 <tr><td>1D</td>
 <td><small>{ (1,TEXTURE1D_WIDTH), 0, 0 }</small></td>
 <td><small>{ (1,SURFACE1D_WIDTH), 0, 0 }</small></td></tr>
 <tr><td>2D</td>
 <td><small>{ (1,TEXTURE2D_WIDTH), (1,TEXTURE2D_HEIGHT), 0 }</small></td>
 <td><small>{ (1,SURFACE2D_WIDTH), (1,SURFACE2D_HEIGHT), 0 }</small></td></tr>
 <tr><td>3D</td>
 <td><small>{ (1,TEXTURE3D_WIDTH), (1,TEXTURE3D_HEIGHT), (1,TEXTURE3D_DEPTH) }
 <br>OR<br>{ (1,TEXTURE3D_WIDTH_ALTERNATE), (1,TEXTURE3D_HEIGHT_ALTERNATE),
 (1,TEXTURE3D_DEPTH_ALTERNATE) }</small></td>
 <td><small>{ (1,SURFACE3D_WIDTH), (1,SURFACE3D_HEIGHT),
 (1,SURFACE3D_DEPTH) }</small></td></tr>
 <tr><td>1D Layered</td>
 <td><small>{ (1,TEXTURE1D_LAYERED_WIDTH), 0,
 (1,TEXTURE1D_LAYERED_LAYERS) }</small></td>
 <td><small>{ (1,SURFACE1D_LAYERED_WIDTH), 0,
 (1,SURFACE1D_LAYERED_LAYERS) }</small></td></tr>
 <tr><td>2D Layered</td>
 <td><small>{ (1,TEXTURE2D_LAYERED_WIDTH), (1,TEXTURE2D_LAYERED_HEIGHT),
 (1,TEXTURE2D_LAYERED_LAYERS) }</small></td>
 <td><small>{ (1,SURFACE2D_LAYERED_WIDTH), (1,SURFACE2D_LAYERED_HEIGHT),
 (1,SURFACE2D_LAYERED_LAYERS) }</small></td></tr>
 <tr><td>Cubemap</td>
 <td><small>{ (1,TEXTURECUBEMAP_WIDTH), (1,TEXTURECUBEMAP_WIDTH), 6 }</small></td>
 <td><small>{ (1,SURFACECUBEMAP_WIDTH),
 (1,SURFACECUBEMAP_WIDTH), 6 }</small></td></tr>
 <tr><td>Cubemap Layered</td>
 <td><small>{ (1,TEXTURECUBEMAP_LAYERED_WIDTH), (1,TEXTURECUBEMAP_LAYERED_WIDTH),
 (1,TEXTURECUBEMAP_LAYERED_LAYERS) }</small></td>
 <td><small>{ (1,SURFACECUBEMAP_LAYERED_WIDTH), (1,SURFACECUBEMAP_LAYERED_WIDTH),
 (1,SURFACECUBEMAP_LAYERED_LAYERS) }</small></td></tr>
 </table>

 Here are examples of CUDA array descriptions:

 Description for a CUDA array of 2048 floats:
 \code
CUDA_ARRAY3D_DESCRIPTOR desc;
desc.Format = CU_AD_FORMAT_FLOAT;
desc.NumChannels = 1;
desc.Width = 2048;
desc.Height = 0;
desc.Depth = 0;
 \endcode

 Description for a 64 x 64 CUDA array of floats:
 \code
CUDA_ARRAY3D_DESCRIPTOR desc;
desc.Format = CU_AD_FORMAT_FLOAT;
desc.NumChannels = 1;
desc.Width = 64;
desc.Height = 64;
desc.Depth = 0;
 \endcode

 Description for a \p width x \p height x \p depth CUDA array of 64-bit,
 4x16-bit float16's:
 \code
CUDA_ARRAY3D_DESCRIPTOR desc;
desc.Format = CU_AD_FORMAT_HALF;
desc.NumChannels = 4;
desc.Width = width;
desc.Height = height;
desc.Depth = depth;
 \endcode

 \param pHandle        - Returned array
 \param pAllocateArray - 3D array descriptor

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuArray3DGetDescriptor, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaMalloc3DArray*/
    fn cuArray3DCreate_v2(
        pHandle: *mut cuda_types::cuda::CUarray,
        pAllocateArray: *const cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get a 3D CUDA array descriptor

 Returns in \p *pArrayDescriptor a descriptor containing information on the
 format and dimensions of the CUDA array \p hArray. It is useful for
 subroutines that have been passed a CUDA array, but need to know the CUDA
 array parameters for validation or other purposes.

 This function may be called on 1D and 2D arrays, in which case the \p Height
 and/or \p Depth members of the descriptor struct will be set to 0.

 \param pArrayDescriptor - Returned 3D array descriptor
 \param hArray           - 3D array to get descriptor of

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED
 \notefnerr

 \sa ::cuArray3DCreate, ::cuArrayCreate,
 ::cuArrayDestroy, ::cuArrayGetDescriptor, ::cuMemAlloc, ::cuMemAllocHost,
 ::cuMemAllocPitch, ::cuMemcpy2D, ::cuMemcpy2DAsync, ::cuMemcpy2DUnaligned,
 ::cuMemcpy3D, ::cuMemcpy3DAsync, ::cuMemcpyAtoA, ::cuMemcpyAtoD,
 ::cuMemcpyAtoH, ::cuMemcpyAtoHAsync, ::cuMemcpyDtoA, ::cuMemcpyDtoD, ::cuMemcpyDtoDAsync,
 ::cuMemcpyDtoH, ::cuMemcpyDtoHAsync, ::cuMemcpyHtoA, ::cuMemcpyHtoAAsync,
 ::cuMemcpyHtoD, ::cuMemcpyHtoDAsync, ::cuMemFree, ::cuMemFreeHost,
 ::cuMemGetAddressRange, ::cuMemGetInfo, ::cuMemHostAlloc,
 ::cuMemHostGetDevicePointer, ::cuMemsetD2D8, ::cuMemsetD2D16,
 ::cuMemsetD2D32, ::cuMemsetD8, ::cuMemsetD16, ::cuMemsetD32,
 ::cudaArrayGetInfo*/
    fn cuArray3DGetDescriptor_v2(
        pArrayDescriptor: *mut cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR,
        hArray: cuda_types::cuda::CUarray,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a CUDA mipmapped array

 Creates a CUDA mipmapped array according to the ::CUDA_ARRAY3D_DESCRIPTOR structure
 \p pMipmappedArrayDesc and returns a handle to the new CUDA mipmapped array in \p *pHandle.
 \p numMipmapLevels specifies the number of mipmap levels to be allocated. This value is
 clamped to the range [1, 1 + floor(log2(max(width, height, depth)))].

 The ::CUDA_ARRAY3D_DESCRIPTOR is defined as:

 \code
typedef struct {
unsigned int Width;
unsigned int Height;
unsigned int Depth;
CUarray_format Format;
unsigned int NumChannels;
unsigned int Flags;
} CUDA_ARRAY3D_DESCRIPTOR;
 \endcode
 where:

 - \p Width, \p Height, and \p Depth are the width, height, and depth of the
 CUDA array (in elements); the following types of CUDA arrays can be allocated:
     - A 1D mipmapped array is allocated if \p Height and \p Depth extents are both zero.
     - A 2D mipmapped array is allocated if only \p Depth extent is zero.
     - A 3D mipmapped array is allocated if all three extents are non-zero.
     - A 1D layered CUDA mipmapped array is allocated if only \p Height is zero and the
       ::CUDA_ARRAY3D_LAYERED flag is set. Each layer is a 1D array. The number
       of layers is determined by the depth extent.
     - A 2D layered CUDA mipmapped array is allocated if all three extents are non-zero and
       the ::CUDA_ARRAY3D_LAYERED flag is set. Each layer is a 2D array. The number
       of layers is determined by the depth extent.
     - A cubemap CUDA mipmapped array is allocated if all three extents are non-zero and the
       ::CUDA_ARRAY3D_CUBEMAP flag is set. \p Width must be equal to \p Height, and
       \p Depth must be six. A cubemap is a special type of 2D layered CUDA array,
       where the six layers represent the six faces of a cube. The order of the six
       layers in memory is the same as that listed in ::CUarray_cubemap_face.
     - A cubemap layered CUDA mipmapped array is allocated if all three extents are non-zero,
       and both, ::CUDA_ARRAY3D_CUBEMAP and ::CUDA_ARRAY3D_LAYERED flags are set.
       \p Width must be equal to \p Height, and \p Depth must be a multiple of six.
       A cubemap layered CUDA array is a special type of 2D layered CUDA array that
       consists of a collection of cubemaps. The first six layers represent the first
       cubemap, the next six layers form the second cubemap, and so on.

 - ::Format specifies the format of the elements; ::CUarray_format is
 defined as:
 \code
typedef enum CUarray_format_enum {
CU_AD_FORMAT_UNSIGNED_INT8 = 0x01,
CU_AD_FORMAT_UNSIGNED_INT16 = 0x02,
CU_AD_FORMAT_UNSIGNED_INT32 = 0x03,
CU_AD_FORMAT_SIGNED_INT8 = 0x08,
CU_AD_FORMAT_SIGNED_INT16 = 0x09,
CU_AD_FORMAT_SIGNED_INT32 = 0x0a,
CU_AD_FORMAT_HALF = 0x10,
CU_AD_FORMAT_FLOAT = 0x20,
CU_AD_FORMAT_NV12 = 0xb0,
CU_AD_FORMAT_UNORM_INT8X1 = 0xc0,
CU_AD_FORMAT_UNORM_INT8X2 = 0xc1,
CU_AD_FORMAT_UNORM_INT8X4 = 0xc2,
CU_AD_FORMAT_UNORM_INT16X1 = 0xc3,
CU_AD_FORMAT_UNORM_INT16X2 = 0xc4,
CU_AD_FORMAT_UNORM_INT16X4 = 0xc5,
CU_AD_FORMAT_SNORM_INT8X1 = 0xc6,
CU_AD_FORMAT_SNORM_INT8X2 = 0xc7,
CU_AD_FORMAT_SNORM_INT8X4 = 0xc8,
CU_AD_FORMAT_SNORM_INT16X1 = 0xc9,
CU_AD_FORMAT_SNORM_INT16X2 = 0xca,
CU_AD_FORMAT_SNORM_INT16X4 = 0xcb,
CU_AD_FORMAT_BC1_UNORM = 0x91,
CU_AD_FORMAT_BC1_UNORM_SRGB = 0x92,
CU_AD_FORMAT_BC2_UNORM = 0x93,
CU_AD_FORMAT_BC2_UNORM_SRGB = 0x94,
CU_AD_FORMAT_BC3_UNORM = 0x95,
CU_AD_FORMAT_BC3_UNORM_SRGB = 0x96,
CU_AD_FORMAT_BC4_UNORM = 0x97,
CU_AD_FORMAT_BC4_SNORM = 0x98,
CU_AD_FORMAT_BC5_UNORM = 0x99,
CU_AD_FORMAT_BC5_SNORM = 0x9a,
CU_AD_FORMAT_BC6H_UF16 = 0x9b,
CU_AD_FORMAT_BC6H_SF16 = 0x9c,
CU_AD_FORMAT_BC7_UNORM = 0x9d,
CU_AD_FORMAT_BC7_UNORM_SRGB = 0x9e,
CU_AD_FORMAT_P010 = 0x9f,
CU_AD_FORMAT_P016 = 0xa1,
CU_AD_FORMAT_NV16 = 0xa2,
CU_AD_FORMAT_P210 = 0xa3,
CU_AD_FORMAT_P216 = 0xa4,
CU_AD_FORMAT_YUY2 = 0xa5,
CU_AD_FORMAT_Y210 = 0xa6,
CU_AD_FORMAT_Y216 = 0xa7,
CU_AD_FORMAT_AYUV = 0xa8,
CU_AD_FORMAT_Y410 = 0xa9,
CU_AD_FORMAT_Y416 = 0xb1,
CU_AD_FORMAT_Y444_PLANAR8 = 0xb2,
CU_AD_FORMAT_Y444_PLANAR10 = 0xb3,
CU_AD_FORMAT_YUV444_8bit_SemiPlanar = 0xb4,
CU_AD_FORMAT_YUV444_16bit_SemiPlanar = 0xb5,
CU_AD_FORMAT_UNORM_INT_101010_2 = 0x50,
} CUarray_format;
  \endcode

 - \p NumChannels specifies the number of packed components per CUDA array
 element; it may be 1, 2, or 4;

 - ::Flags may be set to
   - ::CUDA_ARRAY3D_LAYERED to enable creation of layered CUDA mipmapped arrays. If this flag is set,
     \p Depth specifies the number of layers, not the depth of a 3D array.
   - ::CUDA_ARRAY3D_SURFACE_LDST to enable surface references to be bound to individual mipmap levels of
     the CUDA mipmapped array. If this flag is not set, ::cuSurfRefSetArray will fail when attempting to
     bind a mipmap level of the CUDA mipmapped array to a surface reference.
   - ::CUDA_ARRAY3D_CUBEMAP to enable creation of mipmapped cubemaps. If this flag is set, \p Width must be
     equal to \p Height, and \p Depth must be six. If the ::CUDA_ARRAY3D_LAYERED flag is also set,
     then \p Depth must be a multiple of six.
   - ::CUDA_ARRAY3D_TEXTURE_GATHER to indicate that the CUDA mipmapped array will be used for texture gather.
     Texture gather can only be performed on 2D CUDA mipmapped arrays.

 \p Width, \p Height and \p Depth must meet certain size requirements as listed in the following table.
 All values are specified in elements. Note that for brevity's sake, the full name of the device attribute
 is not specified. For ex., TEXTURE1D_MIPMAPPED_WIDTH refers to the device attribute
 ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH.

 <table>
 <tr><td><b>CUDA array type</b></td>
 <td><b>Valid extents that must always be met<br>{(width range in elements), (height range),
 (depth range)}</b></td>
 <td><b>Valid extents with CUDA_ARRAY3D_SURFACE_LDST set<br>
 {(width range in elements), (height range), (depth range)}</b></td></tr>
 <tr><td>1D</td>
 <td><small>{ (1,TEXTURE1D_MIPMAPPED_WIDTH), 0, 0 }</small></td>
 <td><small>{ (1,SURFACE1D_WIDTH), 0, 0 }</small></td></tr>
 <tr><td>2D</td>
 <td><small>{ (1,TEXTURE2D_MIPMAPPED_WIDTH), (1,TEXTURE2D_MIPMAPPED_HEIGHT), 0 }</small></td>
 <td><small>{ (1,SURFACE2D_WIDTH), (1,SURFACE2D_HEIGHT), 0 }</small></td></tr>
 <tr><td>3D</td>
 <td><small>{ (1,TEXTURE3D_WIDTH), (1,TEXTURE3D_HEIGHT), (1,TEXTURE3D_DEPTH) }
 <br>OR<br>{ (1,TEXTURE3D_WIDTH_ALTERNATE), (1,TEXTURE3D_HEIGHT_ALTERNATE),
 (1,TEXTURE3D_DEPTH_ALTERNATE) }</small></td>
 <td><small>{ (1,SURFACE3D_WIDTH), (1,SURFACE3D_HEIGHT),
 (1,SURFACE3D_DEPTH) }</small></td></tr>
 <tr><td>1D Layered</td>
 <td><small>{ (1,TEXTURE1D_LAYERED_WIDTH), 0,
 (1,TEXTURE1D_LAYERED_LAYERS) }</small></td>
 <td><small>{ (1,SURFACE1D_LAYERED_WIDTH), 0,
 (1,SURFACE1D_LAYERED_LAYERS) }</small></td></tr>
 <tr><td>2D Layered</td>
 <td><small>{ (1,TEXTURE2D_LAYERED_WIDTH), (1,TEXTURE2D_LAYERED_HEIGHT),
 (1,TEXTURE2D_LAYERED_LAYERS) }</small></td>
 <td><small>{ (1,SURFACE2D_LAYERED_WIDTH), (1,SURFACE2D_LAYERED_HEIGHT),
 (1,SURFACE2D_LAYERED_LAYERS) }</small></td></tr>
 <tr><td>Cubemap</td>
 <td><small>{ (1,TEXTURECUBEMAP_WIDTH), (1,TEXTURECUBEMAP_WIDTH), 6 }</small></td>
 <td><small>{ (1,SURFACECUBEMAP_WIDTH),
 (1,SURFACECUBEMAP_WIDTH), 6 }</small></td></tr>
 <tr><td>Cubemap Layered</td>
 <td><small>{ (1,TEXTURECUBEMAP_LAYERED_WIDTH), (1,TEXTURECUBEMAP_LAYERED_WIDTH),
 (1,TEXTURECUBEMAP_LAYERED_LAYERS) }</small></td>
 <td><small>{ (1,SURFACECUBEMAP_LAYERED_WIDTH), (1,SURFACECUBEMAP_LAYERED_WIDTH),
 (1,SURFACECUBEMAP_LAYERED_LAYERS) }</small></td></tr>
 </table>


 \param pHandle             - Returned mipmapped array
 \param pMipmappedArrayDesc - mipmapped array descriptor
 \param numMipmapLevels     - Number of mipmap levels

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cuMipmappedArrayDestroy,
 ::cuMipmappedArrayGetLevel,
 ::cuArrayCreate,
 ::cudaMallocMipmappedArray*/
    fn cuMipmappedArrayCreate(
        pHandle: *mut cuda_types::cuda::CUmipmappedArray,
        pMipmappedArrayDesc: *const cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR,
        numMipmapLevels: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets a mipmap level of a CUDA mipmapped array

 Returns in \p *pLevelArray a CUDA array that represents a single mipmap level
 of the CUDA mipmapped array \p hMipmappedArray.

 If \p level is greater than the maximum number of levels in this mipmapped array,
 ::CUDA_ERROR_INVALID_VALUE is returned.

 \param pLevelArray     - Returned mipmap level CUDA array
 \param hMipmappedArray - CUDA mipmapped array
 \param level           - Mipmap level

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa
 ::cuMipmappedArrayCreate,
 ::cuMipmappedArrayDestroy,
 ::cuArrayCreate,
 ::cudaGetMipmappedArrayLevel*/
    fn cuMipmappedArrayGetLevel(
        pLevelArray: *mut cuda_types::cuda::CUarray,
        hMipmappedArray: cuda_types::cuda::CUmipmappedArray,
        level: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys a CUDA mipmapped array

 Destroys the CUDA mipmapped array \p hMipmappedArray.

 \param hMipmappedArray - Mipmapped array to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ARRAY_IS_MAPPED,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED
 \notefnerr

 \sa
 ::cuMipmappedArrayCreate,
 ::cuMipmappedArrayGetLevel,
 ::cuArrayCreate,
 ::cudaFreeMipmappedArray*/
    fn cuMipmappedArrayDestroy(
        hMipmappedArray: cuda_types::cuda::CUmipmappedArray,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Retrieve handle for an address range

 Get a handle of the specified type to an address range. The address range
 must have been obtained by a prior call to either ::cuMemAlloc or ::cuMemAddressReserve.
 If the address range was obtained via ::cuMemAddressReserve, it must also be fully mapped via ::cuMemMap.
 The address range must have been obtained by a prior call to either ::cuMemAllocHost or
 ::cuMemHostAlloc on Tegra.

 Users must ensure the \p dptr and \p size are aligned to the host page size.

 When requesting CUmemRangeHandleType::CU_MEM_RANGE_HANDLE_TYPE_DMA_BUF_FD,
 users are expected to query for dma_buf support for the platform
 by using ::CU_DEVICE_ATTRIBUTE_DMA_BUF_SUPPORTED device attribute before calling
 this API. The \p handle will be interpreted as a pointer to an integer to store the dma_buf file descriptor.
 Users must ensure the entire address range is backed and mapped when
 the address range is allocated by ::cuMemAddressReserve. All the physical
 allocations backing the address range must be resident on the same device and
 have identical allocation properties. Users are also expected to retrieve a
 new handle every time the underlying physical allocation(s) corresponding
 to a previously queried VA range are changed.

 For CUmemRangeHandleType::CU_MEM_RANGE_HANDLE_TYPE_DMA_BUF_FD, users may set
 flags to ::CU_MEM_RANGE_FLAG_DMA_BUF_MAPPING_TYPE_PCIE. Which when set on a
 supported platform, will give a DMA_BUF handle mapped via PCIE BAR1 or will
 return an error otherwise.

 \param[out] handle     - Pointer to the location where the returned handle will be stored.
 \param[in] dptr        - Pointer to a valid CUDA device allocation. Must be aligned to host page size.
 \param[in] size        - Length of the address range. Must be aligned to host page size.
 \param[in] handleType  - Type of handle requested (defines type and size of the \p handle output parameter)
 \param[in] flags       - When requesting CUmemRangeHandleType::CU_MEM_RANGE_HANDLE_TYPE_DMA_BUF_FD the value could be
                          ::CU_MEM_RANGE_FLAG_DMA_BUF_MAPPING_TYPE_PCIE, otherwise 0.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED*/
    fn cuMemGetHandleForAddressRange(
        handle: *mut ::core::ffi::c_void,
        dptr: cuda_types::cuda::CUdeviceptr,
        size: usize,
        handleType: cuda_types::cuda::CUmemRangeHandleType,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief   Submit a batch of \p count independent decompression operations.

 \details Each of the \p count decompression operations is described by a
          single entry in the \p paramsArray array. Once the batch has been
          submitted, the function will return, and decompression will happen
          asynchronously w.r.t. the CPU. To the work completion tracking
          mechanisms in the CUDA driver, the batch will be considered a single
          unit of work and processed according to stream semantics, i.e., it
          is not possible to query the completion of individual decompression
          operations within a batch.

          The memory pointed to by each of ::CUmemDecompressParams.src,
          ::CUmemDecompressParams.dst, and ::CUmemDecompressParams.dstActBytes,
          must be capable of usage with the hardware decompress feature. That
          is, for each of said pointers, the pointer attribute
          ::CU_POINTER_ATTRIBUTE_IS_MEM_DECOMPRESS_CAPABLE should give a
          non-zero value. To ensure this, the memory backing the pointers
          should have been allocated using one of the following CUDA memory
          allocators:
          * ::cuMemAlloc()
          * ::cuMemCreate() with the usage flag ::CU_MEM_CREATE_USAGE_HW_DECOMPRESS
          * ::cuMemAllocFromPoolAsync() from a pool that was created with
            the usage flag ::CU_MEM_POOL_CREATE_USAGE_HW_DECOMPRESS
          Additionally, ::CUmemDecompressParams.src, ::CUmemDecompressParams.dst,
          and ::CUmemDecompressParams.dstActBytes, must all be accessible from
          the device associated with the context where \p stream was created.
          For information on how to ensure this, see the documentation for the
          allocator of interest.

 \param[in]  paramsArray  The array of structures describing the independent
                          decompression operations.
 \param[in]  count        The number of entries in \p paramsArray array.
 \param[in]  flags        Must be 0.
 \param[out] errorIndex   The index into \p paramsArray of the decompression
                          operation for which the error returned by this
                          function pertains to. If \p index is SIZE_MAX and
                          the value returned is not ::CUDA_SUCCESS, then the
                          error returned by this function should be considered
                          a general error that does not pertain to a
                          particular decompression operation. May be \p NULL,
                          in which case, no index will be recorded in the
                          event of error.
 \param[in]  stream       The stream where the work will be enqueued.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuMemAlloc, ::cuMemPoolCreate, ::cuMemAllocFromPoolAsync*/
    fn cuMemBatchDecompressAsync_ptsz(
        paramsArray: *mut cuda_types::cuda::CUmemDecompressParams,
        count: usize,
        flags: ::core::ffi::c_uint,
        errorIndex: *mut usize,
        stream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allocate an address range reservation.

 Reserves a virtual address range based on the given parameters, giving
 the starting address of the range in \p ptr.  This API requires a system that
 supports UVA.  The size and address parameters must be a multiple of the
 host page size and the alignment must be a power of two or zero for default
 alignment. If \p addr is 0, then the driver chooses the address at which to
 place the start of the reservation whereas when it is non-zero then the driver
 treats it as a hint about where to place the reservation.

 \param[out] ptr       - Resulting pointer to start of virtual address range allocated
 \param[in]  size      - Size of the reserved virtual address range requested
 \param[in]  alignment - Alignment of the reserved virtual address range requested
 \param[in]  addr      - Hint address for the start of the address range
 \param[in]  flags     - Currently unused, must be zero
 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMemAddressFree*/
    fn cuMemAddressReserve(
        ptr: *mut cuda_types::cuda::CUdeviceptr,
        size: usize,
        alignment: usize,
        addr: cuda_types::cuda::CUdeviceptr,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Free an address range reservation.

 Frees a virtual address range reserved by cuMemAddressReserve.  The size
 must match what was given to memAddressReserve and the ptr given must
 match what was returned from memAddressReserve.

 \param[in] ptr  - Starting address of the virtual address range to free
 \param[in] size - Size of the virtual address region to free
 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMemAddressReserve*/
    fn cuMemAddressFree(
        ptr: cuda_types::cuda::CUdeviceptr,
        size: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a CUDA memory handle representing a memory allocation of a given size described by the given properties

 This creates a memory allocation on the target device specified through the
 \p prop structure. The created allocation will not have any device or host
 mappings. The generic memory \p handle for the allocation can be
 mapped to the address space of calling process via ::cuMemMap. This handle
 cannot be transmitted directly to other processes (see
 ::cuMemExportToShareableHandle).  On Windows, the caller must also pass
 an LPSECURITYATTRIBUTE in \p prop to be associated with this handle which
 limits or allows access to this handle for a recipient process (see
 ::CUmemAllocationProp::win32HandleMetaData for more).  The \p size of this
 allocation must be a multiple of the the value given via
 ::cuMemGetAllocationGranularity with the ::CU_MEM_ALLOC_GRANULARITY_MINIMUM
 flag.
 To create a CPU allocation targeting a specific host NUMA node, applications must
 set ::CUmemAllocationProp::CUmemLocation::type to ::CU_MEM_LOCATION_TYPE_HOST_NUMA and
 ::CUmemAllocationProp::CUmemLocation::id must specify the NUMA ID of the CPU.
 On systems where NUMA is not available ::CUmemAllocationProp::CUmemLocation::id must be set to 0.
 Specifying ::CU_MEM_LOCATION_TYPE_HOST_NUMA_CURRENT or ::CU_MEM_LOCATION_TYPE_HOST as the
 ::CUmemLocation::type will result in ::CUDA_ERROR_INVALID_VALUE.

 Applications that intend to use ::CU_MEM_HANDLE_TYPE_FABRIC based memory sharing must ensure:
 (1) `nvidia-caps-imex-channels` character device is created by the driver and is listed under /proc/devices
 (2) have at least one IMEX channel file accessible by the user launching the application.

 When exporter and importer CUDA processes have been granted access to the same IMEX channel, they can securely
 share memory.

 The IMEX channel security model works on a per user basis. Which means all processes under a user can share
 memory if the user has access to a valid IMEX channel. When multi-user isolation is desired, a separate IMEX
 channel is required for each user.

 These channel files exist in /dev/nvidia-caps-imex-channels/channel* and can be created using standard OS
 native calls like mknod on Linux. For example: To create channel0 with the major number from /proc/devices
 users can execute the following command: `mknod /dev/nvidia-caps-imex-channels/channel0 c <major number> 0`

 If ::CUmemAllocationProp::allocFlags::usage contains ::CU_MEM_CREATE_USAGE_TILE_POOL flag then
 the memory allocation is intended only to be used as backing tile pool for sparse CUDA arrays
 and sparse CUDA mipmapped arrays.
 (see ::cuMemMapArrayAsync).

 \param[out] handle - Value of handle returned. All operations on this allocation are to be performed using this handle.
 \param[in]  size   - Size of the allocation requested
 \param[in]  prop   - Properties of the allocation to create.
 \param[in]  flags  - flags for future use, must be zero now.
 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuMemRelease, ::cuMemExportToShareableHandle, ::cuMemImportFromShareableHandle*/
    fn cuMemCreate(
        handle: *mut cuda_types::cuda::CUmemGenericAllocationHandle,
        size: usize,
        prop: *const cuda_types::cuda::CUmemAllocationProp,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Release a memory handle representing a memory allocation which was previously allocated through cuMemCreate.

 Frees the memory that was allocated on a device through cuMemCreate.

 The memory allocation will be freed when all outstanding mappings to the memory
 are unmapped and when all outstanding references to the handle (including it's
 shareable counterparts) are also released. The generic memory handle can be
 freed when there are still outstanding mappings made with this handle. Each
 time a recipient process imports a shareable handle, it needs to pair it with
 ::cuMemRelease for the handle to be freed.  If \p handle is not a valid handle
 the behavior is undefined.

 \param[in] handle Value of handle which was returned previously by cuMemCreate.
 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuMemCreate*/
    fn cuMemRelease(
        handle: cuda_types::cuda::CUmemGenericAllocationHandle,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Maps an allocation handle to a reserved virtual address range.

 Maps bytes of memory represented by \p handle starting from byte \p offset to
 \p size to address range [\p addr, \p addr + \p size]. This range must be an
 address reservation previously reserved with ::cuMemAddressReserve, and
 \p offset + \p size must be less than the size of the memory allocation.
 Both \p ptr, \p size, and \p offset must be a multiple of the value given via
 ::cuMemGetAllocationGranularity with the ::CU_MEM_ALLOC_GRANULARITY_MINIMUM flag.
 If \p handle represents a multicast object, \p ptr, \p size and \p offset must
 be aligned to the value returned by ::cuMulticastGetGranularity with the flag
 ::CU_MULTICAST_MINIMUM_GRANULARITY. For best performance however, it is
 recommended that \p ptr, \p size and \p offset be aligned to the value
 returned by ::cuMulticastGetGranularity with the flag
 ::CU_MULTICAST_RECOMMENDED_GRANULARITY.

 When \p handle represents a multicast object, this call may return
 CUDA_ERROR_ILLEGAL_STATE if the system configuration is in an illegal state.
 In such cases, to continue using multicast, verify that the system
 configuration is in a valid state and all required driver daemons are
 running properly.

 Please note calling ::cuMemMap does not make the address accessible,
 the caller needs to update accessibility of a contiguous mapped VA
 range by calling ::cuMemSetAccess.

 Once a recipient process obtains a shareable memory handle
 from ::cuMemImportFromShareableHandle, the process must
 use ::cuMemMap to map the memory into its address ranges before
 setting accessibility with ::cuMemSetAccess.

 ::cuMemMap can only create mappings on VA range reservations
 that are not currently mapped.

 \param[in] ptr    - Address where memory will be mapped.
 \param[in] size   - Size of the memory mapping.
 \param[in] offset - Offset into the memory represented by
                   - \p handle from which to start mapping
                   - Note: currently must be zero.
 \param[in] handle - Handle to a shareable memory
 \param[in] flags  - flags for future use, must be zero now.
 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_ILLEGAL_STATE
 \notefnerr

 \sa ::cuMemUnmap, ::cuMemSetAccess, ::cuMemCreate, ::cuMemAddressReserve, ::cuMemImportFromShareableHandle*/
    fn cuMemMap(
        ptr: cuda_types::cuda::CUdeviceptr,
        size: usize,
        offset: usize,
        handle: cuda_types::cuda::CUmemGenericAllocationHandle,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Maps or unmaps subregions of sparse CUDA arrays and sparse CUDA mipmapped arrays

 Performs map or unmap operations on subregions of sparse CUDA arrays and sparse CUDA mipmapped arrays.
 Each operation is specified by a ::CUarrayMapInfo entry in the \p mapInfoList array of size \p count.
 The structure ::CUarrayMapInfo is defined as follow:
\code
typedef struct CUarrayMapInfo_st {
CUresourcetype resourceType;
union {
CUmipmappedArray mipmap;
CUarray array;
} resource;

CUarraySparseSubresourceType subresourceType;
union {
struct {
unsigned int level;
unsigned int layer;
unsigned int offsetX;
unsigned int offsetY;
unsigned int offsetZ;
unsigned int extentWidth;
unsigned int extentHeight;
unsigned int extentDepth;
} sparseLevel;
struct {
unsigned int layer;
unsigned long long offset;
unsigned long long size;
} miptail;
} subresource;

CUmemOperationType memOperationType;

CUmemHandleType memHandleType;
union {
CUmemGenericAllocationHandle memHandle;
} memHandle;

unsigned long long offset;
unsigned int deviceBitMask;
unsigned int flags;
unsigned int reserved[2];
} CUarrayMapInfo;
\endcode

 where ::CUarrayMapInfo::resourceType specifies the type of resource to be operated on.
 If ::CUarrayMapInfo::resourceType is set to ::CUresourcetype::CU_RESOURCE_TYPE_ARRAY then
 ::CUarrayMapInfo::resource::array must be set to a valid sparse CUDA array handle.
 The CUDA array must be either a 2D, 2D layered or 3D CUDA array and must have been allocated using
 ::cuArrayCreate or ::cuArray3DCreate with the flag ::CUDA_ARRAY3D_SPARSE
 or ::CUDA_ARRAY3D_DEFERRED_MAPPING.
 For CUDA arrays obtained using ::cuMipmappedArrayGetLevel, ::CUDA_ERROR_INVALID_VALUE will be returned.
 If ::CUarrayMapInfo::resourceType is set to ::CUresourcetype::CU_RESOURCE_TYPE_MIPMAPPED_ARRAY
 then ::CUarrayMapInfo::resource::mipmap must be set to a valid sparse CUDA mipmapped array handle.
 The CUDA mipmapped array must be either a 2D, 2D layered or 3D CUDA mipmapped array and must have been
 allocated using ::cuMipmappedArrayCreate with the flag ::CUDA_ARRAY3D_SPARSE
 or ::CUDA_ARRAY3D_DEFERRED_MAPPING.

 ::CUarrayMapInfo::subresourceType specifies the type of subresource within the resource.
 ::CUarraySparseSubresourceType_enum is defined as:
\code
typedef enum CUarraySparseSubresourceType_enum {
CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_SPARSE_LEVEL = 0,
CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_MIPTAIL = 1
} CUarraySparseSubresourceType;
\endcode

 where ::CUarraySparseSubresourceType::CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_SPARSE_LEVEL indicates a
 sparse-miplevel which spans at least one tile in every dimension. The remaining miplevels which
 are too small to span at least one tile in any dimension constitute the mip tail region as indicated by
 ::CUarraySparseSubresourceType::CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_MIPTAIL subresource type.

 If ::CUarrayMapInfo::subresourceType is set to ::CUarraySparseSubresourceType::CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_SPARSE_LEVEL
 then ::CUarrayMapInfo::subresource::sparseLevel struct must contain valid array subregion offsets and extents.
 The ::CUarrayMapInfo::subresource::sparseLevel::offsetX, ::CUarrayMapInfo::subresource::sparseLevel::offsetY
 and ::CUarrayMapInfo::subresource::sparseLevel::offsetZ must specify valid X, Y and Z offsets respectively.
 The ::CUarrayMapInfo::subresource::sparseLevel::extentWidth, ::CUarrayMapInfo::subresource::sparseLevel::extentHeight
 and ::CUarrayMapInfo::subresource::sparseLevel::extentDepth must specify valid width, height and depth extents respectively.
 These offsets and extents must be aligned to the corresponding tile dimension.
 For CUDA mipmapped arrays ::CUarrayMapInfo::subresource::sparseLevel::level must specify a valid mip level index. Otherwise,
 must be zero.
 For layered CUDA arrays and layered CUDA mipmapped arrays ::CUarrayMapInfo::subresource::sparseLevel::layer must specify a valid layer index. Otherwise,
 must be zero.
 ::CUarrayMapInfo::subresource::sparseLevel::offsetZ must be zero and ::CUarrayMapInfo::subresource::sparseLevel::extentDepth
 must be set to 1 for 2D and 2D layered CUDA arrays and CUDA mipmapped arrays.
 Tile extents can be obtained by calling ::cuArrayGetSparseProperties and ::cuMipmappedArrayGetSparseProperties

 If ::CUarrayMapInfo::subresourceType is set to ::CUarraySparseSubresourceType::CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_MIPTAIL
 then ::CUarrayMapInfo::subresource::miptail struct must contain valid mip tail offset in
 ::CUarrayMapInfo::subresource::miptail::offset and size in ::CUarrayMapInfo::subresource::miptail::size.
 Both, mip tail offset and mip tail size must be aligned to the tile size.
 For layered CUDA mipmapped arrays which don't have the flag ::CU_ARRAY_SPARSE_PROPERTIES_SINGLE_MIPTAIL set in ::CUDA_ARRAY_SPARSE_PROPERTIES::flags
 as returned by ::cuMipmappedArrayGetSparseProperties, ::CUarrayMapInfo::subresource::miptail::layer must specify a valid layer index.
 Otherwise, must be zero.

 If ::CUarrayMapInfo::resource::array or ::CUarrayMapInfo::resource::mipmap was created with ::CUDA_ARRAY3D_DEFERRED_MAPPING
 flag set the ::CUarrayMapInfo::subresourceType and the contents of ::CUarrayMapInfo::subresource will be ignored.

 ::CUarrayMapInfo::memOperationType specifies the type of operation. ::CUmemOperationType is defined as:
\code
typedef enum CUmemOperationType_enum {
CU_MEM_OPERATION_TYPE_MAP = 1,
CU_MEM_OPERATION_TYPE_UNMAP = 2
} CUmemOperationType;
\endcode
 If ::CUarrayMapInfo::memOperationType is set to ::CUmemOperationType::CU_MEM_OPERATION_TYPE_MAP then the subresource
 will be mapped onto the tile pool memory specified by ::CUarrayMapInfo::memHandle at offset ::CUarrayMapInfo::offset.
 The tile pool allocation has to be created by specifying the ::CU_MEM_CREATE_USAGE_TILE_POOL flag when calling ::cuMemCreate. Also,
 ::CUarrayMapInfo::memHandleType must be set to ::CUmemHandleType::CU_MEM_HANDLE_TYPE_GENERIC.

 If ::CUarrayMapInfo::memOperationType is set to ::CUmemOperationType::CU_MEM_OPERATION_TYPE_UNMAP then an unmapping operation
 is performed. ::CUarrayMapInfo::memHandle must be NULL.

 ::CUarrayMapInfo::deviceBitMask specifies the list of devices that must map or unmap physical memory.
 Currently, this mask must have exactly one bit set, and the corresponding device must match the device associated with the stream.
 If ::CUarrayMapInfo::memOperationType is set to ::CUmemOperationType::CU_MEM_OPERATION_TYPE_MAP, the device must also match
 the device associated with the tile pool memory allocation as specified by ::CUarrayMapInfo::memHandle.

 ::CUarrayMapInfo::flags and ::CUarrayMapInfo::reserved[] are unused and must be set to zero.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE

 \param[in] mapInfoList - List of ::CUarrayMapInfo
 \param[in] count       - Count of ::CUarrayMapInfo  in \p mapInfoList
 \param[in] hStream     - Stream identifier for the stream to use for map or unmap operations

 \sa ::cuMipmappedArrayCreate, ::cuArrayCreate, ::cuArray3DCreate, ::cuMemCreate, ::cuArrayGetSparseProperties, ::cuMipmappedArrayGetSparseProperties*/
    fn cuMemMapArrayAsync_ptsz(
        mapInfoList: *mut cuda_types::cuda::CUarrayMapInfo,
        count: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unmap the backing memory of a given address range.

 The range must be the entire contiguous address range that was mapped to.  In
 other words, ::cuMemUnmap cannot unmap a sub-range of an address range mapped
 by ::cuMemCreate / ::cuMemMap.  Any backing memory allocations will be freed
 if there are no existing mappings and there are no unreleased memory handles.

 When ::cuMemUnmap returns successfully the address range is converted to an
 address reservation and can be used for a future calls to ::cuMemMap.  Any new
 mapping to this virtual address will need to have access granted through
 ::cuMemSetAccess, as all mappings start with no accessibility setup.

 \param[in] ptr  - Starting address for the virtual address range to unmap
 \param[in] size - Size of the virtual address range to unmap
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr
 \note_sync

 \sa ::cuMemCreate, ::cuMemAddressReserve*/
    fn cuMemUnmap(
        ptr: cuda_types::cuda::CUdeviceptr,
        size: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Set the access flags for each location specified in \p desc for the given virtual address range

 Given the virtual address range via \p ptr and \p size, and the locations
 in the array given by \p desc and \p count, set the access flags for the
 target locations.  The range must be a fully mapped address range
 containing all allocations created by ::cuMemMap / ::cuMemCreate.
 Users cannot specify ::CU_MEM_LOCATION_TYPE_HOST_NUMA accessibility for allocations created on with other location types.
 Note: When ::CUmemAccessDesc::CUmemLocation::type is ::CU_MEM_LOCATION_TYPE_HOST_NUMA, ::CUmemAccessDesc::CUmemLocation::id
 is ignored.
 When setting the access flags for a virtual address range mapping a multicast
 object, \p ptr and \p size must be aligned to the value returned by
 ::cuMulticastGetGranularity with the flag ::CU_MULTICAST_MINIMUM_GRANULARITY.
 For best performance however, it is recommended that \p ptr and \p size be
 aligned to the value returned by ::cuMulticastGetGranularity with the flag
 ::CU_MULTICAST_RECOMMENDED_GRANULARITY.

 \param[in] ptr   - Starting address for the virtual address range
 \param[in] size  - Length of the virtual address range
 \param[in] desc  - Array of ::CUmemAccessDesc that describe how to change the
                  - mapping for each location specified
 \param[in] count - Number of ::CUmemAccessDesc in \p desc
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr
 \note_sync

 \sa ::cuMemSetAccess, ::cuMemCreate, :cuMemMap*/
    fn cuMemSetAccess(
        ptr: cuda_types::cuda::CUdeviceptr,
        size: usize,
        desc: *const cuda_types::cuda::CUmemAccessDesc,
        count: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get the access \p flags set for the given \p location and \p ptr

 \param[out] flags   - Flags set for this location
 \param[in] location - Location in which to check the flags for
 \param[in] ptr      - Address in which to check the access flags for
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMemSetAccess*/
    fn cuMemGetAccess(
        flags: *mut ::core::ffi::c_ulonglong,
        location: *const cuda_types::cuda::CUmemLocation,
        ptr: cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Exports an allocation to a requested shareable handle type

 Given a CUDA memory handle, create a shareable memory
 allocation handle that can be used to share the memory with other
 processes. The recipient process can convert the shareable handle back into a
 CUDA memory handle using ::cuMemImportFromShareableHandle and map
 it with ::cuMemMap. The implementation of what this handle is and how it
 can be transferred is defined by the requested handle type in \p handleType

 Once all shareable handles are closed and the allocation is released, the allocated
 memory referenced will be released back to the OS and uses of the CUDA handle afterward
 will lead to undefined behavior.

 This API can also be used in conjunction with other APIs (e.g. Vulkan, OpenGL)
 that support importing memory from the shareable type

 \param[out] shareableHandle - Pointer to the location in which to store the requested handle type
 \param[in] handle           - CUDA handle for the memory allocation
 \param[in] handleType       - Type of shareable handle requested (defines type and size of the \p shareableHandle output parameter)
 \param[in] flags            - Reserved, must be zero
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMemImportFromShareableHandle*/
    fn cuMemExportToShareableHandle(
        shareableHandle: *mut ::core::ffi::c_void,
        handle: cuda_types::cuda::CUmemGenericAllocationHandle,
        handleType: cuda_types::cuda::CUmemAllocationHandleType,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Imports an allocation from a requested shareable handle type.

 If the current process cannot support the memory described by this shareable
 handle, this API will error as ::CUDA_ERROR_NOT_SUPPORTED.

 If \p shHandleType is ::CU_MEM_HANDLE_TYPE_FABRIC and the importer process has not been
 granted access to the same IMEX channel as the exporter process, this API will error
 as ::CUDA_ERROR_NOT_PERMITTED.

 \note Importing shareable handles exported from some graphics APIs(VUlkan, OpenGL, etc)
 created on devices under an SLI group may not be supported, and thus this API will
 return CUDA_ERROR_NOT_SUPPORTED.
 There is no guarantee that the contents of \p handle will be the same CUDA memory handle
 for the same given OS shareable handle, or the same underlying allocation.

 \param[out] handle       - CUDA Memory handle for the memory allocation.
 \param[in]  osHandle     - Shareable Handle representing the memory allocation that is to be imported.
 \param[in]  shHandleType - handle type of the exported handle ::CUmemAllocationHandleType.
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMemExportToShareableHandle, ::cuMemMap, ::cuMemRelease*/
    fn cuMemImportFromShareableHandle(
        handle: *mut cuda_types::cuda::CUmemGenericAllocationHandle,
        osHandle: *mut ::core::ffi::c_void,
        shHandleType: cuda_types::cuda::CUmemAllocationHandleType,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Calculates either the minimal or recommended granularity

 Calculates either the minimal or recommended granularity
 for a given allocation specification and returns it in granularity.  This
 granularity can be used as a multiple for alignment, size, or address mapping.

 \param[out] granularity Returned granularity.
 \param[in]  prop Property for which to determine the granularity for
 \param[in]  option Determines which granularity to return
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMemCreate, ::cuMemMap*/
    fn cuMemGetAllocationGranularity(
        granularity: *mut usize,
        prop: *const cuda_types::cuda::CUmemAllocationProp,
        option: cuda_types::cuda::CUmemAllocationGranularity_flags,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Retrieve the contents of the property structure defining properties for this handle

 \param[out] prop  - Pointer to a properties structure which will hold the information about this handle
 \param[in] handle - Handle which to perform the query on
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMemCreate, ::cuMemImportFromShareableHandle*/
    fn cuMemGetAllocationPropertiesFromHandle(
        prop: *mut cuda_types::cuda::CUmemAllocationProp,
        handle: cuda_types::cuda::CUmemGenericAllocationHandle,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Given an address \p addr, returns the allocation handle of the backing memory allocation.

 The handle is guaranteed to be the same handle value used to map the memory. If the address
 requested is not mapped, the function will fail. The returned handle must be released with
 corresponding number of calls to ::cuMemRelease.

 \note The address \p addr, can be any address in a range previously mapped
 by ::cuMemMap, and not necessarily the start address.

 \param[out] handle CUDA Memory handle for the backing memory allocation.
 \param[in] addr Memory address to query, that has been mapped previously.
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMemCreate, ::cuMemRelease, ::cuMemMap*/
    fn cuMemRetainAllocationHandle(
        handle: *mut cuda_types::cuda::CUmemGenericAllocationHandle,
        addr: *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Frees memory with stream ordered semantics

 Inserts a free operation into \p hStream.
 The allocation must not be accessed after stream execution reaches the free.
 After this API returns, accessing the memory from any subsequent work launched on the GPU
 or querying its pointer attributes results in undefined behavior.

 \note During stream capture, this function results in the creation of a free node and
       must therefore be passed the address of a graph allocation.

 \param dptr - memory to free
 \param hStream - The stream establishing the stream ordering contract.
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT (default stream specified with no current context),
 ::CUDA_ERROR_NOT_SUPPORTED*/
    fn cuMemFreeAsync_ptsz(
        dptr: cuda_types::cuda::CUdeviceptr,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allocates memory with stream ordered semantics

 Inserts an allocation operation into \p hStream.
 A pointer to the allocated memory is returned immediately in *dptr.
 The allocation must not be accessed until the the allocation operation completes.
 The allocation comes from the memory pool current to the stream's device.

 \note The default memory pool of a device contains device memory from that device.
 \note Basic stream ordering allows future work submitted into the same stream to use the allocation.
       Stream query, stream synchronize, and CUDA events can be used to guarantee that the allocation
       operation completes before work submitted in a separate stream runs.
 \note During stream capture, this function results in the creation of an allocation node.  In this case,
       the allocation is owned by the graph instead of the memory pool. The memory pool's properties
       are used to set the node's creation parameters.

 \param[out] dptr    - Returned device pointer
 \param[in] bytesize - Number of bytes to allocate
 \param[in] hStream  - The stream establishing the stream ordering contract and the memory pool to allocate from
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT (default stream specified with no current context),
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa ::cuMemAllocFromPoolAsync, ::cuMemFreeAsync, ::cuDeviceSetMemPool,
     ::cuDeviceGetDefaultMemPool, ::cuDeviceGetMemPool, ::cuMemPoolCreate,
     ::cuMemPoolSetAccess, ::cuMemPoolSetAttribute*/
    fn cuMemAllocAsync_ptsz(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        bytesize: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Tries to release memory back to the OS

 Releases memory back to the OS until the pool contains fewer than minBytesToKeep
 reserved bytes, or there is no more memory that the allocator can safely release.
 The allocator cannot release OS allocations that back outstanding asynchronous allocations.
 The OS allocations may happen at different granularity from the user allocations.

 \note: Allocations that have not been freed count as outstanding.
 \note: Allocations that have been asynchronously freed but whose completion has
        not been observed on the host (eg. by a synchronize) can count as outstanding.

 \param[in] pool           - The memory pool to trim
 \param[in] minBytesToKeep - If the pool has less than minBytesToKeep reserved,
 the TrimTo operation is a no-op.  Otherwise the pool will be guaranteed to have
 at least minBytesToKeep bytes reserved after the operation.
 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuMemAllocAsync, ::cuMemFreeAsync, ::cuDeviceGetDefaultMemPool,
     ::cuDeviceGetMemPool, ::cuMemPoolCreate*/
    fn cuMemPoolTrimTo(
        pool: cuda_types::cuda::CUmemoryPool,
        minBytesToKeep: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets attributes of a memory pool

 Supported attributes are:
 - ::CU_MEMPOOL_ATTR_RELEASE_THRESHOLD: (value type = cuuint64_t)
                    Amount of reserved memory in bytes to hold onto before trying
                    to release memory back to the OS. When more than the release
                    threshold bytes of memory are held by the memory pool, the
                    allocator will try to release memory back to the OS on the
                    next call to stream, event or context synchronize. (default 0)
 - ::CU_MEMPOOL_ATTR_REUSE_FOLLOW_EVENT_DEPENDENCIES: (value type = int)
                    Allow ::cuMemAllocAsync to use memory asynchronously freed
                    in another stream as long as a stream ordering dependency
                    of the allocating stream on the free action exists.
                    Cuda events and null stream interactions can create the required
                    stream ordered dependencies. (default enabled)
 - ::CU_MEMPOOL_ATTR_REUSE_ALLOW_OPPORTUNISTIC: (value type = int)
                    Allow reuse of already completed frees when there is no dependency
                    between the free and allocation. (default enabled)
 - ::CU_MEMPOOL_ATTR_REUSE_ALLOW_INTERNAL_DEPENDENCIES: (value type = int)
                    Allow ::cuMemAllocAsync to insert new stream dependencies
                    in order to establish the stream ordering required to reuse
                    a piece of memory released by ::cuMemFreeAsync (default enabled).
 - ::CU_MEMPOOL_ATTR_RESERVED_MEM_HIGH: (value type = cuuint64_t)
                    Reset the high watermark that tracks the amount of backing memory that was
                    allocated for the memory pool. It is illegal to set this attribute to a non-zero value.
 - ::CU_MEMPOOL_ATTR_USED_MEM_HIGH: (value type = cuuint64_t)
                    Reset the high watermark that tracks the amount of used memory that was
                    allocated for the memory pool.

 \param[in] pool  - The memory pool to modify
 \param[in] attr  - The attribute to modify
 \param[in] value - Pointer to the value to assign

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuMemAllocAsync, ::cuMemFreeAsync, ::cuDeviceGetDefaultMemPool,
     ::cuDeviceGetMemPool, ::cuMemPoolCreate*/
    fn cuMemPoolSetAttribute(
        pool: cuda_types::cuda::CUmemoryPool,
        attr: cuda_types::cuda::CUmemPool_attribute,
        value: *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets attributes of a memory pool

 Supported attributes are:
 - ::CU_MEMPOOL_ATTR_RELEASE_THRESHOLD: (value type = cuuint64_t)
                    Amount of reserved memory in bytes to hold onto before trying
                    to release memory back to the OS. When more than the release
                    threshold bytes of memory are held by the memory pool, the
                    allocator will try to release memory back to the OS on the
                    next call to stream, event or context synchronize. (default 0)
 - ::CU_MEMPOOL_ATTR_REUSE_FOLLOW_EVENT_DEPENDENCIES: (value type = int)
                    Allow ::cuMemAllocAsync to use memory asynchronously freed
                    in another stream as long as a stream ordering dependency
                    of the allocating stream on the free action exists.
                    Cuda events and null stream interactions can create the required
                    stream ordered dependencies. (default enabled)
 - ::CU_MEMPOOL_ATTR_REUSE_ALLOW_OPPORTUNISTIC: (value type = int)
                    Allow reuse of already completed frees when there is no dependency
                    between the free and allocation. (default enabled)
 - ::CU_MEMPOOL_ATTR_REUSE_ALLOW_INTERNAL_DEPENDENCIES: (value type = int)
                    Allow ::cuMemAllocAsync to insert new stream dependencies
                    in order to establish the stream ordering required to reuse
                    a piece of memory released by ::cuMemFreeAsync (default enabled).
 - ::CU_MEMPOOL_ATTR_RESERVED_MEM_CURRENT: (value type = cuuint64_t)
                    Amount of backing memory currently allocated for the mempool
 - ::CU_MEMPOOL_ATTR_RESERVED_MEM_HIGH: (value type = cuuint64_t)
                    High watermark of backing memory allocated for the mempool since the
                    last time it was reset.
 - ::CU_MEMPOOL_ATTR_USED_MEM_CURRENT: (value type = cuuint64_t)
                    Amount of memory from the pool that is currently in use by the application.
 - ::CU_MEMPOOL_ATTR_USED_MEM_HIGH: (value type = cuuint64_t)
                    High watermark of the amount of memory from the pool that was in use by the application.

 \param[in] pool   - The memory pool to get attributes of
 \param[in] attr   - The attribute to get
 \param[out] value - Retrieved value

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuMemAllocAsync, ::cuMemFreeAsync, ::cuDeviceGetDefaultMemPool,
     ::cuDeviceGetMemPool, ::cuMemPoolCreate*/
    fn cuMemPoolGetAttribute(
        pool: cuda_types::cuda::CUmemoryPool,
        attr: cuda_types::cuda::CUmemPool_attribute,
        value: *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Controls visibility of pools between devices

 \param[in] pool  - The pool being modified
 \param[in] map   - Array of access descriptors. Each descriptor instructs the access to enable for a single gpu.
 \param[in] count - Number of descriptors in the map array.

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuMemAllocAsync, ::cuMemFreeAsync, ::cuDeviceGetDefaultMemPool,
     ::cuDeviceGetMemPool, ::cuMemPoolCreate*/
    fn cuMemPoolSetAccess(
        pool: cuda_types::cuda::CUmemoryPool,
        map: *const cuda_types::cuda::CUmemAccessDesc,
        count: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the accessibility of a pool from a device

 Returns the accessibility of the pool's memory from the specified location.

 \param[out] flags   - the accessibility of the pool from the specified location
 \param[in] memPool  - the pool being queried
 \param[in] location - the location accessing the pool

 \sa ::cuMemAllocAsync, ::cuMemFreeAsync, ::cuDeviceGetDefaultMemPool,
     ::cuDeviceGetMemPool, ::cuMemPoolCreate*/
    fn cuMemPoolGetAccess(
        flags: *mut cuda_types::cuda::CUmemAccess_flags,
        memPool: cuda_types::cuda::CUmemoryPool,
        location: *mut cuda_types::cuda::CUmemLocation,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a memory pool

 Creates a CUDA memory pool and returns the handle in \p pool.  The \p poolProps determines
 the properties of the pool such as the backing device and IPC capabilities.

 To create a memory pool targeting a specific host NUMA node, applications must
 set ::CUmemPoolProps::CUmemLocation::type to ::CU_MEM_LOCATION_TYPE_HOST_NUMA and
 ::CUmemPoolProps::CUmemLocation::id must specify the NUMA ID of the host memory node.
 Specifying ::CU_MEM_LOCATION_TYPE_HOST_NUMA_CURRENT or ::CU_MEM_LOCATION_TYPE_HOST as the
 ::CUmemPoolProps::CUmemLocation::type will result in ::CUDA_ERROR_INVALID_VALUE.
 By default, the pool's memory will be accessible from the device it is allocated on.
 In the case of pools created with ::CU_MEM_LOCATION_TYPE_HOST_NUMA, their default accessibility
 will be from the host CPU.
 Applications can control the maximum size of the pool by specifying a non-zero value for ::CUmemPoolProps::maxSize.
 If set to 0, the maximum size of the pool will default to a system dependent value.

 Applications that intend to use ::CU_MEM_HANDLE_TYPE_FABRIC based memory sharing must ensure:
 (1) `nvidia-caps-imex-channels` character device is created by the driver and is listed under /proc/devices
 (2) have at least one IMEX channel file accessible by the user launching the application.

 When exporter and importer CUDA processes have been granted access to the same IMEX channel, they can securely
 share memory.

 The IMEX channel security model works on a per user basis. Which means all processes under a user can share
 memory if the user has access to a valid IMEX channel. When multi-user isolation is desired, a separate IMEX
 channel is required for each user.

 These channel files exist in /dev/nvidia-caps-imex-channels/channel* and can be created using standard OS
 native calls like mknod on Linux. For example: To create channel0 with the major number from /proc/devices
 users can execute the following command: `mknod /dev/nvidia-caps-imex-channels/channel0 c <major number> 0`

 \note Specifying CU_MEM_HANDLE_TYPE_NONE creates a memory pool that will not support IPC.

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuDeviceSetMemPool, ::cuDeviceGetMemPool, ::cuDeviceGetDefaultMemPool,
     ::cuMemAllocFromPoolAsync, ::cuMemPoolExportToShareableHandle*/
    fn cuMemPoolCreate(
        pool: *mut cuda_types::cuda::CUmemoryPool,
        poolProps: *const cuda_types::cuda::CUmemPoolProps,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys the specified memory pool

 If any pointers obtained from this pool haven't been freed or
 the pool has free operations that haven't completed
 when ::cuMemPoolDestroy is invoked, the function will return immediately and the
 resources associated with the pool will be released automatically
 once there are no more outstanding allocations.

 Destroying the current mempool of a device sets the default mempool of
 that device as the current mempool for that device.

 \note A device's default memory pool cannot be destroyed.

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuMemFreeAsync, ::cuDeviceSetMemPool, ::cuDeviceGetMemPool,
     ::cuDeviceGetDefaultMemPool, ::cuMemPoolCreate*/
    fn cuMemPoolDestroy(
        pool: cuda_types::cuda::CUmemoryPool,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allocates memory from a specified pool with stream ordered semantics.

 Inserts an allocation operation into \p hStream.
 A pointer to the allocated memory is returned immediately in *dptr.
 The allocation must not be accessed until the the allocation operation completes.
 The allocation comes from the specified memory pool.

 \note
    -  The specified memory pool may be from a device different than that of the specified \p hStream.

    -  Basic stream ordering allows future work submitted into the same stream to use the allocation.
       Stream query, stream synchronize, and CUDA events can be used to guarantee that the allocation
       operation completes before work submitted in a separate stream runs.

 \note During stream capture, this function results in the creation of an allocation node.  In this case,
       the allocation is owned by the graph instead of the memory pool. The memory pool's properties
       are used to set the node's creation parameters.

 \param[out] dptr    - Returned device pointer
 \param[in] bytesize - Number of bytes to allocate
 \param[in] pool     - The pool to allocate from
 \param[in] hStream  - The stream establishing the stream ordering semantic

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT (default stream specified with no current context),
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa ::cuMemAllocAsync, ::cuMemFreeAsync, ::cuDeviceGetDefaultMemPool,
     ::cuDeviceGetMemPool, ::cuMemPoolCreate, ::cuMemPoolSetAccess,
     ::cuMemPoolSetAttribute*/
    fn cuMemAllocFromPoolAsync_ptsz(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        bytesize: usize,
        pool: cuda_types::cuda::CUmemoryPool,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Exports a memory pool to the requested handle type.

 Given an IPC capable mempool, create an OS handle to share the pool with another process.
 A recipient process can convert the shareable handle into a mempool with ::cuMemPoolImportFromShareableHandle.
 Individual pointers can then be shared with the ::cuMemPoolExportPointer and ::cuMemPoolImportPointer APIs.
 The implementation of what the shareable handle is and how it can be transferred is defined by the requested
 handle type.

 \note: To create an IPC capable mempool, create a mempool with a CUmemAllocationHandleType other than CU_MEM_HANDLE_TYPE_NONE.

 \param[out] handle_out  - Returned OS handle
 \param[in] pool         - pool to export
 \param[in] handleType   - the type of handle to create
 \param[in] flags        - must be 0

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa ::cuMemPoolImportFromShareableHandle, ::cuMemPoolExportPointer,
     ::cuMemPoolImportPointer, ::cuMemAllocAsync, ::cuMemFreeAsync,
     ::cuDeviceGetDefaultMemPool, ::cuDeviceGetMemPool, ::cuMemPoolCreate,
     ::cuMemPoolSetAccess, ::cuMemPoolSetAttribute*/
    fn cuMemPoolExportToShareableHandle(
        handle_out: *mut ::core::ffi::c_void,
        pool: cuda_types::cuda::CUmemoryPool,
        handleType: cuda_types::cuda::CUmemAllocationHandleType,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief imports a memory pool from a shared handle.

 Specific allocations can be imported from the imported pool with cuMemPoolImportPointer.

 If \p handleType is ::CU_MEM_HANDLE_TYPE_FABRIC and the importer process has not been
 granted access to the same IMEX channel as the exporter process, this API will error
 as ::CUDA_ERROR_NOT_PERMITTED.


 \note Imported memory pools do not support creating new allocations.
       As such imported memory pools may not be used in cuDeviceSetMemPool
       or ::cuMemAllocFromPoolAsync calls.

 \param[out] pool_out    - Returned memory pool
 \param[in] handle       - OS handle of the pool to open
 \param[in] handleType   - The type of handle being imported
 \param[in] flags        - must be 0

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa ::cuMemPoolExportToShareableHandle, ::cuMemPoolExportPointer, ::cuMemPoolImportPointer*/
    fn cuMemPoolImportFromShareableHandle(
        pool_out: *mut cuda_types::cuda::CUmemoryPool,
        handle: *mut ::core::ffi::c_void,
        handleType: cuda_types::cuda::CUmemAllocationHandleType,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Export data to share a memory pool allocation between processes.

 Constructs \p shareData_out for sharing a specific allocation from an already shared memory pool.
 The recipient process can import the allocation with the ::cuMemPoolImportPointer api.
 The data is not a handle and may be shared through any IPC mechanism.

 \param[out] shareData_out - Returned export data
 \param[in] ptr            - pointer to memory being exported

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa ::cuMemPoolExportToShareableHandle, ::cuMemPoolImportFromShareableHandle, ::cuMemPoolImportPointer*/
    fn cuMemPoolExportPointer(
        shareData_out: *mut cuda_types::cuda::CUmemPoolPtrExportData,
        ptr: cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Import a memory pool allocation from another process.

 Returns in \p ptr_out a pointer to the imported memory.
 The imported memory must not be accessed before the allocation operation completes
 in the exporting process. The imported memory must be freed from all importing processes before
 being freed in the exporting process. The pointer may be freed with cuMemFree
 or cuMemFreeAsync.  If cuMemFreeAsync is used, the free must be completed
 on the importing process before the free operation on the exporting process.

 \note The cuMemFreeAsync api may be used in the exporting process before
       the cuMemFreeAsync operation completes in its stream as long as the
       cuMemFreeAsync in the exporting process specifies a stream with
       a stream dependency on the importing process's cuMemFreeAsync.

 \param[out] ptr_out  - pointer to imported memory
 \param[in] pool      - pool from which to import
 \param[in] shareData - data specifying the memory to import

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa ::cuMemPoolExportToShareableHandle, ::cuMemPoolImportFromShareableHandle, ::cuMemPoolExportPointer*/
    fn cuMemPoolImportPointer(
        ptr_out: *mut cuda_types::cuda::CUdeviceptr,
        pool: cuda_types::cuda::CUmemoryPool,
        shareData: *mut cuda_types::cuda::CUmemPoolPtrExportData,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a generic allocation handle representing a multicast object described by the given properties.

 This creates a multicast object as described by \p prop. The number of
 participating devices is specified by ::CUmulticastObjectProp::numDevices.
 Devices can be added to the multicast object via ::cuMulticastAddDevice.
 All participating devices must be added to the multicast object before memory
 can be bound to it. Memory is bound to the multicast object via either
 ::cuMulticastBindMem or ::cuMulticastBindAddr, and can be unbound via
 ::cuMulticastUnbind. The total amount of memory that can be bound per device
 is specified by :CUmulticastObjectProp::size. This size must be a multiple of
 the value returned by ::cuMulticastGetGranularity with the flag
 ::CU_MULTICAST_GRANULARITY_MINIMUM. For best performance however, the size
 should be aligned to the value returned by ::cuMulticastGetGranularity with
 the flag ::CU_MULTICAST_GRANULARITY_RECOMMENDED.

 After all participating devices have been added, multicast objects can also
 be mapped to a device's virtual address space using the virtual memory
 management APIs (see ::cuMemMap and ::cuMemSetAccess). Multicast objects can
 also be shared with other processes by requesting a shareable handle via
 ::cuMemExportToShareableHandle. Note that the desired types of shareable
 handles must be specified in the bitmask ::CUmulticastObjectProp::handleTypes.
 Multicast objects can be released using the virtual memory management API
 ::cuMemRelease.

 \param[out] mcHandle     Value of handle returned.
 \param[in]  prop         Properties of the multicast object to create.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMulticastAddDevice, ::cuMulticastBindMem, ::cuMulticastBindAddr, ::cuMulticastUnbind
 \sa ::cuMemCreate, ::cuMemRelease, ::cuMemExportToShareableHandle, ::cuMemImportFromShareableHandle*/
    fn cuMulticastCreate(
        mcHandle: *mut cuda_types::cuda::CUmemGenericAllocationHandle,
        prop: *const cuda_types::cuda::CUmulticastObjectProp,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Associate a device to a multicast object.

 Associates a device to a multicast object. The added device will be a part of
 the multicast team of size specified by CUmulticastObjectProp::numDevices
 during ::cuMulticastCreate.
 The association of the device to the multicast object is permanent during
 the life time of the multicast object.
 All devices must be added to the multicast team before any memory can be
 bound to any device in the team. Any calls to ::cuMulticastBindMem or
 ::cuMulticastBindAddr will block until all devices have been added.
 Similarly all devices must be added to the multicast team before a virtual
 address range can be mapped to the multicast object. A call to ::cuMemMap
 will block until all devices have been added.

 \param[in] mcHandle     Handle representing a multicast object.
 \param[in] dev          Device that will be associated to the multicast
                         object.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMulticastCreate, ::cuMulticastBindMem, ::cuMulticastBindAddr*/
    fn cuMulticastAddDevice(
        mcHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Bind a memory allocation represented by a handle to a multicast object.

 Binds a memory allocation specified by \p memHandle and created via
 ::cuMemCreate to a multicast object represented by \p mcHandle and created
 via ::cuMulticastCreate. The intended \p size of the bind, the offset in the
 multicast range \p mcOffset as well as the offset in the memory \p memOffset
 must be a multiple of the value returned by ::cuMulticastGetGranularity with
 the flag ::CU_MULTICAST_GRANULARITY_MINIMUM. For best performance however,
 \p size, \p mcOffset and \p memOffset should be aligned to the granularity of
 the memory allocation(see ::cuMemGetAllocationGranularity) or to the value
 returned by ::cuMulticastGetGranularity with the flag
 ::CU_MULTICAST_GRANULARITY_RECOMMENDED.

 The \p size + \p memOffset cannot be larger than the size of the allocated
 memory. Similarly the \p size + \p mcOffset cannot be larger than the size
 of the multicast object.
 The memory allocation must have beeen created on one of the devices
 that was added to the multicast team via ::cuMulticastAddDevice.
 Externally shareable as well as imported multicast objects can be bound only
 to externally shareable memory.
 Note that this call will return CUDA_ERROR_OUT_OF_MEMORY if there are
 insufficient resources required to perform the bind. This call may also
 return CUDA_ERROR_SYSTEM_NOT_READY if the necessary system software is not
 initialized or running.

 This call may return CUDA_ERROR_ILLEGAL_STATE if the system configuration
 is in an illegal state. In such cases, to continue using multicast, verify
 that the system configuration is in a valid state and all required driver
 daemons are running properly.

 \param[in]  mcHandle     Handle representing a multicast object.
 \param[in]  mcOffset     Offset into the multicast object for attachment.
 \param[in]  memHandle    Handle representing a memory allocation.
 \param[in]  memOffset    Offset into the memory for attachment.
 \param[in]  size         Size of the memory that will be bound to the
                          multicast object.
 \param[in]  flags        Flags for future use, must be zero for now.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_SYSTEM_NOT_READY,
 ::CUDA_ERROR_ILLEGAL_STATE

 \sa ::cuMulticastCreate, ::cuMulticastAddDevice, ::cuMemCreate*/
    fn cuMulticastBindMem(
        mcHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
        mcOffset: usize,
        memHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
        memOffset: usize,
        size: usize,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Bind a memory allocation represented by a virtual address to a multicast object.

 Binds a memory allocation specified by its mapped address \p memptr to a
 multicast object represented by \p mcHandle.
 The memory must have been allocated via ::cuMemCreate or ::cudaMallocAsync.
 The intended \p size of the bind, the offset in the multicast range
 \p mcOffset and \p memptr must be a multiple of the value returned by
 ::cuMulticastGetGranularity with the flag ::CU_MULTICAST_GRANULARITY_MINIMUM.
 For best performance however, \p size, \p mcOffset and \p memptr should be
 aligned to the value returned by ::cuMulticastGetGranularity with the flag
 ::CU_MULTICAST_GRANULARITY_RECOMMENDED.

 The \p size cannot be larger than the size of the allocated memory.
 Similarly the \p size + \p mcOffset cannot be larger than the total size
 of the multicast object.
 The memory allocation must have beeen created on one of the devices
 that was added to the multicast team via ::cuMulticastAddDevice.
 Externally shareable as well as imported multicast objects can be bound only
 to externally shareable memory.
 Note that this call will return CUDA_ERROR_OUT_OF_MEMORY if there are
 insufficient resources required to perform the bind. This call may also
 return CUDA_ERROR_SYSTEM_NOT_READY if the necessary system software is not
 initialized or running.

 This call may return CUDA_ERROR_ILLEGAL_STATE if the system configuration
 is in an illegal state. In such cases, to continue using multicast, verify
 that the system configuration is in a valid state and all required driver
 daemons are running properly.

 \param[in]  mcHandle     Handle representing a multicast object.
 \param[in]  mcOffset     Offset into multicast va range for attachment.
 \param[in]  memptr       Virtual address of the memory allocation.
 \param[in]  size         Size of memory that will be bound to the
                          multicast object.
 \param[in]  flags        Flags for future use, must be zero now.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_OUT_OF_MEMORY,
 ::CUDA_ERROR_SYSTEM_NOT_READY,
 ::CUDA_ERROR_ILLEGAL_STATE

 \sa ::cuMulticastCreate, ::cuMulticastAddDevice, ::cuMemCreate*/
    fn cuMulticastBindAddr(
        mcHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
        mcOffset: usize,
        memptr: cuda_types::cuda::CUdeviceptr,
        size: usize,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unbind any memory allocations bound to a multicast object at a given offset and upto a given size.

 Unbinds any memory allocations hosted on \p dev and bound to a multicast
 object at \p mcOffset and upto a given \p size.
 The intended \p size of the unbind and the offset in the multicast range
 ( \p mcOffset ) must be a multiple of the value returned by
 ::cuMulticastGetGranularity flag ::CU_MULTICAST_GRANULARITY_MINIMUM.
 The \p size + \p mcOffset cannot be larger than the total size of the
 multicast object.

 \note
 Warning:
 The \p mcOffset and the \p size must match the corresponding values specified
 during the bind call. Any other values may result in undefined behavior.

 \param[in]  mcHandle     Handle representing a multicast object.
 \param[in]  dev          Device that hosts the memory allocation.
 \param[in]  mcOffset     Offset into the multicast object.
 \param[in]  size         Desired size to unbind.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMulticastBindMem, ::cuMulticastBindAddr*/
    fn cuMulticastUnbind(
        mcHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
        dev: cuda_types::cuda::CUdevice,
        mcOffset: usize,
        size: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Calculates either the minimal or recommended granularity for multicast object

 Calculates either the minimal or recommended granularity for a given set of
 multicast object properties and returns it in granularity.  This granularity
 can be used as a multiple for size, bind offsets and address mappings of the
 multicast object.

 \param[out] granularity Returned granularity.
 \param[in]  prop        Properties of the multicast object.
 \param[in]  option      Determines which granularity to return.

 \returns
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa ::cuMulticastCreate, ::cuMulticastBindMem, ::cuMulticastBindAddr, ::cuMulticastUnbind*/
    fn cuMulticastGetGranularity(
        granularity: *mut usize,
        prop: *const cuda_types::cuda::CUmulticastObjectProp,
        option: cuda_types::cuda::CUmulticastGranularity_flags,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns information about a pointer

 The supported attributes are:

 - ::CU_POINTER_ATTRIBUTE_CONTEXT:

      Returns in \p *data the ::CUcontext in which \p ptr was allocated or
      registered.
      The type of \p data must be ::CUcontext *.

      If \p ptr was not allocated by, mapped by, or registered with
      a ::CUcontext which uses unified virtual addressing then
      ::CUDA_ERROR_INVALID_VALUE is returned.

 - ::CU_POINTER_ATTRIBUTE_MEMORY_TYPE:

      Returns in \p *data the physical memory type of the memory that
      \p ptr addresses as a ::CUmemorytype enumerated value.
      The type of \p data must be unsigned int.

      If \p ptr addresses device memory then \p *data is set to
      ::CU_MEMORYTYPE_DEVICE.  The particular ::CUdevice on which the
      memory resides is the ::CUdevice of the ::CUcontext returned by the
      ::CU_POINTER_ATTRIBUTE_CONTEXT attribute of \p ptr.

      If \p ptr addresses host memory then \p *data is set to
      ::CU_MEMORYTYPE_HOST.

      If \p ptr was not allocated by, mapped by, or registered with
      a ::CUcontext which uses unified virtual addressing then
      ::CUDA_ERROR_INVALID_VALUE is returned.

      If the current ::CUcontext does not support unified virtual
      addressing then ::CUDA_ERROR_INVALID_CONTEXT is returned.

 - ::CU_POINTER_ATTRIBUTE_DEVICE_POINTER:

      Returns in \p *data the device pointer value through which
      \p ptr may be accessed by kernels running in the current
      ::CUcontext.
      The type of \p data must be CUdeviceptr *.

      If there exists no device pointer value through which
      kernels running in the current ::CUcontext may access
      \p ptr then ::CUDA_ERROR_INVALID_VALUE is returned.

      If there is no current ::CUcontext then
      ::CUDA_ERROR_INVALID_CONTEXT is returned.

      Except in the exceptional disjoint addressing cases discussed
      below, the value returned in \p *data will equal the input
      value \p ptr.

 - ::CU_POINTER_ATTRIBUTE_HOST_POINTER:

      Returns in \p *data the host pointer value through which
      \p ptr may be accessed by by the host program.
      The type of \p data must be void **.
      If there exists no host pointer value through which
      the host program may directly access \p ptr then
      ::CUDA_ERROR_INVALID_VALUE is returned.

      Except in the exceptional disjoint addressing cases discussed
      below, the value returned in \p *data will equal the input
      value \p ptr.

 - ::CU_POINTER_ATTRIBUTE_P2P_TOKENS:

      Returns in \p *data two tokens for use with the nv-p2p.h Linux
      kernel interface. \p data must be a struct of type
      CUDA_POINTER_ATTRIBUTE_P2P_TOKENS.

      \p ptr must be a pointer to memory obtained from :cuMemAlloc().
      Note that p2pToken and vaSpaceToken are only valid for the
      lifetime of the source allocation. A subsequent allocation at
      the same address may return completely different tokens.
      Querying this attribute has a side effect of setting the attribute
      ::CU_POINTER_ATTRIBUTE_SYNC_MEMOPS for the region of memory that
      \p ptr points to.

 - ::CU_POINTER_ATTRIBUTE_SYNC_MEMOPS:

      A boolean attribute which when set, ensures that synchronous memory operations
      initiated on the region of memory that \p ptr points to will always synchronize.
      See further documentation in the section titled "API synchronization behavior"
      to learn more about cases when synchronous memory operations can
      exhibit asynchronous behavior.

 - ::CU_POINTER_ATTRIBUTE_BUFFER_ID:

      Returns in \p *data a buffer ID which is guaranteed to be unique within the process.
      \p data must point to an unsigned long long.

      \p ptr must be a pointer to memory obtained from a CUDA memory allocation API.
      Every memory allocation from any of the CUDA memory allocation APIs will
      have a unique ID over a process lifetime. Subsequent allocations do not reuse IDs
      from previous freed allocations. IDs are only unique within a single process.


 - ::CU_POINTER_ATTRIBUTE_IS_MANAGED:

      Returns in \p *data a boolean that indicates whether the pointer points to
      managed memory or not.

      If \p ptr is not a valid CUDA pointer then ::CUDA_ERROR_INVALID_VALUE is returned.

 - ::CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL:

      Returns in \p *data an integer representing a device ordinal of a device against
      which the memory was allocated or registered.

 - ::CU_POINTER_ATTRIBUTE_IS_LEGACY_CUDA_IPC_CAPABLE:

      Returns in \p *data a boolean that indicates if this pointer maps to
      an allocation that is suitable for ::cudaIpcGetMemHandle.

 - ::CU_POINTER_ATTRIBUTE_RANGE_START_ADDR:

      Returns in \p *data the starting address for the allocation referenced
      by the device pointer \p ptr.  Note that this is not necessarily the
      address of the mapped region, but the address of the mappable address
      range \p ptr references (e.g. from ::cuMemAddressReserve).

 - ::CU_POINTER_ATTRIBUTE_RANGE_SIZE:

      Returns in \p *data the size for the allocation referenced by the device
      pointer \p ptr.  Note that this is not necessarily the size of the mapped
      region, but the size of the mappable address range \p ptr references
      (e.g. from ::cuMemAddressReserve).  To retrieve the size of the mapped
      region, see ::cuMemGetAddressRange

 - ::CU_POINTER_ATTRIBUTE_MAPPED:

      Returns in \p *data a boolean that indicates if this pointer is in a
      valid address range that is mapped to a backing allocation.

 - ::CU_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES:

      Returns a bitmask of the allowed handle types for an allocation that may
      be passed to ::cuMemExportToShareableHandle.

 - ::CU_POINTER_ATTRIBUTE_MEMPOOL_HANDLE:

      Returns in \p *data the handle to the mempool that the allocation was obtained from.

 - ::CU_POINTER_ATTRIBUTE_IS_HW_DECOMPRESS_CAPABLE:

      Returns in \p *data a boolean that indicates whether the pointer points
      to memory that is capable to be used for hardware accelerated
      decompression.

 \par

 Note that for most allocations in the unified virtual address space
 the host and device pointer for accessing the allocation will be the
 same.  The exceptions to this are
  - user memory registered using ::cuMemHostRegister
  - host memory allocated using ::cuMemHostAlloc with the
    ::CU_MEMHOSTALLOC_WRITECOMBINED flag
 For these types of allocation there will exist separate, disjoint host
 and device addresses for accessing the allocation.  In particular
  - The host address will correspond to an invalid unmapped device address
    (which will result in an exception if accessed from the device)
  - The device address will correspond to an invalid unmapped host address
    (which will result in an exception if accessed from the host).
 For these types of allocations, querying ::CU_POINTER_ATTRIBUTE_HOST_POINTER
 and ::CU_POINTER_ATTRIBUTE_DEVICE_POINTER may be used to retrieve the host
 and device addresses from either address.

 \param data      - Returned pointer attribute value
 \param attribute - Pointer attribute to query
 \param ptr       - Pointer

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuPointerSetAttribute,
 ::cuMemAlloc,
 ::cuMemFree,
 ::cuMemAllocHost,
 ::cuMemFreeHost,
 ::cuMemHostAlloc,
 ::cuMemHostRegister,
 ::cuMemHostUnregister,
 ::cudaPointerGetAttributes*/
    fn cuPointerGetAttribute(
        data: *mut ::core::ffi::c_void,
        attribute: cuda_types::cuda::CUpointer_attribute,
        ptr: cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Prefetches memory to the specified destination device

 Note there is a later version of this API, ::cuMemPrefetchAsync_v2. It will
 supplant this version in 13.0, which is retained for minor version compatibility.

 Prefetches memory to the specified destination device.  \p devPtr is the
 base device pointer of the memory to be prefetched and \p dstDevice is the
 destination device. \p count specifies the number of bytes to copy. \p hStream
 is the stream in which the operation is enqueued. The memory range must refer
 to managed memory allocated via ::cuMemAllocManaged or declared via __managed__ variables
 or it may also refer to system-allocated memory on systems with non-zero
 CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS.

 Passing in CU_DEVICE_CPU for \p dstDevice will prefetch the data to host memory. If
 \p dstDevice is a GPU, then the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS
 must be non-zero. Additionally, \p hStream must be associated with a device that has a
 non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS.

 The start address and end address of the memory range will be rounded down and rounded up
 respectively to be aligned to CPU page size before the prefetch operation is enqueued
 in the stream.

 If no physical memory has been allocated for this region, then this memory region
 will be populated and mapped on the destination device. If there's insufficient
 memory to prefetch the desired region, the Unified Memory driver may evict pages from other
 ::cuMemAllocManaged allocations to host memory in order to make room. Device memory
 allocated using ::cuMemAlloc or ::cuArrayCreate will not be evicted.

 By default, any mappings to the previous location of the migrated pages are removed and
 mappings for the new location are only setup on \p dstDevice. The exact behavior however
 also depends on the settings applied to this memory range via ::cuMemAdvise as described
 below:

 If ::CU_MEM_ADVISE_SET_READ_MOSTLY was set on any subset of this memory range,
 then that subset will create a read-only copy of the pages on \p dstDevice.

 If ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION was called on any subset of this memory
 range, then the pages will be migrated to \p dstDevice even if \p dstDevice is not the
 preferred location of any pages in the memory range.

 If ::CU_MEM_ADVISE_SET_ACCESSED_BY was called on any subset of this memory range,
 then mappings to those pages from all the appropriate processors are updated to
 refer to the new location if establishing such a mapping is possible. Otherwise,
 those mappings are cleared.

 Note that this API is not required for functionality and only serves to improve performance
 by allowing the application to migrate data to a suitable location before it is accessed.
 Memory accesses to this range are always coherent and are allowed even when the data is
 actively being migrated.

 Note that this function is asynchronous with respect to the host and all work
 on other devices.

 \param devPtr    - Pointer to be prefetched
 \param count     - Size in bytes
 \param dstDevice - Destination device to prefetch to
 \param hStream    - Stream to enqueue prefetch operation

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuMemcpy, ::cuMemcpyPeer, ::cuMemcpyAsync,
 ::cuMemcpy3DPeerAsync, ::cuMemAdvise, ::cuMemPrefetchAsync
 ::cudaMemPrefetchAsync_v2*/
    fn cuMemPrefetchAsync_ptsz(
        devPtr: cuda_types::cuda::CUdeviceptr,
        count: usize,
        dstDevice: cuda_types::cuda::CUdevice,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Prefetches memory to the specified destination location

 Prefetches memory to the specified destination location.  \p devPtr is the
 base device pointer of the memory to be prefetched and \p location specifies the
 destination location. \p count specifies the number of bytes to copy. \p hStream
 is the stream in which the operation is enqueued. The memory range must refer
 to managed memory allocated via ::cuMemAllocManaged or declared via __managed__ variables.

 Specifying ::CU_MEM_LOCATION_TYPE_DEVICE for ::CUmemLocation::type will prefetch memory to GPU
 specified by device ordinal ::CUmemLocation::id which must have non-zero value for the device attribute
 ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS. Additionally, \p hStream must be associated with a device
 that has a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS.
 Specifying ::CU_MEM_LOCATION_TYPE_HOST as ::CUmemLocation::type will prefetch data to host memory.
 Applications can request prefetching memory to a specific host NUMA node by specifying
 ::CU_MEM_LOCATION_TYPE_HOST_NUMA for ::CUmemLocation::type and a valid host NUMA node id in ::CUmemLocation::id
 Users can also request prefetching memory to the host NUMA node closest to the current thread's CPU by specifying
 ::CU_MEM_LOCATION_TYPE_HOST_NUMA_CURRENT for ::CUmemLocation::type. Note when ::CUmemLocation::type is etiher
 ::CU_MEM_LOCATION_TYPE_HOST OR ::CU_MEM_LOCATION_TYPE_HOST_NUMA_CURRENT, ::CUmemLocation::id will be ignored.

 The start address and end address of the memory range will be rounded down and rounded up
 respectively to be aligned to CPU page size before the prefetch operation is enqueued
 in the stream.

 If no physical memory has been allocated for this region, then this memory region
 will be populated and mapped on the destination device. If there's insufficient
 memory to prefetch the desired region, the Unified Memory driver may evict pages from other
 ::cuMemAllocManaged allocations to host memory in order to make room. Device memory
 allocated using ::cuMemAlloc or ::cuArrayCreate will not be evicted.

 By default, any mappings to the previous location of the migrated pages are removed and
 mappings for the new location are only setup on the destination location. The exact behavior however
 also depends on the settings applied to this memory range via ::cuMemAdvise as described
 below:

 If ::CU_MEM_ADVISE_SET_READ_MOSTLY was set on any subset of this memory range,
 then that subset will create a read-only copy of the pages on destination location.
 If however the destination location is a host NUMA node, then any pages of that subset
 that are already in another host NUMA node will be transferred to the destination.

 If ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION was called on any subset of this memory
 range, then the pages will be migrated to \p location even if \p location is not the
 preferred location of any pages in the memory range.

 If ::CU_MEM_ADVISE_SET_ACCESSED_BY was called on any subset of this memory range,
 then mappings to those pages from all the appropriate processors are updated to
 refer to the new location if establishing such a mapping is possible. Otherwise,
 those mappings are cleared.

 Note that this API is not required for functionality and only serves to improve performance
 by allowing the application to migrate data to a suitable location before it is accessed.
 Memory accesses to this range are always coherent and are allowed even when the data is
 actively being migrated.

 Note that this function is asynchronous with respect to the host and all work
 on other devices.

 \param devPtr    - Pointer to be prefetched
 \param count     - Size in bytes
 \param location  - Location to prefetch to
 \param flags     - flags for future use, must be zero now.
 \param hStream   - Stream to enqueue prefetch operation

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuMemcpy, ::cuMemcpyPeer, ::cuMemcpyAsync,
 ::cuMemcpy3DPeerAsync, ::cuMemAdvise, ::cuMemPrefetchAsync,
 ::cudaMemPrefetchAsync_v2*/
    fn cuMemPrefetchAsync_v2_ptsz(
        devPtr: cuda_types::cuda::CUdeviceptr,
        count: usize,
        location: cuda_types::cuda::CUmemLocation,
        flags: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Advise about the usage of a given memory range

 Note there is a later version of this API, ::cuMemAdvise_v2. It will
 supplant this version in 13.0, which is retained for minor version compatibility.

 Advise the Unified Memory subsystem about the usage pattern for the memory range
 starting at \p devPtr with a size of \p count bytes. The start address and end address of the memory
 range will be rounded down and rounded up respectively to be aligned to CPU page size before the
 advice is applied. The memory range must refer to managed memory allocated via ::cuMemAllocManaged
 or declared via __managed__ variables. The memory range could also refer to system-allocated pageable
 memory provided it represents a valid, host-accessible region of memory and all additional constraints
 imposed by \p advice as outlined below are also satisfied. Specifying an invalid system-allocated pageable
 memory range results in an error being returned.

 The \p advice parameter can take the following values:
 - ::CU_MEM_ADVISE_SET_READ_MOSTLY: This implies that the data is mostly going to be read
 from and only occasionally written to. Any read accesses from any processor to this region will create a
 read-only copy of at least the accessed pages in that processor's memory. Additionally, if ::cuMemPrefetchAsync
 is called on this region, it will create a read-only copy of the data on the destination processor.
 If any processor writes to this region, all copies of the corresponding page will be invalidated
 except for the one where the write occurred. The \p device argument is ignored for this advice.
 Note that for a page to be read-duplicated, the accessing processor must either be the CPU or a GPU
 that has a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS.
 Also, if a context is created on a device that does not have the device attribute
 ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS set, then read-duplication will not occur until
 all such contexts are destroyed.
 If the memory region refers to valid system-allocated pageable memory, then the accessing device must
 have a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS for a read-only
 copy to be created on that device. Note however that if the accessing device also has a non-zero value for the
 device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES, then setting this advice
 will not create a read-only copy when that device accesses this memory region.

 - ::CU_MEM_ADVISE_UNSET_READ_MOSTLY:  Undoes the effect of ::CU_MEM_ADVISE_SET_READ_MOSTLY and also prevents the
 Unified Memory driver from attempting heuristic read-duplication on the memory range. Any read-duplicated
 copies of the data will be collapsed into a single copy. The location for the collapsed
 copy will be the preferred location if the page has a preferred location and one of the read-duplicated
 copies was resident at that location. Otherwise, the location chosen is arbitrary.

 - ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION: This advice sets the preferred location for the
 data to be the memory belonging to \p device. Passing in CU_DEVICE_CPU for \p device sets the
 preferred location as host memory. If \p device is a GPU, then it must have a non-zero value for the
 device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS. Setting the preferred location
 does not cause data to migrate to that location immediately. Instead, it guides the migration policy
 when a fault occurs on that memory region. If the data is already in its preferred location and the
 faulting processor can establish a mapping without requiring the data to be migrated, then
 data migration will be avoided. On the other hand, if the data is not in its preferred location
 or if a direct mapping cannot be established, then it will be migrated to the processor accessing
 it. It is important to note that setting the preferred location does not prevent data prefetching
 done using ::cuMemPrefetchAsync.
 Having a preferred location can override the page thrash detection and resolution logic in the Unified
 Memory driver. Normally, if a page is detected to be constantly thrashing between for example host and device
 memory, the page may eventually be pinned to host memory by the Unified Memory driver. But
 if the preferred location is set as device memory, then the page will continue to thrash indefinitely.
 If ::CU_MEM_ADVISE_SET_READ_MOSTLY is also set on this memory region or any subset of it, then the
 policies associated with that advice will override the policies of this advice, unless read accesses from
 \p device will not result in a read-only copy being created on that device as outlined in description for
 the advice ::CU_MEM_ADVISE_SET_READ_MOSTLY.
 If the memory region refers to valid system-allocated pageable memory, then \p device must have a non-zero
 value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS.

 - ::CU_MEM_ADVISE_UNSET_PREFERRED_LOCATION: Undoes the effect of ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION
 and changes the preferred location to none.

 - ::CU_MEM_ADVISE_SET_ACCESSED_BY: This advice implies that the data will be accessed by \p device.
 Passing in ::CU_DEVICE_CPU for \p device will set the advice for the CPU. If \p device is a GPU, then
 the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS must be non-zero.
 This advice does not cause data migration and has no impact on the location of the data per se. Instead,
 it causes the data to always be mapped in the specified processor's page tables, as long as the
 location of the data permits a mapping to be established. If the data gets migrated for any reason,
 the mappings are updated accordingly.
 This advice is recommended in scenarios where data locality is not important, but avoiding faults is.
 Consider for example a system containing multiple GPUs with peer-to-peer access enabled, where the
 data located on one GPU is occasionally accessed by peer GPUs. In such scenarios, migrating data
 over to the other GPUs is not as important because the accesses are infrequent and the overhead of
 migration may be too high. But preventing faults can still help improve performance, and so having
 a mapping set up in advance is useful. Note that on CPU access of this data, the data may be migrated
 to host memory because the CPU typically cannot access device memory directly. Any GPU that had the
 ::CU_MEM_ADVISE_SET_ACCESSED_BY flag set for this data will now have its mapping updated to point to the
 page in host memory.
 If ::CU_MEM_ADVISE_SET_READ_MOSTLY is also set on this memory region or any subset of it, then the
 policies associated with that advice will override the policies of this advice. Additionally, if the
 preferred location of this memory region or any subset of it is also \p device, then the policies
 associated with ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION will override the policies of this advice.
 If the memory region refers to valid system-allocated pageable memory, then \p device must have a non-zero
 value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS. Additionally, if \p device has
 a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES,
 then this call has no effect.

 - ::CU_MEM_ADVISE_UNSET_ACCESSED_BY: Undoes the effect of ::CU_MEM_ADVISE_SET_ACCESSED_BY. Any mappings to
 the data from \p device may be removed at any time causing accesses to result in non-fatal page faults.
 If the memory region refers to valid system-allocated pageable memory, then \p device must have a non-zero
 value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS. Additionally, if \p device has
 a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES,
 then this call has no effect.

 \param devPtr - Pointer to memory to set the advice for
 \param count  - Size in bytes of the memory range
 \param advice - Advice to be applied for the specified memory range
 \param device - Device to apply the advice for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuMemcpy, ::cuMemcpyPeer, ::cuMemcpyAsync,
 ::cuMemcpy3DPeerAsync, ::cuMemPrefetchAsync, ::cuMemAdvise_v2,
 ::cudaMemAdvise*/
    fn cuMemAdvise(
        devPtr: cuda_types::cuda::CUdeviceptr,
        count: usize,
        advice: cuda_types::cuda::CUmem_advise,
        device: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Advise about the usage of a given memory range

 Advise the Unified Memory subsystem about the usage pattern for the memory range
 starting at \p devPtr with a size of \p count bytes. The start address and end address of the memory
 range will be rounded down and rounded up respectively to be aligned to CPU page size before the
 advice is applied. The memory range must refer to managed memory allocated via ::cuMemAllocManaged
 or declared via __managed__ variables. The memory range could also refer to system-allocated pageable
 memory provided it represents a valid, host-accessible region of memory and all additional constraints
 imposed by \p advice as outlined below are also satisfied. Specifying an invalid system-allocated pageable
 memory range results in an error being returned.

 The \p advice parameter can take the following values:
 - ::CU_MEM_ADVISE_SET_READ_MOSTLY: This implies that the data is mostly going to be read
 from and only occasionally written to. Any read accesses from any processor to this region will create a
 read-only copy of at least the accessed pages in that processor's memory. Additionally, if ::cuMemPrefetchAsync
 or ::cuMemPrefetchAsync_v2 is called on this region, it will create a read-only copy of the data on the destination processor.
 If the target location for ::cuMemPrefetchAsync_v2 is a host NUMA node and a read-only copy already exists on
 another host NUMA node, that copy will be migrated to the targeted host NUMA node.
 If any processor writes to this region, all copies of the corresponding page will be invalidated
 except for the one where the write occurred. If the writing processor is the CPU and the preferred location of
 the page is a host NUMA node, then the page will also be migrated to that host NUMA node. The \p location argument is ignored for this advice.
 Note that for a page to be read-duplicated, the accessing processor must either be the CPU or a GPU
 that has a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS.
 Also, if a context is created on a device that does not have the device attribute
 ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS set, then read-duplication will not occur until
 all such contexts are destroyed.
 If the memory region refers to valid system-allocated pageable memory, then the accessing device must
 have a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS for a read-only
 copy to be created on that device. Note however that if the accessing device also has a non-zero value for the
 device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES, then setting this advice
 will not create a read-only copy when that device accesses this memory region.

 - ::CU_MEM_ADVISE_UNSET_READ_MOSTLY:  Undoes the effect of ::CU_MEM_ADVISE_SET_READ_MOSTLY and also prevents the
 Unified Memory driver from attempting heuristic read-duplication on the memory range. Any read-duplicated
 copies of the data will be collapsed into a single copy. The location for the collapsed
 copy will be the preferred location if the page has a preferred location and one of the read-duplicated
 copies was resident at that location. Otherwise, the location chosen is arbitrary.
 Note: The \p location argument is ignored for this advice.

 - ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION: This advice sets the preferred location for the
 data to be the memory belonging to \p location. When ::CUmemLocation::type is ::CU_MEM_LOCATION_TYPE_HOST,
 ::CUmemLocation::id is ignored and the preferred location is set to be host memory. To set the preferred location
 to a specific host NUMA node, applications must set ::CUmemLocation::type to ::CU_MEM_LOCATION_TYPE_HOST_NUMA and
 ::CUmemLocation::id must specify the NUMA ID of the host NUMA node. If ::CUmemLocation::type is set to ::CU_MEM_LOCATION_TYPE_HOST_NUMA_CURRENT,
 ::CUmemLocation::id will be ignored and the the host NUMA node closest to the calling thread's CPU will be used as the preferred location.
 If ::CUmemLocation::type is a ::CU_MEM_LOCATION_TYPE_DEVICE, then ::CUmemLocation::id must be a valid device ordinal
 and the device must have a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS.
 Setting the preferred location does not cause data to migrate to that location immediately. Instead, it guides the migration policy
 when a fault occurs on that memory region. If the data is already in its preferred location and the
 faulting processor can establish a mapping without requiring the data to be migrated, then
 data migration will be avoided. On the other hand, if the data is not in its preferred location
 or if a direct mapping cannot be established, then it will be migrated to the processor accessing
 it. It is important to note that setting the preferred location does not prevent data prefetching
 done using ::cuMemPrefetchAsync.
 Having a preferred location can override the page thrash detection and resolution logic in the Unified
 Memory driver. Normally, if a page is detected to be constantly thrashing between for example host and device
 memory, the page may eventually be pinned to host memory by the Unified Memory driver. But
 if the preferred location is set as device memory, then the page will continue to thrash indefinitely.
 If ::CU_MEM_ADVISE_SET_READ_MOSTLY is also set on this memory region or any subset of it, then the
 policies associated with that advice will override the policies of this advice, unless read accesses from
 \p location will not result in a read-only copy being created on that procesor as outlined in description for
 the advice ::CU_MEM_ADVISE_SET_READ_MOSTLY.
 If the memory region refers to valid system-allocated pageable memory, and ::CUmemLocation::type is CU_MEM_LOCATION_TYPE_DEVICE
 then ::CUmemLocation::id must be a valid device that has a non-zero alue for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS.

 - ::CU_MEM_ADVISE_UNSET_PREFERRED_LOCATION: Undoes the effect of ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION
 and changes the preferred location to none. The \p location argument is ignored for this advice.

 - ::CU_MEM_ADVISE_SET_ACCESSED_BY: This advice implies that the data will be accessed by processor \p location.
 The ::CUmemLocation::type must be either ::CU_MEM_LOCATION_TYPE_DEVICE with ::CUmemLocation::id representing a valid device
 ordinal or ::CU_MEM_LOCATION_TYPE_HOST and ::CUmemLocation::id will be ignored. All other location types are invalid.
 If ::CUmemLocation::id is a GPU, then the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS must be non-zero.
 This advice does not cause data migration and has no impact on the location of the data per se. Instead,
 it causes the data to always be mapped in the specified processor's page tables, as long as the
 location of the data permits a mapping to be established. If the data gets migrated for any reason,
 the mappings are updated accordingly.
 This advice is recommended in scenarios where data locality is not important, but avoiding faults is.
 Consider for example a system containing multiple GPUs with peer-to-peer access enabled, where the
 data located on one GPU is occasionally accessed by peer GPUs. In such scenarios, migrating data
 over to the other GPUs is not as important because the accesses are infrequent and the overhead of
 migration may be too high. But preventing faults can still help improve performance, and so having
 a mapping set up in advance is useful. Note that on CPU access of this data, the data may be migrated
 to host memory because the CPU typically cannot access device memory directly. Any GPU that had the
 ::CU_MEM_ADVISE_SET_ACCESSED_BY flag set for this data will now have its mapping updated to point to the
 page in host memory.
 If ::CU_MEM_ADVISE_SET_READ_MOSTLY is also set on this memory region or any subset of it, then the
 policies associated with that advice will override the policies of this advice. Additionally, if the
 preferred location of this memory region or any subset of it is also \p location, then the policies
 associated with ::CU_MEM_ADVISE_SET_PREFERRED_LOCATION will override the policies of this advice.
 If the memory region refers to valid system-allocated pageable memory, and ::CUmemLocation::type is ::CU_MEM_LOCATION_TYPE_DEVICE
 then device in ::CUmemLocation::id must have a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS.
 Additionally, if ::CUmemLocation::id has a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES,
 then this call has no effect.

 - ::CU_MEM_ADVISE_UNSET_ACCESSED_BY: Undoes the effect of ::CU_MEM_ADVISE_SET_ACCESSED_BY. Any mappings to
 the data from \p location may be removed at any time causing accesses to result in non-fatal page faults.
 If the memory region refers to valid system-allocated pageable memory, and ::CUmemLocation::type is ::CU_MEM_LOCATION_TYPE_DEVICE
 then device in ::CUmemLocation::id must have a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS.
 Additionally, if ::CUmemLocation::id has a non-zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES,
 then this call has no effect.

 \param devPtr   - Pointer to memory to set the advice for
 \param count    - Size in bytes of the memory range
 \param advice   - Advice to be applied for the specified memory range
 \param location - location to apply the advice for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuMemcpy, ::cuMemcpyPeer, ::cuMemcpyAsync,
 ::cuMemcpy3DPeerAsync, ::cuMemPrefetchAsync, ::cuMemAdvise,
 ::cudaMemAdvise*/
    fn cuMemAdvise_v2(
        devPtr: cuda_types::cuda::CUdeviceptr,
        count: usize,
        advice: cuda_types::cuda::CUmem_advise,
        location: cuda_types::cuda::CUmemLocation,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query an attribute of a given memory range

 Query an attribute about the memory range starting at \p devPtr with a size of \p count bytes. The
 memory range must refer to managed memory allocated via ::cuMemAllocManaged or declared via
 __managed__ variables.

 The \p attribute parameter can take the following values:
 - ::CU_MEM_RANGE_ATTRIBUTE_READ_MOSTLY: If this attribute is specified, \p data will be interpreted
 as a 32-bit integer, and \p dataSize must be 4. The result returned will be 1 if all pages in the given
 memory range have read-duplication enabled, or 0 otherwise.
 - ::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION: If this attribute is specified, \p data will be
 interpreted as a 32-bit integer, and \p dataSize must be 4. The result returned will be a GPU device
 id if all pages in the memory range have that GPU as their preferred location, or it will be CU_DEVICE_CPU
 if all pages in the memory range have the CPU as their preferred location, or it will be CU_DEVICE_INVALID
 if either all the pages don't have the same preferred location or some of the pages don't have a
 preferred location at all. Note that the actual location of the pages in the memory range at the time of
 the query may be different from the preferred location.
 - ::CU_MEM_RANGE_ATTRIBUTE_ACCESSED_BY: If this attribute is specified, \p data will be interpreted
 as an array of 32-bit integers, and \p dataSize must be a non-zero multiple of 4. The result returned
 will be a list of device ids that had ::CU_MEM_ADVISE_SET_ACCESSED_BY set for that entire memory range.
 If any device does not have that advice set for the entire memory range, that device will not be included.
 If \p data is larger than the number of devices that have that advice set for that memory range,
 CU_DEVICE_INVALID will be returned in all the extra space provided. For ex., if \p dataSize is 12
 (i.e. \p data has 3 elements) and only device 0 has the advice set, then the result returned will be
 { 0, CU_DEVICE_INVALID, CU_DEVICE_INVALID }. If \p data is smaller than the number of devices that have
 that advice set, then only as many devices will be returned as can fit in the array. There is no
 guarantee on which specific devices will be returned, however.
 - ::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION: If this attribute is specified, \p data will be
 interpreted as a 32-bit integer, and \p dataSize must be 4. The result returned will be the last location
 to which all pages in the memory range were prefetched explicitly via ::cuMemPrefetchAsync. This will either be
 a GPU id or CU_DEVICE_CPU depending on whether the last location for prefetch was a GPU or the CPU
 respectively. If any page in the memory range was never explicitly prefetched or if all pages were not
 prefetched to the same location, CU_DEVICE_INVALID will be returned. Note that this simply returns the
 last location that the application requested to prefetch the memory range to. It gives no indication as to
 whether the prefetch operation to that location has completed or even begun.
 - ::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_TYPE: If this attribute is specified, \p data will be
 interpreted as a ::CUmemLocationType, and \p dataSize must be sizeof(CUmemLocationType). The ::CUmemLocationType returned will be
 ::CU_MEM_LOCATION_TYPE_DEVICE if all pages in the memory range have the same GPU as their preferred location, or ::CUmemLocationType
 will be ::CU_MEM_LOCATION_TYPE_HOST if all pages in the memory range have the CPU as their preferred location, or it will be ::CU_MEM_LOCATION_TYPE_HOST_NUMA
 if all the pages in the memory range have the same host NUMA node ID as their preferred location or it will be ::CU_MEM_LOCATION_TYPE_INVALID
 if either all the pages don't have the same preferred location or some of the pages don't have a preferred location at all.
 Note that the actual location type of the pages in the memory range at the time of the query may be different from the preferred location type.
  - ::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_ID: If this attribute is specified, \p data will be
 interpreted as a 32-bit integer, and \p dataSize must be 4. If the ::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_TYPE query for the same address range
 returns ::CU_MEM_LOCATION_TYPE_DEVICE, it will be a valid device ordinal or if it returns ::CU_MEM_LOCATION_TYPE_HOST_NUMA, it will be a valid host NUMA node ID
 or if it returns any other location type, the id should be ignored.
 - ::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_TYPE: If this attribute is specified, \p data will be
 interpreted as a ::CUmemLocationType, and \p dataSize must be sizeof(CUmemLocationType). The result returned will be the last location
 to which all pages in the memory range were prefetched explicitly via ::cuMemPrefetchAsync. The ::CUmemLocationType returned
 will be ::CU_MEM_LOCATION_TYPE_DEVICE if the last prefetch location was a GPU or ::CU_MEM_LOCATION_TYPE_HOST if it was the CPU or ::CU_MEM_LOCATION_TYPE_HOST_NUMA if
 the last prefetch location was a specific host NUMA node. If any page in the memory range was never explicitly prefetched or if all pages were not
 prefetched to the same location, ::CUmemLocationType will be ::CU_MEM_LOCATION_TYPE_INVALID.
 Note that this simply returns the last location type that the application requested to prefetch the memory range to. It gives no indication as to
 whether the prefetch operation to that location has completed or even begun.
  - ::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_ID: If this attribute is specified, \p data will be
 interpreted as a 32-bit integer, and \p dataSize must be 4. If the ::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_TYPE query for the same address range
 returns ::CU_MEM_LOCATION_TYPE_DEVICE, it will be a valid device ordinal or if it returns ::CU_MEM_LOCATION_TYPE_HOST_NUMA, it will be a valid host NUMA node ID
 or if it returns any other location type, the id should be ignored.

 \param data      - A pointers to a memory location where the result
                    of each attribute query will be written to.
 \param dataSize  - Array containing the size of data
 \param attribute - The attribute to query
 \param devPtr    - Start of the range to query
 \param count     - Size of the range to query

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr
 \note_async
 \note_null_stream

 \sa ::cuMemRangeGetAttributes, ::cuMemPrefetchAsync,
 ::cuMemAdvise,
 ::cudaMemRangeGetAttribute*/
    fn cuMemRangeGetAttribute(
        data: *mut ::core::ffi::c_void,
        dataSize: usize,
        attribute: cuda_types::cuda::CUmem_range_attribute,
        devPtr: cuda_types::cuda::CUdeviceptr,
        count: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query attributes of a given memory range.

 Query attributes of the memory range starting at \p devPtr with a size of \p count bytes. The
 memory range must refer to managed memory allocated via ::cuMemAllocManaged or declared via
 __managed__ variables. The \p attributes array will be interpreted to have \p numAttributes
 entries. The \p dataSizes array will also be interpreted to have \p numAttributes entries.
 The results of the query will be stored in \p data.

 The list of supported attributes are given below. Please refer to ::cuMemRangeGetAttribute for
 attribute descriptions and restrictions.

 - ::CU_MEM_RANGE_ATTRIBUTE_READ_MOSTLY
 - ::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION
 - ::CU_MEM_RANGE_ATTRIBUTE_ACCESSED_BY
 - ::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION
 - ::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_TYPE
 - ::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_ID
 - ::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_TYPE
 - ::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_ID

 \param data          - A two-dimensional array containing pointers to memory
                        locations where the result of each attribute query will be written to.
 \param dataSizes     - Array containing the sizes of each result
 \param attributes    - An array of attributes to query
                        (numAttributes and the number of attributes in this array should match)
 \param numAttributes - Number of attributes to query
 \param devPtr        - Start of the range to query
 \param count         - Size of the range to query

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa ::cuMemRangeGetAttribute, ::cuMemAdvise,
 ::cuMemPrefetchAsync,
 ::cudaMemRangeGetAttributes*/
    fn cuMemRangeGetAttributes(
        data: *mut *mut ::core::ffi::c_void,
        dataSizes: *mut usize,
        attributes: *mut cuda_types::cuda::CUmem_range_attribute,
        numAttributes: usize,
        devPtr: cuda_types::cuda::CUdeviceptr,
        count: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Set attributes on a previously allocated memory region

 The supported attributes are:

 - ::CU_POINTER_ATTRIBUTE_SYNC_MEMOPS:

      A boolean attribute that can either be set (1) or unset (0). When set,
      the region of memory that \p ptr points to is guaranteed to always synchronize
      memory operations that are synchronous. If there are some previously initiated
      synchronous memory operations that are pending when this attribute is set, the
      function does not return until those memory operations are complete.
      See further documentation in the section titled "API synchronization behavior"
      to learn more about cases when synchronous memory operations can
      exhibit asynchronous behavior.
      \p value will be considered as a pointer to an unsigned integer to which this attribute is to be set.

 \param value     - Pointer to memory containing the value to be set
 \param attribute - Pointer attribute to set
 \param ptr       - Pointer to a memory region allocated using CUDA memory allocation APIs

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa ::cuPointerGetAttribute,
 ::cuPointerGetAttributes,
 ::cuMemAlloc,
 ::cuMemFree,
 ::cuMemAllocHost,
 ::cuMemFreeHost,
 ::cuMemHostAlloc,
 ::cuMemHostRegister,
 ::cuMemHostUnregister*/
    fn cuPointerSetAttribute(
        value: *const ::core::ffi::c_void,
        attribute: cuda_types::cuda::CUpointer_attribute,
        ptr: cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns information about a pointer.

 The supported attributes are (refer to ::cuPointerGetAttribute for attribute descriptions and restrictions):

 - ::CU_POINTER_ATTRIBUTE_CONTEXT
 - ::CU_POINTER_ATTRIBUTE_MEMORY_TYPE
 - ::CU_POINTER_ATTRIBUTE_DEVICE_POINTER
 - ::CU_POINTER_ATTRIBUTE_HOST_POINTER
 - ::CU_POINTER_ATTRIBUTE_SYNC_MEMOPS
 - ::CU_POINTER_ATTRIBUTE_BUFFER_ID
 - ::CU_POINTER_ATTRIBUTE_IS_MANAGED
 - ::CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL
 - ::CU_POINTER_ATTRIBUTE_RANGE_START_ADDR
 - ::CU_POINTER_ATTRIBUTE_RANGE_SIZE
 - ::CU_POINTER_ATTRIBUTE_MAPPED
 - ::CU_POINTER_ATTRIBUTE_IS_LEGACY_CUDA_IPC_CAPABLE
 - ::CU_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES
 - ::CU_POINTER_ATTRIBUTE_MEMPOOL_HANDLE
 - ::CU_POINTER_ATTRIBUTE_IS_HW_DECOMPRESS_CAPABLE

 \param numAttributes - Number of attributes to query
 \param attributes    - An array of attributes to query
                      (numAttributes and the number of attributes in this array should match)
 \param data          - A two-dimensional array containing pointers to memory
                      locations where the result of each attribute query will be written to.
 \param ptr           - Pointer to query

 Unlike ::cuPointerGetAttribute, this function will not return an error when the \p ptr
 encountered is not a valid CUDA pointer. Instead, the attributes are assigned default NULL values
 and CUDA_SUCCESS is returned.

 If \p ptr was not allocated by, mapped by, or registered with a ::CUcontext which uses UVA
 (Unified Virtual Addressing), ::CUDA_ERROR_INVALID_CONTEXT is returned.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuPointerGetAttribute,
 ::cuPointerSetAttribute,
 ::cudaPointerGetAttributes*/
    fn cuPointerGetAttributes(
        numAttributes: ::core::ffi::c_uint,
        attributes: *mut cuda_types::cuda::CUpointer_attribute,
        data: *mut *mut ::core::ffi::c_void,
        ptr: cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a stream

 Creates a stream and returns a handle in \p phStream.  The \p Flags argument
 determines behaviors of the stream.

 Valid values for \p Flags are:
 - ::CU_STREAM_DEFAULT: Default stream creation flag.
 - ::CU_STREAM_NON_BLOCKING: Specifies that work running in the created
   stream may run concurrently with work in stream 0 (the NULL stream), and that
   the created stream should perform no implicit synchronization with stream 0.

 \param phStream - Returned newly created stream
 \param Flags    - Parameters for stream creation

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuStreamDestroy,
 ::cuStreamCreateWithPriority,
 ::cuGreenCtxStreamCreate,
 ::cuStreamGetPriority,
 ::cuStreamGetFlags,
 ::cuStreamGetDevice
 ::cuStreamWaitEvent,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamAddCallback,
 ::cudaStreamCreate,
 ::cudaStreamCreateWithFlags*/
    fn cuStreamCreate(
        phStream: *mut cuda_types::cuda::CUstream,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a stream with the given priority

 Creates a stream with the specified priority and returns a handle in \p phStream.
 This affects the scheduling priority of work in the stream. Priorities provide a
 hint to preferentially run work with higher priority when possible, but do
 not preempt already-running work or provide any other functional guarantee on
 execution order.

 \p priority follows a convention where lower numbers represent higher priorities.
 '0' represents default priority. The range of meaningful numerical priorities can
 be queried using ::cuCtxGetStreamPriorityRange. If the specified priority is
 outside the numerical range returned by ::cuCtxGetStreamPriorityRange,
 it will automatically be clamped to the lowest or the highest number in the range.

 \param phStream    - Returned newly created stream
 \param flags       - Flags for stream creation. See ::cuStreamCreate for a list of
                      valid flags
 \param priority    - Stream priority. Lower numbers represent higher priorities.
                      See ::cuCtxGetStreamPriorityRange for more information about
                      meaningful stream priorities that can be passed.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \note Stream priorities are supported only on GPUs
 with compute capability 3.5 or higher.

 \note In the current implementation, only compute kernels launched in
 priority streams are affected by the stream's priority. Stream priorities have
 no effect on host-to-device and device-to-host memory operations.

 \sa ::cuStreamDestroy,
 ::cuStreamCreate,
 ::cuGreenCtxStreamCreate,
 ::cuStreamGetPriority,
 ::cuCtxGetStreamPriorityRange,
 ::cuStreamGetFlags,
 ::cuStreamGetDevice,
 ::cuStreamWaitEvent,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamAddCallback,
 ::cudaStreamCreateWithPriority*/
    fn cuStreamCreateWithPriority(
        phStream: *mut cuda_types::cuda::CUstream,
        flags: ::core::ffi::c_uint,
        priority: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query the priority of a given stream

 Query the priority of a stream created using ::cuStreamCreate, ::cuStreamCreateWithPriority or ::cuGreenCtxStreamCreate
 and return the priority in \p priority. Note that if the stream was created with a
 priority outside the numerical range returned by ::cuCtxGetStreamPriorityRange,
 this function returns the clamped priority.
 See ::cuStreamCreateWithPriority for details about priority clamping.

 \param hStream    - Handle to the stream to be queried
 \param priority   - Pointer to a signed integer in which the stream's priority is returned
 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuStreamDestroy,
 ::cuStreamCreate,
 ::cuStreamCreateWithPriority,
 ::cuGreenCtxStreamCreate,
 ::cuCtxGetStreamPriorityRange,
 ::cuStreamGetFlags,
 ::cuStreamGetDevice,
 ::cudaStreamGetPriority*/
    fn cuStreamGetPriority_ptsz(
        hStream: cuda_types::cuda::CUstream,
        priority: *mut ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the device handle of the stream

 Returns in \p *device the device handle of the stream

 \param hStream - Handle to the stream to be queried
 \param device - Returns the device to which a stream belongs

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa
 ::cuStreamDestroy,
 ::cuStreamCreate,
 ::cuGreenCtxStreamCreate,
 ::cuStreamGetFlags*/
    fn cuStreamGetDevice_ptsz(
        hStream: cuda_types::cuda::CUstream,
        device: *mut cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query the flags of a given stream

 Query the flags of a stream created using ::cuStreamCreate, ::cuStreamCreateWithPriority or ::cuGreenCtxStreamCreate
 and return the flags in \p flags.

 \param hStream    - Handle to the stream to be queried
 \param flags      - Pointer to an unsigned integer in which the stream's flags are returned
                     The value returned in \p flags is a logical 'OR' of all flags that
                     were used while creating this stream. See ::cuStreamCreate for the list
                     of valid flags
 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuStreamDestroy,
 ::cuStreamCreate,
 ::cuGreenCtxStreamCreate,
 ::cuStreamGetPriority,
 ::cudaStreamGetFlags,
 ::cuStreamGetDevice*/
    fn cuStreamGetFlags_ptsz(
        hStream: cuda_types::cuda::CUstream,
        flags: *mut ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the unique Id associated with the stream handle supplied

 Returns in \p streamId the unique Id which is associated with the given stream handle.
 The Id is unique for the life of the program.

 The stream handle \p hStream can refer to any of the following:
 <ul>
   <li>a stream created via any of the CUDA driver APIs such as ::cuStreamCreate
   and ::cuStreamCreateWithPriority, or their runtime API equivalents such as
   ::cudaStreamCreate, ::cudaStreamCreateWithFlags and ::cudaStreamCreateWithPriority.
   Passing an invalid handle will result in undefined behavior.</li>
   <li>any of the special streams such as the NULL stream, ::CU_STREAM_LEGACY and
   ::CU_STREAM_PER_THREAD. The runtime API equivalents of these are also accepted,
   which are NULL, ::cudaStreamLegacy and ::cudaStreamPerThread respectively.</li>
 </ul>

 \param hStream    - Handle to the stream to be queried
 \param streamId   - Pointer to store the Id of the stream

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuStreamDestroy,
 ::cuStreamCreate,
 ::cuStreamGetPriority,
 ::cudaStreamGetId*/
    fn cuStreamGetId_ptsz(
        hStream: cuda_types::cuda::CUstream,
        streamId: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query the context associated with a stream

 Returns the CUDA context that the stream is associated with.

 Note there is a later version of this API, ::cuStreamGetCtx_v2. It will
 supplant this version in CUDA 13.0. It is recommended to use ::cuStreamGetCtx_v2
 till then as this version will return ::CUDA_ERROR_NOT_SUPPORTED for streams created via the API ::cuGreenCtxStreamCreate.

 The stream handle \p hStream can refer to any of the following:
 <ul>
   <li>a stream created via any of the CUDA driver APIs such as ::cuStreamCreate
   and ::cuStreamCreateWithPriority, or their runtime API equivalents such as
   ::cudaStreamCreate, ::cudaStreamCreateWithFlags and ::cudaStreamCreateWithPriority.
   The returned context is the context that was active in the calling thread when the
   stream was created. Passing an invalid handle will result in undefined behavior.</li>
   <li>any of the special streams such as the NULL stream, ::CU_STREAM_LEGACY and
   ::CU_STREAM_PER_THREAD. The runtime API equivalents of these are also accepted,
   which are NULL, ::cudaStreamLegacy and ::cudaStreamPerThread respectively.
   Specifying any of the special handles will return the context current to the
   calling thread. If no context is current to the calling thread,
   ::CUDA_ERROR_INVALID_CONTEXT is returned.</li>
 </ul>

 \param hStream - Handle to the stream to be queried
 \param pctx    - Returned context associated with the stream

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuStreamDestroy,
 ::cuStreamCreateWithPriority,
 ::cuStreamGetPriority,
 ::cuStreamGetFlags,
 ::cuStreamGetDevice
 ::cuStreamWaitEvent,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamAddCallback,
 ::cudaStreamCreate,
 ::cuStreamGetCtx_v2,
 ::cudaStreamCreateWithFlags*/
    fn cuStreamGetCtx_ptsz(
        hStream: cuda_types::cuda::CUstream,
        pctx: *mut cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query the contexts associated with a stream

 Returns the contexts that the stream is associated with.

 If the stream is associated with a green context, the API returns the green context in \p pGreenCtx
 and the primary context of the associated device in \p pCtx.

 If the stream is associated with a regular context, the API returns the regular context in \p pCtx
 and NULL in \p pGreenCtx.

 The stream handle \p hStream can refer to any of the following:
 <ul>
   <li>a stream created via any of the CUDA driver APIs such as ::cuStreamCreate,
   ::cuStreamCreateWithPriority and ::cuGreenCtxStreamCreate, or their runtime API equivalents such as
   ::cudaStreamCreate, ::cudaStreamCreateWithFlags and ::cudaStreamCreateWithPriority.
   Passing an invalid handle will result in undefined behavior.</li>
   <li>any of the special streams such as the NULL stream, ::CU_STREAM_LEGACY and
   ::CU_STREAM_PER_THREAD. The runtime API equivalents of these are also accepted,
   which are NULL, ::cudaStreamLegacy and ::cudaStreamPerThread respectively.
   If any of the special handles are specified, the API will operate on the context current to the
   calling thread. If a green context (that was converted via ::cuCtxFromGreenCtx() before setting it current)
   is current to the calling thread, the API will return the green context in \p pGreenCtx
   and the primary context of the associated device in \p pCtx. If a regular context is current,
   the API returns the regular context in \p pCtx and NULL in \p pGreenCtx.
   Note that specifying ::CU_STREAM_PER_THREAD or ::cudaStreamPerThread will return ::CUDA_ERROR_INVALID_HANDLE
   if a green context is current to the calling thread.
   If no context is current to the calling thread, ::CUDA_ERROR_INVALID_CONTEXT is returned.</li>
 </ul>

 \param hStream   - Handle to the stream to be queried
 \param pCtx      - Returned regular context associated with the stream
 \param pGreenCtx - Returned green context if the stream is associated with a green context or NULL if not

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuStreamDestroy,
 ::cuStreamCreate
 ::cuStreamCreateWithPriority,
 ::cuGreenCtxStreamCreate,
 ::cuStreamGetPriority,
 ::cuStreamGetFlags,
 ::cuStreamGetDevice,
 ::cuStreamWaitEvent,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamAddCallback,
 ::cudaStreamCreate,
 ::cudaStreamCreateWithFlags,*/
    fn cuStreamGetCtx_v2_ptsz(
        hStream: cuda_types::cuda::CUstream,
        pCtx: *mut cuda_types::cuda::CUcontext,
        pGreenCtx: *mut cuda_types::cuda::CUgreenCtx,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Make a compute stream wait on an event

 Makes all future work submitted to \p hStream wait for all work captured in
 \p hEvent.  See ::cuEventRecord() for details on what is captured by an event.
 The synchronization will be performed efficiently on the device when applicable.
 \p hEvent may be from a different context or device than \p hStream.

 flags include:
 - ::CU_EVENT_WAIT_DEFAULT: Default event creation flag.
 - ::CU_EVENT_WAIT_EXTERNAL: Event is captured in the graph as an external
   event node when performing stream capture. This flag is invalid outside
   of stream capture.

 \param hStream - Stream to wait
 \param hEvent  - Event to wait on (may not be NULL)
 \param Flags   - See ::CUevent_capture_flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 \note_null_stream
 \notefnerr

 \sa ::cuStreamCreate,
 ::cuEventRecord,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamAddCallback,
 ::cuStreamDestroy,
 ::cudaStreamWaitEvent*/
    fn cuStreamWaitEvent_ptsz(
        hStream: cuda_types::cuda::CUstream,
        hEvent: cuda_types::cuda::CUevent,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Add a callback to a compute stream

 \note This function is slated for eventual deprecation and removal. If
 you do not require the callback to execute in case of a device error,
 consider using ::cuLaunchHostFunc. Additionally, this function is not
 supported with ::cuStreamBeginCapture and ::cuStreamEndCapture, unlike
 ::cuLaunchHostFunc.

 Adds a callback to be called on the host after all currently enqueued
 items in the stream have completed.  For each
 cuStreamAddCallback call, the callback will be executed exactly once.
 The callback will block later work in the stream until it is finished.

 The callback may be passed ::CUDA_SUCCESS or an error code.  In the event
 of a device error, all subsequently executed callbacks will receive an
 appropriate ::CUresult.

 Callbacks must not make any CUDA API calls.  Attempting to use a CUDA API
 will result in ::CUDA_ERROR_NOT_PERMITTED.  Callbacks must not perform any
 synchronization that may depend on outstanding device work or other callbacks
 that are not mandated to run earlier.  Callbacks without a mandated order
 (in independent streams) execute in undefined order and may be serialized.

 For the purposes of Unified Memory, callback execution makes a number of
 guarantees:
 <ul>
   <li>The callback stream is considered idle for the duration of the
   callback.  Thus, for example, a callback may always use memory attached
   to the callback stream.</li>
   <li>The start of execution of a callback has the same effect as
   synchronizing an event recorded in the same stream immediately prior to
   the callback.  It thus synchronizes streams which have been "joined"
   prior to the callback.</li>
   <li>Adding device work to any stream does not have the effect of making
   the stream active until all preceding host functions and stream callbacks
   have executed.  Thus, for
   example, a callback might use global attached memory even if work has
   been added to another stream, if the work has been ordered behind the
   callback with an event.</li>
   <li>Completion of a callback does not cause a stream to become
   active except as described above.  The callback stream will remain idle
   if no device work follows the callback, and will remain idle across
   consecutive callbacks without device work in between.  Thus, for example,
   stream synchronization can be done by signaling from a callback at the
   end of the stream.</li>
 </ul>

 \param hStream  - Stream to add callback to
 \param callback - The function to call once preceding stream operations are complete
 \param userData - User specified data to be passed to the callback function
 \param flags    - Reserved for future use, must be 0

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \note_null_stream
 \notefnerr

 \sa ::cuStreamCreate,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamWaitEvent,
 ::cuStreamDestroy,
 ::cuMemAllocManaged,
 ::cuStreamAttachMemAsync,
 ::cuLaunchHostFunc,
 ::cudaStreamAddCallback*/
    fn cuStreamAddCallback_ptsz(
        hStream: cuda_types::cuda::CUstream,
        callback: cuda_types::cuda::CUstreamCallback,
        userData: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Begins graph capture on a stream

 Begin graph capture on \p hStream. When a stream is in capture mode, all operations
 pushed into the stream will not be executed, but will instead be captured into
 a graph, which will be returned via ::cuStreamEndCapture. Capture may not be initiated
 if \p stream is CU_STREAM_LEGACY. Capture must be ended on the same stream in which
 it was initiated, and it may only be initiated if the stream is not already in capture
 mode. The capture mode may be queried via ::cuStreamIsCapturing. A unique id
 representing the capture sequence may be queried via ::cuStreamGetCaptureInfo.

 If \p mode is not ::CU_STREAM_CAPTURE_MODE_RELAXED, ::cuStreamEndCapture must be
 called on this stream from the same thread.

 \param hStream - Stream in which to initiate capture
 \param mode    - Controls the interaction of this capture sequence with other API
                  calls that are potentially unsafe. For more details see
                  ::cuThreadExchangeStreamCaptureMode.

 \note Kernels captured using this API must not use texture and surface references.
       Reading or writing through any texture or surface reference is undefined
       behavior. This restriction does not apply to texture and surface objects.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::cuStreamCreate,
 ::cuStreamIsCapturing,
 ::cuStreamEndCapture,
 ::cuThreadExchangeStreamCaptureMode*/
    fn cuStreamBeginCapture_v2_ptsz(
        hStream: cuda_types::cuda::CUstream,
        mode: cuda_types::cuda::CUstreamCaptureMode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Begins graph capture on a stream to an existing graph

 Begin graph capture on \p hStream, placing new nodes into an existing graph. When a stream is
 in capture mode, all operations pushed into the stream will not be executed, but will instead
 be captured into \p hGraph. The graph will not be instantiable until the user calls
 ::cuStreamEndCapture.

 Capture may not be initiated if \p stream is CU_STREAM_LEGACY. Capture must be ended on the
 same stream in which it was initiated, and it may only be initiated if the stream is not
 already in capture mode. The capture mode may be queried via ::cuStreamIsCapturing. A unique id
 representing the capture sequence may be queried via ::cuStreamGetCaptureInfo.

 If \p mode is not ::CU_STREAM_CAPTURE_MODE_RELAXED, ::cuStreamEndCapture must be
 called on this stream from the same thread.

 \param hStream         - Stream in which to initiate capture.
 \param hGraph          - Graph to capture into.
 \param dependencies    - Dependencies of the first node captured in the stream.  Can be NULL if numDependencies is 0.
 \param dependencyData  - Optional array of data associated with each dependency.
 \param numDependencies - Number of dependencies.
 \param mode            - Controls the interaction of this capture sequence with other API
                          calls that are potentially unsafe. For more details see
                          ::cuThreadExchangeStreamCaptureMode.

 \note Kernels captured using this API must not use texture and surface references.
       Reading or writing through any texture or surface reference is undefined
       behavior. This restriction does not apply to texture and surface objects.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::cuStreamBeginCapture,
 ::cuStreamCreate,
 ::cuStreamIsCapturing,
 ::cuStreamEndCapture,
 ::cuThreadExchangeStreamCaptureMode,
 ::cuGraphAddNode*/
    fn cuStreamBeginCaptureToGraph_ptsz(
        hStream: cuda_types::cuda::CUstream,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
        numDependencies: usize,
        mode: cuda_types::cuda::CUstreamCaptureMode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Swaps the stream capture interaction mode for a thread

 Sets the calling thread's stream capture interaction mode to the value contained
 in \p *mode, and overwrites \p *mode with the previous mode for the thread. To
 facilitate deterministic behavior across function or module boundaries, callers
 are encouraged to use this API in a push-pop fashion: \code
CUstreamCaptureMode mode = desiredMode;
cuThreadExchangeStreamCaptureMode(&mode);
...
cuThreadExchangeStreamCaptureMode(&mode); // restore previous mode
 \endcode

 During stream capture (see ::cuStreamBeginCapture), some actions, such as a call
 to ::cudaMalloc, may be unsafe. In the case of ::cudaMalloc, the operation is
 not enqueued asynchronously to a stream, and is not observed by stream capture.
 Therefore, if the sequence of operations captured via ::cuStreamBeginCapture
 depended on the allocation being replayed whenever the graph is launched, the
 captured graph would be invalid.

 Therefore, stream capture places restrictions on API calls that can be made within
 or concurrently to a ::cuStreamBeginCapture-::cuStreamEndCapture sequence. This
 behavior can be controlled via this API and flags to ::cuStreamBeginCapture.

 A thread's mode is one of the following:
 - \p CU_STREAM_CAPTURE_MODE_GLOBAL: This is the default mode. If the local thread has
   an ongoing capture sequence that was not initiated with
   \p CU_STREAM_CAPTURE_MODE_RELAXED at \p cuStreamBeginCapture, or if any other thread
   has a concurrent capture sequence initiated with \p CU_STREAM_CAPTURE_MODE_GLOBAL,
   this thread is prohibited from potentially unsafe API calls.
 - \p CU_STREAM_CAPTURE_MODE_THREAD_LOCAL: If the local thread has an ongoing capture
   sequence not initiated with \p CU_STREAM_CAPTURE_MODE_RELAXED, it is prohibited
   from potentially unsafe API calls. Concurrent capture sequences in other threads
   are ignored.
 - \p CU_STREAM_CAPTURE_MODE_RELAXED: The local thread is not prohibited from potentially
   unsafe API calls. Note that the thread is still prohibited from API calls which
   necessarily conflict with stream capture, for example, attempting ::cuEventQuery
   on an event that was last recorded inside a capture sequence.

 \param mode - Pointer to mode value to swap with the current mode

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::cuStreamBeginCapture*/
    fn cuThreadExchangeStreamCaptureMode(
        mode: *mut cuda_types::cuda::CUstreamCaptureMode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Ends capture on a stream, returning the captured graph

 End capture on \p hStream, returning the captured graph via \p phGraph.
 Capture must have been initiated on \p hStream via a call to ::cuStreamBeginCapture.
 If capture was invalidated, due to a violation of the rules of stream capture, then
 a NULL graph will be returned.

 If the \p mode argument to ::cuStreamBeginCapture was not
 ::CU_STREAM_CAPTURE_MODE_RELAXED, this call must be from the same thread as
 ::cuStreamBeginCapture.

 \param hStream - Stream to query
 \param phGraph - The captured graph

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_STREAM_CAPTURE_WRONG_THREAD
 \notefnerr

 \sa
 ::cuStreamCreate,
 ::cuStreamBeginCapture,
 ::cuStreamIsCapturing,
 ::cuGraphDestroy*/
    fn cuStreamEndCapture_ptsz(
        hStream: cuda_types::cuda::CUstream,
        phGraph: *mut cuda_types::cuda::CUgraph,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a stream's capture status

 Return the capture status of \p hStream via \p captureStatus. After a successful
 call, \p *captureStatus will contain one of the following:
 - ::CU_STREAM_CAPTURE_STATUS_NONE: The stream is not capturing.
 - ::CU_STREAM_CAPTURE_STATUS_ACTIVE: The stream is capturing.
 - ::CU_STREAM_CAPTURE_STATUS_INVALIDATED: The stream was capturing but an error
   has invalidated the capture sequence. The capture sequence must be terminated
   with ::cuStreamEndCapture on the stream where it was initiated in order to
   continue using \p hStream.

 Note that, if this is called on ::CU_STREAM_LEGACY (the "null stream") while
 a blocking stream in the same context is capturing, it will return
 ::CUDA_ERROR_STREAM_CAPTURE_IMPLICIT and \p *captureStatus is unspecified
 after the call. The blocking stream capture is not invalidated.

 When a blocking stream is capturing, the legacy stream is in an
 unusable state until the blocking stream capture is terminated. The legacy
 stream is not supported for stream capture, but attempted use would have an
 implicit dependency on the capturing stream(s).

 \param hStream       - Stream to query
 \param captureStatus - Returns the stream's capture status

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_STREAM_CAPTURE_IMPLICIT
 \notefnerr

 \sa
 ::cuStreamCreate,
 ::cuStreamBeginCapture,
 ::cuStreamEndCapture*/
    fn cuStreamIsCapturing_ptsz(
        hStream: cuda_types::cuda::CUstream,
        captureStatus: *mut cuda_types::cuda::CUstreamCaptureStatus,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query a stream's capture state

 Query stream state related to stream capture.

 If called on ::CU_STREAM_LEGACY (the "null stream") while a stream not created
 with ::CU_STREAM_NON_BLOCKING is capturing, returns ::CUDA_ERROR_STREAM_CAPTURE_IMPLICIT.

 Valid data (other than capture status) is returned only if both of the following are true:
 - the call returns CUDA_SUCCESS
 - the returned capture status is ::CU_STREAM_CAPTURE_STATUS_ACTIVE

 \param hStream - The stream to query
 \param captureStatus_out - Location to return the capture status of the stream; required
 \param id_out - Optional location to return an id for the capture sequence, which is
           unique over the lifetime of the process
 \param graph_out - Optional location to return the graph being captured into. All
           operations other than destroy and node removal are permitted on the graph
           while the capture sequence is in progress. This API does not transfer
           ownership of the graph, which is transferred or destroyed at
           ::cuStreamEndCapture. Note that the graph handle may be invalidated before
           end of capture for certain errors. Nodes that are or become
           unreachable from the original stream at ::cuStreamEndCapture due to direct
           actions on the graph do not trigger ::CUDA_ERROR_STREAM_CAPTURE_UNJOINED.
 \param dependencies_out - Optional location to store a pointer to an array of nodes.
           The next node to be captured in the stream will depend on this set of nodes,
           absent operations such as event wait which modify this set. The array pointer
           is valid until the next API call which operates on the stream or until the
           capture is terminated. The node handles may be copied out and are valid until
           they or the graph is destroyed. The driver-owned array may also be passed
           directly to APIs that operate on the graph (not the stream) without copying.
 \param numDependencies_out - Optional location to store the size of the array
           returned in dependencies_out.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_STREAM_CAPTURE_IMPLICIT
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuStreamGetCaptureInfo_v3
 ::cuStreamBeginCapture,
 ::cuStreamIsCapturing,
 ::cuStreamUpdateCaptureDependencies*/
    fn cuStreamGetCaptureInfo_v2_ptsz(
        hStream: cuda_types::cuda::CUstream,
        captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
        id_out: *mut cuda_types::cuda::cuuint64_t,
        graph_out: *mut cuda_types::cuda::CUgraph,
        dependencies_out: *mut *const cuda_types::cuda::CUgraphNode,
        numDependencies_out: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query a stream's capture state (12.3+)

 Query stream state related to stream capture.

 If called on ::CU_STREAM_LEGACY (the "null stream") while a stream not created
 with ::CU_STREAM_NON_BLOCKING is capturing, returns ::CUDA_ERROR_STREAM_CAPTURE_IMPLICIT.

 Valid data (other than capture status) is returned only if both of the following are true:
 - the call returns CUDA_SUCCESS
 - the returned capture status is ::CU_STREAM_CAPTURE_STATUS_ACTIVE

 If \p edgeData_out is non-NULL then \p dependencies_out must be as well. If
 \p dependencies_out is non-NULL and \p edgeData_out is NULL, but there is non-zero edge
 data for one or more of the current stream dependencies, the call will return
 ::CUDA_ERROR_LOSSY_QUERY.

 \param hStream - The stream to query
 \param captureStatus_out - Location to return the capture status of the stream; required
 \param id_out - Optional location to return an id for the capture sequence, which is
           unique over the lifetime of the process
 \param graph_out - Optional location to return the graph being captured into. All
           operations other than destroy and node removal are permitted on the graph
           while the capture sequence is in progress. This API does not transfer
           ownership of the graph, which is transferred or destroyed at
           ::cuStreamEndCapture. Note that the graph handle may be invalidated before
           end of capture for certain errors. Nodes that are or become
           unreachable from the original stream at ::cuStreamEndCapture due to direct
           actions on the graph do not trigger ::CUDA_ERROR_STREAM_CAPTURE_UNJOINED.
 \param dependencies_out - Optional location to store a pointer to an array of nodes.
           The next node to be captured in the stream will depend on this set of nodes,
           absent operations such as event wait which modify this set. The array pointer
           is valid until the next API call which operates on the stream or until the
           capture is terminated. The node handles may be copied out and are valid until
           they or the graph is destroyed. The driver-owned array may also be passed
           directly to APIs that operate on the graph (not the stream) without copying.
 \param edgeData_out - Optional location to store a pointer to an array of graph edge
           data. This array parallels \c dependencies_out; the next node to be added
           has an edge to \c dependencies_out[i] with annotation \c edgeData_out[i] for
           each \c i. The array pointer is valid until the next API call which operates
           on the stream or until the capture is terminated.
 \param numDependencies_out - Optional location to store the size of the array
           returned in dependencies_out.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_STREAM_CAPTURE_IMPLICIT,
 ::CUDA_ERROR_LOSSY_QUERY
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuStreamGetCaptureInfo,
 ::cuStreamBeginCapture,
 ::cuStreamIsCapturing,
 ::cuStreamUpdateCaptureDependencies*/
    fn cuStreamGetCaptureInfo_v3_ptsz(
        hStream: cuda_types::cuda::CUstream,
        captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
        id_out: *mut cuda_types::cuda::cuuint64_t,
        graph_out: *mut cuda_types::cuda::CUgraph,
        dependencies_out: *mut *const cuda_types::cuda::CUgraphNode,
        edgeData_out: *mut *const cuda_types::cuda::CUgraphEdgeData,
        numDependencies_out: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Update the set of dependencies in a capturing stream (11.3+)

 Modifies the dependency set of a capturing stream. The dependency set is the set
 of nodes that the next captured node in the stream will depend on.

 Valid flags are ::CU_STREAM_ADD_CAPTURE_DEPENDENCIES and
 ::CU_STREAM_SET_CAPTURE_DEPENDENCIES. These control whether the set passed to
 the API is added to the existing set or replaces it. A flags value of 0 defaults
 to ::CU_STREAM_ADD_CAPTURE_DEPENDENCIES.

 Nodes that are removed from the dependency set via this API do not result in
 ::CUDA_ERROR_STREAM_CAPTURE_UNJOINED if they are unreachable from the stream at
 ::cuStreamEndCapture.

 Returns ::CUDA_ERROR_ILLEGAL_STATE if the stream is not capturing.

 This API is new in CUDA 11.3. Developers requiring compatibility across minor
 versions to CUDA 11.0 should not use this API or provide a fallback.

 \param hStream - The stream to update
 \param dependencies - The set of dependencies to add
 \param numDependencies - The size of the dependencies array
 \param flags - See above

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_ILLEGAL_STATE

 \sa
 ::cuStreamBeginCapture,
 ::cuStreamGetCaptureInfo,*/
    fn cuStreamUpdateCaptureDependencies_ptsz(
        hStream: cuda_types::cuda::CUstream,
        dependencies: *mut cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Update the set of dependencies in a capturing stream (12.3+)

 Modifies the dependency set of a capturing stream. The dependency set is the set
 of nodes that the next captured node in the stream will depend on along with the
 edge data for those dependencies.

 Valid flags are ::CU_STREAM_ADD_CAPTURE_DEPENDENCIES and
 ::CU_STREAM_SET_CAPTURE_DEPENDENCIES. These control whether the set passed to
 the API is added to the existing set or replaces it. A flags value of 0 defaults
 to ::CU_STREAM_ADD_CAPTURE_DEPENDENCIES.

 Nodes that are removed from the dependency set via this API do not result in
 ::CUDA_ERROR_STREAM_CAPTURE_UNJOINED if they are unreachable from the stream at
 ::cuStreamEndCapture.

 Returns ::CUDA_ERROR_ILLEGAL_STATE if the stream is not capturing.

 \param hStream - The stream to update
 \param dependencies - The set of dependencies to add
 \param dependencyData - Optional array of data associated with each dependency.
 \param numDependencies - The size of the dependencies array
 \param flags - See above

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_ILLEGAL_STATE

 \sa
 ::cuStreamBeginCapture,
 ::cuStreamGetCaptureInfo*/
    fn cuStreamUpdateCaptureDependencies_v2_ptsz(
        hStream: cuda_types::cuda::CUstream,
        dependencies: *mut cuda_types::cuda::CUgraphNode,
        dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Attach memory to a stream asynchronously

 Enqueues an operation in \p hStream to specify stream association of
 \p length bytes of memory starting from \p dptr. This function is a
 stream-ordered operation, meaning that it is dependent on, and will
 only take effect when, previous work in stream has completed. Any
 previous association is automatically replaced.

 \p dptr must point to one of the following types of memories:
 - managed memory declared using the __managed__ keyword or allocated with
   ::cuMemAllocManaged.
 - a valid host-accessible region of system-allocated pageable memory. This
   type of memory may only be specified if the device associated with the
   stream reports a non-zero value for the device attribute
   ::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS.

 For managed allocations, \p length must be either zero or the entire
 allocation's size. Both indicate that the entire allocation's stream
 association is being changed. Currently, it is not possible to change stream
 association for a portion of a managed allocation.

 For pageable host allocations, \p length must be non-zero.

 The stream association is specified using \p flags which must be
 one of ::CUmemAttach_flags.
 If the ::CU_MEM_ATTACH_GLOBAL flag is specified, the memory can be accessed
 by any stream on any device.
 If the ::CU_MEM_ATTACH_HOST flag is specified, the program makes a guarantee
 that it won't access the memory on the device from any stream on a device that
 has a zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS.
 If the ::CU_MEM_ATTACH_SINGLE flag is specified and \p hStream is associated with
 a device that has a zero value for the device attribute ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS,
 the program makes a guarantee that it will only access the memory on the device
 from \p hStream. It is illegal to attach singly to the NULL stream, because the
 NULL stream is a virtual global stream and not a specific stream. An error will
 be returned in this case.

 When memory is associated with a single stream, the Unified Memory system will
 allow CPU access to this memory region so long as all operations in \p hStream
 have completed, regardless of whether other streams are active. In effect,
 this constrains exclusive ownership of the managed memory region by
 an active GPU to per-stream activity instead of whole-GPU activity.

 Accessing memory on the device from streams that are not associated with
 it will produce undefined results. No error checking is performed by the
 Unified Memory system to ensure that kernels launched into other streams
 do not access this region.

 It is a program's responsibility to order calls to ::cuStreamAttachMemAsync
 via events, synchronization or other means to ensure legal access to memory
 at all times. Data visibility and coherency will be changed appropriately
 for all kernels which follow a stream-association change.

 If \p hStream is destroyed while data is associated with it, the association is
 removed and the association reverts to the default visibility of the allocation
 as specified at ::cuMemAllocManaged. For __managed__ variables, the default
 association is always ::CU_MEM_ATTACH_GLOBAL. Note that destroying a stream is an
 asynchronous operation, and as a result, the change to default association won't
 happen until all work in the stream has completed.

 \param hStream - Stream in which to enqueue the attach operation
 \param dptr    - Pointer to memory (must be a pointer to managed memory or
                  to a valid host-accessible region of system-allocated
                  pageable memory)
 \param length  - Length of memory
 \param flags   - Must be one of ::CUmemAttach_flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \note_null_stream
 \notefnerr

 \sa ::cuStreamCreate,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamWaitEvent,
 ::cuStreamDestroy,
 ::cuMemAllocManaged,
 ::cudaStreamAttachMemAsync*/
    fn cuStreamAttachMemAsync_ptsz(
        hStream: cuda_types::cuda::CUstream,
        dptr: cuda_types::cuda::CUdeviceptr,
        length: usize,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Determine status of a compute stream

 Returns ::CUDA_SUCCESS if all operations in the stream specified by
 \p hStream have completed, or ::CUDA_ERROR_NOT_READY if not.

 For the purposes of Unified Memory, a return value of ::CUDA_SUCCESS
 is equivalent to having called ::cuStreamSynchronize().

 \param hStream - Stream to query status of

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_READY
 \note_null_stream
 \notefnerr

 \sa ::cuStreamCreate,
 ::cuStreamWaitEvent,
 ::cuStreamDestroy,
 ::cuStreamSynchronize,
 ::cuStreamAddCallback,
 ::cudaStreamQuery*/
    fn cuStreamQuery_ptsz(
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Wait until a stream's tasks are completed

 Waits until the device has completed all operations in the stream specified
 by \p hStream. If the context was created with the
 ::CU_CTX_SCHED_BLOCKING_SYNC flag, the CPU thread will block until the
 stream is finished with all of its tasks.

 \param hStream - Stream to wait for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE

 \note_null_stream
 \notefnerr

 \sa ::cuStreamCreate,
 ::cuStreamDestroy,
 ::cuStreamWaitEvent,
 ::cuStreamQuery,
 ::cuStreamAddCallback,
 ::cudaStreamSynchronize*/
    fn cuStreamSynchronize_ptsz(
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys a stream

 Destroys the stream specified by \p hStream.

 In case the device is still doing work in the stream \p hStream
 when ::cuStreamDestroy() is called, the function will return immediately
 and the resources associated with \p hStream will be released automatically
 once the device has completed all work in \p hStream.

 \param hStream - Stream to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuStreamCreate,
 ::cuStreamWaitEvent,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamAddCallback,
 ::cudaStreamDestroy*/
    fn cuStreamDestroy_v2(
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies attributes from source stream to destination stream.

 Copies attributes from source stream \p src to destination stream \p dst.
 Both streams must have the same context.

 \param[out] dst Destination stream
 \param[in] src Source stream
 For list of attributes see ::CUstreamAttrID

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::CUaccessPolicyWindow*/
    fn cuStreamCopyAttributes_ptsz(
        dst: cuda_types::cuda::CUstream,
        src: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Queries stream attribute.

 Queries attribute \p attr from \p hStream and stores it in corresponding
 member of \p value_out.

 \param[in] hStream
 \param[in] attr
 \param[out] value_out

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa
 ::CUaccessPolicyWindow*/
    fn cuStreamGetAttribute_ptsz(
        hStream: cuda_types::cuda::CUstream,
        attr: cuda_types::cuda::CUstreamAttrID,
        value_out: *mut cuda_types::cuda::CUstreamAttrValue,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets stream attribute.

 Sets attribute \p attr on \p hStream from corresponding attribute of
 \p value. The updated attribute will be applied to subsequent work
 submitted to the stream. It will not affect previously submitted work.

 \param[out] hStream
 \param[in] attr
 \param[in] value

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa
 ::CUaccessPolicyWindow*/
    fn cuStreamSetAttribute_ptsz(
        hStream: cuda_types::cuda::CUstream,
        attr: cuda_types::cuda::CUstreamAttrID,
        value: *const cuda_types::cuda::CUstreamAttrValue,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an event

 Creates an event *phEvent for the current context with the flags specified via
 \p Flags. Valid flags include:
 - ::CU_EVENT_DEFAULT: Default event creation flag.
 - ::CU_EVENT_BLOCKING_SYNC: Specifies that the created event should use blocking
   synchronization.  A CPU thread that uses ::cuEventSynchronize() to wait on
   an event created with this flag will block until the event has actually
   been recorded.
 - ::CU_EVENT_DISABLE_TIMING: Specifies that the created event does not need
   to record timing data.  Events created with this flag specified and
   the ::CU_EVENT_BLOCKING_SYNC flag not specified will provide the best
   performance when used with ::cuStreamWaitEvent() and ::cuEventQuery().
 - ::CU_EVENT_INTERPROCESS: Specifies that the created event may be used as an
   interprocess event by ::cuIpcGetEventHandle(). ::CU_EVENT_INTERPROCESS must
   be specified along with ::CU_EVENT_DISABLE_TIMING.

 \param phEvent - Returns newly created event
 \param Flags   - Event creation flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa
 ::cuEventRecord,
 ::cuEventQuery,
 ::cuEventSynchronize,
 ::cuEventDestroy,
 ::cuEventElapsedTime,
 ::cudaEventCreate,
 ::cudaEventCreateWithFlags*/
    fn cuEventCreate(
        phEvent: *mut cuda_types::cuda::CUevent,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Records an event

 Captures in \p hEvent the contents of \p hStream at the time of this call.
 \p hEvent and \p hStream must be from the same context otherwise
 ::CUDA_ERROR_INVALID_HANDLE is returned.
 Calls such as ::cuEventQuery() or ::cuStreamWaitEvent() will then
 examine or wait for completion of the work that was captured. Uses of
 \p hStream after this call do not modify \p hEvent. See note on default
 stream behavior for what is captured in the default case.

 ::cuEventRecord() can be called multiple times on the same event and
 will overwrite the previously captured state. Other APIs such as
 ::cuStreamWaitEvent() use the most recently captured state at the time
 of the API call, and are not affected by later calls to
 ::cuEventRecord(). Before the first call to ::cuEventRecord(), an
 event represents an empty set of work, so for example ::cuEventQuery()
 would return ::CUDA_SUCCESS.

 \param hEvent  - Event to record
 \param hStream - Stream to record event for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE
 \note_null_stream
 \notefnerr

 \sa ::cuEventCreate,
 ::cuEventQuery,
 ::cuEventSynchronize,
 ::cuStreamWaitEvent,
 ::cuEventDestroy,
 ::cuEventElapsedTime,
 ::cudaEventRecord,
 ::cuEventRecordWithFlags*/
    fn cuEventRecord_ptsz(
        hEvent: cuda_types::cuda::CUevent,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Records an event

 Captures in \p hEvent the contents of \p hStream at the time of this call.
 \p hEvent and \p hStream must be from the same context otherwise
 ::CUDA_ERROR_INVALID_HANDLE is returned.
 Calls such as ::cuEventQuery() or ::cuStreamWaitEvent() will then
 examine or wait for completion of the work that was captured. Uses of
 \p hStream after this call do not modify \p hEvent. See note on default
 stream behavior for what is captured in the default case.

 ::cuEventRecordWithFlags() can be called multiple times on the same event and
 will overwrite the previously captured state. Other APIs such as
 ::cuStreamWaitEvent() use the most recently captured state at the time
 of the API call, and are not affected by later calls to
 ::cuEventRecordWithFlags(). Before the first call to ::cuEventRecordWithFlags(), an
 event represents an empty set of work, so for example ::cuEventQuery()
 would return ::CUDA_SUCCESS.

 flags include:
 - ::CU_EVENT_RECORD_DEFAULT: Default event creation flag.
 - ::CU_EVENT_RECORD_EXTERNAL: Event is captured in the graph as an external
   event node when performing stream capture. This flag is invalid outside
   of stream capture.

 \param hEvent  - Event to record
 \param hStream - Stream to record event for
 \param flags   - See ::CUevent_capture_flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE
 \note_null_stream
 \notefnerr

 \sa ::cuEventCreate,
 ::cuEventQuery,
 ::cuEventSynchronize,
 ::cuStreamWaitEvent,
 ::cuEventDestroy,
 ::cuEventElapsedTime,
 ::cuEventRecord,
 ::cudaEventRecord*/
    fn cuEventRecordWithFlags_ptsz(
        hEvent: cuda_types::cuda::CUevent,
        hStream: cuda_types::cuda::CUstream,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Queries an event's status

 Queries the status of all work currently captured by \p hEvent. See
 ::cuEventRecord() for details on what is captured by an event.

 Returns ::CUDA_SUCCESS if all captured work has been completed, or
 ::CUDA_ERROR_NOT_READY if any captured work is incomplete.

 For the purposes of Unified Memory, a return value of ::CUDA_SUCCESS
 is equivalent to having called ::cuEventSynchronize().

 \param hEvent - Event to query

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_READY
 \notefnerr

 \sa ::cuEventCreate,
 ::cuEventRecord,
 ::cuEventSynchronize,
 ::cuEventDestroy,
 ::cuEventElapsedTime,
 ::cudaEventQuery*/
    fn cuEventQuery(hEvent: cuda_types::cuda::CUevent) -> cuda_types::cuda::CUresult;
    /** \brief Waits for an event to complete

 Waits until the completion of all work currently captured in \p hEvent.
 See ::cuEventRecord() for details on what is captured by an event.

 Waiting for an event that was created with the ::CU_EVENT_BLOCKING_SYNC
 flag will cause the calling CPU thread to block until the event has
 been completed by the device.  If the ::CU_EVENT_BLOCKING_SYNC flag has
 not been set, then the CPU thread will busy-wait until the event has
 been completed by the device.

 \param hEvent - Event to wait for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuEventCreate,
 ::cuEventRecord,
 ::cuEventQuery,
 ::cuEventDestroy,
 ::cuEventElapsedTime,
 ::cudaEventSynchronize*/
    fn cuEventSynchronize(
        hEvent: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys an event

 Destroys the event specified by \p hEvent.

 An event may be destroyed before it is complete (i.e., while
 ::cuEventQuery() would return ::CUDA_ERROR_NOT_READY). In this case, the
 call does not block on completion of the event, and any associated
 resources will automatically be released asynchronously at completion.

 \param hEvent - Event to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuEventCreate,
 ::cuEventRecord,
 ::cuEventQuery,
 ::cuEventSynchronize,
 ::cuEventElapsedTime,
 ::cudaEventDestroy*/
    fn cuEventDestroy_v2(
        hEvent: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Computes the elapsed time between two events

 Computes the elapsed time between two events (in milliseconds with a
 resolution of around 0.5 microseconds).

 If either event was last recorded in a non-NULL stream, the resulting time
 may be greater than expected (even if both used the same stream handle). This
 happens because the ::cuEventRecord() operation takes place asynchronously
 and there is no guarantee that the measured latency is actually just between
 the two events. Any number of other different stream operations could execute
 in between the two measured events, thus altering the timing in a significant
 way.

 If ::cuEventRecord() has not been called on either event then
 ::CUDA_ERROR_INVALID_HANDLE is returned. If ::cuEventRecord() has been called
 on both events but one or both of them has not yet been completed (that is,
 ::cuEventQuery() would return ::CUDA_ERROR_NOT_READY on at least one of the
 events), ::CUDA_ERROR_NOT_READY is returned. If either event was created with
 the ::CU_EVENT_DISABLE_TIMING flag, then this function will return
 ::CUDA_ERROR_INVALID_HANDLE.

 Note there is a later version of this API, ::cuEventElapsedTime_v2. It will
 supplant this version in CUDA 13.0, which is retained for minor version compatibility.

 \param pMilliseconds - Time between \p hStart and \p hEnd in ms
 \param hStart        - Starting event
 \param hEnd          - Ending event

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_READY,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuEventCreate,
 ::cuEventRecord,
 ::cuEventQuery,
 ::cuEventSynchronize,
 ::cuEventDestroy,
 ::cudaEventElapsedTime*/
    fn cuEventElapsedTime(
        pMilliseconds: *mut f32,
        hStart: cuda_types::cuda::CUevent,
        hEnd: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Computes the elapsed time between two events

 Computes the elapsed time between two events (in milliseconds with a
 resolution of around 0.5 microseconds). Note this API is not guaranteed
 to return the latest errors for pending work. As such this API is intended to
 serve as an elapsed time calculation only and any polling for completion on the
 events to be compared should be done with ::cuEventQuery instead.

 If either event was last recorded in a non-NULL stream, the resulting time
 may be greater than expected (even if both used the same stream handle). This
 happens because the ::cuEventRecord() operation takes place asynchronously
 and there is no guarantee that the measured latency is actually just between
 the two events. Any number of other different stream operations could execute
 in between the two measured events, thus altering the timing in a significant
 way.

 If ::cuEventRecord() has not been called on either event then
 ::CUDA_ERROR_INVALID_HANDLE is returned. If ::cuEventRecord() has been called
 on both events but one or both of them has not yet been completed (that is,
 ::cuEventQuery() would return ::CUDA_ERROR_NOT_READY on at least one of the
 events), ::CUDA_ERROR_NOT_READY is returned. If either event was created with
 the ::CU_EVENT_DISABLE_TIMING flag, then this function will return
 ::CUDA_ERROR_INVALID_HANDLE.

 \param pMilliseconds - Time between \p hStart and \p hEnd in ms
 \param hStart        - Starting event
 \param hEnd          - Ending event

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_READY,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuEventCreate,
 ::cuEventRecord,
 ::cuEventQuery,
 ::cuEventSynchronize,
 ::cuEventDestroy,
 ::cudaEventElapsedTime*/
    fn cuEventElapsedTime_v2(
        pMilliseconds: *mut f32,
        hStart: cuda_types::cuda::CUevent,
        hEnd: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Imports an external memory object

 Imports an externally allocated memory object and returns
 a handle to that in \p extMem_out.

 The properties of the handle being imported must be described in
 \p memHandleDesc. The ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC structure
 is defined as follows:

 \code
typedef struct CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st {
CUexternalMemoryHandleType type;
union {
int fd;
struct {
void *handle;
const void *name;
} win32;
const void *nvSciBufObject;
} handle;
unsigned long long size;
unsigned int flags;
} CUDA_EXTERNAL_MEMORY_HANDLE_DESC;
 \endcode

 where ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type specifies the type
 of handle being imported. ::CUexternalMemoryHandleType is
 defined as:

 \code
typedef enum CUexternalMemoryHandleType_enum {
CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD          = 1,
CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32       = 2,
CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT   = 3,
CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP         = 4,
CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE     = 5,
CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE     = 6,
CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT = 7,
CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF           = 8,
} CUexternalMemoryHandleType;
 \endcode

 If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type is
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD, then
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::fd must be a valid
 file descriptor referencing a memory object. Ownership of
 the file descriptor is transferred to the CUDA driver when the
 handle is imported successfully. Performing any operations on the
 file descriptor after it is imported results in undefined behavior.

 If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type is
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32, then exactly one
 of ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::handle and
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::name must not be
 NULL. If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::handle
 is not NULL, then it must represent a valid shared NT handle that
 references a memory object. Ownership of this handle is
 not transferred to CUDA after the import operation, so the
 application must release the handle using the appropriate system
 call. If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::name
 is not NULL, then it must point to a NULL-terminated array of
 UTF-16 characters that refers to a memory object.

 If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type is
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT, then
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::handle must
 be non-NULL and
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::name
 must be NULL. The handle specified must be a globally shared KMT
 handle. This handle does not hold a reference to the underlying
 object, and thus will be invalid when all references to the
 memory object are destroyed.

 If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type is
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP, then exactly one
 of ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::handle and
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::name must not be
 NULL. If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::handle
 is not NULL, then it must represent a valid shared NT handle that
 is returned by ID3D12Device::CreateSharedHandle when referring to a
 ID3D12Heap object. This handle holds a reference to the underlying
 object. If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::name
 is not NULL, then it must point to a NULL-terminated array of
 UTF-16 characters that refers to a ID3D12Heap object.

 If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type is
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE, then exactly one
 of ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::handle and
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::name must not be
 NULL. If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::handle
 is not NULL, then it must represent a valid shared NT handle that
 is returned by ID3D12Device::CreateSharedHandle when referring to a
 ID3D12Resource object. This handle holds a reference to the
 underlying object. If
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::name
 is not NULL, then it must point to a NULL-terminated array of
 UTF-16 characters that refers to a ID3D12Resource object.

 If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type is
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE, then
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::handle must
 represent a valid shared NT handle that is returned by
 IDXGIResource1::CreateSharedHandle when referring to a
 ID3D11Resource object. If
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::name
 is not NULL, then it must point to a NULL-terminated array of
 UTF-16 characters that refers to a ID3D11Resource object.

 If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type is
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT, then
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::handle must
 represent a valid shared KMT handle that is returned by
 IDXGIResource::GetSharedHandle when referring to a
 ID3D11Resource object and
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::win32::name
 must be NULL.

 If ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type is
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF, then
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::handle::nvSciBufObject must be non-NULL
 and reference a valid NvSciBuf object.
 If the NvSciBuf object imported into CUDA is also mapped by other drivers, then the
 application must use ::cuWaitExternalSemaphoresAsync or ::cuSignalExternalSemaphoresAsync
 as appropriate barriers to maintain coherence between CUDA and the other drivers.
 See ::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_SKIP_NVSCIBUF_MEMSYNC and ::CUDA_EXTERNAL_SEMAPHORE_WAIT_SKIP_NVSCIBUF_MEMSYNC
 for memory synchronization.


 The size of the memory object must be specified in
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::size.

 Specifying the flag ::CUDA_EXTERNAL_MEMORY_DEDICATED in
 ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::flags indicates that the
 resource is a dedicated resource. The definition of what a
 dedicated resource is outside the scope of this extension.
 This flag must be set if ::CUDA_EXTERNAL_MEMORY_HANDLE_DESC::type
 is one of the following:
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT

 \param extMem_out    - Returned handle to an external memory object
 \param memHandleDesc - Memory import handle descriptor

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OPERATING_SYSTEM
 \notefnerr

 \note If the Vulkan memory imported into CUDA is mapped on the CPU then the
 application must use vkInvalidateMappedMemoryRanges/vkFlushMappedMemoryRanges
 as well as appropriate Vulkan pipeline barriers to maintain coherence between
 CPU and GPU. For more information on these APIs, please refer to "Synchronization
 and Cache Control" chapter from Vulkan specification.

 \sa ::cuDestroyExternalMemory,
 ::cuExternalMemoryGetMappedBuffer,
 ::cuExternalMemoryGetMappedMipmappedArray*/
    fn cuImportExternalMemory(
        extMem_out: *mut cuda_types::cuda::CUexternalMemory,
        memHandleDesc: *const cuda_types::cuda::CUDA_EXTERNAL_MEMORY_HANDLE_DESC,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Maps a buffer onto an imported memory object

 Maps a buffer onto an imported memory object and returns a device
 pointer in \p devPtr.

 The properties of the buffer being mapped must be described in
 \p bufferDesc. The ::CUDA_EXTERNAL_MEMORY_BUFFER_DESC structure is
 defined as follows:

 \code
typedef struct CUDA_EXTERNAL_MEMORY_BUFFER_DESC_st {
unsigned long long offset;
unsigned long long size;
unsigned int flags;
} CUDA_EXTERNAL_MEMORY_BUFFER_DESC;
 \endcode

 where ::CUDA_EXTERNAL_MEMORY_BUFFER_DESC::offset is the offset in
 the memory object where the buffer's base address is.
 ::CUDA_EXTERNAL_MEMORY_BUFFER_DESC::size is the size of the buffer.
 ::CUDA_EXTERNAL_MEMORY_BUFFER_DESC::flags must be zero.

 The offset and size have to be suitably aligned to match the
 requirements of the external API. Mapping two buffers whose ranges
 overlap may or may not result in the same virtual address being
 returned for the overlapped portion. In such cases, the application
 must ensure that all accesses to that region from the GPU are
 volatile. Otherwise writes made via one address are not guaranteed
 to be visible via the other address, even if they're issued by the
 same thread. It is recommended that applications map the combined
 range instead of mapping separate buffers and then apply the
 appropriate offsets to the returned pointer to derive the
 individual buffers.

 The returned pointer \p devPtr must be freed using ::cuMemFree.

 \param devPtr     - Returned device pointer to buffer
 \param extMem     - Handle to external memory object
 \param bufferDesc - Buffer descriptor

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuImportExternalMemory,
 ::cuDestroyExternalMemory,
 ::cuExternalMemoryGetMappedMipmappedArray*/
    fn cuExternalMemoryGetMappedBuffer(
        devPtr: *mut cuda_types::cuda::CUdeviceptr,
        extMem: cuda_types::cuda::CUexternalMemory,
        bufferDesc: *const cuda_types::cuda::CUDA_EXTERNAL_MEMORY_BUFFER_DESC,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Maps a CUDA mipmapped array onto an external memory object

 Maps a CUDA mipmapped array onto an external object and returns a
 handle to it in \p mipmap.

 The properties of the CUDA mipmapped array being mapped must be
 described in \p mipmapDesc. The structure
 ::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC is defined as follows:

 \code
typedef struct CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_st {
unsigned long long offset;
CUDA_ARRAY3D_DESCRIPTOR arrayDesc;
unsigned int numLevels;
} CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC;
 \endcode

 where ::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC::offset is the
 offset in the memory object where the base level of the mipmap
 chain is.
 ::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC::arrayDesc describes
 the format, dimensions and type of the base level of the mipmap
 chain. For further details on these parameters, please refer to the
 documentation for ::cuMipmappedArrayCreate. Note that if the mipmapped
 array is bound as a color target in the graphics API, then the flag
 ::CUDA_ARRAY3D_COLOR_ATTACHMENT must be specified in
 ::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC::arrayDesc::Flags.
 ::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC::numLevels specifies
 the total number of levels in the mipmap chain.

 If \p extMem was imported from a handle of type ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF, then
 ::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC::numLevels must be equal to 1.


 The returned CUDA mipmapped array must be freed using ::cuMipmappedArrayDestroy.

 \param mipmap     - Returned CUDA mipmapped array
 \param extMem     - Handle to external memory object
 \param mipmapDesc - CUDA array descriptor

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuImportExternalMemory,
 ::cuDestroyExternalMemory,
 ::cuExternalMemoryGetMappedBuffer*/
    fn cuExternalMemoryGetMappedMipmappedArray(
        mipmap: *mut cuda_types::cuda::CUmipmappedArray,
        extMem: cuda_types::cuda::CUexternalMemory,
        mipmapDesc: *const cuda_types::cuda::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys an external memory object.

 Destroys the specified external memory object. Any existing buffers
 and CUDA mipmapped arrays mapped onto this object must no longer be
 used and must be explicitly freed using ::cuMemFree and
 ::cuMipmappedArrayDestroy respectively.

 \param extMem - External memory object to be destroyed

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuImportExternalMemory,
 ::cuExternalMemoryGetMappedBuffer,
 ::cuExternalMemoryGetMappedMipmappedArray*/
    fn cuDestroyExternalMemory(
        extMem: cuda_types::cuda::CUexternalMemory,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Imports an external semaphore

 Imports an externally allocated synchronization object and returns
 a handle to that in \p extSem_out.

 The properties of the handle being imported must be described in
 \p semHandleDesc. The ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC is
 defined as follows:

 \code
typedef struct CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st {
CUexternalSemaphoreHandleType type;
union {
int fd;
struct {
void *handle;
const void *name;
} win32;
const void* NvSciSyncObj;
} handle;
unsigned int flags;
} CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC;
 \endcode

 where ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type specifies the type of
 handle being imported. ::CUexternalSemaphoreHandleType is defined
 as:

 \code
typedef enum CUexternalSemaphoreHandleType_enum {
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD                = 1,
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32             = 2,
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT         = 3,
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE              = 4,
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE              = 5,
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC                = 6,
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX        = 7,
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT    = 8,
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD    = 9,
CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32 = 10
} CUexternalSemaphoreHandleType;
 \endcode

 If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type is
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD, then
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::fd must be a valid
 file descriptor referencing a synchronization object. Ownership of
 the file descriptor is transferred to the CUDA driver when the
 handle is imported successfully. Performing any operations on the
 file descriptor after it is imported results in undefined behavior.

 If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type is
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32, then exactly one
 of ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle and
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name must not be
 NULL. If
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle
 is not NULL, then it must represent a valid shared NT handle that
 references a synchronization object. Ownership of this handle is
 not transferred to CUDA after the import operation, so the
 application must release the handle using the appropriate system
 call. If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name
 is not NULL, then it must name a valid synchronization object.

 If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type is
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT, then
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle must
 be non-NULL and
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name
 must be NULL. The handle specified must be a globally shared KMT
 handle. This handle does not hold a reference to the underlying
 object, and thus will be invalid when all references to the
 synchronization object are destroyed.

 If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type is
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE, then exactly one
 of ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle and
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name must not be
 NULL. If
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle
 is not NULL, then it must represent a valid shared NT handle that
 is returned by ID3D12Device::CreateSharedHandle when referring to a
 ID3D12Fence object. This handle holds a reference to the underlying
 object. If
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name
 is not NULL, then it must name a valid synchronization object that
 refers to a valid ID3D12Fence object.

 If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type is
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE, then
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle
 represents a valid shared NT handle that is returned by
 ID3D11Fence::CreateSharedHandle. If
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name
 is not NULL, then it must name a valid synchronization object that
 refers to a valid ID3D11Fence object.

 If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type is
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC, then
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::nvSciSyncObj
 represents a valid NvSciSyncObj.

 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX, then
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle
 represents a valid shared NT handle that
 is returned by IDXGIResource1::CreateSharedHandle when referring to
 a IDXGIKeyedMutex object. If
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name
 is not NULL, then it must name a valid synchronization object that
 refers to a valid IDXGIKeyedMutex object.

 If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type is
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT, then
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle
 represents a valid shared KMT handle that
 is returned by IDXGIResource::GetSharedHandle when referring to
 a IDXGIKeyedMutex object and
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name must be NULL.

 If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type is
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD, then
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::fd must be a valid
 file descriptor referencing a synchronization object. Ownership of
 the file descriptor is transferred to the CUDA driver when the
 handle is imported successfully. Performing any operations on the
 file descriptor after it is imported results in undefined behavior.

 If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::type is
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32, then exactly one
 of ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle and
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name must not be
 NULL. If
 ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::handle
 is not NULL, then it must represent a valid shared NT handle that
 references a synchronization object. Ownership of this handle is
 not transferred to CUDA after the import operation, so the
 application must release the handle using the appropriate system
 call. If ::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC::handle::win32::name
 is not NULL, then it must name a valid synchronization object.

 \param extSem_out    - Returned handle to an external semaphore
 \param semHandleDesc - Semaphore import handle descriptor

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OPERATING_SYSTEM
 \notefnerr

 \sa ::cuDestroyExternalSemaphore,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync*/
    fn cuImportExternalSemaphore(
        extSem_out: *mut cuda_types::cuda::CUexternalSemaphore,
        semHandleDesc: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Signals a set of external semaphore objects

 Enqueues a signal operation on a set of externally allocated
 semaphore object in the specified stream. The operations will be
 executed when all prior operations in the stream complete.

 The exact semantics of signaling a semaphore depends on the type of
 the object.

 If the semaphore object is any one of the following types:
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT
 then signaling the semaphore will set it to the signaled state.

 If the semaphore object is any one of the following types:
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32
 then the semaphore will be set to the value specified in
 ::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS::params::fence::value.

 If the semaphore object is of the type ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC
 this API sets ::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS::params::nvSciSync::fence
 to a value that can be used by subsequent waiters of the same NvSciSync object
 to order operations with those currently submitted in \p stream. Such an update
 will overwrite previous contents of
 ::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS::params::nvSciSync::fence. By default,
 signaling such an external semaphore object causes appropriate memory synchronization
 operations to be performed over all external memory objects that are imported as
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF. This ensures that any subsequent accesses
 made by other importers of the same set of NvSciBuf memory object(s) are coherent.
 These operations can be skipped by specifying the flag
 ::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_SKIP_NVSCIBUF_MEMSYNC, which can be used as a
 performance optimization when data coherency is not required. But specifying this
 flag in scenarios where data coherency is required results in undefined behavior.
 Also, for semaphore object of the type ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC,
 if the NvSciSyncAttrList used to create the NvSciSyncObj had not set the flags in
 ::cuDeviceGetNvSciSyncAttributes to CUDA_NVSCISYNC_ATTR_SIGNAL, this API will return
 CUDA_ERROR_NOT_SUPPORTED.
 NvSciSyncFence associated with semaphore object of the type
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC can be deterministic. For this the
 NvSciSyncAttrList used to create the semaphore object must have value of
 NvSciSyncAttrKey_RequireDeterministicFences key set to true. Deterministic fences
 allow users to enqueue a wait over the semaphore object even before corresponding
 signal is enqueued. For such a semaphore object, CUDA guarantees that each signal
 operation will increment the fence value by '1'. Users are expected to track count
 of signals enqueued on the semaphore object and insert waits accordingly. When such
 a semaphore object is signaled from multiple streams, due to concurrent stream
 execution, it is possible that the order in which the semaphore gets signaled is
 indeterministic. This could lead to waiters of the semaphore getting unblocked
 incorrectly. Users are expected to handle such situations, either by not using the
 same semaphore object with deterministic fence support enabled in different streams
 or by adding explicit dependency amongst such streams so that the semaphore is
 signaled in order.

 If the semaphore object is any one of the following types:
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT
 then the keyed mutex will be released with the key specified in
 ::CUDA_EXTERNAL_SEMAPHORE_PARAMS::params::keyedmutex::key.

 \param extSemArray - Set of external semaphores to be signaled
 \param paramsArray - Array of semaphore parameters
 \param numExtSems  - Number of semaphores to signal
 \param stream      - Stream to enqueue the signal operations in

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuImportExternalSemaphore,
 ::cuDestroyExternalSemaphore,
 ::cuWaitExternalSemaphoresAsync*/
    fn cuSignalExternalSemaphoresAsync_ptsz(
        extSemArray: *const cuda_types::cuda::CUexternalSemaphore,
        paramsArray: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
        numExtSems: ::core::ffi::c_uint,
        stream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Waits on a set of external semaphore objects

 Enqueues a wait operation on a set of externally allocated
 semaphore object in the specified stream. The operations will be
 executed when all prior operations in the stream complete.

 The exact semantics of waiting on a semaphore depends on the type
 of the object.

 If the semaphore object is any one of the following types:
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT
 then waiting on the semaphore will wait until the semaphore reaches
 the signaled state. The semaphore will then be reset to the
 unsignaled state. Therefore for every signal operation, there can
 only be one wait operation.

 If the semaphore object is any one of the following types:
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32
 then waiting on the semaphore will wait until the value of the
 semaphore is greater than or equal to
 ::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS::params::fence::value.

 If the semaphore object is of the type ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC
 then, waiting on the semaphore will wait until the
 ::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS::params::nvSciSync::fence is signaled by the
 signaler of the NvSciSyncObj that was associated with this semaphore object.
 By default, waiting on such an external semaphore object causes appropriate
 memory synchronization operations to be performed over all external memory objects
 that are imported as ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF. This ensures that
 any subsequent accesses made by other importers of the same set of NvSciBuf memory
 object(s) are coherent. These operations can be skipped by specifying the flag
 ::CUDA_EXTERNAL_SEMAPHORE_WAIT_SKIP_NVSCIBUF_MEMSYNC, which can be used as a
 performance optimization when data coherency is not required. But specifying this
 flag in scenarios where data coherency is required results in undefined behavior.
 Also, for semaphore object of the type ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC,
 if the NvSciSyncAttrList used to create the NvSciSyncObj had not set the flags in
 ::cuDeviceGetNvSciSyncAttributes to CUDA_NVSCISYNC_ATTR_WAIT, this API will return
 CUDA_ERROR_NOT_SUPPORTED.

 If the semaphore object is any one of the following types:
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX,
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT
 then the keyed mutex will be acquired when it is released with the key
 specified in ::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS::params::keyedmutex::key
 or until the timeout specified by
 ::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS::params::keyedmutex::timeoutMs
 has lapsed. The timeout interval can either be a finite value
 specified in milliseconds or an infinite value. In case an infinite
 value is specified the timeout never elapses. The windows INFINITE
 macro must be used to specify infinite timeout.

 \param extSemArray - External semaphores to be waited on
 \param paramsArray - Array of semaphore parameters
 \param numExtSems  - Number of semaphores to wait on
 \param stream      - Stream to enqueue the wait operations in

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_TIMEOUT
 \notefnerr

 \sa ::cuImportExternalSemaphore,
 ::cuDestroyExternalSemaphore,
 ::cuSignalExternalSemaphoresAsync*/
    fn cuWaitExternalSemaphoresAsync_ptsz(
        extSemArray: *const cuda_types::cuda::CUexternalSemaphore,
        paramsArray: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
        numExtSems: ::core::ffi::c_uint,
        stream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys an external semaphore

 Destroys an external semaphore object and releases any references
 to the underlying resource. Any outstanding signals or waits must
 have completed before the semaphore is destroyed.

 \param extSem - External semaphore to be destroyed

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa ::cuImportExternalSemaphore,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync*/
    fn cuDestroyExternalSemaphore(
        extSem: cuda_types::cuda::CUexternalSemaphore,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Wait on a memory location

 Enqueues a synchronization of the stream on the given memory location. Work
 ordered after the operation will block until the given condition on the
 memory is satisfied. By default, the condition is to wait for
 (int32_t)(*addr - value) >= 0, a cyclic greater-or-equal.
 Other condition types can be specified via \p flags.

 If the memory was registered via ::cuMemHostRegister(), the device pointer
 should be obtained with ::cuMemHostGetDevicePointer(). This function cannot
 be used with managed memory (::cuMemAllocManaged).

 Support for CU_STREAM_WAIT_VALUE_NOR can be queried with ::cuDeviceGetAttribute() and
 ::CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR_V2.

 \note
 Warning:
 Improper use of this API may deadlock the application. Synchronization
 ordering established through this API is not visible to CUDA. CUDA tasks
 that are (even indirectly) ordered by this API should also have that order
 expressed with CUDA-visible dependencies such as events. This ensures that
 the scheduler does not serialize them in an improper order.

 \param stream The stream to synchronize on the memory location.
 \param addr The memory location to wait on.
 \param value The value to compare with the memory location.
 \param flags See ::CUstreamWaitValue_flags.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuStreamWaitValue64,
 ::cuStreamWriteValue32,
 ::cuStreamWriteValue64,
 ::cuStreamBatchMemOp,
 ::cuMemHostRegister,
 ::cuStreamWaitEvent*/
    fn cuStreamWaitValue32_v2_ptsz(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint32_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Wait on a memory location

 Enqueues a synchronization of the stream on the given memory location. Work
 ordered after the operation will block until the given condition on the
 memory is satisfied. By default, the condition is to wait for
 (int64_t)(*addr - value) >= 0, a cyclic greater-or-equal.
 Other condition types can be specified via \p flags.

 If the memory was registered via ::cuMemHostRegister(), the device pointer
 should be obtained with ::cuMemHostGetDevicePointer().

 Support for this can be queried with ::cuDeviceGetAttribute() and
 ::CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS.

 \note
 Warning:
 Improper use of this API may deadlock the application. Synchronization
 ordering established through this API is not visible to CUDA. CUDA tasks
 that are (even indirectly) ordered by this API should also have that order
 expressed with CUDA-visible dependencies such as events. This ensures that
 the scheduler does not serialize them in an improper order.

 \param stream The stream to synchronize on the memory location.
 \param addr The memory location to wait on.
 \param value The value to compare with the memory location.
 \param flags See ::CUstreamWaitValue_flags.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuStreamWaitValue32,
 ::cuStreamWriteValue32,
 ::cuStreamWriteValue64,
 ::cuStreamBatchMemOp,
 ::cuMemHostRegister,
 ::cuStreamWaitEvent*/
    fn cuStreamWaitValue64_v2_ptsz(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint64_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Write a value to memory

 Write a value to memory.

 If the memory was registered via ::cuMemHostRegister(), the device pointer
 should be obtained with ::cuMemHostGetDevicePointer(). This function cannot
 be used with managed memory (::cuMemAllocManaged).

 \param stream The stream to do the write in.
 \param addr The device address to write to.
 \param value The value to write.
 \param flags See ::CUstreamWriteValue_flags.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuStreamWriteValue64,
 ::cuStreamWaitValue32,
 ::cuStreamWaitValue64,
 ::cuStreamBatchMemOp,
 ::cuMemHostRegister,
 ::cuEventRecord*/
    fn cuStreamWriteValue32_v2_ptsz(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint32_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Write a value to memory

 Write a value to memory.

 If the memory was registered via ::cuMemHostRegister(), the device pointer
 should be obtained with ::cuMemHostGetDevicePointer().

 Support for this can be queried with ::cuDeviceGetAttribute() and
 ::CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS.

 \param stream The stream to do the write in.
 \param addr The device address to write to.
 \param value The value to write.
 \param flags See ::CUstreamWriteValue_flags.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuStreamWriteValue32,
 ::cuStreamWaitValue32,
 ::cuStreamWaitValue64,
 ::cuStreamBatchMemOp,
 ::cuMemHostRegister,
 ::cuEventRecord*/
    fn cuStreamWriteValue64_v2_ptsz(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint64_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Batch operations to synchronize the stream via memory operations

 This is a batch version of ::cuStreamWaitValue32() and ::cuStreamWriteValue32().
 Batching operations may avoid some performance overhead in both the API call
 and the device execution versus adding them to the stream in separate API
 calls. The operations are enqueued in the order they appear in the array.

 See ::CUstreamBatchMemOpType for the full set of supported operations, and
 ::cuStreamWaitValue32(), ::cuStreamWaitValue64(), ::cuStreamWriteValue32(),
 and ::cuStreamWriteValue64() for details of specific operations.

 See related APIs for details on querying support for specific operations.

 \note
 Warning:
 Improper use of this API may deadlock the application. Synchronization
 ordering established through this API is not visible to CUDA. CUDA tasks
 that are (even indirectly) ordered by this API should also have that order
 expressed with CUDA-visible dependencies such as events. This ensures that
 the scheduler does not serialize them in an improper order. For more
 information, see the Stream Memory Operations section in the programming
 guide(https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html).

 \param stream The stream to enqueue the operations in.
 \param count The number of operations in the array. Must be less than 256.
 \param paramArray The types and parameters of the individual operations.
 \param flags Reserved for future expansion; must be 0.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa ::cuStreamWaitValue32,
 ::cuStreamWaitValue64,
 ::cuStreamWriteValue32,
 ::cuStreamWriteValue64,
 ::cuMemHostRegister*/
    fn cuStreamBatchMemOp_v2_ptsz(
        stream: cuda_types::cuda::CUstream,
        count: ::core::ffi::c_uint,
        paramArray: *mut cuda_types::cuda::CUstreamBatchMemOpParams,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns information about a function

 Returns in \p *pi the integer value of the attribute \p attrib on the kernel
 given by \p hfunc. The supported attributes are:
 - ::CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK: The maximum number of threads
   per block, beyond which a launch of the function would fail. This number
   depends on both the function and the device on which the function is
   currently loaded.
 - ::CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES: The size in bytes of
   statically-allocated shared memory per block required by this function.
   This does not include dynamically-allocated shared memory requested by
   the user at runtime.
 - ::CU_FUNC_ATTRIBUTE_CONST_SIZE_BYTES: The size in bytes of user-allocated
   constant memory required by this function.
 - ::CU_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES: The size in bytes of local memory
   used by each thread of this function.
 - ::CU_FUNC_ATTRIBUTE_NUM_REGS: The number of registers used by each thread
   of this function.
 - ::CU_FUNC_ATTRIBUTE_PTX_VERSION: The PTX virtual architecture version for
   which the function was compiled. This value is the major PTX version * 10
   + the minor PTX version, so a PTX version 1.3 function would return the
   value 13. Note that this may return the undefined value of 0 for cubins
   compiled prior to CUDA 3.0.
 - ::CU_FUNC_ATTRIBUTE_BINARY_VERSION: The binary architecture version for
   which the function was compiled. This value is the major binary
   version * 10 + the minor binary version, so a binary version 1.3 function
   would return the value 13. Note that this will return a value of 10 for
   legacy cubins that do not have a properly-encoded binary architecture
   version.
 - ::CU_FUNC_CACHE_MODE_CA: The attribute to indicate whether the function has
   been compiled with user specified option "-Xptxas --dlcm=ca" set .
 - ::CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES: The maximum size in bytes of
   dynamically-allocated shared memory.
 - ::CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT: Preferred shared memory-L1
   cache split ratio in percent of total shared memory.
 - ::CU_FUNC_ATTRIBUTE_CLUSTER_SIZE_MUST_BE_SET: If this attribute is set, the
   kernel must launch with a valid cluster size specified.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_WIDTH: The required cluster width in
   blocks.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_HEIGHT: The required cluster height in
   blocks.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_DEPTH: The required cluster depth in
   blocks.
 - ::CU_FUNC_ATTRIBUTE_NON_PORTABLE_CLUSTER_SIZE_ALLOWED: Indicates whether
   the function can be launched with non-portable cluster size. 1 is allowed,
   0 is disallowed. A non-portable cluster size may only function on the
   specific SKUs the program is tested on. The launch might fail if the
   program is run on a different hardware platform. CUDA API provides
   cudaOccupancyMaxActiveClusters to assist with checking whether the desired
   size can be launched on the current device. A portable cluster size is
   guaranteed to be functional on all compute capabilities higher than the
   target compute capability. The portable cluster size for sm_90 is 8 blocks
   per cluster. This value may increase for future compute capabilities. The
   specific hardware unit may support higher cluster sizes that’s not
   guaranteed to be portable.
 - ::CU_FUNC_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE: The block
   scheduling policy of a function. The value type is CUclusterSchedulingPolicy.

 With a few execeptions, function attributes may also be queried on unloaded
 function handles returned from ::cuModuleEnumerateFunctions.
 ::CUDA_ERROR_FUNCTION_NOT_LOADED is returned if the attribute requires a fully
 loaded function but the function is not loaded. The loading state of a function
 may be queried using ::cuFuncIsloaded. ::cuFuncLoad may be called to explicitly
 load a function before querying the following attributes that require the function
 to be loaded:
 - ::CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK
 - ::CU_FUNC_ATTRIBUTE_CONST_SIZE_BYTES
 - ::CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES

 \param pi     - Returned attribute value
 \param attrib - Attribute requested
 \param hfunc  - Function to query attribute of

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_FUNCTION_NOT_LOADED
 \notefnerr

 \sa ::cuCtxGetCacheConfig,
 ::cuCtxSetCacheConfig,
 ::cuFuncSetCacheConfig,
 ::cuLaunchKernel,
 ::cudaFuncGetAttributes,
 ::cudaFuncSetAttribute,
 ::cuFuncIsLoaded,
 ::cuFuncLoad,
 ::cuKernelGetAttribute*/
    fn cuFuncGetAttribute(
        pi: *mut ::core::ffi::c_int,
        attrib: cuda_types::cuda::CUfunction_attribute,
        hfunc: cuda_types::cuda::CUfunction,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets information about a function

 This call sets the value of a specified attribute \p attrib on the kernel given
 by \p hfunc to an integer value specified by \p val
 This function returns CUDA_SUCCESS if the new value of the attribute could be
 successfully set. If the set fails, this call will return an error.
 Not all attributes can have values set. Attempting to set a value on a read-only
 attribute will result in an error (CUDA_ERROR_INVALID_VALUE)

 Supported attributes for the cuFuncSetAttribute call are:
 - ::CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES: This maximum size in bytes of
   dynamically-allocated shared memory. The value should contain the requested
   maximum size of dynamically-allocated shared memory. The sum of this value and
   the function attribute ::CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES cannot exceed the
   device attribute ::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN.
   The maximal size of requestable dynamic shared memory may differ by GPU
   architecture.
 - ::CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT: On devices where the L1
   cache and shared memory use the same hardware resources, this sets the shared memory
   carveout preference, in percent of the total shared memory.
   See ::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR
   This is only a hint, and the driver can choose a different ratio if required to execute the function.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_WIDTH: The required cluster width in
   blocks. The width, height, and depth values must either all be 0 or all be
   positive. The validity of the cluster dimensions is checked at launch time.
   If the value is set during compile time, it cannot be set at runtime.
   Setting it at runtime will return CUDA_ERROR_NOT_PERMITTED.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_HEIGHT: The required cluster height in
   blocks. The width, height, and depth values must either all be 0 or all be
   positive. The validity of the cluster dimensions is checked at launch time.
   If the value is set during compile time, it cannot be set at runtime.
   Setting it at runtime will return CUDA_ERROR_NOT_PERMITTED.
 - ::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_DEPTH: The required cluster depth in
   blocks. The width, height, and depth values must either all be 0 or all be
   positive. The validity of the cluster dimensions is checked at launch time.
   If the value is set during compile time, it cannot be set at runtime.
   Setting it at runtime will return CUDA_ERROR_NOT_PERMITTED.
 - ::CU_FUNC_ATTRIBUTE_NON_PORTABLE_CLUSTER_SIZE_ALLOWED: Indicates whether
   the function can be launched with non-portable cluster size. 1 is allowed,
   0 is disallowed.
 - ::CU_FUNC_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE: The block
   scheduling policy of a function. The value type is CUclusterSchedulingPolicy.

 \param hfunc  - Function to query attribute of
 \param attrib - Attribute requested
 \param value   - The value to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxGetCacheConfig,
 ::cuCtxSetCacheConfig,
 ::cuFuncSetCacheConfig,
 ::cuLaunchKernel,
 ::cudaFuncGetAttributes,
 ::cudaFuncSetAttribute,
 ::cuKernelSetAttribute*/
    fn cuFuncSetAttribute(
        hfunc: cuda_types::cuda::CUfunction,
        attrib: cuda_types::cuda::CUfunction_attribute,
        value: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the preferred cache configuration for a device function

 On devices where the L1 cache and shared memory use the same hardware
 resources, this sets through \p config the preferred cache configuration for
 the device function \p hfunc. This is only a preference. The driver will use
 the requested configuration if possible, but it is free to choose a different
 configuration if required to execute \p hfunc.  Any context-wide preference
 set via ::cuCtxSetCacheConfig() will be overridden by this per-function
 setting unless the per-function setting is ::CU_FUNC_CACHE_PREFER_NONE. In
 that case, the current context-wide setting will be used.

 This setting does nothing on devices where the size of the L1 cache and
 shared memory are fixed.

 Launching a kernel with a different preference than the most recent
 preference setting may insert a device-side synchronization point.


 The supported cache configurations are:
 - ::CU_FUNC_CACHE_PREFER_NONE: no preference for shared memory or L1 (default)
 - ::CU_FUNC_CACHE_PREFER_SHARED: prefer larger shared memory and smaller L1 cache
 - ::CU_FUNC_CACHE_PREFER_L1: prefer larger L1 cache and smaller shared memory
 - ::CU_FUNC_CACHE_PREFER_EQUAL: prefer equal sized L1 cache and shared memory

 \param hfunc  - Kernel to configure cache for
 \param config - Requested cache configuration

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa ::cuCtxGetCacheConfig,
 ::cuCtxSetCacheConfig,
 ::cuFuncGetAttribute,
 ::cuLaunchKernel,
 ::cudaFuncSetCacheConfig,
 ::cuKernelSetCacheConfig*/
    fn cuFuncSetCacheConfig(
        hfunc: cuda_types::cuda::CUfunction,
        config: cuda_types::cuda::CUfunc_cache,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a module handle

 Returns in \p *hmod the handle of the module that function \p hfunc
 is located in. The lifetime of the module corresponds to the lifetime of
 the context it was loaded in or until the module is explicitly unloaded.

 The CUDA runtime manages its own modules loaded into the primary context.
 If the handle returned by this API refers to a module loaded by the CUDA runtime,
 calling ::cuModuleUnload() on that module will result in undefined behavior.

 \param hmod - Returned module handle
 \param hfunc   - Function to retrieve module for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_FOUND
 \notefnerr
*/
    fn cuFuncGetModule(
        hmod: *mut cuda_types::cuda::CUmodule,
        hfunc: cuda_types::cuda::CUfunction,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the function name for a ::CUfunction handle

 Returns in \p **name the function name associated with the function handle \p hfunc .
 The function name is returned as a null-terminated string. The returned name is only
 valid when the function handle is valid. If the module is unloaded or reloaded, one
 must call the API again to get the updated name. This API may return a mangled name if
 the function is not declared as having C linkage. If either \p **name or \p hfunc
 is NULL, ::CUDA_ERROR_INVALID_VALUE is returned.

 \param name - The returned name of the function
 \param hfunc - The function handle to retrieve the name for

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr
*/
    fn cuFuncGetName(
        name: *mut *const ::core::ffi::c_char,
        hfunc: cuda_types::cuda::CUfunction,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the offset and size of a kernel parameter in the device-side parameter layout

 Queries the kernel parameter at \p paramIndex into \p func's list of parameters, and returns
 in \p paramOffset and \p paramSize the offset and size, respectively, where the parameter
 will reside in the device-side parameter layout. This information can be used to update kernel
 node parameters from the device via ::cudaGraphKernelNodeSetParam() and
 ::cudaGraphKernelNodeUpdatesApply(). \p paramIndex must be less than the number of parameters
 that \p func takes. \p paramSize can be set to NULL if only the parameter offset is desired.

 \param func        - The function to query
 \param paramIndex  - The parameter index to query
 \param paramOffset - Returns the offset into the device-side parameter layout at which the parameter resides
 \param paramSize   - Optionally returns the size of the parameter in the device-side parameter layout

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \notefnerr

 \sa ::cuKernelGetParamInfo*/
    fn cuFuncGetParamInfo(
        func: cuda_types::cuda::CUfunction,
        paramIndex: usize,
        paramOffset: *mut usize,
        paramSize: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns if the function is loaded

 Returns in \p state the loading state of \p function.

 \param state - returned loading state
 \param function - the function to check

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuFuncLoad,
 ::cuModuleEnumerateFunctions*/
    fn cuFuncIsLoaded(
        state: *mut cuda_types::cuda::CUfunctionLoadingState,
        function: cuda_types::cuda::CUfunction,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Loads a function

 Finalizes function loading for \p function. Calling this API with a
 fully loaded function has no effect.

 \param function - the function to load

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuModuleEnumerateFunctions,
 ::cuFuncIsLoaded*/
    fn cuFuncLoad(function: cuda_types::cuda::CUfunction) -> cuda_types::cuda::CUresult;
    /** \brief Launches a CUDA function ::CUfunction or a CUDA kernel ::CUkernel

 Invokes the function ::CUfunction or the kernel ::CUkernel \p f
 on a \p gridDimX x \p gridDimY x \p gridDimZ grid of blocks.
 Each block contains \p blockDimX x \p blockDimY x
 \p blockDimZ threads.

 \p sharedMemBytes sets the amount of dynamic shared memory that will be
 available to each thread block.

 Kernel parameters to \p f can be specified in one of two ways:

 1) Kernel parameters can be specified via \p kernelParams.  If \p f
 has N parameters, then \p kernelParams needs to be an array of N
 pointers.  Each of \p kernelParams[0] through \p kernelParams[N-1]
 must point to a region of memory from which the actual kernel
 parameter will be copied.  The number of kernel parameters and their
 offsets and sizes do not need to be specified as that information is
 retrieved directly from the kernel's image.

 2) Kernel parameters can also be packaged by the application into
 a single buffer that is passed in via the \p extra parameter.
 This places the burden on the application of knowing each kernel
 parameter's size and alignment/padding within the buffer.  Here is
 an example of using the \p extra parameter in this manner:
 \code
size_t argBufferSize;
char argBuffer[256];

// populate argBuffer and argBufferSize

void *config[] = {
CU_LAUNCH_PARAM_BUFFER_POINTER, argBuffer,
CU_LAUNCH_PARAM_BUFFER_SIZE,    &argBufferSize,
CU_LAUNCH_PARAM_END
};
status = cuLaunchKernel(f, gx, gy, gz, bx, by, bz, sh, s, NULL, config);
 \endcode

 The \p extra parameter exists to allow ::cuLaunchKernel to take
 additional less commonly used arguments.  \p extra specifies a list of
 names of extra settings and their corresponding values.  Each extra
 setting name is immediately followed by the corresponding value.  The
 list must be terminated with either NULL or ::CU_LAUNCH_PARAM_END.

 - ::CU_LAUNCH_PARAM_END, which indicates the end of the \p extra
   array;
 - ::CU_LAUNCH_PARAM_BUFFER_POINTER, which specifies that the next
   value in \p extra will be a pointer to a buffer containing all
   the kernel parameters for launching kernel \p f;
 - ::CU_LAUNCH_PARAM_BUFFER_SIZE, which specifies that the next
   value in \p extra will be a pointer to a size_t containing the
   size of the buffer specified with ::CU_LAUNCH_PARAM_BUFFER_POINTER;

 The error ::CUDA_ERROR_INVALID_VALUE will be returned if kernel
 parameters are specified with both \p kernelParams and \p extra
 (i.e. both \p kernelParams and \p extra are non-NULL).

 Calling ::cuLaunchKernel() invalidates the persistent function state
 set through the following deprecated APIs:
  ::cuFuncSetBlockShape(),
  ::cuFuncSetSharedSize(),
  ::cuParamSetSize(),
  ::cuParamSeti(),
  ::cuParamSetf(),
  ::cuParamSetv().

 Note that to use ::cuLaunchKernel(), the kernel \p f must either have
 been compiled with toolchain version 3.2 or later so that it will
 contain kernel parameter information, or have no kernel parameters.
 If either of these conditions is not met, then ::cuLaunchKernel() will
 return ::CUDA_ERROR_INVALID_IMAGE.

 Note that the API can also be used to launch context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to launch
 the kernel on will either be taken from the specified stream \p hStream
 or the current context in case of NULL stream.

 \param f              - Function ::CUfunction or Kernel ::CUkernel to launch
 \param gridDimX       - Width of grid in blocks
 \param gridDimY       - Height of grid in blocks
 \param gridDimZ       - Depth of grid in blocks
 \param blockDimX      - X dimension of each thread block
 \param blockDimY      - Y dimension of each thread block
 \param blockDimZ      - Z dimension of each thread block
 \param sharedMemBytes - Dynamic shared-memory size per thread block in bytes
 \param hStream        - Stream identifier
 \param kernelParams   - Array of pointers to kernel parameters
 \param extra          - Extra options

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_IMAGE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_LAUNCH_FAILED,
 ::CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES,
 ::CUDA_ERROR_LAUNCH_TIMEOUT,
 ::CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
 ::CUDA_ERROR_NOT_FOUND
 \note_null_stream
 \notefnerr

 \sa ::cuCtxGetCacheConfig,
 ::cuCtxSetCacheConfig,
 ::cuFuncSetCacheConfig,
 ::cuFuncGetAttribute,
 ::cudaLaunchKernel,
 ::cuLibraryGetKernel,
 ::cuKernelSetCacheConfig,
 ::cuKernelGetAttribute,
 ::cuKernelSetAttribute*/
    fn cuLaunchKernel_ptsz(
        f: cuda_types::cuda::CUfunction,
        gridDimX: ::core::ffi::c_uint,
        gridDimY: ::core::ffi::c_uint,
        gridDimZ: ::core::ffi::c_uint,
        blockDimX: ::core::ffi::c_uint,
        blockDimY: ::core::ffi::c_uint,
        blockDimZ: ::core::ffi::c_uint,
        sharedMemBytes: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
        kernelParams: *mut *mut ::core::ffi::c_void,
        extra: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Launches a CUDA function ::CUfunction or a CUDA kernel ::CUkernel with launch-time configuration

 Invokes the function ::CUfunction or the kernel ::CUkernel \p f with the specified launch-time configuration
 \p config.

 The ::CUlaunchConfig structure is defined as:

 \code
       typedef struct CUlaunchConfig_st {
     unsigned int gridDimX;
     unsigned int gridDimY;
     unsigned int gridDimZ;
     unsigned int blockDimX;
     unsigned int blockDimY;
     unsigned int blockDimZ;
     unsigned int sharedMemBytes;
     CUstream hStream;
     CUlaunchAttribute *attrs;
     unsigned int numAttrs;
 } CUlaunchConfig;
 \endcode

 where:
 - ::CUlaunchConfig::gridDimX is the width of the grid in blocks.
 - ::CUlaunchConfig::gridDimY is the height of the grid in blocks.
 - ::CUlaunchConfig::gridDimZ is the depth of the grid in blocks.
 - ::CUlaunchConfig::blockDimX is the X dimension of each thread block.
 - ::CUlaunchConfig::blockDimX is the Y dimension of each thread block.
 - ::CUlaunchConfig::blockDimZ is the Z dimension of each thread block.
 - ::CUlaunchConfig::sharedMemBytes is the dynamic shared-memory size per
   thread block in bytes.
 - ::CUlaunchConfig::hStream is the handle to the stream to perform the launch
   in. The CUDA context associated with this stream must match that associated
   with function f.
 - ::CUlaunchConfig::attrs is an array of ::CUlaunchConfig::numAttrs
   continguous ::CUlaunchAttribute elements. The value of this pointer is not
   considered if ::CUlaunchConfig::numAttrs is zero. However, in that case, it
   is recommended to set the pointer to NULL.
 - ::CUlaunchConfig::numAttrs is the number of attributes populating the
   first ::CUlaunchConfig::numAttrs positions of the ::CUlaunchConfig::attrs
   array.

 Launch-time configuration is specified by adding entries to
 ::CUlaunchConfig::attrs. Each entry is an attribute ID and a corresponding
 attribute value.

 The ::CUlaunchAttribute structure is defined as:
 \code
       typedef struct CUlaunchAttribute_st {
     CUlaunchAttributeID id;
     CUlaunchAttributeValue value;
 } CUlaunchAttribute;
 \endcode
 where:
 - ::CUlaunchAttribute::id is a unique enum identifying the attribute.
 - ::CUlaunchAttribute::value is a union that hold the attribute value.

 An example of using the \p config parameter:
 \code
       CUlaunchAttribute coopAttr = {.id = CU_LAUNCH_ATTRIBUTE_COOPERATIVE,
                               .value = 1};
 CUlaunchConfig config = {... // set block and grid dimensions
                        .attrs = &coopAttr,
                        .numAttrs = 1};

 cuLaunchKernelEx(&config, kernel, NULL, NULL);
 \endcode

 The ::CUlaunchAttributeID enum is defined as:
 \code
       typedef enum CUlaunchAttributeID_enum {
     CU_LAUNCH_ATTRIBUTE_IGNORE = 0,
     CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW   = 1,
     CU_LAUNCH_ATTRIBUTE_COOPERATIVE            = 2,
     CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY = 3,
     CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION                    = 4,
     CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE = 5,
     CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION    = 6,
     CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT                   = 7,
     CU_LAUNCH_ATTRIBUTE_PRIORITY               = 8,
     CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP    = 9,
     CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN        = 10,
     CU_LAUNCH_ATTRIBUTE_PREFERRED_CLUSTER_DIMENSION = 11,
     CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT = 12,
     CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE = 13,
 } CUlaunchAttributeID;
 \endcode

 and the corresponding ::CUlaunchAttributeValue union as :
 \code
       typedef union CUlaunchAttributeValue_union {
     CUaccessPolicyWindow accessPolicyWindow;
     int cooperative;
     CUsynchronizationPolicy syncPolicy;
     struct {
         unsigned int x;
         unsigned int y;
         unsigned int z;
     } clusterDim;
     CUclusterSchedulingPolicy clusterSchedulingPolicyPreference;
     int programmaticStreamSerializationAllowed;
     struct {
         CUevent event;
         int flags;
         int triggerAtBlockStart;
     } programmaticEvent;
     int priority;
     CUlaunchMemSyncDomainMap memSyncDomainMap;
     CUlaunchMemSyncDomain memSyncDomain;
     struct {
         unsigned int x;
         unsigned int y;
         unsigned int z;
     } preferredClusterDim;
     struct {
         CUevent event;
         int flags;
     } launchCompletionEvent;
     struct {
         int deviceUpdatable;
         CUgraphDeviceNode devNode;
     } deviceUpdatableKernelNode;
 } CUlaunchAttributeValue;
 \endcode

 Setting ::CU_LAUNCH_ATTRIBUTE_COOPERATIVE to a non-zero value causes the
 kernel launch to be a cooperative launch, with exactly the same usage and
 semantics of ::cuLaunchCooperativeKernel.

 Setting ::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION to a non-zero
 values causes the kernel to use programmatic means to resolve its stream
 dependency -- enabling the CUDA runtime to opportunistically allow the grid's
 execution to overlap with the previous kernel in the stream, if that kernel
 requests the overlap.

 ::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT records an event along with the
 kernel launch. Event recorded through this launch attribute is guaranteed to
 only trigger after all block in the associated kernel trigger the event. A
 block can trigger the event through PTX launchdep.release or CUDA builtin
 function cudaTriggerProgrammaticLaunchCompletion(). A trigger can also be
 inserted at the beginning of each block's execution if triggerAtBlockStart is
 set to non-0. Note that dependents (including the CPU thread calling
 cuEventSynchronize()) are not guaranteed to observe the release precisely
 when it is released. For example, cuEventSynchronize() may only observe the
 event trigger long after the associated kernel has completed. This recording
 type is primarily meant for establishing programmatic dependency between
 device tasks. The event supplied must not be an interprocess or interop
 event. The event must disable timing (i.e. created with
 ::CU_EVENT_DISABLE_TIMING flag set).

 ::CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT records an event along with
 the kernel launch. Nominally, the event is triggered once all blocks of the
 kernel have begun execution. Currently this is a best effort. If a kernel B
 has a launch completion dependency on a kernel A, B may wait until A is
 complete. Alternatively, blocks of B may begin before all blocks of A have
 begun, for example:

  - If B can claim execution resources unavailable to A, for example if they
    run on different GPUs.
  - If B is a higher priority than A.

 Exercise caution if such an ordering inversion could lead to deadlock. The
 event supplied must not be an interprocess or interop event. The event must
 disable timing (i.e. must be created with the ::CU_EVENT_DISABLE_TIMING flag
 set).

 Setting ::CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE to 1
 on a captured launch causes the resulting kernel node to be device-updatable.
 This attribute is specific to graphs, and passing it to a launch in a
 non-capturing stream results in an error. Passing a value other than 0 or 1 is
 not allowed.

 On success, a handle will be returned via
 ::CUlaunchAttributeValue::deviceUpdatableKernelNode::devNode which can be passed
 to the various device-side update functions to update the node's kernel parameters
 from within another kernel. For more information on the types of device updates
 that can be made, as well as the relevant limitations thereof, see
 ::cudaGraphKernelNodeUpdatesApply.

 Kernel nodes which are device-updatable have additional restrictions compared to regular
 kernel nodes. Firstly, device-updatable nodes cannot be removed from their graph via
 ::cuGraphDestroyNode. Additionally, once opted-in to this functionality, a node cannot
 opt out, and any attempt to set the attribute to 0 will result in an error. Graphs
 containing one or more device-updatable node also do not allow multiple instantiation.

 ::CU_LAUNCH_ATTRIBUTE_PREFERRED_CLUSTER_DIMENSION allows the kernel launch to
 specify a preferred substitute cluster dimension. Blocks may be grouped
 according to either the dimensions specified with this attribute (grouped
 into a "preferred substitute cluster"), or the one specified with
 ::CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION attribute (grouped into a "regular
 cluster"). The cluster dimensions of a "preferred substitute cluster" shall
 be an integer multiple greater than zero of the regular cluster dimensions.
 The device will attempt - on a best-effort basis - to group thread blocks
 into preferred clusters over grouping them into regular clusters. When it
 deems necessary (primarily when the device temporarily runs out of physical
 resources to launch the larger preferred clusters), the device may switch to
 launch the regular clusters instead to attempt to utilize as much of the
 physical device resources as possible.

 Each type of cluster will have its enumeration / coordinate setup as if the
 grid consists solely of its type of cluster. For example, if the preferred
 substitute cluster dimensions double the regular cluster dimensions, there
 might be simultaneously a regular cluster indexed at (1,0,0), and a preferred
 cluster indexed at (1,0,0). In this example, the preferred substitute cluster
 (1,0,0) replaces regular clusters (2,0,0) and (3,0,0) and groups their
 blocks.

 This attribute will only take effect when a regular cluster dimension has
 been specified. The preferred substitute The preferred substitute cluster
 dimension must be an integer multiple greater than zero of the regular
 cluster dimension and must divide the grid. It must also be no more than
 `maxBlocksPerCluster`, if it is set in the kernel's `__launch_bounds__`.
 Otherwise it must be less than the maximum value the driver can support.
 Otherwise, setting this attribute to a value physically unable to fit on any
 particular device is permitted.

 The effect of other attributes is consistent with their effect when set via
 persistent APIs.

 See ::cuStreamSetAttribute for
 - ::CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW
 - ::CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY

 See ::cuFuncSetAttribute for
 - ::CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION
 - ::CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE

 Kernel parameters to \p f can be specified in the same ways that they can be
 using ::cuLaunchKernel.

 Note that the API can also be used to launch context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to launch
 the kernel on will either be taken from the specified stream ::CUlaunchConfig::hStream
 or the current context in case of NULL stream.

 \param config         - Config to launch
 \param f              - Function ::CUfunction or Kernel ::CUkernel to launch
 \param kernelParams   - Array of pointers to kernel parameters
 \param extra          - Extra options

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_IMAGE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_LAUNCH_FAILED,
 ::CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES,
 ::CUDA_ERROR_LAUNCH_TIMEOUT,
 ::CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING,
 ::CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
 ::CUDA_ERROR_NOT_FOUND
 \note_null_stream
 \notefnerr

 \sa ::cuCtxGetCacheConfig,
 ::cuCtxSetCacheConfig,
 ::cuFuncSetCacheConfig,
 ::cuFuncGetAttribute,
 ::cudaLaunchKernel,
 ::cudaLaunchKernelEx,
 ::cuLibraryGetKernel,
 ::cuKernelSetCacheConfig,
 ::cuKernelGetAttribute,
 ::cuKernelSetAttribute*/
    fn cuLaunchKernelEx_ptsz(
        config: *const cuda_types::cuda::CUlaunchConfig,
        f: cuda_types::cuda::CUfunction,
        kernelParams: *mut *mut ::core::ffi::c_void,
        extra: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Launches a CUDA function ::CUfunction or a CUDA kernel ::CUkernel where thread blocks
 can cooperate and synchronize as they execute

 Invokes the function ::CUfunction or the kernel ::CUkernel \p f on a \p gridDimX x \p gridDimY x \p gridDimZ
 grid of blocks. Each block contains \p blockDimX x \p blockDimY x
 \p blockDimZ threads.

 \p sharedMemBytes sets the amount of dynamic shared memory that will be
 available to each thread block.

 The device on which this kernel is invoked must have a non-zero value for
 the device attribute ::CU_DEVICE_ATTRIBUTE_COOPERATIVE_LAUNCH.

 The total number of blocks launched cannot exceed the maximum number of blocks per
 multiprocessor as returned by ::cuOccupancyMaxActiveBlocksPerMultiprocessor (or
 ::cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags) times the number of multiprocessors
 as specified by the device attribute ::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT.

 The kernel cannot make use of CUDA dynamic parallelism.

 Kernel parameters must be specified via \p kernelParams.  If \p f
 has N parameters, then \p kernelParams needs to be an array of N
 pointers.  Each of \p kernelParams[0] through \p kernelParams[N-1]
 must point to a region of memory from which the actual kernel
 parameter will be copied.  The number of kernel parameters and their
 offsets and sizes do not need to be specified as that information is
 retrieved directly from the kernel's image.

 Calling ::cuLaunchCooperativeKernel() sets persistent function state that is
 the same as function state set through ::cuLaunchKernel API

 When the kernel \p f is launched via ::cuLaunchCooperativeKernel(), the previous
 block shape, shared size and parameter info associated with \p f
 is overwritten.

 Note that to use ::cuLaunchCooperativeKernel(), the kernel \p f must either have
 been compiled with toolchain version 3.2 or later so that it will
 contain kernel parameter information, or have no kernel parameters.
 If either of these conditions is not met, then ::cuLaunchCooperativeKernel() will
 return ::CUDA_ERROR_INVALID_IMAGE.

 Note that the API can also be used to launch context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to launch
 the kernel on will either be taken from the specified stream \p hStream
 or the current context in case of NULL stream.

 \param f              - Function ::CUfunction or Kernel ::CUkernel to launch
 \param gridDimX       - Width of grid in blocks
 \param gridDimY       - Height of grid in blocks
 \param gridDimZ       - Depth of grid in blocks
 \param blockDimX      - X dimension of each thread block
 \param blockDimY      - Y dimension of each thread block
 \param blockDimZ      - Z dimension of each thread block
 \param sharedMemBytes - Dynamic shared-memory size per thread block in bytes
 \param hStream        - Stream identifier
 \param kernelParams   - Array of pointers to kernel parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_IMAGE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_LAUNCH_FAILED,
 ::CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES,
 ::CUDA_ERROR_LAUNCH_TIMEOUT,
 ::CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING,
 ::CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
 ::CUDA_ERROR_NOT_FOUND
 \note_null_stream
 \notefnerr

 \sa ::cuCtxGetCacheConfig,
 ::cuCtxSetCacheConfig,
 ::cuFuncSetCacheConfig,
 ::cuFuncGetAttribute,
 ::cuLaunchCooperativeKernelMultiDevice,
 ::cudaLaunchCooperativeKernel,
 ::cuLibraryGetKernel,
 ::cuKernelSetCacheConfig,
 ::cuKernelGetAttribute,
 ::cuKernelSetAttribute*/
    fn cuLaunchCooperativeKernel_ptsz(
        f: cuda_types::cuda::CUfunction,
        gridDimX: ::core::ffi::c_uint,
        gridDimY: ::core::ffi::c_uint,
        gridDimZ: ::core::ffi::c_uint,
        blockDimX: ::core::ffi::c_uint,
        blockDimY: ::core::ffi::c_uint,
        blockDimZ: ::core::ffi::c_uint,
        sharedMemBytes: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
        kernelParams: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Launches CUDA functions on multiple devices where thread blocks can cooperate and synchronize as they execute

 \deprecated This function is deprecated as of CUDA 11.3.

 Invokes kernels as specified in the \p launchParamsList array where each element
 of the array specifies all the parameters required to perform a single kernel launch.
 These kernels can cooperate and synchronize as they execute. The size of the array is
 specified by \p numDevices.

 No two kernels can be launched on the same device. All the devices targeted by this
 multi-device launch must be identical. All devices must have a non-zero value for the
 device attribute ::CU_DEVICE_ATTRIBUTE_COOPERATIVE_MULTI_DEVICE_LAUNCH.

 All kernels launched must be identical with respect to the compiled code. Note that
 any __device__, __constant__ or __managed__ variables present in the module that owns
 the kernel launched on each device, are independently instantiated on every device.
 It is the application's responsibility to ensure these variables are initialized and
 used appropriately.

 The size of the grids as specified in blocks, the size of the blocks themselves
 and the amount of shared memory used by each thread block must also match across
 all launched kernels.

 The streams used to launch these kernels must have been created via either ::cuStreamCreate
 or ::cuStreamCreateWithPriority. The NULL stream or ::CU_STREAM_LEGACY or ::CU_STREAM_PER_THREAD
 cannot be used.

 The total number of blocks launched per kernel cannot exceed the maximum number of blocks
 per multiprocessor as returned by ::cuOccupancyMaxActiveBlocksPerMultiprocessor (or
 ::cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags) times the number of multiprocessors
 as specified by the device attribute ::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT. Since the
 total number of blocks launched per device has to match across all devices, the maximum
 number of blocks that can be launched per device will be limited by the device with the
 least number of multiprocessors.

 The kernels cannot make use of CUDA dynamic parallelism.

 The ::CUDA_LAUNCH_PARAMS structure is defined as:
 \code
typedef struct CUDA_LAUNCH_PARAMS_st
{
CUfunction function;
unsigned int gridDimX;
unsigned int gridDimY;
unsigned int gridDimZ;
unsigned int blockDimX;
unsigned int blockDimY;
unsigned int blockDimZ;
unsigned int sharedMemBytes;
CUstream hStream;
void **kernelParams;
} CUDA_LAUNCH_PARAMS;
 \endcode
 where:
 - ::CUDA_LAUNCH_PARAMS::function specifies the kernel to be launched. All functions must
   be identical with respect to the compiled code.
   Note that you can also specify context-less kernel ::CUkernel by querying the handle
   using ::cuLibraryGetKernel() and then casting to ::CUfunction. In this case, the context to
   launch the kernel on be taken from the specified stream ::CUDA_LAUNCH_PARAMS::hStream.
 - ::CUDA_LAUNCH_PARAMS::gridDimX is the width of the grid in blocks. This must match across
   all kernels launched.
 - ::CUDA_LAUNCH_PARAMS::gridDimY is the height of the grid in blocks. This must match across
   all kernels launched.
 - ::CUDA_LAUNCH_PARAMS::gridDimZ is the depth of the grid in blocks. This must match across
   all kernels launched.
 - ::CUDA_LAUNCH_PARAMS::blockDimX is the X dimension of each thread block. This must match across
   all kernels launched.
 - ::CUDA_LAUNCH_PARAMS::blockDimX is the Y dimension of each thread block. This must match across
   all kernels launched.
 - ::CUDA_LAUNCH_PARAMS::blockDimZ is the Z dimension of each thread block. This must match across
   all kernels launched.
 - ::CUDA_LAUNCH_PARAMS::sharedMemBytes is the dynamic shared-memory size per thread block in bytes.
   This must match across all kernels launched.
 - ::CUDA_LAUNCH_PARAMS::hStream is the handle to the stream to perform the launch in. This cannot
   be the NULL stream or ::CU_STREAM_LEGACY or ::CU_STREAM_PER_THREAD. The CUDA context associated
   with this stream must match that associated with ::CUDA_LAUNCH_PARAMS::function.
 - ::CUDA_LAUNCH_PARAMS::kernelParams is an array of pointers to kernel parameters. If
   ::CUDA_LAUNCH_PARAMS::function has N parameters, then ::CUDA_LAUNCH_PARAMS::kernelParams
   needs to be an array of N pointers. Each of ::CUDA_LAUNCH_PARAMS::kernelParams[0] through
   ::CUDA_LAUNCH_PARAMS::kernelParams[N-1] must point to a region of memory from which the actual
   kernel parameter will be copied. The number of kernel parameters and their offsets and sizes
   do not need to be specified as that information is retrieved directly from the kernel's image.

 By default, the kernel won't begin execution on any GPU until all prior work in all the specified
 streams has completed. This behavior can be overridden by specifying the flag
 ::CUDA_COOPERATIVE_LAUNCH_MULTI_DEVICE_NO_PRE_LAUNCH_SYNC. When this flag is specified, each kernel
 will only wait for prior work in the stream corresponding to that GPU to complete before it begins
 execution.

 Similarly, by default, any subsequent work pushed in any of the specified streams will not begin
 execution until the kernels on all GPUs have completed. This behavior can be overridden by specifying
 the flag ::CUDA_COOPERATIVE_LAUNCH_MULTI_DEVICE_NO_POST_LAUNCH_SYNC. When this flag is specified,
 any subsequent work pushed in any of the specified streams will only wait for the kernel launched
 on the GPU corresponding to that stream to complete before it begins execution.

 Calling ::cuLaunchCooperativeKernelMultiDevice() sets persistent function state that is
 the same as function state set through ::cuLaunchKernel API when called individually for each
 element in \p launchParamsList.

 When kernels are launched via ::cuLaunchCooperativeKernelMultiDevice(), the previous
 block shape, shared size and parameter info associated with each ::CUDA_LAUNCH_PARAMS::function
 in \p launchParamsList is overwritten.

 Note that to use ::cuLaunchCooperativeKernelMultiDevice(), the kernels must either have
 been compiled with toolchain version 3.2 or later so that it will
 contain kernel parameter information, or have no kernel parameters.
 If either of these conditions is not met, then ::cuLaunchCooperativeKernelMultiDevice() will
 return ::CUDA_ERROR_INVALID_IMAGE.

 \param launchParamsList - List of launch parameters, one per device
 \param numDevices       - Size of the \p launchParamsList array
 \param flags            - Flags to control launch behavior

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_IMAGE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_LAUNCH_FAILED,
 ::CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES,
 ::CUDA_ERROR_LAUNCH_TIMEOUT,
 ::CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING,
 ::CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED
 \note_null_stream
 \notefnerr

 \sa ::cuCtxGetCacheConfig,
 ::cuCtxSetCacheConfig,
 ::cuFuncSetCacheConfig,
 ::cuFuncGetAttribute,
 ::cuLaunchCooperativeKernel,
 ::cudaLaunchCooperativeKernelMultiDevice*/
    fn cuLaunchCooperativeKernelMultiDevice(
        launchParamsList: *mut cuda_types::cuda::CUDA_LAUNCH_PARAMS,
        numDevices: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Enqueues a host function call in a stream

 Enqueues a host function to run in a stream.  The function will be called
 after currently enqueued work and will block work added after it.

 The host function must not make any CUDA API calls.  Attempting to use a
 CUDA API may result in ::CUDA_ERROR_NOT_PERMITTED, but this is not required.
 The host function must not perform any synchronization that may depend on
 outstanding CUDA work not mandated to run earlier.  Host functions without a
 mandated order (such as in independent streams) execute in undefined order
 and may be serialized.

 For the purposes of Unified Memory, execution makes a number of guarantees:
 <ul>
   <li>The stream is considered idle for the duration of the function's
   execution.  Thus, for example, the function may always use memory attached
   to the stream it was enqueued in.</li>
   <li>The start of execution of the function has the same effect as
   synchronizing an event recorded in the same stream immediately prior to
   the function.  It thus synchronizes streams which have been "joined"
   prior to the function.</li>
   <li>Adding device work to any stream does not have the effect of making
   the stream active until all preceding host functions and stream callbacks
   have executed.  Thus, for
   example, a function might use global attached memory even if work has
   been added to another stream, if the work has been ordered behind the
   function call with an event.</li>
   <li>Completion of the function does not cause a stream to become
   active except as described above.  The stream will remain idle
   if no device work follows the function, and will remain idle across
   consecutive host functions or stream callbacks without device work in
   between.  Thus, for example,
   stream synchronization can be done by signaling from a host function at the
   end of the stream.</li>
 </ul>

 Note that, in contrast to ::cuStreamAddCallback, the function will not be
 called in the event of an error in the CUDA context.

 \param hStream  - Stream to enqueue function call in
 \param fn       - The function to call once preceding stream operations are complete
 \param userData - User-specified data to be passed to the function

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \note_null_stream
 \notefnerr

 \sa ::cuStreamCreate,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamWaitEvent,
 ::cuStreamDestroy,
 ::cuMemAllocManaged,
 ::cuStreamAttachMemAsync,
 ::cuStreamAddCallback*/
    fn cuLaunchHostFunc_ptsz(
        hStream: cuda_types::cuda::CUstream,
        fn_: cuda_types::cuda::CUhostFn,
        userData: *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the block-dimensions for the function

 \deprecated

 Specifies the \p x, \p y, and \p z dimensions of the thread blocks that are
 created when the kernel given by \p hfunc is launched.

 \param hfunc - Kernel to specify dimensions of
 \param x     - X dimension
 \param y     - Y dimension
 \param z     - Z dimension

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuFuncSetSharedSize,
 ::cuFuncSetCacheConfig,
 ::cuFuncGetAttribute,
 ::cuParamSetSize,
 ::cuParamSeti,
 ::cuParamSetf,
 ::cuParamSetv,
 ::cuLaunch,
 ::cuLaunchGrid,
 ::cuLaunchGridAsync,
 ::cuLaunchKernel*/
    fn cuFuncSetBlockShape(
        hfunc: cuda_types::cuda::CUfunction,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        z: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the dynamic shared-memory size for the function

 \deprecated

 Sets through \p bytes the amount of dynamic shared memory that will be
 available to each thread block when the kernel given by \p hfunc is launched.

 \param hfunc - Kernel to specify dynamic shared-memory size for
 \param bytes - Dynamic shared-memory size per thread in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuFuncSetBlockShape,
 ::cuFuncSetCacheConfig,
 ::cuFuncGetAttribute,
 ::cuParamSetSize,
 ::cuParamSeti,
 ::cuParamSetf,
 ::cuParamSetv,
 ::cuLaunch,
 ::cuLaunchGrid,
 ::cuLaunchGridAsync,
 ::cuLaunchKernel*/
    fn cuFuncSetSharedSize(
        hfunc: cuda_types::cuda::CUfunction,
        bytes: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the parameter size for the function

 \deprecated

 Sets through \p numbytes the total size in bytes needed by the function
 parameters of the kernel corresponding to \p hfunc.

 \param hfunc    - Kernel to set parameter size for
 \param numbytes - Size of parameter list in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuFuncSetBlockShape,
 ::cuFuncSetSharedSize,
 ::cuFuncGetAttribute,
 ::cuParamSetf,
 ::cuParamSeti,
 ::cuParamSetv,
 ::cuLaunch,
 ::cuLaunchGrid,
 ::cuLaunchGridAsync,
 ::cuLaunchKernel*/
    fn cuParamSetSize(
        hfunc: cuda_types::cuda::CUfunction,
        numbytes: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Adds an integer parameter to the function's argument list

 \deprecated

 Sets an integer parameter that will be specified the next time the
 kernel corresponding to \p hfunc will be invoked. \p offset is a byte offset.

 \param hfunc  - Kernel to add parameter to
 \param offset - Offset to add parameter to argument list
 \param value  - Value of parameter

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuFuncSetBlockShape,
 ::cuFuncSetSharedSize,
 ::cuFuncGetAttribute,
 ::cuParamSetSize,
 ::cuParamSetf,
 ::cuParamSetv,
 ::cuLaunch,
 ::cuLaunchGrid,
 ::cuLaunchGridAsync,
 ::cuLaunchKernel*/
    fn cuParamSeti(
        hfunc: cuda_types::cuda::CUfunction,
        offset: ::core::ffi::c_int,
        value: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Adds a floating-point parameter to the function's argument list

 \deprecated

 Sets a floating-point parameter that will be specified the next time the
 kernel corresponding to \p hfunc will be invoked. \p offset is a byte offset.

 \param hfunc  - Kernel to add parameter to
 \param offset - Offset to add parameter to argument list
 \param value  - Value of parameter

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuFuncSetBlockShape,
 ::cuFuncSetSharedSize,
 ::cuFuncGetAttribute,
 ::cuParamSetSize,
 ::cuParamSeti,
 ::cuParamSetv,
 ::cuLaunch,
 ::cuLaunchGrid,
 ::cuLaunchGridAsync,
 ::cuLaunchKernel*/
    fn cuParamSetf(
        hfunc: cuda_types::cuda::CUfunction,
        offset: ::core::ffi::c_int,
        value: f32,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Adds arbitrary data to the function's argument list

 \deprecated

 Copies an arbitrary amount of data (specified in \p numbytes) from \p ptr
 into the parameter space of the kernel corresponding to \p hfunc. \p offset
 is a byte offset.

 \param hfunc    - Kernel to add data to
 \param offset   - Offset to add data to argument list
 \param ptr      - Pointer to arbitrary data
 \param numbytes - Size of data to copy in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuFuncSetBlockShape,
 ::cuFuncSetSharedSize,
 ::cuFuncGetAttribute,
 ::cuParamSetSize,
 ::cuParamSetf,
 ::cuParamSeti,
 ::cuLaunch,
 ::cuLaunchGrid,
 ::cuLaunchGridAsync,
 ::cuLaunchKernel*/
    fn cuParamSetv(
        hfunc: cuda_types::cuda::CUfunction,
        offset: ::core::ffi::c_int,
        ptr: *mut ::core::ffi::c_void,
        numbytes: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Launches a CUDA function

 \deprecated

 Invokes the kernel \p f on a 1 x 1 x 1 grid of blocks. The block
 contains the number of threads specified by a previous call to
 ::cuFuncSetBlockShape().

 The block shape, dynamic shared memory size, and parameter information
 must be set using
  ::cuFuncSetBlockShape(),
  ::cuFuncSetSharedSize(),
  ::cuParamSetSize(),
  ::cuParamSeti(),
  ::cuParamSetf(), and
  ::cuParamSetv()
 prior to calling this function.

 Launching a function via ::cuLaunchKernel() invalidates the function's
 block shape, dynamic shared memory size, and parameter information. After
 launching via cuLaunchKernel, this state must be re-initialized prior to
 calling this function. Failure to do so results in undefined behavior.

 \param f - Kernel to launch

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_LAUNCH_FAILED,
 ::CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES,
 ::CUDA_ERROR_LAUNCH_TIMEOUT,
 ::CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED
 \notefnerr

 \sa ::cuFuncSetBlockShape,
 ::cuFuncSetSharedSize,
 ::cuFuncGetAttribute,
 ::cuParamSetSize,
 ::cuParamSetf,
 ::cuParamSeti,
 ::cuParamSetv,
 ::cuLaunchGrid,
 ::cuLaunchGridAsync,
 ::cuLaunchKernel*/
    fn cuLaunch(f: cuda_types::cuda::CUfunction) -> cuda_types::cuda::CUresult;
    /** \brief Launches a CUDA function

 \deprecated

 Invokes the kernel \p f on a \p grid_width x \p grid_height grid of
 blocks. Each block contains the number of threads specified by a previous
 call to ::cuFuncSetBlockShape().

 The block shape, dynamic shared memory size, and parameter information
 must be set using
  ::cuFuncSetBlockShape(),
  ::cuFuncSetSharedSize(),
  ::cuParamSetSize(),
  ::cuParamSeti(),
  ::cuParamSetf(), and
  ::cuParamSetv()
 prior to calling this function.

 Launching a function via ::cuLaunchKernel() invalidates the function's
 block shape, dynamic shared memory size, and parameter information. After
 launching via cuLaunchKernel, this state must be re-initialized prior to
 calling this function. Failure to do so results in undefined behavior.

 \param f           - Kernel to launch
 \param grid_width  - Width of grid in blocks
 \param grid_height - Height of grid in blocks

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_LAUNCH_FAILED,
 ::CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES,
 ::CUDA_ERROR_LAUNCH_TIMEOUT,
 ::CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED
 \notefnerr

 \sa ::cuFuncSetBlockShape,
 ::cuFuncSetSharedSize,
 ::cuFuncGetAttribute,
 ::cuParamSetSize,
 ::cuParamSetf,
 ::cuParamSeti,
 ::cuParamSetv,
 ::cuLaunch,
 ::cuLaunchGridAsync,
 ::cuLaunchKernel*/
    fn cuLaunchGrid(
        f: cuda_types::cuda::CUfunction,
        grid_width: ::core::ffi::c_int,
        grid_height: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Launches a CUDA function

 \deprecated

 Invokes the kernel \p f on a \p grid_width x \p grid_height grid of
 blocks. Each block contains the number of threads specified by a previous
 call to ::cuFuncSetBlockShape().

 The block shape, dynamic shared memory size, and parameter information
 must be set using
  ::cuFuncSetBlockShape(),
  ::cuFuncSetSharedSize(),
  ::cuParamSetSize(),
  ::cuParamSeti(),
  ::cuParamSetf(), and
  ::cuParamSetv()
 prior to calling this function.

 Launching a function via ::cuLaunchKernel() invalidates the function's
 block shape, dynamic shared memory size, and parameter information. After
 launching via cuLaunchKernel, this state must be re-initialized prior to
 calling this function. Failure to do so results in undefined behavior.

 \param f           - Kernel to launch
 \param grid_width  - Width of grid in blocks
 \param grid_height - Height of grid in blocks
 \param hStream     - Stream identifier

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_LAUNCH_FAILED,
 ::CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES,
 ::CUDA_ERROR_LAUNCH_TIMEOUT,
 ::CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING,
 ::CUDA_ERROR_SHARED_OBJECT_INIT_FAILED

 \note In certain cases where cubins are created with no ABI (i.e., using \p ptxas \p --abi-compile \p no),
       this function may serialize kernel launches. The CUDA driver retains asynchronous behavior by
       growing the per-thread stack as needed per launch and not shrinking it afterwards.

 \note_null_stream
 \notefnerr

 \sa ::cuFuncSetBlockShape,
 ::cuFuncSetSharedSize,
 ::cuFuncGetAttribute,
 ::cuParamSetSize,
 ::cuParamSetf,
 ::cuParamSeti,
 ::cuParamSetv,
 ::cuLaunch,
 ::cuLaunchGrid,
 ::cuLaunchKernel*/
    fn cuLaunchGridAsync(
        f: cuda_types::cuda::CUfunction,
        grid_width: ::core::ffi::c_int,
        grid_height: ::core::ffi::c_int,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Adds a texture-reference to the function's argument list

 \deprecated

 Makes the CUDA array or linear memory bound to the texture reference
 \p hTexRef available to a device program as a texture. In this version of
 CUDA, the texture-reference must be obtained via ::cuModuleGetTexRef() and
 the \p texunit parameter must be set to ::CU_PARAM_TR_DEFAULT.

 \param hfunc   - Kernel to add texture-reference to
 \param texunit - Texture unit (must be ::CU_PARAM_TR_DEFAULT)
 \param hTexRef - Texture-reference to add to argument list

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr*/
    fn cuParamSetTexRef(
        hfunc: cuda_types::cuda::CUfunction,
        texunit: ::core::ffi::c_int,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the shared memory configuration for a device function.

 \deprecated

 On devices with configurable shared memory banks, this function will
 force all subsequent launches of the specified device function to have
 the given shared memory bank size configuration. On any given launch of the
 function, the shared memory configuration of the device will be temporarily
 changed if needed to suit the function's preferred configuration. Changes in
 shared memory configuration between subsequent launches of functions,
 may introduce a device side synchronization point.

 Any per-function setting of shared memory bank size set via
 ::cuFuncSetSharedMemConfig will override the context wide setting set with
 ::cuCtxSetSharedMemConfig.

 Changing the shared memory bank size will not increase shared memory usage
 or affect occupancy of kernels, but may have major effects on performance.
 Larger bank sizes will allow for greater potential bandwidth to shared memory,
 but will change what kinds of accesses to shared memory will result in bank
 conflicts.

 This function will do nothing on devices with fixed shared memory bank size.

 The supported bank configurations are:
 - ::CU_SHARED_MEM_CONFIG_DEFAULT_BANK_SIZE: use the context's shared memory
   configuration when launching this function.
 - ::CU_SHARED_MEM_CONFIG_FOUR_BYTE_BANK_SIZE: set shared memory bank width to
   be natively four bytes when launching this function.
 - ::CU_SHARED_MEM_CONFIG_EIGHT_BYTE_BANK_SIZE: set shared memory bank width to
   be natively eight bytes when launching this function.

 \param hfunc  - kernel to be given a shared memory config
 \param config - requested shared memory configuration

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa ::cuCtxGetCacheConfig,
 ::cuCtxSetCacheConfig,
 ::cuCtxGetSharedMemConfig,
 ::cuCtxSetSharedMemConfig,
 ::cuFuncGetAttribute,
 ::cuLaunchKernel,
 ::cudaFuncSetSharedMemConfig*/
    fn cuFuncSetSharedMemConfig(
        hfunc: cuda_types::cuda::CUfunction,
        config: cuda_types::cuda::CUsharedconfig,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a graph

 Creates an empty graph, which is returned via \p phGraph.

 \param phGraph - Returns newly created graph
 \param flags   - Graph creation flags, must be 0

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddHostNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode,
 ::cuGraphInstantiate,
 ::cuGraphDestroy,
 ::cuGraphGetNodes,
 ::cuGraphGetRootNodes,
 ::cuGraphGetEdges,
 ::cuGraphClone*/
    fn cuGraphCreate(
        phGraph: *mut cuda_types::cuda::CUgraph,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a kernel execution node and adds it to a graph

 Creates a new kernel execution node and adds it to \p hGraph with \p numDependencies
 dependencies specified via \p dependencies and arguments specified in \p nodeParams.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries.
 A handle to the new node will be returned in \p phGraphNode.

 The CUDA_KERNEL_NODE_PARAMS structure is defined as:

 \code
  typedef struct CUDA_KERNEL_NODE_PARAMS_st {
      CUfunction func;
      unsigned int gridDimX;
      unsigned int gridDimY;
      unsigned int gridDimZ;
      unsigned int blockDimX;
      unsigned int blockDimY;
      unsigned int blockDimZ;
      unsigned int sharedMemBytes;
      void **kernelParams;
      void **extra;
      CUkernel kern;
      CUcontext ctx;
  } CUDA_KERNEL_NODE_PARAMS;
 \endcode

 When the graph is launched, the node will invoke kernel \p func on a (\p gridDimX x
 \p gridDimY x \p gridDimZ) grid of blocks. Each block contains
 (\p blockDimX x \p blockDimY x \p blockDimZ) threads.

 \p sharedMemBytes sets the amount of dynamic shared memory that will be
 available to each thread block.

 Kernel parameters to \p func can be specified in one of two ways:

 1) Kernel parameters can be specified via \p kernelParams. If the kernel has N
 parameters, then \p kernelParams needs to be an array of N pointers. Each pointer,
 from \p kernelParams[0] to \p kernelParams[N-1], points to the region of memory from which the actual
 parameter will be copied. The number of kernel parameters and their offsets and sizes do not need
 to be specified as that information is retrieved directly from the kernel's image.

 2) Kernel parameters for non-cooperative kernels can also be packaged by the application into a single
 buffer that is passed in via \p extra. This places the burden on the application of knowing each
 kernel parameter's size and alignment/padding within the buffer. The \p extra parameter exists
 to allow this function to take additional less commonly used arguments. \p extra specifies
 a list of names of extra settings and their corresponding values. Each extra setting name is
 immediately followed by the corresponding value. The list must be terminated with either NULL or
 CU_LAUNCH_PARAM_END.

 - ::CU_LAUNCH_PARAM_END, which indicates the end of the \p extra
   array;
 - ::CU_LAUNCH_PARAM_BUFFER_POINTER, which specifies that the next
   value in \p extra will be a pointer to a buffer
   containing all the kernel parameters for launching kernel
   \p func;
 - ::CU_LAUNCH_PARAM_BUFFER_SIZE, which specifies that the next
   value in \p extra will be a pointer to a size_t
   containing the size of the buffer specified with
   ::CU_LAUNCH_PARAM_BUFFER_POINTER;

 The error ::CUDA_ERROR_INVALID_VALUE will be returned if kernel parameters are specified with both
 \p kernelParams and \p extra (i.e. both \p kernelParams and \p extra are non-NULL).
 ::CUDA_ERROR_INVALID_VALUE will be returned if \p extra is used for a cooperative kernel.

 The \p kernelParams or \p extra array, as well as the argument values it points to,
 are copied during this call.

 \note Kernels launched using graphs must not use texture and surface references. Reading or
       writing through any texture or surface reference is undefined behavior.
       This restriction does not apply to texture and surface objects.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param nodeParams      - Parameters for the GPU execution node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuLaunchKernel,
 ::cuLaunchCooperativeKernel,
 ::cuGraphKernelNodeGetParams,
 ::cuGraphKernelNodeSetParams,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddHostNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddKernelNode_v2(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a kernel node's parameters

 Returns the parameters of kernel node \p hNode in \p nodeParams.
 The \p kernelParams or \p extra array returned in \p nodeParams,
 as well as the argument values it points to, are owned by the node.
 This memory remains valid until the node is destroyed or its
 parameters are modified, and should not be modified
 directly. Use ::cuGraphKernelNodeSetParams to update the
 parameters of this node.

 The params will contain either \p kernelParams or \p extra,
 according to which of these was most recently set on the node.

 \param hNode      - Node to get the parameters for
 \param nodeParams - Pointer to return the parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuLaunchKernel,
 ::cuGraphAddKernelNode,
 ::cuGraphKernelNodeSetParams*/
    fn cuGraphKernelNodeGetParams_v2(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *mut cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets a kernel node's parameters

 Sets the parameters of kernel node \p hNode to \p nodeParams.

 \param hNode      - Node to set the parameters for
 \param nodeParams - Parameters to copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetParams,
 ::cuLaunchKernel,
 ::cuGraphAddKernelNode,
 ::cuGraphKernelNodeGetParams*/
    fn cuGraphKernelNodeSetParams_v2(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a memcpy node and adds it to a graph

 Creates a new memcpy node and adds it to \p hGraph with \p numDependencies
 dependencies specified via \p dependencies.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries.
 A handle to the new node will be returned in \p phGraphNode.

 When the graph is launched, the node will perform the memcpy described by \p copyParams.
 See ::cuMemcpy3D() for a description of the structure and its restrictions.

 Memcpy nodes have some additional restrictions with regards to managed memory, if the
 system contains at least one device which has a zero value for the device attribute
 ::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS. If one or more of the operands refer
 to managed memory, then using the memory type ::CU_MEMORYTYPE_UNIFIED is disallowed
 for those operand(s). The managed memory will be treated as residing on either the
 host or the device, depending on which memory type is specified.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param copyParams      - Parameters for the memory copy
 \param ctx             - Context on which to run the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuMemcpy3D,
 ::cuGraphMemcpyNodeGetParams,
 ::cuGraphMemcpyNodeSetParams,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddHostNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddMemcpyNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        copyParams: *const cuda_types::cuda::CUDA_MEMCPY3D,
        ctx: cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a memcpy node's parameters

 Returns the parameters of memcpy node \p hNode in \p nodeParams.

 \param hNode      - Node to get the parameters for
 \param nodeParams - Pointer to return the parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuMemcpy3D,
 ::cuGraphAddMemcpyNode,
 ::cuGraphMemcpyNodeSetParams*/
    fn cuGraphMemcpyNodeGetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *mut cuda_types::cuda::CUDA_MEMCPY3D,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets a memcpy node's parameters

 Sets the parameters of memcpy node \p hNode to \p nodeParams.

 \param hNode      - Node to set the parameters for
 \param nodeParams - Parameters to copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetParams,
 ::cuMemcpy3D,
 ::cuGraphAddMemcpyNode,
 ::cuGraphMemcpyNodeGetParams*/
    fn cuGraphMemcpyNodeSetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_MEMCPY3D,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a memset node and adds it to a graph

 Creates a new memset node and adds it to \p hGraph with \p numDependencies
 dependencies specified via \p dependencies.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries.
 A handle to the new node will be returned in \p phGraphNode.

 The element size must be 1, 2, or 4 bytes.
 When the graph is launched, the node will perform the memset described by \p memsetParams.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param memsetParams    - Parameters for the memory set
 \param ctx             - Context on which to run the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_CONTEXT
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuMemsetD2D32,
 ::cuGraphMemsetNodeGetParams,
 ::cuGraphMemsetNodeSetParams,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddHostNode,
 ::cuGraphAddMemcpyNode*/
    fn cuGraphAddMemsetNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        memsetParams: *const cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS,
        ctx: cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a memset node's parameters

 Returns the parameters of memset node \p hNode in \p nodeParams.

 \param hNode      - Node to get the parameters for
 \param nodeParams - Pointer to return the parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuMemsetD2D32,
 ::cuGraphAddMemsetNode,
 ::cuGraphMemsetNodeSetParams*/
    fn cuGraphMemsetNodeGetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *mut cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets a memset node's parameters

 Sets the parameters of memset node \p hNode to \p nodeParams.

 \param hNode      - Node to set the parameters for
 \param nodeParams - Parameters to copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetParams,
 ::cuMemsetD2D32,
 ::cuGraphAddMemsetNode,
 ::cuGraphMemsetNodeGetParams*/
    fn cuGraphMemsetNodeSetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a host execution node and adds it to a graph

 Creates a new CPU execution node and adds it to \p hGraph with \p numDependencies
 dependencies specified via \p dependencies and arguments specified in \p nodeParams.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries.
 A handle to the new node will be returned in \p phGraphNode.

 When the graph is launched, the node will invoke the specified CPU function.
 Host nodes are not supported under MPS with pre-Volta GPUs.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param nodeParams      - Parameters for the host node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuLaunchHostFunc,
 ::cuGraphHostNodeGetParams,
 ::cuGraphHostNodeSetParams,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddHostNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        nodeParams: *const cuda_types::cuda::CUDA_HOST_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a host node's parameters

 Returns the parameters of host node \p hNode in \p nodeParams.

 \param hNode      - Node to get the parameters for
 \param nodeParams - Pointer to return the parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuLaunchHostFunc,
 ::cuGraphAddHostNode,
 ::cuGraphHostNodeSetParams*/
    fn cuGraphHostNodeGetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *mut cuda_types::cuda::CUDA_HOST_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets a host node's parameters

 Sets the parameters of host node \p hNode to \p nodeParams.

 \param hNode      - Node to set the parameters for
 \param nodeParams - Parameters to copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetParams,
 ::cuLaunchHostFunc,
 ::cuGraphAddHostNode,
 ::cuGraphHostNodeGetParams*/
    fn cuGraphHostNodeSetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_HOST_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a child graph node and adds it to a graph

 Creates a new node which executes an embedded graph, and adds it to \p hGraph with
 \p numDependencies dependencies specified via \p dependencies.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries.
 A handle to the new node will be returned in \p phGraphNode.

 If \p childGraph contains allocation nodes, free nodes, or conditional nodes, this call will
 return an error.

 The node executes an embedded child graph. The child graph is cloned in this call.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param childGraph      - The graph to clone into this node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphChildGraphNodeGetGraph,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddHostNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode,
 ::cuGraphClone*/
    fn cuGraphAddChildGraphNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        childGraph: cuda_types::cuda::CUgraph,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets a handle to the embedded graph of a child graph node

 Gets a handle to the embedded graph in a child graph node. This call
 does not clone the graph. Changes to the graph will be reflected in
 the node, and the node retains ownership of the graph.

 Allocation and free nodes cannot be added to the returned graph.
 Attempting to do so will return an error.

 \param hNode   - Node to get the embedded graph for
 \param phGraph - Location to store a handle to the graph

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddChildGraphNode,
 ::cuGraphNodeFindInClone*/
    fn cuGraphChildGraphNodeGetGraph(
        hNode: cuda_types::cuda::CUgraphNode,
        phGraph: *mut cuda_types::cuda::CUgraph,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an empty node and adds it to a graph

 Creates a new node which performs no operation, and adds it to \p hGraph with
 \p numDependencies dependencies specified via \p dependencies.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries.
 A handle to the new node will be returned in \p phGraphNode.

 An empty node performs no operation during execution, but can be used for
 transitive ordering. For example, a phased execution graph with 2 groups of n
 nodes with a barrier between them can be represented using an empty node and
 2*n dependency edges, rather than no empty node and n^2 dependency edges.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddHostNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddEmptyNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an event record node and adds it to a graph

 Creates a new event record node and adds it to \p hGraph with \p numDependencies
 dependencies specified via \p dependencies and event specified in \p event.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries.
 A handle to the new node will be returned in \p phGraphNode.

 Each launch of the graph will record \p event to capture execution of the
 node's dependencies.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param event           - Event for the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphAddEventWaitNode,
 ::cuEventRecordWithFlags,
 ::cuStreamWaitEvent,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddEventRecordNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        event: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the event associated with an event record node

 Returns the event of event record node \p hNode in \p event_out.

 \param hNode     - Node to get the event for
 \param event_out - Pointer to return the event

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddEventRecordNode,
 ::cuGraphEventRecordNodeSetEvent,
 ::cuGraphEventWaitNodeGetEvent,
 ::cuEventRecordWithFlags,
 ::cuStreamWaitEvent*/
    fn cuGraphEventRecordNodeGetEvent(
        hNode: cuda_types::cuda::CUgraphNode,
        event_out: *mut cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets an event record node's event

 Sets the event of event record node \p hNode to \p event.

 \param hNode - Node to set the event for
 \param event - Event to use

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetParams,
 ::cuGraphAddEventRecordNode,
 ::cuGraphEventRecordNodeGetEvent,
 ::cuGraphEventWaitNodeSetEvent,
 ::cuEventRecordWithFlags,
 ::cuStreamWaitEvent*/
    fn cuGraphEventRecordNodeSetEvent(
        hNode: cuda_types::cuda::CUgraphNode,
        event: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an event wait node and adds it to a graph

 Creates a new event wait node and adds it to \p hGraph with \p numDependencies
 dependencies specified via \p dependencies and event specified in \p event.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries.
 A handle to the new node will be returned in \p phGraphNode.

 The graph node will wait for all work captured in \p event.  See ::cuEventRecord()
 for details on what is captured by an event. \p event may be from a different context
 or device than the launch stream.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param event           - Event for the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphAddEventRecordNode,
 ::cuEventRecordWithFlags,
 ::cuStreamWaitEvent,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddEventWaitNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        event: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the event associated with an event wait node

 Returns the event of event wait node \p hNode in \p event_out.

 \param hNode     - Node to get the event for
 \param event_out - Pointer to return the event

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddEventWaitNode,
 ::cuGraphEventWaitNodeSetEvent,
 ::cuGraphEventRecordNodeGetEvent,
 ::cuEventRecordWithFlags,
 ::cuStreamWaitEvent*/
    fn cuGraphEventWaitNodeGetEvent(
        hNode: cuda_types::cuda::CUgraphNode,
        event_out: *mut cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets an event wait node's event

 Sets the event of event wait node \p hNode to \p event.

 \param hNode - Node to set the event for
 \param event - Event to use

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetParams,
 ::cuGraphAddEventWaitNode,
 ::cuGraphEventWaitNodeGetEvent,
 ::cuGraphEventRecordNodeSetEvent,
 ::cuEventRecordWithFlags,
 ::cuStreamWaitEvent*/
    fn cuGraphEventWaitNodeSetEvent(
        hNode: cuda_types::cuda::CUgraphNode,
        event: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an external semaphore signal node and adds it to a graph

 Creates a new external semaphore signal node and adds it to \p hGraph with \p
 numDependencies dependencies specified via \p dependencies and arguments specified
 in \p nodeParams. It is possible for \p numDependencies to be 0, in which case the
 node will be placed at the root of the graph. \p dependencies may not have any
 duplicate entries. A handle to the new node will be returned in \p phGraphNode.

 Performs a signal operation on a set of externally allocated semaphore objects
 when the node is launched.  The operation(s) will occur after all of the node's
 dependencies have completed.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param nodeParams      - Parameters for the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphExternalSemaphoresSignalNodeGetParams,
 ::cuGraphExternalSemaphoresSignalNodeSetParams,
 ::cuGraphExecExternalSemaphoresSignalNodeSetParams,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuImportExternalSemaphore,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddEventRecordNode,
 ::cuGraphAddEventWaitNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddExternalSemaphoresSignalNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns an external semaphore signal node's parameters

 Returns the parameters of an external semaphore signal node \p hNode in \p params_out.
 The \p extSemArray and \p paramsArray returned in \p params_out,
 are owned by the node.  This memory remains valid until the node is destroyed or its
 parameters are modified, and should not be modified
 directly. Use ::cuGraphExternalSemaphoresSignalNodeSetParams to update the
 parameters of this node.

 \param hNode      - Node to get the parameters for
 \param params_out - Pointer to return the parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuLaunchKernel,
 ::cuGraphAddExternalSemaphoresSignalNode,
 ::cuGraphExternalSemaphoresSignalNodeSetParams,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync*/
    fn cuGraphExternalSemaphoresSignalNodeGetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        params_out: *mut cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets an external semaphore signal node's parameters

 Sets the parameters of an external semaphore signal node \p hNode to \p nodeParams.

 \param hNode      - Node to set the parameters for
 \param nodeParams - Parameters to copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetParams,
 ::cuGraphAddExternalSemaphoresSignalNode,
 ::cuGraphExternalSemaphoresSignalNodeSetParams,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync*/
    fn cuGraphExternalSemaphoresSignalNodeSetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an external semaphore wait node and adds it to a graph

 Creates a new external semaphore wait node and adds it to \p hGraph with \p numDependencies
 dependencies specified via \p dependencies and arguments specified in \p nodeParams.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries. A handle
 to the new node will be returned in \p phGraphNode.

 Performs a wait operation on a set of externally allocated semaphore objects
 when the node is launched.  The node's dependencies will not be launched until
 the wait operation has completed.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param nodeParams      - Parameters for the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphExternalSemaphoresWaitNodeGetParams,
 ::cuGraphExternalSemaphoresWaitNodeSetParams,
 ::cuGraphExecExternalSemaphoresWaitNodeSetParams,
 ::cuGraphAddExternalSemaphoresSignalNode,
 ::cuImportExternalSemaphore,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddEventRecordNode,
 ::cuGraphAddEventWaitNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddExternalSemaphoresWaitNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns an external semaphore wait node's parameters

 Returns the parameters of an external semaphore wait node \p hNode in \p params_out.
 The \p extSemArray and \p paramsArray returned in \p params_out,
 are owned by the node.  This memory remains valid until the node is destroyed or its
 parameters are modified, and should not be modified
 directly. Use ::cuGraphExternalSemaphoresSignalNodeSetParams to update the
 parameters of this node.

 \param hNode      - Node to get the parameters for
 \param params_out - Pointer to return the parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuLaunchKernel,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuGraphExternalSemaphoresWaitNodeSetParams,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync*/
    fn cuGraphExternalSemaphoresWaitNodeGetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        params_out: *mut cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets an external semaphore wait node's parameters

 Sets the parameters of an external semaphore wait node \p hNode to \p nodeParams.

 \param hNode      - Node to set the parameters for
 \param nodeParams - Parameters to copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetParams,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuGraphExternalSemaphoresWaitNodeSetParams,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync*/
    fn cuGraphExternalSemaphoresWaitNodeSetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a batch memory operation node and adds it to a graph

 Creates a new batch memory operation node and adds it to \p hGraph with \p
 numDependencies dependencies specified via \p dependencies and arguments specified in \p nodeParams.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries.
 A handle to the new node will be returned in \p phGraphNode.

 When the node is added, the paramArray inside \p nodeParams is copied and therefore it can be
 freed after the call returns.

 \note
 Warning:
 Improper use of this API may deadlock the application. Synchronization
 ordering established through this API is not visible to CUDA. CUDA tasks
 that are (even indirectly) ordered by this API should also have that order
 expressed with CUDA-visible dependencies such as events. This ensures that
 the scheduler does not serialize them in an improper order. For more
 information, see the Stream Memory Operations section in the programming
 guide(https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html).

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param nodeParams      - Parameters for the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuStreamBatchMemOp,
 ::cuStreamWaitValue32,
 ::cuStreamWriteValue32,
 ::cuStreamWaitValue64,
 ::cuStreamWriteValue64,
 ::cuGraphBatchMemOpNodeGetParams,
 ::cuGraphBatchMemOpNodeSetParams,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddBatchMemOpNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        nodeParams: *const cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a batch mem op node's parameters

 Returns the parameters of batch mem op node \p hNode in \p nodeParams_out.
 The \p paramArray returned in \p nodeParams_out is owned by the node.
 This memory remains valid until the node is destroyed or its
 parameters are modified, and should not be modified
 directly. Use ::cuGraphBatchMemOpNodeSetParams to update the
 parameters of this node.

 \param hNode          - Node to get the parameters for
 \param nodeParams_out - Pointer to return the parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuStreamBatchMemOp,
 ::cuGraphAddBatchMemOpNode,
 ::cuGraphBatchMemOpNodeSetParams*/
    fn cuGraphBatchMemOpNodeGetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams_out: *mut cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets a batch mem op node's parameters

 Sets the parameters of batch mem op node \p hNode to \p nodeParams.

 The paramArray inside \p nodeParams is copied and therefore it can be
 freed after the call returns.

 \param hNode      - Node to set the parameters for
 \param nodeParams - Parameters to copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetParams,
 ::cuStreamBatchMemOp,
 ::cuGraphAddBatchMemOpNode,
 ::cuGraphBatchMemOpNodeGetParams*/
    fn cuGraphBatchMemOpNodeSetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the parameters for a batch mem op node in the given graphExec

 Sets the parameters of a batch mem op node in an executable graph \p hGraphExec.
 The node is identified by the corresponding node \p hNode in the
 non-executable graph, from which the executable graph was instantiated.

 The following fields on operations may be modified on an executable graph:

  op.waitValue.address
  op.waitValue.value[64]
  op.waitValue.flags bits corresponding to wait type (i.e. CU_STREAM_WAIT_VALUE_FLUSH bit cannot be modified)
  op.writeValue.address
  op.writeValue.value[64]

 Other fields, such as the context, count or type of operations, and other types of operations such as membars,
 may not be modified.

 \p hNode must not have been removed from the original graph.

 The modifications only affect future launches of \p hGraphExec. Already
 enqueued or running launches of \p hGraphExec are not affected by this call.
 \p hNode is also not modified by this call.

 The paramArray inside \p nodeParams is copied and therefore it can be
 freed after the call returns.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - Batch mem op node from the graph from which graphExec was instantiated
 \param nodeParams - Updated Parameters to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuStreamBatchMemOp,
 ::cuGraphAddBatchMemOpNode,
 ::cuGraphBatchMemOpNodeGetParams,
 ::cuGraphBatchMemOpNodeSetParams,
 ::cuGraphInstantiate*/
    fn cuGraphExecBatchMemOpNodeSetParams(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an allocation node and adds it to a graph

 Creates a new allocation node and adds it to \p hGraph with \p numDependencies
 dependencies specified via \p dependencies and arguments specified in \p nodeParams.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries. A handle
 to the new node will be returned in \p phGraphNode.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param nodeParams      - Parameters for the node

 When ::cuGraphAddMemAllocNode creates an allocation node, it returns the address of the allocation in
 \p nodeParams.dptr.  The allocation's address remains fixed across instantiations and launches.

 If the allocation is freed in the same graph, by creating a free node using ::cuGraphAddMemFreeNode,
 the allocation can be accessed by nodes ordered after the allocation node but before the free node.
 These allocations cannot be freed outside the owning graph, and they can only be freed once in the
 owning graph.

 If the allocation is not freed in the same graph, then it can be accessed not only by nodes in the
 graph which are ordered after the allocation node, but also by stream operations ordered after the
 graph's execution but before the allocation is freed.

 Allocations which are not freed in the same graph can be freed by:
 - passing the allocation to ::cuMemFreeAsync or ::cuMemFree;
 - launching a graph with a free node for that allocation; or
 - specifying ::CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH during instantiation, which makes
 each launch behave as though it called ::cuMemFreeAsync for every unfreed allocation.

 It is not possible to free an allocation in both the owning graph and another graph.  If the allocation
 is freed in the same graph, a free node cannot be added to another graph.  If the allocation is freed
 in another graph, a free node can no longer be added to the owning graph.

 The following restrictions apply to graphs which contain allocation and/or memory free nodes:
 - Nodes and edges of the graph cannot be deleted.
 - The graph can only be used in a child node if the ownership is moved to the parent.
 - Only one instantiation of the graph may exist at any point in time.
 - The graph cannot be cloned.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphAddMemFreeNode,
 ::cuGraphMemAllocNodeGetParams,
 ::cuDeviceGraphMemTrim,
 ::cuDeviceGetGraphMemAttribute,
 ::cuDeviceSetGraphMemAttribute,
 ::cuMemAllocAsync,
 ::cuMemFreeAsync,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddEventRecordNode,
 ::cuGraphAddEventWaitNode,
 ::cuGraphAddExternalSemaphoresSignalNode,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddMemAllocNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        nodeParams: *mut cuda_types::cuda::CUDA_MEM_ALLOC_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a memory alloc node's parameters

 Returns the parameters of a memory alloc node \p hNode in \p params_out.
 The \p poolProps and \p accessDescs returned in \p params_out, are owned by the
 node.  This memory remains valid until the node is destroyed.  The returned
 parameters must not be modified.

 \param hNode      - Node to get the parameters for
 \param params_out - Pointer to return the parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddMemAllocNode,
 ::cuGraphMemFreeNodeGetParams*/
    fn cuGraphMemAllocNodeGetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        params_out: *mut cuda_types::cuda::CUDA_MEM_ALLOC_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a memory free node and adds it to a graph

 Creates a new memory free node and adds it to \p hGraph with \p numDependencies
 dependencies specified via \p dependencies and arguments specified in \p nodeParams.
 It is possible for \p numDependencies to be 0, in which case the node will be placed
 at the root of the graph. \p dependencies may not have any duplicate entries. A handle
 to the new node will be returned in \p phGraphNode.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param dptr            - Address of memory to free

 ::cuGraphAddMemFreeNode will return ::CUDA_ERROR_INVALID_VALUE if the user attempts to free:
 - an allocation twice in the same graph.
 - an address that was not returned by an allocation node.
 - an invalid address.

 The following restrictions apply to graphs which contain allocation and/or memory free nodes:
 - Nodes and edges of the graph cannot be deleted.
 - The graph can only be used in a child node if the ownership is moved to the parent.
 - Only one instantiation of the graph may exist at any point in time.
 - The graph cannot be cloned.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphAddMemAllocNode,
 ::cuGraphMemFreeNodeGetParams,
 ::cuDeviceGraphMemTrim,
 ::cuDeviceGetGraphMemAttribute,
 ::cuDeviceSetGraphMemAttribute,
 ::cuMemAllocAsync,
 ::cuMemFreeAsync,
 ::cuGraphCreate,
 ::cuGraphDestroyNode,
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddEventRecordNode,
 ::cuGraphAddEventWaitNode,
 ::cuGraphAddExternalSemaphoresSignalNode,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphAddMemFreeNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        dptr: cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a memory free node's parameters

 Returns the address of a memory free node \p hNode in \p dptr_out.

 \param hNode    - Node to get the parameters for
 \param dptr_out - Pointer to return the device address

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddMemFreeNode,
 ::cuGraphMemAllocNodeGetParams*/
    fn cuGraphMemFreeNodeGetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        dptr_out: *mut cuda_types::cuda::CUdeviceptr,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Free unused memory that was cached on the specified device for use with graphs back to the OS.

 Blocks which are not in use by a graph that is either currently executing or scheduled to execute are
 freed back to the operating system.

 \param device - The device for which cached memory should be freed.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_DEVICE

 \sa
 ::cuGraphAddMemAllocNode,
 ::cuGraphAddMemFreeNode,
 ::cuDeviceSetGraphMemAttribute,
 ::cuDeviceGetGraphMemAttribute*/
    fn cuDeviceGraphMemTrim(
        device: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query asynchronous allocation attributes related to graphs

 Valid attributes are:

 - ::CU_GRAPH_MEM_ATTR_USED_MEM_CURRENT: Amount of memory, in bytes, currently associated with graphs
 - ::CU_GRAPH_MEM_ATTR_USED_MEM_HIGH: High watermark of memory, in bytes, associated with graphs since the
   last time it was reset.  High watermark can only be reset to zero.
 - ::CU_GRAPH_MEM_ATTR_RESERVED_MEM_CURRENT: Amount of memory, in bytes, currently allocated for use by
   the CUDA graphs asynchronous allocator.
 - ::CU_GRAPH_MEM_ATTR_RESERVED_MEM_HIGH: High watermark of memory, in bytes, currently allocated for use by
   the CUDA graphs asynchronous allocator.

 \param device - Specifies the scope of the query
 \param attr - attribute to get
 \param value - retrieved value

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_DEVICE

 \sa
 ::cuDeviceSetGraphMemAttribute,
 ::cuGraphAddMemAllocNode,
 ::cuGraphAddMemFreeNode*/
    fn cuDeviceGetGraphMemAttribute(
        device: cuda_types::cuda::CUdevice,
        attr: cuda_types::cuda::CUgraphMem_attribute,
        value: *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Set asynchronous allocation attributes related to graphs

 Valid attributes are:

 - ::CU_GRAPH_MEM_ATTR_USED_MEM_HIGH: High watermark of memory, in bytes, associated with graphs since the
   last time it was reset.  High watermark can only be reset to zero.
 - ::CU_GRAPH_MEM_ATTR_RESERVED_MEM_HIGH: High watermark of memory, in bytes, currently allocated for use by
   the CUDA graphs asynchronous allocator.

 \param device - Specifies the scope of the query
 \param attr - attribute to get
 \param value - pointer to value to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_DEVICE

 \sa
 ::cuDeviceGetGraphMemAttribute,
 ::cuGraphAddMemAllocNode,
 ::cuGraphAddMemFreeNode*/
    fn cuDeviceSetGraphMemAttribute(
        device: cuda_types::cuda::CUdevice,
        attr: cuda_types::cuda::CUgraphMem_attribute,
        value: *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Clones a graph

 This function creates a copy of \p originalGraph and returns it in \p phGraphClone.
 All parameters are copied into the cloned graph. The original graph may be modified
 after this call without affecting the clone.

 Child graph nodes in the original graph are recursively copied into the clone.

 \note: Cloning is not supported for graphs which contain memory allocation nodes,
        memory free nodes, or conditional nodes.

 \param phGraphClone  - Returns newly created cloned graph
 \param originalGraph - Graph to clone

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphCreate,
 ::cuGraphNodeFindInClone*/
    fn cuGraphClone(
        phGraphClone: *mut cuda_types::cuda::CUgraph,
        originalGraph: cuda_types::cuda::CUgraph,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Finds a cloned version of a node

 This function returns the node in \p hClonedGraph corresponding to \p hOriginalNode
 in the original graph.

 \p hClonedGraph must have been cloned from \p hOriginalGraph via ::cuGraphClone.
 \p hOriginalNode must have been in \p hOriginalGraph at the time of the call to
 ::cuGraphClone, and the corresponding cloned node in \p hClonedGraph must not have
 been removed. The cloned node is then returned via \p phClonedNode.

 \param phNode  - Returns handle to the cloned node
 \param hOriginalNode - Handle to the original node
 \param hClonedGraph - Cloned graph to query

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphClone*/
    fn cuGraphNodeFindInClone(
        phNode: *mut cuda_types::cuda::CUgraphNode,
        hOriginalNode: cuda_types::cuda::CUgraphNode,
        hClonedGraph: cuda_types::cuda::CUgraph,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a node's type

 Returns the node type of \p hNode in \p type.

 \param hNode - Node to query
 \param type  - Pointer to return the node type

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphGetNodes,
 ::cuGraphGetRootNodes,
 ::cuGraphChildGraphNodeGetGraph,
 ::cuGraphKernelNodeGetParams,
 ::cuGraphKernelNodeSetParams,
 ::cuGraphHostNodeGetParams,
 ::cuGraphHostNodeSetParams,
 ::cuGraphMemcpyNodeGetParams,
 ::cuGraphMemcpyNodeSetParams,
 ::cuGraphMemsetNodeGetParams,
 ::cuGraphMemsetNodeSetParams*/
    fn cuGraphNodeGetType(
        hNode: cuda_types::cuda::CUgraphNode,
        type_: *mut cuda_types::cuda::CUgraphNodeType,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a graph's nodes

 Returns a list of \p hGraph's nodes. \p nodes may be NULL, in which case this
 function will return the number of nodes in \p numNodes. Otherwise,
 \p numNodes entries will be filled in. If \p numNodes is higher than the actual
 number of nodes, the remaining entries in \p nodes will be set to NULL, and the
 number of nodes actually obtained will be returned in \p numNodes.

 \param hGraph   - Graph to query
 \param nodes    - Pointer to return the nodes
 \param numNodes - See description

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphCreate,
 ::cuGraphGetRootNodes,
 ::cuGraphGetEdges,
 ::cuGraphNodeGetType,
 ::cuGraphNodeGetDependencies,
 ::cuGraphNodeGetDependentNodes*/
    fn cuGraphGetNodes(
        hGraph: cuda_types::cuda::CUgraph,
        nodes: *mut cuda_types::cuda::CUgraphNode,
        numNodes: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a graph's root nodes

 Returns a list of \p hGraph's root nodes. \p rootNodes may be NULL, in which case this
 function will return the number of root nodes in \p numRootNodes. Otherwise,
 \p numRootNodes entries will be filled in. If \p numRootNodes is higher than the actual
 number of root nodes, the remaining entries in \p rootNodes will be set to NULL, and the
 number of nodes actually obtained will be returned in \p numRootNodes.

 \param hGraph       - Graph to query
 \param rootNodes    - Pointer to return the root nodes
 \param numRootNodes - See description

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphCreate,
 ::cuGraphGetNodes,
 ::cuGraphGetEdges,
 ::cuGraphNodeGetType,
 ::cuGraphNodeGetDependencies,
 ::cuGraphNodeGetDependentNodes*/
    fn cuGraphGetRootNodes(
        hGraph: cuda_types::cuda::CUgraph,
        rootNodes: *mut cuda_types::cuda::CUgraphNode,
        numRootNodes: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a graph's dependency edges

 Returns a list of \p hGraph's dependency edges. Edges are returned via corresponding
 indices in \p from and \p to; that is, the node in \p to[i] has a dependency on the
 node in \p from[i]. \p from and \p to may both be NULL, in which
 case this function only returns the number of edges in \p numEdges. Otherwise,
 \p numEdges entries will be filled in. If \p numEdges is higher than the actual
 number of edges, the remaining entries in \p from and \p to will be set to NULL, and
 the number of edges actually returned will be written to \p numEdges.

 \param hGraph   - Graph to get the edges from
 \param from     - Location to return edge endpoints
 \param to       - Location to return edge endpoints
 \param numEdges - See description

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphGetNodes,
 ::cuGraphGetRootNodes,
 ::cuGraphAddDependencies,
 ::cuGraphRemoveDependencies,
 ::cuGraphNodeGetDependencies,
 ::cuGraphNodeGetDependentNodes*/
    fn cuGraphGetEdges(
        hGraph: cuda_types::cuda::CUgraph,
        from: *mut cuda_types::cuda::CUgraphNode,
        to: *mut cuda_types::cuda::CUgraphNode,
        numEdges: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a graph's dependency edges (12.3+)

 Returns a list of \p hGraph's dependency edges. Edges are returned via corresponding
 indices in \p from, \p to and \p edgeData; that is, the node in \p to[i] has a
 dependency on the node in \p from[i] with data \p edgeData[i]. \p from and \p to may
 both be NULL, in which case this function only returns the number of edges in
 \p numEdges. Otherwise, \p numEdges entries will be filled in. If \p numEdges is higher
 than the actual number of edges, the remaining entries in \p from and \p to will be
 set to NULL, and the number of edges actually returned will be written to \p numEdges.
 \p edgeData may alone be NULL, in which case the edges must all have default (zeroed)
 edge data. Attempting a lossy query via NULL \p edgeData will result in
 ::CUDA_ERROR_LOSSY_QUERY. If \p edgeData is non-NULL then \p from and \p to must be
 as well.

 \param hGraph   - Graph to get the edges from
 \param from     - Location to return edge endpoints
 \param to       - Location to return edge endpoints
 \param edgeData - Optional location to return edge data
 \param numEdges - See description

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_LOSSY_QUERY,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphGetNodes,
 ::cuGraphGetRootNodes,
 ::cuGraphAddDependencies,
 ::cuGraphRemoveDependencies,
 ::cuGraphNodeGetDependencies,
 ::cuGraphNodeGetDependentNodes*/
    fn cuGraphGetEdges_v2(
        hGraph: cuda_types::cuda::CUgraph,
        from: *mut cuda_types::cuda::CUgraphNode,
        to: *mut cuda_types::cuda::CUgraphNode,
        edgeData: *mut cuda_types::cuda::CUgraphEdgeData,
        numEdges: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a node's dependencies

 Returns a list of \p node's dependencies. \p dependencies may be NULL, in which case this
 function will return the number of dependencies in \p numDependencies. Otherwise,
 \p numDependencies entries will be filled in. If \p numDependencies is higher than the actual
 number of dependencies, the remaining entries in \p dependencies will be set to NULL, and the
 number of nodes actually obtained will be returned in \p numDependencies.

 \param hNode           - Node to query
 \param dependencies    - Pointer to return the dependencies
 \param numDependencies - See description

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeGetDependentNodes,
 ::cuGraphGetNodes,
 ::cuGraphGetRootNodes,
 ::cuGraphGetEdges,
 ::cuGraphAddDependencies,
 ::cuGraphRemoveDependencies*/
    fn cuGraphNodeGetDependencies(
        hNode: cuda_types::cuda::CUgraphNode,
        dependencies: *mut cuda_types::cuda::CUgraphNode,
        numDependencies: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a node's dependencies (12.3+)

 Returns a list of \p node's dependencies. \p dependencies may be NULL, in which case this
 function will return the number of dependencies in \p numDependencies. Otherwise,
 \p numDependencies entries will be filled in. If \p numDependencies is higher than the actual
 number of dependencies, the remaining entries in \p dependencies will be set to NULL, and the
 number of nodes actually obtained will be returned in \p numDependencies.

 Note that if an edge has non-zero (non-default) edge data and \p edgeData is NULL,
 this API will return ::CUDA_ERROR_LOSSY_QUERY. If \p edgeData is non-NULL, then
 \p dependencies must be as well.

 \param hNode           - Node to query
 \param dependencies    - Pointer to return the dependencies
 \param edgeData        - Optional array to return edge data for each dependency
 \param numDependencies - See description

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_LOSSY_QUERY,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeGetDependentNodes,
 ::cuGraphGetNodes,
 ::cuGraphGetRootNodes,
 ::cuGraphGetEdges,
 ::cuGraphAddDependencies,
 ::cuGraphRemoveDependencies*/
    fn cuGraphNodeGetDependencies_v2(
        hNode: cuda_types::cuda::CUgraphNode,
        dependencies: *mut cuda_types::cuda::CUgraphNode,
        edgeData: *mut cuda_types::cuda::CUgraphEdgeData,
        numDependencies: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a node's dependent nodes

 Returns a list of \p node's dependent nodes. \p dependentNodes may be NULL, in which
 case this function will return the number of dependent nodes in \p numDependentNodes.
 Otherwise, \p numDependentNodes entries will be filled in. If \p numDependentNodes is
 higher than the actual number of dependent nodes, the remaining entries in
 \p dependentNodes will be set to NULL, and the number of nodes actually obtained will
 be returned in \p numDependentNodes.

 \param hNode             - Node to query
 \param dependentNodes    - Pointer to return the dependent nodes
 \param numDependentNodes - See description

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeGetDependencies,
 ::cuGraphGetNodes,
 ::cuGraphGetRootNodes,
 ::cuGraphGetEdges,
 ::cuGraphAddDependencies,
 ::cuGraphRemoveDependencies*/
    fn cuGraphNodeGetDependentNodes(
        hNode: cuda_types::cuda::CUgraphNode,
        dependentNodes: *mut cuda_types::cuda::CUgraphNode,
        numDependentNodes: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a node's dependent nodes (12.3+)

 Returns a list of \p node's dependent nodes. \p dependentNodes may be NULL, in which
 case this function will return the number of dependent nodes in \p numDependentNodes.
 Otherwise, \p numDependentNodes entries will be filled in. If \p numDependentNodes is
 higher than the actual number of dependent nodes, the remaining entries in
 \p dependentNodes will be set to NULL, and the number of nodes actually obtained will
 be returned in \p numDependentNodes.

 Note that if an edge has non-zero (non-default) edge data and \p edgeData is NULL,
 this API will return ::CUDA_ERROR_LOSSY_QUERY.  If \p edgeData is non-NULL, then
 \p dependentNodes must be as well.

 \param hNode             - Node to query
 \param dependentNodes    - Pointer to return the dependent nodes
 \param edgeData          - Optional pointer to return edge data for dependent nodes
 \param numDependentNodes - See description

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_LOSSY_QUERY,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeGetDependencies,
 ::cuGraphGetNodes,
 ::cuGraphGetRootNodes,
 ::cuGraphGetEdges,
 ::cuGraphAddDependencies,
 ::cuGraphRemoveDependencies*/
    fn cuGraphNodeGetDependentNodes_v2(
        hNode: cuda_types::cuda::CUgraphNode,
        dependentNodes: *mut cuda_types::cuda::CUgraphNode,
        edgeData: *mut cuda_types::cuda::CUgraphEdgeData,
        numDependentNodes: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Adds dependency edges to a graph

 The number of dependencies to be added is defined by \p numDependencies
 Elements in \p from and \p to at corresponding indices define a dependency.
 Each node in \p from and \p to must belong to \p hGraph.

 If \p numDependencies is 0, elements in \p from and \p to will be ignored.
 Specifying an existing dependency will return an error.

 \param hGraph - Graph to which dependencies are added
 \param from - Array of nodes that provide the dependencies
 \param to - Array of dependent nodes
 \param numDependencies - Number of dependencies to be added

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphRemoveDependencies,
 ::cuGraphGetEdges,
 ::cuGraphNodeGetDependencies,
 ::cuGraphNodeGetDependentNodes*/
    fn cuGraphAddDependencies(
        hGraph: cuda_types::cuda::CUgraph,
        from: *const cuda_types::cuda::CUgraphNode,
        to: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Adds dependency edges to a graph (12.3+)

 The number of dependencies to be added is defined by \p numDependencies
 Elements in \p from and \p to at corresponding indices define a dependency.
 Each node in \p from and \p to must belong to \p hGraph.

 If \p numDependencies is 0, elements in \p from and \p to will be ignored.
 Specifying an existing dependency will return an error.

 \param hGraph - Graph to which dependencies are added
 \param from - Array of nodes that provide the dependencies
 \param to - Array of dependent nodes
 \param edgeData - Optional array of edge data. If NULL, default (zeroed) edge data is assumed.
 \param numDependencies - Number of dependencies to be added

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphRemoveDependencies,
 ::cuGraphGetEdges,
 ::cuGraphNodeGetDependencies,
 ::cuGraphNodeGetDependentNodes*/
    fn cuGraphAddDependencies_v2(
        hGraph: cuda_types::cuda::CUgraph,
        from: *const cuda_types::cuda::CUgraphNode,
        to: *const cuda_types::cuda::CUgraphNode,
        edgeData: *const cuda_types::cuda::CUgraphEdgeData,
        numDependencies: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Removes dependency edges from a graph

 The number of \p dependencies to be removed is defined by \p numDependencies.
 Elements in \p from and \p to at corresponding indices define a dependency.
 Each node in \p from and \p to must belong to \p hGraph.

 If \p numDependencies is 0, elements in \p from and \p to will be ignored.
 Specifying a non-existing dependency will return an error.

 Dependencies cannot be removed from graphs which contain allocation or free nodes.
 Any attempt to do so will return an error.

 \param hGraph - Graph from which to remove dependencies
 \param from - Array of nodes that provide the dependencies
 \param to - Array of dependent nodes
 \param numDependencies - Number of dependencies to be removed

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddDependencies,
 ::cuGraphGetEdges,
 ::cuGraphNodeGetDependencies,
 ::cuGraphNodeGetDependentNodes*/
    fn cuGraphRemoveDependencies(
        hGraph: cuda_types::cuda::CUgraph,
        from: *const cuda_types::cuda::CUgraphNode,
        to: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Removes dependency edges from a graph (12.3+)

 The number of \p dependencies to be removed is defined by \p numDependencies.
 Elements in \p from and \p to at corresponding indices define a dependency.
 Each node in \p from and \p to must belong to \p hGraph.

 If \p numDependencies is 0, elements in \p from and \p to will be ignored.
 Specifying an edge that does not exist in the graph, with data matching
 \p edgeData, results in an error. \p edgeData is nullable, which is equivalent
 to passing default (zeroed) data for each edge.

 Dependencies cannot be removed from graphs which contain allocation or free nodes.
 Any attempt to do so will return an error.

 \param hGraph - Graph from which to remove dependencies
 \param from - Array of nodes that provide the dependencies
 \param to - Array of dependent nodes
 \param edgeData - Optional array of edge data. If NULL, edge data is assumed to
                   be default (zeroed).
 \param numDependencies - Number of dependencies to be removed

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddDependencies,
 ::cuGraphGetEdges,
 ::cuGraphNodeGetDependencies,
 ::cuGraphNodeGetDependentNodes*/
    fn cuGraphRemoveDependencies_v2(
        hGraph: cuda_types::cuda::CUgraph,
        from: *const cuda_types::cuda::CUgraphNode,
        to: *const cuda_types::cuda::CUgraphNode,
        edgeData: *const cuda_types::cuda::CUgraphEdgeData,
        numDependencies: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Remove a node from the graph

 Removes \p hNode from its graph. This operation also severs any dependencies of other nodes
 on \p hNode and vice versa.

 Nodes which belong to a graph which contains allocation or free nodes cannot be destroyed.
 Any attempt to do so will return an error.

 \param hNode  - Node to remove

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddChildGraphNode,
 ::cuGraphAddEmptyNode,
 ::cuGraphAddKernelNode,
 ::cuGraphAddHostNode,
 ::cuGraphAddMemcpyNode,
 ::cuGraphAddMemsetNode*/
    fn cuGraphDestroyNode(
        hNode: cuda_types::cuda::CUgraphNode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an executable graph from a graph

 Instantiates \p hGraph as an executable graph. The graph is validated for any
 structural constraints or intra-node constraints which were not previously
 validated. If instantiation is successful, a handle to the instantiated graph
 is returned in \p phGraphExec.

 The \p flags parameter controls the behavior of instantiation and subsequent
 graph launches.  Valid flags are:

 - ::CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH, which configures a
 graph containing memory allocation nodes to automatically free any
 unfreed memory allocations before the graph is relaunched.

 - ::CUDA_GRAPH_INSTANTIATE_FLAG_DEVICE_LAUNCH, which configures the graph for launch
 from the device. If this flag is passed, the executable graph handle returned can be
 used to launch the graph from both the host and device. This flag can only be used
 on platforms which support unified addressing. This flag cannot be used in
 conjunction with ::CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH.

 - ::CUDA_GRAPH_INSTANTIATE_FLAG_USE_NODE_PRIORITY, which causes the graph
 to use the priorities from the per-node attributes rather than the priority
 of the launch stream during execution. Note that priorities are only available
 on kernel nodes, and are copied from stream priority during stream capture.

 If \p hGraph contains any allocation or free nodes, there can be at most one
 executable graph in existence for that graph at a time. An attempt to instantiate
 a second executable graph before destroying the first with ::cuGraphExecDestroy
 will result in an error.
 The same also applies if \p hGraph contains any device-updatable kernel nodes.

 If \p hGraph contains kernels which call device-side cudaGraphLaunch() from multiple
 contexts, this will result in an error.

 Graphs instantiated for launch on the device have additional restrictions which do not
 apply to host graphs:

 - The graph's nodes must reside on a single context.
 - The graph can only contain kernel nodes, memcpy nodes, memset nodes, and child graph nodes.
 - The graph cannot be empty and must contain at least one kernel, memcpy, or memset node.
   Operation-specific restrictions are outlined below.
 - Kernel nodes:
   - Use of CUDA Dynamic Parallelism is not permitted.
   - Cooperative launches are permitted as long as MPS is not in use.
 - Memcpy nodes:
   - Only copies involving device memory and/or pinned device-mapped host memory are permitted.
   - Copies involving CUDA arrays are not permitted.
   - Both operands must be accessible from the current context, and the current context must
     match the context of other nodes in the graph.

 \param phGraphExec - Returns instantiated graph
 \param hGraph      - Graph to instantiate
 \param flags       - Flags to control instantiation.  See ::CUgraphInstantiate_flags.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphInstantiate,
 ::cuGraphCreate,
 ::cuGraphUpload,
 ::cuGraphLaunch,
 ::cuGraphExecDestroy*/
    fn cuGraphInstantiateWithFlags(
        phGraphExec: *mut cuda_types::cuda::CUgraphExec,
        hGraph: cuda_types::cuda::CUgraph,
        flags: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an executable graph from a graph

 Instantiates \p hGraph as an executable graph according to the \p instantiateParams structure.
 The graph is validated for any structural constraints or intra-node constraints
 which were not previously validated. If instantiation is successful, a handle to
 the instantiated graph is returned in \p phGraphExec.

 \p instantiateParams controls the behavior of instantiation and subsequent
 graph launches, as well as returning more detailed information in the event of an error.
 ::CUDA_GRAPH_INSTANTIATE_PARAMS is defined as:

 \code
typedef struct {
cuuint64_t flags;
CUstream hUploadStream;
CUgraphNode hErrNode_out;
CUgraphInstantiateResult result_out;
} CUDA_GRAPH_INSTANTIATE_PARAMS;
 \endcode

 The \p flags field controls the behavior of instantiation and subsequent
 graph launches. Valid flags are:

 - ::CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH, which configures a
 graph containing memory allocation nodes to automatically free any
 unfreed memory allocations before the graph is relaunched.

 - ::CUDA_GRAPH_INSTANTIATE_FLAG_UPLOAD, which will perform an upload of the graph
 into \p hUploadStream once the graph has been instantiated.

 - ::CUDA_GRAPH_INSTANTIATE_FLAG_DEVICE_LAUNCH, which configures the graph for launch
 from the device. If this flag is passed, the executable graph handle returned can be
 used to launch the graph from both the host and device. This flag can only be used
 on platforms which support unified addressing. This flag cannot be used in
 conjunction with ::CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH.

 - ::CUDA_GRAPH_INSTANTIATE_FLAG_USE_NODE_PRIORITY, which causes the graph
 to use the priorities from the per-node attributes rather than the priority
 of the launch stream during execution. Note that priorities are only available
 on kernel nodes, and are copied from stream priority during stream capture.

 If \p hGraph contains any allocation or free nodes, there can be at most one
 executable graph in existence for that graph at a time. An attempt to instantiate a
 second executable graph before destroying the first with ::cuGraphExecDestroy will
 result in an error.
 The same also applies if \p hGraph contains any device-updatable kernel nodes.

 If \p hGraph contains kernels which call device-side cudaGraphLaunch() from multiple
 contexts, this will result in an error.

 Graphs instantiated for launch on the device have additional restrictions which do not
 apply to host graphs:

 - The graph's nodes must reside on a single context.
 - The graph can only contain kernel nodes, memcpy nodes, memset nodes, and child graph nodes.
 - The graph cannot be empty and must contain at least one kernel, memcpy, or memset node.
   Operation-specific restrictions are outlined below.
 - Kernel nodes:
   - Use of CUDA Dynamic Parallelism is not permitted.
   - Cooperative launches are permitted as long as MPS is not in use.
 - Memcpy nodes:
   - Only copies involving device memory and/or pinned device-mapped host memory are permitted.
   - Copies involving CUDA arrays are not permitted.
   - Both operands must be accessible from the current context, and the current context must
     match the context of other nodes in the graph.

 In the event of an error, the \p result_out and \p hErrNode_out fields will contain more
 information about the nature of the error. Possible error reporting includes:

 - ::CUDA_GRAPH_INSTANTIATE_ERROR, if passed an invalid value or if an unexpected error occurred
   which is described by the return value of the function. \p hErrNode_out will be set to NULL.
 - ::CUDA_GRAPH_INSTANTIATE_INVALID_STRUCTURE, if the graph structure is invalid. \p hErrNode_out
   will be set to one of the offending nodes.
 - ::CUDA_GRAPH_INSTANTIATE_NODE_OPERATION_NOT_SUPPORTED, if the graph is instantiated for device
   launch but contains a node of an unsupported node type, or a node which performs unsupported
   operations, such as use of CUDA dynamic parallelism within a kernel node. \p hErrNode_out will
   be set to this node.
 - ::CUDA_GRAPH_INSTANTIATE_MULTIPLE_CTXS_NOT_SUPPORTED, if the graph is instantiated for device
   launch but a node’s context differs from that of another node. This error can also be returned
   if a graph is not instantiated for device launch and it contains kernels which call device-side
   cudaGraphLaunch() from multiple contexts. \p hErrNode_out will be set to this node.

 If instantiation is successful, \p result_out will be set to ::CUDA_GRAPH_INSTANTIATE_SUCCESS,
 and \p hErrNode_out will be set to NULL.

 \param phGraphExec       - Returns instantiated graph
 \param hGraph            - Graph to instantiate
 \param instantiateParams - Instantiation parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphCreate,
 ::cuGraphInstantiate,
 ::cuGraphExecDestroy*/
    fn cuGraphInstantiateWithParams_ptsz(
        phGraphExec: *mut cuda_types::cuda::CUgraphExec,
        hGraph: cuda_types::cuda::CUgraph,
        instantiateParams: *mut cuda_types::cuda::CUDA_GRAPH_INSTANTIATE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query the instantiation flags of an executable graph

 Returns the flags that were passed to instantiation for the given executable graph.
 ::CUDA_GRAPH_INSTANTIATE_FLAG_UPLOAD will not be returned by this API as it does
 not affect the resulting executable graph.

 \param hGraphExec - The executable graph to query
 \param flags      - Returns the instantiation flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphInstantiate,
 ::cuGraphInstantiateWithParams*/
    fn cuGraphExecGetFlags(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        flags: *mut cuda_types::cuda::cuuint64_t,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the parameters for a kernel node in the given graphExec

 Sets the parameters of a kernel node in an executable graph \p hGraphExec.
 The node is identified by the corresponding node \p hNode in the
 non-executable graph, from which the executable graph was instantiated.

 \p hNode must not have been removed from the original graph. All \p nodeParams
 fields may change, but the following restrictions apply to \p func updates:

   - The owning context of the function cannot change.
   - A node whose function originally did not use CUDA dynamic parallelism cannot be updated
     to a function which uses CDP
   - A node whose function originally did not make device-side update calls cannot be updated
     to a function which makes device-side update calls.
   - If \p hGraphExec was not instantiated for device launch, a node whose function originally
     did not use device-side cudaGraphLaunch() cannot be updated to a function which uses
     device-side cudaGraphLaunch() unless the node resides on the same context as nodes which
     contained such calls at instantiate-time. If no such calls were present at instantiation,
     these updates cannot be performed at all.

 The modifications only affect future launches of \p hGraphExec. Already
 enqueued or running launches of \p hGraphExec are not affected by this call.
 \p hNode is also not modified by this call.

 If \p hNode is a device-updatable kernel node, the next upload/launch of \p hGraphExec
 will overwrite any previous device-side updates. Additionally, applying host updates to a
 device-updatable kernel node while it is being updated from the device will result in
 undefined behavior.

 \param hGraphExec  - The executable graph in which to set the specified node
 \param hNode       - kernel node from the graph from which graphExec was instantiated
 \param nodeParams  - Updated Parameters to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuGraphAddKernelNode,
 ::cuGraphKernelNodeSetParams,
 ::cuGraphExecMemcpyNodeSetParams,
 ::cuGraphExecMemsetNodeSetParams,
 ::cuGraphExecHostNodeSetParams,
 ::cuGraphExecChildGraphNodeSetParams,
 ::cuGraphExecEventRecordNodeSetEvent,
 ::cuGraphExecEventWaitNodeSetEvent,
 ::cuGraphExecExternalSemaphoresSignalNodeSetParams,
 ::cuGraphExecExternalSemaphoresWaitNodeSetParams,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecKernelNodeSetParams_v2(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the parameters for a memcpy node in the given graphExec.

 Updates the work represented by \p hNode in \p hGraphExec as though \p hNode had
 contained \p copyParams at instantiation.  hNode must remain in the graph which was
 used to instantiate \p hGraphExec.  Changed edges to and from hNode are ignored.

 The source and destination memory in \p copyParams must be allocated from the same
 contexts as the original source and destination memory.  Both the instantiation-time
 memory operands and the memory operands in \p copyParams must be 1-dimensional.
 Zero-length operations are not supported.

 The modifications only affect future launches of \p hGraphExec.  Already enqueued
 or running launches of \p hGraphExec are not affected by this call.  hNode is also
 not modified by this call.

 Returns CUDA_ERROR_INVALID_VALUE if the memory operands' mappings changed or
 either the original or new memory operands are multidimensional.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - Memcpy node from the graph which was used to instantiate graphExec
 \param copyParams - The updated parameters to set
 \param ctx        - Context on which to run the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuGraphAddMemcpyNode,
 ::cuGraphMemcpyNodeSetParams,
 ::cuGraphExecKernelNodeSetParams,
 ::cuGraphExecMemsetNodeSetParams,
 ::cuGraphExecHostNodeSetParams,
 ::cuGraphExecChildGraphNodeSetParams,
 ::cuGraphExecEventRecordNodeSetEvent,
 ::cuGraphExecEventWaitNodeSetEvent,
 ::cuGraphExecExternalSemaphoresSignalNodeSetParams,
 ::cuGraphExecExternalSemaphoresWaitNodeSetParams,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecMemcpyNodeSetParams(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        copyParams: *const cuda_types::cuda::CUDA_MEMCPY3D,
        ctx: cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the parameters for a memset node in the given graphExec.

 Updates the work represented by \p hNode in \p hGraphExec as though \p hNode had
 contained \p memsetParams at instantiation.  hNode must remain in the graph which was
 used to instantiate \p hGraphExec.  Changed edges to and from hNode are ignored.

 Zero sized operations are not supported.

 The new destination pointer in memsetParams must be to the same kind of allocation
 as the original destination pointer and have the same context association and device mapping
 as the original destination pointer.

 Both the value and pointer address may be updated.
 Changing other aspects of the memset (width, height, element size or pitch) may cause the update to be rejected.
 Specifically, for 2d memsets, all dimension changes are rejected.
 For 1d memsets, changes in height are explicitly rejected and other changes are opportunistically allowed
 if the resulting work maps onto the work resources already allocated for the node.

 The modifications only affect future launches of \p hGraphExec.  Already enqueued
 or running launches of \p hGraphExec are not affected by this call.  hNode is also
 not modified by this call.

 \param hGraphExec   - The executable graph in which to set the specified node
 \param hNode        - Memset node from the graph which was used to instantiate graphExec
 \param memsetParams - The updated parameters to set
 \param ctx          - Context on which to run the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuGraphAddMemsetNode,
 ::cuGraphMemsetNodeSetParams,
 ::cuGraphExecKernelNodeSetParams,
 ::cuGraphExecMemcpyNodeSetParams,
 ::cuGraphExecHostNodeSetParams,
 ::cuGraphExecChildGraphNodeSetParams,
 ::cuGraphExecEventRecordNodeSetEvent,
 ::cuGraphExecEventWaitNodeSetEvent,
 ::cuGraphExecExternalSemaphoresSignalNodeSetParams,
 ::cuGraphExecExternalSemaphoresWaitNodeSetParams,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecMemsetNodeSetParams(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        memsetParams: *const cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS,
        ctx: cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the parameters for a host node in the given graphExec.

 Updates the work represented by \p hNode in \p hGraphExec as though \p hNode had
 contained \p nodeParams at instantiation.  hNode must remain in the graph which was
 used to instantiate \p hGraphExec.  Changed edges to and from hNode are ignored.

 The modifications only affect future launches of \p hGraphExec.  Already enqueued
 or running launches of \p hGraphExec are not affected by this call.  hNode is also
 not modified by this call.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - Host node from the graph which was used to instantiate graphExec
 \param nodeParams - The updated parameters to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuGraphAddHostNode,
 ::cuGraphHostNodeSetParams,
 ::cuGraphExecKernelNodeSetParams,
 ::cuGraphExecMemcpyNodeSetParams,
 ::cuGraphExecMemsetNodeSetParams,
 ::cuGraphExecChildGraphNodeSetParams,
 ::cuGraphExecEventRecordNodeSetEvent,
 ::cuGraphExecEventWaitNodeSetEvent,
 ::cuGraphExecExternalSemaphoresSignalNodeSetParams,
 ::cuGraphExecExternalSemaphoresWaitNodeSetParams,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecHostNodeSetParams(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_HOST_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Updates node parameters in the child graph node in the given graphExec.

 Updates the work represented by \p hNode in \p hGraphExec as though the nodes contained
 in \p hNode's graph had the parameters contained in \p childGraph's nodes at instantiation.
 \p hNode must remain in the graph which was used to instantiate \p hGraphExec.
 Changed edges to and from \p hNode are ignored.

 The modifications only affect future launches of \p hGraphExec.  Already enqueued
 or running launches of \p hGraphExec are not affected by this call.  \p hNode is also
 not modified by this call.

 The topology of \p childGraph, as well as the node insertion order,  must match that
 of the graph contained in \p hNode.  See ::cuGraphExecUpdate() for a list of restrictions
 on what can be updated in an instantiated graph.  The update is recursive, so child graph
 nodes contained within the top level child graph will also be updated.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - Host node from the graph which was used to instantiate graphExec
 \param childGraph - The graph supplying the updated parameters

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuGraphAddChildGraphNode,
 ::cuGraphChildGraphNodeGetGraph,
 ::cuGraphExecKernelNodeSetParams,
 ::cuGraphExecMemcpyNodeSetParams,
 ::cuGraphExecMemsetNodeSetParams,
 ::cuGraphExecHostNodeSetParams,
 ::cuGraphExecEventRecordNodeSetEvent,
 ::cuGraphExecEventWaitNodeSetEvent,
 ::cuGraphExecExternalSemaphoresSignalNodeSetParams,
 ::cuGraphExecExternalSemaphoresWaitNodeSetParams,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecChildGraphNodeSetParams(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        childGraph: cuda_types::cuda::CUgraph,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the event for an event record node in the given graphExec

 Sets the event of an event record node in an executable graph \p hGraphExec.
 The node is identified by the corresponding node \p hNode in the
 non-executable graph, from which the executable graph was instantiated.

 The modifications only affect future launches of \p hGraphExec. Already
 enqueued or running launches of \p hGraphExec are not affected by this call.
 \p hNode is also not modified by this call.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - event record node from the graph from which graphExec was instantiated
 \param event      - Updated event to use

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuGraphAddEventRecordNode,
 ::cuGraphEventRecordNodeGetEvent,
 ::cuGraphEventWaitNodeSetEvent,
 ::cuEventRecordWithFlags,
 ::cuStreamWaitEvent,
 ::cuGraphExecKernelNodeSetParams,
 ::cuGraphExecMemcpyNodeSetParams,
 ::cuGraphExecMemsetNodeSetParams,
 ::cuGraphExecHostNodeSetParams,
 ::cuGraphExecChildGraphNodeSetParams,
 ::cuGraphExecEventWaitNodeSetEvent,
 ::cuGraphExecExternalSemaphoresSignalNodeSetParams,
 ::cuGraphExecExternalSemaphoresWaitNodeSetParams,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecEventRecordNodeSetEvent(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        event: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the event for an event wait node in the given graphExec

 Sets the event of an event wait node in an executable graph \p hGraphExec.
 The node is identified by the corresponding node \p hNode in the
 non-executable graph, from which the executable graph was instantiated.

 The modifications only affect future launches of \p hGraphExec. Already
 enqueued or running launches of \p hGraphExec are not affected by this call.
 \p hNode is also not modified by this call.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - event wait node from the graph from which graphExec was instantiated
 \param event      - Updated event to use

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuGraphAddEventWaitNode,
 ::cuGraphEventWaitNodeGetEvent,
 ::cuGraphEventRecordNodeSetEvent,
 ::cuEventRecordWithFlags,
 ::cuStreamWaitEvent,
 ::cuGraphExecKernelNodeSetParams,
 ::cuGraphExecMemcpyNodeSetParams,
 ::cuGraphExecMemsetNodeSetParams,
 ::cuGraphExecHostNodeSetParams,
 ::cuGraphExecChildGraphNodeSetParams,
 ::cuGraphExecEventRecordNodeSetEvent,
 ::cuGraphExecExternalSemaphoresSignalNodeSetParams,
 ::cuGraphExecExternalSemaphoresWaitNodeSetParams,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecEventWaitNodeSetEvent(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        event: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the parameters for an external semaphore signal node in the given graphExec

 Sets the parameters of an external semaphore signal node in an executable graph \p hGraphExec.
 The node is identified by the corresponding node \p hNode in the
 non-executable graph, from which the executable graph was instantiated.

 \p hNode must not have been removed from the original graph.

 The modifications only affect future launches of \p hGraphExec. Already
 enqueued or running launches of \p hGraphExec are not affected by this call.
 \p hNode is also not modified by this call.

 Changing \p nodeParams->numExtSems is not supported.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - semaphore signal node from the graph from which graphExec was instantiated
 \param nodeParams - Updated Parameters to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuGraphAddExternalSemaphoresSignalNode,
 ::cuImportExternalSemaphore,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync,
 ::cuGraphExecKernelNodeSetParams,
 ::cuGraphExecMemcpyNodeSetParams,
 ::cuGraphExecMemsetNodeSetParams,
 ::cuGraphExecHostNodeSetParams,
 ::cuGraphExecChildGraphNodeSetParams,
 ::cuGraphExecEventRecordNodeSetEvent,
 ::cuGraphExecEventWaitNodeSetEvent,
 ::cuGraphExecExternalSemaphoresWaitNodeSetParams,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecExternalSemaphoresSignalNodeSetParams(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the parameters for an external semaphore wait node in the given graphExec

 Sets the parameters of an external semaphore wait node in an executable graph \p hGraphExec.
 The node is identified by the corresponding node \p hNode in the
 non-executable graph, from which the executable graph was instantiated.

 \p hNode must not have been removed from the original graph.

 The modifications only affect future launches of \p hGraphExec. Already
 enqueued or running launches of \p hGraphExec are not affected by this call.
 \p hNode is also not modified by this call.

 Changing \p nodeParams->numExtSems is not supported.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - semaphore wait node from the graph from which graphExec was instantiated
 \param nodeParams - Updated Parameters to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphExecNodeSetParams,
 ::cuGraphAddExternalSemaphoresWaitNode,
 ::cuImportExternalSemaphore,
 ::cuSignalExternalSemaphoresAsync,
 ::cuWaitExternalSemaphoresAsync,
 ::cuGraphExecKernelNodeSetParams,
 ::cuGraphExecMemcpyNodeSetParams,
 ::cuGraphExecMemsetNodeSetParams,
 ::cuGraphExecHostNodeSetParams,
 ::cuGraphExecChildGraphNodeSetParams,
 ::cuGraphExecEventRecordNodeSetEvent,
 ::cuGraphExecEventWaitNodeSetEvent,
 ::cuGraphExecExternalSemaphoresSignalNodeSetParams,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecExternalSemaphoresWaitNodeSetParams(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Enables or disables the specified node in the given graphExec

 Sets \p hNode to be either enabled or disabled. Disabled nodes are functionally equivalent
 to empty nodes until they are reenabled. Existing node parameters are not affected by
 disabling/enabling the node.

 The node is identified by the corresponding node \p hNode in the non-executable
 graph, from which the executable graph was instantiated.

 \p hNode must not have been removed from the original graph.

 The modifications only affect future launches of \p hGraphExec. Already
 enqueued or running launches of \p hGraphExec are not affected by this call.
 \p hNode is also not modified by this call.

 If \p hNode is a device-updatable kernel node, the next upload/launch of \p hGraphExec
 will overwrite any previous device-side updates. Additionally, applying host updates to a
 device-updatable kernel node while it is being updated from the device will result in
 undefined behavior.

 \note Currently only kernel, memset and memcpy nodes are supported.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - Node from the graph from which graphExec was instantiated
 \param isEnabled  - Node is enabled if != 0, otherwise the node is disabled

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeGetEnabled,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate
 ::cuGraphLaunch*/
    fn cuGraphNodeSetEnabled(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        isEnabled: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query whether a node in the given graphExec is enabled

 Sets isEnabled to 1 if \p hNode is enabled, or 0 if \p hNode is disabled.

 The node is identified by the corresponding node \p hNode in the non-executable
 graph, from which the executable graph was instantiated.

 \p hNode must not have been removed from the original graph.

 \note Currently only kernel, memset and memcpy nodes are supported.
 \note This function will not reflect device-side updates for device-updatable kernel nodes.

 \param hGraphExec - The executable graph in which to set the specified node
 \param hNode      - Node from the graph from which graphExec was instantiated
 \param isEnabled  - Location to return the enabled status of the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphNodeSetEnabled,
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate
 ::cuGraphLaunch*/
    fn cuGraphNodeGetEnabled(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        isEnabled: *mut ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Uploads an executable graph in a stream

 Uploads \p hGraphExec to the device in \p hStream without executing it. Uploads of
 the same \p hGraphExec will be serialized. Each upload is ordered behind both any
 previous work in \p hStream and any previous launches of \p hGraphExec.
 Uses memory cached by \p stream to back the allocations owned by \p hGraphExec.

 \param hGraphExec - Executable graph to upload
 \param hStream    - Stream in which to upload the graph

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphInstantiate,
 ::cuGraphLaunch,
 ::cuGraphExecDestroy*/
    fn cuGraphUpload_ptsz(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Launches an executable graph in a stream

 Executes \p hGraphExec in \p hStream. Only one instance of \p hGraphExec may be executing
 at a time. Each launch is ordered behind both any previous work in \p hStream
 and any previous launches of \p hGraphExec. To execute a graph concurrently, it must be
 instantiated multiple times into multiple executable graphs.

 If any allocations created by \p hGraphExec remain unfreed (from a previous launch) and
 \p hGraphExec was not instantiated with ::CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH,
 the launch will fail with ::CUDA_ERROR_INVALID_VALUE.

 \param hGraphExec - Executable graph to launch
 \param hStream    - Stream in which to launch the graph

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphInstantiate,
 ::cuGraphUpload,
 ::cuGraphExecDestroy*/
    fn cuGraphLaunch_ptsz(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys an executable graph

 Destroys the executable graph specified by \p hGraphExec, as well
 as all of its executable nodes. If the executable graph is
 in-flight, it will not be terminated, but rather freed
 asynchronously on completion.

 \param hGraphExec - Executable graph to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphInstantiate,
 ::cuGraphUpload,
 ::cuGraphLaunch*/
    fn cuGraphExecDestroy(
        hGraphExec: cuda_types::cuda::CUgraphExec,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys a graph

 Destroys the graph specified by \p hGraph, as well as all of its nodes.

 \param hGraph - Graph to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphCreate*/
    fn cuGraphDestroy(hGraph: cuda_types::cuda::CUgraph) -> cuda_types::cuda::CUresult;
    /** \brief Check whether an executable graph can be updated with a graph and perform the update if possible

 Updates the node parameters in the instantiated graph specified by \p hGraphExec with the
 node parameters in a topologically identical graph specified by \p hGraph.

 Limitations:

 - Kernel nodes:
   - The owning context of the function cannot change.
   - A node whose function originally did not use CUDA dynamic parallelism cannot be updated
     to a function which uses CDP.
   - A node whose function originally did not make device-side update calls cannot be updated
     to a function which makes device-side update calls.
   - A cooperative node cannot be updated to a non-cooperative node, and vice-versa.
   - If the graph was instantiated with CUDA_GRAPH_INSTANTIATE_FLAG_USE_NODE_PRIORITY, the
     priority attribute cannot change. Equality is checked on the originally requested
     priority values, before they are clamped to the device's supported range.
   - If \p hGraphExec was not instantiated for device launch, a node whose function originally
     did not use device-side cudaGraphLaunch() cannot be updated to a function which uses
     device-side cudaGraphLaunch() unless the node resides on the same context as nodes which
     contained such calls at instantiate-time. If no such calls were present at instantiation,
     these updates cannot be performed at all.
   - Neither \p hGraph nor \p hGraphExec may contain device-updatable kernel nodes.
 - Memset and memcpy nodes:
   - The CUDA device(s) to which the operand(s) was allocated/mapped cannot change.
   - The source/destination memory must be allocated from the same contexts as the original
     source/destination memory.
   - For 2d memsets, only address and assigned value may be updated.
   - For 1d memsets, updating dimensions is also allowed, but may fail if the resulting operation doesn't
     map onto the work resources already allocated for the node.
 - Additional memcpy node restrictions:
   - Changing either the source or destination memory type(i.e. CU_MEMORYTYPE_DEVICE,
     CU_MEMORYTYPE_ARRAY, etc.) is not supported.
 - External semaphore wait nodes and record nodes:
   - Changing the number of semaphores is not supported.
 - Conditional nodes:
   - Changing node parameters is not supported.
   - Changing parameters of nodes within the conditional body graph is subject to the rules above.
   - Conditional handle flags and default values are updated as part of the graph update.

 Note:  The API may add further restrictions in future releases.  The return code should always be checked.

 cuGraphExecUpdate sets the result member of \p resultInfo to CU_GRAPH_EXEC_UPDATE_ERROR_TOPOLOGY_CHANGED
 under the following conditions:
 - The count of nodes directly in \p hGraphExec and \p hGraph differ, in which case resultInfo->errorNode
   is set to NULL.
 - \p hGraph has more exit nodes than \p hGraph, in which case resultInfo->errorNode is set to one of
   the exit nodes in hGraph.
 - A node in \p hGraph has a different number of dependencies than the node from \p hGraphExec it is paired with,
   in which case resultInfo->errorNode is set to the node from \p hGraph.
 - A node in \p hGraph has a dependency that does not match with the corresponding dependency of the paired node
   from \p hGraphExec. resultInfo->errorNode will be set to the node from \p hGraph. resultInfo->errorFromNode
   will be set to the mismatched dependency. The dependencies are paired based on edge order and a dependency
   does not match when the nodes are already paired based on other edges examined in the graph.

 cuGraphExecUpdate sets the result member of \p resultInfo to:
 - CU_GRAPH_EXEC_UPDATE_ERROR if passed an invalid value.
 - CU_GRAPH_EXEC_UPDATE_ERROR_TOPOLOGY_CHANGED if the graph topology changed
 - CU_GRAPH_EXEC_UPDATE_ERROR_NODE_TYPE_CHANGED if the type of a node changed, in which case
   \p hErrorNode_out is set to the node from \p hGraph.
 - CU_GRAPH_EXEC_UPDATE_ERROR_UNSUPPORTED_FUNCTION_CHANGE if the function changed in an unsupported
   way(see note above), in which case \p hErrorNode_out is set to the node from \p hGraph
 - CU_GRAPH_EXEC_UPDATE_ERROR_PARAMETERS_CHANGED if any parameters to a node changed in a way
   that is not supported, in which case \p hErrorNode_out is set to the node from \p hGraph.
 - CU_GRAPH_EXEC_UPDATE_ERROR_ATTRIBUTES_CHANGED if any attributes of a node changed in a way
   that is not supported, in which case \p hErrorNode_out is set to the node from \p hGraph.
 - CU_GRAPH_EXEC_UPDATE_ERROR_NOT_SUPPORTED if something about a node is unsupported, like
   the node's type or configuration, in which case \p hErrorNode_out is set to the node from \p hGraph

 If the update fails for a reason not listed above, the result member of \p resultInfo will be set
 to CU_GRAPH_EXEC_UPDATE_ERROR. If the update succeeds, the result member will be set to CU_GRAPH_EXEC_UPDATE_SUCCESS.

 cuGraphExecUpdate returns CUDA_SUCCESS when the updated was performed successfully.  It returns
 CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE if the graph update was not performed because it included
 changes which violated constraints specific to instantiated graph update.

 \param hGraphExec The instantiated graph to be updated
 \param hGraph The graph containing the updated parameters
 \param resultInfo the error info structure

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE,
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphInstantiate*/
    fn cuGraphExecUpdate_v2(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hGraph: cuda_types::cuda::CUgraph,
        resultInfo: *mut cuda_types::cuda::CUgraphExecUpdateResultInfo,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Copies attributes from source node to destination node.

 Copies attributes from source node \p src to destination node \p dst.
 Both node must have the same context.

 \param[out] dst Destination node
 \param[in] src Source node
 For list of attributes see ::CUkernelNodeAttrID

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::CUaccessPolicyWindow*/
    fn cuGraphKernelNodeCopyAttributes(
        dst: cuda_types::cuda::CUgraphNode,
        src: cuda_types::cuda::CUgraphNode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Queries node attribute.

 Queries attribute \p attr from node \p hNode and stores it in corresponding
 member of \p value_out.

 \param[in] hNode
 \param[in] attr
 \param[out] value_out

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa
 ::CUaccessPolicyWindow*/
    fn cuGraphKernelNodeGetAttribute(
        hNode: cuda_types::cuda::CUgraphNode,
        attr: cuda_types::cuda::CUkernelNodeAttrID,
        value_out: *mut cuda_types::cuda::CUkernelNodeAttrValue,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets node attribute.

 Sets attribute \p attr on node \p hNode from corresponding attribute of
 \p value.

 \param[out] hNode
 \param[in] attr
 \param[out] value

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE
 \notefnerr

 \sa
 ::CUaccessPolicyWindow*/
    fn cuGraphKernelNodeSetAttribute(
        hNode: cuda_types::cuda::CUgraphNode,
        attr: cuda_types::cuda::CUkernelNodeAttrID,
        value: *const cuda_types::cuda::CUkernelNodeAttrValue,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Write a DOT file describing graph structure

 Using the provided \p hGraph, write to \p path a DOT formatted description of the graph.
 By default this includes the graph topology, node types, node id, kernel names and memcpy direction.
 \p flags can be specified to write more detailed information about each node type such as
 parameter values, kernel attributes, node and function handles.

 \param hGraph - The graph to create a DOT file from
 \param path   - The path to write the DOT file to
 \param flags  - Flags from CUgraphDebugDot_flags for specifying which additional node information to write

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OPERATING_SYSTEM*/
    fn cuGraphDebugDotPrint(
        hGraph: cuda_types::cuda::CUgraph,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a user object

 Create a user object with the specified destructor callback and initial reference count. The
 initial references are owned by the caller.

 Destructor callbacks cannot make CUDA API calls and should avoid blocking behavior, as they
 are executed by a shared internal thread. Another thread may be signaled to perform such
 actions, if it does not block forward progress of tasks scheduled through CUDA.

 See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.

 \param object_out      - Location to return the user object handle
 \param ptr             - The pointer to pass to the destroy function
 \param destroy         - Callback to free the user object when it is no longer in use
 \param initialRefcount - The initial refcount to create the object with, typically 1. The
                          initial references are owned by the calling thread.
 \param flags           - Currently it is required to pass ::CU_USER_OBJECT_NO_DESTRUCTOR_SYNC,
                          which is the only defined flag. This indicates that the destroy
                          callback cannot be waited on by any CUDA API. Users requiring
                          synchronization of the callback should signal its completion
                          manually.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuUserObjectRetain,
 ::cuUserObjectRelease,
 ::cuGraphRetainUserObject,
 ::cuGraphReleaseUserObject,
 ::cuGraphCreate*/
    fn cuUserObjectCreate(
        object_out: *mut cuda_types::cuda::CUuserObject,
        ptr: *mut ::core::ffi::c_void,
        destroy: cuda_types::cuda::CUhostFn,
        initialRefcount: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Retain a reference to a user object

 Retains new references to a user object. The new references are owned by the caller.

 See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.

 \param object - The object to retain
 \param count  - The number of references to retain, typically 1. Must be nonzero
                 and not larger than INT_MAX.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuUserObjectCreate,
 ::cuUserObjectRelease,
 ::cuGraphRetainUserObject,
 ::cuGraphReleaseUserObject,
 ::cuGraphCreate*/
    fn cuUserObjectRetain(
        object: cuda_types::cuda::CUuserObject,
        count: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Release a reference to a user object

 Releases user object references owned by the caller. The object's destructor is invoked if
 the reference count reaches zero.

 It is undefined behavior to release references not owned by the caller, or to use a user
 object handle after all references are released.

 See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.

 \param object - The object to release
 \param count  - The number of references to release, typically 1. Must be nonzero
                 and not larger than INT_MAX.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuUserObjectCreate,
 ::cuUserObjectRetain,
 ::cuGraphRetainUserObject,
 ::cuGraphReleaseUserObject,
 ::cuGraphCreate*/
    fn cuUserObjectRelease(
        object: cuda_types::cuda::CUuserObject,
        count: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Retain a reference to a user object from a graph

 Creates or moves user object references that will be owned by a CUDA graph.

 See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.

 \param graph  - The graph to associate the reference with
 \param object - The user object to retain a reference for
 \param count  - The number of references to add to the graph, typically 1. Must be
                 nonzero and not larger than INT_MAX.
 \param flags  - The optional flag ::CU_GRAPH_USER_OBJECT_MOVE transfers references
                 from the calling thread, rather than create new references. Pass 0
                 to create new references.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuUserObjectCreate,
 ::cuUserObjectRetain,
 ::cuUserObjectRelease,
 ::cuGraphReleaseUserObject,
 ::cuGraphCreate*/
    fn cuGraphRetainUserObject(
        graph: cuda_types::cuda::CUgraph,
        object: cuda_types::cuda::CUuserObject,
        count: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Release a user object reference from a graph

 Releases user object references owned by a graph.

 See CUDA User Objects in the CUDA C++ Programming Guide for more information on user objects.

 \param graph  - The graph that will release the reference
 \param object - The user object to release a reference for
 \param count  - The number of references to release, typically 1. Must be nonzero
                 and not larger than INT_MAX.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuUserObjectCreate,
 ::cuUserObjectRetain,
 ::cuUserObjectRelease,
 ::cuGraphRetainUserObject,
 ::cuGraphCreate*/
    fn cuGraphReleaseUserObject(
        graph: cuda_types::cuda::CUgraph,
        object: cuda_types::cuda::CUuserObject,
        count: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Adds a node of arbitrary type to a graph

 Creates a new node in \p hGraph described by \p nodeParams with \p numDependencies
 dependencies specified via \p dependencies. \p numDependencies may be 0.
 \p dependencies may be null if \p numDependencies is 0. \p dependencies may not have
 any duplicate entries.

 \p nodeParams is a tagged union. The node type should be specified in the \p type field,
 and type-specific parameters in the corresponding union member. All unused bytes - that
 is, \p reserved0 and all bytes past the utilized union member - must be set to zero.
 It is recommended to use brace initialization or memset to ensure all bytes are
 initialized.

 Note that for some node types, \p nodeParams may contain "out parameters" which are
 modified during the call, such as \p nodeParams->alloc.dptr.

 A handle to the new node will be returned in \p phGraphNode.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param numDependencies - Number of dependencies
 \param nodeParams      - Specification of the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_NOT_SUPPORTED
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphCreate,
 ::cuGraphNodeSetParams,
 ::cuGraphExecNodeSetParams*/
    fn cuGraphAddNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        nodeParams: *mut cuda_types::cuda::CUgraphNodeParams,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Adds a node of arbitrary type to a graph (12.3+)

 Creates a new node in \p hGraph described by \p nodeParams with \p numDependencies
 dependencies specified via \p dependencies. \p numDependencies may be 0.
 \p dependencies may be null if \p numDependencies is 0. \p dependencies may not have
 any duplicate entries.

 \p nodeParams is a tagged union. The node type should be specified in the \p type field,
 and type-specific parameters in the corresponding union member. All unused bytes - that
 is, \p reserved0 and all bytes past the utilized union member - must be set to zero.
 It is recommended to use brace initialization or memset to ensure all bytes are
 initialized.

 Note that for some node types, \p nodeParams may contain "out parameters" which are
 modified during the call, such as \p nodeParams->alloc.dptr.

 A handle to the new node will be returned in \p phGraphNode.

 \param phGraphNode     - Returns newly created node
 \param hGraph          - Graph to which to add the node
 \param dependencies    - Dependencies of the node
 \param dependencyData  - Optional edge data for the dependencies. If NULL, the data is
                          assumed to be default (zeroed) for all dependencies.
 \param numDependencies - Number of dependencies
 \param nodeParams      - Specification of the node

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_NOT_SUPPORTED
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphCreate,
 ::cuGraphNodeSetParams,
 ::cuGraphExecNodeSetParams*/
    fn cuGraphAddNode_v2(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
        numDependencies: usize,
        nodeParams: *mut cuda_types::cuda::CUgraphNodeParams,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Update's a graph node's parameters

 Sets the parameters of graph node \p hNode to \p nodeParams. The node type specified by
 \p nodeParams->type must match the type of \p hNode. \p nodeParams must be fully
 initialized and all unused bytes (reserved, padding) zeroed.

 Modifying parameters is not supported for node types CU_GRAPH_NODE_TYPE_MEM_ALLOC and
 CU_GRAPH_NODE_TYPE_MEM_FREE.

 \param hNode      - Node to set the parameters for
 \param nodeParams - Parameters to copy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphExecNodeSetParams*/
    fn cuGraphNodeSetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *mut cuda_types::cuda::CUgraphNodeParams,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Update's a graph node's parameters in an instantiated graph

 Sets the parameters of a node in an executable graph \p hGraphExec. The node is identified
 by the corresponding node \p hNode in the non-executable graph from which the executable
 graph was instantiated. \p hNode must not have been removed from the original graph.

 The modifications only affect future launches of \p hGraphExec. Already
 enqueued or running launches of \p hGraphExec are not affected by this call.
 \p hNode is also not modified by this call.

 Allowed changes to parameters on executable graphs are as follows:
 <table>
   <tr><th>Node type<th>Allowed changes
   <tr><td>kernel<td>See ::cuGraphExecKernelNodeSetParams
   <tr><td>memcpy<td>Addresses for 1-dimensional copies if allocated in same context; see ::cuGraphExecMemcpyNodeSetParams
   <tr><td>memset<td>Addresses for 1-dimensional memsets if allocated in same context; see ::cuGraphExecMemsetNodeSetParams
   <tr><td>host<td>Unrestricted
   <tr><td>child graph<td>Topology must match and restrictions apply recursively; see ::cuGraphExecUpdate
   <tr><td>event wait<td>Unrestricted
   <tr><td>event record<td>Unrestricted
   <tr><td>external semaphore signal<td>Number of semaphore operations cannot change
   <tr><td>external semaphore wait<td>Number of semaphore operations cannot change
   <tr><td>memory allocation<td>API unsupported
   <tr><td>memory free<td>API unsupported
   <tr><td>batch memops<td>Addresses, values, and operation type for wait operations; see ::cuGraphExecBatchMemOpNodeSetParams
 </table>

 \param hGraphExec  - The executable graph in which to update the specified node
 \param hNode       - Corresponding node from the graph from which graphExec was instantiated
 \param nodeParams  - Updated Parameters to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode,
 ::cuGraphNodeSetParams
 ::cuGraphExecUpdate,
 ::cuGraphInstantiate*/
    fn cuGraphExecNodeSetParams(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *mut cuda_types::cuda::CUgraphNodeParams,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a conditional handle

 Creates a conditional handle associated with \p hGraph.

 The conditional handle must be associated with a conditional node in this graph or one of its children.

 Handles not associated with a conditional node may cause graph instantiation to fail.

 Handles can only be set from the context with which they are associated.

 \param pHandle_out        - Pointer used to return the handle to the caller.
 \param hGraph             - Graph which will contain the conditional node using this handle.
 \param ctx                - Context for the handle and associated conditional node.
 \param defaultLaunchValue - Optional initial value for the conditional variable.
                             Applied at the beginning of each graph execution if CU_GRAPH_COND_ASSIGN_DEFAULT is set in \p flags.
 \param flags              - Currently must be CU_GRAPH_COND_ASSIGN_DEFAULT or 0.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \note_graph_thread_safety
 \notefnerr

 \sa
 ::cuGraphAddNode*/
    fn cuGraphConditionalHandleCreate(
        pHandle_out: *mut cuda_types::cuda::CUgraphConditionalHandle,
        hGraph: cuda_types::cuda::CUgraph,
        ctx: cuda_types::cuda::CUcontext,
        defaultLaunchValue: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns occupancy of a function

 Returns in \p *numBlocks the number of the maximum active blocks per
 streaming multiprocessor.

 Note that the API can also be used with context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to use for calculations
 will be the current context.

 \param numBlocks       - Returned occupancy
 \param func            - Kernel for which occupancy is calculated
 \param blockSize       - Block size the kernel is intended to be launched with
 \param dynamicSMemSize - Per-block dynamic shared memory usage intended, in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cudaOccupancyMaxActiveBlocksPerMultiprocessor*/
    fn cuOccupancyMaxActiveBlocksPerMultiprocessor(
        numBlocks: *mut ::core::ffi::c_int,
        func: cuda_types::cuda::CUfunction,
        blockSize: ::core::ffi::c_int,
        dynamicSMemSize: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns occupancy of a function

 Returns in \p *numBlocks the number of the maximum active blocks per
 streaming multiprocessor.

 The \p Flags parameter controls how special cases are handled. The
 valid flags are:

 - ::CU_OCCUPANCY_DEFAULT, which maintains the default behavior as
   ::cuOccupancyMaxActiveBlocksPerMultiprocessor;

 - ::CU_OCCUPANCY_DISABLE_CACHING_OVERRIDE, which suppresses the
   default behavior on platform where global caching affects
   occupancy. On such platforms, if caching is enabled, but
   per-block SM resource usage would result in zero occupancy, the
   occupancy calculator will calculate the occupancy as if caching
   is disabled. Setting ::CU_OCCUPANCY_DISABLE_CACHING_OVERRIDE makes
   the occupancy calculator to return 0 in such cases. More information
   can be found about this feature in the "Unified L1/Texture Cache"
   section of the Maxwell tuning guide.

 Note that the API can also be with launch context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to use for calculations
 will be the current context.

 \param numBlocks       - Returned occupancy
 \param func            - Kernel for which occupancy is calculated
 \param blockSize       - Block size the kernel is intended to be launched with
 \param dynamicSMemSize - Per-block dynamic shared memory usage intended, in bytes
 \param flags           - Requested behavior for the occupancy calculator

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags*/
    fn cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        numBlocks: *mut ::core::ffi::c_int,
        func: cuda_types::cuda::CUfunction,
        blockSize: ::core::ffi::c_int,
        dynamicSMemSize: usize,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Suggest a launch configuration with reasonable occupancy

 Returns in \p *blockSize a reasonable block size that can achieve
 the maximum occupancy (or, the maximum number of active warps with
 the fewest blocks per multiprocessor), and in \p *minGridSize the
 minimum grid size to achieve the maximum occupancy.

 If \p blockSizeLimit is 0, the configurator will use the maximum
 block size permitted by the device / function instead.

 If per-block dynamic shared memory allocation is not needed, the
 user should leave both \p blockSizeToDynamicSMemSize and \p
 dynamicSMemSize as 0.

 If per-block dynamic shared memory allocation is needed, then if
 the dynamic shared memory size is constant regardless of block
 size, the size should be passed through \p dynamicSMemSize, and \p
 blockSizeToDynamicSMemSize should be NULL.

 Otherwise, if the per-block dynamic shared memory size varies with
 different block sizes, the user needs to provide a unary function
 through \p blockSizeToDynamicSMemSize that computes the dynamic
 shared memory needed by \p func for any given block size. \p
 dynamicSMemSize is ignored. An example signature is:

 \code
    // Take block size, returns dynamic shared memory needed
    size_t blockToSmem(int blockSize);
 \endcode

 Note that the API can also be used with context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to use for calculations
 will be the current context.

 \param minGridSize - Returned minimum grid size needed to achieve the maximum occupancy
 \param blockSize   - Returned maximum block size that can achieve the maximum occupancy
 \param func        - Kernel for which launch configuration is calculated
 \param blockSizeToDynamicSMemSize - A function that calculates how much per-block dynamic shared memory \p func uses based on the block size
 \param dynamicSMemSize - Dynamic shared memory usage intended, in bytes
 \param blockSizeLimit  - The maximum block size \p func is designed to handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cudaOccupancyMaxPotentialBlockSize*/
    fn cuOccupancyMaxPotentialBlockSize(
        minGridSize: *mut ::core::ffi::c_int,
        blockSize: *mut ::core::ffi::c_int,
        func: cuda_types::cuda::CUfunction,
        blockSizeToDynamicSMemSize: cuda_types::cuda::CUoccupancyB2DSize,
        dynamicSMemSize: usize,
        blockSizeLimit: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Suggest a launch configuration with reasonable occupancy

 An extended version of ::cuOccupancyMaxPotentialBlockSize. In
 addition to arguments passed to ::cuOccupancyMaxPotentialBlockSize,
 ::cuOccupancyMaxPotentialBlockSizeWithFlags also takes a \p Flags
 parameter.

 The \p Flags parameter controls how special cases are handled. The
 valid flags are:

 - ::CU_OCCUPANCY_DEFAULT, which maintains the default behavior as
   ::cuOccupancyMaxPotentialBlockSize;

 - ::CU_OCCUPANCY_DISABLE_CACHING_OVERRIDE, which suppresses the
   default behavior on platform where global caching affects
   occupancy. On such platforms, the launch configurations that
   produces maximal occupancy might not support global
   caching. Setting ::CU_OCCUPANCY_DISABLE_CACHING_OVERRIDE
   guarantees that the the produced launch configuration is global
   caching compatible at a potential cost of occupancy. More information
   can be found about this feature in the "Unified L1/Texture Cache"
   section of the Maxwell tuning guide.

 Note that the API can also be used with context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to use for calculations
 will be the current context.

 \param minGridSize - Returned minimum grid size needed to achieve the maximum occupancy
 \param blockSize   - Returned maximum block size that can achieve the maximum occupancy
 \param func        - Kernel for which launch configuration is calculated
 \param blockSizeToDynamicSMemSize - A function that calculates how much per-block dynamic shared memory \p func uses based on the block size
 \param dynamicSMemSize - Dynamic shared memory usage intended, in bytes
 \param blockSizeLimit  - The maximum block size \p func is designed to handle
 \param flags       - Options

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cudaOccupancyMaxPotentialBlockSizeWithFlags*/
    fn cuOccupancyMaxPotentialBlockSizeWithFlags(
        minGridSize: *mut ::core::ffi::c_int,
        blockSize: *mut ::core::ffi::c_int,
        func: cuda_types::cuda::CUfunction,
        blockSizeToDynamicSMemSize: cuda_types::cuda::CUoccupancyB2DSize,
        dynamicSMemSize: usize,
        blockSizeLimit: ::core::ffi::c_int,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns dynamic shared memory available per block when launching \p numBlocks blocks on SM

 Returns in \p *dynamicSmemSize the maximum size of dynamic shared memory to allow \p numBlocks blocks per SM.

 Note that the API can also be used with context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to use for calculations
 will be the current context.

 \param dynamicSmemSize - Returned maximum dynamic shared memory
 \param func            - Kernel function for which occupancy is calculated
 \param numBlocks       - Number of blocks to fit on SM
 \param blockSize       - Size of the blocks

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr*/
    fn cuOccupancyAvailableDynamicSMemPerBlock(
        dynamicSmemSize: *mut usize,
        func: cuda_types::cuda::CUfunction,
        numBlocks: ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Given the kernel function (\p func) and launch configuration
 (\p config), return the maximum cluster size in \p *clusterSize.

 The cluster dimensions in \p config are ignored. If func has a required
 cluster size set (see ::cudaFuncGetAttributes / ::cuFuncGetAttribute),\p
 *clusterSize will reflect the required cluster size.

 By default this function will always return a value that's portable on
 future hardware. A higher value may be returned if the kernel function
 allows non-portable cluster sizes.

 This function will respect the compile time launch bounds.

 Note that the API can also be used with context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to use for calculations
 will either be taken from the specified stream \p config->hStream
 or the current context in case of NULL stream.

 \param clusterSize - Returned maximum cluster size that can be launched
                      for the given kernel function and launch configuration
 \param func        - Kernel function for which maximum cluster
                      size is calculated
 \param config      - Launch configuration for the given kernel function

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cudaFuncGetAttributes,
 ::cuFuncGetAttribute*/
    fn cuOccupancyMaxPotentialClusterSize(
        clusterSize: *mut ::core::ffi::c_int,
        func: cuda_types::cuda::CUfunction,
        config: *const cuda_types::cuda::CUlaunchConfig,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Given the kernel function (\p func) and launch configuration
 (\p config), return the maximum number of clusters that could co-exist
 on the target device in \p *numClusters.

 If the function has required cluster size already set (see
 ::cudaFuncGetAttributes / ::cuFuncGetAttribute), the cluster size
 from config must either be unspecified or match the required size.
 Without required sizes, the cluster size must be specified in config,
 else the function will return an error.

 Note that various attributes of the kernel function may affect occupancy
 calculation. Runtime environment may affect how the hardware schedules
 the clusters, so the calculated occupancy is not guaranteed to be achievable.

 Note that the API can also be used with context-less kernel ::CUkernel
 by querying the handle using ::cuLibraryGetKernel() and then passing it
 to the API by casting to ::CUfunction. Here, the context to use for calculations
 will either be taken from the specified stream \p config->hStream
 or the current context in case of NULL stream.

 \param numClusters - Returned maximum number of clusters that
                      could co-exist on the target device
 \param func        - Kernel function for which maximum number
                      of clusters are calculated
 \param config      - Launch configuration for the given kernel function

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_CLUSTER_SIZE,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cudaFuncGetAttributes,
 ::cuFuncGetAttribute*/
    fn cuOccupancyMaxActiveClusters(
        numClusters: *mut ::core::ffi::c_int,
        func: cuda_types::cuda::CUfunction,
        config: *const cuda_types::cuda::CUlaunchConfig,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Binds an array as a texture reference

 \deprecated

 Binds the CUDA array \p hArray to the texture reference \p hTexRef. Any
 previous address or CUDA array state associated with the texture reference
 is superseded by this function. \p Flags must be set to
 ::CU_TRSA_OVERRIDE_FORMAT. Any CUDA array previously bound to \p hTexRef is
 unbound.

 \param hTexRef - Texture reference to bind
 \param hArray  - Array to bind
 \param Flags   - Options (must be ::CU_TRSA_OVERRIDE_FORMAT)

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetArray(
        hTexRef: cuda_types::cuda::CUtexref,
        hArray: cuda_types::cuda::CUarray,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Binds a mipmapped array to a texture reference

 \deprecated

 Binds the CUDA mipmapped array \p hMipmappedArray to the texture reference \p hTexRef.
 Any previous address or CUDA array state associated with the texture reference
 is superseded by this function. \p Flags must be set to ::CU_TRSA_OVERRIDE_FORMAT.
 Any CUDA array previously bound to \p hTexRef is unbound.

 \param hTexRef         - Texture reference to bind
 \param hMipmappedArray - Mipmapped array to bind
 \param Flags           - Options (must be ::CU_TRSA_OVERRIDE_FORMAT)

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetMipmappedArray(
        hTexRef: cuda_types::cuda::CUtexref,
        hMipmappedArray: cuda_types::cuda::CUmipmappedArray,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Binds an address as a texture reference

 \deprecated

 Binds a linear address range to the texture reference \p hTexRef. Any
 previous address or CUDA array state associated with the texture reference
 is superseded by this function. Any memory previously bound to \p hTexRef
 is unbound.

 Since the hardware enforces an alignment requirement on texture base
 addresses, ::cuTexRefSetAddress() passes back a byte offset in
 \p *ByteOffset that must be applied to texture fetches in order to read from
 the desired memory. This offset must be divided by the texel size and
 passed to kernels that read from the texture so they can be applied to the
 ::tex1Dfetch() function.

 If the device memory pointer was returned from ::cuMemAlloc(), the offset
 is guaranteed to be 0 and NULL may be passed as the \p ByteOffset parameter.

 The total number of elements (or texels) in the linear address range
 cannot exceed ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH.
 The number of elements is computed as (\p bytes / bytesPerElement),
 where bytesPerElement is determined from the data format and number of
 components set using ::cuTexRefSetFormat().

 \param ByteOffset - Returned byte offset
 \param hTexRef    - Texture reference to bind
 \param dptr       - Device pointer to bind
 \param bytes      - Size of memory to bind in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetAddress_v2(
        ByteOffset: *mut usize,
        hTexRef: cuda_types::cuda::CUtexref,
        dptr: cuda_types::cuda::CUdeviceptr,
        bytes: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Binds an address as a 2D texture reference

 \deprecated

 Binds a linear address range to the texture reference \p hTexRef. Any
 previous address or CUDA array state associated with the texture reference
 is superseded by this function. Any memory previously bound to \p hTexRef
 is unbound.

 Using a ::tex2D() function inside a kernel requires a call to either
 ::cuTexRefSetArray() to bind the corresponding texture reference to an
 array, or ::cuTexRefSetAddress2D() to bind the texture reference to linear
 memory.

 Function calls to ::cuTexRefSetFormat() cannot follow calls to
 ::cuTexRefSetAddress2D() for the same texture reference.

 It is required that \p dptr be aligned to the appropriate hardware-specific
 texture alignment. You can query this value using the device attribute
 ::CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT. If an unaligned \p dptr is
 supplied, ::CUDA_ERROR_INVALID_VALUE is returned.

 \p Pitch has to be aligned to the hardware-specific texture pitch alignment.
 This value can be queried using the device attribute
 ::CU_DEVICE_ATTRIBUTE_TEXTURE_PITCH_ALIGNMENT. If an unaligned \p Pitch is
 supplied, ::CUDA_ERROR_INVALID_VALUE is returned.

 Width and Height, which are specified in elements (or texels), cannot exceed
 ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH and
 ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT respectively.
 \p Pitch, which is specified in bytes, cannot exceed
 ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH.

 \param hTexRef - Texture reference to bind
 \param desc    - Descriptor of CUDA array
 \param dptr    - Device pointer to bind
 \param Pitch   - Line pitch in bytes

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetAddress2D_v3(
        hTexRef: cuda_types::cuda::CUtexref,
        desc: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR,
        dptr: cuda_types::cuda::CUdeviceptr,
        Pitch: usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the format for a texture reference

 \deprecated

 Specifies the format of the data to be read by the texture reference
 \p hTexRef. \p fmt and \p NumPackedComponents are exactly analogous to the
 ::Format and ::NumChannels members of the ::CUDA_ARRAY_DESCRIPTOR structure:
 They specify the format of each component and the number of components per
 array element.

 \param hTexRef             - Texture reference
 \param fmt                 - Format to set
 \param NumPackedComponents - Number of components per array element

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat,
 ::cudaCreateChannelDesc*/
    fn cuTexRefSetFormat(
        hTexRef: cuda_types::cuda::CUtexref,
        fmt: cuda_types::cuda::CUarray_format,
        NumPackedComponents: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the addressing mode for a texture reference

 \deprecated

 Specifies the addressing mode \p am for the given dimension \p dim of the
 texture reference \p hTexRef. If \p dim is zero, the addressing mode is
 applied to the first parameter of the functions used to fetch from the
 texture; if \p dim is 1, the second, and so on. ::CUaddress_mode is defined
 as:
 \code
typedef enum CUaddress_mode_enum {
CU_TR_ADDRESS_MODE_WRAP = 0,
CU_TR_ADDRESS_MODE_CLAMP = 1,
CU_TR_ADDRESS_MODE_MIRROR = 2,
CU_TR_ADDRESS_MODE_BORDER = 3
} CUaddress_mode;
 \endcode

 Note that this call has no effect if \p hTexRef is bound to linear memory.
 Also, if the flag, ::CU_TRSF_NORMALIZED_COORDINATES, is not set, the only
 supported address mode is ::CU_TR_ADDRESS_MODE_CLAMP.

 \param hTexRef - Texture reference
 \param dim     - Dimension
 \param am      - Addressing mode to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetAddressMode(
        hTexRef: cuda_types::cuda::CUtexref,
        dim: ::core::ffi::c_int,
        am: cuda_types::cuda::CUaddress_mode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the filtering mode for a texture reference

 \deprecated

 Specifies the filtering mode \p fm to be used when reading memory through
 the texture reference \p hTexRef. ::CUfilter_mode_enum is defined as:

 \code
typedef enum CUfilter_mode_enum {
CU_TR_FILTER_MODE_POINT = 0,
CU_TR_FILTER_MODE_LINEAR = 1
} CUfilter_mode;
 \endcode

 Note that this call has no effect if \p hTexRef is bound to linear memory.

 \param hTexRef - Texture reference
 \param fm      - Filtering mode to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetFilterMode(
        hTexRef: cuda_types::cuda::CUtexref,
        fm: cuda_types::cuda::CUfilter_mode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the mipmap filtering mode for a texture reference

 \deprecated

 Specifies the mipmap filtering mode \p fm to be used when reading memory through
 the texture reference \p hTexRef. ::CUfilter_mode_enum is defined as:

 \code
typedef enum CUfilter_mode_enum {
CU_TR_FILTER_MODE_POINT = 0,
CU_TR_FILTER_MODE_LINEAR = 1
} CUfilter_mode;
 \endcode

 Note that this call has no effect if \p hTexRef is not bound to a mipmapped array.

 \param hTexRef - Texture reference
 \param fm      - Filtering mode to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetMipmapFilterMode(
        hTexRef: cuda_types::cuda::CUtexref,
        fm: cuda_types::cuda::CUfilter_mode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the mipmap level bias for a texture reference

 \deprecated

 Specifies the mipmap level bias \p bias to be added to the specified mipmap level when
 reading memory through the texture reference \p hTexRef.

 Note that this call has no effect if \p hTexRef is not bound to a mipmapped array.

 \param hTexRef - Texture reference
 \param bias    - Mipmap level bias

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetMipmapLevelBias(
        hTexRef: cuda_types::cuda::CUtexref,
        bias: f32,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the mipmap min/max mipmap level clamps for a texture reference

 \deprecated

 Specifies the min/max mipmap level clamps, \p minMipmapLevelClamp and \p maxMipmapLevelClamp
 respectively, to be used when reading memory through the texture reference
 \p hTexRef.

 Note that this call has no effect if \p hTexRef is not bound to a mipmapped array.

 \param hTexRef        - Texture reference
 \param minMipmapLevelClamp - Mipmap min level clamp
 \param maxMipmapLevelClamp - Mipmap max level clamp

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetMipmapLevelClamp(
        hTexRef: cuda_types::cuda::CUtexref,
        minMipmapLevelClamp: f32,
        maxMipmapLevelClamp: f32,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the maximum anisotropy for a texture reference

 \deprecated

 Specifies the maximum anisotropy \p maxAniso to be used when reading memory through
 the texture reference \p hTexRef.

 Note that this call has no effect if \p hTexRef is bound to linear memory.

 \param hTexRef  - Texture reference
 \param maxAniso - Maximum anisotropy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetMaxAnisotropy(
        hTexRef: cuda_types::cuda::CUtexref,
        maxAniso: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the border color for a texture reference

 \deprecated

 Specifies the value of the RGBA color via the \p pBorderColor to the texture reference
 \p hTexRef. The color value supports only float type and holds color components in
 the following sequence:
 pBorderColor[0] holds 'R' component
 pBorderColor[1] holds 'G' component
 pBorderColor[2] holds 'B' component
 pBorderColor[3] holds 'A' component

 Note that the color values can be set only when the Address mode is set to
 CU_TR_ADDRESS_MODE_BORDER using ::cuTexRefSetAddressMode.
 Applications using integer border color values have to "reinterpret_cast" their values to float.

 \param hTexRef       - Texture reference
 \param pBorderColor  - RGBA color

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddressMode,
 ::cuTexRefGetAddressMode, ::cuTexRefGetBorderColor*/
    fn cuTexRefSetBorderColor(
        hTexRef: cuda_types::cuda::CUtexref,
        pBorderColor: *mut f32,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the flags for a texture reference

 \deprecated

 Specifies optional flags via \p Flags to specify the behavior of data
 returned through the texture reference \p hTexRef. The valid flags are:

 - ::CU_TRSF_READ_AS_INTEGER, which suppresses the default behavior of
   having the texture promote integer data to floating point data in the
   range [0, 1]. Note that texture with 32-bit integer format
   would not be promoted, regardless of whether or not this
   flag is specified;
 - ::CU_TRSF_NORMALIZED_COORDINATES, which suppresses the
   default behavior of having the texture coordinates range
   from [0, Dim) where Dim is the width or height of the CUDA
   array. Instead, the texture coordinates [0, 1.0) reference
   the entire breadth of the array dimension;
 - ::CU_TRSF_DISABLE_TRILINEAR_OPTIMIZATION, which disables any trilinear
   filtering optimizations. Trilinear optimizations improve texture filtering
   performance by allowing bilinear filtering on textures in scenarios where
   it can closely approximate the expected results.

 \param hTexRef - Texture reference
 \param Flags   - Optional flags to set

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefSetFlags(
        hTexRef: cuda_types::cuda::CUtexref,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the address associated with a texture reference

 \deprecated

 Returns in \p *pdptr the base address bound to the texture reference
 \p hTexRef, or returns ::CUDA_ERROR_INVALID_VALUE if the texture reference
 is not bound to any device memory range.

 \param pdptr   - Returned device address
 \param hTexRef - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefGetAddress_v2(
        pdptr: *mut cuda_types::cuda::CUdeviceptr,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the array bound to a texture reference

 \deprecated

 Returns in \p *phArray the CUDA array bound to the texture reference
 \p hTexRef, or returns ::CUDA_ERROR_INVALID_VALUE if the texture reference
 is not bound to any CUDA array.

 \param phArray - Returned array
 \param hTexRef - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefGetArray(
        phArray: *mut cuda_types::cuda::CUarray,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the mipmapped array bound to a texture reference

 \deprecated

 Returns in \p *phMipmappedArray the CUDA mipmapped array bound to the texture
 reference \p hTexRef, or returns ::CUDA_ERROR_INVALID_VALUE if the texture reference
 is not bound to any CUDA mipmapped array.

 \param phMipmappedArray - Returned mipmapped array
 \param hTexRef          - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefGetMipmappedArray(
        phMipmappedArray: *mut cuda_types::cuda::CUmipmappedArray,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the addressing mode used by a texture reference

 \deprecated

 Returns in \p *pam the addressing mode corresponding to the
 dimension \p dim of the texture reference \p hTexRef. Currently, the only
 valid value for \p dim are 0 and 1.

 \param pam     - Returned addressing mode
 \param hTexRef - Texture reference
 \param dim     - Dimension

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefGetAddressMode(
        pam: *mut cuda_types::cuda::CUaddress_mode,
        hTexRef: cuda_types::cuda::CUtexref,
        dim: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the filter-mode used by a texture reference

 \deprecated

 Returns in \p *pfm the filtering mode of the texture reference
 \p hTexRef.

 \param pfm     - Returned filtering mode
 \param hTexRef - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefGetFilterMode(
        pfm: *mut cuda_types::cuda::CUfilter_mode,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the format used by a texture reference

 \deprecated

 Returns in \p *pFormat and \p *pNumChannels the format and number
 of components of the CUDA array bound to the texture reference \p hTexRef.
 If \p pFormat or \p pNumChannels is NULL, it will be ignored.

 \param pFormat      - Returned format
 \param pNumChannels - Returned number of components
 \param hTexRef      - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags*/
    fn cuTexRefGetFormat(
        pFormat: *mut cuda_types::cuda::CUarray_format,
        pNumChannels: *mut ::core::ffi::c_int,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the mipmap filtering mode for a texture reference

 \deprecated

 Returns the mipmap filtering mode in \p pfm that's used when reading memory through
 the texture reference \p hTexRef.

 \param pfm     - Returned mipmap filtering mode
 \param hTexRef - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefGetMipmapFilterMode(
        pfm: *mut cuda_types::cuda::CUfilter_mode,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the mipmap level bias for a texture reference

 \deprecated

 Returns the mipmap level bias in \p pBias that's added to the specified mipmap
 level when reading memory through the texture reference \p hTexRef.

 \param pbias   - Returned mipmap level bias
 \param hTexRef - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefGetMipmapLevelBias(
        pbias: *mut f32,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the min/max mipmap level clamps for a texture reference

 \deprecated

 Returns the min/max mipmap level clamps in \p pminMipmapLevelClamp and \p pmaxMipmapLevelClamp
 that's used when reading memory through the texture reference \p hTexRef.

 \param pminMipmapLevelClamp - Returned mipmap min level clamp
 \param pmaxMipmapLevelClamp - Returned mipmap max level clamp
 \param hTexRef              - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefGetMipmapLevelClamp(
        pminMipmapLevelClamp: *mut f32,
        pmaxMipmapLevelClamp: *mut f32,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the maximum anisotropy for a texture reference

 \deprecated

 Returns the maximum anisotropy in \p pmaxAniso that's used when reading memory through
 the texture reference \p hTexRef.

 \param pmaxAniso - Returned maximum anisotropy
 \param hTexRef   - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFlags, ::cuTexRefGetFormat*/
    fn cuTexRefGetMaxAnisotropy(
        pmaxAniso: *mut ::core::ffi::c_int,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the border color used by a texture reference

 \deprecated

 Returns in \p pBorderColor, values of the RGBA color used by
 the texture reference \p hTexRef.
 The color value is of type float and holds color components in
 the following sequence:
 pBorderColor[0] holds 'R' component
 pBorderColor[1] holds 'G' component
 pBorderColor[2] holds 'B' component
 pBorderColor[3] holds 'A' component

 \param hTexRef  - Texture reference
 \param pBorderColor   - Returned Type and Value of RGBA color

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddressMode,
 ::cuTexRefSetAddressMode, ::cuTexRefSetBorderColor*/
    fn cuTexRefGetBorderColor(
        pBorderColor: *mut f32,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the flags used by a texture reference

 \deprecated

 Returns in \p *pFlags the flags of the texture reference \p hTexRef.

 \param pFlags  - Returned flags
 \param hTexRef - Texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefSetAddress,
 ::cuTexRefSetAddress2D, ::cuTexRefSetAddressMode, ::cuTexRefSetArray,
 ::cuTexRefSetFilterMode, ::cuTexRefSetFlags, ::cuTexRefSetFormat,
 ::cuTexRefGetAddress, ::cuTexRefGetAddressMode, ::cuTexRefGetArray,
 ::cuTexRefGetFilterMode, ::cuTexRefGetFormat*/
    fn cuTexRefGetFlags(
        pFlags: *mut ::core::ffi::c_uint,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a texture reference

 \deprecated

 Creates a texture reference and returns its handle in \p *pTexRef. Once
 created, the application must call ::cuTexRefSetArray() or
 ::cuTexRefSetAddress() to associate the reference with allocated memory.
 Other texture reference functions are used to specify the format and
 interpretation (addressing, filtering, etc.) to be used when the memory is
 read through this texture reference.

 \param pTexRef - Returned texture reference

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefDestroy*/
    fn cuTexRefCreate(
        pTexRef: *mut cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys a texture reference

 \deprecated

 Destroys the texture reference specified by \p hTexRef.

 \param hTexRef - Texture reference to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuTexRefCreate*/
    fn cuTexRefDestroy(
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets the CUDA array for a surface reference.

 \deprecated

 Sets the CUDA array \p hArray to be read and written by the surface reference
 \p hSurfRef.  Any previous CUDA array state associated with the surface
 reference is superseded by this function.  \p Flags must be set to 0.
 The ::CUDA_ARRAY3D_SURFACE_LDST flag must have been set for the CUDA array.
 Any CUDA array previously bound to \p hSurfRef is unbound.

 \param hSurfRef - Surface reference handle
 \param hArray - CUDA array handle
 \param Flags - set to 0

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuModuleGetSurfRef,
 ::cuSurfRefGetArray*/
    fn cuSurfRefSetArray(
        hSurfRef: cuda_types::cuda::CUsurfref,
        hArray: cuda_types::cuda::CUarray,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Passes back the CUDA array bound to a surface reference.

 \deprecated

 Returns in \p *phArray the CUDA array bound to the surface reference
 \p hSurfRef, or returns ::CUDA_ERROR_INVALID_VALUE if the surface reference
 is not bound to any CUDA array.

 \param phArray - Surface reference handle
 \param hSurfRef - Surface reference handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa ::cuModuleGetSurfRef, ::cuSurfRefSetArray*/
    fn cuSurfRefGetArray(
        phArray: *mut cuda_types::cuda::CUarray,
        hSurfRef: cuda_types::cuda::CUsurfref,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a texture object

 Creates a texture object and returns it in \p pTexObject. \p pResDesc describes
 the data to texture from. \p pTexDesc describes how the data should be sampled.
 \p pResViewDesc is an optional argument that specifies an alternate format for
 the data described by \p pResDesc, and also describes the subresource region
 to restrict access to when texturing. \p pResViewDesc can only be specified if
 the type of resource is a CUDA array or a CUDA mipmapped array not in a block
 compressed format.

 Texture objects are only supported on devices of compute capability 3.0 or higher.
 Additionally, a texture object is an opaque value, and, as such, should only be
 accessed through CUDA API calls.

 The ::CUDA_RESOURCE_DESC structure is defined as:
 \code
typedef struct CUDA_RESOURCE_DESC_st
{
CUresourcetype resType;

union {
struct {
CUarray hArray;
} array;
struct {
CUmipmappedArray hMipmappedArray;
} mipmap;
struct {
CUdeviceptr devPtr;
CUarray_format format;
unsigned int numChannels;
size_t sizeInBytes;
} linear;
struct {
CUdeviceptr devPtr;
CUarray_format format;
unsigned int numChannels;
size_t width;
size_t height;
size_t pitchInBytes;
} pitch2D;
} res;

unsigned int flags;
} CUDA_RESOURCE_DESC;

 \endcode
 where:
 - ::CUDA_RESOURCE_DESC::resType specifies the type of resource to texture from.
 CUresourceType is defined as:
 \code
typedef enum CUresourcetype_enum {
CU_RESOURCE_TYPE_ARRAY           = 0x00,
CU_RESOURCE_TYPE_MIPMAPPED_ARRAY = 0x01,
CU_RESOURCE_TYPE_LINEAR          = 0x02,
CU_RESOURCE_TYPE_PITCH2D         = 0x03
} CUresourcetype;
 \endcode

 \par
 If ::CUDA_RESOURCE_DESC::resType is set to ::CU_RESOURCE_TYPE_ARRAY, ::CUDA_RESOURCE_DESC::res::array::hArray
 must be set to a valid CUDA array handle.

 \par
 If ::CUDA_RESOURCE_DESC::resType is set to ::CU_RESOURCE_TYPE_MIPMAPPED_ARRAY, ::CUDA_RESOURCE_DESC::res::mipmap::hMipmappedArray
 must be set to a valid CUDA mipmapped array handle.

 \par
 If ::CUDA_RESOURCE_DESC::resType is set to ::CU_RESOURCE_TYPE_LINEAR, ::CUDA_RESOURCE_DESC::res::linear::devPtr
 must be set to a valid device pointer, that is aligned to ::CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT.
 ::CUDA_RESOURCE_DESC::res::linear::format and ::CUDA_RESOURCE_DESC::res::linear::numChannels
 describe the format of each component and the number of components per array element. ::CUDA_RESOURCE_DESC::res::linear::sizeInBytes
 specifies the size of the array in bytes. The total number of elements in the linear address range cannot exceed
 ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH. The number of elements is computed as (sizeInBytes / (sizeof(format) * numChannels)).

 \par
 If ::CUDA_RESOURCE_DESC::resType is set to ::CU_RESOURCE_TYPE_PITCH2D, ::CUDA_RESOURCE_DESC::res::pitch2D::devPtr
 must be set to a valid device pointer, that is aligned to ::CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT.
 ::CUDA_RESOURCE_DESC::res::pitch2D::format and ::CUDA_RESOURCE_DESC::res::pitch2D::numChannels
 describe the format of each component and the number of components per array element. ::CUDA_RESOURCE_DESC::res::pitch2D::width
 and ::CUDA_RESOURCE_DESC::res::pitch2D::height specify the width and height of the array in elements, and cannot exceed
 ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH and ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT respectively.
 ::CUDA_RESOURCE_DESC::res::pitch2D::pitchInBytes specifies the pitch between two rows in bytes and has to be aligned to
 ::CU_DEVICE_ATTRIBUTE_TEXTURE_PITCH_ALIGNMENT. Pitch cannot exceed ::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH.

 - ::flags must be set to zero.


 The ::CUDA_TEXTURE_DESC struct is defined as
 \code
typedef struct CUDA_TEXTURE_DESC_st {
CUaddress_mode addressMode[3];
CUfilter_mode filterMode;
unsigned int flags;
unsigned int maxAnisotropy;
CUfilter_mode mipmapFilterMode;
float mipmapLevelBias;
float minMipmapLevelClamp;
float maxMipmapLevelClamp;
} CUDA_TEXTURE_DESC;
 \endcode
 where
 - ::CUDA_TEXTURE_DESC::addressMode specifies the addressing mode for each dimension of the texture data. ::CUaddress_mode is defined as:
   \code
typedef enum CUaddress_mode_enum {
CU_TR_ADDRESS_MODE_WRAP = 0,
CU_TR_ADDRESS_MODE_CLAMP = 1,
CU_TR_ADDRESS_MODE_MIRROR = 2,
CU_TR_ADDRESS_MODE_BORDER = 3
} CUaddress_mode;
   \endcode
   This is ignored if ::CUDA_RESOURCE_DESC::resType is ::CU_RESOURCE_TYPE_LINEAR. Also, if the flag, ::CU_TRSF_NORMALIZED_COORDINATES
   is not set, the only supported address mode is ::CU_TR_ADDRESS_MODE_CLAMP.

 - ::CUDA_TEXTURE_DESC::filterMode specifies the filtering mode to be used when fetching from the texture. CUfilter_mode is defined as:
   \code
typedef enum CUfilter_mode_enum {
CU_TR_FILTER_MODE_POINT = 0,
CU_TR_FILTER_MODE_LINEAR = 1
} CUfilter_mode;
   \endcode
   This is ignored if ::CUDA_RESOURCE_DESC::resType is ::CU_RESOURCE_TYPE_LINEAR.

 - ::CUDA_TEXTURE_DESC::flags can be any combination of the following:
   - ::CU_TRSF_READ_AS_INTEGER, which suppresses the default behavior of
   having the texture promote integer data to floating point data in the
   range [0, 1]. Note that texture with 32-bit integer format would not be
   promoted, regardless of whether or not this flag is specified.
   - ::CU_TRSF_NORMALIZED_COORDINATES, which suppresses the default behavior
   of having the texture coordinates range from [0, Dim) where Dim is the
   width or height of the CUDA array. Instead, the texture coordinates
   [0, 1.0) reference the entire breadth of the array dimension; Note that
   for CUDA mipmapped arrays, this flag has to be set.
   - ::CU_TRSF_DISABLE_TRILINEAR_OPTIMIZATION, which disables any trilinear
   filtering optimizations. Trilinear optimizations improve texture filtering
   performance by allowing bilinear filtering on textures in scenarios where
   it can closely approximate the expected results.
   - ::CU_TRSF_SEAMLESS_CUBEMAP, which enables seamless cube map filtering.
   This flag can only be specified if the underlying resource is a CUDA array
   or a CUDA mipmapped array that was created with the flag ::CUDA_ARRAY3D_CUBEMAP.
   When seamless cube map filtering is enabled, texture address modes specified
   by ::CUDA_TEXTURE_DESC::addressMode are ignored. Instead, if the ::CUDA_TEXTURE_DESC::filterMode
   is set to ::CU_TR_FILTER_MODE_POINT the address mode ::CU_TR_ADDRESS_MODE_CLAMP
   will be applied for all dimensions. If the ::CUDA_TEXTURE_DESC::filterMode is
   set to ::CU_TR_FILTER_MODE_LINEAR seamless cube map filtering will be performed
   when sampling along the cube face borders.

 - ::CUDA_TEXTURE_DESC::maxAnisotropy specifies the maximum anisotropy ratio to be used when doing anisotropic filtering. This value will be
   clamped to the range [1,16].

 - ::CUDA_TEXTURE_DESC::mipmapFilterMode specifies the filter mode when the calculated mipmap level lies between two defined mipmap levels.

 - ::CUDA_TEXTURE_DESC::mipmapLevelBias specifies the offset to be applied to the calculated mipmap level.

 - ::CUDA_TEXTURE_DESC::minMipmapLevelClamp specifies the lower end of the mipmap level range to clamp access to.

 - ::CUDA_TEXTURE_DESC::maxMipmapLevelClamp specifies the upper end of the mipmap level range to clamp access to.


 The ::CUDA_RESOURCE_VIEW_DESC struct is defined as
 \code
typedef struct CUDA_RESOURCE_VIEW_DESC_st
{
CUresourceViewFormat format;
size_t width;
size_t height;
size_t depth;
unsigned int firstMipmapLevel;
unsigned int lastMipmapLevel;
unsigned int firstLayer;
unsigned int lastLayer;
} CUDA_RESOURCE_VIEW_DESC;
 \endcode
 where:
 - ::CUDA_RESOURCE_VIEW_DESC::format specifies how the data contained in the CUDA array or CUDA mipmapped array should
   be interpreted. Note that this can incur a change in size of the texture data. If the resource view format is a block
   compressed format, then the underlying CUDA array or CUDA mipmapped array has to have a base of format ::CU_AD_FORMAT_UNSIGNED_INT32.
   with 2 or 4 channels, depending on the block compressed format. For ex., BC1 and BC4 require the underlying CUDA array to have
   a format of ::CU_AD_FORMAT_UNSIGNED_INT32 with 2 channels. The other BC formats require the underlying resource to have the same base
   format but with 4 channels.

 - ::CUDA_RESOURCE_VIEW_DESC::width specifies the new width of the texture data. If the resource view format is a block
   compressed format, this value has to be 4 times the original width of the resource. For non block compressed formats,
   this value has to be equal to that of the original resource.

 - ::CUDA_RESOURCE_VIEW_DESC::height specifies the new height of the texture data. If the resource view format is a block
   compressed format, this value has to be 4 times the original height of the resource. For non block compressed formats,
   this value has to be equal to that of the original resource.

 - ::CUDA_RESOURCE_VIEW_DESC::depth specifies the new depth of the texture data. This value has to be equal to that of the
   original resource.

 - ::CUDA_RESOURCE_VIEW_DESC::firstMipmapLevel specifies the most detailed mipmap level. This will be the new mipmap level zero.
   For non-mipmapped resources, this value has to be zero.::CUDA_TEXTURE_DESC::minMipmapLevelClamp and ::CUDA_TEXTURE_DESC::maxMipmapLevelClamp
   will be relative to this value. For ex., if the firstMipmapLevel is set to 2, and a minMipmapLevelClamp of 1.2 is specified,
   then the actual minimum mipmap level clamp will be 3.2.

 - ::CUDA_RESOURCE_VIEW_DESC::lastMipmapLevel specifies the least detailed mipmap level. For non-mipmapped resources, this value
   has to be zero.

 - ::CUDA_RESOURCE_VIEW_DESC::firstLayer specifies the first layer index for layered textures. This will be the new layer zero.
   For non-layered resources, this value has to be zero.

 - ::CUDA_RESOURCE_VIEW_DESC::lastLayer specifies the last layer index for layered textures. For non-layered resources,
   this value has to be zero.


 \param pTexObject   - Texture object to create
 \param pResDesc     - Resource descriptor
 \param pTexDesc     - Texture descriptor
 \param pResViewDesc - Resource view descriptor

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexObjectDestroy,
 ::cudaCreateTextureObject*/
    fn cuTexObjectCreate(
        pTexObject: *mut cuda_types::cuda::CUtexObject,
        pResDesc: *const cuda_types::cuda::CUDA_RESOURCE_DESC,
        pTexDesc: *const cuda_types::cuda::CUDA_TEXTURE_DESC,
        pResViewDesc: *const cuda_types::cuda::CUDA_RESOURCE_VIEW_DESC,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys a texture object

 Destroys the texture object specified by \p texObject.

 \param texObject - Texture object to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexObjectCreate,
 ::cudaDestroyTextureObject*/
    fn cuTexObjectDestroy(
        texObject: cuda_types::cuda::CUtexObject,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a texture object's resource descriptor

 Returns the resource descriptor for the texture object specified by \p texObject.

 \param pResDesc  - Resource descriptor
 \param texObject - Texture object

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexObjectCreate,
 ::cudaGetTextureObjectResourceDesc,*/
    fn cuTexObjectGetResourceDesc(
        pResDesc: *mut cuda_types::cuda::CUDA_RESOURCE_DESC,
        texObject: cuda_types::cuda::CUtexObject,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a texture object's texture descriptor

 Returns the texture descriptor for the texture object specified by \p texObject.

 \param pTexDesc  - Texture descriptor
 \param texObject - Texture object

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexObjectCreate,
 ::cudaGetTextureObjectTextureDesc*/
    fn cuTexObjectGetTextureDesc(
        pTexDesc: *mut cuda_types::cuda::CUDA_TEXTURE_DESC,
        texObject: cuda_types::cuda::CUtexObject,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a texture object's resource view descriptor

 Returns the resource view descriptor for the texture object specified by \p texObject.
 If no resource view was set for \p texObject, the ::CUDA_ERROR_INVALID_VALUE is returned.

 \param pResViewDesc - Resource view descriptor
 \param texObject    - Texture object

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTexObjectCreate,
 ::cudaGetTextureObjectResourceViewDesc*/
    fn cuTexObjectGetResourceViewDesc(
        pResViewDesc: *mut cuda_types::cuda::CUDA_RESOURCE_VIEW_DESC,
        texObject: cuda_types::cuda::CUtexObject,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a surface object

 Creates a surface object and returns it in \p pSurfObject. \p pResDesc describes
 the data to perform surface load/stores on. ::CUDA_RESOURCE_DESC::resType must be
 ::CU_RESOURCE_TYPE_ARRAY and  ::CUDA_RESOURCE_DESC::res::array::hArray
 must be set to a valid CUDA array handle. ::CUDA_RESOURCE_DESC::flags must be set to zero.

 Surface objects are only supported on devices of compute capability 3.0 or higher.
 Additionally, a surface object is an opaque value, and, as such, should only be
 accessed through CUDA API calls.

 \param pSurfObject - Surface object to create
 \param pResDesc    - Resource descriptor

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuSurfObjectDestroy,
 ::cudaCreateSurfaceObject*/
    fn cuSurfObjectCreate(
        pSurfObject: *mut cuda_types::cuda::CUsurfObject,
        pResDesc: *const cuda_types::cuda::CUDA_RESOURCE_DESC,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys a surface object

 Destroys the surface object specified by \p surfObject.

 \param surfObject - Surface object to destroy

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuSurfObjectCreate,
 ::cudaDestroySurfaceObject*/
    fn cuSurfObjectDestroy(
        surfObject: cuda_types::cuda::CUsurfObject,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns a surface object's resource descriptor

 Returns the resource descriptor for the surface object specified by \p surfObject.

 \param pResDesc   - Resource descriptor
 \param surfObject - Surface object

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuSurfObjectCreate,
 ::cudaGetSurfaceObjectResourceDesc*/
    fn cuSurfObjectGetResourceDesc(
        pResDesc: *mut cuda_types::cuda::CUDA_RESOURCE_DESC,
        surfObject: cuda_types::cuda::CUsurfObject,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a tensor map descriptor object representing tiled memory region

 Creates a descriptor for Tensor Memory Access (TMA) object specified
 by the parameters describing a tiled region and returns it in \p tensorMap.

 Tensor map objects are only supported on devices of compute capability 9.0 or higher.
 Additionally, a tensor map object is an opaque value, and, as such, should only be
 accessed through CUDA APIs and PTX.

 The parameters passed are bound to the following requirements:

 - \p tensorMap address must be aligned to 64 bytes.

 - \p tensorDataType has to be an enum from ::CUtensorMapDataType which is defined as:
 \code
typedef enum CUtensorMapDataType_enum {
CU_TENSOR_MAP_DATA_TYPE_UINT8 = 0,       // 1 byte
CU_TENSOR_MAP_DATA_TYPE_UINT16,          // 2 bytes
CU_TENSOR_MAP_DATA_TYPE_UINT32,          // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_INT32,           // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_UINT64,          // 8 bytes
CU_TENSOR_MAP_DATA_TYPE_INT64,           // 8 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT16,         // 2 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT32,         // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT64,         // 8 bytes
CU_TENSOR_MAP_DATA_TYPE_BFLOAT16,        // 2 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT32_FTZ,     // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_TFLOAT32,        // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_TFLOAT32_FTZ,    // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B,    // 4 bits
CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B,   // 4 bits
CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B    // 6 bits
} CUtensorMapDataType;
 \endcode
  ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B copies '16 x U4' packed values to memory aligned as 8 bytes. There are no gaps between packed values.
  ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B copies '16 x U4' packed values to memory aligned as 16 bytes. There are 8 byte gaps between every 8 byte chunk of packed values.
  ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B copies '16 x U6' packed values to memory aligned as 16 bytes. There are 4 byte gaps between every 12 byte chunk of packed values.

 - \p tensorRank must be non-zero and less than or equal to the maximum supported dimensionality of 5. If \p interleave is not
 ::CU_TENSOR_MAP_INTERLEAVE_NONE, then \p tensorRank must additionally be greater than or equal to 3.

 - \p globalAddress, which specifies the starting address of the memory region described, must be 16 byte aligned. The following requirements need to also be met:
    - When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_32B, \p globalAddress must be 32 byte aligned.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, \p globalAddress must be 32 byte aligned.

 - \p globalDim array, which specifies tensor size of each of the \p tensorRank dimensions, must be non-zero and less than or
 equal to 2^32. Additionally, the following requirements need to be met for the packed data types:
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, globalDim[0] must be a multiple of 128.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B, \p globalDim[0] must be a multiple of 2.
    - Dimension for the packed data types must reflect the number of individual U# values.

 - \p globalStrides array, which specifies tensor stride of each of the lower \p tensorRank - 1 dimensions in bytes, must be a
 multiple of 16 and less than 2^40. Additionally, the following requirements need to be met:
    - When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_32B, the strides must be a multiple of 32.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, the strides must be a multiple of 32.
 Each following dimension specified includes previous dimension stride:
 \code
globalStrides[0] = globalDim[0] * elementSizeInBytes(tensorDataType) + padding[0];
for (i = 1; i < tensorRank - 1; i++)
globalStrides[i] = globalStrides[i – 1] * (globalDim[i] + padding[i]);
assert(globalStrides[i] >= globalDim[i]);
 \endcode

 - \p boxDim array, which specifies number of elements to be traversed along each of the \p tensorRank dimensions, must be non-zero
 and less than or equal to 256. Additionally, the following requirements need to be met:
    - When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_NONE, { \p boxDim[0] * elementSizeInBytes( \p tensorDataType ) } must be a multiple of 16 bytes.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, boxDim[0] must be 128.

 - \p elementStrides array, which specifies the iteration step along each of the \p tensorRank dimensions, must be non-zero and less
 than or equal to 8. Note that when \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_NONE, the first element of this array is ignored since
 TMA doesn’t support the stride for dimension zero.
 When all elements of \p elementStrides array is one, \p boxDim specifies the number of elements to load. However, if the \p elementStrides[i]
 is not equal to one, then TMA loads ceil( \p boxDim[i] / \p elementStrides[i]) number of elements along i-th dimension. To load N elements along
 i-th dimension, \p boxDim[i] must be set to N * \p elementStrides[i].

 - \p interleave specifies the interleaved layout of type ::CUtensorMapInterleave, which is defined as:
 \code
typedef enum CUtensorMapInterleave_enum {
CU_TENSOR_MAP_INTERLEAVE_NONE = 0,
CU_TENSOR_MAP_INTERLEAVE_16B,
CU_TENSOR_MAP_INTERLEAVE_32B
} CUtensorMapInterleave;
 \endcode
 TMA supports interleaved layouts like NC/8HWC8 where C8 utilizes 16 bytes in memory assuming 2 byte per channel or NC/16HWC16 where C16
 uses 32 bytes.
 When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_NONE and \p swizzle is not ::CU_TENSOR_MAP_SWIZZLE_NONE, the bounding box inner dimension
 (computed as \p boxDim[0] multiplied by element size derived from \p tensorDataType) must be less than or equal to the swizzle size.
    - CU_TENSOR_MAP_SWIZZLE_32B requires the bounding box inner dimension to be <= 32.
    - CU_TENSOR_MAP_SWIZZLE_64B requires the bounding box inner dimension to be <= 64.
    - CU_TENSOR_MAP_SWIZZLE_128B* require the bounding box inner dimension to be <= 128.
 Additionally, \p tensorDataType of ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B requires \p interleave to be ::CU_TENSOR_MAP_INTERLEAVE_NONE.

 - \p swizzle, which specifies the shared memory bank swizzling pattern, has to be of type ::CUtensorMapSwizzle which is defined as:
 \code
typedef enum CUtensorMapSwizzle_enum {
CU_TENSOR_MAP_SWIZZLE_NONE = 0,
CU_TENSOR_MAP_SWIZZLE_32B,                   // Swizzle 16B chunks within 32B  span
CU_TENSOR_MAP_SWIZZLE_64B,                   // Swizzle 16B chunks within 64B  span
CU_TENSOR_MAP_SWIZZLE_128B,                  // Swizzle 16B chunks within 128B span
CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B,         // Swizzle 32B chunks within 128B span
CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B_FLIP_8B, // Swizzle 32B chunks within 128B span, additionally swap lower 8B with upper 8B within each 16B for every alternate row
CU_TENSOR_MAP_SWIZZLE_128B_ATOM_64B          // Swizzle 64B chunks within 128B span
} CUtensorMapSwizzle;
 \endcode
 Data are organized in a specific order in global memory; however, this may not match the order in which the application accesses data
 in shared memory. This difference in data organization may cause bank conflicts when shared memory is accessed. In order to avoid this
 problem, data can be loaded to shared memory with shuffling across shared memory banks.
 When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_32B, \p swizzle must be ::CU_TENSOR_MAP_SWIZZLE_32B.
 Other interleave modes can have any swizzling pattern.
 When the \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B, only the following swizzle modes are supported:
    - CU_TENSOR_MAP_SWIZZLE_NONE (Load & Store)
    - CU_TENSOR_MAP_SWIZZLE_128B (Load & Store)
    - CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B (Load & Store)
    - CU_TENSOR_MAP_SWIZZLE_128B_ATOM_64B (Store only)
 When the \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, only the following swizzle modes are supported:
    - CU_TENSOR_MAP_SWIZZLE_NONE (Load only)
    - CU_TENSOR_MAP_SWIZZLE_128B (Load only)
    - CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B (Load only)

 - \p l2Promotion specifies L2 fetch size which indicates the byte granurality at which L2 requests is filled from DRAM. It must be of
 type ::CUtensorMapL2promotion, which is defined as:
 \code
typedef enum CUtensorMapL2promotion_enum {
CU_TENSOR_MAP_L2_PROMOTION_NONE = 0,
CU_TENSOR_MAP_L2_PROMOTION_L2_64B,
CU_TENSOR_MAP_L2_PROMOTION_L2_128B,
CU_TENSOR_MAP_L2_PROMOTION_L2_256B
} CUtensorMapL2promotion;
 \endcode

 - \p oobFill, which indicates whether zero or a special NaN constant should be used to fill out-of-bound elements, must be of type
 ::CUtensorMapFloatOOBfill which is defined as:
 \code
typedef enum CUtensorMapFloatOOBfill_enum {
CU_TENSOR_MAP_FLOAT_OOB_FILL_NONE = 0,
CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA
} CUtensorMapFloatOOBfill;
 \endcode
 Note that ::CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA can only be used when \p tensorDataType represents a floating-point data type,
 and when \p tensorDataType is not ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B, ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, and ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B.

 \param tensorMap         - Tensor map object to create
 \param tensorDataType    - Tensor data type
 \param tensorRank        - Dimensionality of tensor
 \param globalAddress     - Starting address of memory region described by tensor
 \param globalDim         - Array containing tensor size (number of elements) along each of the \p tensorRank dimensions
 \param globalStrides     - Array containing stride size (in bytes) along each of the \p tensorRank - 1 dimensions
 \param boxDim            - Array containing traversal box size (number of elments) along each of the \p tensorRank dimensions. Specifies how many elements to be traversed along each tensor dimension.
 \param elementStrides    - Array containing traversal stride in each of the \p tensorRank dimensions
 \param interleave        - Type of interleaved layout the tensor addresses
 \param swizzle           - Bank swizzling pattern inside shared memory
 \param l2Promotion       - L2 promotion size
 \param oobFill           - Indicate whether zero or special NaN constant must be used to fill out-of-bound elements

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTensorMapEncodeIm2col,
 ::cuTensorMapEncodeIm2colWide,
 ::cuTensorMapReplaceAddress*/
    fn cuTensorMapEncodeTiled(
        tensorMap: *mut cuda_types::cuda::CUtensorMap,
        tensorDataType: cuda_types::cuda::CUtensorMapDataType,
        tensorRank: cuda_types::cuda::cuuint32_t,
        globalAddress: *mut ::core::ffi::c_void,
        globalDim: *const cuda_types::cuda::cuuint64_t,
        globalStrides: *const cuda_types::cuda::cuuint64_t,
        boxDim: *const cuda_types::cuda::cuuint32_t,
        elementStrides: *const cuda_types::cuda::cuuint32_t,
        interleave: cuda_types::cuda::CUtensorMapInterleave,
        swizzle: cuda_types::cuda::CUtensorMapSwizzle,
        l2Promotion: cuda_types::cuda::CUtensorMapL2promotion,
        oobFill: cuda_types::cuda::CUtensorMapFloatOOBfill,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a tensor map descriptor object representing im2col memory region

 Creates a descriptor for Tensor Memory Access (TMA) object specified
 by the parameters describing a im2col memory layout and returns it in \p tensorMap.

 Tensor map objects are only supported on devices of compute capability 9.0 or higher.
 Additionally, a tensor map object is an opaque value, and, as such, should only be
 accessed through CUDA APIs and PTX.

 The parameters passed are bound to the following requirements:

 - \p tensorMap address must be aligned to 64 bytes.

 - \p tensorDataType has to be an enum from ::CUtensorMapDataType which is defined as:
 \code
typedef enum CUtensorMapDataType_enum {
CU_TENSOR_MAP_DATA_TYPE_UINT8 = 0,       // 1 byte
CU_TENSOR_MAP_DATA_TYPE_UINT16,          // 2 bytes
CU_TENSOR_MAP_DATA_TYPE_UINT32,          // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_INT32,           // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_UINT64,          // 8 bytes
CU_TENSOR_MAP_DATA_TYPE_INT64,           // 8 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT16,         // 2 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT32,         // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT64,         // 8 bytes
CU_TENSOR_MAP_DATA_TYPE_BFLOAT16,        // 2 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT32_FTZ,     // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_TFLOAT32,        // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_TFLOAT32_FTZ     // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B,    // 4 bits
CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B,   // 4 bits
CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B    // 6 bits
} CUtensorMapDataType;
 \endcode
  ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B copies '16 x U4' packed values to memory aligned as 8 bytes. There are no gaps between packed values.
  ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B copies '16 x U4' packed values to memory aligned as 16 bytes. There are 8 byte gaps between every 8 byte chunk of packed values.
  ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B copies '16 x U6' packed values to memory aligned as 16 bytes. There are 4 byte gaps between every 12 byte chunk of packed values.

 - \p tensorRank, which specifies the number of tensor dimensions, must be 3, 4, or 5.

 - \p globalAddress, which specifies the starting address of the memory region described, must be 16 byte aligned. The following requirements need to also be met:
    - When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_32B, \p globalAddress must be 32 byte aligned.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, \p globalAddress must be 32 byte aligned.

 - \p globalDim array, which specifies tensor size of each of the \p tensorRank dimensions, must be non-zero and less than or
 equal to 2^32. Additionally, the following requirements need to be met for the packed data types:
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, globalDim[0] must be a multiple of 128.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B, \p globalDim[0] must be a multiple of 2.
    - Dimension for the packed data types must reflect the number of individual U# values.

 - \p globalStrides array, which specifies tensor stride of each of the lower \p tensorRank - 1 dimensions in bytes, must be a
 multiple of 16 and less than 2^40. Additionally, the following requirements need to be met:
    - When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_32B, the strides must be a multiple of 32.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, the strides must be a multiple of 32.
 Each following dimension specified includes previous dimension stride:
 \code
globalStrides[0] = globalDim[0] * elementSizeInBytes(tensorDataType) + padding[0];
for (i = 1; i < tensorRank - 1; i++)
globalStrides[i] = globalStrides[i – 1] * (globalDim[i] + padding[i]);
assert(globalStrides[i] >= globalDim[i]);
 \endcode

 - \p pixelBoxLowerCorner array specifies the coordinate offsets {D, H, W} of the bounding box from top/left/front corner. The number of
 offsets and their precision depend on the tensor dimensionality:
    - When \p tensorRank is 3, one signed offset within range [-32768, 32767] is supported.
    - When \p tensorRank is 4, two signed offsets each within range [-128, 127] are supported.
    - When \p tensorRank is 5, three offsets each within range [-16, 15] are supported.

 - \p pixelBoxUpperCorner array specifies the coordinate offsets {D, H, W} of the bounding box from bottom/right/back corner. The number of
 offsets and their precision depend on the tensor dimensionality:
    - When \p tensorRank is 3, one signed offset within range [-32768, 32767] is supported.
    - When \p tensorRank is 4, two signed offsets each within range [-128, 127] are supported.
    - When \p tensorRank is 5, three offsets each within range [-16, 15] are supported.
 The bounding box specified by \p pixelBoxLowerCorner and \p pixelBoxUpperCorner must have non-zero area.

 - \p channelsPerPixel, which specifies the number of elements which must be accessed along C dimension, must be less than or equal to 256.
 Additionally, when \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, \p channelsPerPixel must be 128.

 - \p pixelsPerColumn, which specifies the number of elements that must be accessed along the {N, D, H, W} dimensions, must be less than or
 equal to 1024.

 - \p elementStrides array, which specifies the iteration step along each of the \p tensorRank dimensions, must be non-zero and less
 than or equal to 8. Note that when \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_NONE, the first element of this array is ignored since
 TMA doesn’t support the stride for dimension zero.
 When all elements of the \p elementStrides array are one, \p boxDim specifies the number of elements to load. However, if \p elementStrides[i]
 is not equal to one for some \p i, then TMA loads ceil( \p boxDim[i] / \p elementStrides[i]) number of elements along i-th dimension.
 To load N elements along i-th dimension, \p boxDim[i] must be set to N * \p elementStrides[i].

 - \p interleave specifies the interleaved layout of type ::CUtensorMapInterleave, which is defined as:
 \code
typedef enum CUtensorMapInterleave_enum {
CU_TENSOR_MAP_INTERLEAVE_NONE = 0,
CU_TENSOR_MAP_INTERLEAVE_16B,
CU_TENSOR_MAP_INTERLEAVE_32B
} CUtensorMapInterleave;
 \endcode
 TMA supports interleaved layouts like NC/8HWC8 where C8 utilizes 16 bytes in memory assuming 2 byte per channel or NC/16HWC16 where C16
 uses 32 bytes.
 When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_NONE and \p swizzle is not ::CU_TENSOR_MAP_SWIZZLE_NONE, the bounding box inner dimension
 (computed as \p channelsPerPixel multiplied by element size in bytes derived from \p tensorDataType) must be less than or equal to the swizzle size.
    - CU_TENSOR_MAP_SWIZZLE_32B requires the bounding box inner dimension to be <= 32.
    - CU_TENSOR_MAP_SWIZZLE_64B requires the bounding box inner dimension to be <= 64.
    - CU_TENSOR_MAP_SWIZZLE_128B* require the bounding box inner dimension to be <= 128.
 Additionally, \p tensorDataType of ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B requires \p interleave to be ::CU_TENSOR_MAP_INTERLEAVE_NONE.

 - \p swizzle, which specifies the shared memory bank swizzling pattern, has to be of type ::CUtensorMapSwizzle which is defined as:
 \code
typedef enum CUtensorMapSwizzle_enum {
CU_TENSOR_MAP_SWIZZLE_NONE = 0,
CU_TENSOR_MAP_SWIZZLE_32B,                   // Swizzle 16B chunks within 32B  span
CU_TENSOR_MAP_SWIZZLE_64B,                   // Swizzle 16B chunks within 64B  span
CU_TENSOR_MAP_SWIZZLE_128B,                  // Swizzle 16B chunks within 128B span
CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B,         // Swizzle 32B chunks within 128B span
CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B_FLIP_8B, // Swizzle 32B chunks within 128B span, additionally swap lower 8B with upper 8B within each 16B for every alternate row
CU_TENSOR_MAP_SWIZZLE_128B_ATOM_64B          // Swizzle 64B chunks within 128B span
} CUtensorMapSwizzle;
 \endcode
 Data are organized in a specific order in global memory; however, this may not match the order in which the application accesses data
 in shared memory. This difference in data organization may cause bank conflicts when shared memory is accessed. In order to avoid this
 problem, data can be loaded to shared memory with shuffling across shared memory banks.
 When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_32B, \p swizzle must be ::CU_TENSOR_MAP_SWIZZLE_32B.
 Other interleave modes can have any swizzling pattern.
 When the \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B, only the following swizzle modes are supported:
    - CU_TENSOR_MAP_SWIZZLE_NONE (Load & Store)
    - CU_TENSOR_MAP_SWIZZLE_128B (Load & Store)
    - CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B (Load & Store)
    - CU_TENSOR_MAP_SWIZZLE_128B_ATOM_64B (Store only)
 When the \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, only the following swizzle modes are supported:
    - CU_TENSOR_MAP_SWIZZLE_NONE (Load only)
    - CU_TENSOR_MAP_SWIZZLE_128B (Load only)
    - CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B (Load only)

 - \p l2Promotion specifies L2 fetch size which indicates the byte granularity at which L2 requests are filled from DRAM. It must be of
 type ::CUtensorMapL2promotion, which is defined as:
 \code
typedef enum CUtensorMapL2promotion_enum {
CU_TENSOR_MAP_L2_PROMOTION_NONE = 0,
CU_TENSOR_MAP_L2_PROMOTION_L2_64B,
CU_TENSOR_MAP_L2_PROMOTION_L2_128B,
CU_TENSOR_MAP_L2_PROMOTION_L2_256B
} CUtensorMapL2promotion;
 \endcode

 - \p oobFill, which indicates whether zero or a special NaN constant should be used to fill out-of-bound elements, must be of type
 ::CUtensorMapFloatOOBfill which is defined as:
 \code
typedef enum CUtensorMapFloatOOBfill_enum {
CU_TENSOR_MAP_FLOAT_OOB_FILL_NONE = 0,
CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA
} CUtensorMapFloatOOBfill;
 \endcode
 Note that ::CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA can only be used when \p tensorDataType represents a floating-point data type,
 and when \p tensorDataType is not ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B, ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, and ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B.

 \param tensorMap             - Tensor map object to create
 \param tensorDataType        - Tensor data type
 \param tensorRank            - Dimensionality of tensor; must be at least 3
 \param globalAddress         - Starting address of memory region described by tensor
 \param globalDim             - Array containing tensor size (number of elements) along each of the \p tensorRank dimensions
 \param globalStrides         - Array containing stride size (in bytes) along each of the \p tensorRank - 1 dimensions
 \param pixelBoxLowerCorner   - Array containing DHW dimensions of lower box corner
 \param pixelBoxUpperCorner   - Array containing DHW dimensions of upper box corner
 \param channelsPerPixel      - Number of channels per pixel
 \param pixelsPerColumn       - Number of pixels per column
 \param elementStrides        - Array containing traversal stride in each of the \p tensorRank dimensions
 \param interleave            - Type of interleaved layout the tensor addresses
 \param swizzle               - Bank swizzling pattern inside shared memory
 \param l2Promotion           - L2 promotion size
 \param oobFill               - Indicate whether zero or special NaN constant will be used to fill out-of-bound elements

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTensorMapEncodeTiled,
 ::cuTensorMapEncodeIm2colWide,
 ::cuTensorMapReplaceAddress*/
    fn cuTensorMapEncodeIm2col(
        tensorMap: *mut cuda_types::cuda::CUtensorMap,
        tensorDataType: cuda_types::cuda::CUtensorMapDataType,
        tensorRank: cuda_types::cuda::cuuint32_t,
        globalAddress: *mut ::core::ffi::c_void,
        globalDim: *const cuda_types::cuda::cuuint64_t,
        globalStrides: *const cuda_types::cuda::cuuint64_t,
        pixelBoxLowerCorner: *const ::core::ffi::c_int,
        pixelBoxUpperCorner: *const ::core::ffi::c_int,
        channelsPerPixel: cuda_types::cuda::cuuint32_t,
        pixelsPerColumn: cuda_types::cuda::cuuint32_t,
        elementStrides: *const cuda_types::cuda::cuuint32_t,
        interleave: cuda_types::cuda::CUtensorMapInterleave,
        swizzle: cuda_types::cuda::CUtensorMapSwizzle,
        l2Promotion: cuda_types::cuda::CUtensorMapL2promotion,
        oobFill: cuda_types::cuda::CUtensorMapFloatOOBfill,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a tensor map descriptor object representing im2col memory region, but where
 the elements are exclusively loaded along the W dimension.

 Creates a descriptor for Tensor Memory Access (TMA) object specified by the parameters
 describing a im2col memory layout and where the row is always loaded along the W dimensuin
 and returns it in \p tensorMap. This assumes the tensor layout in memory is either NDHWC,
 NHWC, or NWC.

 This API is only supported on devices of compute capability 10.0 or higher.
 Additionally, a tensor map object is an opaque value, and, as such, should only be
 accessed through CUDA APIs and PTX.

 The parameters passed are bound to the following requirements:

 - \p tensorMap address must be aligned to 64 bytes.

 - \p tensorDataType has to be an enum from ::CUtensorMapDataType which is defined as:
 \code
typedef enum CUtensorMapDataType_enum {
CU_TENSOR_MAP_DATA_TYPE_UINT8 = 0,       // 1 byte
CU_TENSOR_MAP_DATA_TYPE_UINT16,          // 2 bytes
CU_TENSOR_MAP_DATA_TYPE_UINT32,          // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_INT32,           // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_UINT64,          // 8 bytes
CU_TENSOR_MAP_DATA_TYPE_INT64,           // 8 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT16,         // 2 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT32,         // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT64,         // 8 bytes
CU_TENSOR_MAP_DATA_TYPE_BFLOAT16,        // 2 bytes
CU_TENSOR_MAP_DATA_TYPE_FLOAT32_FTZ,     // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_TFLOAT32,        // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_TFLOAT32_FTZ     // 4 bytes
CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B,    // 4 bits
CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B,   // 4 bits
CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B    // 6 bits
} CUtensorMapDataType;
 \endcode
  ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B copies '16 x U4' packed values to memory aligned as 8 bytes. There are no gaps between packed values.
  ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B copies '16 x U4' packed values to memory aligned as 16 bytes. There are 8 byte gaps between every 8 byte chunk of packed values.
  ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B copies '16 x U6' packed values to memory aligned as 16 bytes. There are 4 byte gaps between every 12 byte chunk of packed values.

 - \p tensorRank, which specifies the number of tensor dimensions, must be 3, 4, or 5.

 - \p globalAddress, which specifies the starting address of the memory region described, must be 16 byte aligned. The following requirements need to also be met:
    - When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_32B, \p globalAddress must be 32 byte aligned.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, \p globalAddress must be 32 byte aligned.

 - \p globalDim array, which specifies tensor size of each of the \p tensorRank dimensions, must be non-zero and less than or
 equal to 2^32. Additionally, the following requirements need to be met for the packed data types:
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, globalDim[0] must be a multiple of 128.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B, \p globalDim[0] must be a multiple of 2.
    - Dimension for the packed data types must reflect the number of individual U# values.

 - \p globalStrides array, which specifies tensor stride of each of the lower \p tensorRank - 1 dimensions in bytes, must be a
 multiple of 16 and less than 2^40. Additionally, the following requirements need to be met:
    - When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_32B, the strides must be a multiple of 32.
    - When \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, the strides must be a multiple of 32.
 Each following dimension specified includes previous dimension stride:
 \code
globalStrides[0] = globalDim[0] * elementSizeInBytes(tensorDataType) + padding[0];
for (i = 1; i < tensorRank - 1; i++)
globalStrides[i] = globalStrides[i – 1] * (globalDim[i] + padding[i]);
assert(globalStrides[i] >= globalDim[i]);
 \endcode

 - \p pixelBoxLowerCornerWidth specifies the coordinate offset W of the bounding box from left corner. The offset must be
 within range [-32768, 32767].

 - \p pixelBoxUpperCornerWidth specifies the coordinate offset W of the bounding box from right corner. The offset must be
 within range [-32768, 32767].

 The bounding box specified by \p pixelBoxLowerCornerWidth and \p pixelBoxUpperCornerWidth must have non-zero area. Note
 that the size of the box along D and H dimensions is always equal to one.

 - \p channelsPerPixel, which specifies the number of elements which must be accessed along C dimension, must be less than or equal to 256.
 Additionally, when \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B or ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, \p channelsPerPixel must be 128.

 - \p pixelsPerColumn, which specifies the number of elements that must be accessed along the W dimension, must be less than or
 equal to 1024. This field is ignored when \p mode is ::CU_TENSOR_MAP_IM2COL_WIDE_MODE_W128.

 - \p elementStrides array, which specifies the iteration step along each of the \p tensorRank dimensions, must be non-zero and less
 than or equal to 8. Note that when \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_NONE, the first element of this array is ignored since
 TMA doesn’t support the stride for dimension zero.
 When all elements of the \p elementStrides array are one, \p boxDim specifies the number of elements to load. However, if \p elementStrides[i]
 is not equal to one for some \p i, then TMA loads ceil( \p boxDim[i] / \p elementStrides[i]) number of elements along i-th dimension.
 To load N elements along i-th dimension, \p boxDim[i] must be set to N * \p elementStrides[i].

 - \p interleave specifies the interleaved layout of type ::CUtensorMapInterleave, which is defined as:
 \code
typedef enum CUtensorMapInterleave_enum {
CU_TENSOR_MAP_INTERLEAVE_NONE = 0,
CU_TENSOR_MAP_INTERLEAVE_16B,
CU_TENSOR_MAP_INTERLEAVE_32B
} CUtensorMapInterleave;
 \endcode
 TMA supports interleaved layouts like NC/8HWC8 where C8 utilizes 16 bytes in memory assuming 2 byte per channel or NC/16HWC16 where C16
 uses 32 bytes.
 When \p interleave is ::CU_TENSOR_MAP_INTERLEAVE_NONE, the bounding box inner dimension (computed as \p channelsPerPixel multiplied by
 element size in bytes derived from \p tensorDataType) must be less than or equal to the swizzle size.
    - CU_TENSOR_MAP_SWIZZLE_64B requires the bounding box inner dimension to be <= 64.
    - CU_TENSOR_MAP_SWIZZLE_128B* require the bounding box inner dimension to be <= 128.
 Additionally, \p tensorDataType of ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B requires \p interleave to be ::CU_TENSOR_MAP_INTERLEAVE_NONE.

 - \p mode, which describes loading of elements loaded along the W dimension, has to be one of the following ::CUtensorMapIm2ColWideMode types:
 \code
          CU_TENSOR_MAP_IM2COL_WIDE_MODE_W,
          CU_TENSOR_MAP_IM2COL_WIDE_MODE_W128
 \endcode
 ::CU_TENSOR_MAP_IM2COL_WIDE_MODE_W allows the number of elements loaded along the W dimension to be specified
 via the \p pixelsPerColumn field.

 - \p swizzle, which specifies the shared memory bank swizzling pattern, must be one of the following
 ::CUtensorMapSwizzle modes (other swizzle modes are not supported):
 \code
typedef enum CUtensorMapSwizzle_enum {
CU_TENSOR_MAP_SWIZZLE_64B,                   // Swizzle 16B chunks within 64B  span
CU_TENSOR_MAP_SWIZZLE_128B,                  // Swizzle 16B chunks within 128B span
CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B,         // Swizzle 32B chunks within 128B span
} CUtensorMapSwizzle;
 \endcode
 Data are organized in a specific order in global memory; however, this may not match the order in which the application accesses data
 in shared memory. This difference in data organization may cause bank conflicts when shared memory is accessed. In order to avoid this
 problem, data can be loaded to shared memory with shuffling across shared memory banks.
 When the \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B, only the following swizzle modes are supported:
    - CU_TENSOR_MAP_SWIZZLE_128B (Load & Store)
    - CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B (Load & Store)
 When the \p tensorDataType is ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, only the following swizzle modes are supported:
    - CU_TENSOR_MAP_SWIZZLE_128B (Load only)
    - CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B (Load only)

 - \p l2Promotion specifies L2 fetch size which indicates the byte granularity at which L2 requests are filled from DRAM. It must be of
 type ::CUtensorMapL2promotion, which is defined as:
 \code
typedef enum CUtensorMapL2promotion_enum {
CU_TENSOR_MAP_L2_PROMOTION_NONE = 0,
CU_TENSOR_MAP_L2_PROMOTION_L2_64B,
CU_TENSOR_MAP_L2_PROMOTION_L2_128B,
CU_TENSOR_MAP_L2_PROMOTION_L2_256B
} CUtensorMapL2promotion;
 \endcode

 - \p oobFill, which indicates whether zero or a special NaN constant should be used to fill out-of-bound elements, must be of type
 ::CUtensorMapFloatOOBfill which is defined as:
 \code
typedef enum CUtensorMapFloatOOBfill_enum {
CU_TENSOR_MAP_FLOAT_OOB_FILL_NONE = 0,
CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA
} CUtensorMapFloatOOBfill;
 \endcode
 Note that ::CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA can only be used when \p tensorDataType represents a floating-point data type,
 and when \p tensorDataType is not ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B, ::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B, and ::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B.

 \param tensorMap                - Tensor map object to create
 \param tensorDataType           - Tensor data type
 \param tensorRank               - Dimensionality of tensor; must be at least 3
 \param globalAddress            - Starting address of memory region described by tensor
 \param globalDim                - Array containing tensor size (number of elements) along each of the \p tensorRank dimensions
 \param globalStrides            - Array containing stride size (in bytes) along each of the \p tensorRank - 1 dimensions
 \param pixelBoxLowerCornerWidth - Width offset of left box corner
 \param pixelBoxUpperCornerWidth - Width offset of right box corner
 \param channelsPerPixel         - Number of channels per pixel
 \param pixelsPerColumn          - Number of pixels per column
 \param elementStrides           - Array containing traversal stride in each of the \p tensorRank dimensions
 \param interleave               - Type of interleaved layout the tensor addresses
 \param mode                     - W or W128 mode
 \param swizzle                  - Bank swizzling pattern inside shared memory
 \param l2Promotion              - L2 promotion size
 \param oobFill                  - Indicate whether zero or special NaN constant will be used to fill out-of-bound elements

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTensorMapEncodeTiled,
 ::cuTensorMapEncodeIm2col,
 ::cuTensorMapReplaceAddress*/
    fn cuTensorMapEncodeIm2colWide(
        tensorMap: *mut cuda_types::cuda::CUtensorMap,
        tensorDataType: cuda_types::cuda::CUtensorMapDataType,
        tensorRank: cuda_types::cuda::cuuint32_t,
        globalAddress: *mut ::core::ffi::c_void,
        globalDim: *const cuda_types::cuda::cuuint64_t,
        globalStrides: *const cuda_types::cuda::cuuint64_t,
        pixelBoxLowerCornerWidth: ::core::ffi::c_int,
        pixelBoxUpperCornerWidth: ::core::ffi::c_int,
        channelsPerPixel: cuda_types::cuda::cuuint32_t,
        pixelsPerColumn: cuda_types::cuda::cuuint32_t,
        elementStrides: *const cuda_types::cuda::cuuint32_t,
        interleave: cuda_types::cuda::CUtensorMapInterleave,
        mode: cuda_types::cuda::CUtensorMapIm2ColWideMode,
        swizzle: cuda_types::cuda::CUtensorMapSwizzle,
        l2Promotion: cuda_types::cuda::CUtensorMapL2promotion,
        oobFill: cuda_types::cuda::CUtensorMapFloatOOBfill,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Modify an existing tensor map descriptor with an updated global address

 Modifies the descriptor for Tensor Memory Access (TMA) object passed in \p tensorMap with
 an updated \p globalAddress.

 Tensor map objects are only supported on devices of compute capability 9.0 or higher.
 Additionally, a tensor map object is an opaque value, and, as such, should only be
 accessed through CUDA API calls.

 \param tensorMap             - Tensor map object to modify
 \param globalAddress         - Starting address of memory region described by tensor, must follow previous alignment requirements

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuTensorMapEncodeTiled,
 ::cuTensorMapEncodeIm2col,
 ::cuTensorMapEncodeIm2colWide*/
    fn cuTensorMapReplaceAddress(
        tensorMap: *mut cuda_types::cuda::CUtensorMap,
        globalAddress: *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Queries if a device may directly access a peer device's memory.

 Returns in \p *canAccessPeer a value of 1 if contexts on \p dev are capable of
 directly accessing memory from contexts on \p peerDev and 0 otherwise.
 If direct access of \p peerDev from \p dev is possible, then access may be
 enabled on two specific contexts by calling ::cuCtxEnablePeerAccess().

 \param canAccessPeer - Returned access capability
 \param dev           - Device from which allocations on \p peerDev are to
                        be directly accessed.
 \param peerDev       - Device on which the allocations to be directly accessed
                        by \p dev reside.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_DEVICE
 \notefnerr

 \sa
 ::cuCtxEnablePeerAccess,
 ::cuCtxDisablePeerAccess,
 ::cudaDeviceCanAccessPeer*/
    fn cuDeviceCanAccessPeer(
        canAccessPeer: *mut ::core::ffi::c_int,
        dev: cuda_types::cuda::CUdevice,
        peerDev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Enables direct access to memory allocations in a peer context.

 If both the current context and \p peerContext are on devices which support unified
 addressing (as may be queried using ::CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING) and same
 major compute capability, then on success all allocations from \p peerContext will
 immediately be accessible by the current context.  See \ref CUDA_UNIFIED for additional
 details.

 Note that access granted by this call is unidirectional and that in order to access
 memory from the current context in \p peerContext, a separate symmetric call
 to ::cuCtxEnablePeerAccess() is required.

 Note that there are both device-wide and system-wide limitations per system
 configuration, as noted in the CUDA Programming Guide under the section
 "Peer-to-Peer Memory Access".

 Returns ::CUDA_ERROR_PEER_ACCESS_UNSUPPORTED if ::cuDeviceCanAccessPeer() indicates
 that the ::CUdevice of the current context cannot directly access memory
 from the ::CUdevice of \p peerContext.

 Returns ::CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED if direct access of
 \p peerContext from the current context has already been enabled.

 Returns ::CUDA_ERROR_TOO_MANY_PEERS if direct peer access is not possible
 because hardware resources required for peer access have been exhausted.

 Returns ::CUDA_ERROR_INVALID_CONTEXT if there is no current context, \p peerContext
 is not a valid context, or if the current context is \p peerContext.

 Returns ::CUDA_ERROR_INVALID_VALUE if \p Flags is not 0.

 \param peerContext - Peer context to enable direct access to from the current context
 \param Flags       - Reserved for future use and must be set to 0

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED,
 ::CUDA_ERROR_TOO_MANY_PEERS,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_PEER_ACCESS_UNSUPPORTED,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::cuDeviceCanAccessPeer,
 ::cuCtxDisablePeerAccess,
 ::cudaDeviceEnablePeerAccess*/
    fn cuCtxEnablePeerAccess(
        peerContext: cuda_types::cuda::CUcontext,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Disables direct access to memory allocations in a peer context and
 unregisters any registered allocations.

Returns ::CUDA_ERROR_PEER_ACCESS_NOT_ENABLED if direct peer access has
 not yet been enabled from \p peerContext to the current context.

 Returns ::CUDA_ERROR_INVALID_CONTEXT if there is no current context, or if
 \p peerContext is not a valid context.

 \param peerContext - Peer context to disable direct access to

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_PEER_ACCESS_NOT_ENABLED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 \notefnerr

 \sa
 ::cuDeviceCanAccessPeer,
 ::cuCtxEnablePeerAccess,
 ::cudaDeviceDisablePeerAccess*/
    fn cuCtxDisablePeerAccess(
        peerContext: cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Queries attributes of the link between two devices.

 Returns in \p *value the value of the requested attribute \p attrib of the
 link between \p srcDevice and \p dstDevice. The supported attributes are:
 - ::CU_DEVICE_P2P_ATTRIBUTE_PERFORMANCE_RANK: A relative value indicating the
   performance of the link between two devices.
 - ::CU_DEVICE_P2P_ATTRIBUTE_ACCESS_SUPPORTED P2P: 1 if P2P Access is enable.
 - ::CU_DEVICE_P2P_ATTRIBUTE_NATIVE_ATOMIC_SUPPORTED: 1 if Atomic operations over
   the link are supported.
 - ::CU_DEVICE_P2P_ATTRIBUTE_CUDA_ARRAY_ACCESS_SUPPORTED: 1 if cudaArray can
   be accessed over the link.

 Returns ::CUDA_ERROR_INVALID_DEVICE if \p srcDevice or \p dstDevice are not valid
 or if they represent the same device.

 Returns ::CUDA_ERROR_INVALID_VALUE if \p attrib is not valid or if \p value is
 a null pointer.

 \param value         - Returned value of the requested attribute
 \param attrib        - The requested attribute of the link between \p srcDevice and \p dstDevice.
 \param srcDevice     - The source device of the target link.
 \param dstDevice     - The destination device of the target link.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa
 ::cuCtxEnablePeerAccess,
 ::cuCtxDisablePeerAccess,
 ::cuDeviceCanAccessPeer,
 ::cudaDeviceGetP2PAttribute*/
    fn cuDeviceGetP2PAttribute(
        value: *mut ::core::ffi::c_int,
        attrib: cuda_types::cuda::CUdevice_P2PAttribute,
        srcDevice: cuda_types::cuda::CUdevice,
        dstDevice: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unregisters a graphics resource for access by CUDA

 Unregisters the graphics resource \p resource so it is not accessible by
 CUDA unless registered again.

 If \p resource is invalid then ::CUDA_ERROR_INVALID_HANDLE is
 returned.

 \param resource - Resource to unregister

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa
 ::cuGraphicsD3D9RegisterResource,
 ::cuGraphicsD3D10RegisterResource,
 ::cuGraphicsD3D11RegisterResource,
 ::cuGraphicsGLRegisterBuffer,
 ::cuGraphicsGLRegisterImage,
 ::cudaGraphicsUnregisterResource*/
    fn cuGraphicsUnregisterResource(
        resource: cuda_types::cuda::CUgraphicsResource,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get an array through which to access a subresource of a mapped graphics resource.

 Returns in \p *pArray an array through which the subresource of the mapped
 graphics resource \p resource which corresponds to array index \p arrayIndex
 and mipmap level \p mipLevel may be accessed.  The value set in \p *pArray may
 change every time that \p resource is mapped.

 If \p resource is not a texture then it cannot be accessed via an array and
 ::CUDA_ERROR_NOT_MAPPED_AS_ARRAY is returned.
 If \p arrayIndex is not a valid array index for \p resource then
 ::CUDA_ERROR_INVALID_VALUE is returned.
 If \p mipLevel is not a valid mipmap level for \p resource then
 ::CUDA_ERROR_INVALID_VALUE is returned.
 If \p resource is not mapped then ::CUDA_ERROR_NOT_MAPPED is returned.

 \param pArray      - Returned array through which a subresource of \p resource may be accessed
 \param resource    - Mapped resource to access
 \param arrayIndex  - Array index for array textures or cubemap face
                      index as defined by ::CUarray_cubemap_face for
                      cubemap textures for the subresource to access
 \param mipLevel    - Mipmap level for the subresource to access

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_MAPPED,
 ::CUDA_ERROR_NOT_MAPPED_AS_ARRAY
 \notefnerr

 \sa
 ::cuGraphicsResourceGetMappedPointer,
 ::cudaGraphicsSubResourceGetMappedArray*/
    fn cuGraphicsSubResourceGetMappedArray(
        pArray: *mut cuda_types::cuda::CUarray,
        resource: cuda_types::cuda::CUgraphicsResource,
        arrayIndex: ::core::ffi::c_uint,
        mipLevel: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get a mipmapped array through which to access a mapped graphics resource.

 Returns in \p *pMipmappedArray a mipmapped array through which the mapped graphics
 resource \p resource. The value set in \p *pMipmappedArray may change every time
 that \p resource is mapped.

 If \p resource is not a texture then it cannot be accessed via a mipmapped array and
 ::CUDA_ERROR_NOT_MAPPED_AS_ARRAY is returned.
 If \p resource is not mapped then ::CUDA_ERROR_NOT_MAPPED is returned.

 \param pMipmappedArray - Returned mipmapped array through which \p resource may be accessed
 \param resource        - Mapped resource to access

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_MAPPED,
 ::CUDA_ERROR_NOT_MAPPED_AS_ARRAY
 \notefnerr

 \sa
 ::cuGraphicsResourceGetMappedPointer,
 ::cudaGraphicsResourceGetMappedMipmappedArray*/
    fn cuGraphicsResourceGetMappedMipmappedArray(
        pMipmappedArray: *mut cuda_types::cuda::CUmipmappedArray,
        resource: cuda_types::cuda::CUgraphicsResource,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get a device pointer through which to access a mapped graphics resource.

 Returns in \p *pDevPtr a pointer through which the mapped graphics resource
 \p resource may be accessed.
 Returns in \p pSize the size of the memory in bytes which may be accessed from that pointer.
 The value set in \p pPointer may change every time that \p resource is mapped.

 If \p resource is not a buffer then it cannot be accessed via a pointer and
 ::CUDA_ERROR_NOT_MAPPED_AS_POINTER is returned.
 If \p resource is not mapped then ::CUDA_ERROR_NOT_MAPPED is returned.
 *
 \param pDevPtr    - Returned pointer through which \p resource may be accessed
 \param pSize      - Returned size of the buffer accessible starting at \p *pPointer
 \param resource   - Mapped resource to access

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_MAPPED,
 ::CUDA_ERROR_NOT_MAPPED_AS_POINTER
 \notefnerr

 \sa
 ::cuGraphicsMapResources,
 ::cuGraphicsSubResourceGetMappedArray,
 ::cudaGraphicsResourceGetMappedPointer*/
    fn cuGraphicsResourceGetMappedPointer_v2(
        pDevPtr: *mut cuda_types::cuda::CUdeviceptr,
        pSize: *mut usize,
        resource: cuda_types::cuda::CUgraphicsResource,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Set usage flags for mapping a graphics resource

 Set \p flags for mapping the graphics resource \p resource.

 Changes to \p flags will take effect the next time \p resource is mapped.
 The \p flags argument may be any of the following:

 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_NONE: Specifies no hints about how this
   resource will be used. It is therefore assumed that this resource will be
   read from and written to by CUDA kernels.  This is the default value.
 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_READONLY: Specifies that CUDA kernels which
   access this resource will not write to this resource.
 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_WRITEDISCARD: Specifies that CUDA kernels
   which access this resource will not read from this resource and will
   write over the entire contents of the resource, so none of the data
   previously stored in the resource will be preserved.

 If \p resource is presently mapped for access by CUDA then
 ::CUDA_ERROR_ALREADY_MAPPED is returned.
 If \p flags is not one of the above values then ::CUDA_ERROR_INVALID_VALUE is returned.

 \param resource - Registered resource to set flags for
 \param flags    - Parameters for resource mapping

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ALREADY_MAPPED
 \notefnerr

 \sa
 ::cuGraphicsMapResources,
 ::cudaGraphicsResourceSetMapFlags*/
    fn cuGraphicsResourceSetMapFlags_v2(
        resource: cuda_types::cuda::CUgraphicsResource,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Map graphics resources for access by CUDA

 Maps the \p count graphics resources in \p resources for access by CUDA.

 The resources in \p resources may be accessed by CUDA until they
 are unmapped. The graphics API from which \p resources were registered
 should not access any resources while they are mapped by CUDA. If an
 application does so, the results are undefined.

 This function provides the synchronization guarantee that any graphics calls
 issued before ::cuGraphicsMapResources() will complete before any subsequent CUDA
 work issued in \p stream begins.

 If \p resources includes any duplicate entries then ::CUDA_ERROR_INVALID_HANDLE is returned.
 If any of \p resources are presently mapped for access by CUDA then ::CUDA_ERROR_ALREADY_MAPPED is returned.

 \param count      - Number of resources to map
 \param resources  - Resources to map for CUDA usage
 \param hStream    - Stream with which to synchronize

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ALREADY_MAPPED,
 ::CUDA_ERROR_UNKNOWN
 \note_null_stream
 \notefnerr

 \sa
 ::cuGraphicsResourceGetMappedPointer,
 ::cuGraphicsSubResourceGetMappedArray,
 ::cuGraphicsUnmapResources,
 ::cudaGraphicsMapResources*/
    fn cuGraphicsMapResources_ptsz(
        count: ::core::ffi::c_uint,
        resources: *mut cuda_types::cuda::CUgraphicsResource,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unmap graphics resources.

 Unmaps the \p count graphics resources in \p resources.

 Once unmapped, the resources in \p resources may not be accessed by CUDA
 until they are mapped again.

 This function provides the synchronization guarantee that any CUDA work issued
 in \p stream before ::cuGraphicsUnmapResources() will complete before any
 subsequently issued graphics work begins.


 If \p resources includes any duplicate entries then ::CUDA_ERROR_INVALID_HANDLE is returned.
 If any of \p resources are not presently mapped for access by CUDA then ::CUDA_ERROR_NOT_MAPPED is returned.

 \param count      - Number of resources to unmap
 \param resources  - Resources to unmap
 \param hStream    - Stream with which to synchronize

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_MAPPED,
 ::CUDA_ERROR_UNKNOWN
 \note_null_stream
 \notefnerr

 \sa
 ::cuGraphicsMapResources,
 ::cudaGraphicsUnmapResources*/
    fn cuGraphicsUnmapResources_ptsz(
        count: ::core::ffi::c_uint,
        resources: *mut cuda_types::cuda::CUgraphicsResource,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the requested driver API function pointer

 Returns in \p **pfn the address of the CUDA driver function for the requested
 CUDA version and flags.

 The CUDA version is specified as (1000 * major + 10 * minor), so CUDA 11.2
 should be specified as 11020. For a requested driver symbol, if the specified
 CUDA version is greater than or equal to the CUDA version in which the driver symbol
 was introduced, this API will return the function pointer to the corresponding
 versioned function.

 The pointer returned by the API should be cast to a function pointer matching the
 requested driver function's definition in the API header file. The function pointer
 typedef can be picked up from the corresponding typedefs header file. For example,
 cudaTypedefs.h consists of function pointer typedefs for driver APIs defined in cuda.h.

 The API will return ::CUDA_SUCCESS and set the returned \p pfn to NULL if the
 requested driver function is not supported on the platform, no ABI
 compatible driver function exists for the specified \p cudaVersion or if the
 driver symbol is invalid.

 It will also set the optional \p symbolStatus to one of the values in
 ::CUdriverProcAddressQueryResult with the following meanings:
 - ::CU_GET_PROC_ADDRESS_SUCCESS - The requested symbol was succesfully found based
   on input arguments and \p pfn is valid
 - ::CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND - The requested symbol was not found
 - ::CU_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT - The requested symbol was found but is
   not supported by cudaVersion specified

 The requested flags can be:
 - ::CU_GET_PROC_ADDRESS_DEFAULT: This is the default mode. This is equivalent to
   ::CU_GET_PROC_ADDRESS_PER_THREAD_DEFAULT_STREAM if the code is compiled with
   --default-stream per-thread compilation flag or the macro CUDA_API_PER_THREAD_DEFAULT_STREAM
   is defined; ::CU_GET_PROC_ADDRESS_LEGACY_STREAM otherwise.
 - ::CU_GET_PROC_ADDRESS_LEGACY_STREAM: This will enable the search for all driver symbols
   that match the requested driver symbol name except the corresponding per-thread versions.
 - ::CU_GET_PROC_ADDRESS_PER_THREAD_DEFAULT_STREAM: This will enable the search for all
   driver symbols that match the requested driver symbol name including the per-thread
   versions. If a per-thread version is not found, the API will return the legacy version
   of the driver function.

 \param symbol - The base name of the driver API function to look for. As an example,
                 for the driver API ::cuMemAlloc_v2, \p symbol would be cuMemAlloc and
                 \p cudaVersion would be the ABI compatible CUDA version for the _v2 variant.
 \param pfn - Location to return the function pointer to the requested driver function
 \param cudaVersion - The CUDA version to look for the requested driver symbol
 \param flags -  Flags to specify search options.
 \param symbolStatus - Optional location to store the status of the search for
                       \p symbol based on \p cudaVersion. See ::CUdriverProcAddressQueryResult
                       for possible values.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED
 \note_version_mixing

 \sa
 ::cudaGetDriverEntryPoint*/
    fn cuGetProcAddress_v2(
        symbol: *const ::core::ffi::c_char,
        pfn: *mut *mut ::core::ffi::c_void,
        cudaVersion: ::core::ffi::c_int,
        flags: cuda_types::cuda::cuuint64_t,
        symbolStatus: *mut cuda_types::cuda::CUdriverProcAddressQueryResult,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allows caller to fetch a coredump attribute value for the current context

 Returns in \p *value the requested value specified by \p attrib. It is up to the caller
 to ensure that the data type and size of \p *value matches the request.

 If the caller calls this function with \p *value equal to NULL, the size of the memory
 region (in bytes) expected for \p attrib will be placed in \p size.

 The supported attributes are:
 - ::CU_COREDUMP_ENABLE_ON_EXCEPTION: Bool where ::true means that GPU exceptions from
      this context will create a coredump at the location specified by ::CU_COREDUMP_FILE.
      The default value is ::false unless set to ::true globally or locally, or the
      CU_CTX_USER_COREDUMP_ENABLE flag was set during context creation.
 - ::CU_COREDUMP_TRIGGER_HOST: Bool where ::true means that the host CPU will
      also create a coredump. The default value is ::true unless set to ::false globally or
      or locally. This value is deprecated as of CUDA 12.5 - raise the ::CU_COREDUMP_SKIP_ABORT
      flag to disable host device abort() if needed.
 - ::CU_COREDUMP_LIGHTWEIGHT: Bool where ::true means that any resulting coredumps
      will not have a dump of GPU memory or non-reloc ELF images. The default value is
      ::false unless set to ::true globally or locally. This attribute is deprecated as
      of CUDA 12.5, please use ::CU_COREDUMP_GENERATION_FLAGS instead.
 - ::CU_COREDUMP_ENABLE_USER_TRIGGER: Bool where ::true means that a coredump can be
      created by writing to the system pipe specified by ::CU_COREDUMP_PIPE. The default
      value is ::false unless set to ::true globally or locally.
 - ::CU_COREDUMP_FILE: String of up to 1023 characters that defines the location where
      any coredumps generated by this context will be written. The default value is
      ::core.cuda.HOSTNAME.PID where ::HOSTNAME is the host name of the machine running
      the CUDA applications and ::PID is the process ID of the CUDA application.
 - ::CU_COREDUMP_PIPE: String of up to 1023 characters that defines the name of the pipe
      that will be monitored if user-triggered coredumps are enabled. The default value is
      ::corepipe.cuda.HOSTNAME.PID where ::HOSTNAME is the host name of the machine running
      the CUDA application and ::PID is the process ID of the CUDA application.
 - ::CU_COREDUMP_GENERATION_FLAGS: An integer with values to allow granular control the data
      contained in a coredump specified as a bitwise OR combination of the following values:
      + ::CU_COREDUMP_DEFAULT_FLAGS - if set by itself, coredump generation returns to its
          default settings of including all memory regions that it is able to access
      + ::CU_COREDUMP_SKIP_NONRELOCATED_ELF_IMAGES - Coredump will not include the data from
          CUDA source modules that are not relocated at runtime.
      + ::CU_COREDUMP_SKIP_GLOBAL_MEMORY - Coredump will not include device-side global data
          that does not belong to any context.
      + ::CU_COREDUMP_SKIP_SHARED_MEMORY - Coredump will not include grid-scale shared memory
          for the warp that the dumped kernel belonged to.
      + ::CU_COREDUMP_SKIP_LOCAL_MEMORY - Coredump will not include local memory from the kernel.
      + ::CU_COREDUMP_LIGHTWEIGHT_FLAGS - Enables all of the above options. Equiavlent to setting
          the ::CU_COREDUMP_LIGHTWEIGHT attribute to ::true.
      + ::CU_COREDUMP_SKIP_ABORT - If set, GPU exceptions will not raise an abort() in the host CPU
          process. Same functional goal as ::CU_COREDUMP_TRIGGER_HOST but better reflects the default
          behavior.

 \param attrib - The enum defining which value to fetch.
 \param value - void* containing the requested data.
 \param size - The size of the memory region \p value points to.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED

 \sa
 ::cuCoredumpGetAttributeGlobal,
 ::cuCoredumpSetAttribute,
 ::cuCoredumpSetAttributeGlobal*/
    fn cuCoredumpGetAttribute(
        attrib: cuda_types::cuda::CUcoredumpSettings,
        value: *mut ::core::ffi::c_void,
        size: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allows caller to fetch a coredump attribute value for the entire application

 Returns in \p *value the requested value specified by \p attrib. It is up to the caller
 to ensure that the data type and size of \p *value matches the request.

 If the caller calls this function with \p *value equal to NULL, the size of the memory
 region (in bytes) expected for \p attrib will be placed in \p size.

 The supported attributes are:
 - ::CU_COREDUMP_ENABLE_ON_EXCEPTION: Bool where ::true means that GPU exceptions from
      this context will create a coredump at the location specified by ::CU_COREDUMP_FILE.
      The default value is ::false.
 - ::CU_COREDUMP_TRIGGER_HOST: Bool where ::true means that the host CPU will
      also create a coredump. The default value is ::true unless set to ::false globally or
      or locally. This value is deprecated as of CUDA 12.5 - raise the ::CU_COREDUMP_SKIP_ABORT
      flag to disable host device abort() if needed.
 - ::CU_COREDUMP_LIGHTWEIGHT: Bool where ::true means that any resulting coredumps
      will not have a dump of GPU memory or non-reloc ELF images. The default value is
      ::false. This attribute is deprecated as of CUDA 12.5, please use ::CU_COREDUMP_GENERATION_FLAGS
      instead.
 - ::CU_COREDUMP_ENABLE_USER_TRIGGER: Bool where ::true means that a coredump can be
      created by writing to the system pipe specified by ::CU_COREDUMP_PIPE. The default
      value is ::false.
 - ::CU_COREDUMP_FILE: String of up to 1023 characters that defines the location where
      any coredumps generated by this context will be written. The default value is
      ::core.cuda.HOSTNAME.PID where ::HOSTNAME is the host name of the machine running
      the CUDA applications and ::PID is the process ID of the CUDA application.
 - ::CU_COREDUMP_PIPE: String of up to 1023 characters that defines the name of the pipe
      that will be monitored if user-triggered coredumps are enabled. The default value is
      ::corepipe.cuda.HOSTNAME.PID where ::HOSTNAME is the host name of the machine running
      the CUDA application and ::PID is the process ID of the CUDA application.
 - ::CU_COREDUMP_GENERATION_FLAGS: An integer with values to allow granular control the data
      contained in a coredump specified as a bitwise OR combination of the following values:
      + ::CU_COREDUMP_DEFAULT_FLAGS - if set by itself, coredump generation returns to its
          default settings of including all memory regions that it is able to access
      + ::CU_COREDUMP_SKIP_NONRELOCATED_ELF_IMAGES - Coredump will not include the data from
          CUDA source modules that are not relocated at runtime.
      + ::CU_COREDUMP_SKIP_GLOBAL_MEMORY - Coredump will not include device-side global data
          that does not belong to any context.
      + ::CU_COREDUMP_SKIP_SHARED_MEMORY - Coredump will not include grid-scale shared memory
          for the warp that the dumped kernel belonged to.
      + ::CU_COREDUMP_SKIP_LOCAL_MEMORY - Coredump will not include local memory from the kernel.
      + ::CU_COREDUMP_LIGHTWEIGHT_FLAGS - Enables all of the above options. Equiavlent to setting
          the ::CU_COREDUMP_LIGHTWEIGHT attribute to ::true.
      + ::CU_COREDUMP_SKIP_ABORT - If set, GPU exceptions will not raise an abort() in the host CPU
          process. Same functional goal as ::CU_COREDUMP_TRIGGER_HOST but better reflects the default
          behavior.

 \param attrib - The enum defining which value to fetch.
 \param value - void* containing the requested data.
 \param size - The size of the memory region \p value points to.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuCoredumpGetAttribute,
 ::cuCoredumpSetAttribute,
 ::cuCoredumpSetAttributeGlobal*/
    fn cuCoredumpGetAttributeGlobal(
        attrib: cuda_types::cuda::CUcoredumpSettings,
        value: *mut ::core::ffi::c_void,
        size: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allows caller to set a coredump attribute value for the current context

 This function should be considered an alternate interface to the CUDA-GDB environment
 variables defined in this document: https://docs.nvidia.com/cuda/cuda-gdb/index.html#gpu-coredump

 An important design decision to note is that any coredump environment variable values
 set before CUDA initializes will take permanent precedence over any values set with this
 function. This decision was made to ensure no change in behavior for any users that
 may be currently using these variables to get coredumps.

 \p *value shall contain the requested value specified by \p set. It is up to the caller
 to ensure that the data type and size of \p *value matches the request.

 If the caller calls this function with \p *value equal to NULL, the size of the memory
 region (in bytes) expected for \p set will be placed in \p size.

 /note This function will return ::CUDA_ERROR_NOT_SUPPORTED if the caller attempts to set
 ::CU_COREDUMP_ENABLE_ON_EXCEPTION on a GPU of with Compute Capability < 6.0. ::cuCoredumpSetAttributeGlobal
 works on those platforms as an alternative.

 /note ::CU_COREDUMP_ENABLE_USER_TRIGGER and ::CU_COREDUMP_PIPE cannot be set on a per-context basis.

 The supported attributes are:
 - ::CU_COREDUMP_ENABLE_ON_EXCEPTION: Bool where ::true means that GPU exceptions from
      this context will create a coredump at the location specified by ::CU_COREDUMP_FILE.
      The default value is ::false.
 - ::CU_COREDUMP_TRIGGER_HOST: Bool where ::true means that the host CPU will
      also create a coredump. The default value is ::true unless set to ::false globally or
      or locally. This value is deprecated as of CUDA 12.5 - raise the ::CU_COREDUMP_SKIP_ABORT
      flag to disable host device abort() if needed.
 - ::CU_COREDUMP_LIGHTWEIGHT: Bool where ::true means that any resulting coredumps
      will not have a dump of GPU memory or non-reloc ELF images. The default value is
      ::false. This attribute is deprecated as of CUDA 12.5, please use ::CU_COREDUMP_GENERATION_FLAGS
      instead.
 - ::CU_COREDUMP_FILE: String of up to 1023 characters that defines the location where
      any coredumps generated by this context will be written. The default value is
      ::core.cuda.HOSTNAME.PID where ::HOSTNAME is the host name of the machine running
      the CUDA applications and ::PID is the process ID of the CUDA application.
 - ::CU_COREDUMP_GENERATION_FLAGS: An integer with values to allow granular control the data
      contained in a coredump specified as a bitwise OR combination of the following values:
      + ::CU_COREDUMP_DEFAULT_FLAGS - if set by itself, coredump generation returns to its
          default settings of including all memory regions that it is able to access
      + ::CU_COREDUMP_SKIP_NONRELOCATED_ELF_IMAGES - Coredump will not include the data from
          CUDA source modules that are not relocated at runtime.
      + ::CU_COREDUMP_SKIP_GLOBAL_MEMORY - Coredump will not include device-side global data
          that does not belong to any context.
      + ::CU_COREDUMP_SKIP_SHARED_MEMORY - Coredump will not include grid-scale shared memory
          for the warp that the dumped kernel belonged to.
      + ::CU_COREDUMP_SKIP_LOCAL_MEMORY - Coredump will not include local memory from the kernel.
      + ::CU_COREDUMP_LIGHTWEIGHT_FLAGS - Enables all of the above options. Equiavlent to setting
          the ::CU_COREDUMP_LIGHTWEIGHT attribute to ::true.
      + ::CU_COREDUMP_SKIP_ABORT - If set, GPU exceptions will not raise an abort() in the host CPU
          process. Same functional goal as ::CU_COREDUMP_TRIGGER_HOST but better reflects the default
          behavior.

 \param attrib - The enum defining which value to set.
 \param value - void* containing the requested data.
 \param size - The size of the memory region \p value points to.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_PERMITTED,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED,
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa
 ::cuCoredumpGetAttributeGlobal,
 ::cuCoredumpGetAttribute,
 ::cuCoredumpSetAttributeGlobal*/
    fn cuCoredumpSetAttribute(
        attrib: cuda_types::cuda::CUcoredumpSettings,
        value: *mut ::core::ffi::c_void,
        size: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Allows caller to set a coredump attribute value globally

 This function should be considered an alternate interface to the CUDA-GDB environment
 variables defined in this document: https://docs.nvidia.com/cuda/cuda-gdb/index.html#gpu-coredump

 An important design decision to note is that any coredump environment variable values
 set before CUDA initializes will take permanent precedence over any values set with this
 function. This decision was made to ensure no change in behavior for any users that
 may be currently using these variables to get coredumps.

 \p *value shall contain the requested value specified by \p set. It is up to the caller
 to ensure that the data type and size of \p *value matches the request.

 If the caller calls this function with \p *value equal to NULL, the size of the memory
 region (in bytes) expected for \p set will be placed in \p size.

 The supported attributes are:
 - ::CU_COREDUMP_ENABLE_ON_EXCEPTION: Bool where ::true means that GPU exceptions from
      this context will create a coredump at the location specified by ::CU_COREDUMP_FILE.
      The default value is ::false.
 - ::CU_COREDUMP_TRIGGER_HOST: Bool where ::true means that the host CPU will
      also create a coredump. The default value is ::true unless set to ::false globally or
      or locally. This value is deprecated as of CUDA 12.5 - raise the ::CU_COREDUMP_SKIP_ABORT
      flag to disable host device abort() if needed.
 - ::CU_COREDUMP_LIGHTWEIGHT: Bool where ::true means that any resulting coredumps
      will not have a dump of GPU memory or non-reloc ELF images. The default value is
      ::false. This attribute is deprecated as of CUDA 12.5, please use ::CU_COREDUMP_GENERATION_FLAGS
      instead.
 - ::CU_COREDUMP_ENABLE_USER_TRIGGER: Bool where ::true means that a coredump can be
      created by writing to the system pipe specified by ::CU_COREDUMP_PIPE. The default
      value is ::false.
 - ::CU_COREDUMP_FILE: String of up to 1023 characters that defines the location where
      any coredumps generated by this context will be written. The default value is
      ::core.cuda.HOSTNAME.PID where ::HOSTNAME is the host name of the machine running
      the CUDA applications and ::PID is the process ID of the CUDA application.
 - ::CU_COREDUMP_PIPE: String of up to 1023 characters that defines the name of the pipe
      that will be monitored if user-triggered coredumps are enabled. This value may not be
      changed after ::CU_COREDUMP_ENABLE_USER_TRIGGER is set to ::true. The default
      value is ::corepipe.cuda.HOSTNAME.PID where ::HOSTNAME is the host name of the machine
      running the CUDA application and ::PID is the process ID of the CUDA application.
 - ::CU_COREDUMP_GENERATION_FLAGS: An integer with values to allow granular control the data
      contained in a coredump specified as a bitwise OR combination of the following values:
      + ::CU_COREDUMP_DEFAULT_FLAGS - if set by itself, coredump generation returns to its
          default settings of including all memory regions that it is able to access
      + ::CU_COREDUMP_SKIP_NONRELOCATED_ELF_IMAGES - Coredump will not include the data from
          CUDA source modules that are not relocated at runtime.
      + ::CU_COREDUMP_SKIP_GLOBAL_MEMORY - Coredump will not include device-side global data
          that does not belong to any context.
      + ::CU_COREDUMP_SKIP_SHARED_MEMORY - Coredump will not include grid-scale shared memory
          for the warp that the dumped kernel belonged to.
      + ::CU_COREDUMP_SKIP_LOCAL_MEMORY - Coredump will not include local memory from the kernel.
      + ::CU_COREDUMP_LIGHTWEIGHT_FLAGS - Enables all of the above options. Equiavlent to setting
          the ::CU_COREDUMP_LIGHTWEIGHT attribute to ::true.
      + ::CU_COREDUMP_SKIP_ABORT - If set, GPU exceptions will not raise an abort() in the host CPU
          process. Same functional goal as ::CU_COREDUMP_TRIGGER_HOST but better reflects the default
          behavior.

 \param attrib - The enum defining which value to set.
 \param value - void* containing the requested data.
 \param size - The size of the memory region \p value points to.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_PERMITTED

 \sa
 ::cuCoredumpGetAttribute,
 ::cuCoredumpGetAttributeGlobal,
 ::cuCoredumpSetAttribute*/
    fn cuCoredumpSetAttributeGlobal(
        attrib: cuda_types::cuda::CUcoredumpSettings,
        value: *mut ::core::ffi::c_void,
        size: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    /// @}
    fn cuGetExportTable(
        ppExportTable: *mut *const ::core::ffi::c_void,
        pExportTableId: *const cuda_types::cuda::CUuuid,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates a green context with a specified set of resources.

 This API creates a green context with the resources specified in the descriptor \p desc and
 returns it in the handle represented by \p phCtx. This API will retain the primary context on device \p dev,
 which will is released when the green context is destroyed. It is advised to have the primary context active
 before calling this API to avoid the heavy cost of triggering primary context initialization and
 deinitialization multiple times.

 The API does not set the green context current. In order to set it current, you need to explicitly set it current
 by first converting the green context to a CUcontext using ::cuCtxFromGreenCtx and subsequently calling
 ::cuCtxSetCurrent / ::cuCtxPushCurrent. It should be noted that a green context can be current to only one
 thread at a time. There is no internal synchronization to make API calls accessing the same green context
 from multiple threads work.

 Note: The API is not supported on 32-bit platforms.

 \param phCtx - Pointer for the output handle to the green context
 \param desc - Descriptor generated via ::cuDevResourceGenerateDesc which contains the set of resources to be used
 \param dev - Device on which to create the green context.
 \param flags - One of the supported green context creation flags. \p CU_GREEN_CTX_DEFAULT_STREAM is required.

 The supported flags are:
 - \p CU_GREEN_CTX_DEFAULT_STREAM : Creates a default stream to use inside the green context. Required.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_NOT_SUPPORTED,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa
 ::cuGreenCtxDestroy,
 ::cuCtxFromGreenCtx,
 ::cuCtxSetCurrent,
 ::cuCtxPushCurrent,
 ::cuDevResourceGenerateDesc,
 ::cuDevicePrimaryCtxRetain,
 ::cuCtxCreate,
 ::cuCtxCreate_v3*/
    fn cuGreenCtxCreate(
        phCtx: *mut cuda_types::cuda::CUgreenCtx,
        desc: cuda_types::cuda::CUdevResourceDesc,
        dev: cuda_types::cuda::CUdevice,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Destroys a green context

 Destroys the green context, releasing the primary context of the device that this green context was created for.
 Any resources provisioned for this green context (that were initially available via the resource descriptor)
 are released as well.
 \param hCtx - Green context to be destroyed

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_CONTEXT_IS_DESTROYED

 \sa
 ::cuGreenCtxCreate,
 ::cuCtxDestroy*/
    fn cuGreenCtxDestroy(
        hCtx: cuda_types::cuda::CUgreenCtx,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Converts a green context into the primary context

 The API converts a green context into the primary context returned in \p pContext. It is important
 to note that the converted context \p pContext is a normal primary context but with
 the resources of the specified green context \p hCtx. Once converted, it can then
 be used to set the context current with ::cuCtxSetCurrent or with any of the CUDA APIs
 that accept a CUcontext parameter.

 Users are expected to call this API before calling any CUDA APIs that accept a
 CUcontext. Failing to do so will result in the APIs returning ::CUDA_ERROR_INVALID_CONTEXT.

 \param pContext Returned primary context with green context resources
 \param hCtx Green context to convert

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuGreenCtxCreate*/
    fn cuCtxFromGreenCtx(
        pContext: *mut cuda_types::cuda::CUcontext,
        hCtx: cuda_types::cuda::CUgreenCtx,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get device resources

 Get the \p type resources available to the \p device.
 This may often be the starting point for further partitioning or configuring of resources.

 Note: The API is not supported on 32-bit platforms.

 \param device - Device to get resource for
 \param resource - Output pointer to a CUdevResource structure
 \param type - Type of resource to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_RESOURCE_TYPE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_DEVICE

 \sa
 ::cuDevResourceGenerateDesc*/
    fn cuDeviceGetDevResource(
        device: cuda_types::cuda::CUdevice,
        resource: *mut cuda_types::cuda::CUdevResource,
        type_: cuda_types::cuda::CUdevResourceType,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get context resources

 Get the \p type resources available to the context represented by \p hCtx
 \param hCtx - Context to get resource for

 Note: The API is not supported on 32-bit platforms.

 \param resource - Output pointer to a CUdevResource structure
 \param type - Type of resource to retrieve

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_RESOURCE_TYPE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_CONTEXT

 \sa
 ::cuDevResourceGenerateDesc*/
    fn cuCtxGetDevResource(
        hCtx: cuda_types::cuda::CUcontext,
        resource: *mut cuda_types::cuda::CUdevResource,
        type_: cuda_types::cuda::CUdevResourceType,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get green context resources

 Get the \p type resources available to the green context represented by \p hCtx
 \param hCtx - Green context to get resource for
 \param resource - Output pointer to a CUdevResource structure
 \param type - Type of resource to retrieve

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_RESOURCE_TYPE,
 ::CUDA_ERROR_INVALID_VALUE

 \sa
 ::cuDevResourceGenerateDesc*/
    fn cuGreenCtxGetDevResource(
        hCtx: cuda_types::cuda::CUgreenCtx,
        resource: *mut cuda_types::cuda::CUdevResource,
        type_: cuda_types::cuda::CUdevResourceType,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Splits \p CU_DEV_RESOURCE_TYPE_SM resources.

 Splits \p CU_DEV_RESOURCE_TYPE_SM resources into \p nbGroups, adhering to the minimum SM count specified in \p minCount
 and the usage flags in \p useFlags. If \p result is NULL, the API simulates a split and provides the amount of groups that
 would be created in \p nbGroups. Otherwise, \p nbGroups must point to the amount of elements in \p result and on return,
 the API will overwrite \p nbGroups with the amount actually created. The groups are written to the array in \p result.
 \p nbGroups can be less than the total amount if a smaller number of groups is needed.

 This API is used to spatially partition the input resource. The input resource needs to come from one of
 ::cuDeviceGetDevResource, ::cuCtxGetDevResource, or ::cuGreenCtxGetDevResource.
 A limitation of the API is that the output results cannot be split again without
 first creating a descriptor and a green context with that descriptor.

 When creating the groups, the API will take into account the performance and functional characteristics of the
 input resource, and guarantee a split that will create a disjoint set of symmetrical partitions. This may lead to fewer groups created
 than purely dividing the total SM count by the \p minCount due to cluster requirements or
 alignment and granularity requirements for the minCount.

 The \p remainder set does not have the same functional or performance guarantees as the groups in \p result.
 Its use should be carefully planned and future partitions of the \p remainder set are discouraged.

 The following flags are supported:
 - \p CU_DEV_SM_RESOURCE_SPLIT_IGNORE_SM_COSCHEDULING : Lower the minimum SM count and alignment, and treat each SM independent of its hierarchy.
  This allows more fine grained partitions but at the cost of advanced features (such as large clusters on compute capability 9.0+).
 - \p CU_DEV_SM_RESOURCE_SPLIT_MAX_POTENTIAL_CLUSTER_SIZE : Compute Capability 9.0+ only. Attempt to create groups that may allow
  for maximally sized thread clusters. This can be queried post green context creation using ::cuOccupancyMaxPotentialClusterSize.

 A successful API call must either have:
 - A valid array of \p result pointers of size passed in \p nbGroups, with \p input of type \p CU_DEV_RESOURCE_TYPE_SM.
 Value of \p minCount must be between 0 and the SM count specified in \p input. \p remaining may be NULL.
 - NULL passed in for \p result, with a valid integer pointer in \p nbGroups and \p input of type \p CU_DEV_RESOURCE_TYPE_SM.
 Value of \p minCount must be between 0 and the SM count specified in \p input. \p remaining may be NULL.
 This queries the number of groups that would be created by the API.

 Note: The API is not supported on 32-bit platforms.

 \param result - Output array of \p CUdevResource resources. Can be NULL to query the number of groups.
 \param nbGroups - This is a pointer, specifying the number of groups that would be or should be created as described below.
 \param input - Input SM resource to be split. Must be a valid \p CU_DEV_RESOURCE_TYPE_SM resource.
 \param remaining - If the input resource cannot be cleanly split among \p nbGroups, the remaining is placed in here.
 Can be ommitted (NULL) if the user does not need the remaining set.
 \param useFlags - Flags specifying how these partitions are used or which constraints to abide by when splitting the input. Zero is valid for default behavior.
 \param minCount - Minimum number of SMs required

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_RESOURCE_TYPE,
 ::CUDA_ERROR_INVALID_RESOURCE_CONFIGURATION

 \sa
 ::cuGreenCtxGetDevResource,
 ::cuCtxGetDevResource,
 ::cuDeviceGetDevResource*/
    fn cuDevSmResourceSplitByCount(
        result: *mut cuda_types::cuda::CUdevResource,
        nbGroups: *mut ::core::ffi::c_uint,
        input: *const cuda_types::cuda::CUdevResource,
        remaining: *mut cuda_types::cuda::CUdevResource,
        useFlags: ::core::ffi::c_uint,
        minCount: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Generate a resource descriptor

 Generates a single resource descriptor with the set of resources specified in \p resources.
 The generated resource descriptor is necessary for the creation of green contexts via the ::cuGreenCtxCreate API.
 Resources of the same type can be passed in, provided they meet the requirements as noted below.

 A successful API call must have:
 - A valid output pointer for the \p phDesc descriptor as well as a valid array of \p resources pointers,
 with the array size passed in \p nbResources.
 If multiple resources are provided in \p resources, the device they came from must be the same,
 otherwise CUDA_ERROR_INVALID_RESOURCE_CONFIGURATION is returned.
 If multiple resources are provided in \p resources and they are of type ::CU_DEV_RESOURCE_TYPE_SM,
 they must be outputs (whether \p result or \p remaining) from the same split API instance,
 otherwise CUDA_ERROR_INVALID_RESOURCE_CONFIGURATION is returned.

 Note: The API is not supported on 32-bit platforms.

 \param phDesc - Output descriptor
 \param resources - Array of resources to be included in the descriptor
 \param nbResources - Number of resources passed in \p resources

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_RESOURCE_TYPE,
 ::CUDA_ERROR_INVALID_RESOURCE_CONFIGURATION

 \sa
 ::cuDevSmResourceSplitByCount*/
    fn cuDevResourceGenerateDesc(
        phDesc: *mut cuda_types::cuda::CUdevResourceDesc,
        resources: *mut cuda_types::cuda::CUdevResource,
        nbResources: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Records an event.

 Captures in \p hEvent all the activities of the green context of \p hCtx
 at the time of this call. \p hEvent and \p hCtx must be from the same
 primary context otherwise ::CUDA_ERROR_INVALID_HANDLE is returned.
 Calls such as ::cuEventQuery() or ::cuGreenCtxWaitEvent() will
 then examine or wait for completion of the work that was captured. Uses of
 \p hCtx after this call do not modify \p hEvent.

 \note The API will return ::CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED if the
 specified green context \p hCtx has a stream in the capture mode. In such
 a case, the call will invalidate all the conflicting captures.

 \param hCtx - Green context to record event for
 \param hEvent  - Event to record

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED

 \sa
 ::cuGreenCtxWaitEvent,
 ::cuEventRecord,
 ::cuCtxRecordEvent,
 ::cuCtxWaitEvent*/
    fn cuGreenCtxRecordEvent(
        hCtx: cuda_types::cuda::CUgreenCtx,
        hEvent: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Make a green context wait on an event

 Makes all future work submitted to green context \p hCtx wait for all work
 captured in \p hEvent. The synchronization will be performed on the device
 and will not block the calling CPU thread. See ::cuGreenCtxRecordEvent()
 or ::cuEventRecord(), for details on what is captured by an event.

 \note \p hEvent may be from a different context or device than \p hCtx.

 \note The API will return ::CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED and
 invalidate the capture if the specified event \p hEvent is part of an
 ongoing capture sequence or if the specified green context \p hCtx has
 a stream in the capture mode.

 \param hCtx    - Green context to wait
 \param hEvent  - Event to wait on

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED

 \sa
 ::cuGreenCtxRecordEvent,
 ::cuStreamWaitEvent,
 ::cuCtxRecordEvent,
 ::cuCtxWaitEvent*/
    fn cuGreenCtxWaitEvent(
        hCtx: cuda_types::cuda::CUgreenCtx,
        hEvent: cuda_types::cuda::CUevent,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Query the green context associated with a stream

 Returns the CUDA green context that the stream is associated with, or NULL if the stream
 is not associated with any green context.

 The stream handle \p hStream can refer to any of the following:
 <ul>
   <li>
   a stream created via any of the CUDA driver APIs such as ::cuStreamCreate, ::cuStreamCreateWithPriority
   and ::cuGreenCtxStreamCreate, or their runtime API equivalents such as
   ::cudaStreamCreate, ::cudaStreamCreateWithFlags and ::cudaStreamCreateWithPriority.
   If during stream creation the context that was active in the calling thread was obtained
   with cuCtxFromGreenCtx, that green context is returned in \p phCtx.
   Otherwise, \p *phCtx is set to NULL instead.
   </li>
   <li>
   special stream such as the NULL stream or ::CU_STREAM_LEGACY.
   In that case if context that is active in the calling thread was obtained
   with cuCtxFromGreenCtx, that green context is returned.
   Otherwise, \p *phCtx is set to NULL instead.
   </li>
 </ul>
 Passing an invalid handle will result in undefined behavior.

 \param hStream - Handle to the stream to be queried
 \param phCtx   - Returned green context associated with the stream

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_HANDLE,
 \notefnerr

 \sa ::cuStreamDestroy,
 ::cuStreamCreate,
 ::cuStreamCreateWithPriority,
 ::cuStreamGetCtx_v2,
 ::cuGreenCtxStreamCreate,
 ::cuStreamGetPriority,
 ::cuStreamGetFlags,
 ::cuStreamGetDevice,
 ::cuStreamWaitEvent,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamAddCallback,
 ::cudaStreamCreate,
 ::cudaStreamCreateWithFlags*/
    fn cuStreamGetGreenCtx(
        hStream: cuda_types::cuda::CUstream,
        phCtx: *mut cuda_types::cuda::CUgreenCtx,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a stream for use in the green context

 Creates a stream for use in the specified green context \p greenCtx and returns a handle in \p phStream.
 The stream can be destroyed by calling ::cuStreamDestroy(). Note that the API ignores the context that
 is current to the calling thread and creates a stream in the specified green context \p greenCtx.

 The supported values for \p flags are:
 - ::CU_STREAM_NON_BLOCKING: This must be specified. It indicates that work running in the created
   stream may run concurrently with work in the default stream, and that
   the created stream should perform no implicit synchronization with the default stream.

 Specifying \p priority affects the scheduling priority of work in the stream. Priorities provide a
 hint to preferentially run work with higher priority when possible, but do not preempt
 already-running work or provide any other functional guarantee on execution order.
 \p priority follows a convention where lower numbers represent higher priorities.
 '0' represents default priority. The range of meaningful numerical priorities can
 be queried using ::cuCtxGetStreamPriorityRange. If the specified priority is
 outside the numerical range returned by ::cuCtxGetStreamPriorityRange,
 it will automatically be clamped to the lowest or the highest number in the range.

 \param phStream - Returned newly created stream
 \param greenCtx - Green context for which to create the stream for
 \param flags    - Flags for stream creation. \p CU_STREAM_NON_BLOCKING must be specified.
 \param priority - Stream priority. Lower numbers represent higher priorities.
                   See ::cuCtxGetStreamPriorityRange for more information about
                   meaningful stream priorities that can be passed.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \note In the current implementation, only compute kernels launched in
 priority streams are affected by the stream's priority. Stream priorities have
 no effect on host-to-device and device-to-host memory operations.

 \sa ::cuStreamDestroy,
 ::cuGreenCtxCreate
 ::cuStreamCreate,
 ::cuStreamGetPriority,
 ::cuCtxGetStreamPriorityRange,
 ::cuStreamGetFlags,
 ::cuStreamGetDevice,
 ::cuStreamWaitEvent,
 ::cuStreamQuery,
 ::cuStreamSynchronize,
 ::cuStreamAddCallback,
 ::cudaStreamCreateWithPriority*/
    fn cuGreenCtxStreamCreate(
        phStream: *mut cuda_types::cuda::CUstream,
        greenCtx: cuda_types::cuda::CUgreenCtx,
        flags: ::core::ffi::c_uint,
        priority: ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Register a callback function to receive error log messages

 \param callbackFunc  - The function to register as a callback
 \param userData      - A generic pointer to user data. This is passed into the callback function.
 \param callback_out  - Optional location to store the callback handle after it is registered

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE*/
    fn cuLogsRegisterCallback(
        callbackFunc: cuda_types::cuda::CUlogsCallback,
        userData: *mut ::core::ffi::c_void,
        callback_out: *mut cuda_types::cuda::CUlogsCallbackHandle,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unregister a log message callback

 \param callback  - The callback instance to unregister from receiving log messages

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE*/
    fn cuLogsUnregisterCallback(
        callback: cuda_types::cuda::CUlogsCallbackHandle,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Sets log iterator to point to the end of log buffer, where the next message would be written.

 \param iterator_out - Location to store an iterator to the current tail of the logs
 \param flags        - Reserved for future use, must be 0

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE*/
    fn cuLogsCurrent(
        iterator_out: *mut cuda_types::cuda::CUlogIterator,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Dump accumulated driver logs into a file

 Logs generated by the driver are stored in an internal buffer and can be copied out using this API.
 This API dumps all driver logs starting from \p iterator into \p pathToFile provided.

 \note \p iterator is auto-advancing. Dumping logs will update the value of
       \p iterator to receive the next generated log.

 \note The driver reserves limited memory for storing logs.
       The oldest logs may be overwritten and become unrecoverable. An indication will appear in the
       destination outupt if the logs have been truncated. Call dump after each failed API to mitigate this
       risk.

 \param iterator   - Optional auto-advancing iterator specifying the starting log to read. NULL value dumps all logs.
 \param pathToFile - Path to output file for dumping logs
 \param flags      - Reserved for future use, must be 0

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE*/
    fn cuLogsDumpToFile(
        iterator: *mut cuda_types::cuda::CUlogIterator,
        pathToFile: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Dump accumulated driver logs into a buffer

 Logs generated by the driver are stored in an internal buffer and can be copied out using this API.
 This API dumps driver logs from \p iterator into \p buffer up to the size specified in \p *size.
 The driver will always null terminate the buffer but there will not be a null character between log
 entries, only a newline \\n. The driver will then return the actual number of bytes written in
 \p *size, excluding the null terminator. If there are no messages to dump, \p *size will be set to 0
 and the function will return ::CUDA_SUCCESS.
 If the provided \p buffer is not large enough to hold any messages, \p *size will be set to 0 and
 the function will return ::CUDA_ERROR_INVALID_VALUE.

 \note \p iterator is auto-advancing. Dumping logs will update the value of
       \p iterator to receive the next generated log.

 \note The driver reserves limited memory for storing logs. The maximum size of the buffer is 25600 bytes.
       The oldest logs may be overwritten and become unrecoverable. An indication will appear in the
       destination outupt if the logs have been truncated. Call dump after each failed API to mitigate this
       risk.

 \note If the provided value in \p *size is not large enough to hold all buffered messages, a message will
       be added at the head of the buffer indicating this. The driver then computes the number of messages
       it is able to store in \p buffer and writes it out. The final message in \p buffer will always be
       the most recent log message as of when the API is called.

 \param iterator  - Optional auto-advancing iterator specifying the starting log to read. NULL value dumps all logs.
 \param buffer    - Pointer to dump logs
 \param size      - See description
 \param flags     - Reserved for future use, must be 0

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_VALUE*/
    fn cuLogsDumpToMemory(
        iterator: *mut cuda_types::cuda::CUlogIterator,
        buffer: *mut ::core::ffi::c_char,
        size: *mut usize,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemHostRegister(
        p: *mut ::core::ffi::c_void,
        bytesize: usize,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphicsResourceSetMapFlags(
        resource: cuda_types::cuda::CUgraphicsResource,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuLinkCreate(
        numOptions: ::core::ffi::c_uint,
        options: *mut cuda_types::cuda::CUjit_option,
        optionValues: *mut *mut ::core::ffi::c_void,
        stateOut: *mut cuda_types::cuda::CUlinkState,
    ) -> cuda_types::cuda::CUresult;
    fn cuLinkAddData(
        state: cuda_types::cuda::CUlinkState,
        type_: cuda_types::cuda::CUjitInputType,
        data: *mut ::core::ffi::c_void,
        size: usize,
        name: *const ::core::ffi::c_char,
        numOptions: ::core::ffi::c_uint,
        options: *mut cuda_types::cuda::CUjit_option,
        optionValues: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    fn cuLinkAddFile(
        state: cuda_types::cuda::CUlinkState,
        type_: cuda_types::cuda::CUjitInputType,
        path: *const ::core::ffi::c_char,
        numOptions: ::core::ffi::c_uint,
        options: *mut cuda_types::cuda::CUjit_option,
        optionValues: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    fn cuTexRefSetAddress2D_v2(
        hTexRef: cuda_types::cuda::CUtexref,
        desc: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR,
        dptr: cuda_types::cuda::CUdeviceptr,
        Pitch: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuDeviceTotalMem(
        bytes: *mut ::core::ffi::c_uint,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    fn cuCtxCreate(
        pctx: *mut cuda_types::cuda::CUcontext,
        flags: ::core::ffi::c_uint,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    fn cuModuleGetGlobal(
        dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
        bytes: *mut ::core::ffi::c_uint,
        hmod: cuda_types::cuda::CUmodule,
        name: *const ::core::ffi::c_char,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemGetInfo(
        free: *mut ::core::ffi::c_uint,
        total: *mut ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemAlloc(
        dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
        bytesize: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemAllocPitch(
        dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
        pPitch: *mut ::core::ffi::c_uint,
        WidthInBytes: ::core::ffi::c_uint,
        Height: ::core::ffi::c_uint,
        ElementSizeBytes: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemFree(dptr: cuda_types::cuda::CUdeviceptr_v1) -> cuda_types::cuda::CUresult;
    fn cuMemGetAddressRange(
        pbase: *mut cuda_types::cuda::CUdeviceptr_v1,
        psize: *mut ::core::ffi::c_uint,
        dptr: cuda_types::cuda::CUdeviceptr_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemAllocHost(
        pp: *mut *mut ::core::ffi::c_void,
        bytesize: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemHostGetDevicePointer(
        pdptr: *mut cuda_types::cuda::CUdeviceptr_v1,
        p: *mut ::core::ffi::c_void,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyHtoD(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoH(
        dstHost: *mut ::core::ffi::c_void,
        srcDevice: cuda_types::cuda::CUdeviceptr_v1,
        ByteCount: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoD(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        srcDevice: cuda_types::cuda::CUdeviceptr_v1,
        ByteCount: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoA(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: ::core::ffi::c_uint,
        srcDevice: cuda_types::cuda::CUdeviceptr_v1,
        ByteCount: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyAtoD(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: ::core::ffi::c_uint,
        ByteCount: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyHtoA(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: ::core::ffi::c_uint,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyAtoH(
        dstHost: *mut ::core::ffi::c_void,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: ::core::ffi::c_uint,
        ByteCount: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyAtoA(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: ::core::ffi::c_uint,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: ::core::ffi::c_uint,
        ByteCount: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyHtoAAsync(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: ::core::ffi::c_uint,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyAtoHAsync(
        dstHost: *mut ::core::ffi::c_void,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: ::core::ffi::c_uint,
        ByteCount: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy2D(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy2DUnaligned(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy3D(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyHtoDAsync(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoHAsync(
        dstHost: *mut ::core::ffi::c_void,
        srcDevice: cuda_types::cuda::CUdeviceptr_v1,
        ByteCount: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoDAsync(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        srcDevice: cuda_types::cuda::CUdeviceptr_v1,
        ByteCount: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy2DAsync(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D_v1,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy3DAsync(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_v1,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD8(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        uc: ::core::ffi::c_uchar,
        N: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD16(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        us: ::core::ffi::c_ushort,
        N: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD32(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        ui: ::core::ffi::c_uint,
        N: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD2D8(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        dstPitch: ::core::ffi::c_uint,
        uc: ::core::ffi::c_uchar,
        Width: ::core::ffi::c_uint,
        Height: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD2D16(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        dstPitch: ::core::ffi::c_uint,
        us: ::core::ffi::c_ushort,
        Width: ::core::ffi::c_uint,
        Height: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD2D32(
        dstDevice: cuda_types::cuda::CUdeviceptr_v1,
        dstPitch: ::core::ffi::c_uint,
        ui: ::core::ffi::c_uint,
        Width: ::core::ffi::c_uint,
        Height: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuArrayCreate(
        pHandle: *mut cuda_types::cuda::CUarray,
        pAllocateArray: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuArrayGetDescriptor(
        pArrayDescriptor: *mut cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR_v1,
        hArray: cuda_types::cuda::CUarray,
    ) -> cuda_types::cuda::CUresult;
    fn cuArray3DCreate(
        pHandle: *mut cuda_types::cuda::CUarray,
        pAllocateArray: *const cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuArray3DGetDescriptor(
        pArrayDescriptor: *mut cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR_v1,
        hArray: cuda_types::cuda::CUarray,
    ) -> cuda_types::cuda::CUresult;
    fn cuTexRefSetAddress(
        ByteOffset: *mut ::core::ffi::c_uint,
        hTexRef: cuda_types::cuda::CUtexref,
        dptr: cuda_types::cuda::CUdeviceptr_v1,
        bytes: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuTexRefSetAddress2D(
        hTexRef: cuda_types::cuda::CUtexref,
        desc: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR_v1,
        dptr: cuda_types::cuda::CUdeviceptr_v1,
        Pitch: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuTexRefGetAddress(
        pdptr: *mut cuda_types::cuda::CUdeviceptr_v1,
        hTexRef: cuda_types::cuda::CUtexref,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphicsResourceGetMappedPointer(
        pDevPtr: *mut cuda_types::cuda::CUdeviceptr_v1,
        pSize: *mut ::core::ffi::c_uint,
        resource: cuda_types::cuda::CUgraphicsResource,
    ) -> cuda_types::cuda::CUresult;
    fn cuCtxDestroy(ctx: cuda_types::cuda::CUcontext) -> cuda_types::cuda::CUresult;
    fn cuCtxPopCurrent(
        pctx: *mut cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    fn cuCtxPushCurrent(ctx: cuda_types::cuda::CUcontext) -> cuda_types::cuda::CUresult;
    fn cuStreamDestroy(
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuEventDestroy(hEvent: cuda_types::cuda::CUevent) -> cuda_types::cuda::CUresult;
    fn cuDevicePrimaryCtxRelease(
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    fn cuDevicePrimaryCtxReset(
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    fn cuDevicePrimaryCtxSetFlags(
        dev: cuda_types::cuda::CUdevice,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyHtoD_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoH_v2(
        dstHost: *mut ::core::ffi::c_void,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoD_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoA_v2(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: usize,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyAtoD_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: usize,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyHtoA_v2(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: usize,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyAtoH_v2(
        dstHost: *mut ::core::ffi::c_void,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: usize,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyAtoA_v2(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: usize,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: usize,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyHtoAAsync_v2(
        dstArray: cuda_types::cuda::CUarray,
        dstOffset: usize,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyAtoHAsync_v2(
        dstHost: *mut ::core::ffi::c_void,
        srcArray: cuda_types::cuda::CUarray,
        srcOffset: usize,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy2D_v2(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy2DUnaligned_v2(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy3D_v2(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyHtoDAsync_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoHAsync_v2(
        dstHost: *mut ::core::ffi::c_void,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyDtoDAsync_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy2DAsync_v2(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy3DAsync_v2(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD8_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        uc: ::core::ffi::c_uchar,
        N: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD16_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        us: ::core::ffi::c_ushort,
        N: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD32_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        ui: ::core::ffi::c_uint,
        N: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD2D8_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        uc: ::core::ffi::c_uchar,
        Width: usize,
        Height: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD2D16_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        us: ::core::ffi::c_ushort,
        Width: usize,
        Height: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD2D32_v2(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        ui: ::core::ffi::c_uint,
        Width: usize,
        Height: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy(
        dst: cuda_types::cuda::CUdeviceptr,
        src: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyAsync(
        dst: cuda_types::cuda::CUdeviceptr,
        src: cuda_types::cuda::CUdeviceptr,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyPeer(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstContext: cuda_types::cuda::CUcontext,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        srcContext: cuda_types::cuda::CUcontext,
        ByteCount: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyPeerAsync(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstContext: cuda_types::cuda::CUcontext,
        srcDevice: cuda_types::cuda::CUdeviceptr,
        srcContext: cuda_types::cuda::CUcontext,
        ByteCount: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy3DPeer(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_PEER,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy3DPeerAsync(
        pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_PEER,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpyBatchAsync(
        dsts: *mut cuda_types::cuda::CUdeviceptr,
        srcs: *mut cuda_types::cuda::CUdeviceptr,
        sizes: *mut usize,
        count: usize,
        attrs: *mut cuda_types::cuda::CUmemcpyAttributes,
        attrsIdxs: *mut usize,
        numAttrs: usize,
        failIdx: *mut usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemcpy3DBatchAsync(
        numOps: usize,
        opList: *mut cuda_types::cuda::CUDA_MEMCPY3D_BATCH_OP,
        failIdx: *mut usize,
        flags: ::core::ffi::c_ulonglong,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD8Async(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        uc: ::core::ffi::c_uchar,
        N: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD16Async(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        us: ::core::ffi::c_ushort,
        N: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD32Async(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        ui: ::core::ffi::c_uint,
        N: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD2D8Async(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        uc: ::core::ffi::c_uchar,
        Width: usize,
        Height: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD2D16Async(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        us: ::core::ffi::c_ushort,
        Width: usize,
        Height: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemsetD2D32Async(
        dstDevice: cuda_types::cuda::CUdeviceptr,
        dstPitch: usize,
        ui: ::core::ffi::c_uint,
        Width: usize,
        Height: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetPriority(
        hStream: cuda_types::cuda::CUstream,
        priority: *mut ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetId(
        hStream: cuda_types::cuda::CUstream,
        streamId: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetFlags(
        hStream: cuda_types::cuda::CUstream,
        flags: *mut ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetDevice(
        hStream: cuda_types::cuda::CUstream,
        device: *mut cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetCtx(
        hStream: cuda_types::cuda::CUstream,
        pctx: *mut cuda_types::cuda::CUcontext,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetCtx_v2(
        hStream: cuda_types::cuda::CUstream,
        pCtx: *mut cuda_types::cuda::CUcontext,
        pGreenCtx: *mut cuda_types::cuda::CUgreenCtx,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWaitEvent(
        hStream: cuda_types::cuda::CUstream,
        hEvent: cuda_types::cuda::CUevent,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamAddCallback(
        hStream: cuda_types::cuda::CUstream,
        callback: cuda_types::cuda::CUstreamCallback,
        userData: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamAttachMemAsync(
        hStream: cuda_types::cuda::CUstream,
        dptr: cuda_types::cuda::CUdeviceptr,
        length: usize,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamQuery(hStream: cuda_types::cuda::CUstream) -> cuda_types::cuda::CUresult;
    fn cuStreamSynchronize(
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuEventRecord(
        hEvent: cuda_types::cuda::CUevent,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuEventRecordWithFlags(
        hEvent: cuda_types::cuda::CUevent,
        hStream: cuda_types::cuda::CUstream,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuLaunchKernel(
        f: cuda_types::cuda::CUfunction,
        gridDimX: ::core::ffi::c_uint,
        gridDimY: ::core::ffi::c_uint,
        gridDimZ: ::core::ffi::c_uint,
        blockDimX: ::core::ffi::c_uint,
        blockDimY: ::core::ffi::c_uint,
        blockDimZ: ::core::ffi::c_uint,
        sharedMemBytes: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
        kernelParams: *mut *mut ::core::ffi::c_void,
        extra: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    fn cuLaunchKernelEx(
        config: *const cuda_types::cuda::CUlaunchConfig,
        f: cuda_types::cuda::CUfunction,
        kernelParams: *mut *mut ::core::ffi::c_void,
        extra: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    fn cuLaunchHostFunc(
        hStream: cuda_types::cuda::CUstream,
        fn_: cuda_types::cuda::CUhostFn,
        userData: *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphicsMapResources(
        count: ::core::ffi::c_uint,
        resources: *mut cuda_types::cuda::CUgraphicsResource,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphicsUnmapResources(
        count: ::core::ffi::c_uint,
        resources: *mut cuda_types::cuda::CUgraphicsResource,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWriteValue32(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint32_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWaitValue32(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint32_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWriteValue64(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint64_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWaitValue64(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint64_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamBatchMemOp(
        stream: cuda_types::cuda::CUstream,
        count: ::core::ffi::c_uint,
        paramArray: *mut cuda_types::cuda::CUstreamBatchMemOpParams,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWriteValue32_ptsz(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint32_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWaitValue32_ptsz(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint32_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWriteValue64_ptsz(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint64_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWaitValue64_ptsz(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint64_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamBatchMemOp_ptsz(
        stream: cuda_types::cuda::CUstream,
        count: ::core::ffi::c_uint,
        paramArray: *mut cuda_types::cuda::CUstreamBatchMemOpParams,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWriteValue32_v2(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint32_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWaitValue32_v2(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint32_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWriteValue64_v2(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint64_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamWaitValue64_v2(
        stream: cuda_types::cuda::CUstream,
        addr: cuda_types::cuda::CUdeviceptr,
        value: cuda_types::cuda::cuuint64_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamBatchMemOp_v2(
        stream: cuda_types::cuda::CUstream,
        count: ::core::ffi::c_uint,
        paramArray: *mut cuda_types::cuda::CUstreamBatchMemOpParams,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemPrefetchAsync(
        devPtr: cuda_types::cuda::CUdeviceptr,
        count: usize,
        dstDevice: cuda_types::cuda::CUdevice,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemPrefetchAsync_v2(
        devPtr: cuda_types::cuda::CUdeviceptr,
        count: usize,
        location: cuda_types::cuda::CUmemLocation,
        flags: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuLaunchCooperativeKernel(
        f: cuda_types::cuda::CUfunction,
        gridDimX: ::core::ffi::c_uint,
        gridDimY: ::core::ffi::c_uint,
        gridDimZ: ::core::ffi::c_uint,
        blockDimX: ::core::ffi::c_uint,
        blockDimY: ::core::ffi::c_uint,
        blockDimZ: ::core::ffi::c_uint,
        sharedMemBytes: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
        kernelParams: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cuda::CUresult;
    fn cuSignalExternalSemaphoresAsync(
        extSemArray: *const cuda_types::cuda::CUexternalSemaphore,
        paramsArray: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
        numExtSems: ::core::ffi::c_uint,
        stream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuWaitExternalSemaphoresAsync(
        extSemArray: *const cuda_types::cuda::CUexternalSemaphore,
        paramsArray: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
        numExtSems: ::core::ffi::c_uint,
        stream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamBeginCapture(
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamBeginCapture_ptsz(
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamBeginCapture_v2(
        hStream: cuda_types::cuda::CUstream,
        mode: cuda_types::cuda::CUstreamCaptureMode,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamBeginCaptureToGraph(
        hStream: cuda_types::cuda::CUstream,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
        numDependencies: usize,
        mode: cuda_types::cuda::CUstreamCaptureMode,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamEndCapture(
        hStream: cuda_types::cuda::CUstream,
        phGraph: *mut cuda_types::cuda::CUgraph,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamIsCapturing(
        hStream: cuda_types::cuda::CUstream,
        captureStatus: *mut cuda_types::cuda::CUstreamCaptureStatus,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetCaptureInfo(
        hStream: cuda_types::cuda::CUstream,
        captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
        id_out: *mut cuda_types::cuda::cuuint64_t,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetCaptureInfo_ptsz(
        hStream: cuda_types::cuda::CUstream,
        captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
        id_out: *mut cuda_types::cuda::cuuint64_t,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetCaptureInfo_v2(
        hStream: cuda_types::cuda::CUstream,
        captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
        id_out: *mut cuda_types::cuda::cuuint64_t,
        graph_out: *mut cuda_types::cuda::CUgraph,
        dependencies_out: *mut *const cuda_types::cuda::CUgraphNode,
        numDependencies_out: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetCaptureInfo_v3(
        hStream: cuda_types::cuda::CUstream,
        captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
        id_out: *mut cuda_types::cuda::cuuint64_t,
        graph_out: *mut cuda_types::cuda::CUgraph,
        dependencies_out: *mut *const cuda_types::cuda::CUgraphNode,
        edgeData_out: *mut *const cuda_types::cuda::CUgraphEdgeData,
        numDependencies_out: *mut usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphAddKernelNode(
        phGraphNode: *mut cuda_types::cuda::CUgraphNode,
        hGraph: cuda_types::cuda::CUgraph,
        dependencies: *const cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphKernelNodeGetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *mut cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphKernelNodeSetParams(
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphExecKernelNodeSetParams(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hNode: cuda_types::cuda::CUgraphNode,
        nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v1,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphInstantiateWithParams(
        phGraphExec: *mut cuda_types::cuda::CUgraphExec,
        hGraph: cuda_types::cuda::CUgraph,
        instantiateParams: *mut cuda_types::cuda::CUDA_GRAPH_INSTANTIATE_PARAMS,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphExecUpdate(
        hGraphExec: cuda_types::cuda::CUgraphExec,
        hGraph: cuda_types::cuda::CUgraph,
        hErrorNode_out: *mut cuda_types::cuda::CUgraphNode,
        updateResult_out: *mut cuda_types::cuda::CUgraphExecUpdateResult,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphUpload(
        hGraph: cuda_types::cuda::CUgraphExec,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphLaunch(
        hGraph: cuda_types::cuda::CUgraphExec,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamCopyAttributes(
        dstStream: cuda_types::cuda::CUstream,
        srcStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamGetAttribute(
        hStream: cuda_types::cuda::CUstream,
        attr: cuda_types::cuda::CUstreamAttrID,
        value: *mut cuda_types::cuda::CUstreamAttrValue,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamSetAttribute(
        hStream: cuda_types::cuda::CUstream,
        attr: cuda_types::cuda::CUstreamAttrID,
        param: *const cuda_types::cuda::CUstreamAttrValue,
    ) -> cuda_types::cuda::CUresult;
    fn cuIpcOpenMemHandle(
        pdptr: *mut cuda_types::cuda::CUdeviceptr,
        handle: cuda_types::cuda::CUipcMemHandle,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphInstantiate(
        phGraphExec: *mut cuda_types::cuda::CUgraphExec,
        hGraph: cuda_types::cuda::CUgraph,
        phErrorNode: *mut cuda_types::cuda::CUgraphNode,
        logBuffer: *mut ::core::ffi::c_char,
        bufferSize: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuGraphInstantiate_v2(
        phGraphExec: *mut cuda_types::cuda::CUgraphExec,
        hGraph: cuda_types::cuda::CUgraph,
        phErrorNode: *mut cuda_types::cuda::CUgraphNode,
        logBuffer: *mut ::core::ffi::c_char,
        bufferSize: usize,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemMapArrayAsync(
        mapInfoList: *mut cuda_types::cuda::CUarrayMapInfo,
        count: ::core::ffi::c_uint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemFreeAsync(
        dptr: cuda_types::cuda::CUdeviceptr,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemAllocAsync(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        bytesize: usize,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemAllocFromPoolAsync(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        bytesize: usize,
        pool: cuda_types::cuda::CUmemoryPool,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamUpdateCaptureDependencies(
        hStream: cuda_types::cuda::CUstream,
        dependencies: *mut cuda_types::cuda::CUgraphNode,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuStreamUpdateCaptureDependencies_v2(
        hStream: cuda_types::cuda::CUstream,
        dependencies: *mut cuda_types::cuda::CUgraphNode,
        dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuMemBatchDecompressAsync(
        paramsArray: *mut cuda_types::cuda::CUmemDecompressParams,
        count: usize,
        flags: ::core::ffi::c_uint,
        errorIndex: *mut usize,
        stream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuGetProcAddress(
        symbol: *const ::core::ffi::c_char,
        pfn: *mut *mut ::core::ffi::c_void,
        cudaVersion: ::core::ffi::c_int,
        flags: cuda_types::cuda::cuuint64_t,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the restore thread ID for a CUDA process

 Returns in \p *tid the thread ID of the CUDA restore thread for the process
 specified by \p pid.

 \param pid - The process ID of the CUDA process
 \param tid - Returned restore thread ID

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE
 ::CUDA_ERROR_NOT_INITIALIZED
 ::CUDA_ERROR_NOT_SUPPORTED*/
    fn cuCheckpointProcessGetRestoreThreadId(
        pid: ::core::ffi::c_int,
        tid: *mut ::core::ffi::c_int,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Returns the process state of a CUDA process

 Returns in \p *state the current state of the CUDA process specified by \p pid.

 \param pid - The process ID of the CUDA process
 \param state - Returned CUDA process state

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE
 ::CUDA_ERROR_NOT_INITIALIZED
 ::CUDA_ERROR_NOT_SUPPORTED*/
    fn cuCheckpointProcessGetState(
        pid: ::core::ffi::c_int,
        state: *mut cuda_types::cuda::CUprocessState,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Lock a running CUDA process

 Lock the CUDA process specified by \p pid which will block further CUDA API
 calls. Process must be in the RUNNING state in order to lock.

 Upon successful return the process will be in the LOCKED state.

 If timeoutMs is specified and the timeout is reached the process will be left
 in the RUNNING state upon return.

 \param pid - The process ID of the CUDA process
 \param args - Optional lock operation arguments

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE
 ::CUDA_ERROR_NOT_INITIALIZED
 ::CUDA_ERROR_ILLEGAL_STATE
 ::CUDA_ERROR_NOT_SUPPORTED
 ::CUDA_ERROR_NOT_READY*/
    fn cuCheckpointProcessLock(
        pid: ::core::ffi::c_int,
        args: *mut cuda_types::cuda::CUcheckpointLockArgs,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Checkpoint a CUDA process's GPU memory contents

 Checkpoints a CUDA process specified by \p pid that is in the LOCKED
 state. The GPU memory contents will be brought into host memory and all
 underlying references will be released. Process must be in the LOCKED state
 to checkpoint.

 Upon successful return the process will be in the CHECKPOINTED state.

 \param pid - The process ID of the CUDA process
 \param args - Optional checkpoint operation arguments

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE
 ::CUDA_ERROR_NOT_INITIALIZED
 ::CUDA_ERROR_ILLEGAL_STATE
 ::CUDA_ERROR_NOT_SUPPORTED*/
    fn cuCheckpointProcessCheckpoint(
        pid: ::core::ffi::c_int,
        args: *mut cuda_types::cuda::CUcheckpointCheckpointArgs,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Restore a CUDA process's GPU memory contents from its last checkpoint

 Restores a CUDA process specified by \p pid from its last checkpoint. Process
 must be in the CHECKPOINTED state to restore.

 Upon successful return the process will be in the LOCKED state.

 CUDA process restore requires persistence mode to be enabled or ::cuInit to
 have been called before execution.

 \param pid - The process ID of the CUDA process
 \param args - Optional restore operation arguments

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE
 ::CUDA_ERROR_NOT_INITIALIZED
 ::CUDA_ERROR_ILLEGAL_STATE
 ::CUDA_ERROR_NOT_SUPPORTED

 \sa
 ::cuInit*/
    fn cuCheckpointProcessRestore(
        pid: ::core::ffi::c_int,
        args: *mut cuda_types::cuda::CUcheckpointRestoreArgs,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unlock a CUDA process to allow CUDA API calls

 Unlocks a process specified by \p pid allowing it to resume making CUDA API
 calls. Process must be in the LOCKED state.

 Upon successful return the process will be in the RUNNING state.

 \param pid - The process ID of the CUDA process
 \param args - Optional unlock operation arguments

 \return
 ::CUDA_SUCCESS
 ::CUDA_ERROR_INVALID_VALUE
 ::CUDA_ERROR_NOT_INITIALIZED
 ::CUDA_ERROR_ILLEGAL_STATE
 ::CUDA_ERROR_NOT_SUPPORTED*/
    fn cuCheckpointProcessUnlock(
        pid: ::core::ffi::c_int,
        args: *mut cuda_types::cuda::CUcheckpointUnlockArgs,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Initialize the profiling.

 \deprecated

 Note that this function is deprecated and should not be used.
 Starting with CUDA 12.0, it always returns error code ::CUDA_ERROR_NOT_SUPPORTED.

 Using this API user can initialize the CUDA profiler by specifying
 the configuration file, output file and output file format. This
 API is generally used to profile different set of counters by
 looping the kernel launch. The \p configFile parameter can be used
 to select profiling options including profiler counters. Refer to
 the "Compute Command Line Profiler User Guide" for supported
 profiler options and counters.

 Limitation: The CUDA profiler cannot be initialized with this API
 if another profiling tool is already active, as indicated by the
 ::CUDA_ERROR_PROFILER_DISABLED return code.

 Typical usage of the profiling APIs is as follows:

 for each set of counters/options\n
 {\n
     cuProfilerInitialize(); //Initialize profiling, set the counters or options in the config file \n
     ...\n
     cuProfilerStart(); \n
     // code to be profiled \n
     cuProfilerStop(); \n
     ...\n
     cuProfilerStart(); \n
     // code to be profiled \n
     cuProfilerStop(); \n
     ...\n
 }\n

 \param configFile - Name of the config file that lists the counters/options
 for profiling.
 \param outputFile - Name of the outputFile where the profiling results will
 be stored.
 \param outputMode - outputMode, can be ::CU_OUT_KEY_VALUE_PAIR or ::CU_OUT_CSV.

 \return
 ::CUDA_ERROR_NOT_SUPPORTED
 \notefnerr

 \sa
 ::cuProfilerStart,
 ::cuProfilerStop,*/
    fn cuProfilerInitialize(
        configFile: *const ::core::ffi::c_char,
        outputFile: *const ::core::ffi::c_char,
        outputMode: cuda_types::cuda::CUoutput_mode,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Enable profiling.

 Enables profile collection by the active profiling tool for the
 current context. If profiling is already enabled, then
 cuProfilerStart() has no effect.

 cuProfilerStart and cuProfilerStop APIs are used to
 programmatically control the profiling granularity by allowing
 profiling to be done only on selective pieces of code.


 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa
 ::cuProfilerInitialize,
 ::cuProfilerStop,
 ::cudaProfilerStart*/
    fn cuProfilerStart() -> cuda_types::cuda::CUresult;
    /** \brief Disable profiling.

 Disables profile collection by the active profiling tool for the
 current context. If profiling is already disabled, then
 cuProfilerStop() has no effect.

 cuProfilerStart and cuProfilerStop APIs are used to
 programmatically control the profiling granularity by allowing
 profiling to be done only on selective pieces of code.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_CONTEXT
 \notefnerr

 \sa
 ::cuProfilerInitialize,
 ::cuProfilerStart,
 ::cudaProfilerStop*/
    fn cuProfilerStop() -> cuda_types::cuda::CUresult;
    /** \brief Registers an OpenGL buffer object

 Registers the buffer object specified by \p buffer for access by
 CUDA.  A handle to the registered object is returned as \p
 pCudaResource.  The register flags \p Flags specify the intended usage,
 as follows:

 - ::CU_GRAPHICS_REGISTER_FLAGS_NONE: Specifies no hints about how this
   resource will be used. It is therefore assumed that this resource will be
   read from and written to by CUDA. This is the default value.
 - ::CU_GRAPHICS_REGISTER_FLAGS_READ_ONLY: Specifies that CUDA
   will not write to this resource.
 - ::CU_GRAPHICS_REGISTER_FLAGS_WRITE_DISCARD: Specifies that
   CUDA will not read from this resource and will write over the
   entire contents of the resource, so none of the data previously
   stored in the resource will be preserved.

 \param pCudaResource - Pointer to the returned object handle
 \param buffer - name of buffer object to be registered
 \param Flags - Register flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ALREADY_MAPPED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_OPERATING_SYSTEM
 \notefnerr

 \sa
 ::cuGraphicsUnregisterResource,
 ::cuGraphicsMapResources,
 ::cuGraphicsResourceGetMappedPointer,
 ::cudaGraphicsGLRegisterBuffer*/
    fn cuGraphicsGLRegisterBuffer(
        pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
        buffer: cuda_types::cuda::GLuint,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Register an OpenGL texture or renderbuffer object

 Registers the texture or renderbuffer object specified by \p image for access by CUDA.
 A handle to the registered object is returned as \p pCudaResource.

 \p target must match the type of the object, and must be one of ::GL_TEXTURE_2D,
 ::GL_TEXTURE_RECTANGLE, ::GL_TEXTURE_CUBE_MAP, ::GL_TEXTURE_3D, ::GL_TEXTURE_2D_ARRAY,
 or ::GL_RENDERBUFFER.

 The register flags \p Flags specify the intended usage, as follows:

 - ::CU_GRAPHICS_REGISTER_FLAGS_NONE: Specifies no hints about how this
   resource will be used. It is therefore assumed that this resource will be
   read from and written to by CUDA. This is the default value.
 - ::CU_GRAPHICS_REGISTER_FLAGS_READ_ONLY: Specifies that CUDA
   will not write to this resource.
 - ::CU_GRAPHICS_REGISTER_FLAGS_WRITE_DISCARD: Specifies that
   CUDA will not read from this resource and will write over the
   entire contents of the resource, so none of the data previously
   stored in the resource will be preserved.
 - ::CU_GRAPHICS_REGISTER_FLAGS_SURFACE_LDST: Specifies that CUDA will
   bind this resource to a surface reference.
 - ::CU_GRAPHICS_REGISTER_FLAGS_TEXTURE_GATHER: Specifies that CUDA will perform
   texture gather operations on this resource.

 The following image formats are supported. For brevity's sake, the list is abbreviated.
 For ex., {GL_R, GL_RG} X {8, 16} would expand to the following 4 formats
 {GL_R8, GL_R16, GL_RG8, GL_RG16} :
 - GL_RED, GL_RG, GL_RGBA, GL_LUMINANCE, GL_ALPHA, GL_LUMINANCE_ALPHA, GL_INTENSITY
 - {GL_R, GL_RG, GL_RGBA} X {8, 16, 16F, 32F, 8UI, 16UI, 32UI, 8I, 16I, 32I}
 - {GL_LUMINANCE, GL_ALPHA, GL_LUMINANCE_ALPHA, GL_INTENSITY} X
 {8, 16, 16F_ARB, 32F_ARB, 8UI_EXT, 16UI_EXT, 32UI_EXT, 8I_EXT, 16I_EXT, 32I_EXT}

 The following image classes are currently disallowed:
 - Textures with borders
 - Multisampled renderbuffers

 \param pCudaResource - Pointer to the returned object handle
 \param image - name of texture or renderbuffer object to be registered
 \param target - Identifies the type of object specified by \p image
 \param Flags - Register flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ALREADY_MAPPED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_OPERATING_SYSTEM
 \notefnerr

 \sa
 ::cuGraphicsUnregisterResource,
 ::cuGraphicsMapResources,
 ::cuGraphicsSubResourceGetMappedArray,
 ::cudaGraphicsGLRegisterImage*/
    fn cuGraphicsGLRegisterImage(
        pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
        image: cuda_types::cuda::GLuint,
        target: cuda_types::cuda::GLenum,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the CUDA devices associated with the current OpenGL context

 Returns in \p *pCudaDeviceCount the number of CUDA-compatible devices
 corresponding to the current OpenGL context. Also returns in \p *pCudaDevices
 at most cudaDeviceCount of the CUDA-compatible devices corresponding to
 the current OpenGL context. If any of the GPUs being used by the current OpenGL
 context are not CUDA capable then the call will return CUDA_ERROR_NO_DEVICE.

 The \p deviceList argument may be any of the following:
 - ::CU_GL_DEVICE_LIST_ALL: Query all devices used by the current OpenGL context.
 - ::CU_GL_DEVICE_LIST_CURRENT_FRAME: Query the devices used by the current OpenGL context to
   render the current frame (in SLI).
 - ::CU_GL_DEVICE_LIST_NEXT_FRAME: Query the devices used by the current OpenGL context to
   render the next frame (in SLI). Note that this is a prediction, it can't be guaranteed that
   this is correct in all cases.

 \param pCudaDeviceCount - Returned number of CUDA devices.
 \param pCudaDevices     - Returned CUDA devices.
 \param cudaDeviceCount  - The size of the output device array pCudaDevices.
 \param deviceList       - The set of devices to return.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NO_DEVICE,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_GRAPHICS_CONTEXT,
 ::CUDA_ERROR_OPERATING_SYSTEM

 \notefnerr

 \sa
 ::cuWGLGetDevice,
 ::cudaGLGetDevices*/
    fn cuGLGetDevices_v2(
        pCudaDeviceCount: *mut ::core::ffi::c_uint,
        pCudaDevices: *mut cuda_types::cuda::CUdevice,
        cudaDeviceCount: ::core::ffi::c_uint,
        deviceList: cuda_types::cuda::CUGLDeviceList,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a CUDA context for interoperability with OpenGL

 \deprecated This function is deprecated as of Cuda 5.0.

 This function is deprecated and should no longer be used.  It is
 no longer necessary to associate a CUDA context with an OpenGL
 context in order to achieve maximum interoperability performance.

 \param pCtx   - Returned CUDA context
 \param Flags  - Options for CUDA context creation
 \param device - Device on which to create the context

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuCtxCreate, ::cuGLInit, ::cuGLMapBufferObject,
 ::cuGLRegisterBufferObject, ::cuGLUnmapBufferObject,
 ::cuGLUnregisterBufferObject, ::cuGLMapBufferObjectAsync,
 ::cuGLUnmapBufferObjectAsync, ::cuGLSetBufferObjectMapFlags,
 ::cuWGLGetDevice*/
    fn cuGLCtxCreate_v2(
        pCtx: *mut cuda_types::cuda::CUcontext,
        Flags: ::core::ffi::c_uint,
        device: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Initializes OpenGL interoperability

 \deprecated This function is deprecated as of Cuda 3.0.

 Initializes OpenGL interoperability. This function is deprecated
 and calling it is no longer required. It may fail if the needed
 OpenGL driver facilities are not available.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_UNKNOWN
 \notefnerr

 \sa ::cuGLMapBufferObject,
 ::cuGLRegisterBufferObject, ::cuGLUnmapBufferObject,
 ::cuGLUnregisterBufferObject, ::cuGLMapBufferObjectAsync,
 ::cuGLUnmapBufferObjectAsync, ::cuGLSetBufferObjectMapFlags,
 ::cuWGLGetDevice*/
    fn cuGLInit() -> cuda_types::cuda::CUresult;
    /** \brief Registers an OpenGL buffer object

 \deprecated This function is deprecated as of Cuda 3.0.

 Registers the buffer object specified by \p buffer for access by
 CUDA. This function must be called before CUDA can map the buffer
 object.  There must be a valid OpenGL context bound to the current
 thread when this function is called, and the buffer name is
 resolved by that context.

 \param buffer - The name of the buffer object to register.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_ALREADY_MAPPED
 \notefnerr

 \sa ::cuGraphicsGLRegisterBuffer*/
    fn cuGLRegisterBufferObject(
        buffer: cuda_types::cuda::GLuint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Maps an OpenGL buffer object

 \deprecated This function is deprecated as of Cuda 3.0.

 Maps the buffer object specified by \p buffer into the address space of the
 current CUDA context and returns in \p *dptr and \p *size the base pointer
 and size of the resulting mapping.

 There must be a valid OpenGL context bound to the current thread
 when this function is called.  This must be the same context, or a
 member of the same shareGroup, as the context that was bound when
 the buffer was registered.

 All streams in the current CUDA context are synchronized with the
 current GL context.

 \param dptr   - Returned mapped base pointer
 \param size   - Returned size of mapping
 \param buffer - The name of the buffer object to map

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_MAP_FAILED
 \notefnerr

 \sa ::cuGraphicsMapResources*/
    fn cuGLMapBufferObject_v2_ptds(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        size: *mut usize,
        buffer: cuda_types::cuda::GLuint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unmaps an OpenGL buffer object

 \deprecated This function is deprecated as of Cuda 3.0.

 Unmaps the buffer object specified by \p buffer for access by CUDA.

 There must be a valid OpenGL context bound to the current thread
 when this function is called.  This must be the same context, or a
 member of the same shareGroup, as the context that was bound when
 the buffer was registered.

 All streams in the current CUDA context are synchronized with the
 current GL context.

 \param buffer - Buffer object to unmap

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuGraphicsUnmapResources*/
    fn cuGLUnmapBufferObject(
        buffer: cuda_types::cuda::GLuint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unregister an OpenGL buffer object

 \deprecated This function is deprecated as of Cuda 3.0.

 Unregisters the buffer object specified by \p buffer.  This
 releases any resources associated with the registered buffer.
 After this call, the buffer may no longer be mapped for access by
 CUDA.

 There must be a valid OpenGL context bound to the current thread
 when this function is called.  This must be the same context, or a
 member of the same shareGroup, as the context that was bound when
 the buffer was registered.

 \param buffer - Name of the buffer object to unregister

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuGraphicsUnregisterResource*/
    fn cuGLUnregisterBufferObject(
        buffer: cuda_types::cuda::GLuint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Set the map flags for an OpenGL buffer object

 \deprecated This function is deprecated as of Cuda 3.0.

 Sets the map flags for the buffer object specified by \p buffer.

 Changes to \p Flags will take effect the next time \p buffer is mapped.
 The \p Flags argument may be any of the following:
 - ::CU_GL_MAP_RESOURCE_FLAGS_NONE: Specifies no hints about how this
   resource will be used. It is therefore assumed that this resource will be
   read from and written to by CUDA kernels. This is the default value.
 - ::CU_GL_MAP_RESOURCE_FLAGS_READ_ONLY: Specifies that CUDA kernels which
   access this resource will not write to this resource.
 - ::CU_GL_MAP_RESOURCE_FLAGS_WRITE_DISCARD: Specifies that CUDA kernels
   which access this resource will not read from this resource and will
   write over the entire contents of the resource, so none of the data
   previously stored in the resource will be preserved.

 If \p buffer has not been registered for use with CUDA, then
 ::CUDA_ERROR_INVALID_HANDLE is returned. If \p buffer is presently
 mapped for access by CUDA, then ::CUDA_ERROR_ALREADY_MAPPED is returned.

 There must be a valid OpenGL context bound to the current thread
 when this function is called.  This must be the same context, or a
 member of the same shareGroup, as the context that was bound when
 the buffer was registered.

 \param buffer - Buffer object to unmap
 \param Flags  - Map flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ALREADY_MAPPED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 \notefnerr

 \sa ::cuGraphicsResourceSetMapFlags*/
    fn cuGLSetBufferObjectMapFlags(
        buffer: cuda_types::cuda::GLuint,
        Flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Maps an OpenGL buffer object

 \deprecated This function is deprecated as of Cuda 3.0.

 Maps the buffer object specified by \p buffer into the address space of the
 current CUDA context and returns in \p *dptr and \p *size the base pointer
 and size of the resulting mapping.

 There must be a valid OpenGL context bound to the current thread
 when this function is called.  This must be the same context, or a
 member of the same shareGroup, as the context that was bound when
 the buffer was registered.

 Stream \p hStream in the current CUDA context is synchronized with
 the current GL context.

 \param dptr    - Returned mapped base pointer
 \param size    - Returned size of mapping
 \param buffer  - The name of the buffer object to map
 \param hStream - Stream to synchronize

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_MAP_FAILED
 \notefnerr

 \sa ::cuGraphicsMapResources*/
    fn cuGLMapBufferObjectAsync_v2_ptsz(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        size: *mut usize,
        buffer: cuda_types::cuda::GLuint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Unmaps an OpenGL buffer object

 \deprecated This function is deprecated as of Cuda 3.0.

 Unmaps the buffer object specified by \p buffer for access by CUDA.

 There must be a valid OpenGL context bound to the current thread
 when this function is called.  This must be the same context, or a
 member of the same shareGroup, as the context that was bound when
 the buffer was registered.

 Stream \p hStream in the current CUDA context is synchronized with
 the current GL context.

 \param buffer  - Name of the buffer object to unmap
 \param hStream - Stream to synchronize

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuGraphicsUnmapResources*/
    fn cuGLUnmapBufferObjectAsync(
        buffer: cuda_types::cuda::GLuint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuGLGetDevices(
        pCudaDeviceCount: *mut ::core::ffi::c_uint,
        pCudaDevices: *mut cuda_types::cuda::CUdevice,
        cudaDeviceCount: ::core::ffi::c_uint,
        deviceList: cuda_types::cuda::CUGLDeviceList,
    ) -> cuda_types::cuda::CUresult;
    fn cuGLMapBufferObject_v2(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        size: *mut usize,
        buffer: cuda_types::cuda::GLuint,
    ) -> cuda_types::cuda::CUresult;
    fn cuGLMapBufferObjectAsync_v2(
        dptr: *mut cuda_types::cuda::CUdeviceptr,
        size: *mut usize,
        buffer: cuda_types::cuda::GLuint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    fn cuGLCtxCreate(
        pCtx: *mut cuda_types::cuda::CUcontext,
        Flags: ::core::ffi::c_uint,
        device: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult;
    fn cuGLMapBufferObject(
        dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
        size: *mut ::core::ffi::c_uint,
        buffer: cuda_types::cuda::GLuint,
    ) -> cuda_types::cuda::CUresult;
    fn cuGLMapBufferObjectAsync(
        dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
        size: *mut ::core::ffi::c_uint,
        buffer: cuda_types::cuda::GLuint,
        hStream: cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Registers an EGL image

 Registers the EGLImageKHR specified by \p image for access by
 CUDA. A handle to the registered object is returned as \p pCudaResource.
 Additional Mapping/Unmapping is not required for the registered resource and
 ::cuGraphicsResourceGetMappedEglFrame can be directly called on the \p pCudaResource.

 The application will be responsible for synchronizing access to shared objects.
 The application must ensure that any pending operation which access the objects have completed
 before passing control to CUDA. This may be accomplished by issuing and waiting for
 glFinish command on all GLcontexts (for OpenGL and likewise for other APIs).
 The application will be also responsible for ensuring that any pending operation on the
 registered CUDA resource has completed prior to executing subsequent commands in other APIs
 accesing the same memory objects.
 This can be accomplished by calling cuCtxSynchronize or cuEventSynchronize (preferably).

 The surface's intended usage is specified using \p flags, as follows:

 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_NONE: Specifies no hints about how this
   resource will be used. It is therefore assumed that this resource will be
   read from and written to by CUDA. This is the default value.
 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_READ_ONLY: Specifies that CUDA
   will not write to this resource.
 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_WRITE_DISCARD: Specifies that
   CUDA will not read from this resource and will write over the
   entire contents of the resource, so none of the data previously
   stored in the resource will be preserved.

 The EGLImageKHR is an object which can be used to create EGLImage target resource. It is defined as a void pointer.
 typedef void* EGLImageKHR

 \param pCudaResource   - Pointer to the returned object handle
 \param image           - An EGLImageKHR image which can be used to create target resource.
 \param flags           - Map flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ALREADY_MAPPED,
 ::CUDA_ERROR_INVALID_CONTEXT,

 \sa ::cuGraphicsEGLRegisterImage, ::cuGraphicsUnregisterResource,
 ::cuGraphicsResourceSetMapFlags, ::cuGraphicsMapResources,
 ::cuGraphicsUnmapResources,
 ::cudaGraphicsEGLRegisterImage*/
    fn cuGraphicsEGLRegisterImage(
        pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
        image: cuda_types::cuda::EGLImageKHR,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Connect CUDA to EGLStream as a consumer.

 Connect CUDA as a consumer to EGLStreamKHR specified by \p stream.

 The EGLStreamKHR is an EGL object that transfers a sequence of image frames from one
 API to another.

 \param conn            - Pointer to the returned connection handle
 \param stream          - EGLStreamKHR handle

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_CONTEXT,

 \sa ::cuEGLStreamConsumerConnect, ::cuEGLStreamConsumerDisconnect,
 ::cuEGLStreamConsumerAcquireFrame, ::cuEGLStreamConsumerReleaseFrame,
 ::cudaEGLStreamConsumerConnect*/
    fn cuEGLStreamConsumerConnect(
        conn: *mut cuda_types::cuda::CUeglStreamConnection,
        stream: cuda_types::cuda::EGLStreamKHR,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Connect CUDA to EGLStream as a consumer with given flags.

 Connect CUDA as a consumer to EGLStreamKHR specified by \p stream with specified \p flags defined by CUeglResourceLocationFlags.

 The flags specify whether the consumer wants to access frames from system memory or video memory.
 Default is ::CU_EGL_RESOURCE_LOCATION_VIDMEM.

 \param conn              - Pointer to the returned connection handle
 \param stream            - EGLStreamKHR handle
 \param flags             - Flags denote intended location - system or video.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_CONTEXT,

 \sa ::cuEGLStreamConsumerConnect, ::cuEGLStreamConsumerDisconnect,
 ::cuEGLStreamConsumerAcquireFrame, ::cuEGLStreamConsumerReleaseFrame,
 ::cudaEGLStreamConsumerConnectWithFlags*/
    fn cuEGLStreamConsumerConnectWithFlags(
        conn: *mut cuda_types::cuda::CUeglStreamConnection,
        stream: cuda_types::cuda::EGLStreamKHR,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Disconnect CUDA as a consumer to EGLStream .

 Disconnect CUDA as a consumer to EGLStreamKHR.

 \param conn            - Conection to disconnect.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_CONTEXT,

 \sa ::cuEGLStreamConsumerConnect, ::cuEGLStreamConsumerDisconnect,
 ::cuEGLStreamConsumerAcquireFrame, ::cuEGLStreamConsumerReleaseFrame,
 ::cudaEGLStreamConsumerDisconnect*/
    fn cuEGLStreamConsumerDisconnect(
        conn: *mut cuda_types::cuda::CUeglStreamConnection,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Acquire an image frame from the EGLStream with CUDA as a consumer.

 Acquire an image frame from EGLStreamKHR. This API can also acquire an old frame presented
 by the producer unless explicitly disabled by setting EGL_SUPPORT_REUSE_NV flag to EGL_FALSE
 during stream initialization. By default, EGLStream is created with this flag set to EGL_TRUE.
 ::cuGraphicsResourceGetMappedEglFrame can be called on \p pCudaResource to get
 ::CUeglFrame.

 \param conn            - Connection on which to acquire
 \param pCudaResource   - CUDA resource on which the stream frame will be mapped for use.
 \param pStream         - CUDA stream for synchronization and any data migrations
                          implied by ::CUeglResourceLocationFlags.
 \param timeout         - Desired timeout in usec for a new frame to be acquired.
                          If set as ::CUDA_EGL_INFINITE_TIMEOUT, acquire waits infinitely.
                          After timeout occurs CUDA consumer tries to acquire an old frame
                          if available and EGL_SUPPORT_REUSE_NV flag is set.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_LAUNCH_TIMEOUT,

 \sa ::cuEGLStreamConsumerConnect, ::cuEGLStreamConsumerDisconnect,
 ::cuEGLStreamConsumerAcquireFrame, ::cuEGLStreamConsumerReleaseFrame,
 ::cudaEGLStreamConsumerAcquireFrame*/
    fn cuEGLStreamConsumerAcquireFrame(
        conn: *mut cuda_types::cuda::CUeglStreamConnection,
        pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
        pStream: *mut cuda_types::cuda::CUstream,
        timeout: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Releases the last frame acquired from the EGLStream.

 Release the acquired image frame specified by \p pCudaResource to EGLStreamKHR.
 If EGL_SUPPORT_REUSE_NV flag is set to EGL_TRUE, at the time of EGL creation
 this API doesn't release the last frame acquired on the EGLStream.
 By default, EGLStream is created with this flag set to EGL_TRUE.

 \param conn            - Connection on which to release
 \param pCudaResource   - CUDA resource whose corresponding frame is to be released
 \param pStream         - CUDA stream on which release will be done.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,

 \sa ::cuEGLStreamConsumerConnect, ::cuEGLStreamConsumerDisconnect,
 ::cuEGLStreamConsumerAcquireFrame, ::cuEGLStreamConsumerReleaseFrame,
 ::cudaEGLStreamConsumerReleaseFrame*/
    fn cuEGLStreamConsumerReleaseFrame(
        conn: *mut cuda_types::cuda::CUeglStreamConnection,
        pCudaResource: cuda_types::cuda::CUgraphicsResource,
        pStream: *mut cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Connect CUDA to EGLStream as a producer.

 Connect CUDA as a producer to EGLStreamKHR specified by \p stream.

 The EGLStreamKHR is an EGL object that transfers a sequence of image frames from one
 API to another.

 \param conn   - Pointer to the returned connection handle
 \param stream - EGLStreamKHR handle
 \param width  - width of the image to be submitted to the stream
 \param height - height of the image to be submitted to the stream

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_CONTEXT,

 \sa ::cuEGLStreamProducerConnect, ::cuEGLStreamProducerDisconnect,
 ::cuEGLStreamProducerPresentFrame,
 ::cudaEGLStreamProducerConnect*/
    fn cuEGLStreamProducerConnect(
        conn: *mut cuda_types::cuda::CUeglStreamConnection,
        stream: cuda_types::cuda::EGLStreamKHR,
        width: cuda_types::cuda::EGLint,
        height: cuda_types::cuda::EGLint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Disconnect CUDA as a producer  to EGLStream .

 Disconnect CUDA as a producer to EGLStreamKHR.

 \param conn            - Conection to disconnect.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_INVALID_CONTEXT,

 \sa ::cuEGLStreamProducerConnect, ::cuEGLStreamProducerDisconnect,
 ::cuEGLStreamProducerPresentFrame,
 ::cudaEGLStreamProducerDisconnect*/
    fn cuEGLStreamProducerDisconnect(
        conn: *mut cuda_types::cuda::CUeglStreamConnection,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Present a CUDA eglFrame to the EGLStream with CUDA as a producer.

 When a frame is presented by the producer, it gets associated with the EGLStream
 and thus it is illegal to free the frame before the producer is disconnected.
 If a frame is freed and reused it may lead to undefined behavior.

 If producer and consumer are on different GPUs (iGPU and dGPU) then frametype
 ::CU_EGL_FRAME_TYPE_ARRAY is not supported. ::CU_EGL_FRAME_TYPE_PITCH can be used for
 such cross-device applications.

 The ::CUeglFrame is defined as:
 \code
 typedef struct CUeglFrame_st {
     union {
         CUarray pArray[MAX_PLANES];
         void*   pPitch[MAX_PLANES];
     } frame;
     unsigned int width;
     unsigned int height;
     unsigned int depth;
     unsigned int pitch;
     unsigned int planeCount;
     unsigned int numChannels;
     CUeglFrameType frameType;
     CUeglColorFormat eglColorFormat;
     CUarray_format cuFormat;
 } CUeglFrame;
 \endcode

 For ::CUeglFrame of type ::CU_EGL_FRAME_TYPE_PITCH, the application may present sub-region of a memory
 allocation. In that case, the pitched pointer will specify the start address of the sub-region in
 the allocation and corresponding ::CUeglFrame fields will specify the dimensions of the sub-region.

 \param conn            - Connection on which to present the CUDA array
 \param eglframe        - CUDA Eglstream Proucer Frame handle to be sent to the consumer over EglStream.
 \param pStream         - CUDA stream on which to present the frame.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,

 \sa ::cuEGLStreamProducerConnect, ::cuEGLStreamProducerDisconnect,
 ::cuEGLStreamProducerReturnFrame,
 ::cudaEGLStreamProducerPresentFrame*/
    fn cuEGLStreamProducerPresentFrame(
        conn: *mut cuda_types::cuda::CUeglStreamConnection,
        eglframe: cuda_types::cuda::CUeglFrame,
        pStream: *mut cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Return the CUDA eglFrame to the EGLStream released by the consumer.

 This API can potentially return CUDA_ERROR_LAUNCH_TIMEOUT if the consumer has not
 returned a frame to EGL stream. If timeout is returned the application can retry.

 \param conn            - Connection on which to return
 \param eglframe        - CUDA Eglstream Proucer Frame handle returned from the consumer over EglStream.
 \param pStream         - CUDA stream on which to return the frame.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_LAUNCH_TIMEOUT

 \sa ::cuEGLStreamProducerConnect, ::cuEGLStreamProducerDisconnect,
 ::cuEGLStreamProducerPresentFrame,
 ::cudaEGLStreamProducerReturnFrame*/
    fn cuEGLStreamProducerReturnFrame(
        conn: *mut cuda_types::cuda::CUeglStreamConnection,
        eglframe: *mut cuda_types::cuda::CUeglFrame,
        pStream: *mut cuda_types::cuda::CUstream,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Get an eglFrame through which to access a registered EGL graphics resource.

 Returns in \p *eglFrame an eglFrame pointer through which the registered graphics resource
 \p resource may be accessed.
 This API can only be called for registered EGL graphics resources.

 The ::CUeglFrame is defined as:
 \code
 typedef struct CUeglFrame_st {
     union {
         CUarray pArray[MAX_PLANES];
         void*   pPitch[MAX_PLANES];
     } frame;
     unsigned int width;
     unsigned int height;
     unsigned int depth;
     unsigned int pitch;
     unsigned int planeCount;
     unsigned int numChannels;
     CUeglFrameType frameType;
     CUeglColorFormat eglColorFormat;
     CUarray_format cuFormat;
 } CUeglFrame;
 \endcode

 If \p resource is not registered then ::CUDA_ERROR_NOT_MAPPED is returned.
 *
 \param eglFrame   - Returned eglFrame.
 \param resource   - Registered resource to access.
 \param index      - Index for cubemap surfaces.
 \param mipLevel   - Mipmap level for the subresource to access.

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_NOT_MAPPED

 \sa
 ::cuGraphicsMapResources,
 ::cuGraphicsSubResourceGetMappedArray,
 ::cuGraphicsResourceGetMappedPointer,
 ::cudaGraphicsResourceGetMappedEglFrame*/
    fn cuGraphicsResourceGetMappedEglFrame(
        eglFrame: *mut cuda_types::cuda::CUeglFrame,
        resource: cuda_types::cuda::CUgraphicsResource,
        index: ::core::ffi::c_uint,
        mipLevel: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Creates an event from EGLSync object

 Creates an event *phEvent from an EGLSyncKHR eglSync with the flags specified
 via \p flags. Valid flags include:
 - ::CU_EVENT_DEFAULT: Default event creation flag.
 - ::CU_EVENT_BLOCKING_SYNC: Specifies that the created event should use blocking
 synchronization.  A CPU thread that uses ::cuEventSynchronize() to wait on
 an event created with this flag will block until the event has actually
 been completed.

 Once the \p eglSync gets destroyed, ::cuEventDestroy is the only API
 that can be invoked on the event.

 ::cuEventRecord and TimingData are not supported for events created from EGLSync.

 The EGLSyncKHR is an opaque handle to an EGL sync object.
 typedef void* EGLSyncKHR

 \param phEvent - Returns newly created event
 \param eglSync - Opaque handle to EGLSync object
 \param flags   - Event creation flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY

 \sa
 ::cuEventQuery,
 ::cuEventSynchronize,
 ::cuEventDestroy*/
    fn cuEventCreateFromEGLSync(
        phEvent: *mut cuda_types::cuda::CUevent,
        eglSync: cuda_types::cuda::EGLSyncKHR,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Gets the CUDA device associated with a VDPAU device

 Returns in \p *pDevice the CUDA device associated with a \p vdpDevice, if
 applicable.

 \param pDevice           - Device associated with vdpDevice
 \param vdpDevice         - A VdpDevice handle
 \param vdpGetProcAddress - VDPAU's VdpGetProcAddress function pointer

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE
 \notefnerr

 \sa ::cuCtxCreate, ::cuVDPAUCtxCreate, ::cuGraphicsVDPAURegisterVideoSurface,
 ::cuGraphicsVDPAURegisterOutputSurface, ::cuGraphicsUnregisterResource,
 ::cuGraphicsResourceSetMapFlags, ::cuGraphicsMapResources,
 ::cuGraphicsUnmapResources, ::cuGraphicsSubResourceGetMappedArray,
 ::cudaVDPAUGetDevice*/
    fn cuVDPAUGetDevice(
        pDevice: *mut cuda_types::cuda::CUdevice,
        vdpDevice: cuda_types::cuda::VdpDevice,
        vdpGetProcAddress: cuda_types::cuda::VdpGetProcAddress,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Create a CUDA context for interoperability with VDPAU

 Creates a new CUDA context, initializes VDPAU interoperability, and
 associates the CUDA context with the calling thread. It must be called
 before performing any other VDPAU interoperability operations. It may fail
 if the needed VDPAU driver facilities are not available. For usage of the
 \p flags parameter, see ::cuCtxCreate().

 \param pCtx              - Returned CUDA context
 \param flags             - Options for CUDA context creation
 \param device            - Device on which to create the context
 \param vdpDevice         - The VdpDevice to interop with
 \param vdpGetProcAddress - VDPAU's VdpGetProcAddress function pointer

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_DEINITIALIZED,
 ::CUDA_ERROR_NOT_INITIALIZED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 ::CUDA_ERROR_INVALID_VALUE,
 ::CUDA_ERROR_OUT_OF_MEMORY
 \notefnerr

 \sa ::cuCtxCreate, ::cuGraphicsVDPAURegisterVideoSurface,
 ::cuGraphicsVDPAURegisterOutputSurface, ::cuGraphicsUnregisterResource,
 ::cuGraphicsResourceSetMapFlags, ::cuGraphicsMapResources,
 ::cuGraphicsUnmapResources, ::cuGraphicsSubResourceGetMappedArray,
 ::cuVDPAUGetDevice*/
    fn cuVDPAUCtxCreate_v2(
        pCtx: *mut cuda_types::cuda::CUcontext,
        flags: ::core::ffi::c_uint,
        device: cuda_types::cuda::CUdevice,
        vdpDevice: cuda_types::cuda::VdpDevice,
        vdpGetProcAddress: cuda_types::cuda::VdpGetProcAddress,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Registers a VDPAU VdpVideoSurface object

 Registers the VdpVideoSurface specified by \p vdpSurface for access by
 CUDA. A handle to the registered object is returned as \p pCudaResource.
 The surface's intended usage is specified using \p flags, as follows:

 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_NONE: Specifies no hints about how this
   resource will be used. It is therefore assumed that this resource will be
   read from and written to by CUDA. This is the default value.
 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_READ_ONLY: Specifies that CUDA
   will not write to this resource.
 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_WRITE_DISCARD: Specifies that
   CUDA will not read from this resource and will write over the
   entire contents of the resource, so none of the data previously
   stored in the resource will be preserved.

 The VdpVideoSurface is presented as an array of subresources that may be
 accessed using pointers returned by ::cuGraphicsSubResourceGetMappedArray.
 The exact number of valid \p arrayIndex values depends on the VDPAU surface
 format. The mapping is shown in the table below. \p mipLevel must be 0.

 \htmlonly
 <table>
 <tr><th>VdpChromaType                               </th><th>arrayIndex</th><th>Size     </th><th>Format</th><th>Content            </th></tr>
 <tr><td rowspan="4" valign="top">VDP_CHROMA_TYPE_420</td><td>0         </td><td>w   x h/2</td><td>R8    </td><td>Top-field luma     </td></tr>
 <tr>                                                     <td>1         </td><td>w   x h/2</td><td>R8    </td><td>Bottom-field luma  </td></tr>
 <tr>                                                     <td>2         </td><td>w/2 x h/4</td><td>R8G8  </td><td>Top-field chroma   </td></tr>
 <tr>                                                     <td>3         </td><td>w/2 x h/4</td><td>R8G8  </td><td>Bottom-field chroma</td></tr>
 <tr><td rowspan="4" valign="top">VDP_CHROMA_TYPE_422</td><td>0         </td><td>w   x h/2</td><td>R8    </td><td>Top-field luma     </td></tr>
 <tr>                                                     <td>1         </td><td>w   x h/2</td><td>R8    </td><td>Bottom-field luma  </td></tr>
 <tr>                                                     <td>2         </td><td>w/2 x h/2</td><td>R8G8  </td><td>Top-field chroma   </td></tr>
 <tr>                                                     <td>3         </td><td>w/2 x h/2</td><td>R8G8  </td><td>Bottom-field chroma</td></tr>
 </table>
 \endhtmlonly

 \latexonly
 \begin{tabular}{|l|l|l|l|l|}
 \hline
 VdpChromaType          & arrayIndex & Size      & Format & Content             \\
 \hline
 VDP\_CHROMA\_TYPE\_420 & 0          & w x h/2   & R8     & Top-field luma      \\
                        & 1          & w x h/2   & R8     & Bottom-field luma   \\
                        & 2          & w/2 x h/4 & R8G8   & Top-field chroma    \\
                        & 3          & w/2 x h/4 & R8G8   & Bottom-field chroma \\
 \hline
 VDP\_CHROMA\_TYPE\_422 & 0          & w x h/2   & R8     & Top-field luma      \\
                        & 1          & w x h/2   & R8     & Bottom-field luma   \\
                        & 2          & w/2 x h/2 & R8G8   & Top-field chroma    \\
                        & 3          & w/2 x h/2 & R8G8   & Bottom-field chroma \\
 \hline
 \end{tabular}
 \endlatexonly

 \param pCudaResource - Pointer to the returned object handle
 \param vdpSurface    - The VdpVideoSurface to be registered
 \param flags         - Map flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ALREADY_MAPPED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 \notefnerr

 \sa ::cuCtxCreate, ::cuVDPAUCtxCreate,
 ::cuGraphicsVDPAURegisterOutputSurface, ::cuGraphicsUnregisterResource,
 ::cuGraphicsResourceSetMapFlags, ::cuGraphicsMapResources,
 ::cuGraphicsUnmapResources, ::cuGraphicsSubResourceGetMappedArray,
 ::cuVDPAUGetDevice,
 ::cudaGraphicsVDPAURegisterVideoSurface*/
    fn cuGraphicsVDPAURegisterVideoSurface(
        pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
        vdpSurface: cuda_types::cuda::VdpVideoSurface,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    /** \brief Registers a VDPAU VdpOutputSurface object

 Registers the VdpOutputSurface specified by \p vdpSurface for access by
 CUDA. A handle to the registered object is returned as \p pCudaResource.
 The surface's intended usage is specified using \p flags, as follows:

 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_NONE: Specifies no hints about how this
   resource will be used. It is therefore assumed that this resource will be
   read from and written to by CUDA. This is the default value.
 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_READ_ONLY: Specifies that CUDA
   will not write to this resource.
 - ::CU_GRAPHICS_MAP_RESOURCE_FLAGS_WRITE_DISCARD: Specifies that
   CUDA will not read from this resource and will write over the
   entire contents of the resource, so none of the data previously
   stored in the resource will be preserved.

 The VdpOutputSurface is presented as an array of subresources that may be
 accessed using pointers returned by ::cuGraphicsSubResourceGetMappedArray.
 The exact number of valid \p arrayIndex values depends on the VDPAU surface
 format. The mapping is shown in the table below. \p mipLevel must be 0.

 \htmlonly
 <table>
 <tr><th>VdpRGBAFormat              </th><th>arrayIndex</th><th>Size </th><th>Format </th><th>Content       </th></tr>
 <tr><td>VDP_RGBA_FORMAT_B8G8R8A8   </td><td>0         </td><td>w x h</td><td>ARGB8  </td><td>Entire surface</td></tr>
 <tr><td>VDP_RGBA_FORMAT_R10G10B10A2</td><td>0         </td><td>w x h</td><td>A2BGR10</td><td>Entire surface</td></tr>
 </table>
 \endhtmlonly

 \latexonly
 \begin{tabular}{|l|l|l|l|l|}
 \hline
 VdpRGBAFormat                  & arrayIndex & Size  & Format  & Content        \\
 \hline
 VDP\_RGBA\_FORMAT\_B8G8R8A8    & 0          & w x h & ARGB8   & Entire surface \\
 VDP\_RGBA\_FORMAT\_R10G10B10A2 & 0          & w x h & A2BGR10 & Entire surface \\
 \hline
 \end{tabular}
 \endlatexonly

 \param pCudaResource - Pointer to the returned object handle
 \param vdpSurface    - The VdpOutputSurface to be registered
 \param flags         - Map flags

 \return
 ::CUDA_SUCCESS,
 ::CUDA_ERROR_INVALID_HANDLE,
 ::CUDA_ERROR_ALREADY_MAPPED,
 ::CUDA_ERROR_INVALID_CONTEXT,
 \notefnerr

 \sa ::cuCtxCreate, ::cuVDPAUCtxCreate,
 ::cuGraphicsVDPAURegisterVideoSurface, ::cuGraphicsUnregisterResource,
 ::cuGraphicsResourceSetMapFlags, ::cuGraphicsMapResources,
 ::cuGraphicsUnmapResources, ::cuGraphicsSubResourceGetMappedArray,
 ::cuVDPAUGetDevice,
 ::cudaGraphicsVDPAURegisterOutputSurface*/
    fn cuGraphicsVDPAURegisterOutputSurface(
        pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
        vdpSurface: cuda_types::cuda::VdpOutputSurface,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::cuda::CUresult;
    fn cuVDPAUCtxCreate(
        pCtx: *mut cuda_types::cuda::CUcontext,
        flags: ::core::ffi::c_uint,
        device: cuda_types::cuda::CUdevice,
        vdpDevice: cuda_types::cuda::VdpDevice,
        vdpGetProcAddress: cuda_types::cuda::VdpGetProcAddress,
    ) -> cuda_types::cuda::CUresult;
}
