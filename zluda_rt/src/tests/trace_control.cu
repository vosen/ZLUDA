// nvcc trace_control.cu -I"C:\dev\OptiX SDK 6.5.0\include"  -ptx -x cu -dc
#include <optix.h>
#include <optixu/optixu_math_namespace.h>
#include <optix_world.h>

using namespace optix;

rtDeclareVariable(float4, sphere, , );
rtBuffer<uint, 1> temp_buffer;
rtBuffer<uint, 1> output_buffer;
rtDeclareVariable(uint2, launch_index, rtLaunchIndex, );
rtDeclareVariable(float, closest_distance, rtIntersectionDistance, );
rtDeclareVariable(uint, increment, attribute increment, );
rtDeclareVariable(rtObject, bvh, , );
rtDeclareVariable(optix::Ray, ray, rtCurrentRay, );

RT_PROGRAM void start(void)
{
  Ray ray = make_Ray(make_float3(0, 0, -1), make_float3(0, 0, 1), 0, 0.0, RT_DEFAULT_MAX);
  char unused = 0;
  rtTrace(bvh, ray, unused);
}

RT_PROGRAM void intersect(int primIdx)
{
  float3 center = make_float3(sphere);
  float3 O = ray.origin - center;
  float l = 1 / length(ray.direction);
  float3 D = ray.direction * l;
  float radius = sphere.w;

  float b = dot(O, D);
  float c = dot(O, O) - radius * radius;
  float disc = b * b - c;
  if (disc > 0.0f)
  {
    float sdisc = sqrtf(disc);
    float root1 = (-b - sdisc);

    float root11 = 0.0f;

    bool check_second = true;
    if (rtPotentialIntersection(((root1 + root11) * l) - temp_buffer[launch_index.x]))
    {
      increment = temp_buffer[launch_index.x];
      if (rtReportIntersection(0))
        check_second = false;
    }
    if (check_second)
    {
      float root2 = (-b + sdisc);
      if (rtPotentialIntersection((root2 * l) - temp_buffer[launch_index.x]))
      {
        increment = temp_buffer[launch_index.x];
        rtReportIntersection(0);
      }
    }
  }
}

RT_PROGRAM void bounds(int, float result[6])
{
  const float3 cen = make_float3(sphere);
  const float3 rad = make_float3(sphere.w);

  optix::Aabb *aabb = (optix::Aabb *)result;

  if (rad.x > 0.0f && !isinf(rad.x))
  {
    aabb->m_min = cen - rad;
    aabb->m_max = cen + rad;
  }
  else
  {
    aabb->invalidate();
  }
}

RT_PROGRAM void any_hit_ignore(void)
{
  temp_buffer[launch_index.x] += 1;
  if (temp_buffer[launch_index.x] >= 3)
  {
    rtIgnoreIntersection();
  }
}

RT_PROGRAM void any_hit_terminate(void)
{
  temp_buffer[launch_index.x] += 1;
  if (temp_buffer[launch_index.x] >= 3)
  {
    rtTerminateRay();
  }
}

RT_PROGRAM void attribute1(void)
{
  increment = 0xc4bb2187 + temp_buffer[0];
}

RT_PROGRAM void any_hit_plus_one(void)
{
  temp_buffer[0] += 1;
}

RT_PROGRAM void attribute2(void)
{
  increment = 0xc4bb2187;
}

RT_PROGRAM void any_hit_always_ignore(void)
{
  temp_buffer[0] += 1;
  rtIgnoreIntersection();
}

RT_PROGRAM void closest_hit(void)
{
  output_buffer[launch_index.x] = increment;
}