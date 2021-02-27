#ifndef SKIP_RAYTRACING_GLOBALS
    #include "raytracing_defines.hpp"

    #ifndef FUNCTION_NAME
        #error FUNCTION_NAME required
    #endif
    #ifndef EXPORTED_FUNCTION
        #error EXPORTED_FUNCTION required
    #endif
    #ifndef EXPORTED_ATTRIBUTE_FUNCTION
        #error EXPORTED_ATTRIBUTE_FUNCTION required
    #endif
    #ifndef EXPORTED_KERNEL
        #error EXPORTED_KERNEL required
    #endif

    #define FUNC(NAME) __attribute__((device)) __zluda_rt_ptx_impl__##NAME

    #define GLOBAL_SPACE __attribute__((address_space(1)))
    #define SHARED_SPACE __attribute__((address_space(3)))
    #define CONSTANT_SPACE __attribute__((address_space(4)))
    #define PRIVATE_SPACE __attribute__((address_space(5)))

    #define EXPORTED(X) X ## EXPORT_SUFFIX

    constexpr float INFINITY = __builtin_inff();

    typedef unsigned char uint8_t;
    typedef unsigned short uint16_t;
#endif

#define ZLUDA_RT(NAME) __zluda_rt_ptx_impl__##NAME
// This function is for argument passing for compatibility with LLVM
// We still use float3 in fields because sizeof(zluda_float3) == 16
typedef float zluda_float3 __attribute__((ext_vector_type(3)));
#define ZLUDA_NAN __int_as_float(0x7fffffff)

struct OptixBuffer {
    uint8_t *ptr;
    uint64_t x;
    uint64_t y;
};

struct OptixTransform {
    float transform_matrix[16];
    float inverse_transform_matrix[16];
};

struct OptixRay {
    // We don't use zluda_float3 below because sizeof(zluda_float3) == 16
    float3 origin;
    float3 direction;
    unsigned int ray_type;
    float tmin;
    float tmax;
};
static_assert((sizeof(OptixRay) == 36), "");

typedef uint32_t (*raytracing_fn)(
    /* vvv INJECTED vvv */
    uint8_t SHARED_SPACE* global_state,
    uint32_t prim_idx,
    uint2::Native_vec_ launch_idx,
    uint2::Native_vec_ launch_dim,
    OptixRay PRIVATE_SPACE* current_ray,
    float current_time,
    uint8_t PRIVATE_SPACE* payload,
    float current_distance,
    float2::Native_vec_ barycentric, // in case it is an attribute function
    zluda_float3 normals, // in case it is an attribute function
    uint8_t GLOBAL_SPACE* variable_block,
    uint8_t GLOBAL_SPACE* attribute_block,
    uint8_t GLOBAL_SPACE* transform_block
);

typedef uint32_t (*attribute_fn)(
    /* vvv INJECTED vvv */
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
    /* vvv EXTRA INJECTED vvv */
    uint64_t any_hit_fn
);

typedef void(*bounding_box_fn)(
    uint32_t prim_idx,
    float GLOBAL_SPACE* result,
    /* vvv INJECTED vvv */
    uint8_t SHARED_SPACE* global_state,
    uint8_t GLOBAL_SPACE* variable_block
);

/*
       uint32_t            variable             uint32_t           uint32_t                    variable               variable
┌┬─────────────────┬┬────────────────────┬┬──────────────────┬──────────────────┬─────┬┬──────────────────────┬──────────────────────┬─────┬┐
││ materials_start ││var_block_intersect ││ offset_mat0_ray0 │ offset_mat0_ray1 │ ... ││ prog_block_mat0_ray0 │ prog_block_mat0_ray1 │ ... ││
└┴────────┬────────┴┴────────────────────┘▲─────────┬────────┴────────┬─────────┴─────┘▲──────────────────────▲──────────────────────┴─────┴┘
          │                               │         │                 │                │                      │
          └───────────────────────────────┘         │                 └────────────────┼──────────────────────┘
                                                    │                                  │
                                                    └──────────────────────────────────┘
 */
struct IntersectionInput {
    uint8_t GLOBAL_SPACE* transform_block;
    uint32_t materials_start;
};

struct IntersectionPayload {
    uint32_t result;
    uint32_t ray_type;
    float tmin;
    float tmax;
    float time;
    uint8_t SHARED_SPACE* global_state;
    uint8_t PRIVATE_SPACE* payload;
    float closest_distance;
    uint32_t material;
    uint8_t GLOBAL_SPACE* attribute_block;
};

template <typename T>
__device__ static inline T get_program_block_inner(
    void GLOBAL_SPACE *offset_block,
    uint32_t prim_idx
) {
    uint32_t byte_offset = reinterpret_cast<uint32_t GLOBAL_SPACE *>(offset_block)[prim_idx];
    if (byte_offset == ~uint32_t(0))
        return nullptr;
    uint8_t GLOBAL_SPACE *program_block = reinterpret_cast<uint8_t GLOBAL_SPACE *>(offset_block) + byte_offset;
    return (T)(program_block);
}

