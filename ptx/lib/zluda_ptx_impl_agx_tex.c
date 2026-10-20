// Texture and surface instructions for AGX, in software (AGX's own texture
// unit is not used yet: its descriptors are not known).
//
// The texture, sampler and surface objects that CUDA handles point to are
// descriptors in device memory in the encoding of PTX-formalization's model
// (semantics/ptx-exec-tex.k), which zluda_agx's cuTexObjectCreate /
// cuSurfObjectCreate write, and this code follows the model's sampling rules:
// coordinates scale by the extent when normalized, linear filtering weights
// are quantised to 1/256, products and sums are exact (double here) and
// rounded once, mipmap levels are stored one after the other, cubemaps are six
// consecutive layers (+X -X +Y -Y +Z -Z, OpenGL face rules, clamped inside the
// face), depth compare is LEQUAL per texel before filtering.
//
// Compiled to LLVM IR by ptx/lib/gen_agx_tex.py (clang -target nvptx64) into
// the generated section of zluda_ptx_impl_agx.ll; regenerate after editing.

#include <stdint.h>

#define GLOBAL __attribute__((address_space(1)))
#define API __attribute__((always_inline))

// The loops below are not unrolled: each tex instruction becomes one copy of the texel fetch (all its
// formats) and of the software double precision arithmetic, instead of up to 16.
#define NO_UNROLL _Pragma("clang loop unroll(disable)")

typedef const GLOBAL uint64_t *desc_t;

// ---------------------------------------------------------------- descriptors

enum { TEX_MAGIC = 0x54585854, SAMPLER_MAGIC = 0x5343504d, SURF_MAGIC = 0x53545446 };

// texture: 1 base, 2 type code, 3 channels, 4 filter, 5 address mode,
// 6 normalized, 7 read-normalized, 8 layers, 9..11 dims, 12 levels,
// 13 flags (bit 0 depth, bit 1 cube), 14 row pitch (0: dense; not in the model), 15 mip filter
// sampler: 1 filter, 2 address mode, 3 normalized, 4 mip filter, 5 min lod,
// 6 max lod (1/256 units, signed)
// surface: 1 base, 2 texel bytes, 3 layers, 4..6 dims, 7 sample format,
// 8 channels, 9 channel data type, 10 channel order, 11 memory layout,
// 12 array flag

static inline int64_t imax(int64_t a, int64_t b) { return a > b ? a : b; }
static inline int64_t imin(int64_t a, int64_t b) { return a < b ? a : b; }
static inline double dmax(double a, double b) { return a >= b ? a : b; }
static inline double dmin(double a, double b) { return a <= b ? a : b; }

static inline uint64_t tw(desc_t d, int i) { return d[i]; }
static inline int64_t tx_layers(desc_t t) { return imax(1, (int64_t)tw(t, 8)); }
static inline int64_t tx_dim(desc_t t, int i) { return imax(1, (int64_t)tw(t, 9 + i)); }
static inline int64_t tx_levels(desc_t t) { return imax(1, (int64_t)tw(t, 12)); }
static inline int64_t lv_dim(desc_t t, int lv, int i) { return imax(1, tx_dim(t, i) >> lv); }

static inline int64_t lv_texels(desc_t t, int lv) {
    return tx_layers(t) * lv_dim(t, lv, 0) * lv_dim(t, lv, 1) * lv_dim(t, lv, 2);
}

static inline int64_t lv_off(desc_t t, int lv) {
    int64_t off = 0;
    for (int i = 0; i < lv; i++)
        off += lv_texels(t, i);
    return off;
}

static inline int code_bytes(uint64_t code) {
    switch (code) {
    case 0:
    case 20:
        return 1;
    case 1:
    case 21:
    case 6:
        return 2;
    default:
        return 4;
    }
}

static inline int code_bits(uint64_t code) { return 8 * code_bytes(code); }
static inline int code_signed(uint64_t code) { return code == 20 || code == 21 || code == 3 || code == 5; }

// the sampler overrides filter, address mode and normalization
struct sampling {
    int filter, addr, norm, mip;
    double min_lod, max_lod;
};

static inline struct sampling sampling_of(desc_t t, desc_t s) {
    struct sampling r;
    if (s) {
        r.filter = (int)tw(s, 1);
        r.addr = (int)tw(s, 2);
        r.norm = (int)tw(s, 3);
        r.mip = (int)tw(s, 4);
        r.min_lod = (double)(int64_t)tw(s, 5) / 256.0;
        r.max_lod = (double)(int64_t)tw(s, 6) / 256.0;
    } else {
        r.filter = (int)tw(t, 4);
        r.addr = (int)tw(t, 5);
        r.norm = (int)tw(t, 6);
        r.mip = (int)tw(t, 15);
        r.min_lod = 0.0;
        r.max_lod = 1048576.0 / 256.0;
    }
    return r;
}

// ---------------------------------------------------------------- texels

