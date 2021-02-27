#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtDeclareVariable(float4,  sphere, , );
rtDeclareVariable(uint2, launch_index, rtLaunchIndex, );
rtDeclareVariable(unsigned int, b_index, , );
rtBuffer<unsigned int, 1>   output_buffer;
rtBuffer<unsigned int, 1>   output_buffer2;
rtDeclareVariable(rtObject, bvh, , );
rtDeclareVariable(optix::Ray, ray, rtCurrentRay, );

RT_PROGRAM void start(void)
{
    Ray ray = make_Ray(make_float3(0, 0, -1), make_float3(0,0,1), 0, 0.0, RT_DEFAULT_MAX);
    char unused = 0;
    rtTrace(bvh, ray, unused);
}

RT_PROGRAM void set_buffer(void)
{
    atomicAdd(&output_buffer[b_index], 1);
}

RT_PROGRAM void set_buffer2(void)
{
    atomicAdd(&output_buffer2[b_index], 1);
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
      //shading_normal = geometric_normal = (O + (root1 + root11)*D)/radius;
      if(rtReportIntersection(launch_index.x))
        check_second = false;
    } 
    if(check_second) {
      float root2 = (-b + sdisc);
      if( rtPotentialIntersection( root2 * l ) ) {
        //shading_normal = geometric_normal = (O + root2*D)/radius;
        rtReportIntersection(launch_index.x);
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