template <typename T>
__device__ static inline T get_program_block_outer(
    void GLOBAL_SPACE *offset_block,
    uint32_t id,
    uint32_t subid
) {
    uint32_t byte_offset = reinterpret_cast<uint32_t GLOBAL_SPACE*>(offset_block)[id];
    if (byte_offset == ~uint32_t(0))
        return nullptr;
    if (byte_offset & 1U) { // is bottom bit set?
        byte_offset &= ~(1U); // clear bottom bit
        uint8_t GLOBAL_SPACE* inner_chain = reinterpret_cast<uint8_t GLOBAL_SPACE*>(offset_block) + byte_offset;
        return get_program_block_inner<T>(
            inner_chain,
            subid
        );
    } else {
        uint8_t GLOBAL_SPACE* program_block = reinterpret_cast<uint8_t GLOBAL_SPACE*>(offset_block) + byte_offset;
        return (T)(program_block);
    }
}
__device__ static inline uint32_t call_raytracing_fn_inner(
    uint8_t SHARED_SPACE* global_state,
    void GLOBAL_SPACE* offset_block,
    uint32_t prim_idx,
    uint2::Native_vec_ launch_idx,
    uint2::Native_vec_ launch_dim,
    OptixRay PRIVATE_SPACE* current_ray,
    float current_time,
    uint8_t PRIVATE_SPACE* payload,
    float current_distance,
    float2::Native_vec_ barycentric,
    zluda_float3 normals,
    uint8_t GLOBAL_SPACE* attribute_block,
    uint8_t GLOBAL_SPACE* transform_block
) {
    uint32_t byte_offset = reinterpret_cast<uint32_t GLOBAL_SPACE*>(offset_block)[prim_idx];
    if (byte_offset == ~uint32_t(0))
        return 0;
    uint8_t GLOBAL_SPACE* program_block = reinterpret_cast<uint8_t GLOBAL_SPACE*>(offset_block) + byte_offset;
    raytracing_fn GLOBAL_SPACE* fn = reinterpret_cast<raytracing_fn GLOBAL_SPACE*>(program_block);
    return (*fn)(
        global_state,
        prim_idx,
        launch_idx,
        launch_dim,
        current_ray,
        current_time,
        payload,
        current_distance,
        barycentric,
        normals,
        program_block,
        attribute_block,
        transform_block
    );
}

template<typename T>
__device__ void GLOBAL_SPACE* byte_offset(T GLOBAL_SPACE* ptr, uint32_t offset) {
    uint8_t GLOBAL_SPACE* byte_ptr = reinterpret_cast<uint8_t GLOBAL_SPACE*>(ptr);
    return reinterpret_cast<void GLOBAL_SPACE*>(byte_ptr + offset);
}

// https://stackoverflow.com/a/9194117
__device__ static inline int ZLUDA_RT(round_up)(int number, int multiple) 
{
    return (number + multiple - 1) & -multiple;
}

struct BvhDetails;

struct GlobalState {
    BvhDetails CONSTANT_SPACE* scenes;
    void CONSTANT_SPACE* miss_programs;
    OptixBuffer CONSTANT_SPACE* buffers;
    uint32_t GLOBAL_SPACE* callable_programs;
    hipTextureObject_t CONSTANT_SPACE* textures;
    uint32_t ray_type_count;
    uint32_t uv_attribute_offset;
    uint32_t width;
    uint32_t height;
    uint16_t attribute_block_size; // in DWORDs
    uint16_t attribute_block_align; // in DWORDs
    uint16_t thread_global_stack_size;
    uint16_t _padding;
};
static_assert((sizeof(GlobalState) == 64), "");

// We are executing with 256 VGPRs/per wavefront (due to virtual calls), consequently
// * Dual CU has four SIMDs
// * One SIMD has 1024 VGPRs
// * At maximum 16 wavefronts / dual CU
// * Dual CU has 128kB LDS
// * At minimum 8kBs LDS per wavefront
// * At minimum 2048 dwords LDS per wavefront
// * At minimum 64 dwords per thread
// We might raise LDS/thread in the future
struct OptixStack {
    constexpr static auto LOCAL_STACK_SIZE = 61;
    constexpr static auto LOCAL_STACK_BITS = 6;
    constexpr static auto LOCAL_STACK_MASK = 63;

    GlobalState globals;
    // Bottom 6 bits are for LDS, stack, top 10 bits are for global stack
    // Both are in dwords
    uint32_t stack_pointers[32];
    uint32_t stacks[32 * LOCAL_STACK_SIZE];
    uint32_t GLOBAL_SPACE* global_stack;

    __device__ uint32_t& stack_pointer(void) {
        return this->stack_pointers[__lane_id()];
    }

    // does not change the value of stack offset
    __device__ uint32_t soft_push_attribute_block(uint8_t GLOBAL_SPACE** stack_pointer) {
        uint32_t size = this->globals.attribute_block_size;
        uint32_t align = this->globals.attribute_block_align;
        uint32_t original_stack_pointer = this->stack_pointer();
        uint32_t global_stack_offset = original_stack_pointer >> LOCAL_STACK_BITS;
        uint32_t aligned_start_in_dwords = uint32_t(ZLUDA_RT(round_up)(int32_t(global_stack_offset), int32_t(align)));
        uint32_t aligned_start = this->globals.thread_global_stack_size * uint32_t(__lane_id()) + aligned_start_in_dwords;
        *stack_pointer = (uint8_t GLOBAL_SPACE*)&this->global_stack[aligned_start];
        return (original_stack_pointer & LOCAL_STACK_MASK) | ((aligned_start_in_dwords + size) << LOCAL_STACK_BITS);
    }

