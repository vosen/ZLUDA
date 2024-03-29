#include "raytracing.hpp"

extern "C" {
    __attribute__((noinline))
    __device__ void FUNCTION_NAME(
        uint32_t prim_index,
        uint64_t result_ptr,
        uint8_t SHARED_SPACE* global_state,
        uint8_t GLOBAL_SPACE* variable_block
    );
    
    namespace {
        static __device__ void wrapper(uint32_t prim_idx, float GLOBAL_SPACE* result, uint8_t SHARED_SPACE* global_state, uint8_t GLOBAL_SPACE* variable_block) {
            uint64_t result_ptr = reinterpret_cast<uint64_t>(result);
            FUNCTION_NAME(prim_idx, result_ptr, global_state, variable_block);
        }
    }

    __global__ void EXPORTED_KERNEL(uint32_t prim_index, uint64_t result_ptr, uint8_t GLOBAL_SPACE* variable_block) {
        FUNCTION_NAME(prim_index, result_ptr, 0, variable_block);
    }

    __device__ bounding_box_fn EXPORTED_FUNCTION = wrapper;
}

#define extern auto hack = 
#define constexpr ;
 