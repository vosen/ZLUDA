// Every time this file changes it must te rebuilt, you need `rocm-llvm-dev` and `llvm-17`
// `fdenormal-fp-math=dynamic` is required to make functions eligible for inlining
//  /opt/rocm/llvm/bin/clang -std=c++20 -Xclang -fdenormal-fp-math=dynamic  -Wall -Wextra -Wsign-compare -Wconversion -x hip zluda_ptx_impl.cpp -nogpulib -O3 -mno-wavefrontsize64 -o zluda_ptx_impl.bc -emit-llvm -c --offload-device-only --offload-arch=gfx1100 && /opt/rocm/llvm/bin/llvm-dis zluda_ptx_impl.bc -o - | sed '/@llvm.used/d' | sed '/wchar_size/d' | sed '/llvm.module.flags/d' | sed 's/define hidden/define linkonce_odr/g' | sed 's/\"target-cpu\"=\"gfx1100\"//g' | sed -E 's/\"target-features\"=\"[^\"]+\"//g' | sed 's/ nneg / /g' | sed 's/ disjoint / /g' | sed '/__hip_cuid/d' | sed 's/external protected/external hidden/g' | sed 's/trunc nuw/trunc/' | sed 's/trunc nsw/trunc/' | llvm-as-17 - -o  zluda_ptx_impl.bc && /opt/rocm/llvm/bin/llvm-dis zluda_ptx_impl.bc

#include <cstddef>
#include <cstdint>
#include <bit>
#include <cmath>
#include <hip/hip_runtime.h>
#include <hip/amd_detail/amd_device_functions.h>
#include <hip/hip_fp8.h>

#define SHARED_SPACE __attribute__((address_space(3)))
#define CONSTANT_SPACE __attribute__((address_space(4)))

typedef _Float16 half16 __attribute__((ext_vector_type(16)));
typedef float float8 __attribute__((ext_vector_type(8)));

#define FUNC(NAME) __device__ __attribute__((retain)) __zluda_ptx_impl_##NAME
#define FUNC_CALL(NAME) __zluda_ptx_impl_##NAME
#define ATTR(NAME) __ZLUDA_PTX_IMPL_ATTRIBUTE_##NAME
#define DECLARE_ATTR(TYPE, NAME)                                        \
    extern "C" __attribute__((constant)) CONSTANT_SPACE TYPE ATTR(NAME) \
    __device__

