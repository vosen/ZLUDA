// Every time this file changes it must te rebuilt:
//  ocloc -file zluda_ptx_impl.cl -64 -options "-cl-std=CL2.0 -Dcl_intel_bit_instructions -DINTEL" -out_dir . -device kbl -output_no_suffix -spv_only
//  /opt/rocm/llvm/bin/clang -Wall -Wextra -Wsign-compare -Wconversion -x cl -Xclang -finclude-default-header zluda_ptx_impl.cl -cl-std=CL2.0 -c -target amdgcn-amd-amdhsa -o zluda_ptx_impl.bc -emit-llvm
// Additionally you should strip names:
//  spirv-opt --strip-debug zluda_ptx_impl.spv -o zluda_ptx_impl.spv --target-env=spv1.3

#pragma OPENCL EXTENSION cl_khr_int64_base_atomics : enable
#pragma OPENCL EXTENSION cl_khr_int64_extended_atomics : enable

#define FUNC(NAME) __zluda_ptx_impl__ ## NAME

#define atomic_inc(NAME, SUCCESS, FAILURE, SCOPE, SPACE)                                                                                    \
    uint FUNC(NAME)(SPACE uint* ptr, uint threshold) {                                                                                      \
        uint expected = *ptr;                                                                                                               \
        uint desired;                                                                                                                       \
        do {                                                                                                                                \
            desired = (expected >= threshold) ? 0 : expected + 1;                                                                           \
        } while (!atomic_compare_exchange_strong_explicit((volatile SPACE atomic_uint*)ptr, &expected, desired, SUCCESS, FAILURE, SCOPE));  \
        return expected;                                                                                                                    \
    }

#define atomic_dec(NAME, SUCCESS, FAILURE, SCOPE, SPACE)                                                                                    \
    uint FUNC(NAME)(SPACE uint* ptr, uint threshold) {                                                                                      \
        uint expected = *ptr;                                                                                                               \
        uint desired;                                                                                                                       \
        do {                                                                                                                                \
            desired = (expected == 0 || expected > threshold) ? threshold : expected - 1;                                                   \
        } while (!atomic_compare_exchange_strong_explicit((volatile SPACE atomic_uint*)ptr, &expected, desired, SUCCESS, FAILURE, SCOPE));  \
        return expected;                                                                                                                    \
    }

#define atomic_add(NAME, SUCCESS, FAILURE, SCOPE, SPACE, TYPE, ATOMIC_TYPE, INT_TYPE)                                                                                    \
    TYPE FUNC(NAME)(SPACE TYPE* ptr, TYPE value) {                                                                                          \
        volatile SPACE ATOMIC_TYPE* atomic_ptr = (volatile SPACE ATOMIC_TYPE*)ptr;                                                          \
        union {                                                                                                                             \
            INT_TYPE int_view;                                                                                                              \
            TYPE     float_view;                                                                                                            \
        } expected, desired;                                                                                                                \
        expected.float_view = *ptr;                                                                                                         \
        do {                                                                                                                                \
            desired.float_view = expected.float_view + value;                                                                               \
        } while (!atomic_compare_exchange_strong_explicit(atomic_ptr, &expected.int_view, desired.int_view, SUCCESS, FAILURE, SCOPE));      \
        return expected.float_view;                                                                                                         \
    }

// We are doing all this mess instead of accepting memory_order and memory_scope parameters
// because ocloc emits broken (failing spirv-dis) SPIR-V when memory_order or memory_scope is a parameter

// atom.inc
atomic_inc(atom_relaxed_cta_generic_inc, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, );
atomic_inc(atom_acquire_cta_generic_inc, memory_order_acquire, memory_order_acquire, memory_scope_work_group, );
atomic_inc(atom_release_cta_generic_inc, memory_order_release, memory_order_acquire, memory_scope_work_group, );
atomic_inc(atom_acq_rel_cta_generic_inc, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, );

atomic_inc(atom_relaxed_gpu_generic_inc, memory_order_relaxed, memory_order_relaxed, memory_scope_device, );
atomic_inc(atom_acquire_gpu_generic_inc, memory_order_acquire, memory_order_acquire, memory_scope_device, );
atomic_inc(atom_release_gpu_generic_inc, memory_order_release, memory_order_acquire, memory_scope_device, );
atomic_inc(atom_acq_rel_gpu_generic_inc, memory_order_acq_rel, memory_order_acquire, memory_scope_device, );