static inline double half_to_double(uint32_t h) {
    uint32_t s = h >> 15, e = (h >> 10) & 31, m = h & 1023;
    double v;
    if (e == 0)
        v = (double)m / 16777216.0;
    else if (e == 31)
        v = m ? __builtin_nan("") : __builtin_inf();
    else {
        v = (double)(m | 1024);
        int x = (int)e - 25;
        while (x > 0) {
            v *= 2.0;
            x--;
        }
        while (x < 0) {
            v *= 0.5;
            x++;
        }
    }
    return s ? -v : v;
}

static inline double bits_to_float(uint32_t b) {
    union {
        uint32_t u;
        float f;
    } c = {b};
    return (double)c.f;
}

static inline uint32_t float_bits(float f) {
    union {
        float f;
        uint32_t u;
    } c = {f};
    return c.u;
}

// a channel of a texel, as an exact value
static inline double channel(desc_t t, uint64_t addr) {
    uint64_t code = tw(t, 2);
    int bytes = code_bytes(code);
    const GLOBAL uint8_t *p = (const GLOBAL uint8_t *)addr;
    uint32_t v = 0;
    for (int i = 0; i < bytes; i++)
        v |= (uint32_t)p[i] << (8 * i);
    if (code == 6)
        return half_to_double(v);
    if (code == 7)
        return bits_to_float(v);
    int bits = code_bits(code);
    int64_t iv = v;
    if (code_signed(code) && bits < 64 && (v >> (bits - 1)) & 1)
        iv = (int64_t)v - ((int64_t)1 << bits);
    if (!tw(t, 7))
        return (double)iv;
    int64_t max = code_signed(code) ? ((int64_t)1 << (bits - 1)) - 1 : ((int64_t)1 << bits) - 1;
    return dmax((double)iv / (double)max, -1.0);
}

struct q {
    int lv;
    int off[3];
    int has_ref;
    uint32_t ref;
};

// a texel or filtered value (a vector: values in registers, not in memory)
typedef double rgba __attribute__((ext_vector_type(4)));

static inline rgba texel0(desc_t t, int lv, int64_t i, int64_t j, int64_t k, int64_t l) {
    rgba r = {0, 0, 0, 0};
    if (i < 0 || j < 0 || k < 0)
        return r;
    int64_t ch = (int64_t)tw(t, 3);
    int bytes = code_bytes(tw(t, 2));
    // word 14 (zluda_agx's extension): the row pitch in bytes of a pitch-linear texture, 0 for dense
    int64_t pitch = (int64_t)tw(t, 14);
    int64_t idx = lv_off(t, lv) + ((l * lv_dim(t, lv, 2) + k) * lv_dim(t, lv, 1) + j) * lv_dim(t, lv, 0) + i;
    uint64_t a = pitch ? tw(t, 1) + (uint64_t)((((l * lv_dim(t, lv, 2) + k) * lv_dim(t, lv, 1) + j) * pitch) + i * ch * bytes)
                       : tw(t, 1) + (uint64_t)(idx * ch * bytes);
    NO_UNROLL
    for (int c = 0; c < 4; c++)
        r[c] = c < ch ? channel(t, a + (uint64_t)(c * bytes)) : (c == 3 ? 1.0 : 0.0);
    return r;
}

static inline rgba texel(desc_t t, struct q *q, int64_t i, int64_t j, int64_t k, int64_t l) {
    rgba r = texel0(t, q->lv, i, j, k, l);
    if (!q->has_ref)
        return r;
    rgba d = {bits_to_float(q->ref) <= r[0] ? 1.0 : 0.0, 0, 0, 1.0};
    return d;
}

// ---------------------------------------------------------------- coordinates

static inline int64_t floor_i(double x) { return (int64_t)__builtin_floor(x); }

static inline int64_t mod_pos(int64_t a, int64_t n) { return ((a % n) + n) % n; }

// index with the address mode applied (-1: border)
static inline int64_t ad_idx(int mode, int64_t i, int64_t n) {
    switch (mode) {
    case 0:
        return mod_pos(i, n);
    case 2:
        return i >= 0 && i < n ? i : -1;
    case 3: {
        int64_t m = mod_pos(i, 2 * n);
        return m < n ? m : 2 * n - m - 1;
    }
    default:
        return imin(imax(i, 0), n - 1);
    }
}

static inline double pos(double x, int64_t n, int norm) { return norm == 1 ? x * (double)n : x; }

// the weight of the upper texel, quantised to 1/256
static inline double qw(double xb) { return (double)floor_i((xb - __builtin_floor(xb)) * 256.0 + 0.5) / 256.0; }

static inline int64_t lin_idx(desc_t t, struct q *q, int addr, int norm, double x, int d, int dim) {
    int64_t n = lv_dim(t, q->lv, dim);
    return ad_idx(addr, floor_i(pos(x, n, norm) - 0.5) + d + q->off[dim], n);
}

