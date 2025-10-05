// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
extern "system" {
    #[must_use]
    /** Initialize NVML, but don't initialize any GPUs yet.

 \note nvmlInit_v3 introduces a "flags" argument, that allows passing boolean values
       modifying the behaviour of nvmlInit().
 \note In NVML 5.319 new nvmlInit_v2 has replaced nvmlInit"_v1" (default in NVML 4.304 and older) that
       did initialize all GPU devices in the system.

 This allows NVML to communicate with a GPU
 when other GPUs in the system are unstable or in a bad state.  When using this API, GPUs are
 discovered and initialized in nvmlDeviceGetHandleBy* functions instead.

 \note To contrast nvmlInit_v2 with nvmlInit"_v1", NVML 4.304 nvmlInit"_v1" will fail when any detected GPU is in
       a bad or unstable state.

 For all products.

 This method, should be called once before invoking any other methods in the library.
 A reference count of the number of initializations is maintained.  Shutdown only occurs
 when the reference count reaches zero.

 @return
         - \ref NVML_SUCCESS                   if NVML has been properly initialized
         - \ref NVML_ERROR_DRIVER_NOT_LOADED   if NVIDIA driver is not running
         - \ref NVML_ERROR_NO_PERMISSION       if NVML does not have permission to talk to the driver
         - \ref NVML_ERROR_UNKNOWN             on any unexpected error*/
    fn nvmlInit_v2() -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** nvmlInitWithFlags is a variant of nvmlInit(), that allows passing a set of boolean values
       modifying the behaviour of nvmlInit().
       Other than the "flags" parameter it is completely similar to \ref nvmlInit_v2.

 For all products.

 @param flags                                 behaviour modifier flags

 @return
         - \ref NVML_SUCCESS                   if NVML has been properly initialized
         - \ref NVML_ERROR_DRIVER_NOT_LOADED   if NVIDIA driver is not running
         - \ref NVML_ERROR_NO_PERMISSION       if NVML does not have permission to talk to the driver
         - \ref NVML_ERROR_UNKNOWN             on any unexpected error*/
    fn nvmlInitWithFlags(flags: ::core::ffi::c_uint) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Shut down NVML by releasing all GPU resources previously allocated with \ref nvmlInit_v2().

 For all products.

 This method should be called after NVML work is done, once for each call to \ref nvmlInit_v2()
 A reference count of the number of initializations is maintained.  Shutdown only occurs
 when the reference count reaches zero.  For backwards compatibility, no error is reported if
 nvmlShutdown() is called more times than nvmlInit().

 @return
         - \ref NVML_SUCCESS                 if NVML has been properly shut down
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlShutdown() -> cuda_types::nvml::nvmlReturn_t;
    /** Helper method for converting NVML error codes into readable strings.

 For all products.

 @param result                               NVML error code to convert

 @return String representation of the error.
*/
    fn nvmlErrorString(
        result: cuda_types::nvml::nvmlReturn_t,
    ) -> *const ::core::ffi::c_char;
    #[must_use]
    /** Retrieves the version of the system's graphics driver.

 For all products.

 The version identifier is an alphanumeric string.  It will not exceed 80 characters in length
 (including the NULL terminator).  See \ref nvmlConstants::NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE.

 @param version                              Reference in which to return the version identifier
 @param length                               The maximum allowed length of the string returned in \a version

 @return
         - \ref NVML_SUCCESS                 if \a version has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a version is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small*/
    fn nvmlSystemGetDriverVersion(
        version: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the version of the NVML library.

 For all products.

 The version identifier is an alphanumeric string.  It will not exceed 80 characters in length
 (including the NULL terminator).  See \ref nvmlConstants::NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE.

 @param version                              Reference in which to return the version identifier
 @param length                               The maximum allowed length of the string returned in \a version

 @return
         - \ref NVML_SUCCESS                 if \a version has been set
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a version is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small*/
    fn nvmlSystemGetNVMLVersion(
        version: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the version of the CUDA driver.

 For all products.

 The CUDA driver version returned will be retreived from the currently installed version of CUDA.
 If the cuda library is not found, this function will return a known supported version number.

 @param cudaDriverVersion                    Reference in which to return the version identifier

 @return
         - \ref NVML_SUCCESS                 if \a cudaDriverVersion has been set
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a cudaDriverVersion is NULL*/
    fn nvmlSystemGetCudaDriverVersion(
        cudaDriverVersion: *mut ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the version of the CUDA driver from the shared library.

 For all products.

 The returned CUDA driver version by calling cuDriverGetVersion()

 @param cudaDriverVersion                    Reference in which to return the version identifier

 @return
         - \ref NVML_SUCCESS                  if \a cudaDriverVersion has been set
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a cudaDriverVersion is NULL
         - \ref NVML_ERROR_LIBRARY_NOT_FOUND  if \a libcuda.so.1 or libcuda.dll is not found
         - \ref NVML_ERROR_FUNCTION_NOT_FOUND if \a cuDriverGetVersion() is not found in the shared library*/
    fn nvmlSystemGetCudaDriverVersion_v2(
        cudaDriverVersion: *mut ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets name of the process with provided process id

 For all products.

 Returned process name is cropped to provided length.
 name string is encoded in ANSI.

 @param pid                                  The identifier of the process
 @param name                                 Reference in which to return the process name
 @param length                               The maximum allowed length of the string returned in \a name

 @return
         - \ref NVML_SUCCESS                 if \a name has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a name is NULL or \a length is 0.
         - \ref NVML_ERROR_NOT_FOUND         if process doesn't exists
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlSystemGetProcessName(
        pid: ::core::ffi::c_uint,
        name: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the IDs and firmware versions for any Host Interface Cards (HICs) in the system.

 For S-class products.

 The \a hwbcCount argument is expected to be set to the size of the input \a hwbcEntries array.
 The HIC must be connected to an S-class system for it to be reported by this function.

 @param hwbcCount                            Size of hwbcEntries array
 @param hwbcEntries                          Array holding information about hwbc

 @return
         - \ref NVML_SUCCESS                 if \a hwbcCount and \a hwbcEntries have been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if either \a hwbcCount or \a hwbcEntries is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a hwbcCount indicates that the \a hwbcEntries array is too small*/
    fn nvmlSystemGetHicVersion(
        hwbcCount: *mut ::core::ffi::c_uint,
        hwbcEntries: *mut cuda_types::nvml::nvmlHwbcEntry_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the set of GPUs that have a CPU affinity with the given CPU number
 For all products.
 Supported on Linux only.

 @param cpuNumber                            The CPU number
 @param count                                When zero, is set to the number of matching GPUs such that \a deviceArray
                                             can be malloc'd.  When non-zero, \a deviceArray will be filled with \a count
                                             number of device handles.
 @param deviceArray                          An array of device handles for GPUs found with affinity to \a cpuNumber

 @return
         - \ref NVML_SUCCESS                 if \a deviceArray or \a count (if initially zero) has been set
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a cpuNumber, or \a count is invalid, or \a deviceArray is NULL with a non-zero \a count
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device or OS does not support this feature
         - \ref NVML_ERROR_UNKNOWN           an error has occurred in underlying topology discovery*/
    fn nvmlSystemGetTopologyGpuSet(
        cpuNumber: ::core::ffi::c_uint,
        count: *mut ::core::ffi::c_uint,
        deviceArray: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the driver branch of the NVIDIA driver installed on the system.

 For all products.

 The branch identifier is an alphanumeric string.  It will not exceed 80 characters in length
 (including the NULL terminator).  See \ref nvmlConstants::NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE.

 @param branchInfo                            Pointer to the driver branch information structure \a nvmlSystemDriverBranchInfo_t
 @param length                                The maximum allowed length of the driver branch string

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a branchInfo is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlSystemGetDriverBranch(
        branchInfo: *mut cuda_types::nvml::nvmlSystemDriverBranchInfo_t,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the number of units in the system.

 For S-class products.

 @param unitCount                            Reference in which to return the number of units

 @return
         - \ref NVML_SUCCESS                 if \a unitCount has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a unitCount is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlUnitGetCount(
        unitCount: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Acquire the handle for a particular unit, based on its index.

 For S-class products.

 Valid indices are derived from the \a unitCount returned by \ref nvmlUnitGetCount().
   For example, if \a unitCount is 2 the valid indices are 0 and 1, corresponding to UNIT 0 and UNIT 1.

 The order in which NVML enumerates units has no guarantees of consistency between reboots.

 @param index                                The index of the target unit, >= 0 and < \a unitCount
 @param unit                                 Reference in which to return the unit handle

 @return
         - \ref NVML_SUCCESS                 if \a unit has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a index is invalid or \a unit is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlUnitGetHandleByIndex(
        index: ::core::ffi::c_uint,
        unit: *mut cuda_types::nvml::nvmlUnit_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the static information associated with a unit.

 For S-class products.

 See \ref nvmlUnitInfo_t for details on available unit info.

 @param unit                                 The identifier of the target unit
 @param info                                 Reference in which to return the unit information

 @return
         - \ref NVML_SUCCESS                 if \a info has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a unit is invalid or \a info is NULL*/
    fn nvmlUnitGetUnitInfo(
        unit: cuda_types::nvml::nvmlUnit_t,
        info: *mut cuda_types::nvml::nvmlUnitInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the LED state associated with this unit.

 For S-class products.

 See \ref nvmlLedState_t for details on allowed states.

 @param unit                                 The identifier of the target unit
 @param state                                Reference in which to return the current LED state

 @return
         - \ref NVML_SUCCESS                 if \a state has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a unit is invalid or \a state is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this is not an S-class product
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlUnitSetLedState()*/
    fn nvmlUnitGetLedState(
        unit: cuda_types::nvml::nvmlUnit_t,
        state: *mut cuda_types::nvml::nvmlLedState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the PSU stats for the unit.

 For S-class products.

 See \ref nvmlPSUInfo_t for details on available PSU info.

 @param unit                                 The identifier of the target unit
 @param psu                                  Reference in which to return the PSU information

 @return
         - \ref NVML_SUCCESS                 if \a psu has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a unit is invalid or \a psu is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this is not an S-class product
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlUnitGetPsuInfo(
        unit: cuda_types::nvml::nvmlUnit_t,
        psu: *mut cuda_types::nvml::nvmlPSUInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the temperature readings for the unit, in degrees C.

 For S-class products.

 Depending on the product, readings may be available for intake (type=0),
 exhaust (type=1) and board (type=2).

 @param unit                                 The identifier of the target unit
 @param type                                 The type of reading to take
 @param temp                                 Reference in which to return the intake temperature

 @return
         - \ref NVML_SUCCESS                 if \a temp has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a unit or \a type is invalid or \a temp is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this is not an S-class product
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlUnitGetTemperature(
        unit: cuda_types::nvml::nvmlUnit_t,
        type_: ::core::ffi::c_uint,
        temp: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the fan speed readings for the unit.

 For S-class products.

 See \ref nvmlUnitFanSpeeds_t for details on available fan speed info.

 @param unit                                 The identifier of the target unit
 @param fanSpeeds                            Reference in which to return the fan speed information

 @return
         - \ref NVML_SUCCESS                 if \a fanSpeeds has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a unit is invalid or \a fanSpeeds is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this is not an S-class product
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlUnitGetFanSpeedInfo(
        unit: cuda_types::nvml::nvmlUnit_t,
        fanSpeeds: *mut cuda_types::nvml::nvmlUnitFanSpeeds_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the set of GPU devices that are attached to the specified unit.

 For S-class products.

 The \a deviceCount argument is expected to be set to the size of the input \a devices array.

 @param unit                                 The identifier of the target unit
 @param deviceCount                          Reference in which to provide the \a devices array size, and
                                             to return the number of attached GPU devices
 @param devices                              Reference in which to return the references to the attached GPU devices

 @return
         - \ref NVML_SUCCESS                 if \a deviceCount and \a devices have been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a deviceCount indicates that the \a devices array is too small
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a unit is invalid, either of \a deviceCount or \a devices is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlUnitGetDevices(
        unit: cuda_types::nvml::nvmlUnit_t,
        deviceCount: *mut ::core::ffi::c_uint,
        devices: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the number of compute devices in the system. A compute device is a single GPU.

 For all products.

 Note: New nvmlDeviceGetCount_v2 (default in NVML 5.319) returns count of all devices in the system
       even if nvmlDeviceGetHandleByIndex_v2 returns NVML_ERROR_NO_PERMISSION for such device.
       Update your code to handle this error, or use NVML 4.304 or older nvml header file.
       For backward binary compatibility reasons _v1 version of the API is still present in the shared
       library.
       Old _v1 version of nvmlDeviceGetCount doesn't count devices that NVML has no permission to talk to.

 @param deviceCount                          Reference in which to return the number of accessible devices

 @return
         - \ref NVML_SUCCESS                 if \a deviceCount has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a deviceCount is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetCount_v2(
        deviceCount: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get attributes (engine counts etc.) for the given NVML device handle.

 @note This API currently only supports MIG device handles.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               NVML device handle
 @param attributes                           Device attributes

 @return
        - \ref NVML_SUCCESS                  if \a device attributes were successfully retrieved
        - \ref NVML_ERROR_INVALID_ARGUMENT   if \a device handle is invalid
        - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
        - \ref NVML_ERROR_NOT_SUPPORTED      if this query is not supported by the device
        - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetAttributes_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        attributes: *mut cuda_types::nvml::nvmlDeviceAttributes_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Acquire the handle for a particular device, based on its index.

 For all products.

 Valid indices are derived from the \a accessibleDevices count returned by
   \ref nvmlDeviceGetCount_v2(). For example, if \a accessibleDevices is 2 the valid indices
   are 0 and 1, corresponding to GPU 0 and GPU 1.

 The order in which NVML enumerates devices has no guarantees of consistency between reboots. For that reason it
   is recommended that devices be looked up by their PCI ids or UUID. See
   \ref nvmlDeviceGetHandleByUUID() and \ref nvmlDeviceGetHandleByPciBusId_v2().

 Note: The NVML index may not correlate with other APIs, such as the CUDA device index.

 Starting from NVML 5, this API causes NVML to initialize the target GPU
 NVML may initialize additional GPUs if:
  - The target GPU is an SLI slave

 Note: New nvmlDeviceGetCount_v2 (default in NVML 5.319) returns count of all devices in the system
       even if nvmlDeviceGetHandleByIndex_v2 returns NVML_ERROR_NO_PERMISSION for such device.
       Update your code to handle this error, or use NVML 4.304 or older nvml header file.
       For backward binary compatibility reasons _v1 version of the API is still present in the shared
       library.
       Old _v1 version of nvmlDeviceGetCount doesn't count devices that NVML has no permission to talk to.

       This means that nvmlDeviceGetHandleByIndex_v2 and _v1 can return different devices for the same index.
       If you don't touch macros that map old (_v1) versions to _v2 versions at the top of the file you don't
       need to worry about that.

 @param index                                The index of the target GPU, >= 0 and < \a accessibleDevices
 @param device                               Reference in which to return the device handle

 @return
         - \ref NVML_SUCCESS                  if \a device has been set
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a index is invalid or \a device is NULL
         - \ref NVML_ERROR_INSUFFICIENT_POWER if any attached devices have improperly attached external power cables
         - \ref NVML_ERROR_NO_PERMISSION      if the user doesn't have permission to talk to this device
         - \ref NVML_ERROR_IRQ_ISSUE          if NVIDIA kernel detected an interrupt issue with the attached GPUs
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error

 @see nvmlDeviceGetIndex
 @see nvmlDeviceGetCount*/
    fn nvmlDeviceGetHandleByIndex_v2(
        index: ::core::ffi::c_uint,
        device: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Acquire the handle for a particular device, based on its board serial number.

 For Fermi &tm; or newer fully supported devices.

 This number corresponds to the value printed directly on the board, and to the value returned by
   \ref nvmlDeviceGetSerial().

 @deprecated Since more than one GPU can exist on a single board this function is deprecated in favor
             of \ref nvmlDeviceGetHandleByUUID.
             For dual GPU boards this function will return NVML_ERROR_INVALID_ARGUMENT.

 Starting from NVML 5, this API causes NVML to initialize the target GPU
 NVML may initialize additional GPUs as it searches for the target GPU

 @param serial                               The board serial number of the target GPU
 @param device                               Reference in which to return the device handle

 @return
         - \ref NVML_SUCCESS                  if \a device has been set
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a serial is invalid, \a device is NULL or more than one
                                              device has the same serial (dual GPU boards)
         - \ref NVML_ERROR_NOT_FOUND          if \a serial does not match a valid device on the system
         - \ref NVML_ERROR_INSUFFICIENT_POWER if any attached devices have improperly attached external power cables
         - \ref NVML_ERROR_IRQ_ISSUE          if NVIDIA kernel detected an interrupt issue with the attached GPUs
         - \ref NVML_ERROR_GPU_IS_LOST        if any GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error

 @see nvmlDeviceGetSerial
 @see nvmlDeviceGetHandleByUUID*/
    fn nvmlDeviceGetHandleBySerial(
        serial: *const ::core::ffi::c_char,
        device: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Acquire the handle for a particular device, based on its globally unique immutable UUID (in ASCII format) associated with each device.

 For all products.

 @param uuid                                 The UUID of the target GPU or MIG instance
 @param device                               Reference in which to return the device handle or MIG device handle

 Starting from NVML 5, this API causes NVML to initialize the target GPU
 NVML may initialize additional GPUs as it searches for the target GPU

 @return
         - \ref NVML_SUCCESS                  if \a device has been set
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a uuid is invalid or \a device is null
         - \ref NVML_ERROR_NOT_FOUND          if \a uuid does not match a valid device on the system
         - \ref NVML_ERROR_INSUFFICIENT_POWER if any attached devices have improperly attached external power cables
         - \ref NVML_ERROR_IRQ_ISSUE          if NVIDIA kernel detected an interrupt issue with the attached GPUs
         - \ref NVML_ERROR_GPU_IS_LOST        if any GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error

 @see nvmlDeviceGetUUID*/
    fn nvmlDeviceGetHandleByUUID(
        uuid: *const ::core::ffi::c_char,
        device: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Acquire the handle for a particular device, based on its globally unique immutable UUID (in either ASCII or binary format) associated with each device.
 See \ref nvmlUUID_v1_t for more information on the UUID struct. The caller must set the appropriate version prior to calling this API.

 For all products.

 @param[in] uuid                                  The UUID of the target GPU or MIG instance
 @param[out] device                               Reference in which to return the device handle or MIG device handle

 This API causes NVML to initialize the target GPU
 NVML may initialize additional GPUs as it searches for the target GPU

 @return
         - \ref NVML_SUCCESS                          if \a device has been set
         - \ref NVML_ERROR_UNINITIALIZED              if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           if \a uuid is invalid, \a device is null or \a uuid->type is invalid
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  if the provided version is invalid/unsupported
         - \ref NVML_ERROR_NOT_FOUND                  if \a uuid does not match a valid device on the system
         - \ref NVML_ERROR_GPU_IS_LOST                if any GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN                    on any unexpected error*/
    fn nvmlDeviceGetHandleByUUIDV(
        uuid: *const cuda_types::nvml::nvmlUUID_t,
        device: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Acquire the handle for a particular device, based on its PCI bus id.

 For all products.

 This value corresponds to the nvmlPciInfo_t::busId returned by \ref nvmlDeviceGetPciInfo_v3().

 Starting from NVML 5, this API causes NVML to initialize the target GPU
 NVML may initialize additional GPUs if:
  - The target GPU is an SLI slave

 \note NVML 4.304 and older version of nvmlDeviceGetHandleByPciBusId"_v1" returns NVML_ERROR_NOT_FOUND
       instead of NVML_ERROR_NO_PERMISSION.

 @param pciBusId                             The PCI bus id of the target GPU
                                             Accept the following formats (all numbers in hexadecimal):
                                               domain:bus:device.function in format %x:%x:%x.%x
                                               domain:bus:device in format %x:%x:%x
                                               bus:device.function in format %x:%x.%x

 @param device                               Reference in which to return the device handle

 @return
         - \ref NVML_SUCCESS                  if \a device has been set
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a pciBusId is invalid or \a device is NULL
         - \ref NVML_ERROR_NOT_FOUND          if \a pciBusId does not match a valid device on the system
         - \ref NVML_ERROR_INSUFFICIENT_POWER if the attached device has improperly attached external power cables
         - \ref NVML_ERROR_NO_PERMISSION      if the user doesn't have permission to talk to this device
         - \ref NVML_ERROR_IRQ_ISSUE          if NVIDIA kernel detected an interrupt issue with the attached GPUs
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetHandleByPciBusId_v2(
        pciBusId: *const ::core::ffi::c_char,
        device: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the name of this device.

 For all products.

 The name is an alphanumeric string that denotes a particular product, e.g. Tesla &tm; C2070. It will not
 exceed 96 characters in length (including the NULL terminator).  See \ref
 nvmlConstants::NVML_DEVICE_NAME_V2_BUFFER_SIZE.

 When used with MIG device handles the API returns MIG device names which can be used to identify devices
 based on their attributes.

 @param device                               The identifier of the target device
 @param name                                 Reference in which to return the product name
 @param length                               The maximum allowed length of the string returned in \a name

 @return
         - \ref NVML_SUCCESS                 if \a name has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a name is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetName(
        device: cuda_types::nvml::nvmlDevice_t,
        name: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the brand of this device.

 For all products.

 The type is a member of \ref nvmlBrandType_t defined above.

 @param device                               The identifier of the target device
 @param type                                 Reference in which to return the product brand type

 @return
         - \ref NVML_SUCCESS                 if \a name has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a type is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetBrand(
        device: cuda_types::nvml::nvmlDevice_t,
        type_: *mut cuda_types::nvml::nvmlBrandType_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the NVML index of this device.

 For all products.

 Valid indices are derived from the \a accessibleDevices count returned by
   \ref nvmlDeviceGetCount_v2(). For example, if \a accessibleDevices is 2 the valid indices
   are 0 and 1, corresponding to GPU 0 and GPU 1.

 The order in which NVML enumerates devices has no guarantees of consistency between reboots. For that reason it
   is recommended that devices be looked up by their PCI ids or GPU UUID. See
   \ref nvmlDeviceGetHandleByPciBusId_v2() and \ref nvmlDeviceGetHandleByUUID().

 When used with MIG device handles this API returns indices that can be
 passed to \ref nvmlDeviceGetMigDeviceHandleByIndex to retrieve an identical handle.
 MIG device indices are unique within a device.

 Note: The NVML index may not correlate with other APIs, such as the CUDA device index.

 @param device                               The identifier of the target device
 @param index                                Reference in which to return the NVML index of the device

 @return
         - \ref NVML_SUCCESS                 if \a index has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a index is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetHandleByIndex()
 @see nvmlDeviceGetCount()*/
    fn nvmlDeviceGetIndex(
        device: cuda_types::nvml::nvmlDevice_t,
        index: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the globally unique board serial number associated with this device's board.

 For all products with an inforom.

 The serial number is an alphanumeric string that will not exceed 30 characters (including the NULL terminator).
 This number matches the serial number tag that is physically attached to the board.  See \ref
 nvmlConstants::NVML_DEVICE_SERIAL_BUFFER_SIZE.

 @param device                               The identifier of the target device
 @param serial                               Reference in which to return the board/module serial number
 @param length                               The maximum allowed length of the string returned in \a serial

 @return
         - \ref NVML_SUCCESS                 if \a serial has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a serial is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetSerial(
        device: cuda_types::nvml::nvmlDevice_t,
        serial: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get a unique identifier for the device module on the baseboard

 This API retrieves a unique identifier for each GPU module that exists on a given baseboard.
 For non-baseboard products, this ID would always be 0.

 @param device                               The identifier of the target device
 @param moduleId                             Unique identifier for the GPU module

 @return
         - \ref NVML_SUCCESS                 if \a moduleId has been successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a moduleId is invalid
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetModuleId(
        device: cuda_types::nvml::nvmlDevice_t,
        moduleId: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the Device's C2C Mode information

 @param device                               The identifier of the target device
 @param c2cModeInfo                          Output struct containing the device's C2C Mode info

 @return
         - \ref NVML_SUCCESS                 if \a C2C Mode Infor query is successful
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a serial is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetC2cModeInfoV(
        device: cuda_types::nvml::nvmlDevice_t,
        c2cModeInfo: *mut cuda_types::nvml::nvmlC2cModeInfo_v1_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves an array of unsigned ints (sized to nodeSetSize) of bitmasks with
 the ideal memory affinity within node or socket for the device.
 For example, if NUMA node 0, 1 are ideal within the socket for the device and nodeSetSize ==  1,
     result[0] = 0x3

 \note If requested scope is not applicable to the target topology, the API
       will fall back to reporting the memory affinity for the immediate non-I/O
       ancestor of the device.

 For Kepler &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               The identifier of the target device
 @param nodeSetSize                          The size of the nodeSet array that is safe to access
 @param nodeSet                              Array reference in which to return a bitmask of NODEs, 64 NODEs per
                                             unsigned long on 64-bit machines, 32 on 32-bit machines
 @param scope                                Scope that change the default behavior

 @return
         - \ref NVML_SUCCESS                 if \a NUMA node Affinity has been filled
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, nodeSetSize == 0, nodeSet is NULL or scope is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMemoryAffinity(
        device: cuda_types::nvml::nvmlDevice_t,
        nodeSetSize: ::core::ffi::c_uint,
        nodeSet: *mut ::core::ffi::c_ulong,
        scope: cuda_types::nvml::nvmlAffinityScope_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves an array of unsigned ints (sized to cpuSetSize) of bitmasks with the
 ideal CPU affinity within node or socket for the device.
 For example, if processors 0, 1, 32, and 33 are ideal for the device and cpuSetSize == 2,
     result[0] = 0x3, result[1] = 0x3

 \note If requested scope is not applicable to the target topology, the API
       will fall back to reporting the CPU affinity for the immediate non-I/O
       ancestor of the device.

 For Kepler &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               The identifier of the target device
 @param cpuSetSize                           The size of the cpuSet array that is safe to access
 @param cpuSet                               Array reference in which to return a bitmask of CPUs, 64 CPUs per
                                                 unsigned long on 64-bit machines, 32 on 32-bit machines
 @param scope                                Scope that change the default behavior

 @return
         - \ref NVML_SUCCESS                 if \a cpuAffinity has been filled
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, cpuSetSize == 0, cpuSet is NULL or sope is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetCpuAffinityWithinScope(
        device: cuda_types::nvml::nvmlDevice_t,
        cpuSetSize: ::core::ffi::c_uint,
        cpuSet: *mut ::core::ffi::c_ulong,
        scope: cuda_types::nvml::nvmlAffinityScope_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves an array of unsigned ints (sized to cpuSetSize) of bitmasks with the ideal CPU affinity for the device
 For example, if processors 0, 1, 32, and 33 are ideal for the device and cpuSetSize == 2,
     result[0] = 0x3, result[1] = 0x3
 This is equivalent to calling \ref nvmlDeviceGetCpuAffinityWithinScope with \ref NVML_AFFINITY_SCOPE_NODE.

 For Kepler &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               The identifier of the target device
 @param cpuSetSize                           The size of the cpuSet array that is safe to access
 @param cpuSet                               Array reference in which to return a bitmask of CPUs, 64 CPUs per
                                                 unsigned long on 64-bit machines, 32 on 32-bit machines

 @return
         - \ref NVML_SUCCESS                 if \a cpuAffinity has been filled
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, cpuSetSize == 0, or cpuSet is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetCpuAffinity(
        device: cuda_types::nvml::nvmlDevice_t,
        cpuSetSize: ::core::ffi::c_uint,
        cpuSet: *mut ::core::ffi::c_ulong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Sets the ideal affinity for the calling thread and device using the guidelines
 given in nvmlDeviceGetCpuAffinity().  Note, this is a change as of version 8.0.
 Older versions set the affinity for a calling process and all children.
 Currently supports up to 1024 processors.

 For Kepler &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               The identifier of the target device

 @return
         - \ref NVML_SUCCESS                 if the calling process has been successfully bound
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetCpuAffinity(
        device: cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Clear all affinity bindings for the calling thread.  Note, this is a change as of version
 8.0 as older versions cleared the affinity for a calling process and all children.

 For Kepler &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               The identifier of the target device

 @return
         - \ref NVML_SUCCESS                 if the calling process has been successfully unbound
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceClearCpuAffinity(
        device: cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the NUMA node of the given GPU device.
 This only applies to platforms where the GPUs are NUMA nodes.

 @param[in]      device                  The device handle
 @param[out]     node                    NUMA node ID of the device

 @returns
         - \ref NVML_SUCCESS                  if the NUMA node is retrieved successfully
         - \ref NVML_ERROR_NOT_SUPPORTED      if request is not supported on the current platform
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a device \a node is invalid*/
    fn nvmlDeviceGetNumaNodeId(
        device: cuda_types::nvml::nvmlDevice_t,
        node: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /// @}
    fn nvmlDeviceGetTopologyCommonAncestor(
        device1: cuda_types::nvml::nvmlDevice_t,
        device2: cuda_types::nvml::nvmlDevice_t,
        pathInfo: *mut cuda_types::nvml::nvmlGpuTopologyLevel_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the set of GPUs that are nearest to a given device at a specific interconnectivity level
 For all products.
 Supported on Linux only.

 @param device                               The identifier of the first device
 @param level                                The \ref nvmlGpuTopologyLevel_t level to search for other GPUs
 @param count                                When zero, is set to the number of matching GPUs such that \a deviceArray
                                             can be malloc'd.  When non-zero, \a deviceArray will be filled with \a count
                                             number of device handles.
 @param deviceArray                          An array of device handles for GPUs found at \a level

 @return
         - \ref NVML_SUCCESS                 if \a deviceArray or \a count (if initially zero) has been set
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a level, or \a count is invalid, or \a deviceArray is NULL with a non-zero \a count
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device or OS does not support this feature
         - \ref NVML_ERROR_UNKNOWN           an error has occurred in underlying topology discovery*/
    fn nvmlDeviceGetTopologyNearestGpus(
        device: cuda_types::nvml::nvmlDevice_t,
        level: cuda_types::nvml::nvmlGpuTopologyLevel_t,
        count: *mut ::core::ffi::c_uint,
        deviceArray: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the status for a given p2p capability index between a given pair of GPU

 @param device1                              The first device
 @param device2                              The second device
 @param p2pIndex                             p2p Capability Index being looked for between \a device1 and \a device2
 @param p2pStatus                            Reference in which to return the status of the \a p2pIndex
                                             between \a device1 and \a device2
 @return
         - \ref NVML_SUCCESS         if \a p2pStatus has been populated
         - \ref NVML_ERROR_INVALID_ARGUMENT     if \a device1 or \a device2 or \a p2pIndex is invalid or \a p2pStatus is NULL
         - \ref NVML_ERROR_UNKNOWN              on any unexpected error*/
    fn nvmlDeviceGetP2PStatus(
        device1: cuda_types::nvml::nvmlDevice_t,
        device2: cuda_types::nvml::nvmlDevice_t,
        p2pIndex: cuda_types::nvml::nvmlGpuP2PCapsIndex_t,
        p2pStatus: *mut cuda_types::nvml::nvmlGpuP2PStatus_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the globally unique immutable UUID associated with this device, as a 5 part hexadecimal string,
 that augments the immutable, board serial identifier.

 For all products.

 The UUID is a globally unique identifier. It is the only available identifier for pre-Fermi-architecture products.
 It does NOT correspond to any identifier printed on the board.  It will not exceed 96 characters in length
 (including the NULL terminator).  See \ref nvmlConstants::NVML_DEVICE_UUID_V2_BUFFER_SIZE.

 When used with MIG device handles the API returns globally unique UUIDs which can be used to identify MIG
 devices across both GPU and MIG devices. UUIDs are immutable for the lifetime of a MIG device.

 @param device                               The identifier of the target device
 @param uuid                                 Reference in which to return the GPU UUID
 @param length                               The maximum allowed length of the string returned in \a uuid

 @return
         - \ref NVML_SUCCESS                 if \a uuid has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a uuid is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetUUID(
        device: cuda_types::nvml::nvmlDevice_t,
        uuid: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves minor number for the device. The minor number for the device is such that the Nvidia device node file for
 each GPU will have the form /dev/nvidia[minor number].

 For all products.
 Supported only for Linux

 @param device                                The identifier of the target device
 @param minorNumber                           Reference in which to return the minor number for the device
 @return
         - \ref NVML_SUCCESS                 if the minor number is successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a minorNumber is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMinorNumber(
        device: cuda_types::nvml::nvmlDevice_t,
        minorNumber: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the the device board part number which is programmed into the board's InfoROM

 For all products.

 @param device                                Identifier of the target device
 @param partNumber                            Reference to the buffer to return
 @param length                                Length of the buffer reference

 @return
         - \ref NVML_SUCCESS                  if \a partNumber has been set
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_NOT_SUPPORTED      if the needed VBIOS fields have not been filled
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a device is invalid or \a serial is NULL
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetBoardPartNumber(
        device: cuda_types::nvml::nvmlDevice_t,
        partNumber: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the version information for the device's infoROM object.

 For all products with an inforom.

 Fermi and higher parts have non-volatile on-board memory for persisting device info, such as aggregate
 ECC counts. The version of the data structures in this memory may change from time to time. It will not
 exceed 16 characters in length (including the NULL terminator).
 See \ref nvmlConstants::NVML_DEVICE_INFOROM_VERSION_BUFFER_SIZE.

 See \ref nvmlInforomObject_t for details on the available infoROM objects.

 @param device                               The identifier of the target device
 @param object                               The target infoROM object
 @param version                              Reference in which to return the infoROM version
 @param length                               The maximum allowed length of the string returned in \a version

 @return
         - \ref NVML_SUCCESS                 if \a version has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a version is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not have an infoROM
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetInforomImageVersion*/
    fn nvmlDeviceGetInforomVersion(
        device: cuda_types::nvml::nvmlDevice_t,
        object: cuda_types::nvml::nvmlInforomObject_t,
        version: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the global infoROM image version

 For all products with an inforom.

 Image version just like VBIOS version uniquely describes the exact version of the infoROM flashed on the board
 in contrast to infoROM object version which is only an indicator of supported features.
 Version string will not exceed 16 characters in length (including the NULL terminator).
 See \ref nvmlConstants::NVML_DEVICE_INFOROM_VERSION_BUFFER_SIZE.

 @param device                               The identifier of the target device
 @param version                              Reference in which to return the infoROM image version
 @param length                               The maximum allowed length of the string returned in \a version

 @return
         - \ref NVML_SUCCESS                 if \a version has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a version is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not have an infoROM
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetInforomVersion*/
    fn nvmlDeviceGetInforomImageVersion(
        device: cuda_types::nvml::nvmlDevice_t,
        version: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the checksum of the configuration stored in the device's infoROM.

 For all products with an inforom.

 Can be used to make sure that two GPUs have the exact same configuration.
 Current checksum takes into account configuration stored in PWR and ECC infoROM objects.
 Checksum can change between driver releases or when user changes configuration (e.g. disable/enable ECC)

 @param device                               The identifier of the target device
 @param checksum                             Reference in which to return the infoROM configuration checksum

 @return
         - \ref NVML_SUCCESS                 if \a checksum has been set
         - \ref NVML_ERROR_CORRUPTED_INFOROM if the device's checksum couldn't be retrieved due to infoROM corruption
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a checksum is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetInforomConfigurationChecksum(
        device: cuda_types::nvml::nvmlDevice_t,
        checksum: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Reads the infoROM from the flash and verifies the checksums.

 For all products with an inforom.

 @param device                               The identifier of the target device

 @return
         - \ref NVML_SUCCESS                 if infoROM is not corrupted
         - \ref NVML_ERROR_CORRUPTED_INFOROM if the device's infoROM is corrupted
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceValidateInforom(
        device: cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the timestamp and the duration of the last flush of the BBX (blackbox) infoROM object during the current run.

 For all products with an inforom.

 @param device                               The identifier of the target device
 @param timestamp                            The start timestamp of the last BBX Flush
 @param durationUs                           The duration (us) of the last BBX Flush

 @return
         - \ref NVML_SUCCESS                 if \a timestamp and \a durationUs are successfully retrieved
         - \ref NVML_ERROR_NOT_READY         if the BBX object has not been flushed yet
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not have an infoROM
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetInforomVersion*/
    fn nvmlDeviceGetLastBBXFlushTime(
        device: cuda_types::nvml::nvmlDevice_t,
        timestamp: *mut ::core::ffi::c_ulonglong,
        durationUs: *mut ::core::ffi::c_ulong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the display mode for the device.

 For all products.

 This method indicates whether a physical display (e.g. monitor) is currently connected to
 any of the device's connectors.

 See \ref nvmlEnableState_t for details on allowed modes.

 @param device                               The identifier of the target device
 @param display                              Reference in which to return the display mode

 @return
         - \ref NVML_SUCCESS                 if \a display has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a display is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetDisplayMode(
        device: cuda_types::nvml::nvmlDevice_t,
        display: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the display active state for the device.

 For all products.

 This method indicates whether a display is initialized on the device.
 For example whether X Server is attached to this device and has allocated memory for the screen.

 Display can be active even when no monitor is physically attached.

 See \ref nvmlEnableState_t for details on allowed modes.

 @param device                               The identifier of the target device
 @param isActive                             Reference in which to return the display active state

 @return
         - \ref NVML_SUCCESS                 if \a isActive has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a isActive is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetDisplayActive(
        device: cuda_types::nvml::nvmlDevice_t,
        isActive: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the persistence mode associated with this device.

 For all products.
 For Linux only.

 When driver persistence mode is enabled the driver software state is not torn down when the last
 client disconnects. By default this feature is disabled.

 See \ref nvmlEnableState_t for details on allowed modes.

 @param device                               The identifier of the target device
 @param mode                                 Reference in which to return the current driver persistence mode

 @return
         - \ref NVML_SUCCESS                 if \a mode has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a mode is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceSetPersistenceMode()*/
    fn nvmlDeviceGetPersistenceMode(
        device: cuda_types::nvml::nvmlDevice_t,
        mode: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves PCI attributes of this device.

 For all products.

 See \ref nvmlPciInfoExt_v1_t for details on the available PCI info.

 @param device                               The identifier of the target device
 @param pci                                  Reference in which to return the PCI info

 @return
         - \ref NVML_SUCCESS                 if \a pci has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a pci is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPciInfoExt(
        device: cuda_types::nvml::nvmlDevice_t,
        pci: *mut cuda_types::nvml::nvmlPciInfoExt_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the PCI attributes of this device.

 For all products.

 See \ref nvmlPciInfo_t for details on the available PCI info.

 @param device                               The identifier of the target device
 @param pci                                  Reference in which to return the PCI info

 @return
         - \ref NVML_SUCCESS                 if \a pci has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a pci is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPciInfo_v3(
        device: cuda_types::nvml::nvmlDevice_t,
        pci: *mut cuda_types::nvml::nvmlPciInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the maximum PCIe link generation possible with this device and system

 I.E. for a generation 2 PCIe device attached to a generation 1 PCIe bus the max link generation this function will
 report is generation 1.

 For Fermi &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param maxLinkGen                           Reference in which to return the max PCIe link generation

 @return
         - \ref NVML_SUCCESS                 if \a maxLinkGen has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a maxLinkGen is null
         - \ref NVML_ERROR_NOT_SUPPORTED     if PCIe link information is not available
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMaxPcieLinkGeneration(
        device: cuda_types::nvml::nvmlDevice_t,
        maxLinkGen: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the maximum PCIe link generation supported by this device

 For Fermi &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param maxLinkGenDevice                     Reference in which to return the max PCIe link generation

 @return
         - \ref NVML_SUCCESS                 if \a maxLinkGenDevice has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a maxLinkGenDevice is null
         - \ref NVML_ERROR_NOT_SUPPORTED     if PCIe link information is not available
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetGpuMaxPcieLinkGeneration(
        device: cuda_types::nvml::nvmlDevice_t,
        maxLinkGenDevice: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the maximum PCIe link width possible with this device and system

 I.E. for a device with a 16x PCIe bus width attached to a 8x PCIe system bus this function will report
 a max link width of 8.

 For Fermi &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param maxLinkWidth                         Reference in which to return the max PCIe link generation

 @return
         - \ref NVML_SUCCESS                 if \a maxLinkWidth has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a maxLinkWidth is null
         - \ref NVML_ERROR_NOT_SUPPORTED     if PCIe link information is not available
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMaxPcieLinkWidth(
        device: cuda_types::nvml::nvmlDevice_t,
        maxLinkWidth: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current PCIe link generation

 For Fermi &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param currLinkGen                          Reference in which to return the current PCIe link generation

 @return
         - \ref NVML_SUCCESS                 if \a currLinkGen has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a currLinkGen is null
         - \ref NVML_ERROR_NOT_SUPPORTED     if PCIe link information is not available
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetCurrPcieLinkGeneration(
        device: cuda_types::nvml::nvmlDevice_t,
        currLinkGen: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current PCIe link width

 For Fermi &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param currLinkWidth                        Reference in which to return the current PCIe link generation

 @return
         - \ref NVML_SUCCESS                 if \a currLinkWidth has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a currLinkWidth is null
         - \ref NVML_ERROR_NOT_SUPPORTED     if PCIe link information is not available
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetCurrPcieLinkWidth(
        device: cuda_types::nvml::nvmlDevice_t,
        currLinkWidth: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve PCIe utilization information.
 This function is querying a byte counter over a 20ms interval and thus is the
   PCIe throughput over that interval.

 For Maxwell &tm; or newer fully supported devices.

 This method is not supported in virtual machines running virtual GPU (vGPU).

 @param device                               The identifier of the target device
 @param counter                              The specific counter that should be queried \ref nvmlPcieUtilCounter_t
 @param value                                Reference in which to return throughput in KB/s

 @return
         - \ref NVML_SUCCESS                 if \a value has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a counter is invalid, or \a value is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPcieThroughput(
        device: cuda_types::nvml::nvmlDevice_t,
        counter: cuda_types::nvml::nvmlPcieUtilCounter_t,
        value: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the PCIe replay counter.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param value                                Reference in which to return the counter's value

 @return
         - \ref NVML_SUCCESS                 if \a value has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a value is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPcieReplayCounter(
        device: cuda_types::nvml::nvmlDevice_t,
        value: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current clock speeds for the device.

 For Fermi &tm; or newer fully supported devices.

 See \ref nvmlClockType_t for details on available clock information.

 @param device                               The identifier of the target device
 @param type                                 Identify which clock domain to query
 @param clock                                Reference in which to return the clock speed in MHz

 @return
         - \ref NVML_SUCCESS                 if \a clock has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a clock is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device cannot report the specified clock
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetClockInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        type_: cuda_types::nvml::nvmlClockType_t,
        clock: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the maximum clock speeds for the device.

 For Fermi &tm; or newer fully supported devices.

 See \ref nvmlClockType_t for details on available clock information.

 \note On GPUs from Fermi family current P0 clocks (reported by \ref nvmlDeviceGetClockInfo) can differ from max clocks
       by few MHz.

 @param device                               The identifier of the target device
 @param type                                 Identify which clock domain to query
 @param clock                                Reference in which to return the clock speed in MHz

 @return
         - \ref NVML_SUCCESS                 if \a clock has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a clock is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device cannot report the specified clock
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMaxClockInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        type_: cuda_types::nvml::nvmlClockType_t,
        clock: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the GPCCLK VF offset value
 @param[in]   device                         The identifier of the target device
 @param[out]  offset                         The retrieved GPCCLK VF offset value

 @return
         - \ref NVML_SUCCESS                 if \a offset has been successfully queried
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a offset is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetGpcClkVfOffset(
        device: cuda_types::nvml::nvmlDevice_t,
        offset: *mut ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current setting of a clock that applications will use unless an overspec situation occurs.
 Can be changed using \ref nvmlDeviceSetApplicationsClocks.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param clockType                            Identify which clock domain to query
 @param clockMHz                             Reference in which to return the clock in MHz

 @return
         - \ref NVML_SUCCESS                 if \a clockMHz has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a clockMHz is NULL or \a clockType is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetApplicationsClock(
        device: cuda_types::nvml::nvmlDevice_t,
        clockType: cuda_types::nvml::nvmlClockType_t,
        clockMHz: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the default applications clock that GPU boots with or
 defaults to after \ref nvmlDeviceResetApplicationsClocks call.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param clockType                            Identify which clock domain to query
 @param clockMHz                             Reference in which to return the default clock in MHz

 @return
         - \ref NVML_SUCCESS                 if \a clockMHz has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a clockMHz is NULL or \a clockType is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 \see nvmlDeviceGetApplicationsClock*/
    fn nvmlDeviceGetDefaultApplicationsClock(
        device: cuda_types::nvml::nvmlDevice_t,
        clockType: cuda_types::nvml::nvmlClockType_t,
        clockMHz: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the clock speed for the clock specified by the clock type and clock ID.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param clockType                            Identify which clock domain to query
 @param clockId                              Identify which clock in the domain to query
 @param clockMHz                             Reference in which to return the clock in MHz

 @return
         - \ref NVML_SUCCESS                 if \a clockMHz has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a clockMHz is NULL or \a clockType is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetClock(
        device: cuda_types::nvml::nvmlDevice_t,
        clockType: cuda_types::nvml::nvmlClockType_t,
        clockId: cuda_types::nvml::nvmlClockId_t,
        clockMHz: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the customer defined maximum boost clock speed specified by the given clock type.

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param clockType                            Identify which clock domain to query
 @param clockMHz                             Reference in which to return the clock in MHz

 @return
         - \ref NVML_SUCCESS                 if \a clockMHz has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a clockMHz is NULL or \a clockType is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device or the \a clockType on this device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMaxCustomerBoostClock(
        device: cuda_types::nvml::nvmlDevice_t,
        clockType: cuda_types::nvml::nvmlClockType_t,
        clockMHz: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the list of possible memory clocks that can be used as an argument for \ref nvmlDeviceSetApplicationsClocks.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param count                                Reference in which to provide the \a clocksMHz array size, and
                                             to return the number of elements
 @param clocksMHz                            Reference in which to return the clock in MHz

 @return
         - \ref NVML_SUCCESS                 if \a count and \a clocksMHz have been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a count is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a count is too small (\a count is set to the number of
                                                required elements)
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceSetApplicationsClocks
 @see nvmlDeviceGetSupportedGraphicsClocks*/
    fn nvmlDeviceGetSupportedMemoryClocks(
        device: cuda_types::nvml::nvmlDevice_t,
        count: *mut ::core::ffi::c_uint,
        clocksMHz: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the list of possible graphics clocks that can be used as an argument for \ref nvmlDeviceSetApplicationsClocks.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param memoryClockMHz                       Memory clock for which to return possible graphics clocks
 @param count                                Reference in which to provide the \a clocksMHz array size, and
                                             to return the number of elements
 @param clocksMHz                            Reference in which to return the clocks in MHz

 @return
         - \ref NVML_SUCCESS                 if \a count and \a clocksMHz have been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_NOT_FOUND         if the specified \a memoryClockMHz is not a supported frequency
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a clock is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a count is too small
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceSetApplicationsClocks
 @see nvmlDeviceGetSupportedMemoryClocks*/
    fn nvmlDeviceGetSupportedGraphicsClocks(
        device: cuda_types::nvml::nvmlDevice_t,
        memoryClockMHz: ::core::ffi::c_uint,
        count: *mut ::core::ffi::c_uint,
        clocksMHz: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the current state of Auto Boosted clocks on a device and store it in \a isEnabled

 For Kepler &tm; or newer fully supported devices.

 Auto Boosted clocks are enabled by default on some hardware, allowing the GPU to run at higher clock rates
 to maximize performance as thermal limits allow.

 On Pascal and newer hardware, Auto Aoosted clocks are controlled through application clocks.
 Use \ref nvmlDeviceSetApplicationsClocks and \ref nvmlDeviceResetApplicationsClocks to control Auto Boost
 behavior.

 @param device                               The identifier of the target device
 @param isEnabled                            Where to store the current state of Auto Boosted clocks of the target device
 @param defaultIsEnabled                     Where to store the default Auto Boosted clocks behavior of the target device that the device will
                                                 revert to when no applications are using the GPU

 @return
         - \ref NVML_SUCCESS                 If \a isEnabled has been been set with the Auto Boosted clocks state of \a device
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a isEnabled is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support Auto Boosted clocks
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error
*/
    fn nvmlDeviceGetAutoBoostedClocksEnabled(
        device: cuda_types::nvml::nvmlDevice_t,
        isEnabled: *mut cuda_types::nvml::nvmlEnableState_t,
        defaultIsEnabled: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the intended operating speed of the device's fan.

 Note: The reported speed is the intended fan speed.  If the fan is physically blocked and unable to spin, the
 output will not match the actual fan speed.

 For all discrete products with dedicated fans.

 The fan speed is expressed as a percentage of the product's maximum noise tolerance fan speed.
 This value may exceed 100% in certain cases.

 @param device                               The identifier of the target device
 @param speed                                Reference in which to return the fan speed percentage

 @return
         - \ref NVML_SUCCESS                 if \a speed has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a speed is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not have a fan
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetFanSpeed(
        device: cuda_types::nvml::nvmlDevice_t,
        speed: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the intended operating speed of the device's specified fan.

 Note: The reported speed is the intended fan speed. If the fan is physically blocked and unable to spin, the
 output will not match the actual fan speed.

 For all discrete products with dedicated fans.

 The fan speed is expressed as a percentage of the product's maximum noise tolerance fan speed.
 This value may exceed 100% in certain cases.

 @param device                                The identifier of the target device
 @param fan                                   The index of the target fan, zero indexed.
 @param speed                                 Reference in which to return the fan speed percentage

 @return
        - \ref NVML_SUCCESS                   if \a speed has been set
        - \ref NVML_ERROR_UNINITIALIZED       if the library has not been successfully initialized
        - \ref NVML_ERROR_INVALID_ARGUMENT    if \a device is invalid, \a fan is not an acceptable index, or \a speed is NULL
        - \ref NVML_ERROR_NOT_SUPPORTED       if the device does not have a fan or is newer than Maxwell
        - \ref NVML_ERROR_GPU_IS_LOST         if the target GPU has fallen off the bus or is otherwise inaccessible
        - \ref NVML_ERROR_UNKNOWN             on any unexpected error*/
    fn nvmlDeviceGetFanSpeed_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        fan: ::core::ffi::c_uint,
        speed: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the intended operating speed in rotations per minute (RPM) of the device's specified fan.

 For Maxwell &tm; or newer fully supported devices.

 For all discrete products with dedicated fans.

 Note: The reported speed is the intended fan speed. If the fan is physically blocked and unable to spin, the
 output will not match the actual fan speed.

 @param device                               The identifier of the target device
 @param fanSpeed                             Structure specifying the index of the target fan (input) and
                                             retrieved fan speed value (output)

 @return
         - \ref NVML_SUCCESS                         If everything worked
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device is invalid, \a fan is not an acceptable
                                                          index, or \a speed is NULL
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_NOT_SUPPORTED             If the \a device does not support this feature*/
    fn nvmlDeviceGetFanSpeedRPM(
        device: cuda_types::nvml::nvmlDevice_t,
        fanSpeed: *mut cuda_types::nvml::nvmlFanSpeedInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the intended target speed of the device's specified fan.

 Normally, the driver dynamically adjusts the fan based on
 the needs of the GPU.  But when user set fan speed using nvmlDeviceSetFanSpeed_v2,
 the driver will attempt to make the fan achieve the setting in
 nvmlDeviceSetFanSpeed_v2.  The actual current speed of the fan
 is reported in nvmlDeviceGetFanSpeed_v2.

 For all discrete products with dedicated fans.

 The fan speed is expressed as a percentage of the product's maximum noise tolerance fan speed.
 This value may exceed 100% in certain cases.

 @param device                                The identifier of the target device
 @param fan                                   The index of the target fan, zero indexed.
 @param targetSpeed                           Reference in which to return the fan speed percentage

 @return
        - \ref NVML_SUCCESS                   if \a speed has been set
        - \ref NVML_ERROR_UNINITIALIZED       if the library has not been successfully initialized
        - \ref NVML_ERROR_INVALID_ARGUMENT    if \a device is invalid, \a fan is not an acceptable index, or \a speed is NULL
        - \ref NVML_ERROR_NOT_SUPPORTED       if the device does not have a fan or is newer than Maxwell
        - \ref NVML_ERROR_GPU_IS_LOST         if the target GPU has fallen off the bus or is otherwise inaccessible
        - \ref NVML_ERROR_UNKNOWN             on any unexpected error*/
    fn nvmlDeviceGetTargetFanSpeed(
        device: cuda_types::nvml::nvmlDevice_t,
        fan: ::core::ffi::c_uint,
        targetSpeed: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the min and max fan speed that user can set for the GPU fan.

 For all cuda-capable discrete products with fans

 @param device                        The identifier of the target device
 @param minSpeed                      The minimum speed allowed to set
 @param maxSpeed                      The maximum speed allowed to set

 return
         NVML_SUCCESS                 if speed has been adjusted
         NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         NVML_ERROR_INVALID_ARGUMENT  if device is invalid
         NVML_ERROR_NOT_SUPPORTED     if the device does not support this
                                      (doesn't have fans)
         NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMinMaxFanSpeed(
        device: cuda_types::nvml::nvmlDevice_t,
        minSpeed: *mut ::core::ffi::c_uint,
        maxSpeed: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets current fan control policy.

 For Maxwell &tm; or newer fully supported devices.

 For all cuda-capable discrete products with fans

 device                               The identifier of the target \a device
 policy                               Reference in which to return the fan control \a policy

 return
         NVML_SUCCESS                 if \a policy has been populated
         NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a policy is null or the \a fan given doesn't reference
                                            a fan that exists.
         NVML_ERROR_NOT_SUPPORTED     if the \a device is older than Maxwell
         NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetFanControlPolicy_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        fan: ::core::ffi::c_uint,
        policy: *mut cuda_types::nvml::nvmlFanControlPolicy_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the number of fans on the device.

 For all discrete products with dedicated fans.

 @param device                               The identifier of the target device
 @param numFans                              The number of fans

 @return
         - \ref NVML_SUCCESS                 if \a fan number query was successful
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a numFans is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not have a fan
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetNumFans(
        device: cuda_types::nvml::nvmlDevice_t,
        numFans: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /// @deprecated Use \ref nvmlDeviceGetTemperatureV instead
    fn nvmlDeviceGetTemperature(
        device: cuda_types::nvml::nvmlDevice_t,
        sensorType: cuda_types::nvml::nvmlTemperatureSensors_t,
        temp: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the cooler's information.
 Returns a cooler's control signal characteristics.  The possible types are restricted, Variable and Toggle.
 See \ref nvmlCoolerControl_t for details on available signal types.
 Returns objects that cooler cools. Targets may be GPU, Memory, Power Supply or All of these.
 See \ref nvmlCoolerTarget_t for details on available targets.

 For Maxwell &tm; or newer fully supported devices.

 For all discrete products with dedicated fans.

 @param[in]  device                               The identifier of the target device
 @param[out] coolerInfo                           Structure specifying the cooler's control signal characteristics (out)
                                                  and the target that cooler cools (out)

 @return
         - \ref NVML_SUCCESS                         If everything worked
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device is invalid, \a signalType or \a target is NULL
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_NOT_SUPPORTED             If the \a device does not support this feature*/
    fn nvmlDeviceGetCoolerInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        coolerInfo: *mut cuda_types::nvml::nvmlCoolerInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current temperature readings (in degrees C) for the given device.

 For all products.

 @param[in]       device                      Target device identifier.
 @param[in,out]   temperature                 Structure specifying the sensor type (input) and retrieved
                                              temperature value (output).

 @return
         - \ref NVML_SUCCESS                  if \a temp has been set
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a device is invalid, \a sensorType is invalid or \a temp is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED      if the device does not have the specified sensor
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetTemperatureV(
        device: cuda_types::nvml::nvmlDevice_t,
        temperature: *mut cuda_types::nvml::nvmlTemperature_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the temperature threshold for the GPU with the specified threshold type in degrees C.

 For Kepler &tm; or newer fully supported devices.

 See \ref nvmlTemperatureThresholds_t for details on available temperature thresholds.

 Note: This API is no longer the preferred interface for retrieving the following temperature thresholds
 on Ada and later architectures: NVML_TEMPERATURE_THRESHOLD_SHUTDOWN, NVML_TEMPERATURE_THRESHOLD_SLOWDOWN,
 NVML_TEMPERATURE_THRESHOLD_MEM_MAX and NVML_TEMPERATURE_THRESHOLD_GPU_MAX.

 Support for reading these temperature thresholds for Ada and later architectures would be removed from this
 API in future releases. Please use \ref nvmlDeviceGetFieldValues with NVML_FI_DEV_TEMPERATURE_* fields to retrieve
 temperature thresholds on these architectures.

 @param device                               The identifier of the target device
 @param thresholdType                        The type of threshold value queried
 @param temp                                 Reference in which to return the temperature reading
 @return
         - \ref NVML_SUCCESS                 if \a temp has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a thresholdType is invalid or \a temp is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not have a temperature sensor or is unsupported
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetTemperatureThreshold(
        device: cuda_types::nvml::nvmlDevice_t,
        thresholdType: cuda_types::nvml::nvmlTemperatureThresholds_t,
        temp: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the thermal margin temperature (distance to nearest slowdown threshold).

 @param[in]     device                                The identifier of the target device
 @param[in,out] marginTempInfo                        Versioned structure in which to return the temperature reading

 @returns
         - \ref NVML_SUCCESS                           if the margin temperature was retrieved successfully
         - \ref NVML_ERROR_NOT_SUPPORTED               if request is not supported on the current platform
         - \ref NVML_ERROR_INVALID_ARGUMENT            if \a device is invalid or \a temperature is NULL
         - \ref NVML_ERROR_GPU_IS_LOST                 if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH   if the right versioned structure is not used
         - \ref NVML_ERROR_UNKNOWN                     on any unexpected error*/
    fn nvmlDeviceGetMarginTemperature(
        device: cuda_types::nvml::nvmlDevice_t,
        marginTempInfo: *mut cuda_types::nvml::nvmlMarginTemperature_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Used to execute a list of thermal system instructions.

 @param device                               The identifier of the target device
 @param sensorIndex                          The index of the thermal sensor
 @param pThermalSettings                     Reference in which to return the thermal sensor information

 @return
         - \ref NVML_SUCCESS                 if \a pThermalSettings has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a pThermalSettings is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetThermalSettings(
        device: cuda_types::nvml::nvmlDevice_t,
        sensorIndex: ::core::ffi::c_uint,
        pThermalSettings: *mut cuda_types::nvml::nvmlGpuThermalSettings_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current performance state for the device.

 For Fermi &tm; or newer fully supported devices.

 See \ref nvmlPstates_t for details on allowed performance states.

 @param device                               The identifier of the target device
 @param pState                               Reference in which to return the performance state reading

 @return
         - \ref NVML_SUCCESS                 if \a pState has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a pState is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPerformanceState(
        device: cuda_types::nvml::nvmlDevice_t,
        pState: *mut cuda_types::nvml::nvmlPstates_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves current clocks event reasons.

 For all fully supported products.

 \note More than one bit can be enabled at the same time. Multiple reasons can be affecting clocks at once.

 @param device                                The identifier of the target device
 @param clocksEventReasons                    Reference in which to return bitmask of active clocks event
                                                  reasons

 @return
         - \ref NVML_SUCCESS                 if \a clocksEventReasons has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a clocksEventReasons is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlClocksEventReasons
 @see nvmlDeviceGetSupportedClocksEventReasons*/
    fn nvmlDeviceGetCurrentClocksEventReasons(
        device: cuda_types::nvml::nvmlDevice_t,
        clocksEventReasons: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /// @deprecated Use \ref nvmlDeviceGetCurrentClocksEventReasons instead
    fn nvmlDeviceGetCurrentClocksThrottleReasons(
        device: cuda_types::nvml::nvmlDevice_t,
        clocksThrottleReasons: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves bitmask of supported clocks event reasons that can be returned by
 \ref nvmlDeviceGetCurrentClocksEventReasons

 For all fully supported products.

 This method is not supported in virtual machines running virtual GPU (vGPU).

 @param device                               The identifier of the target device
 @param supportedClocksEventReasons       Reference in which to return bitmask of supported
                                              clocks event reasons

 @return
         - \ref NVML_SUCCESS                 if \a supportedClocksEventReasons has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a supportedClocksEventReasons is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlClocksEventReasons
 @see nvmlDeviceGetCurrentClocksEventReasons*/
    fn nvmlDeviceGetSupportedClocksEventReasons(
        device: cuda_types::nvml::nvmlDevice_t,
        supportedClocksEventReasons: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /// @deprecated Use \ref nvmlDeviceGetSupportedClocksEventReasons instead
    fn nvmlDeviceGetSupportedClocksThrottleReasons(
        device: cuda_types::nvml::nvmlDevice_t,
        supportedClocksThrottleReasons: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Deprecated: Use \ref nvmlDeviceGetPerformanceState. This function exposes an incorrect generalization.

 Retrieve the current performance state for the device.

 For Fermi &tm; or newer fully supported devices.

 See \ref nvmlPstates_t for details on allowed performance states.

 @param device                               The identifier of the target device
 @param pState                               Reference in which to return the performance state reading

 @return
         - \ref NVML_SUCCESS                 if \a pState has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a pState is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPowerState(
        device: cuda_types::nvml::nvmlDevice_t,
        pState: *mut cuda_types::nvml::nvmlPstates_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve performance monitor samples from the associated subdevice.

 @param device
 @param pDynamicPstatesInfo

 @return
         - \ref NVML_SUCCESS                 if \a pDynamicPstatesInfo has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a pDynamicPstatesInfo is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetDynamicPstatesInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        pDynamicPstatesInfo: *mut cuda_types::nvml::nvmlGpuDynamicPstatesInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the MemClk (Memory Clock) VF offset value.
 @param[in]   device                         The identifier of the target device
 @param[out]  offset                         The retrieved MemClk VF offset value

 @return
         - \ref NVML_SUCCESS                 if \a offset has been successfully queried
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a offset is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMemClkVfOffset(
        device: cuda_types::nvml::nvmlDevice_t,
        offset: *mut ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve min and max clocks of some clock domain for a given PState

 @param device                               The identifier of the target device
 @param type                                 Clock domain
 @param pstate                               PState to query
 @param minClockMHz                          Reference in which to return min clock frequency
 @param maxClockMHz                          Reference in which to return max clock frequency

 @return
         - \ref NVML_SUCCESS                 if everything worked
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a type or \a pstate are invalid or both
                                                  \a minClockMHz and \a maxClockMHz are NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature*/
    fn nvmlDeviceGetMinMaxClockOfPState(
        device: cuda_types::nvml::nvmlDevice_t,
        type_: cuda_types::nvml::nvmlClockType_t,
        pstate: cuda_types::nvml::nvmlPstates_t,
        minClockMHz: *mut ::core::ffi::c_uint,
        maxClockMHz: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get all supported Performance States (P-States) for the device.

 The returned array would contain a contiguous list of valid P-States supported by
 the device. If the number of supported P-States is fewer than the size of the array
 supplied missing elements would contain \a NVML_PSTATE_UNKNOWN.

 The number of elements in the returned list will never exceed \a NVML_MAX_GPU_PERF_PSTATES.

 @param device                               The identifier of the target device
 @param pstates                              Container to return the list of performance states
                                             supported by device
 @param size                                 Size of the supplied \a pstates array in bytes

 @return
         - \ref NVML_SUCCESS                 if \a pstates array has been retrieved
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if the the container supplied was not large enough to
                                             hold the resulting list
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a pstates is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support performance state readings
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetSupportedPerformanceStates(
        device: cuda_types::nvml::nvmlDevice_t,
        pstates: *mut cuda_types::nvml::nvmlPstates_t,
        size: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the GPCCLK min max VF offset value.
 @param[in]   device                         The identifier of the target device
 @param[out]  minOffset                      The retrieved GPCCLK VF min offset value
 @param[out]  maxOffset                      The retrieved GPCCLK VF max offset value

 @return
         - \ref NVML_SUCCESS                 if \a offset has been successfully queried
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a offset is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetGpcClkMinMaxVfOffset(
        device: cuda_types::nvml::nvmlDevice_t,
        minOffset: *mut ::core::ffi::c_int,
        maxOffset: *mut ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the MemClk (Memory Clock) min max VF offset value.
 @param[in]   device                         The identifier of the target device
 @param[out]  minOffset                      The retrieved MemClk VF min offset value
 @param[out]  maxOffset                      The retrieved MemClk VF max offset value

 @return
         - \ref NVML_SUCCESS                 if \a offset has been successfully queried
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a offset is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMemClkMinMaxVfOffset(
        device: cuda_types::nvml::nvmlDevice_t,
        minOffset: *mut ::core::ffi::c_int,
        maxOffset: *mut ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve min, max and current clock offset of some clock domain for a given PState

 For Maxwell &tm; or newer fully supported devices.

 Note: \ref nvmlDeviceGetGpcClkVfOffset, \ref nvmlDeviceGetMemClkVfOffset, \ref nvmlDeviceGetGpcClkMinMaxVfOffset and
       \ref nvmlDeviceGetMemClkMinMaxVfOffset will be deprecated in a future release.
Use \ref nvmlDeviceGetClockOffsets instead.

 @param device                               The identifier of the target device
 @param info                                 Structure specifying the clock type (input) and the pstate (input)
                                             retrieved clock offset value (output), min clock offset (output)
                                             and max clock offset (output)

 @return
         - \ref NVML_SUCCESS                         If everything worked
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device, \a type or \a pstate are invalid or both
                                                             \a minClockOffsetMHz and \a maxClockOffsetMHz are NULL
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_NOT_SUPPORTED             If the device does not support this feature*/
    fn nvmlDeviceGetClockOffsets(
        device: cuda_types::nvml::nvmlDevice_t,
        info: *mut cuda_types::nvml::nvmlClockOffset_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Control current clock offset of some clock domain for a given PState

 For Maxwell &tm; or newer fully supported devices.

 Requires privileged user.

 @param device                               The identifier of the target device
 @param info                                 Structure specifying the clock type (input), the pstate (input)
                                             and clock offset value (input)

 @return
         - \ref NVML_SUCCESS                         If everything worked
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_NO_PERMISSION             If the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device, \a type or \a pstate are invalid or both
                                                             \a clockOffsetMHz is out of allowed range.
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_NOT_SUPPORTED             If the device does not support this feature*/
    fn nvmlDeviceSetClockOffsets(
        device: cuda_types::nvml::nvmlDevice_t,
        info: *mut cuda_types::nvml::nvmlClockOffset_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves a performance mode string with all the
 performance modes defined for this device along with their associated
 GPU Clock and Memory Clock values.
 Not all tokens will be reported on all GPUs, and additional tokens
 may be added in the future.
 For backwards compatibility we still provide nvclock and memclock;
 those are the same as nvclockmin and memclockmin.

 Note: These clock values take into account the offset
 set by clients through /ref nvmlDeviceSetClockOffsets.

 Maximum available Pstate (P15) shows the minimum performance level (0) and vice versa.

 Each performance modes are returned as a comma-separated list of
 "token=value" pairs.  Each set of performance mode tokens are separated
 by a ";".  Valid tokens:

    Token                    Value
   "perf"                    unsigned int   - the Performance level
   "nvclock"                 unsigned int   - the GPU clocks (in MHz) for the perf level
   "nvclockmin"              unsigned int   - the GPU clocks min (in MHz) for the perf level
   "nvclockmax"              unsigned int   - the GPU clocks max (in MHz) for the perf level
   "nvclockeditable"         unsigned int   - if the GPU clock domain is editable for the perf level
   "memclock"                unsigned int   - the memory clocks (in MHz) for the perf level
   "memclockmin"             unsigned int   - the memory clocks min (in MHz) for the perf level
   "memclockmax"             unsigned int   - the memory clocks max (in MHz) for the perf level
   "memclockeditable"        unsigned int   - if the memory clock domain is editable for the perf level
   "memtransferrate"         unsigned int   - the memory transfer rate (in MHz) for the perf level
   "memtransferratemin"      unsigned int   - the memory transfer rate min (in MHz) for the perf level
   "memtransferratemax"      unsigned int   - the memory transfer rate max (in MHz) for the perf level
   "memtransferrateeditable" unsigned int   - if the memory transfer rate is editable for the perf level

 Example:

 perf=0, nvclock=324, nvclockmin=324, nvclockmax=324, nvclockeditable=0,
 memclock=324, memclockmin=324, memclockmax=324, memclockeditable=0,
 memtransferrate=648, memtransferratemin=648, memtransferratemax=648,
 memtransferrateeditable=0 ;
 perf=1, nvclock=324, nvclockmin=324, nvclockmax=640, nvclockeditable=0,
 memclock=810, memclockmin=810, memclockmax=810, memclockeditable=0,
 memtransferrate=1620, memtransferrate=1620, memtransferrate=1620,
 memtransferrateeditable=0 ;


 @param device                               The identifier of the target device
 @param perfModes                            Reference in which to return the performance level string

 @return
         - \ref NVML_SUCCESS                 if \a perfModes has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a name is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPerformanceModes(
        device: cuda_types::nvml::nvmlDevice_t,
        perfModes: *mut cuda_types::nvml::nvmlDevicePerfModes_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves a string with the associated current GPU Clock and Memory Clock values.

 Not all tokens will be reported on all GPUs, and additional tokens
 may be added in the future.

 Note: These clock values take into account the offset
 set by clients through /ref nvmlDeviceSetClockOffsets.

 Clock values are returned as a comma-separated list of
 "token=value" pairs.
 Valid tokens:

    Token                    Value
   "perf"                    unsigned int   - the Performance level
   "nvclock"                 unsigned int   - the GPU clocks (in MHz) for the perf level
   "nvclockmin"              unsigned int   - the GPU clocks min (in MHz) for the perf level
   "nvclockmax"              unsigned int   - the GPU clocks max (in MHz) for the perf level
   "nvclockeditable"         unsigned int   - if the GPU clock domain is editable for the perf level
   "memclock"                unsigned int   - the memory clocks (in MHz) for the perf level
   "memclockmin"             unsigned int   - the memory clocks min (in MHz) for the perf level
   "memclockmax"             unsigned int   - the memory clocks max (in MHz) for the perf level
   "memclockeditable"        unsigned int   - if the memory clock domain is editable for the perf level
   "memtransferrate"         unsigned int   - the memory transfer rate (in MHz) for the perf level
   "memtransferratemin"      unsigned int   - the memory transfer rate min (in MHz) for the perf level
   "memtransferratemax"      unsigned int   - the memory transfer rate max (in MHz) for the perf level
   "memtransferrateeditable" unsigned int   - if the memory transfer rate is editable for the perf level

 Example:

 nvclock=324, nvclockmin=324, nvclockmax=324, nvclockeditable=0,
 memclock=324, memclockmin=324, memclockmax=324, memclockeditable=0,
 memtransferrate=648, memtransferratemin=648, memtransferratemax=648,
 memtransferrateeditable=0 ;


 @param device                               The identifier of the target device
 @param currentClockFreqs                    Reference in which to return the performance level string

 @return
         - \ref NVML_SUCCESS                 if \a currentClockFreqs has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a name is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetCurrentClockFreqs(
        device: cuda_types::nvml::nvmlDevice_t,
        currentClockFreqs: *mut cuda_types::nvml::nvmlDeviceCurrentClockFreqs_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** This API has been deprecated.

 Retrieves the power management mode associated with this device.

 For products from the Fermi family.
     - Requires \a NVML_INFOROM_POWER version 3.0 or higher.

 For from the Kepler or newer families.
     - Does not require \a NVML_INFOROM_POWER object.

 This flag indicates whether any power management algorithm is currently active on the device. An
 enabled state does not necessarily mean the device is being actively throttled -- only that
 that the driver will do so if the appropriate conditions are met.

 See \ref nvmlEnableState_t for details on allowed modes.

 @param device                               The identifier of the target device
 @param mode                                 Reference in which to return the current power management mode

 @return
         - \ref NVML_SUCCESS                 if \a mode has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a mode is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPowerManagementMode(
        device: cuda_types::nvml::nvmlDevice_t,
        mode: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the power management limit associated with this device.

 For Fermi &tm; or newer fully supported devices.

 The power limit defines the upper boundary for the card's power draw. If
 the card's total power draw reaches this limit the power management algorithm kicks in.

 This reading is only available if power management mode is supported.
 See \ref nvmlDeviceGetPowerManagementMode.

 @param device                               The identifier of the target device
 @param limit                                Reference in which to return the power management limit in milliwatts

 @return
         - \ref NVML_SUCCESS                 if \a limit has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a limit is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPowerManagementLimit(
        device: cuda_types::nvml::nvmlDevice_t,
        limit: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves information about possible values of power management limits on this device.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param minLimit                             Reference in which to return the minimum power management limit in milliwatts
 @param maxLimit                             Reference in which to return the maximum power management limit in milliwatts

 @return
         - \ref NVML_SUCCESS                 if \a minLimit and \a maxLimit have been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a minLimit or \a maxLimit is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceSetPowerManagementLimit*/
    fn nvmlDeviceGetPowerManagementLimitConstraints(
        device: cuda_types::nvml::nvmlDevice_t,
        minLimit: *mut ::core::ffi::c_uint,
        maxLimit: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves default power management limit on this device, in milliwatts.
 Default power management limit is a power management limit that the device boots with.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param defaultLimit                         Reference in which to return the default power management limit in milliwatts

 @return
         - \ref NVML_SUCCESS                 if \a defaultLimit has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a defaultLimit is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPowerManagementDefaultLimit(
        device: cuda_types::nvml::nvmlDevice_t,
        defaultLimit: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves power usage for this GPU in milliwatts and its associated circuitry (e.g. memory)

 For Fermi &tm; or newer fully supported devices.

 On Fermi and Kepler GPUs the reading is accurate to within +/- 5% of current power draw. On Ampere
 (except GA100) or newer GPUs, the API returns power averaged over 1 sec interval. On GA100 and
 older architectures, instantaneous power is returned.

 See \ref NVML_FI_DEV_POWER_AVERAGE and \ref NVML_FI_DEV_POWER_INSTANT to query specific power
 values.

 It is only available if power management mode is supported. See \ref nvmlDeviceGetPowerManagementMode.

 @param device                               The identifier of the target device
 @param power                                Reference in which to return the power usage information

 @return
         - \ref NVML_SUCCESS                 if \a power has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a power is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support power readings
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPowerUsage(
        device: cuda_types::nvml::nvmlDevice_t,
        power: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves total energy consumption for this GPU in millijoules (mJ) since the driver was last reloaded

 For Volta &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param energy                               Reference in which to return the energy consumption information

 @return
         - \ref NVML_SUCCESS                 if \a energy has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a energy is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support energy readings
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetTotalEnergyConsumption(
        device: cuda_types::nvml::nvmlDevice_t,
        energy: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the effective power limit that the driver enforces after taking into account all limiters

 Note: This can be different from the \ref nvmlDeviceGetPowerManagementLimit if other limits are set elsewhere
 This includes the out of band power limit interface

 For Kepler &tm; or newer fully supported devices.

 @param device                           The device to communicate with
 @param limit                            Reference in which to return the power management limit in milliwatts

 @return
         - \ref NVML_SUCCESS                 if \a limit has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a limit is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetEnforcedPowerLimit(
        device: cuda_types::nvml::nvmlDevice_t,
        limit: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current GOM and pending GOM (the one that GPU will switch to after reboot).

 For GK110 M-class and X-class Tesla &tm; products from the Kepler family.
 Modes \ref NVML_GOM_LOW_DP and \ref NVML_GOM_ALL_ON are supported on fully supported GeForce products.
 Not supported on Quadro &reg; and Tesla &tm; C-class products.

 @param device                               The identifier of the target device
 @param current                              Reference in which to return the current GOM
 @param pending                              Reference in which to return the pending GOM

 @return
         - \ref NVML_SUCCESS                 if \a mode has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a current or \a pending is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlGpuOperationMode_t
 @see nvmlDeviceSetGpuOperationMode*/
    fn nvmlDeviceGetGpuOperationMode(
        device: cuda_types::nvml::nvmlDevice_t,
        current: *mut cuda_types::nvml::nvmlGpuOperationMode_t,
        pending: *mut cuda_types::nvml::nvmlGpuOperationMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the amount of used, free, reserved and total memory available on the device, in bytes.
 The reserved amount is supported on version 2 only.

 For all products.

 Enabling ECC reduces the amount of total available memory, due to the extra required parity bits.
 Under WDDM most device memory is allocated and managed on startup by Windows.

 Under Linux and Windows TCC, the reported amount of used memory is equal to the sum of memory allocated
 by all active channels on the device.

 See \ref nvmlMemory_v2_t for details on available memory info.

 @note In MIG mode, if device handle is provided, the API returns aggregate
       information, only if the caller has appropriate privileges. Per-instance
       information can be queried by using specific MIG device handles.

 @note nvmlDeviceGetMemoryInfo_v2 adds additional memory information.

 @note On systems where GPUs are NUMA nodes, the accuracy of FB memory utilization
       provided by this API depends on the memory accounting of the operating system.
       This is because FB memory is managed by the operating system instead of the NVIDIA GPU driver.
       Typically, pages allocated from FB memory are not released even after
       the process terminates to enhance performance. In scenarios where
       the operating system is under memory pressure, it may resort to utilizing FB memory.
       Such actions can result in discrepancies in the accuracy of memory reporting.

 @param device                               The identifier of the target device
 @param memory                               Reference in which to return the memory information

 @return
         - \ref NVML_SUCCESS                 if \a memory has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a memory is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMemoryInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        memory: *mut cuda_types::nvml::nvmlMemory_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /// nvmlDeviceGetMemoryInfo_v2 accounts separately for reserved memory and includes it in the used memory amount.
    fn nvmlDeviceGetMemoryInfo_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        memory: *mut cuda_types::nvml::nvmlMemory_v2_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current compute mode for the device.

 For all products.

 See \ref nvmlComputeMode_t for details on allowed compute modes.

 @param device                               The identifier of the target device
 @param mode                                 Reference in which to return the current compute mode

 @return
         - \ref NVML_SUCCESS                 if \a mode has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a mode is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceSetComputeMode()*/
    fn nvmlDeviceGetComputeMode(
        device: cuda_types::nvml::nvmlDevice_t,
        mode: *mut cuda_types::nvml::nvmlComputeMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the CUDA compute capability of the device.

 For all products.

 Returns the major and minor compute capability version numbers of the
 device.  The major and minor versions are equivalent to the
 CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR and
 CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR attributes that would be
 returned by CUDA's cuDeviceGetAttribute().

 @param device                               The identifier of the target device
 @param major                                Reference in which to return the major CUDA compute capability
 @param minor                                Reference in which to return the minor CUDA compute capability

 @return
         - \ref NVML_SUCCESS                 if \a major and \a minor have been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a major or \a minor are NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetCudaComputeCapability(
        device: cuda_types::nvml::nvmlDevice_t,
        major: *mut ::core::ffi::c_int,
        minor: *mut ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current and pending DRAM Encryption modes for the device.

 %BLACKWELL_OR_NEWER%
 Only applicable to devices that support DRAM Encryption
 Requires \a NVML_INFOROM_DEN version 1.0 or higher.

 Changing DRAM Encryption modes requires a reboot. The "pending" DRAM Encryption mode refers to the target mode following
 the next reboot.

 See \ref nvmlEnableState_t for details on allowed modes.

 @param device                               The identifier of the target device
 @param current                              Reference in which to return the current DRAM Encryption mode
 @param pending                              Reference in which to return the pending DRAM Encryption mode

 @return
         - \ref NVML_SUCCESS                         if \a current and \a pending have been set
         - \ref NVML_ERROR_UNINITIALIZED             if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          if \a device is invalid or either \a current or \a pending is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED             if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST               if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH if the argument version is not supported
         - \ref NVML_ERROR_UNKNOWN                   on any unexpected error

 @see nvmlDeviceSetDramEncryptionMode()*/
    fn nvmlDeviceGetDramEncryptionMode(
        device: cuda_types::nvml::nvmlDevice_t,
        current: *mut cuda_types::nvml::nvmlDramEncryptionInfo_t,
        pending: *mut cuda_types::nvml::nvmlDramEncryptionInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the DRAM Encryption mode for the device.

 For Kepler &tm; or newer fully supported devices.
 Only applicable to devices that support DRAM Encryption.
 Requires \a NVML_INFOROM_DEN version 1.0 or higher.
 Requires root/admin permissions.

 The DRAM Encryption mode determines whether the GPU enables its DRAM Encryption support.

 This operation takes effect after the next reboot.

 See \ref nvmlEnableState_t for details on available modes.

 @param device                               The identifier of the target device
 @param dramEncryption                       The target DRAM Encryption mode

 @return
         - \ref NVML_SUCCESS                         if the DRAM Encryption mode was set
         - \ref NVML_ERROR_UNINITIALIZED             if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          if \a device is invalid or \a DRAM Encryption is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED             if the device does not support this feature
         - \ref NVML_ERROR_NO_PERMISSION             if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_GPU_IS_LOST               if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH if the argument version is not supported
         - \ref NVML_ERROR_UNKNOWN                   on any unexpected error

 @see nvmlDeviceGetDramEncryptionMode()*/
    fn nvmlDeviceSetDramEncryptionMode(
        device: cuda_types::nvml::nvmlDevice_t,
        dramEncryption: *const cuda_types::nvml::nvmlDramEncryptionInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current and pending ECC modes for the device.

 For Fermi &tm; or newer fully supported devices.
 Only applicable to devices with ECC.
 Requires \a NVML_INFOROM_ECC version 1.0 or higher.

 Changing ECC modes requires a reboot. The "pending" ECC mode refers to the target mode following
 the next reboot.

 See \ref nvmlEnableState_t for details on allowed modes.

 @param device                               The identifier of the target device
 @param current                              Reference in which to return the current ECC mode
 @param pending                              Reference in which to return the pending ECC mode

 @return
         - \ref NVML_SUCCESS                 if \a current and \a pending have been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or either \a current or \a pending is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceSetEccMode()*/
    fn nvmlDeviceGetEccMode(
        device: cuda_types::nvml::nvmlDevice_t,
        current: *mut cuda_types::nvml::nvmlEnableState_t,
        pending: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the default ECC modes for the device.

 For Fermi &tm; or newer fully supported devices.
 Only applicable to devices with ECC.
 Requires \a NVML_INFOROM_ECC version 1.0 or higher.

 See \ref nvmlEnableState_t for details on allowed modes.

 @param device                               The identifier of the target device
 @param defaultMode                          Reference in which to return the default ECC mode

 @return
         - \ref NVML_SUCCESS                 if \a current and \a pending have been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a default is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceSetEccMode()*/
    fn nvmlDeviceGetDefaultEccMode(
        device: cuda_types::nvml::nvmlDevice_t,
        defaultMode: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the device boardId from 0-N.
 Devices with the same boardId indicate GPUs connected to the same PLX.  Use in conjunction with
  \ref nvmlDeviceGetMultiGpuBoard() to decide if they are on the same board as well.
  The boardId returned is a unique ID for the current configuration.  Uniqueness and ordering across
  reboots and system configurations is not guaranteed (i.e. if a Tesla K40c returns 0x100 and
  the two GPUs on a Tesla K10 in the same system returns 0x200 it is not guaranteed they will
  always return those values but they will always be different from each other).


 For Fermi &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param boardId                              Reference in which to return the device's board ID

 @return
         - \ref NVML_SUCCESS                 if \a boardId has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a boardId is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetBoardId(
        device: cuda_types::nvml::nvmlDevice_t,
        boardId: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves whether the device is on a Multi-GPU Board
 Devices that are on multi-GPU boards will set \a multiGpuBool to a non-zero value.

 For Fermi &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param multiGpuBool                         Reference in which to return a zero or non-zero value
                                                 to indicate whether the device is on a multi GPU board

 @return
         - \ref NVML_SUCCESS                 if \a multiGpuBool has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a multiGpuBool is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMultiGpuBoard(
        device: cuda_types::nvml::nvmlDevice_t,
        multiGpuBool: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the total ECC error counts for the device.

 For Fermi &tm; or newer fully supported devices.
 Only applicable to devices with ECC.
 Requires \a NVML_INFOROM_ECC version 1.0 or higher.
 Requires ECC Mode to be enabled.

 The total error count is the sum of errors across each of the separate memory systems, i.e. the total set of
 errors across the entire device.

 See \ref nvmlMemoryErrorType_t for a description of available error types.\n
 See \ref nvmlEccCounterType_t for a description of available counter types.

 @param device                               The identifier of the target device
 @param errorType                            Flag that specifies the type of the errors.
 @param counterType                          Flag that specifies the counter-type of the errors.
 @param eccCounts                            Reference in which to return the specified ECC errors

 @return
         - \ref NVML_SUCCESS                 if \a eccCounts has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a errorType or \a counterType is invalid, or \a eccCounts is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceClearEccErrorCounts()*/
    fn nvmlDeviceGetTotalEccErrors(
        device: cuda_types::nvml::nvmlDevice_t,
        errorType: cuda_types::nvml::nvmlMemoryErrorType_t,
        counterType: cuda_types::nvml::nvmlEccCounterType_t,
        eccCounts: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the detailed ECC error counts for the device.

 @deprecated   This API supports only a fixed set of ECC error locations
               On different GPU architectures different locations are supported
               See \ref nvmlDeviceGetMemoryErrorCounter

 For Fermi &tm; or newer fully supported devices.
 Only applicable to devices with ECC.
 Requires \a NVML_INFOROM_ECC version 2.0 or higher to report aggregate location-based ECC counts.
 Requires \a NVML_INFOROM_ECC version 1.0 or higher to report all other ECC counts.
 Requires ECC Mode to be enabled.

 Detailed errors provide separate ECC counts for specific parts of the memory system.

 Reports zero for unsupported ECC error counters when a subset of ECC error counters are supported.

 See \ref nvmlMemoryErrorType_t for a description of available bit types.\n
 See \ref nvmlEccCounterType_t for a description of available counter types.\n
 See \ref nvmlEccErrorCounts_t for a description of provided detailed ECC counts.

 @param device                               The identifier of the target device
 @param errorType                            Flag that specifies the type of the errors.
 @param counterType                          Flag that specifies the counter-type of the errors.
 @param eccCounts                            Reference in which to return the specified ECC errors

 @return
         - \ref NVML_SUCCESS                 if \a eccCounts has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a errorType or \a counterType is invalid, or \a eccCounts is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceClearEccErrorCounts()*/
    fn nvmlDeviceGetDetailedEccErrors(
        device: cuda_types::nvml::nvmlDevice_t,
        errorType: cuda_types::nvml::nvmlMemoryErrorType_t,
        counterType: cuda_types::nvml::nvmlEccCounterType_t,
        eccCounts: *mut cuda_types::nvml::nvmlEccErrorCounts_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the requested memory error counter for the device.

 For Fermi &tm; or newer fully supported devices.
 Requires \a NVML_INFOROM_ECC version 2.0 or higher to report aggregate location-based memory error counts.
 Requires \a NVML_INFOROM_ECC version 1.0 or higher to report all other memory error counts.

 Only applicable to devices with ECC.

 Requires ECC Mode to be enabled.

 @note On MIG-enabled GPUs, per instance information can be queried using specific
       MIG device handles. Per instance information is currently only supported for
       non-DRAM uncorrectable volatile errors. Querying volatile errors using device
       handles is currently not supported.

 See \ref nvmlMemoryErrorType_t for a description of available memory error types.\n
 See \ref nvmlEccCounterType_t for a description of available counter types.\n
 See \ref nvmlMemoryLocation_t for a description of available counter locations.\n

 @param device                               The identifier of the target device
 @param errorType                            Flag that specifies the type of error.
 @param counterType                          Flag that specifies the counter-type of the errors.
 @param locationType                         Specifies the location of the counter.
 @param count                                Reference in which to return the ECC counter

 @return
         - \ref NVML_SUCCESS                 if \a count has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a bitTyp,e \a counterType or \a locationType is
                                             invalid, or \a count is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support ECC error reporting in the specified memory
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMemoryErrorCounter(
        device: cuda_types::nvml::nvmlDevice_t,
        errorType: cuda_types::nvml::nvmlMemoryErrorType_t,
        counterType: cuda_types::nvml::nvmlEccCounterType_t,
        locationType: cuda_types::nvml::nvmlMemoryLocation_t,
        count: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current utilization rates for the device's major subsystems.

 For Fermi &tm; or newer fully supported devices.

 See \ref nvmlUtilization_t for details on available utilization rates.

 \note During driver initialization when ECC is enabled one can see high GPU and Memory Utilization readings.
       This is caused by ECC Memory Scrubbing mechanism that is performed during driver initialization.

 @note On MIG-enabled GPUs, querying device utilization rates is not currently supported.

 @param device                               The identifier of the target device
 @param utilization                          Reference in which to return the utilization information

 @return
         - \ref NVML_SUCCESS                 if \a utilization has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a utilization is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetUtilizationRates(
        device: cuda_types::nvml::nvmlDevice_t,
        utilization: *mut cuda_types::nvml::nvmlUtilization_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current utilization and sampling size in microseconds for the Encoder

 For Kepler &tm; or newer fully supported devices.

 @note On MIG-enabled GPUs, querying encoder utilization is not currently supported.

 @param device                               The identifier of the target device
 @param utilization                          Reference to an unsigned int for encoder utilization info
 @param samplingPeriodUs                     Reference to an unsigned int for the sampling period in US

 @return
         - \ref NVML_SUCCESS                 if \a utilization has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a utilization is NULL, or \a samplingPeriodUs is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetEncoderUtilization(
        device: cuda_types::nvml::nvmlDevice_t,
        utilization: *mut ::core::ffi::c_uint,
        samplingPeriodUs: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current capacity of the device's encoder, as a percentage of maximum encoder capacity with valid values in the range 0-100.

 For Maxwell &tm; or newer fully supported devices.

 @param device                            The identifier of the target device
 @param encoderQueryType                  Type of encoder to query
 @param encoderCapacity                   Reference to an unsigned int for the encoder capacity

 @return
         - \ref NVML_SUCCESS                  if \a encoderCapacity is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a encoderCapacity is NULL, or \a device or \a encoderQueryType
                                              are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED      if device does not support the encoder specified in \a encodeQueryType
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetEncoderCapacity(
        device: cuda_types::nvml::nvmlDevice_t,
        encoderQueryType: cuda_types::nvml::nvmlEncoderType_t,
        encoderCapacity: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current encoder statistics for a given device.

 For Maxwell &tm; or newer fully supported devices.

 @param device                            The identifier of the target device
 @param sessionCount                      Reference to an unsigned int for count of active encoder sessions
 @param averageFps                        Reference to an unsigned int for trailing average FPS of all active sessions
 @param averageLatency                    Reference to an unsigned int for encode latency in microseconds

 @return
         - \ref NVML_SUCCESS                  if \a sessionCount, \a averageFps and \a averageLatency is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a sessionCount, or \a device or \a averageFps,
                                              or \a averageLatency is NULL
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetEncoderStats(
        device: cuda_types::nvml::nvmlDevice_t,
        sessionCount: *mut ::core::ffi::c_uint,
        averageFps: *mut ::core::ffi::c_uint,
        averageLatency: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves information about active encoder sessions on a target device.

 An array of active encoder sessions is returned in the caller-supplied buffer pointed at by \a sessionInfos. The
 array element count is passed in \a sessionCount, and \a sessionCount is used to return the number of sessions
 written to the buffer.

 If the supplied buffer is not large enough to accommodate the active session array, the function returns
 NVML_ERROR_INSUFFICIENT_SIZE, with the element count of nvmlEncoderSessionInfo_t array required in \a sessionCount.
 To query the number of active encoder sessions, call this function with *sessionCount = 0.  The code will return
 NVML_SUCCESS with number of active encoder sessions updated in *sessionCount.

 For Maxwell &tm; or newer fully supported devices.

 @param device                            The identifier of the target device
 @param sessionCount                      Reference to caller supplied array size, and returns the number of sessions.
 @param sessionInfos                      Reference in which to return the session information

 @return
         - \ref NVML_SUCCESS                  if \a sessionInfos is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INSUFFICIENT_SIZE  if \a sessionCount is too small, array element count is returned in \a sessionCount
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a sessionCount is NULL.
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_SUPPORTED      if this query is not supported by \a device
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetEncoderSessions(
        device: cuda_types::nvml::nvmlDevice_t,
        sessionCount: *mut ::core::ffi::c_uint,
        sessionInfos: *mut cuda_types::nvml::nvmlEncoderSessionInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current utilization and sampling size in microseconds for the Decoder

 For Kepler &tm; or newer fully supported devices.

 @note On MIG-enabled GPUs, querying decoder utilization is not currently supported.

 @param device                               The identifier of the target device
 @param utilization                          Reference to an unsigned int for decoder utilization info
 @param samplingPeriodUs                     Reference to an unsigned int for the sampling period in US

 @return
         - \ref NVML_SUCCESS                 if \a utilization has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a utilization is NULL, or \a samplingPeriodUs is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetDecoderUtilization(
        device: cuda_types::nvml::nvmlDevice_t,
        utilization: *mut ::core::ffi::c_uint,
        samplingPeriodUs: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current utilization and sampling size in microseconds for the JPG

 %TURING_OR_NEWER%

 @note On MIG-enabled GPUs, querying decoder utilization is not currently supported.

 @param device                               The identifier of the target device
 @param utilization                          Reference to an unsigned int for jpg utilization info
 @param samplingPeriodUs                     Reference to an unsigned int for the sampling period in US

 @return
         - \ref NVML_SUCCESS                 if \a utilization has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a utilization is NULL, or \a samplingPeriodUs is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetJpgUtilization(
        device: cuda_types::nvml::nvmlDevice_t,
        utilization: *mut ::core::ffi::c_uint,
        samplingPeriodUs: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current utilization and sampling size in microseconds for the OFA (Optical Flow Accelerator)

 %TURING_OR_NEWER%

 @note On MIG-enabled GPUs, querying decoder utilization is not currently supported.

 @param device                               The identifier of the target device
 @param utilization                          Reference to an unsigned int for ofa utilization info
 @param samplingPeriodUs                     Reference to an unsigned int for the sampling period in US

 @return
         - \ref NVML_SUCCESS                 if \a utilization has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a utilization is NULL, or \a samplingPeriodUs is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetOfaUtilization(
        device: cuda_types::nvml::nvmlDevice_t,
        utilization: *mut ::core::ffi::c_uint,
        samplingPeriodUs: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the active frame buffer capture sessions statistics for a given device.

 For Maxwell &tm; or newer fully supported devices.

 @param device                            The identifier of the target device
 @param fbcStats                          Reference to nvmlFBCStats_t structure containing NvFBC stats

 @return
         - \ref NVML_SUCCESS                  if \a fbcStats is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a fbcStats is NULL
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetFBCStats(
        device: cuda_types::nvml::nvmlDevice_t,
        fbcStats: *mut cuda_types::nvml::nvmlFBCStats_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves information about active frame buffer capture sessions on a target device.

 An array of active FBC sessions is returned in the caller-supplied buffer pointed at by \a sessionInfo. The
 array element count is passed in \a sessionCount, and \a sessionCount is used to return the number of sessions
 written to the buffer.

 If the supplied buffer is not large enough to accommodate the active session array, the function returns
 NVML_ERROR_INSUFFICIENT_SIZE, with the element count of nvmlFBCSessionInfo_t array required in \a sessionCount.
 To query the number of active FBC sessions, call this function with *sessionCount = 0.  The code will return
 NVML_SUCCESS with number of active FBC sessions updated in *sessionCount.

 For Maxwell &tm; or newer fully supported devices.

 @note hResolution, vResolution, averageFPS and averageLatency data for a FBC session returned in \a sessionInfo may
       be zero if there are no new frames captured since the session started.

 @param device                            The identifier of the target device
 @param sessionCount                      Reference to caller supplied array size, and returns the number of sessions.
 @param sessionInfo                       Reference in which to return the session information

 @return
         - \ref NVML_SUCCESS                  if \a sessionInfo is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INSUFFICIENT_SIZE  if \a sessionCount is too small, array element count is returned in \a sessionCount
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a sessionCount is NULL.
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetFBCSessions(
        device: cuda_types::nvml::nvmlDevice_t,
        sessionCount: *mut ::core::ffi::c_uint,
        sessionInfo: *mut cuda_types::nvml::nvmlFBCSessionInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current and pending driver model for the device.

 For Kepler &tm; or newer fully supported devices.
 For windows only.

 On Windows platforms the device driver can run in either WDDM, MCDM or WDM (TCC) modes. If a display is attached
 to the device it must run in WDDM mode. MCDM mode is preferred if a display is not attached. TCC mode is deprecated.

 See \ref nvmlDriverModel_t for details on available driver models.

 @param device                               The identifier of the target device
 @param current                              Reference in which to return the current driver model
 @param pending                              Reference in which to return the pending driver model

 @return
         - \ref NVML_SUCCESS                 if either \a current and/or \a pending have been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or both \a current and \a pending are NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the platform is not windows
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceSetDriverModel_v2()*/
    fn nvmlDeviceGetDriverModel_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        current: *mut cuda_types::nvml::nvmlDriverModel_t,
        pending: *mut cuda_types::nvml::nvmlDriverModel_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get VBIOS version of the device.

 For all products.

 The VBIOS version may change from time to time. It will not exceed 32 characters in length
 (including the NULL terminator).  See \ref nvmlConstants::NVML_DEVICE_VBIOS_VERSION_BUFFER_SIZE.

 @param device                               The identifier of the target device
 @param version                              Reference to which to return the VBIOS version
 @param length                               The maximum allowed length of the string returned in \a version

 @return
         - \ref NVML_SUCCESS                 if \a version has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a version is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetVbiosVersion(
        device: cuda_types::nvml::nvmlDevice_t,
        version: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Bridge Chip Information for all the bridge chips on the board.

 For all fully supported products.
 Only applicable to multi-GPU products.

 @param device                                The identifier of the target device
 @param bridgeHierarchy                       Reference to the returned bridge chip Hierarchy

 @return
         - \ref NVML_SUCCESS                 if bridge chip exists
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a bridgeInfo is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if bridge chip not supported on the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error
*/
    fn nvmlDeviceGetBridgeChipInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        bridgeHierarchy: *mut cuda_types::nvml::nvmlBridgeChipHierarchy_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get information about processes with a compute context on a device

 For Fermi &tm; or newer fully supported devices.

 This function returns information only about compute running processes (e.g. CUDA application which have
 active context). Any graphics applications (e.g. using OpenGL, DirectX) won't be listed by this function.

 To query the current number of running compute processes, call this function with *infoCount = 0. The
 return code will be NVML_ERROR_INSUFFICIENT_SIZE, or NVML_SUCCESS if none are running. For this call
 \a infos is allowed to be NULL.

 The usedGpuMemory field returned is all of the memory used by the application.

 Keep in mind that information returned by this call is dynamic and the number of elements might change in
 time. Allocate more space for \a infos table in case new compute processes are spawned.

 @note In MIG mode, if device handle is provided, the API returns aggregate information, only if
       the caller has appropriate privileges. Per-instance information can be queried by using
       specific MIG device handles.
       Querying per-instance information using MIG device handles is not supported if the device is in vGPU Host virtualization mode.

 @param device                               The device handle or MIG device handle
 @param infoCount                            Reference in which to provide the \a infos array size, and
                                             to return the number of returned elements
 @param infos                                Reference in which to return the process information

 @return
         - \ref NVML_SUCCESS                 if \a infoCount and \a infos have been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a infoCount indicates that the \a infos array is too small
                                             \a infoCount will contain minimal amount of space necessary for
                                             the call to complete
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, either of \a infoCount or \a infos is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by \a device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see \ref nvmlSystemGetProcessName*/
    fn nvmlDeviceGetComputeRunningProcesses_v3(
        device: cuda_types::nvml::nvmlDevice_t,
        infoCount: *mut ::core::ffi::c_uint,
        infos: *mut cuda_types::nvml::nvmlProcessInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get information about processes with a graphics context on a device

 For Kepler &tm; or newer fully supported devices.

 This function returns information only about graphics based processes
 (eg. applications using OpenGL, DirectX)

 To query the current number of running graphics processes, call this function with *infoCount = 0. The
 return code will be NVML_ERROR_INSUFFICIENT_SIZE, or NVML_SUCCESS if none are running. For this call
 \a infos is allowed to be NULL.

 The usedGpuMemory field returned is all of the memory used by the application.

 Keep in mind that information returned by this call is dynamic and the number of elements might change in
 time. Allocate more space for \a infos table in case new graphics processes are spawned.

 @note In MIG mode, if device handle is provided, the API returns aggregate information, only if
       the caller has appropriate privileges. Per-instance information can be queried by using
       specific MIG device handles.
       Querying per-instance information using MIG device handles is not supported if the device is in vGPU Host virtualization mode.

 @param device                               The device handle or MIG device handle
 @param infoCount                            Reference in which to provide the \a infos array size, and
                                             to return the number of returned elements
 @param infos                                Reference in which to return the process information

 @return
         - \ref NVML_SUCCESS                 if \a infoCount and \a infos have been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a infoCount indicates that the \a infos array is too small
                                             \a infoCount will contain minimal amount of space necessary for
                                             the call to complete
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, either of \a infoCount or \a infos is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by \a device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see \ref nvmlSystemGetProcessName*/
    fn nvmlDeviceGetGraphicsRunningProcesses_v3(
        device: cuda_types::nvml::nvmlDevice_t,
        infoCount: *mut ::core::ffi::c_uint,
        infos: *mut cuda_types::nvml::nvmlProcessInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get information about processes with a Multi-Process Service (MPS) compute context on a device

 For Volta &tm; or newer fully supported devices.

 This function returns information only about compute running processes (e.g. CUDA application which have
 active context) utilizing MPS. Any graphics applications (e.g. using OpenGL, DirectX) won't be listed by
 this function.

 To query the current number of running compute processes, call this function with *infoCount = 0. The
 return code will be NVML_ERROR_INSUFFICIENT_SIZE, or NVML_SUCCESS if none are running. For this call
 \a infos is allowed to be NULL.

 The usedGpuMemory field returned is all of the memory used by the application.

 Keep in mind that information returned by this call is dynamic and the number of elements might change in
 time. Allocate more space for \a infos table in case new compute processes are spawned.

 @note In MIG mode, if device handle is provided, the API returns aggregate information, only if
       the caller has appropriate privileges. Per-instance information can be queried by using
       specific MIG device handles.
       Querying per-instance information using MIG device handles is not supported if the device is in vGPU Host virtualization mode.

 @param device                               The device handle or MIG device handle
 @param infoCount                            Reference in which to provide the \a infos array size, and
                                             to return the number of returned elements
 @param infos                                Reference in which to return the process information

 @return
         - \ref NVML_SUCCESS                 if \a infoCount and \a infos have been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a infoCount indicates that the \a infos array is too small
                                             \a infoCount will contain minimal amount of space necessary for
                                             the call to complete
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, either of \a infoCount or \a infos is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by \a device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see \ref nvmlSystemGetProcessName*/
    fn nvmlDeviceGetMPSComputeRunningProcesses_v3(
        device: cuda_types::nvml::nvmlDevice_t,
        infoCount: *mut ::core::ffi::c_uint,
        infos: *mut cuda_types::nvml::nvmlProcessInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get information about running processes on a device for input context

 For Hopper &tm; or newer fully supported devices.

 This function returns information only about running processes (e.g. CUDA application which have
 active context).

 To determine the size of the \a plist->procArray array to allocate, call the function with
 \a plist->numProcArrayEntries set to zero and \a plist->procArray set to NULL. The return
 code will be either NVML_ERROR_INSUFFICIENT_SIZE (if there are valid processes of type
 \a plist->mode to report on, in which case the \a plist->numProcArrayEntries field will
 indicate the required number of entries in the array) or NVML_SUCCESS (if no processes of type
 \a plist->mode exist).

 The usedGpuMemory field returned is all of the memory used by the application.
 The usedGpuCcProtectedMemory field returned is all of the protected memory used by the application.

 Keep in mind that information returned by this call is dynamic and the number of elements might change in
 time. Allocate more space for \a plist->procArray table in case new processes are spawned.

 @note In MIG mode, if device handle is provided, the API returns aggregate information, only if
       the caller has appropriate privileges. Per-instance information can be queried by using
       specific MIG device handles.
       Querying per-instance information using MIG device handles is not supported if the device is in
       vGPU Host virtualization mode.
       Protected memory usage is currently not available in MIG mode and in windows.

 @param device                               The device handle or MIG device handle
 @param plist                                Reference in which to process detail list
 \a plist->version                       The api version
 \a plist->mode                          The process mode
 \a plist->procArray                     Reference in which to return the process information
 \a plist->numProcArrayEntries           Proc array size of returned entries

 @return
         - \ref NVML_SUCCESS                 if \a plist->numprocArrayEntries and \a plist->procArray have been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a plist->numprocArrayEntries indicates that the \a plist->procArray is too small
                                             \a plist->numprocArrayEntries will contain minimal amount of space necessary for
                                             the call to complete
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a plist is NULL, \a plist->version is invalid,
                                             \a plist->mode is invalid,
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by \a device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error
*/
    fn nvmlDeviceGetRunningProcessDetailList(
        device: cuda_types::nvml::nvmlDevice_t,
        plist: *mut cuda_types::nvml::nvmlProcessDetailList_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Check if the GPU devices are on the same physical board.

 For all fully supported products.

 @param device1                               The first GPU device
 @param device2                               The second GPU device
 @param onSameBoard                           Reference in which to return the status.
                                              Non-zero indicates that the GPUs are on the same board.

 @return
         - \ref NVML_SUCCESS                 if \a onSameBoard has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a dev1 or \a dev2 are invalid or \a onSameBoard is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this check is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the either GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceOnSameBoard(
        device1: cuda_types::nvml::nvmlDevice_t,
        device2: cuda_types::nvml::nvmlDevice_t,
        onSameBoard: *mut ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the root/admin permissions on the target API. See \a nvmlRestrictedAPI_t for the list of supported APIs.
 If an API is restricted only root users can call that API. See \a nvmlDeviceSetAPIRestriction to change current permissions.

 For all fully supported products.

 @param device                               The identifier of the target device
 @param apiType                              Target API type for this operation
 @param isRestricted                         Reference in which to return the current restriction
                                             NVML_FEATURE_ENABLED indicates that the API is root-only
                                             NVML_FEATURE_DISABLED indicates that the API is accessible to all users

 @return
         - \ref NVML_SUCCESS                 if \a isRestricted has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a apiType incorrect or \a isRestricted is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device or the device does not support
                                                 the feature that is being queried (E.G. Enabling/disabling Auto Boosted clocks is
                                                 not supported by the device)
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlRestrictedAPI_t*/
    fn nvmlDeviceGetAPIRestriction(
        device: cuda_types::nvml::nvmlDevice_t,
        apiType: cuda_types::nvml::nvmlRestrictedAPI_t,
        isRestricted: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets recent samples for the GPU.

 For Kepler &tm; or newer fully supported devices.

 Based on type, this method can be used to fetch the power, utilization or clock samples maintained in the buffer by
 the driver.

 Power, Utilization and Clock samples are returned as type "unsigned int" for the union nvmlValue_t.

 To get the size of samples that user needs to allocate, the method is invoked with samples set to NULL.
 The returned samplesCount will provide the number of samples that can be queried. The user needs to
 allocate the buffer with size as samplesCount * sizeof(nvmlSample_t).

 lastSeenTimeStamp represents CPU timestamp in microseconds. Set it to 0 to fetch all the samples maintained by the
 underlying buffer. Set lastSeenTimeStamp to one of the timeStamps retrieved from the date of the previous query
 to get more recent samples.

 This method fetches the number of entries which can be accommodated in the provided samples array, and the
 reference samplesCount is updated to indicate how many samples were actually retrieved. The advantage of using this
 method for samples in contrast to polling via existing methods is to get get higher frequency data at lower polling cost.

 @note On MIG-enabled GPUs, querying the following sample types, NVML_GPU_UTILIZATION_SAMPLES, NVML_MEMORY_UTILIZATION_SAMPLES
       NVML_ENC_UTILIZATION_SAMPLES and NVML_DEC_UTILIZATION_SAMPLES, is not currently supported.

 @param device                        The identifier for the target device
 @param type                          Type of sampling event
 @param lastSeenTimeStamp             Return only samples with timestamp greater than lastSeenTimeStamp.
 @param sampleValType                 Output parameter to represent the type of sample value as described in nvmlSampleVal_t
 @param sampleCount                   Reference to provide the number of elements which can be queried in samples array
 @param samples                       Reference in which samples are returned

 @return
         - \ref NVML_SUCCESS                 if samples are successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a samplesCount is NULL or
                                             reference to \a sampleCount is 0 for non null \a samples
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_FOUND         if sample entries are not found
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetSamples(
        device: cuda_types::nvml::nvmlDevice_t,
        type_: cuda_types::nvml::nvmlSamplingType_t,
        lastSeenTimeStamp: ::core::ffi::c_ulonglong,
        sampleValType: *mut cuda_types::nvml::nvmlValueType_t,
        sampleCount: *mut ::core::ffi::c_uint,
        samples: *mut cuda_types::nvml::nvmlSample_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets Total, Available and Used size of BAR1 memory.

 BAR1 is used to map the FB (device memory) so that it can be directly accessed by the CPU or by 3rd party
 devices (peer-to-peer on the PCIE bus).

 @note In MIG mode, if device handle is provided, the API returns aggregate
       information, only if the caller has appropriate privileges. Per-instance
       information can be queried by using specific MIG device handles.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param bar1Memory                           Reference in which BAR1 memory
                                             information is returned.

 @return
         - \ref NVML_SUCCESS                 if BAR1 memory is successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a bar1Memory is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error
*/
    fn nvmlDeviceGetBAR1MemoryInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        bar1Memory: *mut cuda_types::nvml::nvmlBAR1Memory_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets the duration of time during which the device was throttled (lower than requested clocks) due to power
 or thermal constraints.

 The method is important to users who are tying to understand if their GPUs throttle at any point during their applications. The
 difference in violation times at two different reference times gives the indication of GPU throttling event.

 Violation for thermal capping is not supported at this time.

 For Kepler &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param perfPolicyType                       Represents Performance policy which can trigger GPU throttling
 @param violTime                             Reference to which violation time related information is returned


 @return
         - \ref NVML_SUCCESS                 if violation time is successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a perfPolicyType is invalid, or \a violTime is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
*/
    fn nvmlDeviceGetViolationStatus(
        device: cuda_types::nvml::nvmlDevice_t,
        perfPolicyType: cuda_types::nvml::nvmlPerfPolicyType_t,
        violTime: *mut cuda_types::nvml::nvmlViolationTime_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets the device's interrupt number

 @param device                               The identifier of the target device
 @param irqNum                               The interrupt number associated with the specified device

 @return
         - \ref NVML_SUCCESS                 if irq number is successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a irqNum is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
*/
    fn nvmlDeviceGetIrqNum(
        device: cuda_types::nvml::nvmlDevice_t,
        irqNum: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets the device's core count

 @param device                               The identifier of the target device
 @param numCores                             The number of cores for the specified device

 @return
         - \ref NVML_SUCCESS                 if GPU core count is successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a numCores is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
*/
    fn nvmlDeviceGetNumGpuCores(
        device: cuda_types::nvml::nvmlDevice_t,
        numCores: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets the devices power source

 @param device                               The identifier of the target device
 @param powerSource                          The power source of the device

 @return
         - \ref NVML_SUCCESS                 if the current power source was successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a powerSource is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
*/
    fn nvmlDeviceGetPowerSource(
        device: cuda_types::nvml::nvmlDevice_t,
        powerSource: *mut cuda_types::nvml::nvmlPowerSource_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets the device's memory bus width

 @param device                               The identifier of the target device
 @param busWidth                             The devices's memory bus width

 @return
         - \ref NVML_SUCCESS                 if the memory bus width is successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a busWidth is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
*/
    fn nvmlDeviceGetMemoryBusWidth(
        device: cuda_types::nvml::nvmlDevice_t,
        busWidth: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets the device's PCIE Max Link speed in MBPS

 @param device                               The identifier of the target device
 @param maxSpeed                             The devices's PCIE Max Link speed in MBPS

 @return
         - \ref NVML_SUCCESS                 if PCIe Max Link Speed is successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a maxSpeed is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
*/
    fn nvmlDeviceGetPcieLinkMaxSpeed(
        device: cuda_types::nvml::nvmlDevice_t,
        maxSpeed: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets the device's PCIe Link speed in Mbps

 @param device                               The identifier of the target device
 @param pcieSpeed                            The devices's PCIe Max Link speed in Mbps

 @return
         - \ref NVML_SUCCESS                 if \a pcieSpeed has been retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a pcieSpeed is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support PCIe speed getting
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetPcieSpeed(
        device: cuda_types::nvml::nvmlDevice_t,
        pcieSpeed: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Gets the device's Adaptive Clock status

 @param device                               The identifier of the target device
 @param adaptiveClockStatus                  The current adaptive clocking status, either
                                             NVML_ADAPTIVE_CLOCKING_INFO_STATUS_DISABLED
                                             or NVML_ADAPTIVE_CLOCKING_INFO_STATUS_ENABLED

 @return
         - \ref NVML_SUCCESS                 if the current adaptive clocking status is successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, or \a adaptiveClockStatus is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
*/
    fn nvmlDeviceGetAdaptiveClockInfoStatus(
        device: cuda_types::nvml::nvmlDevice_t,
        adaptiveClockStatus: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the type of the GPU Bus (PCIe, PCI, ...)

 @param device                               The identifier of the target device
 @param type                                 The PCI Bus type

 return
         - \ref NVML_SUCCESS                 if the bus \a type is successfully retreived
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a type is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetBusType(
        device: cuda_types::nvml::nvmlDevice_t,
        type_: *mut cuda_types::nvml::nvmlBusType_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Deprecated: Will be deprecated in a future release. Use \ref nvmlDeviceGetGpuFabricInfoV instead

 Get fabric information associated with the device.

 For Hopper &tm; or newer fully supported devices.

 On Hopper + NVSwitch systems, GPU is registered with the NVIDIA Fabric Manager
 Upon successful registration, the GPU is added to the NVLink fabric to enable
 peer-to-peer communication.
 This API reports the current state of the GPU in the NVLink fabric
 along with other useful information.


 @param device                               The identifier of the target device
 @param gpuFabricInfo                        Information about GPU fabric state

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't support gpu fabric*/
    fn nvmlDeviceGetGpuFabricInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        gpuFabricInfo: *mut cuda_types::nvml::nvmlGpuFabricInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Versioned wrapper around \ref nvmlDeviceGetGpuFabricInfo that accepts a versioned
 \ref nvmlGpuFabricInfo_v2_t or later output structure.

 @note The caller must set the \ref nvmlGpuFabricInfoV_t.version field to the
 appropriate version prior to calling this function. For example:
 \code
     nvmlGpuFabricInfoV_t fabricInfo =
         { .version = nvmlGpuFabricInfo_v2 };
     nvmlReturn_t result = nvmlDeviceGetGpuFabricInfoV(device,&fabricInfo);
 \endcode

 For Hopper &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param gpuFabricInfo                        Information about GPU fabric state

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't support gpu fabric*/
    fn nvmlDeviceGetGpuFabricInfoV(
        device: cuda_types::nvml::nvmlDevice_t,
        gpuFabricInfo: *mut cuda_types::nvml::nvmlGpuFabricInfoV_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Conf Computing System capabilities.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param capabilities                         System CC capabilities

 @return
         - \ref NVML_SUCCESS                 if \a capabilities were successfully queried
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a capabilities is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device*/
    fn nvmlSystemGetConfComputeCapabilities(
        capabilities: *mut cuda_types::nvml::nvmlConfComputeSystemCaps_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Conf Computing System State.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param state                                System CC State

 @return
         - \ref NVML_SUCCESS                 if \a state were successfully queried
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a state is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device*/
    fn nvmlSystemGetConfComputeState(
        state: *mut cuda_types::nvml::nvmlConfComputeSystemState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Conf Computing Protected and Unprotected Memory Sizes.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param device                               Device handle
 @param memInfo                              Protected/Unprotected Memory sizes

 @return
         - \ref NVML_SUCCESS                 if \a memInfo were successfully queried
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a memInfo or \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device*/
    fn nvmlDeviceGetConfComputeMemSizeInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        memInfo: *mut cuda_types::nvml::nvmlConfComputeMemSizeInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Conf Computing GPUs ready state.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param isAcceptingWork                      Returns GPU current work accepting state,
                                             NVML_CC_ACCEPTING_CLIENT_REQUESTS_TRUE or
                                             NVML_CC_ACCEPTING_CLIENT_REQUESTS_FALSE

 return
         - \ref NVML_SUCCESS                 if \a current GPUs ready state were successfully queried
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a isAcceptingWork is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device*/
    fn nvmlSystemGetConfComputeGpusReadyState(
        isAcceptingWork: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Conf Computing protected memory usage.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param device                               The identifier of the target device
 @param memory                               Reference in which to return the memory information

 @return
         - \ref NVML_SUCCESS                 if \a memory has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a memory is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetConfComputeProtectedMemoryUsage(
        device: cuda_types::nvml::nvmlDevice_t,
        memory: *mut cuda_types::nvml::nvmlMemory_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Conf Computing GPU certificate details.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param device                               The identifier of the target device
 @param gpuCert                              Reference in which to return the gpu certificate information

 @return
         - \ref NVML_SUCCESS                 if \a gpu certificate info has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a memory is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetConfComputeGpuCertificate(
        device: cuda_types::nvml::nvmlDevice_t,
        gpuCert: *mut cuda_types::nvml::nvmlConfComputeGpuCertificate_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Conf Computing GPU attestation report.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param device                               The identifier of the target device
 @param gpuAtstReport                        Reference in which to return the gpu attestation report

 @return
         - \ref NVML_SUCCESS                 if \a gpu attestation report has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a memory is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetConfComputeGpuAttestationReport(
        device: cuda_types::nvml::nvmlDevice_t,
        gpuAtstReport: *mut cuda_types::nvml::nvmlConfComputeGpuAttestationReport_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Conf Computing key rotation threshold detail.

 For Hopper &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param pKeyRotationThrInfo                  Reference in which to return the key rotation threshold data

 @return
         - \ref NVML_SUCCESS                 if \a gpu key rotation threshold info has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a memory is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlSystemGetConfComputeKeyRotationThresholdInfo(
        pKeyRotationThrInfo: *mut cuda_types::nvml::nvmlConfComputeGetKeyRotationThresholdInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set Conf Computing Unprotected Memory Size.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param device                               Device Handle
 @param sizeKiB                              Unprotected Memory size to be set in KiB

 @return
         - \ref NVML_SUCCESS                 if \a sizeKiB successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device*/
    fn nvmlDeviceSetConfComputeUnprotectedMemSize(
        device: cuda_types::nvml::nvmlDevice_t,
        sizeKiB: ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set Conf Computing GPUs ready state.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param isAcceptingWork                      GPU accepting new work, NVML_CC_ACCEPTING_CLIENT_REQUESTS_TRUE or
                                             NVML_CC_ACCEPTING_CLIENT_REQUESTS_FALSE

 return
         - \ref NVML_SUCCESS                 if \a current GPUs ready state is successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a isAcceptingWork is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device*/
    fn nvmlSystemSetConfComputeGpusReadyState(
        isAcceptingWork: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set Conf Computing key rotation threshold.

 For Hopper &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 This function is to set the confidential compute key rotation threshold parameters.
 \a pKeyRotationThrInfo->maxAttackerAdvantage should be in the range from
 NVML_CC_KEY_ROTATION_THRESHOLD_ATTACKER_ADVANTAGE_MIN to NVML_CC_KEY_ROTATION_THRESHOLD_ATTACKER_ADVANTAGE_MAX.
 Default value is 60.

 @param pKeyRotationThrInfo                  Reference to the key rotation threshold data

 @return
         - \ref NVML_SUCCESS                 if \a key rotation threashold max attacker advantage has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a memory is NULL
         - \ref NVML_ERROR_INVALID_STATE     if confidential compute GPU ready state is enabled
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlSystemSetConfComputeKeyRotationThresholdInfo(
        pKeyRotationThrInfo: *mut cuda_types::nvml::nvmlConfComputeSetKeyRotationThresholdInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Conf Computing System Settings.

 For Hopper &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param settings                                     System CC settings

 @return
         - \ref NVML_SUCCESS                         If the query is success
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device is invalid or \a counters is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED             If the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST               If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_UNKNOWN                   On any unexpected error*/
    fn nvmlSystemGetConfComputeSettings(
        settings: *mut cuda_types::nvml::nvmlSystemConfComputeSettings_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve GSP firmware version.

 The caller passes in buffer via \a version and corresponding GSP firmware numbered version
 is returned with the same parameter in string format.

 @param device                               Device handle
 @param version                              The retrieved GSP firmware version

 @return
         - \ref NVML_SUCCESS                 if GSP firmware version is sucessfully retrieved
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or GSP \a version pointer is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if GSP firmware is not enabled for GPU
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetGspFirmwareVersion(
        device: cuda_types::nvml::nvmlDevice_t,
        version: *mut ::core::ffi::c_char,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve GSP firmware mode.

 The caller passes in integer pointers. GSP firmware enablement and default mode information is returned with
 corresponding parameters. The return value in \a isEnabled and \a defaultMode should be treated as boolean.

 @param device                               Device handle
 @param isEnabled                            Pointer to specify if GSP firmware is enabled
 @param defaultMode                          Pointer to specify if GSP firmware is supported by default on \a device

 @return
         - \ref NVML_SUCCESS                 if GSP firmware mode is sucessfully retrieved
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or any of \a isEnabled or \a defaultMode is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if GSP firmware is not enabled for GPU
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetGspFirmwareMode(
        device: cuda_types::nvml::nvmlDevice_t,
        isEnabled: *mut ::core::ffi::c_uint,
        defaultMode: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get SRAM ECC error status of this device.

 For Ampere &tm; or newer fully supported devices.
 Requires root/admin permissions.

 See \ref nvmlEccSramErrorStatus_v1_t for more information on the struct.

 @param device                               The identifier of the target device
 @param status                               Returns SRAM ECC error status

 @return
         - \ref NVML_SUCCESS                          If \a limit has been set
         - \ref NVML_ERROR_UNINITIALIZED              If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a device is invalid or \a counters is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED              If the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST                If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a nvmlEccSramErrorStatus_t is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlDeviceGetSramEccErrorStatus(
        device: cuda_types::nvml::nvmlDevice_t,
        status: *mut cuda_types::nvml::nvmlEccSramErrorStatus_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Queries the state of per process accounting mode.

 For Kepler &tm; or newer fully supported devices.

 See \ref nvmlDeviceGetAccountingStats for more details.
 See \ref nvmlDeviceSetAccountingMode

 @param device                               The identifier of the target device
 @param mode                                 Reference in which to return the current accounting mode

 @return
         - \ref NVML_SUCCESS                 if the mode has been successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a mode are NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetAccountingMode(
        device: cuda_types::nvml::nvmlDevice_t,
        mode: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Queries process's accounting stats.

 For Kepler &tm; or newer fully supported devices.

 Accounting stats capture GPU utilization and other statistics across the lifetime of a process.
 Accounting stats can be queried during life time of the process and after its termination.
 The time field in \ref nvmlAccountingStats_t is reported as 0 during the lifetime of the process and
 updated to actual running time after its termination.
 Accounting stats are kept in a circular buffer, newly created processes overwrite information about old
 processes.

 See \ref nvmlAccountingStats_t for description of each returned metric.
 List of processes that can be queried can be retrieved from \ref nvmlDeviceGetAccountingPids.

 @note Accounting Mode needs to be on. See \ref nvmlDeviceGetAccountingMode.
 @note Only compute and graphics applications stats can be queried. Monitoring applications stats can't be
         queried since they don't contribute to GPU utilization.
 @note In case of pid collision stats of only the latest process (that terminated last) will be reported

 @warning On Kepler devices per process statistics are accurate only if there's one process running on a GPU.

 @param device                               The identifier of the target device
 @param pid                                  Process Id of the target process to query stats for
 @param stats                                Reference in which to return the process's accounting stats

 @return
         - \ref NVML_SUCCESS                 if stats have been successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a stats are NULL
         - \ref NVML_ERROR_NOT_FOUND         if process stats were not found
         - \ref NVML_ERROR_NOT_SUPPORTED     if \a device doesn't support this feature or accounting mode is disabled
                                              or on vGPU host.
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetAccountingBufferSize*/
    fn nvmlDeviceGetAccountingStats(
        device: cuda_types::nvml::nvmlDevice_t,
        pid: ::core::ffi::c_uint,
        stats: *mut cuda_types::nvml::nvmlAccountingStats_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Queries list of processes that can be queried for accounting stats. The list of processes returned
 can be in running or terminated state.

 For Kepler &tm; or newer fully supported devices.

 To query the number of processes under Accounting Mode, call this function with *count = 0 and pids=NULL.
 The return code will be NVML_ERROR_INSUFFICIENT_SIZE with an updated count value indicating the number of processes.

 For more details see \ref nvmlDeviceGetAccountingStats.

 @note In case of PID collision some processes might not be accessible before the circular buffer is full.

 @param device                               The identifier of the target device
 @param count                                Reference in which to provide the \a pids array size, and
                                               to return the number of elements ready to be queried
 @param pids                                 Reference in which to return list of process ids

 @return
         - \ref NVML_SUCCESS                 if pids were successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a count is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if \a device doesn't support this feature or accounting mode is disabled
                                              or on vGPU host.
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a count is too small (\a count is set to
                                                 expected value)
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetAccountingBufferSize*/
    fn nvmlDeviceGetAccountingPids(
        device: cuda_types::nvml::nvmlDevice_t,
        count: *mut ::core::ffi::c_uint,
        pids: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns the number of processes that the circular buffer with accounting pids can hold.

 For Kepler &tm; or newer fully supported devices.

 This is the maximum number of processes that accounting information will be stored for before information
 about oldest processes will get overwritten by information about new processes.

 @param device                               The identifier of the target device
 @param bufferSize                           Reference in which to provide the size (in number of elements)
                                               of the circular buffer for accounting stats.

 @return
         - \ref NVML_SUCCESS                 if buffer size was successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a bufferSize is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature or accounting mode is disabled
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetAccountingStats
 @see nvmlDeviceGetAccountingPids*/
    fn nvmlDeviceGetAccountingBufferSize(
        device: cuda_types::nvml::nvmlDevice_t,
        bufferSize: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns the list of retired pages by source, including pages that are pending retirement
 The address information provided from this API is the hardware address of the page that was retired.  Note
 that this does not match the virtual address used in CUDA, but will match the address information in Xid 63

 For Kepler &tm; or newer fully supported devices.

 @param device                            The identifier of the target device
 @param cause                             Filter page addresses by cause of retirement
 @param pageCount                         Reference in which to provide the \a addresses buffer size, and
                                          to return the number of retired pages that match \a cause
                                          Set to 0 to query the size without allocating an \a addresses buffer
 @param addresses                         Buffer to write the page addresses into

 @return
         - \ref NVML_SUCCESS                 if \a pageCount was populated and \a addresses was filled
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a pageCount indicates the buffer is not large enough to store all the
                                             matching page addresses.  \a pageCount is set to the needed size.
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a pageCount is NULL, \a cause is invalid, or
                                             \a addresses is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetRetiredPages(
        device: cuda_types::nvml::nvmlDevice_t,
        cause: cuda_types::nvml::nvmlPageRetirementCause_t,
        pageCount: *mut ::core::ffi::c_uint,
        addresses: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns the list of retired pages by source, including pages that are pending retirement
 The address information provided from this API is the hardware address of the page that was retired.  Note
 that this does not match the virtual address used in CUDA, but will match the address information in Xid 63

 \note nvmlDeviceGetRetiredPages_v2 adds an additional timestamps parameter to return the time of each page's
       retirement.

 For Kepler &tm; or newer fully supported devices.

 @param device                            The identifier of the target device
 @param cause                             Filter page addresses by cause of retirement
 @param pageCount                         Reference in which to provide the \a addresses buffer size, and
                                          to return the number of retired pages that match \a cause
                                          Set to 0 to query the size without allocating an \a addresses buffer
 @param addresses                         Buffer to write the page addresses into
 @param timestamps                        Buffer to write the timestamps of page retirement, additional for _v2

 @return
         - \ref NVML_SUCCESS                 if \a pageCount was populated and \a addresses was filled
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a pageCount indicates the buffer is not large enough to store all the
                                             matching page addresses.  \a pageCount is set to the needed size.
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a pageCount is NULL, \a cause is invalid, or
                                             \a addresses is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetRetiredPages_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        cause: cuda_types::nvml::nvmlPageRetirementCause_t,
        pageCount: *mut ::core::ffi::c_uint,
        addresses: *mut ::core::ffi::c_ulonglong,
        timestamps: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Check if any pages are pending retirement and need a reboot to fully retire.

 For Kepler &tm; or newer fully supported devices.

 @param device                            The identifier of the target device
 @param isPending                         Reference in which to return the pending status

 @return
         - \ref NVML_SUCCESS                 if \a isPending was populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a isPending is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetRetiredPagesPendingStatus(
        device: cuda_types::nvml::nvmlDevice_t,
        isPending: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get number of remapped rows. The number of rows reported will be based on
 the cause of the remapping. isPending indicates whether or not there are
 pending remappings. A reset will be required to actually remap the row.
 failureOccurred will be set if a row remapping ever failed in the past. A
 pending remapping won't affect future work on the GPU since
 error-containment and dynamic page blacklisting will take care of that.

 @note On MIG-enabled GPUs with active instances, querying the number of
 remapped rows is not supported

 For Ampere &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param corrRows                             Reference for number of rows remapped due to correctable errors
 @param uncRows                              Reference for number of rows remapped due to uncorrectable errors
 @param isPending                            Reference for whether or not remappings are pending
 @param failureOccurred                      Reference that is set when a remapping has failed in the past

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a corrRows, \a uncRows, \a isPending or \a failureOccurred is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If MIG is enabled or if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           Unexpected error*/
    fn nvmlDeviceGetRemappedRows(
        device: cuda_types::nvml::nvmlDevice_t,
        corrRows: *mut ::core::ffi::c_uint,
        uncRows: *mut ::core::ffi::c_uint,
        isPending: *mut ::core::ffi::c_uint,
        failureOccurred: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the row remapper histogram. Returns the remap availability for each bank
 on the GPU.

 @param device                               Device handle
 @param values                               Histogram values

 @return
        - \ref NVML_SUCCESS                  On success
        - \ref NVML_ERROR_UNKNOWN            On any unexpected error*/
    fn nvmlDeviceGetRowRemapperHistogram(
        device: cuda_types::nvml::nvmlDevice_t,
        values: *mut cuda_types::nvml::nvmlRowRemapperHistogramValues_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get architecture for device

 @param device                               The identifier of the target device
 @param arch                                 Reference where architecture is returned, if call successful.
                                             Set to NVML_DEVICE_ARCH_* upon success

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device or \a arch (output refererence) are invalid*/
    fn nvmlDeviceGetArchitecture(
        device: cuda_types::nvml::nvmlDevice_t,
        arch: *mut cuda_types::nvml::nvmlDeviceArchitecture_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the frequency monitor fault status for the device.

 For Ampere &tm; or newer fully supported devices.
 Requires root user.

 See \ref nvmlClkMonStatus_t for details on decoding the status output.

 @param device                               The identifier of the target device
 @param status                               Reference in which to return the clkmon fault status

 @return
         - \ref NVML_SUCCESS                 if \a status has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a status is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetClkMonStatus()*/
    fn nvmlDeviceGetClkMonStatus(
        device: cuda_types::nvml::nvmlDevice_t,
        status: *mut cuda_types::nvml::nvmlClkMonStatus_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current utilization and process ID

 For Maxwell &tm; or newer fully supported devices.

 Reads recent utilization of GPU SM (3D/Compute), framebuffer, video encoder, and video decoder for processes running.
 Utilization values are returned as an array of utilization sample structures in the caller-supplied buffer pointed at
 by \a utilization. One utilization sample structure is returned per process running, that had some non-zero utilization
 during the last sample period. It includes the CPU timestamp at which  the samples were recorded. Individual utilization values
 are returned as "unsigned int" values. If no valid sample entries are found since the lastSeenTimeStamp, NVML_ERROR_NOT_FOUND
 is returned.

 To read utilization values, first determine the size of buffer required to hold the samples by invoking the function with
 \a utilization set to NULL. The caller should allocate a buffer of size
 processSamplesCount * sizeof(nvmlProcessUtilizationSample_t). Invoke the function again with the allocated buffer passed
 in \a utilization, and \a processSamplesCount set to the number of entries the buffer is sized for.

 On successful return, the function updates \a processSamplesCount with the number of process utilization sample
 structures that were actually written. This may differ from a previously read value as instances are created or
 destroyed.

 lastSeenTimeStamp represents the CPU timestamp in microseconds at which utilization samples were last read. Set it to 0
 to read utilization based on all the samples maintained by the driver's internal sample buffer. Set lastSeenTimeStamp
 to a timeStamp retrieved from a previous query to read utilization since the previous query.

 @note On MIG-enabled GPUs, querying process utilization is not currently supported.

 @param device                    The identifier of the target device
 @param utilization               Pointer to caller-supplied buffer in which guest process utilization samples are returned
 @param processSamplesCount       Pointer to caller-supplied array size, and returns number of processes running
 @param lastSeenTimeStamp         Return only samples with timestamp greater than lastSeenTimeStamp.

 @return
         - \ref NVML_SUCCESS                 if \a utilization has been populated
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a utilization is NULL, or \a samplingPeriodUs is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_NOT_FOUND         if sample entries are not found
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetProcessUtilization(
        device: cuda_types::nvml::nvmlDevice_t,
        utilization: *mut cuda_types::nvml::nvmlProcessUtilizationSample_t,
        processSamplesCount: *mut ::core::ffi::c_uint,
        lastSeenTimeStamp: ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the recent utilization and process ID for all running processes

 For Maxwell &tm; or newer fully supported devices.

 Reads recent utilization of GPU SM (3D/Compute), framebuffer, video encoder, and video decoder, jpeg decoder, OFA (Optical Flow Accelerator)
 for all running processes. Utilization values are returned as an array of utilization sample structures in the caller-supplied buffer pointed at
 by \a procesesUtilInfo->procUtilArray. One utilization sample structure is returned per process running, that had some non-zero utilization
 during the last sample period. It includes the CPU timestamp at which  the samples were recorded. Individual utilization values
 are returned as "unsigned int" values.

 The caller should allocate a buffer of size processSamplesCount * sizeof(nvmlProcessUtilizationInfo_t). If the buffer is too small, the API will
 return \a NVML_ERROR_INSUFFICIENT_SIZE, with the recommended minimal buffer size at \a procesesUtilInfo->processSamplesCount. The caller should
 invoke the function again with the allocated buffer passed in \a procesesUtilInfo->procUtilArray, and \a procesesUtilInfo->processSamplesCount
 set to the number no less than the recommended value by the previous API return.

 On successful return, the function updates \a procesesUtilInfo->processSamplesCount with the number of process utilization info structures
 that were actually written. This may differ from a previously read value as instances are created or destroyed.

 \a procesesUtilInfo->lastSeenTimeStamp represents the CPU timestamp in microseconds at which utilization samples were last read. Set it to 0
 to read utilization based on all the samples maintained by the driver's internal sample buffer. Set \a procesesUtilInfo->lastSeenTimeStamp
 to a timeStamp retrieved from a previous query to read utilization since the previous query.

 \a procesesUtilInfo->version is the version number of the structure nvmlProcessesUtilizationInfo_t, the caller should set the correct version
 number to retrieve the specific version of processes utilization information.

 @note On MIG-enabled GPUs, querying process utilization is not currently supported.

 @param device                    The identifier of the target device
 @param procesesUtilInfo          Pointer to the caller-provided structure of nvmlProcessesUtilizationInfo_t.

 @return
         - \ref NVML_SUCCESS                          If \a procesesUtilInfo->procUtilArray has been populated
         - \ref NVML_ERROR_UNINITIALIZED              If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a device is invalid, or \a procesesUtilInfo is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED              If the device does not support this feature
         - \ref NVML_ERROR_NOT_FOUND                  If sample entries are not found
         - \ref NVML_ERROR_GPU_IS_LOST                If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a procesesUtilInfo is invalid
         - \ref NVML_ERROR_INSUFFICIENT_SIZE          If \a procesesUtilInfo->procUtilArray is NULL, or the buffer size of procesesUtilInfo->procUtilArray is too small.
                                                      The caller should check the minimul array size from the returned procesesUtilInfo->processSamplesCount, and call
                                                      the function again with a buffer no smaller than procesesUtilInfo->processSamplesCount * sizeof(nvmlProcessUtilizationInfo_t)
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlDeviceGetProcessesUtilizationInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        procesesUtilInfo: *mut cuda_types::nvml::nvmlProcessesUtilizationInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get platform information of this device.

 %BLACKWELL_OR_NEWER%

 See \ref nvmlPlatformInfo_v2_t for more information on the struct.

 @param device                               The identifier of the target device
 @param platformInfo                         Pointer to the caller-provided structure of nvmlPlatformInfo_t.

 @return
         - \ref NVML_SUCCESS                          If \a platformInfo has been retrieved
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a device is invalid or \a platformInfo is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED              If the device does not support this feature
         - \ref NVML_ERROR_MEMORY                     if system memory is insufficient
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a nvmlPlatformInfo_t is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlDeviceGetPlatformInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        platformInfo: *mut cuda_types::nvml::nvmlPlatformInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the LED state for the unit. The LED can be either green (0) or amber (1).

 For S-class products.
 Requires root/admin permissions.

 This operation takes effect immediately.


 <b>Current S-Class products don't provide unique LEDs for each unit. As such, both front
 and back LEDs will be toggled in unison regardless of which unit is specified with this command.</b>

 See \ref nvmlLedColor_t for available colors.

 @param unit                                 The identifier of the target unit
 @param color                                The target LED color

 @return
         - \ref NVML_SUCCESS                 if the LED color has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a unit or \a color is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this is not an S-class product
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlUnitGetLedState()*/
    fn nvmlUnitSetLedState(
        unit: cuda_types::nvml::nvmlUnit_t,
        color: cuda_types::nvml::nvmlLedColor_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the persistence mode for the device.

 For all products.
 For Linux only.
 Requires root/admin permissions.

 The persistence mode determines whether the GPU driver software is torn down after the last client
 exits.

 This operation takes effect immediately. It is not persistent across reboots. After each reboot the
 persistence mode is reset to "Disabled".

 See \ref nvmlEnableState_t for available modes.

 After calling this API with mode set to NVML_FEATURE_DISABLED on a device that has its own NUMA
 memory, the given device handle will no longer be valid, and to continue to interact with this
 device, a new handle should be obtained from one of the nvmlDeviceGetHandleBy*() APIs. This
 limitation is currently only applicable to devices that have a coherent NVLink connection to
 system memory.

 @param device                               The identifier of the target device
 @param mode                                 The target persistence mode

 @return
         - \ref NVML_SUCCESS                 if the persistence mode was set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a mode is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetPersistenceMode()*/
    fn nvmlDeviceSetPersistenceMode(
        device: cuda_types::nvml::nvmlDevice_t,
        mode: cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the compute mode for the device.

 For all products.
 Requires root/admin permissions.

 The compute mode determines whether a GPU can be used for compute operations and whether it can
 be shared across contexts.

 This operation takes effect immediately. Under Linux it is not persistent across reboots and
 always resets to "Default". Under windows it is persistent.

 Under windows compute mode may only be set to DEFAULT when running in WDDM

 @note On MIG-enabled GPUs, compute mode would be set to DEFAULT and changing it is not supported.

 See \ref nvmlComputeMode_t for details on available compute modes.

 @param device                               The identifier of the target device
 @param mode                                 The target compute mode

 @return
         - \ref NVML_SUCCESS                 if the compute mode was set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a mode is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetComputeMode()*/
    fn nvmlDeviceSetComputeMode(
        device: cuda_types::nvml::nvmlDevice_t,
        mode: cuda_types::nvml::nvmlComputeMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the ECC mode for the device.

 For Kepler &tm; or newer fully supported devices.
 Only applicable to devices with ECC.
 Requires \a NVML_INFOROM_ECC version 1.0 or higher.
 Requires root/admin permissions.

 The ECC mode determines whether the GPU enables its ECC support.

 This operation takes effect after the next reboot.

 See \ref nvmlEnableState_t for details on available modes.

 @param device                               The identifier of the target device
 @param ecc                                  The target ECC mode

 @return
         - \ref NVML_SUCCESS                 if the ECC mode was set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a ecc is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetEccMode()*/
    fn nvmlDeviceSetEccMode(
        device: cuda_types::nvml::nvmlDevice_t,
        ecc: cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Clear the ECC error and other memory error counts for the device.

 For Kepler &tm; or newer fully supported devices.
 Only applicable to devices with ECC.
 Requires \a NVML_INFOROM_ECC version 2.0 or higher to clear aggregate location-based ECC counts.
 Requires \a NVML_INFOROM_ECC version 1.0 or higher to clear all other ECC counts.
 Requires root/admin permissions.
 Requires ECC Mode to be enabled.

 Sets all of the specified ECC counters to 0, including both detailed and total counts.

 This operation takes effect immediately.

 See \ref nvmlMemoryErrorType_t for details on available counter types.

 @param device                               The identifier of the target device
 @param counterType                          Flag that indicates which type of errors should be cleared.

 @return
         - \ref NVML_SUCCESS                 if the error counts were cleared
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a counterType is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see
      - nvmlDeviceGetDetailedEccErrors()
      - nvmlDeviceGetTotalEccErrors()*/
    fn nvmlDeviceClearEccErrorCounts(
        device: cuda_types::nvml::nvmlDevice_t,
        counterType: cuda_types::nvml::nvmlEccCounterType_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the driver model for the device.

 For Fermi &tm; or newer fully supported devices.
 For windows only.
 Requires root/admin permissions.

 On Windows platforms the device driver can run in either WDDM or WDM (TCC) mode. If a display is attached
 to the device it must run in WDDM mode.

 It is possible to force the change to WDM (TCC) while the display is still attached with a force flag (nvmlFlagForce).
 This should only be done if the host is subsequently powered down and the display is detached from the device
 before the next reboot.

 This operation takes effect after the next reboot.

 Windows driver model may only be set to WDDM when running in DEFAULT compute mode.

 Change driver model to WDDM is not supported when GPU doesn't support graphics acceleration or
 will not support it after reboot. See \ref nvmlDeviceSetGpuOperationMode.

 See \ref nvmlDriverModel_t for details on available driver models.
 See \ref nvmlFlagDefault and \ref nvmlFlagForce

 @param device                               The identifier of the target device
 @param driverModel                          The target driver model
 @param flags                                Flags that change the default behavior

 @return
         - \ref NVML_SUCCESS                 if the driver model has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a driverModel is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the platform is not windows or the device does not support this feature
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetDriverModel()*/
    fn nvmlDeviceSetDriverModel(
        device: cuda_types::nvml::nvmlDevice_t,
        driverModel: cuda_types::nvml::nvmlDriverModel_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set clocks that device will lock to.

 Sets the clocks that the device will be running at to the value in the range of minGpuClockMHz to maxGpuClockMHz.
 Setting this will supersede application clock values and take effect regardless if a cuda app is running.
 See /ref nvmlDeviceSetApplicationsClocks

 Can be used as a setting to request constant performance.

 This can be called with a pair of integer clock frequencies in MHz, or a pair of /ref nvmlClockLimitId_t values.
 See the table below for valid combinations of these values.

 minGpuClock | maxGpuClock | Effect
 ------------+-------------+--------------------------------------------------
     tdp     |     tdp     | Lock clock to TDP
  unlimited  |     tdp     | Upper bound is TDP but clock may drift below this
     tdp     |  unlimited  | Lower bound is TDP but clock may boost above this
  unlimited  |  unlimited  | Unlocked (== nvmlDeviceResetGpuLockedClocks)

 If one arg takes one of these values, the other must be one of these values as
 well. Mixed numeric and symbolic calls return NVML_ERROR_INVALID_ARGUMENT.

 Requires root/admin permissions.

 After system reboot or driver reload applications clocks go back to their default value.
 See \ref nvmlDeviceResetGpuLockedClocks.

 For Volta &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param minGpuClockMHz                       Requested minimum gpu clock in MHz
 @param maxGpuClockMHz                       Requested maximum gpu clock in MHz

 @return
         - \ref NVML_SUCCESS                 if new settings were successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a minGpuClockMHz and \a maxGpuClockMHz
                                                 is not a valid clock combination
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetGpuLockedClocks(
        device: cuda_types::nvml::nvmlDevice_t,
        minGpuClockMHz: ::core::ffi::c_uint,
        maxGpuClockMHz: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Resets the gpu clock to the default value

 This is the gpu clock that will be used after system reboot or driver reload.
 Default values are idle clocks, but the current values can be changed using \ref nvmlDeviceSetApplicationsClocks.

 @see nvmlDeviceSetGpuLockedClocks

 For Volta &tm; or newer fully supported devices.

 @param device                               The identifier of the target device

 @return
         - \ref NVML_SUCCESS                 if new settings were successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceResetGpuLockedClocks(
        device: cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set memory clocks that device will lock to.

 Sets the device's memory clocks to the value in the range of minMemClockMHz to maxMemClockMHz.
 Setting this will supersede application clock values and take effect regardless of whether a cuda app is running.
 See /ref nvmlDeviceSetApplicationsClocks

 Can be used as a setting to request constant performance.

 Requires root/admin permissions.

 After system reboot or driver reload applications clocks go back to their default value.
 See \ref nvmlDeviceResetMemoryLockedClocks.

 For Ampere &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param minMemClockMHz                       Requested minimum memory clock in MHz
 @param maxMemClockMHz                       Requested maximum memory clock in MHz

 @return
         - \ref NVML_SUCCESS                 if new settings were successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a minGpuClockMHz and \a maxGpuClockMHz
                                                 is not a valid clock combination
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetMemoryLockedClocks(
        device: cuda_types::nvml::nvmlDevice_t,
        minMemClockMHz: ::core::ffi::c_uint,
        maxMemClockMHz: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Resets the memory clock to the default value

 This is the memory clock that will be used after system reboot or driver reload.
 Default values are idle clocks, but the current values can be changed using \ref nvmlDeviceSetApplicationsClocks.

 @see nvmlDeviceSetMemoryLockedClocks

 For Ampere &tm; or newer fully supported devices.

 @param device                               The identifier of the target device

 @return
         - \ref NVML_SUCCESS                 if new settings were successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceResetMemoryLockedClocks(
        device: cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set clocks that applications will lock to.

 Sets the clocks that compute and graphics applications will be running at.
 e.g. CUDA driver requests these clocks during context creation which means this property
 defines clocks at which CUDA applications will be running unless some overspec event
 occurs (e.g. over power, over thermal or external HW brake).

 Can be used as a setting to request constant performance.

 On Pascal and newer hardware, this will automatically disable automatic boosting of clocks.

 On K80 and newer Kepler and Maxwell GPUs, users desiring fixed performance should also call
 \ref nvmlDeviceSetAutoBoostedClocksEnabled to prevent clocks from automatically boosting
 above the clock value being set.

 For Kepler &tm; or newer non-GeForce fully supported devices and Maxwell or newer GeForce devices.
 Requires root/admin permissions.

 See \ref nvmlDeviceGetSupportedMemoryClocks and \ref nvmlDeviceGetSupportedGraphicsClocks
 for details on how to list available clocks combinations.

 After system reboot or driver reload applications clocks go back to their default value.
 See \ref nvmlDeviceResetApplicationsClocks.

 @param device                               The identifier of the target device
 @param memClockMHz                          Requested memory clock in MHz
 @param graphicsClockMHz                     Requested graphics clock in MHz

 @return
         - \ref NVML_SUCCESS                 if new settings were successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a memClockMHz and \a graphicsClockMHz
                                                 is not a valid clock combination
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetApplicationsClocks(
        device: cuda_types::nvml::nvmlDevice_t,
        memClockMHz: ::core::ffi::c_uint,
        graphicsClockMHz: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Resets the application clock to the default value

 This is the applications clock that will be used after system reboot or driver reload.
 Default value is constant, but the current value an be changed using \ref nvmlDeviceSetApplicationsClocks.

 On Pascal and newer hardware, if clocks were previously locked with \ref nvmlDeviceSetApplicationsClocks,
 this call will unlock clocks. This returns clocks their default behavior ofautomatically boosting above
 base clocks as thermal limits allow.

 @see nvmlDeviceGetApplicationsClock
 @see nvmlDeviceSetApplicationsClocks

 For Fermi &tm; or newer non-GeForce fully supported devices and Maxwell or newer GeForce devices.

 @param device                               The identifier of the target device

 @return
         - \ref NVML_SUCCESS                 if new settings were successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceResetApplicationsClocks(
        device: cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Try to set the current state of Auto Boosted clocks on a device.

 For Kepler &tm; or newer fully supported devices.

 Auto Boosted clocks are enabled by default on some hardware, allowing the GPU to run at higher clock rates
 to maximize performance as thermal limits allow. Auto Boosted clocks should be disabled if fixed clock
 rates are desired.

 Non-root users may use this API by default but can be restricted by root from using this API by calling
 \ref nvmlDeviceSetAPIRestriction with apiType=NVML_RESTRICTED_API_SET_AUTO_BOOSTED_CLOCKS.
 Note: Persistence Mode is required to modify current Auto Boost settings, therefore, it must be enabled.

 On Pascal and newer hardware, Auto Boosted clocks are controlled through application clocks.
 Use \ref nvmlDeviceSetApplicationsClocks and \ref nvmlDeviceResetApplicationsClocks to control Auto Boost
 behavior.

 @param device                               The identifier of the target device
 @param enabled                              What state to try to set Auto Boosted clocks of the target device to

 @return
         - \ref NVML_SUCCESS                 If the Auto Boosted clocks were successfully set to the state specified by \a enabled
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support Auto Boosted clocks
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error
*/
    fn nvmlDeviceSetAutoBoostedClocksEnabled(
        device: cuda_types::nvml::nvmlDevice_t,
        enabled: cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Try to set the default state of Auto Boosted clocks on a device. This is the default state that Auto Boosted clocks will
 return to when no compute running processes (e.g. CUDA application which have an active context) are running

 For Kepler &tm; or newer non-GeForce fully supported devices and Maxwell or newer GeForce devices.
 Requires root/admin permissions.

 Auto Boosted clocks are enabled by default on some hardware, allowing the GPU to run at higher clock rates
 to maximize performance as thermal limits allow. Auto Boosted clocks should be disabled if fixed clock
 rates are desired.

 On Pascal and newer hardware, Auto Boosted clocks are controlled through application clocks.
 Use \ref nvmlDeviceSetApplicationsClocks and \ref nvmlDeviceResetApplicationsClocks to control Auto Boost
 behavior.

 @param device                               The identifier of the target device
 @param enabled                              What state to try to set default Auto Boosted clocks of the target device to
 @param flags                                Flags that change the default behavior. Currently Unused.

 @return
         - \ref NVML_SUCCESS                 If the Auto Boosted clock's default state was successfully set to the state specified by \a enabled
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_NO_PERMISSION     If the calling user does not have permission to change Auto Boosted clock's default state.
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support Auto Boosted clocks
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error
*/
    fn nvmlDeviceSetDefaultAutoBoostedClocksEnabled(
        device: cuda_types::nvml::nvmlDevice_t,
        enabled: cuda_types::nvml::nvmlEnableState_t,
        flags: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Sets the speed of the fan control policy to default.

 For all cuda-capable discrete products with fans

 @param device                        The identifier of the target device
 @param fan                           The index of the fan, starting at zero

 return
         NVML_SUCCESS                 if speed has been adjusted
         NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         NVML_ERROR_INVALID_ARGUMENT  if device is invalid
         NVML_ERROR_NOT_SUPPORTED     if the device does not support this
                                      (doesn't have fans)
         NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetDefaultFanSpeed_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        fan: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Sets current fan control policy.

 For Maxwell &tm; or newer fully supported devices.

 Requires privileged user.

 For all cuda-capable discrete products with fans

 device                               The identifier of the target \a device
 policy                               The fan control \a policy to set

 return
         NVML_SUCCESS                 if \a policy has been set
         NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a policy is null or the \a fan given doesn't reference
                                            a fan that exists.
         NVML_ERROR_NOT_SUPPORTED     if the \a device is older than Maxwell
         NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetFanControlPolicy(
        device: cuda_types::nvml::nvmlDevice_t,
        fan: ::core::ffi::c_uint,
        policy: cuda_types::nvml::nvmlFanControlPolicy_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Sets the temperature threshold for the GPU with the specified threshold type in degrees C.

 For Maxwell &tm; or newer fully supported devices.

 See \ref nvmlTemperatureThresholds_t for details on available temperature thresholds.

 @param device                               The identifier of the target device
 @param thresholdType                        The type of threshold value to be set
 @param temp                                 Reference which hold the value to be set
 @return
         - \ref NVML_SUCCESS                 if \a temp has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a thresholdType is invalid or \a temp is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not have a temperature sensor or is unsupported
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetTemperatureThreshold(
        device: cuda_types::nvml::nvmlDevice_t,
        thresholdType: cuda_types::nvml::nvmlTemperatureThresholds_t,
        temp: *mut ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set new power limit of this device.

 For Kepler &tm; or newer fully supported devices.
 Requires root/admin permissions.

 See \ref nvmlDeviceGetPowerManagementLimitConstraints to check the allowed ranges of values.

 \note Limit is not persistent across reboots or driver unloads.
 Enable persistent mode to prevent driver from unloading when no application is using the device.

 @param device                               The identifier of the target device
 @param limit                                Power management limit in milliwatts to set

 @return
         - \ref NVML_SUCCESS                 if \a limit has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a defaultLimit is out of range
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceGetPowerManagementLimitConstraints
 @see nvmlDeviceGetPowerManagementDefaultLimit*/
    fn nvmlDeviceSetPowerManagementLimit(
        device: cuda_types::nvml::nvmlDevice_t,
        limit: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Sets new GOM. See \a nvmlGpuOperationMode_t for details.

 For GK110 M-class and X-class Tesla &tm; products from the Kepler family.
 Modes \ref NVML_GOM_LOW_DP and \ref NVML_GOM_ALL_ON are supported on fully supported GeForce products.
 Not supported on Quadro &reg; and Tesla &tm; C-class products.
 Requires root/admin permissions.

 Changing GOMs requires a reboot.
 The reboot requirement might be removed in the future.

 Compute only GOMs don't support graphics acceleration. Under windows switching to these GOMs when
 pending driver model is WDDM is not supported. See \ref nvmlDeviceSetDriverModel.

 @param device                               The identifier of the target device
 @param mode                                 Target GOM

 @return
         - \ref NVML_SUCCESS                 if \a mode has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a mode incorrect
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support GOM or specific mode
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlGpuOperationMode_t
 @see nvmlDeviceGetGpuOperationMode*/
    fn nvmlDeviceSetGpuOperationMode(
        device: cuda_types::nvml::nvmlDevice_t,
        mode: cuda_types::nvml::nvmlGpuOperationMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Changes the root/admin restructions on certain APIs. See \a nvmlRestrictedAPI_t for the list of supported APIs.
 This method can be used by a root/admin user to give non-root/admin access to certain otherwise-restricted APIs.
 The new setting lasts for the lifetime of the NVIDIA driver; it is not persistent. See \a nvmlDeviceGetAPIRestriction
 to query the current restriction settings.

 For Kepler &tm; or newer fully supported devices.
 Requires root/admin permissions.

 @param device                               The identifier of the target device
 @param apiType                              Target API type for this operation
 @param isRestricted                         The target restriction

 @return
         - \ref NVML_SUCCESS                 if \a isRestricted has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a apiType incorrect
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support changing API restrictions or the device does not support
                                                 the feature that api restrictions are being set for (E.G. Enabling/disabling auto
                                                 boosted clocks is not supported by the device)
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlRestrictedAPI_t*/
    fn nvmlDeviceSetAPIRestriction(
        device: cuda_types::nvml::nvmlDevice_t,
        apiType: cuda_types::nvml::nvmlRestrictedAPI_t,
        isRestricted: cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Sets the speed of a specified fan.

 WARNING: This function changes the fan control policy to manual. It means that YOU have to monitor
          the temperature and adjust the fan speed accordingly.
          If you set the fan speed too low you can burn your GPU!
          Use nvmlDeviceSetDefaultFanSpeed_v2 to restore default control policy.

 For all cuda-capable discrete products with fans that are Maxwell or Newer.

 device                                The identifier of the target device
 fan                                   The index of the fan, starting at zero
 speed                                 The target speed of the fan [0-100] in % of max speed

 return
        NVML_SUCCESS                   if the fan speed has been set
        NVML_ERROR_UNINITIALIZED       if the library has not been successfully initialized
        NVML_ERROR_INVALID_ARGUMENT    if the device is not valid, or the speed is outside acceptable ranges,
                                              or if the fan index doesn't reference an actual fan.
        NVML_ERROR_NOT_SUPPORTED       if the device is older than Maxwell.
        NVML_ERROR_UNKNOWN             if there was an unexpected error.*/
    fn nvmlDeviceSetFanSpeed_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        fan: ::core::ffi::c_uint,
        speed: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Deprecated: Will be deprecated in a future release. Use \ref nvmlDeviceSetClockOffsets instead. It works
             on Maxwell onwards GPU architectures.

 Set the GPCCLK VF offset value
 @param[in]   device                         The identifier of the target device
 @param[in]   offset                         The GPCCLK VF offset value to set

 @return
         - \ref NVML_SUCCESS                 if \a offset has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a offset is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetGpcClkVfOffset(
        device: cuda_types::nvml::nvmlDevice_t,
        offset: ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Deprecated: Will be deprecated in a future release. Use \ref nvmlDeviceSetClockOffsets instead. It works
             on Maxwell onwards GPU architectures.

 Set the MemClk (Memory Clock) VF offset value. It requires elevated privileges.
 @param[in]   device                         The identifier of the target device
 @param[in]   offset                         The MemClk VF offset value to set

 @return
         - \ref NVML_SUCCESS                 if \a offset has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a offset is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetMemClkVfOffset(
        device: cuda_types::nvml::nvmlDevice_t,
        offset: ::core::ffi::c_int,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Enables or disables per process accounting.

 For Kepler &tm; or newer fully supported devices.
 Requires root/admin permissions.

 @note This setting is not persistent and will default to disabled after driver unloads.
       Enable persistence mode to be sure the setting doesn't switch off to disabled.

 @note Enabling accounting mode has no negative impact on the GPU performance.

 @note Disabling accounting clears all accounting pids information.

 @note On MIG-enabled GPUs, accounting mode would be set to DISABLED and changing it is not supported.

 See \ref nvmlDeviceGetAccountingMode
 See \ref nvmlDeviceGetAccountingStats
 See \ref nvmlDeviceClearAccountingPids

 @param device                               The identifier of the target device
 @param mode                                 The target accounting mode

 @return
         - \ref NVML_SUCCESS                 if the new mode has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a mode are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetAccountingMode(
        device: cuda_types::nvml::nvmlDevice_t,
        mode: cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Clears accounting information about all processes that have already terminated.

 For Kepler &tm; or newer fully supported devices.
 Requires root/admin permissions.

 See \ref nvmlDeviceGetAccountingMode
 See \ref nvmlDeviceGetAccountingStats
 See \ref nvmlDeviceSetAccountingMode

 @param device                               The identifier of the target device

 @return
         - \ref NVML_SUCCESS                 if accounting information has been cleared
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceClearAccountingPids(
        device: cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set new power limit of this device.

 For Kepler &tm; or newer fully supported devices.
 Requires root/admin permissions.

 See \ref nvmlDeviceGetPowerManagementLimitConstraints to check the allowed ranges of values.

 See \ref nvmlPowerValue_v2_t for more information on the struct.

 \note Limit is not persistent across reboots or driver unloads.
 Enable persistent mode to prevent driver from unloading when no application is using the device.

 This API replaces nvmlDeviceSetPowerManagementLimit. It can be used as a drop-in replacement for the older version.

 @param device                               The identifier of the target device
 @param powerValue                           Power management limit in milliwatts to set

 @return
         - \ref NVML_SUCCESS                 if \a limit has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a powerValue is NULL or contains invalid values
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see NVML_FI_DEV_POWER_AVERAGE
 @see NVML_FI_DEV_POWER_INSTANT
 @see NVML_FI_DEV_POWER_MIN_LIMIT
 @see NVML_FI_DEV_POWER_MAX_LIMIT
 @see NVML_FI_DEV_POWER_CURRENT_LIMIT*/
    fn nvmlDeviceSetPowerManagementLimit_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        powerValue: *mut cuda_types::nvml::nvmlPowerValue_v2_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the state of the device's NvLink for the link specified

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param link                                 Specifies the NvLink link to be queried
 @param isActive                             \a nvmlEnableState_t where NVML_FEATURE_ENABLED indicates that
                                             the link is active and NVML_FEATURE_DISABLED indicates it
                                             is inactive

 @return
         - \ref NVML_SUCCESS                 if \a isActive has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a link is invalid or \a isActive is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetNvLinkState(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        isActive: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the version of the device's NvLink for the link specified

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param link                                 Specifies the NvLink link to be queried
 @param version                              Requested NvLink version from nvmlNvlinkVersion_t

 @return
         - \ref NVML_SUCCESS                 if \a version has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a link is invalid or \a version is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetNvLinkVersion(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        version: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the requested capability from the device's NvLink for the link specified
 Please refer to the \a nvmlNvLinkCapability_t structure for the specific caps that can be queried
 The return value should be treated as a boolean.

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param link                                 Specifies the NvLink link to be queried
 @param capability                           Specifies the \a nvmlNvLinkCapability_t to be queried
 @param capResult                            A boolean for the queried capability indicating that feature is available

 @return
         - \ref NVML_SUCCESS                 if \a capResult has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a link, or \a capability is invalid or \a capResult is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetNvLinkCapability(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        capability: cuda_types::nvml::nvmlNvLinkCapability_t,
        capResult: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the PCI information for the remote node on a NvLink link
 Note: pciSubSystemId is not filled in this function and is indeterminate

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param link                                 Specifies the NvLink link to be queried
 @param pci                                  \a nvmlPciInfo_t of the remote node for the specified link

 @return
         - \ref NVML_SUCCESS                 if \a pci has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a link is invalid or \a pci is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetNvLinkRemotePciInfo_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        pci: *mut cuda_types::nvml::nvmlPciInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the specified error counter value
 Please refer to \a nvmlNvLinkErrorCounter_t for error counters that are available

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param link                                 Specifies the NvLink link to be queried
 @param counter                              Specifies the NvLink counter to be queried
 @param counterValue                         Returned counter value

 @return
         - \ref NVML_SUCCESS                 if \a counter has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a link, or \a counter is invalid or \a counterValue is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetNvLinkErrorCounter(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        counter: cuda_types::nvml::nvmlNvLinkErrorCounter_t,
        counterValue: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Resets all error counters to zero
 Please refer to \a nvmlNvLinkErrorCounter_t for the list of error counters that are reset

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param link                                 Specifies the NvLink link to be queried

 @return
         - \ref NVML_SUCCESS                 if the reset is successful
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a link is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceResetNvLinkErrorCounters(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Deprecated: Setting utilization counter control is no longer supported.

 Set the NVLINK utilization counter control information for the specified counter, 0 or 1.
 Please refer to \a nvmlNvLinkUtilizationControl_t for the structure definition.  Performs a reset
 of the counters if the reset parameter is non-zero.

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param counter                              Specifies the counter that should be set (0 or 1).
 @param link                                 Specifies the NvLink link to be queried
 @param control                              A reference to the \a nvmlNvLinkUtilizationControl_t to set
 @param reset                                Resets the counters on set if non-zero

 @return
         - \ref NVML_SUCCESS                 if the control has been set successfully
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a counter, \a link, or \a control is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceSetNvLinkUtilizationControl(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        counter: ::core::ffi::c_uint,
        control: *mut cuda_types::nvml::nvmlNvLinkUtilizationControl_t,
        reset: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Deprecated: Getting utilization counter control is no longer supported.

 Get the NVLINK utilization counter control information for the specified counter, 0 or 1.
 Please refer to \a nvmlNvLinkUtilizationControl_t for the structure definition

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param counter                              Specifies the counter that should be set (0 or 1).
 @param link                                 Specifies the NvLink link to be queried
 @param control                              A reference to the \a nvmlNvLinkUtilizationControl_t to place information

 @return
         - \ref NVML_SUCCESS                 if the control has been set successfully
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a counter, \a link, or \a control is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetNvLinkUtilizationControl(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        counter: ::core::ffi::c_uint,
        control: *mut cuda_types::nvml::nvmlNvLinkUtilizationControl_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Deprecated: Use \ref nvmlDeviceGetFieldValues with NVML_FI_DEV_NVLINK_THROUGHPUT_* as field values instead.

 Retrieve the NVLINK utilization counter based on the current control for a specified counter.
 In general it is good practice to use \a nvmlDeviceSetNvLinkUtilizationControl
  before reading the utilization counters as they have no default state

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param link                                 Specifies the NvLink link to be queried
 @param counter                              Specifies the counter that should be read (0 or 1).
 @param rxcounter                            Receive counter return value
 @param txcounter                            Transmit counter return value

 @return
         - \ref NVML_SUCCESS                 if \a rxcounter and \a txcounter have been successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a counter, or \a link is invalid or \a rxcounter or \a txcounter are NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetNvLinkUtilizationCounter(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        counter: ::core::ffi::c_uint,
        rxcounter: *mut ::core::ffi::c_ulonglong,
        txcounter: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Deprecated: Freezing NVLINK utilization counters is no longer supported.

 Freeze the NVLINK utilization counters
 Both the receive and transmit counters are operated on by this function

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param link                                 Specifies the NvLink link to be queried
 @param counter                              Specifies the counter that should be frozen (0 or 1).
 @param freeze                               NVML_FEATURE_ENABLED = freeze the receive and transmit counters
                                             NVML_FEATURE_DISABLED = unfreeze the receive and transmit counters

 @return
         - \ref NVML_SUCCESS                 if counters were successfully frozen or unfrozen
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a link, \a counter, or \a freeze is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceFreezeNvLinkUtilizationCounter(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        counter: ::core::ffi::c_uint,
        freeze: cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Deprecated: Resetting NVLINK utilization counters is no longer supported.

 Reset the NVLINK utilization counters
 Both the receive and transmit counters are operated on by this function

 For Pascal &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param link                                 Specifies the NvLink link to be reset
 @param counter                              Specifies the counter that should be reset (0 or 1)

 @return
         - \ref NVML_SUCCESS                 if counters were successfully reset
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a link, or \a counter is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceResetNvLinkUtilizationCounter(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        counter: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the NVLink device type of the remote device connected over the given link.

 @param device                                The device handle of the target GPU
 @param link                                  The NVLink link index on the target GPU
 @param pNvLinkDeviceType                     Pointer in which the output remote device type is returned

 @return
         - \ref NVML_SUCCESS                  if \a pNvLinkDeviceType has been set
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_NOT_SUPPORTED      if NVLink is not supported
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a device or \a link is invalid, or
                                              \a pNvLinkDeviceType is NULL
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is
                                              otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetNvLinkRemoteDeviceType(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        pNvLinkDeviceType: *mut cuda_types::nvml::nvmlIntNvLinkDeviceType_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set NvLink Low Power Threshold for device.

 For Hopper &tm; or newer fully supported devices.

 @param device                               The identifier of the target device
 @param info                                 Reference to \a nvmlNvLinkPowerThres_t struct
                                             input parameters

 @return
        - \ref NVML_SUCCESS                 if the \a Threshold is successfully set
        - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
        - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a Threshold is not within range
        - \ref NVML_ERROR_NOT_READY         if an internal driver setting prevents the threshold from being used
        - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
*/
    fn nvmlDeviceSetNvLinkDeviceLowPowerThreshold(
        device: cuda_types::nvml::nvmlDevice_t,
        info: *mut cuda_types::nvml::nvmlNvLinkPowerThres_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the global nvlink bandwith mode

 @param nvlinkBwMode             nvlink bandwidth mode
 @return
         - \ref NVML_SUCCESS                on success
         - \ref NVML_ERROR_INVALID_ARGUMENT if an invalid argument is provided
         - \ref NVML_ERROR_IN_USE           if P2P object exists
         - \ref NVML_ERROR_NOT_SUPPORTED    if GPU is not Hopper or newer architecture.
         - \ref NVML_ERROR_NO_PERMISSION    if not root user*/
    fn nvmlSystemSetNvlinkBwMode(
        nvlinkBwMode: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the global nvlink bandwith mode

 @param nvlinkBwMode             reference of nvlink bandwidth mode
 @return
         - \ref NVML_SUCCESS                on success
         - \ref NVML_ERROR_INVALID_ARGUMENT if an invalid pointer is provided
         - \ref NVML_ERROR_NOT_SUPPORTED    if GPU is not Hopper or newer architecture.
         - \ref NVML_ERROR_NO_PERMISSION    if not root user*/
    fn nvmlSystemGetNvlinkBwMode(
        nvlinkBwMode: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the supported NvLink Reduced Bandwidth Modes of the device

 %BLACKWELL_OR_NEWER%

 @param device                                      The identifier of the target device
 @param supportedBwMode                             Reference to \a nvmlNvlinkSupportedBwModes_t

 @return
        - \ref NVML_SUCCESS                         if the query was successful
        - \ref NVML_ERROR_INVALID_ARGUMENT          if device is invalid or supportedBwMode is NULL
        - \ref NVML_ERROR_NOT_SUPPORTED             if this feature is not supported by the device
        - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH if the version specified is not supported*/
    fn nvmlDeviceGetNvlinkSupportedBwModes(
        device: cuda_types::nvml::nvmlDevice_t,
        supportedBwMode: *mut cuda_types::nvml::nvmlNvlinkSupportedBwModes_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the NvLink Reduced Bandwidth Mode for the device

 %BLACKWELL_OR_NEWER%

 @param device                                      The identifier of the target device
 @param getBwMode                                   Reference to \a nvmlNvlinkGetBwMode_t

 @return
        - \ref NVML_SUCCESS                         if the query was successful
        - \ref NVML_ERROR_INVALID_ARGUMENT          if device is invalid or getBwMode is NULL
        - \ref NVML_ERROR_NOT_SUPPORTED             if this feature is not supported by the device
        - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH if the version specified is not supported*/
    fn nvmlDeviceGetNvlinkBwMode(
        device: cuda_types::nvml::nvmlDevice_t,
        getBwMode: *mut cuda_types::nvml::nvmlNvlinkGetBwMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the NvLink Reduced Bandwidth Mode for the device

 %BLACKWELL_OR_NEWER%

 @param device                                      The identifier of the target device
 @param setBwMode                                   Reference to \a nvmlNvlinkSetBwMode_t

 @return
        - \ref NVML_SUCCESS                         if the Bandwidth mode was successfully set
        - \ref NVML_ERROR_INVALID_ARGUMENT          if device is invalid or setBwMode is NULL
        - \ref NVML_ERROR_NO_PERMISSION             if user does not have permission to change Bandwidth mode
        - \ref NVML_ERROR_NOT_SUPPORTED             if this feature is not supported by the device
        - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH if the version specified is not supported*/
    fn nvmlDeviceSetNvlinkBwMode(
        device: cuda_types::nvml::nvmlDevice_t,
        setBwMode: *mut cuda_types::nvml::nvmlNvlinkSetBwMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Create an empty set of events.
 Event set should be freed by \ref nvmlEventSetFree

 For Fermi &tm; or newer fully supported devices.
 @param set                                  Reference in which to return the event handle

 @return
         - \ref NVML_SUCCESS                 if the event has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a set is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlEventSetFree*/
    fn nvmlEventSetCreate(
        set: *mut cuda_types::nvml::nvmlEventSet_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Starts recording of events on a specified devices and add the events to specified \ref nvmlEventSet_t

 For Fermi &tm; or newer fully supported devices.
 ECC events are available only on ECC-enabled devices (see \ref nvmlDeviceGetTotalEccErrors)
 Power capping events are available only on Power Management enabled devices (see \ref nvmlDeviceGetPowerManagementMode)

 For Linux only.

 This call starts recording of events on specific device.
 All events that occurred before this call are not recorded.
 Checking if some event occurred can be done with \ref nvmlEventSetWait_v2

 If function reports NVML_ERROR_UNKNOWN, event set is in undefined state and should be freed.
 If function reports NVML_ERROR_NOT_SUPPORTED, event set can still be used. None of the requested eventTypes
     are registered in that case.

 @param device                               The identifier of the target device
 @param eventTypes                           Bitmask of \ref nvmlEventType to record
 @param set                                  Set to which add new event types

 @return
         - \ref NVML_SUCCESS                 if the event has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a eventTypes is invalid or \a set is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if the platform does not support this feature or some of requested event types
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlEventType
 @see nvmlDeviceGetSupportedEventTypes
 @see nvmlEventSetWait
 @see nvmlEventSetFree*/
    fn nvmlDeviceRegisterEvents(
        device: cuda_types::nvml::nvmlDevice_t,
        eventTypes: ::core::ffi::c_ulonglong,
        set: cuda_types::nvml::nvmlEventSet_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns information about events supported on device

 For Fermi &tm; or newer fully supported devices.

 Events are not supported on Windows. So this function returns an empty mask in \a eventTypes on Windows.

 @param device                               The identifier of the target device
 @param eventTypes                           Reference in which to return bitmask of supported events

 @return
         - \ref NVML_SUCCESS                 if the eventTypes has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a eventType is NULL
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlEventType
 @see nvmlDeviceRegisterEvents*/
    fn nvmlDeviceGetSupportedEventTypes(
        device: cuda_types::nvml::nvmlDevice_t,
        eventTypes: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Waits on events and delivers events

 For Fermi &tm; or newer fully supported devices.

 If some events are ready to be delivered at the time of the call, function returns immediately.
 If there are no events ready to be delivered, function sleeps till event arrives
 but not longer than specified timeout. This function in certain conditions can return before
 specified timeout passes (e.g. when interrupt arrives)

 On Windows, in case of Xid error, the function returns the most recent Xid error type seen by the system.
 If there are multiple Xid errors generated before nvmlEventSetWait is invoked then the last seen Xid error
 type is returned for all Xid error events.

 On Linux, every Xid error event would return the associated event data and other information if applicable.

 In MIG mode, if device handle is provided, the API reports all the events for the available instances,
 only if the caller has appropriate privileges. In absence of required privileges, only the events which
 affect all the instances (i.e. whole device) are reported.

 This API does not currently support per-instance event reporting using MIG device handles.

 @param set                                  Reference to set of events to wait on
 @param data                                 Reference in which to return event data
 @param timeoutms                            Maximum amount of wait time in milliseconds for registered event

 @return
         - \ref NVML_SUCCESS                 if the data has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a data is NULL
         - \ref NVML_ERROR_TIMEOUT           if no event arrived in specified timeout or interrupt arrived
         - \ref NVML_ERROR_GPU_IS_LOST       if a GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlEventType
 @see nvmlDeviceRegisterEvents*/
    fn nvmlEventSetWait_v2(
        set: cuda_types::nvml::nvmlEventSet_t,
        data: *mut cuda_types::nvml::nvmlEventData_t,
        timeoutms: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Releases events in the set

 For Fermi &tm; or newer fully supported devices.

 @param set                                  Reference to events to be released

 @return
         - \ref NVML_SUCCESS                 if the event has been successfully released
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlDeviceRegisterEvents*/
    fn nvmlEventSetFree(
        set: cuda_types::nvml::nvmlEventSet_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlSystemEventSetCreate(
        request: *mut cuda_types::nvml::nvmlSystemEventSetCreateRequest_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Releases system event set

 For Fermi &tm; or newer fully supported devices.

 @param set                                  Reference to nvmlSystemEventSetFreeRequest_t

 @return
         - \ref NVML_SUCCESS                         if the event has been set
         - \ref NVML_ERROR_UNINITIALIZED             if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          if request is NULL
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH for unsupported version
         - \ref NVML_ERROR_UNKNOWN                   on any unexpected error

 @see nvmlDeviceRegisterEvents*/
    fn nvmlSystemEventSetFree(
        request: *mut cuda_types::nvml::nvmlSystemEventSetFreeRequest_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Starts recording of events on system and add the events to specified \ref nvmlSystemEventSet_t

 For Linux only.

 This call starts recording of events on specific device.
 All events that occurred before this call are not recorded.
 Checking if some event occurred can be done with \ref nvmlSystemEventSetWait

 If function reports NVML_ERROR_UNKNOWN, event set is in undefined state and should be freed.
 If function reports NVML_ERROR_NOT_SUPPORTED, event set can still be used. None of the requested eventTypes
     are registered in that case.

 @param request                              Reference to the struct nvmlSystemRegisterEventRequest_t

 @return
         - \ref NVML_SUCCESS                         if the event has been set
         - \ref NVML_ERROR_UNINITIALIZED             if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          if request is NULL
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH for unsupported version
         - \ref NVML_ERROR_UNKNOWN                   on any unexpected error

 @see nvmlSystemEventType
 @see nvmlSystemEventSetWait
 @see nvmlEventSetFree*/
    fn nvmlSystemRegisterEvents(
        request: *mut cuda_types::nvml::nvmlSystemRegisterEventRequest_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Waits on system events and delivers events

 For Fermi &tm; or newer fully supported devices.

 If some events are ready to be delivered at the time of the call, function returns immediately.
 If there are no events ready to be delivered, function sleeps till event arrives
 but not longer than specified timeout. This function in certain conditions can return before
 specified timeout passes (e.g. when interrupt arrives)

 if the return request->numEvent equals to request->dataSize, there might be outstanding
 event, it is recommended to call nvmlSystemEventSetWait again to query all the events.

 @param request                              Reference in which to nvmlSystemEventSetWaitRequest_t

 @return
         - \ref NVML_SUCCESS                         if the event has been set
         - \ref NVML_ERROR_UNINITIALIZED             if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          if request is NULL
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH for unsupported version
         - \ref NVML_ERROR_TIMEOUT                   if no event notification after timeoutms
         - \ref NVML_ERROR_UNKNOWN                   on any unexpected error

 @see nvmlSystemEventType
 @see nvmlSystemRegisterEvents*/
    fn nvmlSystemEventSetWait(
        request: *mut cuda_types::nvml::nvmlSystemEventSetWaitRequest_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Modify the drain state of a GPU.  This method forces a GPU to no longer accept new incoming requests.
 Any new NVML process will no longer see this GPU.  Persistence mode for this GPU must be turned off before
 this call is made.
 Must be called as administrator.
 For Linux only.

 For Pascal &tm; or newer fully supported devices.
 Some Kepler devices supported.

 @param pciInfo                              The PCI address of the GPU drain state to be modified
 @param newState                             The drain state that should be entered, see \ref nvmlEnableState_t

 @return
         - \ref NVML_SUCCESS                 if counters were successfully reset
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a nvmlIndex or \a newState is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_NO_PERMISSION     if the calling process has insufficient permissions to perform operation
         - \ref NVML_ERROR_IN_USE            if the device has persistence mode turned on
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceModifyDrainState(
        pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
        newState: cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Query the drain state of a GPU.  This method is used to check if a GPU is in a currently draining
 state.
 For Linux only.

 For Pascal &tm; or newer fully supported devices.
 Some Kepler devices supported.

 @param pciInfo                              The PCI address of the GPU drain state to be queried
 @param currentState                         The current drain state for this GPU, see \ref nvmlEnableState_t

 @return
         - \ref NVML_SUCCESS                 if counters were successfully reset
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a nvmlIndex or \a currentState is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceQueryDrainState(
        pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
        currentState: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** This method will remove the specified GPU from the view of both NVML and the NVIDIA kernel driver
 as long as no other processes are attached. If other processes are attached, this call will return
 NVML_ERROR_IN_USE and the GPU will be returned to its original "draining" state. Note: the
 only situation where a process can still be attached after nvmlDeviceModifyDrainState() is called
 to initiate the draining state is if that process was using, and is still using, a GPU before the
 call was made. Also note, persistence mode counts as an attachment to the GPU thus it must be disabled
 prior to this call.

 For long-running NVML processes please note that this will change the enumeration of current GPUs.
 For example, if there are four GPUs present and GPU1 is removed, the new enumeration will be 0-2.
 Also, device handles after the removed GPU will not be valid and must be re-established.
 Must be run as administrator.
 For Linux only.

 For Pascal &tm; or newer fully supported devices.
 Some Kepler devices supported.

 @param pciInfo                              The PCI address of the GPU to be removed
 @param gpuState                             Whether the GPU is to be removed, from the OS
                                             see \ref nvmlDetachGpuState_t
 @param linkState                            Requested upstream PCIe link state, see \ref nvmlPcieLinkState_t

 @return
         - \ref NVML_SUCCESS                 if counters were successfully reset
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a nvmlIndex is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the device doesn't support this feature
         - \ref NVML_ERROR_IN_USE            if the device is still in use and cannot be removed*/
    fn nvmlDeviceRemoveGpu_v2(
        pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
        gpuState: cuda_types::nvml::nvmlDetachGpuState_t,
        linkState: cuda_types::nvml::nvmlPcieLinkState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Request the OS and the NVIDIA kernel driver to rediscover a portion of the PCI subsystem looking for GPUs that
 were previously removed. The portion of the PCI tree can be narrowed by specifying a domain, bus, and device.
 If all are zeroes then the entire PCI tree will be searched.  Please note that for long-running NVML processes
 the enumeration will change based on how many GPUs are discovered and where they are inserted in bus order.

 In addition, all newly discovered GPUs will be initialized and their ECC scrubbed which may take several seconds
 per GPU. Also, all device handles are no longer guaranteed to be valid post discovery.

 Must be run as administrator.
 For Linux only.

 For Pascal &tm; or newer fully supported devices.
 Some Kepler devices supported.

 @param pciInfo                              The PCI tree to be searched.  Only the domain, bus, and device
                                             fields are used in this call.

 @return
         - \ref NVML_SUCCESS                 if counters were successfully reset
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a pciInfo is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if the operating system does not support this feature
         - \ref NVML_ERROR_OPERATING_SYSTEM  if the operating system is denying this feature
         - \ref NVML_ERROR_NO_PERMISSION     if the calling process has insufficient permissions to perform operation
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceDiscoverGpus(
        pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Request values for a list of fields for a device. This API allows multiple fields to be queried at once.
 If any of the underlying fieldIds are populated by the same driver call, the results for those field IDs
 will be populated from a single call rather than making a driver call for each fieldId.

 @param device                               The device handle of the GPU to request field values for
 @param valuesCount                          Number of entries in values that should be retrieved
 @param values                               Array of \a valuesCount structures to hold field values.
                                             Each value's fieldId must be populated prior to this call

 @return
         - \ref NVML_SUCCESS                 if any values in \a values were populated. Note that you must
                                             check the nvmlReturn field of each value for each individual
                                             status
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a values is NULL*/
    fn nvmlDeviceGetFieldValues(
        device: cuda_types::nvml::nvmlDevice_t,
        valuesCount: ::core::ffi::c_int,
        values: *mut cuda_types::nvml::nvmlFieldValue_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Clear values for a list of fields for a device. This API allows multiple fields to be cleared at once.

 @param device                               The device handle of the GPU to request field values for
 @param valuesCount                          Number of entries in values that should be cleared
 @param values                               Array of \a valuesCount structures to hold field values.
                                             Each value's fieldId must be populated prior to this call

 @return
         - \ref NVML_SUCCESS                 if any values in \a values were cleared. Note that you must
                                             check the nvmlReturn field of each value for each individual
                                             status
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid or \a values is NULL*/
    fn nvmlDeviceClearFieldValues(
        device: cuda_types::nvml::nvmlDevice_t,
        valuesCount: ::core::ffi::c_int,
        values: *mut cuda_types::nvml::nvmlFieldValue_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** This method is used to get the virtualization mode corresponding to the GPU.

 For Kepler &tm; or newer fully supported devices.

 @param device                    Identifier of the target device
 @param pVirtualMode              Reference to virtualization mode. One of NVML_GPU_VIRTUALIZATION_?

 @return
         - \ref NVML_SUCCESS                  if \a pVirtualMode is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a device is invalid or \a pVirtualMode is NULL
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetVirtualizationMode(
        device: cuda_types::nvml::nvmlDevice_t,
        pVirtualMode: *mut cuda_types::nvml::nvmlGpuVirtualizationMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Queries if SR-IOV host operation is supported on a vGPU supported device.

 Checks whether SR-IOV host capability is supported by the device and the
 driver, and indicates device is in SR-IOV mode if both of these conditions
 are true.

 @param device                                The identifier of the target device
 @param pHostVgpuMode                         Reference in which to return the current vGPU mode

 @return
         - \ref NVML_SUCCESS                  if device's vGPU mode has been successfully retrieved
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a device handle is 0 or \a pVgpuMode is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED      if \a device doesn't support this feature.
         - \ref NVML_ERROR_UNKNOWN            if any unexpected error occurred*/
    fn nvmlDeviceGetHostVgpuMode(
        device: cuda_types::nvml::nvmlDevice_t,
        pHostVgpuMode: *mut cuda_types::nvml::nvmlHostVgpuMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** This method is used to set the virtualization mode corresponding to the GPU.

 For Kepler &tm; or newer fully supported devices.

 @param device                    Identifier of the target device
 @param virtualMode               virtualization mode. One of NVML_GPU_VIRTUALIZATION_?

 @return
         - \ref NVML_SUCCESS                  if \a virtualMode is set
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a device is invalid or \a virtualMode is NULL
         - \ref NVML_ERROR_GPU_IS_LOST        if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_SUPPORTED      if setting of virtualization mode is not supported.
         - \ref NVML_ERROR_NO_PERMISSION      if setting of virtualization mode is not allowed for this client.*/
    fn nvmlDeviceSetVirtualizationMode(
        device: cuda_types::nvml::nvmlDevice_t,
        virtualMode: cuda_types::nvml::nvmlGpuVirtualizationMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the vGPU heterogeneous mode for the device.

 When in heterogeneous mode, a vGPU can concurrently host timesliced vGPUs with differing framebuffer sizes.

 On successful return, the function returns \a pHeterogeneousMode->mode with the current vGPU heterogeneous mode.
 \a pHeterogeneousMode->version is the version number of the structure nvmlVgpuHeterogeneousMode_t, the caller should
 set the correct version number to retrieve the vGPU heterogeneous mode.
 \a pHeterogeneousMode->mode can either be \ref NVML_FEATURE_ENABLED or \ref NVML_FEATURE_DISABLED.

 @param device                               The identifier of the target device
 @param pHeterogeneousMode                   Pointer to the caller-provided structure of nvmlVgpuHeterogeneousMode_t

 @return
         - \ref NVML_SUCCESS                          Upon success
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a device is invalid or \a pHeterogeneousMode is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED              If MIG is enabled or \a device doesn't support this feature
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pHeterogeneousMode is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlDeviceGetVgpuHeterogeneousMode(
        device: cuda_types::nvml::nvmlDevice_t,
        pHeterogeneousMode: *mut cuda_types::nvml::nvmlVgpuHeterogeneousMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Enable or disable vGPU heterogeneous mode for the device.

 When in heterogeneous mode, a vGPU can concurrently host timesliced vGPUs with differing framebuffer sizes.

 API would return an appropriate error code upon unsuccessful activation. For example, the heterogeneous mode
 set will fail with error \ref NVML_ERROR_IN_USE if any vGPU instance is active on the device. The caller of this API
 is expected to shutdown the vGPU VMs and retry setting the \a mode.
 On KVM platform, setting heterogeneous mode is allowed, if no MDEV device is created on the device, else will fail
 with same error \ref NVML_ERROR_IN_USE.
 On successful return, the function updates the vGPU heterogeneous mode with the user provided \a pHeterogeneousMode->mode.
 \a pHeterogeneousMode->version is the version number of the structure nvmlVgpuHeterogeneousMode_t, the caller should
 set the correct version number to set the vGPU heterogeneous mode.

 @param device                               Identifier of the target device
 @param pHeterogeneousMode                   Pointer to the caller-provided structure of nvmlVgpuHeterogeneousMode_t

 @return
         - \ref NVML_SUCCESS                          Upon success
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a device or \a pHeterogeneousMode is NULL or \a pHeterogeneousMode->mode is invalid
         - \ref NVML_ERROR_IN_USE                     If the \a device is in use
         - \ref NVML_ERROR_NO_PERMISSION              If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_NOT_SUPPORTED              If MIG is enabled or \a device doesn't support this feature
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pHeterogeneousMode is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlDeviceSetVgpuHeterogeneousMode(
        device: cuda_types::nvml::nvmlDevice_t,
        pHeterogeneousMode: *const cuda_types::nvml::nvmlVgpuHeterogeneousMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Query the placement ID of active vGPU instance.

 When in vGPU heterogeneous mode, this function returns a valid placement ID as \a pPlacement->placementId
 else NVML_INVALID_VGPU_PLACEMENT_ID is returned.
 \a pPlacement->version is the version number of the structure nvmlVgpuPlacementId_t, the caller should
 set the correct version number to get placement id of the vGPU instance \a vgpuInstance.

 @param vgpuInstance                         Identifier of the target vGPU instance
 @param pPlacement                           Pointer to vGPU placement ID structure \a nvmlVgpuPlacementId_t

 @return
         - \ref NVML_SUCCESS                          If information is successfully retrieved
         - \ref NVML_ERROR_NOT_FOUND                  If \a vgpuInstance does not match a valid active vGPU instance
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a vgpuInstance is invalid or \a pPlacement is NULL
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pPlacement is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlVgpuInstanceGetPlacementId(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        pPlacement: *mut cuda_types::nvml::nvmlVgpuPlacementId_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Query the supported vGPU placement ID of the vGPU type.

 The function returns an array of supported vGPU placement IDs for the specified vGPU type ID in the buffer provided
 by the caller at \a pPlacementList->placementIds. The required memory for the placementIds array must be allocated
 based on the maximum number of vGPU type instances, which is retrievable through \ref nvmlVgpuTypeGetMaxInstances().
 If the provided count by the caller is insufficient, the function will return NVML_ERROR_INSUFFICIENT_SIZE along with
 the number of required entries in \a pPlacementList->count. The caller should then reallocate a buffer with the size
 of pPlacementList->count * sizeof(pPlacementList->placementIds) and invoke the function again.

 To obtain a list of homogeneous placement IDs, the caller needs to set \a pPlacementList->mode to NVML_VGPU_PGPU_HOMOGENEOUS_MODE.
 For heterogeneous placement IDs, \a pPlacementList->mode should be set to NVML_VGPU_PGPU_HETEROGENEOUS_MODE.
 By default, a list of heterogeneous placement IDs is returned.

 @param device                               Identifier of the target device
 @param vgpuTypeId                           Handle to vGPU type. The vGPU type ID
 @param pPlacementList                       Pointer to the vGPU placement structure \a nvmlVgpuPlacementList_t

 @return
         - \ref NVML_SUCCESS                          Upon success
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a device or \a vgpuTypeId is invalid or \a pPlacementList is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED              If \a device or \a vgpuTypeId isn't supported
         - \ref NVML_ERROR_NO_PERMISSION              If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pPlacementList is invalid
         - \ref NVML_ERROR_INSUFFICIENT_SIZE          If the buffer is small, element count is returned in \a pPlacementList->count
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlDeviceGetVgpuTypeSupportedPlacements(
        device: cuda_types::nvml::nvmlDevice_t,
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        pPlacementList: *mut cuda_types::nvml::nvmlVgpuPlacementList_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Query the creatable vGPU placement ID of the vGPU type.

 An array of creatable vGPU placement IDs for the vGPU type ID indicated by \a vgpuTypeId is returned in the
 caller-supplied buffer of \a pPlacementList->placementIds. Memory needed for the placementIds array should be
 allocated based on maximum instances of a vGPU type which can be queried via \ref nvmlVgpuTypeGetMaxInstances().
 If the provided count by the caller is insufficient, the function will return NVML_ERROR_INSUFFICIENT_SIZE along with
 the number of required entries in \a pPlacementList->count. The caller should then reallocate a buffer with the size
 of pPlacementList->count * sizeof(pPlacementList->placementIds) and invoke the function again.

 The creatable vGPU placement IDs may differ over time, as there may be restrictions on what type of vGPU the
 vGPU instance is running.

 @param device                               The identifier of the target device
 @param vgpuTypeId                           Handle to vGPU type. The vGPU type ID
 @param pPlacementList                       Pointer to the list of vGPU placement structure \a nvmlVgpuPlacementList_t

 @return
         - \ref NVML_SUCCESS                          Upon success
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a device or \a vgpuTypeId is invalid or \a pPlacementList is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED              If MIG is enabled or \a device or \a vgpuTypeId isn't supported
         - \ref NVML_ERROR_NO_PERMISSION              If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pPlacementList is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlDeviceGetVgpuTypeCreatablePlacements(
        device: cuda_types::nvml::nvmlDevice_t,
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        pPlacementList: *mut cuda_types::nvml::nvmlVgpuPlacementList_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the static GSP heap size of the vGPU type in bytes

 @param vgpuTypeId                           Handle to vGPU type
 @param gspHeapSize                          Reference to return the GSP heap size value
 @return
         - \ref NVML_SUCCESS                 Successful completion
         - \ref NVML_ERROR_UNINITIALIZED     If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a vgpuTypeId is invalid, or \a gspHeapSize is NULL
         - \ref NVML_ERROR_UNKNOWN           On any unexpected error*/
    fn nvmlVgpuTypeGetGspHeapSize(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        gspHeapSize: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the static framebuffer reservation of the vGPU type in bytes

 @param vgpuTypeId                           Handle to vGPU type
 @param fbReservation                        Reference to return the framebuffer reservation
 @return
         - \ref NVML_SUCCESS                 Successful completion
         - \ref NVML_ERROR_UNINITIALIZED     If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a vgpuTypeId is invalid, or \a fbReservation is NULL
         - \ref NVML_ERROR_UNKNOWN           On any unexpected error*/
    fn nvmlVgpuTypeGetFbReservation(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        fbReservation: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the currently used runtime state size of the vGPU instance

 This size represents the maximum in-memory data size utilized by a vGPU instance during standard operation.
 This measurement is exclusive of frame buffer (FB) data size assigned to the vGPU instance.

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuInstance                         Identifier of the target vGPU instance
 @param pState                               Pointer to the vGPU runtime state's structure \a nvmlVgpuRuntimeState_t

 @return
         - \ref NVML_SUCCESS                          If information is successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED              If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a vgpuInstance is invalid, or \a pState is NULL
         - \ref NVML_ERROR_NOT_FOUND                  If \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pState is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlVgpuInstanceGetRuntimeStateSize(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        pState: *mut cuda_types::nvml::nvmlVgpuRuntimeState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the desirable vGPU capability of a device

 Refer to the \a nvmlDeviceVgpuCapability_t structure for the specific capabilities that can be set.
 See \ref nvmlEnableState_t for available state.

 @param device                               The identifier of the target device
 @param capability                           Specifies the \a nvmlDeviceVgpuCapability_t to be set
 @param state                                The target capability mode

 @return
      - \ref NVML_SUCCESS                    Successful completion
      - \ref NVML_ERROR_UNINITIALIZED        If the library has not been successfully initialized
      - \ref NVML_ERROR_INVALID_ARGUMENT     If \a device is invalid, or \a capability is invalid, or \a state is invalid
      - \ref NVML_ERROR_NOT_SUPPORTED        The API is not supported in current state, or \a device not in vGPU mode
      - \ref NVML_ERROR_UNKNOWN              On any unexpected error*/
    fn nvmlDeviceSetVgpuCapabilities(
        device: cuda_types::nvml::nvmlDevice_t,
        capability: cuda_types::nvml::nvmlDeviceVgpuCapability_t,
        state: cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the vGPU Software licensable features.

 Identifies whether the system supports vGPU Software Licensing. If it does, return the list of licensable feature(s)
 and their current license status.

 @param device                    Identifier of the target device
 @param pGridLicensableFeatures   Pointer to structure in which vGPU software licensable features are returned

 @return
         - \ref NVML_SUCCESS                 if licensable features are successfully retrieved
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a pGridLicensableFeatures is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetGridLicensableFeatures_v4(
        device: cuda_types::nvml::nvmlDevice_t,
        pGridLicensableFeatures: *mut cuda_types::nvml::nvmlGridLicensableFeatures_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the requested vGPU driver capability.

 Refer to the \a nvmlVgpuDriverCapability_t structure for the specific capabilities that can be queried.
 The return value in \a capResult should be treated as a boolean, with a non-zero value indicating that the capability
 is supported.

 For Maxwell &tm; or newer fully supported devices.

 @param capability      Specifies the \a nvmlVgpuDriverCapability_t to be queried
 @param capResult       A boolean for the queried capability indicating that feature is supported

 @return
      - \ref NVML_SUCCESS                      successful completion
      - \ref NVML_ERROR_UNINITIALIZED          if the library has not been successfully initialized
      - \ref NVML_ERROR_INVALID_ARGUMENT       if \a capability is invalid, or \a capResult is NULL
      - \ref NVML_ERROR_NOT_SUPPORTED          the API is not supported in current state or \a devices not in vGPU mode
      - \ref NVML_ERROR_UNKNOWN                on any unexpected error*/
    fn nvmlGetVgpuDriverCapabilities(
        capability: cuda_types::nvml::nvmlVgpuDriverCapability_t,
        capResult: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the requested vGPU capability for GPU.

 Refer to the \a nvmlDeviceVgpuCapability_t structure for the specific capabilities that can be queried.
 The return value in \a capResult reports a non-zero value indicating that the capability
 is supported, and also reports the capability's data based on the queried capability.

 For Maxwell &tm; or newer fully supported devices.

 @param device     The identifier of the target device
 @param capability Specifies the \a nvmlDeviceVgpuCapability_t to be queried
 @param capResult  Specifies that the queried capability is supported, and also returns capability's data

 @return
      - \ref NVML_SUCCESS                      successful completion
      - \ref NVML_ERROR_UNINITIALIZED          if the library has not been successfully initialized
      - \ref NVML_ERROR_INVALID_ARGUMENT       if \a device is invalid, or \a capability is invalid, or \a capResult is NULL
      - \ref NVML_ERROR_NOT_SUPPORTED          the API is not supported in current state or \a device not in vGPU mode
      - \ref NVML_ERROR_UNKNOWN                on any unexpected error*/
    fn nvmlDeviceGetVgpuCapabilities(
        device: cuda_types::nvml::nvmlDevice_t,
        capability: cuda_types::nvml::nvmlDeviceVgpuCapability_t,
        capResult: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the supported vGPU types on a physical GPU (device).

 An array of supported vGPU types for the physical GPU indicated by \a device is returned in the caller-supplied buffer
 pointed at by \a vgpuTypeIds. The element count of nvmlVgpuTypeId_t array is passed in \a vgpuCount, and \a vgpuCount
 is used to return the number of vGPU types written to the buffer.

 If the supplied buffer is not large enough to accommodate the vGPU type array, the function returns
 NVML_ERROR_INSUFFICIENT_SIZE, with the element count of nvmlVgpuTypeId_t array required in \a vgpuCount.
 To query the number of vGPU types supported for the GPU, call this function with *vgpuCount = 0.
 The code will return NVML_ERROR_INSUFFICIENT_SIZE, or NVML_SUCCESS if no vGPU types are supported.

 @param device                   The identifier of the target device
 @param vgpuCount                Pointer to caller-supplied array size, and returns number of vGPU types
 @param vgpuTypeIds              Pointer to caller-supplied array in which to return list of vGPU types

 @return
         - \ref NVML_SUCCESS                      successful completion
         - \ref NVML_ERROR_INSUFFICIENT_SIZE      \a vgpuTypeIds buffer is too small, array element count is returned in \a vgpuCount
         - \ref NVML_ERROR_INVALID_ARGUMENT       if \a vgpuCount is NULL or \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED          if vGPU is not supported by the device
         - \ref NVML_ERROR_UNKNOWN                on any unexpected error*/
    fn nvmlDeviceGetSupportedVgpus(
        device: cuda_types::nvml::nvmlDevice_t,
        vgpuCount: *mut ::core::ffi::c_uint,
        vgpuTypeIds: *mut cuda_types::nvml::nvmlVgpuTypeId_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the currently creatable vGPU types on a physical GPU (device).

 An array of creatable vGPU types for the physical GPU indicated by \a device is returned in the caller-supplied buffer
 pointed at by \a vgpuTypeIds. The element count of nvmlVgpuTypeId_t array is passed in \a vgpuCount, and \a vgpuCount
 is used to return the number of vGPU types written to the buffer.

 The creatable vGPU types for a device may differ over time, as there may be restrictions on what type of vGPU types
 can concurrently run on a device.  For example, if only one vGPU type is allowed at a time on a device, then the creatable
 list will be restricted to whatever vGPU type is already running on the device.

 If the supplied buffer is not large enough to accommodate the vGPU type array, the function returns
 NVML_ERROR_INSUFFICIENT_SIZE, with the element count of nvmlVgpuTypeId_t array required in \a vgpuCount.
 To query the number of vGPU types that can be created for the GPU, call this function with *vgpuCount = 0.
 The code will return NVML_ERROR_INSUFFICIENT_SIZE, or NVML_SUCCESS if no vGPU types are creatable.

 @param device                   The identifier of the target device
 @param vgpuCount                Pointer to caller-supplied array size, and returns number of vGPU types
 @param vgpuTypeIds              Pointer to caller-supplied array in which to return list of vGPU types

 @return
         - \ref NVML_SUCCESS                      successful completion
         - \ref NVML_ERROR_INSUFFICIENT_SIZE      \a vgpuTypeIds buffer is too small, array element count is returned in \a vgpuCount
         - \ref NVML_ERROR_INVALID_ARGUMENT       if \a vgpuCount is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED          if vGPU is not supported by the device
         - \ref NVML_ERROR_UNKNOWN                on any unexpected error*/
    fn nvmlDeviceGetCreatableVgpus(
        device: cuda_types::nvml::nvmlDevice_t,
        vgpuCount: *mut ::core::ffi::c_uint,
        vgpuTypeIds: *mut cuda_types::nvml::nvmlVgpuTypeId_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the class of a vGPU type. It will not exceed 64 characters in length (including the NUL terminator).
 See \ref nvmlConstants::NVML_DEVICE_NAME_BUFFER_SIZE.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param vgpuTypeClass            Pointer to string array to return class in
 @param size                     Size of string

 @return
         - \ref NVML_SUCCESS                   successful completion
         - \ref NVML_ERROR_INVALID_ARGUMENT    if \a vgpuTypeId is invalid, or \a vgpuTypeClass is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE   if \a size is too small
         - \ref NVML_ERROR_UNKNOWN             on any unexpected error*/
    fn nvmlVgpuTypeGetClass(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        vgpuTypeClass: *mut ::core::ffi::c_char,
        size: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the vGPU type name.

 The name is an alphanumeric string that denotes a particular vGPU, e.g. GRID M60-2Q. It will not
 exceed 64 characters in length (including the NUL terminator).  See \ref
 nvmlConstants::NVML_DEVICE_NAME_BUFFER_SIZE.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param vgpuTypeName             Pointer to buffer to return name
 @param size                     Size of buffer

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a name is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a size is too small
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetName(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        vgpuTypeName: *mut ::core::ffi::c_char,
        size: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the GPU Instance Profile ID for the given vGPU type ID.
 The API will return a valid GPU Instance Profile ID for the MIG capable vGPU types, else INVALID_GPU_INSTANCE_PROFILE_ID is
 returned.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param gpuInstanceProfileId     GPU Instance Profile ID

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_NOT_SUPPORTED     if \a device is not in vGPU Host virtualization mode
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a gpuInstanceProfileId is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetGpuInstanceProfileId(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        gpuInstanceProfileId: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the device ID of a vGPU type.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param deviceID                 Device ID and vendor ID of the device contained in single 32 bit value
 @param subsystemID              Subsystem ID and subsystem vendor ID of the device contained in single 32 bit value

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a deviceId or \a subsystemID are NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetDeviceID(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        deviceID: *mut ::core::ffi::c_ulonglong,
        subsystemID: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the vGPU framebuffer size in bytes.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param fbSize                   Pointer to framebuffer size in bytes

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a fbSize is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetFramebufferSize(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        fbSize: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve count of vGPU's supported display heads.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param numDisplayHeads          Pointer to number of display heads

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a numDisplayHeads is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetNumDisplayHeads(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        numDisplayHeads: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve vGPU display head's maximum supported resolution.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param displayIndex             Zero-based index of display head
 @param xdim                     Pointer to maximum number of pixels in X dimension
 @param ydim                     Pointer to maximum number of pixels in Y dimension

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a xdim or \a ydim are NULL, or \a displayIndex
                                             is out of range.
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetResolution(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        displayIndex: ::core::ffi::c_uint,
        xdim: *mut ::core::ffi::c_uint,
        ydim: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve license requirements for a vGPU type

 The license type and version required to run the specified vGPU type is returned as an alphanumeric string, in the form
 "<license name>,<version>", for example "GRID-Virtual-PC,2.0". If a vGPU is runnable with* more than one type of license,
 the licenses are delimited by a semicolon, for example "GRID-Virtual-PC,2.0;GRID-Virtual-WS,2.0;GRID-Virtual-WS-Ext,2.0".

 The total length of the returned string will not exceed 128 characters, including the NUL terminator.
 See \ref nvmlVgpuConstants::NVML_GRID_LICENSE_BUFFER_SIZE.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param vgpuTypeLicenseString    Pointer to buffer to return license info
 @param size                     Size of \a vgpuTypeLicenseString buffer

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a vgpuTypeLicenseString is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a size is too small
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetLicense(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        vgpuTypeLicenseString: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the static frame rate limit value of the vGPU type

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param frameRateLimit           Reference to return the frame rate limit value
 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_NOT_SUPPORTED     if frame rate limiter is turned off for the vGPU type
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a frameRateLimit is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetFrameRateLimit(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        frameRateLimit: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the maximum number of vGPU instances creatable on a device for given vGPU type

 For Kepler &tm; or newer fully supported devices.

 @param device                   The identifier of the target device
 @param vgpuTypeId               Handle to vGPU type
 @param vgpuInstanceCount        Pointer to get the max number of vGPU instances
                                 that can be created on a deicve for given vgpuTypeId
 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid or is not supported on target device,
                                             or \a vgpuInstanceCount is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetMaxInstances(
        device: cuda_types::nvml::nvmlDevice_t,
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        vgpuInstanceCount: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the maximum number of vGPU instances supported per VM for given vGPU type

 For Kepler &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param vgpuInstanceCountPerVm   Pointer to get the max number of vGPU instances supported per VM for given \a vgpuTypeId
 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a vgpuInstanceCountPerVm is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetMaxInstancesPerVm(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        vgpuInstanceCountPerVm: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the BAR1 info for given vGPU type.

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuTypeId               Handle to vGPU type
 @param bar1Info                 Pointer to the vGPU type BAR1 information structure \a nvmlVgpuTypeBar1Info_t

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a bar1Info is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetBAR1Info(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        bar1Info: *mut cuda_types::nvml::nvmlVgpuTypeBar1Info_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the active vGPU instances on a device.

 An array of active vGPU instances is returned in the caller-supplied buffer pointed at by \a vgpuInstances. The
 array element count is passed in \a vgpuCount, and \a vgpuCount is used to return the number of vGPU instances
 written to the buffer.

 If the supplied buffer is not large enough to accommodate the vGPU instance array, the function returns
 NVML_ERROR_INSUFFICIENT_SIZE, with the element count of nvmlVgpuInstance_t array required in \a vgpuCount.
 To query the number of active vGPU instances, call this function with *vgpuCount = 0.  The code will return
 NVML_ERROR_INSUFFICIENT_SIZE, or NVML_SUCCESS if no vGPU Types are supported.

 For Kepler &tm; or newer fully supported devices.

 @param device                   The identifier of the target device
 @param vgpuCount                Pointer which passes in the array size as well as get
                                 back the number of types
 @param vgpuInstances            Pointer to array in which to return list of vGPU instances

 @return
         - \ref NVML_SUCCESS                  successful completion
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a device is invalid, or \a vgpuCount is NULL
         - \ref NVML_ERROR_INSUFFICIENT_SIZE  if \a size is too small
         - \ref NVML_ERROR_NOT_SUPPORTED      if vGPU is not supported by the device
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlDeviceGetActiveVgpus(
        device: cuda_types::nvml::nvmlDevice_t,
        vgpuCount: *mut ::core::ffi::c_uint,
        vgpuInstances: *mut cuda_types::nvml::nvmlVgpuInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the VM ID associated with a vGPU instance.

 The VM ID is returned as a string, not exceeding 80 characters in length (including the NUL terminator).
 See \ref nvmlConstants::NVML_DEVICE_UUID_BUFFER_SIZE.

 The format of the VM ID varies by platform, and is indicated by the type identifier returned in \a vmIdType.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuInstance             Identifier of the target vGPU instance
 @param vmId                     Pointer to caller-supplied buffer to hold VM ID
 @param size                     Size of buffer in bytes
 @param vmIdType                 Pointer to hold VM ID type

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vmId or \a vmIdType is NULL, or \a vgpuInstance is 0
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a size is too small
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetVmID(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        vmId: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
        vmIdType: *mut cuda_types::nvml::nvmlVgpuVmIdType_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the UUID of a vGPU instance.

 The UUID is a globally unique identifier associated with the vGPU, and is returned as a 5-part hexadecimal string,
 not exceeding 80 characters in length (including the NULL terminator).
 See \ref nvmlConstants::NVML_DEVICE_UUID_BUFFER_SIZE.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuInstance             Identifier of the target vGPU instance
 @param uuid                     Pointer to caller-supplied buffer to hold vGPU UUID
 @param size                     Size of buffer in bytes

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a uuid is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a size is too small
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetUUID(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        uuid: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the NVIDIA driver version installed in the VM associated with a vGPU.

 The version is returned as an alphanumeric string in the caller-supplied buffer \a version. The length of the version
 string will not exceed 80 characters in length (including the NUL terminator).
 See \ref nvmlConstants::NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE.

 nvmlVgpuInstanceGetVmDriverVersion() may be called at any time for a vGPU instance. The guest VM driver version is
 returned as "Not Available" if no NVIDIA driver is installed in the VM, or the VM has not yet booted to the point where the
 NVIDIA driver is loaded and initialized.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuInstance             Identifier of the target vGPU instance
 @param version                  Caller-supplied buffer to return driver version string
 @param length                   Size of \a version buffer

 @return
         - \ref NVML_SUCCESS                 if \a version has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetVmDriverVersion(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        version: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the framebuffer usage in bytes.

 Framebuffer usage is the amont of vGPU framebuffer memory that is currently in use by the VM.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuInstance             The identifier of the target instance
 @param fbUsage                  Pointer to framebuffer usage in bytes

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a fbUsage is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetFbUsage(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        fbUsage: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** @deprecated Use \ref nvmlVgpuInstanceGetLicenseInfo_v2.

 Retrieve the current licensing state of the vGPU instance.

 If the vGPU is currently licensed, \a licensed is set to 1, otherwise it is set to 0.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuInstance             Identifier of the target vGPU instance
 @param licensed                 Reference to return the licensing status

 @return
         - \ref NVML_SUCCESS                 if \a licensed has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a licensed is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetLicenseStatus(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        licensed: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the vGPU type of a vGPU instance.

 Returns the vGPU type ID of vgpu assigned to the vGPU instance.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuInstance             Identifier of the target vGPU instance
 @param vgpuTypeId               Reference to return the vgpuTypeId

 @return
         - \ref NVML_SUCCESS                 if \a vgpuTypeId has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a vgpuTypeId is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetType(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        vgpuTypeId: *mut cuda_types::nvml::nvmlVgpuTypeId_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the frame rate limit set for the vGPU instance.

 Returns the value of the frame rate limit set for the vGPU instance

 For Kepler &tm; or newer fully supported devices.

 @param vgpuInstance             Identifier of the target vGPU instance
 @param frameRateLimit           Reference to return the frame rate limit

 @return
         - \ref NVML_SUCCESS                 if \a frameRateLimit has been set
         - \ref NVML_ERROR_NOT_SUPPORTED     if frame rate limiter is turned off for the vGPU type
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a frameRateLimit is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetFrameRateLimit(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        frameRateLimit: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the current ECC mode of vGPU instance.

 @param vgpuInstance            The identifier of the target vGPU instance
 @param eccMode                 Reference in which to return the current ECC mode

 @return
         - \ref NVML_SUCCESS                 if the vgpuInstance's ECC mode has been successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a mode is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_NOT_SUPPORTED     if the vGPU doesn't support this feature
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetEccMode(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        eccMode: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the encoder capacity of a vGPU instance, as a percentage of maximum encoder capacity with valid values in the range 0-100.

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuInstance             Identifier of the target vGPU instance
 @param encoderCapacity          Reference to an unsigned int for the encoder capacity

 @return
         - \ref NVML_SUCCESS                 if \a encoderCapacity has been retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a encoderQueryType is invalid
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetEncoderCapacity(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        encoderCapacity: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set the encoder capacity of a vGPU instance, as a percentage of maximum encoder capacity with valid values in the range 0-100.

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuInstance             Identifier of the target vGPU instance
 @param encoderCapacity          Unsigned int for the encoder capacity value

 @return
         - \ref NVML_SUCCESS                 if \a encoderCapacity has been set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a encoderCapacity is out of range of 0-100.
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceSetEncoderCapacity(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        encoderCapacity: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the current encoder statistics of a vGPU Instance

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuInstance                      Identifier of the target vGPU instance
 @param sessionCount                      Reference to an unsigned int for count of active encoder sessions
 @param averageFps                        Reference to an unsigned int for trailing average FPS of all active sessions
 @param averageLatency                    Reference to an unsigned int for encode latency in microseconds

 @return
         - \ref NVML_SUCCESS                  if \a sessionCount, \a averageFps and \a averageLatency is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a sessionCount , or \a averageFps or \a averageLatency is NULL
                                              or \a vgpuInstance is 0.
         - \ref NVML_ERROR_NOT_FOUND          if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlVgpuInstanceGetEncoderStats(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        sessionCount: *mut ::core::ffi::c_uint,
        averageFps: *mut ::core::ffi::c_uint,
        averageLatency: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves information about all active encoder sessions on a vGPU Instance.

 An array of active encoder sessions is returned in the caller-supplied buffer pointed at by \a sessionInfo. The
 array element count is passed in \a sessionCount, and \a sessionCount is used to return the number of sessions
 written to the buffer.

 If the supplied buffer is not large enough to accommodate the active session array, the function returns
 NVML_ERROR_INSUFFICIENT_SIZE, with the element count of nvmlEncoderSessionInfo_t array required in \a sessionCount.
 To query the number of active encoder sessions, call this function with *sessionCount = 0. The code will return
 NVML_SUCCESS with number of active encoder sessions updated in *sessionCount.

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuInstance                      Identifier of the target vGPU instance
 @param sessionCount                      Reference to caller supplied array size, and returns
                                          the number of sessions.
 @param sessionInfo                       Reference to caller supplied array in which the list
                                          of session information us returned.

 @return
         - \ref NVML_SUCCESS                  if \a sessionInfo is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INSUFFICIENT_SIZE  if \a sessionCount is too small, array element count is
returned in \a sessionCount
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a sessionCount is NULL, or \a vgpuInstance is 0.
         - \ref NVML_ERROR_NOT_FOUND          if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlVgpuInstanceGetEncoderSessions(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        sessionCount: *mut ::core::ffi::c_uint,
        sessionInfo: *mut cuda_types::nvml::nvmlEncoderSessionInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the active frame buffer capture sessions statistics of a vGPU Instance

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuInstance                      Identifier of the target vGPU instance
 @param fbcStats                          Reference to nvmlFBCStats_t structure containing NvFBC stats

 @return
         - \ref NVML_SUCCESS                  if \a fbcStats is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a vgpuInstance is 0, or \a fbcStats is NULL
         - \ref NVML_ERROR_NOT_FOUND          if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlVgpuInstanceGetFBCStats(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        fbcStats: *mut cuda_types::nvml::nvmlFBCStats_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves information about active frame buffer capture sessions on a vGPU Instance.

 An array of active FBC sessions is returned in the caller-supplied buffer pointed at by \a sessionInfo. The
 array element count is passed in \a sessionCount, and \a sessionCount is used to return the number of sessions
 written to the buffer.

 If the supplied buffer is not large enough to accommodate the active session array, the function returns
 NVML_ERROR_INSUFFICIENT_SIZE, with the element count of nvmlFBCSessionInfo_t array required in \a sessionCount.
 To query the number of active FBC sessions, call this function with *sessionCount = 0.  The code will return
 NVML_SUCCESS with number of active FBC sessions updated in *sessionCount.

 For Maxwell &tm; or newer fully supported devices.

 @note hResolution, vResolution, averageFPS and averageLatency data for a FBC session returned in \a sessionInfo may
       be zero if there are no new frames captured since the session started.

 @param vgpuInstance                      Identifier of the target vGPU instance
 @param sessionCount                      Reference to caller supplied array size, and returns the number of sessions.
 @param sessionInfo                       Reference in which to return the session information

 @return
         - \ref NVML_SUCCESS                  if \a sessionInfo is fetched
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a vgpuInstance is 0, or \a sessionCount is NULL.
         - \ref NVML_ERROR_NOT_FOUND          if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_INSUFFICIENT_SIZE  if \a sessionCount is too small, array element count is returned in \a sessionCount
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlVgpuInstanceGetFBCSessions(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        sessionCount: *mut ::core::ffi::c_uint,
        sessionInfo: *mut cuda_types::nvml::nvmlFBCSessionInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the GPU Instance ID for the given vGPU Instance.
 The API will return a valid GPU Instance ID for MIG backed vGPU Instance, else INVALID_GPU_INSTANCE_ID is returned.

 For Kepler &tm; or newer fully supported devices.

 @param vgpuInstance                      Identifier of the target vGPU instance
 @param gpuInstanceId                     GPU Instance ID

 @return
         - \ref NVML_SUCCESS                  successful completion
         - \ref NVML_ERROR_UNINITIALIZED      if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a vgpuInstance is 0, or \a gpuInstanceId is NULL.
         - \ref NVML_ERROR_NOT_FOUND          if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN            on any unexpected error*/
    fn nvmlVgpuInstanceGetGpuInstanceId(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        gpuInstanceId: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the PCI Id of the given vGPU Instance i.e. the PCI Id of the GPU as seen inside the VM.

 The vGPU PCI id is returned as "00000000:00:00.0" if NVIDIA driver is not installed on the vGPU instance.

 @param vgpuInstance                         Identifier of the target vGPU instance
 @param vgpuPciId                            Caller-supplied buffer to return vGPU PCI Id string
 @param length                               Size of the vgpuPciId buffer

 @return
         - \ref NVML_SUCCESS                 if vGPU PCI Id is sucessfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a vgpuPciId is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_DRIVER_NOT_LOADED if NVIDIA driver is not running on the vGPU instance
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a length is too small, \a length is set to required length
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetGpuPciId(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        vgpuPciId: *mut ::core::ffi::c_char,
        length: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the requested capability for a given vGPU type. Refer to the \a nvmlVgpuCapability_t structure
 for the specific capabilities that can be queried. The return value in \a capResult should be treated as
 a boolean, with a non-zero value indicating that the capability is supported.

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuTypeId                           Handle to vGPU type
 @param capability                           Specifies the \a nvmlVgpuCapability_t to be queried
 @param capResult                            A boolean for the queried capability indicating that feature is supported

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuTypeId is invalid, or \a capability is invalid, or \a capResult is NULL
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuTypeGetCapabilities(
        vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
        capability: cuda_types::nvml::nvmlVgpuCapability_t,
        capResult: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the MDEV UUID of a vGPU instance.

 The MDEV UUID is a globally unique identifier of the mdev device assigned to the VM, and is returned as a 5-part hexadecimal string,
 not exceeding 80 characters in length (including the NULL terminator).
 MDEV UUID is displayed only on KVM platform.
 See \ref nvmlConstants::NVML_DEVICE_UUID_BUFFER_SIZE.

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuInstance             Identifier of the target vGPU instance
 @param mdevUuid                 Pointer to caller-supplied buffer to hold MDEV UUID
 @param size                     Size of buffer in bytes

 @return
         - \ref NVML_SUCCESS                 successful completion
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_NOT_SUPPORTED     on any hypervisor other than KVM
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a mdevUuid is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a size is too small
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetMdevUUID(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        mdevUuid: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Query the currently creatable vGPU types on a specific GPU Instance.

 The function returns an array of vGPU types that can be created for a specified GPU instance. This array is stored
 in a caller-supplied buffer, with the buffer's element count passed through \a pVgpus->vgpuCount. The number of
 vGPU types written to the buffer is indicated by \a pVgpus->vgpuCount. If the buffer is too small to hold the vGPU
 type array, the function returns NVML_ERROR_INSUFFICIENT_SIZE and updates \a pVgpus->vgpuCount with the required
 element count.

 To determine the creatable vGPUs for a GPU Instance, invoke this function with \a pVgpus->vgpuCount set to 0 and
 \a pVgpus->vgpuTypeIds as NULL. This will result in NVML_ERROR_INSUFFICIENT_SIZE being returned, along with the
 count value in \a pVgpus->vgpuCount.

 The creatable vGPU types may differ over time, as there may be restrictions on what type of vGPUs can concurrently
 run on the device.

 @param gpuInstance                          The GPU instance handle
 @param pVgpus                               Pointer to the caller-provided structure of nvmlVgpuTypeIdInfo_t

 @return
         - \ref NVML_SUCCESS                          Upon success
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a gpuInstance is NULL or invalid, or \a pVgpus is NULL
                                                      or GPU Instance Id is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED              If not on a vGPU host or an unsupported GPU
         - \ref NVML_ERROR_INSUFFICIENT_SIZE          If \a pVgpus->vgpuTypeIds buffer is small
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pVgpus is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlGpuInstanceGetCreatableVgpus(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        pVgpus: *mut cuda_types::nvml::nvmlVgpuTypeIdInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the maximum number of vGPU instances per GPU instance for given vGPU type

 @param pMaxInstance                         Pointer to the caller-provided structure of nvmlVgpuTypeMaxInstance_t

 @return
         - \ref NVML_SUCCESS                          Upon success
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a pMaxInstance is NULL or \a pMaxInstance->vgpuTypeId is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED              If not on a vGPU host or an unsupported GPU or non-MIG vGPU type
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pMaxInstance is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlVgpuTypeGetMaxInstancesPerGpuInstance(
        pMaxInstance: *mut cuda_types::nvml::nvmlVgpuTypeMaxInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieve the active vGPU instances within a GPU instance.

 An array of active vGPU instances is returned in the caller-supplied buffer pointed
 at by \a pVgpuInstanceInfo->vgpuInstances. The array element count is passed in
 \a pVgpuInstanceInfo->vgpuCount, and \a pVgpuInstanceInfo->vgpuCount is used to return
 the number of vGPU instances written to the buffer.

 If the supplied buffer is not large enough to accommodate the vGPU instance array,
 the function returns NVML_ERROR_INSUFFICIENT_SIZE, with the element count of
 nvmlVgpuInstance_t array required in \a pVgpuInstanceInfo->vgpuCount. To query the
 number of active vGPU instances, call this function with pVgpuInstanceInfo->vgpuCount = 0
 and pVgpuInstanceInfo->vgpuTypeIds = NULL. The code will return NVML_ERROR_INSUFFICIENT_SIZE,
 or NVML_SUCCESS if no vGPU Types are active.

 @param gpuInstance          The GPU instance handle
 @param pVgpuInstanceInfo    Pointer to the vGPU instance information structure \a nvmlActiveVgpuInstanceInfo_t

 @return
         - \ref NVML_SUCCESS                          Successful completion
         - \ref NVML_ERROR_UNINITIALIZED              If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a gpuInstance is NULL or invalid, or \a pVgpuInstanceInfo is NULL
                                                      or GPU Instance Id is invalid
         - \ref NVML_ERROR_INSUFFICIENT_SIZE          \a pVgpuInstanceInfo->vgpuTypeIds buffer is too small,
                                                      array element count is returned in \a pVgpuInstanceInfo->vgpuCount
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pVgpuInstanceInfo is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED              If not on a vGPU host or an unsupported GPU
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlGpuInstanceGetActiveVgpus(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        pVgpuInstanceInfo: *mut cuda_types::nvml::nvmlActiveVgpuInstanceInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set vGPU scheduler state for the given GPU instance

 %GB20X_OR_NEWER%

 Scheduler state and params will be allowed to set only when no VM is running within the GPU instance.
 In \a nvmlVgpuSchedulerState_t, IFF enableARRMode is enabled then provide the avgFactor and frequency
 as input. If enableARRMode is disabled then provide timeslice as input.

 The scheduler state change won't persist across module load/unload and GPU Instance creation/deletion.

 @param gpuInstance                          The GPU instance handle
 @param pScheduler                           Pointer to the caller-provided structure of nvmlVgpuSchedulerState_t

 @return
         - \ref NVML_SUCCESS                          Upon success
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a gpuInstance is NULL or invalid, or \a pScheduler is NULL
                                                      or GPU Instance Id is invalid
         - \ref NVML_ERROR_RESET_REQUIRED             If setting the state failed with fatal error, reboot is required
         - \ref NVML_ERROR_NOT_SUPPORTED              If not on a vGPU host or an unsupported GPU or if any vGPU instance exists
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pScheduler is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlGpuInstanceSetVgpuSchedulerState(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        pScheduler: *mut cuda_types::nvml::nvmlVgpuSchedulerState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns the vGPU scheduler state for the given GPU instance.
 The information returned in \a nvmlVgpuSchedulerStateInfo_t is not relevant if the BEST EFFORT policy is set.

 %GB20X_OR_NEWER%

 @param gpuInstance                The GPU instance handle
 @param pSchedulerStateInfo        Reference in which \a pSchedulerStateInfo is returned

 @return
         - \ref NVML_SUCCESS                          vGPU scheduler state is successfully obtained
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a gpuInstance is NULL or invalid, or \a pSchedulerStateInfo is NULL
                                                      or GPU Instance Id is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED              If not on a vGPU host or an unsupported GPU
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pSchedulerStateInfo is invalid
         - \ref NVML_ERROR_UNKNOWN                    on any unexpected error*/
    fn nvmlGpuInstanceGetVgpuSchedulerState(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        pSchedulerStateInfo: *mut cuda_types::nvml::nvmlVgpuSchedulerStateInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns the vGPU scheduler logs for the given GPU instance.
 \a pSchedulerLogInfo points to a caller-allocated structure to contain the logs. The number of elements returned will
 never exceed \a NVML_SCHEDULER_SW_MAX_LOG_ENTRIES.

 To get the entire logs, call the function atleast 5 times a second.

 %GB20X_OR_NEWER%

 @param gpuInstance               The GPU instance handle
 @param pSchedulerLogInfo         Reference in which \a pSchedulerLogInfo is written

 @return
         - \ref NVML_SUCCESS                          vGPU scheduler logs are successfully obtained
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a gpuInstance is NULL or invalid, or \a pSchedulerLogInfo is NULL
                                                      or GPU Instance Id is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED              If not on a vGPU host or an unsupported GPU
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pSchedulerLogInfo is invalid
         - \ref NVML_ERROR_UNKNOWN                    on any unexpected error*/
    fn nvmlGpuInstanceGetVgpuSchedulerLog(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        pSchedulerLogInfo: *mut cuda_types::nvml::nvmlVgpuSchedulerLogInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Query the creatable vGPU placement ID of the vGPU type within a GPU instance.

 %GB20X_OR_NEWER%

 An array of creatable vGPU placement IDs for the vGPU type ID indicated by \a pCreatablePlacementInfo->vgpuTypeId
 is returned in the caller-supplied buffer of \a pCreatablePlacementInfo->placementIds. Memory needed for the
 placementIds array should be allocated based on maximum instances of a vGPU type per GPU instance which can be
 queried via \ref nvmlVgpuTypeGetMaxInstancesPerGpuInstance().
 If the provided count by the caller is insufficient, the function will return NVML_ERROR_INSUFFICIENT_SIZE along with
 the number of required entries in \a pCreatablePlacementInfo->count. The caller should then reallocate a buffer with the size
 of pCreatablePlacementInfo->count * sizeof(pCreatablePlacementInfo->placementIds) and invoke the function again.
 The creatable vGPU placement IDs may differ over time, as there may be restrictions on what type of vGPU the
 vGPU instance is running.

 @param gpuInstance                The GPU instance handle
 @param pCreatablePlacementInfo    Pointer to the list of vGPU creatable placement structure \a nvmlVgpuCreatablePlacementInfo_t

 @return
         - \ref NVML_SUCCESS                          Successful completion
         - \ref NVML_ERROR_UNINITIALIZED              If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a gpuInstance is NULL or invalid, or \a pCreatablePlacementInfo is NULL
                                                      or GPU Instance Id is invalid
         - \ref NVML_ERROR_INSUFFICIENT_SIZE          If the buffer is small, element count is returned in \a pCreatablePlacementInfo->count
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pCreatablePlacementInfo is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED              If not on a vGPU host or an unsupported GPU or vGPU heterogeneous mode is not enabled
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlGpuInstanceGetVgpuTypeCreatablePlacements(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        pCreatablePlacementInfo: *mut cuda_types::nvml::nvmlVgpuCreatablePlacementInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the vGPU heterogeneous mode for the GPU instance.

 When in heterogeneous mode, a vGPU can concurrently host timesliced vGPUs with differing framebuffer sizes.

 On successful return, the function returns \a pHeterogeneousMode->mode with the current vGPU heterogeneous mode.
 \a pHeterogeneousMode->version is the version number of the structure nvmlVgpuHeterogeneousMode_t, the caller should
 set the correct version number to retrieve the vGPU heterogeneous mode.
 \a pHeterogeneousMode->mode can either be \ref NVML_FEATURE_ENABLED or \ref NVML_FEATURE_DISABLED.

 %GB20X_OR_NEWER%

 @param gpuInstance               The GPU instance handle
 @param pHeterogeneousMode        Pointer to the caller-provided structure of nvmlVgpuHeterogeneousMode_t

 @return
         - \ref NVML_SUCCESS                          Upon success
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a gpuInstance is NULL or invalid, or \a pHeterogeneousMode is NULL
                                                      or GPU Instance Id is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED              If not on a vGPU host or an unsupported GPU or not in MIG mode
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pHeterogeneousMode is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlGpuInstanceGetVgpuHeterogeneousMode(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        pHeterogeneousMode: *mut cuda_types::nvml::nvmlVgpuHeterogeneousMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Enable or disable vGPU heterogeneous mode for the GPU instance.

 When in heterogeneous mode, a vGPU can concurrently host timesliced vGPUs with differing framebuffer sizes.

 API would return an appropriate error code upon unsuccessful activation. For example, the heterogeneous mode
 set will fail with error \ref NVML_ERROR_IN_USE if any vGPU instance is active within the GPU instance.
 The caller of this API is expected to shutdown the vGPU VMs and retry setting the \a mode.
 On successful return, the function updates the vGPU heterogeneous mode with the user provided \a pHeterogeneousMode->mode.
 \a pHeterogeneousMode->version is the version number of the structure nvmlVgpuHeterogeneousMode_t, the caller should
 set the correct version number to set the vGPU heterogeneous mode.

 %GB20X_OR_NEWER%

 @param gpuInstance               The GPU instance handle
 @param pHeterogeneousMode        Pointer to the caller-provided structure of nvmlVgpuHeterogeneousMode_t

 @return
         - \ref NVML_SUCCESS                          Upon success
         - \ref NVML_ERROR_UNINITIALIZED              If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a gpuInstance  is NULL or invalid,
                                                      or \a pHeterogeneousMode is NULL or \a pHeterogeneousMode->mode is invalid
                                                      or GPU Instance Id is invalid
         - \ref NVML_ERROR_IN_USE                     If the \a gpuInstance is in use
         - \ref NVML_ERROR_NOT_SUPPORTED              If not on a vGPU host or an unsupported GPU
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a pHeterogeneousMode is invalid
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlGpuInstanceSetVgpuHeterogeneousMode(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        pHeterogeneousMode: *const cuda_types::nvml::nvmlVgpuHeterogeneousMode_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns vGPU metadata structure for a running vGPU. The structure contains information about the vGPU and its associated VM
 such as the currently installed NVIDIA guest driver version, together with host driver version and an opaque data section
 containing internal state.

 nvmlVgpuInstanceGetMetadata() may be called at any time for a vGPU instance. Some fields in the returned structure are
 dependent on information obtained from the guest VM, which may not yet have reached a state where that information
 is available. The current state of these dependent fields is reflected in the info structure's \ref nvmlVgpuGuestInfoState_t field.

 The VMM may choose to read and save the vGPU's VM info as persistent metadata associated with the VM, and provide
 it to Virtual GPU Manager when creating a vGPU for subsequent instances of the VM.

 The caller passes in a buffer via \a vgpuMetadata, with the size of the buffer in \a bufferSize. If the vGPU Metadata structure
 is too large to fit in the supplied buffer, the function returns NVML_ERROR_INSUFFICIENT_SIZE with the size needed
 in \a bufferSize.

 @param vgpuInstance             vGPU instance handle
 @param vgpuMetadata             Pointer to caller-supplied buffer into which vGPU metadata is written
 @param bufferSize               Size of vgpuMetadata buffer

 @return
         - \ref NVML_SUCCESS                   vGPU metadata structure was successfully returned
         - \ref NVML_ERROR_INSUFFICIENT_SIZE   vgpuMetadata buffer is too small, required size is returned in \a bufferSize
         - \ref NVML_ERROR_INVALID_ARGUMENT    if \a bufferSize is NULL or \a vgpuInstance is 0; if \a vgpuMetadata is NULL and the value of \a bufferSize is not 0.
         - \ref NVML_ERROR_NOT_FOUND           if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_UNKNOWN             on any unexpected error*/
    fn nvmlVgpuInstanceGetMetadata(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        vgpuMetadata: *mut cuda_types::nvml::nvmlVgpuMetadata_t,
        bufferSize: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns a vGPU metadata structure for the physical GPU indicated by \a device. The structure contains information about
 the GPU and the currently installed NVIDIA host driver version that's controlling it, together with an opaque data section
 containing internal state.

 The caller passes in a buffer via \a pgpuMetadata, with the size of the buffer in \a bufferSize. If the \a pgpuMetadata
 structure is too large to fit in the supplied buffer, the function returns NVML_ERROR_INSUFFICIENT_SIZE with the size needed
 in \a bufferSize.

 @param device                The identifier of the target device
 @param pgpuMetadata          Pointer to caller-supplied buffer into which \a pgpuMetadata is written
 @param bufferSize            Pointer to size of \a pgpuMetadata buffer

 @return
         - \ref NVML_SUCCESS                   GPU metadata structure was successfully returned
         - \ref NVML_ERROR_INSUFFICIENT_SIZE   pgpuMetadata buffer is too small, required size is returned in \a bufferSize
         - \ref NVML_ERROR_INVALID_ARGUMENT    if \a bufferSize is NULL or \a device is invalid; if \a pgpuMetadata is NULL and the value of \a bufferSize is not 0.
         - \ref NVML_ERROR_NOT_SUPPORTED       vGPU is not supported by the system
         - \ref NVML_ERROR_UNKNOWN             on any unexpected error*/
    fn nvmlDeviceGetVgpuMetadata(
        device: cuda_types::nvml::nvmlDevice_t,
        pgpuMetadata: *mut cuda_types::nvml::nvmlVgpuPgpuMetadata_t,
        bufferSize: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Takes a vGPU instance metadata structure read from \ref nvmlVgpuInstanceGetMetadata(), and a vGPU metadata structure for a
 physical GPU read from \ref nvmlDeviceGetVgpuMetadata(), and returns compatibility information of the vGPU instance and the
 physical GPU.

 The caller passes in a buffer via \a compatibilityInfo, into which a compatibility information structure is written. The
 structure defines the states in which the vGPU / VM may be booted on the physical GPU. If the vGPU / VM compatibility
 with the physical GPU is limited, a limit code indicates the factor limiting compatability.
 (see \ref nvmlVgpuPgpuCompatibilityLimitCode_t for details).

 Note: vGPU compatibility does not take into account dynamic capacity conditions that may limit a system's ability to
       boot a given vGPU or associated VM.

 @param vgpuMetadata          Pointer to caller-supplied vGPU metadata structure
 @param pgpuMetadata          Pointer to caller-supplied GPU metadata structure
 @param compatibilityInfo     Pointer to caller-supplied buffer to hold compatibility info

 @return
         - \ref NVML_SUCCESS                   vGPU metadata structure was successfully returned
         - \ref NVML_ERROR_INVALID_ARGUMENT    If \a vgpuMetadata or \a pgpuMetadata or \a bufferSize are NULL
         - \ref NVML_ERROR_UNKNOWN             On any unexpected error*/
    fn nvmlGetVgpuCompatibility(
        vgpuMetadata: *mut cuda_types::nvml::nvmlVgpuMetadata_t,
        pgpuMetadata: *mut cuda_types::nvml::nvmlVgpuPgpuMetadata_t,
        compatibilityInfo: *mut cuda_types::nvml::nvmlVgpuPgpuCompatibility_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns the properties of the physical GPU indicated by the device in an ascii-encoded string format.

 The caller passes in a buffer via \a pgpuMetadata, with the size of the buffer in \a bufferSize. If the
 string is too large to fit in the supplied buffer, the function returns NVML_ERROR_INSUFFICIENT_SIZE with the size needed
 in \a bufferSize.

 @param device                The identifier of the target device
 @param pgpuMetadata          Pointer to caller-supplied buffer into which \a pgpuMetadata is written
 @param bufferSize            Pointer to size of \a pgpuMetadata buffer

 @return
         - \ref NVML_SUCCESS                   GPU metadata structure was successfully returned
         - \ref NVML_ERROR_INSUFFICIENT_SIZE   \a pgpuMetadata buffer is too small, required size is returned in \a bufferSize
         - \ref NVML_ERROR_INVALID_ARGUMENT    If \a bufferSize is NULL or \a device is invalid; if \a pgpuMetadata is NULL and the value of \a bufferSize is not 0.
         - \ref NVML_ERROR_NOT_SUPPORTED       If vGPU is not supported by the system
         - \ref NVML_ERROR_UNKNOWN             On any unexpected error*/
    fn nvmlDeviceGetPgpuMetadataString(
        device: cuda_types::nvml::nvmlDevice_t,
        pgpuMetadata: *mut ::core::ffi::c_char,
        bufferSize: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns the vGPU Software scheduler logs.
 \a pSchedulerLog points to a caller-allocated structure to contain the logs. The number of elements returned will
 never exceed \a NVML_SCHEDULER_SW_MAX_LOG_ENTRIES.

 To get the entire logs, call the function atleast 5 times a second.

 For Pascal &tm; or newer fully supported devices.

 @param device                The identifier of the target \a device
 @param pSchedulerLog         Reference in which \a pSchedulerLog is written

 @return
         - \ref NVML_SUCCESS                   vGPU scheduler logs were successfully obtained
         - \ref NVML_ERROR_INVALID_ARGUMENT    If \a pSchedulerLog is NULL or \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED       If MIG is enabled or \a device not in vGPU host mode
         - \ref NVML_ERROR_UNKNOWN             On any unexpected error*/
    fn nvmlDeviceGetVgpuSchedulerLog(
        device: cuda_types::nvml::nvmlDevice_t,
        pSchedulerLog: *mut cuda_types::nvml::nvmlVgpuSchedulerLog_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns the vGPU scheduler state.
 The information returned in \a nvmlVgpuSchedulerGetState_t is not relevant if the BEST EFFORT policy is set.

 For Pascal &tm; or newer fully supported devices.

 @param device                The identifier of the target \a device
 @param pSchedulerState       Reference in which \a pSchedulerState is returned

 @return
         - \ref NVML_SUCCESS                   vGPU scheduler state is successfully obtained
         - \ref NVML_ERROR_INVALID_ARGUMENT    If \a pSchedulerState is NULL or \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED       If MIG is enabled or \a device not in vGPU host mode
         - \ref NVML_ERROR_UNKNOWN             On any unexpected error*/
    fn nvmlDeviceGetVgpuSchedulerState(
        device: cuda_types::nvml::nvmlDevice_t,
        pSchedulerState: *mut cuda_types::nvml::nvmlVgpuSchedulerGetState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Returns the vGPU scheduler capabilities.
 The list of supported vGPU schedulers returned in \a nvmlVgpuSchedulerCapabilities_t is from
 the NVML_VGPU_SCHEDULER_POLICY_*. This list enumerates the supported scheduler policies
 if the engine is Graphics type.
 The other values in \a nvmlVgpuSchedulerCapabilities_t are also applicable if the engine is
 Graphics type. For other engine types, it is BEST EFFORT policy.
 If ARR is supported and enabled, scheduling frequency and averaging factor are applicable
 else timeSlice is applicable.

 For Pascal &tm; or newer fully supported devices.

 @param device                The identifier of the target \a device
 @param pCapabilities         Reference in which \a pCapabilities is written

 @return
         - \ref NVML_SUCCESS                   vGPU scheduler capabilities were successfully obtained
         - \ref NVML_ERROR_INVALID_ARGUMENT    If \a pCapabilities is NULL or \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED       The API is not supported in current state or \a device not in vGPU host mode
         - \ref NVML_ERROR_UNKNOWN             On any unexpected error*/
    fn nvmlDeviceGetVgpuSchedulerCapabilities(
        device: cuda_types::nvml::nvmlDevice_t,
        pCapabilities: *mut cuda_types::nvml::nvmlVgpuSchedulerCapabilities_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Sets the vGPU scheduler state.

 For Pascal &tm; or newer fully supported devices.

 The scheduler state change won't persist across module load/unload.
 Scheduler state and params will be allowed to set only when no VM is running.
 In \a nvmlVgpuSchedulerSetState_t, IFF enableARRMode is enabled then
 provide avgFactorForARR and frequency as input. If enableARRMode is disabled
 then provide timeslice as input.

 @param device                The identifier of the target \a device
 @param pSchedulerState       vGPU \a pSchedulerState to set

 @return
         - \ref NVML_SUCCESS                  vGPU scheduler state has been successfully set
         - \ref NVML_ERROR_INVALID_ARGUMENT   If \a pSchedulerState is NULL or \a device is invalid
         - \ref NVML_ERROR_RESET_REQUIRED     If setting \a pSchedulerState failed with fatal error,
                                              reboot is required to overcome from this error.
         - \ref NVML_ERROR_NOT_SUPPORTED      If MIG is enabled or \a device not in vGPU host mode
                                              or if any vGPU instance currently exists on the \a device
         - \ref NVML_ERROR_UNKNOWN            On any unexpected error*/
    fn nvmlDeviceSetVgpuSchedulerState(
        device: cuda_types::nvml::nvmlDevice_t,
        pSchedulerState: *mut cuda_types::nvml::nvmlVgpuSchedulerSetState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Query the ranges of supported vGPU versions.

 This function gets the linear range of supported vGPU versions that is preset for the NVIDIA vGPU Manager and the range set by an administrator.
 If the preset range has not been overridden by \ref nvmlSetVgpuVersion, both ranges are the same.

 The caller passes pointers to the following \ref nvmlVgpuVersion_t structures, into which the NVIDIA vGPU Manager writes the ranges:
 1. \a supported structure that represents the preset range of vGPU versions supported by the NVIDIA vGPU Manager.
 2. \a current structure that represents the range of supported vGPU versions set by an administrator. By default, this range is the same as the preset range.

 @param supported  Pointer to the structure in which the preset range of vGPU versions supported by the NVIDIA vGPU Manager is written
 @param current    Pointer to the structure in which the range of supported vGPU versions set by an administrator is written

 @return
 - \ref NVML_SUCCESS                 The vGPU version range structures were successfully obtained.
 - \ref NVML_ERROR_NOT_SUPPORTED     The API is not supported.
 - \ref NVML_ERROR_INVALID_ARGUMENT  The \a supported parameter or the \a current parameter is NULL.
 - \ref NVML_ERROR_UNKNOWN           An error occurred while the data was being fetched.*/
    fn nvmlGetVgpuVersion(
        supported: *mut cuda_types::nvml::nvmlVgpuVersion_t,
        current: *mut cuda_types::nvml::nvmlVgpuVersion_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Override the preset range of vGPU versions supported by the NVIDIA vGPU Manager with a range set by an administrator.

 This function configures the NVIDIA vGPU Manager with a range of supported vGPU versions set by an administrator. This range must be a subset of the
 preset range that the NVIDIA vGPU Manager supports. The custom range set by an administrator takes precedence over the preset range and is advertised to
 the guest VM for negotiating the vGPU version. See \ref nvmlGetVgpuVersion for details of how to query the preset range of versions supported.

 This function takes a pointer to vGPU version range structure \ref nvmlVgpuVersion_t as input to override the preset vGPU version range that the NVIDIA vGPU Manager supports.

 After host system reboot or driver reload, the range of supported versions reverts to the range that is preset for the NVIDIA vGPU Manager.

 @note 1. The range set by the administrator must be a subset of the preset range that the NVIDIA vGPU Manager supports. Otherwise, an error is returned.
       2. If the range of supported guest driver versions does not overlap the range set by the administrator, the guest driver fails to load.
       3. If the range of supported guest driver versions overlaps the range set by the administrator, the guest driver will load with a negotiated
          vGPU version that is the maximum value in the overlapping range.
       4. No VMs must be running on the host when this function is called. If a VM is running on the host, the call to this function fails.

 @param vgpuVersion   Pointer to a caller-supplied range of supported vGPU versions.

 @return
 - \ref NVML_SUCCESS                 The preset range of supported vGPU versions was successfully overridden.
 - \ref NVML_ERROR_NOT_SUPPORTED     The API is not supported.
 - \ref NVML_ERROR_IN_USE            The range was not overridden because a VM is running on the host.
 - \ref NVML_ERROR_INVALID_ARGUMENT  The \a vgpuVersion parameter specifies a range that is outside the range supported by the NVIDIA vGPU Manager or if \a vgpuVersion is NULL.*/
    fn nvmlSetVgpuVersion(
        vgpuVersion: *mut cuda_types::nvml::nvmlVgpuVersion_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves current utilization for vGPUs on a physical GPU (device).

 For Kepler &tm; or newer fully supported devices.

 Reads recent utilization of GPU SM (3D/Compute), framebuffer, video encoder, and video decoder for vGPU instances running
 on a device. Utilization values are returned as an array of utilization sample structures in the caller-supplied buffer
 pointed at by \a utilizationSamples. One utilization sample structure is returned per vGPU instance, and includes the
 CPU timestamp at which the samples were recorded. Individual utilization values are returned as "unsigned int" values
 in nvmlValue_t unions. The function sets the caller-supplied \a sampleValType to NVML_VALUE_TYPE_UNSIGNED_INT to
 indicate the returned value type.

 To read utilization values, first determine the size of buffer required to hold the samples by invoking the function with
 \a utilizationSamples set to NULL. The function will return NVML_ERROR_INSUFFICIENT_SIZE, with the current vGPU instance
 count in \a vgpuInstanceSamplesCount, or NVML_SUCCESS if the current vGPU instance count is zero. The caller should allocate
 a buffer of size vgpuInstanceSamplesCount * sizeof(nvmlVgpuInstanceUtilizationSample_t). Invoke the function again with
 the allocated buffer passed in \a utilizationSamples, and \a vgpuInstanceSamplesCount set to the number of entries the
 buffer is sized for.

 On successful return, the function updates \a vgpuInstanceSampleCount with the number of vGPU utilization sample
 structures that were actually written. This may differ from a previously read value as vGPU instances are created or
 destroyed.

 lastSeenTimeStamp represents the CPU timestamp in microseconds at which utilization samples were last read. Set it to 0
 to read utilization based on all the samples maintained by the driver's internal sample buffer. Set lastSeenTimeStamp
 to a timeStamp retrieved from a previous query to read utilization since the previous query.

 @param device                        The identifier for the target device
 @param lastSeenTimeStamp             Return only samples with timestamp greater than lastSeenTimeStamp.
 @param sampleValType                 Pointer to caller-supplied buffer to hold the type of returned sample values
 @param vgpuInstanceSamplesCount      Pointer to caller-supplied array size, and returns number of vGPU instances
 @param utilizationSamples            Pointer to caller-supplied buffer in which vGPU utilization samples are returned

 @return
         - \ref NVML_SUCCESS                 if utilization samples are successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a vgpuInstanceSamplesCount or \a sampleValType is
                                             NULL, or a sample count of 0 is passed with a non-NULL \a utilizationSamples
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if supplied \a vgpuInstanceSamplesCount is too small to return samples for all
                                             vGPU instances currently executing on the device
         - \ref NVML_ERROR_NOT_SUPPORTED     if vGPU is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_FOUND         if sample entries are not found
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetVgpuUtilization(
        device: cuda_types::nvml::nvmlDevice_t,
        lastSeenTimeStamp: ::core::ffi::c_ulonglong,
        sampleValType: *mut cuda_types::nvml::nvmlValueType_t,
        vgpuInstanceSamplesCount: *mut ::core::ffi::c_uint,
        utilizationSamples: *mut cuda_types::nvml::nvmlVgpuInstanceUtilizationSample_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves recent utilization for vGPU instances running on a physical GPU (device).

 For Kepler &tm; or newer fully supported devices.

 Reads recent utilization of GPU SM (3D/Compute), framebuffer, video encoder, video decoder, jpeg decoder, and OFA for vGPU
 instances running on a device. Utilization values are returned as an array of utilization sample structures in the caller-supplied
 buffer pointed at by \a vgpuUtilInfo->vgpuUtilArray. One utilization sample structure is returned per vGPU instance, and includes the
 CPU timestamp at which the samples were recorded. Individual utilization values are returned as "unsigned int" values
 in nvmlValue_t unions. The function sets the caller-supplied \a vgpuUtilInfo->sampleValType to NVML_VALUE_TYPE_UNSIGNED_INT to
 indicate the returned value type.

 To read utilization values, first determine the size of buffer required to hold the samples by invoking the function with
 \a vgpuUtilInfo->vgpuUtilArray set to NULL. The function will return NVML_ERROR_INSUFFICIENT_SIZE, with the current vGPU instance
 count in \a vgpuUtilInfo->vgpuInstanceCount, or NVML_SUCCESS if the current vGPU instance count is zero. The caller should allocate
 a buffer of size vgpuUtilInfo->vgpuInstanceCount * sizeof(nvmlVgpuInstanceUtilizationInfo_t). Invoke the function again with
 the allocated buffer passed in \a vgpuUtilInfo->vgpuUtilArray, and \a vgpuUtilInfo->vgpuInstanceCount set to the number of entries the
 buffer is sized for.

 On successful return, the function updates \a vgpuUtilInfo->vgpuInstanceCount with the number of vGPU utilization sample
 structures that were actually written. This may differ from a previously read value as vGPU instances are created or
 destroyed.

 \a vgpuUtilInfo->lastSeenTimeStamp represents the CPU timestamp in microseconds at which utilization samples were last read. Set it to 0
 to read utilization based on all the samples maintained by the driver's internal sample buffer. Set \a vgpuUtilInfo->lastSeenTimeStamp
 to a timeStamp retrieved from a previous query to read utilization since the previous query.

 @param device                        The identifier for the target device
 @param vgpuUtilInfo                  Pointer to the caller-provided structure of nvmlVgpuInstancesUtilizationInfo_t

 @return
         - \ref NVML_SUCCESS                          If utilization samples are successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED              If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a device is invalid, \a vgpuUtilInfo is NULL, or \a vgpuUtilInfo->vgpuInstanceCount is 0
         - \ref NVML_ERROR_NOT_SUPPORTED              If vGPU is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST                If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a vgpuUtilInfo is invalid
         - \ref NVML_ERROR_INSUFFICIENT_SIZE          If \a vgpuUtilInfo->vgpuUtilArray is NULL, or the buffer size of vgpuUtilInfo->vgpuInstanceCount is too small.
                                                      The caller should check the current vGPU instance count from the returned vgpuUtilInfo->vgpuInstanceCount, and call
                                                      the function again with a buffer of size vgpuUtilInfo->vgpuInstanceCount * sizeof(nvmlVgpuInstanceUtilizationInfo_t)
         - \ref NVML_ERROR_NOT_FOUND                  If sample entries are not found
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlDeviceGetVgpuInstancesUtilizationInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        vgpuUtilInfo: *mut cuda_types::nvml::nvmlVgpuInstancesUtilizationInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves current utilization for processes running on vGPUs on a physical GPU (device).

 For Maxwell &tm; or newer fully supported devices.

 Reads recent utilization of GPU SM (3D/Compute), framebuffer, video encoder, and video decoder for processes running on
 vGPU instances active on a device. Utilization values are returned as an array of utilization sample structures in the
 caller-supplied buffer pointed at by \a utilizationSamples. One utilization sample structure is returned per process running
 on vGPU instances, that had some non-zero utilization during the last sample period. It includes the CPU timestamp at which
 the samples were recorded. Individual utilization values are returned as "unsigned int" values.

 To read utilization values, first determine the size of buffer required to hold the samples by invoking the function with
 \a utilizationSamples set to NULL. The function will return NVML_ERROR_INSUFFICIENT_SIZE, with the current vGPU instance
 count in \a vgpuProcessSamplesCount. The caller should allocate a buffer of size
 vgpuProcessSamplesCount * sizeof(nvmlVgpuProcessUtilizationSample_t). Invoke the function again with
 the allocated buffer passed in \a utilizationSamples, and \a vgpuProcessSamplesCount set to the number of entries the
 buffer is sized for.

 On successful return, the function updates \a vgpuSubProcessSampleCount with the number of vGPU sub process utilization sample
 structures that were actually written. This may differ from a previously read value depending on the number of processes that are active
 in any given sample period.

 lastSeenTimeStamp represents the CPU timestamp in microseconds at which utilization samples were last read. Set it to 0
 to read utilization based on all the samples maintained by the driver's internal sample buffer. Set lastSeenTimeStamp
 to a timeStamp retrieved from a previous query to read utilization since the previous query.

 @param device                        The identifier for the target device
 @param lastSeenTimeStamp             Return only samples with timestamp greater than lastSeenTimeStamp.
 @param vgpuProcessSamplesCount       Pointer to caller-supplied array size, and returns number of processes running on vGPU instances
 @param utilizationSamples            Pointer to caller-supplied buffer in which vGPU sub process utilization samples are returned

 @return
         - \ref NVML_SUCCESS                 if utilization samples are successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid, \a vgpuProcessSamplesCount or a sample count of 0 is
                                             passed with a non-NULL \a utilizationSamples
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if supplied \a vgpuProcessSamplesCount is too small to return samples for all
                                             vGPU instances currently executing on the device
         - \ref NVML_ERROR_NOT_SUPPORTED     if vGPU is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST       if the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_FOUND         if sample entries are not found
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetVgpuProcessUtilization(
        device: cuda_types::nvml::nvmlDevice_t,
        lastSeenTimeStamp: ::core::ffi::c_ulonglong,
        vgpuProcessSamplesCount: *mut ::core::ffi::c_uint,
        utilizationSamples: *mut cuda_types::nvml::nvmlVgpuProcessUtilizationSample_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves recent utilization for processes running on vGPU instances on a physical GPU (device).

 For Maxwell &tm; or newer fully supported devices.

 Reads recent utilization of GPU SM (3D/Compute), framebuffer, video encoder, video decoder, jpeg decoder, and OFA for processes running
 on vGPU instances active on a device. Utilization values are returned as an array of utilization sample structures in the caller-supplied
 buffer pointed at by \a vgpuProcUtilInfo->vgpuProcUtilArray. One utilization sample structure is returned per process running
 on vGPU instances, that had some non-zero utilization during the last sample period. It includes the CPU timestamp at which
 the samples were recorded. Individual utilization values are returned as "unsigned int" values.

 To read utilization values, first determine the size of buffer required to hold the samples by invoking the function with
 \a vgpuProcUtilInfo->vgpuProcUtilArray set to NULL. The function will return NVML_ERROR_INSUFFICIENT_SIZE, with the current processes' count
 running on vGPU instances in \a vgpuProcUtilInfo->vgpuProcessCount. The caller should allocate a buffer of size
 vgpuProcUtilInfo->vgpuProcessCount * sizeof(nvmlVgpuProcessUtilizationSample_t). Invoke the function again with the allocated buffer passed
 in \a vgpuProcUtilInfo->vgpuProcUtilArray, and \a vgpuProcUtilInfo->vgpuProcessCount set to the number of entries the buffer is sized for.

 On successful return, the function updates \a vgpuProcUtilInfo->vgpuProcessCount with the number of vGPU sub process utilization sample
 structures that were actually written. This may differ from a previously read value depending on the number of processes that are active
 in any given sample period.

 vgpuProcUtilInfo->lastSeenTimeStamp represents the CPU timestamp in microseconds at which utilization samples were last read. Set it to 0
 to read utilization based on all the samples maintained by the driver's internal sample buffer. Set vgpuProcUtilInfo->lastSeenTimeStamp
 to a timeStamp retrieved from a previous query to read utilization since the previous query.

 @param device                        The identifier for the target device
 @param vgpuProcUtilInfo              Pointer to the caller-provided structure of nvmlVgpuProcessesUtilizationInfo_t

 @return
         - \ref NVML_SUCCESS                          If utilization samples are successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED              If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT           If \a device is invalid, or \a vgpuProcUtilInfo is null
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH  If the version of \a vgpuProcUtilInfo is invalid
         - \ref NVML_ERROR_INSUFFICIENT_SIZE          If \a vgpuProcUtilInfo->vgpuProcUtilArray is null, or supplied \a vgpuProcUtilInfo->vgpuProcessCount
                                                      is too small to return samples for all processes on vGPU instances currently executing on the device.
                                                      The caller should check the current processes count from the returned \a vgpuProcUtilInfo->vgpuProcessCount,
                                                      and call the function again with a buffer of size
                                                      vgpuProcUtilInfo->vgpuProcessCount * sizeof(nvmlVgpuProcessUtilizationSample_t)
         - \ref NVML_ERROR_NOT_SUPPORTED              If vGPU is not supported by the device
         - \ref NVML_ERROR_GPU_IS_LOST                If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_NOT_FOUND                  If sample entries are not found
         - \ref NVML_ERROR_UNKNOWN                    On any unexpected error*/
    fn nvmlDeviceGetVgpuProcessesUtilizationInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        vgpuProcUtilInfo: *mut cuda_types::nvml::nvmlVgpuProcessesUtilizationInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Queries the state of per process accounting mode on vGPU.

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuInstance            The identifier of the target vGPU instance
 @param mode                    Reference in which to return the current accounting mode

 @return
         - \ref NVML_SUCCESS                 if the mode has been successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a mode is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_NOT_SUPPORTED     if the vGPU doesn't support this feature
         - \ref NVML_ERROR_DRIVER_NOT_LOADED if NVIDIA driver is not running on the vGPU instance
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetAccountingMode(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        mode: *mut cuda_types::nvml::nvmlEnableState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Queries list of processes running on vGPU that can be queried for accounting stats. The list of processes
 returned can be in running or terminated state.

 For Maxwell &tm; or newer fully supported devices.

 To just query the maximum number of processes that can be queried, call this function with *count = 0 and
 pids=NULL. The return code will be NVML_ERROR_INSUFFICIENT_SIZE, or NVML_SUCCESS if list is empty.

 For more details see \ref nvmlVgpuInstanceGetAccountingStats.

 @note In case of PID collision some processes might not be accessible before the circular buffer is full.

 @param vgpuInstance            The identifier of the target vGPU instance
 @param count                   Reference in which to provide the \a pids array size, and
                                to return the number of elements ready to be queried
 @param pids                    Reference in which to return list of process ids

 @return
         - \ref NVML_SUCCESS                 if pids were successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a count is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_NOT_SUPPORTED     if the vGPU doesn't support this feature or accounting mode is disabled
         - \ref NVML_ERROR_INSUFFICIENT_SIZE if \a count is too small (\a count is set to expected value)
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error

 @see nvmlVgpuInstanceGetAccountingPids*/
    fn nvmlVgpuInstanceGetAccountingPids(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        count: *mut ::core::ffi::c_uint,
        pids: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Queries process's accounting stats.

 For Maxwell &tm; or newer fully supported devices.

 Accounting stats capture GPU utilization and other statistics across the lifetime of a process, and
 can be queried during life time of the process or after its termination.
 The time field in \ref nvmlAccountingStats_t is reported as 0 during the lifetime of the process and
 updated to actual running time after its termination.
 Accounting stats are kept in a circular buffer, newly created processes overwrite information about old
 processes.

 See \ref nvmlAccountingStats_t for description of each returned metric.
 List of processes that can be queried can be retrieved from \ref nvmlVgpuInstanceGetAccountingPids.

 @note Accounting Mode needs to be on. See \ref nvmlVgpuInstanceGetAccountingMode.
 @note Only compute and graphics applications stats can be queried. Monitoring applications stats can't be
         queried since they don't contribute to GPU utilization.
 @note In case of pid collision stats of only the latest process (that terminated last) will be reported

 @param vgpuInstance            The identifier of the target vGPU instance
 @param pid                     Process Id of the target process to query stats for
 @param stats                   Reference in which to return the process's accounting stats

 @return
         - \ref NVML_SUCCESS                 if stats have been successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a stats is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
                                             or \a stats is not found
         - \ref NVML_ERROR_NOT_SUPPORTED     if the vGPU doesn't support this feature or accounting mode is disabled
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetAccountingStats(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        pid: ::core::ffi::c_uint,
        stats: *mut cuda_types::nvml::nvmlAccountingStats_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Clears accounting information of the vGPU instance that have already terminated.

 For Maxwell &tm; or newer fully supported devices.
 Requires root/admin permissions.

 @note Accounting Mode needs to be on. See \ref nvmlVgpuInstanceGetAccountingMode.
 @note Only compute and graphics applications stats are reported and can be cleared since monitoring applications
         stats don't contribute to GPU utilization.

 @param vgpuInstance            The identifier of the target vGPU instance

 @return
         - \ref NVML_SUCCESS                 if accounting information has been cleared
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is invalid
         - \ref NVML_ERROR_NO_PERMISSION     if the user doesn't have permission to perform this operation
         - \ref NVML_ERROR_NOT_SUPPORTED     if the vGPU doesn't support this feature or accounting mode is disabled
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceClearAccountingPids(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Query the license information of the vGPU instance.

 For Maxwell &tm; or newer fully supported devices.

 @param vgpuInstance              Identifier of the target vGPU instance
 @param licenseInfo               Pointer to vGPU license information structure

 @return
         - \ref NVML_SUCCESS                 if information is successfully retrieved
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a vgpuInstance is 0, or \a licenseInfo is NULL
         - \ref NVML_ERROR_NOT_FOUND         if \a vgpuInstance does not match a valid active vGPU instance on the system
         - \ref NVML_ERROR_DRIVER_NOT_LOADED if NVIDIA driver is not running on the vGPU instance
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlVgpuInstanceGetLicenseInfo_v2(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        licenseInfo: *mut cuda_types::nvml::nvmlVgpuLicenseInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Retrieves the number of excluded GPU devices in the system.

 For all products.

 @param deviceCount                          Reference in which to return the number of excluded devices

 @return
         - \ref NVML_SUCCESS                 if \a deviceCount has been set
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a deviceCount is NULL*/
    fn nvmlGetExcludedDeviceCount(
        deviceCount: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Acquire the device information for an excluded GPU device, based on its index.

 For all products.

 Valid indices are derived from the \a deviceCount returned by
   \ref nvmlGetExcludedDeviceCount(). For example, if \a deviceCount is 2 the valid indices
   are 0 and 1, corresponding to GPU 0 and GPU 1.

 @param index                                The index of the target GPU, >= 0 and < \a deviceCount
 @param info                                 Reference in which to return the device information

 @return
         - \ref NVML_SUCCESS                  if \a device has been set
         - \ref NVML_ERROR_INVALID_ARGUMENT   if \a index is invalid or \a info is NULL

 @see nvmlGetExcludedDeviceCount*/
    fn nvmlGetExcludedDeviceInfoByIndex(
        index: ::core::ffi::c_uint,
        info: *mut cuda_types::nvml::nvmlExcludedDeviceInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set MIG mode for the device.

 For Ampere &tm; or newer fully supported devices.
 Requires root user.

 This mode determines whether a GPU instance can be created.

 This API may unbind or reset the device to activate the requested mode. Thus, the attributes associated with the
 device, such as minor number, might change. The caller of this API is expected to query such attributes again.

 On certain platforms like pass-through virtualization, where reset functionality may not be exposed directly, VM
 reboot is required. \a activationStatus would return \ref NVML_ERROR_RESET_REQUIRED for such cases.

 \a activationStatus would return the appropriate error code upon unsuccessful activation. For example, if device
 unbind fails because the device isn't idle, \ref NVML_ERROR_IN_USE would be returned. The caller of this API
 is expected to idle the device and retry setting the \a mode.

 @note On Windows, only disabling MIG mode is supported. \a activationStatus would return \ref
       NVML_ERROR_NOT_SUPPORTED as GPU reset is not supported on Windows through this API.

 @param device                               The identifier of the target device
 @param mode                                 The mode to be set, \ref NVML_DEVICE_MIG_DISABLE or
                                             \ref NVML_DEVICE_MIG_ENABLE
 @param activationStatus                     The activationStatus status

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device,\a mode or \a activationStatus are invalid
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't support MIG mode*/
    fn nvmlDeviceSetMigMode(
        device: cuda_types::nvml::nvmlDevice_t,
        mode: ::core::ffi::c_uint,
        activationStatus: *mut cuda_types::nvml::nvmlReturn_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get MIG mode for the device.

 For Ampere &tm; or newer fully supported devices.

 Changing MIG modes may require device unbind or reset. The "pending" MIG mode refers to the target mode following the
 next activation trigger.

 @param device                               The identifier of the target device
 @param currentMode                          Returns the current mode, \ref NVML_DEVICE_MIG_DISABLE or
                                             \ref NVML_DEVICE_MIG_ENABLE
 @param pendingMode                          Returns the pending mode, \ref NVML_DEVICE_MIG_DISABLE or
                                             \ref NVML_DEVICE_MIG_ENABLE

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device, \a currentMode or \a pendingMode are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't support MIG mode*/
    fn nvmlDeviceGetMigMode(
        device: cuda_types::nvml::nvmlDevice_t,
        currentMode: *mut ::core::ffi::c_uint,
        pendingMode: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get GPU instance profile information

 Information provided by this API is immutable throughout the lifetime of a MIG mode.

 @note This API can be used to enumerate all MIG profiles supported by NVML in a forward compatible
 way by invoking it on \a profile values starting from 0, until the API returns \ref NVML_ERROR_INVALID_ARGUMENT.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               The identifier of the target device
 @param profile                              One of the NVML_GPU_INSTANCE_PROFILE_*
 @param info                                 Returns detailed profile information

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device, \a profile or \a info are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't support MIG or \a profile isn't supported
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlDeviceGetGpuInstanceProfileInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        profile: ::core::ffi::c_uint,
        info: *mut cuda_types::nvml::nvmlGpuInstanceProfileInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Versioned wrapper around \ref nvmlDeviceGetGpuInstanceProfileInfo that accepts a versioned
 \ref nvmlGpuInstanceProfileInfo_v2_t or later output structure.

 @note The caller must set the \ref nvmlGpuInstanceProfileInfo_v2_t.version field to the
 appropriate version prior to calling this function. For example:
 \code
     nvmlGpuInstanceProfileInfo_v2_t profileInfo =
         { .version = nvmlGpuInstanceProfileInfo_v2 };
     nvmlReturn_t result = nvmlDeviceGetGpuInstanceProfileInfoV(device,
                                                                profile,
                                                                &profileInfo);
 \endcode

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               The identifier of the target device
 @param profile                              One of the NVML_GPU_INSTANCE_PROFILE_*
 @param info                                 Returns detailed profile information

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device, \a profile, \a info, or \a info->version are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't have MIG mode enabled or \a profile isn't supported
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlDeviceGetGpuInstanceProfileInfoV(
        device: cuda_types::nvml::nvmlDevice_t,
        profile: ::core::ffi::c_uint,
        info: *mut cuda_types::nvml::nvmlGpuInstanceProfileInfo_v2_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get GPU instance placements.

 A placement represents the location of a GPU instance within a device. This API only returns all the possible
 placements for the given profile regardless of whether MIG is enabled or not.
 A created GPU instance occupies memory slices described by its placement. Creation of new GPU instance will
 fail if there is overlap with the already occupied memory slices.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 @param device                               The identifier of the target device
 @param profileId                            The GPU instance profile ID. See \ref nvmlDeviceGetGpuInstanceProfileInfo
 @param placements                           Returns placements allowed for the profile. Can be NULL to discover number
                                             of allowed placements for this profile. If non-NULL must be large enough
                                             to accommodate the placements supported by the profile.
 @param count                                Returns number of allowed placemenets for the profile.

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device, \a profileId or \a count are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't support MIG or \a profileId isn't supported
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlDeviceGetGpuInstancePossiblePlacements_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        profileId: ::core::ffi::c_uint,
        placements: *mut cuda_types::nvml::nvmlGpuInstancePlacement_t,
        count: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get GPU instance profile capacity.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 @param device                               The identifier of the target device
 @param profileId                            The GPU instance profile ID. See \ref nvmlDeviceGetGpuInstanceProfileInfo
 @param count                                Returns remaining instance count for the profile ID

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device, \a profileId or \a count are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't have MIG mode enabled or \a profileId isn't supported
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlDeviceGetGpuInstanceRemainingCapacity(
        device: cuda_types::nvml::nvmlDevice_t,
        profileId: ::core::ffi::c_uint,
        count: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Create GPU instance.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 If the parent device is unbound, reset or the GPU instance is destroyed explicitly, the GPU instance handle would
 become invalid. The GPU instance must be recreated to acquire a valid handle.

 @param device                               The identifier of the target device
 @param profileId                            The GPU instance profile ID. See \ref nvmlDeviceGetGpuInstanceProfileInfo
 @param gpuInstance                          Returns the GPU instance handle

 @return
         - \ref NVML_SUCCESS                       Upon success
         - \ref NVML_ERROR_UNINITIALIZED           If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT        If \a device, \a profile, \a profileId or \a gpuInstance are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED           If \a device doesn't have MIG mode enabled or in vGPU guest
         - \ref NVML_ERROR_NO_PERMISSION           If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_INSUFFICIENT_RESOURCES  If the requested GPU instance could not be created*/
    fn nvmlDeviceCreateGpuInstance(
        device: cuda_types::nvml::nvmlDevice_t,
        profileId: ::core::ffi::c_uint,
        gpuInstance: *mut cuda_types::nvml::nvmlGpuInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Create GPU instance with the specified placement.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 If the parent device is unbound, reset or the GPU instance is destroyed explicitly, the GPU instance handle would
 become invalid. The GPU instance must be recreated to acquire a valid handle.

 @param device                               The identifier of the target device
 @param profileId                            The GPU instance profile ID. See \ref nvmlDeviceGetGpuInstanceProfileInfo
 @param placement                            The requested placement. See \ref nvmlDeviceGetGpuInstancePossiblePlacements_v2
 @param gpuInstance                          Returns the GPU instance handle

 @return
         - \ref NVML_SUCCESS                       Upon success
         - \ref NVML_ERROR_UNINITIALIZED           If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT        If \a device, \a profile, \a profileId, \a placement or \a gpuInstance
                                                   are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED           If \a device doesn't have MIG mode enabled or in vGPU guest
         - \ref NVML_ERROR_NO_PERMISSION           If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_INSUFFICIENT_RESOURCES  If the requested GPU instance could not be created*/
    fn nvmlDeviceCreateGpuInstanceWithPlacement(
        device: cuda_types::nvml::nvmlDevice_t,
        profileId: ::core::ffi::c_uint,
        placement: *const cuda_types::nvml::nvmlGpuInstancePlacement_t,
        gpuInstance: *mut cuda_types::nvml::nvmlGpuInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Destroy GPU instance.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 @param gpuInstance                          The GPU instance handle

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a gpuInstance is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't have MIG mode enabled or in vGPU guest
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_IN_USE            If the GPU instance is in use. This error would be returned if processes
                                             (e.g. CUDA application) or compute instances are active on the
                                             GPU instance.*/
    fn nvmlGpuInstanceDestroy(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get GPU instances for given profile ID.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 @param device                               The identifier of the target device
 @param profileId                            The GPU instance profile ID. See \ref nvmlDeviceGetGpuInstanceProfileInfo
 @param gpuInstances                         Returns pre-exiting GPU instances, the buffer must be large enough to
                                             accommodate the instances supported by the profile.
                                             See \ref nvmlDeviceGetGpuInstanceProfileInfo
 @param count                                The count of returned GPU instances

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device, \a profileId, \a gpuInstances or \a count are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't have MIG mode enabled
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlDeviceGetGpuInstances(
        device: cuda_types::nvml::nvmlDevice_t,
        profileId: ::core::ffi::c_uint,
        gpuInstances: *mut cuda_types::nvml::nvmlGpuInstance_t,
        count: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get GPU instances for given instance ID.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 @param device                               The identifier of the target device
 @param id                                   The GPU instance ID
 @param gpuInstance                          Returns GPU instance

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device, \a id or \a gpuInstance are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't have MIG mode enabled
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_NOT_FOUND         If the GPU instance is not found.*/
    fn nvmlDeviceGetGpuInstanceById(
        device: cuda_types::nvml::nvmlDevice_t,
        id: ::core::ffi::c_uint,
        gpuInstance: *mut cuda_types::nvml::nvmlGpuInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get GPU instance information.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param gpuInstance                          The GPU instance handle
 @param info                                 Return GPU instance information

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a gpuInstance or \a info are invalid
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlGpuInstanceGetInfo(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        info: *mut cuda_types::nvml::nvmlGpuInstanceInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get compute instance profile information.

 Information provided by this API is immutable throughout the lifetime of a MIG mode.

 @note This API can be used to enumerate all MIG profiles supported by NVML in a forward compatible
 way by invoking it on \a profile values starting from 0, until the API returns \ref NVML_ERROR_INVALID_ARGUMENT.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param gpuInstance                          The identifier of the target GPU instance
 @param profile                              One of the NVML_COMPUTE_INSTANCE_PROFILE_*
 @param engProfile                           One of the NVML_COMPUTE_INSTANCE_ENGINE_PROFILE_*
 @param info                                 Returns detailed profile information

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a gpuInstance, \a profile, \a engProfile or \a info are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a profile isn't supported
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlGpuInstanceGetComputeInstanceProfileInfo(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        profile: ::core::ffi::c_uint,
        engProfile: ::core::ffi::c_uint,
        info: *mut cuda_types::nvml::nvmlComputeInstanceProfileInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Versioned wrapper around \ref nvmlGpuInstanceGetComputeInstanceProfileInfo that accepts a versioned
 \ref nvmlComputeInstanceProfileInfo_v2_t or later output structure.

 @note The caller must set the \ref nvmlGpuInstanceProfileInfo_v2_t.version field to the
 appropriate version prior to calling this function. For example:
 \code
     nvmlComputeInstanceProfileInfo_v2_t profileInfo =
         { .version = nvmlComputeInstanceProfileInfo_v2 };
     nvmlReturn_t result = nvmlGpuInstanceGetComputeInstanceProfileInfoV(gpuInstance,
                                                                         profile,
                                                                         engProfile,
                                                                         &profileInfo);
 \endcode

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param gpuInstance                          The identifier of the target GPU instance
 @param profile                              One of the NVML_COMPUTE_INSTANCE_PROFILE_*
 @param engProfile                           One of the NVML_COMPUTE_INSTANCE_ENGINE_PROFILE_*
 @param info                                 Returns detailed profile information

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a gpuInstance, \a profile, \a engProfile, \a info, or \a info->version are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a profile isn't supported
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlGpuInstanceGetComputeInstanceProfileInfoV(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        profile: ::core::ffi::c_uint,
        engProfile: ::core::ffi::c_uint,
        info: *mut cuda_types::nvml::nvmlComputeInstanceProfileInfo_v2_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get compute instance profile capacity.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 @param gpuInstance                          The identifier of the target GPU instance
 @param profileId                            The compute instance profile ID.
                                             See \ref nvmlGpuInstanceGetComputeInstanceProfileInfo
 @param count                                Returns remaining instance count for the profile ID

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a gpuInstance, \a profileId or \a availableCount are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a profileId isn't supported
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlGpuInstanceGetComputeInstanceRemainingCapacity(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        profileId: ::core::ffi::c_uint,
        count: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get compute instance placements.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 A placement represents the location of a compute instance within a GPU instance. This API only returns all the possible
 placements for the given profile.
 A created compute instance occupies compute slices described by its placement. Creation of new compute instance will
 fail if there is overlap with the already occupied compute slices.

 @param gpuInstance                          The identifier of the target GPU instance
 @param profileId                            The compute instance profile ID. See \ref  nvmlGpuInstanceGetComputeInstanceProfileInfo
 @param placements                           Returns placements allowed for the profile. Can be NULL to discover number
                                             of allowed placements for this profile. If non-NULL must be large enough
                                             to accommodate the placements supported by the profile.
 @param count                                Returns number of allowed placemenets for the profile.

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a gpuInstance, \a profileId or \a count are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't have MIG mode enabled or \a profileId isn't supported
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlGpuInstanceGetComputeInstancePossiblePlacements(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        profileId: ::core::ffi::c_uint,
        placements: *mut cuda_types::nvml::nvmlComputeInstancePlacement_t,
        count: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Create compute instance.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 If the parent device is unbound, reset or the parent GPU instance is destroyed or the compute instance is destroyed
 explicitly, the compute instance handle would become invalid. The compute instance must be recreated to acquire
 a valid handle.

 @param gpuInstance                          The identifier of the target GPU instance
 @param profileId                            The compute instance profile ID.
                                             See \ref nvmlGpuInstanceGetComputeInstanceProfileInfo
 @param computeInstance                      Returns the compute instance handle

 @return
         - \ref NVML_SUCCESS                       Upon success
         - \ref NVML_ERROR_UNINITIALIZED           If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT        If \a gpuInstance, \a profile, \a profileId or \a computeInstance
                                                   are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED           If \a profileId isn't supported
         - \ref NVML_ERROR_NO_PERMISSION           If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_INSUFFICIENT_RESOURCES  If the requested compute instance could not be created*/
    fn nvmlGpuInstanceCreateComputeInstance(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        profileId: ::core::ffi::c_uint,
        computeInstance: *mut cuda_types::nvml::nvmlComputeInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Create compute instance with the specified placement.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 If the parent device is unbound, reset or the parent GPU instance is destroyed or the compute instance is destroyed
 explicitly, the compute instance handle would become invalid. The compute instance must be recreated to acquire
 a valid handle.

 @param gpuInstance                          The identifier of the target GPU instance
 @param profileId                            The compute instance profile ID.
                                             See \ref nvmlGpuInstanceGetComputeInstanceProfileInfo
 @param placement                            The requested placement. See \ref nvmlGpuInstanceGetComputeInstancePossiblePlacements
 @param computeInstance                      Returns the compute instance handle

 @return
         - \ref NVML_SUCCESS                       Upon success
         - \ref NVML_ERROR_UNINITIALIZED           If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT        If \a gpuInstance, \a profile, \a profileId or \a computeInstance
                                                   are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED           If \a profileId isn't supported
         - \ref NVML_ERROR_NO_PERMISSION           If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_INSUFFICIENT_RESOURCES  If the requested compute instance could not be created*/
    fn nvmlGpuInstanceCreateComputeInstanceWithPlacement(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        profileId: ::core::ffi::c_uint,
        placement: *const cuda_types::nvml::nvmlComputeInstancePlacement_t,
        computeInstance: *mut cuda_types::nvml::nvmlComputeInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Destroy compute instance.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 @param computeInstance                      The compute instance handle

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a computeInstance is invalid
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_IN_USE            If the compute instance is in use. This error would be returned if
                                             processes (e.g. CUDA application) are active on the compute instance.*/
    fn nvmlComputeInstanceDestroy(
        computeInstance: cuda_types::nvml::nvmlComputeInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get compute instances for given profile ID.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 @param gpuInstance                          The identifier of the target GPU instance
 @param profileId                            The compute instance profile ID.
                                             See \ref nvmlGpuInstanceGetComputeInstanceProfileInfo
 @param computeInstances                     Returns pre-exiting compute instances, the buffer must be large enough to
                                             accommodate the instances supported by the profile.
                                             See \ref nvmlGpuInstanceGetComputeInstanceProfileInfo
 @param count                                The count of returned compute instances

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a gpuInstance, \a profileId, \a computeInstances or \a count
                                             are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a profileId isn't supported
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlGpuInstanceGetComputeInstances(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        profileId: ::core::ffi::c_uint,
        computeInstances: *mut cuda_types::nvml::nvmlComputeInstance_t,
        count: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get compute instance for given instance ID.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.
 Requires privileged user.

 @param gpuInstance                          The identifier of the target GPU instance
 @param id                                   The compute instance ID
 @param computeInstance                      Returns compute instance

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a device, \a ID or \a computeInstance are invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     If \a device doesn't have MIG mode enabled
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation
         - \ref NVML_ERROR_NOT_FOUND         If the compute instance is not found.*/
    fn nvmlGpuInstanceGetComputeInstanceById(
        gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
        id: ::core::ffi::c_uint,
        computeInstance: *mut cuda_types::nvml::nvmlComputeInstance_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get compute instance information.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param computeInstance                      The compute instance handle
 @param info                                 Return compute instance information

 @return
         - \ref NVML_SUCCESS                 Upon success
         - \ref NVML_ERROR_UNINITIALIZED     If library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  If \a computeInstance or \a info are invalid
         - \ref NVML_ERROR_NO_PERMISSION     If user doesn't have permission to perform the operation*/
    fn nvmlComputeInstanceGetInfo_v2(
        computeInstance: cuda_types::nvml::nvmlComputeInstance_t,
        info: *mut cuda_types::nvml::nvmlComputeInstanceInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Test if the given handle refers to a MIG device.

 A MIG device handle is an NVML abstraction which maps to a MIG compute instance.
 These overloaded references can be used (with some restrictions) interchangeably
 with a GPU device handle to execute queries at a per-compute instance granularity.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               NVML handle to test
 @param isMigDevice                          True when handle refers to a MIG device

 @return
         - \ref NVML_SUCCESS                 if \a device status was successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device handle or \a isMigDevice reference is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this check is not supported by the device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceIsMigDeviceHandle(
        device: cuda_types::nvml::nvmlDevice_t,
        isMigDevice: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get GPU instance ID for the given MIG device handle.

 GPU instance IDs are unique per device and remain valid until the GPU instance is destroyed.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               Target MIG device handle
 @param id                                   GPU instance ID

 @return
         - \ref NVML_SUCCESS                 if instance ID was successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a id reference is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetGpuInstanceId(
        device: cuda_types::nvml::nvmlDevice_t,
        id: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get compute instance ID for the given MIG device handle.

 Compute instance IDs are unique per GPU instance and remain valid until the compute instance
 is destroyed.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               Target MIG device handle
 @param id                                   Compute instance ID

 @return
         - \ref NVML_SUCCESS                 if instance ID was successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a id reference is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetComputeInstanceId(
        device: cuda_types::nvml::nvmlDevice_t,
        id: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get the maximum number of MIG devices that can exist under a given parent NVML device.

 Returns zero if MIG is not supported or enabled.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               Target device handle
 @param count                                Count of MIG devices

 @return
         - \ref NVML_SUCCESS                 if \a count was successfully retrieved
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device or \a count reference is invalid
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMaxMigDeviceCount(
        device: cuda_types::nvml::nvmlDevice_t,
        count: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get MIG device handle for the given index under its parent NVML device.

 If the compute instance is destroyed either explicitly or by destroying,
 resetting or unbinding the parent GPU instance or the GPU device itself
 the MIG device handle would remain invalid and must be requested again
 using this API. Handles may be reused and their properties can change in
 the process.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param device                               Reference to the parent GPU device handle
 @param index                                Index of the MIG device
 @param migDevice                            Reference to the MIG device handle

 @return
         - \ref NVML_SUCCESS                 if \a migDevice handle was successfully created
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device, \a index or \a migDevice reference is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_NOT_FOUND         if no valid MIG device was found at \a index
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetMigDeviceHandleByIndex(
        device: cuda_types::nvml::nvmlDevice_t,
        index: ::core::ffi::c_uint,
        migDevice: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get parent device handle from a MIG device handle.

 For Ampere &tm; or newer fully supported devices.
 Supported on Linux only.

 @param migDevice                            MIG device handle
 @param device                               Device handle

 @return
         - \ref NVML_SUCCESS                 if \a device handle was successfully created
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a migDevice or \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device
         - \ref NVML_ERROR_UNKNOWN           on any unexpected error*/
    fn nvmlDeviceGetDeviceHandleFromMigDeviceHandle(
        migDevice: cuda_types::nvml::nvmlDevice_t,
        device: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Calculate GPM metrics from two samples.

 For Hopper &tm; or newer fully supported devices.

 To retrieve metrics, the user must first allocate the two sample buffers at \a metricsGet->sample1
 and \a metricsGet->sample2 by calling \a nvmlGpmSampleAlloc(). Next, the user should fill in the ID of each metric
 in \a metricsGet->metrics[i].metricId and specify the total number of metrics to retrieve in \a metricsGet->numMetrics,
 The version should be set to NVML_GPM_METRICS_GET_VERSION in \a metricsGet->version. The user then calls the
 \a nvmlGpmSampleGet() API twice to obtain 2 samples of counters. \note that the interval between these
 two \a nvmlGpmSampleGet() calls should be greater than 100ms due to the internal sample refresh rate.
 Finally, the user calls \a nvmlGpmMetricsGet to retrieve the metrics, which will be stored at \a metricsGet->metrics

 @param metricsGet             IN/OUT: populated \a nvmlGpmMetricsGet_t struct

 @return
         - \ref NVML_SUCCESS on success
         - Nonzero NVML_ERROR_? enum on error*/
    fn nvmlGpmMetricsGet(
        metricsGet: *mut cuda_types::nvml::nvmlGpmMetricsGet_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Free an allocated sample buffer that was allocated with \ref nvmlGpmSampleAlloc()

 For Hopper &tm; or newer fully supported devices.

 @param gpmSample              Sample to free

 @return
         - \ref NVML_SUCCESS                on success
         - \ref NVML_ERROR_INVALID_ARGUMENT if an invalid pointer is provided*/
    fn nvmlGpmSampleFree(
        gpmSample: cuda_types::nvml::nvmlGpmSample_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Allocate a sample buffer to be used with NVML GPM . You will need to allocate
 at least two of these buffers to use with the NVML GPM feature

 For Hopper &tm; or newer fully supported devices.

 @param gpmSample             Where  the allocated sample will be stored

 @return
         - \ref NVML_SUCCESS                on success
         - \ref NVML_ERROR_INVALID_ARGUMENT if an invalid pointer is provided
         - \ref NVML_ERROR_MEMORY           if system memory is insufficient*/
    fn nvmlGpmSampleAlloc(
        gpmSample: *mut cuda_types::nvml::nvmlGpmSample_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Read a sample of GPM metrics into the provided \a gpmSample buffer. After
 two samples are gathered, you can call nvmlGpmMetricGet on those samples to
 retrive metrics

 For Hopper &tm; or newer fully supported devices.

 @note The interval between two \a nvmlGpmSampleGet() calls should be greater than 100ms due to
 the internal sample refresh rate.

 @param device                Device to get samples for
 @param gpmSample             Buffer to read samples into

 @return
         - \ref NVML_SUCCESS on success
         - Nonzero NVML_ERROR_? enum on error*/
    fn nvmlGpmSampleGet(
        device: cuda_types::nvml::nvmlDevice_t,
        gpmSample: cuda_types::nvml::nvmlGpmSample_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Read a sample of GPM metrics into the provided \a gpmSample buffer for a MIG GPU Instance.

 After two samples are gathered, you can call nvmlGpmMetricGet on those
 samples to retrive metrics

 For Hopper &tm; or newer fully supported devices.

 @note The interval between two \a nvmlGpmMigSampleGet() calls should be greater than 100ms due to
 the internal sample refresh rate.

 @param device                Device to get samples for
 @param gpuInstanceId         MIG GPU Instance ID
 @param gpmSample             Buffer to read samples into

 @return
         - \ref NVML_SUCCESS on success
         - Nonzero NVML_ERROR_? enum on error*/
    fn nvmlGpmMigSampleGet(
        device: cuda_types::nvml::nvmlDevice_t,
        gpuInstanceId: ::core::ffi::c_uint,
        gpmSample: cuda_types::nvml::nvmlGpmSample_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Indicate whether the supplied device supports GPM

 For Hopper &tm; or newer fully supported devices.

 @param device                NVML device to query for
 @param gpmSupport            Structure to indicate GPM support \a nvmlGpmSupport_t. Indicates
                              GPM support per system for the supplied device

 @return
         - NVML_SUCCESS on success
         - Nonzero NVML_ERROR_? enum if there is an error in processing the query*/
    fn nvmlGpmQueryDeviceSupport(
        device: cuda_types::nvml::nvmlDevice_t,
        gpmSupport: *mut cuda_types::nvml::nvmlGpmSupport_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get GPM stream state.

 For Hopper &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param device                               The identifier of the target device
 @param state                                Returns GPM stream state
                                             NVML_FEATURE_DISABLED or NVML_FEATURE_ENABLED

 @return
         - \ref NVML_SUCCESS                 if \a current GPM stream state were successfully queried
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a  device is invalid or \a state is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device*/
    fn nvmlGpmQueryIfStreamingEnabled(
        device: cuda_types::nvml::nvmlDevice_t,
        state: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set GPM stream state.

 For Hopper &tm; or newer fully supported devices.
 Supported on Linux, Windows TCC.

 @param device                               The identifier of the target device
 @param state                                GPM stream state,
                                             NVML_FEATURE_DISABLED or NVML_FEATURE_ENABLED

 @return
         - \ref NVML_SUCCESS                 if \a current GPM stream state is successfully set
         - \ref NVML_ERROR_UNINITIALIZED     if the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT  if \a device is invalid
         - \ref NVML_ERROR_NOT_SUPPORTED     if this query is not supported by the device*/
    fn nvmlGpmSetStreamingEnabled(
        device: cuda_types::nvml::nvmlDevice_t,
        state: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get device capabilities

 See \ref  nvmlDeviceCapabilities_v1_t for more information on the struct.

 @param device                               The identifier of the target device
 @param caps                                 Returns GPU's capabilities

 @return
         - \ref NVML_SUCCESS                         If the query is success
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device is invalid or \a counters is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED             If the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST               If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_UNKNOWN                   On any unexpected error*/
    fn nvmlDeviceGetCapabilities(
        device: cuda_types::nvml::nvmlDevice_t,
        caps: *mut cuda_types::nvml::nvmlDeviceCapabilities_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Performance Profiles Information

 %BLACKWELL_OR_NEWER%
 See \ref nvmlWorkloadPowerProfileProfilesInfo_v1_t for more information on the struct.
 The mask \a perfProfilesMask is bitmask of all supported mode indices where the
 mode is supported if the index is 1. Each supported mode will have a corresponding
 entry in the \a perfProfile array which will contain the \a profileId, the
 \a priority of this mode, where the lower the value, the higher the priority,
 and a \a conflictingMask, where each bit set in the mask corresponds to a different
 profile which cannot be used in conjunction with the given profile.

 @param device                               The identifier of the target device
 @param profilesInfo                         Reference to struct \a nvmlWorkloadPowerProfileProfilesInfo_t

 @return
         - \ref NVML_SUCCESS                         If the query is successful
         - \ref NVML_ERROR_INSUFFICIENT_SIZE         If struct is fully allocated
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device is invalid or \a pointer to struct is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED             If the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST               If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_UNKNOWN                   On any unexpected error*/
    fn nvmlDeviceWorkloadPowerProfileGetProfilesInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        profilesInfo: *mut cuda_types::nvml::nvmlWorkloadPowerProfileProfilesInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Get Current Performance Profiles

 %BLACKWELL_OR_NEWER%
 See \ref nvmlWorkloadPowerProfileCurrentProfiles_v1_t for more information on the struct.
 This API returns a stuct which contains the current \a perfProfilesMask,
 \a requestedProfilesMask and \a enforcedProfilesMask. Each bit set in each
 bitmasks indicates the profile is supported, currently requested or currently
 engaged, respectively.

 @param device                The identifier of the target device
 @param currentProfiles       Reference to struct \a nvmlWorkloadPowerProfileCurrentProfiles_v1_t

 @return
         - \ref NVML_SUCCESS                         If the query is successful
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device is invalid or the pointer to struct is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED             If the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST               If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_UNKNOWN                   On any unexpected error*/
    fn nvmlDeviceWorkloadPowerProfileGetCurrentProfiles(
        device: cuda_types::nvml::nvmlDevice_t,
        currentProfiles: *mut cuda_types::nvml::nvmlWorkloadPowerProfileCurrentProfiles_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Set Requested Performance Profiles

 %BLACKWELL_OR_NEWER%
 See \ref nvmlWorkloadPowerProfileRequestedProfiles_v1_t for more information on the struct.
 Reuqest one or more performance profiles be activated using the input bitmask
 \a requestedProfilesMask, where each bit set corresponds to a supported bit from
 the \a perfProfilesMask. These profiles will be added to existing list of
 currently requested profiles.
 Requires root/admin permissions.

 @param device                The identifier of the target device
 @param requestedProfiles     Reference to struct \a nvmlWorkloadPowerProfileRequestedProfiles_v1_t

 @return
         - \ref NVML_SUCCESS                         If the query is successful
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device is invalid or \a pointer to struct is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED             If the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST               If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_UNKNOWN                   On any unexpected error*/
    fn nvmlDeviceWorkloadPowerProfileSetRequestedProfiles(
        device: cuda_types::nvml::nvmlDevice_t,
        requestedProfiles: *mut cuda_types::nvml::nvmlWorkloadPowerProfileRequestedProfiles_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Clear Requested Performance Profiles

 %BLACKWELL_OR_NEWER%
 See \ref nvmlWorkloadPowerProfileRequestedProfiles_v1_t for more information on the struct.
 Clear one or more performance profiles be using the input bitmask
 \a requestedProfilesMask, where each bit set corresponds to a supported bit from
 the \a perfProfilesMask. These profiles will be removed from the existing list of
 currently requested profiles.
 Requires root/admin permissions.

 @param device                The identifier of the target device
 @param requestedProfiles     Reference to struct \a nvmlWorkloadPowerProfileRequestedProfiles_v1_t

 @return
         - \ref NVML_SUCCESS                         If the query is successful
         - \ref NVML_ERROR_UNINITIALIZED             If the library has not been successfully initialized
         - \ref NVML_ERROR_INVALID_ARGUMENT          If \a device is invalid or \a pointer to struct is NULL
         - \ref NVML_ERROR_NOT_SUPPORTED             If the device does not support this feature
         - \ref NVML_ERROR_GPU_IS_LOST               If the target GPU has fallen off the bus or is otherwise inaccessible
         - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH If the provided version is invalid/unsupported
         - \ref NVML_ERROR_UNKNOWN                   On any unexpected error*/
    fn nvmlDeviceWorkloadPowerProfileClearRequestedProfiles(
        device: cuda_types::nvml::nvmlDevice_t,
        requestedProfiles: *mut cuda_types::nvml::nvmlWorkloadPowerProfileRequestedProfiles_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Activiate a specific preset profile for datacenter power smoothing.
 The API only sets the active preset profile based on the input profileId,
 and ignores the other parameters of the structure.
 Requires root/admin permissions.

 %BLACKWELL_OR_NEWER%

 @param device                                The identifier of the target device
 @param profile                               Reference to \ref nvmlPowerSmoothingProfile_v1_t.
                                              Note that only \a profile->profileId is used and
                                              the rest of the structure is ignored.

 @return
        - \ref NVML_SUCCESS                   if the Desired Profile was successfully set
        - \ref NVML_ERROR_INVALID_ARGUMENT    if device is invalid or structure was NULL
        - \ref NVML_ERROR_NO_PERMISSION       if user does not have permission to change the profile number
        - \ref NVML_ERROR_NOT_SUPPORTED       if this feature is not supported by the device
*/
    fn nvmlDevicePowerSmoothingActivatePresetProfile(
        device: cuda_types::nvml::nvmlDevice_t,
        profile: *mut cuda_types::nvml::nvmlPowerSmoothingProfile_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Update the value of a specific profile parameter contained within \ref nvmlPowerSmoothingProfile_v1_t.
 Requires root/admin permissions.

 %BLACKWELL_OR_NEWER%

 NVML_POWER_SMOOTHING_PROFILE_PARAM_PERCENT_TMP_FLOOR expects a value as a percentage from 00.00-100.00%
 NVML_POWER_SMOOTHING_PROFILE_PARAM_RAMP_UP_RATE expects a value in W/s
 NVML_POWER_SMOOTHING_PROFILE_PARAM_RAMP_DOWN_RATE expects a value in W/s
 NVML_POWER_SMOOTHING_PROFILE_PARAM_RAMP_DOWN_HYSTERESIS expects a value in ms

 @param device                                      The identifier of the target device
 @param profile                                     Reference to \ref nvmlPowerSmoothingProfile_v1_t struct

 @return
        - \ref NVML_SUCCESS                         if the Active Profile was successfully set
        - \ref NVML_ERROR_INVALID_ARGUMENT          if device is invalid or profile parameter/value was invalid
        - \ref NVML_ERROR_NO_PERMISSION             if user does not have permission to change any profile parameters
        - \ref NVML_ERROR_ARGUMENT_VERSION_MISMATCH if the structure version is not supported
*/
    fn nvmlDevicePowerSmoothingUpdatePresetProfileParam(
        device: cuda_types::nvml::nvmlDevice_t,
        profile: *mut cuda_types::nvml::nvmlPowerSmoothingProfile_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    /** Enable or disable the Power Smoothing Feature.
 Requires root/admin permissions.

 %BLACKWELL_OR_NEWER%

 See \ref nvmlEnableState_t for details on allowed states

 @param device                                      The identifier of the target device
 @param state                                       Reference to \ref nvmlPowerSmoothingState_v1_t

 @return
        - \ref NVML_SUCCESS                         if the feature state was successfully set
        - \ref NVML_ERROR_INVALID_ARGUMENT          if device is invalid or state is NULL
        - \ref NVML_ERROR_NO_PERMISSION             if user does not have permission to change feature state
        - \ref NVML_ERROR_NOT_SUPPORTED             if this feature is not supported by the device
*/
    fn nvmlDevicePowerSmoothingSetState(
        device: cuda_types::nvml::nvmlDevice_t,
        state: *mut cuda_types::nvml::nvmlPowerSmoothingState_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlInit() -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetCount(
        deviceCount: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetHandleByIndex(
        index: ::core::ffi::c_uint,
        device: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetHandleByPciBusId(
        pciBusId: *const ::core::ffi::c_char,
        device: *mut cuda_types::nvml::nvmlDevice_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetPciInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        pci: *mut cuda_types::nvml::nvmlPciInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetPciInfo_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        pci: *mut cuda_types::nvml::nvmlPciInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetNvLinkRemotePciInfo(
        device: cuda_types::nvml::nvmlDevice_t,
        link: ::core::ffi::c_uint,
        pci: *mut cuda_types::nvml::nvmlPciInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetGridLicensableFeatures(
        device: cuda_types::nvml::nvmlDevice_t,
        pGridLicensableFeatures: *mut cuda_types::nvml::nvmlGridLicensableFeatures_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetGridLicensableFeatures_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        pGridLicensableFeatures: *mut cuda_types::nvml::nvmlGridLicensableFeatures_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetGridLicensableFeatures_v3(
        device: cuda_types::nvml::nvmlDevice_t,
        pGridLicensableFeatures: *mut cuda_types::nvml::nvmlGridLicensableFeatures_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceRemoveGpu(
        pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlEventSetWait(
        set: cuda_types::nvml::nvmlEventSet_t,
        data: *mut cuda_types::nvml::nvmlEventData_t,
        timeoutms: ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetAttributes(
        device: cuda_types::nvml::nvmlDevice_t,
        attributes: *mut cuda_types::nvml::nvmlDeviceAttributes_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlComputeInstanceGetInfo(
        computeInstance: cuda_types::nvml::nvmlComputeInstance_t,
        info: *mut cuda_types::nvml::nvmlComputeInstanceInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetComputeRunningProcesses(
        device: cuda_types::nvml::nvmlDevice_t,
        infoCount: *mut ::core::ffi::c_uint,
        infos: *mut cuda_types::nvml::nvmlProcessInfo_v1_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetComputeRunningProcesses_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        infoCount: *mut ::core::ffi::c_uint,
        infos: *mut cuda_types::nvml::nvmlProcessInfo_v2_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetGraphicsRunningProcesses(
        device: cuda_types::nvml::nvmlDevice_t,
        infoCount: *mut ::core::ffi::c_uint,
        infos: *mut cuda_types::nvml::nvmlProcessInfo_v1_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetGraphicsRunningProcesses_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        infoCount: *mut ::core::ffi::c_uint,
        infos: *mut cuda_types::nvml::nvmlProcessInfo_v2_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetMPSComputeRunningProcesses(
        device: cuda_types::nvml::nvmlDevice_t,
        infoCount: *mut ::core::ffi::c_uint,
        infos: *mut cuda_types::nvml::nvmlProcessInfo_v1_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetMPSComputeRunningProcesses_v2(
        device: cuda_types::nvml::nvmlDevice_t,
        infoCount: *mut ::core::ffi::c_uint,
        infos: *mut cuda_types::nvml::nvmlProcessInfo_v2_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetGpuInstancePossiblePlacements(
        device: cuda_types::nvml::nvmlDevice_t,
        profileId: ::core::ffi::c_uint,
        placements: *mut cuda_types::nvml::nvmlGpuInstancePlacement_t,
        count: *mut ::core::ffi::c_uint,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlVgpuInstanceGetLicenseInfo(
        vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
        licenseInfo: *mut cuda_types::nvml::nvmlVgpuLicenseInfo_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
    #[must_use]
    fn nvmlDeviceGetDriverModel(
        device: cuda_types::nvml::nvmlDevice_t,
        current: *mut cuda_types::nvml::nvmlDriverModel_t,
        pending: *mut cuda_types::nvml::nvmlDriverModel_t,
    ) -> cuda_types::nvml::nvmlReturn_t;
}
