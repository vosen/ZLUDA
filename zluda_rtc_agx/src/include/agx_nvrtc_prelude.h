// The prelude zluda_rtc_agx's NVRTC includes before every program: what
// NVRTC provides without headers (the CUDA qualifiers and built-in
// variables, vector types, atomics, warp and math functions), on clang's CUDA
// headers and an implementation of the libdevice functions they call.
#pragma once
#define __CUDACC__ 1
#define __NVCC__ 1
#define __CUDACC_RTC__ 1
#define __CUDACC_VER_MAJOR__ 12
#define __CUDACC_VER_MINOR__ 5
#define CUDA_VERSION 12050
#define __host__ __attribute__((host))
#define __device__ __attribute__((device))
#define __global__ __attribute__((global))
#define __shared__ __attribute__((shared))
#define __constant__ __attribute__((constant))
#define __managed__ __attribute__((managed))
#define __launch_bounds__(...) __attribute__((launch_bounds(__VA_ARGS__)))
#define __forceinline__ __inline__ __attribute__((always_inline))
#define __noinline__ __attribute__((noinline))
#define __align__(n) __attribute__((aligned(n)))
#define __restrict__ __restrict
#define __grid_constant__
#define CUDART_INF_F __builtin_inff()
#define CUDART_NAN_F __builtin_nanf("")
#define CUDART_INF __builtin_inf()
#define CUDART_NAN __builtin_nan("")
#define CUDART_PI_F 3.141592654f
#define CUDART_PI 3.1415926535897931e+0
#define INFINITY __builtin_inff()
#define NAN __builtin_nanf("")
#define HUGE_VAL __builtin_huge_val()
#define HUGE_VALF __builtin_huge_valf()
#include <stddef.h>
#include <stdint.h>
#include <limits.h>
typedef uint32_t cuuint32_t;
typedef uint64_t cuuint64_t;
// ---- vector types

#define AGX_VEC(T, N, A)                                   \
  struct __attribute__((aligned(A))) N##1 { T x; };        \
  struct __attribute__((aligned(2 * A))) N##2 { T x, y; }; \
  struct N##3 { T x, y, z; };                              \
  struct __attribute__((aligned(4 * A))) N##4 { T x, y, z, w; }; \
  typedef struct N##1 N##1;                                \
  typedef struct N##2 N##2;                                \
  typedef struct N##3 N##3;                                \
  typedef struct N##4 N##4;                                \
  static __host__ __device__ __forceinline__ N##1 make_##N##1(T x) { return N##1{x}; } \
  static __host__ __device__ __forceinline__ N##2 make_##N##2(T x, T y) { return N##2{x, y}; } \
  static __host__ __device__ __forceinline__ N##3 make_##N##3(T x, T y, T z) { return N##3{x, y, z}; } \
  static __host__ __device__ __forceinline__ N##4 make_##N##4(T x, T y, T z, T w) { return N##4{x, y, z, w}; }
AGX_VEC(signed char, char, 1)
AGX_VEC(unsigned char, uchar, 1)
AGX_VEC(short, short, 2)
AGX_VEC(unsigned short, ushort, 2)
AGX_VEC(int, int, 4)
AGX_VEC(unsigned int, uint, 4)
AGX_VEC(long, long, 8)
AGX_VEC(unsigned long, ulong, 8)
AGX_VEC(long long, longlong, 8)
AGX_VEC(unsigned long long, ulonglong, 8)
AGX_VEC(float, float, 4)
AGX_VEC(double, double, 8)

#include <__clang_cuda_builtin_vars.h>
#include <__clang_cuda_libdevice_declares.h>
#include <__clang_cuda_device_functions.h>
#include <__clang_cuda_math.h>
#include <__clang_cuda_intrinsics.h>

#define AGX_NV extern "C" __device__ inline __attribute__((always_inline))
#include "agx_nvrtc_math.h"

// ---- libdevice functions on PTX instructions

#define AGX_ROUNDED(name, op, rnd, ty, b) \
  AGX_NV ty __nv_##name##_##rnd(ty x, ty y) { return __nvvm_##op##_##rnd##_##b(x, y); }
#define AGX_ROUNDED4(name, op, ty, b)  \
  AGX_ROUNDED(name, op, rn, ty, b)     \
  AGX_ROUNDED(name, op, rz, ty, b)     \
  AGX_NV ty __nv_##name##_ru(ty x, ty y) { return __nvvm_##op##_rp_##b(x, y); } \
  AGX_NV ty __nv_##name##_rd(ty x, ty y) { return __nvvm_##op##_rm_##b(x, y); }
