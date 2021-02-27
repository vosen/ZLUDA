// Compile and disassemble:
//   /opt/rocm/llvm/bin/clang -Xclang -no-opaque-pointers -Wall -Wextra -Wsign-compare -Wconversion -x hip zluda_rt_ptx_impl.cpp -S -emit-llvm --cuda-device-only -nogpulib -O3 -mno-wavefrontsize64 -Xclang -fallow-half-arguments-and-returns -o - | sed -e 's/define/define linkonce_odr/g' | sed -e '/@llvm.used/d' | sed -e 's/\"target-cpu\"=\"[^\"]*\"//g' | sed -e 's/\"target-features\"=\"[^\"]*\"//g' | sed -e 's/\"denormal-fp-math-f32\"=\"[^\"]*\"//g' | sed -e 's/!0, !1, !2//g' | llvm-as-13 -o zluda_rt_ptx_impl.bc && /opt/rocm/llvm/bin/llvm-dis zluda_rt_ptx_impl.bc
// Compile to binary:
//   /opt/rocm/llvm/bin/clang -x ir -target amdgcn-amd-amdhsa -Xlinker --no-undefined zluda_rt_ptx_impl.bc -mno-wavefrontsize64 -mcpu=gfx1030
// Decompile:
//   /opt/rocm/llvm/bin/llvm-objdump -d a.out --triple amdgcn-amd-amdhsa
// List of builtins:
//   https://github.com/llvm/llvm-project/blob/main/clang/include/clang/Basic/BuiltinsAMDGPU.def
//   https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/IR/IntrinsicsAMDGPU.td
// Extra information:
//   https://llvm.org/docs/AMDGPUUsage.html

#include <cstdint>
#include <hip/hip_runtime.h>
#include <hip/amd_detail/amd_device_functions.h>
#include <hip/amd_detail/amd_surface_functions.h>

#define FUNC(NAME) __attribute__((device)) __attribute__((retain)) __zluda_rt_ptx_impl__##NAME

#define GENERIC_SPACE __attribute__((address_space(0)))
#define GLOBAL_SPACE __attribute__((address_space(1)))
#define SHARED_SPACE __attribute__((address_space(3)))
#define CONSTANT_SPACE __attribute__((address_space(4)))
#define PRIVATE_SPACE __attribute__((address_space(5)))

typedef __fp16 half;
typedef half half4 __attribute__((ext_vector_type(4)));

#define SKIP_RAYTRACING_GLOBALS
#include "raytracing.hpp"

