// libdevice's math functions (__nv_*) for zluda_rtc_agx's NVRTC, in portable
// C++ on IEEE single and double precision arithmetic, so the same file is
// tested against the host's libm (zluda_rtc_agx/tests). AGX_NV is the
// declaration prefix: device inline functions under NVRTC. Accuracy goals:
// a few ulp for the elementary functions on their usual ranges (the
// trigonometric reductions are exact to |x| ~ 1e5 in single and 1e9 in
// double precision), 1e-6 absolute for erff.

#define AGX_F2U(x) __builtin_bit_cast(unsigned int, (float)(x))
#define AGX_U2F(x) __builtin_bit_cast(float, (unsigned int)(x))
#define AGX_D2U(x) __builtin_bit_cast(unsigned long long, (double)(x))
#define AGX_U2D(x) __builtin_bit_cast(double, (unsigned long long)(x))

AGX_NV int __nv_isnanf(float x) { return x != x; }
AGX_NV int __nv_isinff(float x) { return __builtin_fabsf(x) == __builtin_inff(); }
AGX_NV int __nv_finitef(float x) { return __builtin_fabsf(x) < __builtin_inff(); }
AGX_NV int __nv_signbitf(float x) { return (int)(AGX_F2U(x) >> 31); }
AGX_NV int __nv_isnand(double x) { return x != x; }
AGX_NV int __nv_isinfd(double x) { return __builtin_fabs(x) == __builtin_inf(); }
AGX_NV int __nv_isfinited(double x) { return __builtin_fabs(x) < __builtin_inf(); }
AGX_NV int __nv_signbitd(double x) { return (int)(AGX_D2U(x) >> 63); }

AGX_NV float __nv_fabsf(float x) { return __builtin_fabsf(x); }
AGX_NV double __nv_fabs(double x) { return __builtin_fabs(x); }
AGX_NV float __nv_copysignf(float x, float y) { return __builtin_copysignf(x, y); }
AGX_NV double __nv_copysign(double x, double y) { return __builtin_copysign(x, y); }
AGX_NV float __nv_floorf(float x) { return __builtin_floorf(x); }
AGX_NV double __nv_floor(double x) { return __builtin_floor(x); }
AGX_NV float __nv_ceilf(float x) { return __builtin_ceilf(x); }
AGX_NV double __nv_ceil(double x) { return __builtin_ceil(x); }
AGX_NV float __nv_truncf(float x) { return __builtin_truncf(x); }
AGX_NV double __nv_trunc(double x) { return __builtin_trunc(x); }
AGX_NV float __nv_rintf(float x) { return __builtin_rintf(x); }
AGX_NV double __nv_rint(double x) { return __builtin_rint(x); }
AGX_NV float __nv_nearbyintf(float x) { return __builtin_rintf(x); }
AGX_NV double __nv_nearbyint(double x) { return __builtin_rint(x); }
// half away from zero
AGX_NV float __nv_roundf(float x) {
  float t = __builtin_truncf(x);
  return __builtin_fabsf(x - t) >= 0.5f ? t + __builtin_copysignf(1.0f, x) : t;
}
AGX_NV double __nv_round(double x) {
  double t = __builtin_trunc(x);
  return __builtin_fabs(x - t) >= 0.5 ? t + __builtin_copysign(1.0, x) : t;
}
AGX_NV long long __nv_llrintf(float x) { return (long long)__builtin_rintf(x); }
AGX_NV long long __nv_llrint(double x) { return (long long)__builtin_rint(x); }
AGX_NV long long __nv_llroundf(float x) { return (long long)__nv_roundf(x); }
AGX_NV long long __nv_llround(double x) { return (long long)__nv_round(x); }
AGX_NV float __nv_fminf(float x, float y) { return __builtin_fminf(x, y); }
AGX_NV double __nv_fmin(double x, double y) { return __builtin_fmin(x, y); }
AGX_NV float __nv_fmaxf(float x, float y) { return __builtin_fmaxf(x, y); }
AGX_NV double __nv_fmax(double x, double y) { return __builtin_fmax(x, y); }
AGX_NV float __nv_fdimf(float x, float y) { return x > y ? x - y : (x != x || y != y ? x + y : 0.0f); }
AGX_NV double __nv_fdim(double x, double y) { return x > y ? x - y : (x != x || y != y ? x + y : 0.0); }
AGX_NV float __nv_fmaf(float x, float y, float z) { return __builtin_fmaf(x, y, z); }
AGX_NV double __nv_fma(double x, double y, double z) { return __builtin_fma(x, y, z); }
AGX_NV float __nv_sqrtf(float x) { return __builtin_sqrtf(x); }
AGX_NV double __nv_sqrt(double x) { return __builtin_sqrt(x); }
AGX_NV float __nv_rsqrtf(float x) { return 1.0f / __builtin_sqrtf(x); }
AGX_NV double __nv_rsqrt(double x) { return 1.0 / __builtin_sqrt(x); }
AGX_NV float __nv_saturatef(float x) { return x > 0.0f ? (x < 1.0f ? x : 1.0f) : 0.0f; }

// ---- exponent manipulation

