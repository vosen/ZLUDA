// Every time this file changes it must te rebuilt, you need `rocm-llvm-dev` and `llvm-17`
// `fdenormal-fp-math=dynamic` is required to make functions eligible for inlining
//  /opt/rocm/llvm/bin/clang -Xclang -fdenormal-fp-math=dynamic  -Wall -Wextra -Wsign-compare -Wconversion -x hip zluda_ptx_impl.cpp -nogpulib -O3 -mno-wavefrontsize64 -o zluda_ptx_impl.bc -emit-llvm -c --offload-device-only --offload-arch=gfx1010 && /opt/rocm/llvm/bin/llvm-dis zluda_ptx_impl.bc -o - | sed '/@llvm.used/d' | sed '/wchar_size/d' | sed '/llvm.module.flags/d' | sed 's/define hidden/define linkonce_odr/g' | sed 's/\"target-cpu\"=\"gfx1010\"//g' | sed -E 's/\"target-features\"=\"[^\"]+\"//g' | sed 's/ nneg / /g' | sed 's/ disjoint / /g' | llvm-as-17 - -o  zluda_ptx_impl.bc && /opt/rocm/llvm/bin/llvm-dis zluda_ptx_impl.bc

#include <cstddef>
#include <cstdint>

#include <hip/amd_detail/amd_device_functions.h>

#define FUNC(NAME) __device__ __attribute__((retain)) __zluda_ptx_impl_##NAME
#define ATTR(NAME) __ZLUDA_PTX_IMPL_ATTRIBUTE_##NAME
#define DECLARE_ATTR(TYPE, NAME) extern const TYPE ATTR(NAME) __device__

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

    static __device__ uint32_t sub_sat(uint32_t x, uint32_t y)
    {
        uint32_t result;
        if (__builtin_sub_overflow(x, y, &result))
        {
            return 0;
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
            len = sub_sat(64, pos);
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

    #define BAR_RED_IMPL(reducer)                                                                                        \
    bool FUNC(bar_red_##reducer##_pred)(uint32_t barrier __attribute__((unused)), bool predicate, bool invert_predicate) \
    {                                                                                                                    \
        /* TODO: handle barrier */                                                                                       \
        return __ockl_wgred_##reducer##_i32(predicate ^ invert_predicate);                                               \
    }

    BAR_RED_IMPL(and);
    BAR_RED_IMPL(or);

    struct ShflSyncResult {
        uint32_t output;
        bool in_bounds;
    };

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

    #define SHFL_SYNC_IMPL(mode, calculate_index, CMP)                                                                                          \
    ShflSyncResult FUNC(shfl_sync_##mode##_b32_pred)(uint32_t input, int32_t delta, uint32_t opts, uint32_t membermask __attribute__((unused))) \
    {                                                                                                                                           \
        int32_t section_mask = (opts >> 8) & 0b11111;                                                                                           \
        int32_t warp_end = opts & 0b11111;                                                                                                      \
        int32_t self = (int32_t)__lane_id();                                                                                                    \
        int32_t subsection = section_mask & self;                                                                                               \
        int32_t subsection_end = subsection | (~section_mask & warp_end);                                                                       \
        int32_t idx = calculate_index;                                                                                                          \
        bool out_of_bounds = idx CMP subsection_end;                                                                                            \
        if (out_of_bounds) {                                                                                                                    \
            idx = self;                                                                                                                         \
        }                                                                                                                                       \
        int32_t output = __builtin_amdgcn_ds_bpermute(idx<<2, (int32_t)input);                                                                  \
        return {(uint32_t)output, !out_of_bounds};                                                                                              \
    }                                                                                                                                           \
                                                                                                                                                \
    uint32_t FUNC(shfl_sync_##mode##_b32)(uint32_t input, int32_t delta, uint32_t opts, uint32_t membermask)                                    \
    {                                                                                                                                           \
        return __zluda_ptx_impl_shfl_sync_##mode##_b32_pred(input, delta, opts, membermask).output;                                             \
    }
    
    // We are using the HIP __shfl intrinsics to implement these, rather than the __shfl_sync
    // intrinsics, as those only add an assertion checking that the membermask is used correctly.
    // They do not return the result of the range check, so we must replicate that logic here.

    SHFL_SYNC_IMPL(up,   self - delta,                         <);
    SHFL_SYNC_IMPL(down, self + delta,                         >);
    SHFL_SYNC_IMPL(bfly, self ^ delta,                         >);
    SHFL_SYNC_IMPL(idx,  (delta & ~section_mask) | subsection, >);

    DECLARE_ATTR(uint32_t, CLOCK_RATE);
    void FUNC(nanosleep_u32)(uint32_t nanoseconds) {
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
}
