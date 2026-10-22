// Conversions to and from the narrow formats added in PTX ISA 9.1 / 9.4, for AGX:
// .pzo, the .scaled::n1::ue8m0 / .scaled::n2::ue8m0 scale factors, .ue5m3x2 and
// .s2f6x2. Compiled to LLVM IR by gen_agx_tex.py.
//
// Every value is handled exactly as a sign, an integer significand and a power of
// two, so the scale factors (2^-127 .. 2^127) never round or overflow.

typedef unsigned int uint32;
typedef int int32;
typedef unsigned long long uint64;
typedef long long int64;

enum { K_E4M3, K_E5M2, K_E2M3, K_E3M2, K_E2M1, K_UE5M3, K_S2F6, K_UE8M0 };
// R_RP rounds a magnitude up (away from zero); the instructions' .rp / .rm (R_RM) are signed
enum { R_RN, R_RZ, R_RP, R_RM };
enum { F_RELU = 4, F_PZO = 8, F_N1 = 16, F_N2 = 32, F_SATFINITE = 64 };
enum { S_F32, S_F16X2, S_BF16X2 };

typedef struct {
    int neg, nan, inf;
    uint64 m; // 0 for zero
    int e;    // value = m * 2^e
} value;

static value from_f32(uint32 x) {
    value v = {(int)(x >> 31), 0, 0, 0, 0};
    uint32 exponent = (x >> 23) & 255, fraction = x & 0x7fffff;
    if (exponent == 255) {
        v.nan = fraction != 0;
        v.inf = fraction == 0;
    } else if (exponent == 0) {
        v.m = fraction;
        v.e = -149;
    } else {
        v.m = fraction | 0x800000;
        v.e = (int)exponent - 150;
    }
    return v;
}

static value from_f16(uint32 x) {
    value v = {(int)((x >> 15) & 1), 0, 0, 0, 0};
    uint32 exponent = (x >> 10) & 31, fraction = x & 0x3ff;
    if (exponent == 31) {
        v.nan = fraction != 0;
        v.inf = fraction == 0;
    } else if (exponent == 0) {
        v.m = fraction;
        v.e = -24;
    } else {
        v.m = fraction | 0x400;
        v.e = (int)exponent - 25;
    }
    return v;
}

static int bit_length(uint64 m) {
    int n = 0;
    while (m) {
        m >>= 1;
        n++;
    }
    return n;
}

// m * 2^shift rounded to an integer
static uint64 round_shift(uint64 m, int shift, int rounding) {
    if (shift >= 0)
        return shift > 40 ? (1ull << 62) : m << shift;
    int right = -shift;
    uint64 q = right >= 64 ? 0 : m >> right;
    uint64 rest = right >= 64 ? m : m & ((1ull << right) - 1);
    if (rest == 0 || rounding == R_RZ)
        return q;
    if (rounding == R_RP)
        return q + 1;
    if (right > 63)
        return q;
    uint64 half = 1ull << (right - 1);
    return q + (rest > half || (rest == half && (q & 1)));
}

// the magnitude code of m * 2^e (m > 0) in a format with the given exponent and
// mantissa bits, unbounded above (the caller saturates)
static int64 encode(uint64 m, int e, int exponent_bits, int mantissa_bits, int rounding) {
    int bias = (1 << (exponent_bits - 1)) - 1;
    int min_exponent = 1 - bias;
    int p = e + bit_length(m) - 1;
    if (p < min_exponent)
        p = min_exponent;
    if (p > 200)
        return 1ll << 40;
    uint64 n = round_shift(m, e - (p - mantissa_bits), rounding);
    if (n >= (2ull << mantissa_bits))
        return (int64)(p + 1 + bias) << mantissa_bits;
    if (n >= (1ull << mantissa_bits))
        return ((int64)(p + bias) << mantissa_bits) + (int64)(n - (1ull << mantissa_bits));
    return (int64)n;
}

static int exponent_bits(uint32 kind) {
    return kind == K_E4M3 ? 4 : kind == K_E2M3 || kind == K_E2M1 ? 2 : kind == K_E3M2 ? 3 : 5;
}
static int mantissa_bits(uint32 kind) {
    return kind == K_E5M2 || kind == K_E3M2 ? 2 : kind == K_E2M1 ? 1 : 3;
}
static int top(uint32 kind) {
    return kind == K_E4M3 ? 126 : kind == K_E5M2 ? 123 : kind == K_E2M1 ? 7 : kind == K_UE5M3 ? 254 : 31;
}
static int element_bits(uint32 kind) {
    return kind == K_E2M3 || kind == K_E3M2 ? 6 : kind == K_E2M1 ? 4 : 8;
}

// x / 2^(scale - 127), flushed first with .scaled::n1 below 2^flush_exponent
static value prepare(value v, uint32 flags, uint32 scale, int flush_exponent) {
    if ((flags & F_N1) && !v.nan && !v.inf && v.m && v.e + bit_length(v.m) - 1 < flush_exponent) {
        v.m = 0;
        v.neg = 0;
    }
    if (scale == 255)
        v.nan = 1;
    v.e -= (int)scale - 127;
    return v;
}

// the rounding of the magnitude for the instruction's rounding of a value of the sign: .rp and .rm toward positive
// and negative infinity
static int magnitude_rounding(int rounding, int neg) {
    if (rounding == R_RP)
        return neg ? R_RZ : R_RP;
    if (rounding == R_RM)
        return neg ? R_RP : R_RZ;
    return rounding;
}

// .ue8m0 (.satfinite): an |x| beyond 2^127 (infinities too) gives 0xfe, also for a negative x; other negative
// values and zero give 0; .rz rounds the exponent down, .rp up, .rn to the nearest power of two (ties, 1.5 * 2^e, to
// the even code)
static uint32 ue8m0_element(int rounding, value v) {
    if (v.nan)
        return 255;
    if (v.inf || (v.m && v.e + bit_length(v.m) - 1 >= 127 && !(v.e + bit_length(v.m) - 1 == 127 && !(v.m & (v.m - 1)))))
        return 254;
    if (v.neg || !v.m)
        return 0;
    int length = bit_length(v.m);
    int p = v.e + length - 1;
    int power = !(v.m & (v.m - 1));
    if (rounding == R_RP && !power)
        p++;
    else if (rounding == R_RN && !power) {
        uint64 half = 3ull << (length - 2);
        if (v.m > half || (v.m == half && ((p + 127) & 1)))
            p++;
    }
    int code = p + 127;
    return code < 0 ? 0 : code > 254 ? 254 : (uint32)code;
}

