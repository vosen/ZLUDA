// Compile and disassemble:
//   python3 ./cvt.py > cvt.h && /opt/rocm/llvm/bin/clang -std=c++20 -Xclang -no-opaque-pointers -Wall -Wextra -Wsign-compare -Wconversion -x hip zluda_ptx_impl.cpp -S -emit-llvm --cuda-device-only -nogpulib -O3 -Xclang -fallow-half-arguments-and-returns -o - | sed -e 's/define/define linkonce_odr/g' | sed -e '/@llvm.used/d' | sed -e 's/\"target-cpu\"=\"[^\"]*\"//g' | sed -e 's/\"target-features\"=\"[^\"]*\"//g' | sed -e 's/\"denormal-fp-math-f32\"=\"[^\"]*\"//g' | sed -e 's/!llvm.module.flags = !{!0, !1, !2, !3, !4}/!llvm.module.flags = !{ }/g' | sed -e 's/memory(none)/readnone/g' | sed -e 's/memory(argmem: readwrite, inaccessiblemem: readwrite)/inaccessiblemem_or_argmemonly/g' | sed -e 's/memory(read)/readonly/g' | sed -e 's/memory(argmem: readwrite)/argmemonly/g'  | llvm-as-13 -o zluda_ptx_impl.bc && /opt/rocm/llvm/bin/llvm-dis zluda_ptx_impl.bc
// Compile to binary:
//   /opt/rocm/llvm/bin/clang -x ir -target amdgcn-amd-amdhsa -Xlinker --no-undefined zluda_ptx_impl.bc -mno-wavefrontsize64 -mcpu=gfx1030
// Decompile:
//   /opt/rocm/llvm/bin/llvm-objdump -d a.out --triple amdgcn-amd-amdhsa
// List of builtins:
//   https://github.com/llvm/llvm-project/blob/main/clang/include/clang/Basic/BuiltinsAMDGPU.def
//   https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/IR/IntrinsicsAMDGPU.td
// Extra information:
//   https://llvm.org/docs/AMDGPUUsage.html

#include <cstdint>
#include <bit>
#include <hip/hip_runtime.h>
#define HIP_NO_HALF
#include <hip/amd_detail/amd_hip_fp16.h>
#undef HIP_NO_HALF
#include <hip/amd_detail/amd_device_functions.h>
#include <hip/amd_detail/amd_surface_functions.h>

enum class CompilationMode : uint8_t
{
    /*
              warp[0]     warp[1]
           ┌─┬─────┬──┬┬──┬─────┬──┐
      CUDA │0│ ... │31││32│ ... │63│
           └─┴──┬──┴──┴┴──┴──┬──┴──┘
                │            │
                │            │
           ┌─┬──▼──┬──┬┬──┬──▼──┬──┐
    AMDGPU │0│ ... │31││32│ ... │63│
           └─┴─────┴──┴┴──┴─────┴──┘
            wfront[0]    wfront[1]
    */
    Wave32 = 1,
    /*
              warp[0]     warp[1]
           ┌─┬─────┬──┬┬──┬─────┬──┐
      CUDA │0│ ... │31││32│ ... │63│
           └─┴──┬──┴──┴┴──┴──┬──┴──┘
                │            │
                │            └─────┐
                │                  │
           ┌─┬──▼────────┬──┬┬──┬──▼────────┬───┐
    AMDGPU │0│ ... ¦     │63││64│ ... ¦     │127│
           └─┴───────────┴──┴┴──┴───────────┴───┘
               wfront[0]          wfront[1]
    */
    Wave32OnWave64 = 2,
    /*
              warp[0]     warp[1]
           ┌─┬─────┬──┬┬──┬─────┬──┐
      CUDA │0│ ... │31││32│ ... │63│
           └─┴──┬──┴──┴┴──┴──┬──┴──┘
                │            │
                │     ┌──────┘
                │     │
           ┌─┬──▼─────▼──┬──┐
    AMDGPU │0│ ... ¦ ... │63│
           └─┴───────────┴──┘
               wfront[0]
    */
    DoubleWave32OnWave64 = 3
};

#define FUNC(NAME) __attribute__((device)) __attribute__((retain)) __zluda_ptx_impl__##NAME
#define STRUCT(NAME) struct __zluda_ptx_impl__##NAME
#define FUNC_CALL(NAME) __zluda_ptx_impl__##NAME

// The depths of madness of conversions with rounding modes:
// * AMD GPUs do not have HW support for conversions with rounding modes
// * Not a problem, LLVM supports it. Haha, no:
//      * `llvm.experimental.constrained.fptrunc` is exactly the same as `fptrunc`on AMDGPU
//      * `llvm.fptrunc.round` looks fine, but it's LLVM 14+ so avoiding it for now
//      * Nothing for float->int and int->flot
// * Not a problem, HIP supports it. Haha, no. This is HIP implementation right now:
//      __device__ static inline float __double2float_rz(double x) { return (double)x; }
// * Not a problem, ROCm-Device-Libs supports it. Haha, Somewhat:
//      * Only when building for OpenCL (that's why we pass opencl.bc and related explictly). There's no non-deprecated way to convince comgr to link it
//      * To top it all, by default, OpenCL `half` type gets mangled differently than HIP `half` or Clang `_Float16`
//          * There's __fp16, but by default it can't be passed as an argument. Thankfully there's a hidden flag to fix this
#include "cvt.h"

#define GENERIC_SPACE __attribute__((address_space(0)))
#define GLOBAL_SPACE __attribute__((address_space(1)))
#define SHARED_SPACE __attribute__((address_space(3)))
#define CONSTANT_SPACE __attribute__((address_space(4)))
#define PRIVATE_SPACE __attribute__((address_space(5)))

typedef half half4 __attribute__((ext_vector_type(4)));

extern "C" __device__ const CONSTANT_SPACE CompilationMode FUNC_CALL(COMPILATION_MODE);
extern "C" __device__ const CONSTANT_SPACE bool FUNC_CALL(IS_WINDOWS);

template <
    typename T,
    typename std::enable_if<!std::is_scalar<T>::value && sizeof(T) / sizeof(typename T::value_type) == 1 && sizeof(typename T::value_type) <= sizeof(float)>::type * = nullptr>
static __device__ float4::Native_vec_ __pack_to_float4(const T &t)
{
    union result_element
    {
        float float_;
        typename T::value_type value;
    };
    float4::Native_vec_ result;
    result_element x = {0};
    x.value = t.x;
    result.x = x.float_;
    return result;
}

template <
    typename T,
    typename std::enable_if<!std::is_scalar<T>::value && sizeof(T) / sizeof(typename T::value_type) == 2 && sizeof(typename T::value_type) <= sizeof(float)>::type * = nullptr>
static __device__ float4::Native_vec_ __pack_to_float4(const T &t)
{
    union result_element
    {
        float float_;
        typename T::value_type value;
    };
    float4::Native_vec_ result;
    result_element x = {0};
    x.value = t.x;
    result.x = x.float_;
    result_element y = {0};
    y.value = t.y;
    result.y = y.float_;
    return result;
}

template <
    typename T,
    typename std::enable_if<!std::is_scalar<T>::value && sizeof(T) / sizeof(typename T::value_type) == 4 && sizeof(typename T::value_type) <= sizeof(float)>::type * = nullptr>
static __device__ float4::Native_vec_ __pack_to_float4(const T &t)
{
    union result_element
    {
        float float_;
        typename T::value_type value;
    };
    float4::Native_vec_ result;
    result_element x = {0};
    x.value = t.x;
    result.x = x.float_;
    result_element y = {0};
    y.value = t.y;
    result.y = y.float_;
    result_element z = {0};
    z.value = t.z;
    result.z = z.float_;
    result_element w = {0};
    w.value = t.w;
    result.w = w.float_;
    return result;
}

typedef uint32_t uint8 __attribute__((ext_vector_type(8)));
typedef uint32_t zluda_uint3 __attribute__((ext_vector_type(3)));
typedef uint8 CONSTANT_SPACE *surface_ptr;

template <typename To, typename From>
static __device__ To transmute(From f)
{
    if constexpr (sizeof(To) == sizeof(From))
    {
        return std::bit_cast<To>(f);
    }
    else if constexpr (sizeof(To) > sizeof(From))
    {
        union
        {
            To t;
            From f;
        } u = {To{0}};
        u.f = f;
        return u.t;
    }
    else if constexpr (sizeof(To) < sizeof(From))
    {
        union
        {
            From f;
            To t;
        } u = {From{f}};
        return u.t;
    }
    else
    {
        static_assert(sizeof(To) == 0);
    }
}

enum class ImageGeometry
{
    _1D,
    _2D,
    _3D,
    A1D,
    A2D
};

// clang-format off
template <ImageGeometry> struct Coordinates;
template <> struct Coordinates<ImageGeometry::_1D> { using type = uint1::Native_vec_; };
template <> struct Coordinates<ImageGeometry::_2D> { using type = uint2::Native_vec_; };
template <> struct Coordinates<ImageGeometry::_3D> { using type = uint4::Native_vec_; };
template <> struct Coordinates<ImageGeometry::A1D>
{
    using type = uint2::Native_vec_; using arg_type = uint1::Native_vec_;
    static __device__ type pack_layer(uint32_t layer, arg_type coord)
    {
        return type { coord.x, layer };
    }
};
template <> struct Coordinates<ImageGeometry::A2D>
{
    using type = zluda_uint3; using arg_type = uint2::Native_vec_;
    static __device__ type pack_layer(uint32_t layer, arg_type coord)
    {
        return type { coord.x, coord.y, layer };
    }
};
// clang-format on