AGX_NV float __nv_ldexpf(float x, int e) {
  // three steps keep the intermediate scales normal
  if (e > 300) e = 300;
  if (e < -300) e = -300;
  int a = e / 3, b = e - 2 * a;
  float s = AGX_U2F((unsigned)(a + 127) << 23 & 0x7f800000u);
  float t = AGX_U2F((unsigned)(b + 127) << 23 & 0x7f800000u);
  if (a + 127 <= 0 || a + 127 >= 255 || b + 127 <= 0 || b + 127 >= 255) {
    float r = x;
    for (int i = 0; i < (e > 0 ? e : -e); i++) r *= e > 0 ? 2.0f : 0.5f;
    return r;
  }
  return x * s * s * t;
}
AGX_NV double __nv_ldexp(double x, int e) {
  if (e > 3000) e = 3000;
  if (e < -3000) e = -3000;
  int a = e / 3, b = e - 2 * a;
  if (a + 1023 <= 0 || a + 1023 >= 2047 || b + 1023 <= 0 || b + 1023 >= 2047) {
    double r = x;
    for (int i = 0; i < (e > 0 ? e : -e); i++) r *= e > 0 ? 2.0 : 0.5;
    return r;
  }
  double s = AGX_U2D((unsigned long long)(a + 1023) << 52);
  double t = AGX_U2D((unsigned long long)(b + 1023) << 52);
  return x * s * s * t;
}
AGX_NV float __nv_scalbnf(float x, int e) { return __nv_ldexpf(x, e); }
AGX_NV double __nv_scalbn(double x, int e) { return __nv_ldexp(x, e); }
AGX_NV float __nv_frexpf(float x, int *e) {
  if (x == 0.0f || x != x || __nv_isinff(x)) {
    *e = 0;
    return x;
  }
  int k = 0;
  if (__builtin_fabsf(x) < 1.17549435e-38f) {
    x *= 16777216.0f;
    k = -24;
  }
  unsigned u = AGX_F2U(x);
  *e = (int)((u >> 23) & 0xff) - 126 + k;
  return AGX_U2F((u & 0x807fffffu) | 0x3f000000u);
}
AGX_NV double __nv_frexp(double x, int *e) {
  if (x == 0.0 || x != x || __nv_isinfd(x)) {
    *e = 0;
    return x;
  }
  int k = 0;
  if (__builtin_fabs(x) < 2.2250738585072014e-308) {
    x *= 18014398509481984.0;
    k = -54;
  }
  unsigned long long u = AGX_D2U(x);
  *e = (int)((u >> 52) & 0x7ff) - 1022 + k;
  return AGX_U2D((u & 0x800fffffffffffffull) | 0x3fe0000000000000ull);
}
AGX_NV int __nv_ilogbf(float x) {
  int e;
  if (x == 0.0f) return -2147483647 - 1;
  if (x != x) return -2147483647 - 1;
  if (__nv_isinff(x)) return 2147483647;
  __nv_frexpf(x, &e);
  return e - 1;
}
AGX_NV int __nv_ilogb(double x) {
  int e;
  if (x == 0.0) return -2147483647 - 1;
  if (x != x) return -2147483647 - 1;
  if (__nv_isinfd(x)) return 2147483647;
  __nv_frexp(x, &e);
  return e - 1;
}
AGX_NV float __nv_logbf(float x) {
  if (x == 0.0f) return -__builtin_inff();
  if (x != x || __nv_isinff(x)) return __builtin_fabsf(x);
  return (float)__nv_ilogbf(x);
}
AGX_NV double __nv_logb(double x) {
  if (x == 0.0) return -__builtin_inf();
  if (x != x || __nv_isinfd(x)) return __builtin_fabs(x);
  return (double)__nv_ilogb(x);
}
AGX_NV float __nv_modff(float x, float *i) {
  float t = __builtin_truncf(x);
  *i = t;
  return __nv_isinff(x) ? __builtin_copysignf(0.0f, x) : __builtin_copysignf(x - t, x);
}
AGX_NV double __nv_modf(double x, double *i) {
  double t = __builtin_trunc(x);
  *i = t;
  return __nv_isinfd(x) ? __builtin_copysign(0.0, x) : __builtin_copysign(x - t, x);
}
AGX_NV float __nv_nextafterf(float x, float y) {
  if (x != x || y != y) return x + y;
  if (x == y) return y;
  if (x == 0.0f) return __builtin_copysignf(AGX_U2F(1u), y);
  unsigned u = AGX_F2U(x);
  u += ((x < y) == (x > 0.0f)) ? 1 : -1;
  return AGX_U2F(u);
}
AGX_NV double __nv_nextafter(double x, double y) {
  if (x != x || y != y) return x + y;
  if (x == y) return y;
  if (x == 0.0) return __builtin_copysign(AGX_U2D(1ull), y);
  unsigned long long u = AGX_D2U(x);
  u += ((x < y) == (x > 0.0)) ? 1 : -1;
  return AGX_U2D(u);
}
// x - n y, n the truncated (fmod) or nearest even (remainder) quotient, by
// long division on the exponents (exact)
AGX_NV double __nv_fmod(double x, double y) {
  if (y == 0.0 || x != x || y != y || __nv_isinfd(x)) return __builtin_nan("");
  double ax = __builtin_fabs(x), ay = __builtin_fabs(y);
  if (ax < ay) return x;
  int ex, ey;
  __nv_frexp(ax, &ex);
  __nv_frexp(ay, &ey);
  double r = ax;
  for (int e = ex - ey; e >= 0; e--) {
    double s = __nv_ldexp(ay, e);
    if (r >= s) r -= s;
  }
  return __builtin_copysign(r, x);
}
AGX_NV float __nv_fmodf(float x, float y) { return (float)__nv_fmod(x, y); }
AGX_NV double __nv_remainder(double x, double y) {
  if (y == 0.0 || x != x || y != y || __nv_isinfd(x)) return __builtin_nan("");
  double ay = __builtin_fabs(y);
  double r = __builtin_fabs(__nv_fmod(x, y));
  // the quotient's parity: fmod of 2|y|
  double r2 = __builtin_fabs(__nv_fmod(x, 2.0 * ay));
  int odd = r2 >= ay;
  if (r > 0.5 * ay || (r == 0.5 * ay && odd)) r -= ay;
  return __builtin_copysign(1.0, x) * r;
}
AGX_NV float __nv_remainderf(float x, float y) { return (float)__nv_remainder(x, y); }

// ---- single precision elementary functions

// 2^n for integer n in [-149, 127]
AGX_NV float agx_exp2i_f(int n) {
  if (n < -126) return AGX_U2F(1u << (n + 149));
  return AGX_U2F((unsigned)(n + 127) << 23);
}