    __device__ void init(
        uint32_t local_x,
        uint32_t local_y,
        uint32_t wavefront_id,
        GlobalState globals_,
        void GLOBAL_SPACE* global_stack_space
    ) {
        if (local_x == 0 && local_y == 0) {
            this->globals = globals_;
        }
        if (__lane_id() == 0)
        {
            uint32_t wavefront_stack_offset = globals_.thread_global_stack_size * 32 * wavefront_id;
            this->global_stack = &((uint32_t GLOBAL_SPACE*)global_stack_space)[wavefront_stack_offset];
        }
        this->stack_pointers[__lane_id()] = 0;
    }
};
static_assert((sizeof(OptixStack) <= 8192), "");

static __shared__ OptixStack ZLUDA_RT(STACK);
static __shared__ GlobalState ZLUDA_RT(BB_GLOBALS);

#ifndef SKIP_RAYTRACING_GLOBALS
/*
     uint32_t           uint32_t                    uint32_t    uint32_t              variable          variable                uint32_t               variable
┌┬───────────────┬┬───────────────────┬┬─────────┬┬───────────┬───────────┬─────┬┬─────────────────┬─────────────────┬─────┬┬──────────────┬─────┬┬─────────────────┬─────┬┐
││ any_hit_start ││ closest_hit_start ││ padding ││ any_hit_0 │ any_hit_1 │ ... ││ prog_block_ah_0 │ prog_block_ah_1 │ ... ││ closet_hit_0 │ ... ││ prog_block_ch_0 │ ... ││
└┴────┬──────────┴┴────────┬──────────┴┴─────────┘▲────────┬──┴────────┬──┴─────┘▲─────────────────▲─────────────────┴─────┘►──────┬───────┴─────┘▲─────────────────┴─────┴┘
      │                    │                      │        │           │         │                 │                        │      │              │
      │                    │                      │        │           └─────────┼─────────────────┘                        │      └──────────────┘
      │                    │                      │        │                     │                                          │
      └────────────────────┼──────────────────────┘        └─────────────────────┘                                          │
                           │                                                                                                │
                           └────────────────────────────────────────────────────────────────────────────────────────────────┘
 */
// Due to alignment there might be padding between those two variables and the chain proper
struct HitProgramChain {
    uint32_t any_hit_start;
    uint32_t closest_hit_start;
};

struct BvhDetails {
    hiprtScene scene;
    hiprtCustomFuncTable func_set;
    void CONSTANT_SPACE* attribute_programs;
    uint8_t GLOBAL_SPACE* GLOBAL_SPACE* transform_blocks;
    HitProgramChain GLOBAL_SPACE* GLOBAL_SPACE* hit_chains;
};

template<typename T>
__device__ T GLOBAL_SPACE* global_space_cast(T CONSTANT_SPACE* ptr) {
    return (T GLOBAL_SPACE*)(ptr);
}

extern "C" {
    struct ZLUDA_RT(TraversalStack) {
        __device__ int  pop() {
            OptixStack SHARED_SPACE* stack = (OptixStack SHARED_SPACE*)0;
            if (global_offset > (bottom_pointer >> OptixStack::LOCAL_STACK_BITS)) {
                return thread_global_stack[--global_offset];
            } else {
                auto pos = (__lane_id() * OptixStack::LOCAL_STACK_SIZE) + --shared_offset;
                return stack->stacks[pos];
            }
        }
        __device__ void push( int val ) {
            OptixStack SHARED_SPACE* stack = (OptixStack SHARED_SPACE*)0;
            if (shared_offset != OptixStack::LOCAL_STACK_SIZE) {
                auto pos = (__lane_id() * OptixStack::LOCAL_STACK_SIZE) + shared_offset++;
                stack->stacks[pos] = val;
            } else {
                thread_global_stack[global_offset++] = val;
            }
        }
        __device__ bool empty() const {
            return (global_offset == (bottom_pointer >> OptixStack::LOCAL_STACK_BITS))
                && (shared_offset == (bottom_pointer & OptixStack::LOCAL_STACK_MASK));
        }
        __device__ int  vacancy() const {
            int32_t global_vacancy = int32_t(global_stack_size) - int32_t(global_offset);
            int32_t local_vacancy = int32_t(OptixStack::LOCAL_STACK_SIZE) - int32_t(shared_offset);
            int32_t vacancy =  global_vacancy + local_vacancy;
            return vacancy;
        }
        __device__ void reset() {
            global_offset = (bottom_pointer >> OptixStack::LOCAL_STACK_BITS);
            shared_offset = (bottom_pointer & OptixStack::LOCAL_STACK_MASK);
        }

        __device__ ZLUDA_RT(TraversalStack)(uint32_t stack_pointer) : bottom_pointer(stack_pointer) {
            OptixStack SHARED_SPACE* stack = (OptixStack SHARED_SPACE*)0;
            global_stack_size = stack->globals.thread_global_stack_size;
            thread_global_stack = &stack->global_stack[global_stack_size * __lane_id()];
            reset();
        }

        uint32_t GLOBAL_SPACE* thread_global_stack;
        const uint32_t bottom_pointer;
        uint32_t global_stack_size;
        uint32_t global_offset;
        uint16_t shared_offset;
    };

    /*
      uint32_t uint32_t             variable            variable
    ┌┬────────┬────────┬─────┬┬───────────────────┬───────────────────┬─────┬┐
    ││ miss_0 │ miss_1 │ ... ││ prog_block_miss_0 │ prog_block_miss_1 │ ... ││
    └┴───┬────┴───┬────┴─────┘▲───────────────────▲───────────────────┴─────┴┘
         │        │           │                   │
         │        └───────────┼───────────────────┘
         │                    │
         └────────────────────┘
    */
// HACK START
}