template <typename T, ImageGeometry geo>
static __device__ void image_store_pck(T value, typename Coordinates<geo>::type coord, surface_ptr surface)
{
    if constexpr (sizeof(T) <= sizeof(uint))
    {
        uint value_dword = transmute<uint>(value);
        if constexpr (geo == ImageGeometry::_1D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x1 dim:1D unorm" : : "v"(value_dword), "v"(coord.x), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_2D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x1 dim:2D unorm" : : "v"(value_dword), "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_3D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x1 dim:3D unorm" : : "v"(value_dword), "v"(transmute<zluda_uint3>(coord)), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A1D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x1 dim:1D_ARRAY unorm" : : "v"(value_dword), "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A2D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x1 dim:2D_ARRAY unorm" : : "v"(value_dword), "v"(coord), "s"(*surface) : "memory");
        }
        else
        {
            static_assert(sizeof(T) == 0, "Invalid geometry");
        }
    }
    else if constexpr (sizeof(T) == sizeof(uint2::Native_vec_))
    {
        uint2::Native_vec_ value_dword2 = transmute<uint2::Native_vec_>(value);
        if constexpr (geo == ImageGeometry::_1D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x3 dim:1D unorm" : : "v"(value_dword2), "v"(coord.x), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_2D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x3 dim:2D unorm" : : "v"(value_dword2), "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_3D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x3 dim:3D unorm" : : "v"(value_dword2), "v"(transmute<zluda_uint3>(coord)), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A1D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x3 dim:1D_ARRAY unorm" : : "v"(value_dword2), "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A2D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0x3 dim:2D_ARRAY unorm" : : "v"(value_dword2), "v"(coord), "s"(*surface) : "memory");
        }
        else
        {
            static_assert(sizeof(T) == 0, "Invalid geometry");
        }
    }
    else if constexpr (sizeof(T) == sizeof(uint4::Native_vec_))
    {
        uint4::Native_vec_ value_dword4 = transmute<uint4::Native_vec_>(value);
        if constexpr (geo == ImageGeometry::_1D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0xf dim:1D unorm" : : "v"(value_dword4), "v"(coord.x), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_2D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0xf dim:2D unorm" : : "v"(value_dword4), "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_3D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0xf dim:3D unorm" : : "v"(value_dword4), "v"(transmute<zluda_uint3>(coord)), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A1D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0xf dim:1D_ARRAY unorm" : : "v"(value_dword4), "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A2D)
        {
            asm volatile("image_store_pck %0, %1, %2 dmask:0xf dim:2D_ARRAY unorm" : : "v"(value_dword4), "v"(coord), "s"(*surface) : "memory");
        }
        else
        {
            static_assert(sizeof(T) == 0, "Invalid geometry");
        }
    }
    else
    {
        static_assert(sizeof(T) == 0, "Invalid vector size");
    }
}

template <typename T, ImageGeometry geo>
static __device__ T image_load_pck(typename Coordinates<geo>::type coord, surface_ptr surface)
{
    if constexpr (sizeof(T) <= sizeof(uint))
    {
        uint data;
        if constexpr (geo == ImageGeometry::_1D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x1 dim:1D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord.x), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_2D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x1 dim:2D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_3D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x1 dim:3D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(transmute<zluda_uint3>(coord)), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A1D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x1 dim:1D_ARRAY unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A2D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x1 dim:2D_ARRAY unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
        }
        else
        {
            static_assert(sizeof(ImageGeometry) == 0, "Invalid geometry");
        }
        return transmute<T>(data);
    }
    else if constexpr (sizeof(T) == sizeof(uint2::Native_vec_))
    {
        uint2::Native_vec_ data;
        if constexpr (geo == ImageGeometry::_1D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x3 dim:1D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord.x), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_2D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x3 dim:2D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_3D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x3 dim:3D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(transmute<zluda_uint3>(coord)), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A1D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x3 dim:1D_ARRAY unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A2D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0x3 dim:2D_ARRAY unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
        }
        else
        {
            static_assert(sizeof(ImageGeometry) == 0, "Invalid geometry");
        }
        return transmute<T>(data);
    }
    else if constexpr (sizeof(T) == sizeof(uint4::Native_vec_))
    {
        uint4::Native_vec_ data;
        if constexpr (geo == ImageGeometry::_1D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:1D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord.x), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_2D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:2D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::_3D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:3D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(transmute<zluda_uint3>(coord)), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A1D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:1D_ARRAY unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
        }
        else if constexpr (geo == ImageGeometry::A2D)
        {
            asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:2D_ARRAY unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
        }
        else
        {
            static_assert(sizeof(ImageGeometry) == 0, "Invalid geometry");
        }
        return transmute<T>(data);
    }
    else
    {
        static_assert(sizeof(T) == 0, "Invalid vector size");
    }
}

template <ImageGeometry geo>
static __device__ uint4::Native_vec_ image_load_pck_full(typename Coordinates<geo>::type coord, surface_ptr surface)
{
    uint4::Native_vec_ data;
    if constexpr (geo == ImageGeometry::_1D)
    {
        asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:1D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord.x), "s"(*surface) : "memory");
    }
    else if constexpr (geo == ImageGeometry::_2D)
    {
        asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:2D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
    }
    else if constexpr (geo == ImageGeometry::_3D)
    {
        asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:3D unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(transmute<zluda_uint3>(coord)), "s"(*surface) : "memory");
    }
    else if constexpr (geo == ImageGeometry::A1D)
    {
        asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:1D_ARRAY unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
    }
    else if constexpr (geo == ImageGeometry::A2D)
    {
        asm volatile("image_load_pck %0, %1, %2 dmask:0xf dim:2D_ARRAY unorm\ns_waitcnt vmcnt(0)" : "=v"(data) : "v"(coord), "s"(*surface) : "memory");
    }
    else
    {
        static_assert(sizeof(ImageGeometry) == 0, "Invalid geometry");
    }
    return data;
}

template <typename T, ImageGeometry geo>
static __device__ void image_store_pck_full_with(uint4::Native_vec_ data, T value, typename Coordinates<geo>::type coord, surface_ptr surface)
{
    // We avoid unions for types smaller than sizeof(uint32_t),
    // because in those cases we get this garbage:
    //     ds_write_b128 v2, v[5:8]
    //     ds_write_b16 v2, v9
    //     ds_read_b128 v[5:8], v2
    // tested with ROCm 5.7.1 on gfx1030
    if constexpr (sizeof(T) == sizeof(uint8_t))
    {
        uint32_t x = uint32_t(std::bit_cast<uint8_t>(value));
        uint32_t data_0 = ((data[0]) >> 8) << 8;
        data[0] = data_0 | x;
    }
    else if constexpr (sizeof(T) == sizeof(uint16_t))
    {
        uint32_t x = uint32_t(std::bit_cast<uint16_t>(value));
        uint32_t data_0 = ((data[0]) >> 16) << 16;
        data[0] = data_0 | x;
    }
    else
    {
        union
        {
            uint4::Native_vec_ full_vec;
            T value;
        } u = {0};
        u.full_vec = data;
        u.value = value;
        data = u.full_vec;
    }
    image_store_pck<uint4::Native_vec_, geo>(data, coord, surface);
}

constexpr auto IMAGE_RESERVED_TOP_BITS = 3;

static __device__ surface_ptr get_surface_pointer(uint64_t s)
{
    return (surface_ptr)((s << IMAGE_RESERVED_TOP_BITS) >> IMAGE_RESERVED_TOP_BITS);
}

static __device__ surface_ptr get_surface_pointer(struct textureReference GLOBAL_SPACE *surf_ref)
{
    return (surface_ptr)(surf_ref->textureObject);
}

static __device__ uint32_t x_coordinate_shift(uint64_t s)
{
    return uint32_t(s >> (64 - IMAGE_RESERVED_TOP_BITS));
}

static __device__ uint32_t x_coordinate_shift(struct textureReference GLOBAL_SPACE *ptr)
{
    uint32_t channels = uint32_t(ptr->numChannels);
    uint32_t format_width = 0;
    hipArray_Format format = ptr->format;
    switch (format)
    {
    case hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8:
    case hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8:
        format_width = 1;
        break;
    case hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16:
    case hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16:
    case hipArray_Format::HIP_AD_FORMAT_HALF:
        format_width = 2;
        break;
    case hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32:
    case hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32:
    case hipArray_Format::HIP_AD_FORMAT_FLOAT:
        format_width = 4;
        break;
    default:
        __builtin_unreachable();
    }
    return uint32_t(__builtin_ctz(format_width * channels));
}

template <typename T, ImageGeometry geo, typename Surface>
static __device__ T suld_b_zero(Surface surf_arg, typename Coordinates<geo>::type coord)
{
    surface_ptr surface = get_surface_pointer(surf_arg);
    uint32_t shift_x = x_coordinate_shift(surf_arg);
    coord.x = coord.x >> shift_x;
    return image_load_pck<T, geo>(coord, surface);
}

template <typename T, ImageGeometry geo, typename Surface>
static __device__ void sust_b_zero(Surface surf_arg, typename Coordinates<geo>::type coord, T data)
{
    surface_ptr surface = get_surface_pointer(surf_arg);
    uint32_t shift_x = x_coordinate_shift(surf_arg);
    coord.x = coord.x >> shift_x;
    if (shift_x <= __builtin_ctz(sizeof(T))) [[likely]]
    {
        image_store_pck<T, geo>(data, coord, surface);
    }
    else
    {
        uint4::Native_vec_ pixel = image_load_pck_full<geo>(coord, surface);
        image_store_pck_full_with<T, geo>(pixel, data, coord, surface);
    }
}

