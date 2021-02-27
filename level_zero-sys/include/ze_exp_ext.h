/*
 * Copyright (C) 2020 Intel Corporation
 *
 * SPDX-License-Identifier: MIT
 *
 */

#pragma once

#include <level_zero/ze_api.h>

///////////////////////////////////////////////////////////////////////////////
// Intel 'oneAPI' Level-Zero Extension for supporting module programs.
///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_MODULE_PROGRAM_EXP_NAME
/// @brief Module Program Extension Name
#define ZE_MODULE_PROGRAM_EXP_NAME "ZE_experimental_module_program"
#endif // ZE_MODULE_PROGRAM_EXP_NAME

///////////////////////////////////////////////////////////////////////////////
/// @brief Module Program Extension Version(s)
typedef enum _ze_module_program_exp_version_t {
    ZE_MODULE_PROGRAM_EXP_VERSION_1_0 = ZE_MAKE_VERSION(1, 0),     ///< version 1.0
    ZE_MODULE_PROGRAM_EXP_VERSION_CURRENT = ZE_MAKE_VERSION(1, 0), ///< latest known version
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
typedef struct _ze_module_program_exp_desc_t {
    ze_structure_type_t stype;                ///< [in] type of this structure
    const void *pNext;                        ///< [in][optional] pointer to extension-specific structure
    uint32_t count;                           ///< [in] Count of input modules
    const size_t *inputSizes;                 ///< [in][range(0, count)] sizes of each input IL module in pInputModules.
    const uint8_t **pInputModules;            ///< [in][range(0, count)] pointer to an array of IL (e.g. SPIR-V modules).
                                              ///< Valid only for SPIR-V input.
    const char **pBuildFlags;                 ///< [in][optional][range(0, count)] array of strings containing build
                                              ///< flags. See pBuildFlags in ::ze_module_desc_t.
    const ze_module_constants_t **pConstants; ///< [in][optional][range(0, count)] pointer to array of specialization
                                              ///< constant strings. Valid only for SPIR-V input. This must be set to
                                              ///< nullptr if no specialization constants are provided.

} ze_module_program_exp_desc_t;

///////////////////////////////////////////////////////////////////////////////
// Intel 'oneAPI' Level-Zero Extension for supporting global work offset
///////////////////////////////////////////////////////////////////////////////
#ifndef ZE_GLOBAL_OFFSET_EXP_NAME
/// @brief Global Offset Extension Name
#define ZE_GLOBAL_OFFSET_EXP_NAME "ZE_experimental_global_offset"
#endif // ZE_GLOBAL_OFFSET_EXP_NAME

///////////////////////////////////////////////////////////////////////////////
/// @brief Global work offset Extension Version(s)
typedef enum _ze_global_offset_exp_version_t {
    ZE_GLOBAL_OFFSET_EXP_VERSION_1_0 = ZE_MAKE_VERSION(1, 0),     ///< version 1.0
    ZE_GLOBAL_OFFSET_EXP_VERSION_CURRENT = ZE_MAKE_VERSION(1, 0), ///< latest known version
    ZE_GLOBAL_OFFSET_EXP_VERSION_FORCE_UINT32 = 0x7fffffff

} ze_global_offset_exp_version_t;