atomic_inc(atom_relaxed_sys_generic_inc, memory_order_relaxed, memory_order_relaxed, memory_scope_device, );
atomic_inc(atom_acquire_sys_generic_inc, memory_order_acquire, memory_order_acquire, memory_scope_device, );
atomic_inc(atom_release_sys_generic_inc, memory_order_release, memory_order_acquire, memory_scope_device, );
atomic_inc(atom_acq_rel_sys_generic_inc, memory_order_acq_rel, memory_order_acquire, memory_scope_device, );

atomic_inc(atom_relaxed_cta_global_inc, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, __global);
atomic_inc(atom_acquire_cta_global_inc, memory_order_acquire, memory_order_acquire, memory_scope_work_group, __global);
atomic_inc(atom_release_cta_global_inc, memory_order_release, memory_order_acquire, memory_scope_work_group, __global);
atomic_inc(atom_acq_rel_cta_global_inc, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, __global);

atomic_inc(atom_relaxed_gpu_global_inc, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __global);
atomic_inc(atom_acquire_gpu_global_inc, memory_order_acquire, memory_order_acquire, memory_scope_device, __global);
atomic_inc(atom_release_gpu_global_inc, memory_order_release, memory_order_acquire, memory_scope_device, __global);
atomic_inc(atom_acq_rel_gpu_global_inc, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __global);

atomic_inc(atom_relaxed_sys_global_inc, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __global);
atomic_inc(atom_acquire_sys_global_inc, memory_order_acquire, memory_order_acquire, memory_scope_device, __global);
atomic_inc(atom_release_sys_global_inc, memory_order_release, memory_order_acquire, memory_scope_device, __global);
atomic_inc(atom_acq_rel_sys_global_inc, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __global);

atomic_inc(atom_relaxed_cta_shared_inc, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, __local);
atomic_inc(atom_acquire_cta_shared_inc, memory_order_acquire, memory_order_acquire, memory_scope_work_group, __local);
atomic_inc(atom_release_cta_shared_inc, memory_order_release, memory_order_acquire, memory_scope_work_group, __local);
atomic_inc(atom_acq_rel_cta_shared_inc, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, __local);

atomic_inc(atom_relaxed_gpu_shared_inc, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __local);
atomic_inc(atom_acquire_gpu_shared_inc, memory_order_acquire, memory_order_acquire, memory_scope_device, __local);
atomic_inc(atom_release_gpu_shared_inc, memory_order_release, memory_order_acquire, memory_scope_device, __local);
atomic_inc(atom_acq_rel_gpu_shared_inc, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __local);

atomic_inc(atom_relaxed_sys_shared_inc, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __local);
atomic_inc(atom_acquire_sys_shared_inc, memory_order_acquire, memory_order_acquire, memory_scope_device, __local);
atomic_inc(atom_release_sys_shared_inc, memory_order_release, memory_order_acquire, memory_scope_device, __local);
atomic_inc(atom_acq_rel_sys_shared_inc, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __local);

// atom.dec
atomic_dec(atom_relaxed_cta_generic_dec, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, );
atomic_dec(atom_acquire_cta_generic_dec, memory_order_acquire, memory_order_acquire, memory_scope_work_group, );
atomic_dec(atom_release_cta_generic_dec, memory_order_release, memory_order_acquire, memory_scope_work_group, );
atomic_dec(atom_acq_rel_cta_generic_dec, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, );

atomic_dec(atom_relaxed_gpu_generic_dec, memory_order_relaxed, memory_order_relaxed, memory_scope_device, );
atomic_dec(atom_acquire_gpu_generic_dec, memory_order_acquire, memory_order_acquire, memory_scope_device, );
atomic_dec(atom_release_gpu_generic_dec, memory_order_release, memory_order_acquire, memory_scope_device, );
atomic_dec(atom_acq_rel_gpu_generic_dec, memory_order_acq_rel, memory_order_acquire, memory_scope_device, );