AGX_ROUNDED4(fadd, add, float, f)
AGX_ROUNDED4(fmul, mul, float, f)
AGX_ROUNDED4(fdiv, div, float, f)
AGX_ROUNDED4(dadd, add, double, d)
AGX_ROUNDED4(dmul, mul, double, d)
AGX_ROUNDED4(ddiv, div, double, d)
AGX_NV float __nv_fsub_rn(float x, float y) { return __nvvm_add_rn_f(x, -y); }
AGX_NV float __nv_fsub_rz(float x, float y) { return __nvvm_add_rz_f(x, -y); }
AGX_NV float __nv_fsub_ru(float x, float y) { return __nvvm_add_rp_f(x, -y); }
AGX_NV float __nv_fsub_rd(float x, float y) { return __nvvm_add_rm_f(x, -y); }
AGX_NV double __nv_dsub_rn(double x, double y) { return __nvvm_add_rn_d(x, -y); }
AGX_NV double __nv_dsub_rz(double x, double y) { return __nvvm_add_rz_d(x, -y); }
AGX_NV double __nv_dsub_ru(double x, double y) { return __nvvm_add_rp_d(x, -y); }
AGX_NV double __nv_dsub_rd(double x, double y) { return __nvvm_add_rm_d(x, -y); }
#define AGX_UNARY4(name, op, ty, b)                                                       \
  AGX_NV ty __nv_##name##_rn(ty x) { return __nvvm_##op##_rn_##b(x); }                  \
  AGX_NV ty __nv_##name##_rz(ty x) { return __nvvm_##op##_rz_##b(x); }                  \
  AGX_NV ty __nv_##name##_ru(ty x) { return __nvvm_##op##_rp_##b(x); }                  \
  AGX_NV ty __nv_##name##_rd(ty x) { return __nvvm_##op##_rm_##b(x); }
AGX_UNARY4(fsqrt, sqrt, float, f)
AGX_UNARY4(frcp, rcp, float, f)
AGX_UNARY4(dsqrt, sqrt, double, d)
AGX_UNARY4(drcp, rcp, double, d)
AGX_NV float __nv_frsqrt_rn(float x) { return 1.0f / __nvvm_sqrt_rn_f(x); }
#define AGX_FMA4(name, ty, b)                                                                     \
  AGX_NV ty __nv_##name##_rn(ty x, ty y, ty z) { return __nvvm_fma_rn_##b(x, y, z); }           \
  AGX_NV ty __nv_##name##_rz(ty x, ty y, ty z) { return __nvvm_fma_rz_##b(x, y, z); }           \
  AGX_NV ty __nv_##name##_ru(ty x, ty y, ty z) { return __nvvm_fma_rp_##b(x, y, z); }           \
  AGX_NV ty __nv_##name##_rd(ty x, ty y, ty z) { return __nvvm_fma_rm_##b(x, y, z); }
AGX_FMA4(fmaf, float, f)
AGX_FMA4(fmaf_ieee, float, f)
AGX_FMA4(fma, double, d)

// the fast intrinsics: the hardware's approximations
AGX_NV float __nv_fast_sinf(float x) { return __nvvm_sin_approx_f(x); }
AGX_NV float __nv_fast_cosf(float x) { return __nvvm_cos_approx_f(x); }
AGX_NV void __nv_fast_sincosf(float x, float *s, float *c) {
  *s = __nvvm_sin_approx_f(x);
  *c = __nvvm_cos_approx_f(x);
}
AGX_NV float __nv_fast_tanf(float x) { return __nvvm_sin_approx_f(x) / __nvvm_cos_approx_f(x); }
AGX_NV float __nv_fast_expf(float x) { return __nvvm_ex2_approx_f(x * 1.44269504f); }
AGX_NV float __nv_fast_exp10f(float x) { return __nvvm_ex2_approx_f(x * 3.32192809f); }
AGX_NV float __nv_fast_logf(float x) { return __nvvm_lg2_approx_f(x) * 0.693147181f; }
AGX_NV float __nv_fast_log2f(float x) { return __nvvm_lg2_approx_f(x); }
AGX_NV float __nv_fast_log10f(float x) { return __nvvm_lg2_approx_f(x) * 0.301029996f; }
AGX_NV float __nv_fast_powf(float x, float y) { return __nvvm_ex2_approx_f(y * __nvvm_lg2_approx_f(x)); }
AGX_NV float __nv_fast_fdividef(float x, float y) { return __nvvm_div_approx_f(x, y); }

// ---- conversions

#define AGX_CVT4(name, op, from, to)                                                  \
  AGX_NV to __nv_##name##_rn(from x) { return __nvvm_##op##_rn(x); }                \
  AGX_NV to __nv_##name##_rz(from x) { return __nvvm_##op##_rz(x); }                \
  AGX_NV to __nv_##name##_ru(from x) { return __nvvm_##op##_rp(x); }                \
  AGX_NV to __nv_##name##_rd(from x) { return __nvvm_##op##_rm(x); }