static inline double lin_w(desc_t t, struct q *q, int norm, double x, int dim) {
    return qw(pos(x, lv_dim(t, q->lv, dim), norm) - 0.5);
}

// point sampling, or linear filtering as the sum over the footprint of (product of the per-axis
// weights) * texel: the exact value of the model's nested lerps, rounded once at the end
static inline rgba sample(desc_t t, struct q *q, int filter, int addr, int norm, double x, double y, double z,
                          int64_t layer, int rank) {
    if (filter != 1) {
        int64_t n0 = lv_dim(t, q->lv, 0), n1 = lv_dim(t, q->lv, 1), n2 = lv_dim(t, q->lv, 2);
        int64_t i = ad_idx(addr, floor_i(pos(x, n0, norm)) + q->off[0], n0);
        int64_t j = rank >= 2 ? ad_idx(addr, floor_i(pos(y, n1, norm)) + q->off[1], n1) : 0;
        int64_t k = rank >= 3 ? ad_idx(addr, floor_i(pos(z, n2, norm)) + q->off[2], n2) : 0;
        return texel(t, q, i, j, k, layer);
    }
    int64_t i0 = lin_idx(t, q, addr, norm, x, 0, 0), i1 = lin_idx(t, q, addr, norm, x, 1, 0);
    int64_t j0 = rank >= 2 ? lin_idx(t, q, addr, norm, y, 0, 1) : 0;
    int64_t j1 = rank >= 2 ? lin_idx(t, q, addr, norm, y, 1, 1) : 0;
    int64_t k0 = rank >= 3 ? lin_idx(t, q, addr, norm, z, 0, 2) : 0;
    int64_t k1 = rank >= 3 ? lin_idx(t, q, addr, norm, z, 1, 2) : 0;
    double wx = lin_w(t, q, norm, x, 0);
    double wy = rank >= 2 ? lin_w(t, q, norm, y, 1) : 0.0;
    double wz = rank >= 3 ? lin_w(t, q, norm, z, 2) : 0.0;
    int taps = 1 << rank;
    rgba acc = {0, 0, 0, 0};
    NO_UNROLL
    for (int n = 0; n < taps; n++) {
        int dx = n & 1, dy = (n >> 1) & 1, dz = (n >> 2) & 1;
        double w = (dx ? wx : 1.0 - wx) * (dy ? wy : 1.0 - wy) * (dz ? wz : 1.0 - wz);
        acc += w * texel(t, q, dx ? i1 : i0, dy ? j1 : j0, dz ? k1 : k0, layer);
    }
    return acc;
}

// ---------------------------------------------------------------- mipmaps

// log2 for x > 0 at double precision (the model's log(x) / log(2) at 53 bits)
static inline double log2_d(double x) {
    union {
        double d;
        uint64_t u;
    } c = {x};
    int e = (int)((c.u >> 52) & 2047);
    if (e == 0) {
        // denormal: scale up
        c.d = x * 18014398509481984.0; // 2^54
        e = (int)((c.u >> 52) & 2047) - 54;
    }
    int exp = e - 1023;
    c.u = (c.u & 0x000fffffffffffffull) | 0x3ff0000000000000ull; // m in [1, 2)
    double m = c.d;
    if (m > 1.4142135623730951) {
        m *= 0.5;
        exp++;
    }
    // ln(m) = 2 atanh(s), s = (m - 1) / (m + 1), |s| <= 0.1716
    double s = (m - 1.0) / (m + 1.0), s2 = s * s, term = s, sum = 0.0;
    for (int k = 1; k < 40; k += 2) {
        sum += term / (double)k;
        term *= s2;
    }
    return (double)exp + 2.0 * sum / 0.6931471805599453;
}

static inline rgba mip_sample(desc_t t, struct sampling *sm, int use_lod, double lam, struct q *q, int filter,
                                     int addr, int norm, double x, double y, double z, int64_t layer, int rank) {
    if (!use_lod)
        return sample(t, q, filter, addr, norm, x, y, z, layer, rank);
    int64_t levels = tx_levels(t);
    double lc = dmin(dmax(dmax(lam, sm->min_lod), 0.0), dmin(sm->max_lod, (double)(levels - 1)));
    // the nearest level, or a blend of two by a weight quantised to 1/256
    int linear = sm->mip == 1;
    int64_t base = linear ? floor_i(lc) : imin(floor_i(lc + 0.5), levels - 1);
    double wl = linear ? qw(lc) : 0.0;
    rgba acc = {0, 0, 0, 0};
    NO_UNROLL
    for (int n = 0; n <= linear; n++) {
        struct q q2 = *q;
        q2.lv = (int)imin(base + n, levels - 1);
        acc += (n ? wl : 1.0 - wl) * sample(t, &q2, filter, addr, norm, x, y, z, layer, rank);
    }
    return acc;
}