static uint32 narrow_element(uint32 kind, uint32 flags, value v) {
    if (kind == K_UE8M0)
        return ue8m0_element(flags & 3, v);
    int rounding = magnitude_rounding(flags & 3, v.neg);
    int relu = (flags & F_RELU) != 0;
    if (kind == K_S2F6) {
        if (v.nan)
            return 127;
        if (relu && v.neg)
            return 0;
        if (v.inf)
            return v.neg ? 128 : 127;
        uint64 n = v.m ? round_shift(v.m, v.e + 6, rounding) : 0;
        if (v.neg)
            return (uint32)(-(int64)(n > 128 ? 128 : n)) & 255;
        return n > 127 ? 127 : (uint32)n;
    }
    if (v.nan)
        return kind == K_E4M3 || kind == K_E5M2 ? 127 : kind == K_UE5M3 ? 255 : (uint32)top(kind);
    // .satfinite gives .ue5m3 the positive MAX_NORM for an |x| beyond it, also for a negative x (without .relu)
    if (kind == K_UE5M3 && !relu && v.neg &&
        (v.inf || (v.m && encode(v.m, v.e, exponent_bits(kind), mantissa_bits(kind), R_RP) > top(kind))))
        return (uint32)top(kind);
    if ((relu || kind == K_UE5M3) && v.neg)
        return 0;
    uint32 sign = v.neg && kind != K_UE5M3 ? 1u << (element_bits(kind) - 1) : 0;
    int64 code = top(kind);
    if (!v.inf) {
        code = v.m ? encode(v.m, v.e, exponent_bits(kind), mantissa_bits(kind), rounding) : 0;
        if (code > top(kind))
            code = top(kind);
    }
    uint32 result = sign | (uint32)code;
    if ((flags & F_PZO) && kind != K_UE5M3 && result == (1u << (element_bits(kind) - 1)))
        result = 0;
    return result;
}

// cvt.rnd{.relu}.satfinite{.pzo}{.scaled::n1::ue8m0|.scaled::n2::ue8m0}.<kind>x2.<src> d, a{, b}{, scale}
// a goes into the high element; f16x2 / bf16x2 sources hold both elements in a
unsigned short __zluda_ptx_impl_agx_cvt_narrow(uint32 kind, uint32 flags, uint32 src_kind, uint32 a, uint32 b,
                                               uint32 scale) {
    value high, low;
    int flush_exponent;
    if (src_kind == S_F32) {
        high = from_f32(a);
        low = from_f32(b);
        flush_exponent = -126;
    } else if (src_kind == S_F16X2) {
        high = from_f16(a >> 16);
        low = from_f16(a & 0xffff);
        flush_exponent = -14;
    } else {
        high = from_f32(a & 0xffff0000);
        low = from_f32(a << 16);
        flush_exponent = -14;
    }
    uint32 scale_high = (flags & F_N2) ? (scale >> 8) & 255 : scale & 255;
    uint32 scale_low = scale & 255;
    uint32 h = narrow_element(kind, flags, prepare(high, flags, scale_high, flush_exponent));
    uint32 l = narrow_element(kind, flags, prepare(low, flags, scale_low, flush_exponent));
    return (unsigned short)(kind == K_E2M1 ? (h << 4) | l : (h << 8) | l);
}

static value decode_element(uint32 kind, uint32 c) {
    value v = {0, 0, 0, 0, 0};
    if (kind == K_S2F6) {
        int32 s = (int32)(c & 255) - ((c & 128) ? 256 : 0);
        v.neg = s < 0;
        v.m = (uint64)(s < 0 ? -s : s);
        v.e = -6;
        return v;
    }
    int bits = element_bits(kind);
    uint32 magnitude = kind == K_UE5M3 ? c & 255 : c & ((1u << (bits - 1)) - 1);
    v.neg = kind != K_UE5M3 && ((c >> (bits - 1)) & 1);
    if ((kind == K_E4M3 && magnitude == 127) || (kind == K_E5M2 && magnitude > 124) ||
        (kind == K_UE5M3 && magnitude == 255)) {
        v.nan = 1;
        return v;
    }
    if (kind == K_E5M2 && magnitude == 124) {
        v.inf = 1;
        return v;
    }
    int mb = mantissa_bits(kind);
    int bias = (1 << (exponent_bits(kind) - 1)) - 1;
    uint32 exponent = magnitude >> mb, fraction = magnitude & ((1u << mb) - 1);
    if (exponent == 0) {
        v.m = fraction;
        v.e = 1 - bias - mb;
    } else {
        v.m = fraction | (1u << mb);
        v.e = (int)exponent - bias - mb;
    }
    return v;
}

// round to nearest even into f16 (bf16 = 0) or bf16, overflowing to infinity; .relu makes a negative result 0 and a
// NaN the canonical 0x7fff, .satfinite an |x| beyond the largest finite value that value with its sign
static uint32 to_half(value v, int bf16, uint32 flags) {
    int eb = bf16 ? 8 : 5, mb = bf16 ? 7 : 10;
    uint32 infinity = ((1u << eb) - 1) << mb;
    if (v.nan)
        return (flags & F_RELU) ? 0x7fff : bf16 ? 0x7fc0 : 0x7e00;
    if ((flags & F_RELU) && v.neg)
        return 0;
    uint32 sign = v.neg ? 0x8000 : 0;
    uint32 over = (flags & F_SATFINITE) ? infinity - 1 : infinity;
    if (v.inf)
        return sign | over;
    if (!v.m)
        return sign;
    int64 code = encode(v.m, v.e, eb, mb, R_RN);
    return sign | (code >= infinity ? over : (uint32)code);
}

