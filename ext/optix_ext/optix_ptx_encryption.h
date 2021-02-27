// Copyright (c) 2017, NVIDIA CORPORATION.
// TO THE MAXIMUM EXTENT PERMITTED BY APPLICABLE LAW, THIS SOFTWARE IS PROVIDED
// *AS IS* AND NVIDIA AND ITS SUPPLIERS DISCLAIM ALL WARRANTIES, EITHER EXPRESS
// OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS FOR A PARTICULAR PURPOSE.  IN NO EVENT SHALL NVIDIA OR ITS SUPPLIERS
// BE LIABLE FOR ANY SPECIAL, INCIDENTAL, INDIRECT, OR CONSEQUENTIAL DAMAGES
// WHATSOEVER (INCLUDING, WITHOUT LIMITATION, DAMAGES FOR LOSS OF BUSINESS PROFITS,
// BUSINESS INTERRUPTION, LOSS OF BUSINESS INFORMATION, OR ANY OTHER PECUNIARY LOSS)
// ARISING OUT OF THE USE OF OR INABILITY TO USE THIS SOFTWARE, EVEN IF NVIDIA HAS
// BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES

#pragma once

#ifdef _WIN32
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN 1
#endif
#include <windows.h>
#undef min
#undef max
#undef IGNORE
#else
#include <cstdlib>
#include <ctime>
#include <sys/time.h>
#endif

#include <cassert>
#include <cstdint>
#include <cstring>
#include <vector>

#ifndef OPTIX_PTX_ENCRYPTION_STANDALONE
#include <optixu/optixpp_namespace.h>
#endif  // OPTIX_PTX_ENCRYPTION_STANDALONE

namespace optix {

#ifndef OPTIX_PTX_ENCRYPTION_STANDALONE

class PtxEncryption
{
  public:
    PtxEncryption( RTcontext context, const char* publicVendorKey, const char* secretVendorKey );

    PtxEncryption( Context context, const char* publicVendorKey, const char* secretVendorKey );

    PtxEncryption( RTcontext context, const char* publicVendorKey, const char* secretVendorKey, const char* optixSalt, const char* vendorSalt );

    PtxEncryption( Context context, const char* publicVendorKey, const char* secretVendorKey, const char* optixSalt, const char* vendorSalt );

    ::std::string encrypt( const ::std::string& ptx ) const;

    static ::std::string getPrefix() { return "eptx0001"; }

  private:
    // Magic number used by the TEA algorithm.
    static const uint32_t MAGIC = 0x9e3779b9;
    // Uses TEA to encrypt 8 bytes v[0] and v[1] using the key k.
    static void encryptTeaBlock( uint32_t* v, const uint32_t k[4] );
    // Uses TEA to encrypt \p sz bytes at \p data using the key \p key.
    static void encryptTea( unsigned char* data, size_t sz, const uint32_t key[4] );
    // Encodes the given string to ensure that it does not contain any null bytes.
    static ::std::string encode( const ::std::vector<unsigned char>& input );

