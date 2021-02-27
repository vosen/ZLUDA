// nvcc oob.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtBuffer<unsigned int> index_;
rtBuffer<unsigned int> input;
rtBuffer<unsigned int> output;

RT_PROGRAM void start() {
    output[0] = input[index_[0]];
    output[1] = input[index_[1]];
    output[2] = *((unsigned int *)rt_buffer_get_id(0, 1, 4, 10,10,0,0));
}