// lod from gradients: log2 of the larger squared footprint, halved
static inline double grad_lam(desc_t t, int norm, int rank, const double *dx, const double *dy, double sd) {
    double lx = 0.0, ly = 0.0;
    for (int i = 0; i < rank; i++) {
        double scale = norm == 1 ? (double)tx_dim(t, i) / sd : 1.0 / sd;
        double a = dx[i] * scale, b = dy[i] * scale;
        lx += a * a;
        ly += b * b;
    }
    double s = dmax(lx, ly);
    return s > 0.0 ? log2_d(s) / 2.0 : -1000.0;
}

// ---------------------------------------------------------------- cubemaps

static inline int cube_face(double x, double y, double z) {
    double ax = __builtin_fabs(x), ay = __builtin_fabs(y), az = __builtin_fabs(z);
    if (ax >= ay && ax >= az)
        return x >= 0.0 ? 0 : 1;
    if (ay > ax && ay >= az)
        return y >= 0.0 ? 2 : 3;
    return z >= 0.0 ? 4 : 5;
}

static inline double cube_ma(double x, double y, double z) {
    int f = cube_face(x, y, z);
    return f < 2 ? __builtin_fabs(x) : f < 4 ? __builtin_fabs(y) : __builtin_fabs(z);
}

static inline double cube_sc(double x, double y, double z) {
    switch (cube_face(x, y, z)) {
    case 0:
        return -z;
    case 1:
        return z;
    case 5:
        return -x;
    default:
        return x;
    }
}

static inline double cube_tc(double x, double y, double z) {
    switch (cube_face(x, y, z)) {
    case 2:
        return z;
    case 3:
        return -z;
    default:
        return -y;
    }
}

static inline double cube_norm(double s, double ma) { return ma > 0.0 ? (s / ma + 1.0) * 0.5 : 0.5; }

// ---------------------------------------------------------------- results

// the destination format: 0 f32 (round to nearest), 1 s32, 2 u32
static inline uint32_t out_bits(int dtype, double v) {
    if (dtype == 0)
        return float_bits((float)v);
    if (v != v)
        return 0;
    return (uint32_t)(int64_t)v;
}

static inline double coord(uint32_t bits, int ctype_float) {
    return ctype_float ? bits_to_float(bits) : (double)(int32_t)bits;
}

static inline int64_t layer_of(desc_t t, uint32_t bits) { return imin((int64_t)bits, tx_layers(t) - 1); }

static inline int64_t cube_layer(desc_t t, int arrayed, uint32_t index, double x, double y, double z) {
    int64_t cube = imin(arrayed ? (int64_t)index : 0, imax(tx_layers(t) / 6 - 1, 0));
    return 6 * cube + cube_face(x, y, z);
}

// geometries
enum { G_1D, G_2D, G_3D, G_A1D, G_A2D, G_CUBE, G_ACUBE };

typedef uint32_t v4u32 __attribute__((ext_vector_type(4)));
typedef uint64_t v4u64 __attribute__((ext_vector_type(4)));

static inline int geom_rank(int g) { return g == G_1D || g == G_A1D ? 1 : g == G_3D ? 3 : 2; }
static inline int geom_array(int g) { return g == G_A1D || g == G_A2D || g == G_ACUBE; }
static inline int geom_cube(int g) { return g == G_CUBE || g == G_ACUBE; }

// tex[.base|.level|.grad].geom.v4.dtype.ctype d, [t {, s}, {c0..c3}] {, lod | , dPdx, dPdy} {, offsets} {, ref}
// lod_mode: 0 none (.base too), 1 .level, 2 .grad
API v4u32 __zluda_ptx_impl_agx_tex(desc_t t, desc_t s, int geom, int lod_mode, int ctype_float, int dtype,
                                   uint32_t c0, uint32_t c1, uint32_t c2, uint32_t c3, float lod, float dx0,
                                   float dx1, float dx2, float dy0, float dy1, float dy2, int32_t o0, int32_t o1,
                                   int32_t o2, int has_ref, uint32_t ref) {
    struct sampling sm = sampling_of(t, s);
    struct q q = {0, {o0, o1, o2}, has_ref, ref};
    double dx[3] = {dx0, dx1, dx2}, dy[3] = {dy0, dy1, dy2};
    uint32_t cs[4] = {c0, c1, c2, c3};
    rgba r;
    if (geom_cube(geom)) {
        int a = geom == G_ACUBE;
        double x = bits_to_float(cs[a]), y = bits_to_float(cs[a + 1]), z = bits_to_float(cs[a + 2]);
        double ma = cube_ma(x, y, z);
        double lam = lod_mode == 1 ? (double)lod : grad_lam(t, 1, 3, dx, dy, ma * 2.0);
        r = mip_sample(t, &sm, lod_mode != 0, lam, &q, sm.filter, 1, 1, cube_norm(cube_sc(x, y, z), ma),
                       cube_norm(cube_tc(x, y, z), ma), 0.0, cube_layer(t, a, c0, x, y, z), 2);
    } else {
        int a = geom_array(geom), rank = geom_rank(geom);
        double lam = lod_mode == 1 ? (double)lod : grad_lam(t, sm.norm, rank, dx, dy, 1.0);
        r = mip_sample(t, &sm, lod_mode != 0, lam, &q, ctype_float ? sm.filter : 0, sm.addr, sm.norm,
                       coord(cs[a], ctype_float), a + 1 < 4 ? coord(cs[a + 1], ctype_float) : 0.0,
                       a + 2 < 4 ? coord(cs[a + 2], ctype_float) : 0.0, a ? layer_of(t, c0) : 0, rank);
    }
    v4u32 out = {out_bits(dtype, r[0]), out_bits(dtype, r[1]), out_bits(dtype, r[2]), out_bits(dtype, r[3])};
    return out;
}