// cvt.rn{.scaled::n2::ue8m0}.{f16x2,bf16x2}.<kind>x2 d, a{, scale}
uint32 __zluda_ptx_impl_agx_cvt_widen(uint32 kind, uint32 to_bf16, uint32 flags, uint32 a, uint32 scale) {
    uint32 lane = kind == K_E2M1 ? 4 : 8;
    uint32 result = 0;
    for (uint32 i = 0; i < 2; i++) {
        value v = decode_element(kind, (a >> (lane * i)) & ((1u << lane) - 1));
        uint32 s = (i == 1 && (flags & F_N2)) ? (scale >> 8) & 255 : scale & 255;
        if (s == 255)
            v.nan = 1;
        v.e += (int)s - 127;
        result |= to_half(v, (int)to_bf16, flags) << (16 * i);
    }
    return result;
}

// ---------------------------------------------------------------- .pzo (PTX ISA 9.4 9.7.10.24)
// cvt.rn{.relu}{.satfinite}.pzo.{f16,bf16}.f32 (kinds 0, 1), .tf32.f32 (2) and {f16x2,bf16x2}.f32 a, b (3, 4: a in the
// high half): a -0.0 result is +0.0
static uint32 pzo_half(value v, int bf16, uint32 flags) {
    uint32 h = v.nan ? 0x7fff : to_half(v, bf16, flags);
    return h == 0x8000 ? 0 : h;
}

uint32 __zluda_ptx_impl_agx_cvt_pzo(uint32 kind, uint32 flags, uint32 a, uint32 b) {
    value v = from_f32(a);
    if (kind == 2) {
        if (v.nan)
            return 0x7fffffff;
        if ((flags & F_RELU) && v.neg)
            return 0;
        uint32 sign = v.neg ? 0x80000000u : 0;
        uint32 infinity = 255u << 10, over = (flags & F_SATFINITE) ? infinity - 1 : infinity;
        int64 code = v.inf ? infinity : v.m ? encode(v.m, v.e, 8, 10, R_RN) : 0;
        uint32 bits = sign | (code >= infinity ? over : (uint32)code) << 13;
        return bits == 0x80000000u ? 0 : bits;
    }
    if (kind <= 1)
        return pzo_half(v, (int)kind, flags);
    return pzo_half(v, kind == 4, flags) << 16 | pzo_half(from_f32(b), kind == 4, flags);
}

// ---------------------------------------------------------------- stochastic rounding (cvt.rs, PTX ISA 9.7.9.22)
// The magnitude code of v (unbounded above) rounded toward zero, or away from zero when the k random bits r added to
// the k most significant discarded bits carry out
static int64 encode_rs(value v, int exponent_bits_, int mantissa_bits_, uint32 r, int k) {
    int bias = (1 << (exponent_bits_ - 1)) - 1;
    int p = v.e + bit_length(v.m) - 1;
    if (p < 1 - bias)
        p = 1 - bias;
    int discarded = (p - mantissa_bits_) - v.e;
    uint64 t = 0;
    if (discarded > 0) {
        if (discarded >= k)
            t = discarded - k >= 64 ? 0 : (v.m >> (discarded - k)) & ((1ull << k) - 1);
        else
            t = (v.m & ((1ull << discarded) - 1)) << (k - discarded);
    }
    int away = discarded > 0 && t + r >= (1ull << k);
    return encode(v.m, v.e, exponent_bits_, mantissa_bits_, away ? R_RP : R_RZ);
}

static uint32 rs_half(value v, int bf16, uint32 flags, uint32 r) {
    int eb = bf16 ? 8 : 5, mb = bf16 ? 7 : 10;
    uint32 infinity = ((1u << eb) - 1) << mb;
    uint32 result;
    if (v.nan)
        result = 0x7fff;
    else if ((flags & F_RELU) && v.neg)
        result = 0;
    else {
        uint32 sign = v.neg ? 0x8000 : 0;
        uint32 over = (flags & F_SATFINITE) ? infinity - 1 : infinity;
        int64 code = v.inf ? infinity : v.m ? encode_rs(v, eb, mb, r, bf16 ? 16 : 13) : 0;
        result = sign | (code >= infinity ? over : (uint32)code);
    }
    return (flags & F_PZO) && result == 0x8000 ? 0 : result;
}

static uint32 rs_mini(uint32 kind, value v, uint32 flags, uint32 r) {
    int k = kind == K_E2M1 ? 8 : 16;
    uint32 sign = v.neg ? 1u << (element_bits(kind) - 1) : 0;
    uint32 code;
    if (v.nan)
        return kind == K_E4M3 || kind == K_E5M2 ? 127 : (uint32)top(kind);
    if ((flags & F_RELU) && v.neg)
        return 0;
    if (v.inf)
        code = sign | (uint32)top(kind);
    else {
        int64 c = v.m ? encode_rs(v, exponent_bits(kind), mantissa_bits(kind), r, k) : 0;
        code = sign | (uint32)(c > top(kind) ? top(kind) : c);
    }
    return (flags & F_PZO) && code == (1u << (element_bits(kind) - 1)) ? 0 : code;
}

// cvt.rs{.relu}{.satfinite}{.pzo}.{f16x2,bf16x2}.f32 d, a, b, rbits (kind 8, 9: a in the high half, rbits[28:16] /
// rbits[12:0] for f16, rbits[31:16] / rbits[15:0] for bf16) and
// cvt.rs{.relu}.satfinite{.pzo}.{e4m3x4,e5m2x4,e2m3x4,e3m2x4,e2m1x4}.f32 d, {a, b, e, f}, rbits (a in the most
// significant element; rbits[31:24], [15:8], [23:16], [7:0] for the elements of .e2m1x4, rbits[31:16] for a and b
// and rbits[15:0] for e and f otherwise)
uint32 __zluda_ptx_impl_agx_cvt_rs(uint32 kind, uint32 flags, uint32 a, uint32 b, uint32 e, uint32 f, uint32 rbits) {
    if (kind >= 8) {
        int bf16 = kind == 9;
        uint32 mask = bf16 ? 0xffff : 0x1fff;
        return rs_half(from_f32(a), bf16, flags, (rbits >> 16) & mask) << 16 | rs_half(from_f32(b), bf16, flags, rbits & mask);
    }
    if (kind == K_E2M1)
        return rs_mini(kind, from_f32(a), flags, rbits >> 24) << 12 | rs_mini(kind, from_f32(b), flags, (rbits >> 8) & 255) << 8 |
               rs_mini(kind, from_f32(e), flags, (rbits >> 16) & 255) << 4 | rs_mini(kind, from_f32(f), flags, rbits & 255);
    return rs_mini(kind, from_f32(a), flags, rbits >> 16) << 24 | rs_mini(kind, from_f32(b), flags, rbits >> 16) << 16 |
           rs_mini(kind, from_f32(e), flags, rbits & 0xffff) << 8 | rs_mini(kind, from_f32(f), flags, rbits & 0xffff);
}

