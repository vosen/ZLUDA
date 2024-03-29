#include "raytracing.hpp"

extern "C" {
    __device__ uint32_t FUNCTION_NAME(
        uint8_t SHARED_SPACE* global_state,
        uint32_t prim_idx,
        uint2::Native_vec_ launch_idx,
        uint2::Native_vec_ launch_dim,
        OptixRay PRIVATE_SPACE* current_ray,
        float current_time,
        uint8_t PRIVATE_SPACE* payload,
        float current_distance,
        float2::Native_vec_ barycentric,
        zluda_float3 normals,
        uint8_t GLOBAL_SPACE* variable_block,
        uint8_t GLOBAL_SPACE* attribute_block,
        uint8_t GLOBAL_SPACE* transform_block
    );

    __device__ uint32_t __zluda_rt_ptx_impl__rollback_wrapper(
        uint8_t SHARED_SPACE* global_state,
        uint32_t prim_idx,
        uint2::Native_vec_ launch_idx,
        uint2::Native_vec_ launch_dim,
        OptixRay PRIVATE_SPACE* current_ray,
        float current_time,
        uint8_t PRIVATE_SPACE* payload,
        float current_distance,
        float2::Native_vec_ barycentric,
        zluda_float3 normals,
        uint8_t GLOBAL_SPACE* variable_block,
        uint8_t GLOBAL_SPACE* attribute_block,
        uint8_t GLOBAL_SPACE* transform_block,
        uint64_t anyhit_block
    );

    __global__ void EXPORTED_KERNEL(
        GlobalState globals,
        void GLOBAL_SPACE* global_stack_space,
        uint8_t GLOBAL_SPACE* variable_block,
        raytracing_fn exception,
        uint8_t GLOBAL_SPACE* exception_block
    ) {
        asm ("" : : :  "s105");
        asm ("" : : :  "v255");

        uint32_t local_x = threadIdx.x;
        uint32_t local_y = threadIdx.y;

        uint32_t x = blockIdx.x * blockDim.x + threadIdx.x;
        uint32_t y = blockIdx.y * blockDim.y + threadIdx.y;

        uint32_t wavefront_id = gridDim.x * blockIdx.y + blockIdx.x;

        ZLUDA_RT(STACK).init(local_x, local_y, wavefront_id, globals, global_stack_space);
        __syncthreads();

        //if (x != 0 || y != 32)
        //    return;

        if (x < globals.width && y < globals.height) {
            uint32_t result = FUNCTION_NAME((uint8_t SHARED_SPACE*)&ZLUDA_RT(STACK),  0, uint2(x, y).data, uint2(globals.width, globals.height).data, nullptr, 0.0, nullptr, 0.0, float2(0.0, 0.0).data, zluda_float3{0.0, 0.0, 0.0}, variable_block, nullptr, nullptr);
            if (result >= 1024 && exception != nullptr) {
                exception((uint8_t SHARED_SPACE*)&ZLUDA_RT(STACK), result, uint2(x, y).data, uint2(globals.width, globals.height).data, nullptr, 0.0, nullptr, 0.0, float2(0.0, 0.0).data, zluda_float3{0.0, 0.0, 0.0}, exception_block, nullptr, nullptr);
            }
        }
    }

    // I'd rather have a function that calls the generated function, but
    // AMDGPU LLVM internalizes all function declarations. Sometimes in really
    // sneaky ways, e.g. AMDGPUPropagateAttributes will copy a function and do:
    //      NewF->setVisibility(GlobalValue::DefaultVisibility);
    //      NewF->setLinkage(GlobalValue::InternalLinkage);
    __device__ raytracing_fn EXPORTED_FUNCTION = FUNCTION_NAME;
    __device__ attribute_fn EXPORTED_ATTRIBUTE_FUNCTION = __zluda_rt_ptx_impl__rollback_wrapper;
}

#define extern auto hack = 
#define constexpr ;
 