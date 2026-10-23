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

// ---------------------------------------------------------------- multimem (PTX ISA 9.7.10.15), on one GPU
// A multimem address in a window of the table at MM_BASE (filled by whoever maps the windows: the runtime, the
// conformance harness's .multimem) stands for the same offset in each of the window's device buffers: window k
// starts at MM_BASE + (k + 1) * MM_WINDOW and table entry k holds the count and the buffers. Any other address is a
// multicast object of one device (its own memory). ld_reduce loads each device's element and folds them in device
// order; st and red store or reduce on each device in device order.
#define MM_BASE 0x700000000000ull
#define MM_WINDOW (1ull << 36)
typedef __attribute__((address_space(1))) uint64 global_uint64;
typedef __attribute__((address_space(1))) unsigned short global_uint16;
typedef __attribute__((address_space(1))) unsigned char global_uint8;

static uint32 mm_count(uint64 a) {
    if (a < MM_BASE + MM_WINDOW || a >= MM_BASE + 256 * MM_WINDOW)
        return 1;
    uint64 k = (a - MM_BASE) / MM_WINDOW - 1;
    return *(global_uint32 *)(unsigned long)(MM_BASE + 64 * k);
}

static uint64 mm_device(uint64 a, uint32 i) {
    if (a < MM_BASE + MM_WINDOW || a >= MM_BASE + 256 * MM_WINDOW)
        return a;
    uint64 k = (a - MM_BASE) / MM_WINDOW - 1;
    return *(global_uint64 *)(unsigned long)(MM_BASE + 64 * k + 8 + 8 * i) + (a - MM_BASE) % MM_WINDOW;
}

enum { M_ADD, M_MIN, M_MAX, M_AND, M_OR, M_XOR };
enum { T_B32, T_U32, T_S32, T_B64, T_U64, T_S64, T_F32, T_F64, T_F16, T_BF16, T_F16X2, T_BF16X2, T_E4M3, T_E5M2,
       T_E4M3X2, T_E5M2X2, T_E4M3X4, T_E5M2X4 };
// the floating point formats of a lane and of an accumulator
enum { P_F32, P_F16, P_BF16, P_E4M3, P_E5M2, P_F64 };

typedef union {
    double d;
    uint64 u;
} f64_bits;

static value from_f64(uint64 x) {
    value v = {(int)(x >> 63), 0, 0, 0, 0};
    uint32 exponent = (x >> 52) & 2047;
    uint64 fraction = x & ((1ull << 52) - 1);
    if (exponent == 2047) {
        v.nan = fraction != 0;
        v.inf = fraction == 0;
    } else if (exponent == 0) {
        v.m = fraction;
        v.e = -1074;
    } else {
        v.m = fraction | (1ull << 52);
        v.e = (int)exponent - 1075;
    }
    return v;
}

// the value as a double (exact for the formats here)
static double to_f64(value v) {
    f64_bits b;
    if (v.nan)
        b.u = 0x7ff8000000000000ull;
    else if (v.inf)
        b.u = 0x7ff0000000000000ull;
    else if (!v.m)
        b.u = 0;
    else {
        // a normal double: the significands here have at most 53 bits
        int length = bit_length(v.m);
        b.u = (uint64)(v.e + length - 1 + 1023) << 52 | ((v.m << (53 - length)) & ((1ull << 52) - 1));
    }
    if (v.neg)
        b.u |= 1ull << 63;
    return b.d;
}

static double mm_decode(uint32 p, uint32 bits) {
    switch (p) {
    case P_F32:
        return to_f64(from_f32(bits));
    case P_F16:
        return to_f64(from_f16(bits));
    case P_BF16:
        return to_f64(from_f32(bits << 16));
    case P_E4M3:
        return to_f64(decode_element(K_E4M3, bits & 255));
    default:
        return to_f64(decode_element(K_E5M2, bits & 255));
    }
}