// ---------------------------------------------------------------- alternate floating point x4 (PTX ISA 9.4 9.7.6)
// add / sub{.rn}{.satfinite}.dtype.atype d, a, c; mul.dtype.atype.ctype d, a, c; fma.dtype.atype.btype d, a, b, c:
// four elements, each operation exact and rounded to nearest even into the .e4m3 / .e5m2 elements of d (and c of
// add / sub / fma). Without .satfinite an .e4m3 overflow is NaN (0x7f), an .e5m2 one infinity; a NaN is 0x7f.
enum { X_E4M3, X_E5M2, X_E2M3, X_E3M2, X_E2M1, X_UE8M0, X_E2M1P4 };
enum { A_ADD, A_SUB, A_MUL, A_FMA };

static value x4_element(uint32 kind, uint32 v, uint32 i) {
    if (kind == X_UE8M0) {
        uint32 c = (v >> (8 * i)) & 255;
        value r = {0, c == 255, 0, 1, (int)c - 127};
        return r;
    }
    if (kind == X_E2M1)
        return decode_element(K_E2M1, (v >> (4 * i)) & 15);
    if (kind == X_E2M1P4)
        return decode_element(K_E2M1, (v >> (8 * i)) & 15);
    uint32 k = kind == X_E4M3 ? K_E4M3 : kind == X_E5M2 ? K_E5M2 : kind == X_E2M3 ? K_E2M3 : K_E3M2;
    return decode_element(k, (v >> (8 * i)) & ((1u << element_bits(k)) - 1));
}

static value x4_mul(value x, value y) {
    value r = {x.neg ^ y.neg, 0, 0, 0, 0};
    if (x.nan || y.nan || ((x.inf || y.inf) && ((!x.m && !x.inf) || (!y.m && !y.inf)))) {
        r.nan = 1;
        return r;
    }
    if (x.inf || y.inf) {
        r.inf = 1;
        return r;
    }
    r.m = x.m * y.m;
    r.e = x.e + y.e;
    return r;
}

// x + y exactly; a much smaller operand only decides the rounding, as a sticky bit below the larger one's
static value x4_add(value x, value y) {
    value r = {0, 0, 0, 0, 0};
    if (x.nan || y.nan || (x.inf && y.inf && x.neg != y.neg)) {
        r.nan = 1;
        return r;
    }
    if (x.inf || y.inf) {
        r.inf = 1;
        r.neg = x.inf ? x.neg : y.neg;
        return r;
    }
    if (!x.m || !y.m) {
        if (!x.m && !y.m) {
            r.neg = x.neg && y.neg;
            return r;
        }
        return x.m ? x : y;
    }
    int px = x.e + bit_length(x.m), py = y.e + bit_length(y.m);
    if (px < py || (px == py && x.e > y.e)) {
        value t = x;
        x = y;
        y = t;
        int u = px;
        px = py;
        py = u;
    }
    // x is the larger in magnitude
    if (px - py > 40) {
        r.neg = x.neg;
        r.m = (x.m << 3) + (x.neg == y.neg ? 1 : (uint64)-1);
        r.e = x.e - 3;
        return r;
    }
    int e = x.e < y.e ? x.e : y.e;
    int64 a = (int64)(x.m << (x.e - e)), b = (int64)(y.m << (y.e - e));
    int64 sum = (x.neg ? -a : a) + (y.neg ? -b : b);
    r.neg = sum < 0;
    r.m = (uint64)(sum < 0 ? -sum : sum);
    r.e = e;
    return r;
}

static uint32 x4_encode(uint32 kind, value z, uint32 satfinite) {
    uint32 k = kind == X_E5M2 ? K_E5M2 : K_E4M3;
    uint32 sign = z.neg ? 128 : 0;
    uint32 over = satfinite ? sign + (uint32)top(k) : k == K_E5M2 ? sign + 124 : 127;
    if (z.nan)
        return 127;
    if (z.inf)
        return over;
    if (!z.m)
        return sign;
    int64 code = encode(z.m, z.e, exponent_bits(k), mantissa_bits(k), R_RN);
    return code <= top(k) ? sign + (uint32)code : over;
}

// kinds: d | a << 4 | b << 8 | c << 12
uint32 __zluda_ptx_impl_agx_altfp_x4(uint32 op, uint32 kinds, uint32 satfinite, uint32 a, uint32 b, uint32 c) {
    uint32 dk = kinds & 15, ak = (kinds >> 4) & 15, bk = (kinds >> 8) & 15, ck = (kinds >> 12) & 15;
    uint32 result = 0;
    for (uint32 i = 0; i < 4; i++) {
        value x = x4_element(ak, a, i), y = x4_element(ck, c, i), z;
        if (op == A_MUL)
            z = x4_mul(x, y);
        else if (op == A_FMA)
            z = x4_add(x4_mul(x, x4_element(bk, b, i)), y);
        else {
            if (op == A_SUB)
                y.neg ^= 1;
            z = x4_add(x, y);
        }
        result |= x4_encode(dk, z, satfinite) << (8 * i);
    }
    return result;
}

// ---------------------------------------------------------------- spcompress / spdecompress (PTX ISA 9.4)
// Structured sparsity of register vectors, one 64-bit iteration of the source at a time.

