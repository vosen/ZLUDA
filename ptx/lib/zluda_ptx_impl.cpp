// Every time this file changes it must te rebuilt, you need llvm-17:
//  /opt/rocm/llvm/bin/clang -Wall -Wextra -Wsign-compare -Wconversion -x hip zluda_ptx_impl.cpp -nogpulib -O3 -mno-wavefrontsize64 -o zluda_ptx_impl.bc -emit-llvm -c --offload-device-only --offload-arch=gfx1010 && llvm-dis-17 zluda_ptx_impl.bc -o - | sed '/@llvm.used/d' | sed '/wchar_size/d' | sed '/llvm.module.flags/d' | sed 's/define hidden/define linkonce_odr/g' | sed 's/\"target-cpu\"=\"gfx1010\"//g' | sed -E 's/\"target-features\"=\"[^\"]+\"//g' | llvm-as-17 - -o  zluda_ptx_impl.bc && llvm-dis-17 zluda_ptx_impl.bc

#include <cstddef>
#include <cstdint>

#define FUNC(NAME) __device__ __attribute__((retain)) __zluda_ptx_impl_ ## NAME

extern "C" {
    uint32_t FUNC(activemask)() {
        return __builtin_amdgcn_read_exec_lo();
    }

    size_t __ockl_get_local_size(uint32_t) __device__;
    uint32_t FUNC(sreg_ntid)(uint8_t member) {
        return (uint32_t)__ockl_get_local_size(member);
    }
}