// e^x = 2^n e^r, |r| <= ln 2 / 2, r = x - n ln 2 in two parts
AGX_NV float __nv_expf(float x) {
  if (x != x) return x;
  if (x > 88.72284f) return __builtin_inff();
  if (x < -103.97208f) return 0.0f;
  float n = __builtin_rintf(x * 1.44269504f);
  float r = __builtin_fmaf(n, -0.693145752f, x);
  r = __builtin_fmaf(n, -1.42860677e-6f, r);
  float p = 1.98412698e-4f;  // 1/7!
  p = __builtin_fmaf(p, r, 1.38888889e-3f);
  p = __builtin_fmaf(p, r, 8.33333333e-3f);
  p = __builtin_fmaf(p, r, 4.16666667e-2f);
  p = __builtin_fmaf(p, r, 1.66666667e-1f);
  p = __builtin_fmaf(p, r, 0.5f);
  p = __builtin_fmaf(p, r * r, r);
  int k = (int)n;
  float e = 1.0f + p;
  // split the scale so 2^k for k up to 128 and down to -150 stays exact
  return (e * agx_exp2i_f(k / 2)) * agx_exp2i_f(k - k / 2);
}
AGX_NV float __nv_exp2f(float x) {
  if (x != x) return x;
  if (x >= 128.0f) return __builtin_inff();
  if (x < -150.0f) return 0.0f;
  float n = __builtin_rintf(x);
  float r = (x - n) * 0.693147181f;
  float p = 1.98412698e-4f;
  p = __builtin_fmaf(p, r, 1.38888889e-3f);
  p = __builtin_fmaf(p, r, 8.33333333e-3f);
  p = __builtin_fmaf(p, r, 4.16666667e-2f);
  p = __builtin_fmaf(p, r, 1.66666667e-1f);
  p = __builtin_fmaf(p, r, 0.5f);
  p = __builtin_fmaf(p, r * r, r);
  int k = (int)n;
  return ((1.0f + p) * agx_exp2i_f(k / 2)) * agx_exp2i_f(k - k / 2);
}
AGX_NV float __nv_exp10f(float x) {
  // 10^x = e^(x ln 10), with the product in two parts
  float hi = x * 2.30258512f;
  float lo = __builtin_fmaf(x, 2.30258512f, -hi) + x * -3.19754365e-8f;
  return __nv_expf(hi) * (1.0f + lo);
}
AGX_NV float __nv_expm1f(float x) {
  if (__builtin_fabsf(x) < 0.35f) {
    // Taylor to x^9
    float p = 2.75573192e-6f;
    p = __builtin_fmaf(p, x, 2.48015873e-5f);
    p = __builtin_fmaf(p, x, 1.98412698e-4f);
    p = __builtin_fmaf(p, x, 1.38888889e-3f);
    p = __builtin_fmaf(p, x, 8.33333333e-3f);
    p = __builtin_fmaf(p, x, 4.16666667e-2f);
    p = __builtin_fmaf(p, x, 1.66666667e-1f);
    p = __builtin_fmaf(p, x, 0.5f);
    return __builtin_fmaf(p, x * x, x);
  }
  if (x > 88.0f) return __nv_expf(x);
  if (x < -18.0f) return -1.0f;
  // e^x - 1 = 2^n expm1(r) + (2^n - 1)
  float n = __builtin_rintf(x * 1.44269504f);
  float r = __builtin_fmaf(n, -0.693145752f, x);
  r = __builtin_fmaf(n, -1.42860677e-6f, r);
  float p = 1.98412698e-4f;
  p = __builtin_fmaf(p, r, 1.38888889e-3f);
  p = __builtin_fmaf(p, r, 8.33333333e-3f);
  p = __builtin_fmaf(p, r, 4.16666667e-2f);
  p = __builtin_fmaf(p, r, 1.66666667e-1f);
  p = __builtin_fmaf(p, r, 0.5f);
  float t = __builtin_fmaf(p, r * r, r);
  float s = agx_exp2i_f((int)n);
  return __builtin_fmaf(s, t, s - 1.0f);
}

// f / d with one correction step: exact to about an ulp even where the
// hardware divides approximately (zluda_agx without --strict-fp)
AGX_NV float agx_div_f(float f, float d) {
  float q = f / d;
  return __builtin_fmaf(__builtin_fmaf(-q, d, f), 1.0f / d, q);
}

// log x = e ln 2 + 2 atanh(s), m in [sqrt(1/2), sqrt(2)), s = (m - 1) / (m + 1)
AGX_NV float __nv_logf(float x) {
  if (x != x || x < 0.0f) return __builtin_nanf("");
  if (x == 0.0f) return -__builtin_inff();
  if (__nv_isinff(x)) return x;
  int e;
  float m = __nv_frexpf(x, &e) * 2.0f;
  e -= 1;
  if (m > 1.41421356f) {
    m *= 0.5f;
    e += 1;
  }
  float s = agx_div_f(m - 1.0f, m + 1.0f);
  float s2 = s * s;
  float p = 1.0f / 11.0f;
  p = __builtin_fmaf(p, s2, 1.0f / 9.0f);
  p = __builtin_fmaf(p, s2, 1.0f / 7.0f);
  p = __builtin_fmaf(p, s2, 0.2f);
  p = __builtin_fmaf(p, s2, 1.0f / 3.0f);
  float t = 2.0f * s * s2 * p;  // 2 atanh(s) - 2 s
  float fe = (float)e;
  // e ln2_hi is exact for |e| < 2^11
  return __builtin_fmaf(fe, 0.693145752f, __builtin_fmaf(fe, 1.42860677e-6f, t) + 2.0f * s);
}
AGX_NV float __nv_log2f(float x) {
  if (x != x || x < 0.0f) return __builtin_nanf("");
  if (x == 0.0f) return -__builtin_inff();
  if (__nv_isinff(x)) return x;
  int e;
  float m = __nv_frexpf(x, &e) * 2.0f;
  e -= 1;
  if (m > 1.41421356f) {
    m *= 0.5f;
    e += 1;
  }
  float s = agx_div_f(m - 1.0f, m + 1.0f);
  float s2 = s * s;
  float p = 1.0f / 11.0f;
  p = __builtin_fmaf(p, s2, 1.0f / 9.0f);
  p = __builtin_fmaf(p, s2, 1.0f / 7.0f);
  p = __builtin_fmaf(p, s2, 0.2f);
  p = __builtin_fmaf(p, s2, 1.0f / 3.0f);
  float ln = __builtin_fmaf(2.0f * s * s2, p, 2.0f * s);
  return __builtin_fmaf(ln, 1.44269504f, (float)e);
}
AGX_NV float __nv_log10f(float x) { return __nv_logf(x) * 0.434294482f; }
AGX_NV float __nv_log1pf(float x) {
  if (x > -0.29f && x < 0.41f) {
    // 2 atanh(s) with s = x / (2 + x) directly
    float s = agx_div_f(x, 2.0f + x);
    float s2 = s * s;
    float p = 1.0f / 11.0f;
    p = __builtin_fmaf(p, s2, 1.0f / 9.0f);
    p = __builtin_fmaf(p, s2, 1.0f / 7.0f);
    p = __builtin_fmaf(p, s2, 0.2f);
    p = __builtin_fmaf(p, s2, 1.0f / 3.0f);
    // 2 s = x - s x
    return __builtin_fmaf(2.0f * s * s2, p, __builtin_fmaf(-s, x, x));
  }
  // log(u) (x / (u - 1)) corrects the rounding of u = 1 + x
  float u = 1.0f + x;
  return __nv_logf(u) * (x / (u - 1.0f));
}

