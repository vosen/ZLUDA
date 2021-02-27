// nvcc transform.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtBuffer<unsigned int, 1> is_triangle;
rtBuffer<rtBufferId<float3, 1>> origin;
rtBuffer<rtBufferId<float3, 1>> direction;
rtDeclareVariable(optix::Ray, ray, rtCurrentRay, );
rtDeclareVariable(rtObject, bvh, , );
rtDeclareVariable(uint2, launch_index, rtLaunchIndex, );
rtDeclareVariable(float4,  sphere, , );

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
        origin[launch_index.x][0] = ray.origin;
        direction[launch_index.x][0] = ray.direction;
        check_second = false;
      }
    } 
    if(check_second) {
      float root2 = (-b + sdisc);
      if( rtPotentialIntersection( root2 * l ) ) {
        if(rtReportIntersection(0))
        {
            origin[launch_index.x][0] = ray.origin;
            direction[launch_index.x][0] = ray.direction;
        }
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

// attribute uses the same ray as related *hit function,
// so transformed for anyhit and untransformed for closesthit
RT_PROGRAM void attribute() {
    origin[launch_index.x][0] = ray.origin;
    direction[launch_index.x][0] = ray.direction;
}

RT_PROGRAM void any_hit() {
    origin[launch_index.x][1] = ray.origin;
    direction[launch_index.x][1] = ray.direction;
}

RT_PROGRAM void closest_hit() {
    origin[launch_index.x][2] = ray.origin;
    direction[launch_index.x][2] = ray.direction;
    is_triangle[launch_index.x] = rtIsTriangleHit();
}
