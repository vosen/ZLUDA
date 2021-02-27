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

#ifndef __optix_optix_ext_ptx_encryption_h__
#define __optix_optix_ext_ptx_encryption_h__

#include <optix_types.h>

#ifdef __cplusplus
extern "C" {
#endif


// Returns the OptiX salt for PTX encryption. The buffer for the salt value must consist of 32 bytes.
//
// Instead of using this function directly, consider using the C++ interface optix::PtxEncryption. This utility class
// also takes care of computing the session key needed for encryption.
OptixResult optixExtPtxEncryptionGetOptixSalt( OptixDeviceContext context, void* optixSalt, size_t optixSaltSizeInBytes );

// Sets the OptiX salt for PTX encryption. The salt value must consist of 32 bytes.
//
// Instead of using this function directly, consider using the C++ interface optix::PtxEncryption. This utility class
// also takes care of computing the session key needed for encryption.
OptixResult optixExtPtxEncryptionSetOptixSalt( OptixDeviceContext context, const void* optixSalt, size_t optixSaltSizeInBytes );

// Sets the vendor salt for PTX encryption. The salt value must consist of 32 bytes.
//
// Instead of using this function directly, consider using the C++ interface optix::PtxEncryption. This utility class
// also takes care of computing the session key needed for encryption.
OptixResult optixExtPtxEncryptionSetVendorSalt( OptixDeviceContext context, const void* vendorSalt, size_t vendorSaltSizeInBytes );

// Sets the public vendor key for PTX encryption.
//
// Instead of using this function directly, consider using the C++ interface optix::PtxEncryption. This utility class
// also takes care of computing the session key needed for encryption.
OptixResult optixExtPtxEncryptionSetPublicVendorKey( OptixDeviceContext context, const void* publicVendorKey, size_t publicVendorKeySizeInBytes );


#ifdef OPTIX_OPTIONAL_FEATURE_OPTIX7_INTERNAL_DOCUMENTATION
// When changing the ABI version make sure you know exactly what you are doing. See
// apps/optix/exp/functionTable/functionTable.cpp for instructions. See
// https://confluence.nvidia.com/display/RAV/ABI+Versions+in+the+Wild for released ABI versions.
#endif  // OPTIX_OPTIONAL_FEATURE_OPTIX7_INTERNAL_DOCUMENTATION
#define OPTIX_EXT_PTX_ENCRYPTION_ABI_VERSION 2001

typedef struct OptixExtPtxEncryptionFunctionTable
{
    OptixResult ( *optixExtPtxEncryptionGetOptixSalt )( OptixDeviceContext context, void* optixSalt, size_t optixSaltSizeInBytes );

    OptixResult ( *optixExtPtxEncryptionSetOptixSalt )( OptixDeviceContext context, const void* optixSalt, size_t optixSaltSizeInBytes );

    OptixResult ( *optixExtPtxEncryptionSetVendorSalt )( OptixDeviceContext context, const void* vendorSalt, size_t vendorSaltSizeInBytes );

    OptixResult ( *optixExtPtxEncryptionSetPublicVendorKey )( OptixDeviceContext context,
                                                              const void*        publicVendorKey,
                                                              size_t             publicVendorKeySizeInBytes );

} OptixExtPtxEncryptionFunctionTable;


#ifdef __cplusplus
}
#endif

#endif /* __optix_optix_ext_ptx_encryption_h__ */