AGX_NV float __nv_powif(float x, int n) {
  float r = 1.0f, b = x;
  unsigned k = n < 0 ? -(unsigned)n : (unsigned)n;
  while (k) {
    if (k & 1) r *= b;
    b *= b;
    k >>= 1;
  }
  return n < 0 ? 1.0f / r : r;
}
AGX_NV double agx_log2_d(double x);
AGX_NV double agx_exp2_d(double x);
// pow through log2 in double precision (exact enough for every float)
AGX_NV float __nv_powf(float x, float y) {
  if (y == 0.0f || x == 1.0f) return 1.0f;
  if (x != x || y != y) return x + y;
  float ay = __builtin_fabsf(y);
  int yint = __builtin_truncf(y) == y;
  int yodd = yint && ay < 16777216.0f && ((long long)ay & 1);
  if (x == 0.0f) {
    if (y < 0.0f) return yodd ? __builtin_copysignf(__builtin_inff(), x) : __builtin_inff();
    return yodd ? x : 0.0f;
  }
  if (__nv_isinff(y)) {
    float ax = __builtin_fabsf(x);
    if (ax == 1.0f) return 1.0f;
    return (ax > 1.0f) == (y > 0.0f) ? __builtin_inff() : 0.0f;
  }
  if (x < 0.0f && !yint) return __builtin_nanf("");
  if (yint && ay <= 64.0f) return __nv_powif(x, (int)y);
  double l = agx_log2_d(__builtin_fabs((double)x)) * (double)y;
  float r = (float)agx_exp2_d(l);
  return (x < 0.0f && yodd) ? -r : r;
}

// sin and cos of x = q pi/2 + r, |r| <= pi/4; the quadrant q from x 2/pi,
// r = x - q pi/2 with pi/2 in three parts (exact for |q| < 2^12)
AGX_NV float agx_sin_poly_f(float r) {
  float r2 = r * r;
  float p = -2.50521084e-8f;  // -1/11!
  p = __builtin_fmaf(p, r2, 2.75573192e-6f);
  p = __builtin_fmaf(p, r2, -1.98412698e-4f);
  p = __builtin_fmaf(p, r2, 8.33333333e-3f);
  p = __builtin_fmaf(p, r2, -1.66666667e-1f);
  return __builtin_fmaf(p * r2, r, r);
}
AGX_NV float agx_cos_poly_f(float r) {
  float r2 = r * r;
  float p = 2.08767570e-9f;  // 1/12!
  p = __builtin_fmaf(p, r2, -2.75573192e-7f);
  p = __builtin_fmaf(p, r2, 2.48015873e-5f);
  p = __builtin_fmaf(p, r2, -1.38888889e-3f);
  p = __builtin_fmaf(p, r2, 4.16666667e-2f);
  p = __builtin_fmaf(p, r2, -0.5f);
  return __builtin_fmaf(p, r2, 1.0f);
}
// r and q of x (q mod 4); large |x| reduce in double precision
AGX_NV float agx_reduce_f(float x, int *q) {
  if (__builtin_fabsf(x) < 4096.0f) {
    float n = __builtin_rintf(x * 0.636619772f);
    float r = __builtin_fmaf(n, -1.5703125f, x);
    r = __builtin_fmaf(n, -4.83751297e-4f, r);
    r = __builtin_fmaf(n, -7.54978995e-8f, r);
    *q = (int)n & 3;
    return r;
  }
  double xd = x;
  double n = __builtin_rint(xd * 0.63661977236758134);
  double r = __builtin_fma(n, -1.5707963267948966, xd);
  r = __builtin_fma(n, -6.123233995736766e-17, r);
  *q = (int)(long long)__nv_fmod(n, 4.0) & 3;
  return (float)r;
}
AGX_NV void __nv_sincosf(float x, float *s, float *c) {
  if (__nv_isinff(x) || x != x) {
    *s = *c = __builtin_nanf("");
    return;
  }
  int q;
  float r = agx_reduce_f(x, &q);
  float sn = agx_sin_poly_f(r), cs = agx_cos_poly_f(r);
  switch (q) {
    case 0: *s = sn; *c = cs; break;
    case 1: *s = cs; *c = -sn; break;
    case 2: *s = -sn; *c = -cs; break;
    default: *s = -cs; *c = sn; break;
  }
}
AGX_NV float __nv_sinf(float x) {
  float s, c;
  __nv_sincosf(x, &s, &c);
  return s;
}
AGX_NV float __nv_cosf(float x) {
  float s, c;
  __nv_sincosf(x, &s, &c);
  return c;
}
AGX_NV float __nv_tanf(float x) {
  float s, c;
  __nv_sincosf(x, &s, &c);
  return s / c;
}
// sin(pi x) and cos(pi x): x = q/2 + d exactly, |d| <= 1/4
AGX_NV void __nv_sincospif(float x, float *s, float *c) {
  if (__nv_isinff(x) || x != x) {
    *s = *c = __builtin_nanf("");
    return;
  }
  float n = __builtin_rintf(2.0f * x);
  float d = __builtin_fmaf(n, -0.5f, x);
  float r = d * 3.14159265f;
  int q = (int)(long long)__nv_fmod(n, 4.0f) & 3;
  float sn = agx_sin_poly_f(r), cs = agx_cos_poly_f(r);
  switch (q) {
    case 0: *s = sn; *c = cs; break;
    case 1: *s = cs; *c = -sn; break;
    case 2: *s = -sn; *c = -cs; break;
    default: *s = -cs; *c = sn; break;
  }
  if (*s == 0.0f) *s = __builtin_copysignf(0.0f, x);
}
AGX_NV float __nv_sinpif(float x) {
  float s, c;
  __nv_sincospif(x, &s, &c);
  return s;
}
AGX_NV float __nv_cospif(float x) {
  float s, c;
  __nv_sincospif(x, &s, &c);
  return c;
}

