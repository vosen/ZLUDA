// Every time this file changes it must te rebuilt, you need `rocm-llvm-dev` and `llvm-17`
// `fdenormal-fp-math=dynamic` is required to make functions eligible for inlining
//  /opt/rocm/llvm/bin/clang -std=c++20 -Xclang -fdenormal-fp-math=dynamic  -Wall -Wextra -Wsign-compare -Wconversion -x hip zluda_ptx_impl.cpp -nogpulib -O3 -mno-wavefrontsize64 -o zluda_ptx_impl.bc -emit-llvm -c --offload-device-only --offload-arch=gfx1010 && /opt/rocm/llvm/bin/llvm-dis zluda_ptx_impl.bc -o - | sed '/@llvm.used/d' | sed '/wchar_size/d' | sed '/llvm.module.flags/d' | sed 's/define hidden/define linkonce_odr/g' | sed 's/\"target-cpu\"=\"gfx1010\"//g' | sed -E 's/\"target-features\"=\"[^\"]+\"//g' | sed 's/ nneg / /g' | sed 's/ disjoint / /g' | sed '/__hip_cuid/d' | sed 's/external protected/external hidden/g' | llvm-as-17 - -o  zluda_ptx_impl.bc && /opt/rocm/llvm/bin/llvm-dis zluda_ptx_impl.bc

#include <cstddef>
#include <cstdint>
#include <bit>
#include <cmath>
#include <hip/amd_detail/amd_device_functions.h>

#define CONSTANT_SPACE __attribute__((address_space(4)))

#define FUNC(NAME) __device__ __attribute__((retain)) __zluda_ptx_impl_##NAME
#define ATTR(NAME) __ZLUDA_PTX_IMPL_ATTRIBUTE_##NAME
#define DECLARE_ATTR(TYPE, NAME) \
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
        return {(uint32_t)output, uint32_t(!out_of_bounds)};                                                                                              \
    }                                                                                                                                           \
                                                                                                                                                \
    uint32_t FUNC(shfl_sync_##mode##_b32)(uint32_t input, int32_t delta, uint32_t opts, uint32_t membermask)                                    \
    {                                                                                                                                           \
        return __zluda_ptx_impl_shfl_sync_##mode##_b32_pred(input, delta, opts, membermask).x;                                             \
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
}