// an integer that orders like the value of the element (type code t of spdesc[4:2], element bits es);
// absolute: of the magnitude; *nan: the element is a NaN
static int64 sparse_key(uint32 t, uint32 es, uint32 b, int absolute, int *nan) {
    uint32 sign = 0, magnitude = 0;
    *nan = 0;
    if (es == 8 && t == 0) {
        magnitude = b & 255;
    } else if (es == 8 && t == 1) {
        int32 v = (int32)(b & 255) - ((b & 128) ? 256 : 0);
        sign = v < 0;
        magnitude = (uint32)(v < 0 ? -v : v);
    } else if (es == 16 && t == 0) {
        sign = (b >> 15) & 1;
        magnitude = b & 0x7fff;
        *nan = magnitude > 0x7c00;
    } else if (es == 16 && t == 1) {
        sign = (b >> 15) & 1;
        magnitude = b & 0x7fff;
        *nan = magnitude > 0x7f80;
    } else if (t == 2 || t == 3) {
        sign = (b >> 7) & 1;
        magnitude = b & 0x7f;
        *nan = t == 2 ? magnitude > 0x7c : magnitude == 0x7f;
    } else if (t == 4 || t == 5) {
        sign = (b >> 5) & 1;
        magnitude = b & 0x1f;
    } else if (t == 6) {
        // f32 bits in a narrower element: a small positive value
        magnitude = b;
    } else if (t == 7) {
        // f32 (tcgen05.ld.spcompress)
        sign = b >> 31;
        magnitude = b & 0x7fffffff;
        *nan = magnitude > 0x7f800000;
    }
    if (absolute)
        return magnitude;
    return sign ? -(int64)magnitude : (int64)magnitude;
}

// does element j precede element i in the selection order (op 0 max, 1 maxabs, 2 min, 3 minabs)?
static int sparse_before(uint32 op, int64 kj, int nj, uint32 j, int64 ki, int ni, uint32 i) {
    if (nj)
        return !ni || j < i;
    if (ni)
        return 0;
    if (kj == ki)
        return j < i;
    return op < 2 ? kj > ki : kj < ki;
}

// one iteration of spcompress.sp::2:4: the 64 source bits hi:lo hold groups of four elements of es bits; two of each
// group are kept. The result: the kept elements (32 bits) in the low half, their indices (ib bits each, 2 per group)
// in the high half.
uint64 __zluda_ptx_impl_agx_spc_iter(uint32 desc, uint32 es, uint32 ib, uint32 lo, uint32 hi) {
    uint64 data = ((uint64)hi << 32) | lo;
    uint32 op = desc & 3, t = (desc >> 2) & 7;
    uint32 groups = 64 / (4 * es);
    uint64 kept = 0, indices = 0;
    uint32 kept_count = 0, index_count = 0;
    for (uint32 g = 0; g < groups; g++) {
        uint32 mask = (1u << es) - 1, base = es * 4 * g;
        uint32 e0 = (uint32)(data >> base) & mask, e1 = (uint32)(data >> (base + es)) & mask;
        uint32 e2 = (uint32)(data >> (base + 2 * es)) & mask, e3 = (uint32)(data >> (base + 3 * es)) & mask;
        int n0, n1, n2, n3;
        int64 k0 = sparse_key(t, es, e0, op & 1, &n0), k1 = sparse_key(t, es, e1, op & 1, &n1);
        int64 k2 = sparse_key(t, es, e2, op & 1, &n2), k3 = sparse_key(t, es, e3, op & 1, &n3);
#define BEFORE(j, i) sparse_before(op, k##j, n##j, j, k##i, n##i, i)
#define KEEP(i, rank)                                                                                                  \
    if ((rank) < 2) {                                                                                                  \
        kept |= (uint64)e##i << (es * kept_count++);                                                                   \
        indices |= (uint64)(i) << (ib * index_count++);                                                                \
    }
        KEEP(0, BEFORE(1, 0) + BEFORE(2, 0) + BEFORE(3, 0))
        KEEP(1, BEFORE(0, 1) + BEFORE(2, 1) + BEFORE(3, 1))
        KEEP(2, BEFORE(0, 2) + BEFORE(1, 2) + BEFORE(3, 2))
        KEEP(3, BEFORE(0, 3) + BEFORE(1, 3) + BEFORE(2, 3))
#undef BEFORE
#undef KEEP
    }
    return (kept & 0xffffffff) | (indices << 32);
}

// one iteration of spdecompress.sp::ns:nt: ns elements of es bits in c, their indices (ib bits each) in m; the result
// holds nt elements, the others zero
uint64 __zluda_ptx_impl_agx_spd_iter(uint32 es, uint32 ib, uint32 ns, uint32 nt, uint64 c, uint32 m) {
    uint64 result = 0;
    uint64 element_mask = es == 64 ? ~0ull : (1ull << es) - 1;
    for (uint32 s = 0; s < ns; s++) {
        uint32 index = (m >> (ib * s)) & ((1u << ib) - 1);
        if (index < nt)
            result |= ((c >> (es * s)) & element_mask) << (es * index);
    }
    return result;
}

// ---------------------------------------------------------------- tcgen05 tensor memory
// The tensor memory of a CTA (128 lanes x 576 32-bit columns, the most an exclusive allocation takes, in rows of
// 1024 words) is a slab of device memory. A CTA claims a free slab at its first tcgen05.alloc (the owner word holds
// its linear CTA index + 1) and gives it back when its last allocation is freed; the allocation map (18 granules of
// 32 columns, bit 31 while an exclusive allocation is live) is next to the owner word. A CTA that finds no free slab
// waits for one.
#define TMEM_SLABS 4
#define TMEM_COLS 1024
#define TMEM_WORDS (128 * TMEM_COLS)
#define TMEM_EXCLUSIVE 0x80000000u
typedef __attribute__((address_space(1))) uint32 global_uint32;
static global_uint32 __zluda_agx_tmem_owner[TMEM_SLABS];
static global_uint32 __zluda_agx_tmem_map[TMEM_SLABS];
static global_uint32 __zluda_agx_tmem_pool[TMEM_SLABS * TMEM_WORDS];

uint32 __zluda_ptx_impl_sreg_ctaid(unsigned char member);
uint32 __zluda_ptx_impl_sreg_nctaid(unsigned char member);