// x rounded to nearest even into the format (an infinity on overflow, saturating for the fp8 formats), as its bits
static uint32 mm_encode(uint32 p, double x) {
    f64_bits b;
    b.d = x;
    value v = from_f64(b.u);
    if (p == P_E4M3 || p == P_E5M2)
        return narrow_element(p == P_E4M3 ? K_E4M3 : K_E5M2, R_RN, v);
    if (p == P_F32) {
        if (v.nan)
            return 0x7fffffff;
        uint32 sign = v.neg ? 0x80000000u : 0;
        int64 code = v.inf ? 255ll << 23 : v.m ? encode(v.m, v.e, 8, 23, R_RN) : 0;
        return sign | (uint32)(code > (255ll << 23) ? 255ll << 23 : code);
    }
    return v.nan ? 0x7fff : to_half(v, p == P_BF16, 0);
}

// PTX min / max: a NaN operand gives the other one, -0 is below +0
static double mm_op(uint32 op, double x, double y) {
    if (op == M_ADD)
        return x + y;
    if (x != x)
        return y;
    if (y != y)
        return x;
    if (x == y) {
        f64_bits a, b;
        a.d = x;
        b.d = y;
        a.u = op == M_MIN ? a.u | b.u : a.u & b.u;
        return a.d;
    }
    return (op == M_MIN) == (x < y) ? x : y;
}

static double mm_round(uint32 p, double x) { return p == P_F64 ? x : mm_decode(p, mm_encode(p, x)); }

// one step of the reduction in the accumulator format (the fp8 formats operate in f16)
static double mm_comb(uint32 op, uint32 a, double x, double y) {
    double r = mm_op(op, x, y);
    if (a == P_E4M3 || a == P_E5M2)
        r = mm_round(P_F16, r);
    return mm_round(a, r);
}

static uint32 mm_lane_format(uint32 type) {
    switch (type) {
    case T_F16:
    case T_F16X2:
        return P_F16;
    case T_BF16:
    case T_BF16X2:
        return P_BF16;
    case T_E4M3:
    case T_E4M3X2:
    case T_E4M3X4:
        return P_E4M3;
    case T_E5M2:
    case T_E5M2X2:
    case T_E5M2X4:
        return P_E5M2;
    case T_F64:
        return P_F64;
    default:
        return P_F32;
    }
}

static uint32 mm_lanes(uint32 type) {
    return type == T_F16X2 || type == T_BF16X2 || type == T_E4M3X2 || type == T_E5M2X2 ? 2
           : type == T_E4M3X4 || type == T_E5M2X4                                     ? 4
                                                                                       : 1;
}

static uint32 mm_bytes(uint32 type) {
    if (type == T_E4M3 || type == T_E5M2)
        return 1;
    if (type == T_F16 || type == T_BF16 || type == T_E4M3X2 || type == T_E5M2X2)
        return 2;
    if (type == T_B64 || type == T_U64 || type == T_S64 || type == T_F64)
        return 8;
    return 4;
}

static uint64 mm_load(uint64 a, uint32 bytes) {
    if (bytes == 1)
        return *(global_uint8 *)(unsigned long)a;
    if (bytes == 2)
        return *(global_uint16 *)(unsigned long)a;
    if (bytes == 4)
        return *(global_uint32 *)(unsigned long)a;
    return *(global_uint64 *)(unsigned long)a;
}

// acc and x combined as the type's elements (the integers as in atom)
static uint64 mm_combine(uint32 op, uint32 type, uint32 acc_format, uint64 acc, uint64 x) {
    if (type <= T_S64) {
        int wide = type >= T_B64;
        int is_signed = type == T_S32 || type == T_S64;
        uint64 mask = wide ? ~0ull : 0xffffffffull;
        int64 sa = wide ? (int64)acc : (int64)(int32)(uint32)acc, sx = wide ? (int64)x : (int64)(int32)(uint32)x;
        uint64 ua = acc & mask, ux = x & mask;
        switch (op) {
        case M_ADD:
            return (ua + ux) & mask;
        case M_MIN:
            return (is_signed ? (sx < sa ? x : acc) : (ux < ua ? x : acc)) & mask;
        case M_MAX:
            return (is_signed ? (sx > sa ? x : acc) : (ux > ua ? x : acc)) & mask;
        case M_AND:
            return ua & ux;
        case M_OR:
            return ua | ux;
        default:
            return ua ^ ux;
        }
    }
    if (type == T_F64) {
        f64_bits a, b;
        a.u = acc;
        b.u = x;
        a.d = mm_op(op, a.d, b.d);
        return a.u;
    }
    uint32 l = mm_lane_format(type), width = 8 * mm_bytes(type) / mm_lanes(type);
    uint64 out = 0;
    for (uint32 j = 0; j < mm_lanes(type); j++) {
        uint32 lane_mask = width == 32 ? 0xffffffffu : (1u << width) - 1;
        double y = mm_decode(l, (uint32)(acc >> (width * j)) & lane_mask);
        double z = mm_decode(l, (uint32)(x >> (width * j)) & lane_mask);
        out |= (uint64)(mm_encode(l, mm_comb(op, l, y, z)) & lane_mask) << (width * j);
    }
    (void)acc_format;
    return out;
}