atomic_dec(atom_relaxed_sys_generic_dec, memory_order_relaxed, memory_order_relaxed, memory_scope_device, );
atomic_dec(atom_acquire_sys_generic_dec, memory_order_acquire, memory_order_acquire, memory_scope_device, );
atomic_dec(atom_release_sys_generic_dec, memory_order_release, memory_order_acquire, memory_scope_device, );
atomic_dec(atom_acq_rel_sys_generic_dec, memory_order_acq_rel, memory_order_acquire, memory_scope_device, );

atomic_dec(atom_relaxed_cta_global_dec, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, __global);
atomic_dec(atom_acquire_cta_global_dec, memory_order_acquire, memory_order_acquire, memory_scope_work_group, __global);
atomic_dec(atom_release_cta_global_dec, memory_order_release, memory_order_acquire, memory_scope_work_group, __global);
atomic_dec(atom_acq_rel_cta_global_dec, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, __global);

atomic_dec(atom_relaxed_gpu_global_dec, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __global);
atomic_dec(atom_acquire_gpu_global_dec, memory_order_acquire, memory_order_acquire, memory_scope_device, __global);
atomic_dec(atom_release_gpu_global_dec, memory_order_release, memory_order_acquire, memory_scope_device, __global);
atomic_dec(atom_acq_rel_gpu_global_dec, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __global);

atomic_dec(atom_relaxed_sys_global_dec, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __global);
atomic_dec(atom_acquire_sys_global_dec, memory_order_acquire, memory_order_acquire, memory_scope_device, __global);
atomic_dec(atom_release_sys_global_dec, memory_order_release, memory_order_acquire, memory_scope_device, __global);
atomic_dec(atom_acq_rel_sys_global_dec, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __global);

atomic_dec(atom_relaxed_cta_shared_dec, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, __local);
atomic_dec(atom_acquire_cta_shared_dec, memory_order_acquire, memory_order_acquire, memory_scope_work_group, __local);
atomic_dec(atom_release_cta_shared_dec, memory_order_release, memory_order_acquire, memory_scope_work_group, __local);
atomic_dec(atom_acq_rel_cta_shared_dec, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, __local);

atomic_dec(atom_relaxed_gpu_shared_dec, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __local);
atomic_dec(atom_acquire_gpu_shared_dec, memory_order_acquire, memory_order_acquire, memory_scope_device, __local);
atomic_dec(atom_acq_rel_sys_shared_dec, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __local);

// atom.add.f32
atomic_add(atom_relaxed_cta_generic_add_f32, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, , float, atomic_uint, uint);
atomic_add(atom_acquire_cta_generic_add_f32, memory_order_acquire, memory_order_acquire, memory_scope_work_group, , float, atomic_uint, uint);
atomic_add(atom_release_cta_generic_add_f32, memory_order_release, memory_order_acquire, memory_scope_work_group, , float, atomic_uint, uint);
atomic_add(atom_acq_rel_cta_generic_add_f32, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, , float, atomic_uint, uint);

atomic_add(atom_relaxed_gpu_generic_add_f32, memory_order_relaxed, memory_order_relaxed, memory_scope_device, , float, atomic_uint, uint);
atomic_add(atom_acquire_gpu_generic_add_f32, memory_order_acquire, memory_order_acquire, memory_scope_device, , float, atomic_uint, uint);
atomic_add(atom_release_gpu_generic_add_f32, memory_order_release, memory_order_acquire, memory_scope_device, , float, atomic_uint, uint);
atomic_add(atom_acq_rel_gpu_generic_add_f32, memory_order_acq_rel, memory_order_acquire, memory_scope_device, , float, atomic_uint, uint);

atomic_add(atom_relaxed_sys_generic_add_f32, memory_order_relaxed, memory_order_relaxed, memory_scope_device, , float, atomic_uint, uint);
atomic_add(atom_acquire_sys_generic_add_f32, memory_order_acquire, memory_order_acquire, memory_scope_device, , float, atomic_uint, uint);
atomic_add(atom_release_sys_generic_add_f32, memory_order_release, memory_order_acquire, memory_scope_device, , float, atomic_uint, uint);
atomic_add(atom_acq_rel_sys_generic_add_f32, memory_order_acq_rel, memory_order_acquire, memory_scope_device, , float, atomic_uint, uint);

