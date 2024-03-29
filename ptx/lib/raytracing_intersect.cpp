#include "raytracing.hpp"

extern "C" {
    __device__ void FUNCTION_NAME(
        uint32_t prim_idx,
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state,
        uint2::Native_vec_ launch_idx,
        uint2::Native_vec_ dim_idx,
        uint8_t PRIVATE_SPACE* current_ray,
        float current_time,
        uint8_t PRIVATE_SPACE* payload,
        float current_distance,
        uint8_t GLOBAL_SPACE* variable_block,
        uint8_t GLOBAL_SPACE* attribute_block,
        uint8_t GLOBAL_SPACE* transform_block,
        float PRIVATE_SPACE* new_distance,
        uint32_t PRIVATE_SPACE* intersection_result,
        uint32_t PRIVATE_SPACE* material_result
    );

    namespace {
        // Data we need:
        // * Globally:
        //   * variable block
        //   * ptr to GeometryInstance
        // * Locally:
        //   * payload ptr (to pass to any hit callbacks)
        //   * current closest hit distance (for rtPotentialIntersection)
        //   * ray_type (to reconstruct OptixRay)
        //   * tmin (to reconstruct OptixRay)
        //   * attribute block (to pass to inner function)
        // * On return:
        //   * flush attribute block (so any-hit and closest-hit can inspect it):
        //     "If the any-hit program invokes rtIgnoreIntersection, any attributes
        //     computed will be reset to their previous values and the previous
        //     t-interval will be restored."
        __attribute__((noinline))
        static __device__ IntersectResult wrapper(
            hiprtRay ray,
            uint32_t		prim_idx,
            const void*		global_data,
            void*		    payload
        ) {
            uint32_t launch_x = blockIdx.x * blockDim.x + threadIdx.x;
            uint32_t launch_y = blockIdx.y * blockDim.y + threadIdx.y;

            // Imagine this case: one geo group, two geo instances, each instance has a sphere,
            // both instances have the same program assigned, but a different variable block.
            // Two rays in a single wavefront start. they rays are in the same accel structure and
            // have the same ray type. They both hit our spheres. HIP RT runs our fn pointer with
            // different arguments. Thus a intra-wavefront divergence of variable block.
            IntersectionInput GLOBAL_SPACE* intersect_input = (IntersectionInput GLOBAL_SPACE*)(global_data);
            IntersectionPayload PRIVATE_SPACE* user_ptr = (IntersectionPayload PRIVATE_SPACE*)(payload);
            GlobalState SHARED_SPACE* globals = (GlobalState SHARED_SPACE*)user_ptr->global_state;
            uint32_t size_x = globals->width;
            uint32_t size_y = globals->height;
            OptixRay current_ray {
                ray.origin,
                ray.direction,
                user_ptr->ray_type,
                user_ptr->tmin,
                user_ptr->tmax
            };
            // Values are:
            // 0: _rt_report_intersection was not called
            // non-0: value returned by anyhit + 1
            uint32_t intersection_result = 0;
            float t {};
            FUNCTION_NAME(
                prim_idx,
                user_ptr->global_state,
                uint2(launch_x, launch_y).data,
                uint2(size_x, size_y).data,
                (uint8_t PRIVATE_SPACE*)&current_ray,
                user_ptr->time,
                user_ptr->payload,
                user_ptr->closest_distance,
                reinterpret_cast<uint8_t GLOBAL_SPACE*>(intersect_input),
                user_ptr->attribute_block,
                intersect_input->transform_block,
                (float PRIVATE_SPACE*)&t,
                (uint32_t PRIVATE_SPACE*)&intersection_result,
                &user_ptr->material
            );
            if (intersection_result != 0) {
                user_ptr->result = intersection_result - 1;
            }
            bool is_hit = intersection_result != 0 && intersection_result != 2;
            return IntersectResult { is_hit, t };
        }

        __global__ void EXPORTED_KERNEL(
            hiprtRay        ray,
            uint32_t		prim_idx,
            const void*		global_data,
            void*		    payload
        ) {
            wrapper(ray, prim_idx, global_data, payload);
        }
    }

    __device__ hiprtIntersectFunc2 EXPORTED_FUNCTION  = wrapper;
}

#define extern auto hack = 
#define constexpr ;
 