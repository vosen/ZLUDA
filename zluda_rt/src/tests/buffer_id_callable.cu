// nvcc buffer_id_callable.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtBuffer<rtBufferId<unsigned int> > buffers;
rtDeclareVariable(rtCallableProgramId<void(void)>, program,,);

RT_CALLABLE_PROGRAM void callable() {
    buffers[0][2] = 0x0118378c;
    buffers[0][1] = buffers[0].size();
}

RT_PROGRAM void start() {
    program();
}