static uint32 tmem_key(void) {
    uint32 x = __zluda_ptx_impl_sreg_ctaid(0), y = __zluda_ptx_impl_sreg_ctaid(1), z = __zluda_ptx_impl_sreg_ctaid(2);
    uint32 nx = __zluda_ptx_impl_sreg_nctaid(0), ny = __zluda_ptx_impl_sreg_nctaid(1);
    return x + nx * (y + ny * z) + 1;
}

static int tmem_find(uint32 key) {
    for (int i = 0; i < TMEM_SLABS; i++)
        if (__atomic_load_n(&__zluda_agx_tmem_owner[i], __ATOMIC_ACQUIRE) == key)
            return i;
    return -1;
}

// the slab of the CTA, claimed if it has none
static int tmem_claim(uint32 key) {
    int slab = tmem_find(key);
    while (slab < 0) {
        for (int i = 0; i < TMEM_SLABS && slab < 0; i++) {
            uint32 expected = 0;
            if (__atomic_compare_exchange_n(&__zluda_agx_tmem_owner[i], &expected, key, 0, __ATOMIC_ACQ_REL,
                                            __ATOMIC_ACQUIRE) ||
                expected == key)
                slab = i;
        }
    }
    return slab;
}

// tcgen05.alloc (the elected lane of the warp): the column of a free, aligned run of ncols columns. An exclusive
// allocation (any multiple of 32 columns up to 576) waits until the CTA has no other live allocation and starts at
// column 0; the other allocations wait while it is live.
uint32 __zluda_ptx_impl_agx_tmem_alloc(uint32 ncols, uint32 leader, uint32 exclusive) {
    if (!leader)
        return 0;
    if (exclusive ? (ncols < 32 || ncols > 576 || (ncols & 31)) : (ncols < 32 || ncols > 512 || (ncols & (ncols - 1))))
        __builtin_trap();
    int slab = tmem_claim(tmem_key());
    uint32 n = ncols / 32;
    uint32 ones = (1u << n) - 1;
    for (;;) {
        uint32 map = __atomic_load_n(&__zluda_agx_tmem_map[slab], __ATOMIC_ACQUIRE);
        if (exclusive) {
            uint32 expected = 0;
            if (__atomic_compare_exchange_n(&__zluda_agx_tmem_map[slab], &expected, ones | TMEM_EXCLUSIVE, 0,
                                            __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE))
                return 0;
            continue;
        }
        if (map & TMEM_EXCLUSIVE)
            continue;
        uint32 b = 0;
        while (b + n <= 16 && (map & (ones << b)))
            b += n;
        if (b + n > 16)
            __builtin_trap();
        if (__atomic_compare_exchange_n(&__zluda_agx_tmem_map[slab], &map, map | (ones << b), 0, __ATOMIC_ACQ_REL,
                                        __ATOMIC_ACQUIRE))
            return b * 32;
    }
}

uint64 __zluda_ptx_impl_agx_tmem_base(void);

// The tensor memory of a new CTA reads as zero: after tcgen05.alloc each lane of the warp zeroes the words lane,
// lane + 32, ... of the 128 lanes x ncols columns at taddr
void __zluda_ptx_impl_agx_tmem_zero(uint32 taddr, uint32 ncols, uint32 lane) {
    global_uint32 *base = (global_uint32 *)(unsigned long)__zluda_ptx_impl_agx_tmem_base();
    uint32 column = taddr & 0xffff;
    for (uint32 w = lane; w < 128 * ncols; w += 32)
        base[(w / ncols) * TMEM_COLS + ((column + w % ncols) & (TMEM_COLS - 1))] = 0;
}

// tcgen05.dealloc (the elected lane): frees the columns at taddr, and the slab with the last of them
void __zluda_ptx_impl_agx_tmem_dealloc(uint32 taddr, uint32 ncols, uint32 leader, uint32 exclusive) {
    if (!leader)
        return;
    int slab = tmem_find(tmem_key());
    uint32 column = taddr & 0xffff;
    if (slab < 0 || (column & 31) || ncols < 32 || ncols > (exclusive ? 576 : 512))
        __builtin_trap();
    uint32 n = ncols / 32;
    uint32 bits = ((1u << n) - 1) << (column / 32);
    if (exclusive)
        bits |= TMEM_EXCLUSIVE;
    uint32 old = __atomic_fetch_and(&__zluda_agx_tmem_map[slab], ~bits, __ATOMIC_ACQ_REL);
    if ((old & bits) != bits)
        __builtin_trap();
    if ((old & ~bits) == 0)
        __atomic_store_n(&__zluda_agx_tmem_owner[slab], 0, __ATOMIC_RELEASE);
}

// The address of the CTA's tensor memory: lane l, column c is the 32-bit word l * TMEM_COLS + c
uint64 __zluda_ptx_impl_agx_tmem_base(void) {
    int slab = tmem_find(tmem_key());
    if (slab < 0)
        __builtin_trap();
    return (uint64)(unsigned long)&__zluda_agx_tmem_pool[slab * TMEM_WORDS];
}

// An A or B element of tcgen05.mma as the f32 of the same value (every format of the kinds f16, tf32 and f8f6f4 is
// exactly representable): 0 f16, 1 bf16, 2 tf32, 3 e4m3, 4 e5m2, 5 e2m3, 6 e3m2, 7 e2m1 (the central 4 bits of a byte)
uint32 __zluda_ptx_impl_agx_mma_elem_f32(uint32 code, uint32 raw) {
    value v;
    switch (code) {
    case 0:
        v = from_f16(raw & 0xffff);
        break;
    case 1:
        return (raw & 0xffff) << 16;
    case 2:
        return raw & 0xffffe000;
    case 3:
        v = decode_element(K_E4M3, raw & 255);
        break;
    case 4:
        v = decode_element(K_E5M2, raw & 255);
        break;
    case 5:
        v = decode_element(K_E2M3, raw & 63);
        break;
    case 6:
        v = decode_element(K_E3M2, raw & 63);
        break;
    default:
        v = decode_element(K_E2M1, (raw >> 2) & 15);
        break;
    }
    uint32 sign = v.neg ? 0x80000000u : 0;
    if (v.nan)
        return 0x7fffffff;
    if (v.inf)
        return sign | 0x7f800000;
    if (!v.m)
        return sign;
    return sign | (uint32)encode(v.m, v.e, 8, 23, R_RN);
}

