// nvcc default_variable.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtBuffer<unsigned int, 1> var_buffer;
rtDeclareVariable(unsigned int, x, , ) = 55;

RT_PROGRAM void start() {
    var_buffer[0] = x;
}
