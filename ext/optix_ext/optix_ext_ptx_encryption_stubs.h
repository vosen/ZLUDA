/*
 * Copyright (c) 2019, NVIDIA CORPORATION. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *  * Neither the name of NVIDIA CORPORATION nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef __optix_optix_ext_ptx_encryption_stubs_h__
#define __optix_optix_ext_ptx_encryption_stubs_h__

#include "optix_ext_ptx_encryption.h"

#ifdef _WIN32
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN 1
#endif
#include <windows.h>
#else
#include <dlfcn.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* The function table needs to be defined in exactly one translation unit. This can be
   achieved by including optix_ext_ptx_encryption_function_table_definition.h in that translation unit.
 */
extern OptixExtPtxEncryptionFunctionTable g_optixExtPtxEncryptionFunctionTable;

// Initializes the function table used by the stubs for the extension API for PTX encryption.
//
// The function requires a handle to the loaded OptiX library. This handle can be obtained by using
// optixInitWithHandle() instead of optixInit(), for example (error handling ommitted):
//
//     void* handle;
//     optixInitWithHandle( &handle );
//     optixExtPtxEncryptionInit( handle );
//
inline OptixResult optixExtPtxEncryptionInit( void* handle )
{
    if( !handle )
        return OPTIX_ERROR_INVALID_VALUE;

#ifdef _WIN32
    void* symbol = GetProcAddress( (HMODULE)handle, "optixQueryFunctionTable" );
    if( !symbol )
        return OPTIX_ERROR_ENTRY_SYMBOL_NOT_FOUND;
#else
    void* symbol = dlsym( handle, "optixQueryFunctionTable" );
    if( !symbol )
        return OPTIX_ERROR_ENTRY_SYMBOL_NOT_FOUND;
#endif

    OptixQueryFunctionTable_t* optixQueryFunctionTable = (OptixQueryFunctionTable_t*)symbol;

    return optixQueryFunctionTable( OPTIX_EXT_PTX_ENCRYPTION_ABI_VERSION, 0, 0, 0, &g_optixExtPtxEncryptionFunctionTable,
                                    sizeof( g_optixExtPtxEncryptionFunctionTable ) );
}

/* Stub functions that forward calls to the corresponding function pointer in the function table. */

inline OptixResult optixExtPtxEncryptionGetOptixSalt( OptixDeviceContext context, void* optixSalt, size_t optixSaltSizeInBytes )
{
    return g_optixExtPtxEncryptionFunctionTable.optixExtPtxEncryptionGetOptixSalt( context, optixSalt, optixSaltSizeInBytes );
}

inline OptixResult optixExtPtxEncryptionSetOptixSalt( OptixDeviceContext context, const void* optixSalt, size_t optixSaltSizeInBytes )
{
    return g_optixExtPtxEncryptionFunctionTable.optixExtPtxEncryptionSetOptixSalt( context, optixSalt, optixSaltSizeInBytes );
}

inline OptixResult optixExtPtxEncryptionSetVendorSalt( OptixDeviceContext context, const void* vendorSalt, size_t vendorSaltSizeInBytes )
{
    return g_optixExtPtxEncryptionFunctionTable.optixExtPtxEncryptionSetVendorSalt( context, vendorSalt, vendorSaltSizeInBytes );
}

inline OptixResult optixExtPtxEncryptionSetPublicVendorKey( OptixDeviceContext context, const void* publicVendorKey, size_t publicVendorKeySizeInBytes )
{
    return g_optixExtPtxEncryptionFunctionTable.optixExtPtxEncryptionSetPublicVendorKey( context, publicVendorKey,
                                                                                         publicVendorKeySizeInBytes );
}

#ifdef __cplusplus
}
#endif

#endif /* __optix_optix_ext_ptx_encryption_stubs_h__ */