using BoxNode = hiprt::BoxNode;
using Ray = hiprt::Ray;
using InstanceNode = hiprt::InstanceNode;
using CustomNode = hiprt::CustomNode;
using TriangleNode = hiprt::TriangleNode;
using GeomHeader = hiprt::GeomHeader;
using Frame = hiprt::Frame;
using SceneHeader = hiprt::SceneHeader;
using Transform = hiprt::Transform;
template <typename Stack>
using TraversalBase = hiprt::TraversalBase<Stack>;
template<u32 StackSize>
using PrivateStack = hiprt::PrivateStack<StackSize>;

struct IntersectResult {
    bool hit;
    float t;
    float2 uv;
    float3 normal;
};

struct hiprtHit2 {
	uint32_t instanceID;
	uint32_t primID;
	float2 uv;
	float3 normal;
	float t;
    float3 origin;
    float3 direction;

	HIPRT_DEVICE bool hasHit() const { return primID != hiprtInvalidValue; }
};

typedef IntersectResult ( *hiprtIntersectFunc2 )(hiprtRay ray, u32 primIdx, const void* data, void* payload );

template <typename Stack, hiprtTraversalType TraversalType>
class SceneTraversal2 : public TraversalBase<Stack>
{
public:
	HIPRT_DEVICE SceneTraversal2( hiprtScene scene, const hiprtRay& ray, hiprtRayMask mask, Stack& stack );

	HIPRT_DEVICE SceneTraversal2(
		hiprtScene scene, const hiprtRay& ray, hiprtRayMask mask, hiprtCustomFuncTable funcTable, Stack& stack, void* payload = nullptr );

	HIPRT_DEVICE void transformRay( Ray& ray, float3& invD, float3& oxInvD, const InstanceNode& node ) const;

	HIPRT_DEVICE void restoreRay( Ray& ray, float3& invD, float3& oxInvD ) const;

    HIPRT_DEVICE void set_ray_maxT(float x)
    {
        m_ray.maxT = x;
    }

	HIPRT_DEVICE bool testLeafNode(
		const Ray&	  ray,
		const float3& invD,
		int			  leafIndex,
		int			  shapeId,
		int&		  hitIndex,
		float2&		  hitUv,
		float3&		  hitNormal,
		float&		  hitT );

	HIPRT_DEVICE hiprtHit2 getNextHit2();
	HIPRT_DEVICE hiprtHit getNextHit() {
        abort();
        return hiprtHit {};
    }

protected:
	using TraversalBase<Stack>::m_ray;
	using TraversalBase<Stack>::m_state;
	using TraversalBase<Stack>::m_stack;
	using TraversalBase<Stack>::m_payload;
#if defined( __USE_HWI__ )
	using TraversalBase<Stack>::m_descriptor;
#endif

	int					 m_nodeIndex;
	int					 m_instanceIndex;
	BoxNode*			 m_boxNodes;
	InstanceNode*		 m_instanceNodes;
	GeomHeader**		 m_geoms;
	Frame*				 m_frames;
	hiprtRayMask		 m_mask;
	hiprtCustomFuncTable m_funcTable;
};

template <typename Stack, hiprtTraversalType TraversalType>
HIPRT_DEVICE SceneTraversal2<Stack, TraversalType>::SceneTraversal2( hiprtScene scene, const hiprtRay& ray, hiprtRayMask mask, Stack& stack )
	: TraversalBase<Stack>( ray, stack, nullptr ), m_funcTable( nullptr ), m_mask( mask ), m_instanceIndex( hiprtInvalidValue ), m_nodeIndex( hiprt::RootIndex )
{
	SceneHeader* data = (SceneHeader*)scene;
	m_boxNodes		  = data->m_boxNodes;
	m_instanceNodes	  = data->m_primNodes;
	m_geoms			  = data->m_geoms;
	m_frames		  = data->m_frames;
	m_stack.reset();
}

