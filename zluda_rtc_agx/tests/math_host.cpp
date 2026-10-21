// Checks agx_nvrtc_math.h on the host against libm: the worst error in ulp
// of each function over sampled arguments. Built and run by tests/math.rs.
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#define AGX_NV extern "C" inline
#include "../src/include/agx_nvrtc_math.h"

static double ulp_f(float got, double want) {
  if (std::isnan(want)) return std::isnan(got) ? 0 : 1e9;
  if (std::isinf(want)) return got == (float)want ? 0 : 1e9;
  float w = (float)want;
  double u = std::fabs(std::nextafterf(w, INFINITY) - w);
  if (u == 0 || std::isinf(u)) u = 1.4e-45;
  return std::fabs((double)got - want) / u;
}
static double ulp_d(double got, long double want) {
  if (std::isnan((double)want)) return std::isnan(got) ? 0 : 1e9;
  if (std::isinf((double)want)) return got == (double)want ? 0 : 1e9;
  double w = (double)want;
  double u = std::fabs(std::nextafter(w, INFINITY) - w);
  if (u == 0) u = 4.9e-324;
  return (double)(std::fabs((long double)got - want) / u);
}

static unsigned long long rng = 88172645463325252ull;
static double uniform(double lo, double hi) {
  rng ^= rng << 13;
  rng ^= rng >> 7;
  rng ^= rng << 17;
  return lo + (hi - lo) * (double)(rng >> 11) / 9007199254740992.0;
}

// sin(pi x) and cos(pi x) in long double with the same exact reduction
static long double sinpi_ref(long double x, int cosine) {
  long double n = rintl(2 * x), d = x - n / 2;
  const long double pi = 3.14159265358979323846264338327950288L;
  long double sn = sinl(pi * d), cs = cosl(pi * d);
  int q = (int)fmodl(n, 4.0L);
  q &= 3;
  long double s = q == 0 ? sn : q == 1 ? cs : q == 2 ? -sn : -cs;
  long double c = q == 0 ? cs : q == 1 ? -sn : q == 2 ? -cs : sn;
  return cosine ? c : s;
}

int failures = 0;
#define CHECK_F(name, expr, ref, lo, hi, limit)                                    \
  do {                                                                            \
    double worst = 0, at = 0;                                                     \
    for (int i = 0; i < 200000; i++) {                                            \
      float x = (float)uniform(lo, hi);                                          \
      double e = ulp_f(expr, ref);                                               \
      if (e > worst) worst = e, at = x;                                          \
    }                                                                             \
    printf("%-10s [%g, %g] %8.2f ulp at %g%s\n", name, (double)lo, (double)hi,   \
           worst, at, worst > limit ? "  FAIL" : "");                           \
    failures += worst > limit;                                                    \
  } while (0)
#define CHECK_D(name, expr, ref, lo, hi, limit)                                    \
  do {                                                                            \
    double worst = 0, at = 0;                                                     \
    for (int i = 0; i < 200000; i++) {                                            \
      double x = uniform(lo, hi);                                                \
      double e = ulp_d(expr, ref);                                               \
      if (e > worst) worst = e, at = x;                                          \
    }                                                                             \
    printf("%-10s [%g, %g] %8.2f ulp at %.17g%s\n", name, (double)lo, (double)hi, \
           worst, at, worst > limit ? "  FAIL" : "");                           \
    failures += worst > limit;                                                    \
  } while (0)