// tld4.comp.geom.v4.dtype.f32 d, [t {, s}, {c..}] {, offsets} {, ref}: the 2x2 footprint, counter-clockwise from
// the lower-left texel
API v4u32 __zluda_ptx_impl_agx_tld4(desc_t t, desc_t s, int geom, int comp, int dtype, uint32_t c0, uint32_t c1,
                                    uint32_t c2, uint32_t c3, int32_t o0, int32_t o1, int has_ref, uint32_t ref) {
    struct sampling sm = sampling_of(t, s);
    struct q q = {0, {o0, o1, 0}, has_ref, ref};
    uint32_t cs[4] = {c0, c1, c2, c3};
    double x, y;
    int64_t layer;
    int norm;
    if (geom_cube(geom)) {
        int a = geom == G_ACUBE;
        double cx = bits_to_float(cs[a]), cy = bits_to_float(cs[a + 1]), cz = bits_to_float(cs[a + 2]);
        double ma = cube_ma(cx, cy, cz);
        x = cube_norm(cube_sc(cx, cy, cz), ma);
        y = cube_norm(cube_tc(cx, cy, cz), ma);
        layer = cube_layer(t, a, c0, cx, cy, cz);
        norm = 1;
    } else {
        int a = geom_array(geom);
        x = bits_to_float(cs[a]);
        y = bits_to_float(cs[a + 1]);
        layer = a ? layer_of(t, c0) : 0;
        norm = sm.norm;
    }
    int addr = sm.addr;
    v4u32 out;
    NO_UNROLL
    for (int i = 0; i < 4; i++) {
        int dx = i == 1 || i == 2, dy = i < 2;
        rgba v = texel(t, &q, lin_idx(t, &q, addr, norm, x, dx, 0), lin_idx(t, &q, addr, norm, y, dy, 1), 0, layer);
        out[i] = out_bits(dtype, v[comp]);
    }
    return out;
}

// txq.field.b32: 0 width, 1 height, 2 depth, 3 array_size, 4 normalized_coords, 5 filter_mode,
// 6 addr_mode_*, 7 num_mipmap_levels (others 0)
API uint32_t __zluda_ptx_impl_agx_txq(desc_t t, int field) {
    switch (field) {
    case 0:
    case 1:
    case 2:
        return (uint32_t)tx_dim(t, field);
    case 3:
        return (uint32_t)(tw(t, 13) & 2 ? tw(t, 8) / 6 : tw(t, 8));
    case 4:
        return (uint32_t)tw(t, 6);
    case 5:
        return (uint32_t)tw(t, 4);
    case 6:
        return (uint32_t)tw(t, 5);
    case 7:
        return (uint32_t)tx_levels(t);
    default:
        return 0;
    }
}

// ---------------------------------------------------------------- surfaces

static inline int64_t sf_tb(desc_t f) { return imax(1, (int64_t)tw(f, 2)); }
static inline int64_t sf_layers(desc_t f) { return imax(1, (int64_t)tw(f, 3)); }
static inline int64_t sf_dim(desc_t f, int i) { return imax(1, (int64_t)tw(f, 4 + i)); }

// mode: 0 trap, 1 clamp, 2 zero
enum { SF_TRAP, SF_CLAMP, SF_ZERO };

// byte address of an access of n bytes at (x bytes, y, z, layer), or 0 outside
static inline uint64_t sf_addr(desc_t f, int mode, int64_t x, int64_t y, int64_t z, int64_t l, int64_t n) {
    if (mode == SF_CLAMP) {
        x = imin(imax(x, 0), imax(sf_dim(f, 0) * sf_tb(f) - n, 0));
        y = imin(imax(y, 0), sf_dim(f, 1) - 1);
        z = imin(imax(z, 0), sf_dim(f, 2) - 1);
        l = imin(imax(l, 0), sf_layers(f) - 1);
    }
    if (x < 0 || x + n > sf_dim(f, 0) * sf_tb(f) || y < 0 || y >= sf_dim(f, 1) || z < 0 || z >= sf_dim(f, 2) ||
        l < 0 || l >= sf_layers(f))
        return 0;
    return tw(f, 1) + (uint64_t)((((l * sf_dim(f, 2) + z) * sf_dim(f, 1) + y) * sf_dim(f, 0)) * sf_tb(f) + x);
}

