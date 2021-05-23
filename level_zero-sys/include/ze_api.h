/*
 *
 * Copyright (C) 2019 Intel Corporation
 *
 * SPDX-License-Identifier: MIT
 *
 * @file ze_api.h
 * @version v1.1-r1.1.10
 *
 */
#ifndef _ZE_API_H
#define _ZE_API_H
#if defined(__cplusplus)
#pragma once
#endif

// standard headers
#include <stdint.h>
#include <stddef.h>

#if defined(__cplusplus)
extern "C" {
#endif

// Intel 'oneAPI' Level-Zero API common types
#if !defined(__GNUC__)
#pragma region common
#endif
///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAKE_VERSION
/// @brief Generates generic 'oneAPI' API versions
#define ZE_MAKE_VERSION( _major, _minor )  (( _major << 16 )|( _minor & 0x0000ffff))
#endif // ZE_MAKE_VERSION

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAJOR_VERSION
/// @brief Extracts 'oneAPI' API major version
#define ZE_MAJOR_VERSION( _ver )  ( _ver >> 16 )
#endif // ZE_MAJOR_VERSION

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MINOR_VERSION
/// @brief Extracts 'oneAPI' API minor version
#define ZE_MINOR_VERSION( _ver )  ( _ver & 0x0000ffff )
#endif // ZE_MINOR_VERSION

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_APICALL
#if defined(_WIN32)
/// @brief Calling convention for all API functions
#define ZE_APICALL  __cdecl
#else
#define ZE_APICALL  
#endif // defined(_WIN32)
#endif // ZE_APICALL

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_APIEXPORT
#if defined(_WIN32)
/// @brief Microsoft-specific dllexport storage-class attribute
#define ZE_APIEXPORT  __declspec(dllexport)
#else
#define ZE_APIEXPORT  
#endif // defined(_WIN32)
#endif // ZE_APIEXPORT

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_DLLEXPORT
#if defined(_WIN32)
/// @brief Microsoft-specific dllexport storage-class attribute
#define ZE_DLLEXPORT  __declspec(dllexport)
#endif // defined(_WIN32)
#endif // ZE_DLLEXPORT

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_DLLEXPORT
#if __GNUC__ >= 4
/// @brief GCC-specific dllexport storage-class attribute
#define ZE_DLLEXPORT  __attribute__ ((visibility ("default")))
#else
#define ZE_DLLEXPORT  
#endif // __GNUC__ >= 4
#endif // ZE_DLLEXPORT

///////////////////////////////////////////////////////////////////////////////
/// @brief compiler-independent type
typedef uint8_t ze_bool_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of a driver instance
typedef struct _ze_driver_handle_t *ze_driver_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's device object
typedef struct _ze_device_handle_t *ze_device_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's context object
typedef struct _ze_context_handle_t *ze_context_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's command queue object
typedef struct _ze_command_queue_handle_t *ze_command_queue_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's command list object
typedef struct _ze_command_list_handle_t *ze_command_list_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's fence object
typedef struct _ze_fence_handle_t *ze_fence_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's event pool object
typedef struct _ze_event_pool_handle_t *ze_event_pool_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's event object
typedef struct _ze_event_handle_t *ze_event_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's image object
typedef struct _ze_image_handle_t *ze_image_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's module object
typedef struct _ze_module_handle_t *ze_module_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of module's build log object
typedef struct _ze_module_build_log_handle_t *ze_module_build_log_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's kernel object
typedef struct _ze_kernel_handle_t *ze_kernel_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of driver's sampler object
typedef struct _ze_sampler_handle_t *ze_sampler_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Handle of physical memory object
typedef struct _ze_physical_mem_handle_t *ze_physical_mem_handle_t;

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAX_IPC_HANDLE_SIZE
/// @brief Maximum IPC handle size
#define ZE_MAX_IPC_HANDLE_SIZE  64
#endif // ZE_MAX_IPC_HANDLE_SIZE

///////////////////////////////////////////////////////////////////////////////
/// @brief IPC handle to a memory allocation
typedef struct _ze_ipc_mem_handle_t
{
    char data[ZE_MAX_IPC_HANDLE_SIZE];              ///< [out] Opaque data representing an IPC handle

} ze_ipc_mem_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief IPC handle to a event pool allocation
typedef struct _ze_ipc_event_pool_handle_t
{
    char data[ZE_MAX_IPC_HANDLE_SIZE];              ///< [out] Opaque data representing an IPC handle

} ze_ipc_event_pool_handle_t;

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_BIT
/// @brief Generic macro for enumerator bit masks
#define ZE_BIT( _i )  ( 1 << _i )
#endif // ZE_BIT

///////////////////////////////////////////////////////////////////////////////
/// @brief Defines Return/Error codes
typedef enum _ze_result_t
{
    ZE_RESULT_SUCCESS = 0,                          ///< [Core] success
    ZE_RESULT_NOT_READY = 1,                        ///< [Core] synchronization primitive not signaled
    ZE_RESULT_ERROR_DEVICE_LOST = 0x70000001,       ///< [Core] device hung, reset, was removed, or driver update occurred
    ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY = 0x70000002,///< [Core] insufficient host memory to satisfy call
    ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY = 0x70000003,  ///< [Core] insufficient device memory to satisfy call
    ZE_RESULT_ERROR_MODULE_BUILD_FAILURE = 0x70000004,  ///< [Core] error occurred when building module, see build log for details
    ZE_RESULT_ERROR_MODULE_LINK_FAILURE = 0x70000005,   ///< [Core] error occurred when linking modules, see build log for details
    ZE_RESULT_ERROR_INSUFFICIENT_PERMISSIONS = 0x70010000,  ///< [Sysman] access denied due to permission level
    ZE_RESULT_ERROR_NOT_AVAILABLE = 0x70010001,     ///< [Sysman] resource already in use and simultaneous access not allowed
                                                    ///< or resource was removed
    ZE_RESULT_ERROR_DEPENDENCY_UNAVAILABLE = 0x70020000,///< [Tools] external required dependency is unavailable or missing
    ZE_RESULT_ERROR_UNINITIALIZED = 0x78000001,     ///< [Validation] driver is not initialized
    ZE_RESULT_ERROR_UNSUPPORTED_VERSION = 0x78000002,   ///< [Validation] generic error code for unsupported versions
    ZE_RESULT_ERROR_UNSUPPORTED_FEATURE = 0x78000003,   ///< [Validation] generic error code for unsupported features
    ZE_RESULT_ERROR_INVALID_ARGUMENT = 0x78000004,  ///< [Validation] generic error code for invalid arguments
    ZE_RESULT_ERROR_INVALID_NULL_HANDLE = 0x78000005,   ///< [Validation] handle argument is not valid
    ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE = 0x78000006,  ///< [Validation] object pointed to by handle still in-use by device
    ZE_RESULT_ERROR_INVALID_NULL_POINTER = 0x78000007,  ///< [Validation] pointer argument may not be nullptr
    ZE_RESULT_ERROR_INVALID_SIZE = 0x78000008,      ///< [Validation] size argument is invalid (e.g., must not be zero)
    ZE_RESULT_ERROR_UNSUPPORTED_SIZE = 0x78000009,  ///< [Validation] size argument is not supported by the device (e.g., too
                                                    ///< large)
    ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT = 0x7800000a, ///< [Validation] alignment argument is not supported by the device (e.g.,
                                                    ///< too small)
    ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT = 0x7800000b,///< [Validation] synchronization object in invalid state
    ZE_RESULT_ERROR_INVALID_ENUMERATION = 0x7800000c,   ///< [Validation] enumerator argument is not valid
    ZE_RESULT_ERROR_UNSUPPORTED_ENUMERATION = 0x7800000d,   ///< [Validation] enumerator argument is not supported by the device
    ZE_RESULT_ERROR_UNSUPPORTED_IMAGE_FORMAT = 0x7800000e,  ///< [Validation] image format is not supported by the device
    ZE_RESULT_ERROR_INVALID_NATIVE_BINARY = 0x7800000f, ///< [Validation] native binary is not supported by the device
    ZE_RESULT_ERROR_INVALID_GLOBAL_NAME = 0x78000010,   ///< [Validation] global variable is not found in the module
    ZE_RESULT_ERROR_INVALID_KERNEL_NAME = 0x78000011,   ///< [Validation] kernel name is not found in the module
    ZE_RESULT_ERROR_INVALID_FUNCTION_NAME = 0x78000012, ///< [Validation] function name is not found in the module
    ZE_RESULT_ERROR_INVALID_GROUP_SIZE_DIMENSION = 0x78000013,  ///< [Validation] group size dimension is not valid for the kernel or
                                                    ///< device
    ZE_RESULT_ERROR_INVALID_GLOBAL_WIDTH_DIMENSION = 0x78000014,///< [Validation] global width dimension is not valid for the kernel or
                                                    ///< device
    ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_INDEX = 0x78000015, ///< [Validation] kernel argument index is not valid for kernel
    ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_SIZE = 0x78000016,  ///< [Validation] kernel argument size does not match kernel
    ZE_RESULT_ERROR_INVALID_KERNEL_ATTRIBUTE_VALUE = 0x78000017,///< [Validation] value of kernel attribute is not valid for the kernel or
                                                    ///< device
    ZE_RESULT_ERROR_INVALID_MODULE_UNLINKED = 0x78000018,   ///< [Validation] module with imports needs to be linked before kernels can
                                                    ///< be created from it.
    ZE_RESULT_ERROR_INVALID_COMMAND_LIST_TYPE = 0x78000019, ///< [Validation] command list type does not match command queue type
    ZE_RESULT_ERROR_OVERLAPPING_REGIONS = 0x7800001a,   ///< [Validation] copy operations do not support overlapping regions of
                                                    ///< memory
    ZE_RESULT_ERROR_UNKNOWN = 0x7ffffffe,           ///< [Core] unknown or internal error
    ZE_RESULT_FORCE_UINT32 = 0x7fffffff

} ze_result_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Defines structure types
typedef enum _ze_structure_type_t
{
    ZE_STRUCTURE_TYPE_DRIVER_PROPERTIES = 0x1,      ///< ::ze_driver_properties_t
    ZE_STRUCTURE_TYPE_DRIVER_IPC_PROPERTIES = 0x2,  ///< ::ze_driver_ipc_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_PROPERTIES = 0x3,      ///< ::ze_device_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_COMPUTE_PROPERTIES = 0x4,  ///< ::ze_device_compute_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_MODULE_PROPERTIES = 0x5,   ///< ::ze_device_module_properties_t
    ZE_STRUCTURE_TYPE_COMMAND_QUEUE_GROUP_PROPERTIES = 0x6, ///< ::ze_command_queue_group_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_MEMORY_PROPERTIES = 0x7,   ///< ::ze_device_memory_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_MEMORY_ACCESS_PROPERTIES = 0x8,///< ::ze_device_memory_access_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_CACHE_PROPERTIES = 0x9,///< ::ze_device_cache_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_IMAGE_PROPERTIES = 0xa,///< ::ze_device_image_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_P2P_PROPERTIES = 0xb,  ///< ::ze_device_p2p_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_EXTERNAL_MEMORY_PROPERTIES = 0xc,  ///< ::ze_device_external_memory_properties_t
    ZE_STRUCTURE_TYPE_CONTEXT_DESC = 0xd,           ///< ::ze_context_desc_t
    ZE_STRUCTURE_TYPE_COMMAND_QUEUE_DESC = 0xe,     ///< ::ze_command_queue_desc_t
    ZE_STRUCTURE_TYPE_COMMAND_LIST_DESC = 0xf,      ///< ::ze_command_list_desc_t
    ZE_STRUCTURE_TYPE_EVENT_POOL_DESC = 0x10,       ///< ::ze_event_pool_desc_t
    ZE_STRUCTURE_TYPE_EVENT_DESC = 0x11,            ///< ::ze_event_desc_t
    ZE_STRUCTURE_TYPE_FENCE_DESC = 0x12,            ///< ::ze_fence_desc_t
    ZE_STRUCTURE_TYPE_IMAGE_DESC = 0x13,            ///< ::ze_image_desc_t
    ZE_STRUCTURE_TYPE_IMAGE_PROPERTIES = 0x14,      ///< ::ze_image_properties_t
    ZE_STRUCTURE_TYPE_DEVICE_MEM_ALLOC_DESC = 0x15, ///< ::ze_device_mem_alloc_desc_t
    ZE_STRUCTURE_TYPE_HOST_MEM_ALLOC_DESC = 0x16,   ///< ::ze_host_mem_alloc_desc_t
    ZE_STRUCTURE_TYPE_MEMORY_ALLOCATION_PROPERTIES = 0x17,  ///< ::ze_memory_allocation_properties_t
    ZE_STRUCTURE_TYPE_EXTERNAL_MEMORY_EXPORT_DESC = 0x18,   ///< ::ze_external_memory_export_desc_t
    ZE_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMPORT_FD = 0x19, ///< ::ze_external_memory_import_fd_t
    ZE_STRUCTURE_TYPE_EXTERNAL_MEMORY_EXPORT_FD = 0x1a, ///< ::ze_external_memory_export_fd_t
    ZE_STRUCTURE_TYPE_MODULE_DESC = 0x1b,           ///< ::ze_module_desc_t
    ZE_STRUCTURE_TYPE_MODULE_PROPERTIES = 0x1c,     ///< ::ze_module_properties_t
    ZE_STRUCTURE_TYPE_KERNEL_DESC = 0x1d,           ///< ::ze_kernel_desc_t
    ZE_STRUCTURE_TYPE_KERNEL_PROPERTIES = 0x1e,     ///< ::ze_kernel_properties_t
    ZE_STRUCTURE_TYPE_SAMPLER_DESC = 0x1f,          ///< ::ze_sampler_desc_t
    ZE_STRUCTURE_TYPE_PHYSICAL_MEM_DESC = 0x20,     ///< ::ze_physical_mem_desc_t
    ZE_STRUCTURE_TYPE_DEVICE_RAYTRACING_EXT_PROPERTIES = 0x00010001,///< ::ze_device_raytracing_ext_properties_t
    ZE_STRUCTURE_TYPE_RAYTRACING_MEM_ALLOC_EXT_DESC = 0x10002,  ///< ::ze_raytracing_mem_alloc_ext_desc_t
    ZE_STRUCTURE_TYPE_FLOAT_ATOMIC_EXT_PROPERTIES = 0x10003,///< ::ze_float_atomic_ext_properties_t
    ZE_STRUCTURE_TYPE_RELAXED_ALLOCATION_LIMITS_EXP_DESC = 0x00020001,  ///< ::ze_relaxed_allocation_limits_exp_desc_t
    ZE_STRUCTURE_TYPE_MODULE_PROGRAM_EXP_DESC = 0x00020002, ///< ::ze_module_program_exp_desc_t
    ZE_STRUCTURE_TYPE_FORCE_UINT32 = 0x7fffffff

} ze_structure_type_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief External memory type flags
typedef uint32_t ze_external_memory_type_flags_t;
typedef enum _ze_external_memory_type_flag_t
{
    ZE_EXTERNAL_MEMORY_TYPE_FLAG_OPAQUE_FD = ZE_BIT(0), ///< an opaque POSIX file descriptor handle
    ZE_EXTERNAL_MEMORY_TYPE_FLAG_DMA_BUF = ZE_BIT(1),   ///< a file descriptor handle for a Linux dma_buf
    ZE_EXTERNAL_MEMORY_TYPE_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_external_memory_type_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Base for all properties types
typedef struct _ze_base_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure

} ze_base_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Base for all descriptor types
typedef struct _ze_base_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure

} ze_base_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forces driver to only report devices (and sub-devices) as specified by
///        values

///////////////////////////////////////////////////////////////////////////////
/// @brief Forces driver to report devices from lowest to highest PCI bus ID

///////////////////////////////////////////////////////////////////////////////
/// @brief Forces all shared allocations into device memory

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_ipc_mem_handle_t
typedef struct _ze_ipc_mem_handle_t ze_ipc_mem_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_ipc_event_pool_handle_t
typedef struct _ze_ipc_event_pool_handle_t ze_ipc_event_pool_handle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_base_properties_t
typedef struct _ze_base_properties_t ze_base_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_base_desc_t
typedef struct _ze_base_desc_t ze_base_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_driver_uuid_t
typedef struct _ze_driver_uuid_t ze_driver_uuid_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_driver_properties_t
typedef struct _ze_driver_properties_t ze_driver_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_driver_ipc_properties_t
typedef struct _ze_driver_ipc_properties_t ze_driver_ipc_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_driver_extension_properties_t
typedef struct _ze_driver_extension_properties_t ze_driver_extension_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_uuid_t
typedef struct _ze_device_uuid_t ze_device_uuid_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_properties_t
typedef struct _ze_device_properties_t ze_device_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_thread_t
typedef struct _ze_device_thread_t ze_device_thread_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_compute_properties_t
typedef struct _ze_device_compute_properties_t ze_device_compute_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_native_kernel_uuid_t
typedef struct _ze_native_kernel_uuid_t ze_native_kernel_uuid_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_module_properties_t
typedef struct _ze_device_module_properties_t ze_device_module_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_command_queue_group_properties_t
typedef struct _ze_command_queue_group_properties_t ze_command_queue_group_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_memory_properties_t
typedef struct _ze_device_memory_properties_t ze_device_memory_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_memory_access_properties_t
typedef struct _ze_device_memory_access_properties_t ze_device_memory_access_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_cache_properties_t
typedef struct _ze_device_cache_properties_t ze_device_cache_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_image_properties_t
typedef struct _ze_device_image_properties_t ze_device_image_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_external_memory_properties_t
typedef struct _ze_device_external_memory_properties_t ze_device_external_memory_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_p2p_properties_t
typedef struct _ze_device_p2p_properties_t ze_device_p2p_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_context_desc_t
typedef struct _ze_context_desc_t ze_context_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_command_queue_desc_t
typedef struct _ze_command_queue_desc_t ze_command_queue_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_command_list_desc_t
typedef struct _ze_command_list_desc_t ze_command_list_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_copy_region_t
typedef struct _ze_copy_region_t ze_copy_region_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_image_region_t
typedef struct _ze_image_region_t ze_image_region_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_event_pool_desc_t
typedef struct _ze_event_pool_desc_t ze_event_pool_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_event_desc_t
typedef struct _ze_event_desc_t ze_event_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_kernel_timestamp_data_t
typedef struct _ze_kernel_timestamp_data_t ze_kernel_timestamp_data_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_kernel_timestamp_result_t
typedef struct _ze_kernel_timestamp_result_t ze_kernel_timestamp_result_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_fence_desc_t
typedef struct _ze_fence_desc_t ze_fence_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_image_format_t
typedef struct _ze_image_format_t ze_image_format_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_image_desc_t
typedef struct _ze_image_desc_t ze_image_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_image_properties_t
typedef struct _ze_image_properties_t ze_image_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_mem_alloc_desc_t
typedef struct _ze_device_mem_alloc_desc_t ze_device_mem_alloc_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_host_mem_alloc_desc_t
typedef struct _ze_host_mem_alloc_desc_t ze_host_mem_alloc_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_memory_allocation_properties_t
typedef struct _ze_memory_allocation_properties_t ze_memory_allocation_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_external_memory_export_desc_t
typedef struct _ze_external_memory_export_desc_t ze_external_memory_export_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_external_memory_import_fd_t
typedef struct _ze_external_memory_import_fd_t ze_external_memory_import_fd_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_external_memory_export_fd_t
typedef struct _ze_external_memory_export_fd_t ze_external_memory_export_fd_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_module_constants_t
typedef struct _ze_module_constants_t ze_module_constants_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_module_desc_t
typedef struct _ze_module_desc_t ze_module_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_module_properties_t
typedef struct _ze_module_properties_t ze_module_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_kernel_desc_t
typedef struct _ze_kernel_desc_t ze_kernel_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_kernel_uuid_t
typedef struct _ze_kernel_uuid_t ze_kernel_uuid_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_kernel_properties_t
typedef struct _ze_kernel_properties_t ze_kernel_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_group_count_t
typedef struct _ze_group_count_t ze_group_count_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_module_program_exp_desc_t
typedef struct _ze_module_program_exp_desc_t ze_module_program_exp_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_device_raytracing_ext_properties_t
typedef struct _ze_device_raytracing_ext_properties_t ze_device_raytracing_ext_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_raytracing_mem_alloc_ext_desc_t
typedef struct _ze_raytracing_mem_alloc_ext_desc_t ze_raytracing_mem_alloc_ext_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_sampler_desc_t
typedef struct _ze_sampler_desc_t ze_sampler_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_physical_mem_desc_t
typedef struct _ze_physical_mem_desc_t ze_physical_mem_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_float_atomic_ext_properties_t
typedef struct _ze_float_atomic_ext_properties_t ze_float_atomic_ext_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Forward-declare ze_relaxed_allocation_limits_exp_desc_t
typedef struct _ze_relaxed_allocation_limits_exp_desc_t ze_relaxed_allocation_limits_exp_desc_t;