// multimem.ld_reduce of the element at a: desc = op | type << 3 | acc << 8 (0 none, 1 .acc::f32, 2 .acc::f16)
uint64 __zluda_ptx_impl_agx_mm_ld_reduce(uint64 a, uint32 desc) {
    uint32 op = desc & 7, type = (desc >> 3) & 31, acc = (desc >> 8) & 3;
    uint32 n = mm_count(a), bytes = mm_bytes(type);
    if (type <= T_S64 || type == T_F64 || acc == 0) {
        uint64 r = mm_load(mm_device(a, 0), bytes);
        for (uint32 i = 1; i < n; i++)
            r = mm_combine(op, type, 0, r, mm_load(mm_device(a, i), bytes));
        return r;
    }
    // in the accumulator's format, each lane rounded back at the end
    uint32 l = mm_lane_format(type), p = acc == 1 ? P_F32 : P_F16;
    uint32 width = 8 * bytes / mm_lanes(type), lane_mask = width == 32 ? 0xffffffffu : (1u << width) - 1;
    uint64 out = 0;
    for (uint32 j = 0; j < mm_lanes(type); j++) {
        double r = 0;
        for (uint32 i = 0; i < n; i++) {
            double x = mm_round(p, mm_decode(l, (uint32)(mm_load(mm_device(a, i), bytes) >> (width * j)) & lane_mask));
            r = i == 0 ? x : mm_comb(op, p, r, x);
        }
        out |= (uint64)(mm_encode(l, r) & lane_mask) << (width * j);
    }
    return out;
}

void __zluda_ptx_impl_agx_mm_st(uint64 a, uint64 v, uint32 bytes) {
    for (uint32 i = 0; i < mm_count(a); i++) {
        uint64 d = mm_device(a, i);
        if (bytes == 1)
            *(global_uint8 *)(unsigned long)d = (unsigned char)v;
        else if (bytes == 2)
            *(global_uint16 *)(unsigned long)d = (unsigned short)v;
        else if (bytes == 4)
            *(global_uint32 *)(unsigned long)d = (uint32)v;
        else
            *(global_uint64 *)(unsigned long)d = v;
    }
}

// .f32 additions in device memory flush subnormal operands and results to zero (red.global.add.f32)
static uint32 mm_ftz32(uint32 x) { return (x & 0x7f800000) == 0 ? x & 0x80000000u : x; }

// multimem.red: an atomic update of each device's element (compare and swap on the aligned word holding it)
void __zluda_ptx_impl_agx_mm_red(uint64 a, uint64 v, uint32 desc) {
    uint32 op = desc & 7, type = (desc >> 3) & 31, bytes = mm_bytes(type);
    for (uint32 i = 0; i < mm_count(a); i++) {
        uint64 d = mm_device(a, i);
        if (bytes == 8) {
            global_uint64 *p = (global_uint64 *)(unsigned long)d;
            uint64 old = *p;
            while (!__atomic_compare_exchange_n(p, &old, mm_combine(op, type, 0, old, v), 0, __ATOMIC_RELAXED,
                                                __ATOMIC_RELAXED))
                ;
            continue;
        }
        global_uint32 *p = (global_uint32 *)(unsigned long)(d & ~3ull);
        uint32 shift = 8 * (uint32)(d & 3), mask = bytes == 4 ? 0xffffffffu : ((1u << (8 * bytes)) - 1) << shift;
        uint32 old = *p;
        for (;;) {
            uint32 e = (old & mask) >> shift, x = (uint32)v;
            uint32 r = type == T_F32 && op == M_ADD ? mm_ftz32((uint32)mm_combine(op, type, 0, mm_ftz32(e), mm_ftz32(x)))
                                                    : (uint32)mm_combine(op, type, 0, e, x);
            uint32 next = (old & ~mask) | ((r << shift) & mask);
            if (__atomic_compare_exchange_n(p, &old, next, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED))
                break;
        }
    }
}