template <typename Stack, hiprtTraversalType TraversalType>
HIPRT_DEVICE SceneTraversal2<Stack, TraversalType>::SceneTraversal2(
	hiprtScene scene, const hiprtRay& ray, hiprtRayMask mask, hiprtCustomFuncTable funcTable, Stack& stack, void* payload )
	: TraversalBase<Stack>( ray, stack, payload ), m_funcTable( funcTable ), m_mask( mask ), m_instanceIndex( hiprtInvalidValue ),
	  m_nodeIndex( hiprt::RootIndex )
{
	SceneHeader* data = (SceneHeader*)scene;
	m_boxNodes		  = data->m_boxNodes;
	m_instanceNodes	  = data->m_primNodes;
	m_geoms			  = data->m_geoms;
	m_frames		  = data->m_frames;
	m_stack.reset();
}

__device__ static inline float3 invTransform2(Frame& frame, const float3& p )
{
	float3 result = p;
	result = result - frame.m_translation;
	result = hiprt::qtInvRotate( hiprt::qtFromAxisAngle( frame.m_rotation ), result );
	result = result / frame.m_scale;
	return result;
}

__device__ static inline Ray transformRay2(Transform& transform, Ray& ray)
{
	Frame frame		   = transform.interpolateFrames( ray.m_time );
    if ( frame.m_translation.x != 0.0f || frame.m_translation.y != 0.0f || frame.m_translation.z != 0.0f
        || frame.m_rotation.w != 0.0f
        || frame.m_scale.x != 1.0f || frame.m_scale.y != 1.0f || frame.m_scale.z != 1.0f )
    {
		Ray	  outRay;
		outRay.m_origin	   = invTransform2(frame, ray.m_origin );
		outRay.m_direction = invTransform2(frame, ray.m_origin + ray.m_direction );
		outRay.m_direction = outRay.m_direction - outRay.m_origin;
		outRay.m_time	   = ray.m_time;
		outRay.m_maxT	   = ray.m_maxT;
        return outRay;
    }
    else 
    {
        return ray;
    }
}

template <typename Stack, hiprtTraversalType TraversalType>
HIPRT_DEVICE void SceneTraversal2<Stack, TraversalType>::transformRay(
	Ray& ray, float3& invD, float3& oxInvD, const InstanceNode& node ) const
{
	Transform tr( m_frames + node.m_frameIndex, node.m_frameCount );
	ray	   = transformRay2( tr, ray );
	invD   = hiprt::safeInv( ray.m_direction );
	oxInvD = -ray.m_origin * invD;
}

template <typename Stack, hiprtTraversalType TraversalType>
HIPRT_DEVICE void
SceneTraversal2<Stack, TraversalType>::restoreRay( Ray& ray, float3& invD, float3& oxInvD ) const
{
	ray.m_origin	= m_ray.origin;
	ray.m_direction = m_ray.direction;
	invD			= hiprt::safeInv( m_ray.direction );
	oxInvD			= -m_ray.origin * invD;
}

template <typename Stack, hiprtTraversalType TraversalType>
HIPRT_DEVICE bool SceneTraversal2<Stack, TraversalType>::testLeafNode(
	const Ray&	  ray,
	const float3& invD,
	int			  leafIndex,
	int			  instanceId,
	int&		  hitIndex,
	float2&		  hitUv,
	float3&		  hitNormal,
	float&		  hitT )
{
	bool			  hit  = false;
	const GeomHeader* geom = m_geoms[instanceId];
	int				  type = geom->m_customType;
	if ( type == hiprtInvalidValue )
	{
		TriangleNode node = ( (TriangleNode*)geom->m_primNodes )[hiprt::getNodeAddr( leafIndex )];
#if !defined( __USE_HWI__ )
		hit = node.m_triangle.intersect( ray, hitUv, hitT );
		if ( hit )
		{
			hitIndex  = node.m_primIndex;
			hitNormal = node.m_triangle.normal();
		}
#else
		auto result = __builtin_amdgcn_image_bvh_intersect_ray_l(
			hiprt::encodeBaseAddr( geom->m_primNodes, leafIndex ),
			ray.m_maxT,
			make_float4( ray.m_origin.x, ray.m_origin.y, ray.m_origin.z, ray.m_origin.z ).data,
			make_float4( ray.m_direction.x, ray.m_direction.y, ray.m_direction.z, ray.m_direction.z ).data,
			make_float4( invD.x, invD.y, invD.z, invD.z ).data,
			m_descriptor.data );
		float invDenom = __ocml_native_recip_f32( __int_as_float( result[1] ) );
		float t		   = __int_as_float( result[0] ) * invDenom;
		hit			   = t <= ray.m_maxT;
		if ( hit )
		{
			hitT	  = t;
			hitUv.x	  = __int_as_float( result[2] ) * invDenom;
			hitUv.y	  = __int_as_float( result[3] ) * invDenom;
			hitIndex  = node.m_primIndex;
			hitNormal = node.m_triangle.normal();
		}
#endif
	}
	else
	{
		CustomNode			node	  = ( (CustomNode*)geom->m_primNodes )[hiprt::getNodeAddr( leafIndex )];
		hiprtCustomFuncSet* funcTable = (hiprtCustomFuncSet*)m_funcTable;
		IntersectResult     result    = ((hiprtIntersectFunc2)(funcTable[type].intersectFunc))(
			  reinterpret_cast<const hiprtRay&>(ray),
			  node.m_primIndex,
			  funcTable[type].intersectFuncData,
			  m_payload );
		if ( result.hit ) {
            hitIndex = node.m_primIndex;
            hit = true;
            hitT = result.t;
            hitUv = result.uv;
            hitNormal = result.normal;
        }
	}
	return hit;
}