atomic_add(atom_relaxed_cta_global_add_f32, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, __global, float, atomic_uint, uint);
atomic_add(atom_acquire_cta_global_add_f32, memory_order_acquire, memory_order_acquire, memory_scope_work_group, __global, float, atomic_uint, uint);
atomic_add(atom_release_cta_global_add_f32, memory_order_release, memory_order_acquire, memory_scope_work_group, __global, float, atomic_uint, uint);
atomic_add(atom_acq_rel_cta_global_add_f32, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, __global, float, atomic_uint, uint);

atomic_add(atom_relaxed_gpu_global_add_f32, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __global, float, atomic_uint, uint);
atomic_add(atom_acquire_gpu_global_add_f32, memory_order_acquire, memory_order_acquire, memory_scope_device, __global, float, atomic_uint, uint);
atomic_add(atom_release_gpu_global_add_f32, memory_order_release, memory_order_acquire, memory_scope_device, __global, float, atomic_uint, uint);
atomic_add(atom_acq_rel_gpu_global_add_f32, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __global, float, atomic_uint, uint);

atomic_add(atom_relaxed_sys_global_add_f32, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __global, float, atomic_uint, uint);
atomic_add(atom_acquire_sys_global_add_f32, memory_order_acquire, memory_order_acquire, memory_scope_device, __global, float, atomic_uint, uint);
atomic_add(atom_release_sys_global_add_f32, memory_order_release, memory_order_acquire, memory_scope_device, __global, float, atomic_uint, uint);
atomic_add(atom_acq_rel_sys_global_add_f32, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __global, float, atomic_uint, uint);

atomic_add(atom_relaxed_cta_shared_add_f32, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, __local, float, atomic_uint, uint);
atomic_add(atom_acquire_cta_shared_add_f32, memory_order_acquire, memory_order_acquire, memory_scope_work_group, __local, float, atomic_uint, uint);
atomic_add(atom_release_cta_shared_add_f32, memory_order_release, memory_order_acquire, memory_scope_work_group, __local, float, atomic_uint, uint);
atomic_add(atom_acq_rel_cta_shared_add_f32, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, __local, float, atomic_uint, uint);

atomic_add(atom_relaxed_gpu_shared_add_f32, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __local, float, atomic_uint, uint);
atomic_add(atom_acquire_gpu_shared_add_f32, memory_order_acquire, memory_order_acquire, memory_scope_device, __local, float, atomic_uint, uint);
atomic_add(atom_release_gpu_shared_add_f32, memory_order_release, memory_order_acquire, memory_scope_device, __local, float, atomic_uint, uint);
atomic_add(atom_acq_rel_gpu_shared_add_f32, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __local, float, atomic_uint, uint);

atomic_add(atom_relaxed_sys_shared_add_f32, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __local, float, atomic_uint, uint);
atomic_add(atom_acquire_sys_shared_add_f32, memory_order_acquire, memory_order_acquire, memory_scope_device, __local, float, atomic_uint, uint);
atomic_add(atom_release_sys_shared_add_f32, memory_order_release, memory_order_acquire, memory_scope_device, __local, float, atomic_uint, uint);
atomic_add(atom_acq_rel_sys_shared_add_f32, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __local, float, atomic_uint, uint);

atomic_add(atom_relaxed_cta_generic_add_f64, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, , double, atomic_ulong, ulong);
atomic_add(atom_acquire_cta_generic_add_f64, memory_order_acquire, memory_order_acquire, memory_scope_work_group, , double, atomic_ulong, ulong);
atomic_add(atom_release_cta_generic_add_f64, memory_order_release, memory_order_acquire, memory_scope_work_group, , double, atomic_ulong, ulong);
atomic_add(atom_acq_rel_cta_generic_add_f64, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, , double, atomic_ulong, ulong);