// ---------------------------------------------------------------- cp.reduce.async.bulk on .f16 / .bf16
// each element updated atomically (a compare and swap on its aligned word): op 10 add (keeping subnormals, as .noftz),
// 11 min, 12 max (PTX's: a NaN operand gives the other one), with bit 8 for .bf16
typedef __attribute__((address_space(3))) uint32 shared_word;
static uint32 h16_update(uint32 op, uint32 e, uint32 x) {
    uint32 p = (op >> 8) & 1 ? P_BF16 : P_F16;
    uint32 o = (op & 255) == 10 ? M_ADD : (op & 255) == 11 ? M_MIN : M_MAX;
    return mm_encode(p, mm_op(o, mm_decode(p, e), mm_decode(p, x))) & 0xffff;
}

void __zluda_ptx_impl_agx_cp_reduce_bulk_h16_global(global_uint16 *dst, __attribute__((address_space(3))) unsigned short *src,
                                                     uint32 size, uint32 op) {
    for (uint32 i = 0; i < size / 2; i++) {
        uint64 a = (uint64)(unsigned long)(dst + i);
        global_uint32 *w = (global_uint32 *)(unsigned long)(a & ~3ull);
        uint32 shift = 8 * (uint32)(a & 3), x = src[i];
        uint32 old = *w;
        while (!__atomic_compare_exchange_n(w, &old, (old & ~(0xffffu << shift)) | h16_update(op, (old >> shift) & 0xffff, x) << shift,
                                            0, __ATOMIC_RELAXED, __ATOMIC_RELAXED))
            ;
    }
}

void __zluda_ptx_impl_agx_cp_reduce_bulk_h16_shared(__attribute__((address_space(3))) unsigned short *dst,
                                                     __attribute__((address_space(3))) unsigned short *src, uint32 size, uint32 op) {
    for (uint32 i = 0; i < size / 2; i++) {
        uint32 a = (uint32)(unsigned long)(dst + i);
        shared_word *w = (shared_word *)(unsigned long)(a & ~3u);
        uint32 shift = 8 * (a & 3), x = src[i];
        uint32 old = *w;
        while (!__atomic_compare_exchange_n(w, &old, (old & ~(0xffffu << shift)) | h16_update(op, (old >> shift) & 0xffff, x) << shift,
                                            0, __ATOMIC_RELAXED, __ATOMIC_RELAXED))
            ;
    }
}

// ---------------------------------------------------------------- fabric.* (PTX ISA 9.4 9.7.10.4-5), on one GPU
// The logical endpoints are those of the table at FAB_TABLE (filled by the conformance harness's .fabric): entry le
// (64 bytes) holds the device count, the size of the endpoint's memory and up to 7 device buffers. A unicast
// operation reaches the first device, a .multimem one each device in order. The transfers complete at once, on the
// mbarrier: try_get and try_pullred count bytes, try_put and try_red 16-byte units, try_atom one unit.
#define FAB_TABLE 0x6ff000000000ull
typedef __attribute__((address_space(3))) unsigned char shared_byte;
typedef __attribute__((address_space(3))) uint64 shared_uint64;
void __zluda_ptx_impl_agx_mbarrier_complete_tx(shared_uint64 *mbar, uint32 count);

static uint32 fab_count(uint32 le) { return *(global_uint32 *)(unsigned long)(FAB_TABLE + 64ull * (le & 255)); }

// the device address of [le, off .. off + n) on device i, trapping outside the endpoint
static uint64 fab_device(uint32 le, uint64 off, uint64 n, uint32 i) {
    uint64 entry = FAB_TABLE + 64ull * (le & 255);
    uint64 size = *(global_uint32 *)(unsigned long)(entry + 4);
    if (le > 255 || i >= fab_count(le) || off + n > size)
        __builtin_trap();
    return *(global_uint64 *)(unsigned long)(entry + 8 + 8 * i) + off;
}

