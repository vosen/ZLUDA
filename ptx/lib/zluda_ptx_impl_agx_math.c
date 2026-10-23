// Approximate double precision instructions of PTX that have an exact definition: rcp.approx.ftz.f64 and
// rsqrt.approx.ftz.f64 (PTX ISA 9.7.3.13, 9.7.3.16) work on the upper word of the operand (1.11.20), put the result
// in 1.11.20 into the upper word and zero the lower one; subnormal inputs and results are sign-preserving zeros and
// NaN inputs give 0x7fffffff00000000. The result is the exact one rounded to nearest even: the double precision
// quotient (root) is rounded once more, which cannot cross a midpoint of 21 bits.
//
// Compiled to LLVM IR by ptx/lib/gen_agx_tex.py (clang -target nvptx64) into the generated section of
// zluda_ptx_impl_agx.ll; regenerate after editing.

#include <stdint.h>

#define API __attribute__((always_inline))

static uint64_t bits_of(double x) {
    uint64_t b;
    __builtin_memcpy(&b, &x, 8);
    return b;
}

static double of_bits(uint64_t b) {
    double x;
    __builtin_memcpy(&x, &b, 8);
    return x;
}

static const uint64_t SIGN = 1ull << 63, EXP = 0x7ffull << 52, NAN_RESULT = 0x7fffffff00000000ull;

// the upper word, subnormals flushed
static uint64_t operand(uint64_t a) {
    a &= 0xffffffff00000000ull;
    return (a & EXP) == 0 ? a & SIGN : a;
}

// to 1.11.20, nearest even; subnormal results flushed
static uint64_t result(double r) {
    uint64_t b = bits_of(r);
    if ((b & EXP) != EXP)
        b += 0x7fffffffull + ((b >> 32) & 1);
    b &= 0xffffffff00000000ull;
    return (b & EXP) == 0 ? b & SIGN : b;
}

API double __zluda_ptx_impl_rcp_approx_ftz_f64(double x) {
    uint64_t a = operand(bits_of(x));
    if ((a & ~SIGN) > EXP)
        return of_bits(NAN_RESULT);
    if ((a & ~SIGN) == 0)
        return of_bits((a & SIGN) | EXP);
    if ((a & ~SIGN) == EXP)
        return of_bits(a & SIGN);
    return of_bits(result(1.0 / of_bits(a)));
}

API double __zluda_ptx_impl_rsqrt_approx_ftz_f64(double x) {
    uint64_t a = operand(bits_of(x));
    if ((a & ~SIGN) > EXP || ((a & SIGN) && (a & ~SIGN) != 0))
        return of_bits(NAN_RESULT);
    if ((a & ~SIGN) == 0)
        return of_bits((a & SIGN) | EXP);
    if (a == EXP)
        return 0.0;
    return of_bits(result(1.0 / __builtin_sqrt(of_bits(a))));
}