#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs
#if !defined(__GNUC__)
#pragma region driver
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Supported initialization flags
typedef uint32_t ze_init_flags_t;
typedef enum _ze_init_flag_t
{
    ZE_INIT_FLAG_GPU_ONLY = ZE_BIT(0),              ///< only initialize GPU drivers
    ZE_INIT_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_init_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Initialize the 'oneAPI' driver(s)
/// 
/// @details
///     - The application must call this function before calling any other
///       function.
///     - If this function is not called then all other functions will return
///       ::ZE_RESULT_ERROR_UNINITIALIZED.
///     - Only one instance of each driver will be initialized per process.
///     - The application may call this function multiple times with different
///       flags or environment variables enabled.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe for scenarios
///       where multiple libraries may initialize the driver(s) simultaneously.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x1 < flags`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeInit(
    ze_init_flags_t flags                           ///< [in] initialization flags.
                                                    ///< must be 0 (default) or a combination of ::ze_init_flag_t.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves driver instances
/// 
/// @details
///     - A driver represents a collection of physical devices.
///     - Multiple calls to this function will return identical driver handles,
///       in the same order.
///     - The application may pass nullptr for pDrivers when only querying the
///       number of drivers.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clGetPlatformIDs
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pCount`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDriverGet(
    uint32_t* pCount,                               ///< [in,out] pointer to the number of driver instances.
                                                    ///< if count is zero, then the loader shall update the value with the
                                                    ///< total number of drivers available.
                                                    ///< if count is greater than the number of drivers available, then the
                                                    ///< loader shall update the value with the correct number of drivers available.
    ze_driver_handle_t* phDrivers                   ///< [in,out][optional][range(0, *pCount)] array of driver instance handles.
                                                    ///< if count is less than the number of drivers available, then the loader
                                                    ///< shall only retrieve that number of drivers.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported API versions
/// 
/// @details
///     - API versions contain major and minor attributes, use
///       ::ZE_MAJOR_VERSION and ::ZE_MINOR_VERSION
typedef enum _ze_api_version_t
{
    ZE_API_VERSION_1_0 = ZE_MAKE_VERSION( 1, 0 ),   ///< version 1.0
    ZE_API_VERSION_1_1 = ZE_MAKE_VERSION( 1, 1 ),   ///< version 1.1
    ZE_API_VERSION_CURRENT = ZE_MAKE_VERSION( 1, 1 ),   ///< latest known version
    ZE_API_VERSION_FORCE_UINT32 = 0x7fffffff

} ze_api_version_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Returns the API version supported by the specified driver
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDriver`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == version`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDriverGetApiVersion(
    ze_driver_handle_t hDriver,                     ///< [in] handle of the driver instance
    ze_api_version_t* version                       ///< [out] api version
    );

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAX_DRIVER_UUID_SIZE
/// @brief Maximum driver universal unique id (UUID) size in bytes
#define ZE_MAX_DRIVER_UUID_SIZE  16
#endif // ZE_MAX_DRIVER_UUID_SIZE

///////////////////////////////////////////////////////////////////////////////
/// @brief Driver universal unique id (UUID)
typedef struct _ze_driver_uuid_t
{
    uint8_t id[ZE_MAX_DRIVER_UUID_SIZE];            ///< [out] opaque data representing a driver UUID

} ze_driver_uuid_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Driver properties queried using ::zeDriverGetProperties
typedef struct _ze_driver_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_driver_uuid_t uuid;                          ///< [out] universal unique identifier.
    uint32_t driverVersion;                         ///< [out] driver version
                                                    ///< The driver version is a non-zero, monotonically increasing value where
                                                    ///< higher values always indicate a more recent version.

} ze_driver_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves properties of the driver.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **clGetPlatformInfo**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDriver`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pDriverProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDriverGetProperties(
    ze_driver_handle_t hDriver,                     ///< [in] handle of the driver instance
    ze_driver_properties_t* pDriverProperties       ///< [in,out] query result for driver properties
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported IPC property flags
typedef uint32_t ze_ipc_property_flags_t;
typedef enum _ze_ipc_property_flag_t
{
    ZE_IPC_PROPERTY_FLAG_MEMORY = ZE_BIT(0),        ///< Supports passing memory allocations between processes. See
                                                    ///< ::zeMemGetIpcHandle.
    ZE_IPC_PROPERTY_FLAG_EVENT_POOL = ZE_BIT(1),    ///< Supports passing event pools between processes. See
                                                    ///< ::zeEventPoolGetIpcHandle.
    ZE_IPC_PROPERTY_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_ipc_property_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief IPC properties queried using ::zeDriverGetIpcProperties
typedef struct _ze_driver_ipc_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_ipc_property_flags_t flags;                  ///< [out] 0 (none) or a valid combination of ::ze_ipc_property_flag_t

} ze_driver_ipc_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves IPC attributes of the driver
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDriver`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pIpcProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDriverGetIpcProperties(
    ze_driver_handle_t hDriver,                     ///< [in] handle of the driver instance
    ze_driver_ipc_properties_t* pIpcProperties      ///< [in,out] query result for IPC properties
    );

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAX_EXTENSION_NAME
/// @brief Maximum extension name string size
#define ZE_MAX_EXTENSION_NAME  256
#endif // ZE_MAX_EXTENSION_NAME

///////////////////////////////////////////////////////////////////////////////
/// @brief Extension properties queried using ::zeDriverGetExtensionProperties
typedef struct _ze_driver_extension_properties_t
{
    char name[ZE_MAX_EXTENSION_NAME];               ///< [out] extension name
    uint32_t version;                               ///< [out] extension version using ::ZE_MAKE_VERSION

} ze_driver_extension_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves extension properties
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **vkEnumerateInstanceExtensionProperties**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDriver`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pCount`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDriverGetExtensionProperties(
    ze_driver_handle_t hDriver,                     ///< [in] handle of the driver instance
    uint32_t* pCount,                               ///< [in,out] pointer to the number of extension properties.
                                                    ///< if count is zero, then the driver shall update the value with the
                                                    ///< total number of extension properties available.
                                                    ///< if count is greater than the number of extension properties available,
                                                    ///< then the driver shall update the value with the correct number of
                                                    ///< extension properties available.
    ze_driver_extension_properties_t* pExtensionProperties  ///< [in,out][optional][range(0, *pCount)] array of query results for
                                                    ///< extension properties.
                                                    ///< if count is less than the number of extension properties available,
                                                    ///< then driver shall only retrieve that number of extension properties.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves function pointer for vendor-specific or experimental
///        extensions
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDriver`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == name`
///         + `nullptr == ppFunctionAddress`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDriverGetExtensionFunctionAddress(
    ze_driver_handle_t hDriver,                     ///< [in] handle of the driver instance
    const char* name,                               ///< [in] extension name
    void** ppFunctionAddress                        ///< [out] pointer to function pointer
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Device
#if !defined(__GNUC__)
#pragma region device
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves devices within a driver
/// 
/// @details
///     - Multiple calls to this function will return identical device handles,
///       in the same order.
///     - The number and order of handles returned from this function is
///       affected by the ::ZE_AFFINITY_MASK and ::ZE_ENABLE_PCI_ID_DEVICE_ORDER
///       environment variables.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDriver`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pCount`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGet(
    ze_driver_handle_t hDriver,                     ///< [in] handle of the driver instance
    uint32_t* pCount,                               ///< [in,out] pointer to the number of devices.
                                                    ///< if count is zero, then the driver shall update the value with the
                                                    ///< total number of devices available.
                                                    ///< if count is greater than the number of devices available, then the
                                                    ///< driver shall update the value with the correct number of devices available.
    ze_device_handle_t* phDevices                   ///< [in,out][optional][range(0, *pCount)] array of handle of devices.
                                                    ///< if count is less than the number of devices available, then driver
                                                    ///< shall only retrieve that number of devices.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves a sub-device from a device
/// 
/// @details
///     - Multiple calls to this function will return identical device handles,
///       in the same order.
///     - The number of handles returned from this function is affected by the
///       ::ZE_AFFINITY_MASK environment variable.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clCreateSubDevices
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pCount`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetSubDevices(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device object
    uint32_t* pCount,                               ///< [in,out] pointer to the number of sub-devices.
                                                    ///< if count is zero, then the driver shall update the value with the
                                                    ///< total number of sub-devices available.
                                                    ///< if count is greater than the number of sub-devices available, then the
                                                    ///< driver shall update the value with the correct number of sub-devices available.
    ze_device_handle_t* phSubdevices                ///< [in,out][optional][range(0, *pCount)] array of handle of sub-devices.
                                                    ///< if count is less than the number of sub-devices available, then driver
                                                    ///< shall only retrieve that number of sub-devices.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported device types
typedef enum _ze_device_type_t
{
    ZE_DEVICE_TYPE_GPU = 1,                         ///< Graphics Processing Unit
    ZE_DEVICE_TYPE_CPU = 2,                         ///< Central Processing Unit
    ZE_DEVICE_TYPE_FPGA = 3,                        ///< Field Programmable Gate Array
    ZE_DEVICE_TYPE_MCA = 4,                         ///< Memory Copy Accelerator
    ZE_DEVICE_TYPE_FORCE_UINT32 = 0x7fffffff

} ze_device_type_t;

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAX_DEVICE_UUID_SIZE
/// @brief Maximum device universal unique id (UUID) size in bytes
#define ZE_MAX_DEVICE_UUID_SIZE  16
#endif // ZE_MAX_DEVICE_UUID_SIZE

///////////////////////////////////////////////////////////////////////////////
/// @brief Device universal unique id (UUID)
typedef struct _ze_device_uuid_t
{
    uint8_t id[ZE_MAX_DEVICE_UUID_SIZE];            ///< [out] opaque data representing a device UUID

} ze_device_uuid_t;

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAX_DEVICE_NAME
/// @brief Maximum device name string size
#define ZE_MAX_DEVICE_NAME  256
#endif // ZE_MAX_DEVICE_NAME

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported device property flags
typedef uint32_t ze_device_property_flags_t;
typedef enum _ze_device_property_flag_t
{
    ZE_DEVICE_PROPERTY_FLAG_INTEGRATED = ZE_BIT(0), ///< Device is integrated with the Host.
    ZE_DEVICE_PROPERTY_FLAG_SUBDEVICE = ZE_BIT(1),  ///< Device handle used for query represents a sub-device.
    ZE_DEVICE_PROPERTY_FLAG_ECC = ZE_BIT(2),        ///< Device supports error correction memory access.
    ZE_DEVICE_PROPERTY_FLAG_ONDEMANDPAGING = ZE_BIT(3), ///< Device supports on-demand page-faulting.
    ZE_DEVICE_PROPERTY_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_device_property_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Device properties queried using ::zeDeviceGetProperties
typedef struct _ze_device_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_device_type_t type;                          ///< [out] generic device type
    uint32_t vendorId;                              ///< [out] vendor id from PCI configuration
    uint32_t deviceId;                              ///< [out] device id from PCI configuration
    ze_device_property_flags_t flags;               ///< [out] 0 (none) or a valid combination of ::ze_device_property_flag_t
    uint32_t subdeviceId;                           ///< [out] sub-device id. Only valid if ::ZE_DEVICE_PROPERTY_FLAG_SUBDEVICE
                                                    ///< is set.
    uint32_t coreClockRate;                         ///< [out] Clock rate for device core.
    uint64_t maxMemAllocSize;                       ///< [out] Maximum memory allocation size.
    uint32_t maxHardwareContexts;                   ///< [out] Maximum number of logical hardware contexts.
    uint32_t maxCommandQueuePriority;               ///< [out] Maximum priority for command queues. Higher value is higher
                                                    ///< priority.
    uint32_t numThreadsPerEU;                       ///< [out] Number of threads per EU.
    uint32_t physicalEUSimdWidth;                   ///< [out] The physical EU simd width.
    uint32_t numEUsPerSubslice;                     ///< [out] Number of EUs per sub-slice.
    uint32_t numSubslicesPerSlice;                  ///< [out] Number of sub-slices per slice.
    uint32_t numSlices;                             ///< [out] Number of slices.
    uint64_t timerResolution;                       ///< [out] Returns the resolution of device timer in cycles per second used
                                                    ///< for profiling, timestamps, etc.
    uint32_t timestampValidBits;                    ///< [out] Returns the number of valid bits in the timestamp value.
    uint32_t kernelTimestampValidBits;              ///< [out] Returns the number of valid bits in the kernel timestamp values
    ze_device_uuid_t uuid;                          ///< [out] universal unique identifier. Note: Subdevices will have their
                                                    ///< own uuid.
    char name[ZE_MAX_DEVICE_NAME];                  ///< [out] Device name

} ze_device_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Device thread identifier.
typedef struct _ze_device_thread_t
{
    uint32_t slice;                                 ///< [in,out] the slice number.
                                                    ///< Must be UINT32_MAX (all) or less than ::ze_device_properties_t.numSlices.
    uint32_t subslice;                              ///< [in,out] the sub-slice number within its slice.
                                                    ///< Must be UINT32_MAX (all) or less than ::ze_device_properties_t.numSubslicesPerSlice.
    uint32_t eu;                                    ///< [in,out] the EU number within its sub-slice.
                                                    ///< Must be UINT32_MAX (all) or less than ::ze_device_properties_t.numEUsPerSubslice.
    uint32_t thread;                                ///< [in,out] the thread number within its EU.
                                                    ///< Must be UINT32_MAX (all) or less than ::ze_device_properties_t.numThreadsPerEU.

} ze_device_thread_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves properties of the device.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clGetDeviceInfo
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pDeviceProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    ze_device_properties_t* pDeviceProperties       ///< [in,out] query result for device properties
    );

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_SUBGROUPSIZE_COUNT
/// @brief Maximum number of subgroup sizes supported.
#define ZE_SUBGROUPSIZE_COUNT  8
#endif // ZE_SUBGROUPSIZE_COUNT

///////////////////////////////////////////////////////////////////////////////
/// @brief Device compute properties queried using ::zeDeviceGetComputeProperties
typedef struct _ze_device_compute_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    uint32_t maxTotalGroupSize;                     ///< [out] Maximum items per compute group. (groupSizeX * groupSizeY *
                                                    ///< groupSizeZ) <= maxTotalGroupSize
    uint32_t maxGroupSizeX;                         ///< [out] Maximum items for X dimension in group
    uint32_t maxGroupSizeY;                         ///< [out] Maximum items for Y dimension in group
    uint32_t maxGroupSizeZ;                         ///< [out] Maximum items for Z dimension in group
    uint32_t maxGroupCountX;                        ///< [out] Maximum groups that can be launched for x dimension
    uint32_t maxGroupCountY;                        ///< [out] Maximum groups that can be launched for y dimension
    uint32_t maxGroupCountZ;                        ///< [out] Maximum groups that can be launched for z dimension
    uint32_t maxSharedLocalMemory;                  ///< [out] Maximum shared local memory per group.
    uint32_t numSubGroupSizes;                      ///< [out] Number of subgroup sizes supported. This indicates number of
                                                    ///< entries in subGroupSizes.
    uint32_t subGroupSizes[ZE_SUBGROUPSIZE_COUNT];  ///< [out] Size group sizes supported.

} ze_device_compute_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves compute properties of the device.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clGetDeviceInfo
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pComputeProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetComputeProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    ze_device_compute_properties_t* pComputeProperties  ///< [in,out] query result for compute properties
    );

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAX_NATIVE_KERNEL_UUID_SIZE
/// @brief Maximum native kernel universal unique id (UUID) size in bytes
#define ZE_MAX_NATIVE_KERNEL_UUID_SIZE  16
#endif // ZE_MAX_NATIVE_KERNEL_UUID_SIZE

