// nvcc buffer_id.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtDeclareVariable( uint, texture_id, , );
rtBuffer<uint2, 1>   output_buffer;

RT_PROGRAM void start() {
    bool   isResident;
    uint4 val0 = rtTex2DLodLoadOrRequest<uint4>( texture_id, 0, 0, 0, isResident );
    output_buffer[0] = make_uint2(val0.x, val0.y);
    uint4 val1 = rtTex2DLodLoadOrRequest<uint4>( texture_id, 0, 0, 1000, isResident );
    output_buffer[1] = make_uint2(val1.x, val1.y);
}