// ---------------------------------------------------------------- tcgen05.cp / shift
typedef __attribute__((address_space(3))) uint32 shared_uint32;

// Word w of row i of a K-major tile of 32-bit elements in shared memory, through an sm_100 descriptor
// (ptx-exec-wg.k wgElem with ptx-exec-tc.k tcDesc)
static uint32 tile_word(uint64 desc, uint32 i, uint32 w) {
    uint32 start = (uint32)(desc & 0x3fff) << 4;
    uint32 lbo = (uint32)((desc >> 16) & 0x3fff) << 4;
    uint32 sbo = (uint32)((desc >> 32) & 0x3fff) << 4;
    uint32 swz = (uint32)(desc >> 61);
    uint32 mode = swz == 2 ? 1 : swz == 4 ? 2 : swz == 6 ? 3 : swz == 1 ? 4 : 0;
    uint32 atom = mode == 0 ? 1 : mode == 2 ? 4 : mode == 3 ? 2 : 8;
    // 4 elements per 16 bytes; the strides in elements
    uint32 t = 4, leading = lbo / 4, stride = sbo / 4;
    uint32 row_stride = mode ? atom * t : t, k_stride = mode ? t : leading;
    uint32 offset = i % 8 * row_stride + i / 8 * stride + w % t + w / t * k_stride;
    uint32 a = start + offset * 4;
    uint32 line = a >> 7;
    if (mode >= 1 && mode <= 3) {
        uint32 b = 4 - mode;
        a ^= (a & (((1u << b) - 1) << 7)) >> 3;
    } else if (mode == 4) {
        a ^= (line & 3) << 5;
    }
    return *(shared_uint32 *)(unsigned long)a;
}

static global_uint32 *tmem_word(global_uint32 *base, uint32 lane, uint32 column) {
    return &base[(lane & 127) * TMEM_COLS + (column & (TMEM_COLS - 1))];
}

// The cluster placeholders of zluda_ptx_impl_agx.ll (llvm_zluda's LowerClusters): __zluda_agx_cluster(3..5) the
// cluster dimensions, (6) the rank, (8..10) the cluster's position in the grid, (11..13) the grid in clusters;
// __zluda_agx_cluster_mapa maps a shared address to another CTA's window
uint32 __zluda_agx_cluster(uint32 which);
uint32 __zluda_agx_cluster_mapa(uint32 address, uint32 rank);

// tmem_key of the CTA of a rank in the cluster
static uint32 tmem_key_of(uint32 rank) {
    uint32 d0 = __zluda_agx_cluster(3), d1 = __zluda_agx_cluster(4), d2 = __zluda_agx_cluster(5);
    uint32 x = __zluda_agx_cluster(8) * d0 + rank % d0;
    uint32 y = __zluda_agx_cluster(9) * d1 + rank / d0 % d1;
    uint32 z = __zluda_agx_cluster(10) * d2 + rank / (d0 * d1);
    uint32 nx = __zluda_agx_cluster(11) * d0, ny = __zluda_agx_cluster(12) * d1;
    return x + nx * (y + ny * z) + 1;
}

// The tensor memory of the CTA of a rank in the cluster (tcgen05 .cta_group::2 reaches the peer CTA's)
uint64 __zluda_ptx_impl_agx_tmem_base_of(uint32 rank) {
    int slab = tmem_find(tmem_key_of(rank));
    if (slab < 0)
        __builtin_trap();
    return (uint64)(unsigned long)&__zluda_agx_tmem_pool[slab * TMEM_WORDS];
}

uint32 __zluda_ptx_impl_agx_cluster_rank(void) { return __zluda_agx_cluster(6); }

// A shared memory descriptor (start address in bits 13:0, 16-byte units) moved to the window of another CTA
uint64 __zluda_ptx_impl_agx_desc_to(uint64 desc, uint32 rank) {
    uint32 start = (uint32)(desc & 0x3fff) << 4;
    uint32 mapped = __zluda_agx_cluster_mapa(start, rank);
    return (desc & ~(uint64)0x3fff) | ((mapped >> 4) & 0x3fff);
}

static void tcgen05_cp(global_uint32 *base, uint32 taddr, uint64 desc, uint32 shape, uint32 nb);
static void tcgen05_shift(global_uint32 *base, uint32 taddr);

// tcgen05.cp.cta_group::N.shape{.b8x16.b6x16_p32 | .b8x16.b4x16_p64} [taddr], s-desc (one thread); with
// .cta_group::2 the peer CTA copies from its own shared memory too
void __zluda_ptx_impl_agx_tcgen05_cp(uint32 taddr, uint64 desc, uint32 shape, uint32 nb, uint32 pair) {
    tcgen05_cp((global_uint32 *)(unsigned long)__zluda_ptx_impl_agx_tmem_base(), taddr, desc, shape, nb);
    if (pair) {
        uint32 peer = __zluda_agx_cluster(6) ^ 1;
        tcgen05_cp((global_uint32 *)(unsigned long)__zluda_ptx_impl_agx_tmem_base_of(peer), taddr,
                   __zluda_ptx_impl_agx_desc_to(desc, peer), shape, nb);
    }
}

void __zluda_ptx_impl_agx_tcgen05_shift(uint32 taddr, uint32 pair) {
    tcgen05_shift((global_uint32 *)(unsigned long)__zluda_ptx_impl_agx_tmem_base(), taddr);
    if (pair)
        tcgen05_shift((global_uint32 *)(unsigned long)__zluda_ptx_impl_agx_tmem_base_of(__zluda_agx_cluster(6) ^ 1),
                      taddr);
}

