// GENERATED AUTOMATICALLY BY cvt.py, DON'T CHANGE MANUALLY
extern "C"
{
typedef __fp16 half;
half convert_half_rte(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_f32)(float x) { auto temp = __float2half_rn(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtz(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_f32)(float x) { auto temp = __float2half_rz(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtn(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_f32)(float x) { auto temp = __float2half_rd(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtp(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_f32)(float x) { auto temp = __float2half_ru(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_f64)(double x) { return convert_half_rte(x); }
half convert_half_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_f64)(double x) { return convert_half_rtz(x); }
half convert_half_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_f64)(double x) { return convert_half_rtn(x); }
half convert_half_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_f64)(double x) { return convert_half_rtp(x); }
float convert_float_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rn_f32_f64)(double x) { return __double2float_rn(x); }
float convert_float_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rz_f32_f64)(double x) { return __double2float_rz(x); }
float convert_float_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rm_f32_f64)(double x) { return __double2float_rd(x); }
float convert_float_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rp_f32_f64)(double x) { return __double2float_ru(x); }
half convert_half_rte(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_s8)(char x) { return convert_half_rte(x); }
half convert_half_rtz(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_s8)(char x) { return convert_half_rtz(x); }
half convert_half_rtn(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_s8)(char x) { return convert_half_rtn(x); }
half convert_half_rtp(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_s8)(char x) { return convert_half_rtp(x); }
float convert_float_rte(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rn_f32_s8)(char x) { return convert_float_rte(x); }
float convert_float_rtz(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rz_f32_s8)(char x) { return convert_float_rtz(x); }
float convert_float_rtn(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rm_f32_s8)(char x) { return convert_float_rtn(x); }
float convert_float_rtp(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rp_f32_s8)(char x) { return convert_float_rtp(x); }
double convert_double_rte(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rn_f64_s8)(char x) { return convert_double_rte(x); }
double convert_double_rtz(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rz_f64_s8)(char x) { return convert_double_rtz(x); }
double convert_double_rtn(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rm_f64_s8)(char x) { return convert_double_rtn(x); }
double convert_double_rtp(char) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rp_f64_s8)(char x) { return convert_double_rtp(x); }
half convert_half_rte(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_s16)(short x) { auto temp = __short2half_rn(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtz(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_s16)(short x) { auto temp = __short2half_rz(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtn(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_s16)(short x) { auto temp = __short2half_rd(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtp(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_s16)(short x) { auto temp = __short2half_ru(x); return *reinterpret_cast<half*>(&temp); }
float convert_float_rte(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rn_f32_s16)(short x) { return convert_float_rte(x); }
float convert_float_rtz(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rz_f32_s16)(short x) { return convert_float_rtz(x); }
float convert_float_rtn(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rm_f32_s16)(short x) { return convert_float_rtn(x); }
float convert_float_rtp(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rp_f32_s16)(short x) { return convert_float_rtp(x); }
double convert_double_rte(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rn_f64_s16)(short x) { return convert_double_rte(x); }
double convert_double_rtz(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rz_f64_s16)(short x) { return convert_double_rtz(x); }
double convert_double_rtn(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rm_f64_s16)(short x) { return convert_double_rtn(x); }
double convert_double_rtp(short) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rp_f64_s16)(short x) { return convert_double_rtp(x); }
half convert_half_rte(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_s32)(int x) { auto temp = __int2half_rn(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtz(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_s32)(int x) { auto temp = __int2half_rz(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtn(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_s32)(int x) { auto temp = __int2half_rd(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtp(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_s32)(int x) { auto temp = __int2half_ru(x); return *reinterpret_cast<half*>(&temp); }
float convert_float_rte(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rn_f32_s32)(int x) { return __int2float_rn(x); }
float convert_float_rtz(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rz_f32_s32)(int x) { return __int2float_rz(x); }
float convert_float_rtn(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rm_f32_s32)(int x) { return __int2float_rd(x); }
float convert_float_rtp(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rp_f32_s32)(int x) { return __int2float_ru(x); }
double convert_double_rte(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rn_f64_s32)(int x) { return __int2double_rn(x); }
double convert_double_rtz(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rz_f64_s32)(int x) { return convert_double_rtz(x); }
double convert_double_rtn(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rm_f64_s32)(int x) { return convert_double_rtn(x); }
double convert_double_rtp(int) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rp_f64_s32)(int x) { return convert_double_rtp(x); }
half convert_half_rte(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_s64)(long x) { auto temp = __ll2half_rn(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtz(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_s64)(long x) { auto temp = __ll2half_rz(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtn(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_s64)(long x) { auto temp = __ll2half_rd(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtp(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_s64)(long x) { auto temp = __ll2half_ru(x); return *reinterpret_cast<half*>(&temp); }
float convert_float_rte(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rn_f32_s64)(long x) { return __ll2float_rn(x); }
float convert_float_rtz(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rz_f32_s64)(long x) { return __ll2float_rz(x); }
float convert_float_rtn(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rm_f32_s64)(long x) { return __ll2float_rd(x); }
float convert_float_rtp(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rp_f32_s64)(long x) { return __ll2float_ru(x); }
double convert_double_rte(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rn_f64_s64)(long x) { return __ll2double_rn(x); }
double convert_double_rtz(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rz_f64_s64)(long x) { return __ll2double_rz(x); }
double convert_double_rtn(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rm_f64_s64)(long x) { return __ll2double_rd(x); }
double convert_double_rtp(long) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rp_f64_s64)(long x) { return __ll2double_ru(x); }
half convert_half_rte(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_u8)(uchar x) { return convert_half_rte(x); }
half convert_half_rtz(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_u8)(uchar x) { return convert_half_rtz(x); }
half convert_half_rtn(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_u8)(uchar x) { return convert_half_rtn(x); }
half convert_half_rtp(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_u8)(uchar x) { return convert_half_rtp(x); }
float convert_float_rte(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rn_f32_u8)(uchar x) { return convert_float_rte(x); }
float convert_float_rtz(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rz_f32_u8)(uchar x) { return convert_float_rtz(x); }
float convert_float_rtn(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rm_f32_u8)(uchar x) { return convert_float_rtn(x); }
float convert_float_rtp(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rp_f32_u8)(uchar x) { return convert_float_rtp(x); }
double convert_double_rte(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rn_f64_u8)(uchar x) { return convert_double_rte(x); }
double convert_double_rtz(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rz_f64_u8)(uchar x) { return convert_double_rtz(x); }
double convert_double_rtn(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rm_f64_u8)(uchar x) { return convert_double_rtn(x); }
double convert_double_rtp(uchar) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rp_f64_u8)(uchar x) { return convert_double_rtp(x); }
half convert_half_rte(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_u16)(ushort x) { auto temp = __ushort2half_rn(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtz(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_u16)(ushort x) { auto temp = __ushort2half_rz(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtn(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_u16)(ushort x) { auto temp = __ushort2half_rd(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtp(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_u16)(ushort x) { auto temp = __ushort2half_ru(x); return *reinterpret_cast<half*>(&temp); }
float convert_float_rte(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rn_f32_u16)(ushort x) { return convert_float_rte(x); }
float convert_float_rtz(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rz_f32_u16)(ushort x) { return convert_float_rtz(x); }
float convert_float_rtn(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rm_f32_u16)(ushort x) { return convert_float_rtn(x); }
float convert_float_rtp(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rp_f32_u16)(ushort x) { return convert_float_rtp(x); }
double convert_double_rte(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rn_f64_u16)(ushort x) { return convert_double_rte(x); }
double convert_double_rtz(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rz_f64_u16)(ushort x) { return convert_double_rtz(x); }
double convert_double_rtn(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rm_f64_u16)(ushort x) { return convert_double_rtn(x); }
double convert_double_rtp(ushort) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rp_f64_u16)(ushort x) { return convert_double_rtp(x); }
half convert_half_rte(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_u32)(uint x) { auto temp = __uint2half_rn(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtz(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_u32)(uint x) { auto temp = __uint2half_rz(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtn(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_u32)(uint x) { auto temp = __uint2half_rd(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtp(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_u32)(uint x) { auto temp = __uint2half_ru(x); return *reinterpret_cast<half*>(&temp); }
float convert_float_rte(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rn_f32_u32)(uint x) { return __uint2float_rn(x); }
float convert_float_rtz(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rz_f32_u32)(uint x) { return __uint2float_rz(x); }
float convert_float_rtn(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rm_f32_u32)(uint x) { return __uint2float_rd(x); }
float convert_float_rtp(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rp_f32_u32)(uint x) { return __uint2float_ru(x); }
double convert_double_rte(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rn_f64_u32)(uint x) { return convert_double_rte(x); }
double convert_double_rtz(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rz_f64_u32)(uint x) { return convert_double_rtz(x); }
double convert_double_rtn(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rm_f64_u32)(uint x) { return convert_double_rtn(x); }
double convert_double_rtp(uint) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rp_f64_u32)(uint x) { return convert_double_rtp(x); }
half convert_half_rte(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rn_f16_u64)(ulong x) { auto temp = __ull2half_rn(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtz(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rz_f16_u64)(ulong x) { auto temp = __ull2half_rz(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtn(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rm_f16_u64)(ulong x) { auto temp = __ull2half_rd(x); return *reinterpret_cast<half*>(&temp); }
half convert_half_rtp(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) half FUNC(cvt_rp_f16_u64)(ulong x) { auto temp = __ull2half_ru(x); return *reinterpret_cast<half*>(&temp); }
float convert_float_rte(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rn_f32_u64)(ulong x) { return __ull2float_rn(x); }
float convert_float_rtz(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rz_f32_u64)(ulong x) { return __ull2float_rz(x); }
float convert_float_rtn(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rm_f32_u64)(ulong x) { return __ull2float_rd(x); }
float convert_float_rtp(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) float FUNC(cvt_rp_f32_u64)(ulong x) { return __ull2float_ru(x); }
double convert_double_rte(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rn_f64_u64)(ulong x) { return __ull2double_rn(x); }
double convert_double_rtz(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rz_f64_u64)(ulong x) { return __ull2double_rz(x); }
double convert_double_rtn(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rm_f64_u64)(ulong x) { return __ull2double_rd(x); }
double convert_double_rtp(ulong) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) double FUNC(cvt_rp_f64_u64)(ulong x) { return __ull2double_ru(x); }
char convert_char_rte(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rn_s8_f16)(half x) { return convert_char_rte(x); }
char convert_char_rtz(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rz_s8_f16)(half x) { return convert_char_rtz(x); }
char convert_char_rtn(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rm_s8_f16)(half x) { return convert_char_rtn(x); }
char convert_char_rtp(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rp_s8_f16)(half x) { return convert_char_rtp(x); }
short convert_short_rte(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rn_s16_f16)(half x) { return __half2short_rn(reinterpret_cast<half&>(x)); }
short convert_short_rtz(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rz_s16_f16)(half x) { return __half2short_rz(reinterpret_cast<half&>(x)); }
short convert_short_rtn(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rm_s16_f16)(half x) { return __half2short_rd(reinterpret_cast<half&>(x)); }
short convert_short_rtp(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rp_s16_f16)(half x) { return __half2short_ru(reinterpret_cast<half&>(x)); }
int convert_int_rte(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rn_s32_f16)(half x) { return __half2int_rn(reinterpret_cast<half&>(x)); }
int convert_int_rtz(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rz_s32_f16)(half x) { return __half2int_rz(reinterpret_cast<half&>(x)); }
int convert_int_rtn(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rm_s32_f16)(half x) { return __half2int_rd(reinterpret_cast<half&>(x)); }
int convert_int_rtp(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rp_s32_f16)(half x) { return __half2int_ru(reinterpret_cast<half&>(x)); }
long convert_long_rte(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rn_s64_f16)(half x) { return __half2ll_rn(reinterpret_cast<half&>(x)); }
long convert_long_rtz(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rz_s64_f16)(half x) { return __half2ll_rz(reinterpret_cast<half&>(x)); }
long convert_long_rtn(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rm_s64_f16)(half x) { return __half2ll_rd(reinterpret_cast<half&>(x)); }
long convert_long_rtp(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rp_s64_f16)(half x) { return __half2ll_ru(reinterpret_cast<half&>(x)); }
char convert_char_rte(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rn_s8_f32)(float x) { return convert_char_rte(x); }
char convert_char_rtz(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rz_s8_f32)(float x) { return convert_char_rtz(x); }
char convert_char_rtn(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rm_s8_f32)(float x) { return convert_char_rtn(x); }
char convert_char_rtp(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rp_s8_f32)(float x) { return convert_char_rtp(x); }
short convert_short_rte(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rn_s16_f32)(float x) { return convert_short_rte(x); }
short convert_short_rtz(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rz_s16_f32)(float x) { return convert_short_rtz(x); }
short convert_short_rtn(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rm_s16_f32)(float x) { return convert_short_rtn(x); }
short convert_short_rtp(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rp_s16_f32)(float x) { return convert_short_rtp(x); }
int convert_int_rte(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rn_s32_f32)(float x) { return __float2int_rn(x); }
int convert_int_rtz(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rz_s32_f32)(float x) { return __float2int_rz(x); }
int convert_int_rtn(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rm_s32_f32)(float x) { return __float2int_rd(x); }
int convert_int_rtp(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rp_s32_f32)(float x) { return __float2int_ru(x); }
long convert_long_rte(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rn_s64_f32)(float x) { return __float2ll_rn(x); }
long convert_long_rtz(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rz_s64_f32)(float x) { return __float2ll_rz(x); }
long convert_long_rtn(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rm_s64_f32)(float x) { return __float2ll_rd(x); }
long convert_long_rtp(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rp_s64_f32)(float x) { return __float2ll_ru(x); }
char convert_char_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rn_s8_f64)(double x) { return convert_char_rte(x); }
char convert_char_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rz_s8_f64)(double x) { return convert_char_rtz(x); }
char convert_char_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rm_s8_f64)(double x) { return convert_char_rtn(x); }
char convert_char_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) char FUNC(cvt_rp_s8_f64)(double x) { return convert_char_rtp(x); }
short convert_short_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rn_s16_f64)(double x) { return convert_short_rte(x); }
short convert_short_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rz_s16_f64)(double x) { return convert_short_rtz(x); }
short convert_short_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rm_s16_f64)(double x) { return convert_short_rtn(x); }
short convert_short_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) short FUNC(cvt_rp_s16_f64)(double x) { return convert_short_rtp(x); }
int convert_int_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rn_s32_f64)(double x) { return __double2int_rn(x); }
int convert_int_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rz_s32_f64)(double x) { return __double2int_rz(x); }
int convert_int_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rm_s32_f64)(double x) { return __double2int_rd(x); }
int convert_int_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) int FUNC(cvt_rp_s32_f64)(double x) { return __double2int_ru(x); }
long convert_long_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rn_s64_f64)(double x) { return __double2ll_rn(x); }
long convert_long_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rz_s64_f64)(double x) { return __double2ll_rz(x); }
long convert_long_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rm_s64_f64)(double x) { return __double2ll_rd(x); }
long convert_long_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) long FUNC(cvt_rp_s64_f64)(double x) { return __double2ll_ru(x); }
uchar convert_uchar_rte(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rn_u8_f16)(half x) { return convert_uchar_rte(x); }
uchar convert_uchar_rtz(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rz_u8_f16)(half x) { return convert_uchar_rtz(x); }
uchar convert_uchar_rtn(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rm_u8_f16)(half x) { return convert_uchar_rtn(x); }
uchar convert_uchar_rtp(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rp_u8_f16)(half x) { return convert_uchar_rtp(x); }
ushort convert_ushort_rte(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rn_u16_f16)(half x) { return __half2ushort_rn(reinterpret_cast<half&>(x)); }
ushort convert_ushort_rtz(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rz_u16_f16)(half x) { return __half2ushort_rz(reinterpret_cast<half&>(x)); }
ushort convert_ushort_rtn(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rm_u16_f16)(half x) { return __half2ushort_rd(reinterpret_cast<half&>(x)); }
ushort convert_ushort_rtp(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rp_u16_f16)(half x) { return __half2ushort_ru(reinterpret_cast<half&>(x)); }
uint convert_uint_rte(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rn_u32_f16)(half x) { return __half2uint_rn(reinterpret_cast<half&>(x)); }
uint convert_uint_rtz(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rz_u32_f16)(half x) { return __half2uint_rz(reinterpret_cast<half&>(x)); }
uint convert_uint_rtn(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rm_u32_f16)(half x) { return __half2uint_rd(reinterpret_cast<half&>(x)); }
uint convert_uint_rtp(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rp_u32_f16)(half x) { return __half2uint_ru(reinterpret_cast<half&>(x)); }
ulong convert_ulong_rte(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rn_u64_f16)(half x) { return __half2ull_rn(reinterpret_cast<half&>(x)); }
ulong convert_ulong_rtz(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rz_u64_f16)(half x) { return __half2ull_rz(reinterpret_cast<half&>(x)); }
ulong convert_ulong_rtn(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rm_u64_f16)(half x) { return __half2ull_rd(reinterpret_cast<half&>(x)); }
ulong convert_ulong_rtp(half) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rp_u64_f16)(half x) { return __half2ull_ru(reinterpret_cast<half&>(x)); }
uchar convert_uchar_rte(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rn_u8_f32)(float x) { return convert_uchar_rte(x); }
uchar convert_uchar_rtz(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rz_u8_f32)(float x) { return convert_uchar_rtz(x); }
uchar convert_uchar_rtn(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rm_u8_f32)(float x) { return convert_uchar_rtn(x); }
uchar convert_uchar_rtp(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rp_u8_f32)(float x) { return convert_uchar_rtp(x); }
ushort convert_ushort_rte(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rn_u16_f32)(float x) { return convert_ushort_rte(x); }
ushort convert_ushort_rtz(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rz_u16_f32)(float x) { return convert_ushort_rtz(x); }
ushort convert_ushort_rtn(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rm_u16_f32)(float x) { return convert_ushort_rtn(x); }
ushort convert_ushort_rtp(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rp_u16_f32)(float x) { return convert_ushort_rtp(x); }
uint convert_uint_rte(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rn_u32_f32)(float x) { return __float2uint_rn(x); }
uint convert_uint_rtz(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rz_u32_f32)(float x) { return __float2uint_rz(x); }
uint convert_uint_rtn(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rm_u32_f32)(float x) { return __float2uint_rd(x); }
uint convert_uint_rtp(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rp_u32_f32)(float x) { return __float2uint_ru(x); }
ulong convert_ulong_rte(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rn_u64_f32)(float x) { return __float2ull_rn(x); }
ulong convert_ulong_rtz(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rz_u64_f32)(float x) { return __float2ull_rz(x); }
ulong convert_ulong_rtn(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rm_u64_f32)(float x) { return __float2ull_rd(x); }
ulong convert_ulong_rtp(float) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rp_u64_f32)(float x) { return __float2ull_ru(x); }
uchar convert_uchar_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rn_u8_f64)(double x) { return convert_uchar_rte(x); }
uchar convert_uchar_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rz_u8_f64)(double x) { return convert_uchar_rtz(x); }
uchar convert_uchar_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rm_u8_f64)(double x) { return convert_uchar_rtn(x); }
uchar convert_uchar_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uchar FUNC(cvt_rp_u8_f64)(double x) { return convert_uchar_rtp(x); }
ushort convert_ushort_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rn_u16_f64)(double x) { return convert_ushort_rte(x); }
ushort convert_ushort_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rz_u16_f64)(double x) { return convert_ushort_rtz(x); }
ushort convert_ushort_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rm_u16_f64)(double x) { return convert_ushort_rtn(x); }
ushort convert_ushort_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ushort FUNC(cvt_rp_u16_f64)(double x) { return convert_ushort_rtp(x); }
uint convert_uint_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rn_u32_f64)(double x) { return __double2uint_rn(x); }
uint convert_uint_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rz_u32_f64)(double x) { return __double2uint_rz(x); }
uint convert_uint_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rm_u32_f64)(double x) { return __double2uint_rd(x); }
uint convert_uint_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) uint FUNC(cvt_rp_u32_f64)(double x) { return __double2uint_ru(x); }
ulong convert_ulong_rte(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rn_u64_f64)(double x) { return __double2ull_rn(x); }
ulong convert_ulong_rtz(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rz_u64_f64)(double x) { return __double2ull_rz(x); }
ulong convert_ulong_rtn(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rm_u64_f64)(double x) { return __double2ull_rd(x); }
ulong convert_ulong_rtp(double) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));
__attribute__((always_inline)) ulong FUNC(cvt_rp_u64_f64)(double x) { return __double2ull_ru(x); }
}
