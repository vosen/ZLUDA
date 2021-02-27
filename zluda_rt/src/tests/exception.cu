// nvcc exception.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtBuffer<unsigned int, 1> var_buffer;
rtDeclareVariable(rtObject, bvh, , );
rtDeclareVariable(uint2, launch_index, rtLaunchIndex, );
typedef rtCallableProgramId<unsigned int(unsigned int)> int_operator;
rtDeclareVariable(int_operator, callable1,,);
rtDeclareVariable(int_operator, callable2,,);
rtDeclareVariable(int_operator, callable3,,);

RT_PROGRAM void trace() {
    Ray ray = make_Ray(make_float3(float(launch_index.x), 0, -1), make_float3(0,0,1), 0, 0.0, RT_DEFAULT_MAX);
    char unused = 0;
    rtTrace(bvh, ray, unused);
}

RT_PROGRAM void throw_() {
    rtThrow(RT_EXCEPTION_USER);
}

RT_PROGRAM void exception() {
    var_buffer[0] = rtGetExceptionCode();
}

RT_PROGRAM void call_callable1() {
    callable1(1);
}

RT_CALLABLE_PROGRAM  unsigned int call_callable2(unsigned int x) {
    return callable2(x);
}

RT_CALLABLE_PROGRAM  unsigned int throw_callable(unsigned int x) {
    rtThrow(RT_EXCEPTION_USER + x);
    return x;
}

__noinline__ __device__ void throw_callable_sub() {
    callable3(1);
}

RT_CALLABLE_PROGRAM  unsigned int throw_callable_main(unsigned int x) {
    throw_callable_sub();
    return x;
}