void __zluda_ptx_impl_agx_fabric_get(shared_byte *dst, uint32 le, uint64 off, uint32 n, shared_uint64 *mbar) {
    global_uint8 *src = (global_uint8 *)(unsigned long)fab_device(le, off, n, 0);
    for (uint32 i = 0; i < n; i++)
        dst[i] = src[i];
    __zluda_ptx_impl_agx_mbarrier_complete_tx(mbar, n);
}

// flags: bit 0 .multimem, bit 1 a byte mask (.cp_mask: byte i is written when bit i % 16 is set), bit 2 a counter
// (.counted::bytes: n added to the 64-bit word at counter on each destination device)
void __zluda_ptx_impl_agx_fabric_put(uint32 le, uint64 off, uint64 counter, shared_byte *src, uint32 n, uint32 mask,
                                     uint32 flags, shared_uint64 *mbar) {
    uint32 devices = (flags & 1) ? fab_count(le) : 1;
    for (uint32 d = 0; d < devices; d++) {
        global_uint8 *dst = (global_uint8 *)(unsigned long)fab_device(le, off, n, d);
        for (uint32 i = 0; i < n; i++)
            if (!(flags & 2) || ((mask >> (i % 16)) & 1))
                dst[i] = src[i];
        if (flags & 4)
            __atomic_fetch_add((global_uint64 *)(unsigned long)fab_device(le, counter, 8, d), (uint64)n, __ATOMIC_RELAXED);
    }
    __zluda_ptx_impl_agx_mbarrier_complete_tx(mbar, n / 16);
}

static uint64 fab_load(shared_byte *p, uint32 bytes) {
    uint64 v = 0;
    for (uint32 i = 0; i < bytes; i++)
        v |= (uint64)p[i] << (8 * i);
    return v;
}

// an atomic update of the element at a: op as multimem (0 add, 1 min, 2 max, 3 and, 4 or, 5 xor) or 6 inc, 7 dec,
// 8 exch, 9 cas (with y); returns the old value
static uint64 fab_atomic(uint64 a, uint32 op, uint32 type, uint64 x, uint64 y) {
    uint32 bytes = mm_bytes(type);
    if (bytes == 8) {
        global_uint64 *p = (global_uint64 *)(unsigned long)a;
        uint64 old = *p, next;
        do {
            next = op == 8 ? x : op == 9 ? (old == x ? y : old) : mm_combine(op, type, 0, old, x);
        } while (!__atomic_compare_exchange_n(p, &old, next, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED));
        return old;
    }
    global_uint32 *p = (global_uint32 *)(unsigned long)(a & ~3ull);
    uint32 shift = 8 * (uint32)(a & 3), mask = bytes == 4 ? 0xffffffffu : ((1u << (8 * bytes)) - 1) << shift;
    uint32 old = *p, e, r;
    for (;;) {
        e = (old & mask) >> shift;
        uint32 xv = (uint32)x;
        if (op == 6)
            r = e >= xv ? 0 : e + 1;
        else if (op == 7)
            r = e == 0 || e > xv ? xv : e - 1;
        else if (op == 8)
            r = xv;
        else if (op == 9)
            r = e == xv ? (uint32)y : e;
        else
            r = (uint32)mm_combine(op, type, 0, e, xv);
        uint32 next = (old & ~mask) | ((r << shift) & mask);
        if (__atomic_compare_exchange_n(p, &old, next, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED))
            break;
    }
    return e;
}

// desc = op | type << 4 (the multimem type numbers)
void __zluda_ptx_impl_agx_fabric_red(uint32 le, uint64 off, uint64 counter, shared_byte *src, uint32 n, uint32 desc,
                                     uint32 flags, shared_uint64 *mbar) {
    uint32 op = desc & 15, type = (desc >> 4) & 31, bytes = mm_bytes(type);
    uint32 devices = (flags & 1) ? fab_count(le) : 1;
    for (uint32 d = 0; d < devices; d++) {
        uint64 dst = fab_device(le, off, n, d);
        for (uint32 i = 0; i + bytes <= n; i += bytes)
            fab_atomic(dst + i, op, type, fab_load(src + i, bytes), 0);
        if (flags & 4)
            __atomic_fetch_add((global_uint64 *)(unsigned long)fab_device(le, counter, 8, d), (uint64)n, __ATOMIC_RELAXED);
    }
    __zluda_ptx_impl_agx_mbarrier_complete_tx(mbar, n / 16);
}