AGX_CVT4(float2int, f2i, float, int)
AGX_CVT4(float2uint, f2ui, float, unsigned)
AGX_CVT4(float2ll, f2ll, float, long long)
AGX_CVT4(float2ull, f2ull, float, unsigned long long)
AGX_CVT4(double2int, d2i, double, int)
AGX_CVT4(double2uint, d2ui, double, unsigned)
AGX_CVT4(double2ll, d2ll, double, long long)
AGX_CVT4(double2ull, d2ull, double, unsigned long long)
AGX_CVT4(double2float, d2f, double, float)
AGX_CVT4(int2float, i2f, int, float)
AGX_CVT4(uint2float, ui2f, unsigned, float)
AGX_CVT4(ll2float, ll2f, long long, float)
AGX_CVT4(ull2float, ull2f, unsigned long long, float)
AGX_CVT4(ll2double, ll2d, long long, double)
AGX_CVT4(ull2double, ull2d, unsigned long long, double)
AGX_NV double __nv_int2double_rn(int x) { return (double)x; }
AGX_NV double __nv_uint2double_rn(unsigned x) { return (double)x; }
AGX_NV int __nv_float_as_int(float x) { return __builtin_bit_cast(int, x); }
AGX_NV unsigned __nv_float_as_uint(float x) { return __builtin_bit_cast(unsigned, x); }
AGX_NV float __nv_int_as_float(int x) { return __builtin_bit_cast(float, x); }
AGX_NV float __nv_uint_as_float(unsigned x) { return __builtin_bit_cast(float, x); }
AGX_NV unsigned long long __nv_double_as_longlong(double x) { return __builtin_bit_cast(unsigned long long, x); }
AGX_NV double __nv_longlong_as_double(long long x) { return __builtin_bit_cast(double, x); }
AGX_NV int __nv_double2hiint(double x) { return (int)(__builtin_bit_cast(unsigned long long, x) >> 32); }
AGX_NV int __nv_double2loint(double x) { return (int)__builtin_bit_cast(unsigned long long, x); }
AGX_NV double __nv_hiloint2double(int hi, int lo) {
  return __builtin_bit_cast(double, ((unsigned long long)(unsigned)hi << 32) | (unsigned)lo);
}

// ---- integer intrinsics

AGX_NV int __nv_abs(int x) { return x < 0 ? -x : x; }
AGX_NV long long __nv_llabs(long long x) { return x < 0 ? -x : x; }
AGX_NV int __nv_min(int x, int y) { return x < y ? x : y; }
AGX_NV int __nv_max(int x, int y) { return x > y ? x : y; }
AGX_NV unsigned __nv_umin(unsigned x, unsigned y) { return x < y ? x : y; }
AGX_NV unsigned __nv_umax(unsigned x, unsigned y) { return x > y ? x : y; }
AGX_NV long long __nv_llmin(long long x, long long y) { return x < y ? x : y; }
AGX_NV long long __nv_llmax(long long x, long long y) { return x > y ? x : y; }
AGX_NV unsigned long long __nv_ullmin(unsigned long long x, unsigned long long y) { return x < y ? x : y; }
AGX_NV unsigned long long __nv_ullmax(unsigned long long x, unsigned long long y) { return x > y ? x : y; }
AGX_NV int __nv_clz(int x) { return x == 0 ? 32 : __builtin_clz((unsigned)x); }
AGX_NV int __nv_clzll(long long x) { return x == 0 ? 64 : __builtin_clzll((unsigned long long)x); }
AGX_NV int __nv_popc(int x) { return __builtin_popcount((unsigned)x); }
AGX_NV int __nv_popcll(long long x) { return __builtin_popcountll((unsigned long long)x); }
AGX_NV unsigned __nv_brev(unsigned x) { return __builtin_bitreverse32(x); }
AGX_NV unsigned long long __nv_brevll(unsigned long long x) { return __builtin_bitreverse64(x); }
AGX_NV int __nv_ffs(int x) { return x == 0 ? 0 : __builtin_ctz((unsigned)x) + 1; }
AGX_NV int __nv_ffsll(long long x) { return x == 0 ? 0 : __builtin_ctzll((unsigned long long)x) + 1; }
AGX_NV int __nv_mulhi(int x, int y) { return (int)(((long long)x * y) >> 32); }
AGX_NV unsigned __nv_umulhi(unsigned x, unsigned y) { return (unsigned)(((unsigned long long)x * y) >> 32); }
AGX_NV long long __nv_mul64hi(long long x, long long y) { return (long long)(((__int128)x * y) >> 64); }
AGX_NV unsigned long long __nv_umul64hi(unsigned long long x, unsigned long long y) {
  return (unsigned long long)(((unsigned __int128)x * y) >> 64);
}
AGX_NV int __nv_mul24(int x, int y) { return ((x << 8) >> 8) * ((y << 8) >> 8); }
AGX_NV unsigned __nv_umul24(unsigned x, unsigned y) { return (x & 0xffffff) * (y & 0xffffff); }
AGX_NV int __nv_sad(int x, int y, int z) { return z + (x > y ? x - y : y - x); }
AGX_NV unsigned __nv_usad(unsigned x, unsigned y, unsigned z) { return z + (x > y ? x - y : y - x); }
AGX_NV int __nv_byte_perm(int x, int y, int s) { return (int)__nvvm_prmt(x, y, s); }
AGX_NV int __nv_hadd(int x, int y) { return (int)(((long long)x + y) >> 1); }
AGX_NV int __nv_rhadd(int x, int y) { return (int)(((long long)x + y + 1) >> 1); }
AGX_NV int __nv_uhadd(unsigned x, unsigned y) { return (unsigned)(((unsigned long long)x + y) >> 1); }
AGX_NV unsigned __nv_urhadd(unsigned x, unsigned y) { return (unsigned)(((unsigned long long)x + y + 1) >> 1); }