atomic_add(atom_relaxed_gpu_generic_add_f64, memory_order_relaxed, memory_order_relaxed, memory_scope_device, , double, atomic_ulong, ulong);
atomic_add(atom_acquire_gpu_generic_add_f64, memory_order_acquire, memory_order_acquire, memory_scope_device, , double, atomic_ulong, ulong);
atomic_add(atom_release_gpu_generic_add_f64, memory_order_release, memory_order_acquire, memory_scope_device, , double, atomic_ulong, ulong);
atomic_add(atom_acq_rel_gpu_generic_add_f64, memory_order_acq_rel, memory_order_acquire, memory_scope_device, , double, atomic_ulong, ulong);

atomic_add(atom_relaxed_sys_generic_add_f64, memory_order_relaxed, memory_order_relaxed, memory_scope_device, , double, atomic_ulong, ulong);
atomic_add(atom_acquire_sys_generic_add_f64, memory_order_acquire, memory_order_acquire, memory_scope_device, , double, atomic_ulong, ulong);
atomic_add(atom_release_sys_generic_add_f64, memory_order_release, memory_order_acquire, memory_scope_device, , double, atomic_ulong, ulong);
atomic_add(atom_acq_rel_sys_generic_add_f64, memory_order_acq_rel, memory_order_acquire, memory_scope_device, , double, atomic_ulong, ulong);
// atom.add.f64
atomic_add(atom_relaxed_cta_global_add_f64, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, __global, double, atomic_ulong, ulong);
atomic_add(atom_acquire_cta_global_add_f64, memory_order_acquire, memory_order_acquire, memory_scope_work_group, __global, double, atomic_ulong, ulong);
atomic_add(atom_release_cta_global_add_f64, memory_order_release, memory_order_acquire, memory_scope_work_group, __global, double, atomic_ulong, ulong);
atomic_add(atom_acq_rel_cta_global_add_f64, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, __global, double, atomic_ulong, ulong);

atomic_add(atom_relaxed_gpu_global_add_f64, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __global, double, atomic_ulong, ulong);
atomic_add(atom_acquire_gpu_global_add_f64, memory_order_acquire, memory_order_acquire, memory_scope_device, __global, double, atomic_ulong, ulong);
atomic_add(atom_release_gpu_global_add_f64, memory_order_release, memory_order_acquire, memory_scope_device, __global, double, atomic_ulong, ulong);
atomic_add(atom_acq_rel_gpu_global_add_f64, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __global, double, atomic_ulong, ulong);

atomic_add(atom_relaxed_sys_global_add_f64, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __global, double, atomic_ulong, ulong);
atomic_add(atom_acquire_sys_global_add_f64, memory_order_acquire, memory_order_acquire, memory_scope_device, __global, double, atomic_ulong, ulong);
atomic_add(atom_release_sys_global_add_f64, memory_order_release, memory_order_acquire, memory_scope_device, __global, double, atomic_ulong, ulong);
atomic_add(atom_acq_rel_sys_global_add_f64, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __global, double, atomic_ulong, ulong);

atomic_add(atom_relaxed_cta_shared_add_f64, memory_order_relaxed, memory_order_relaxed, memory_scope_work_group, __local, double, atomic_ulong, ulong);
atomic_add(atom_acquire_cta_shared_add_f64, memory_order_acquire, memory_order_acquire, memory_scope_work_group, __local, double, atomic_ulong, ulong);
atomic_add(atom_release_cta_shared_add_f64, memory_order_release, memory_order_acquire, memory_scope_work_group, __local, double, atomic_ulong, ulong);
atomic_add(atom_acq_rel_cta_shared_add_f64, memory_order_acq_rel, memory_order_acquire, memory_scope_work_group, __local, double, atomic_ulong, ulong);

atomic_add(atom_relaxed_gpu_shared_add_f64, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __local, double, atomic_ulong, ulong);
atomic_add(atom_acquire_gpu_shared_add_f64, memory_order_acquire, memory_order_acquire, memory_scope_device, __local, double, atomic_ulong, ulong);
atomic_add(atom_release_gpu_shared_add_f64, memory_order_release, memory_order_acquire, memory_scope_device, __local, double, atomic_ulong, ulong);
atomic_add(atom_acq_rel_gpu_shared_add_f64, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __local, double, atomic_ulong, ulong);