// try_pullred (lane 0): each device's elements, folded in device order as multimem.ld_reduce does
void __zluda_ptx_impl_agx_fabric_pullred(shared_byte *dst, uint32 le, uint64 off, uint32 n, uint32 desc, uint32 lane,
                                         shared_uint64 *mbar) {
    if (lane != 0)
        return;
    uint32 op = desc & 15, type = (desc >> 4) & 31, bytes = mm_bytes(type);
    for (uint32 i = 0; i + bytes <= n; i += bytes) {
        uint64 r = mm_load(fab_device(le, off + i, bytes, 0), bytes);
        for (uint32 d = 1; d < fab_count(le); d++)
            r = mm_combine(op, type, 0, r, mm_load(fab_device(le, off + i, bytes, d), bytes));
        for (uint32 k = 0; k < bytes; k++)
            dst[i + k] = (unsigned char)(r >> (8 * k));
    }
    __zluda_ptx_impl_agx_mbarrier_complete_tx(mbar, n);
}

// try_atom [le, off], [dst], [src], [mbar]: the operand at src + off % 16 (and, for cas, the new value 16 bytes on),
// the old value to dst + off % 16
void __zluda_ptx_impl_agx_fabric_atom(uint32 le, uint64 off, shared_byte *dst, shared_byte *src, uint32 desc,
                                      shared_uint64 *mbar) {
    uint32 op = desc & 15, type = (desc >> 4) & 31, bytes = mm_bytes(type), so = (uint32)(off % 16);
    uint64 x = fab_load(src + so, bytes), y = op == 9 ? fab_load(src + 16 + so, bytes) : 0;
    uint64 old = fab_atomic(fab_device(le, off, bytes, 0), op, type, x, y);
    for (uint32 k = 0; k < bytes; k++)
        dst[so + k] = (unsigned char)(old >> (8 * k));
    __zluda_ptx_impl_agx_mbarrier_complete_tx(mbar, 1);
}

// the .tensor forms (tile mode): the tensor map with its global address replaced by the endpoint's memory at off on
// each device (and, with .override::global_dim, its nd dimensions), copied as cp.async.bulk.tensor does. flags: bit 0
// .multimem, bit 1 try_put. try_get counts bytes on the mbarrier, try_put size in 16-byte units.
void __zluda_ptx_impl_agx_tma_override(uint64 *dst, const uint64 *tm, _Bool has_address, uint64 address, uint32 nd,
                                       uint32 d0, uint32 d1, uint32 d2, uint32 d3, uint32 d4, uint32 ns, uint32 s0,
                                       uint32 s1, uint32 s2, uint32 s3, uint32 up);
uint32 __zluda_ptx_impl_agx_tma_copy(const uint64 *tm, uint32 c0, uint32 c1, uint32 c2, uint32 c3, uint32 c4,
                                     uint32 imode, uint32 i0, uint32 i1, uint32 i2, uint32 smem, uint32 dir);
void __zluda_ptx_impl_agx_fabric_tensor(const uint64 *tm, uint32 le, uint64 off, uint32 nd, uint32 d0, uint32 d1,
                                        uint32 d2, uint32 d3, uint32 d4, uint32 c0, uint32 c1, uint32 c2, uint32 c3,
                                        uint32 c4, shared_byte *smem, uint32 size, uint32 flags, shared_uint64 *mbar) {
    uint64 map[16];
    uint32 store = flags & 2, devices = store && (flags & 1) ? fab_count(le) : 1, bytes = 0;
    for (uint32 d = 0; d < devices; d++) {
        __zluda_ptx_impl_agx_tma_override(map, tm, 1, fab_device(le, off, 0, d), nd, d0, d1, d2, d3, d4, 0, 0, 0, 0,
                                          0, 0);
        bytes = __zluda_ptx_impl_agx_tma_copy(map, c0, c1, c2, c3, c4, ~0u, 0, 0, 0, (uint32)(unsigned long)smem,
                                              store ? ~1u : ~0u);
    }
    __zluda_ptx_impl_agx_mbarrier_complete_tx(mbar, store ? size / 16 : bytes);
}