// atan of |t| <= tan(pi/12) by its Taylor series
AGX_NV float agx_atan_poly_f(float t) {
  float t2 = t * t;
  float p = 1.0f / 13.0f;
  p = __builtin_fmaf(p, -t2, 1.0f / 11.0f);
  p = __builtin_fmaf(p, -t2, 1.0f / 9.0f);
  p = __builtin_fmaf(p, -t2, 1.0f / 7.0f);
  p = __builtin_fmaf(p, -t2, 0.2f);
  p = __builtin_fmaf(p, -t2, 1.0f / 3.0f);
  return __builtin_fmaf(-p * t2, t, t);
}
// atan x: |x| > 1 through pi/2 - atan(1/x), then x > tan(pi/12) through
// pi/6 + atan((x sqrt3 - 1) / (x + sqrt3))
AGX_NV float __nv_atanf(float x) {
  if (x != x) return x;
  float a = __builtin_fabsf(x), off = 0.0f;
  int inv = a > 1.0f;
  if (inv) a = 1.0f / a;
  if (a > 0.267949194f) {
    a = (a * 1.73205081f - 1.0f) / (a + 1.73205081f);
    off = 0.523598776f;
  }
  float r = off + agx_atan_poly_f(a);
  if (inv) r = 1.57079633f - r;
  return __builtin_copysignf(r, x);
}
AGX_NV float __nv_atan2f(float y, float x) {
  if (x != x || y != y) return x + y;
  float pi = 3.14159265f;
  if (y == 0.0f) return __nv_signbitf(x) ? __builtin_copysignf(pi, y) : y;
  if (x == 0.0f) return __builtin_copysignf(pi / 2.0f, y);
  if (__nv_isinff(x)) {
    if (__nv_isinff(y)) return __builtin_copysignf(x > 0.0f ? pi / 4.0f : 3.0f * pi / 4.0f, y);
    return x > 0.0f ? __builtin_copysignf(0.0f, y) : __builtin_copysignf(pi, y);
  }
  if (__nv_isinff(y)) return __builtin_copysignf(pi / 2.0f, y);
  float ax = __builtin_fabsf(x), ay = __builtin_fabsf(y);
  float r = ay <= ax ? __nv_atanf(ay / ax) : pi / 2.0f - __nv_atanf(ax / ay);
  if (x < 0.0f) r = pi - r;
  return __builtin_copysignf(r, y);
}
AGX_NV float __nv_asinf(float x) {
  if (__builtin_fabsf(x) > 1.0f) return __builtin_nanf("");
  return __nv_atan2f(x, __builtin_sqrtf((1.0f - x) * (1.0f + x)));
}
AGX_NV float __nv_acosf(float x) {
  if (__builtin_fabsf(x) > 1.0f) return __builtin_nanf("");
  return __nv_atan2f(__builtin_sqrtf((1.0f - x) * (1.0f + x)), x);
}
AGX_NV float __nv_sinhf(float x) {
  float a = __builtin_fabsf(x);
  if (a < 1.0f) {
    float e = __nv_expm1f(a);
    return __builtin_copysignf(0.5f * (e + e / (e + 1.0f)), x);
  }
  if (a > 89.0f) return __builtin_copysignf(__nv_expf(a - 0.693147181f), x);
  float e = __nv_expf(a);
  return __builtin_copysignf(0.5f * (e - 1.0f / e), x);
}
AGX_NV float __nv_coshf(float x) {
  float a = __builtin_fabsf(x);
  if (a > 89.0f) return __nv_expf(a - 0.693147181f);
  float e = __nv_expf(a);
  return 0.5f * (e + 1.0f / e);
}
AGX_NV float __nv_tanhf(float x) {
  float a = __builtin_fabsf(x);
  if (a > 9.0f) return __builtin_copysignf(1.0f, x);
  float e = __nv_expm1f(2.0f * a);
  return __builtin_copysignf(e / (e + 2.0f), x);
}
AGX_NV float __nv_asinhf(float x) {
  float a = __builtin_fabsf(x);
  if (a > 1e10f) return __builtin_copysignf(__nv_logf(a) + 0.693147181f, x);
  float a2 = a * a;
  return __builtin_copysignf(__nv_log1pf(a + a2 / (1.0f + __builtin_sqrtf(1.0f + a2))), x);
}
AGX_NV float __nv_acoshf(float x) {
  if (x < 1.0f) return __builtin_nanf("");
  if (x > 1e10f) return __nv_logf(x) + 0.693147181f;
  float t = x - 1.0f;
  return __nv_log1pf(t + __builtin_sqrtf(2.0f * t + t * t));
}
AGX_NV float __nv_atanhf(float x) {
  float a = __builtin_fabsf(x);
  if (a > 1.0f) return __builtin_nanf("");
  return __builtin_copysignf(0.5f * __nv_log1pf(2.0f * a / (1.0f - a)), x);
}
AGX_NV float __nv_hypotf(float x, float y) {
  if (__nv_isinff(x) || __nv_isinff(y)) return __builtin_inff();
  return (float)__builtin_sqrt((double)x * x + (double)y * y);
}
AGX_NV float __nv_rhypotf(float x, float y) { return 1.0f / __nv_hypotf(x, y); }
AGX_NV float __nv_cbrtf(float x) {
  if (x == 0.0f || x != x || __nv_isinff(x)) return x;
  float a = __builtin_fabsf(x);
  float r = (float)agx_exp2_d(agx_log2_d(a) / 3.0);
  // one Newton step
  r = r - (r * r * r - a) / (3.0f * r * r);
  return __builtin_copysignf(r, x);
}
AGX_NV float __nv_rcbrtf(float x) { return 1.0f / __nv_cbrtf(x); }
// Abramowitz and Stegun 7.1.26, below 0.5 the Taylor series
AGX_NV float __nv_erff(float x) {
  float a = __builtin_fabsf(x);
  if (a < 0.5f) {
    float x2 = x * x;
    float p = -1.0f / 1320.0f;
    p = __builtin_fmaf(p, x2, 1.0f / 216.0f);
    p = __builtin_fmaf(p, x2, -1.0f / 42.0f);
    p = __builtin_fmaf(p, x2, 0.1f);
    p = __builtin_fmaf(p, x2, -1.0f / 3.0f);
    p = __builtin_fmaf(p, x2, 1.0f);
    return x * p * 1.12837917f;
  }
  if (a > 4.0f) return __builtin_copysignf(1.0f, x);
  double t = 1.0 / (1.0 + 0.3275911 * a);
  double y = 1.0 - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t + 0.254829592) * t *
                       (double)__nv_expf(-a * a);
  return __builtin_copysignf((float)y, x);
}
AGX_NV float __nv_erfcf(float x) { return 1.0f - __nv_erff(x); }
AGX_NV float __nv_normcdff(float x) { return 0.5f * __nv_erfcf(-x * 0.707106781f); }

// ---- double precision elementary functions

