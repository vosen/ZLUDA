// nvcc barycentrics.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtDeclareVariable(float2, barycentrics, attribute rtTriangleBarycentrics, );
rtBuffer<float2, 1>   output_buffer1;
rtBuffer<float2, 1>   output_buffer2;
rtBuffer<int, 1>   output_buffer3;
rtDeclareVariable(rtObject, bvh, , );
rtDeclareVariable(float4,  sphere, , );

rtDeclareVariable(optix::Ray, ray, rtCurrentRay, );
rtDeclareVariable(int2, launch_index, rtLaunchIndex, );

RT_PROGRAM void start() {
    Ray ray = make_Ray(make_float3(float(launch_index.x), 0, -1), make_float3(0,0,1), 0, 0.0, RT_DEFAULT_MAX);
    char unused = 0;
    rtTrace(bvh, ray, unused);
}

RT_PROGRAM void intersect(int primIdx)
{
  float3 center = make_float3(sphere);
  float3 O = ray.origin - center;
  float  l = 1 / length(ray.direction);
  float3 D = ray.direction * l;
  float radius = sphere.w;

  float b = dot(O, D);
  float c = dot(O, O)-radius*radius;
  float disc = b*b-c;
  if(disc > 0.0f){
    float sdisc = sqrtf(disc);
    float root1 = (-b - sdisc);

    float root11 = 0.0f;

    bool check_second = true;
    if( rtPotentialIntersection( (root1 + root11) * l ) ) {
      barycentrics = make_float2(100, 200);
      if(rtReportIntersection(0))
        check_second = false;
    } 
    if(check_second) {
      float root2 = (-b + sdisc);
      if( rtPotentialIntersection( root2 * l ) ) {
        barycentrics = make_float2(100, 200);
        rtReportIntersection(0);
      }
    }
  }
}

RT_PROGRAM void bounds (int, float result[6])
{
  const float3 cen = make_float3( sphere );
  const float3 rad = make_float3( sphere.w );

  optix::Aabb* aabb = (optix::Aabb*)result;
  
  if( rad.x > 0.0f  && !isinf(rad.x) ) {
    aabb->m_min = cen - rad;
    aabb->m_max = cen + rad;
  } else {
    aabb->invalidate();
  }
}

RT_PROGRAM void attribute_program() {
    float2 read_barycentrics = rtGetTriangleBarycentrics();
    barycentrics.x = read_barycentrics.x;
    barycentrics.y = 0.1;
}

RT_PROGRAM void closest_hit() {
    output_buffer1[launch_index.x] = barycentrics;
    // rtGetTriangleBarycentrics() happens to work here,
    // but is only valid in attribute programs
    //output_buffer2[launch_index.x] = rtGetTriangleBarycentrics();
    output_buffer3[launch_index.x] = rtGetPrimitiveIndex();
}