// ---- atomics (the device_atomic_functions of the CUDA headers)

#define AGX_ATOMIC(name, T, f) \
  static __device__ __forceinline__ T name(T *p, T v) { return f(p, v); }
AGX_ATOMIC(atomicAdd, int, __iAtomicAdd)
AGX_ATOMIC(atomicAdd, unsigned int, __uAtomicAdd)
AGX_ATOMIC(atomicAdd, unsigned long long, __ullAtomicAdd)
AGX_ATOMIC(atomicAdd, float, __fAtomicAdd)
AGX_ATOMIC(atomicAdd, double, __dAtomicAdd)
AGX_ATOMIC(atomicExch, int, __iAtomicExch)
AGX_ATOMIC(atomicExch, unsigned int, __uAtomicExch)
AGX_ATOMIC(atomicExch, unsigned long long, __ullAtomicExch)
AGX_ATOMIC(atomicExch, float, __fAtomicExch)
AGX_ATOMIC(atomicMin, int, __iAtomicMin)
AGX_ATOMIC(atomicMin, unsigned int, __uAtomicMin)
AGX_ATOMIC(atomicMin, long long, __illAtomicMin)
AGX_ATOMIC(atomicMin, unsigned long long, __ullAtomicMin)
AGX_ATOMIC(atomicMax, int, __iAtomicMax)
AGX_ATOMIC(atomicMax, unsigned int, __uAtomicMax)
AGX_ATOMIC(atomicMax, long long, __illAtomicMax)
AGX_ATOMIC(atomicMax, unsigned long long, __ullAtomicMax)
AGX_ATOMIC(atomicAnd, int, __iAtomicAnd)
AGX_ATOMIC(atomicAnd, unsigned int, __uAtomicAnd)
AGX_ATOMIC(atomicAnd, unsigned long long, __ullAtomicAnd)
AGX_ATOMIC(atomicOr, int, __iAtomicOr)
AGX_ATOMIC(atomicOr, unsigned int, __uAtomicOr)
AGX_ATOMIC(atomicOr, unsigned long long, __ullAtomicOr)
AGX_ATOMIC(atomicXor, int, __iAtomicXor)
AGX_ATOMIC(atomicXor, unsigned int, __uAtomicXor)
AGX_ATOMIC(atomicXor, unsigned long long, __ullAtomicXor)
AGX_ATOMIC(atomicInc, unsigned int, __uAtomicInc)
AGX_ATOMIC(atomicDec, unsigned int, __uAtomicDec)
static __device__ __forceinline__ int atomicSub(int *p, int v) { return __iAtomicAdd(p, -v); }
static __device__ __forceinline__ unsigned int atomicSub(unsigned int *p, unsigned int v) { return __uAtomicAdd(p, -v); }
static __device__ __forceinline__ int atomicCAS(int *p, int c, int v) { return __iAtomicCAS(p, c, v); }
static __device__ __forceinline__ unsigned int atomicCAS(unsigned int *p, unsigned int c, unsigned int v) { return __uAtomicCAS(p, c, v); }
static __device__ __forceinline__ unsigned long long atomicCAS(unsigned long long *p, unsigned long long c, unsigned long long v) {
  return __ullAtomicCAS(p, c, v);
}
