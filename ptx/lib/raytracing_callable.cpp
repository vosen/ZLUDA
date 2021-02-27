#include "raytracing.hpp"

extern "C" {
    __device__ void FUNCTION_NAME();
    __device__ void* EXPORTED_FUNCTION = (void*)FUNCTION_NAME;
    static __global__ void EXPORTED_KERNEL() { }
}

#define extern auto hack = 
#define constexpr ;