    // The session key used by #encrypt().
    ::std::vector<unsigned char> m_sessionKey;
};

#endif  // OPTIX_PTX_ENCRYPTION_STANDALONE

namespace detail {

// Generates a salt.
//
// \param[out] buffer       The buffer for the salt. Needs to be able to hold 32 bytes.
void generateSalt( unsigned char* buffer );

// Computes a SHA256 hash value.
//
// \param input           The input for which to compute the SHA256 hash value.
// \param input_length    The size of the input.
// \param[out] buffer     The buffer for the SHA256 hash value. Needs to be able to hold 32 bytes.
void sha256( const unsigned char* input, unsigned int input_length, unsigned char* buffer );

}  // end namespace detail

#ifndef OPTIX_PTX_ENCRYPTION_STANDALONE

// Magic key used to obfuscate the last incomplete block.
const unsigned char KEY[7] = {164, 195, 147, 255, 203, 161, 184};

inline PtxEncryption::PtxEncryption( RTcontext context, const char* publicVendorKey, const char* secretVendorKeyHex )
{
    // Get optix salt
    const size_t  optixSaltLength = 32;
    unsigned char optixSalt[optixSaltLength];
    RTresult      result = rtContextGetAttribute( context, RT_CONTEXT_ATTRIBUTE_OPTIX_SALT, 32, &optixSalt[0] );
    assert( result == RT_SUCCESS );
    (void)result;

    // Generate and set vendor salt
    const size_t  vendorSaltLength = 32;
    unsigned char vendorSalt[vendorSaltLength];
    detail::generateSalt( &vendorSalt[0] );
    result = rtContextSetAttribute( context, RT_CONTEXT_ATTRIBUTE_VENDOR_SALT, 32, &vendorSalt[0] );
    assert( result == RT_SUCCESS );

    // Set vendor public key
    const size_t publicVendorKeyLength = strlen( publicVendorKey );
    result = rtContextSetAttribute( context, RT_CONTEXT_ATTRIBUTE_PUBLIC_VENDOR_KEY, publicVendorKeyLength, &publicVendorKey[0] );
    assert( result == RT_SUCCESS );

    // Generate sessionKey = hash(optixSalt, secretVendorKeyHex, vendorSalt).
    const size_t                 secretVendorKeyHexLength = strlen( secretVendorKeyHex );
    const size_t                 sessionKeyInputLength = optixSaltLength + secretVendorKeyHexLength + vendorSaltLength;
    ::std::vector<unsigned char> sessionKeyInput( sessionKeyInputLength );
    memcpy( &sessionKeyInput[0], &optixSalt[0], optixSaltLength );
    memcpy( &sessionKeyInput[optixSaltLength], secretVendorKeyHex, secretVendorKeyHexLength );
    memcpy( &sessionKeyInput[optixSaltLength + secretVendorKeyHexLength], vendorSalt, vendorSaltLength );

    m_sessionKey.resize( 32 );
    detail::sha256( &sessionKeyInput[0], static_cast<unsigned int>( sessionKeyInputLength ), &m_sessionKey[0] );

    for( size_t i = 0; i < 16; ++i )
        m_sessionKey[i] += m_sessionKey[i + 16];
    m_sessionKey.resize( 16 );
}

inline PtxEncryption::PtxEncryption( Context context, const char* publicVendorKey, const char* secretVendorKeyHex )
    : PtxEncryption( context->get(), publicVendorKey, secretVendorKeyHex )
{
}

inline PtxEncryption::PtxEncryption( RTcontext   context,
                                     const char* publicVendorKey,
                                     const char* secretVendorKeyHex,
                                     const char* optixSalt,
                                     const char* vendorSalt )
{
    // Set optix salt
    const size_t optixSaltLength = strlen( optixSalt );
    RTresult     result = rtContextSetAttribute( context, RT_CONTEXT_ATTRIBUTE_OPTIX_SALT, optixSaltLength, optixSalt );
    assert( result == RT_SUCCESS );
    (void)result;

    // Set vendor salt
    const size_t vendorSaltLength = strlen( vendorSalt );
    result = rtContextSetAttribute( context, RT_CONTEXT_ATTRIBUTE_VENDOR_SALT, vendorSaltLength, vendorSalt );
    assert( result == RT_SUCCESS );

    // Set vendor public key
    const size_t publicVendorKeyLength = strlen( publicVendorKey );
    result = rtContextSetAttribute( context, RT_CONTEXT_ATTRIBUTE_PUBLIC_VENDOR_KEY, publicVendorKeyLength, &publicVendorKey[0] );
    assert( result == RT_SUCCESS );

    // Generate sessionKey = hash(optixSalt, secretVendorKeyHex, vendorSalt).
    const size_t                 secretVendorKeyHexLength = strlen( secretVendorKeyHex );
    const size_t                 sessionKeyInputLength = optixSaltLength + secretVendorKeyHexLength + vendorSaltLength;
    ::std::vector<unsigned char> sessionKeyInput( sessionKeyInputLength );
    memcpy( &sessionKeyInput[0], &optixSalt[0], optixSaltLength );
    memcpy( &sessionKeyInput[optixSaltLength], secretVendorKeyHex, secretVendorKeyHexLength );
    memcpy( &sessionKeyInput[optixSaltLength + secretVendorKeyHexLength], vendorSalt, vendorSaltLength );

    m_sessionKey.resize( 32 );
    detail::sha256( &sessionKeyInput[0], static_cast<unsigned int>( sessionKeyInputLength ), &m_sessionKey[0] );

    for( size_t i = 0; i < 16; ++i )
        m_sessionKey[i] += m_sessionKey[i + 16];
    m_sessionKey.resize( 16 );
}

inline PtxEncryption::PtxEncryption( Context     context,
                                     const char* publicVendorKey,
                                     const char* secretVendorKeyHex,
                                     const char* optixSalt,
                                     const char* vendorSalt )
    : PtxEncryption( context->get(), publicVendorKey, secretVendorKeyHex, optixSalt, vendorSalt )
{
}

inline ::std::string PtxEncryption::encrypt( const ::std::string& ptx ) const
{
    uint32_t teaKey[4];
    assert( m_sessionKey.size() == sizeof( teaKey ) );
    memcpy( &teaKey[0], &m_sessionKey[0], sizeof( teaKey ) );

    ::std::vector<unsigned char> result( ptx.size() );
    memcpy( &result[0], &ptx[0], ptx.size() );
    encryptTea( &result[0], result.size(), teaKey );

    return getPrefix() + encode( result );
}

inline ::std::string PtxEncryption::encode( const ::std::vector<unsigned char>& input )
{
    ::std::string output;

    // Replace '\0' by '\1\1' and '\1' by '\1\2'.
    for( size_t i = 0; i < input.size(); ++i )
    {
        char c = input[i];
        if( c == '\0' || c == '\1' )
        {
            output.push_back( '\1' );
            output.push_back( c + 1 );
        }
        else
            output.push_back( c );
    }

    return output;
}

inline void PtxEncryption::encryptTeaBlock( uint32_t* v, const uint32_t k[4] )
{
    uint32_t v0 = v[0];
    uint32_t v1 = v[1];
    uint32_t s0 = 0;

    for( uint32_t n = 0; n < 16; n++ )
    {
        s0 += MAGIC;
        v0 += ( ( v1 << 4 ) + k[0] ) ^ ( v1 + s0 ) ^ ( ( v1 >> 5 ) + k[1] );
        v1 += ( ( v0 << 4 ) + k[2] ) ^ ( v0 + s0 ) ^ ( ( v0 >> 5 ) + k[3] );
    }

    v[0] = v0;
    v[1] = v1;
}

inline void PtxEncryption::encryptTea( unsigned char* data, size_t sz, const uint32_t key[4] )
{
    // Encrypt 8 byte blocks with TEA
    const size_t n = sz / static_cast<size_t>( 8 );
    uint32_t*    v = reinterpret_cast<uint32_t*>( data );
    for( size_t i = 0; i < n; ++i )
        encryptTeaBlock( &v[2 * i], key );

    // Slightly obfuscate leftover bytes (at most 7) with simple xor.
    for( size_t i = 8 * n, k = 0; i < sz; ++i, ++k )
        data[i] = data[i] ^ KEY[k];
}

#endif  // OPTIX_PTX_ENCRYPTION_STANDALONE

namespace detail {

inline void generateSalt( unsigned char* buffer )
{
    if( buffer == 0 )
        return;

    uint64_t number = 0;
#ifdef _WIN32
    srand( GetTickCount() );
    LARGE_INTEGER tmp;
    QueryPerformanceCounter( &tmp );
    number += tmp.QuadPart + GetCurrentProcessId() + GetTickCount();
#else
    srand( static_cast<unsigned>( time( 0 ) ) );
    struct timeval tv;
    gettimeofday( &tv, 0 );
    number += static_cast<uint64_t>( tv.tv_sec + tv.tv_usec );
#endif

    unsigned char buf[sizeof( uint32_t ) + 3 * sizeof( uint64_t )] = {0};
    int           r                                                = rand();
    memcpy( buf, &r, sizeof( uint32_t ) );
    memcpy( buf + sizeof( uint32_t ), &number, sizeof( uint64_t ) );
    memcpy( buf + sizeof( uint32_t ) + sizeof( uint64_t ), &number, sizeof( uint64_t ) );
    number += static_cast<uint64_t>( rand() );
    memcpy( buf + sizeof( uint32_t ) + 2 * sizeof( uint64_t ), &number, sizeof( uint64_t ) );
    sha256( buf, static_cast<uint32_t>( sizeof( buf ) ), buffer );
}

namespace {

// Table of round constants.
// First 32 bits of the fractional parts of the cube roots of the first 64 primes 2..311
const uint32_t sha256_constants[] = {0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
                                     0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
                                     0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
                                     0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
                                     0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
                                     0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
                                     0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
                                     0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
                                     0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
                                     0xc67178f2};

// Reverses the byte order of the bytes of \p x
uint32_t flip32( uint32_t x )
{
    return ( ( ( (x)&0xff000000 ) >> 24 ) | ( ( (x)&0x00ff0000 ) >> 8 ) | ( ( (x)&0x0000ff00 ) << 8 )
             | ( ( (x)&0x000000ff ) << 24 ) );
}

// Rotates the bits of \p x to the right by \p y bits
uint32_t rightrotate( uint32_t x, uint32_t y )
{
    return ( ( x >> y ) | ( x << ( 32 - y ) ) );
}

}  // end namespace

inline void sha256( const unsigned char* input, unsigned int input_length, unsigned char* buffer )
{
    if( ( input_length <= 0 ) || ( input == 0 ) || ( buffer == 0 ) )
        return;

    // First 32 bits of the fractional parts of the square roots of the first 8 primes 2..19
    uint32_t state[] = {0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19};

    // k is the number of '0' bits >= 0 such that the resulting message length is 448 (mod 512)
    unsigned int r = ( input_length * 8 + 1 ) % 512;
    unsigned int k = r > 448 ? 960 - r : 448 - r;

    unsigned int pos = 0;
    for( unsigned int chunk = 0; k != 0; ++chunk )
    {
        uint32_t     W[64]   = {0};
        uint8_t*     ptr     = reinterpret_cast<uint8_t*>( W );
        unsigned int to_copy = input_length - pos;
        to_copy              = to_copy > 64 ? 64 : to_copy;
        if( to_copy > 0 )
        {
            memcpy( ptr, input + pos, to_copy );
            pos += to_copy;
        }

        // If we are at the end of input message
        if( pos == input_length )
        {
            // If we still have not padded and have space to add a 1, add it
            if( ( k > 0 ) && ( pos / 64 == chunk ) )
                ptr[pos % 64] |= static_cast<uint8_t>( 0x80 );
            // If we can pad and still have space to add the length, add it
            if( ( pos * 8 + 1 + k ) - ( chunk * 512 ) <= 448 )
            {
                uint64_t value = input_length * 8;
                ptr            = reinterpret_cast<uint8_t*>( &W[14] );
                ptr[0]         = static_cast<uint8_t>( ( value >> 56 ) & 0xff );
                ptr[1]         = static_cast<uint8_t>( ( value >> 48 ) & 0xff );
                ptr[2]         = static_cast<uint8_t>( ( value >> 40 ) & 0xff );
                ptr[3]         = static_cast<uint8_t>( ( value >> 32 ) & 0xff );
                ptr[4]         = static_cast<uint8_t>( ( value >> 24 ) & 0xff );
                ptr[5]         = static_cast<uint8_t>( ( value >> 16 ) & 0xff );
                ptr[6]         = static_cast<uint8_t>( ( value >> 8 ) & 0xff );
                ptr[7]         = static_cast<uint8_t>( value & 0xff );
                k              = 0;
            }
        }

        // Flip to big endian
        for( int i = 0; i < 16; ++i )
            W[i]   = flip32( W[i] );

        // Extend the sixteen 32-bit words into 64 32-bit words
        for( uint32_t i = 16; i < 64; ++i )
        {
            uint32_t s0 = rightrotate( W[i - 15], 7 ) ^ rightrotate( W[i - 15], 18 ) ^ ( W[i - 15] >> 3 );
            uint32_t s1 = rightrotate( W[i - 2], 17 ) ^ rightrotate( W[i - 2], 19 ) ^ ( W[i - 2] >> 10 );
            W[i]        = W[i - 16] + s0 + W[i - 7] + s1;
        }

        // Initialize hash value for this chunk
        uint32_t a = state[0];
        uint32_t b = state[1];
        uint32_t c = state[2];
        uint32_t d = state[3];
        uint32_t e = state[4];
        uint32_t f = state[5];
        uint32_t g = state[6];
        uint32_t h = state[7];

        for( uint32_t j = 0; j < 64; ++j )
        {
            uint32_t s0  = rightrotate( a, 2 ) ^ rightrotate( a, 13 ) ^ rightrotate( a, 22 );
            uint32_t maj = ( a & b ) ^ ( a & c ) ^ ( b & c );
            uint32_t t2  = s0 + maj;
            uint32_t s1  = rightrotate( e, 6 ) ^ rightrotate( e, 11 ) ^ rightrotate( e, 25 );
            uint32_t ch  = ( e & f ) ^ ( ( ~e ) & g );
            uint32_t t1  = h + s1 + ch + sha256_constants[j] + W[j];

            h = g;
            g = f;
            f = e;
            e = d + t1;
            d = c;
            c = b;
            b = a;
            a = t1 + t2;
        }

        // Add this chunk's hash value to result so far
        state[0] += a;
        state[1] += b;
        state[2] += c;
        state[3] += d;
        state[4] += e;
        state[5] += f;
        state[6] += g;
        state[7] += h;
    }

    // Flip to little endian
    for( int i   = 0; i < 8; ++i )
        state[i] = flip32( state[i] );

    memcpy( buffer, reinterpret_cast<char*>( state ), 32 );
}

}  // end namespace detail

}  // end namespace optix