// the coordinates {l?, x, y?, z?} of a geometry as (x, y, z, layer)
static inline void sf_coords(int geom, int32_t c0, int32_t c1, int32_t c2, int32_t c3, int64_t *xs) {
    int32_t cs[4] = {c0, c1, c2, c3};
    if (geom_array(geom)) {
        xs[0] = cs[1];
        xs[1] = geom_rank(geom) >= 2 ? cs[2] : 0;
        xs[2] = 0;
        xs[3] = (uint32_t)cs[0];
    } else {
        xs[0] = cs[0];
        xs[1] = geom_rank(geom) >= 2 ? cs[1] : 0;
        xs[2] = geom_rank(geom) >= 3 ? cs[2] : 0;
        xs[3] = 0;
    }
}

__attribute__((noreturn)) void __zluda_ptx_impl_agx_surface_trap(void);

// suld.b.geom.vN.btype.mode: n elements of b bytes into out (64-bit slots); returns 0
API v4u64 __zluda_ptx_impl_agx_suld_b(desc_t f, int geom, int mode, int n, int b, int32_t c0, int32_t c1, int32_t c2,
                                      int32_t c3) {
    int64_t xs[4];
    sf_coords(geom, c0, c1, c2, c3, xs);
    uint64_t a = sf_addr(f, mode, xs[0], xs[1], xs[2], xs[3], (int64_t)n * b);
    v4u64 out = {0, 0, 0, 0};
    if (!a) {
        if (mode != SF_ZERO)
            __zluda_ptx_impl_agx_surface_trap();
        return out;
    }
    const GLOBAL uint8_t *p = (const GLOBAL uint8_t *)a;
    for (int i = 0; i < n; i++) {
        uint64_t v = 0;
        for (int j = 0; j < b; j++)
            v |= (uint64_t)p[i * b + j] << (8 * j);
        out[i] = v;
    }
    return out;
}

static inline void store_bytes(uint64_t a, const uint64_t *vs, int n, int b) {
    GLOBAL uint8_t *p = (GLOBAL uint8_t *)a;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < b; j++)
            p[i * b + j] = (uint8_t)(vs[i] >> (8 * j));
}

// sust.b.geom.vN.btype.mode
API void __zluda_ptx_impl_agx_sust_b(desc_t f, int geom, int mode, int n, int b, int32_t c0, int32_t c1, int32_t c2,
                                     int32_t c3, uint64_t v0, uint64_t v1, uint64_t v2, uint64_t v3) {
    int64_t xs[4];
    sf_coords(geom, c0, c1, c2, c3, xs);
    uint64_t a = sf_addr(f, mode, xs[0], xs[1], xs[2], xs[3], (int64_t)n * b);
    if (!a) {
        if (mode != SF_ZERO)
            __zluda_ptx_impl_agx_surface_trap();
        return;
    }
    uint64_t vs[4] = {v0, v1, v2, v3};
    store_bytes(a, vs, n, b);
}

// formatted stores: sample formats 0 UNORM8 1 SNORM8 2 UINT8 3 SINT8 4 UNORM16 5 SNORM16 6 UINT16 7 SINT16
// 8 FLOAT16 9 UINT32 10 SINT32 11 FLOAT32
static inline int fmt_bytes(int64_t f) { return f <= 3 ? 1 : f <= 8 ? 2 : 4; }
static inline int fmt_signed(int64_t f) { return f == 3 || f == 7 || f == 10; }

static inline int64_t rne(double x) {
    double fl = __builtin_floor(x), d = x - fl;
    int64_t i = (int64_t)fl;
    return d > 0.5 || (d == 0.5 && (i & 1)) ? i + 1 : i;
}

static inline uint32_t float_to_half_rn(double v) {
    float f = (float)v;
    uint32_t u = float_bits(f), s = (u >> 16) & 0x8000;
    int e = (int)((u >> 23) & 255) - 127 + 15;
    uint32_t m = u & 0x7fffff;
    if (((u >> 23) & 255) == 255)
        return s | 0x7c00 | (m ? 0x200 : 0);
    if (e >= 31)
        return s | 0x7c00;
    if (e <= 0) {
        if (e < -10)
            return s;
        m |= 0x800000;
        int shift = 14 - e;
        uint32_t r = m >> shift, rem = m & ((1u << shift) - 1), halfway = 1u << (shift - 1);
        if (rem > halfway || (rem == halfway && (r & 1)))
            r++;
        return s | r;
    }
    uint32_t r = ((uint32_t)e << 10) | (m >> 13), rem = m & 0x1fff;
    if (rem > 0x1000 || (rem == 0x1000 && (r & 1)))
        r++;
    return s | r;
}

