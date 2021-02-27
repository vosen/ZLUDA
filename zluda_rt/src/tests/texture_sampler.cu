// nvcc texture_sampler.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtBuffer<float4, 2>   output_buffer;
rtTextureSampler<float4, 2> image1;
rtDeclareVariable(unsigned int, image2, , );

RT_PROGRAM void start() {
    output_buffer[make_uint2(0,0)] = tex2D(image1, 0, 0);
    output_buffer[make_uint2(1,0)] = tex2D(image1, 1, 0);
    output_buffer[make_uint2(0,1)] = tex2D(image1, 0, 1);
    output_buffer[make_uint2(1,1)] = tex2D(image1, 1, 1);
    output_buffer[make_uint2(0,2)] = rtTex2D<float4>(image2, 0, 0);
    output_buffer[make_uint2(1,2)] = rtTex2D<float4>(image2, 1, 0);
    output_buffer[make_uint2(0,3)] = rtTex2D<float4>(image2, 0, 1);
    output_buffer[make_uint2(1,3)] = rtTex2D<float4>(image2, 1, 1);
}