AGX_NV double agx_exp2i_d(int n) {
  if (n < -1022) return AGX_U2D(1ull << (n + 1074));
  return AGX_U2D((unsigned long long)(n + 1023) << 52);
}
AGX_NV double agx_exp_poly_d(double r) {
  // e^r - 1 - r for |r| <= ln2/2, Taylor to r^13
  double p = 1.6059043836821613e-10;
  p = __builtin_fma(p, r, 2.08767569878681e-09);
  p = __builtin_fma(p, r, 2.505210838544172e-08);
  p = __builtin_fma(p, r, 2.755731922398589e-07);
  p = __builtin_fma(p, r, 2.7557319223985893e-06);
  p = __builtin_fma(p, r, 2.48015873015873e-05);
  p = __builtin_fma(p, r, 0.0001984126984126984);
  p = __builtin_fma(p, r, 0.001388888888888889);
  p = __builtin_fma(p, r, 0.008333333333333333);
  p = __builtin_fma(p, r, 0.041666666666666664);
  p = __builtin_fma(p, r, 0.16666666666666666);
  p = __builtin_fma(p, r, 0.5);
  return p * r * r;
}
AGX_NV double __nv_exp(double x) {
  if (x != x) return x;
  if (x > 709.782712893384) return __builtin_inf();
  if (x < -745.1332191019412) return 0.0;
  double n = __builtin_rint(x * 1.4426950408889634);
  double r = __builtin_fma(n, -0.6931471803691238, x);
  r = __builtin_fma(n, -1.9082149292705877e-10, r);
  double e = 1.0 + (r + agx_exp_poly_d(r));
  int k = (int)n;
  return (e * agx_exp2i_d(k / 2)) * agx_exp2i_d(k - k / 2);
}
AGX_NV double agx_exp2_d(double x) {
  if (x != x) return x;
  if (x >= 1024.0) return __builtin_inf();
  if (x < -1075.0) return 0.0;
  double n = __builtin_rint(x);
  double f = x - n;
  // r = f ln 2 in two parts
  double r = f * 0.6931471805599453;
  double rl = __builtin_fma(f, 0.6931471805599453, -r) + f * 2.3190468138462996e-17;
  double e = 1.0 + (r + (rl + agx_exp_poly_d(r)));
  int k = (int)n;
  return (e * agx_exp2i_d(k / 2)) * agx_exp2i_d(k - k / 2);
}
AGX_NV double __nv_exp2(double x) { return agx_exp2_d(x); }
AGX_NV double __nv_exp10(double x) {
  double hi = x * 2.302585092994046;
  double lo = __builtin_fma(x, 2.302585092994046, -hi) + x * -2.1707562233822494e-16;
  double e = __nv_exp(hi);
  return __builtin_fma(e, lo, e);
}
AGX_NV double __nv_expm1(double x) {
  if (__builtin_fabs(x) < 0.34657359027997264) return x + agx_exp_poly_d(x);
  if (x > 709.0) return __nv_exp(x);
  if (x < -38.0) return -1.0;
  double n = __builtin_rint(x * 1.4426950408889634);
  double r = __builtin_fma(n, -0.6931471803691238, x);
  r = __builtin_fma(n, -1.9082149292705877e-10, r);
  double t = r + agx_exp_poly_d(r);
  double s = agx_exp2i_d((int)n);
  return __builtin_fma(s, t, s - 1.0);
}
// log m = 2 atanh(s) = 2 s + 2 s^3 (1/3 + s^2/5 + ...), m in [sqrt(1/2), sqrt(2))
AGX_NV double agx_log_parts(double x, int *e) {
  double m = __nv_frexp(x, e) * 2.0;
  *e -= 1;
  if (m > 1.4142135623730951) {
    m *= 0.5;
    *e += 1;
  }
  return m;
}
AGX_NV double agx_atanh_tail(double s) {
  // 2 atanh(s) - 2 s
  double s2 = s * s;
  double p = 1.0 / 23.0;
  p = __builtin_fma(p, s2, 1.0 / 21.0);
  p = __builtin_fma(p, s2, 1.0 / 19.0);
  p = __builtin_fma(p, s2, 1.0 / 17.0);
  p = __builtin_fma(p, s2, 1.0 / 15.0);
  p = __builtin_fma(p, s2, 1.0 / 13.0);
  p = __builtin_fma(p, s2, 1.0 / 11.0);
  p = __builtin_fma(p, s2, 1.0 / 9.0);
  p = __builtin_fma(p, s2, 1.0 / 7.0);
  p = __builtin_fma(p, s2, 0.2);
  p = __builtin_fma(p, s2, 1.0 / 3.0);
  return 2.0 * s * s2 * p;
}
AGX_NV double __nv_log(double x) {
  if (x != x || x < 0.0) return __builtin_nan("");
  if (x == 0.0) return -__builtin_inf();
  if (__nv_isinfd(x)) return x;
  int e;
  double m = agx_log_parts(x, &e);
  double s = (m - 1.0) / (m + 1.0);
  double fe = (double)e;
  // 2 s with its rounding error: m - 1 = 2 s + s (m - 1)... computed as
  // (m - 1) - s (m - 1)
  double f = m - 1.0;
  double hs = f - s * f;  // = 2 s
  return __builtin_fma(fe, 0.6931471803691238, __builtin_fma(fe, 1.9082149292705877e-10, agx_atanh_tail(s)) + hs);
}
AGX_NV double agx_log2_d(double x) {
  if (x != x || x < 0.0) return __builtin_nan("");
  if (x == 0.0) return -__builtin_inf();
  if (__nv_isinfd(x)) return x;
  int e;
  double m = agx_log_parts(x, &e);
  double s = (m - 1.0) / (m + 1.0);
  double f = m - 1.0;
  double ln = (f - s * f) + agx_atanh_tail(s);
  return __builtin_fma(ln, 1.4426950408889634, (double)e);
}
AGX_NV double __nv_log2(double x) { return agx_log2_d(x); }
AGX_NV double __nv_log10(double x) { return __nv_log(x) * 0.4342944819032518; }
AGX_NV double __nv_log1p(double x) {
  if (x > -0.29 && x < 0.41) {
    double s = x / (2.0 + x);
    return __builtin_fma(-s, x, x) + agx_atanh_tail(s);
  }
  double u = 1.0 + x;
  return __nv_log(u) * (x / (u - 1.0));
}
AGX_NV double __nv_powi(double x, int n) {
  double r = 1.0, b = x;
  unsigned k = n < 0 ? -(unsigned)n : (unsigned)n;
  while (k) {
    if (k & 1) r *= b;
    b *= b;
    k >>= 1;
  }
  return n < 0 ? 1.0 / r : r;
}
// pow = 2^(y log2 x) with log2 x in two parts (hi + lo) so y log2 x keeps
// about 64 bits
AGX_NV double __nv_pow(double x, double y) {
  if (y == 0.0 || x == 1.0) return 1.0;
  if (x != x || y != y) return x + y;
  double ay = __builtin_fabs(y);
  int yint = __builtin_trunc(y) == y;
  int yodd = yint && ay < 9007199254740992.0 && ((long long)ay & 1);
  if (x == 0.0) {
    if (y < 0.0) return yodd ? __builtin_copysign(__builtin_inf(), x) : __builtin_inf();
    return yodd ? x : 0.0;
  }
  if (__nv_isinfd(y)) {
    double ax = __builtin_fabs(x);
    if (ax == 1.0) return 1.0;
    return (ax > 1.0) == (y > 0.0) ? __builtin_inf() : 0.0;
  }
  if (x < 0.0 && !yint) return __builtin_nan("");
  if (yint && ay <= 32.0) return __nv_powi(x, (int)y);
  int e;
  double m = agx_log_parts(__builtin_fabs(x), &e);
  // s = (m - 1) / (m + 1) in two parts: f = m - 1 is exact, d = m + 1 is
  // dh + dl
  double f = m - 1.0;
  double dh = m + 1.0;
  double dl = (m - (dh - 1.0)) + (1.0 - (dh - (dh - 1.0)));
  double sh = f / dh;
  double sl = (__builtin_fma(-sh, dh, f) - sh * dl) / dh;
  // ln m = 2 sh + (2 sl + tail) in double-double, then log2 x = e + ln m / ln 2
  double t = 2.0 * sl + agx_atanh_tail(sh);
  double hs = 2.0 * sh;
  double ln = hs + t;
  double lnl = (hs - ln) + t;
  double l2 = ln * 1.4426950408889634;
  double l2l = __builtin_fma(ln, 1.4426950408889634, -l2) + (lnl * 1.4426950408889634 + ln * 2.0355273740931033e-17);
  double hi = (double)e + l2;
  double lo = ((double)e - hi) + l2 + l2l;
  double p = y * hi;
  double pl = __builtin_fma(y, hi, -p) + y * lo;
  if (p > 1100.0) return yodd && x < 0.0 ? -__builtin_inf() : __builtin_inf();
  if (p < -1100.0) return yodd && x < 0.0 ? -0.0 : 0.0;
  double n = __builtin_rint(p);
  double r = (p - n) + pl;
  double v = agx_exp2_d(r);
  int k = (int)n;
  v = (v * agx_exp2i_d(k / 2)) * agx_exp2i_d(k - k / 2);
  return (x < 0.0 && yodd) ? -v : v;
}

