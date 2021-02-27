// nvcc exception.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtBuffer<unsigned int, 1> var_buffer;
rtDeclareVariable(rtObject, bvh, , );
rtDeclareVariable(uint2, launch_index, rtLaunchIndex, );

__device__ __noinline__ void trace() {
    Ray ray = make_Ray(make_float3(float(launch_index.x), 0, -1), make_float3(0,0,1), 0, 0.0, RT_DEFAULT_MAX);
    char unused = 0;
    rtTrace(bvh, ray, unused);
}

RT_PROGRAM void start() {
    trace();
}

RT_PROGRAM void throw_() {
    rtThrow(RT_EXCEPTION_USER);
}

RT_PROGRAM void exception() {
    var_buffer[0] = rtGetExceptionCode();
}