int main() {
  CHECK_F("expf", __nv_expf(x), std::exp((double)x), -100, 88, 2);
  CHECK_F("exp2f", __nv_exp2f(x), std::exp2((double)x), -140, 127, 2);
  CHECK_F("exp10f", __nv_exp10f(x), std::pow(10.0, (double)x), -37, 38, 3);
  CHECK_F("expm1f", __nv_expm1f(x), std::expm1((double)x), -5, 5, 3);
  CHECK_F("logf", __nv_logf(x), std::log((double)x), 1e-30, 1e30, 2);
  CHECK_F("logf", __nv_logf(x), std::log((double)x), 0.5, 2, 2);
  CHECK_F("log2f", __nv_log2f(x), std::log2((double)x), 1e-30, 1e30, 2);
  CHECK_F("log2f", __nv_log2f(x), std::log2((double)x), 0.5, 2, 3);
  CHECK_F("log10f", __nv_log10f(x), std::log10((double)x), 1e-30, 1e30, 3);
  CHECK_F("log1pf", __nv_log1pf(x), std::log1p((double)x), -0.9, 10, 3);
  CHECK_F("sinf", __nv_sinf(x), std::sin((double)x), -10, 10, 2);
  CHECK_F("sinf", __nv_sinf(x), std::sin((double)x), -1e5, 1e5, 3);
  CHECK_F("cosf", __nv_cosf(x), std::cos((double)x), -10, 10, 2);
  CHECK_F("tanf", __nv_tanf(x), std::tan((double)x), -1.5, 1.5, 4);
  CHECK_F("sinpif", __nv_sinpif(x), (double)sinpi_ref(x, 0), -100, 100, 3);
  CHECK_F("cospif", __nv_cospif(x), (double)sinpi_ref(x, 1), -100, 100, 3);
  CHECK_F("atanf", __nv_atanf(x), std::atan((double)x), -20, 20, 3);
  CHECK_F("atan2f", __nv_atan2f(x, 0.7f), std::atan2((double)x, 0.7), -20, 20, 3);
  CHECK_F("asinf", __nv_asinf(x), std::asin((double)x), -1, 1, 3);
  CHECK_F("acosf", __nv_acosf(x), std::acos((double)x), -1, 1, 3);
  CHECK_F("sinhf", __nv_sinhf(x), std::sinh((double)x), -20, 20, 3);
  CHECK_F("coshf", __nv_coshf(x), std::cosh((double)x), -20, 20, 3);
  CHECK_F("tanhf", __nv_tanhf(x), std::tanh((double)x), -10, 10, 3);
  CHECK_F("asinhf", __nv_asinhf(x), std::asinh((double)x), -100, 100, 3);
  CHECK_F("acoshf", __nv_acoshf(x), std::acosh((double)x), 1, 100, 3);
  CHECK_F("atanhf", __nv_atanhf(x), std::atanh((double)x), -0.99, 0.99, 3);
  CHECK_F("powf", __nv_powf(x, 2.7f), std::pow((double)x, (double)2.7f), 0, 100, 2);
  CHECK_F("powf", __nv_powf(1.3f, x), std::pow((double)1.3f, (double)x), -300, 300, 2);
  CHECK_F("cbrtf", __nv_cbrtf(x), std::cbrt((double)x), -1e6, 1e6, 2);
  CHECK_F("hypotf", __nv_hypotf(x, 3.0f), std::hypot((double)x, 3.0), -1e3, 1e3, 1);
  CHECK_F("erff", __nv_erff(x), std::erf((double)x), -0.5, 0.5, 4);
  CHECK_F("fmodf", __nv_fmodf(x, 0.37f), std::fmod((double)x, (double)0.37f), -1e4, 1e4, 0);
  CHECK_F("remainf", __nv_remainderf(x, 0.37f), std::remainder((double)x, (double)0.37f), -1e4, 1e4, 0);
  {
    double worst = 0;
    for (int i = 0; i < 200000; i++) {
      float x = (float)uniform(-4, 4);
      worst = std::fmax(worst, std::fabs(__nv_erff(x) - std::erf((double)x)));
    }
    printf("%-10s abs error %g%s\n", "erff", worst, worst > 1e-6 ? "  FAIL" : "");
    failures += worst > 1e-6;
  }
  CHECK_D("exp", __nv_exp(x), std::exp((long double)x), -700, 700, 2);
  CHECK_D("exp2", __nv_exp2(x), std::exp2((long double)x), -1000, 1000, 2);
  CHECK_D("exp10", __nv_exp10(x), std::pow(10.0L, (long double)x), -300, 300, 3);
  CHECK_D("expm1", __nv_expm1(x), std::expm1((long double)x), -5, 5, 2);
  CHECK_D("log", __nv_log(x), std::log((long double)x), 1e-300, 1e300, 2);
  CHECK_D("log", __nv_log(x), std::log((long double)x), 0.5, 2, 2);
  CHECK_D("log2", __nv_log2(x), std::log2((long double)x), 0.5, 2, 3);
  CHECK_D("log10", __nv_log10(x), std::log10((long double)x), 1e-300, 1e300, 3);
  CHECK_D("log1p", __nv_log1p(x), std::log1p((long double)x), -0.9, 10, 3);
  CHECK_D("sin", __nv_sin(x), std::sin((long double)x), -10, 10, 2);
  CHECK_D("sin", __nv_sin(x), std::sin((long double)x), -1e5, 1e5, 3);
  CHECK_D("cos", __nv_cos(x), std::cos((long double)x), -10, 10, 2);
  CHECK_D("tan", __nv_tan(x), std::tan((long double)x), -1.5, 1.5, 4);
  CHECK_D("sinpi", __nv_sinpi(x), sinpi_ref(x, 0), -100, 100, 3);
  CHECK_D("atan", __nv_atan(x), std::atan((long double)x), -20, 20, 3);
  CHECK_D("atan2", __nv_atan2(x, 0.7), std::atan2((long double)x, (long double)0.7), -20, 20, 3);
  CHECK_D("asin", __nv_asin(x), std::asin((long double)x), -1, 1, 4);
  CHECK_D("acos", __nv_acos(x), std::acos((long double)x), -1, 1, 4);
  CHECK_D("sinh", __nv_sinh(x), std::sinh((long double)x), -20, 20, 3);
  CHECK_D("cosh", __nv_cosh(x), std::cosh((long double)x), -20, 20, 3);
  CHECK_D("tanh", __nv_tanh(x), std::tanh((long double)x), -10, 10, 3);
  CHECK_D("asinh", __nv_asinh(x), std::asinh((long double)x), -100, 100, 3);
  CHECK_D("acosh", __nv_acosh(x), std::acosh((long double)x), 1, 100, 3);
  CHECK_D("atanh", __nv_atanh(x), std::atanh((long double)x), -0.99, 0.99, 3);
  CHECK_D("pow", __nv_pow(x, 2.7), std::pow((long double)x, (long double)2.7), 0, 100, 2);
  CHECK_D("pow", __nv_pow(1.3, x), std::pow((long double)1.3, (long double)x), -2000, 2000, 3);
  CHECK_D("cbrt", __nv_cbrt(x), std::cbrt((long double)x), -1e6, 1e6, 2);
  CHECK_D("hypot", __nv_hypot(x, 3.0), std::hypot((long double)x, 3.0L), -1e3, 1e3, 2);
  CHECK_D("erf", __nv_erf(x), std::erf((long double)x), -6, 6, 100);
  CHECK_D("erfc", __nv_erfc(x), std::erfc((long double)x), 2.5, 26, 64);
  CHECK_D("fmod", __nv_fmod(x, 0.37), std::fmod((long double)x, (long double)0.37), -1e4, 1e4, 0);
  CHECK_D("remain", __nv_remainder(x, 0.37), std::remainder((long double)x, (long double)0.37), -1e4, 1e4, 0);
  {
    int bad = 0, e1, e2;
    for (int i = 0; i < 100000; i++) {
      double x = uniform(-1, 1) * std::pow(2.0, uniform(-1070, 1020));
      double m1 = __nv_frexp(x, &e1), m2 = std::frexp(x, &e2);
      bad += m1 != m2 || e1 != e2 || __nv_ldexp(m1, e1) != x;
      float xf = (float)x;
      float n1 = __nv_frexpf(xf, &e1), n2 = std::frexp(xf, &e2);
      bad += xf != 0 && (n1 != n2 || e1 != e2 || __nv_ldexpf(n1, e1) != xf);
      bad += __nv_roundf(xf) != std::round(xf) || __nv_round(x) != std::round(x);
      bad += __nv_nextafter(x, 1.0) != std::nextafter(x, 1.0);
    }
    printf("frexp/ldexp/round/nextafter mismatches %d%s\n", bad, bad ? "  FAIL" : "");
    failures += bad != 0;
  }
  printf("%d failures\n", failures);
  return failures != 0;
}