AGX_NV double agx_sin_poly_d(double r) {
  double r2 = r * r;
  double p = 2.8114572543455206e-15;
  p = __builtin_fma(p, r2, -7.647163731819816e-13);
  p = __builtin_fma(p, r2, 1.6059043836821613e-10);
  p = __builtin_fma(p, r2, -2.505210838544172e-08);
  p = __builtin_fma(p, r2, 2.7557319223985893e-06);
  p = __builtin_fma(p, r2, -0.0001984126984126984);
  p = __builtin_fma(p, r2, 0.008333333333333333);
  p = __builtin_fma(p, r2, -0.16666666666666666);
  return __builtin_fma(p * r2, r, r);
}
AGX_NV double agx_cos_poly_d(double r) {
  double r2 = r * r;
  double p = -1.5619206968586225e-16;
  p = __builtin_fma(p, r2, 4.779477332387385e-14);
  p = __builtin_fma(p, r2, -1.1470745597729725e-11);
  p = __builtin_fma(p, r2, 2.08767569878681e-09);
  p = __builtin_fma(p, r2, -2.755731922398589e-07);
  p = __builtin_fma(p, r2, 2.48015873015873e-05);
  p = __builtin_fma(p, r2, -0.001388888888888889);
  p = __builtin_fma(p, r2, 0.041666666666666664);
  p = __builtin_fma(p, r2, -0.5);
  return __builtin_fma(p, r2, 1.0);
}
// x = n pi/2 + r with pi/2 in three parts of 33, 33 and 53 bits (exact
// products for |n| < 2^20)
AGX_NV double agx_reduce_d(double x, int *q) {
  double n = __builtin_rint(x * 0.63661977236758134);
  double r = __builtin_fma(n, -1.5707963267341256, x);
  r = __builtin_fma(n, -6.077100506303966e-11, r);
  r = __builtin_fma(n, -2.0222662487959506e-21, r);
  *q = (int)(long long)__nv_fmod(n, 4.0) & 3;
  return r;
}
AGX_NV void __nv_sincos(double x, double *s, double *c) {
  if (__nv_isinfd(x) || x != x) {
    *s = *c = __builtin_nan("");
    return;
  }
  int q;
  double r = agx_reduce_d(x, &q);
  double sn = agx_sin_poly_d(r), cs = agx_cos_poly_d(r);
  switch (q) {
    case 0: *s = sn; *c = cs; break;
    case 1: *s = cs; *c = -sn; break;
    case 2: *s = -sn; *c = -cs; break;
    default: *s = -cs; *c = sn; break;
  }
}
AGX_NV double __nv_sin(double x) {
  double s, c;
  __nv_sincos(x, &s, &c);
  return s;
}
AGX_NV double __nv_cos(double x) {
  double s, c;
  __nv_sincos(x, &s, &c);
  return c;
}
AGX_NV double __nv_tan(double x) {
  double s, c;
  __nv_sincos(x, &s, &c);
  return s / c;
}
AGX_NV void __nv_sincospi(double x, double *s, double *c) {
  if (__nv_isinfd(x) || x != x) {
    *s = *c = __builtin_nan("");
    return;
  }
  double n = __builtin_rint(2.0 * x);
  double d = __builtin_fma(n, -0.5, x);
  double r = d * 3.141592653589793;
  int q = (int)(long long)__nv_fmod(n, 4.0) & 3;
  double sn = agx_sin_poly_d(r), cs = agx_cos_poly_d(r);
  switch (q) {
    case 0: *s = sn; *c = cs; break;
    case 1: *s = cs; *c = -sn; break;
    case 2: *s = -sn; *c = -cs; break;
    default: *s = -cs; *c = sn; break;
  }
  if (*s == 0.0) *s = __builtin_copysign(0.0, x);
}
AGX_NV double __nv_sinpi(double x) {
  double s, c;
  __nv_sincospi(x, &s, &c);
  return s;
}
AGX_NV double __nv_cospi(double x) {
  double s, c;
  __nv_sincospi(x, &s, &c);
  return c;
}
// atan of |t| <= tan(pi/24) (0.1317) by its Taylor series to t^31
AGX_NV double agx_atan_poly_d(double t) {
  double t2 = t * t;
  double p = 1.0 / 31.0;
  for (int k = 29; k >= 3; k -= 2) p = __builtin_fma(p, -t2, 1.0 / k);
  return __builtin_fma(-p * t2, t, t);
}
// as atanf, with the reduction by pi/6 and then a halving of the angle
AGX_NV double __nv_atan(double x) {
  if (x != x) return x;
  double a = __builtin_fabs(x), off = 0.0;
  int inv = a > 1.0;
  if (inv) a = 1.0 / a;
  if (a > 0.2679491924311227) {
    a = (a * 1.7320508075688772 - 1.0) / (a + 1.7320508075688772);
    off = 0.5235987755982989;
  }
  // atan a = 2 atan(a / (1 + sqrt(1 + a^2))), |a| <= tan(pi/24) after
  double h = a / (1.0 + __builtin_sqrt(__builtin_fma(a, a, 1.0)));
  double r = off + 2.0 * agx_atan_poly_d(h);
  if (inv) r = 1.5707963267948966 - r;
  return __builtin_copysign(r, x);
}
AGX_NV double __nv_atan2(double y, double x) {
  if (x != x || y != y) return x + y;
  double pi = 3.141592653589793;
  if (y == 0.0) return __nv_signbitd(x) ? __builtin_copysign(pi, y) : y;
  if (x == 0.0) return __builtin_copysign(pi / 2.0, y);
  if (__nv_isinfd(x)) {
    if (__nv_isinfd(y)) return __builtin_copysign(x > 0.0 ? pi / 4.0 : 3.0 * pi / 4.0, y);
    return x > 0.0 ? __builtin_copysign(0.0, y) : __builtin_copysign(pi, y);
  }
  if (__nv_isinfd(y)) return __builtin_copysign(pi / 2.0, y);
  double ax = __builtin_fabs(x), ay = __builtin_fabs(y);
  double r = ay <= ax ? __nv_atan(ay / ax) : pi / 2.0 - __nv_atan(ax / ay);
  if (x < 0.0) r = pi - r;
  return __builtin_copysign(r, y);
}
AGX_NV double __nv_asin(double x) {
  if (__builtin_fabs(x) > 1.0) return __builtin_nan("");
  return __nv_atan2(x, __builtin_sqrt((1.0 - x) * (1.0 + x)));
}
AGX_NV double __nv_acos(double x) {
  if (__builtin_fabs(x) > 1.0) return __builtin_nan("");
  return __nv_atan2(__builtin_sqrt((1.0 - x) * (1.0 + x)), x);
}
AGX_NV double __nv_sinh(double x) {
  double a = __builtin_fabs(x);
  if (a < 1.0) {
    double e = __nv_expm1(a);
    return __builtin_copysign(0.5 * (e + e / (e + 1.0)), x);
  }
  if (a > 709.0) return __builtin_copysign(__nv_exp(a - 0.6931471805599453), x);
  double e = __nv_exp(a);
  return __builtin_copysign(0.5 * (e - 1.0 / e), x);
}
AGX_NV double __nv_cosh(double x) {
  double a = __builtin_fabs(x);
  if (a > 709.0) return __nv_exp(a - 0.6931471805599453);
  double e = __nv_exp(a);
  return 0.5 * (e + 1.0 / e);
}
AGX_NV double __nv_tanh(double x) {
  double a = __builtin_fabs(x);
  if (a > 19.0) return __builtin_copysign(1.0, x);
  double e = __nv_expm1(2.0 * a);
  return __builtin_copysign(e / (e + 2.0), x);
}
AGX_NV double __nv_asinh(double x) {
  double a = __builtin_fabs(x);
  if (a > 1e150) return __builtin_copysign(__nv_log(a) + 0.6931471805599453, x);
  double a2 = a * a;
  return __builtin_copysign(__nv_log1p(a + a2 / (1.0 + __builtin_sqrt(1.0 + a2))), x);
}
AGX_NV double __nv_acosh(double x) {
  if (x < 1.0) return __builtin_nan("");
  if (x > 1e150) return __nv_log(x) + 0.6931471805599453;
  double t = x - 1.0;
  return __nv_log1p(t + __builtin_sqrt(2.0 * t + t * t));
}
AGX_NV double __nv_atanh(double x) {
  double a = __builtin_fabs(x);
  if (a > 1.0) return __builtin_nan("");
  return __builtin_copysign(0.5 * __nv_log1p(2.0 * a / (1.0 - a)), x);
}
AGX_NV double __nv_hypot(double x, double y) {
  if (__nv_isinfd(x) || __nv_isinfd(y)) return __builtin_inf();
  double ax = __builtin_fabs(x), ay = __builtin_fabs(y);
  double m = ax > ay ? ax : ay, n = ax > ay ? ay : ax;
  if (m == 0.0) return 0.0;
  double r = n / m;
  return m * __builtin_sqrt(__builtin_fma(r, r, 1.0));
}
AGX_NV double __nv_rhypot(double x, double y) { return 1.0 / __nv_hypot(x, y); }
AGX_NV double __nv_cbrt(double x) {
  if (x == 0.0 || x != x || __nv_isinfd(x)) return x;
  double a = __builtin_fabs(x);
  double r = agx_exp2_d(agx_log2_d(a) / 3.0);
  r = r - (r * r * r - a) / (3.0 * r * r);
  return __builtin_copysign(r, x);
}
AGX_NV double __nv_rcbrt(double x) { return 1.0 / __nv_cbrt(x); }
// erf: the Taylor series below 2.5, erfc's continued fraction above
AGX_NV double agx_erfc_cf(double a) {
  // erfc a = e^-a^2 / sqrt(pi) * 1/(a + (1/2)/(a + 1/(a + (3/2)/(a + ...)))), from the back
  double f = a;
  for (int k = 60; k >= 1; k--) f = a + (0.5 * k) / f;
  // e^-a^2 with a^2 = sq + sl exactly
  double sq = a * a, sl = __builtin_fma(a, a, -sq);
  double e = __nv_exp(-sq);
  return __builtin_fma(e, -sl, e) / (f * 1.7724538509055160);
}
AGX_NV double __nv_erf(double x) {
  double a = __builtin_fabs(x);
  if (a < 2.5) {
    double x2 = x * x, term = x, sum = x;
    for (int n = 1; n < 60; n++) {
      term *= -x2 / n;
      double add = term / (2 * n + 1);
      sum += add;
      if (__builtin_fabs(add) < 1e-17 * __builtin_fabs(sum)) break;
    }
    return sum * 1.1283791670955126;
  }
  if (a > 6.0) return __builtin_copysign(1.0, x);
  return __builtin_copysign(1.0 - agx_erfc_cf(a), x);
}
AGX_NV double __nv_erfc(double x) {
  if (x < 2.5) return 1.0 - __nv_erf(x);
  if (x > 27.3) return 0.0;
  return agx_erfc_cf(x);
}
AGX_NV double __nv_normcdf(double x) { return 0.5 * __nv_erfc(-x * 0.7071067811865476); }