atomic_add(atom_relaxed_sys_shared_add_f64, memory_order_relaxed, memory_order_relaxed, memory_scope_device, __local, double, atomic_ulong, ulong);
atomic_add(atom_acquire_sys_shared_add_f64, memory_order_acquire, memory_order_acquire, memory_scope_device, __local, double, atomic_ulong, ulong);
atomic_add(atom_release_sys_shared_add_f64, memory_order_release, memory_order_acquire, memory_scope_device, __local, double, atomic_ulong, ulong);
atomic_add(atom_acq_rel_sys_shared_add_f64, memory_order_acq_rel, memory_order_acquire, memory_scope_device, __local, double, atomic_ulong, ulong);

#ifdef INTEL
    uint FUNC(bfe_u32)(uint base, uint pos, uint len) {
        return intel_ubfe(base, pos, len);
    }

    ulong FUNC(bfe_u64)(ulong base, uint pos, uint len) {
        return intel_ubfe(base, pos, len);
    }

    int FUNC(bfe_s32)(int base, uint pos, uint len) {
        return intel_sbfe(base, pos, len);
    }

    long FUNC(bfe_s64)(long base, uint pos, uint len) {
        return intel_sbfe(base, pos, len);
    }

    uint FUNC(bfi_b32)(uint insert, uint base, uint offset, uint count) {
        return intel_bfi(base, insert, offset, count);
    }

    ulong FUNC(bfi_b64)(ulong insert, ulong base, uint offset, uint count) {
        return intel_bfi(base, insert, offset, count);
    }

    uint FUNC(brev_b32)(uint base) {
        return intel_bfrev(base);
    }

    ulong FUNC(brev_b64)(ulong base) {
        return intel_bfrev(base);
    }
#else
    uint FUNC(bfe_u32)(uint base, uint pos, uint len) {
        return amd_bfe(base, pos, len);
    }

    ulong FUNC(bfe_u64)(ulong base, uint pos, uint len) {
        return (base >> pos) & len;
    }

    int FUNC(bfe_s32)(int base, uint pos, uint len) {
        return amd_bfe(base, pos, len);
    }

    long FUNC(bfe_s64)(long base, uint pos, uint len) {
        return (base >> pos) & len;
    }

    uint FUNC(bfi_b32)(uint insert, uint base, uint offset, uint count) {
        uint mask = amd_bfm(count, offset);
        return (~mask & base) | (mask & insert);
    }

    ulong FUNC(bfi_b64)(ulong insert, ulong base, uint offset, uint count) {
        ulong mask = ((1UL << (count & 0x3f)) - 1UL) << (offset & 0x3f);
        return (~mask & base) | (mask & insert);
    }

    extern __attribute__((const)) uint __llvm_bitreverse_i32(uint) __asm("llvm.bitreverse.i32");
    uint FUNC(brev_b32)(uint base) {
        return __llvm_bitreverse_i32(base);
    }

    extern __attribute__((const)) ulong __llvm_bitreverse_i64(ulong) __asm("llvm.bitreverse.i64");
    ulong FUNC(brev_b64)(ulong base) {
        return __llvm_bitreverse_i64(base);
    }

    // Taken from __ballot definition in hipamd/include/hip/amd_detail/amd_device_functions.h
    uint FUNC(activemask)() {
        return (uint)__builtin_amdgcn_uicmp(1, 0, 33);
    }

    uint FUNC(sreg_clock)() {
        return (uint)__builtin_amdgcn_s_memtime();
    }

    // Taken from __ballot definition in hipamd/include/hip/amd_detail/amd_device_functions.h
    // They return active threads, which I think is incorrect
    extern __attribute__((const)) uint __ockl_lane_u32();
    uint FUNC(sreg_lanemask_lt)() {
        uint lane_idx = __ockl_lane_u32();
        ulong mask = (1UL << lane_idx) - 1UL;
        return (uint)mask;
    }
#endif

void FUNC(__assertfail)(
    __attribute__((unused)) __private ulong* message,
    __attribute__((unused)) __private ulong* file,
    __attribute__((unused)) __private uint* line,
    __attribute__((unused)) __private ulong* function,
    __attribute__((unused)) __private ulong* charSize
) {
}
