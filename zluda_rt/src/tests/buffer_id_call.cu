// nvcc buffer_id_call.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtBuffer<rtBufferId<unsigned int> > buffers;

__noinline__
__device__ void start2() {
    buffers[0][2] = 0x0118378c;
    buffers[0][1] = buffers[0].size();
}

__noinline__
__device__ void start1() {
    start2();
}

RT_PROGRAM void start() {
    start1();
}