///////////////////////////////////////////////////////////////////////////////
/// @brief Native kernel universal unique id (UUID)
typedef struct _ze_native_kernel_uuid_t
{
    uint8_t id[ZE_MAX_NATIVE_KERNEL_UUID_SIZE];     ///< [out] opaque data representing a native kernel UUID

} ze_native_kernel_uuid_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported device module flags
typedef uint32_t ze_device_module_flags_t;
typedef enum _ze_device_module_flag_t
{
    ZE_DEVICE_MODULE_FLAG_FP16 = ZE_BIT(0),         ///< Device supports 16-bit floating-point operations
    ZE_DEVICE_MODULE_FLAG_FP64 = ZE_BIT(1),         ///< Device supports 64-bit floating-point operations
    ZE_DEVICE_MODULE_FLAG_INT64_ATOMICS = ZE_BIT(2),///< Device supports 64-bit atomic operations
    ZE_DEVICE_MODULE_FLAG_DP4A = ZE_BIT(3),         ///< Device supports four component dot product and accumulate operations
    ZE_DEVICE_MODULE_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_device_module_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported floating-Point capability flags
typedef uint32_t ze_device_fp_flags_t;
typedef enum _ze_device_fp_flag_t
{
    ZE_DEVICE_FP_FLAG_DENORM = ZE_BIT(0),           ///< Supports denorms
    ZE_DEVICE_FP_FLAG_INF_NAN = ZE_BIT(1),          ///< Supports INF and quiet NaNs
    ZE_DEVICE_FP_FLAG_ROUND_TO_NEAREST = ZE_BIT(2), ///< Supports rounding to nearest even rounding mode
    ZE_DEVICE_FP_FLAG_ROUND_TO_ZERO = ZE_BIT(3),    ///< Supports rounding to zero.
    ZE_DEVICE_FP_FLAG_ROUND_TO_INF = ZE_BIT(4),     ///< Supports rounding to both positive and negative INF.
    ZE_DEVICE_FP_FLAG_FMA = ZE_BIT(5),              ///< Supports IEEE754-2008 fused multiply-add.
    ZE_DEVICE_FP_FLAG_ROUNDED_DIVIDE_SQRT = ZE_BIT(6),  ///< Supports rounding as defined by IEEE754 for divide and sqrt
                                                    ///< operations.
    ZE_DEVICE_FP_FLAG_SOFT_FLOAT = ZE_BIT(7),       ///< Uses software implementation for basic floating-point operations.
    ZE_DEVICE_FP_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_device_fp_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Device module properties queried using ::zeDeviceGetModuleProperties
typedef struct _ze_device_module_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    uint32_t spirvVersionSupported;                 ///< [out] Maximum supported SPIR-V version.
                                                    ///< Returns zero if SPIR-V is not supported.
                                                    ///< Contains major and minor attributes, use ::ZE_MAJOR_VERSION and ::ZE_MINOR_VERSION.
    ze_device_module_flags_t flags;                 ///< [out] 0 or a valid combination of ::ze_device_module_flag_t
    ze_device_fp_flags_t fp16flags;                 ///< [out] Capabilities for half-precision floating-point operations.
                                                    ///< returns 0 (if ::ZE_DEVICE_MODULE_FLAG_FP16 is not set) or a
                                                    ///< combination of ::ze_device_fp_flag_t.
    ze_device_fp_flags_t fp32flags;                 ///< [out] Capabilities for single-precision floating-point operations.
                                                    ///< returns a combination of ::ze_device_fp_flag_t.
    ze_device_fp_flags_t fp64flags;                 ///< [out] Capabilities for double-precision floating-point operations.
                                                    ///< returns 0 (if ::ZE_DEVICE_MODULE_FLAG_FP64 is not set) or a
                                                    ///< combination of ::ze_device_fp_flag_t.
    uint32_t maxArgumentsSize;                      ///< [out] Maximum kernel argument size that is supported.
    uint32_t printfBufferSize;                      ///< [out] Maximum size of internal buffer that holds output of printf
                                                    ///< calls from kernel.
    ze_native_kernel_uuid_t nativeKernelSupported;  ///< [out] Compatibility UUID of supported native kernel.
                                                    ///< UUID may or may not be the same across driver release, devices, or
                                                    ///< operating systems.
                                                    ///< Application is responsible for ensuring UUID matches before creating
                                                    ///< module using
                                                    ///< previously created native kernel.

} ze_device_module_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves module properties of the device
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pModuleProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetModuleProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    ze_device_module_properties_t* pModuleProperties///< [in,out] query result for module properties
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported command queue group property flags
typedef uint32_t ze_command_queue_group_property_flags_t;
typedef enum _ze_command_queue_group_property_flag_t
{
    ZE_COMMAND_QUEUE_GROUP_PROPERTY_FLAG_COMPUTE = ZE_BIT(0),   ///< Command queue group supports enqueing compute commands.
    ZE_COMMAND_QUEUE_GROUP_PROPERTY_FLAG_COPY = ZE_BIT(1),  ///< Command queue group supports enqueing copy commands.
    ZE_COMMAND_QUEUE_GROUP_PROPERTY_FLAG_COOPERATIVE_KERNELS = ZE_BIT(2),   ///< Command queue group supports cooperative kernels.
                                                    ///< See ::zeCommandListAppendLaunchCooperativeKernel for more details.
    ZE_COMMAND_QUEUE_GROUP_PROPERTY_FLAG_METRICS = ZE_BIT(3),   ///< Command queue groups supports metric queries.
    ZE_COMMAND_QUEUE_GROUP_PROPERTY_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_command_queue_group_property_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Command queue group properties queried using
///        ::zeDeviceGetCommandQueueGroupProperties
typedef struct _ze_command_queue_group_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_command_queue_group_property_flags_t flags;  ///< [out] 0 (none) or a valid combination of
                                                    ///< ::ze_command_queue_group_property_flag_t
    size_t maxMemoryFillPatternSize;                ///< [out] maximum `pattern_size` supported by command queue group.
                                                    ///< See ::zeCommandListAppendMemoryFill for more details.
    uint32_t numQueues;                             ///< [out] the number of physical engines within the group.

} ze_command_queue_group_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves command queue group properties of the device.
/// 
/// @details
///     - Properties are reported for each physical command queue type supported
///       by the device.
///     - Multiple calls to this function will return properties in the same
///       order.
///     - The order in which the properties are returned defines the command
///       queue group's ordinal.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **vkGetPhysicalDeviceQueueFamilyProperties**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pCount`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetCommandQueueGroupProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    uint32_t* pCount,                               ///< [in,out] pointer to the number of command queue group properties.
                                                    ///< if count is zero, then the driver shall update the value with the
                                                    ///< total number of command queue group properties available.
                                                    ///< if count is greater than the number of command queue group properties
                                                    ///< available, then the driver shall update the value with the correct
                                                    ///< number of command queue group properties available.
    ze_command_queue_group_properties_t* pCommandQueueGroupProperties   ///< [in,out][optional][range(0, *pCount)] array of query results for
                                                    ///< command queue group properties.
                                                    ///< if count is less than the number of command queue group properties
                                                    ///< available, then driver shall only retrieve that number of command
                                                    ///< queue group properties.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported device memory property flags
typedef uint32_t ze_device_memory_property_flags_t;
typedef enum _ze_device_memory_property_flag_t
{
    ZE_DEVICE_MEMORY_PROPERTY_FLAG_TBD = ZE_BIT(0), ///< reserved for future use
    ZE_DEVICE_MEMORY_PROPERTY_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_device_memory_property_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Device local memory properties queried using
///        ::zeDeviceGetMemoryProperties
typedef struct _ze_device_memory_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_device_memory_property_flags_t flags;        ///< [out] 0 (none) or a valid combination of
                                                    ///< ::ze_device_memory_property_flag_t
    uint32_t maxClockRate;                          ///< [out] Maximum clock rate for device memory.
    uint32_t maxBusWidth;                           ///< [out] Maximum bus width between device and memory.
    uint64_t totalSize;                             ///< [out] Total memory size in bytes that is available to the device.
    char name[ZE_MAX_DEVICE_NAME];                  ///< [out] Memory name

} ze_device_memory_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves local memory properties of the device.
/// 
/// @details
///     - Properties are reported for each physical memory type supported by the
///       device.
///     - Multiple calls to this function will return properties in the same
///       order.
///     - The order in which the properties are returned defines the device's
///       local memory ordinal.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clGetDeviceInfo
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pCount`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetMemoryProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    uint32_t* pCount,                               ///< [in,out] pointer to the number of memory properties.
                                                    ///< if count is zero, then the driver shall update the value with the
                                                    ///< total number of memory properties available.
                                                    ///< if count is greater than the number of memory properties available,
                                                    ///< then the driver shall update the value with the correct number of
                                                    ///< memory properties available.
    ze_device_memory_properties_t* pMemProperties   ///< [in,out][optional][range(0, *pCount)] array of query results for
                                                    ///< memory properties.
                                                    ///< if count is less than the number of memory properties available, then
                                                    ///< driver shall only retrieve that number of memory properties.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Memory access capability flags
/// 
/// @details
///     - Supported access capabilities for different types of memory
///       allocations
typedef uint32_t ze_memory_access_cap_flags_t;
typedef enum _ze_memory_access_cap_flag_t
{
    ZE_MEMORY_ACCESS_CAP_FLAG_RW = ZE_BIT(0),       ///< Supports load/store access
    ZE_MEMORY_ACCESS_CAP_FLAG_ATOMIC = ZE_BIT(1),   ///< Supports atomic access
    ZE_MEMORY_ACCESS_CAP_FLAG_CONCURRENT = ZE_BIT(2),   ///< Supports concurrent access
    ZE_MEMORY_ACCESS_CAP_FLAG_CONCURRENT_ATOMIC = ZE_BIT(3),///< Supports concurrent atomic access
    ZE_MEMORY_ACCESS_CAP_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_memory_access_cap_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Device memory access properties queried using
///        ::zeDeviceGetMemoryAccessProperties
typedef struct _ze_device_memory_access_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_memory_access_cap_flags_t hostAllocCapabilities; ///< [out] host memory capabilities.
                                                    ///< returns 0 (unsupported) or a combination of ::ze_memory_access_cap_flag_t.
    ze_memory_access_cap_flags_t deviceAllocCapabilities;   ///< [out] device memory capabilities.
                                                    ///< returns 0 (unsupported) or a combination of ::ze_memory_access_cap_flag_t.
    ze_memory_access_cap_flags_t sharedSingleDeviceAllocCapabilities;   ///< [out] shared, single-device memory capabilities.
                                                    ///< returns 0 (unsupported) or a combination of ::ze_memory_access_cap_flag_t.
    ze_memory_access_cap_flags_t sharedCrossDeviceAllocCapabilities;///< [out] shared, cross-device memory capabilities.
                                                    ///< returns 0 (unsupported) or a combination of ::ze_memory_access_cap_flag_t.
    ze_memory_access_cap_flags_t sharedSystemAllocCapabilities; ///< [out] shared, system memory capabilities.
                                                    ///< returns 0 (unsupported) or a combination of ::ze_memory_access_cap_flag_t.

} ze_device_memory_access_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves memory access properties of the device.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clGetDeviceInfo
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pMemAccessProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetMemoryAccessProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    ze_device_memory_access_properties_t* pMemAccessProperties  ///< [in,out] query result for memory access properties
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported cache control property flags
typedef uint32_t ze_device_cache_property_flags_t;
typedef enum _ze_device_cache_property_flag_t
{
    ZE_DEVICE_CACHE_PROPERTY_FLAG_USER_CONTROL = ZE_BIT(0), ///< Device support User Cache Control (i.e. SLM section vs Generic Cache)
    ZE_DEVICE_CACHE_PROPERTY_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_device_cache_property_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Device cache properties queried using ::zeDeviceGetCacheProperties
typedef struct _ze_device_cache_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_device_cache_property_flags_t flags;         ///< [out] 0 (none) or a valid combination of
                                                    ///< ::ze_device_cache_property_flag_t
    size_t cacheSize;                               ///< [out] Per-cache size, in bytes

} ze_device_cache_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves cache properties of the device
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clGetDeviceInfo
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pCount`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetCacheProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    uint32_t* pCount,                               ///< [in,out] pointer to the number of cache properties.
                                                    ///< if count is zero, then the driver shall update the value with the
                                                    ///< total number of cache properties available.
                                                    ///< if count is greater than the number of cache properties available,
                                                    ///< then the driver shall update the value with the correct number of
                                                    ///< cache properties available.
    ze_device_cache_properties_t* pCacheProperties  ///< [in,out][optional][range(0, *pCount)] array of query results for cache properties.
                                                    ///< if count is less than the number of cache properties available, then
                                                    ///< driver shall only retrieve that number of cache properties.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Device image properties queried using ::zeDeviceGetImageProperties
typedef struct _ze_device_image_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    uint32_t maxImageDims1D;                        ///< [out] Maximum image dimensions for 1D resources. if 0, then 1D images
                                                    ///< are unsupported.
    uint32_t maxImageDims2D;                        ///< [out] Maximum image dimensions for 2D resources. if 0, then 2D images
                                                    ///< are unsupported.
    uint32_t maxImageDims3D;                        ///< [out] Maximum image dimensions for 3D resources. if 0, then 3D images
                                                    ///< are unsupported.
    uint64_t maxImageBufferSize;                    ///< [out] Maximum image buffer size in bytes. if 0, then buffer images are
                                                    ///< unsupported.
    uint32_t maxImageArraySlices;                   ///< [out] Maximum image array slices. if 0, then image arrays are
                                                    ///< unsupported.
    uint32_t maxSamplers;                           ///< [out] Max samplers that can be used in kernel. if 0, then sampling is
                                                    ///< unsupported.
    uint32_t maxReadImageArgs;                      ///< [out] Returns the maximum number of simultaneous image objects that
                                                    ///< can be read from by a kernel. if 0, then reading images is
                                                    ///< unsupported.
    uint32_t maxWriteImageArgs;                     ///< [out] Returns the maximum number of simultaneous image objects that
                                                    ///< can be written to by a kernel. if 0, then writing images is
                                                    ///< unsupported.

} ze_device_image_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves image properties of the device
/// 
/// @details
///     - See ::zeImageGetProperties for format-specific capabilities.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pImageProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetImageProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    ze_device_image_properties_t* pImageProperties  ///< [in,out] query result for image properties
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Device external memory import and export properties
typedef struct _ze_device_external_memory_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_external_memory_type_flags_t memoryAllocationImportTypes;///< [out] Supported external memory import types for memory allocations.
    ze_external_memory_type_flags_t memoryAllocationExportTypes;///< [out] Supported external memory export types for memory allocations.
    ze_external_memory_type_flags_t imageImportTypes;   ///< [out] Supported external memory import types for images.
    ze_external_memory_type_flags_t imageExportTypes;   ///< [out] Supported external memory export types for images.

} ze_device_external_memory_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves external memory import and export of the device
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pExternalMemoryProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetExternalMemoryProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    ze_device_external_memory_properties_t* pExternalMemoryProperties   ///< [in,out] query result for external memory properties
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported device peer-to-peer property flags
typedef uint32_t ze_device_p2p_property_flags_t;
typedef enum _ze_device_p2p_property_flag_t
{
    ZE_DEVICE_P2P_PROPERTY_FLAG_ACCESS = ZE_BIT(0), ///< Device supports access between peer devices.
    ZE_DEVICE_P2P_PROPERTY_FLAG_ATOMICS = ZE_BIT(1),///< Device supports atomics between peer devices.
    ZE_DEVICE_P2P_PROPERTY_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_device_p2p_property_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Device peer-to-peer properties queried using
///        ::zeDeviceGetP2PProperties
typedef struct _ze_device_p2p_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_device_p2p_property_flags_t flags;           ///< [out] 0 (none) or a valid combination of
                                                    ///< ::ze_device_p2p_property_flag_t

} ze_device_p2p_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves peer-to-peer properties between one device and a peer
///        devices
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///         + `nullptr == hPeerDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pP2PProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetP2PProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device performing the access
    ze_device_handle_t hPeerDevice,                 ///< [in] handle of the peer device with the allocation
    ze_device_p2p_properties_t* pP2PProperties      ///< [in,out] Peer-to-Peer properties between source and peer device
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Queries if one device can directly access peer device allocations
/// 
/// @details
///     - Any device can access any other device within a node through a
///       scale-up fabric.
///     - The following are conditions for CanAccessPeer query.
///         + If both device and peer device are the same then return true.
///         + If both sub-device and peer sub-device are the same then return
///           true.
///         + If both are sub-devices and share the same parent device then
///           return true.
///         + If both device and remote device are connected by a direct or
///           indirect scale-up fabric or over PCIe (same root complex or shared
///           PCIe switch) then true.
///         + If both sub-device and remote parent device (and vice-versa) are
///           connected by a direct or indirect scale-up fabric or over PCIe
///           (same root complex or shared PCIe switch) then true.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///         + `nullptr == hPeerDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == value`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceCanAccessPeer(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device performing the access
    ze_device_handle_t hPeerDevice,                 ///< [in] handle of the peer device with the allocation
    ze_bool_t* value                                ///< [out] returned access capability
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Returns current status of the device.
/// 
/// @details
///     - Once a device is reset, this call will update the OS handle attached
///       to the device handle.
///     - The application may call this function from simultaneous threads with
///       the same device handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_SUCCESS
///         + Device is available for use.
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///         + Device is lost; must be reset for use.
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetStatus(
    ze_device_handle_t hDevice                      ///< [in] handle of the device
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Returns synchronized Host and device global timestamps.
/// 
/// @details
///     - The application may call this function from simultaneous threads with
///       the same device handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == hostTimestamp`
///         + `nullptr == deviceTimestamp`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeDeviceGetGlobalTimestamps(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    uint64_t* hostTimestamp,                        ///< [out] value of the Host's global timestamp that correlates with the
                                                    ///< Device's global timestamp value
    uint64_t* deviceTimestamp                       ///< [out] value of the Device's global timestamp that correlates with the
                                                    ///< Host's global timestamp value
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Context
#if !defined(__GNUC__)
#pragma region context
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Supported context creation flags
typedef uint32_t ze_context_flags_t;
typedef enum _ze_context_flag_t
{
    ZE_CONTEXT_FLAG_TBD = ZE_BIT(0),                ///< reserved for future use
    ZE_CONTEXT_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_context_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Context descriptor
typedef struct _ze_context_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_context_flags_t flags;                       ///< [in] creation flags.
                                                    ///< must be 0 (default) or a valid combination of ::ze_context_flag_t;
                                                    ///< default behavior may use implicit driver-based heuristics.

} ze_context_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates a context for the driver.
/// 
/// @details
///     - The application must only use the context for the driver which was
///       provided during creation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDriver`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phContext`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x1 < desc->flags`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeContextCreate(
    ze_driver_handle_t hDriver,                     ///< [in] handle of the driver object
    const ze_context_desc_t* desc,                  ///< [in] pointer to context descriptor
    ze_context_handle_t* phContext                  ///< [out] pointer to handle of context object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates a context for the driver.
/// 
/// @details
///     - The application must only use the context for the driver which was
///       provided during creation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDriver`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phContext`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x1 < desc->flags`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phDevices) && (0 < numDevices)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeContextCreateEx(
    ze_driver_handle_t hDriver,                     ///< [in] handle of the driver object
    const ze_context_desc_t* desc,                  ///< [in] pointer to context descriptor
    uint32_t numDevices,                            ///< [in][optional] number of device handles; must be 0 if `nullptr ==
                                                    ///< phDevices`
    ze_device_handle_t* phDevices,                  ///< [in][optional][range(0, numDevices)] array of device handles which
                                                    ///< context has visibility.
                                                    ///< if nullptr, then all devices supported by the driver instance are
                                                    ///< visible to the context.
                                                    ///< otherwise, context only has visibility to devices in this array.
    ze_context_handle_t* phContext                  ///< [out] pointer to handle of context object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Destroys a context.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the context before it is deleted.
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this context.
///     - The application must **not** call this function from simultaneous
///       threads with the same context handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeContextDestroy(
    ze_context_handle_t hContext                    ///< [in][release] handle of context object to destroy
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Returns current status of the context.
/// 
/// @details
///     - The application may call this function from simultaneous threads with
///       the same context handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_SUCCESS
///         + Context is available for use.
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///         + Context is invalid; due to device lost or reset.
ZE_APIEXPORT ze_result_t ZE_APICALL
zeContextGetStatus(
    ze_context_handle_t hContext                    ///< [in] handle of context object
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Command Queue
#if !defined(__GNUC__)
#pragma region cmdqueue
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Supported command queue flags
typedef uint32_t ze_command_queue_flags_t;
typedef enum _ze_command_queue_flag_t
{
    ZE_COMMAND_QUEUE_FLAG_EXPLICIT_ONLY = ZE_BIT(0),///< command queue should be optimized for submission to a single device engine.
                                                    ///< driver **must** disable any implicit optimizations for distributing
                                                    ///< work across multiple engines.
                                                    ///< this flag should be used when applications want full control over
                                                    ///< multi-engine submission and scheduling.
    ZE_COMMAND_QUEUE_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_command_queue_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported command queue modes
typedef enum _ze_command_queue_mode_t
{
    ZE_COMMAND_QUEUE_MODE_DEFAULT = 0,              ///< implicit default behavior; uses driver-based heuristics
    ZE_COMMAND_QUEUE_MODE_SYNCHRONOUS = 1,          ///< Device execution always completes immediately on execute;
                                                    ///< Host thread is blocked using wait on implicit synchronization object
    ZE_COMMAND_QUEUE_MODE_ASYNCHRONOUS = 2,         ///< Device execution is scheduled and will complete in future;
                                                    ///< explicit synchronization object must be used to determine completeness
    ZE_COMMAND_QUEUE_MODE_FORCE_UINT32 = 0x7fffffff

} ze_command_queue_mode_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported command queue priorities
typedef enum _ze_command_queue_priority_t
{
    ZE_COMMAND_QUEUE_PRIORITY_NORMAL = 0,           ///< [default] normal priority
    ZE_COMMAND_QUEUE_PRIORITY_PRIORITY_LOW = 1,     ///< lower priority than normal
    ZE_COMMAND_QUEUE_PRIORITY_PRIORITY_HIGH = 2,    ///< higher priority than normal
    ZE_COMMAND_QUEUE_PRIORITY_FORCE_UINT32 = 0x7fffffff

} ze_command_queue_priority_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Command Queue descriptor
typedef struct _ze_command_queue_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    uint32_t ordinal;                               ///< [in] command queue group ordinal
    uint32_t index;                                 ///< [in] command queue index within the group;
                                                    ///< must be zero if ::ZE_COMMAND_QUEUE_FLAG_EXPLICIT_ONLY is not set
    ze_command_queue_flags_t flags;                 ///< [in] usage flags.
                                                    ///< must be 0 (default) or a valid combination of ::ze_command_queue_flag_t;
                                                    ///< default behavior may use implicit driver-based heuristics to balance
                                                    ///< latency and throughput.
    ze_command_queue_mode_t mode;                   ///< [in] operation mode
    ze_command_queue_priority_t priority;           ///< [in] priority

} ze_command_queue_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates a command queue on the context.
/// 
/// @details
///     - A command queue represents a logical input stream to the device, tied
///       to a physical input stream.
///     - The application must only use the command queue for the device, or its
///       sub-devices, which was provided during creation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @remarks
///   _Analogues_
///     - **clCreateCommandQueue**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phCommandQueue`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x1 < desc->flags`
///         + `::ZE_COMMAND_QUEUE_MODE_ASYNCHRONOUS < desc->mode`
///         + `::ZE_COMMAND_QUEUE_PRIORITY_PRIORITY_HIGH < desc->priority`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandQueueCreate(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device object
    const ze_command_queue_desc_t* desc,            ///< [in] pointer to command queue descriptor
    ze_command_queue_handle_t* phCommandQueue       ///< [out] pointer to handle of command queue object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Destroys a command queue.
/// 
/// @details
///     - The application must destroy all fence handles created from the
///       command queue before destroying the command queue itself
///     - The application must ensure the device is not currently referencing
///       the command queue before it is deleted
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this command queue
///     - The application must **not** call this function from simultaneous
///       threads with the same command queue handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @remarks
///   _Analogues_
///     - **clReleaseCommandQueue**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandQueue`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandQueueDestroy(
    ze_command_queue_handle_t hCommandQueue         ///< [in][release] handle of command queue object to destroy
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Executes a command list in a command queue.
/// 
/// @details
///     - The command lists are submitted to the device in the order they are
///       received, whether from multiple calls (on the same or different
///       threads) or a single call with multiple command lists.
///     - The application must ensure the command lists are accessible by the
///       device on which the command queue was created.
///     - The application must ensure the command lists are not currently
///       referencing the command list since the implementation is allowed to
///       modify the contents of the command list for submission.
///     - The application must only execute command lists created with an
///       identical command queue group ordinal to the command queue.
///     - The application must use a fence created using the same command queue.
///     - The application must ensure the command queue, command list and fence
///       were created on the same context.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - vkQueueSubmit
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandQueue`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == phCommandLists`
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `0 == numCommandLists`
///     - ::ZE_RESULT_ERROR_INVALID_COMMAND_LIST_TYPE
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandQueueExecuteCommandLists(
    ze_command_queue_handle_t hCommandQueue,        ///< [in] handle of the command queue
    uint32_t numCommandLists,                       ///< [in] number of command lists to execute
    ze_command_list_handle_t* phCommandLists,       ///< [in][range(0, numCommandLists)] list of handles of the command lists
                                                    ///< to execute
    ze_fence_handle_t hFence                        ///< [in][optional] handle of the fence to signal on completion
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Synchronizes a command queue by waiting on the host.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandQueue`
///     - ::ZE_RESULT_NOT_READY
///         + timeout expired
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandQueueSynchronize(
    ze_command_queue_handle_t hCommandQueue,        ///< [in] handle of the command queue
    uint64_t timeout                                ///< [in] if non-zero, then indicates the maximum time (in nanoseconds) to
                                                    ///< yield before returning ::ZE_RESULT_SUCCESS or ::ZE_RESULT_NOT_READY;
                                                    ///< if zero, then immediately returns the status of the command queue;
                                                    ///< if UINT64_MAX, then function will not return until complete or device
                                                    ///< is lost.
                                                    ///< Due to external dependencies, timeout may be rounded to the closest
                                                    ///< value allowed by the accuracy of those dependencies.
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Command List
#if !defined(__GNUC__)
#pragma region cmdlist
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Supported command list creation flags
typedef uint32_t ze_command_list_flags_t;
typedef enum _ze_command_list_flag_t
{
    ZE_COMMAND_LIST_FLAG_RELAXED_ORDERING = ZE_BIT(0),  ///< driver may reorder commands (e.g., kernels, copies) between barriers
                                                    ///< and synchronization primitives.
                                                    ///< using this flag may increase Host overhead of ::zeCommandListClose.
                                                    ///< therefore, this flag should **not** be set for low-latency usage-models.
    ZE_COMMAND_LIST_FLAG_MAXIMIZE_THROUGHPUT = ZE_BIT(1),   ///< driver may perform additional optimizations that increase execution
                                                    ///< throughput. 
                                                    ///< using this flag may increase Host overhead of ::zeCommandListClose and ::zeCommandQueueExecuteCommandLists.
                                                    ///< therefore, this flag should **not** be set for low-latency usage-models.
    ZE_COMMAND_LIST_FLAG_EXPLICIT_ONLY = ZE_BIT(2), ///< command list should be optimized for submission to a single command
                                                    ///< queue and device engine.
                                                    ///< driver **must** disable any implicit optimizations for distributing
                                                    ///< work across multiple engines.
                                                    ///< this flag should be used when applications want full control over
                                                    ///< multi-engine submission and scheduling.
    ZE_COMMAND_LIST_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_command_list_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Command List descriptor
typedef struct _ze_command_list_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    uint32_t commandQueueGroupOrdinal;              ///< [in] command queue group ordinal to which this command list will be
                                                    ///< submitted
    ze_command_list_flags_t flags;                  ///< [in] usage flags.
                                                    ///< must be 0 (default) or a valid combination of ::ze_command_list_flag_t;
                                                    ///< default behavior may use implicit driver-based heuristics to balance
                                                    ///< latency and throughput.

} ze_command_list_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates a command list on the context.
/// 
/// @details
///     - A command list represents a sequence of commands for execution on a
///       command queue.
///     - The command list is created in the 'open' state.
///     - The application must only use the command list for the device, or its
///       sub-devices, which was provided during creation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x7 < desc->flags`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListCreate(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device object
    const ze_command_list_desc_t* desc,             ///< [in] pointer to command list descriptor
    ze_command_list_handle_t* phCommandList         ///< [out] pointer to handle of command list object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates an immediate command list on the context.
/// 
/// @details
///     - An immediate command list is used for low-latency submission of
///       commands.
///     - An immediate command list creates an implicit command queue.
///     - The command list is created in the 'open' state and never needs to be
///       closed.
///     - The application must only use the command list for the device, or its
///       sub-devices, which was provided during creation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == altdesc`
///         + `nullptr == phCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x1 < altdesc->flags`
///         + `::ZE_COMMAND_QUEUE_MODE_ASYNCHRONOUS < altdesc->mode`
///         + `::ZE_COMMAND_QUEUE_PRIORITY_PRIORITY_HIGH < altdesc->priority`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListCreateImmediate(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device object
    const ze_command_queue_desc_t* altdesc,         ///< [in] pointer to command queue descriptor
    ze_command_list_handle_t* phCommandList         ///< [out] pointer to handle of command list object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Destroys a command list.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the command list before it is deleted.
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this command list.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListDestroy(
    ze_command_list_handle_t hCommandList           ///< [in][release] handle of command list object to destroy
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Closes a command list; ready to be executed by a command queue.
/// 
/// @details
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListClose(
    ze_command_list_handle_t hCommandList           ///< [in] handle of command list object to close
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Reset a command list to initial (empty) state; ready for appending
///        commands.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the command list before it is reset
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListReset(
    ze_command_list_handle_t hCommandList           ///< [in] handle of command list object to reset
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Appends a memory write of the device's global timestamp value into a
///        command list.
/// 
/// @details
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - The timestamp frequency can be queried from
///       ::ze_device_properties_t.timerResolution.
///     - The number of valid bits in the timestamp value can be queried from
///       ::ze_device_properties_t.timestampValidBits.
///     - The application must ensure the memory pointed to by dstptr is
///       accessible by the device on which the command list was created.
///     - The application must ensure the command list and events were created,
///       and the memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == dstptr`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendWriteGlobalTimestamp(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    uint64_t* dstptr,                               ///< [in,out] pointer to memory where timestamp value will be written; must
                                                    ///< be 8byte-aligned.
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before executing query;
                                                    ///< must be 0 if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before executing query
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Barrier
#if !defined(__GNUC__)
#pragma region barrier
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Appends an execution and global memory barrier into a command list.
/// 
/// @details
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - If numWaitEvents is zero, then all previous commands are completed
///       prior to the execution of the barrier.
///     - If numWaitEvents is non-zero, then then all phWaitEvents must be
///       signaled prior to the execution of the barrier.
///     - This command blocks all following commands from beginning until the
///       execution of the barrier completes.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **vkCmdPipelineBarrier**
///     - clEnqueueBarrierWithWaitList
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendBarrier(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before executing barrier;
                                                    ///< must be 0 if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before executing barrier
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Appends a global memory ranges barrier into a command list.
/// 
/// @details
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - If numWaitEvents is zero, then all previous commands are completed
///       prior to the execution of the barrier.
///     - If numWaitEvents is non-zero, then then all phWaitEvents must be
///       signaled prior to the execution of the barrier.
///     - This command blocks all following commands from beginning until the
///       execution of the barrier completes.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pRangeSizes`
///         + `nullptr == pRanges`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendMemoryRangesBarrier(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    uint32_t numRanges,                             ///< [in] number of memory ranges
    const size_t* pRangeSizes,                      ///< [in][range(0, numRanges)] array of sizes of memory range
    const void** pRanges,                           ///< [in][range(0, numRanges)] array of memory ranges
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before executing barrier;
                                                    ///< must be 0 if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before executing barrier
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Ensures in-bound writes to the device are globally observable.
/// 
/// @details
///     - This is a special-case system level barrier that can be used to ensure
///       global observability of writes; 
///       typically needed after a producer (e.g., NIC) performs direct writes
///       to the device's memory (e.g., Direct RDMA writes).
///       This is typically required when the memory corresponding to the writes
///       is subsequently accessed from a remote device.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeContextSystemBarrier(
    ze_context_handle_t hContext,                   ///< [in] handle of context object
    ze_device_handle_t hDevice                      ///< [in] handle of the device
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Copies
#if !defined(__GNUC__)
#pragma region copy
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Copies host, device, or shared memory.
/// 
/// @details
///     - The application must ensure the memory pointed to by dstptr and srcptr
///       is accessible by the device on which the command list was created.
///     - The implementation must not access the memory pointed to by dstptr and
///       srcptr as they are free to be modified by either the Host or device up
///       until execution.
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - The application must ensure the command list and events were created,
///       and the memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **clEnqueueCopyBuffer**
///     - **clEnqueueReadBuffer**
///     - **clEnqueueWriteBuffer**
///     - **clEnqueueSVMMemcpy**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == dstptr`
///         + `nullptr == srcptr`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendMemoryCopy(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    void* dstptr,                                   ///< [in] pointer to destination memory to copy to
    const void* srcptr,                             ///< [in] pointer to source memory to copy from
    size_t size,                                    ///< [in] size in bytes to copy
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Initializes host, device, or shared memory.
/// 
/// @details
///     - The application must ensure the memory pointed to by dstptr is
///       accessible by the device on which the command list was created.
///     - The implementation must not access the memory pointed to by dstptr as
///       it is free to be modified by either the Host or device up until
///       execution.
///     - The value to initialize memory to is described by the pattern and the
///       pattern size.
///     - The pattern size must be a power-of-two and less than
///       ::ze_command_queue_group_properties_t.maxMemoryFillPatternSize.
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - The application must enusre the command list and events were created,
///       and the memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **clEnqueueFillBuffer**
///     - **clEnqueueSVMMemFill**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///         + `nullptr == pattern`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendMemoryFill(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    void* ptr,                                      ///< [in] pointer to memory to initialize
    const void* pattern,                            ///< [in] pointer to value to initialize memory to
    size_t pattern_size,                            ///< [in] size in bytes of the value to initialize memory to
    size_t size,                                    ///< [in] size in bytes to initialize
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Copy region descriptor
typedef struct _ze_copy_region_t
{
    uint32_t originX;                               ///< [in] The origin x offset for region in bytes
    uint32_t originY;                               ///< [in] The origin y offset for region in rows
    uint32_t originZ;                               ///< [in] The origin z offset for region in slices
    uint32_t width;                                 ///< [in] The region width relative to origin in bytes
    uint32_t height;                                ///< [in] The region height relative to origin in rows
    uint32_t depth;                                 ///< [in] The region depth relative to origin in slices. Set this to 0 for
                                                    ///< 2D copy.

} ze_copy_region_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Copies a region from a 2D or 3D array of host, device, or shared
///        memory.
/// 
/// @details
///     - The application must ensure the memory pointed to by dstptr and srcptr
///       is accessible by the device on which the command list was created.
///     - The implementation must not access the memory pointed to by dstptr and
///       srcptr as they are free to be modified by either the Host or device up
///       until execution.
///     - The region width, height, and depth for both src and dst must be same.
///       The origins can be different.
///     - The src and dst regions cannot be overlapping.
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - The application must ensure the command list and events were created,
///       and the memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == dstptr`
///         + `nullptr == dstRegion`
///         + `nullptr == srcptr`
///         + `nullptr == srcRegion`
///     - ::ZE_RESULT_ERROR_OVERLAPPING_REGIONS
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendMemoryCopyRegion(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    void* dstptr,                                   ///< [in] pointer to destination memory to copy to
    const ze_copy_region_t* dstRegion,              ///< [in] pointer to destination region to copy to
    uint32_t dstPitch,                              ///< [in] destination pitch in bytes
    uint32_t dstSlicePitch,                         ///< [in] destination slice pitch in bytes. This is required for 3D region
                                                    ///< copies where ::ze_copy_region_t.depth is not 0, otherwise it's
                                                    ///< ignored.
    const void* srcptr,                             ///< [in] pointer to source memory to copy from
    const ze_copy_region_t* srcRegion,              ///< [in] pointer to source region to copy from
    uint32_t srcPitch,                              ///< [in] source pitch in bytes
    uint32_t srcSlicePitch,                         ///< [in] source slice pitch in bytes. This is required for 3D region
                                                    ///< copies where ::ze_copy_region_t.depth is not 0, otherwise it's
                                                    ///< ignored.
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Copies host, device, or shared memory from another context.
/// 
/// @details
///     - The current active and source context must be from the same driver.
///     - The application must ensure the memory pointed to by dstptr and srcptr
///       is accessible by the device on which the command list was created.
///     - The implementation must not access the memory pointed to by dstptr and
///       srcptr as they are free to be modified by either the Host or device up
///       until execution.
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - The application must ensure the command list and events were created,
///       and the memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hContextSrc`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == dstptr`
///         + `nullptr == srcptr`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendMemoryCopyFromContext(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    void* dstptr,                                   ///< [in] pointer to destination memory to copy to
    ze_context_handle_t hContextSrc,                ///< [in] handle of source context object
    const void* srcptr,                             ///< [in] pointer to source memory to copy from
    size_t size,                                    ///< [in] size in bytes to copy
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Copies an image.
/// 
/// @details
///     - The application must ensure the image and events are accessible by the
///       device on which the command list was created.
///     - The application must ensure the image format descriptors for both
///       source and destination images are the same.
///     - The application must ensure the command list, images and events were
///       created on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **clEnqueueCopyImage**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hDstImage`
///         + `nullptr == hSrcImage`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendImageCopy(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    ze_image_handle_t hDstImage,                    ///< [in] handle of destination image to copy to
    ze_image_handle_t hSrcImage,                    ///< [in] handle of source image to copy from
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Region descriptor
typedef struct _ze_image_region_t
{
    uint32_t originX;                               ///< [in] The origin x offset for region in pixels
    uint32_t originY;                               ///< [in] The origin y offset for region in pixels
    uint32_t originZ;                               ///< [in] The origin z offset for region in pixels
    uint32_t width;                                 ///< [in] The region width relative to origin in pixels
    uint32_t height;                                ///< [in] The region height relative to origin in pixels
    uint32_t depth;                                 ///< [in] The region depth relative to origin. For 1D or 2D images, set
                                                    ///< this to 1.

} ze_image_region_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Copies a region of an image to another image.
/// 
/// @details
///     - The application must ensure the image and events are accessible by the
///       device on which the command list was created.
///     - The region width and height for both src and dst must be same. The
///       origins can be different.
///     - The src and dst regions cannot be overlapping.
///     - The application must ensure the image format descriptors for both
///       source and destination images are the same.
///     - The application must ensure the command list, images and events were
///       created, and the memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hDstImage`
///         + `nullptr == hSrcImage`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_OVERLAPPING_REGIONS
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendImageCopyRegion(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    ze_image_handle_t hDstImage,                    ///< [in] handle of destination image to copy to
    ze_image_handle_t hSrcImage,                    ///< [in] handle of source image to copy from
    const ze_image_region_t* pDstRegion,            ///< [in][optional] destination region descriptor
    const ze_image_region_t* pSrcRegion,            ///< [in][optional] source region descriptor
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Copies from an image to device or shared memory.
/// 
/// @details
///     - The application must ensure the memory pointed to by dstptr is
///       accessible by the device on which the command list was created.
///     - The implementation must not access the memory pointed to by dstptr as
///       it is free to be modified by either the Host or device up until
///       execution.
///     - The application must ensure the image and events are accessible by the
///       device on which the command list was created.
///     - The application must ensure the image format descriptor for the source
///       image is not a media format.
///     - The application must ensure the command list, image and events were
///       created, and the memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clEnqueueReadImage
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hSrcImage`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == dstptr`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendImageCopyToMemory(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    void* dstptr,                                   ///< [in] pointer to destination memory to copy to
    ze_image_handle_t hSrcImage,                    ///< [in] handle of source image to copy from
    const ze_image_region_t* pSrcRegion,            ///< [in][optional] source region descriptor
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Copies to an image from device or shared memory.
/// 
/// @details
///     - The application must ensure the memory pointed to by srcptr is
///       accessible by the device on which the command list was created.
///     - The implementation must not access the memory pointed to by srcptr as
///       it is free to be modified by either the Host or device up until
///       execution.
///     - The application must ensure the image and events are accessible by the
///       device on which the command list was created.
///     - The application must ensure the image format descriptor for the
///       destination image is not a media format.
///     - The application must ensure the command list, image and events were
///       created, and the memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clEnqueueWriteImage
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hDstImage`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == srcptr`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendImageCopyFromMemory(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    ze_image_handle_t hDstImage,                    ///< [in] handle of destination image to copy to
    const void* srcptr,                             ///< [in] pointer to source memory to copy from
    const ze_image_region_t* pDstRegion,            ///< [in][optional] destination region descriptor
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Asynchronously prefetches shared memory to the device associated with
///        the specified command list
/// 
/// @details
///     - This is a hint to improve performance only and is not required for
///       correctness.
///     - Only prefetching to the device associated with the specified command
///       list is supported.
///       Prefetching to the host or to a peer device is not supported.
///     - Prefetching may not be supported for all allocation types for all devices.
///       If memory prefetching is not supported for the specified memory range
///       the prefetch hint may be ignored.
///     - Prefetching may only be supported at a device-specific granularity,
///       such as at a page boundary.
///       In this case, the memory range may be expanded such that the start and
///       end of the range satisfy granularity requirements.
///     - The application must ensure the memory pointed to by ptr is accessible
///       by the device on which the command list was created.
///     - The application must ensure the command list was created, and the
///       memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clEnqueueSVMMigrateMem
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendMemoryPrefetch(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    const void* ptr,                                ///< [in] pointer to start of the memory range to prefetch
    size_t size                                     ///< [in] size in bytes of the memory range to prefetch
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported memory advice hints
typedef enum _ze_memory_advice_t
{
    ZE_MEMORY_ADVICE_SET_READ_MOSTLY = 0,           ///< hint that memory will be read from frequently and written to rarely
    ZE_MEMORY_ADVICE_CLEAR_READ_MOSTLY = 1,         ///< removes the affect of ::ZE_MEMORY_ADVICE_SET_READ_MOSTLY
    ZE_MEMORY_ADVICE_SET_PREFERRED_LOCATION = 2,    ///< hint that the preferred memory location is the specified device
    ZE_MEMORY_ADVICE_CLEAR_PREFERRED_LOCATION = 3,  ///< removes the affect of ::ZE_MEMORY_ADVICE_SET_PREFERRED_LOCATION
    ZE_MEMORY_ADVICE_SET_NON_ATOMIC_MOSTLY = 4,     ///< hints that memory will mostly be accessed non-atomically
    ZE_MEMORY_ADVICE_CLEAR_NON_ATOMIC_MOSTLY = 5,   ///< removes the affect of ::ZE_MEMORY_ADVICE_SET_NON_ATOMIC_MOSTLY
    ZE_MEMORY_ADVICE_BIAS_CACHED = 6,               ///< hints that memory should be cached
    ZE_MEMORY_ADVICE_BIAS_UNCACHED = 7,             ///< hints that memory should be not be cached
    ZE_MEMORY_ADVICE_FORCE_UINT32 = 0x7fffffff

} ze_memory_advice_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Provides advice about the use of a shared memory range
/// 
/// @details
///     - Memory advice is a performance hint only and is not required for
///       functional correctness.
///     - Memory advice can be used to override driver heuristics to explicitly
///       control shared memory behavior.
///     - Not all memory advice hints may be supported for all allocation types
///       for all devices.
///       If a memory advice hint is not supported by the device it will be ignored.
///     - Memory advice may only be supported at a device-specific granularity,
///       such as at a page boundary.
///       In this case, the memory range may be expanded such that the start and
///       end of the range satisfy granularity requirements.
///     - The application must ensure the memory pointed to by ptr is accessible
///       by the device on which the command list was created.
///     - The application must ensure the command list was created, and memory
///       was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle, and the memory was
///       allocated.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `::ZE_MEMORY_ADVICE_BIAS_UNCACHED < advice`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendMemAdvise(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of command list
    ze_device_handle_t hDevice,                     ///< [in] device associated with the memory advice
    const void* ptr,                                ///< [in] Pointer to the start of the memory range
    size_t size,                                    ///< [in] Size in bytes of the memory range
    ze_memory_advice_t advice                       ///< [in] Memory advice for the memory range
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Event
#if !defined(__GNUC__)
#pragma region event
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Supported event pool creation flags
typedef uint32_t ze_event_pool_flags_t;
typedef enum _ze_event_pool_flag_t
{
    ZE_EVENT_POOL_FLAG_HOST_VISIBLE = ZE_BIT(0),    ///< signals and waits are also visible to host
    ZE_EVENT_POOL_FLAG_IPC = ZE_BIT(1),             ///< signals and waits may be shared across processes
    ZE_EVENT_POOL_FLAG_KERNEL_TIMESTAMP = ZE_BIT(2),///< Indicates all events in pool will contain kernel timestamps; cannot be
                                                    ///< combined with ::ZE_EVENT_POOL_FLAG_IPC
    ZE_EVENT_POOL_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_event_pool_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Event pool descriptor
typedef struct _ze_event_pool_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_event_pool_flags_t flags;                    ///< [in] creation flags.
                                                    ///< must be 0 (default) or a valid combination of ::ze_event_pool_flag_t;
                                                    ///< default behavior is signals and waits are visible to the entire device
                                                    ///< and peer devices.
    uint32_t count;                                 ///< [in] number of events within the pool; must be greater than 0

} ze_event_pool_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates a pool of events on the context.
/// 
/// @details
///     - The application must only use events within the pool for the
///       device(s), or their sub-devices, which were provided during creation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phEventPool`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x7 < desc->flags`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `0 == desc->count`
///         + `(nullptr == phDevices) && (0 < numDevices)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventPoolCreate(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const ze_event_pool_desc_t* desc,               ///< [in] pointer to event pool descriptor
    uint32_t numDevices,                            ///< [in][optional] number of device handles; must be 0 if `nullptr ==
                                                    ///< phDevices`
    ze_device_handle_t* phDevices,                  ///< [in][optional][range(0, numDevices)] array of device handles which
                                                    ///< have visibility to the event pool.
                                                    ///< if nullptr, then event pool is visible to all devices supported by the
                                                    ///< driver instance.
    ze_event_pool_handle_t* phEventPool             ///< [out] pointer handle of event pool object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Deletes an event pool object.
/// 
/// @details
///     - The application must destroy all event handles created from the pool
///       before destroying the pool itself.
///     - The application must ensure the device is not currently referencing
///       the any event within the pool before it is deleted.
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this event pool.
///     - The application must **not** call this function from simultaneous
///       threads with the same event pool handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEventPool`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventPoolDestroy(
    ze_event_pool_handle_t hEventPool               ///< [in][release] handle of event pool object to destroy
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported event scope flags
typedef uint32_t ze_event_scope_flags_t;
typedef enum _ze_event_scope_flag_t
{
    ZE_EVENT_SCOPE_FLAG_SUBDEVICE = ZE_BIT(0),      ///< cache hierarchies are flushed or invalidated sufficient for local
                                                    ///< sub-device access
    ZE_EVENT_SCOPE_FLAG_DEVICE = ZE_BIT(1),         ///< cache hierarchies are flushed or invalidated sufficient for global
                                                    ///< device access and peer device access
    ZE_EVENT_SCOPE_FLAG_HOST = ZE_BIT(2),           ///< cache hierarchies are flushed or invalidated sufficient for device and
                                                    ///< host access
    ZE_EVENT_SCOPE_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_event_scope_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Event descriptor
typedef struct _ze_event_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    uint32_t index;                                 ///< [in] index of the event within the pool; must be less-than the count
                                                    ///< specified during pool creation
    ze_event_scope_flags_t signal;                  ///< [in] defines the scope of relevant cache hierarchies to flush on a
                                                    ///< signal action before the event is triggered.
                                                    ///< must be 0 (default) or a valid combination of ::ze_event_scope_flag_t;
                                                    ///< default behavior is synchronization within the command list only, no
                                                    ///< additional cache hierarchies are flushed.
    ze_event_scope_flags_t wait;                    ///< [in] defines the scope of relevant cache hierarchies to invalidate on
                                                    ///< a wait action after the event is complete.
                                                    ///< must be 0 (default) or a valid combination of ::ze_event_scope_flag_t;
                                                    ///< default behavior is synchronization within the command list only, no
                                                    ///< additional cache hierarchies are invalidated.

} ze_event_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates an event from the pool.
/// 
/// @details
///     - An event is used to communicate fine-grain host-to-device,
///       device-to-host or device-to-device dependencies have completed.
///     - The application must ensure the location in the pool is not being used
///       by another event.
///     - The application must **not** call this function from simultaneous
///       threads with the same event pool handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **clCreateUserEvent**
///     - vkCreateEvent
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEventPool`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phEvent`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x7 < desc->signal`
///         + `0x7 < desc->wait`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventCreate(
    ze_event_pool_handle_t hEventPool,              ///< [in] handle of the event pool
    const ze_event_desc_t* desc,                    ///< [in] pointer to event descriptor
    ze_event_handle_t* phEvent                      ///< [out] pointer to handle of event object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Deletes an event object.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the event before it is deleted.
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this event.
///     - The application must **not** call this function from simultaneous
///       threads with the same event handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **clReleaseEvent**
///     - vkDestroyEvent
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEvent`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventDestroy(
    ze_event_handle_t hEvent                        ///< [in][release] handle of event object to destroy
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Gets an IPC event pool handle for the specified event handle that can
///        be shared with another process.
/// 
/// @details
///     - Event pool must have been created with ::ZE_EVENT_POOL_FLAG_IPC.
///     - The application may call this function from simultaneous threads.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEventPool`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == phIpc`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventPoolGetIpcHandle(
    ze_event_pool_handle_t hEventPool,              ///< [in] handle of event pool object
    ze_ipc_event_pool_handle_t* phIpc               ///< [out] Returned IPC event handle
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Opens an IPC event pool handle to retrieve an event pool handle from
///        another process.
/// 
/// @details
///     - Multiple calls to this function with the same IPC handle will return
///       unique event pool handles.
///     - The event handle in this process should not be freed with
///       ::zeEventPoolDestroy, but rather with ::zeEventPoolCloseIpcHandle.
///     - The application may call this function from simultaneous threads.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == phEventPool`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventPoolOpenIpcHandle(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object to associate with the IPC event pool
                                                    ///< handle
    ze_ipc_event_pool_handle_t hIpc,                ///< [in] IPC event pool handle
    ze_event_pool_handle_t* phEventPool             ///< [out] pointer handle of event pool object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Closes an IPC event handle in the current process.
/// 
/// @details
///     - Closes an IPC event handle by destroying events that were opened in
///       this process using ::zeEventPoolOpenIpcHandle.
///     - The application must **not** call this function from simultaneous
///       threads with the same event pool handle.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEventPool`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventPoolCloseIpcHandle(
    ze_event_pool_handle_t hEventPool               ///< [in][release] handle of event pool object
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Appends a signal of the event from the device into a command list.
/// 
/// @details
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - The duration of an event created from an event pool that was created
///       using ::ZE_EVENT_POOL_FLAG_KERNEL_TIMESTAMP flag is undefined.
///       However, for consistency and orthogonality the event will report
///       correctly as signaled when used by other event API functionality.
///     - The application must ensure the command list and events were created
///       on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **clSetUserEventStatus**
///     - vkCmdSetEvent
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hEvent`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendSignalEvent(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    ze_event_handle_t hEvent                        ///< [in] handle of the event
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Appends wait on event(s) on the device into a command list.
/// 
/// @details
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - The application must ensure the command list and events were created
///       on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == phEvents`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendWaitOnEvents(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    uint32_t numEvents,                             ///< [in] number of events to wait on before continuing
    ze_event_handle_t* phEvents                     ///< [in][range(0, numEvents)] handles of the events to wait on before
                                                    ///< continuing
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Signals a event from host.
/// 
/// @details
///     - The duration of an event created from an event pool that was created
///       using ::ZE_EVENT_POOL_FLAG_KERNEL_TIMESTAMP flag is undefined.
///       However, for consistency and orthogonality the event will report
///       correctly as signaled when used by other event API functionality.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clSetUserEventStatus
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEvent`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventHostSignal(
    ze_event_handle_t hEvent                        ///< [in] handle of the event
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief The current host thread waits on an event to be signaled.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - clWaitForEvents
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEvent`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_NOT_READY
///         + timeout expired
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventHostSynchronize(
    ze_event_handle_t hEvent,                       ///< [in] handle of the event
    uint64_t timeout                                ///< [in] if non-zero, then indicates the maximum time (in nanoseconds) to
                                                    ///< yield before returning ::ZE_RESULT_SUCCESS or ::ZE_RESULT_NOT_READY;
                                                    ///< if zero, then operates exactly like ::zeEventQueryStatus;
                                                    ///< if UINT64_MAX, then function will not return until complete or device
                                                    ///< is lost.
                                                    ///< Due to external dependencies, timeout may be rounded to the closest
                                                    ///< value allowed by the accuracy of those dependencies.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Queries an event object's status on the host.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **clGetEventInfo**
///     - vkGetEventStatus
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEvent`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_NOT_READY
///         + not signaled
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventQueryStatus(
    ze_event_handle_t hEvent                        ///< [in] handle of the event
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Appends a reset of an event back to not signaled state into a command
///        list.
/// 
/// @details
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - The application must ensure the command list and events were created
///       on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - vkResetEvent
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hEvent`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendEventReset(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    ze_event_handle_t hEvent                        ///< [in] handle of the event
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief The current host thread resets an event back to not signaled state.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - vkResetEvent
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEvent`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventHostReset(
    ze_event_handle_t hEvent                        ///< [in] handle of the event
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Kernel timestamp clock data
/// 
/// @details
///     - The timestamp frequency can be queried from
///       ::ze_device_properties_t.timerResolution.
///     - The number of valid bits in the timestamp value can be queried from
///       ::ze_device_properties_t.kernelTimestampValidBits.
typedef struct _ze_kernel_timestamp_data_t
{
    uint64_t kernelStart;                           ///< [out] device clock at start of kernel execution
    uint64_t kernelEnd;                             ///< [out] device clock at end of kernel execution

} ze_kernel_timestamp_data_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Kernel timestamp result
typedef struct _ze_kernel_timestamp_result_t
{
    ze_kernel_timestamp_data_t global;              ///< [out] wall-clock data
    ze_kernel_timestamp_data_t context;             ///< [out] context-active data; only includes clocks while device context
                                                    ///< was actively executing.

} ze_kernel_timestamp_result_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Queries an event's timestamp value on the host.
/// 
/// @details
///     - The application must ensure the event was created from an event pool
///       that was created using ::ZE_EVENT_POOL_FLAG_KERNEL_TIMESTAMP flag.
///     - The destination memory will be unmodified if the event has not been
///       signaled.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hEvent`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == dstptr`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_NOT_READY
///         + not signaled
ZE_APIEXPORT ze_result_t ZE_APICALL
zeEventQueryKernelTimestamp(
    ze_event_handle_t hEvent,                       ///< [in] handle of the event
    ze_kernel_timestamp_result_t* dstptr            ///< [in,out] pointer to memory for where timestamp result will be written.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Appends a query of an events' timestamp value(s) into a command list.
/// 
/// @details
///     - The application must ensure the events are accessible by the device on
///       which the command list was created.
///     - The application must ensure the events were created from an event pool
///       that was created using ::ZE_EVENT_POOL_FLAG_KERNEL_TIMESTAMP flag.
///     - The application must ensure the memory pointed to by both dstptr and
///       pOffsets is accessible by the device on which the command list was
///       created.
///     - The value(s) written to the destination buffer are undefined if any
///       timestamp event has not been signaled.
///     - If pOffsets is nullptr, then multiple results will be appended
///       sequentially into memory in the same order as phEvents.
///     - The application must ensure the command list and events were created,
///       and the memory was allocated, on the same context.
///     - The application must **not** call this function from simultaneous
///       threads with the same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == phEvents`
///         + `nullptr == dstptr`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendQueryKernelTimestamps(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    uint32_t numEvents,                             ///< [in] the number of timestamp events to query
    ze_event_handle_t* phEvents,                    ///< [in][range(0, numEvents)] handles of timestamp events to query
    void* dstptr,                                   ///< [in,out] pointer to memory where ::ze_kernel_timestamp_result_t will
                                                    ///< be written; must be size-aligned.
    const size_t* pOffsets,                         ///< [in][optional][range(0, numEvents)] offset, in bytes, to write
                                                    ///< results; address must be 4byte-aligned and offsets must be
                                                    ///< size-aligned.
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before executing query;
                                                    ///< must be 0 if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before executing query
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Fence
#if !defined(__GNUC__)
#pragma region fence
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Supported fence creation flags
typedef uint32_t ze_fence_flags_t;
typedef enum _ze_fence_flag_t
{
    ZE_FENCE_FLAG_SIGNALED = ZE_BIT(0),             ///< fence is created in the signaled state, otherwise not signaled.
    ZE_FENCE_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_fence_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Fence descriptor
typedef struct _ze_fence_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_fence_flags_t flags;                         ///< [in] creation flags.
                                                    ///< must be 0 (default) or a valid combination of ::ze_fence_flag_t.

} ze_fence_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates a fence for the command queue.
/// 
/// @details
///     - A fence is a heavyweight synchronization primitive used to communicate
///       to the host that command list execution has completed.
///     - The application must only use the fence for the command queue which
///       was provided during creation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @remarks
///   _Analogues_
///     - **vkCreateFence**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandQueue`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phFence`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x1 < desc->flags`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeFenceCreate(
    ze_command_queue_handle_t hCommandQueue,        ///< [in] handle of command queue
    const ze_fence_desc_t* desc,                    ///< [in] pointer to fence descriptor
    ze_fence_handle_t* phFence                      ///< [out] pointer to handle of fence object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Deletes a fence object.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the fence before it is deleted.
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this fence.
///     - The application must **not** call this function from simultaneous
///       threads with the same fence handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @remarks
///   _Analogues_
///     - **vkDestroyFence**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hFence`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeFenceDestroy(
    ze_fence_handle_t hFence                        ///< [in][release] handle of fence object to destroy
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief The current host thread waits on a fence to be signaled.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **vkWaitForFences**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hFence`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_NOT_READY
///         + timeout expired
ZE_APIEXPORT ze_result_t ZE_APICALL
zeFenceHostSynchronize(
    ze_fence_handle_t hFence,                       ///< [in] handle of the fence
    uint64_t timeout                                ///< [in] if non-zero, then indicates the maximum time (in nanoseconds) to
                                                    ///< yield before returning ::ZE_RESULT_SUCCESS or ::ZE_RESULT_NOT_READY;
                                                    ///< if zero, then operates exactly like ::zeFenceQueryStatus;
                                                    ///< if UINT64_MAX, then function will not return until complete or device
                                                    ///< is lost.
                                                    ///< Due to external dependencies, timeout may be rounded to the closest
                                                    ///< value allowed by the accuracy of those dependencies.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Queries a fence object's status.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **vkGetFenceStatus**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hFence`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_NOT_READY
///         + not signaled
ZE_APIEXPORT ze_result_t ZE_APICALL
zeFenceQueryStatus(
    ze_fence_handle_t hFence                        ///< [in] handle of the fence
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Reset a fence back to the not signaled state.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @remarks
///   _Analogues_
///     - **vkResetFences**
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hFence`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeFenceReset(
    ze_fence_handle_t hFence                        ///< [in] handle of the fence
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Images
#if !defined(__GNUC__)
#pragma region image
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Supported image creation flags
typedef uint32_t ze_image_flags_t;
typedef enum _ze_image_flag_t
{
    ZE_IMAGE_FLAG_KERNEL_WRITE = ZE_BIT(0),         ///< kernels will write contents
    ZE_IMAGE_FLAG_BIAS_UNCACHED = ZE_BIT(1),        ///< device should not cache contents
    ZE_IMAGE_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_image_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported image types
typedef enum _ze_image_type_t
{
    ZE_IMAGE_TYPE_1D = 0,                           ///< 1D
    ZE_IMAGE_TYPE_1DARRAY = 1,                      ///< 1D array
    ZE_IMAGE_TYPE_2D = 2,                           ///< 2D
    ZE_IMAGE_TYPE_2DARRAY = 3,                      ///< 2D array
    ZE_IMAGE_TYPE_3D = 4,                           ///< 3D
    ZE_IMAGE_TYPE_BUFFER = 5,                       ///< Buffer
    ZE_IMAGE_TYPE_FORCE_UINT32 = 0x7fffffff

} ze_image_type_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported image format layouts
typedef enum _ze_image_format_layout_t
{
    ZE_IMAGE_FORMAT_LAYOUT_8 = 0,                   ///< 8-bit single component layout
    ZE_IMAGE_FORMAT_LAYOUT_16 = 1,                  ///< 16-bit single component layout
    ZE_IMAGE_FORMAT_LAYOUT_32 = 2,                  ///< 32-bit single component layout
    ZE_IMAGE_FORMAT_LAYOUT_8_8 = 3,                 ///< 2-component 8-bit layout
    ZE_IMAGE_FORMAT_LAYOUT_8_8_8_8 = 4,             ///< 4-component 8-bit layout
    ZE_IMAGE_FORMAT_LAYOUT_16_16 = 5,               ///< 2-component 16-bit layout
    ZE_IMAGE_FORMAT_LAYOUT_16_16_16_16 = 6,         ///< 4-component 16-bit layout
    ZE_IMAGE_FORMAT_LAYOUT_32_32 = 7,               ///< 2-component 32-bit layout
    ZE_IMAGE_FORMAT_LAYOUT_32_32_32_32 = 8,         ///< 4-component 32-bit layout
    ZE_IMAGE_FORMAT_LAYOUT_10_10_10_2 = 9,          ///< 4-component 10_10_10_2 layout
    ZE_IMAGE_FORMAT_LAYOUT_11_11_10 = 10,           ///< 3-component 11_11_10 layout
    ZE_IMAGE_FORMAT_LAYOUT_5_6_5 = 11,              ///< 3-component 5_6_5 layout
    ZE_IMAGE_FORMAT_LAYOUT_5_5_5_1 = 12,            ///< 4-component 5_5_5_1 layout
    ZE_IMAGE_FORMAT_LAYOUT_4_4_4_4 = 13,            ///< 4-component 4_4_4_4 layout
    ZE_IMAGE_FORMAT_LAYOUT_Y8 = 14,                 ///< Media Format: Y8. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_NV12 = 15,               ///< Media Format: NV12. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_YUYV = 16,               ///< Media Format: YUYV. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_VYUY = 17,               ///< Media Format: VYUY. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_YVYU = 18,               ///< Media Format: YVYU. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_UYVY = 19,               ///< Media Format: UYVY. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_AYUV = 20,               ///< Media Format: AYUV. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_P010 = 21,               ///< Media Format: P010. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_Y410 = 22,               ///< Media Format: Y410. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_P012 = 23,               ///< Media Format: P012. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_Y16 = 24,                ///< Media Format: Y16. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_P016 = 25,               ///< Media Format: P016. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_Y216 = 26,               ///< Media Format: Y216. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_P216 = 27,               ///< Media Format: P216. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_P8 = 28,                 ///< Media Format: P8. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_YUY2 = 29,               ///< Media Format: YUY2. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_A8P8 = 30,               ///< Media Format: A8P8. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_IA44 = 31,               ///< Media Format: IA44. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_AI44 = 32,               ///< Media Format: AI44. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_Y416 = 33,               ///< Media Format: Y416. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_Y210 = 34,               ///< Media Format: Y210. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_I420 = 35,               ///< Media Format: I420. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_YV12 = 36,               ///< Media Format: YV12. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_400P = 37,               ///< Media Format: 400P. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_422H = 38,               ///< Media Format: 422H. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_422V = 39,               ///< Media Format: 422V. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_444P = 40,               ///< Media Format: 444P. Format type and swizzle is ignored for this.
    ZE_IMAGE_FORMAT_LAYOUT_FORCE_UINT32 = 0x7fffffff

} ze_image_format_layout_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported image format types
typedef enum _ze_image_format_type_t
{
    ZE_IMAGE_FORMAT_TYPE_UINT = 0,                  ///< Unsigned integer
    ZE_IMAGE_FORMAT_TYPE_SINT = 1,                  ///< Signed integer
    ZE_IMAGE_FORMAT_TYPE_UNORM = 2,                 ///< Unsigned normalized integer
    ZE_IMAGE_FORMAT_TYPE_SNORM = 3,                 ///< Signed normalized integer
    ZE_IMAGE_FORMAT_TYPE_FLOAT = 4,                 ///< Float
    ZE_IMAGE_FORMAT_TYPE_FORCE_UINT32 = 0x7fffffff

} ze_image_format_type_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported image format component swizzle into channel
typedef enum _ze_image_format_swizzle_t
{
    ZE_IMAGE_FORMAT_SWIZZLE_R = 0,                  ///< Red component
    ZE_IMAGE_FORMAT_SWIZZLE_G = 1,                  ///< Green component
    ZE_IMAGE_FORMAT_SWIZZLE_B = 2,                  ///< Blue component
    ZE_IMAGE_FORMAT_SWIZZLE_A = 3,                  ///< Alpha component
    ZE_IMAGE_FORMAT_SWIZZLE_0 = 4,                  ///< Zero
    ZE_IMAGE_FORMAT_SWIZZLE_1 = 5,                  ///< One
    ZE_IMAGE_FORMAT_SWIZZLE_X = 6,                  ///< Don't care
    ZE_IMAGE_FORMAT_SWIZZLE_FORCE_UINT32 = 0x7fffffff

} ze_image_format_swizzle_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Image format 
typedef struct _ze_image_format_t
{
    ze_image_format_layout_t layout;                ///< [in] image format component layout
    ze_image_format_type_t type;                    ///< [in] image format type. Media formats can't be used for
                                                    ///< ::ZE_IMAGE_TYPE_BUFFER.
    ze_image_format_swizzle_t x;                    ///< [in] image component swizzle into channel x
    ze_image_format_swizzle_t y;                    ///< [in] image component swizzle into channel y
    ze_image_format_swizzle_t z;                    ///< [in] image component swizzle into channel z
    ze_image_format_swizzle_t w;                    ///< [in] image component swizzle into channel w

} ze_image_format_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Image descriptor
typedef struct _ze_image_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_image_flags_t flags;                         ///< [in] creation flags.
                                                    ///< must be 0 (default) or a valid combination of ::ze_image_flag_t;
                                                    ///< default is read-only, cached access.
    ze_image_type_t type;                           ///< [in] image type
    ze_image_format_t format;                       ///< [in] image format
    uint64_t width;                                 ///< [in] width dimension.
                                                    ///< ::ZE_IMAGE_TYPE_BUFFER: size in bytes; see
                                                    ///< ::ze_device_image_properties_t.maxImageBufferSize for limits.
                                                    ///< ::ZE_IMAGE_TYPE_1D, ::ZE_IMAGE_TYPE_1DARRAY: width in pixels; see
                                                    ///< ::ze_device_image_properties_t.maxImageDims1D for limits.
                                                    ///< ::ZE_IMAGE_TYPE_2D, ::ZE_IMAGE_TYPE_2DARRAY: width in pixels; see
                                                    ///< ::ze_device_image_properties_t.maxImageDims2D for limits.
                                                    ///< ::ZE_IMAGE_TYPE_3D: width in pixels; see
                                                    ///< ::ze_device_image_properties_t.maxImageDims3D for limits.
    uint32_t height;                                ///< [in] height dimension.
                                                    ///< ::ZE_IMAGE_TYPE_2D, ::ZE_IMAGE_TYPE_2DARRAY: height in pixels; see
                                                    ///< ::ze_device_image_properties_t.maxImageDims2D for limits.
                                                    ///< ::ZE_IMAGE_TYPE_3D: height in pixels; see
                                                    ///< ::ze_device_image_properties_t.maxImageDims3D for limits.
                                                    ///< other: ignored.
    uint32_t depth;                                 ///< [in] depth dimension.
                                                    ///< ::ZE_IMAGE_TYPE_3D: depth in pixels; see
                                                    ///< ::ze_device_image_properties_t.maxImageDims3D for limits.
                                                    ///< other: ignored.
    uint32_t arraylevels;                           ///< [in] array levels.
                                                    ///< ::ZE_IMAGE_TYPE_1DARRAY, ::ZE_IMAGE_TYPE_2DARRAY: see
                                                    ///< ::ze_device_image_properties_t.maxImageArraySlices for limits.
                                                    ///< other: ignored.
    uint32_t miplevels;                             ///< [in] mipmap levels (must be 0)

} ze_image_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported sampler filtering flags
typedef uint32_t ze_image_sampler_filter_flags_t;
typedef enum _ze_image_sampler_filter_flag_t
{
    ZE_IMAGE_SAMPLER_FILTER_FLAG_POINT = ZE_BIT(0), ///< device supports point filtering
    ZE_IMAGE_SAMPLER_FILTER_FLAG_LINEAR = ZE_BIT(1),///< device supports linear filtering
    ZE_IMAGE_SAMPLER_FILTER_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_image_sampler_filter_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Image properties
typedef struct _ze_image_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_image_sampler_filter_flags_t samplerFilterFlags; ///< [out] supported sampler filtering.
                                                    ///< returns 0 (unsupported) or a combination of ::ze_image_sampler_filter_flag_t.

} ze_image_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves supported properties of an image.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == pImageProperties`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x3 < desc->flags`
///         + `::ZE_IMAGE_TYPE_BUFFER < desc->type`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeImageGetProperties(
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    const ze_image_desc_t* desc,                    ///< [in] pointer to image descriptor
    ze_image_properties_t* pImageProperties         ///< [out] pointer to image properties
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates an image on the context.
/// 
/// @details
///     - The application must only use the image for the device, or its
///       sub-devices, which was provided during creation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @remarks
///   _Analogues_
///     - clCreateImage
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phImage`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x3 < desc->flags`
///         + `::ZE_IMAGE_TYPE_BUFFER < desc->type`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_IMAGE_FORMAT
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeImageCreate(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    const ze_image_desc_t* desc,                    ///< [in] pointer to image descriptor
    ze_image_handle_t* phImage                      ///< [out] pointer to handle of image object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Deletes an image object.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the image before it is deleted.
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this image.
///     - The application must **not** call this function from simultaneous
///       threads with the same image handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hImage`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeImageDestroy(
    ze_image_handle_t hImage                        ///< [in][release] handle of image object to destroy
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Memory
#if !defined(__GNUC__)
#pragma region memory
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Supported memory allocation flags
typedef uint32_t ze_device_mem_alloc_flags_t;
typedef enum _ze_device_mem_alloc_flag_t
{
    ZE_DEVICE_MEM_ALLOC_FLAG_BIAS_CACHED = ZE_BIT(0),   ///< device should cache allocation
    ZE_DEVICE_MEM_ALLOC_FLAG_BIAS_UNCACHED = ZE_BIT(1), ///< device should not cache allocation (UC)
    ZE_DEVICE_MEM_ALLOC_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_device_mem_alloc_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Device memory allocation descriptor
typedef struct _ze_device_mem_alloc_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_device_mem_alloc_flags_t flags;              ///< [in] flags specifying additional allocation controls.
                                                    ///< must be 0 (default) or a valid combination of ::ze_device_mem_alloc_flag_t;
                                                    ///< default behavior may use implicit driver-based heuristics.
    uint32_t ordinal;                               ///< [in] ordinal of the device's local memory to allocate from.
                                                    ///< must be less than the count returned from ::zeDeviceGetMemoryProperties.

} ze_device_mem_alloc_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported host memory allocation flags
typedef uint32_t ze_host_mem_alloc_flags_t;
typedef enum _ze_host_mem_alloc_flag_t
{
    ZE_HOST_MEM_ALLOC_FLAG_BIAS_CACHED = ZE_BIT(0), ///< host should cache allocation
    ZE_HOST_MEM_ALLOC_FLAG_BIAS_UNCACHED = ZE_BIT(1),   ///< host should not cache allocation (UC)
    ZE_HOST_MEM_ALLOC_FLAG_BIAS_WRITE_COMBINED = ZE_BIT(2), ///< host memory should be allocated write-combined (WC)
    ZE_HOST_MEM_ALLOC_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_host_mem_alloc_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Host memory allocation descriptor
typedef struct _ze_host_mem_alloc_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_host_mem_alloc_flags_t flags;                ///< [in] flags specifying additional allocation controls.
                                                    ///< must be 0 (default) or a valid combination of ::ze_host_mem_alloc_flag_t;
                                                    ///< default behavior may use implicit driver-based heuristics.

} ze_host_mem_alloc_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Allocates shared memory on the context.
/// 
/// @details
///     - Shared allocations share ownership between the host and one or more
///       devices.
///     - Shared allocations may optionally be associated with a device by
///       passing a handle to the device.
///     - Devices supporting only single-device shared access capabilities may
///       access shared memory associated with the device.
///       For these devices, ownership of the allocation is shared between the
///       host and the associated device only.
///     - Passing nullptr as the device handle does not associate the shared
///       allocation with any device.
///       For allocations with no associated device, ownership of the allocation
///       is shared between the host and all devices supporting cross-device
///       shared access capabilities.
///     - The application must only use the memory allocation for the context
///       and device, or its sub-devices, which was provided during allocation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == device_desc`
///         + `nullptr == host_desc`
///         + `nullptr == pptr`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x3 < device_desc->flags`
///         + `0x7 < host_desc->flags`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT
///         + Must be zero or a power-of-two
///         + `0 != (alignment & (alignment - 1))`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeMemAllocShared(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const ze_device_mem_alloc_desc_t* device_desc,  ///< [in] pointer to device memory allocation descriptor
    const ze_host_mem_alloc_desc_t* host_desc,      ///< [in] pointer to host memory allocation descriptor
    size_t size,                                    ///< [in] size in bytes to allocate; must be less-than
                                                    ///< ::ze_device_properties_t.maxMemAllocSize.
    size_t alignment,                               ///< [in] minimum alignment in bytes for the allocation; must be a power of
                                                    ///< two.
    ze_device_handle_t hDevice,                     ///< [in][optional] device handle to associate with
    void** pptr                                     ///< [out] pointer to shared allocation
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Allocates device memory on the context.
/// 
/// @details
///     - Device allocations are owned by a specific device.
///     - In general, a device allocation may only be accessed by the device
///       that owns it.
///     - The application must only use the memory allocation for the context
///       and device, or its sub-devices, which was provided during allocation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == device_desc`
///         + `nullptr == pptr`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x3 < device_desc->flags`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT
///         + Must be zero or a power-of-two
///         + `0 != (alignment & (alignment - 1))`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeMemAllocDevice(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const ze_device_mem_alloc_desc_t* device_desc,  ///< [in] pointer to device memory allocation descriptor
    size_t size,                                    ///< [in] size in bytes to allocate; must be less-than
                                                    ///< ::ze_device_properties_t.maxMemAllocSize.
    size_t alignment,                               ///< [in] minimum alignment in bytes for the allocation; must be a power of
                                                    ///< two.
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    void** pptr                                     ///< [out] pointer to device allocation
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Allocates host memory on the context.
/// 
/// @details
///     - Host allocations are owned by the host process.
///     - Host allocations are accessible by the host and all devices within the
///       driver's context.
///     - Host allocations are frequently used as staging areas to transfer data
///       to or from devices.
///     - The application must only use the memory allocation for the context
///       which was provided during allocation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == host_desc`
///         + `nullptr == pptr`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x7 < host_desc->flags`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT
///         + Must be zero or a power-of-two
///         + `0 != (alignment & (alignment - 1))`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeMemAllocHost(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const ze_host_mem_alloc_desc_t* host_desc,      ///< [in] pointer to host memory allocation descriptor
    size_t size,                                    ///< [in] size in bytes to allocate; must be less-than
                                                    ///< ::ze_device_properties_t.maxMemAllocSize.
    size_t alignment,                               ///< [in] minimum alignment in bytes for the allocation; must be a power of
                                                    ///< two.
    void** pptr                                     ///< [out] pointer to host allocation
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Frees allocated host memory, device memory, or shared memory on the
///        context.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the memory before it is freed
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this memory
///     - The application must **not** call this function from simultaneous
///       threads with the same pointer.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeMemFree(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    void* ptr                                       ///< [in][release] pointer to memory to free
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Memory allocation type
typedef enum _ze_memory_type_t
{
    ZE_MEMORY_TYPE_UNKNOWN = 0,                     ///< the memory pointed to is of unknown type
    ZE_MEMORY_TYPE_HOST = 1,                        ///< the memory pointed to is a host allocation
    ZE_MEMORY_TYPE_DEVICE = 2,                      ///< the memory pointed to is a device allocation
    ZE_MEMORY_TYPE_SHARED = 3,                      ///< the memory pointed to is a shared ownership allocation
    ZE_MEMORY_TYPE_FORCE_UINT32 = 0x7fffffff

} ze_memory_type_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Memory allocation properties queried using ::zeMemGetAllocProperties
typedef struct _ze_memory_allocation_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_memory_type_t type;                          ///< [out] type of allocated memory
    uint64_t id;                                    ///< [out] identifier for this allocation
    uint64_t pageSize;                              ///< [out] page size used for allocation

} ze_memory_allocation_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves attributes of a memory allocation
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The application may query attributes of a memory allocation unrelated
///       to the context.
///       When this occurs, the returned allocation type will be
///       ::ZE_MEMORY_TYPE_UNKNOWN, and the returned identifier and associated
///       device is unspecified.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///         + `nullptr == pMemAllocProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeMemGetAllocProperties(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* ptr,                                ///< [in] memory pointer to query
    ze_memory_allocation_properties_t* pMemAllocProperties, ///< [in,out] query result for memory allocation properties
    ze_device_handle_t* phDevice                    ///< [out][optional] device associated with this allocation
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves the base address and/or size of an allocation
/// 
/// @details
///     - The application may call this function from simultaneous threads.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeMemGetAddressRange(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* ptr,                                ///< [in] memory pointer to query
    void** pBase,                                   ///< [in,out][optional] base address of the allocation
    size_t* pSize                                   ///< [in,out][optional] size of the allocation
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates an IPC memory handle for the specified allocation
/// 
/// @details
///     - Takes a pointer to a device memory allocation and creates an IPC
///       memory handle for exporting it for use in another process.
///     - The pointer must be base pointer of the device memory allocation; i.e.
///       the value returned from ::zeMemAllocDevice.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///         + `nullptr == pIpcHandle`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeMemGetIpcHandle(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* ptr,                                ///< [in] pointer to the device memory allocation
    ze_ipc_mem_handle_t* pIpcHandle                 ///< [out] Returned IPC memory handle
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported IPC memory flags
typedef uint32_t ze_ipc_memory_flags_t;
typedef enum _ze_ipc_memory_flag_t
{
    ZE_IPC_MEMORY_FLAG_TBD = ZE_BIT(0),             ///< reserved for future use
    ZE_IPC_MEMORY_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_ipc_memory_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Opens an IPC memory handle to retrieve a device pointer on the
///        context.
/// 
/// @details
///     - Takes an IPC memory handle from a remote process and associates it
///       with a device pointer usable in this process.
///     - The device pointer in this process should not be freed with
///       ::zeMemFree, but rather with ::zeMemCloseIpcHandle.
///     - Multiple calls to this function with the same IPC handle will return
///       unique pointers.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x1 < flags`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pptr`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeMemOpenIpcHandle(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device to associate with the IPC memory handle
    ze_ipc_mem_handle_t handle,                     ///< [in] IPC memory handle
    ze_ipc_memory_flags_t flags,                    ///< [in] flags controlling the operation.
                                                    ///< must be 0 (default) or a valid combination of ::ze_ipc_memory_flag_t.
    void** pptr                                     ///< [out] pointer to device allocation in this process
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Closes an IPC memory handle
/// 
/// @details
///     - Closes an IPC memory handle by unmapping memory that was opened in
///       this process using ::zeMemOpenIpcHandle.
///     - The application must **not** call this function from simultaneous
///       threads with the same pointer.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeMemCloseIpcHandle(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* ptr                                 ///< [in][release] pointer to device allocation in this process
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Additional allocation descriptor for exporting external memory
/// 
/// @details
///     - This structure may be passed to ::zeMemAllocDevice, via the `pNext`
///       member of ::ze_device_mem_alloc_desc_t, to indicate an exportable
///       memory allocation.
///     - This structure may be passed to ::zeImageCreate, via the `pNext`
///       member of ::ze_image_desc_t, to indicate an exportable image.
typedef struct _ze_external_memory_export_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_external_memory_type_flags_t flags;          ///< [in] flags specifying memory export types for this allocation.
                                                    ///< must be 0 (default) or a valid combination of ::ze_external_memory_type_flags_t

} ze_external_memory_export_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Additional allocation descriptor for importing external memory as a
///        file descriptor
/// 
/// @details
///     - This structure may be passed to ::zeMemAllocDevice, via the `pNext`
///       member of ::ze_device_mem_alloc_desc_t, to import memory from a file
///       descriptor.
///     - This structure may be passed to ::zeImageCreate, via the `pNext`
///       member of ::ze_image_desc_t, to import memory from a file descriptor.
typedef struct _ze_external_memory_import_fd_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_external_memory_type_flags_t flags;          ///< [in] flags specifying the memory import type for the file descriptor.
                                                    ///< must be 0 (default) or a valid combination of ::ze_external_memory_type_flags_t
    int fd;                                         ///< [in] the file descriptor handle to import

} ze_external_memory_import_fd_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Exports an allocation as a file descriptor
/// 
/// @details
///     - This structure may be passed to ::zeMemGetAllocProperties, via the
///       `pNext` member of ::ze_memory_allocation_properties_t, to export a
///       memory allocation as a file descriptor.
///     - This structure may be passed to ::zeImageGetProperties, via the
///       `pNext` member of ::ze_image_properties_t, to export an image as a
///       file descriptor.
///     - The requested memory export type must have been specified when the
///       allocation was made.
typedef struct _ze_external_memory_export_fd_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_external_memory_type_flags_t flags;          ///< [in] flags specifying the memory export type for the file descriptor.
                                                    ///< must be 0 (default) or a valid combination of ::ze_external_memory_type_flags_t
    int fd;                                         ///< [out] the exported file descriptor handle representing the allocation.

} ze_external_memory_export_fd_t;

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Module
#if !defined(__GNUC__)
#pragma region module
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Supported module creation input formats
typedef enum _ze_module_format_t
{
    ZE_MODULE_FORMAT_IL_SPIRV = 0,                  ///< Format is SPIRV IL format
    ZE_MODULE_FORMAT_NATIVE = 1,                    ///< Format is device native format
    ZE_MODULE_FORMAT_FORCE_UINT32 = 0x7fffffff

} ze_module_format_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Specialization constants - User defined constants
typedef struct _ze_module_constants_t
{
    uint32_t numConstants;                          ///< [in] Number of specialization constants.
    const uint32_t* pConstantIds;                   ///< [in][range(0, numConstants)] Array of IDs that is sized to
                                                    ///< numConstants.
    const void** pConstantValues;                   ///< [in][range(0, numConstants)] Array of pointers to values that is sized
                                                    ///< to numConstants.

} ze_module_constants_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Module descriptor
typedef struct _ze_module_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_module_format_t format;                      ///< [in] Module format passed in with pInputModule
    size_t inputSize;                               ///< [in] size of input IL or ISA from pInputModule.
    const uint8_t* pInputModule;                    ///< [in] pointer to IL or ISA
    const char* pBuildFlags;                        ///< [in][optional] string containing compiler flags. Following options are supported.
                                                    ///<  - "-ze-opt-disable"
                                                    ///<       - Disable optimizations
                                                    ///<  - "-ze-opt-greater-than-4GB-buffer-required"
                                                    ///<       - Use 64-bit offset calculations for buffers.
                                                    ///<  - "-ze-opt-large-register-file"
                                                    ///<       - Increase number of registers available to threads.
                                                    ///<  - "-ze-opt-has-buffer-offset-arg"
                                                    ///<       - Extend stateless to stateful optimization to more
                                                    ///<         cases with the use of additional offset (e.g. 64-bit
                                                    ///<         pointer to binding table with 32-bit offset).
                                                    ///<  - "-g"
                                                    ///<       - Include debugging information.
    const ze_module_constants_t* pConstants;        ///< [in][optional] pointer to specialization constants. Valid only for
                                                    ///< SPIR-V input. This must be set to nullptr if no specialization
                                                    ///< constants are provided.

} ze_module_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates a module on the context.
/// 
/// @details
///     - Compiles the module for execution on the device.
///     - The application must only use the module for the device, or its
///       sub-devices, which was provided during creation.
///     - The module can be copied to other devices and contexts within the same
///       driver instance by using ::zeModuleGetNativeBinary.
///     - A build log can optionally be returned to the caller. The caller is
///       responsible for destroying build log using ::zeModuleBuildLogDestroy.
///     - The module descriptor constants are only supported for SPIR-V
///       specialization constants.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == desc->pInputModule`
///         + `nullptr == phModule`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `::ZE_MODULE_FORMAT_NATIVE < desc->format`
///     - ::ZE_RESULT_ERROR_INVALID_NATIVE_BINARY
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `0 == desc->inputSize`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
///     - ::ZE_RESULT_ERROR_MODULE_BUILD_FAILURE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleCreate(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    const ze_module_desc_t* desc,                   ///< [in] pointer to module descriptor
    ze_module_handle_t* phModule,                   ///< [out] pointer to handle of module object created
    ze_module_build_log_handle_t* phBuildLog        ///< [out][optional] pointer to handle of module's build log.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Destroys module
/// 
/// @details
///     - The application must destroy all kernel and build log handles created
///       from the module before destroying the module itself.
///     - The application must ensure the device is not currently referencing
///       the module before it is deleted.
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this module.
///     - The application must **not** call this function from simultaneous
///       threads with the same module handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hModule`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleDestroy(
    ze_module_handle_t hModule                      ///< [in][release] handle of the module
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Dynamically link modules together that share import/export linkage
///        dependencies.
/// 
/// @details
///     - Modules support import and export linkage for functions and global
///       variables.
///     - Modules that have imports can be dynamically linked to export modules
///       that satisfy those import requirements.
///     - Modules can have both import and export linkages.
///     - Modules that do not have any imports or exports do not need to be
///       linked.
///     - Modules cannot be partially linked. All modules needed to satisfy all
///       import dependencies for a module must be passed in or
///       ::ZE_RESULT_ERROR_MODULE_LINK_FAILURE will returned.
///     - Modules with imports need to be linked before kernel objects can be
///       created from them.
///     - Modules will only be linked once. A module can be used in multiple
///       link calls if it has exports but it's imports will not be re-linked.
///     - Ambiguous dependencies, where multiple modules satisfy the import
///       dependencies for another module, is not allowed.
///     - ModuleGetNativeBinary can be called on any module regardless of
///       whether it is linked or not.
///     - A link log can optionally be returned to the caller. The caller is
///       responsible for destroying build log using ::zeModuleBuildLogDestroy.
///     - See SPIR-V specification for linkage details.
///     - The application must ensure the modules being linked were created on
///       the same context.
///     - The application may call this function from simultaneous threads as
///       long as the import modules being linked are not the same.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == phModules`
///     - ::ZE_RESULT_ERROR_MODULE_LINK_FAILURE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleDynamicLink(
    uint32_t numModules,                            ///< [in] number of modules to be linked pointed to by phModules.
    ze_module_handle_t* phModules,                  ///< [in][range(0, numModules)] pointer to an array of modules to
                                                    ///< dynamically link together.
    ze_module_build_log_handle_t* phLinkLog         ///< [out][optional] pointer to handle of dynamic link log.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Destroys module build log object
/// 
/// @details
///     - The implementation of this function may immediately free all Host
///       allocations associated with this object.
///     - The application must **not** call this function from simultaneous
///       threads with the same build log handle.
///     - The implementation of this function should be lock-free.
///     - This function can be called before or after ::zeModuleDestroy for the
///       associated module.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hModuleBuildLog`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleBuildLogDestroy(
    ze_module_build_log_handle_t hModuleBuildLog    ///< [in][release] handle of the module build log object.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieves text string for build log.
/// 
/// @details
///     - The caller can pass nullptr for pBuildLog when querying only for size.
///     - The caller must provide memory for build log.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hModuleBuildLog`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pSize`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleBuildLogGetString(
    ze_module_build_log_handle_t hModuleBuildLog,   ///< [in] handle of the module build log object.
    size_t* pSize,                                  ///< [in,out] size of build log string.
    char* pBuildLog                                 ///< [in,out][optional] pointer to null-terminated string of the log.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieve native binary from Module.
/// 
/// @details
///     - The native binary output can be cached to disk and new modules can be
///       later constructed from the cached copy.
///     - The native binary will retain debugging information that is associated
///       with a module.
///     - The caller can pass nullptr for pModuleNativeBinary when querying only
///       for size.
///     - The implementation will copy the native binary into a buffer supplied
///       by the caller.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hModule`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pSize`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleGetNativeBinary(
    ze_module_handle_t hModule,                     ///< [in] handle of the module
    size_t* pSize,                                  ///< [in,out] size of native binary in bytes.
    uint8_t* pModuleNativeBinary                    ///< [in,out][optional] byte pointer to native binary
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieve global variable pointer from Module.
/// 
/// @details
///     - The application may query global pointer from any module that either
///       exports or imports it.
///     - The application must dynamically link a module that imports a global
///       before the global pointer can be queried from it.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hModule`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pGlobalName`
///     - ::ZE_RESULT_ERROR_INVALID_GLOBAL_NAME
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleGetGlobalPointer(
    ze_module_handle_t hModule,                     ///< [in] handle of the module
    const char* pGlobalName,                        ///< [in] name of global variable in module
    size_t* pSize,                                  ///< [in,out][optional] size of global variable
    void** pptr                                     ///< [in,out][optional] device visible pointer
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieve all kernel names in the module.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hModule`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pCount`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleGetKernelNames(
    ze_module_handle_t hModule,                     ///< [in] handle of the module
    uint32_t* pCount,                               ///< [in,out] pointer to the number of names.
                                                    ///< if count is zero, then the driver shall update the value with the
                                                    ///< total number of names available.
                                                    ///< if count is greater than the number of names available, then the
                                                    ///< driver shall update the value with the correct number of names available.
    const char** pNames                             ///< [in,out][optional][range(0, *pCount)] array of names of functions.
                                                    ///< if count is less than the number of names available, then driver shall
                                                    ///< only retrieve that number of names.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported module property flags
typedef uint32_t ze_module_property_flags_t;
typedef enum _ze_module_property_flag_t
{
    ZE_MODULE_PROPERTY_FLAG_IMPORTS = ZE_BIT(0),    ///< Module has imports (i.e. imported global variables and/or kernels).
                                                    ///< See ::zeModuleDynamicLink.
    ZE_MODULE_PROPERTY_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_module_property_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Module properties
typedef struct _ze_module_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_module_property_flags_t flags;               ///< [out] 0 (none) or a valid combination of ::ze_module_property_flag_t

} ze_module_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieve module properties.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hModule`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pModuleProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleGetProperties(
    ze_module_handle_t hModule,                     ///< [in] handle of the module
    ze_module_properties_t* pModuleProperties       ///< [in,out] query result for module properties.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported kernel creation flags
typedef uint32_t ze_kernel_flags_t;
typedef enum _ze_kernel_flag_t
{
    ZE_KERNEL_FLAG_FORCE_RESIDENCY = ZE_BIT(0),     ///< force all device allocations to be resident during execution
    ZE_KERNEL_FLAG_EXPLICIT_RESIDENCY = ZE_BIT(1),  ///< application is responsible for all residency of device allocations.
                                                    ///< driver may disable implicit residency management.
    ZE_KERNEL_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_kernel_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Kernel descriptor
typedef struct _ze_kernel_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_kernel_flags_t flags;                        ///< [in] creation flags.
                                                    ///< must be 0 (default) or a valid combination of ::ze_kernel_flag_t;
                                                    ///< default behavior may use driver-based residency.
    const char* pKernelName;                        ///< [in] null-terminated name of kernel in module

} ze_kernel_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Create a kernel from the module.
/// 
/// @details
///     - Modules that have unresolved imports need to be dynamically linked
///       before a kernel can be created from them. (See ::zeModuleDynamicLink)
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hModule`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == desc->pKernelName`
///         + `nullptr == phKernel`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x3 < desc->flags`
///     - ::ZE_RESULT_ERROR_INVALID_KERNEL_NAME
///     - ::ZE_RESULT_ERROR_INVALID_MODULE_UNLINKED
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelCreate(
    ze_module_handle_t hModule,                     ///< [in] handle of the module
    const ze_kernel_desc_t* desc,                   ///< [in] pointer to kernel descriptor
    ze_kernel_handle_t* phKernel                    ///< [out] handle of the Function object
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Destroys a kernel object
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the kernel before it is deleted.
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this kernel.
///     - The application must **not** call this function from simultaneous
///       threads with the same kernel handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelDestroy(
    ze_kernel_handle_t hKernel                      ///< [in][release] handle of the kernel object
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieve a function pointer from a module by name
/// 
/// @details
///     - The function pointer is unique for the device on which the module was
///       created.
///     - The function pointer is no longer valid if module is destroyed.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hModule`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pFunctionName`
///         + `nullptr == pfnFunction`
///     - ::ZE_RESULT_ERROR_INVALID_FUNCTION_NAME
ZE_APIEXPORT ze_result_t ZE_APICALL
zeModuleGetFunctionPointer(
    ze_module_handle_t hModule,                     ///< [in] handle of the module
    const char* pFunctionName,                      ///< [in] Name of function to retrieve function pointer for.
    void** pfnFunction                              ///< [out] pointer to function.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Set group size for a kernel on the current Host thread.
/// 
/// @details
///     - The group size will be used when a ::zeCommandListAppendLaunchKernel
///       variant is called.
///     - The application must **not** call this function from simultaneous
///       threads with the same kernel handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_GROUP_SIZE_DIMENSION
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelSetGroupSize(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    uint32_t groupSizeX,                            ///< [in] group size for X dimension to use for this kernel
    uint32_t groupSizeY,                            ///< [in] group size for Y dimension to use for this kernel
    uint32_t groupSizeZ                             ///< [in] group size for Z dimension to use for this kernel
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Query a suggested group size for a kernel given a global size for each
///        dimension.
/// 
/// @details
///     - This function ignores the group size that is set using
///       ::zeKernelSetGroupSize.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == groupSizeX`
///         + `nullptr == groupSizeY`
///         + `nullptr == groupSizeZ`
///     - ::ZE_RESULT_ERROR_INVALID_GLOBAL_WIDTH_DIMENSION
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelSuggestGroupSize(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    uint32_t globalSizeX,                           ///< [in] global width for X dimension
    uint32_t globalSizeY,                           ///< [in] global width for Y dimension
    uint32_t globalSizeZ,                           ///< [in] global width for Z dimension
    uint32_t* groupSizeX,                           ///< [out] recommended size of group for X dimension
    uint32_t* groupSizeY,                           ///< [out] recommended size of group for Y dimension
    uint32_t* groupSizeZ                            ///< [out] recommended size of group for Z dimension
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Query a suggested max group count for a cooperative kernel.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == totalGroupCount`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelSuggestMaxCooperativeGroupCount(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    uint32_t* totalGroupCount                       ///< [out] recommended total group count.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Set kernel argument for a kernel on the current Host thread.
/// 
/// @details
///     - The argument values will be used when a
///       ::zeCommandListAppendLaunchKernel variant is called.
///     - The application must **not** call this function from simultaneous
///       threads with the same kernel handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_INDEX
///     - ::ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_SIZE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelSetArgumentValue(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    uint32_t argIndex,                              ///< [in] argument index in range [0, num args - 1]
    size_t argSize,                                 ///< [in] size of argument type
    const void* pArgValue                           ///< [in][optional] argument value represented as matching arg type. If
                                                    ///< null then argument value is considered null.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Kernel indirect access flags
typedef uint32_t ze_kernel_indirect_access_flags_t;
typedef enum _ze_kernel_indirect_access_flag_t
{
    ZE_KERNEL_INDIRECT_ACCESS_FLAG_HOST = ZE_BIT(0),///< Indicates that the kernel accesses host allocations indirectly.
    ZE_KERNEL_INDIRECT_ACCESS_FLAG_DEVICE = ZE_BIT(1),  ///< Indicates that the kernel accesses device allocations indirectly.
    ZE_KERNEL_INDIRECT_ACCESS_FLAG_SHARED = ZE_BIT(2),  ///< Indicates that the kernel accesses shared allocations indirectly.
    ZE_KERNEL_INDIRECT_ACCESS_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_kernel_indirect_access_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Sets kernel indirect access flags.
/// 
/// @details
///     - The application should specify which allocations will be indirectly
///       accessed by the kernel to allow driver to optimize which allocations
///       are made resident
///     - This function may **not** be called from simultaneous threads with the
///       same Kernel handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x7 < flags`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelSetIndirectAccess(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    ze_kernel_indirect_access_flags_t flags         ///< [in] kernel indirect access flags
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieve kernel indirect access flags.
/// 
/// @details
///     - This function may be called from simultaneous threads with the same
///       Kernel handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pFlags`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelGetIndirectAccess(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    ze_kernel_indirect_access_flags_t* pFlags       ///< [out] query result for kernel indirect access flags.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieve all declared kernel attributes (i.e. can be specified with
///        __attribute__ in runtime language).
/// 
/// @details
///     - This function may be called from simultaneous threads with the same
///       Kernel handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pSize`
///         + `nullptr == pString`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelGetSourceAttributes(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    uint32_t* pSize,                                ///< [in,out] pointer to size of string in bytes.
    char** pString                                  ///< [in,out] pointer to null-terminated string, whose lifetime is tied to
                                                    ///< the kernel object, where kernel source attributes are separated by
                                                    ///< space.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported Cache Config flags
typedef uint32_t ze_cache_config_flags_t;
typedef enum _ze_cache_config_flag_t
{
    ZE_CACHE_CONFIG_FLAG_LARGE_SLM = ZE_BIT(0),     ///< Large SLM size
    ZE_CACHE_CONFIG_FLAG_LARGE_DATA = ZE_BIT(1),    ///< Large General Data size
    ZE_CACHE_CONFIG_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_cache_config_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Sets the preferred cache configuration for a kernel on the current
///        Host thread.
/// 
/// @details
///     - The cache configuration will be used when a
///       ::zeCommandListAppendLaunchKernel variant is called.
///     - The application must **not** call this function from simultaneous
///       threads with the same kernel handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x3 < flags`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_FEATURE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelSetCacheConfig(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    ze_cache_config_flags_t flags                   ///< [in] cache configuration. 
                                                    ///< must be 0 (default configuration) or a valid combination of ::ze_cache_config_flag_t.
    );

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAX_KERNEL_UUID_SIZE
/// @brief Maximum kernel universal unique id (UUID) size in bytes
#define ZE_MAX_KERNEL_UUID_SIZE  16
#endif // ZE_MAX_KERNEL_UUID_SIZE

///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MAX_MODULE_UUID_SIZE
/// @brief Maximum module universal unique id (UUID) size in bytes
#define ZE_MAX_MODULE_UUID_SIZE  16
#endif // ZE_MAX_MODULE_UUID_SIZE

///////////////////////////////////////////////////////////////////////////////
/// @brief Kernel universal unique id (UUID)
typedef struct _ze_kernel_uuid_t
{
    uint8_t kid[ZE_MAX_KERNEL_UUID_SIZE];           ///< [out] opaque data representing a kernel UUID
    uint8_t mid[ZE_MAX_MODULE_UUID_SIZE];           ///< [out] opaque data representing the kernel's module UUID

} ze_kernel_uuid_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Kernel properties
typedef struct _ze_kernel_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    uint32_t numKernelArgs;                         ///< [out] number of kernel arguments.
    uint32_t requiredGroupSizeX;                    ///< [out] required group size in the X dimension,
                                                    ///< or zero if there is no required group size
    uint32_t requiredGroupSizeY;                    ///< [out] required group size in the Y dimension,
                                                    ///< or zero if there is no required group size
    uint32_t requiredGroupSizeZ;                    ///< [out] required group size in the Z dimension,
                                                    ///< or zero if there is no required group size
    uint32_t requiredNumSubGroups;                  ///< [out] required number of subgroups per thread group,
                                                    ///< or zero if there is no required number of subgroups
    uint32_t requiredSubgroupSize;                  ///< [out] required subgroup size,
                                                    ///< or zero if there is no required subgroup size
    uint32_t maxSubgroupSize;                       ///< [out] maximum subgroup size
    uint32_t maxNumSubgroups;                       ///< [out] maximum number of subgroups per thread group
    uint32_t localMemSize;                          ///< [out] local memory size used by each thread group
    uint32_t privateMemSize;                        ///< [out] private memory size allocated by compiler used by each thread
    uint32_t spillMemSize;                          ///< [out] spill memory size allocated by compiler
    ze_kernel_uuid_t uuid;                          ///< [out] universal unique identifier.

} ze_kernel_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieve kernel properties.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pKernelProperties`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelGetProperties(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    ze_kernel_properties_t* pKernelProperties       ///< [in,out] query result for kernel properties.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Retrieve kernel name from Kernel.
/// 
/// @details
///     - The caller can pass nullptr for pName when querying only for size.
///     - The implementation will copy the kernel name into a buffer supplied by
///       the caller.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pSize`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelGetName(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    size_t* pSize,                                  ///< [in,out] size of kernel name string, including null terminator, in
                                                    ///< bytes.
    char* pName                                     ///< [in,out][optional] char pointer to kernel name.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Kernel dispatch group count.
typedef struct _ze_group_count_t
{
    uint32_t groupCountX;                           ///< [in] number of thread groups in X dimension
    uint32_t groupCountY;                           ///< [in] number of thread groups in Y dimension
    uint32_t groupCountZ;                           ///< [in] number of thread groups in Z dimension

} ze_group_count_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Launch kernel over one or more work groups.
/// 
/// @details
///     - The application must ensure the kernel and events are accessible by
///       the device on which the command list was created.
///     - This may **only** be called for a command list created with command
///       queue group ordinal that supports compute.
///     - The application must ensure the command list, kernel and events were
///       created on the same context.
///     - This function may **not** be called from simultaneous threads with the
///       same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pLaunchFuncArgs`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendLaunchKernel(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    const ze_group_count_t* pLaunchFuncArgs,        ///< [in] thread group launch arguments
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Launch kernel cooperatively over one or more work groups.
/// 
/// @details
///     - The application must ensure the kernel and events are accessible by
///       the device on which the command list was created.
///     - This may **only** be called for a command list created with command
///       queue group ordinal that supports compute.
///     - This may only be used for a command list that are submitted to command
///       queue with cooperative flag set.
///     - The application must ensure the command list, kernel and events were
///       created on the same context.
///     - This function may **not** be called from simultaneous threads with the
///       same command list handle.
///     - The implementation of this function should be lock-free.
///     - Use ::zeKernelSuggestMaxCooperativeGroupCount to recommend max group
///       count for device for cooperative functions that device supports.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pLaunchFuncArgs`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendLaunchCooperativeKernel(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    const ze_group_count_t* pLaunchFuncArgs,        ///< [in] thread group launch arguments
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Launch kernel over one or more work groups using indirect arguments.
/// 
/// @details
///     - The application must ensure the kernel and events are accessible by
///       the device on which the command list was created.
///     - The application must ensure the launch arguments are visible to the
///       device on which the command list was created.
///     - The implementation must not access the contents of the launch
///       arguments as they are free to be modified by either the Host or device
///       up until execution.
///     - This may **only** be called for a command list created with command
///       queue group ordinal that supports compute.
///     - The application must ensure the command list, kernel and events were
///       created, and the memory was allocated, on the same context.
///     - This function may **not** be called from simultaneous threads with the
///       same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///         + `nullptr == hKernel`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pLaunchArgumentsBuffer`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendLaunchKernelIndirect(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    const ze_group_count_t* pLaunchArgumentsBuffer, ///< [in] pointer to device buffer that will contain thread group launch
                                                    ///< arguments
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Launch multiple kernels over one or more work groups using an array of
///        indirect arguments.
/// 
/// @details
///     - The application must ensure the kernel and events are accessible by
///       the device on which the command list was created.
///     - The application must ensure the array of launch arguments and count
///       buffer are visible to the device on which the command list was
///       created.
///     - The implementation must not access the contents of the array of launch
///       arguments or count buffer as they are free to be modified by either
///       the Host or device up until execution.
///     - This may **only** be called for a command list created with command
///       queue group ordinal that supports compute.
///     - The application must enusre the command list, kernel and events were
///       created, and the memory was allocated, on the same context.
///     - This function may **not** be called from simultaneous threads with the
///       same command list handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hCommandList`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == phKernels`
///         + `nullptr == pCountBuffer`
///         + `nullptr == pLaunchArgumentsBuffer`
///     - ::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT
///     - ::ZE_RESULT_ERROR_INVALID_SIZE
///         + `(nullptr == phWaitEvents) && (0 < numWaitEvents)`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeCommandListAppendLaunchMultipleKernelsIndirect(
    ze_command_list_handle_t hCommandList,          ///< [in] handle of the command list
    uint32_t numKernels,                            ///< [in] maximum number of kernels to launch
    ze_kernel_handle_t* phKernels,                  ///< [in][range(0, numKernels)] handles of the kernel objects
    const uint32_t* pCountBuffer,                   ///< [in] pointer to device memory location that will contain the actual
                                                    ///< number of kernels to launch; value must be less-than or equal-to
                                                    ///< numKernels
    const ze_group_count_t* pLaunchArgumentsBuffer, ///< [in][range(0, numKernels)] pointer to device buffer that will contain
                                                    ///< a contiguous array of thread group launch arguments
    ze_event_handle_t hSignalEvent,                 ///< [in][optional] handle of the event to signal on completion
    uint32_t numWaitEvents,                         ///< [in][optional] number of events to wait on before launching; must be 0
                                                    ///< if `nullptr == phWaitEvents`
    ze_event_handle_t* phWaitEvents                 ///< [in][optional][range(0, numWaitEvents)] handle of the events to wait
                                                    ///< on before launching
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero Extension for supporting module programs.
#if !defined(__GNUC__)
#pragma region program
#endif
///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MODULE_PROGRAM_EXP_NAME
/// @brief Module Program Extension Name
#define ZE_MODULE_PROGRAM_EXP_NAME  "ZE_experimental_module_program"
#endif // ZE_MODULE_PROGRAM_EXP_NAME

///////////////////////////////////////////////////////////////////////////////
/// @brief Module Program Extension Version(s)
typedef enum _ze_module_program_exp_version_t
{
    ZE_MODULE_PROGRAM_EXP_VERSION_1_0 = ZE_MAKE_VERSION( 1, 0 ),///< version 1.0
    ZE_MODULE_PROGRAM_EXP_VERSION_CURRENT = ZE_MAKE_VERSION( 1, 0 ),///< latest known version
    ZE_MODULE_PROGRAM_EXP_VERSION_FORCE_UINT32 = 0x7fffffff

} ze_module_program_exp_version_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Module extended descriptor to support multiple input modules.
/// 
/// @details
///     - Implementation must support ::ZE_experimental_module_program extension
///     - pInputModules, pBuildFlags, and pConstants from ::ze_module_desc_t is
///       ignored.
///     - Format in ::ze_module_desc_t needs to be set to
///       ::ZE_MODULE_FORMAT_IL_SPIRV.
typedef struct _ze_module_program_exp_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    uint32_t count;                                 ///< [in] Count of input modules
    const size_t* inputSizes;                       ///< [in][range(0, count)] sizes of each input IL module in pInputModules.
    const uint8_t** pInputModules;                  ///< [in][range(0, count)] pointer to an array of IL (e.g. SPIR-V modules).
                                                    ///< Valid only for SPIR-V input.
    const char** pBuildFlags;                       ///< [in][optional][range(0, count)] array of strings containing build
                                                    ///< flags. See pBuildFlags in ::ze_module_desc_t.
    const ze_module_constants_t** pConstants;       ///< [in][optional][range(0, count)] pointer to array of specialization
                                                    ///< constant strings. Valid only for SPIR-V input. This must be set to
                                                    ///< nullptr if no specialization constants are provided.

} ze_module_program_exp_desc_t;

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero Extension APIs for Raytracing
#if !defined(__GNUC__)
#pragma region raytracing
#endif
///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_RAYTRACING_EXT_NAME
/// @brief Raytracing Extension Name
#define ZE_RAYTRACING_EXT_NAME  "ZE_extension_raytracing"
#endif // ZE_RAYTRACING_EXT_NAME

///////////////////////////////////////////////////////////////////////////////
/// @brief Raytracing Extension Version(s)
typedef enum _ze_raytracing_ext_version_t
{
    ZE_RAYTRACING_EXT_VERSION_1_0 = ZE_MAKE_VERSION( 1, 0 ),///< version 1.0
    ZE_RAYTRACING_EXT_VERSION_CURRENT = ZE_MAKE_VERSION( 1, 0 ),///< latest known version
    ZE_RAYTRACING_EXT_VERSION_FORCE_UINT32 = 0x7fffffff

} ze_raytracing_ext_version_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported raytracing capability flags
typedef uint32_t ze_device_raytracing_ext_flags_t;
typedef enum _ze_device_raytracing_ext_flag_t
{
    ZE_DEVICE_RAYTRACING_EXT_FLAG_RAYQUERY = ZE_BIT(0), ///< Supports rayquery
    ZE_DEVICE_RAYTRACING_EXT_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_device_raytracing_ext_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Raytracing properties queried using ::zeDeviceGetModuleProperties
/// 
/// @details
///     - This structure may be returned from ::zeDeviceGetModuleProperties, via
///       `pNext` member of ::ze_device_module_properties_t.
typedef struct _ze_device_raytracing_ext_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_device_raytracing_ext_flags_t flags;         ///< [out] 0 or a valid combination of ::ze_device_raytracing_ext_flags_t
    uint32_t maxBVHLevels;                          ///< [out] Maximum number of BVH levels supported

} ze_device_raytracing_ext_properties_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported raytracing memory allocation flags
typedef uint32_t ze_raytracing_mem_alloc_ext_flags_t;
typedef enum _ze_raytracing_mem_alloc_ext_flag_t
{
    ZE_RAYTRACING_MEM_ALLOC_EXT_FLAG_TBD = ZE_BIT(0),   ///< reserved for future use
    ZE_RAYTRACING_MEM_ALLOC_EXT_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_raytracing_mem_alloc_ext_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Raytracing memory allocation descriptor
/// 
/// @details
///     - This structure must be passed to ::zeMemAllocShared or
///       ::zeMemAllocDevice, via `pNext` member of
///       ::ze_device_mem_alloc_desc_t, for any memory allocation that is to be
///       accessed by raytracing fixed-function of the device.
typedef struct _ze_raytracing_mem_alloc_ext_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_raytracing_mem_alloc_ext_flags_t flags;      ///< [in] flags specifying additional allocation controls.
                                                    ///< must be 0 (default) or a valid combination of ::ze_raytracing_mem_alloc_ext_flag_t;
                                                    ///< default behavior may use implicit driver-based heuristics.

} ze_raytracing_mem_alloc_ext_desc_t;

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Memory Residency
#if !defined(__GNUC__)
#pragma region residency
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Makes memory resident for the device.
/// 
/// @details
///     - The application must ensure the memory is resident before being
///       referenced by the device
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeContextMakeMemoryResident(
    ze_context_handle_t hContext,                   ///< [in] handle of context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    void* ptr,                                      ///< [in] pointer to memory to make resident
    size_t size                                     ///< [in] size in bytes to make resident
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Allows memory to be evicted from the device.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the memory before it is evicted
///     - The application may free the memory without evicting; the memory is
///       implicitly evicted when freed.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeContextEvictMemory(
    ze_context_handle_t hContext,                   ///< [in] handle of context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    void* ptr,                                      ///< [in] pointer to memory to evict
    size_t size                                     ///< [in] size in bytes to evict
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Makes image resident for the device.
/// 
/// @details
///     - The application must ensure the image is resident before being
///       referenced by the device
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///         + `nullptr == hImage`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeContextMakeImageResident(
    ze_context_handle_t hContext,                   ///< [in] handle of context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    ze_image_handle_t hImage                        ///< [in] handle of image to make resident
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Allows image to be evicted from the device.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the image before it is evicted
///     - The application may destroy the image without evicting; the image is
///       implicitly evicted when destroyed.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///         + `nullptr == hImage`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeContextEvictImage(
    ze_context_handle_t hContext,                   ///< [in] handle of context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    ze_image_handle_t hImage                        ///< [in] handle of image to make evict
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Sampler
#if !defined(__GNUC__)
#pragma region sampler
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Sampler addressing modes
typedef enum _ze_sampler_address_mode_t
{
    ZE_SAMPLER_ADDRESS_MODE_NONE = 0,               ///< No coordinate modifications for out-of-bounds image access.
    ZE_SAMPLER_ADDRESS_MODE_REPEAT = 1,             ///< Out-of-bounds coordinates are wrapped back around.
    ZE_SAMPLER_ADDRESS_MODE_CLAMP = 2,              ///< Out-of-bounds coordinates are clamped to edge.
    ZE_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,    ///< Out-of-bounds coordinates are clamped to border color which is (0.0f,
                                                    ///< 0.0f, 0.0f, 0.0f) if image format swizzle contains alpha, otherwise
                                                    ///< (0.0f, 0.0f, 0.0f, 1.0f).
    ZE_SAMPLER_ADDRESS_MODE_MIRROR = 4,             ///< Out-of-bounds coordinates are mirrored starting from edge.
    ZE_SAMPLER_ADDRESS_MODE_FORCE_UINT32 = 0x7fffffff

} ze_sampler_address_mode_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Sampler filtering modes
typedef enum _ze_sampler_filter_mode_t
{
    ZE_SAMPLER_FILTER_MODE_NEAREST = 0,             ///< No coordinate modifications for out of bounds image access.
    ZE_SAMPLER_FILTER_MODE_LINEAR = 1,              ///< Out-of-bounds coordinates are wrapped back around.
    ZE_SAMPLER_FILTER_MODE_FORCE_UINT32 = 0x7fffffff

} ze_sampler_filter_mode_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Sampler descriptor
typedef struct _ze_sampler_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_sampler_address_mode_t addressMode;          ///< [in] Sampler addressing mode to determine how out-of-bounds
                                                    ///< coordinates are handled.
    ze_sampler_filter_mode_t filterMode;            ///< [in] Sampler filter mode to determine how samples are filtered.
    ze_bool_t isNormalized;                         ///< [in] Are coordinates normalized [0, 1] or not.

} ze_sampler_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates sampler on the context.
/// 
/// @details
///     - The application must only use the sampler for the device, or its
///       sub-devices, which was provided during creation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phSampler`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `::ZE_SAMPLER_ADDRESS_MODE_MIRROR < desc->addressMode`
///         + `::ZE_SAMPLER_FILTER_MODE_LINEAR < desc->filterMode`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeSamplerCreate(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device
    const ze_sampler_desc_t* desc,                  ///< [in] pointer to sampler descriptor
    ze_sampler_handle_t* phSampler                  ///< [out] handle of the sampler
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Destroys sampler object
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the sampler before it is deleted.
///     - The implementation of this function may immediately free all Host and
///       Device allocations associated with this sampler.
///     - The application must **not** call this function from simultaneous
///       threads with the same sampler handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hSampler`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zeSamplerDestroy(
    ze_sampler_handle_t hSampler                    ///< [in][release] handle of the sampler
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero APIs for Virtual Memory Management
#if !defined(__GNUC__)
#pragma region virtual
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Virtual memory page access attributes
typedef enum _ze_memory_access_attribute_t
{
    ZE_MEMORY_ACCESS_ATTRIBUTE_NONE = 0,            ///< Indicates the memory page is inaccessible.
    ZE_MEMORY_ACCESS_ATTRIBUTE_READWRITE = 1,       ///< Indicates the memory page supports read write access.
    ZE_MEMORY_ACCESS_ATTRIBUTE_READONLY = 2,        ///< Indicates the memory page supports read-only access.
    ZE_MEMORY_ACCESS_ATTRIBUTE_FORCE_UINT32 = 0x7fffffff

} ze_memory_access_attribute_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Reserves pages in virtual address space.
/// 
/// @details
///     - The application must only use the memory allocation on the context for
///       which it was created.
///     - The starting address and size must be page aligned. See
///       ::zeVirtualMemQueryPageSize.
///     - If pStart is not null then implementation will attempt to reserve
///       starting from that address. If not available then will find another
///       suitable starting address.
///     - The application may call this function from simultaneous threads.
///     - The access attributes will default to none to indicate reservation is
///       inaccessible.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pStart`
///         + `nullptr == pptr`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
ZE_APIEXPORT ze_result_t ZE_APICALL
zeVirtualMemReserve(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* pStart,                             ///< [in] pointer to start of region to reserve. If nullptr then
                                                    ///< implementation will choose a start address.
    size_t size,                                    ///< [in] size in bytes to reserve; must be page aligned.
    void** pptr                                     ///< [out] pointer to virtual reservation.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Free pages in a reserved virtual address range.
/// 
/// @details
///     - Any existing virtual mappings for the range will be unmapped.
///     - Physical allocations objects that were mapped to this range will not
///       be destroyed. These need to be destroyed explicitly.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT
ZE_APIEXPORT ze_result_t ZE_APICALL
zeVirtualMemFree(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* ptr,                                ///< [in] pointer to start of region to free.
    size_t size                                     ///< [in] size in bytes to free; must be page aligned.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Queries page size to use for aligning virtual memory reservations and
///        physical memory allocations.
/// 
/// @details
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == pagesize`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeVirtualMemQueryPageSize(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device object
    size_t size,                                    ///< [in] unaligned allocation size in bytes
    size_t* pagesize                                ///< [out] pointer to page size to use for start address and size
                                                    ///< alignments.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported physical memory creation flags
typedef uint32_t ze_physical_mem_flags_t;
typedef enum _ze_physical_mem_flag_t
{
    ZE_PHYSICAL_MEM_FLAG_TBD = ZE_BIT(0),           ///< reserved for future use.
    ZE_PHYSICAL_MEM_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_physical_mem_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Physical memory descriptor
typedef struct _ze_physical_mem_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_physical_mem_flags_t flags;                  ///< [in] creation flags.
                                                    ///< must be 0 (default) or a valid combination of ::ze_physical_mem_flag_t.
    size_t size;                                    ///< [in] size in bytes to reserve; must be page aligned.

} ze_physical_mem_desc_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Creates a physical memory object for the context.
/// 
/// @details
///     - The application must only use the physical memory object on the
///       context for which it was created.
///     - The size must be page aligned. See ::zeVirtualMemQueryPageSize.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hDevice`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == desc`
///         + `nullptr == phPhysicalMemory`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `0x1 < desc->flags`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == desc->size`
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT
ZE_APIEXPORT ze_result_t ZE_APICALL
zePhysicalMemCreate(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_device_handle_t hDevice,                     ///< [in] handle of the device object
    ze_physical_mem_desc_t* desc,                   ///< [in] pointer to physical memory descriptor.
    ze_physical_mem_handle_t* phPhysicalMemory      ///< [out] pointer to handle of physical memory object created
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Destroys a physical memory object.
/// 
/// @details
///     - The application must ensure the device is not currently referencing
///       the physical memory object before it is deleted
///     - The application must **not** call this function from simultaneous
///       threads with the same physical memory handle.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hPhysicalMemory`
///     - ::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE
ZE_APIEXPORT ze_result_t ZE_APICALL
zePhysicalMemDestroy(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    ze_physical_mem_handle_t hPhysicalMemory        ///< [in][release] handle of physical memory object to destroy
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Maps pages in virtual address space to pages from physical memory
///        object.
/// 
/// @details
///     - The virtual address range must have been reserved using
///       ::zeVirtualMemReserve.
///     - The application must only use the mapped memory allocation on the
///       context for which it was created.
///     - The virtual start address and size must be page aligned. See
///       ::zeVirtualMemQueryPageSize.
///     - The application should use, for the starting address and size, the
///       same size alignment used for the physical allocation.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///         + `nullptr == hPhysicalMemory`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `::ZE_MEMORY_ACCESS_ATTRIBUTE_READONLY < access`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT
ZE_APIEXPORT ze_result_t ZE_APICALL
zeVirtualMemMap(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* ptr,                                ///< [in] pointer to start of virtual address range to map.
    size_t size,                                    ///< [in] size in bytes of virtual address range to map; must be page
                                                    ///< aligned.
    ze_physical_mem_handle_t hPhysicalMemory,       ///< [in] handle to physical memory object.
    size_t offset,                                  ///< [in] offset into physical memory allocation object; must be page
                                                    ///< aligned.
    ze_memory_access_attribute_t access             ///< [in] specifies page access attributes to apply to the virtual address
                                                    ///< range.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Unmaps pages in virtual address space from pages from a physical
///        memory object.
/// 
/// @details
///     - The page access attributes for virtual address range will revert back
///       to none.
///     - The application may call this function from simultaneous threads.
///     - The implementation of this function must be thread-safe.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///     - ::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY
///     - ::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT - "Address must be page aligned"
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
///         + Size must be page aligned
ZE_APIEXPORT ze_result_t ZE_APICALL
zeVirtualMemUnmap(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* ptr,                                ///< [in] pointer to start of region to unmap.
    size_t size                                     ///< [in] size in bytes to unmap; must be page aligned.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Set memory access attributes for a virtual address range.
/// 
/// @details
///     - This function may be called from simultaneous threads with the same
///       function handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///     - ::ZE_RESULT_ERROR_INVALID_ENUMERATION
///         + `::ZE_MEMORY_ACCESS_ATTRIBUTE_READONLY < access`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT - "Address must be page aligned"
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
///         + Size must be page aligned
ZE_APIEXPORT ze_result_t ZE_APICALL
zeVirtualMemSetAccessAttribute(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* ptr,                                ///< [in] pointer to start of reserved virtual address region.
    size_t size,                                    ///< [in] size in bytes; must be page aligned.
    ze_memory_access_attribute_t access             ///< [in] specifies page access attributes to apply to the virtual address
                                                    ///< range.
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Get memory access attribute for a virtual address range.
/// 
/// @details
///     - If size and outSize are equal then the pages in the specified virtual
///       address range have the same access attributes.
///     - This function may be called from simultaneous threads with the same
///       function handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hContext`
///     - ::ZE_RESULT_ERROR_INVALID_NULL_POINTER
///         + `nullptr == ptr`
///         + `nullptr == access`
///         + `nullptr == outSize`
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT - "Address must be page aligned"
///     - ::ZE_RESULT_ERROR_UNSUPPORTED_SIZE
///         + `0 == size`
///         + Size must be page aligned
ZE_APIEXPORT ze_result_t ZE_APICALL
zeVirtualMemGetAccessAttribute(
    ze_context_handle_t hContext,                   ///< [in] handle of the context object
    const void* ptr,                                ///< [in] pointer to start of virtual address region for query.
    size_t size,                                    ///< [in] size in bytes; must be page aligned.
    ze_memory_access_attribute_t* access,           ///< [out] query result for page access attribute.
    size_t* outSize                                 ///< [out] query result for size of virtual address range, starting at ptr,
                                                    ///< that shares same access attribute.
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero Extension APIs for Floating-Point Atomics
#if !defined(__GNUC__)
#pragma region floatAtomics
#endif
///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_FLOAT_ATOMICS_EXT_NAME
/// @brief Floating-Point Atomics Extension Name
#define ZE_FLOAT_ATOMICS_EXT_NAME  "ZE_extension_float_atomics"
#endif // ZE_FLOAT_ATOMICS_EXT_NAME

///////////////////////////////////////////////////////////////////////////////
/// @brief Floating-Point Atomics Extension Version(s)
typedef enum _ze_float_atomics_ext_version_t
{
    ZE_FLOAT_ATOMICS_EXT_VERSION_1_0 = ZE_MAKE_VERSION( 1, 0 ), ///< version 1.0
    ZE_FLOAT_ATOMICS_EXT_VERSION_CURRENT = ZE_MAKE_VERSION( 1, 0 ), ///< latest known version
    ZE_FLOAT_ATOMICS_EXT_VERSION_FORCE_UINT32 = 0x7fffffff

} ze_float_atomics_ext_version_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported floating-point atomic capability flags
typedef uint32_t ze_device_fp_atomic_ext_flags_t;
typedef enum _ze_device_fp_atomic_ext_flag_t
{
    ZE_DEVICE_FP_ATOMIC_EXT_FLAG_GLOBAL_LOAD_STORE = ZE_BIT(0), ///< Supports atomic load, store, and exchange
    ZE_DEVICE_FP_ATOMIC_EXT_FLAG_GLOBAL_ADD = ZE_BIT(1),///< Supports atomic add and subtract
    ZE_DEVICE_FP_ATOMIC_EXT_FLAG_GLOBAL_MIN_MAX = ZE_BIT(2),///< Supports atomic min and max
    ZE_DEVICE_FP_ATOMIC_EXT_FLAG_LOCAL_LOAD_STORE = ZE_BIT(16), ///< Supports atomic load, store, and exchange
    ZE_DEVICE_FP_ATOMIC_EXT_FLAG_LOCAL_ADD = ZE_BIT(17),///< Supports atomic add and subtract
    ZE_DEVICE_FP_ATOMIC_EXT_FLAG_LOCAL_MIN_MAX = ZE_BIT(18),///< Supports atomic min and max
    ZE_DEVICE_FP_ATOMIC_EXT_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_device_fp_atomic_ext_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Device floating-point atomic properties queried using
///        ::zeDeviceGetModuleProperties
/// 
/// @details
///     - This structure may be returned from ::zeDeviceGetModuleProperties, via
///       `pNext` member of ::ze_device_module_properties_t.
typedef struct _ze_float_atomic_ext_properties_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    void* pNext;                                    ///< [in,out][optional] pointer to extension-specific structure
    ze_device_fp_atomic_ext_flags_t fp16Flags;      ///< [out] Capabilities for half-precision floating-point atomic operations
    ze_device_fp_atomic_ext_flags_t fp32Flags;      ///< [out] Capabilities for single-precision floating-point atomic
                                                    ///< operations
    ze_device_fp_atomic_ext_flags_t fp64Flags;      ///< [out] Capabilities for double-precision floating-point atomic
                                                    ///< operations

} ze_float_atomic_ext_properties_t;

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero Extension for supporting kernel global work offset.
#if !defined(__GNUC__)
#pragma region globaloffset
#endif
///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_GLOBAL_OFFSET_EXP_NAME
/// @brief Global Offset Extension Name
#define ZE_GLOBAL_OFFSET_EXP_NAME  "ZE_experimental_global_offset"
#endif // ZE_GLOBAL_OFFSET_EXP_NAME

///////////////////////////////////////////////////////////////////////////////
/// @brief Global Offset Extension Version(s)
typedef enum _ze_global_offset_exp_version_t
{
    ZE_GLOBAL_OFFSET_EXP_VERSION_1_0 = ZE_MAKE_VERSION( 1, 0 ), ///< version 1.0
    ZE_GLOBAL_OFFSET_EXP_VERSION_CURRENT = ZE_MAKE_VERSION( 1, 0 ), ///< latest known version
    ZE_GLOBAL_OFFSET_EXP_VERSION_FORCE_UINT32 = 0x7fffffff

} ze_global_offset_exp_version_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Set global work offset for a kernel on the current Host thread.
/// 
/// @details
///     - The global work offset will be used when
///       a ::zeCommandListAppendLaunchKernel() variant is called.
///     - The application must **not** call this function from simultaneous
///       threads with the same kernel handle.
///     - The implementation of this function should be lock-free.
/// 
/// @returns
///     - ::ZE_RESULT_SUCCESS
///     - ::ZE_RESULT_ERROR_UNINITIALIZED
///     - ::ZE_RESULT_ERROR_DEVICE_LOST
///     - ::ZE_RESULT_ERROR_INVALID_NULL_HANDLE
///         + `nullptr == hKernel`
ZE_APIEXPORT ze_result_t ZE_APICALL
zeKernelSetGlobalOffsetExp(
    ze_kernel_handle_t hKernel,                     ///< [in] handle of the kernel object
    uint32_t offsetX,                               ///< [in] global offset for X dimension to use for this kernel
    uint32_t offsetY,                               ///< [in] global offset for Y dimension to use for this kernel
    uint32_t offsetZ                                ///< [in] global offset for Z dimension to use for this kernel
    );

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero Extension for supporting relaxed allocation limits.
#if !defined(__GNUC__)
#pragma region relaxedAllocLimits
#endif
///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_RELAXED_ALLOCATION_LIMITS_EXP_NAME
/// @brief Relaxed Allocation Limits Extension Name
#define ZE_RELAXED_ALLOCATION_LIMITS_EXP_NAME  "ZE_experimental_relaxed_allocation_limits"
#endif // ZE_RELAXED_ALLOCATION_LIMITS_EXP_NAME

///////////////////////////////////////////////////////////////////////////////
/// @brief Relaxed Allocation Limits Extension Version(s)
typedef enum _ze_relaxed_allocation_limits_exp_version_t
{
    ZE_RELAXED_ALLOCATION_LIMITS_EXP_VERSION_1_0 = ZE_MAKE_VERSION( 1, 0 ), ///< version 1.0
    ZE_RELAXED_ALLOCATION_LIMITS_EXP_VERSION_CURRENT = ZE_MAKE_VERSION( 1, 0 ), ///< latest known version
    ZE_RELAXED_ALLOCATION_LIMITS_EXP_VERSION_FORCE_UINT32 = 0x7fffffff

} ze_relaxed_allocation_limits_exp_version_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Supported relaxed memory allocation flags
typedef uint32_t ze_relaxed_allocation_limits_exp_flags_t;
typedef enum _ze_relaxed_allocation_limits_exp_flag_t
{
    ZE_RELAXED_ALLOCATION_LIMITS_EXP_FLAG_MAX_SIZE = ZE_BIT(0), ///< Allocation size may exceed ::ze_device_properties_t.maxMemAllocSize
    ZE_RELAXED_ALLOCATION_LIMITS_EXP_FLAG_FORCE_UINT32 = 0x7fffffff

} ze_relaxed_allocation_limits_exp_flag_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Relaxed limits memory allocation descriptor
/// 
/// @details
///     - This structure may be passed to ::zeMemAllocShared or
///       ::zeMemAllocDevice, via `pNext` member of
///       ::ze_device_mem_alloc_desc_t.
///     - This structure may also be passed to ::zeMemAllocHost, via `pNext`
///       member of ::ze_host_mem_alloc_desc_t.
typedef struct _ze_relaxed_allocation_limits_exp_desc_t
{
    ze_structure_type_t stype;                      ///< [in] type of this structure
    const void* pNext;                              ///< [in][optional] pointer to extension-specific structure
    ze_relaxed_allocation_limits_exp_flags_t flags; ///< [in] flags specifying allocation limits to relax.
                                                    ///< must be 0 (default) or a valid combination of ::ze_relaxed_allocation_limits_exp_flag_t;

} ze_relaxed_allocation_limits_exp_desc_t;

#if !defined(__GNUC__)
#pragma endregion
#endif
// Intel 'oneAPI' Level-Zero API Callbacks
#if !defined(__GNUC__)
#pragma region callbacks
#endif
///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeInit 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_init_params_t
{
    ze_init_flags_t* pflags;
} ze_init_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeInit 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnInitCb_t)(
    ze_init_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Global callback functions pointers
typedef struct _ze_global_callbacks_t
{
    ze_pfnInitCb_t                                                  pfnInitCb;
} ze_global_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDriverGet 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_driver_get_params_t
{
    uint32_t** ppCount;
    ze_driver_handle_t** pphDrivers;
} ze_driver_get_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDriverGet 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDriverGetCb_t)(
    ze_driver_get_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDriverGetApiVersion 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_driver_get_api_version_params_t
{
    ze_driver_handle_t* phDriver;
    ze_api_version_t** pversion;
} ze_driver_get_api_version_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDriverGetApiVersion 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDriverGetApiVersionCb_t)(
    ze_driver_get_api_version_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDriverGetProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_driver_get_properties_params_t
{
    ze_driver_handle_t* phDriver;
    ze_driver_properties_t** ppDriverProperties;
} ze_driver_get_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDriverGetProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDriverGetPropertiesCb_t)(
    ze_driver_get_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDriverGetIpcProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_driver_get_ipc_properties_params_t
{
    ze_driver_handle_t* phDriver;
    ze_driver_ipc_properties_t** ppIpcProperties;
} ze_driver_get_ipc_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDriverGetIpcProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDriverGetIpcPropertiesCb_t)(
    ze_driver_get_ipc_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDriverGetExtensionProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_driver_get_extension_properties_params_t
{
    ze_driver_handle_t* phDriver;
    uint32_t** ppCount;
    ze_driver_extension_properties_t** ppExtensionProperties;
} ze_driver_get_extension_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDriverGetExtensionProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDriverGetExtensionPropertiesCb_t)(
    ze_driver_get_extension_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Driver callback functions pointers
typedef struct _ze_driver_callbacks_t
{
    ze_pfnDriverGetCb_t                                             pfnGetCb;
    ze_pfnDriverGetApiVersionCb_t                                   pfnGetApiVersionCb;
    ze_pfnDriverGetPropertiesCb_t                                   pfnGetPropertiesCb;
    ze_pfnDriverGetIpcPropertiesCb_t                                pfnGetIpcPropertiesCb;
    ze_pfnDriverGetExtensionPropertiesCb_t                          pfnGetExtensionPropertiesCb;
} ze_driver_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGet 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_params_t
{
    ze_driver_handle_t* phDriver;
    uint32_t** ppCount;
    ze_device_handle_t** pphDevices;
} ze_device_get_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGet 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetCb_t)(
    ze_device_get_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetSubDevices 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_sub_devices_params_t
{
    ze_device_handle_t* phDevice;
    uint32_t** ppCount;
    ze_device_handle_t** pphSubdevices;
} ze_device_get_sub_devices_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetSubDevices 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetSubDevicesCb_t)(
    ze_device_get_sub_devices_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_properties_params_t
{
    ze_device_handle_t* phDevice;
    ze_device_properties_t** ppDeviceProperties;
} ze_device_get_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetPropertiesCb_t)(
    ze_device_get_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetComputeProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_compute_properties_params_t
{
    ze_device_handle_t* phDevice;
    ze_device_compute_properties_t** ppComputeProperties;
} ze_device_get_compute_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetComputeProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetComputePropertiesCb_t)(
    ze_device_get_compute_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetModuleProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_module_properties_params_t
{
    ze_device_handle_t* phDevice;
    ze_device_module_properties_t** ppModuleProperties;
} ze_device_get_module_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetModuleProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetModulePropertiesCb_t)(
    ze_device_get_module_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetCommandQueueGroupProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_command_queue_group_properties_params_t
{
    ze_device_handle_t* phDevice;
    uint32_t** ppCount;
    ze_command_queue_group_properties_t** ppCommandQueueGroupProperties;
} ze_device_get_command_queue_group_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetCommandQueueGroupProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetCommandQueueGroupPropertiesCb_t)(
    ze_device_get_command_queue_group_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetMemoryProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_memory_properties_params_t
{
    ze_device_handle_t* phDevice;
    uint32_t** ppCount;
    ze_device_memory_properties_t** ppMemProperties;
} ze_device_get_memory_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetMemoryProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetMemoryPropertiesCb_t)(
    ze_device_get_memory_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetMemoryAccessProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_memory_access_properties_params_t
{
    ze_device_handle_t* phDevice;
    ze_device_memory_access_properties_t** ppMemAccessProperties;
} ze_device_get_memory_access_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetMemoryAccessProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetMemoryAccessPropertiesCb_t)(
    ze_device_get_memory_access_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetCacheProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_cache_properties_params_t
{
    ze_device_handle_t* phDevice;
    uint32_t** ppCount;
    ze_device_cache_properties_t** ppCacheProperties;
} ze_device_get_cache_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetCacheProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetCachePropertiesCb_t)(
    ze_device_get_cache_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetImageProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_image_properties_params_t
{
    ze_device_handle_t* phDevice;
    ze_device_image_properties_t** ppImageProperties;
} ze_device_get_image_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetImageProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetImagePropertiesCb_t)(
    ze_device_get_image_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetExternalMemoryProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_external_memory_properties_params_t
{
    ze_device_handle_t* phDevice;
    ze_device_external_memory_properties_t** ppExternalMemoryProperties;
} ze_device_get_external_memory_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetExternalMemoryProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetExternalMemoryPropertiesCb_t)(
    ze_device_get_external_memory_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetP2PProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_p2_p_properties_params_t
{
    ze_device_handle_t* phDevice;
    ze_device_handle_t* phPeerDevice;
    ze_device_p2p_properties_t** ppP2PProperties;
} ze_device_get_p2_p_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetP2PProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetP2PPropertiesCb_t)(
    ze_device_get_p2_p_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceCanAccessPeer 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_can_access_peer_params_t
{
    ze_device_handle_t* phDevice;
    ze_device_handle_t* phPeerDevice;
    ze_bool_t** pvalue;
} ze_device_can_access_peer_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceCanAccessPeer 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceCanAccessPeerCb_t)(
    ze_device_can_access_peer_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeDeviceGetStatus 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_device_get_status_params_t
{
    ze_device_handle_t* phDevice;
} ze_device_get_status_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeDeviceGetStatus 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnDeviceGetStatusCb_t)(
    ze_device_get_status_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Device callback functions pointers
typedef struct _ze_device_callbacks_t
{
    ze_pfnDeviceGetCb_t                                             pfnGetCb;
    ze_pfnDeviceGetSubDevicesCb_t                                   pfnGetSubDevicesCb;
    ze_pfnDeviceGetPropertiesCb_t                                   pfnGetPropertiesCb;
    ze_pfnDeviceGetComputePropertiesCb_t                            pfnGetComputePropertiesCb;
    ze_pfnDeviceGetModulePropertiesCb_t                             pfnGetModulePropertiesCb;
    ze_pfnDeviceGetCommandQueueGroupPropertiesCb_t                  pfnGetCommandQueueGroupPropertiesCb;
    ze_pfnDeviceGetMemoryPropertiesCb_t                             pfnGetMemoryPropertiesCb;
    ze_pfnDeviceGetMemoryAccessPropertiesCb_t                       pfnGetMemoryAccessPropertiesCb;
    ze_pfnDeviceGetCachePropertiesCb_t                              pfnGetCachePropertiesCb;
    ze_pfnDeviceGetImagePropertiesCb_t                              pfnGetImagePropertiesCb;
    ze_pfnDeviceGetExternalMemoryPropertiesCb_t                     pfnGetExternalMemoryPropertiesCb;
    ze_pfnDeviceGetP2PPropertiesCb_t                                pfnGetP2PPropertiesCb;
    ze_pfnDeviceCanAccessPeerCb_t                                   pfnCanAccessPeerCb;
    ze_pfnDeviceGetStatusCb_t                                       pfnGetStatusCb;
} ze_device_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeContextCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_context_create_params_t
{
    ze_driver_handle_t* phDriver;
    const ze_context_desc_t** pdesc;
    ze_context_handle_t** pphContext;
} ze_context_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeContextCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnContextCreateCb_t)(
    ze_context_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeContextDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_context_destroy_params_t
{
    ze_context_handle_t* phContext;
} ze_context_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeContextDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnContextDestroyCb_t)(
    ze_context_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeContextGetStatus 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_context_get_status_params_t
{
    ze_context_handle_t* phContext;
} ze_context_get_status_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeContextGetStatus 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnContextGetStatusCb_t)(
    ze_context_get_status_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeContextSystemBarrier 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_context_system_barrier_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
} ze_context_system_barrier_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeContextSystemBarrier 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnContextSystemBarrierCb_t)(
    ze_context_system_barrier_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeContextMakeMemoryResident 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_context_make_memory_resident_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    void** pptr;
    size_t* psize;
} ze_context_make_memory_resident_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeContextMakeMemoryResident 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnContextMakeMemoryResidentCb_t)(
    ze_context_make_memory_resident_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeContextEvictMemory 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_context_evict_memory_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    void** pptr;
    size_t* psize;
} ze_context_evict_memory_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeContextEvictMemory 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnContextEvictMemoryCb_t)(
    ze_context_evict_memory_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeContextMakeImageResident 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_context_make_image_resident_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    ze_image_handle_t* phImage;
} ze_context_make_image_resident_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeContextMakeImageResident 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnContextMakeImageResidentCb_t)(
    ze_context_make_image_resident_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeContextEvictImage 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_context_evict_image_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    ze_image_handle_t* phImage;
} ze_context_evict_image_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeContextEvictImage 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnContextEvictImageCb_t)(
    ze_context_evict_image_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Context callback functions pointers
typedef struct _ze_context_callbacks_t
{
    ze_pfnContextCreateCb_t                                         pfnCreateCb;
    ze_pfnContextDestroyCb_t                                        pfnDestroyCb;
    ze_pfnContextGetStatusCb_t                                      pfnGetStatusCb;
    ze_pfnContextSystemBarrierCb_t                                  pfnSystemBarrierCb;
    ze_pfnContextMakeMemoryResidentCb_t                             pfnMakeMemoryResidentCb;
    ze_pfnContextEvictMemoryCb_t                                    pfnEvictMemoryCb;
    ze_pfnContextMakeImageResidentCb_t                              pfnMakeImageResidentCb;
    ze_pfnContextEvictImageCb_t                                     pfnEvictImageCb;
} ze_context_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandQueueCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_queue_create_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    const ze_command_queue_desc_t** pdesc;
    ze_command_queue_handle_t** pphCommandQueue;
} ze_command_queue_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandQueueCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandQueueCreateCb_t)(
    ze_command_queue_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandQueueDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_queue_destroy_params_t
{
    ze_command_queue_handle_t* phCommandQueue;
} ze_command_queue_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandQueueDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandQueueDestroyCb_t)(
    ze_command_queue_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandQueueExecuteCommandLists 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_queue_execute_command_lists_params_t
{
    ze_command_queue_handle_t* phCommandQueue;
    uint32_t* pnumCommandLists;
    ze_command_list_handle_t** pphCommandLists;
    ze_fence_handle_t* phFence;
} ze_command_queue_execute_command_lists_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandQueueExecuteCommandLists 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandQueueExecuteCommandListsCb_t)(
    ze_command_queue_execute_command_lists_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandQueueSynchronize 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_queue_synchronize_params_t
{
    ze_command_queue_handle_t* phCommandQueue;
    uint64_t* ptimeout;
} ze_command_queue_synchronize_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandQueueSynchronize 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandQueueSynchronizeCb_t)(
    ze_command_queue_synchronize_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of CommandQueue callback functions pointers
typedef struct _ze_command_queue_callbacks_t
{
    ze_pfnCommandQueueCreateCb_t                                    pfnCreateCb;
    ze_pfnCommandQueueDestroyCb_t                                   pfnDestroyCb;
    ze_pfnCommandQueueExecuteCommandListsCb_t                       pfnExecuteCommandListsCb;
    ze_pfnCommandQueueSynchronizeCb_t                               pfnSynchronizeCb;
} ze_command_queue_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_create_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    const ze_command_list_desc_t** pdesc;
    ze_command_list_handle_t** pphCommandList;
} ze_command_list_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListCreateCb_t)(
    ze_command_list_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListCreateImmediate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_create_immediate_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    const ze_command_queue_desc_t** paltdesc;
    ze_command_list_handle_t** pphCommandList;
} ze_command_list_create_immediate_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListCreateImmediate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListCreateImmediateCb_t)(
    ze_command_list_create_immediate_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_destroy_params_t
{
    ze_command_list_handle_t* phCommandList;
} ze_command_list_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListDestroyCb_t)(
    ze_command_list_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListClose 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_close_params_t
{
    ze_command_list_handle_t* phCommandList;
} ze_command_list_close_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListClose 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListCloseCb_t)(
    ze_command_list_close_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListReset 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_reset_params_t
{
    ze_command_list_handle_t* phCommandList;
} ze_command_list_reset_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListReset 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListResetCb_t)(
    ze_command_list_reset_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendWriteGlobalTimestamp 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_write_global_timestamp_params_t
{
    ze_command_list_handle_t* phCommandList;
    uint64_t** pdstptr;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_write_global_timestamp_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendWriteGlobalTimestamp 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendWriteGlobalTimestampCb_t)(
    ze_command_list_append_write_global_timestamp_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendBarrier 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_barrier_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_barrier_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendBarrier 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendBarrierCb_t)(
    ze_command_list_append_barrier_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendMemoryRangesBarrier 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_memory_ranges_barrier_params_t
{
    ze_command_list_handle_t* phCommandList;
    uint32_t* pnumRanges;
    const size_t** ppRangeSizes;
    const void*** ppRanges;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_memory_ranges_barrier_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendMemoryRangesBarrier 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendMemoryRangesBarrierCb_t)(
    ze_command_list_append_memory_ranges_barrier_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendMemoryCopy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_memory_copy_params_t
{
    ze_command_list_handle_t* phCommandList;
    void** pdstptr;
    const void** psrcptr;
    size_t* psize;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_memory_copy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendMemoryCopy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendMemoryCopyCb_t)(
    ze_command_list_append_memory_copy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendMemoryFill 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_memory_fill_params_t
{
    ze_command_list_handle_t* phCommandList;
    void** pptr;
    const void** ppattern;
    size_t* ppattern_size;
    size_t* psize;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_memory_fill_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendMemoryFill 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendMemoryFillCb_t)(
    ze_command_list_append_memory_fill_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendMemoryCopyRegion 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_memory_copy_region_params_t
{
    ze_command_list_handle_t* phCommandList;
    void** pdstptr;
    const ze_copy_region_t** pdstRegion;
    uint32_t* pdstPitch;
    uint32_t* pdstSlicePitch;
    const void** psrcptr;
    const ze_copy_region_t** psrcRegion;
    uint32_t* psrcPitch;
    uint32_t* psrcSlicePitch;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_memory_copy_region_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendMemoryCopyRegion 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendMemoryCopyRegionCb_t)(
    ze_command_list_append_memory_copy_region_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendMemoryCopyFromContext 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_memory_copy_from_context_params_t
{
    ze_command_list_handle_t* phCommandList;
    void** pdstptr;
    ze_context_handle_t* phContextSrc;
    const void** psrcptr;
    size_t* psize;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_memory_copy_from_context_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendMemoryCopyFromContext 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendMemoryCopyFromContextCb_t)(
    ze_command_list_append_memory_copy_from_context_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendImageCopy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_image_copy_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_image_handle_t* phDstImage;
    ze_image_handle_t* phSrcImage;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_image_copy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendImageCopy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendImageCopyCb_t)(
    ze_command_list_append_image_copy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendImageCopyRegion 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_image_copy_region_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_image_handle_t* phDstImage;
    ze_image_handle_t* phSrcImage;
    const ze_image_region_t** ppDstRegion;
    const ze_image_region_t** ppSrcRegion;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_image_copy_region_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendImageCopyRegion 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendImageCopyRegionCb_t)(
    ze_command_list_append_image_copy_region_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendImageCopyToMemory 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_image_copy_to_memory_params_t
{
    ze_command_list_handle_t* phCommandList;
    void** pdstptr;
    ze_image_handle_t* phSrcImage;
    const ze_image_region_t** ppSrcRegion;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_image_copy_to_memory_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendImageCopyToMemory 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendImageCopyToMemoryCb_t)(
    ze_command_list_append_image_copy_to_memory_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendImageCopyFromMemory 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_image_copy_from_memory_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_image_handle_t* phDstImage;
    const void** psrcptr;
    const ze_image_region_t** ppDstRegion;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_image_copy_from_memory_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendImageCopyFromMemory 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendImageCopyFromMemoryCb_t)(
    ze_command_list_append_image_copy_from_memory_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendMemoryPrefetch 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_memory_prefetch_params_t
{
    ze_command_list_handle_t* phCommandList;
    const void** pptr;
    size_t* psize;
} ze_command_list_append_memory_prefetch_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendMemoryPrefetch 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendMemoryPrefetchCb_t)(
    ze_command_list_append_memory_prefetch_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendMemAdvise 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_mem_advise_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_device_handle_t* phDevice;
    const void** pptr;
    size_t* psize;
    ze_memory_advice_t* padvice;
} ze_command_list_append_mem_advise_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendMemAdvise 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendMemAdviseCb_t)(
    ze_command_list_append_mem_advise_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendSignalEvent 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_signal_event_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_event_handle_t* phEvent;
} ze_command_list_append_signal_event_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendSignalEvent 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendSignalEventCb_t)(
    ze_command_list_append_signal_event_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendWaitOnEvents 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_wait_on_events_params_t
{
    ze_command_list_handle_t* phCommandList;
    uint32_t* pnumEvents;
    ze_event_handle_t** pphEvents;
} ze_command_list_append_wait_on_events_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendWaitOnEvents 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendWaitOnEventsCb_t)(
    ze_command_list_append_wait_on_events_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendEventReset 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_event_reset_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_event_handle_t* phEvent;
} ze_command_list_append_event_reset_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendEventReset 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendEventResetCb_t)(
    ze_command_list_append_event_reset_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendQueryKernelTimestamps 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_query_kernel_timestamps_params_t
{
    ze_command_list_handle_t* phCommandList;
    uint32_t* pnumEvents;
    ze_event_handle_t** pphEvents;
    void** pdstptr;
    const size_t** ppOffsets;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_query_kernel_timestamps_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendQueryKernelTimestamps 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendQueryKernelTimestampsCb_t)(
    ze_command_list_append_query_kernel_timestamps_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendLaunchKernel 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_launch_kernel_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_kernel_handle_t* phKernel;
    const ze_group_count_t** ppLaunchFuncArgs;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_launch_kernel_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendLaunchKernel 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendLaunchKernelCb_t)(
    ze_command_list_append_launch_kernel_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendLaunchCooperativeKernel 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_launch_cooperative_kernel_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_kernel_handle_t* phKernel;
    const ze_group_count_t** ppLaunchFuncArgs;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_launch_cooperative_kernel_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendLaunchCooperativeKernel 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendLaunchCooperativeKernelCb_t)(
    ze_command_list_append_launch_cooperative_kernel_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendLaunchKernelIndirect 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_launch_kernel_indirect_params_t
{
    ze_command_list_handle_t* phCommandList;
    ze_kernel_handle_t* phKernel;
    const ze_group_count_t** ppLaunchArgumentsBuffer;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_launch_kernel_indirect_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendLaunchKernelIndirect 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendLaunchKernelIndirectCb_t)(
    ze_command_list_append_launch_kernel_indirect_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeCommandListAppendLaunchMultipleKernelsIndirect 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_command_list_append_launch_multiple_kernels_indirect_params_t
{
    ze_command_list_handle_t* phCommandList;
    uint32_t* pnumKernels;
    ze_kernel_handle_t** pphKernels;
    const uint32_t** ppCountBuffer;
    const ze_group_count_t** ppLaunchArgumentsBuffer;
    ze_event_handle_t* phSignalEvent;
    uint32_t* pnumWaitEvents;
    ze_event_handle_t** pphWaitEvents;
} ze_command_list_append_launch_multiple_kernels_indirect_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeCommandListAppendLaunchMultipleKernelsIndirect 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnCommandListAppendLaunchMultipleKernelsIndirectCb_t)(
    ze_command_list_append_launch_multiple_kernels_indirect_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of CommandList callback functions pointers
typedef struct _ze_command_list_callbacks_t
{
    ze_pfnCommandListCreateCb_t                                     pfnCreateCb;
    ze_pfnCommandListCreateImmediateCb_t                            pfnCreateImmediateCb;
    ze_pfnCommandListDestroyCb_t                                    pfnDestroyCb;
    ze_pfnCommandListCloseCb_t                                      pfnCloseCb;
    ze_pfnCommandListResetCb_t                                      pfnResetCb;
    ze_pfnCommandListAppendWriteGlobalTimestampCb_t                 pfnAppendWriteGlobalTimestampCb;
    ze_pfnCommandListAppendBarrierCb_t                              pfnAppendBarrierCb;
    ze_pfnCommandListAppendMemoryRangesBarrierCb_t                  pfnAppendMemoryRangesBarrierCb;
    ze_pfnCommandListAppendMemoryCopyCb_t                           pfnAppendMemoryCopyCb;
    ze_pfnCommandListAppendMemoryFillCb_t                           pfnAppendMemoryFillCb;
    ze_pfnCommandListAppendMemoryCopyRegionCb_t                     pfnAppendMemoryCopyRegionCb;
    ze_pfnCommandListAppendMemoryCopyFromContextCb_t                pfnAppendMemoryCopyFromContextCb;
    ze_pfnCommandListAppendImageCopyCb_t                            pfnAppendImageCopyCb;
    ze_pfnCommandListAppendImageCopyRegionCb_t                      pfnAppendImageCopyRegionCb;
    ze_pfnCommandListAppendImageCopyToMemoryCb_t                    pfnAppendImageCopyToMemoryCb;
    ze_pfnCommandListAppendImageCopyFromMemoryCb_t                  pfnAppendImageCopyFromMemoryCb;
    ze_pfnCommandListAppendMemoryPrefetchCb_t                       pfnAppendMemoryPrefetchCb;
    ze_pfnCommandListAppendMemAdviseCb_t                            pfnAppendMemAdviseCb;
    ze_pfnCommandListAppendSignalEventCb_t                          pfnAppendSignalEventCb;
    ze_pfnCommandListAppendWaitOnEventsCb_t                         pfnAppendWaitOnEventsCb;
    ze_pfnCommandListAppendEventResetCb_t                           pfnAppendEventResetCb;
    ze_pfnCommandListAppendQueryKernelTimestampsCb_t                pfnAppendQueryKernelTimestampsCb;
    ze_pfnCommandListAppendLaunchKernelCb_t                         pfnAppendLaunchKernelCb;
    ze_pfnCommandListAppendLaunchCooperativeKernelCb_t              pfnAppendLaunchCooperativeKernelCb;
    ze_pfnCommandListAppendLaunchKernelIndirectCb_t                 pfnAppendLaunchKernelIndirectCb;
    ze_pfnCommandListAppendLaunchMultipleKernelsIndirectCb_t        pfnAppendLaunchMultipleKernelsIndirectCb;
} ze_command_list_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeFenceCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_fence_create_params_t
{
    ze_command_queue_handle_t* phCommandQueue;
    const ze_fence_desc_t** pdesc;
    ze_fence_handle_t** pphFence;
} ze_fence_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeFenceCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnFenceCreateCb_t)(
    ze_fence_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeFenceDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_fence_destroy_params_t
{
    ze_fence_handle_t* phFence;
} ze_fence_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeFenceDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnFenceDestroyCb_t)(
    ze_fence_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeFenceHostSynchronize 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_fence_host_synchronize_params_t
{
    ze_fence_handle_t* phFence;
    uint64_t* ptimeout;
} ze_fence_host_synchronize_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeFenceHostSynchronize 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnFenceHostSynchronizeCb_t)(
    ze_fence_host_synchronize_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeFenceQueryStatus 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_fence_query_status_params_t
{
    ze_fence_handle_t* phFence;
} ze_fence_query_status_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeFenceQueryStatus 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnFenceQueryStatusCb_t)(
    ze_fence_query_status_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeFenceReset 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_fence_reset_params_t
{
    ze_fence_handle_t* phFence;
} ze_fence_reset_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeFenceReset 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnFenceResetCb_t)(
    ze_fence_reset_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Fence callback functions pointers
typedef struct _ze_fence_callbacks_t
{
    ze_pfnFenceCreateCb_t                                           pfnCreateCb;
    ze_pfnFenceDestroyCb_t                                          pfnDestroyCb;
    ze_pfnFenceHostSynchronizeCb_t                                  pfnHostSynchronizeCb;
    ze_pfnFenceQueryStatusCb_t                                      pfnQueryStatusCb;
    ze_pfnFenceResetCb_t                                            pfnResetCb;
} ze_fence_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventPoolCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_pool_create_params_t
{
    ze_context_handle_t* phContext;
    const ze_event_pool_desc_t** pdesc;
    uint32_t* pnumDevices;
    ze_device_handle_t** pphDevices;
    ze_event_pool_handle_t** pphEventPool;
} ze_event_pool_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventPoolCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventPoolCreateCb_t)(
    ze_event_pool_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventPoolDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_pool_destroy_params_t
{
    ze_event_pool_handle_t* phEventPool;
} ze_event_pool_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventPoolDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventPoolDestroyCb_t)(
    ze_event_pool_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventPoolGetIpcHandle 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_pool_get_ipc_handle_params_t
{
    ze_event_pool_handle_t* phEventPool;
    ze_ipc_event_pool_handle_t** pphIpc;
} ze_event_pool_get_ipc_handle_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventPoolGetIpcHandle 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventPoolGetIpcHandleCb_t)(
    ze_event_pool_get_ipc_handle_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventPoolOpenIpcHandle 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_pool_open_ipc_handle_params_t
{
    ze_context_handle_t* phContext;
    ze_ipc_event_pool_handle_t* phIpc;
    ze_event_pool_handle_t** pphEventPool;
} ze_event_pool_open_ipc_handle_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventPoolOpenIpcHandle 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventPoolOpenIpcHandleCb_t)(
    ze_event_pool_open_ipc_handle_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventPoolCloseIpcHandle 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_pool_close_ipc_handle_params_t
{
    ze_event_pool_handle_t* phEventPool;
} ze_event_pool_close_ipc_handle_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventPoolCloseIpcHandle 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventPoolCloseIpcHandleCb_t)(
    ze_event_pool_close_ipc_handle_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of EventPool callback functions pointers
typedef struct _ze_event_pool_callbacks_t
{
    ze_pfnEventPoolCreateCb_t                                       pfnCreateCb;
    ze_pfnEventPoolDestroyCb_t                                      pfnDestroyCb;
    ze_pfnEventPoolGetIpcHandleCb_t                                 pfnGetIpcHandleCb;
    ze_pfnEventPoolOpenIpcHandleCb_t                                pfnOpenIpcHandleCb;
    ze_pfnEventPoolCloseIpcHandleCb_t                               pfnCloseIpcHandleCb;
} ze_event_pool_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_create_params_t
{
    ze_event_pool_handle_t* phEventPool;
    const ze_event_desc_t** pdesc;
    ze_event_handle_t** pphEvent;
} ze_event_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventCreateCb_t)(
    ze_event_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_destroy_params_t
{
    ze_event_handle_t* phEvent;
} ze_event_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventDestroyCb_t)(
    ze_event_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventHostSignal 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_host_signal_params_t
{
    ze_event_handle_t* phEvent;
} ze_event_host_signal_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventHostSignal 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventHostSignalCb_t)(
    ze_event_host_signal_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventHostSynchronize 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_host_synchronize_params_t
{
    ze_event_handle_t* phEvent;
    uint64_t* ptimeout;
} ze_event_host_synchronize_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventHostSynchronize 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventHostSynchronizeCb_t)(
    ze_event_host_synchronize_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventQueryStatus 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_query_status_params_t
{
    ze_event_handle_t* phEvent;
} ze_event_query_status_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventQueryStatus 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventQueryStatusCb_t)(
    ze_event_query_status_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventHostReset 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_host_reset_params_t
{
    ze_event_handle_t* phEvent;
} ze_event_host_reset_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventHostReset 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventHostResetCb_t)(
    ze_event_host_reset_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeEventQueryKernelTimestamp 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_event_query_kernel_timestamp_params_t
{
    ze_event_handle_t* phEvent;
    ze_kernel_timestamp_result_t** pdstptr;
} ze_event_query_kernel_timestamp_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeEventQueryKernelTimestamp 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnEventQueryKernelTimestampCb_t)(
    ze_event_query_kernel_timestamp_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Event callback functions pointers
typedef struct _ze_event_callbacks_t
{
    ze_pfnEventCreateCb_t                                           pfnCreateCb;
    ze_pfnEventDestroyCb_t                                          pfnDestroyCb;
    ze_pfnEventHostSignalCb_t                                       pfnHostSignalCb;
    ze_pfnEventHostSynchronizeCb_t                                  pfnHostSynchronizeCb;
    ze_pfnEventQueryStatusCb_t                                      pfnQueryStatusCb;
    ze_pfnEventHostResetCb_t                                        pfnHostResetCb;
    ze_pfnEventQueryKernelTimestampCb_t                             pfnQueryKernelTimestampCb;
} ze_event_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeImageGetProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_image_get_properties_params_t
{
    ze_device_handle_t* phDevice;
    const ze_image_desc_t** pdesc;
    ze_image_properties_t** ppImageProperties;
} ze_image_get_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeImageGetProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnImageGetPropertiesCb_t)(
    ze_image_get_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeImageCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_image_create_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    const ze_image_desc_t** pdesc;
    ze_image_handle_t** pphImage;
} ze_image_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeImageCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnImageCreateCb_t)(
    ze_image_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeImageDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_image_destroy_params_t
{
    ze_image_handle_t* phImage;
} ze_image_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeImageDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnImageDestroyCb_t)(
    ze_image_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Image callback functions pointers
typedef struct _ze_image_callbacks_t
{
    ze_pfnImageGetPropertiesCb_t                                    pfnGetPropertiesCb;
    ze_pfnImageCreateCb_t                                           pfnCreateCb;
    ze_pfnImageDestroyCb_t                                          pfnDestroyCb;
} ze_image_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_create_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    const ze_module_desc_t** pdesc;
    ze_module_handle_t** pphModule;
    ze_module_build_log_handle_t** pphBuildLog;
} ze_module_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleCreateCb_t)(
    ze_module_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_destroy_params_t
{
    ze_module_handle_t* phModule;
} ze_module_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleDestroyCb_t)(
    ze_module_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleDynamicLink 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_dynamic_link_params_t
{
    uint32_t* pnumModules;
    ze_module_handle_t** pphModules;
    ze_module_build_log_handle_t** pphLinkLog;
} ze_module_dynamic_link_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleDynamicLink 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleDynamicLinkCb_t)(
    ze_module_dynamic_link_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleGetNativeBinary 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_get_native_binary_params_t
{
    ze_module_handle_t* phModule;
    size_t** ppSize;
    uint8_t** ppModuleNativeBinary;
} ze_module_get_native_binary_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleGetNativeBinary 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleGetNativeBinaryCb_t)(
    ze_module_get_native_binary_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleGetGlobalPointer 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_get_global_pointer_params_t
{
    ze_module_handle_t* phModule;
    const char** ppGlobalName;
    size_t** ppSize;
    void*** ppptr;
} ze_module_get_global_pointer_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleGetGlobalPointer 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleGetGlobalPointerCb_t)(
    ze_module_get_global_pointer_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleGetKernelNames 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_get_kernel_names_params_t
{
    ze_module_handle_t* phModule;
    uint32_t** ppCount;
    const char*** ppNames;
} ze_module_get_kernel_names_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleGetKernelNames 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleGetKernelNamesCb_t)(
    ze_module_get_kernel_names_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleGetProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_get_properties_params_t
{
    ze_module_handle_t* phModule;
    ze_module_properties_t** ppModuleProperties;
} ze_module_get_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleGetProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleGetPropertiesCb_t)(
    ze_module_get_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleGetFunctionPointer 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_get_function_pointer_params_t
{
    ze_module_handle_t* phModule;
    const char** ppFunctionName;
    void*** ppfnFunction;
} ze_module_get_function_pointer_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleGetFunctionPointer 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleGetFunctionPointerCb_t)(
    ze_module_get_function_pointer_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Module callback functions pointers
typedef struct _ze_module_callbacks_t
{
    ze_pfnModuleCreateCb_t                                          pfnCreateCb;
    ze_pfnModuleDestroyCb_t                                         pfnDestroyCb;
    ze_pfnModuleDynamicLinkCb_t                                     pfnDynamicLinkCb;
    ze_pfnModuleGetNativeBinaryCb_t                                 pfnGetNativeBinaryCb;
    ze_pfnModuleGetGlobalPointerCb_t                                pfnGetGlobalPointerCb;
    ze_pfnModuleGetKernelNamesCb_t                                  pfnGetKernelNamesCb;
    ze_pfnModuleGetPropertiesCb_t                                   pfnGetPropertiesCb;
    ze_pfnModuleGetFunctionPointerCb_t                              pfnGetFunctionPointerCb;
} ze_module_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleBuildLogDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_build_log_destroy_params_t
{
    ze_module_build_log_handle_t* phModuleBuildLog;
} ze_module_build_log_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleBuildLogDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleBuildLogDestroyCb_t)(
    ze_module_build_log_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeModuleBuildLogGetString 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_module_build_log_get_string_params_t
{
    ze_module_build_log_handle_t* phModuleBuildLog;
    size_t** ppSize;
    char** ppBuildLog;
} ze_module_build_log_get_string_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeModuleBuildLogGetString 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnModuleBuildLogGetStringCb_t)(
    ze_module_build_log_get_string_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of ModuleBuildLog callback functions pointers
typedef struct _ze_module_build_log_callbacks_t
{
    ze_pfnModuleBuildLogDestroyCb_t                                 pfnDestroyCb;
    ze_pfnModuleBuildLogGetStringCb_t                               pfnGetStringCb;
} ze_module_build_log_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_create_params_t
{
    ze_module_handle_t* phModule;
    const ze_kernel_desc_t** pdesc;
    ze_kernel_handle_t** pphKernel;
} ze_kernel_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelCreateCb_t)(
    ze_kernel_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_destroy_params_t
{
    ze_kernel_handle_t* phKernel;
} ze_kernel_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelDestroyCb_t)(
    ze_kernel_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelSetCacheConfig 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_set_cache_config_params_t
{
    ze_kernel_handle_t* phKernel;
    ze_cache_config_flags_t* pflags;
} ze_kernel_set_cache_config_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelSetCacheConfig 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelSetCacheConfigCb_t)(
    ze_kernel_set_cache_config_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelSetGroupSize 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_set_group_size_params_t
{
    ze_kernel_handle_t* phKernel;
    uint32_t* pgroupSizeX;
    uint32_t* pgroupSizeY;
    uint32_t* pgroupSizeZ;
} ze_kernel_set_group_size_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelSetGroupSize 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelSetGroupSizeCb_t)(
    ze_kernel_set_group_size_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelSuggestGroupSize 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_suggest_group_size_params_t
{
    ze_kernel_handle_t* phKernel;
    uint32_t* pglobalSizeX;
    uint32_t* pglobalSizeY;
    uint32_t* pglobalSizeZ;
    uint32_t** pgroupSizeX;
    uint32_t** pgroupSizeY;
    uint32_t** pgroupSizeZ;
} ze_kernel_suggest_group_size_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelSuggestGroupSize 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelSuggestGroupSizeCb_t)(
    ze_kernel_suggest_group_size_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelSuggestMaxCooperativeGroupCount 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_suggest_max_cooperative_group_count_params_t
{
    ze_kernel_handle_t* phKernel;
    uint32_t** ptotalGroupCount;
} ze_kernel_suggest_max_cooperative_group_count_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelSuggestMaxCooperativeGroupCount 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelSuggestMaxCooperativeGroupCountCb_t)(
    ze_kernel_suggest_max_cooperative_group_count_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelSetArgumentValue 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_set_argument_value_params_t
{
    ze_kernel_handle_t* phKernel;
    uint32_t* pargIndex;
    size_t* pargSize;
    const void** ppArgValue;
} ze_kernel_set_argument_value_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelSetArgumentValue 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelSetArgumentValueCb_t)(
    ze_kernel_set_argument_value_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelSetIndirectAccess 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_set_indirect_access_params_t
{
    ze_kernel_handle_t* phKernel;
    ze_kernel_indirect_access_flags_t* pflags;
} ze_kernel_set_indirect_access_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelSetIndirectAccess 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelSetIndirectAccessCb_t)(
    ze_kernel_set_indirect_access_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelGetIndirectAccess 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_get_indirect_access_params_t
{
    ze_kernel_handle_t* phKernel;
    ze_kernel_indirect_access_flags_t** ppFlags;
} ze_kernel_get_indirect_access_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelGetIndirectAccess 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelGetIndirectAccessCb_t)(
    ze_kernel_get_indirect_access_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelGetSourceAttributes 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_get_source_attributes_params_t
{
    ze_kernel_handle_t* phKernel;
    uint32_t** ppSize;
    char*** ppString;
} ze_kernel_get_source_attributes_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelGetSourceAttributes 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelGetSourceAttributesCb_t)(
    ze_kernel_get_source_attributes_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelGetProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_get_properties_params_t
{
    ze_kernel_handle_t* phKernel;
    ze_kernel_properties_t** ppKernelProperties;
} ze_kernel_get_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelGetProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelGetPropertiesCb_t)(
    ze_kernel_get_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeKernelGetName 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_kernel_get_name_params_t
{
    ze_kernel_handle_t* phKernel;
    size_t** ppSize;
    char** ppName;
} ze_kernel_get_name_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeKernelGetName 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnKernelGetNameCb_t)(
    ze_kernel_get_name_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Kernel callback functions pointers
typedef struct _ze_kernel_callbacks_t
{
    ze_pfnKernelCreateCb_t                                          pfnCreateCb;
    ze_pfnKernelDestroyCb_t                                         pfnDestroyCb;
    ze_pfnKernelSetCacheConfigCb_t                                  pfnSetCacheConfigCb;
    ze_pfnKernelSetGroupSizeCb_t                                    pfnSetGroupSizeCb;
    ze_pfnKernelSuggestGroupSizeCb_t                                pfnSuggestGroupSizeCb;
    ze_pfnKernelSuggestMaxCooperativeGroupCountCb_t                 pfnSuggestMaxCooperativeGroupCountCb;
    ze_pfnKernelSetArgumentValueCb_t                                pfnSetArgumentValueCb;
    ze_pfnKernelSetIndirectAccessCb_t                               pfnSetIndirectAccessCb;
    ze_pfnKernelGetIndirectAccessCb_t                               pfnGetIndirectAccessCb;
    ze_pfnKernelGetSourceAttributesCb_t                             pfnGetSourceAttributesCb;
    ze_pfnKernelGetPropertiesCb_t                                   pfnGetPropertiesCb;
    ze_pfnKernelGetNameCb_t                                         pfnGetNameCb;
} ze_kernel_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeSamplerCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_sampler_create_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    const ze_sampler_desc_t** pdesc;
    ze_sampler_handle_t** pphSampler;
} ze_sampler_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeSamplerCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnSamplerCreateCb_t)(
    ze_sampler_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeSamplerDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_sampler_destroy_params_t
{
    ze_sampler_handle_t* phSampler;
} ze_sampler_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeSamplerDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnSamplerDestroyCb_t)(
    ze_sampler_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Sampler callback functions pointers
typedef struct _ze_sampler_callbacks_t
{
    ze_pfnSamplerCreateCb_t                                         pfnCreateCb;
    ze_pfnSamplerDestroyCb_t                                        pfnDestroyCb;
} ze_sampler_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zePhysicalMemCreate 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_physical_mem_create_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    ze_physical_mem_desc_t** pdesc;
    ze_physical_mem_handle_t** pphPhysicalMemory;
} ze_physical_mem_create_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zePhysicalMemCreate 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnPhysicalMemCreateCb_t)(
    ze_physical_mem_create_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zePhysicalMemDestroy 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_physical_mem_destroy_params_t
{
    ze_context_handle_t* phContext;
    ze_physical_mem_handle_t* phPhysicalMemory;
} ze_physical_mem_destroy_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zePhysicalMemDestroy 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnPhysicalMemDestroyCb_t)(
    ze_physical_mem_destroy_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of PhysicalMem callback functions pointers
typedef struct _ze_physical_mem_callbacks_t
{
    ze_pfnPhysicalMemCreateCb_t                                     pfnCreateCb;
    ze_pfnPhysicalMemDestroyCb_t                                    pfnDestroyCb;
} ze_physical_mem_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeMemAllocShared 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_mem_alloc_shared_params_t
{
    ze_context_handle_t* phContext;
    const ze_device_mem_alloc_desc_t** pdevice_desc;
    const ze_host_mem_alloc_desc_t** phost_desc;
    size_t* psize;
    size_t* palignment;
    ze_device_handle_t* phDevice;
    void*** ppptr;
} ze_mem_alloc_shared_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeMemAllocShared 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnMemAllocSharedCb_t)(
    ze_mem_alloc_shared_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeMemAllocDevice 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_mem_alloc_device_params_t
{
    ze_context_handle_t* phContext;
    const ze_device_mem_alloc_desc_t** pdevice_desc;
    size_t* psize;
    size_t* palignment;
    ze_device_handle_t* phDevice;
    void*** ppptr;
} ze_mem_alloc_device_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeMemAllocDevice 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnMemAllocDeviceCb_t)(
    ze_mem_alloc_device_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeMemAllocHost 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_mem_alloc_host_params_t
{
    ze_context_handle_t* phContext;
    const ze_host_mem_alloc_desc_t** phost_desc;
    size_t* psize;
    size_t* palignment;
    void*** ppptr;
} ze_mem_alloc_host_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeMemAllocHost 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnMemAllocHostCb_t)(
    ze_mem_alloc_host_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeMemFree 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_mem_free_params_t
{
    ze_context_handle_t* phContext;
    void** pptr;
} ze_mem_free_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeMemFree 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnMemFreeCb_t)(
    ze_mem_free_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeMemGetAllocProperties 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_mem_get_alloc_properties_params_t
{
    ze_context_handle_t* phContext;
    const void** pptr;
    ze_memory_allocation_properties_t** ppMemAllocProperties;
    ze_device_handle_t** pphDevice;
} ze_mem_get_alloc_properties_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeMemGetAllocProperties 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnMemGetAllocPropertiesCb_t)(
    ze_mem_get_alloc_properties_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeMemGetAddressRange 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_mem_get_address_range_params_t
{
    ze_context_handle_t* phContext;
    const void** pptr;
    void*** ppBase;
    size_t** ppSize;
} ze_mem_get_address_range_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeMemGetAddressRange 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnMemGetAddressRangeCb_t)(
    ze_mem_get_address_range_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeMemGetIpcHandle 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_mem_get_ipc_handle_params_t
{
    ze_context_handle_t* phContext;
    const void** pptr;
    ze_ipc_mem_handle_t** ppIpcHandle;
} ze_mem_get_ipc_handle_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeMemGetIpcHandle 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnMemGetIpcHandleCb_t)(
    ze_mem_get_ipc_handle_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeMemOpenIpcHandle 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_mem_open_ipc_handle_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    ze_ipc_mem_handle_t* phandle;
    ze_ipc_memory_flags_t* pflags;
    void*** ppptr;
} ze_mem_open_ipc_handle_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeMemOpenIpcHandle 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnMemOpenIpcHandleCb_t)(
    ze_mem_open_ipc_handle_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeMemCloseIpcHandle 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_mem_close_ipc_handle_params_t
{
    ze_context_handle_t* phContext;
    const void** pptr;
} ze_mem_close_ipc_handle_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeMemCloseIpcHandle 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnMemCloseIpcHandleCb_t)(
    ze_mem_close_ipc_handle_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of Mem callback functions pointers
typedef struct _ze_mem_callbacks_t
{
    ze_pfnMemAllocSharedCb_t                                        pfnAllocSharedCb;
    ze_pfnMemAllocDeviceCb_t                                        pfnAllocDeviceCb;
    ze_pfnMemAllocHostCb_t                                          pfnAllocHostCb;
    ze_pfnMemFreeCb_t                                               pfnFreeCb;
    ze_pfnMemGetAllocPropertiesCb_t                                 pfnGetAllocPropertiesCb;
    ze_pfnMemGetAddressRangeCb_t                                    pfnGetAddressRangeCb;
    ze_pfnMemGetIpcHandleCb_t                                       pfnGetIpcHandleCb;
    ze_pfnMemOpenIpcHandleCb_t                                      pfnOpenIpcHandleCb;
    ze_pfnMemCloseIpcHandleCb_t                                     pfnCloseIpcHandleCb;
} ze_mem_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeVirtualMemReserve 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_virtual_mem_reserve_params_t
{
    ze_context_handle_t* phContext;
    const void** ppStart;
    size_t* psize;
    void*** ppptr;
} ze_virtual_mem_reserve_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeVirtualMemReserve 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnVirtualMemReserveCb_t)(
    ze_virtual_mem_reserve_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeVirtualMemFree 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_virtual_mem_free_params_t
{
    ze_context_handle_t* phContext;
    const void** pptr;
    size_t* psize;
} ze_virtual_mem_free_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeVirtualMemFree 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnVirtualMemFreeCb_t)(
    ze_virtual_mem_free_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeVirtualMemQueryPageSize 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_virtual_mem_query_page_size_params_t
{
    ze_context_handle_t* phContext;
    ze_device_handle_t* phDevice;
    size_t* psize;
    size_t** ppagesize;
} ze_virtual_mem_query_page_size_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeVirtualMemQueryPageSize 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnVirtualMemQueryPageSizeCb_t)(
    ze_virtual_mem_query_page_size_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeVirtualMemMap 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_virtual_mem_map_params_t
{
    ze_context_handle_t* phContext;
    const void** pptr;
    size_t* psize;
    ze_physical_mem_handle_t* phPhysicalMemory;
    size_t* poffset;
    ze_memory_access_attribute_t* paccess;
} ze_virtual_mem_map_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeVirtualMemMap 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnVirtualMemMapCb_t)(
    ze_virtual_mem_map_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeVirtualMemUnmap 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_virtual_mem_unmap_params_t
{
    ze_context_handle_t* phContext;
    const void** pptr;
    size_t* psize;
} ze_virtual_mem_unmap_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeVirtualMemUnmap 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnVirtualMemUnmapCb_t)(
    ze_virtual_mem_unmap_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeVirtualMemSetAccessAttribute 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_virtual_mem_set_access_attribute_params_t
{
    ze_context_handle_t* phContext;
    const void** pptr;
    size_t* psize;
    ze_memory_access_attribute_t* paccess;
} ze_virtual_mem_set_access_attribute_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeVirtualMemSetAccessAttribute 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnVirtualMemSetAccessAttributeCb_t)(
    ze_virtual_mem_set_access_attribute_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function parameters for zeVirtualMemGetAccessAttribute 
/// @details Each entry is a pointer to the parameter passed to the function;
///     allowing the callback the ability to modify the parameter's value
typedef struct _ze_virtual_mem_get_access_attribute_params_t
{
    ze_context_handle_t* phContext;
    const void** pptr;
    size_t* psize;
    ze_memory_access_attribute_t** paccess;
    size_t** poutSize;
} ze_virtual_mem_get_access_attribute_params_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Callback function-pointer for zeVirtualMemGetAccessAttribute 
/// @param[in] params Parameters passed to this instance
/// @param[in] result Return value
/// @param[in] pTracerUserData Per-Tracer user data
/// @param[in,out] ppTracerInstanceUserData Per-Tracer, Per-Instance user data
typedef void (ZE_APICALL *ze_pfnVirtualMemGetAccessAttributeCb_t)(
    ze_virtual_mem_get_access_attribute_params_t* params,
    ze_result_t result,
    void* pTracerUserData,
    void** ppTracerInstanceUserData
    );

///////////////////////////////////////////////////////////////////////////////
/// @brief Table of VirtualMem callback functions pointers
typedef struct _ze_virtual_mem_callbacks_t
{
    ze_pfnVirtualMemReserveCb_t                                     pfnReserveCb;
    ze_pfnVirtualMemFreeCb_t                                        pfnFreeCb;
    ze_pfnVirtualMemQueryPageSizeCb_t                               pfnQueryPageSizeCb;
    ze_pfnVirtualMemMapCb_t                                         pfnMapCb;
    ze_pfnVirtualMemUnmapCb_t                                       pfnUnmapCb;
    ze_pfnVirtualMemSetAccessAttributeCb_t                          pfnSetAccessAttributeCb;
    ze_pfnVirtualMemGetAccessAttributeCb_t                          pfnGetAccessAttributeCb;
} ze_virtual_mem_callbacks_t;

///////////////////////////////////////////////////////////////////////////////
/// @brief Container for all callbacks
typedef struct _ze_callbacks_t
{
    ze_global_callbacks_t               Global;
    ze_driver_callbacks_t               Driver;
    ze_device_callbacks_t               Device;
    ze_context_callbacks_t              Context;
    ze_command_queue_callbacks_t        CommandQueue;
    ze_command_list_callbacks_t         CommandList;
    ze_fence_callbacks_t                Fence;
    ze_event_pool_callbacks_t           EventPool;
    ze_event_callbacks_t                Event;
    ze_image_callbacks_t                Image;
    ze_module_callbacks_t               Module;
    ze_module_build_log_callbacks_t     ModuleBuildLog;
    ze_kernel_callbacks_t               Kernel;
    ze_sampler_callbacks_t              Sampler;
    ze_physical_mem_callbacks_t         PhysicalMem;
    ze_mem_callbacks_t                  Mem;
    ze_virtual_mem_callbacks_t          VirtualMem;
} ze_callbacks_t;

#if !defined(__GNUC__)
#pragma endregion
#endif

#if defined(__cplusplus)
} // extern "C"
#endif

#endif // _ZE_API_H