static inline uint64_t fmt_conv(int64_t f, uint32_t v) {
    int bits = 8 * fmt_bytes(f);
    double x = bits_to_float(v);
    switch (f) {
    case 0:
    case 4: {
        double c = x != x ? 0.0 : dmin(dmax(x, 0.0), 1.0);
        return (uint64_t)rne(c * (double)(((int64_t)1 << bits) - 1));
    }
    case 1:
    case 5: {
        double c = x != x ? 0.0 : dmin(dmax(x, -1.0), 1.0);
        return (uint64_t)rne(c * (double)(((int64_t)1 << (bits - 1)) - 1)) & (((uint64_t)1 << bits) - 1);
    }
    case 2:
    case 6:
    case 9:
        return (uint64_t)imin(v, ((int64_t)1 << bits) - 1);
    case 3:
    case 7:
    case 10: {
        int64_t lo = -((int64_t)1 << (bits - 1)), hi = ((int64_t)1 << (bits - 1)) - 1;
        return (uint64_t)imin(imax((int32_t)v, lo), hi) & (((uint64_t)1 << bits) - 1);
    }
    case 8:
        return float_to_half_rn(x);
    default:
        return v;
    }
}

// sust.p.geom.vN.b32.mode: the lowest coordinate counts samples; source components beyond the
// surface's channels are ignored
API void __zluda_ptx_impl_agx_sust_p(desc_t f, int geom, int mode, int n, int32_t c0, int32_t c1, int32_t c2,
                                     int32_t c3, uint32_t v0, uint32_t v1, uint32_t v2, uint32_t v3) {
    int64_t xs[4];
    sf_coords(geom, c0, c1, c2, c3, xs);
    xs[0] *= sf_tb(f);
    int64_t fmt = (int64_t)tw(f, 7), ch = imax(1, (int64_t)tw(f, 8));
    int cb = fmt_bytes(fmt);
    int count = (int)imin(n, ch);
    uint64_t a = sf_addr(f, mode, xs[0], xs[1], xs[2], xs[3], (int64_t)count * cb);
    if (!a) {
        if (mode != SF_ZERO)
            __zluda_ptx_impl_agx_surface_trap();
        return;
    }
    // (no arrays: the count is the surface's, so they would live in memory)
    GLOBAL uint8_t *p = (GLOBAL uint8_t *)a;
    for (int i = 0; i < 4; i++) {
        if (i >= count)
            break;
        uint64_t c = fmt_conv(fmt, i == 0 ? v0 : i == 1 ? v1 : i == 2 ? v2 : v3);
        for (int j = 0; j < 4; j++)
            if (j < cb)
                p[i * cb + j] = (uint8_t)(c >> (8 * j));
    }
}

// the address of a reduction (0 outside, after a trap unless .zero); formatted (.p) forms scale x by the data size
API uint64_t __zluda_ptx_impl_agx_sured_addr(desc_t f, int geom, int mode, int formatted, int b, int32_t c0,
                                             int32_t c1, int32_t c2, int32_t c3) {
    int64_t xs[4];
    sf_coords(geom, c0, c1, c2, c3, xs);
    if (formatted)
        xs[0] *= b;
    uint64_t a = sf_addr(f, mode, xs[0], xs[1], xs[2], xs[3], b);
    if (!a && mode != SF_ZERO)
        __zluda_ptx_impl_agx_surface_trap();
    return a;
}

// sured: op 0 add, 1 and, 2 or, 3 min, 4 max (signed or not), on b = 4 or 8 bytes
API void __zluda_ptx_impl_agx_sured(desc_t f, int geom, int mode, int formatted, int b, int32_t c0, int32_t c1,
                                    int32_t c2, int32_t c3, int op, int is_signed, uint64_t v) {
    uint64_t a = __zluda_ptx_impl_agx_sured_addr(f, geom, mode, formatted, b, c0, c1, c2, c3);
    if (!a)
        return;
    if (b == 8) {
        GLOBAL uint64_t *p = (GLOBAL uint64_t *)a;
        GLOBAL int64_t *ps = (GLOBAL int64_t *)a;
        switch (op) {
        case 0:
            __atomic_fetch_add(p, v, __ATOMIC_RELAXED);
            break;
        case 1:
            __atomic_fetch_and(p, v, __ATOMIC_RELAXED);
            break;
        case 2:
            __atomic_fetch_or(p, v, __ATOMIC_RELAXED);
            break;
        case 3:
            if (is_signed)
                __atomic_fetch_min(ps, (int64_t)v, __ATOMIC_RELAXED);
            else
                __atomic_fetch_min(p, v, __ATOMIC_RELAXED);
            break;
        default:
            if (is_signed)
                __atomic_fetch_max(ps, (int64_t)v, __ATOMIC_RELAXED);
            else
                __atomic_fetch_max(p, v, __ATOMIC_RELAXED);
            break;
        }
        return;
    }
    GLOBAL uint32_t *p = (GLOBAL uint32_t *)a;
    GLOBAL int32_t *ps = (GLOBAL int32_t *)a;
    uint32_t w = (uint32_t)v;
    switch (op) {
    case 0:
        __atomic_fetch_add(p, w, __ATOMIC_RELAXED);
        break;
    case 1:
        __atomic_fetch_and(p, w, __ATOMIC_RELAXED);
        break;
    case 2:
        __atomic_fetch_or(p, w, __ATOMIC_RELAXED);
        break;
    case 3:
        if (is_signed)
            __atomic_fetch_min(ps, (int32_t)w, __ATOMIC_RELAXED);
        else
            __atomic_fetch_min(p, w, __ATOMIC_RELAXED);
        break;
    default:
        if (is_signed)
            __atomic_fetch_max(ps, (int32_t)w, __ATOMIC_RELAXED);
        else
            __atomic_fetch_max(p, w, __ATOMIC_RELAXED);
        break;
    }
}

