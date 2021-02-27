#include <optix.h>

using namespace optix;

rtBuffer<float4> output;
rtDeclareVariable(rtCallableProgramId<float3()>, eval, , );

RT_PROGRAM void closest_hit()
{
    float3 result = eval();
    output[0] = make_float4(result.x, result.y, result.y, 0.0);
}