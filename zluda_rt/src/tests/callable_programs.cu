// nvcc callable_programs.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtDeclareVariable(unsigned int, value, , );
rtBuffer<unsigned int, 1>   output_buffer;

typedef rtCallableProgramId<unsigned int(unsigned int)> int_operator;
rtDeclareVariable(int_operator, add_fn,,);
rtDeclareVariable(int_operator, mult_fn,,);

RT_CALLABLE_PROGRAM unsigned int add_value(unsigned int input) {
    return input + value;
}

RT_CALLABLE_PROGRAM unsigned int multiply_value(unsigned int input) {
    return input * value;
}

RT_PROGRAM void start() {    
    unsigned int x = value;
    x = add_fn(x);
    x = mult_fn(x);
    output_buffer[0] = x;
}
