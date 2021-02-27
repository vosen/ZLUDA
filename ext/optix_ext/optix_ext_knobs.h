/*
 * Copyright (c) 2019, NVIDIA CORPORATION.  All rights reserved.
 *
 * NVIDIA Corporation and its licensors retain all intellectual property and proprietary
 * rights in and to this software, related documentation and any modifications thereto.
 * Any use, reproduction, disclosure or distribution of this software and related
 * documentation without an express license agreement from NVIDIA Corporation is strictly
 * prohibited.
 *
 * TO THE MAXIMUM EXTENT PERMITTED BY APPLICABLE LAW, THIS SOFTWARE IS PROVIDED *AS IS*
 * AND NVIDIA AND ITS SUPPLIERS DISCLAIM ALL WARRANTIES, EITHER EXPRESS OR IMPLIED,
 * INCLUDING, BUT NOT LIMITED TO, IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE.  IN NO EVENT SHALL NVIDIA OR ITS SUPPLIERS BE LIABLE FOR ANY
 * SPECIAL, INCIDENTAL, INDIRECT, OR CONSEQUENTIAL DAMAGES WHATSOEVER (INCLUDING, WITHOUT
 * LIMITATION, DAMAGES FOR LOSS OF BUSINESS PROFITS, BUSINESS INTERRUPTION, LOSS OF
 * BUSINESS INFORMATION, OR ANY OTHER PECUNIARY LOSS) ARISING OUT OF THE USE OF OR
 * INABILITY TO USE THIS SOFTWARE, EVEN IF NVIDIA HAS BEEN ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES
 */

#ifndef __optix_optix_ext_knobs_h__
#define __optix_optix_ext_knobs_h__

#include <optix_types.h>

#ifdef __cplusplus
extern "C" {
#endif


// Sets values of multiple knobs based on some configuration file.
//
// Comments start with two slashes and are stripped. Whitespace at the begin and end of each line is also stripped.
// Empty lines are ignored. The first non-whitespace to whitespace transition marks the end of the name of the knob,
// the following whitespace is also stripped. The remainder of the line is the value of the knob, unless it is quoted
// by double apostrophes, which are also stripped.
//
// This method can only be used before the first device context has been created.
OptixResult optixExtKnobsSetKnobsFromFile( const char* filename );

// Sets the value of a single knob.
//
// This method can only be used before the first device context has been created.
OptixResult optixExtKnobsSetKnob( const char* name, const char* value );


#ifdef OPTIX_OPTIONAL_FEATURE_OPTIX7_INTERNAL_DOCUMENTATION
// When changing the ABI version make sure you know exactly what you are doing. See
// apps/optix/exp/functionTable/functionTable.cpp for instructions. See
// https://confluence.nvidia.com/display/RAV/ABI+Versions+in+the+Wild for released ABI versions.
#endif  // OPTIX_OPTIONAL_FEATURE_OPTIX7_INTERNAL_DOCUMENTATION
#define OPTIX_EXT_KNOBS_ABI_VERSION 1001

typedef struct OptixExtKnobsFunctionTable
{
    OptixResult ( *optixExtKnobsSetKnobsFromFile )( const char* filename );

    OptixResult ( *optixExtKnobsSetKnob )( const char* name, const char* value );

} OptixExtKnobsFunctionTable;


#ifdef __cplusplus
}
#endif

#endif /* __optix_optix_ext_knobs_h__ */