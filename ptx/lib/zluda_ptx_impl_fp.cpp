#define FUNC(NAME) __device__ __attribute__((retain)) __zluda_ptx_impl_##NAME

const float DENORMAL_TO_NORMAL_FACTOR_F32 = 16777216.0f;
// * Largest subnormal is ~1.175494210692441e × 10^(-38)
// * Then any value equal or larger than following will produce subnormals: 8.50706018714406320806444272332455743547934627837873057975602739772164... × 10^37
const float RCP_DENORMAL_OUTPUT = 8.50706018714406320806444272332455743547934627837873057975602739772164e37f;
const float REVERSE_DENORMAL_TO_NORMAL_FACTOR_F32 = 0.029387360490963111877208252592662410455594571842846914442095471744599661631813495980086003637902577995683214210345151992265999035207077609582844f;

extern "C"
{
    float FUNC(rcp_approx_f32)(float x)
    {
        float factor = 1.0f;
        if (__builtin_isfpclass(x, __FPCLASS_NEGSUBNORMAL | __FPCLASS_POSSUBNORMAL))
        {
            factor = DENORMAL_TO_NORMAL_FACTOR_F32;
        }
        if (std::fabs(x) >= RCP_DENORMAL_OUTPUT)
        {
            factor = REVERSE_DENORMAL_TO_NORMAL_FACTOR_F32;
        }
        return __builtin_amdgcn_rcpf(x * factor) * factor;
    }
}