extern "C"
{
    // This could be emitted directly during raytracing passes, but we keep it
    // here in case we ever wanted to change the layout of variable blocks
    __attribute__((always_inline)) GLOBAL_SPACE uint8_t *FUNC(get_variable_pointer_global)(
        GLOBAL_SPACE uint8_t *variable_block_ptr,
        uint32_t variable_offset)
    {
        return variable_block_ptr + variable_offset;
    }

    __attribute__((always_inline)) PRIVATE_SPACE uint8_t *FUNC(get_variable_pointer_local)(
        PRIVATE_SPACE uint8_t *variable_block_ptr,
        uint32_t variable_offset)
    {
        return variable_block_ptr + variable_offset;
    }

    __device__ uint32_t ZLUDA_RT(_rt_trace_time_mask_flags_64) (
        uint32_t bvh_index,
        float ray_origin_x,
        float ray_origin_y,
        float ray_origin_z,
        float ray_direction_x,
        float ray_direction_y,
        float ray_direction_z,
        uint32_t ray_type,
        float tmin,
        float tmax,
        float time,
        uint32_t mask,
        uint32_t flags,
        uint64_t payload_ptr,
        uint32_t payload_size,
        uint8_t SHARED_SPACE* global_state
    );

    __device__ uint32_t FUNC(_rt_trace_mask_flags_64) (
        uint32_t bvh_index,
        float ray_origin_x,
        float ray_origin_y,
        float ray_origin_z,
        float ray_direction_x,
        float ray_direction_y,
        float ray_direction_z,
        uint32_t ray_type,
        float tmin,
        float tmax,
        uint32_t mask,
        uint32_t flags,
        uint64_t payload_ptr,
        uint32_t payload_size,
        uint8_t SHARED_SPACE* global_state
    ) {
        return ZLUDA_RT(_rt_trace_time_mask_flags_64)(
            bvh_index,
            ray_origin_x,
            ray_origin_y,
            ray_origin_z,
            ray_direction_x,
            ray_direction_y,
            ray_direction_z,
            ray_type,
            tmin,
            tmax,
            0.0,
            mask,
            flags,
            payload_ptr,
            payload_size,
            global_state
        );
    }

    __attribute__((always_inline)) uint64_t FUNC(_rt_buffer_get_64)(
        uint64_t untyped_ptr,
        uint32_t dimensions,
        uint32_t element_size,
        uint64_t x,
        uint64_t y,
        uint64_t z,
        uint64_t w __attribute__((unused))
    ) {
        OptixBuffer *buffer_ptr = reinterpret_cast<OptixBuffer *>(untyped_ptr);
        switch (dimensions)
        {
        case 1:
            return reinterpret_cast<uint64_t>(buffer_ptr->ptr + (element_size * x));
        case 2:
            return reinterpret_cast<uint64_t>(buffer_ptr->ptr + (element_size * ((buffer_ptr->x * y) + x)));
        case 3:
            return reinterpret_cast<uint64_t>(buffer_ptr->ptr + (element_size * ((buffer_ptr->y * buffer_ptr->x * z) + (buffer_ptr->x * y) + x)));
        default:
            __builtin_unreachable();
        }
    }

    __attribute__((always_inline)) uint64_t FUNC(_rt_buffer_get_id_64)(
        uint32_t buffer_id,
        uint32_t dimensions,
        uint32_t element_size,
        uint64_t x,
        uint64_t y,
        uint64_t z,
        uint64_t w,
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state
    ) {
        GlobalState SHARED_SPACE* globals = (GlobalState SHARED_SPACE*)global_state;
        uint64_t untyped_ptr = reinterpret_cast<uint64_t>(&globals->buffers[buffer_id]);
        return ZLUDA_RT(_rt_buffer_get_64)(untyped_ptr, dimensions, element_size, x, y, z, w);
    }

    struct buffer_size_result
    {
        uint64_t x, y, z, w;
    };
    __attribute__((always_inline)) buffer_size_result FUNC(_rt_buffer_get_size_64)(uint64_t untyped_ptr, uint32_t dimensions __attribute__((unused)), uint32_t element_size __attribute__((unused)))
    {
        OptixBuffer *buffer_ptr = reinterpret_cast<OptixBuffer *>(untyped_ptr);
        return buffer_size_result{buffer_ptr->x, buffer_ptr->y, 0, 0};
    }

    __attribute__((always_inline)) buffer_size_result FUNC(_rt_buffer_get_id_size_64)(
        uint32_t buffer_id,
        uint32_t dimensions,
        uint32_t element_size,
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state
    ) {
        GlobalState SHARED_SPACE* globals = (GlobalState SHARED_SPACE*)global_state;
        uint64_t untyped_ptr = reinterpret_cast<uint64_t>(&globals->buffers[buffer_id]);
        return ZLUDA_RT(_rt_buffer_get_size_64)(untyped_ptr, dimensions, element_size);
    }

    uint32_t FUNC(_rt_print_active)(void)
    {
        return 0;
    }

    void FUNC(_rt_throw)(uint32_t)
    {
        abort();
    }

    // TODO: implement
    struct float4_struct
    {
        float x, y, z, w;
    };
    float4_struct FUNC(_rt_transform_tuple)(
        uint32_t,
        float x,
        float y,
        float z,
        float w)
    {
        return float4_struct{x, y, z, w};
    }

    uint32_t FUNC(_rt_potential_intersection)(
        float distance,
        /* vvv INJECTED vvv */
        uint8_t(PRIVATE_SPACE *current_ray)[36],
        float current_distance,
        float PRIVATE_SPACE *potential_distance)
    {
        OptixRay PRIVATE_SPACE *optix_ray = reinterpret_cast<OptixRay PRIVATE_SPACE *>(current_ray);
        if (distance > optix_ray->tmin && distance < current_distance)
        {
            *potential_distance = distance;
            return 1;
        }
        else
        {
            return 0;
        }
    }

    uint32_t FUNC(_rt_report_intersection)(
        uint32_t material,
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state,
        uint2::Native_vec_ launch_idx,
        uint2::Native_vec_ dim_idx,
        uint8_t(PRIVATE_SPACE *current_ray)[36],
        float current_time,
        uint8_t PRIVATE_SPACE *payload,
        uint8_t GLOBAL_SPACE *variable_block,
        uint8_t GLOBAL_SPACE *attribute_block,
        uint8_t GLOBAL_SPACE *transform_block,
        float PRIVATE_SPACE *new_distance,
        uint32_t PRIVATE_SPACE *intersection_result,
        uint32_t PRIVATE_SPACE *material_result,
        float PRIVATE_SPACE *potential_distance)
    {
        OptixRay PRIVATE_SPACE *ray_ptr = reinterpret_cast<OptixRay PRIVATE_SPACE *>(current_ray);
        GlobalState SHARED_SPACE* globals = (GlobalState SHARED_SPACE*)global_state;
        uint32_t ray_type_count = globals->ray_type_count;
        uint32_t entry_index = (ray_type_count * material) + ray_ptr->ray_type;
        IntersectionInput GLOBAL_SPACE* intersect_input = reinterpret_cast<IntersectionInput GLOBAL_SPACE*>(variable_block);
        uint8_t GLOBAL_SPACE* offsets_start = reinterpret_cast<uint8_t GLOBAL_SPACE*>(byte_offset(intersect_input, intersect_input->materials_start));
        uint32_t anyhit_result = call_raytracing_fn_inner(
            global_state,
            offsets_start,
            entry_index,
            launch_idx,
            dim_idx,
            ray_ptr,
            current_time,
            payload,
            // TODO: double check
            *potential_distance,
            make_float2(0.0, 0.0).data,
            zluda_float3(ZLUDA_NAN),
            attribute_block,
            transform_block
        );
        *intersection_result = anyhit_result + 1;
        if (anyhit_result != 1) {
            *new_distance = *potential_distance;
            *material_result = material;
        }
        return anyhit_result != 1;
    }

    __attribute__((always_inline)) uint64_t FUNC(_rt_callable_program_from_id_64)(
        uint32_t program,
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state
    ) {
        GlobalState SHARED_SPACE* globals = (GlobalState SHARED_SPACE*)global_state;
        uint32_t program_offset = globals->callable_programs[program - 1];
        return reinterpret_cast<uint64_t>(reinterpret_cast<uint8_t GLOBAL_SPACE *>(globals->callable_programs) + program_offset);
    }

    __attribute__((always_inline)) uint64_t FUNC(_rt_callable_program_from_id_v2_64)(
        uint32_t program,
        uint64_t unused __attribute__((unused)),
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state
    ) {
        return ZLUDA_RT(_rt_callable_program_from_id_64)(program, global_state);
    }

    float4_struct FUNC(_rt_texture_get_f_id)(
        unsigned int tex,
        unsigned int dim,
        float x,
        float y,
        float z,
        float w __attribute__((unused)),
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state
    ) {
        GlobalState SHARED_SPACE* globals = (GlobalState SHARED_SPACE*)global_state;
        hipTextureObject_t tex_object = globals->textures[tex - 1];
        float4 result;
        switch (dim)
        {
        case 1:
            result = tex1D<float4>(tex_object, x);
            break;
        case 2:
            result = tex2D<float4>(tex_object, x, y);
            break;
        case 3:
            result = tex3D<float4>(tex_object, x, y, z);
            break;
        default:
            __builtin_unreachable();
        }
        return float4_struct{result.x, result.y, result.z, result.w};
    }

    uint32_t FUNC(_rt_is_triangle_hit)(zluda_float3 normals)
    {
        return !__builtin_isnan(normals[0]);
    }

    __device__ static inline float ZLUDA_RT(dot3)(float3 p0, zluda_float3 p1)
    {
        return __builtin_fmaf(p0.z, p1[2], __builtin_fmaf(p0.y, p1[1], p0.x*p1[0])); 
    }

    uint32_t FUNC(_rt_is_triangle_hit_front_face)(uint8_t (PRIVATE_SPACE* current_ray)[36], zluda_float3 normals)
    {
        OptixRay* optix_ray = (OptixRay*)reinterpret_cast<OptixRay PRIVATE_SPACE *>(current_ray);
        return !__builtin_isnan(normals[0]) && ZLUDA_RT(dot3)(optix_ray->direction, normals) < 0.0f;
    }

    uint32_t FUNC(_rt_is_triangle_hit_back_face)(uint8_t (PRIVATE_SPACE* current_ray)[36], zluda_float3 normals)
    {
        OptixRay* optix_ray = (OptixRay*)reinterpret_cast<OptixRay PRIVATE_SPACE *>(current_ray);
        return !__builtin_isnan(normals[0]) && ZLUDA_RT(dot3)(optix_ray->direction, normals) > 0.0f;
    }

    struct transform_result
    {
        float x0, x1, x2, x3,
              x4, x5, x6, x7,
              x8, x9, x10, x11,
              x12, x13, x14, x15;
    };
    transform_result FUNC(_rt_get_transform)(uint32_t kind, uint8_t GLOBAL_SPACE* transform_block)
    {
        if (transform_block == nullptr)
        {
            return transform_result
            {
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            };
        }
        OptixTransform GLOBAL_SPACE* optix_transform = reinterpret_cast<OptixTransform GLOBAL_SPACE*>(transform_block);
        if(kind == 3841)
        {
            float GLOBAL_SPACE* m = optix_transform->transform_matrix;
            return transform_result{ m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8], m[9], m[10], m[11], m[12], m[13], m[14], m[15] };
        } 
        else if (kind == 3840)
        {
            float GLOBAL_SPACE* m = optix_transform->inverse_transform_matrix;
            return transform_result{ m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8], m[9], m[10], m[11], m[12], m[13], m[14], m[15] };
        }
        else
        {
            transform_result result = {};
            return result;
        }
    }

    float4_struct FUNC(_rt_texture_grad_load_or_request_f_id)(
        unsigned int tex,
        unsigned int dim,
        float x,
        float y,
        float z,
        float w __attribute__((unused)),
        float dPdx_x,
        float dPdx_y,
        float dPdx_z,
        float dPdy_x,
        float dPdy_y,
        float dPdy_z,
        uint64_t isResident,
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state
    ) {
        GlobalState SHARED_SPACE* globals = (GlobalState SHARED_SPACE*)global_state;
        hipTextureObject_t tex_object = globals->textures[tex - 1];
        float4 result;
        switch (dim)
        {
        case 1:
            result = tex1DGrad<float4>(tex_object, x, dPdx_x, dPdy_x);
            break;
        case 2:
            result = tex2DGrad<float4>(tex_object, x, y, float2(dPdx_x, dPdx_y), float2(dPdy_x, dPdy_y));
            break;
        case 3:
            result = tex3DGrad<float4>(tex_object, x, y, z, float4(dPdx_x, dPdx_y, dPdx_z, 0.0), float4(dPdy_x, dPdy_y, dPdy_z, 0.0));
            break;
        default:
            __builtin_unreachable();
        }
        *((uint8_t*)isResident) = 1;
        return float4_struct{result.x, result.y, result.z, result.w};
    }

    float4_struct FUNC(_rt_texture_lod_load_or_request_f_id)(
        unsigned int tex,
        unsigned int dim,
        float x,
        float y,
        float z,
        float w __attribute__((unused)),
        float level __attribute__((unused)), // TODO: respect lod argument when HIP fixes mipmapped arrays
        uint64_t isResident,
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state
    ) {
        GlobalState SHARED_SPACE* globals = (GlobalState SHARED_SPACE*)global_state;
        hipTextureObject_t tex_object = globals->textures[tex - 1];
        float4 result;
        switch (dim)
        {
        case 1:
            result = tex1D<float4>(tex_object, x);
            break;
        case 2:
            result = tex2D<float4>(tex_object, x, y);
            break;
        case 3:
            result = tex3D<float4>(tex_object, x, y, z);
            break;
        default:
            __builtin_unreachable();
        }
        *((uint8_t*)isResident) = 1;
        return float4_struct{result.x, result.y, result.z, result.w};
    }

    //  _  _   _   ___ _  __    _   _    ___ ___ _____
    // | || | /_\ / __| |/ /   /_\ | |  | __| _ \_   _|
    // | __ |/ _ \ (__| ' <   / _ \| |__| _||   / | |
    // |_||_/_/ \_\___|_|\_\ /_/ \_\____|___|_|_\ |_|
    //
    // Normal ZLUDA tex functions operate on texturereferences, we do a special
    // override here to operate on textureobjects.
    // This is temporary until we fix ZLUDA tex functions to also use
    // textureobjects