// shape: 0 128x256b, 1 4x256b, 2 128x128b, 3 64x128b.warpx2::02_13, 4 64x128b.warpx2::01_23, 5 32x128b.warpx4;
// shape: 0 128x256b, 1 4x256b, 2 128x128b, 3 64x128b.warpx2::02_13, 4 64x128b.warpx2::01_23, 5 32x128b.warpx4;
// nb: the bits of the elements to expand to bytes (6 or 4), 0 for none
static void tcgen05_cp(global_uint32 *base, uint32 taddr, uint64 desc, uint32 shape, uint32 nb) {
    uint32 lane = taddr >> 16, column = taddr & 0xffff;
    uint32 rows = shape == 0 || shape == 2 ? 128 : shape == 1 ? 4 : shape == 5 ? 32 : 64;
    uint32 words = shape <= 1 ? 8 : 4;
    for (uint32 i = 0; i < rows; i++) {
        for (uint32 w = 0; w < words; w += 4) {
            uint32 v0 = tile_word(desc, i, w), v1 = tile_word(desc, i, w + 1);
            uint32 v2 = tile_word(desc, i, w + 2), v3 = tile_word(desc, i, w + 3);
            if (nb) {
                // 16 elements of nb bits in the first 96 bits become 16 bytes (fp4 in the central bits)
                uint64 low = (uint64)v0 | ((uint64)v1 << 32);
                uint32 out0 = 0, out1 = 0, out2 = 0, out3 = 0;
                for (uint32 e = 0; e < 16; e++) {
                    uint32 bit = e * nb;
                    uint32 x = bit < 64 ? (uint32)(low >> bit) : 0;
                    if (bit + nb > 64)
                        x |= bit < 64 ? v2 << (64 - bit) : v2 >> (bit - 64);
                    x &= (1u << nb) - 1;
                    if (nb == 4)
                        x <<= 2;
                    uint32 shifted = x << (8 * (e % 4));
                    if (e < 4)
                        out0 |= shifted;
                    else if (e < 8)
                        out1 |= shifted;
                    else if (e < 12)
                        out2 |= shifted;
                    else
                        out3 |= shifted;
                }
                v0 = out0;
                v1 = out1;
                v2 = out2;
                v3 = out3;
            }
            for (uint32 copy = 0; copy < (shape == 5 ? 4 : shape >= 3 ? 2 : 1); copy++) {
                uint32 r = i;
                if (shape == 5)
                    r = i + 32 * copy;
                else if (shape == 3)
                    r = i % 32 + 32 * (2 * (i / 32) + copy);
                else if (shape == 4)
                    r = i % 32 + 32 * (i / 32 + 2 * copy);
                *tmem_word(base, lane + r, column + w) = v0;
                *tmem_word(base, lane + r, column + w + 1) = v1;
                *tmem_word(base, lane + r, column + w + 2) = v2;
                *tmem_word(base, lane + r, column + w + 3) = v3;
            }
        }
    }
}

// tcgen05.shift.down [taddr]: the 8-word rows of the 32-lane block move down one lane, the last is dropped
static void tcgen05_shift(global_uint32 *base, uint32 taddr) {
    uint32 lane = taddr >> 16, column = taddr & 0xffff;
    for (int r = 30; r >= 0; r--)
        for (uint32 w = 0; w < 8; w++)
            *tmem_word(base, lane + r + 1, column + w) = *tmem_word(base, lane + r, column + w);
}

// ---------------------------------------------------------------- tcgen05.ld.red / ld.spcompress
// One step of the reduction of tcgen05.ld.red (ptx-exec-tc.k tcRed): mode bit 0 min (else max), bits 2:1 the type
// (0 f32, 1 u32, 2 s32), bit 3 .NaN (NaNs propagate as the canonical NaN; otherwise a NaN operand is ignored).
// f32 min / max order -0 below +0.
uint32 __zluda_ptx_impl_agx_tcgen05_red(uint32 mode, uint32 acc, uint32 x) {
    int is_min = mode & 1;
    uint32 type = (mode >> 1) & 3;
    if (type == 1)
        return is_min ? (x < acc ? x : acc) : (x > acc ? x : acc);
    if (type == 2)
        return is_min ? ((int32)x < (int32)acc ? x : acc) : ((int32)x > (int32)acc ? x : acc);
    int nan_acc = (acc & 0x7fffffff) > 0x7f800000, nan_x = (x & 0x7fffffff) > 0x7f800000;
    if (nan_acc || nan_x) {
        if ((mode & 8) || (nan_acc && nan_x))
            return 0x7fffffff;
        return nan_acc ? x : acc;
    }
    // an integer key that orders like the value, -0 below +0
    int64 ka = (acc >> 31) ? -(int64)(acc & 0x7fffffff) - 1 : (int64)acc;
    int64 kx = (x >> 31) ? -(int64)(x & 0x7fffffff) - 1 : (int64)x;
    return is_min ? (kx < ka ? x : acc) : (kx > ka ? x : acc);
}

// tcgen05.ld.spcompress: the two kept elements of a group of four f32 (op 0 max, 1 maxabs, 2 min, 3 minabs; NaNs
// first, ties to the lower index) as their indices, the lower one in bits 1:0 and the other in bits 3:2
uint32 __zluda_ptx_impl_agx_tcgen05_spc_f32(uint32 op, uint32 e0, uint32 e1, uint32 e2, uint32 e3) {
    int n0, n1, n2, n3;
    int64 k0 = sparse_key(7, 32, e0, op & 1, &n0), k1 = sparse_key(7, 32, e1, op & 1, &n1);
    int64 k2 = sparse_key(7, 32, e2, op & 1, &n2), k3 = sparse_key(7, 32, e3, op & 1, &n3);
#define BEFORE(j, i) sparse_before(op, k##j, n##j, j, k##i, n##i, i)
    uint32 keep0 = BEFORE(1, 0) + BEFORE(2, 0) + BEFORE(3, 0) < 2;
    uint32 keep1 = BEFORE(0, 1) + BEFORE(2, 1) + BEFORE(3, 1) < 2;
    uint32 keep2 = BEFORE(0, 2) + BEFORE(1, 2) + BEFORE(3, 2) < 2;
    uint32 keep3 = BEFORE(0, 3) + BEFORE(1, 3) + BEFORE(2, 3) < 2;
#undef BEFORE
    uint32 result = 0, count = 0;
    if (keep0)
        result |= 0u << (2 * count++);
    if (keep1)
        result |= 1u << (2 * count++);
    if (keep2)
        result |= 2u << (2 * count++);
    if (keep3)
        result |= 3u << (2 * count++);
    return result;
}