template <typename Stack, hiprtTraversalType TraversalType>
HIPRT_DEVICE hiprtHit2 SceneTraversal2<Stack, TraversalType>::getNextHit2()
{
	int instanceId	  = hiprtInvalidValue;
	int nodeIndex	  = m_nodeIndex;
	int instanceIndex = m_instanceIndex;

	Ray	   ray = *(Ray*)&m_ray;
	float3 invD, oxInvD;
	if ( instanceIndex == hiprtInvalidValue )
	{
		restoreRay( ray, invD, oxInvD );
	}
	else
	{
		InstanceNode node = m_instanceNodes[hiprt::getNodeAddr( instanceIndex )];
		instanceId		  = node.m_primIndex;
		transformRay( ray, invD, oxInvD, node );
	}

	int	   hitInstanceId	= hiprtInvalidValue;
	int	   hitInstanceIndex = hiprtInvalidValue;
	int	   hitPrimIndex		= hiprtInvalidValue;
	float  hitT				= 0.0f;
	float2 hitUv			= make_float2( 0.0f, 0.0f );
	float3 hitNormal;

	if ( m_stack.empty() ) m_stack.push( hiprtInvalidValue );

	while ( nodeIndex != hiprtInvalidValue && m_state != hiprtTraversalStateStackOverflow )
	{
		if ( hiprt::isInternalNode( nodeIndex ) )
		{
			BoxNode* nodes = ( instanceId == hiprtInvalidValue ) ? m_boxNodes : m_geoms[instanceId]->m_boxNodes;
			if ( this->testInternalNode( ray, invD, oxInvD, nodes, nodeIndex ) )
				continue;
		}
		else
		{
			if ( instanceId != hiprtInvalidValue )
			{
				if ( testLeafNode( ray, invD, nodeIndex, instanceId, hitPrimIndex, hitUv, hitNormal, hitT ) )
				{
					if constexpr ( TraversalType == hiprtTraversalTerminateAtAnyHit )
					{
						m_nodeIndex		= m_stack.pop();
						m_instanceIndex = instanceIndex;
						m_state			= hiprtTraversalStateHit;
						if ( m_nodeIndex == hiprtInvalidValue && !m_stack.empty() )
						{
							m_instanceIndex = hiprtInvalidValue;
							m_nodeIndex		= m_stack.pop();
						}
						InstanceNode& node = m_instanceNodes[hiprt::getNodeAddr( instanceIndex )];
						Transform	 tr( m_frames + node.m_frameIndex, node.m_frameCount );
						hitNormal = tr.transformNormal( hitNormal, ray.m_time );
						return hiprtHit2
                        { 
                            (u32)instanceId,
                            (u32)hitPrimIndex,
                            hitUv,
                            hitNormal,
                            hitT,
                            ray.m_origin,
                            ray.m_direction
                        };
					}
					else
					{
						hitInstanceId	 = instanceId;
						hitInstanceIndex = instanceIndex;
						ray.m_maxT		 = hitT;
					}
				}
			}
			else
			{
				InstanceNode node = m_instanceNodes[hiprt::getNodeAddr( nodeIndex )];
				if ( node.m_mask & m_mask )
				{
					if ( instanceId != node.m_primIndex )
					{
						if ( m_stack.vacancy() < 1 )
						{
							m_state = hiprtTraversalStateStackOverflow;
							continue;
						}
						m_stack.push( hiprtInvalidValue );
					}
					instanceIndex = nodeIndex;
					nodeIndex	  = hiprt::RootIndex;
					instanceId	  = node.m_primIndex;
					transformRay( ray, invD, oxInvD, node );
					continue;
				}
			}
		}
		nodeIndex = m_stack.pop();
		if ( nodeIndex == hiprtInvalidValue && !m_stack.empty() )
		{
			instanceIndex = hiprtInvalidValue;
			instanceId	  = hiprtInvalidValue;
			nodeIndex	  = m_stack.pop();
			restoreRay( ray, invD, oxInvD );
		}
	}

	hiprtHit2 result{ hiprtInvalidValue, hiprtInvalidValue, { 0.0f, 0.0f }, { 0.0f, 0.0f, 0.0f }, -1.0f, { 0.0f, 0.0f, 0.0f }, { 0.0f, 0.0f, 0.0f } };
	if ( hitInstanceId != hiprtInvalidValue )
	{
		InstanceNode node = m_instanceNodes[hiprt::getNodeAddr( hitInstanceIndex )];
		Transform	 tr( m_frames + node.m_frameIndex, node.m_frameCount );
		result.t		  = hitT;
		result.instanceID = hitInstanceId;
		result.primID	  = hitPrimIndex;
		result.uv		  = hitUv;
		result.normal	  = tr.transformNormal( hitNormal, ray.m_time );
		result.origin	  = ray.m_origin;
		result.direction  = ray.m_direction;
	}
	if ( m_state != hiprtTraversalStateStackOverflow )
		m_state = hiprtTraversalStateFinished;
	return result;
}