extern "C"
{
    uint32_t FUNC(activemask)()
    {
        return __builtin_amdgcn_read_exec_lo();
    }

    size_t __ockl_get_local_id(uint32_t) __device__;
    uint32_t FUNC(sreg_tid)(uint8_t member)
    {
        return (uint32_t)__ockl_get_local_id(member);
    }

    size_t __ockl_get_local_size(uint32_t) __device__;
    uint32_t FUNC(sreg_ntid)(uint8_t member)
    {
        return (uint32_t)__ockl_get_local_size(member);
    }

    size_t __ockl_get_group_id(uint32_t) __device__;
    uint32_t FUNC(sreg_ctaid)(uint8_t member)
    {
        return (uint32_t)__ockl_get_group_id(member);
    }

    size_t __ockl_get_num_groups(uint32_t) __device__;
    uint32_t FUNC(sreg_nctaid)(uint8_t member)
    {
        return (uint32_t)__ockl_get_num_groups(member);
    }

    uint32_t FUNC(sreg_laneid)()
    {
        return __lane_id();
    }

    uint32_t FUNC(sreg_lanemask_lt)()
    {
        uint32_t lane_idx = FUNC_CALL(sreg_laneid)();
        return (1U << lane_idx) - 1U;
    }

    uint32_t FUNC(sreg_lanemask_ge)()
    {
        uint32_t lane_idx = FUNC_CALL(sreg_laneid)();
        return (~0U) << lane_idx;
    }

    uint32_t __ockl_bfe_u32(uint32_t, uint32_t, uint32_t) __device__;
    uint32_t FUNC(bfe_u32)(uint32_t base, uint32_t pos_32, uint32_t len_32)
    {
        uint32_t pos = pos_32 & 0xFFU;
        uint32_t len = len_32 & 0xFFU;
        if (pos >= 32)
            return 0;
        // V_BFE_U32 only uses bits [4:0] for len (max value is 31)
        if (len >= 32)
            return base >> pos;
        len = std::min(len, 31U);
        return __ockl_bfe_u32(base, pos, len);
    }

    // LLVM contains mentions of llvm.amdgcn.ubfe.i64 and llvm.amdgcn.sbfe.i64,
    // but using it only leads to LLVM crashes on RDNA2
    uint64_t FUNC(bfe_u64)(uint64_t base, uint32_t pos, uint32_t len)
    {
        // NVIDIA docs are incorrect. In 64 bit `bfe` both `pos` and `len`
        // parameters use whole 32 bit number and not just bottom 8 bits
        if (pos >= 64)
            return 0;
        if (len >= 64)
            return base >> pos;
        len = std::min(len, 63U);
        return (base >> pos) & ((1UL << len) - 1UL);
    }

    int32_t __ockl_bfe_i32(int32_t, uint32_t, uint32_t) __device__;
    int32_t FUNC(bfe_s32)(int32_t base, uint32_t pos_32, uint32_t len_32)
    {
        uint32_t pos = pos_32 & 0xFFU;
        uint32_t len = len_32 & 0xFFU;
        if (len == 0)
            return 0;
        if (pos >= 32)
            return (base >> 31);
        // V_BFE_I32 only uses bits [4:0] for len (max value is 31)
        if (len >= 32)
            return base >> pos;
        len = std::min(len, 31U);
        return __ockl_bfe_i32(base, pos, len);
    }

    static __device__ uint32_t add_sat(uint32_t x, uint32_t y)
    {
        uint32_t result;
        if (__builtin_add_overflow(x, y, &result))
        {
            return UINT32_MAX;
        }
        else
        {
            return result;
        }
    }

    int64_t FUNC(bfe_s64)(int64_t base, uint32_t pos, uint32_t len)
    {
        // NVIDIA docs are incorrect. In 64 bit `bfe` both `pos` and `len`
        // parameters use whole 32 bit number and not just bottom 8 bits
        if (len == 0)
            return 0;
        if (pos >= 64)
            return (base >> 63U);
        if (add_sat(pos, len) >= 64)
            len = 64 - pos;
        return (base << (64U - pos - len)) >> (64U - len);
    }

    uint32_t __ockl_bfm_u32(uint32_t count, uint32_t offset) __device__;
    uint32_t FUNC(bfi_b32)(uint32_t insert, uint32_t base, uint32_t pos_32, uint32_t len_32)
    {
        uint32_t pos = pos_32 & 0xFFU;
        uint32_t len = len_32 & 0xFFU;
        if (pos >= 32)
            return base;
        uint32_t mask;
        if (len >= 32)
            mask = UINT32_MAX << pos;
        else
            mask = __ockl_bfm_u32(len, pos);
        return (~mask & base) | (mask & (insert << pos));
    }

    uint64_t FUNC(bfi_b64)(uint64_t insert, uint64_t base, uint32_t pos, uint32_t len)
    {
        // NVIDIA docs are incorrect. In 64 bit `bfe` both `pos` and `len`
        // parameters use whole 32 bit number and not just bottom 8 bits
        if (pos >= 64)
            return base;
        uint64_t mask;
        if (len >= 64)
            mask = UINT64_MAX << pos;
        else
            mask = ((1UL << len) - 1UL) << (pos);
        return (~mask & base) | (mask & (insert << pos));
    }

    void FUNC(bar_sync)(uint32_t)
    {
        __builtin_amdgcn_fence(__ATOMIC_SEQ_CST, "workgroup");
        __builtin_amdgcn_s_barrier();
    }

    int32_t __ockl_wgred_and_i32(int32_t) __device__;
    int32_t __ockl_wgred_or_i32(int32_t) __device__;

#define BAR_RED_IMPL(reducer)                                                                                            \
    bool FUNC(bar_red_##reducer##_pred)(uint32_t barrier __attribute__((unused)), bool predicate, bool invert_predicate) \
    {                                                                                                                    \
        /* TODO: handle barrier */                                                                                       \
        return __ockl_wgred_##reducer##_i32(predicate ^ invert_predicate);                                               \
    }

    BAR_RED_IMPL(and);
    BAR_RED_IMPL(or);

    typedef uint32_t ShflSyncResult __attribute__((ext_vector_type(2)));

    // shfl.sync opts consists of two values, the warp end ID and the subsection mask.
    //
    // The current warp is partitioned into some number of subsections with a width of w. The
    // subsection mask is 32 - w, and indicates which bits of the lane id are part of the subsection
    // address. For example, if each subsection is 8 lanes wide, the subsection mask will be 24 –
    // 11000 in binary. This indicates that the two most significant bits in the 5-bit lane ID are
    // the subsection address. For example, for a lane ID 13 (0b01101) the address of the beginning
    // of the subsection is 0b01000 (8).
    //
    // The warp end ID is the max lane ID for a specific mode. For the CUDA __shfl_sync
    // intrinsics, it is always 31 for idx, bfly, and down, and 0 for up. This is used for the
    // bounds check.
#define SHFL_SYNC_IMPL(mode, calculate_index, CMP)                                                                                              \
    ShflSyncResult FUNC(shfl_sync_##mode##_b32_pred)(uint32_t input, int32_t delta, uint32_t opts, uint32_t membermask __attribute__((unused))) \
    {                                                                                                                                           \
        int32_t section_mask = (opts >> 8) & 0b11111;                                                                                           \
        int32_t warp_end = opts & 0b11111;                                                                                                      \
        int32_t self = (int32_t)__lane_id();                                                                                                    \
        int32_t subsection = section_mask & self;                                                                                               \
        int32_t subsection_end = subsection | (~section_mask & warp_end);                                                                       \
        int32_t idx = calculate_index;                                                                                                          \
        bool out_of_bounds = idx CMP subsection_end;                                                                                            \
        if (out_of_bounds)                                                                                                                      \
        {                                                                                                                                       \
            idx = self;                                                                                                                         \
        }                                                                                                                                       \
        int32_t output = __builtin_amdgcn_ds_bpermute(idx << 2, (int32_t)input);                                                                \
        return {(uint32_t)output, uint32_t(!out_of_bounds)};                                                                                    \
    }                                                                                                                                           \
                                                                                                                                                \
    uint32_t FUNC(shfl_sync_##mode##_b32)(uint32_t input, int32_t delta, uint32_t opts, uint32_t membermask)                                    \
    {                                                                                                                                           \
        return __zluda_ptx_impl_shfl_sync_##mode##_b32_pred(input, delta, opts, membermask).x;                                                  \
    }

    // We are using the HIP __shfl intrinsics to implement these, rather than the __shfl_sync
    // intrinsics, as those only add an assertion checking that the membermask is used correctly.
    // They do not return the result of the range check, so we must replicate that logic here.

    SHFL_SYNC_IMPL(up, self - delta, <);
    SHFL_SYNC_IMPL(down, self + delta, >);
    SHFL_SYNC_IMPL(bfly, self ^ delta, >);
    SHFL_SYNC_IMPL(idx, (delta & ~section_mask) | subsection, >);

    DECLARE_ATTR(uint32_t, CLOCK_RATE);
    void FUNC(nanosleep_u32)(uint32_t nanoseconds)
    {
        // clock_rate is in kHz
        uint64_t cycles_per_ns = ATTR(CLOCK_RATE) / 1000000;
        uint64_t cycles = nanoseconds * cycles_per_ns;
        // Avoid small sleep values resulting in s_sleep 0
        cycles += 63;
        // s_sleep N sleeps for 64 * N cycles
        uint64_t sleep_amount = cycles / 64;

        // The argument to s_sleep must be a constant
        for (size_t i = 0; i < sleep_amount >> 4; i++)
            __builtin_amdgcn_s_sleep(16);
        if (sleep_amount & 8U)
            __builtin_amdgcn_s_sleep(8);
        if (sleep_amount & 4U)
            __builtin_amdgcn_s_sleep(4);
        if (sleep_amount & 2U)
            __builtin_amdgcn_s_sleep(2);
        if (sleep_amount & 1U)
            __builtin_amdgcn_s_sleep(1);
    }

    void FUNC(__assertfail)(uint64_t message,
                            uint64_t file,
                            uint32_t line,
                            uint64_t function,
                            uint64_t char_size)
    {
        (void)char_size;
        __assert_fail((const char *)message, (const char *)file, line, (const char *)function);
    }

    // * Smallest denormal is 1.4 × 10^-45
    // * Smallest normal is ~1.175494351 × 10^(-38)
    // * Now, 1.175494351×10^-38 / 1.4 × 10^-45  = 8396388 + 31/140
    // * Next power of 2 is 16777216
    const float DENORMAL_TO_NORMAL_FACTOR_F32 = 16777216.0f;
    // * Largest subnormal is ~1.175494210692441e × 10^(-38)
    // * Then any value equal or larger than following will produce subnormals: 8.50706018714406320806444272332455743547934627837873057975602739772164... × 10^37
    const float RCP_DENORMAL_OUTPUT = 8.50706018714406320806444272332455743547934627837873057975602739772164e37f;
    const float REVERSE_DENORMAL_TO_NORMAL_FACTOR_F32 = 0.029387360490963111877208252592662410455594571842846914442095471744599661631813495980086003637902577995683214210345151992265999035207077609582844f;

    float FUNC(sqrt_approx_f32)(float x)
    {
        bool is_subnormal = __builtin_isfpclass(x, __FPCLASS_NEGSUBNORMAL | __FPCLASS_POSSUBNORMAL);
        float input = x;
        if (is_subnormal)
            input = x * DENORMAL_TO_NORMAL_FACTOR_F32;
        float value = __builtin_amdgcn_sqrtf(input);
        if (is_subnormal)
            return value * 0.000244140625f;
        else
            return value;
    }

    float FUNC(rsqrt_approx_f32)(float x)
    {
        bool is_subnormal = __builtin_isfpclass(x, __FPCLASS_NEGSUBNORMAL | __FPCLASS_POSSUBNORMAL);
        float input = x;
        if (is_subnormal)
            input = x * DENORMAL_TO_NORMAL_FACTOR_F32;
        float value = __builtin_amdgcn_rsqf(input);
        if (is_subnormal)
            return value * 4096.0f;
        else
            return value;
    }

    float FUNC(rcp_approx_f32)(float x)
    {
        float factor = 1.0f;
        if (__builtin_isfpclass(x, __FPCLASS_NEGSUBNORMAL | __FPCLASS_POSSUBNORMAL))
        {
            factor = DENORMAL_TO_NORMAL_FACTOR_F32;
        }
        if (std::fabs(x) >= RCP_DENORMAL_OUTPUT)
        {
            factor = REVERSE_DENORMAL_TO_NORMAL_FACTOR_F32;
        }
        return __builtin_amdgcn_rcpf(x * factor) * factor;
    }

    // When x = -126, exp2(x) = 2^(-126) ≈ 1.175494351 × 10^(-38),
    // which is the smallest normalized number in FP32
    float FUNC(ex2_approx_f32)(float x)
    {
        bool special_handling = x < -126.0f;
        float input = x;
        if (special_handling)
            input *= 0.5f;
        float result = __builtin_amdgcn_exp2f(input);
        if (special_handling)
            return result * result;
        else
            return result;
    }

    float FUNC(lg2_approx_f32)(float x)
    {
        bool is_subnormal = __builtin_isfpclass(x, __FPCLASS_NEGSUBNORMAL | __FPCLASS_POSSUBNORMAL);
        float input = x;
        if (is_subnormal)
            input = x * DENORMAL_TO_NORMAL_FACTOR_F32;
        float value = __builtin_amdgcn_logf(input);
        if (is_subnormal)
            return value - 24.0f;
        else
            return value;
    }

    // Logic taken from legalizeFSQRTF32/lowerFSQRTF32 in LLVM AMDGPU target
    __device__ static float precise_square_root(float x, bool needs_denorm_handling)
    {

        // Constants for denormal handling
        const float scale_threshold = 0x1.0p-96f;   // Very small value threshold
        const float scale_up_factor = 0x1.0p+32f;   // 2^32
        const float scale_down_factor = 0x1.0p-16f; // 2^-16

        // Check if input needs scaling (for very small values)
        bool need_scale = scale_threshold > x;
        auto scaled = scale_up_factor * x;

        // Scale up input if needed
        float sqrt_x = need_scale ? scaled : x;

        float sqrt_s;

        // Check if we need special denormal handling

        if (needs_denorm_handling)
        {
            // Use hardware sqrt as initial approximation
            sqrt_s = __builtin_sqrtf(sqrt_x); // Or equivalent hardware instruction

            // Bit manipulations to get next values up/down
            uint32_t sqrt_s_bits = std::bit_cast<uint32_t>(sqrt_s);

            // Next value down (subtract 1 from bit pattern)
            uint32_t sqrt_s_next_down_bits = sqrt_s_bits - 1;
            float sqrt_s_next_down = std::bit_cast<float>(sqrt_s_next_down_bits);

            // Calculate residual: x - sqrt_next_down * sqrt
            float neg_sqrt_s_next_down = -sqrt_s_next_down;
            float sqrt_vp = std::fma(neg_sqrt_s_next_down, sqrt_s, sqrt_x);

            // Next value up (add 1 to bit pattern)
            uint32_t sqrt_s_next_up_bits = sqrt_s_bits + 1;
            float sqrt_s_next_up = std::bit_cast<float>(sqrt_s_next_up_bits);

            // Calculate residual: x - sqrt_next_up * sqrt
            float neg_sqrt_s_next_up = -sqrt_s_next_up;
            float sqrt_vs = std::fma(neg_sqrt_s_next_up, sqrt_s, sqrt_x);

            // Select correctly rounded result
            if (sqrt_vp <= 0.0f)
            {
                sqrt_s = sqrt_s_next_down;
            }

            if (sqrt_vs > 0.0f)
            {
                sqrt_s = sqrt_s_next_up;
            }
        }
        else
        {
            // Use Newton-Raphson method with reciprocal square root

            // Initial approximation
            float sqrt_r = __builtin_amdgcn_rsqf(sqrt_x); // Or equivalent hardware 1/sqrt instruction
            sqrt_s = sqrt_x * sqrt_r;

            // Refine approximation
            float half = 0.5f;
            float sqrt_h = sqrt_r * half;
            float neg_sqrt_h = -sqrt_h;

            // Calculate error term
            float sqrt_e = std::fma(neg_sqrt_h, sqrt_s, half);

            // First refinement
            sqrt_h = std::fma(sqrt_h, sqrt_e, sqrt_h);
            sqrt_s = std::fma(sqrt_s, sqrt_e, sqrt_s);

            // Second refinement
            float neg_sqrt_s = -sqrt_s;
            float sqrt_d = std::fma(neg_sqrt_s, sqrt_s, sqrt_x);
            sqrt_s = std::fma(sqrt_d, sqrt_h, sqrt_s);
        }

        // Scale back if input was scaled
        if (need_scale)
        {
            sqrt_s *= scale_down_factor;
        }

        // Special case handling for zero and infinity
        bool is_zero_or_inf = __builtin_isfpclass(sqrt_x, __FPCLASS_POSINF | __FPCLASS_POSZERO | __FPCLASS_NEGZERO);

        return is_zero_or_inf ? sqrt_x : sqrt_s;
    }

    float FUNC(sqrt_rn_f32)(float x)
    {
        return precise_square_root(x, true);
    }

    float FUNC(sqrt_rn_ftz_f32)(float x)
    {
        return precise_square_root(x, false);
    }

    struct DivRnFtzF32Part1Result
    {
        float fma_4;
        float fma_1;
        float fma_3;
        uint8_t numerator_scaled_flag;
    };

    DivRnFtzF32Part1Result FUNC(div_f32_part1)(float lhs, float rhs)
    {
        float one = 1.0f;

        // Division scale operations
        bool denominator_scaled_flag;
        float denominator_scaled = __builtin_amdgcn_div_scalef(lhs, rhs, false, &denominator_scaled_flag);

        bool numerator_scaled_flag;
        float numerator_scaled = __builtin_amdgcn_div_scalef(lhs, rhs, true, &numerator_scaled_flag);

        // Reciprocal approximation
        float approx_rcp = __builtin_amdgcn_rcpf(denominator_scaled);
        float neg_div_scale0 = -denominator_scaled;

        // Perform division approximation steps
        float fma_0 = fmaf(neg_div_scale0, approx_rcp, one);
        float fma_1 = fmaf(fma_0, approx_rcp, approx_rcp);
        float mul = numerator_scaled * fma_1;
        float fma_2 = fmaf(neg_div_scale0, mul, numerator_scaled);
        float fma_3 = fmaf(fma_2, fma_1, mul);
        float fma_4 = fmaf(neg_div_scale0, fma_3, numerator_scaled);
        return {fma_4, fma_1, fma_3, numerator_scaled_flag};
    }

    __device__ static float div_f32_part2(float x, float y, DivRnFtzF32Part1Result part1)
    {
        float fmas = __builtin_amdgcn_div_fmasf(part1.fma_4, part1.fma_1, part1.fma_3, part1.numerator_scaled_flag);
        float result = __builtin_amdgcn_div_fixupf(fmas, y, x);

        return result;
    }

    float FUNC(div_f32_part2)(float x, float y,
                              float fma_4,
                              float fma_1,
                              float fma_3,
                              uint8_t numerator_scaled_flag)
    {
        return div_f32_part2(x, y, {fma_4, fma_1, fma_3, numerator_scaled_flag});
    }

    __device__ static __hip_fp8_storage_t cvt_float_to_fp8(float f, __hip_fp8_interpretation_t interp)
    {
        const uint32_t bits = reinterpret_cast<uint32_t &>(f);
        const uint8_t sign = (bits & 0x80000000) ? 0x80 : 0x0;
        const uint32_t abs = bits & 0x7fffffff;

        const uint32_t min = interp == __HIP_E4M3 ? 0x3A800000 : 0x37000000;
        if (abs < min)
        {
            return sign; // +/- 0
        }

        return __hip_cvt_float_to_fp8(f, __HIP_SATFINITE, interp);
    }

    struct Fp8x2
    {
        __hip_fp8_storage_t b : 8;
        __hip_fp8_storage_t a : 8;
    };

    Fp8x2 FUNC(cvt_rn_satfinite_e4m3x2_f32)(float a, float b)
    {
        // If built-in support for fp8 formats is added to LLVM IR we should switch to use that.
        return {cvt_float_to_fp8(b, __HIP_E4M3), cvt_float_to_fp8(a, __HIP_E4M3)};
    }

    Fp8x2 FUNC(cvt_rn_satfinite_e5m2x2_f32)(float a, float b)
    {
        return {cvt_float_to_fp8(b, __HIP_E5M2), cvt_float_to_fp8(a, __HIP_E5M2)};
    }

    __half2 FUNC(cvt_rn_f16x2_e4m3x2)(__hip_fp8x2_e4m3 in)
    {
        return in;
    }

    __half2 FUNC(cvt_rn_f16x2_e5m2x2)(__hip_fp8x2_e5m2 in)
    {
        return in;
    }

    __device__ static inline uint32_t ballot(bool value, bool negate)
    {
        __builtin_amdgcn_wave_barrier();
        return __builtin_amdgcn_ballot_w32(negate ? !value : value);
    }

    bool FUNC(vote_sync_any_pred)(bool value, uint32_t membermask __attribute__((unused)))
    {
        return ballot(value, false) != 0;
    }

    bool FUNC(vote_sync_any_pred_negate)(bool value, uint32_t membermask __attribute__((unused)))
    {
        return ballot(value, true) != 0;
    }

    // IMPORTANT: exec mask must be a subset of membermask, the behavior is undefined otherwise
    bool FUNC(vote_sync_all_pred)(bool value, uint32_t membermask __attribute__((unused)))
    {
        return ballot(value, false) == __builtin_amdgcn_read_exec_lo();
    }

    // also known as "none"
    bool FUNC(vote_sync_all_pred_negate)(bool value, uint32_t membermask __attribute__((unused)))
    {
        return ballot(value, false) == 0;
    }

    uint32_t FUNC(vote_sync_ballot_b32)(bool value, uint32_t membermask __attribute__((unused)))
    {
        return ballot(value, false);
    }

    uint32_t FUNC(vote_sync_ballot_b32_negate)(bool value, uint32_t membermask __attribute__((unused)))
    {
        return ballot(value, true);
    }

#define REDUX_SYNC_TYPE_IMPL(reducer, ptx_type, amd_type, cpp_type)                                             \
    cpp_type __ockl_wfred_##reducer##_##amd_type(cpp_type) __device__;                                          \
    cpp_type FUNC(redux_sync_##reducer##_##ptx_type)(cpp_type src, uint32_t membermask __attribute__((unused))) \
    {                                                                                                           \
        return __ockl_wfred_##reducer##_##amd_type(src);                                                        \
    }

#define REDUX_SYNC_IMPL(reducer)                      \
    REDUX_SYNC_TYPE_IMPL(reducer, u32, u32, uint32_t) \
    REDUX_SYNC_TYPE_IMPL(reducer, s32, i32, int32_t)

    REDUX_SYNC_IMPL(add);
    REDUX_SYNC_IMPL(min);
    REDUX_SYNC_IMPL(max);

    __device__ inline static uint32_t load_single_matrix(void SHARED_SPACE *lds_address, uint32_t warp_offset)
    {
        uint32_t laneid = __zluda_ptx_impl_sreg_laneid();
        int32_t row_address = __builtin_amdgcn_ds_bpermute((int32_t)(warp_offset + (laneid / 4U)) << 2U, (int32_t)lds_address);
        uint32_t matrix_cell_address = (uint32_t)row_address + ((laneid % 4) * 4);
        return *((uint32_t SHARED_SPACE *)matrix_cell_address);
    }

    __device__ inline static uint32_t load_single_matrix_trans(void SHARED_SPACE *lds_address, uint32_t warp_offset)
    {
        uint32_t laneid = __zluda_ptx_impl_sreg_laneid();
        int32_t row_address_lo = __builtin_amdgcn_ds_bpermute((int32_t)(warp_offset + ((laneid % 4U) * 2)) << 2U, (int32_t)lds_address);
        uint32_t address_lo = (uint32_t)row_address_lo + ((laneid / 4) * 2);
        uint16_t lo = *((uint16_t SHARED_SPACE *)address_lo);
        int32_t row_address_hi = __builtin_amdgcn_ds_bpermute((int32_t)(warp_offset + ((laneid % 4U) * 2) + 1) << 2U, (int32_t)lds_address);
        uint32_t address_hi = (uint32_t)row_address_hi + ((laneid / 4) * 2);
        uint16_t hi = *((uint16_t SHARED_SPACE *)address_hi);
        return std::bit_cast<uint32_t>(ushort2::Native_vec_{lo, hi});
    }

    uint2::Native_vec_ FUNC(ldmatrix_m8n8_x2_b16)(void SHARED_SPACE *address)
    {
        uint32_t x0 = load_single_matrix(address, 0);
        uint32_t x1 = load_single_matrix(address, 8);
        return uint2::Native_vec_{x0, x1};
    }

    uint4::Native_vec_ FUNC(ldmatrix_m8n8_x4_b16)(void SHARED_SPACE *address)
    {
        uint32_t x0 = load_single_matrix(address, 0);
        uint32_t x1 = load_single_matrix(address, 8);
        uint32_t x2 = load_single_matrix(address, 16);
        uint32_t x3 = load_single_matrix(address, 24);
        return uint4::Native_vec_{x0, x1, x2, x3};
    }

    uint4::Native_vec_ FUNC(ldmatrix_m8n8_x4_trans_b16)(void SHARED_SPACE *address)
    {
        uint32_t x0 = load_single_matrix_trans(address, 0);
        uint32_t x1 = load_single_matrix_trans(address, 8);
        uint32_t x2 = load_single_matrix_trans(address, 16);
        uint32_t x3 = load_single_matrix_trans(address, 24);
        return uint4::Native_vec_{x0, x1, x2, x3};
    }

    static inline __device__ _Float16 top16_as_fp16(uint32_t value)
    {
        uint16_t half_bits = static_cast<uint16_t>((value >> 16) & 0xFFFF);
        return *reinterpret_cast<_Float16 *>(&half_bits);
    }
    static inline __device__ _Float16 bottom16_as_fp16(uint32_t value)
    {
        uint16_t half_bits = static_cast<uint16_t>(value & 0xFFFF);
        return *reinterpret_cast<_Float16 *>(&half_bits);
    }

    static inline __device__ float bpermute_lane(int lane, float x)
    {
        return __hip_ds_bpermutef(4 * lane, x);
    }
    static inline __device__ uint32_t bpermute_lane(int lane, uint32_t x)
    {
        return __hip_ds_bpermute(4 * lane, x);
    }

    static __device__ half16 shuffle_a(uint4::Native_vec_ a_reg)
    {
        const unsigned lIdx = threadIdx.x;
        const int lane = lIdx % 16; // Lanes 0-15 (the other 16 lanes are a duplicate in w32 mode)
        half16 aFrag;

        for (int vGPR = 0; vGPR < 8; ++vGPR)
        {
            int cudaChunk = (vGPR / 4) * 2; // will be 0 or 2
            int cudaTID = (vGPR % 4 + lane * 4) % 32;
            uint32_t reg0, reg1;
            // Select the two consecutive elements from a_reg:
            if (cudaChunk == 0)
            {
                reg0 = a_reg.x;
                reg1 = a_reg.y;
            }
            else
            { // cudaChunk==2
                reg0 = a_reg.z;
                reg1 = a_reg.w;
            }
            uint32_t a_tmp0 = bpermute_lane(cudaTID, reg0);
            uint32_t a_tmp1 = bpermute_lane(cudaTID, reg1);
            uint32_t a_Frag_reg = (lane < 8) ? a_tmp0 : a_tmp1;
            aFrag[2 * vGPR] = bottom16_as_fp16(a_Frag_reg);
            aFrag[2 * vGPR + 1] = top16_as_fp16(a_Frag_reg);
        }
        return aFrag;
    }

    static __device__ half16 shuffle_b(uint2::Native_vec_ b_reg)
    {
        const unsigned lIdx = threadIdx.x;
        const int lane = lIdx % 16;
        half16 bFrag;

        for (int vGPR = 0; vGPR < 8; ++vGPR)
        {
            int cudaChunk = vGPR / 4; // will be 0 or 1
            int cudaTID = vGPR % 4 + (lane * 4) % 64;
            uint32_t reg = (cudaChunk == 0) ? b_reg.x : b_reg.y;
            uint32_t b_Frag_reg = bpermute_lane(cudaTID, reg);
            if (lane < 8)
            {
                bFrag[2 * vGPR] = bottom16_as_fp16(b_Frag_reg);
                bFrag[2 * vGPR + 1] = top16_as_fp16(b_Frag_reg);
            }
            else
            {
                bFrag[2 * vGPR] = 0.0f;
                bFrag[2 * vGPR + 1] = 0.0f;
            }
        }
        return bFrag;
    }

    static __device__ float8 shuffle_c(float4::Native_vec_ c_reg)
    {
        const int lIdx = (int)threadIdx.x;
        float8 cFrag;

        // Loop over the eight vector GPRs.
        for (int vGPR = 0; vGPR < 8; ++vGPR)
        {
            int cudaChunk = (vGPR / 4) * 2; // will be 0 or 2: selects which pair of components to use.
            int lIdx8 = (lIdx < 8) ? lIdx : lIdx - 8;
            int cudaTID = (vGPR % 4) * 8 + lIdx8 / 2;
            float ctmp0, ctmp1;

            if (cudaChunk == 0)
            {
                ctmp0 = bpermute_lane(cudaTID, c_reg.x);
                ctmp1 = bpermute_lane(cudaTID, c_reg.y);
            }
            else
            { // cudaChunk == 2
                ctmp0 = bpermute_lane(cudaTID, c_reg.z);
                ctmp1 = bpermute_lane(cudaTID, c_reg.w);
            }

            // Select one of the two values based on the thread index's LSB.
            cFrag[vGPR] = (lIdx & 1) ? ctmp1 : ctmp0;

            // Zero out for specific thread indices.
            if ((lIdx > 7 && lIdx < 16) || (lIdx > 23 && lIdx < 32))
                cFrag[vGPR] = 0.0f;
        }
        return cFrag;
    }

    static inline __device__ std::pair<uint32_t, uint32_t> select_registers_for_permlane(
        uint32_t laneid,
        uint32_t lower_regs[4],
        uint32_t upper_regs[4],
        uint32_t rotation)
    {
        uint32_t rotated_half_lane_id = (laneid + rotation) % 16;
        bool rotated_half_lower_half = rotated_half_lane_id < 8;
        if (rotated_half_lower_half)
        {
            uint32_t lower = lower_regs[rotated_half_lane_id % 2];
            uint32_t upper = upper_regs[rotated_half_lane_id % 2];
            return std::make_pair(lower, upper);
        }
        else
        {
            uint32_t lower = lower_regs[2 + (rotated_half_lane_id % 2)];
            uint32_t upper = upper_regs[2 + (rotated_half_lane_id % 2)];
            return std::make_pair(lower, upper);
        }
    }

    static __device__ float8 shuffle_d8(float8 registers)
    {
        uint32_t laneid = __zluda_ptx_impl_sreg_laneid();
        bool low_half = laneid < 16;
        // We start with:
        // T0       T1       T2       T3       T4       T5       T6       T7       T8       T9       T10       T11       T12       T13       T14       T15       T16      T17      T18      T19      T20      T21      T22      T23      T24      T25      T26       T27       T28       T29       T30       T31
        // -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------  -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------
        // (0, 0)   (0, 1)   (0, 2)   (0, 3)   (0, 4)   (0, 5)   (0, 6)   (0, 7)   (0, 8)   (0, 9)   (0, 10)   (0, 11)   (0, 12)   (0, 13)   (0, 14)   (0, 15)   (1, 0)   (1, 1)   (1, 2)   (1, 3)   (1, 4)   (1, 5)   (1, 6)   (1, 7)   (1, 8)   (1, 9)   (1, 10)   (1, 11)   (1, 12)   (1, 13)   (1, 14)   (1, 15)
        // (2, 0)   (2, 1)   (2, 2)   (2, 3)   (2, 4)   (2, 5)   (2, 6)   (2, 7)   (2, 8)   (2, 9)   (2, 10)   (2, 11)   (2, 12)   (2, 13)   (2, 14)   (2, 15)   (3, 0)   (3, 1)   (3, 2)   (3, 3)   (3, 4)   (3, 5)   (3, 6)   (3, 7)   (3, 8)   (3, 9)   (3, 10)   (3, 11)   (3, 12)   (3, 13)   (3, 14)   (3, 15)
        // (4, 0)   (4, 1)   (4, 2)   (4, 3)   (4, 4)   (4, 5)   (4, 6)   (4, 7)   (4, 8)   (4, 9)   (4, 10)   (4, 11)   (4, 12)   (4, 13)   (4, 14)   (4, 15)   (5, 0)   (5, 1)   (5, 2)   (5, 3)   (5, 4)   (5, 5)   (5, 6)   (5, 7)   (5, 8)   (5, 9)   (5, 10)   (5, 11)   (5, 12)   (5, 13)   (5, 14)   (5, 15)
        // (6, 0)   (6, 1)   (6, 2)   (6, 3)   (6, 4)   (6, 5)   (6, 6)   (6, 7)   (6, 8)   (6, 9)   (6, 10)   (6, 11)   (6, 12)   (6, 13)   (6, 14)   (6, 15)   (7, 0)   (7, 1)   (7, 2)   (7, 3)   (7, 4)   (7, 5)   (7, 6)   (7, 7)   (7, 8)   (7, 9)   (7, 10)   (7, 11)   (7, 12)   (7, 13)   (7, 14)   (7, 15)
        // (8, 0)   (8, 1)   (8, 2)   (8, 3)   (8, 4)   (8, 5)   (8, 6)   (8, 7)   (8, 8)   (8, 9)   (8, 10)   (8, 11)   (8, 12)   (8, 13)   (8, 14)   (8, 15)   (9, 0)   (9, 1)   (9, 2)   (9, 3)   (9, 4)   (9, 5)   (9, 6)   (9, 7)   (9, 8)   (9, 9)   (9, 10)   (9, 11)   (9, 12)   (9, 13)   (9, 14)   (9, 15)
        // (10, 0)  (10, 1)  (10, 2)  (10, 3)  (10, 4)  (10, 5)  (10, 6)  (10, 7)  (10, 8)  (10, 9)  (10, 10)  (10, 11)  (10, 12)  (10, 13)  (10, 14)  (10, 15)  (11, 0)  (11, 1)  (11, 2)  (11, 3)  (11, 4)  (11, 5)  (11, 6)  (11, 7)  (11, 8)  (11, 9)  (11, 10)  (11, 11)  (11, 12)  (11, 13)  (11, 14)  (11, 15)
        // (12, 0)  (12, 1)  (12, 2)  (12, 3)  (12, 4)  (12, 5)  (12, 6)  (12, 7)  (12, 8)  (12, 9)  (12, 10)  (12, 11)  (12, 12)  (12, 13)  (12, 14)  (12, 15)  (13, 0)  (13, 1)  (13, 2)  (13, 3)  (13, 4)  (13, 5)  (13, 6)  (13, 7)  (13, 8)  (13, 9)  (13, 10)  (13, 11)  (13, 12)  (13, 13)  (13, 14)  (13, 15)
        // (14, 0)  (14, 1)  (14, 2)  (14, 3)  (14, 4)  (14, 5)  (14, 6)  (14, 7)  (14, 8)  (14, 9)  (14, 10)  (14, 11)  (14, 12)  (14, 13)  (14, 14)  (14, 15)  (15, 0)  (15, 1)  (15, 2)  (15, 3)  (15, 4)  (15, 5)  (15, 6)  (15, 7)  (15, 8)  (15, 9)  (15, 10)  (15, 11)  (15, 12)  (15, 13)  (15, 14)  (15, 15)

        // And the result is:
        // T0      T1       T2       T3       T4      T5       T6       T7       T8       T9        T10       T11       T12      T13       T14       T15       T16      T17       T18       T19       T20      T21       T22       T23       T24      T25       T26       T27       T28      T29       T30       T31
        // ------  -------  -------  -------  ------  -------  -------  -------  -------  --------  --------  --------  -------  --------  --------  --------  -------  --------  --------  --------  -------  --------  --------  --------  -------  --------  --------  --------  -------  --------  --------  --------
        // (0, 0)  (0, 2)   (0, 4)   (0, 6)   (1, 0)  (1, 2)   (1, 4)   (1, 6)   (2, 0)   (2, 2)    (2, 4)    (2, 6)    (3, 0)   (3, 2)    (3, 4)    (3, 6)    (4, 0)   (4, 2)    (4, 4)    (4, 6)    (5, 0)   (5, 2)    (5, 4)    (5, 6)    (6, 0)   (6, 2)    (6, 4)    (6, 6)    (7, 0)   (7, 2)    (7, 4)    (7, 6)
        // (0, 1)  (0, 3)   (0, 5)   (0, 7)   (1, 1)  (1, 3)   (1, 5)   (1, 7)   (2, 1)   (2, 3)    (2, 5)    (2, 7)    (3, 1)   (3, 3)    (3, 5)    (3, 7)    (4, 1)   (4, 3)    (4, 5)    (4, 7)    (5, 1)   (5, 3)    (5, 5)    (5, 7)    (6, 1)   (6, 3)    (6, 5)    (6, 7)    (7, 1)   (7, 3)    (7, 5)    (7, 7)
        // (8, 0)  (8, 2)   (8, 4)   (8, 6)   (9, 0)  (9, 2)   (9, 4)   (9, 6)   (10, 0)  (10, 2)   (10, 4)   (10, 6)   (11, 0)  (11, 2)   (11, 4)   (11, 6)   (12, 0)  (12, 2)   (12, 4)   (12, 6)   (13, 0)  (13, 2)   (13, 4)   (13, 6)   (14, 0)  (14, 2)   (14, 4)   (14, 6)   (15, 0)  (15, 2)   (15, 4)   (15, 6)
        // (8, 1)  (8, 3)   (8, 5)   (8, 7)   (9, 1)  (9, 3)   (9, 5)   (9, 7)   (10, 1)  (10, 3)   (10, 5)   (10, 7)   (11, 1)  (11, 3)   (11, 5)   (11, 7)   (12, 1)  (12, 3)   (12, 5)   (12, 7)   (13, 1)  (13, 3)   (13, 5)   (13, 7)   (14, 1)  (14, 3)   (14, 5)   (14, 7)   (15, 1)  (15, 3)   (15, 5)   (15, 7)
        // (0, 8)  (0, 10)  (0, 12)  (0, 14)  (1, 8)  (1, 10)  (1, 12)  (1, 14)  (2, 8)   (2, 10)   (2, 12)   (2, 14)   (3, 8)   (3, 10)   (3, 12)   (3, 14)   (4, 8)   (4, 10)   (4, 12)   (4, 14)   (5, 8)   (5, 10)   (5, 12)   (5, 14)   (6, 8)   (6, 10)   (6, 12)   (6, 14)   (7, 8)   (7, 10)   (7, 12)   (7, 14)
        // (0, 9)  (0, 11)  (0, 13)  (0, 15)  (1, 9)  (1, 11)  (1, 13)  (1, 15)  (2, 9)   (2, 11)   (2, 13)   (2, 15)   (3, 9)   (3, 11)   (3, 13)   (3, 15)   (4, 9)   (4, 11)   (4, 13)   (4, 15)   (5, 9)   (5, 11)   (5, 13)   (5, 15)   (6, 9)   (6, 11)   (6, 13)   (6, 15)   (7, 9)   (7, 11)   (7, 13)   (7, 15)
        // (8, 8)  (8, 10)  (8, 12)  (8, 14)  (9, 8)  (9, 10)  (9, 12)  (9, 14)  (10, 8)  (10, 10)  (10, 12)  (10, 14)  (11, 8)  (11, 10)  (11, 12)  (11, 14)  (12, 8)  (12, 10)  (12, 12)  (12, 14)  (13, 8)  (13, 10)  (13, 12)  (13, 14)  (14, 8)  (14, 10)  (14, 12)  (14, 14)  (15, 8)  (15, 10)  (15, 12)  (15, 14)
        // (8, 9)  (8, 11)  (8, 13)  (8, 15)  (9, 9)  (9, 11)  (9, 13)  (9, 15)  (10, 9)  (10, 11)  (10, 13)  (10, 15)  (11, 9)  (11, 11)  (11, 13)  (11, 15)  (12, 9)  (12, 11)  (12, 13)  (12, 15)  (13, 9)  (13, 11)  (13, 13)  (13, 15)  (14, 9)  (14, 11)  (14, 13)  (14, 15)  (15, 9)  (15, 11)  (15, 13)  (15, 15)
        // We could do all the manipulations with ds_bpermutes, but RDNA
        // documentation says "It uses LDS hardware", which to me implies that
        // it does reduce LDS bandwidth for real LDS instructions. MMA operations
        // usually follow loads from LDS and precede stores to LDS, so we
        // do need all the LDS bandwidth we can get
        // That's why this function avoids ds_bpermute.

        // Approximate cost table (measured on RDNA3 using a crappy benchmark):
        // * v_permlanex16_b32 ~ 2.5 cycles
        // * ds_bpermute_b32 ~ 6 cycles
        // * v_mov_b32_dpp (unfolded) ~ 3 cycles

        uint32_t v0 = std::bit_cast<uint32_t>(registers[0]);
        uint32_t v1 = std::bit_cast<uint32_t>(registers[1]);
        uint32_t v2 = std::bit_cast<uint32_t>(registers[2]);
        uint32_t v3 = std::bit_cast<uint32_t>(registers[3]);
        uint32_t v4 = std::bit_cast<uint32_t>(registers[4]);
        uint32_t v5 = std::bit_cast<uint32_t>(registers[5]);
        uint32_t v6 = std::bit_cast<uint32_t>(registers[6]);
        uint32_t v7 = std::bit_cast<uint32_t>(registers[7]);

        // Moving registers to prepare for permlanex16:
        // T0       T1       T2       T3       T4       T5       T6       T7       T8       T9       T10       T11       T12       T13       T14       T15       T16      T17      T18      T19      T20      T21      T22      T23      T24      T25      T26       T27       T28       T29       T30       T31
        // -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------  -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------
        // (4, 0)   (4, 1)   (4, 2)   (4, 3)   (4, 4)   (4, 5)   (4, 6)   (4, 7)   (4, 8)   (4, 9)   (4, 10)   (4, 11)   (4, 12)   (4, 13)   (4, 14)   (4, 15)   (1, 0)   (1, 1)   (1, 2)   (1, 3)   (1, 4)   (1, 5)   (1, 6)   (1, 7)   (1, 8)   (1, 9)   (1, 10)   (1, 11)   (1, 12)   (1, 13)   (1, 14)   (1, 15)
        // (6, 0)   (6, 1)   (6, 2)   (6, 3)   (6, 4)   (6, 5)   (6, 6)   (6, 7)   (6, 8)   (6, 9)   (6, 10)   (6, 11)   (6, 12)   (6, 13)   (6, 14)   (6, 15)   (3, 0)   (3, 1)   (3, 2)   (3, 3)   (3, 4)   (3, 5)   (3, 6)   (3, 7)   (3, 8)   (3, 9)   (3, 10)   (3, 11)   (3, 12)   (3, 13)   (3, 14)   (3, 15)
        // (0, 0)   (0, 1)   (0, 2)   (0, 3)   (0, 4)   (0, 5)   (0, 6)   (0, 7)   (0, 8)   (0, 9)   (0, 10)   (0, 11)   (0, 12)   (0, 13)   (0, 14)   (0, 15)   (5, 0)   (5, 1)   (5, 2)   (5, 3)   (5, 4)   (5, 5)   (5, 6)   (5, 7)   (5, 8)   (5, 9)   (5, 10)   (5, 11)   (5, 12)   (5, 13)   (5, 14)   (5, 15)
        // (2, 0)   (2, 1)   (2, 2)   (2, 3)   (2, 4)   (2, 5)   (2, 6)   (2, 7)   (2, 8)   (2, 9)   (2, 10)   (2, 11)   (2, 12)   (2, 13)   (2, 14)   (2, 15)   (7, 0)   (7, 1)   (7, 2)   (7, 3)   (7, 4)   (7, 5)   (7, 6)   (7, 7)   (7, 8)   (7, 9)   (7, 10)   (7, 11)   (7, 12)   (7, 13)   (7, 14)   (7, 15)
        // (12, 0)  (12, 1)  (12, 2)  (12, 3)  (12, 4)  (12, 5)  (12, 6)  (12, 7)  (12, 8)  (12, 9)  (12, 10)  (12, 11)  (12, 12)  (12, 13)  (12, 14)  (12, 15)  (9, 0)   (9, 1)   (9, 2)   (9, 3)   (9, 4)   (9, 5)   (9, 6)   (9, 7)   (9, 8)   (9, 9)   (9, 10)   (9, 11)   (9, 12)   (9, 13)   (9, 14)   (9, 15)
        // (14, 0)  (14, 1)  (14, 2)  (14, 3)  (14, 4)  (14, 5)  (14, 6)  (14, 7)  (14, 8)  (14, 9)  (14, 10)  (14, 11)  (14, 12)  (14, 13)  (14, 14)  (14, 15)  (11, 0)  (11, 1)  (11, 2)  (11, 3)  (11, 4)  (11, 5)  (11, 6)  (11, 7)  (11, 8)  (11, 9)  (11, 10)  (11, 11)  (11, 12)  (11, 13)  (11, 14)  (11, 15)
        // (8, 0)   (8, 1)   (8, 2)   (8, 3)   (8, 4)   (8, 5)   (8, 6)   (8, 7)   (8, 8)   (8, 9)   (8, 10)   (8, 11)   (8, 12)   (8, 13)   (8, 14)   (8, 15)   (13, 0)  (13, 1)  (13, 2)  (13, 3)  (13, 4)  (13, 5)  (13, 6)  (13, 7)  (13, 8)  (13, 9)  (13, 10)  (13, 11)  (13, 12)  (13, 13)  (13, 14)  (13, 15)
        // (10, 0)  (10, 1)  (10, 2)  (10, 3)  (10, 4)  (10, 5)  (10, 6)  (10, 7)  (10, 8)  (10, 9)  (10, 10)  (10, 11)  (10, 12)  (10, 13)  (10, 14)  (10, 15)  (15, 0)  (15, 1)  (15, 2)  (15, 3)  (15, 4)  (15, 5)  (15, 6)  (15, 7)  (15, 8)  (15, 9)  (15, 10)  (15, 11)  (15, 12)  (15, 13)  (15, 14)  (15, 15)
        if (low_half)
        {
            std::swap(v0, v2);
            std::swap(v1, v3);
            std::swap(v4, v6);
            std::swap(v5, v7);
        }

        // Transfer halves with permlanex16:
        // T0       T1       T2       T3       T4       T5       T6       T7       T8       T9       T10       T11       T12       T13       T14       T15       T16      T17      T18      T19      T20      T21      T22      T23      T24      T25      T26       T27       T28       T29       T30       T31
        // -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------  -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------
        // (1, 0)   (1, 1)   (1, 2)   (1, 3)   (1, 4)   (1, 5)   (1, 6)   (1, 7)   (1, 8)   (1, 9)   (1, 10)   (1, 11)   (1, 12)   (1, 13)   (1, 14)   (1, 15)   (4, 0)   (4, 1)   (4, 2)   (4, 3)   (4, 4)   (4, 5)   (4, 6)   (4, 7)   (4, 8)   (4, 9)   (4, 10)   (4, 11)   (4, 12)   (4, 13)   (4, 14)   (4, 15)
        // (3, 0)   (3, 1)   (3, 2)   (3, 3)   (3, 4)   (3, 5)   (3, 6)   (3, 7)   (3, 8)   (3, 9)   (3, 10)   (3, 11)   (3, 12)   (3, 13)   (3, 14)   (3, 15)   (6, 0)   (6, 1)   (6, 2)   (6, 3)   (6, 4)   (6, 5)   (6, 6)   (6, 7)   (6, 8)   (6, 9)   (6, 10)   (6, 11)   (6, 12)   (6, 13)   (6, 14)   (6, 15)
        // (0, 0)   (0, 1)   (0, 2)   (0, 3)   (0, 4)   (0, 5)   (0, 6)   (0, 7)   (0, 8)   (0, 9)   (0, 10)   (0, 11)   (0, 12)   (0, 13)   (0, 14)   (0, 15)   (5, 0)   (5, 1)   (5, 2)   (5, 3)   (5, 4)   (5, 5)   (5, 6)   (5, 7)   (5, 8)   (5, 9)   (5, 10)   (5, 11)   (5, 12)   (5, 13)   (5, 14)   (5, 15)
        // (2, 0)   (2, 1)   (2, 2)   (2, 3)   (2, 4)   (2, 5)   (2, 6)   (2, 7)   (2, 8)   (2, 9)   (2, 10)   (2, 11)   (2, 12)   (2, 13)   (2, 14)   (2, 15)   (7, 0)   (7, 1)   (7, 2)   (7, 3)   (7, 4)   (7, 5)   (7, 6)   (7, 7)   (7, 8)   (7, 9)   (7, 10)   (7, 11)   (7, 12)   (7, 13)   (7, 14)   (7, 15)
        // (9, 0)   (9, 1)   (9, 2)   (9, 3)   (9, 4)   (9, 5)   (9, 6)   (9, 7)   (9, 8)   (9, 9)   (9, 10)   (9, 11)   (9, 12)   (9, 13)   (9, 14)   (9, 15)   (12, 0)  (12, 1)  (12, 2)  (12, 3)  (12, 4)  (12, 5)  (12, 6)  (12, 7)  (12, 8)  (12, 9)  (12, 10)  (12, 11)  (12, 12)  (12, 13)  (12, 14)  (12, 15)
        // (11, 0)  (11, 1)  (11, 2)  (11, 3)  (11, 4)  (11, 5)  (11, 6)  (11, 7)  (11, 8)  (11, 9)  (11, 10)  (11, 11)  (11, 12)  (11, 13)  (11, 14)  (11, 15)  (14, 0)  (14, 1)  (14, 2)  (14, 3)  (14, 4)  (14, 5)  (14, 6)  (14, 7)  (14, 8)  (14, 9)  (14, 10)  (14, 11)  (14, 12)  (14, 13)  (14, 14)  (14, 15)
        // (8, 0)   (8, 1)   (8, 2)   (8, 3)   (8, 4)   (8, 5)   (8, 6)   (8, 7)   (8, 8)   (8, 9)   (8, 10)   (8, 11)   (8, 12)   (8, 13)   (8, 14)   (8, 15)   (13, 0)  (13, 1)  (13, 2)  (13, 3)  (13, 4)  (13, 5)  (13, 6)  (13, 7)  (13, 8)  (13, 9)  (13, 10)  (13, 11)  (13, 12)  (13, 13)  (13, 14)  (13, 15)
        // (10, 0)  (10, 1)  (10, 2)  (10, 3)  (10, 4)  (10, 5)  (10, 6)  (10, 7)  (10, 8)  (10, 9)  (10, 10)  (10, 11)  (10, 12)  (10, 13)  (10, 14)  (10, 15)  (15, 0)  (15, 1)  (15, 2)  (15, 3)  (15, 4)  (15, 5)  (15, 6)  (15, 7)  (15, 8)  (15, 9)  (15, 10)  (15, 11)  (15, 12)  (15, 13)  (15, 14)  (15, 15)
        constexpr uint32_t permlanex16_mask_lo = 0b0111'0110'0101'0100'0011'0010'0001'0000;
        constexpr uint32_t permlanex16_mask_hi = 0b1111'1110'1101'1100'1011'1010'1001'1000;
        v0 = __builtin_amdgcn_permlanex16(v0, v0, permlanex16_mask_lo, permlanex16_mask_hi, true, true);
        v1 = __builtin_amdgcn_permlanex16(v1, v1, permlanex16_mask_lo, permlanex16_mask_hi, true, true);
        v4 = __builtin_amdgcn_permlanex16(v4, v4, permlanex16_mask_lo, permlanex16_mask_hi, true, true);
        v5 = __builtin_amdgcn_permlanex16(v5, v5, permlanex16_mask_lo, permlanex16_mask_hi, true, true);

        // Readjusting rows in the lower half to match the ordering of the upper half:
        // T0       T1       T2       T3       T4       T5       T6       T7       T8       T9       T10       T11       T12       T13       T14       T15       T16      T17      T18      T19      T20      T21      T22      T23      T24      T25      T26       T27       T28       T29       T30       T31
        // -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------  -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------
        // (0, 0)   (0, 1)   (0, 2)   (0, 3)   (0, 4)   (0, 5)   (0, 6)   (0, 7)   (0, 8)   (0, 9)   (0, 10)   (0, 11)   (0, 12)   (0, 13)   (0, 14)   (0, 15)   (4, 0)   (4, 1)   (4, 2)   (4, 3)   (4, 4)   (4, 5)   (4, 6)   (4, 7)   (4, 8)   (4, 9)   (4, 10)   (4, 11)   (4, 12)   (4, 13)   (4, 14)   (4, 15)
        // (2, 0)   (2, 1)   (2, 2)   (2, 3)   (2, 4)   (2, 5)   (2, 6)   (2, 7)   (2, 8)   (2, 9)   (2, 10)   (2, 11)   (2, 12)   (2, 13)   (2, 14)   (2, 15)   (6, 0)   (6, 1)   (6, 2)   (6, 3)   (6, 4)   (6, 5)   (6, 6)   (6, 7)   (6, 8)   (6, 9)   (6, 10)   (6, 11)   (6, 12)   (6, 13)   (6, 14)   (6, 15)
        // (1, 0)   (1, 1)   (1, 2)   (1, 3)   (1, 4)   (1, 5)   (1, 6)   (1, 7)   (1, 8)   (1, 9)   (1, 10)   (1, 11)   (1, 12)   (1, 13)   (1, 14)   (1, 15)   (5, 0)   (5, 1)   (5, 2)   (5, 3)   (5, 4)   (5, 5)   (5, 6)   (5, 7)   (5, 8)   (5, 9)   (5, 10)   (5, 11)   (5, 12)   (5, 13)   (5, 14)   (5, 15)
        // (3, 0)   (3, 1)   (3, 2)   (3, 3)   (3, 4)   (3, 5)   (3, 6)   (3, 7)   (3, 8)   (3, 9)   (3, 10)   (3, 11)   (3, 12)   (3, 13)   (3, 14)   (3, 15)   (7, 0)   (7, 1)   (7, 2)   (7, 3)   (7, 4)   (7, 5)   (7, 6)   (7, 7)   (7, 8)   (7, 9)   (7, 10)   (7, 11)   (7, 12)   (7, 13)   (7, 14)   (7, 15)
        // (8, 0)   (8, 1)   (8, 2)   (8, 3)   (8, 4)   (8, 5)   (8, 6)   (8, 7)   (8, 8)   (8, 9)   (8, 10)   (8, 11)   (8, 12)   (8, 13)   (8, 14)   (8, 15)   (12, 0)  (12, 1)  (12, 2)  (12, 3)  (12, 4)  (12, 5)  (12, 6)  (12, 7)  (12, 8)  (12, 9)  (12, 10)  (12, 11)  (12, 12)  (12, 13)  (12, 14)  (12, 15)
        // (10, 0)  (10, 1)  (10, 2)  (10, 3)  (10, 4)  (10, 5)  (10, 6)  (10, 7)  (10, 8)  (10, 9)  (10, 10)  (10, 11)  (10, 12)  (10, 13)  (10, 14)  (10, 15)  (14, 0)  (14, 1)  (14, 2)  (14, 3)  (14, 4)  (14, 5)  (14, 6)  (14, 7)  (14, 8)  (14, 9)  (14, 10)  (14, 11)  (14, 12)  (14, 13)  (14, 14)  (14, 15)
        // (9, 0)   (9, 1)   (9, 2)   (9, 3)   (9, 4)   (9, 5)   (9, 6)   (9, 7)   (9, 8)   (9, 9)   (9, 10)   (9, 11)   (9, 12)   (9, 13)   (9, 14)   (9, 15)   (13, 0)  (13, 1)  (13, 2)  (13, 3)  (13, 4)  (13, 5)  (13, 6)  (13, 7)  (13, 8)  (13, 9)  (13, 10)  (13, 11)  (13, 12)  (13, 13)  (13, 14)  (13, 15)
        // (11, 0)  (11, 1)  (11, 2)  (11, 3)  (11, 4)  (11, 5)  (11, 6)  (11, 7)  (11, 8)  (11, 9)  (11, 10)  (11, 11)  (11, 12)  (11, 13)  (11, 14)  (11, 15)  (15, 0)  (15, 1)  (15, 2)  (15, 3)  (15, 4)  (15, 5)  (15, 6)  (15, 7)  (15, 8)  (15, 9)  (15, 10)  (15, 11)  (15, 12)  (15, 13)  (15, 14)  (15, 15)
        if (low_half)
        {
            std::swap(v0, v2);
            std::swap(v1, v3);
            std::swap(v4, v6);
            std::swap(v5, v7);
        }

        // Rotate to avoid a value required by separate permlane threads being held in the same thread, but different registers:
        // T0       T1       T2       T3       T4       T5       T6       T7       T8       T9       T10       T11       T12       T13       T14       T15
        // -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------
        // [0, 0]   <0, 1>   [0, 2]   <0, 3>   [0, 4]   <0, 5>   [0, 6]   <0, 7>   {0, 8}   (0, 9)   {0, 10}   (0, 11)   {0, 12}   (0, 13)   {0, 14}   (0, 15)  // rotate 0
        // (2, 15)  [2, 0]   <2, 1>   [2, 2]   <2, 3>   [2, 4]   <2, 5>   [2, 6]   <2, 7>   {2, 8}   (2, 9)    {2, 10}   (2, 11)   {2, 12}   (2, 13)   {2, 14}  // rotate 1
        // {1, 8}   (1, 9)   {1, 10}  (1, 11)  {1, 12}  (1, 13)  {1, 14}  (1, 15)  [1, 0]   <1, 1>   [1, 2]    <1, 3>    [1, 4]    <1, 5>    [1, 6]    <1, 7>   // rotate 8
        // <3, 7>   {3, 8}   (3, 9)   {3, 10}  (3, 11)  {3, 12}  (3, 13)  {3, 14}  (3, 15)  [3, 0]   <3, 1>    [3, 2]    <3, 3>    [3, 4]    <3, 5>    [3, 6]   // rotate 9
        // T0       T1       T2       T3       T4       T5       T6       T7       T8       T9       T10       T11       T12       T13       T14       T15
        // -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------
        // [8, 0]   <8, 1>   [8, 2]   <8, 3>   [8, 4]   <8, 5>   [8, 6]   <8, 7>   {8, 8}   (8, 9)   {8, 10}   (8, 11)   {8, 12}   (8, 13)   {8, 14}   (8, 15)  // rotate 0
        // (10, 15) [10, 0]  <10, 1>  [10, 2]  <10, 3>  [10, 4]  <10, 5>  [10, 6]  <10, 7>  {10, 8}  (10, 9)   {10, 10}  (10, 11)  {10, 12}  (10, 13)  {10, 14} // rotate 1
        // {9, 8}   (9, 9)   {9, 10}  (9, 11)  {9, 12}  (9, 13)  {9, 14}  (9, 15)  [9, 0]   <9, 1>   [9, 2]    <9, 3>    [9, 4]    <9, 5>    [9, 6]    <9, 7>   // rotate 8
        // <11, 7>  {11, 8}  (11, 9)  {11, 10} (11, 11) {11, 12} (11, 13) {11, 14} (11, 15) [11, 0]  <11, 1>   [11, 2]   <11, 3>   [11, 4]   <11, 5>   [11, 6]  // rotate 9
        v1 = std::bit_cast<uint32_t>(__builtin_amdgcn_mov_dpp(std::bit_cast<int32_t>(v1), 0x121, 0xf, 0xf, true));
        v2 = std::bit_cast<uint32_t>(__builtin_amdgcn_mov_dpp(std::bit_cast<int32_t>(v2), 0x128, 0xf, 0xf, true));
        v3 = std::bit_cast<uint32_t>(__builtin_amdgcn_mov_dpp(std::bit_cast<int32_t>(v3), 0x129, 0xf, 0xf, true));
        v5 = std::bit_cast<uint32_t>(__builtin_amdgcn_mov_dpp(std::bit_cast<int32_t>(v5), 0x121, 0xf, 0xf, true));
        v6 = std::bit_cast<uint32_t>(__builtin_amdgcn_mov_dpp(std::bit_cast<int32_t>(v6), 0x128, 0xf, 0xf, true));
        v7 = std::bit_cast<uint32_t>(__builtin_amdgcn_mov_dpp(std::bit_cast<int32_t>(v7), 0x129, 0xf, 0xf, true));

        // Move values into correct registers:
        // T0       T1       T2       T3       T4       T5       T6       T7       T8       T9       T10       T11       T12       T13       T14       T15
        // -------  -------  -------  -------  -------  -------  -------  -------  -------  -------  --------  --------  --------  --------  --------  --------
        // [0, 0]   [2, 0]   [0, 2]   [2, 2]   [0, 4]   [2, 4]   [0, 6]   [2, 6]   [1, 0]   [3, 0]   [1, 2]   [3, 2]     [1, 4]    [3, 4]    [1, 6]    [3, 6]
        // <3, 7>   <0, 1>   <2, 1>   <0, 3>   <2, 3>   <0, 5>   <2, 5>   <0, 7>   <2, 7>   <1, 1>   <3, 1>   <1, 3>     <3, 3>    <1, 5>    <3, 5>    <1, 7>
        // [8, 0]   [10, 0]  [8, 2]   [10, 2]  [8, 4]   [10, 4]  [8, 6]   [10, 6]  [9, 0]   [11, 0]  [9, 2]   [11, 2]    [9, 4]    [11, 4]   [9, 6]    [11, 6]
        // <11, 7>  <8, 1>   <10, 1>  <8, 3>   <10, 3>  <8, 5>   <10, 5>  <8, 7>   <10, 7>  <9, 1>   <11, 1>  <9, 3>     <11, 3>   <9, 5>    <11, 5>   <9, 7>
        // {1, 8}   {3, 8}   {1, 10}  {3, 10}  {1, 12}  {3, 12}  {1, 14}  {3, 14}  {0, 8}   {2, 8}   {0, 10}  {2, 10}    {0, 12}   {2, 12}   {0, 14}   {2, 14}
        // (2, 15)  (1, 9)   (3, 9)   (1, 11)  (3, 11)  (1, 13)  (3, 13)  (1, 15)  (3, 15)  (0, 9)   (2, 9)   (0, 11)    (2, 11)   (0, 13)   (2, 13)   (0, 15)
        // {9, 8}   {11, 8}  {9, 10}  {11, 10} {9, 12}  {11, 12} {9, 14}  {11, 14} {8, 8}   {10, 8}  {8, 10}  {10, 10}   {8, 12}   {10, 12}  {8, 14}   {10, 14}
        // (10, 15) (9, 9)   (11, 9)  (9, 11)  (11, 11) (9, 13)  (11, 13) (9, 15)  (11, 15) (8, 9)   (10, 9)  (8, 11)    (10, 11)  (8, 13)   (10, 13)  (8, 15)
        uint32_t lower_regs[4] = {v0, v1, v2, v3};
        uint32_t upper_regs[4] = {v4, v5, v6, v7};
        std::tie(v0, v2) = select_registers_for_permlane(laneid, lower_regs, upper_regs, 0);
        std::tie(v1, v3) = select_registers_for_permlane(laneid, lower_regs, upper_regs, 1);
        std::tie(v4, v6) = select_registers_for_permlane(laneid, lower_regs, upper_regs, 8);
        std::tie(v5, v7) = select_registers_for_permlane(laneid, lower_regs, upper_regs, 9);

        // Do permlane operations to finalize shuffle:
        v0 = __builtin_amdgcn_permlane16(v0, v0, 0xeca86420, 0xfdb97531, true, true);
        v1 = __builtin_amdgcn_permlane16(v1, v1, 0xfdb97531, 0x0eca8642, true, true);
        v2 = __builtin_amdgcn_permlane16(v2, v2, 0xeca86420, 0xfdb97531, true, true);
        v3 = __builtin_amdgcn_permlane16(v3, v3, 0xfdb97531, 0x0eca8642, true, true);
        v4 = __builtin_amdgcn_permlane16(v4, v4, 0x6420eca8, 0x7531fdb9, true, true);
        v5 = __builtin_amdgcn_permlane16(v5, v5, 0x7531fdb9, 0x6420eca8, true, true);
        v6 = __builtin_amdgcn_permlane16(v6, v6, 0x6420eca8, 0x7531fdb9, true, true);
        v7 = __builtin_amdgcn_permlane16(v7, v7, 0x7531fdb9, 0x6420eca8, true, true);

        return float8{
            std::bit_cast<float>(v0),
            std::bit_cast<float>(v1),
            std::bit_cast<float>(v2),
            std::bit_cast<float>(v3),
            std::bit_cast<float>(v4),
            std::bit_cast<float>(v5),
            std::bit_cast<float>(v6),
            std::bit_cast<float>(v7)};
    }

    static inline __device__ float4::Native_vec_ shuffle_d(float8 dFrag)
    {
        auto result = shuffle_d8(dFrag);
        return float4::Native_vec_{result.x, result.y, result.z, result.w};
    }

    float4::Native_vec_ FUNC(mma_sync_aligned_m16n8k16_row_col_f32_f16_f16_f32)(uint4::Native_vec_ a_reg, uint2::Native_vec_ b_reg, float4::Native_vec_ c_reg)
    {
        // Reshuffle from Nvidia-like register layout to AMD layout:
        half16 aFrag = shuffle_a(a_reg);
        half16 bFrag = shuffle_b(b_reg);
        float8 cFrag = shuffle_c(c_reg);

        // Call the (built‐in) 16x16 MMA instruction. It returns a float8.
        float8 dFrag = __builtin_amdgcn_wmma_f32_16x16x16_f16_w32(aFrag, bFrag, cFrag);

        // Unshuffle back into Nvidia expected float4 result
        float4::Native_vec_ d_out = shuffle_d(dFrag);

        return d_out;
    }

    float4::Native_vec_ FUNC(mma_sync_aligned_m16n8k16_row_col_f32_bf16_bf16_f32)(uint4::Native_vec_ a_reg, uint2::Native_vec_ b_reg, float4::Native_vec_ c_reg)
    {
        // Reshuffle from Nvidia-like register layout to AMD layout:
        half16 aFrag = shuffle_a(a_reg);
        half16 bFrag = shuffle_b(b_reg);
        float8 cFrag = shuffle_c(c_reg);

        // Call the (built‐in) 16x16 MMA instruction. It returns a float8.
        float8 dFrag = __builtin_amdgcn_wmma_f32_16x16x16_bf16_w32(aFrag, bFrag, cFrag);

        // Unshuffle back into Nvidia expected float4 result
        float4::Native_vec_ d_out = shuffle_d(dFrag);

        return d_out;
    }

    struct byte4
    {
        union
        {
            uint32_t u32;
            uint8_t u8x4[4];
        };
    } __attribute__((aligned(4)));

    struct byte8
    {
        union
        {
            uint32_t u32x2[2];
            uint8_t u8x8[8];
        };
    } __attribute__((aligned(8)));

    uint32_t FUNC(prmt_b32)(uint32_t x, uint32_t y, uint32_t s)
    {
        byte4 v_perm_selector;
        v_perm_selector.u32 = 0;

        byte8 input;
        input.u32x2[0] = x;
        input.u32x2[1] = y;

        for (size_t i = 0; i < 4; i++)
        {
            uint8_t sel = static_cast<uint8_t>(s >> (i * 4));
            uint8_t addr = sel & 0x7;
            if (sel & 0x8)
            {
                if (addr % 2 == 1)
                {
                    v_perm_selector.u8x4[i] = 0x8 + addr / 2;
                    continue;
                }
            }
            v_perm_selector.u8x4[i] = addr;
        }

        byte4 output;
        output.u32 = __builtin_amdgcn_perm(input.u32x2[1], input.u32x2[0], v_perm_selector.u32);

        for (size_t i = 0; i < 4; i++)
        {
            uint8_t sel = static_cast<uint8_t>(s >> (i * 4));
            uint8_t addr = sel & 0x7;
            if (sel & 0x8)
            {
                if (addr % 2 != 1)
                {
                    output.u8x4[i] = (output.u8x4[i] & 0x80) * 0xff;
                    continue;
                }
            }
        }

        return output.u32;
    }

    int FUNC(vprintf)(const char *format __attribute__((unused)), void *vlist __attribute__((unused)))
    {
        // TODO: replace calls to vprintf with a raising pass to printf when we have a mechanism
        // to write SSA passes
        // Use https://github.com/ROCm/llvm-project/blob/99a81d16b9d811cadd420190bed16981a0a57bc6/llvm/lib/Transforms/Utils/AMDGPUEmitPrintf.cpp#L426
        return -1;
    }
}