extern "C"
{
#define atomic_inc(NAME, SUCCESS, FAILURE, SCOPE, SPACE)                                                                              \
    uint32_t FUNC(NAME)(SPACE uint32_t * ptr, uint32_t threshold)                                                                     \
    {                                                                                                                                 \
        uint32_t expected = *ptr;                                                                                                     \
        uint32_t desired;                                                                                                             \
        do                                                                                                                            \
        {                                                                                                                             \
            desired = (expected >= threshold) ? 0 : expected + 1;                                                                     \
        } while (!__hip_atomic_compare_exchange_strong((volatile SPACE uint32_t *)ptr, &expected, desired, SUCCESS, FAILURE, SCOPE)); \
        return expected;                                                                                                              \
    }

#define atomic_dec(NAME, SUCCESS, FAILURE, SCOPE, SPACE)                                                                              \
    uint32_t FUNC(NAME)(SPACE uint32_t * ptr, uint32_t threshold)                                                                     \
    {                                                                                                                                 \
        uint32_t expected = *ptr;                                                                                                     \
        uint32_t desired;                                                                                                             \
        do                                                                                                                            \
        {                                                                                                                             \
            desired = (expected == 0 || expected > threshold) ? threshold : expected - 1;                                             \
        } while (!__hip_atomic_compare_exchange_strong((volatile SPACE uint32_t *)ptr, &expected, desired, SUCCESS, FAILURE, SCOPE)); \
        return expected;                                                                                                              \
    }

    // atom.inc
    atomic_inc(atom_relaxed_cta_generic_inc, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_WORKGROUP, GENERIC_SPACE);
    atomic_inc(atom_acquire_cta_generic_inc, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GENERIC_SPACE);
    atomic_inc(atom_release_cta_generic_inc, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GENERIC_SPACE);
    atomic_inc(atom_acq_rel_cta_generic_inc, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GENERIC_SPACE);

    atomic_inc(atom_relaxed_gpu_generic_inc, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_AGENT, GENERIC_SPACE);
    atomic_inc(atom_acquire_gpu_generic_inc, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GENERIC_SPACE);
    atomic_inc(atom_release_gpu_generic_inc, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GENERIC_SPACE);
    atomic_inc(atom_acq_rel_gpu_generic_inc, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GENERIC_SPACE);

    atomic_inc(atom_relaxed_sys_generic_inc, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_SYSTEM, GENERIC_SPACE);
    atomic_inc(atom_acquire_sys_generic_inc, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GENERIC_SPACE);
    atomic_inc(atom_release_sys_generic_inc, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GENERIC_SPACE);
    atomic_inc(atom_acq_rel_sys_generic_inc, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GENERIC_SPACE);

    atomic_inc(atom_relaxed_cta_global_inc, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_WORKGROUP, GLOBAL_SPACE);
    atomic_inc(atom_acquire_cta_global_inc, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GLOBAL_SPACE);
    atomic_inc(atom_release_cta_global_inc, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GLOBAL_SPACE);
    atomic_inc(atom_acq_rel_cta_global_inc, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GLOBAL_SPACE);

    atomic_inc(atom_relaxed_gpu_global_inc, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_AGENT, GLOBAL_SPACE);
    atomic_inc(atom_acquire_gpu_global_inc, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GLOBAL_SPACE);
    atomic_inc(atom_release_gpu_global_inc, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GLOBAL_SPACE);
    atomic_inc(atom_acq_rel_gpu_global_inc, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GLOBAL_SPACE);

    atomic_inc(atom_relaxed_sys_global_inc, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_SYSTEM, GLOBAL_SPACE);
    atomic_inc(atom_acquire_sys_global_inc, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GLOBAL_SPACE);
    atomic_inc(atom_release_sys_global_inc, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GLOBAL_SPACE);
    atomic_inc(atom_acq_rel_sys_global_inc, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GLOBAL_SPACE);

    atomic_inc(atom_relaxed_cta_shared_inc, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_WORKGROUP, SHARED_SPACE);
    atomic_inc(atom_acquire_cta_shared_inc, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, SHARED_SPACE);
    atomic_inc(atom_release_cta_shared_inc, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, SHARED_SPACE);
    atomic_inc(atom_acq_rel_cta_shared_inc, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, SHARED_SPACE);

    atomic_inc(atom_relaxed_gpu_shared_inc, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_AGENT, SHARED_SPACE);
    atomic_inc(atom_acquire_gpu_shared_inc, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, SHARED_SPACE);
    atomic_inc(atom_release_gpu_shared_inc, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, SHARED_SPACE);
    atomic_inc(atom_acq_rel_gpu_shared_inc, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, SHARED_SPACE);

    atomic_inc(atom_relaxed_sys_shared_inc, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_SYSTEM, SHARED_SPACE);
    atomic_inc(atom_acquire_sys_shared_inc, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, SHARED_SPACE);
    atomic_inc(atom_release_sys_shared_inc, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, SHARED_SPACE);
    atomic_inc(atom_acq_rel_sys_shared_inc, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, SHARED_SPACE);

    // atom.dec
    atomic_dec(atom_relaxed_cta_generic_dec, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_WORKGROUP, GENERIC_SPACE);
    atomic_dec(atom_acquire_cta_generic_dec, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GENERIC_SPACE);
    atomic_dec(atom_release_cta_generic_dec, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GENERIC_SPACE);
    atomic_dec(atom_acq_rel_cta_generic_dec, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GENERIC_SPACE);

    atomic_dec(atom_relaxed_gpu_generic_dec, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_AGENT, GENERIC_SPACE);
    atomic_dec(atom_acquire_gpu_generic_dec, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GENERIC_SPACE);
    atomic_dec(atom_release_gpu_generic_dec, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GENERIC_SPACE);
    atomic_dec(atom_acq_rel_gpu_generic_dec, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GENERIC_SPACE);

    atomic_dec(atom_relaxed_sys_generic_dec, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_SYSTEM, GENERIC_SPACE);
    atomic_dec(atom_acquire_sys_generic_dec, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GENERIC_SPACE);
    atomic_dec(atom_release_sys_generic_dec, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GENERIC_SPACE);
    atomic_dec(atom_acq_rel_sys_generic_dec, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GENERIC_SPACE);

    atomic_dec(atom_relaxed_cta_global_dec, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_WORKGROUP, GLOBAL_SPACE);
    atomic_dec(atom_acquire_cta_global_dec, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GLOBAL_SPACE);
    atomic_dec(atom_release_cta_global_dec, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GLOBAL_SPACE);
    atomic_dec(atom_acq_rel_cta_global_dec, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, GLOBAL_SPACE);

    atomic_dec(atom_relaxed_gpu_global_dec, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_AGENT, GLOBAL_SPACE);
    atomic_dec(atom_acquire_gpu_global_dec, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GLOBAL_SPACE);
    atomic_dec(atom_release_gpu_global_dec, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GLOBAL_SPACE);
    atomic_dec(atom_acq_rel_gpu_global_dec, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, GLOBAL_SPACE);

    atomic_dec(atom_relaxed_sys_global_dec, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_SYSTEM, GLOBAL_SPACE);
    atomic_dec(atom_acquire_sys_global_dec, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GLOBAL_SPACE);
    atomic_dec(atom_release_sys_global_dec, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GLOBAL_SPACE);
    atomic_dec(atom_acq_rel_sys_global_dec, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, GLOBAL_SPACE);

    atomic_dec(atom_relaxed_cta_shared_dec, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_WORKGROUP, SHARED_SPACE);
    atomic_dec(atom_acquire_cta_shared_dec, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, SHARED_SPACE);
    atomic_dec(atom_release_cta_shared_dec, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, SHARED_SPACE);
    atomic_dec(atom_acq_rel_cta_shared_dec, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_WORKGROUP, SHARED_SPACE);

    atomic_dec(atom_relaxed_gpu_shared_dec, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_AGENT, SHARED_SPACE);
    atomic_dec(atom_acquire_gpu_shared_dec, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, SHARED_SPACE);
    atomic_dec(atom_release_gpu_shared_dec, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, SHARED_SPACE);
    atomic_dec(atom_acq_rel_gpu_shared_dec, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_AGENT, SHARED_SPACE);

    atomic_dec(atom_relaxed_sys_shared_dec, __ATOMIC_RELAXED, __ATOMIC_RELAXED, __HIP_MEMORY_SCOPE_SYSTEM, SHARED_SPACE);
    atomic_dec(atom_acquire_sys_shared_dec, __ATOMIC_ACQUIRE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, SHARED_SPACE);
    atomic_dec(atom_release_sys_shared_dec, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, SHARED_SPACE);
    atomic_dec(atom_acq_rel_sys_shared_dec, __ATOMIC_ACQ_REL, __ATOMIC_ACQUIRE, __HIP_MEMORY_SCOPE_SYSTEM, SHARED_SPACE);

#define tex_1d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE, TEX_FN)                                                                                 \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_1d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, HIP_COORD_TYPE##1 ::Native_vec_ x) \
    {                                                                                                                                                              \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                                                     \
        return TEX_FN<HIP_CHANNEL_TYPE##4>(textureObject, x.x).data;                                                                                               \
    }                                                                                                                                                              \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_indirect_1d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(uint64_t texobj, HIP_COORD_TYPE##1 ::Native_vec_ x)                   \
    {                                                                                                                                                              \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                                                             \
        return TEX_FN<HIP_CHANNEL_TYPE##4>(textureObject, x.x).data;                                                                                               \
    }

    __device__ half4 __ockl_image_sampleh_1D(unsigned int CONSTANT_SPACE *i, unsigned int CONSTANT_SPACE *s, float c);
    __device__ half4 static inline FUNC_CALL(tex1D_f16)(hipTextureObject_t textureObject, float x)
    {
        TEXTURE_OBJECT_PARAMETERS_INIT;
        return __ockl_image_sampleh_1D(i, s, x);
    }

    __device__ half4 __ockl_image_loadh_1Db(unsigned int CONSTANT_SPACE *i, int c);
    __device__ half4 static inline FUNC_CALL(tex1Dfetch_f16)(hipTextureObject_t textureObject, int x)
    {
        TEXTURE_OBJECT_PARAMETERS_INIT;
        (void)s;
        return __ockl_image_loadh_1Db(i, x);
    }

#define tex_1d_f16(COORD_TYPE, HIP_COORD_TYPE, TEX_FN)                                                                    \
    half4 FUNC(tex_1d_v4_f16_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, HIP_COORD_TYPE##1 ::Native_vec_ x) \
    {                                                                                                                     \
        hipTextureObject_t textureObject = ptr->textureObject;                                                            \
        return FUNC_CALL(TEX_FN)(textureObject, x.x);                                                                     \
    }                                                                                                                     \
    half4 FUNC(tex_indirect_1d_v4_f16_##COORD_TYPE)(uint64_t texobj, HIP_COORD_TYPE##1 ::Native_vec_ x)                   \
    {                                                                                                                     \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                    \
        return FUNC_CALL(TEX_FN)(textureObject, x.x);                                                                     \
    }

    tex_1d(u32, uint, s32, int, tex1Dfetch);
    tex_1d(u32, uint, f32, float, tex1D);
    tex_1d(s32, int, s32, int, tex1Dfetch);
    tex_1d(s32, int, f32, float, tex1D);
    tex_1d(f32, float, s32, int, tex1Dfetch);
    tex_1d(f32, float, f32, float, tex1D);
    tex_1d_f16(s32, int, tex1Dfetch_f16);
    tex_1d_f16(f32, float, tex1D_f16);

#define tex_2d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE)                                                                                             \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_2d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, HIP_COORD_TYPE##2 ::Native_vec_ coord) \
    {                                                                                                                                                                  \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                                                         \
        return tex2D<HIP_CHANNEL_TYPE##4>(textureObject, float(coord.x), float(coord.y)).data;                                                                         \
    }                                                                                                                                                                  \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_indirect_2d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(uint64_t texobj, HIP_COORD_TYPE##2 ::Native_vec_ coord)                   \
    {                                                                                                                                                                  \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                                                                 \
        return tex2D<HIP_CHANNEL_TYPE##4>(textureObject, float(coord.x), float(coord.y)).data;                                                                         \
    }

    __device__ half4 __ockl_image_sampleh_2D(unsigned int CONSTANT_SPACE *i, unsigned int ADDRESS_SPACE_CONSTANT *s, float2::Native_vec_ c);
#define tex_2d_f16(COORD_TYPE, HIP_COORD_TYPE)                                                                                \
    half4 FUNC(tex_2d_v4_f16_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, HIP_COORD_TYPE##2 ::Native_vec_ coord) \
    {                                                                                                                         \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                       \
        return __ockl_image_sampleh_2D(i, s, (float2){float(coord.x), float(coord.y)}.data);                                  \
    }                                                                                                                         \
    half4 FUNC(tex_indirect_2d_v4_f16_##COORD_TYPE)(uint64_t texobj, HIP_COORD_TYPE##2 ::Native_vec_ coord)                   \
    {                                                                                                                         \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                        \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                       \
        return __ockl_image_sampleh_2D(i, s, (float2){float(coord.x), float(coord.y)}.data);                                  \
    }

    tex_2d(u32, uint, s32, int);
    tex_2d(u32, uint, f32, float);
    tex_2d(s32, int, s32, int);
    tex_2d(s32, int, f32, float);
    tex_2d(f32, float, s32, int);
    tex_2d(f32, float, f32, float);
    tex_2d_f16(s32, int);
    tex_2d_f16(f32, float);

#define tex_3d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE)                                                                                             \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_3d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, HIP_COORD_TYPE##4 ::Native_vec_ coord) \
    {                                                                                                                                                                  \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                                                         \
        return tex3D<HIP_CHANNEL_TYPE##4>(textureObject, float(coord.x), float(coord.y), float(coord.z)).data;                                                         \
    }                                                                                                                                                                  \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_indirect_3d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(uint64_t texobj, HIP_COORD_TYPE##4 ::Native_vec_ coord)                   \
    {                                                                                                                                                                  \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                                                                 \
        return tex3D<HIP_CHANNEL_TYPE##4>(textureObject, float(coord.x), float(coord.y), float(coord.z)).data;                                                         \
    }

    __device__ half4 __ockl_image_sampleh_3D(unsigned int CONSTANT_SPACE *i, unsigned int ADDRESS_SPACE_CONSTANT *s, float4::Native_vec_ c);
#define tex_3d_f16(COORD_TYPE, HIP_COORD_TYPE)                                                                                \
    half4 FUNC(tex_3d_v4_f16_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, HIP_COORD_TYPE##4 ::Native_vec_ coord) \
    {                                                                                                                         \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                       \
        return __ockl_image_sampleh_3D(i, s, (float4){float(coord.x), float(coord.y), float(coord.z), float(coord.w)}.data);  \
    }                                                                                                                         \
    half4 FUNC(tex_indirect_3d_v4_f16_##COORD_TYPE)(uint64_t texobj, HIP_COORD_TYPE##4 ::Native_vec_ coord)                   \
    {                                                                                                                         \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                        \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                       \
        return __ockl_image_sampleh_3D(i, s, (float4){float(coord.x), float(coord.y), float(coord.z), float(coord.w)}.data);  \
    }

    tex_3d(u32, uint, s32, int);
    tex_3d(u32, uint, f32, float);
    tex_3d(s32, int, s32, int);
    tex_3d(s32, int, f32, float);
    tex_3d(f32, float, s32, int);
    tex_3d(f32, float, f32, float);
    tex_3d_f16(s32, int);
    tex_3d_f16(f32, float);

#define tex_a1d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE)                                                                                        \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_a1d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, uint32_t layer, HIP_COORD_TYPE x) \
    {                                                                                                                                                              \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                                                     \
        return tex1DLayered<HIP_CHANNEL_TYPE##4>(textureObject, float(x), int(layer)).data;                                                                        \
    }                                                                                                                                                              \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_indirect_a1d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(uint64_t texobj, uint32_t layer, HIP_COORD_TYPE x)                   \
    {                                                                                                                                                              \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                                                             \
        return tex1DLayered<HIP_CHANNEL_TYPE##4>(textureObject, float(x), int(layer)).data;                                                                        \
    }

    __device__ half4 __ockl_image_sampleh_1Da(unsigned int CONSTANT_SPACE *i, unsigned int ADDRESS_SPACE_CONSTANT *s, float2::Native_vec_ c);
#define tex_a1d_f16(COORD_TYPE, HIP_COORD_TYPE)                                                                           \
    half4 FUNC(tex_a1d_v4_f16_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, uint32_t layer, HIP_COORD_TYPE x) \
    {                                                                                                                     \
        hipTextureObject_t textureObject = ptr->textureObject;                                                            \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                   \
        return __ockl_image_sampleh_1Da(i, s, (float2){float(x), float(layer)}.data);                                     \
    }                                                                                                                     \
    half4 FUNC(tex_indirect_a1d_v4_f16_##COORD_TYPE)(uint64_t texobj, uint32_t layer, HIP_COORD_TYPE x)                   \
    {                                                                                                                     \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                    \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                   \
        return __ockl_image_sampleh_1Da(i, s, (float2){float(x), float(layer)}.data);                                     \
    }

    tex_a1d(u32, uint, s32, int);
    tex_a1d(u32, uint, f32, float);
    tex_a1d(s32, int, s32, int);
    tex_a1d(s32, int, f32, float);
    tex_a1d(f32, float, s32, int);
    tex_a1d(f32, float, f32, float);
    tex_a1d_f16(s32, int);
    tex_a1d_f16(f32, float);

#define tex_a2d(CHANNEL_TYPE, HIP_CHANNEL_TYPE, COORD_TYPE, HIP_COORD_TYPE)                                                                                                          \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_a2d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, uint32_t layer, HIP_COORD_TYPE x, HIP_COORD_TYPE y) \
    {                                                                                                                                                                                \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                                                                       \
        return tex2DLayered<HIP_CHANNEL_TYPE##4>(textureObject, float(x), float(y), int(layer)).data;                                                                                \
    }                                                                                                                                                                                \
    HIP_CHANNEL_TYPE##4 ::Native_vec_ FUNC(tex_indirect_a2d_v4_##CHANNEL_TYPE##_##COORD_TYPE)(uint64_t texobj, uint32_t layer, HIP_COORD_TYPE x, HIP_COORD_TYPE y)                   \
    {                                                                                                                                                                                \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                                                                               \
        return tex2DLayered<HIP_CHANNEL_TYPE##4>(textureObject, float(x), float(y), int(layer)).data;                                                                                \
    }

    __device__ half4 __ockl_image_sampleh_2Da(unsigned int CONSTANT_SPACE *i, unsigned int ADDRESS_SPACE_CONSTANT *s, float4::Native_vec_ c);
#define tex_a2d_f16(COORD_TYPE, HIP_COORD_TYPE)                                                                                             \
    half4 FUNC(tex_a2d_v4_f16_##COORD_TYPE)(struct textureReference GLOBAL_SPACE * ptr, uint32_t layer, HIP_COORD_TYPE x, HIP_COORD_TYPE y) \
    {                                                                                                                                       \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                              \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                                     \
        return __ockl_image_sampleh_2Da(i, s, (float4){float(x), float(y), float(layer), 0.0}.data);                                        \
    }                                                                                                                                       \
    half4 FUNC(tex_indirect_a2d_v4_f16_##COORD_TYPE)(uint64_t texobj, uint32_t layer, HIP_COORD_TYPE x, HIP_COORD_TYPE y)                   \
    {                                                                                                                                       \
        hipTextureObject_t textureObject = (hipTextureObject_t)texobj;                                                                      \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                                     \
        return __ockl_image_sampleh_2Da(i, s, (float4){float(x), float(y), float(layer), 0.0}.data);                                        \
    }

    tex_a2d(u32, uint, s32, int);
    tex_a2d(u32, uint, f32, float);
    tex_a2d(s32, int, s32, int);
    tex_a2d(s32, int, f32, float);
    tex_a2d(f32, float, s32, int);
    tex_a2d(f32, float, f32, float);
    tex_a2d_f16(s32, int);
    tex_a2d_f16(f32, float);

#define suld_b_1d_vec(VEC, TYPE, HIP_TYPE)                                                                                      \
    HIP_TYPE::Native_vec_ FUNC(suld_b_1d##VEC##_##TYPE##_trap)(struct textureReference GLOBAL_SPACE * ptr, int1::Native_vec_ x) \
    {                                                                                                                           \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                  \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                         \
        (void)s;                                                                                                                \
        int byte_coord = __hipGetPixelAddr(x.x, __ockl_image_channel_data_type_1D(i), __ockl_image_channel_order_1D(i));        \
        return __hipMapFrom<HIP_TYPE>(__ockl_image_load_1D(i, byte_coord)).data;                                                \
    }                                                                                                                           \
                                                                                                                                \
    HIP_TYPE::Native_vec_ FUNC(suld_b_indirect_1d##VEC##_##TYPE##_trap)(uint64_t serf_arg, int1::Native_vec_ x)                 \
    {                                                                                                                           \
        hipSurfaceObject_t surfObj = (hipSurfaceObject_t)serf_arg;                                                              \
        HIP_TYPE result;                                                                                                        \
        surf1Dread(&result, surfObj, x.x, hipBoundaryModeTrap);                                                                 \
        return result.data;                                                                                                     \
    }

    suld_b_1d_vec(, b8, uchar1);
    suld_b_1d_vec(, b16, ushort1);
    suld_b_1d_vec(, b32, uint1);
    // suld_b_1d_vec(, b64, ulong1);
    suld_b_1d_vec(_v2, b8, uchar2);
    suld_b_1d_vec(_v2, b16, ushort2);
    suld_b_1d_vec(_v2, b32, uint2);
    // suld_b_1d_vec(_v2, b64, ulong2);
    suld_b_1d_vec(_v4, b8, uchar4);
    suld_b_1d_vec(_v4, b16, ushort4);
    suld_b_1d_vec(_v4, b32, uint4);
    // suld_b_1d_vec(_v4, b64, ulong4);

#define suld_b_2d_vec(VEC, TYPE, HIP_TYPE)                                                                                          \
    HIP_TYPE::Native_vec_ FUNC(suld_b_2d##VEC##_##TYPE##_trap)(struct textureReference GLOBAL_SPACE * ptr, int2::Native_vec_ coord) \
    {                                                                                                                               \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                      \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                             \
        (void)s;                                                                                                                    \
        int byte_coord = __hipGetPixelAddr(coord.x, __ockl_image_channel_data_type_2D(i), __ockl_image_channel_order_2D(i));        \
        return __hipMapFrom<HIP_TYPE>(__ockl_image_load_2D(i, int2(byte_coord, coord.y).data)).data;                                \
    }                                                                                                                               \
                                                                                                                                    \
    HIP_TYPE::Native_vec_ FUNC(suld_b_indirect_2d##VEC##_##TYPE##_trap)(uint64_t serf_arg, int2::Native_vec_ x)                     \
    {                                                                                                                               \
        hipSurfaceObject_t surfObj = (hipSurfaceObject_t)serf_arg;                                                                  \
        HIP_TYPE result;                                                                                                            \
        surf2Dread(&result, surfObj, x.x, x.y);                                                                                     \
        return result.data;                                                                                                         \
    }

    suld_b_2d_vec(, b8, uchar1);
    suld_b_2d_vec(, b16, ushort1);
    suld_b_2d_vec(, b32, uint1);
    // suld_b_2d_vec(, b64, ulong1);
    suld_b_2d_vec(_v2, b8, uchar2);
    suld_b_2d_vec(_v2, b16, ushort2);
    suld_b_2d_vec(_v2, b32, uint2);
    // suld_b_2d_vec(_v2, b64, ulong2);
    suld_b_2d_vec(_v4, b8, uchar4);
    suld_b_2d_vec(_v4, b16, ushort4);
    suld_b_2d_vec(_v4, b32, uint4);
    // suld_b_2d_vec(_v4, b64, ulong4);

#define suld_b_3d_vec(VEC, TYPE, HIP_TYPE)                                                                                          \
    HIP_TYPE::Native_vec_ FUNC(suld_b_3d##VEC##_##TYPE##_trap)(struct textureReference GLOBAL_SPACE * ptr, int4::Native_vec_ coord) \
    {                                                                                                                               \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                      \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                             \
        (void)s;                                                                                                                    \
        int byte_coord = __hipGetPixelAddr(coord.x, __ockl_image_channel_data_type_3D(i), __ockl_image_channel_order_3D(i));        \
        return __hipMapFrom<HIP_TYPE>(__ockl_image_load_3D(i, int4(byte_coord, coord.y, coord.z, 0).data)).data;                    \
    }                                                                                                                               \
                                                                                                                                    \
    HIP_TYPE::Native_vec_ FUNC(suld_b_indirect_3d##VEC##_##TYPE##_trap)(uint64_t serf_arg, int4::Native_vec_ x)                     \
    {                                                                                                                               \
        hipSurfaceObject_t surfObj = (hipSurfaceObject_t)serf_arg;                                                                  \
        HIP_TYPE result;                                                                                                            \
        surf3Dread(&result, surfObj, x.x, x.y, x.z);                                                                                \
        return result.data;                                                                                                         \
    }

    suld_b_3d_vec(, b8, uchar1);
    suld_b_3d_vec(, b16, ushort1);
    suld_b_3d_vec(, b32, uint1);
    // suld_b_3d_vec(, b64, ulong1);
    suld_b_3d_vec(_v2, b8, uchar2);
    suld_b_3d_vec(_v2, b16, ushort2);
    suld_b_3d_vec(_v2, b32, uint2);
    // suld_b_3d_vec(_v2, b64, ulong2);
    suld_b_3d_vec(_v4, b8, uchar4);
    suld_b_3d_vec(_v4, b16, ushort4);
    suld_b_3d_vec(_v4, b32, uint4);
    // suld_b_3d_vec(_v4, b64, ulong4);

#define suld_b_a1d_vec(VEC, TYPE, HIP_TYPE)                                                                                    \
    HIP_TYPE::Native_vec_ FUNC(suld_b_a1d##VEC##_##TYPE##_trap)(struct textureReference GLOBAL_SPACE * ptr, uint layer, int x) \
    {                                                                                                                          \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                 \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                        \
        (void)s;                                                                                                               \
        int byte_coord = __hipGetPixelAddr(x, __ockl_image_channel_data_type_1Da(i), __ockl_image_channel_order_1Da(i));       \
        return __hipMapFrom<HIP_TYPE>(__ockl_image_load_1Da(i, int2(byte_coord, int(layer)).data)).data;                       \
    }                                                                                                                          \
                                                                                                                               \
    HIP_TYPE::Native_vec_ FUNC(suld_b_indirect_a1d##VEC##_##TYPE##_trap)(uint64_t serf_arg, uint layer, int x)                 \
    {                                                                                                                          \
        hipSurfaceObject_t surfObj = (hipSurfaceObject_t)serf_arg;                                                             \
        __HIP_SURFACE_OBJECT_PARAMETERS_INIT                                                                                   \
        int byte_coord = __hipGetPixelAddr(x, __ockl_image_channel_data_type_1Da(i), __ockl_image_channel_order_1Da(i));       \
        return __hipMapFrom<HIP_TYPE>(__ockl_image_load_1Da(i, int2(byte_coord, int(layer)).data)).data;                       \
    }

    suld_b_a1d_vec(, b8, uchar1);
    suld_b_a1d_vec(, b16, ushort1);
    suld_b_a1d_vec(, b32, uint1);
    // suld_b_a1d_vec(, b64, ulong1);
    suld_b_a1d_vec(_v2, b8, uchar2);
    suld_b_a1d_vec(_v2, b16, ushort2);
    suld_b_a1d_vec(_v2, b32, uint2);
    // suld_b_a1d_vec(_v2, b64, ulong2);
    suld_b_a1d_vec(_v4, b8, uchar4);
    suld_b_a1d_vec(_v4, b16, ushort4);
    suld_b_a1d_vec(_v4, b32, uint4);
    // suld_b_a1d_vec(_v4, b64, ulong4);

#define suld_b_a2d_vec(VEC, TYPE, HIP_TYPE)                                                                                           \
    HIP_TYPE::Native_vec_ FUNC(suld_b_a2d##VEC##_##TYPE##_trap)(struct textureReference GLOBAL_SPACE * ptr, uint layer, int x, int y) \
    {                                                                                                                                 \
        hipTextureObject_t textureObject = ptr->textureObject;                                                                        \
        TEXTURE_OBJECT_PARAMETERS_INIT;                                                                                               \
        (void)s;                                                                                                                      \
        int byte_coord = __hipGetPixelAddr(x, __ockl_image_channel_data_type_2Da(i), __ockl_image_channel_order_2Da(i));              \
        return __hipMapFrom<HIP_TYPE>(__ockl_image_load_2Da(i, int4(byte_coord, y, int(layer), 0).data)).data;                        \
    }                                                                                                                                 \
                                                                                                                                      \
    HIP_TYPE::Native_vec_ FUNC(suld_b_indirect_a2d##VEC##_##TYPE##_trap)(uint64_t serf_arg, uint layer, int x, int y)                 \
    {                                                                                                                                 \
        hipSurfaceObject_t surfObj = (hipSurfaceObject_t)serf_arg;                                                                    \
        __HIP_SURFACE_OBJECT_PARAMETERS_INIT                                                                                          \
        int byte_coord = __hipGetPixelAddr(x, __ockl_image_channel_data_type_2Da(i), __ockl_image_channel_order_2Da(i));              \
        return __hipMapFrom<HIP_TYPE>(__ockl_image_load_2Da(i, int4(byte_coord, y, int(layer), 0).data)).data;                        \
    }

    suld_b_a2d_vec(, b8, uchar1);
    suld_b_a2d_vec(, b16, ushort1);
    suld_b_a2d_vec(, b32, uint1);
    // suld_b_a2d_vec(, b64, ulong1);
    suld_b_a2d_vec(_v2, b8, uchar2);
    suld_b_a2d_vec(_v2, b16, ushort2);
    suld_b_a2d_vec(_v2, b32, uint2);
    // suld_b_a2d_vec(_v2, b64, ulong2);
    suld_b_a2d_vec(_v4, b8, uchar4);
    suld_b_a2d_vec(_v4, b16, ushort4);
    suld_b_a2d_vec(_v4, b32, uint4);
    // suld_b_a2d_vec(_v4, b64, ulong4);

#define SUST_B_ZERO(TYPE, GEOMETRY, HIP_TYPE)                                                                                                           \
    HIP_TYPE::Native_vec_ FUNC(suld_b_indirect_##TYPE##_zero)(uint64_t surf_arg, typename Coordinates<GEOMETRY>::type coord)                            \
    {                                                                                                                                                   \
        return suld_b_zero<HIP_TYPE::Native_vec_, GEOMETRY>(surf_arg, coord);                                                                           \
    }                                                                                                                                                   \
    void FUNC(sust_b_indirect_##TYPE##_zero)(uint64_t surf_arg, typename Coordinates<GEOMETRY>::type coord, HIP_TYPE::Native_vec_ data)                 \
    {                                                                                                                                                   \
        sust_b_zero<HIP_TYPE::Native_vec_, GEOMETRY>(surf_arg, coord, data);                                                                            \
    }                                                                                                                                                   \
    HIP_TYPE::Native_vec_ FUNC(suld_b_##TYPE##_zero)(struct textureReference GLOBAL_SPACE * ptr, typename Coordinates<GEOMETRY>::type coord)            \
    {                                                                                                                                                   \
        return suld_b_zero<HIP_TYPE::Native_vec_, GEOMETRY>(ptr, coord);                                                                                \
    }                                                                                                                                                   \
    void FUNC(sust_b_##TYPE##_zero)(struct textureReference GLOBAL_SPACE * ptr, typename Coordinates<GEOMETRY>::type coord, HIP_TYPE::Native_vec_ data) \
    {                                                                                                                                                   \
        sust_b_zero<HIP_TYPE::Native_vec_, GEOMETRY>(ptr, coord, data);                                                                                 \
    }

#define SUST_B_ZERO_ARRAY(TYPE, GEOMETRY, HIP_TYPE)                                                                                                                         \
    HIP_TYPE::Native_vec_ FUNC(suld_b_indirect_##TYPE##_zero)(uint64_t surf_arg, uint32_t layer, typename Coordinates<GEOMETRY>::arg_type coord)                            \
    {                                                                                                                                                                       \
        auto coord_array = Coordinates<GEOMETRY>::pack_layer(layer, coord);                                                                                                 \
        return suld_b_zero<HIP_TYPE::Native_vec_, GEOMETRY>(surf_arg, coord_array);                                                                                         \
    }                                                                                                                                                                       \
    void FUNC(sust_b_indirect_##TYPE##_zero)(uint64_t surf_arg, uint32_t layer, typename Coordinates<GEOMETRY>::arg_type coord, HIP_TYPE::Native_vec_ data)                 \
    {                                                                                                                                                                       \
        auto coord_array = Coordinates<GEOMETRY>::pack_layer(layer, coord);                                                                                                 \
        sust_b_zero<HIP_TYPE::Native_vec_, GEOMETRY>(surf_arg, coord_array, data);                                                                                          \
    }                                                                                                                                                                       \
    HIP_TYPE::Native_vec_ FUNC(suld_b_##TYPE##_zero)(struct textureReference GLOBAL_SPACE * ptr, uint32_t layer, typename Coordinates<GEOMETRY>::arg_type coord)            \
    {                                                                                                                                                                       \
        auto coord_array = Coordinates<GEOMETRY>::pack_layer(layer, coord);                                                                                                 \
        return suld_b_zero<HIP_TYPE::Native_vec_, GEOMETRY>(ptr, coord_array);                                                                                              \
    }                                                                                                                                                                       \
    void FUNC(sust_b_##TYPE##_zero)(struct textureReference GLOBAL_SPACE * ptr, uint32_t layer, typename Coordinates<GEOMETRY>::arg_type coord, HIP_TYPE::Native_vec_ data) \
    {                                                                                                                                                                       \
        auto coord_array = Coordinates<GEOMETRY>::pack_layer(layer, coord);                                                                                                 \
        sust_b_zero<HIP_TYPE::Native_vec_, GEOMETRY>(ptr, coord_array, data);                                                                                               \
    }

    SUST_B_ZERO(1d_b8, ImageGeometry::_1D, uchar1);
    SUST_B_ZERO(1d_b16, ImageGeometry::_1D, ushort1);
    SUST_B_ZERO(1d_b32, ImageGeometry::_1D, uint1);
    SUST_B_ZERO(1d_b64, ImageGeometry::_1D, ulong1);
    SUST_B_ZERO(1d_v2_b8, ImageGeometry::_1D, uchar2);
    SUST_B_ZERO(1d_v2_b16, ImageGeometry::_1D, ushort2);
    SUST_B_ZERO(1d_v2_b32, ImageGeometry::_1D, uint2);
    SUST_B_ZERO(1d_v2_b64, ImageGeometry::_1D, ulong2);
    SUST_B_ZERO(1d_v4_b8, ImageGeometry::_1D, uchar4);
    SUST_B_ZERO(1d_v4_b16, ImageGeometry::_1D, ushort4);
    SUST_B_ZERO(1d_v4_b32, ImageGeometry::_1D, uint4);
    SUST_B_ZERO(2d_b8, ImageGeometry::_2D, uchar1);
    SUST_B_ZERO(2d_b16, ImageGeometry::_2D, ushort1);
    SUST_B_ZERO(2d_b32, ImageGeometry::_2D, uint1);
    SUST_B_ZERO(2d_b64, ImageGeometry::_2D, ulong1);
    SUST_B_ZERO(2d_v2_b8, ImageGeometry::_2D, uchar2);
    SUST_B_ZERO(2d_v2_b16, ImageGeometry::_2D, ushort2);
    SUST_B_ZERO(2d_v2_b32, ImageGeometry::_2D, uint2);
    SUST_B_ZERO(2d_v2_b64, ImageGeometry::_2D, ulong2);
    SUST_B_ZERO(2d_v4_b8, ImageGeometry::_2D, uchar4);
    SUST_B_ZERO(2d_v4_b16, ImageGeometry::_2D, ushort4);
    SUST_B_ZERO(2d_v4_b32, ImageGeometry::_2D, uint4);
    SUST_B_ZERO(3d_b8, ImageGeometry::_3D, uchar1);
    SUST_B_ZERO(3d_b16, ImageGeometry::_3D, ushort1);
    SUST_B_ZERO(3d_b32, ImageGeometry::_3D, uint1);
    SUST_B_ZERO(3d_b64, ImageGeometry::_3D, ulong1);
    SUST_B_ZERO(3d_v2_b8, ImageGeometry::_3D, uchar2);
    SUST_B_ZERO(3d_v2_b16, ImageGeometry::_3D, ushort2);
    SUST_B_ZERO(3d_v2_b32, ImageGeometry::_3D, uint2);
    SUST_B_ZERO(3d_v2_b64, ImageGeometry::_3D, ulong2);
    SUST_B_ZERO(3d_v4_b8, ImageGeometry::_3D, uchar4);
    SUST_B_ZERO(3d_v4_b16, ImageGeometry::_3D, ushort4);
    SUST_B_ZERO(3d_v4_b32, ImageGeometry::_3D, uint4);
    SUST_B_ZERO_ARRAY(a1d_b8, ImageGeometry::A1D, uchar1);
    SUST_B_ZERO_ARRAY(a1d_b16, ImageGeometry::A1D, ushort1);
    SUST_B_ZERO_ARRAY(a1d_b32, ImageGeometry::A1D, uint1);
    SUST_B_ZERO_ARRAY(a1d_b64, ImageGeometry::A1D, ulong1);
    SUST_B_ZERO_ARRAY(a1d_v2_b8, ImageGeometry::A1D, uchar2);
    SUST_B_ZERO_ARRAY(a1d_v2_b16, ImageGeometry::A1D, ushort2);
    SUST_B_ZERO_ARRAY(a1d_v2_b32, ImageGeometry::A1D, uint2);
    SUST_B_ZERO_ARRAY(a1d_v2_b64, ImageGeometry::A1D, ulong2);
    SUST_B_ZERO_ARRAY(a1d_v4_b8, ImageGeometry::A1D, uchar4);
    SUST_B_ZERO_ARRAY(a1d_v4_b16, ImageGeometry::A1D, ushort4);
    SUST_B_ZERO_ARRAY(a1d_v4_b32, ImageGeometry::A1D, uint4);
    SUST_B_ZERO_ARRAY(a2d_b8, ImageGeometry::A2D, uchar1);
    SUST_B_ZERO_ARRAY(a2d_b16, ImageGeometry::A2D, ushort1);
    SUST_B_ZERO_ARRAY(a2d_b32, ImageGeometry::A2D, uint1);
    SUST_B_ZERO_ARRAY(a2d_b64, ImageGeometry::A2D, ulong1);
    SUST_B_ZERO_ARRAY(a2d_v2_b8, ImageGeometry::A2D, uchar2);
    SUST_B_ZERO_ARRAY(a2d_v2_b16, ImageGeometry::A2D, ushort2);
    SUST_B_ZERO_ARRAY(a2d_v2_b32, ImageGeometry::A2D, uint2);
    SUST_B_ZERO_ARRAY(a2d_v2_b64, ImageGeometry::A2D, ulong2);
    SUST_B_ZERO_ARRAY(a2d_v4_b8, ImageGeometry::A2D, uchar4);
    SUST_B_ZERO_ARRAY(a2d_v4_b16, ImageGeometry::A2D, ushort4);
    SUST_B_ZERO_ARRAY(a2d_v4_b32, ImageGeometry::A2D, uint4);

    __device__ static inline bool is_upper_warp()
    {
        return FUNC_CALL(COMPILATION_MODE) == CompilationMode::DoubleWave32OnWave64 && __lane_id() >= 32;
    }

    uint32_t FUNC(sreg_laneid)()
    {
        switch (FUNC_CALL(COMPILATION_MODE))
        {
        case CompilationMode::Wave32:
        {
            return __builtin_amdgcn_mbcnt_lo(~0U, 0);
        }
        case CompilationMode::Wave32OnWave64:
        {
            uint32_t lane_id = __lane_id();
            return lane_id;
        }
        case CompilationMode::DoubleWave32OnWave64:
        {
            uint32_t lane_id = __lane_id();
            return lane_id & 31U;
        }
        default:
        {
            __builtin_unreachable();
            return 0;
        }
        }
    }

#define shfl(NAME, EXPR)                                                                                       \
    uint32_t FUNC(shfl_##NAME##_b32_slow)(uint32_t a, uint32_t b, uint32_t c)                                  \
    {                                                                                                          \
        __builtin_amdgcn_wave_barrier();                                                                       \
        int32_t lane = (int32_t)FUNC_CALL(sreg_laneid());                                                      \
        int32_t bval = b & 31U;                                                                                \
        int32_t cval = c & 31U;                                                                                \
        int32_t mask = (c >> 8) & 31U;                                                                         \
        int32_t max_lane = (lane & mask) | (cval & ~mask);                                                     \
        int32_t min_lane __attribute__((unused)) = (lane & mask);                                              \
        int32_t j, pval;                                                                                       \
        EXPR;                                                                                                  \
        if (!pval)                                                                                             \
            j = lane;                                                                                          \
        if (is_upper_warp())                                                                                   \
            j += 32;                                                                                           \
        int32_t shfl_width = (FUNC_CALL(COMPILATION_MODE) == CompilationMode::DoubleWave32OnWave64) ? 64 : 32; \
        return __shfl(a, j, shfl_width);                                                                       \
    }                                                                                                          \
                                                                                                               \
    uint2::Native_vec_ FUNC(shfl_##NAME##_b32_pred_slow)(uint32_t a, uint32_t b, uint32_t c)                   \
    {                                                                                                          \
        __builtin_amdgcn_wave_barrier();                                                                       \
        int32_t lane = (int32_t)FUNC_CALL(sreg_laneid());                                                      \
        int32_t bval = b & 31U;                                                                                \
        int32_t cval = c & 31U;                                                                                \
        int32_t mask = (c >> 8) & 31U;                                                                         \
        int32_t max_lane = (lane & mask) | (cval & ~mask);                                                     \
        int32_t min_lane __attribute__((unused)) = (lane & mask);                                              \
        int32_t j, pval;                                                                                       \
        EXPR;                                                                                                  \
        if (!pval)                                                                                             \
            j = lane;                                                                                          \
        if (is_upper_warp())                                                                                   \
            j += 32;                                                                                           \
        int32_t shfl_width = (FUNC_CALL(COMPILATION_MODE) == CompilationMode::DoubleWave32OnWave64) ? 64 : 32; \
        return uint2(__shfl(a, j, shfl_width), pval).data;                                                     \
    }

    shfl(up, j = lane - bval; pval = (j >= max_lane));
    shfl(down, j = lane + bval; pval = (j <= max_lane));
    shfl(bfly, j = lane ^ bval; pval = (j <= max_lane));
    shfl(idx, j = min_lane | (bval & ~mask); pval = (j <= max_lane));

    void FUNC(__assertfail)(uint64_t message,
                            uint64_t file,
                            uint32_t line,
                            uint64_t function,
                            __attribute__((unused)) uint64_t char_size)
    {
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wignored-attributes"
        [[clang::always_inline]] __assert_fail((const char *)message, (const char *)file, line, (const char *)function);
#pragma clang diagnostic pop
    }

    uint64_t FUNC(malloc)(uint64_t size)
    {
        return reinterpret_cast<uint64_t>(malloc(size));
    }

    void FUNC(free)(uint64_t ptr)
    {
        return free(reinterpret_cast<void *>(ptr));
    }

    __device__ static inline uint32_t match_any_sync_b32_32(int32_t a, uint32_t membermask)
    {
        int32_t result = 0;
        for (int i = 0; i < 32; i++)
        {
            int32_t current = __builtin_amdgcn_readlane(a, i);
            result |= ((a == current) << i);
        }
        return ((uint32_t)result) & membermask;
    }

    __device__ static inline uint32_t match_any_sync_b32_double32(int32_t a, uint32_t membermask)
    {
        int32_t result = 0;
        int start = is_upper_warp() ? 32 : 0;
        for (int i = start; i < start + 32; i++)
        {
            int32_t current = __shfl(a, i, 64);
            result |= ((a == current) << (i % 32));
        }
        return ((uint32_t)result) & membermask;
    }

    uint32_t FUNC(match_any_sync_b32)(int32_t a, uint32_t membermask)
    {
        if (FUNC_CALL(COMPILATION_MODE) == CompilationMode::DoubleWave32OnWave64)
            return match_any_sync_b32_double32(a, membermask);
        else
            return match_any_sync_b32_32(a, membermask);
    }

    // https://cplusplus.com/reference/cstdio/printf/
    __device__ static uint8_t parse_printf_length(const char *s, uint8_t &len)
    {
        switch (*s)
        {
        case 'h':
            switch (s[1])
            {
            case 'd':
            case 'i':
            case 'u':
            case 'o':
            case 'x':
            case 'X':
                len = 2;
                break;
            case 'n':
                len = 8;
                break;
            default:
                return 0;
            }
            return 2;
        case 'l':
            switch (s[1])
            {
            case 'd':
            case 'i':
            case 'u':
            case 'o':
            case 'x':
            case 'X':
                len = FUNC_CALL(IS_WINDOWS) ? 4 : 8;
                break;
            case 'c':
                len = FUNC_CALL(IS_WINDOWS) ? 2 : 4;
                break;
            case 's':
            case 'n':
                len = 8;
                break;
            case 'l':
                switch (s[2])
                {
                case 'd':
                case 'i':
                case 'u':
                case 'o':
                case 'x':
                case 'X':
                case 'n':
                    len = 8;
                    break;
                default:
                    return 0;
                }
                return 3;
            default:
                return 0;
            }
            return 2;
        default:
            return 0;
        }
    }

    __device__ static bool parse_printf_specifier(const char *s, uint8_t &len)
    {
        switch (*s)
        {
        case 'd':
        case 'i':
        case 'u':
        case 'o':
        case 'x':
        case 'X':
        case 'c':
            len = 4;
            break;
        case 'f':
        case 'F':
        case 'e':
        case 'E':
        case 'g':
        case 'G':
        case 'a':
        case 'A':
        case 's':
        case 'p':
        case 'n':
            len = 8;
            break;
        default:
            return false;
        }
        return true;
    }

    __device__ static uint32_t round_up(uint32_t numToRound, uint32_t multiple_2)
    {
        return (numToRound + multiple_2 - uint32_t(1)) & -multiple_2;
    }

    __device__ static uint64_t read_valist(const char *valist_ptr, uint32_t &offset, uint8_t len)
    {
        // add padding
        offset = round_up(offset, len);
        uint64_t result;
        switch (len)
        {
        case 1:
            result = uint64_t(*(const uint8_t *)(valist_ptr + offset));
            break;
        case 2:
            result = uint64_t(*(const uint16_t *)(valist_ptr + offset));
            break;
        case 4:
            result = uint64_t(*(const uint32_t *)(valist_ptr + offset));
            break;
        case 8:
            result = uint64_t(*(const uint64_t *)(valist_ptr + offset));
            break;
        default:
            __builtin_unreachable();
        }
        offset += len;
        return result;
    }

    __device__ uint64_t __ockl_printf_begin(uint64_t version);
    __device__ uint64_t __ockl_printf_append_string_n(uint64_t msg_desc, const char *data, uint64_t length, uint32_t is_last);
    __device__ uint64_t __ockl_printf_append_args(uint64_t msg_desc, uint32_t num_args, uint64_t value0, uint64_t value1, uint64_t value2, uint64_t value3, uint64_t value4, uint64_t value5, uint64_t value6, uint32_t is_last);

    __device__ static uint32_t strlen_plus_one(const char *s)
    {
        const char *c = s;
        while ((*(c++)) != 0)
        {
        }
        return uint32_t(c - s);
    }

    uint32_t FUNC(vprintf)(uint64_t format, uint64_t valist)
    {
        uint64_t handle = __ockl_printf_begin(0);
        const char *msg = (const char *)format;
        const char *valist_ptr = (const char *)valist;
        const char *s = msg;
        uint32_t valist_offset = 0;
        uint32_t len = strlen_plus_one(msg);
        handle = __ockl_printf_append_string_n(handle, msg, len, 0);

        while (true)
        {
            char c = *(s++);
            if (c == 0)
                break;
            if (c != '%')
                continue;

            // %% requires no additional handling
            if (*s == '%')
            {
                s++;
                continue;
            }

            // %s uses __ockl_printf_append_string_n
            // https://github.com/ROCm/ROCm-Device-Libs/blob/rocm-5.7.x/ockl/src/services.cl#L343
            if (*s == 's')
            {
                s++;
                const char *value = (const char *)read_valist(valist_ptr, valist_offset, 8);
                handle = __ockl_printf_append_string_n(handle, value, strlen_plus_one(value), 0);
                continue;
            }

            // Keep scanning until we figure out the length of this specifier or if we reach the end of the string
            while (*s != 0) {
                // "The width is not specified in the format string, but as an additional integer value argument preceding the argument that has to be formatted."
                if (*s == '*') {
                    s++;
                    uint64_t value = read_valist(valist_ptr, valist_offset, 4);
                    handle = __ockl_printf_append_args(handle, 1, value, 0, 0, 0, 0, 0, 0, 0);
                    continue;
                }

                uint8_t len = 0;
                if (parse_printf_specifier(s, len))
                {
                    s++;
                }
                else
                {
                    uint8_t specifier_with_length = parse_printf_length(s, len);
                    if (specifier_with_length)
                    {
                        s += specifier_with_length;
                    } else {
                        // Assume the unknown character is a sub-specifier and move on
                        s++;
                        continue;
                    }
                }

                if (len > 0)
                {
                    uint64_t value = read_valist(valist_ptr, valist_offset, len);
                    handle = __ockl_printf_append_args(handle, 1, value, 0, 0, 0, 0, 0, 0, 0);
                }
                break;
            }
        }
        __ockl_printf_append_args(handle, 0, 0, 0, 0, 0, 0, 0, 0, 1);
        return 1;
    }

    int64_t __ockl_mul_hi_i64(int64_t x, int64_t y) __attribute__((device));
    int64_t FUNC(mul_hi_s64)(int64_t x, int64_t y)
    {
        return __ockl_mul_hi_i64(x, y);
    }

    int64_t FUNC(mad_hi_s64)(int64_t a, int64_t b, int64_t c)
    {
        int64_t temp = FUNC_CALL(mul_hi_s64)(a, b);
        return temp + c;
    }

    uint64_t __ockl_mul_hi_u64(uint64_t x, uint64_t y) __attribute__((device));
    uint64_t FUNC(mul_hi_u64)(uint64_t x, uint64_t y)
    {
        return __ockl_mul_hi_u64(x, y);
    }

    uint64_t FUNC(mad_hi_u64)(uint64_t a, uint64_t b, uint64_t c)
    {
        uint64_t temp = FUNC_CALL(mul_hi_u64)(a, b);
        return temp + c;
    }

    uint32_t __ockl_bfe_u32(uint32_t, uint32_t, uint32_t) __attribute__((device));
    uint32_t FUNC(bfe_u32)(uint32_t base, uint32_t pos, uint32_t len)
    {
        return __ockl_bfe_u32(base, pos, len);
    }

    uint64_t FUNC(bfe_u64)(uint64_t base, uint32_t pos, uint32_t len)
    {
        // No V_BFE_U64
        return (base >> pos) & ((1U << len) - 1U);
    }

    int32_t __ockl_bfe_i32(int32_t, uint32_t, uint32_t) __attribute__((device));
    int32_t FUNC(bfe_s32)(int32_t base, uint32_t pos, uint32_t len)
    {
        return __ockl_bfe_i32(base, pos, len);
    }

    int64_t FUNC(bfe_s64)(int64_t base, uint32_t pos, uint32_t len)
    {
        // No V_BFE_I64
        return (base << (64U - pos - len)) >> (64U - len);
    }

    uint32_t __ockl_bfm_u32(uint32_t count, uint32_t offset) __attribute__((device)) __attribute__((const));

    uint32_t FUNC(bfi_b32)(uint32_t insert, uint32_t base, uint32_t offset, uint32_t count)
    {
        offset = offset & 0xffU;
        if (offset > 31U)
        {
            return base;
        }
        count = count & 0xffU;
        uint32_t mask;
        if (count > 31U)
        {
            mask = UINT32_MAX << offset;
        }
        else
        {
            mask = __ockl_bfm_u32(count, offset);
        }
        return (~mask & base) | (mask & (insert << offset));
    }

    // NVIDIA docs are incorrect.
    // In bfi, operands c and d are restricted to 0..255 _only_ in b32 mode
    uint64_t FUNC(bfi_b64)(uint64_t insert, uint64_t base, uint32_t offset, uint32_t count)
    {
        if (offset > 63U)
        {
            return base;
        }
        uint64_t mask;
        if (count > 63U)
        {
            mask = UINT64_MAX << offset;
        }
        else
        {
            mask = ((1UL << count) - 1UL) << (offset);
        }
        return (~mask & base) | (mask & (insert << offset));
    }

    uint32_t FUNC(activemask)(void)
    {
        if (FUNC_CALL(COMPILATION_MODE) == CompilationMode::DoubleWave32OnWave64)
        {
            uint64_t exec = __builtin_amdgcn_read_exec();
            if (is_upper_warp())
            {
                return (uint32_t)(exec >> 32);
            }
            else
            {
                return (uint32_t)exec;
            }
        }
        return __builtin_amdgcn_read_exec_lo();
    }

    // Taken from __ballot definition in hipamd/include/hip/amd_detail/amd_device_functions.h
    // They return active threads, which I think is incorrect
    uint32_t FUNC(sreg_lanemask_lt)(void)
    {
        uint32_t lane_idx = FUNC_CALL(sreg_laneid)();
        uint32_t mask = (1U << lane_idx) - 1U;
        return mask;
    }

    uint32_t FUNC(sreg_lanemask_ge)(void)
    {
        uint32_t lane_idx = FUNC_CALL(sreg_laneid)();
        uint32_t mask = (~0U) << lane_idx;
        return mask;
    }

    uint32_t FUNC(sreg_lanemask_le)(void)
    {
        uint32_t lane_idx = FUNC_CALL(sreg_laneid)();
        uint32_t mask = 1U << lane_idx;
        return mask | (mask - 1);
    }

    size_t __ockl_get_local_linear_id() __device__;
    size_t __ockl_get_local_id(uint32_t) __device__;
    size_t __ockl_get_local_size(uint32_t) __device__;
    uint32_t FUNC(sreg_tid)(uchar dim)
    {
        if (FUNC_CALL(COMPILATION_MODE) == CompilationMode::Wave32OnWave64)
        {
            uint32_t linear_id_hip = (uint32_t)__ockl_get_local_linear_id();
            uint32_t wavefront_id = linear_id_hip / 64U;
            uint32_t laneid = linear_id_hip % 64U;
            uint32_t linear_id_cuda = (wavefront_id * 32) + laneid;
            if (dim == 0)
                return linear_id_cuda % __ockl_get_local_size(0);
            if (dim == 1)
                return (linear_id_cuda / __ockl_get_local_size(0)) % __ockl_get_local_size(1);
            if (dim == 2)
                return (linear_id_cuda / (__ockl_get_local_size(0) * __ockl_get_local_size(1))) % (__ockl_get_local_size(2) / 2);
            return 0;
        }
        else
        {
            return (uint32_t)__ockl_get_local_id(dim);
        }
    }

    uint32_t FUNC(sreg_ntid)(uchar dim)
    {
        uint32_t local_size = (uint32_t)__ockl_get_local_size(dim);
        if (FUNC_CALL(COMPILATION_MODE) == CompilationMode::Wave32OnWave64 && dim == 2)
        {
            local_size /= 2U;
        }
        return local_size;
    }

    size_t __ockl_get_group_id(uint32_t) __device__;
    uint32_t FUNC(sreg_ctaid)(uchar dim)
    {
        return (uint32_t)__ockl_get_group_id(dim);
    }

    size_t __ockl_get_num_groups(uint32_t) __device__;
    uint32_t FUNC(sreg_nctaid)(uchar dim)
    {
        return (uint32_t)__ockl_get_num_groups(dim);
    }

    uint64_t __ockl_cyclectr_u64(void) __device__;
    __attribute__((always_inline)) uint32_t FUNC(sreg_clock)(void)
    {
        return (uint32_t)__ockl_cyclectr_u64();
    }

    __attribute__((always_inline)) uint64_t FUNC(sreg_clock64)()
    {
        return __ockl_cyclectr_u64();
    }

    void FUNC(barrier_sync)(uint32_t)
    {
        // I'm not 100% how should a barrier be defined:
        // * In NVIDIA world barrier.sync == __syncthreads()
        // * Majority of internet sources claim that __syncthreads() is
        //   equivalent to OpenCL barrier(CLK_LOCAL_MEM_FENCE), but this
        //   can't be true. Both docs for __syncthreads(...) and barrier
        //   imply that it syncs global scope memory too
        // * AMDGPU has two barrier definitions, one in HIP
        //   (https://github.com/ROCm-Developer-Tools/hipamd/blob/312dff7b794337aa040be0691acc78e9f968a8d2/include/hip/amd_detail/amd_device_functions.h#L867),
        //   where the sequence is fence(release), s_barrier, fence(acquire) no matter the fence flags.
        //   Another is in ROCm-Device-Libs
        //   (https://github.com/RadeonOpenCompute/ROCm-Device-Libs/blob/amd-stg-open/opencl/src/workgroup/wgbarrier.cl#L21)
        //   where the sequence is fence(seq_cst), s_barrier, fence(seq_cst)
        // * __builtin_amdgcn_fence(..., "workgroup") produce s_waitcnt on everything and L1 flush if it's seq_cst
        //   I don't see much point in emitting the fence twice. It's not like there were any memory loads or writes
        //   in-between those fense to wait on
        __builtin_amdgcn_fence(__ATOMIC_SEQ_CST, "workgroup");
        __builtin_amdgcn_s_barrier();
    }

    half __ockl_median3_f16(half, half, half) __attribute__((device));
    half FUNC(cvt_sat_f16_f16)(half x)
    {
        return __ockl_median3_f16(x, 0, 1);
    }

    float __ockl_median3_f32(float, float, float) __attribute__((device));
    float FUNC(cvt_sat_f32_f32)(float x)
    {
        return __ockl_median3_f32(x, 0.0f, 1.0f);
    }

    double FUNC(cvt_sat_f64_f64)(double x)
    {
        return fmin(fmax(x, 0.0), 1.0);
    }

    __device__ uint32_t __llvm_fshl_i32(uint32_t a, uint32_t b, uint32_t c) __asm("llvm.fshl.i32");
    uint32_t FUNC(shf_l_clamp_b32)(uint32_t a, uint32_t b, uint32_t c)
    {
        if (c >= 32)
            return a;
        return __llvm_fshl_i32(b, a, c);
    }

    __device__ uint32_t __llvm_fshr_i32(uint32_t a, uint32_t b, uint32_t c) __asm("llvm.fshr.i32");
    uint32_t FUNC(shf_r_clamp_b32)(uint32_t a, uint32_t b, uint32_t c)
    {
        if (c >= 32)
            return b;
        return __llvm_fshr_i32(b, a, c);
    }

    uint32_t __ockl_udot4(uint32_t, uint32_t, uint32_t, bool) __attribute__((device));
    uint32_t FUNC(dp4a_u32_u32)(uint32_t a, uint32_t b, uint32_t c)
    {
        return __ockl_udot4(a, b, c, false);
    }

    int32_t __ockl_sdot4(int32_t, int32_t, int32_t, bool) __attribute__((device));
    int32_t FUNC(dp4a_s32_s32)(int32_t a, int32_t b, int32_t c)
    {
        return __ockl_sdot4(a, b, c, false);
    }
}
__device__ uint32_t __llvm_amdgcn_ballot_i32(bool) __asm("llvm.amdgcn.ballot.i32");
__device__ uint64_t __llvm_amdgcn_ballot_i64(bool) __asm("llvm.amdgcn.ballot.i64");

template <CompilationMode compilation_mode>
__device__ static inline uint32_t vote_sync_pred(bool value, uint32_t membermask, bool negate)
{
    __builtin_amdgcn_wave_barrier();
    if constexpr (compilation_mode == CompilationMode::DoubleWave32OnWave64)
    {
        uint64_t vote = __llvm_amdgcn_ballot_i64(negate ? !value : value);
        if (is_upper_warp())
        {
            return ((uint32_t)(vote >> 32)) & membermask;
        }
        else
        {
            return ((uint32_t)vote) & membermask;
        }
    }
    else if constexpr (compilation_mode == CompilationMode::Wave32)
    {
        return __llvm_amdgcn_ballot_i32(negate ? !value : value) & membermask;
    }
    else if constexpr (compilation_mode == CompilationMode::Wave32OnWave64)
    {
        return ((uint32_t)__llvm_amdgcn_ballot_i64(negate ? !value : value)) & membermask;
    }
    else
    {
        __builtin_unreachable();
        return 0;
    }
}

extern "C"
{

#define GENERATE_VOTE_SYNC(SUFFIX, MODE)                                                 \
    bool FUNC(vote_sync_any_pred_##SUFFIX)(bool value, uint32_t membermask)              \
    {                                                                                    \
        return vote_sync_pred<MODE>(value, membermask, false) != 0;                      \
    }                                                                                    \
                                                                                         \
    bool FUNC(vote_sync_any_pred_negate_##SUFFIX)(bool value, uint32_t membermask)       \
    {                                                                                    \
        return vote_sync_pred<MODE>(value, membermask, true) != 0;                       \
    }                                                                                    \
                                                                                         \
    bool FUNC(vote_sync_all_pred_##SUFFIX)(bool value, uint32_t membermask)              \
    {                                                                                    \
        return vote_sync_pred<MODE>(value, membermask, false) == membermask;             \
    }                                                                                    \
                                                                                         \
    bool FUNC(vote_sync_all_pred_negate_##SUFFIX)(bool value, uint32_t membermask)       \
    {                                                                                    \
        return vote_sync_pred<MODE>(value, membermask, true) == membermask;              \
    }                                                                                    \
                                                                                         \
    uint32_t FUNC(vote_sync_ballot_b32_##SUFFIX)(bool value, uint32_t membermask)        \
    {                                                                                    \
        return vote_sync_pred<MODE>(value, membermask, false);                           \
    }                                                                                    \
                                                                                         \
    uint32_t FUNC(vote_sync_ballot_b32_negate_##SUFFIX)(bool value, uint32_t membermask) \
    {                                                                                    \
        return vote_sync_pred<MODE>(value, membermask, true);                            \
    }

    GENERATE_VOTE_SYNC(32, CompilationMode::Wave32);
    GENERATE_VOTE_SYNC(32on64, CompilationMode::Wave32OnWave64);
    GENERATE_VOTE_SYNC(double32on64, CompilationMode::DoubleWave32OnWave64);

    // From NVIDIA PTX documentation:
    // "Instruction barrier{.cta} has optional .aligned modifier. When specified,
    //  it indicates that all threads in CTA will execute the same barrier{.cta} instruction.
    //  In conditionally executed code, an aligned barrier{.cta} instruction should
    //  only be used if it is known that all threads in CTA evaluate the condition identically,
    //  otherwise behavior is undefined."
    // This would imply that unaligned bar.red makes sense, but not sure how much sense does it make?
    // Maybe with non-zero barriers?
    int32_t __ockl_wgred_and_i32(int32_t) __attribute__((device));
    bool FUNC(bar_red_and_pred)(__attribute__((unused)) uint32_t barrier, bool predicate)
    {
        return __ockl_wgred_and_i32(predicate);
    }

    int32_t __ockl_wgred_or_i32(int32_t) __attribute__((device));
    bool FUNC(bar_red_or_pred)(__attribute__((unused)) uint32_t barrier, bool predicate)
    {
        return __ockl_wgred_or_i32(predicate);
    }

    int32_t __ockl_wgred_add_i32(int32_t) __attribute__((device));
    uint32_t FUNC(bar_red_popc_u32)(__attribute__((unused)) uint32_t barrier, bool predicate)
    {
        return (uint32_t)__ockl_wgred_add_i32(predicate);
    }

    void FUNC(nanosleep_u32)(uint32_t ns)
    {
        // There's no good conversion here, we should read base clock from the environment
        // and use it here, but that's a lot of work for very little gain
        // So instead I'm assuming here:
        // * 2000 MHz if it's wave32 because that's the game clock on my RX 6800 XT,
        //   meaning 0.5ns for a clock cycle
        // * 1000 MHz if it's wave64 because that's the base clock for MI300,
        //   meaning 1ns for a clock cycle
        // Expected sleep time for S_SLEEP x is (x*64)-32 clocks
        // We change that 32 offset to 31/63 to avoid small sleep values resulting in s_sleep 0
        uint32_t sleep_amount;
        if (FUNC_CALL(COMPILATION_MODE) == CompilationMode::Wave32)
        {
            sleep_amount = (ns + 31U) / 32U;
        }
        else
        {
            sleep_amount = (ns + 63U) / 64U;
        }
        // We do this garbage because argument to __builtin_amdgcn_s_sleep must be a constant
        for (uint32_t i = 0; i < (sleep_amount >> 4U); i++)
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
}
