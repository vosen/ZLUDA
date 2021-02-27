// nvcc alloca_bug.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>

rtDeclareVariable(rtCallableProgramId<float3(float3 &mat, float3 &prd)>, sysBRDFEval, , );
rtBuffer<float3> sysMaterialParameters;

RT_PROGRAM void closest_hit()
{
	float3 mat = sysMaterialParameters[0];

	if (mat.x != 0)
	{
		const float3 texColor = make_float3(0, 0,0);
		mat = make_float3(powf(texColor.x, 2.2f), 0,0);
	}
	float3 prd2;
	float3 f = sysBRDFEval(mat, prd2);

	if (prd2.x > 0.0f)
		prd2 *= f; 
}