// whether a formatted surface holds signed integers (sured.p's .b32 / .b64)
API int __zluda_ptx_impl_agx_surface_signed(desc_t f) { return fmt_signed((int64_t)tw(f, 7)); }

// suq.field.b32: 0 width, 1 height, 2 depth, 3 array_size, 4 channel_data_type, 5 channel_order,
// 6 memory_layout
API uint32_t __zluda_ptx_impl_agx_suq(desc_t f, int field) {
    switch (field) {
    case 0:
    case 1:
    case 2:
        return (uint32_t)sf_dim(f, field);
    case 3:
        return (uint32_t)(tw(f, 12) ? tw(f, 3) : 0);
    case 4:
        return (uint32_t)tw(f, 9);
    case 5:
        return (uint32_t)tw(f, 10);
    case 6:
        return (uint32_t)tw(f, 11);
    default:
        return 0;
    }
}

// istypep.{texref,samplerref,surfref}: whether the handle is a descriptor of that kind (0 texture,
// 1 sampler, 2 surface); handles below 4096 are not objects
API int __zluda_ptx_impl_agx_istypep(uint64_t h, int kind) {
    if (h < 4096 || (h & 7))
        return 0;
    uint64_t magic = kind == 0 ? TEX_MAGIC : kind == 1 ? SAMPLER_MAGIC : SURF_MAGIC;
    return *(const GLOBAL uint64_t *)h == magic;
}

// ---------------------------------------------------------------- the instruction grammar's tex.{1d,2d,3d}

// tex.geom.v4.dtype.ctype d, [t, c] as replace_instructions_with_functions calls it (u32 results as s32)
typedef float v1f32 __attribute__((ext_vector_type(1)));
typedef float v2f32 __attribute__((ext_vector_type(2)));
typedef float v4f32 __attribute__((ext_vector_type(4)));
typedef int32_t v1s32 __attribute__((ext_vector_type(1)));
typedef int32_t v2s32 __attribute__((ext_vector_type(2)));
typedef int32_t v4s32 __attribute__((ext_vector_type(4)));

#define TEXOBJ(GEOM, CODE, DTYPE, DCODE, CTYPE, CFLOAT, N, C1, C2)                                                   \
    API v4##DTYPE __zluda_ptx_impl_texobj_##GEOM##_v4_##DTYPE##_##CTYPE(uint64_t texobj, v##N##CTYPE coord) {        \
        union {                                                                                                     \
            CTYPE##_t v;                                                                                            \
            uint32_t u;                                                                                             \
        } x = {coord[0]}, y = {C1}, z = {C2};                                                                       \
        v4u32 out = __zluda_ptx_impl_agx_tex((desc_t)texobj, 0, CODE, 0, CFLOAT, DCODE, x.u, y.u, z.u, 0, 0, 0, 0, \
                                             0, 0, 0, 0, 0, 0, 0, 0, 0);                                            \
        union {                                                                                                     \
            uint32_t u[4];                                                                                          \
            v4##DTYPE v;                                                                                            \
        } r = {{out[0], out[1], out[2], out[3]}};                                                                   \
        return r.v;                                                                                                 \
    }

typedef float f32_t;
typedef int32_t s32_t;
#define ALL_TEXOBJ(DTYPE, DCODE)                                                                                     \
    TEXOBJ(1d, G_1D, DTYPE, DCODE, f32, 1, 1, 0, 0)                                                                 \
    TEXOBJ(1d, G_1D, DTYPE, DCODE, s32, 0, 1, 0, 0)                                                                 \
    TEXOBJ(2d, G_2D, DTYPE, DCODE, f32, 1, 2, coord[1], 0)                                                          \
    TEXOBJ(2d, G_2D, DTYPE, DCODE, s32, 0, 2, coord[1], 0)                                                          \
    TEXOBJ(3d, G_3D, DTYPE, DCODE, f32, 1, 4, coord[1], coord[2])                                                   \
    TEXOBJ(3d, G_3D, DTYPE, DCODE, s32, 0, 4, coord[1], coord[2])
ALL_TEXOBJ(f32, 0)
ALL_TEXOBJ(s32, 1)