__device__ static inline void ZLUDA_RT(set_attr_barycentrics)(
    uint8_t GLOBAL_SPACE* attr_block,
    float2 value
) {
    OptixStack SHARED_SPACE* stack = (OptixStack SHARED_SPACE*)0;
    if (stack->globals.uv_attribute_offset != ~uint32_t(0)) {
        *(float2*)(attr_block + stack->globals.uv_attribute_offset) = value;
    }
}

extern "C" {
// HACK END
    __device__ uint32_t FUNC(_rt_trace_time_mask_flags_64) (
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
        /* vvv INJECTED vvv */
        uint8_t SHARED_SPACE* global_state
    ) {
        OptixStack SHARED_SPACE* stack = (OptixStack SHARED_SPACE*)global_state;
        uint32_t launch_x = blockIdx.x * blockDim.x + threadIdx.x;
        uint32_t launch_y = blockIdx.y * blockDim.y + threadIdx.y;
        uint2::Native_vec_ launch_idx = uint2(launch_x, launch_y).data;
        uint32_t size_x = stack->globals.width;
        uint32_t size_y = stack->globals.height;
        uint2::Native_vec_ launch_dim = uint2(size_x, size_y).data;
        hiprtRay ray {
            make_float3(ray_origin_x, ray_origin_y, ray_origin_z),
            time,
            make_float3(ray_direction_x, ray_direction_y, ray_direction_z),
            tmax
        };
        OptixRay optix_ray {
            make_float3(ray_origin_x, ray_origin_y, ray_origin_z),
            make_float3(ray_direction_x, ray_direction_y, ray_direction_z),
            ray_type,
            tmin,
            tmax
        };
        PRIVATE_SPACE uint8_t* payload = reinterpret_cast<PRIVATE_SPACE uint8_t*>(payload_ptr);
        uint32_t ray_type_count = stack->globals.ray_type_count;
        BvhDetails CONSTANT_SPACE* bvh = &stack->globals.scenes[bvh_index];
        HitProgramChain GLOBAL_SPACE* programs_chain = bvh->hit_chains[ray_type];
        IntersectionPayload intersection_payload {
            0,
            ray_type,
            tmin,
            tmax,
            time,
            global_state,
            payload,
            tmax,
            ~uint32_t(0)
        };
        uint32_t old_stack_bottom = ((OptixStack*)stack)->stack_pointer();
        uint32_t new_stack_bottom = ((OptixStack*)stack)->soft_push_attribute_block(&intersection_payload.attribute_block);
        ZLUDA_RT(TraversalStack) rt_stack(new_stack_bottom);
        if (rt_stack.vacancy() < 1) {
            abort();
        }
        SceneTraversal2<ZLUDA_RT(TraversalStack), hiprtTraversalTerminateAtAnyHit> tr { bvh->scene, ray, mask, bvh->func_set, rt_stack, &intersection_payload };
        unsigned int closest_instance_id = hiprtInvalidValue;
        unsigned int closest_subinstance_id = 0;
        attribute_fn* closest_attr_fn = nullptr;
        float2 closest_uv {};
        float3 closest_normals(ZLUDA_NAN, ZLUDA_NAN, ZLUDA_NAN);
        while (true) {
            intersection_payload.result = 0;
            intersection_payload.material = ~uint32_t(0);
            hiprtHit2 hit = tr.getNextHit2();
            if (tr.getCurrentState() == hiprtTraversalStateStackOverflow) {
                abort();
            }
            if (!hit.hasHit())
                break;
            if (hit.t < tmin)
                continue; 
            tr.set_ray_maxT(hit.t);
            // Successful custom intersection hit
            if (intersection_payload.material != ~uint32_t(0)) {
                if (intersection_payload.result != 1) {
                    closest_attr_fn = nullptr;
                    closest_normals = {ZLUDA_NAN, ZLUDA_NAN, ZLUDA_NAN};
                    closest_instance_id = hit.instanceID;
                    closest_subinstance_id = intersection_payload.material;
                    intersection_payload.closest_distance = hit.t;
                }
                if (intersection_payload.result == 2)
                    break;
                continue;
            }
            attribute_fn* attr_fn = get_program_block_inner<attribute_fn*>(
                global_space_cast(bvh->attribute_programs),
                hit.instanceID
            );
            raytracing_fn* anyhit_fn = get_program_block_outer<raytracing_fn*>(
                byte_offset(programs_chain, programs_chain->any_hit_start),
                hit.instanceID,
                hit.primID
            );
            uint32_t intersection_result = 0;
            if (attr_fn != nullptr) {
                OptixRay transformed_ray {
                    { hit.origin.x, hit.origin.y, hit.origin.z },
                    { hit.direction.x, hit.direction.y, hit.direction.z },
                    ray_type,
                    tmin,
                    tmax
                };
                intersection_result = (*attr_fn)(
                    global_state,
                    hit.primID,
                    launch_idx,
                    launch_dim,
                    (OptixRay PRIVATE_SPACE*)(&transformed_ray),
                    time,
                    payload,
                    hit.t,
                    hit.uv.data,
                    zluda_float3{hit.normal.x, hit.normal.y, hit.normal.z},
                    (uint8_t GLOBAL_SPACE *)(attr_fn),
                    intersection_payload.attribute_block,
                    bvh->transform_blocks[hit.instanceID],
                    reinterpret_cast<uint64_t>(anyhit_fn)
                );
            } else {
                ZLUDA_RT(set_attr_barycentrics)(intersection_payload.attribute_block, hit.uv);
                if (anyhit_fn != nullptr) {
                    OptixRay transformed_ray {
                        { hit.origin.x, hit.origin.y, hit.origin.z },
                        { hit.direction.x, hit.direction.y, hit.direction.z },
                        ray_type,
                        tmin,
                        tmax
                    };
                    intersection_result = (*anyhit_fn)(
                        global_state,
                        hit.primID,
                        launch_idx,
                        launch_dim,
                        (OptixRay PRIVATE_SPACE*)(&transformed_ray),
                        time,
                        payload,
                        hit.t,
                        hit.uv.data,
                        zluda_float3{hit.normal.x, hit.normal.y, hit.normal.z},
                        (uint8_t GLOBAL_SPACE *)(anyhit_fn),
                        intersection_payload.attribute_block,
                        bvh->transform_blocks[hit.instanceID]
                    );
                }
            }
            if (intersection_result >= 1024) {
                return intersection_result;
            }
            if (intersection_result != 1) {
                closest_attr_fn = attr_fn;
                closest_uv = hit.uv;
                closest_normals = hit.normal;
                closest_instance_id = hit.instanceID;
                closest_subinstance_id = hit.primID;
                intersection_payload.closest_distance = hit.t;
            }
            if (intersection_result == 2)
                break;
        }
        if (closest_instance_id != hiprtInvalidValue) {
            raytracing_fn* closest_hit_fn = get_program_block_outer<raytracing_fn*>(
                byte_offset(programs_chain, programs_chain->closest_hit_start),
                closest_instance_id,
                closest_subinstance_id
            );
            if (closest_hit_fn != nullptr) {
                ((OptixStack*)stack)->stack_pointer() = new_stack_bottom;
                uint32_t result = 0;
                if (closest_attr_fn != nullptr) {
                    result = (*closest_attr_fn)(
                        global_state,
                        closest_subinstance_id,
                        launch_idx,
                        launch_dim,
                        (OptixRay PRIVATE_SPACE*)(&optix_ray),
                        time,
                        payload,
                        intersection_payload.closest_distance,
                        closest_uv.data,
                        zluda_float3{closest_normals.x, closest_normals.y, closest_normals.z},
                        (uint8_t GLOBAL_SPACE *)(closest_attr_fn),
                        intersection_payload.attribute_block,
                        bvh->transform_blocks[closest_instance_id],
                        reinterpret_cast<uint64_t>(closest_hit_fn)
                    );
                } else {
                    result = (*closest_hit_fn)(
                        global_state,
                        closest_subinstance_id,
                        launch_idx,
                        launch_dim,
                        (OptixRay PRIVATE_SPACE*)(&optix_ray),
                        time,
                        payload,
                        intersection_payload.closest_distance,
                        float2(0.0, 0.0).data,
                        zluda_float3{closest_normals.x, closest_normals.y, closest_normals.z},
                        (uint8_t GLOBAL_SPACE *)(closest_hit_fn),
                        intersection_payload.attribute_block,
                        bvh->transform_blocks[closest_instance_id]
                    );
                }
                if (result >= 1024) {
                    return result;
                }
                ((OptixStack*)stack)->stack_pointer() = old_stack_bottom;
            }
            return 0;
        } else {
            return call_raytracing_fn_inner(
                global_state,
                global_space_cast(stack->globals.miss_programs),
                ray_type,
                launch_idx,
                launch_dim,
                (OptixRay PRIVATE_SPACE*)(&optix_ray),
                0.0,
                payload,
                0.0,
                float2(0.0, 0.0).data,
                zluda_float3{0.0, 0.0, 0.0},
                nullptr,
                nullptr
            );
        }
    }

    __global__ void ZLUDA_RT(generate_bounding_box)(
        GlobalState globals,
        bounding_box_fn bb_func,
        uint32_t primitive_count,
        float GLOBAL_SPACE* output,
        uint8_t GLOBAL_SPACE* variable_block
    ) {
        asm ("" : : :  "s105");
        asm ("" : : :  "v255");
        if (threadIdx.x == 0)
            ZLUDA_RT(BB_GLOBALS) = globals;
        __syncthreads();

        uint32_t x = blockIdx.x * blockDim.x + threadIdx.x;
        if (x >= primitive_count)
            return;
        bb_func(x, &output[x*8], (uint8_t SHARED_SPACE*)&ZLUDA_RT(BB_GLOBALS), variable_block);
    }
}
#endif
 