#define tex_1d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE)                                                                                            \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_1d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, HIP_COORD_TYPE##1 ::Native_vec_ x) \
    {                                                                                                                                                                 \
        hipTextureObject_t textureObject = *texture_ptr;                                                                                                              \
        return tex1Dfetch<HIP_CHANNEL_TYPE##4>(textureObject, int(x.x)).data;                                                                                         \
    }

    __device__ half4 __ockl_image_loadh_1Db(unsigned int CONSTANT_SPACE *i, int c);
#define tex_1d_f16(COORD_TYPE, HIP_COORD_TYPE)                                                                               \
    half4 FUNC(tex_1d_v4_f16_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, HIP_COORD_TYPE##1 ::Native_vec_ x) \
    {                                                                                                                        \
        hipTextureObject_t textureObject = *texture_ptr;                                                                     \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                      \
        (void)s;                                                                                                             \
        return __ockl_image_loadh_1Db(i, int(x.x));                                                                          \
    }

    tex_1d(u32, uint, s32, int);
    tex_1d(u32, uint, f32, float);
    tex_1d(s32, int, s32, int);
    tex_1d(s32, int, f32, float);
    tex_1d(f32, float, s32, int);
    tex_1d(f32, float, f32, float);
    tex_1d_f16(s32, int);
    tex_1d_f16(f32, float);

#define tex_2d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE)                                                                                                \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_2d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, HIP_COORD_TYPE##2 ::Native_vec_ coord) \
    {                                                                                                                                                                     \
        hipTextureObject_t textureObject = *texture_ptr;                                                                                                                  \
        return tex2D<HIP_CHANNEL_TYPE##4>(textureObject, float(coord.x), float(coord.y)).data;                                                                            \
    }

    __device__ half4 __ockl_image_sampleh_2D(unsigned int CONSTANT_SPACE *i, unsigned int ADDRESS_SPACE_CONSTANT *s, float2::Native_vec_ c);
#define tex_2d_f16(COORD_TYPE, HIP_COORD_TYPE)                                                                                   \
    half4 FUNC(tex_2d_v4_f16_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, HIP_COORD_TYPE##2 ::Native_vec_ coord) \
    {                                                                                                                            \
        hipTextureObject_t textureObject = *texture_ptr;                                                                         \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                          \
        return __ockl_image_sampleh_2D(i, s, (float2){float(coord.x), float(coord.y)}.data);                                     \
    }

    tex_2d(u32, uint, s32, int);
    tex_2d(u32, uint, f32, float);
    tex_2d(s32, int, s32, int);
    tex_2d(s32, int, f32, float);
    tex_2d(f32, float, s32, int);
    tex_2d(f32, float, f32, float);
    tex_2d_f16(s32, int);
    tex_2d_f16(f32, float);

#define tex_3d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE)                                                                                                \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_3d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, HIP_COORD_TYPE##4 ::Native_vec_ coord) \
    {                                                                                                                                                                     \
        hipTextureObject_t textureObject = *texture_ptr;                                                                                                                  \
        return tex3D<HIP_CHANNEL_TYPE##4>(textureObject, float(coord.x), float(coord.y), float(coord.z)).data;                                                            \
    }

    __device__ half4 __ockl_image_sampleh_3D(unsigned int CONSTANT_SPACE *i, unsigned int ADDRESS_SPACE_CONSTANT *s, float4::Native_vec_ c);
#define tex_3d_f16(COORD_TYPE, HIP_COORD_TYPE)                                                                                   \
    half4 FUNC(tex_3d_v4_f16_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, HIP_COORD_TYPE##4 ::Native_vec_ coord) \
    {                                                                                                                            \
        hipTextureObject_t textureObject = *texture_ptr;                                                                         \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                          \
        return __ockl_image_sampleh_3D(i, s, (float4){float(coord.x), float(coord.y), float(coord.z), float(coord.w)}.data);     \
    }

    tex_3d(u32, uint, s32, int);
    tex_3d(u32, uint, f32, float);
    tex_3d(s32, int, s32, int);
    tex_3d(s32, int, f32, float);
    tex_3d(f32, float, s32, int);
    tex_3d(f32, float, f32, float);
    tex_3d_f16(s32, int);
    tex_3d_f16(f32, float);

#define tex_a1d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE)                                                                                           \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_a1d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, uint32_t layer, HIP_COORD_TYPE x) \
    {                                                                                                                                                                 \
        hipTextureObject_t textureObject = *texture_ptr;                                                                                                              \
        return tex1DLayered<HIP_CHANNEL_TYPE##4>(textureObject, float(x), int(layer)).data;                                                                           \
    }

    __device__ half4 __ockl_image_sampleh_1Da(unsigned int CONSTANT_SPACE *i, unsigned int ADDRESS_SPACE_CONSTANT *s, float2::Native_vec_ c);
#define tex_a1d_f16(COORD_TYPE, HIP_COORD_TYPE)                                                                              \
    half4 FUNC(tex_a1d_v4_f16_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, uint32_t layer, HIP_COORD_TYPE x) \
    {                                                                                                                        \
        hipTextureObject_t textureObject = *texture_ptr;                                                                     \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                      \
        return __ockl_image_sampleh_1Da(i, s, (float2){float(x), float(layer)}.data);                                        \
    }

    tex_a1d(u32, uint, s32, int);
    tex_a1d(u32, uint, f32, float);
    tex_a1d(s32, int, s32, int);
    tex_a1d(s32, int, f32, float);
    tex_a1d(f32, float, s32, int);
    tex_a1d(f32, float, f32, float);
    tex_a1d_f16(s32, int);
    tex_a1d_f16(f32, float);

#define tex_a2d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE)                                                                                                             \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_a2d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, uint32_t layer, HIP_COORD_TYPE x, HIP_COORD_TYPE y) \
    {                                                                                                                                                                                   \
        hipTextureObject_t textureObject = *texture_ptr;                                                                                                                                \
        return tex2DLayered<HIP_CHANNEL_TYPE##4>(textureObject, float(x), float(y), int(layer)).data;                                                                                   \
    }

    __device__ half4 __ockl_image_sampleh_2Da(unsigned int CONSTANT_SPACE *i, unsigned int ADDRESS_SPACE_CONSTANT *s, float4::Native_vec_ c);
#define tex_a2d_f16(COORD_TYPE, HIP_COORD_TYPE)                                                                                                \
    half4 FUNC(tex_a2d_v4_f16_##COORD_TYPE)(hipTextureObject_t GLOBAL_SPACE * texture_ptr, uint32_t layer, HIP_COORD_TYPE x, HIP_COORD_TYPE y) \
    {                                                                                                                                          \
        hipTextureObject_t textureObject = *texture_ptr;                                                                                       \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                                        \
        return __ockl_image_sampleh_2Da(i, s, (float4){float(x), float(y), float(layer), 0.0}.data);                                           \
    }

    tex_a2d(u32, uint, s32, int);
    tex_a2d(u32, uint, f32, float);
    tex_a2d(s32, int, s32, int);
    tex_a2d(s32, int, f32, float);
    tex_a2d(f32, float, s32, int);
    tex_a2d(f32, float, f32, float);
    tex_a2d_f16(s32, int);
    tex_a2d_f16(f32, float);
}
