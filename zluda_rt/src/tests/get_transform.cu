// nvcc get_transform.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtBuffer<float, 1> object_transforms;
rtDeclareVariable(optix::Ray, ray, rtCurrentRay, );
rtDeclareVariable(rtObject, bvh, , );
rtDeclareVariable(float4,  sphere, , );
rtDeclareVariable(uint2, launch_index, rtLaunchIndex, );

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
      if(rtReportIntersection(0))
      {
        rtGetTransform(RT_OBJECT_TO_WORLD, &object_transforms[16*0]);
        check_second = false;
      }
    } 
    if(check_second) {
      float root2 = (-b + sdisc);
      if( rtPotentialIntersection( root2 * l ) ) {
        if(rtReportIntersection(0))
        {
          rtGetTransform(RT_OBJECT_TO_WORLD, &object_transforms[16*0]);
        }
      }
    }
  }
}

RT_PROGRAM void bounds (int, float result[6])
{
  // fails compilation
  //rtGetTransform(RT_OBJECT_TO_WORLD, &object_transforms[16*0]);
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

RT_PROGRAM void any_hit() {
  rtGetTransform(RT_OBJECT_TO_WORLD, &object_transforms[16*1]);
}

RT_PROGRAM void closest_hit() {
  rtGetTransform(RT_WORLD_TO_OBJECT, &object_transforms[16*2]);
}
