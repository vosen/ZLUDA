use crate::optix_test;
use crate::test_common::OptixFns;
use float_cmp::assert_approx_eq;
use glam::{Mat4, Quat, Vec3};
use optix_types::*;
use std::{ffi::CStr, mem, ptr};

const _SET_VARIABLE_CU: &'static [u8] = b"
#include <optix.h>
#include <optixu/optixu_math_namespace.h>

using namespace optix;

struct Payload {
    float data;
};

rtDeclareVariable(uint2, launch_index, rtLaunchIndex, );
rtDeclareVariable(float, value, , );
rtBuffer<float, 1>   output_buffer;
rtDeclareVariable(rtObject, bvh, , );
rtDeclareVariable(Payload, payload, rtPayload, );

RT_PROGRAM void start(void)
{
    Payload p { float(launch_index.x) };
    Ray ray = make_Ray(make_float3(float(launch_index.x), 0, -1), make_float3(0,0,1), 0, 0.0, RT_DEFAULT_MAX);
    rtTrace(bvh, ray, p);
}

RT_PROGRAM void set_variable(void)
{
    output_buffer[launch_index.x] = value;
}
";

const SET_VARIABLE_PTX: &'static [u8] = b"    
.version 7.0
.target sm_52
.address_size 64

    // .globl	_Z5startv
.global .align 8 .b8 launch_index[8];
.global .align 4 .f32 value;
.global .align 1 .b8 output_buffer[1];
.global .align 4 .b8 bvh[4];
.global .align 4 .b8 payload[4];
.global .align 4 .b8 _ZN21rti_internal_typeinfo12launch_indexE[8] = {82, 97, 121, 0, 8, 0, 0, 0};
.global .align 4 .b8 _ZN21rti_internal_typeinfo5valueE[8] = {82, 97, 121, 0, 4, 0, 0, 0};
.global .align 4 .b8 _ZN21rti_internal_typeinfo3bvhE[8] = {82, 97, 121, 0, 4, 0, 0, 0};
.global .align 4 .b8 _ZN21rti_internal_typeinfo7payloadE[8] = {82, 97, 121, 0, 4, 0, 0, 0};
.global .align 8 .u64 _ZN21rti_internal_register20reg_bitness_detectorE;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail0E;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail1E;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail2E;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail3E;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail4E;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail5E;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail6E;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail7E;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail8E;
.global .align 8 .u64 _ZN21rti_internal_register24reg_exception_64_detail9E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail0E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail1E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail2E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail3E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail4E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail5E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail6E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail7E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail8E;
.global .align 4 .u32 _ZN21rti_internal_register21reg_exception_detail9E;
.global .align 4 .u32 _ZN21rti_internal_register14reg_rayIndex_xE;
.global .align 4 .u32 _ZN21rti_internal_register14reg_rayIndex_yE;
.global .align 4 .u32 _ZN21rti_internal_register14reg_rayIndex_zE;
.global .align 1 .b8 _ZN21rti_internal_typename12launch_indexE[6] = {117, 105, 110, 116, 50, 0};
.global .align 1 .b8 _ZN21rti_internal_typename5valueE[6] = {102, 108, 111, 97, 116, 0};
.global .align 1 .b8 _ZN21rti_internal_typename3bvhE[9] = {114, 116, 79, 98, 106, 101, 99, 116, 0};
.global .align 1 .b8 _ZN21rti_internal_typename7payloadE[8] = {80, 97, 121, 108, 111, 97, 100, 0};
.global .align 4 .u32 _ZN21rti_internal_typeenum12launch_indexE = 4919;
.global .align 4 .u32 _ZN21rti_internal_typeenum5valueE = 4919;
.global .align 4 .u32 _ZN21rti_internal_typeenum3bvhE = 4919;
.global .align 4 .u32 _ZN21rti_internal_typeenum7payloadE = 4919;
.global .align 1 .b8 _ZN21rti_internal_semantic12launch_indexE[14] = {114, 116, 76, 97, 117, 110, 99, 104, 73, 110, 100, 101, 120, 0};
.global .align 1 .b8 _ZN21rti_internal_semantic5valueE[1];
.global .align 1 .b8 _ZN21rti_internal_semantic3bvhE[1];
.global .align 1 .b8 _ZN21rti_internal_semantic7payloadE[10] = {114, 116, 80, 97, 121, 108, 111, 97, 100, 0};
.global .align 1 .b8 _ZN23rti_internal_annotation12launch_indexE[1];
.global .align 1 .b8 _ZN23rti_internal_annotation5valueE[1];
.global .align 1 .b8 _ZN23rti_internal_annotation3bvhE[1];
.global .align 1 .b8 _ZN23rti_internal_annotation7payloadE[1];

.visible .entry _Z5startv(

)
{
    .local .align 4 .b8 	__local_depot0[4];
    .reg .b64 	%SP;
    .reg .b64 	%SPL;
    .reg .f32 	%f<9>;
    .reg .b32 	%r<7>;
    .reg .b64 	%rd<3>;


    mov.u64 	%SPL, __local_depot0;
    cvta.local.u64 	%SP, %SPL;
    add.u64 	%rd1, %SP, 0;
    add.u64 	%rd2, %SPL, 0;
    ld.global.u32 	%r6, [launch_index];
    cvt.rn.f32.u32	%f1, %r6;
    st.local.f32 	[%rd2], %f1;
    ld.global.u32 	%r1, [bvh];
    mov.u32 	%r3, 255;
    mov.u32 	%r4, 0;
    mov.u32 	%r5, 4;
    mov.f32 	%f3, 0fBF800000;
    mov.f32 	%f6, 0f3F800000;
    mov.f32 	%f7, 0f00000000;
    mov.f32 	%f8, 0f6C4ECB8F;
    // inline asm
    call _rt_trace_mask_flags_64, (%r1, %f1, %f7, %f3, %f7, %f7, %f6, %r4, %f7, %f8, %r3, %r4, %rd1, %r5);
    // inline asm
    ret;
}

    // .globl	_Z12set_variablev
.visible .entry _Z12set_variablev(

)
{
    .reg .f32 	%f<2>;
    .reg .b32 	%r<3>;
    .reg .b64 	%rd<8>;


    ld.global.f32 	%f1, [value];
    ld.global.u32 	%rd3, [launch_index];
    mov.u64 	%rd7, output_buffer;
    cvta.global.u64 	%rd2, %rd7;
    mov.u32 	%r1, 1;
    mov.u32 	%r2, 4;
    mov.u64 	%rd6, 0;
    // inline asm
    call (%rd1), _rt_buffer_get_64, (%rd2, %r1, %r2, %rd3, %rd6, %rd6, %rd6);
    // inline asm
    st.f32 	[%rd1], %f1;
    ret;
}\0";

optix_test!(variable_scoping);

// List of points, `+` means variable is set, `-` means variable is unset
// 0: Program-, GI-, Material-, Context+ => produces 4
// 1: Program+, GI-, Material-, Context+ => produces 1
// 2: Program-, GI+, Material-, Context+ => produces 2
// 3: Program-, GI-, Material+, Context+ => produces 3
unsafe fn variable_scoping<Optix: OptixFns>(mut o: Optix) {
    let variable_key = b"value\0";
    let triangles = 4;
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let mut ctx_variable = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, variable_key.as_ptr() as _, &mut ctx_variable);
    o.rtVariableSet1f(ctx_variable, 4f32);
    let mut output_buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_OUTPUT.0, &mut output_buffer);
    o.rtBufferSetSize1D(output_buffer, triangles as u64);
    o.rtBufferSetFormat(output_buffer, RTformat::RT_FORMAT_FLOAT);
    let mut output_buffer_var = ptr::null_mut();
    o.rtContextDeclareVariable(
        ctx,
        b"output_buffer\0".as_ptr() as _,
        &mut output_buffer_var,
    );
    o.rtVariableSetObject(output_buffer_var, RTobject(output_buffer as _));
    let mut raygen = mem::zeroed();
    o.rtProgramCreateFromPTXString(
        ctx,
        SET_VARIABLE_PTX.as_ptr() as _,
        b"start\0".as_ptr() as _,
        &mut raygen,
    );
    o.rtContextSetRayGenerationProgram(ctx, 0, raygen);
    // we are reusing material&program pair to see how ZLUDA handles this
    let anyhit_unset = create_anyhit_program(&mut o, ctx);
    let material_unset = create_material(&mut o, ctx);
    let triangle_instances = (0..triangles)
        .map(|triangle| create_triangles(&mut o, ctx, triangle, 1, 0.0).0)
        .collect::<Vec<_>>();
    setup_geometry_instance(
        &mut o,
        triangle_instances[0],
        material_unset,
        anyhit_unset,
        ptr::null_mut(),
    );
    let anyhit_1 = create_anyhit_program_with_var(&mut o, ctx, variable_key, 1f32);
    let material_1 = create_material(&mut o, ctx);
    setup_geometry_instance(
        &mut o,
        triangle_instances[1],
        material_1,
        anyhit_1,
        ptr::null_mut(),
    );
    let mut variable_2 = ptr::null_mut();
    o.rtGeometryInstanceDeclareVariable(
        triangle_instances[2],
        variable_key.as_ptr() as _,
        &mut variable_2,
    );
    o.rtVariableSet1f(variable_2, 2f32);
    setup_geometry_instance(
        &mut o,
        triangle_instances[2],
        material_unset,
        anyhit_unset,
        ptr::null_mut(),
    );
    let anyhit_3 = create_anyhit_program(&mut o, ctx);
    let material_3 = create_material_with_var(&mut o, ctx, variable_key, 3f32);
    setup_geometry_instance(
        &mut o,
        triangle_instances[3],
        material_3,
        anyhit_3,
        ptr::null_mut(),
    );
    let geo_group = create_geometry_group(&o, ctx, &triangle_instances);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    o.rtContextValidate(ctx);
    launch_2d(&mut o, ctx, 0, triangles as u64, 1);
    assert_buffer_eq(&mut o, output_buffer, &[4f32, 1f32, 2f32, 3f32][..]);
    o.rtContextDestroy(ctx);
}

unsafe fn create_context<Optix: OptixFns>(o: &Optix) -> RTcontext {
    let mut ctx = ptr::null_mut();
    o.rtContextCreate(&mut ctx);
    let disable_cache = 0u32;
    o.rtContextSetAttribute(
        ctx,
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_DISK_CACHE_ENABLED,
        mem::size_of::<u32>() as u64,
        &disable_cache as *const _ as _,
    );
    ctx
}

unsafe fn create_geometry_group<Optix: OptixFns>(
    o: &Optix,
    ctx: *mut RTcontext_api,
    triangle_instances: &[*mut RTgeometryinstance_api],
) -> *mut RTgeometrygroup_api {
    let mut geo_group = ptr::null_mut();
    o.rtGeometryGroupCreate(ctx, &mut geo_group);
    o.rtGeometryGroupSetChildCount(geo_group, triangle_instances.len() as u32);
    for (idx, triangle) in triangle_instances.iter().enumerate() {
        o.rtGeometryGroupSetChild(geo_group, idx as u32, *triangle);
    }
    geo_group
}

unsafe fn create_set_accelerator<Optix: OptixFns>(
    o: &mut Optix,
    ctx: *mut RTcontext_api,
    geo_group: *mut RTgeometrygroup_api,
    name: &[u8],
) {
    let mut accel = ptr::null_mut();
    o.rtAccelerationCreate(ctx, &mut accel);
    o.rtGeometryGroupSetAcceleration(geo_group, accel);
    o.rtAccelerationSetBuilder(accel, b"Bvh\0".as_ptr() as _);
    let mut bvh_var = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, name.as_ptr() as _, &mut bvh_var);
    o.rtVariableSetObject(bvh_var, RTobject(geo_group as _));
}

unsafe fn create_set_accelerator_group<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    group: RTgroup,
    name: &[u8],
) {
    let mut accel = ptr::null_mut();
    o.rtAccelerationCreate(ctx, &mut accel);
    o.rtGroupSetAcceleration(group, accel);
    o.rtAccelerationSetBuilder(accel, b"Bvh\0".as_ptr() as _);
    let mut bvh_var = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, name.as_ptr() as _, &mut bvh_var);
    o.rtVariableSetObject(bvh_var, RTobject(group as _));
}

unsafe fn setup_geometry_instance<Optix: OptixFns>(
    o: &mut Optix,
    triangle: RTgeometryinstance,
    material: RTmaterial,
    any_hit: RTprogram,
    closest_hit: RTprogram,
) {
    o.rtGeometryInstanceSetMaterialCount(triangle, 1);
    o.rtGeometryInstanceSetMaterial(triangle, 0, material);
    if any_hit != ptr::null_mut() {
        o.rtMaterialSetAnyHitProgram(material, 0, any_hit);
    }
    if closest_hit != ptr::null_mut() {
        o.rtMaterialSetClosestHitProgram(material, 0, closest_hit);
    }
}

unsafe fn create_anyhit_program_with_var<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    variable_key: &[u8],
    value: f32,
) -> RTprogram {
    let program = create_anyhit_program(o, ctx);
    let mut program_variable = mem::zeroed();
    o.rtProgramDeclareVariable(program, variable_key.as_ptr() as _, &mut program_variable);
    o.rtVariableSet1f(program_variable, value);
    program
}

unsafe fn create_anyhit_program<Optix: OptixFns>(o: &mut Optix, ctx: RTcontext) -> RTprogram {
    let mut anyhit = mem::zeroed();
    o.rtProgramCreateFromPTXString(
        ctx,
        SET_VARIABLE_PTX.as_ptr() as _,
        b"set_variable\0".as_ptr() as _,
        &mut anyhit,
    );
    anyhit
}

unsafe fn create_material_with_var<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    variable_key: &[u8],
    value: f32,
) -> RTmaterial {
    let material = create_material(o, ctx);
    let mut program_variable = mem::zeroed();
    o.rtMaterialDeclareVariable(material, variable_key.as_ptr() as _, &mut program_variable);
    o.rtVariableSet1f(program_variable, value);
    material
}

unsafe fn create_material<Optix: OptixFns>(o: &mut Optix, ctx: RTcontext) -> RTmaterial {
    let mut material_unset = mem::zeroed();
    o.rtMaterialCreate(ctx, &mut material_unset);
    material_unset
}

unsafe fn create_triangles<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    starting_triangle: usize,
    triangle_count: usize,
    depth: f32,
) -> (RTgeometryinstance, RTgeometrytriangles) {
    create_triangles_scaled(o, ctx, starting_triangle, triangle_count, depth, 0.1)
}

unsafe fn create_triangles_scaled<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    starting_triangle: usize,
    triangle_count: usize,
    depth: f32,
    scale: f32,
) -> (RTgeometryinstance, RTgeometrytriangles) {
    let mut input_buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_INPUT.0, &mut input_buffer);
    o.rtBufferSetFormat(input_buffer, RTformat::RT_FORMAT_FLOAT3);
    o.rtBufferSetSize1D(input_buffer, (triangle_count * 3) as u64);
    {
        let mut host_ptr = ptr::null_mut();
        o.rtBufferMap(input_buffer, &mut host_ptr);
        let ptr = host_ptr as *mut [(f32, f32, f32); 3];
        for i in 0..triangle_count {
            let point = (starting_triangle + i) as f32;
            let coords = [
                (point - scale, -scale, depth),
                (point + scale, -scale, depth),
                (point, scale, depth),
            ];
            *ptr.add(i) = coords;
        }
        o.rtBufferUnmap(input_buffer);
    }
    let mut index_buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_INPUT.0, &mut index_buffer);
    o.rtBufferSetFormat(index_buffer, RTformat::RT_FORMAT_UNSIGNED_INT3);
    o.rtBufferSetSize1D(index_buffer, triangle_count as u64);
    {
        let mut host_ptr = ptr::null_mut();
        o.rtBufferMap(index_buffer, &mut host_ptr);
        let host_ptr = host_ptr as *mut u32;
        let indices = (0..(triangle_count * 3) as u32).collect::<Vec<_>>();
        ptr::copy_nonoverlapping(indices.as_ptr(), host_ptr, indices.len());
        o.rtBufferUnmap(index_buffer);
    }
    let mut geometry_triangles = ptr::null_mut();
    o.rtGeometryTrianglesCreate(ctx, &mut geometry_triangles);
    o.rtGeometryTrianglesSetPrimitiveCount(geometry_triangles, triangle_count as u32);
    o.rtGeometryTrianglesSetVertices(
        geometry_triangles,
        3 * triangle_count as u32,
        input_buffer,
        0,
        12,
        RTformat::RT_FORMAT_FLOAT3,
    );
    o.rtGeometryTrianglesSetTriangleIndices(
        geometry_triangles,
        index_buffer,
        0,
        12,
        RTformat::RT_FORMAT_UNSIGNED_INT3,
    );
    let mut geometry_instance = ptr::null_mut();
    o.rtGeometryTrianglesValidate(geometry_triangles);
    o.rtGeometryInstanceCreate(ctx, &mut geometry_instance);
    o.rtGeometryInstanceSetGeometryTriangles(geometry_instance, geometry_triangles);
    (geometry_instance, geometry_triangles)
}

pub const ANY_HIT_INTERSECT_PTX: &'static str =
    concat!(include_str!("any_hit_intersect.ptx"), "\0");

optix_test!(fail_on_multi_material_triangles);
unsafe fn fail_on_multi_material_triangles<Optix: OptixFns>(mut o: Optix) {
    let variable_key = b"b_index\0";
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let material_0 = create_material_with_var_u32(&mut o, ctx, variable_key, 0);
    let material_1 = create_material_with_var_u32(&mut o, ctx, variable_key, 1);
    let triangle = create_triangles(&mut o, ctx, 0, 1, 0.0).0;
    o.rtGeometryInstanceSetMaterialCount(triangle, 2);
    o.rtGeometryInstanceSetMaterial(triangle, 0, material_0);
    o.rtGeometryInstanceSetMaterial(triangle, 1, material_1);
    // Triangles instance: if it has multiple materials, those materials must be partitioned into
    // triangles with rtGeometryTrianglesSetMaterialIndices
    assert_eq!(
        RTresult::RT_ERROR_INVALID_CONTEXT,
        o.rtGeometryInstanceValidate_unchecked(triangle)
    );
    o.rtContextDestroy(ctx);
}

unsafe fn launch_2d<Optix: OptixFns>(
    o: &Optix,
    ctx: *mut RTcontext_api,
    entry_point: u32,
    width: u64,
    height: u64,
) {
    let error = o.rtContextLaunch2D_unchecked(ctx, entry_point, width, height);
    if error != RTresult::RT_SUCCESS {
        let mut err_string = ptr::null();
        o.rtContextGetErrorString(ctx, error, &mut err_string);
        panic!(
            "{:?} {}",
            error,
            CStr::from_ptr(err_string).to_str().unwrap()
        );
    }
}

unsafe fn assert_buffer_eq<T: Copy + Default + PartialEq + std::fmt::Debug, Optix: OptixFns>(
    o: &Optix,
    output_buffer: *mut RTbuffer_api,
    buff: &[T],
) {
    let mut host_ptr = ptr::null_mut();
    let mut result = vec![T::default(); buff.len()];
    o.rtBufferMap(output_buffer, &mut host_ptr);
    ptr::copy_nonoverlapping::<T>(host_ptr as *const T, result.as_mut_ptr(), buff.len());
    o.rtBufferUnmap(output_buffer);
    assert_eq!(&*result, buff);
}

unsafe fn assert_buffer_eq_float<Optix: OptixFns>(
    o: &Optix,
    epsilon: f32,
    output_buffer: *mut RTbuffer_api,
    buff: &[f32],
) {
    let mut host_ptr = ptr::null_mut();
    let mut result = vec![0f32; buff.len()];
    o.rtBufferMap(output_buffer, &mut host_ptr);
    ptr::copy_nonoverlapping::<f32>(host_ptr as *const f32, result.as_mut_ptr(), buff.len());
    o.rtBufferUnmap(output_buffer);
    assert_approx_eq!(&[f32], buff, &*result, epsilon = epsilon);
}

unsafe fn create_material_with_var_u32<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    variable_key: &[u8],
    value: u32,
) -> RTmaterial {
    let material = create_material(o, ctx);
    let mut program_variable = mem::zeroed();
    o.rtMaterialDeclareVariable(material, variable_key.as_ptr() as _, &mut program_variable);
    o.rtVariableSet1ui(program_variable, value);
    material
}

optix_test!(any_hit_multiple_materials);
unsafe fn any_hit_multiple_materials<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 2);
    let mut sphere = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, b"sphere\0".as_ptr() as _, &mut sphere);
    o.rtVariableSet4f(sphere, 0.0, 0.0, 0.0, 100.0);
    let output_buffer = create_buffer_u32(&mut o, ctx, "output_buffer", 2);
    let output_buffer2 = create_buffer_u32(&mut o, ctx, "output_buffer2", 2);
    let raygen = create_program(&mut o, ctx, ANY_HIT_INTERSECT_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, raygen);
    let any_hit = create_program(&mut o, ctx, ANY_HIT_INTERSECT_PTX, "set_buffer\0");
    let closest_hit = create_program(&mut o, ctx, ANY_HIT_INTERSECT_PTX, "set_buffer2\0");
    let material_0 = create_material_with_var_u32(&mut o, ctx, b"b_index\0", 0);
    let material_1 = create_material_with_var_u32(&mut o, ctx, b"b_index\0", 1);
    let bb_prog = create_program(&mut o, ctx, ANY_HIT_INTERSECT_PTX, "bounds\0");
    let intersect_prog = create_program(&mut o, ctx, ANY_HIT_INTERSECT_PTX, "intersect\0");
    let sphere = create_custom_geometry(&mut o, ctx, 1, bb_prog, intersect_prog);
    o.rtGeometryInstanceSetMaterialCount(sphere, 2);
    o.rtGeometryInstanceSetMaterial(sphere, 0, material_0);
    o.rtGeometryInstanceSetMaterial(sphere, 1, material_1);
    o.rtMaterialSetAnyHitProgram(material_0, 0, any_hit);
    o.rtMaterialSetAnyHitProgram(material_1, 0, any_hit);
    o.rtMaterialSetClosestHitProgram(material_0, 0, closest_hit);
    o.rtMaterialSetClosestHitProgram(material_1, 0, closest_hit);
    let geo_group = create_geometry_group(&o, ctx, &[sphere]);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    launch_2d(&mut o, ctx, 0, 2, 1);
    assert_buffer_eq(&mut o, output_buffer, &[1u32, 1][..]);
    assert_buffer_eq(&mut o, output_buffer2, &[1u32, 1][..]);
    o.rtContextDestroy(ctx);
}

unsafe fn create_custom_geometry<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    primitives: u32,
    bb_program: RTprogram,
    intersect_program: RTprogram,
) -> RTgeometryinstance {
    let mut geometry = ptr::null_mut();
    o.rtGeometryCreate(ctx, &mut geometry);
    o.rtGeometrySetPrimitiveCount(geometry, primitives);
    o.rtGeometrySetBoundingBoxProgram(geometry, bb_program);
    o.rtGeometrySetIntersectionProgram(geometry, intersect_program);
    let mut geometry_instance = ptr::null_mut();
    o.rtGeometryInstanceCreate(ctx, &mut geometry_instance);
    o.rtGeometryInstanceSetGeometry(geometry_instance, geometry);
    geometry_instance
}

unsafe fn create_program<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    text: &str,
    name: &str,
) -> RTprogram {
    let mut program = mem::zeroed();
    o.rtProgramCreateFromPTXString(ctx, text.as_ptr() as _, name.as_ptr() as _, &mut program);
    program
}

unsafe fn create_buffer_u32<Optix: OptixFns>(
    o: &mut Optix,
    ctx: *mut RTcontext_api,
    name: &str,
    len: usize,
) -> RTbuffer {
    let mut output_buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_OUTPUT.0, &mut output_buffer);
    o.rtBufferSetSize1D(output_buffer, len as u64);
    o.rtBufferSetFormat(output_buffer, RTformat::RT_FORMAT_UNSIGNED_INT);
    let mut host_ptr = ptr::null_mut();
    o.rtBufferMap(output_buffer, &mut host_ptr);
    ptr::write_bytes(host_ptr as *mut u32, 0, len);
    o.rtBufferUnmap(output_buffer);
    let mut output_buffer_var = ptr::null_mut();
    let mut name = name.to_string();
    name.push('\0');
    o.rtContextDeclareVariable(ctx, name.as_ptr() as _, &mut output_buffer_var);
    o.rtVariableSetObject(output_buffer_var, RTobject(output_buffer as _));
    output_buffer
}

unsafe fn create_buffer_u32_with_values<Optix: OptixFns>(
    o: &mut Optix,
    ctx: *mut RTcontext_api,
    name: &str,
    values: &[u32],
) -> RTbuffer {
    let mut output_buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_INPUT_OUTPUT.0, &mut output_buffer);
    o.rtBufferSetSize1D(output_buffer, values.len() as u64);
    o.rtBufferSetFormat(output_buffer, RTformat::RT_FORMAT_USER);
    o.rtBufferSetElementSize(output_buffer, 4);
    let mut host_ptr: *mut std::ffi::c_void = ptr::null_mut();
    o.rtBufferMap(output_buffer, &mut host_ptr);
    ptr::copy_nonoverlapping(values.as_ptr(), host_ptr.cast::<u32>(), values.len());
    o.rtBufferUnmap(output_buffer);
    let mut output_buffer_var = ptr::null_mut();
    let mut name = name.to_string();
    name.push('\0');
    o.rtContextDeclareVariable(ctx, name.as_ptr() as _, &mut output_buffer_var);
    o.rtVariableSetObject(output_buffer_var, RTobject(output_buffer as _));
    output_buffer
}

const CALLABLE_PROGRAMS_PTX: &'static str = concat!(include_str!("callable_programs.ptx"), "\0");

optix_test!(callable_programs);
unsafe fn callable_programs<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, CALLABLE_PROGRAMS_PTX, "start\0");
    let add_value = create_program(&mut o, ctx, CALLABLE_PROGRAMS_PTX, "add_value\0");
    let multiply_value = create_program(&mut o, ctx, CALLABLE_PROGRAMS_PTX, "multiply_value\0");
    let variable_name = b"value\0";
    context_set_u32(&mut o, ctx, variable_name, 1);
    program_set_u32(&mut o, add_value, variable_name, 2);
    program_set_u32(&mut o, multiply_value, variable_name, 2);
    let add_fn_id = program_get_id(&mut o, add_value);
    let multiply_fn_id = program_get_id(&mut o, multiply_value);
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    context_set_u32(&mut o, ctx, b"add_fn\0", add_fn_id as u32);
    context_set_u32(&mut o, ctx, b"mult_fn\0", multiply_fn_id as u32);
    let output_buffer = create_buffer_u32(&mut o, ctx, "output_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, output_buffer, &[6u32]);
    o.rtContextDestroy(ctx);
}

unsafe fn program_get_id<Optix: OptixFns>(o: &mut Optix, program: *mut RTprogram_api) -> i32 {
    let mut prog_id = 0;
    o.rtProgramGetId(program, &mut prog_id);
    prog_id
}

unsafe fn context_set_u32<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    variable_name: &[u8],
    value: u32,
) -> RTvariable {
    let mut variable = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, variable_name.as_ptr() as _, &mut variable);
    o.rtVariableSet1ui(variable, value);
    variable
}

unsafe fn program_set_u32<Optix: OptixFns>(
    o: &mut Optix,
    program: RTprogram,
    variable_name: &[u8],
    value: u32,
) -> RTvariable {
    let mut variable = ptr::null_mut();
    o.rtProgramDeclareVariable(program, variable_name.as_ptr() as _, &mut variable);
    o.rtVariableSet1ui(variable, value);
    variable
}

const TEXTURE_SAMPLER_PTX: &'static str = concat!(include_str!("texture_sampler.ptx"), "\0");

optix_test!(texture_sampler);
unsafe fn texture_sampler<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, TEXTURE_SAMPLER_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let image1 = create_image(
        &mut o,
        ctx,
        &[
            (0.0, 0.25, 0.5, 0.75),
            (1.0, 1.25, 1.5, 1.75),
            (2.0, 2.25, 2.5, 2.75),
            (3.0, 3.25, 3.5, 3.75),
        ],
        2,
    );
    let image2 = create_image(
        &mut o,
        ctx,
        &[
            (4.0, 4.25, 4.5, 4.75),
            (5.0, 5.25, 5.5, 5.75),
            (6.0, 6.25, 6.5, 6.75),
            (7.0, 7.25, 7.5, 7.75),
        ],
        2,
    );
    let output_buffer = create_buffer_2d(
        &mut o,
        ctx,
        RTformat::RT_FORMAT_FLOAT4,
        "output_buffer",
        (2, 4),
    );
    context_set_object(&mut o, ctx, b"image1\0", image1);
    let mut image2_id = 0;
    o.rtTextureSamplerGetId(image2, &mut image2_id);
    context_set_u32(&mut o, ctx, b"image2\0", image2_id as u32);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq::<(f32, f32, f32, f32), _>(
        &o,
        output_buffer,
        &[
            (0.0, 0.25, 0.5, 0.75),
            (1.0, 1.25, 1.5, 1.75),
            (2.0, 2.25, 2.5, 2.75),
            (3.0, 3.25, 3.5, 3.75),
            (4.0, 4.25, 4.5, 4.75),
            (5.0, 5.25, 5.5, 5.75),
            (6.0, 6.25, 6.5, 6.75),
            (7.0, 7.25, 7.5, 7.75),
        ],
    );
    o.rtContextDestroy(ctx);
}

unsafe fn context_set_object<T, Optix: OptixFns>(
    o: &mut Optix,
    ctx: *mut RTcontext_api,
    variable_name: &[u8],
    value: *mut T,
) -> RTvariable {
    let mut variable = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, variable_name.as_ptr() as _, &mut variable);
    o.rtVariableSetObject(variable, RTobject(value as _));
    variable
}

unsafe fn create_image<Optix: OptixFns>(
    o: &mut Optix,
    ctx: *mut RTcontext_api,
    content: &[(f32, f32, f32, f32)],
    width: u32,
) -> RTtexturesampler {
    let mut image = ptr::null_mut();
    o.rtTextureSamplerCreate(ctx, &mut image);
    let mut buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_INPUT.0, &mut buffer);
    o.rtBufferSetSize2D(buffer, width as u64, (content.len() as u32 / width) as u64);
    o.rtBufferSetFormat(buffer, RTformat::RT_FORMAT_FLOAT4);
    let mut buffer_pointer = ptr::null_mut();
    o.rtBufferMap(buffer, &mut buffer_pointer);
    ptr::copy_nonoverlapping(
        content.as_ptr(),
        buffer_pointer as *mut (f32, f32, f32, f32),
        content.len(),
    );
    o.rtBufferUnmap(buffer);
    o.rtTextureSamplerSetWrapMode(image, 0, RTwrapmode::RT_WRAP_REPEAT);
    o.rtTextureSamplerSetWrapMode(image, 1, RTwrapmode::RT_WRAP_REPEAT);
    o.rtTextureSamplerSetFilteringModes(
        image,
        RTfiltermode::RT_FILTER_NEAREST,
        RTfiltermode::RT_FILTER_NEAREST,
        RTfiltermode::RT_FILTER_NONE,
    );
    o.rtTextureSamplerSetIndexingMode(image, RTtextureindexmode::RT_TEXTURE_INDEX_ARRAY_INDEX);
    o.rtTextureSamplerSetReadMode(image, RTtexturereadmode::RT_TEXTURE_READ_NORMALIZED_FLOAT);
    o.rtTextureSamplerSetMaxAnisotropy(image, 1.0f32);
    o.rtTextureSamplerSetBuffer(image, 0, 0, buffer);
    image
}

unsafe fn create_buffer_1d<Optix: OptixFns>(
    o: &mut Optix,
    ctx: *mut RTcontext_api,
    format: RTformat,
    name: &str,
    width: u64,
) -> RTbuffer {
    let mut output_buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_OUTPUT.0, &mut output_buffer);
    o.rtBufferSetSize1D(output_buffer, width);
    o.rtBufferSetFormat(output_buffer, format);
    let mut output_buffer_var = ptr::null_mut();
    let mut name = name.to_string();
    name.push('\0');
    o.rtContextDeclareVariable(ctx, name.as_ptr() as _, &mut output_buffer_var);
    o.rtVariableSetObject(output_buffer_var, RTobject(output_buffer as _));
    output_buffer
}

unsafe fn create_buffer_2d<Optix: OptixFns>(
    o: &mut Optix,
    ctx: *mut RTcontext_api,
    format: RTformat,
    name: &str,
    (width, height): (u64, u64),
) -> RTbuffer {
    let mut output_buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_OUTPUT.0, &mut output_buffer);
    o.rtBufferSetSize2D(output_buffer, width, height);
    o.rtBufferSetFormat(output_buffer, format);
    let mut output_buffer_var = ptr::null_mut();
    let mut name = name.to_string();
    name.push('\0');
    o.rtContextDeclareVariable(ctx, name.as_ptr() as _, &mut output_buffer_var);
    o.rtVariableSetObject(output_buffer_var, RTobject(output_buffer as _));
    output_buffer
}

const BARYCENTRICS_PTX: &'static str = concat!(include_str!("barycentrics.ptx"), "\0");

optix_test!(barycentrics);
unsafe fn barycentrics<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, BARYCENTRICS_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let attribute_program = create_program(&mut o, ctx, BARYCENTRICS_PTX, "attribute_program\0");
    let closest_hit = create_program(&mut o, ctx, BARYCENTRICS_PTX, "closest_hit\0");
    let bounds = create_program(&mut o, ctx, BARYCENTRICS_PTX, "bounds\0");
    let intersect = create_program(&mut o, ctx, BARYCENTRICS_PTX, "intersect\0");
    let mut sphere = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, b"sphere\0".as_ptr() as _, &mut sphere);
    o.rtVariableSet4f(sphere, 0.0, 0.0, 0.0, 100.0);
    let output_buffer1 =
        create_buffer(&mut o, ctx, RTformat::RT_FORMAT_FLOAT2, "output_buffer1", 4);
    create_buffer(&mut o, ctx, RTformat::RT_FORMAT_FLOAT2, "output_buffer2", 4);
    let output_buffer3 = create_buffer(
        &mut o,
        ctx,
        RTformat::RT_FORMAT_UNSIGNED_INT,
        "output_buffer3",
        4,
    );
    let sphere = create_custom_geometry(&mut o, ctx, 1, bounds, intersect);
    let (triangles_instance, triangles) = create_triangles(&mut o, ctx, 0, 2, 0.0);
    let (triangles_instance2, _) = create_triangles(&mut o, ctx, 2, 1, 0.0);
    o.rtGeometryTrianglesSetAttributeProgram(triangles, attribute_program);
    let material = create_material(&mut o, ctx);
    setup_geometry_instance(
        &mut o,
        triangles_instance,
        material,
        ptr::null_mut(),
        closest_hit,
    );
    setup_geometry_instance(
        &mut o,
        triangles_instance2,
        material,
        ptr::null_mut(),
        closest_hit,
    );
    setup_geometry_instance(&mut o, sphere, material, ptr::null_mut(), closest_hit);
    let mut group = ptr::null_mut();
    let triangles_group =
        create_geometry_group(&o, ctx, &[triangles_instance, triangles_instance2]);
    create_set_accelerator(&mut o, ctx, triangles_group, b"unused1\0");
    let sphere_group = create_geometry_group(&o, ctx, &[sphere]);
    create_set_accelerator(&mut o, ctx, sphere_group, b"unused2\0");
    o.rtGroupCreate(ctx, &mut group);
    o.rtGroupSetChildCount(group, 2);
    o.rtGroupSetChild(group, 0, RTobject(triangles_group as _));
    o.rtGroupSetChild(group, 1, RTobject(sphere_group as _));
    create_set_accelerator_group(&mut o, ctx, group, b"bvh\0");
    launch_2d(&mut o, ctx, 0, 4, 1);
    assert_buffer_eq_float(
        &o,
        0.000001,
        output_buffer1,
        &[
            0.25f32, 0.1f32, 0.25f32, 0.1f32, 0.25f32, 0.5f32, 100f32, 200f32,
        ],
    );
    /*
    assert_buffer_eq_float(
        &o,
        0.000001,
        output_buffer2,
        &[
            0.25f32, 0.5f32, 0.25f32, 0.5f32, 0.25f32, 0.5f32, 100f32, 200f32,
        ],
    );
    */
    assert_buffer_eq(&o, output_buffer3, &[0u32, 1u32, 0u32, 0u32]);
}

unsafe fn create_buffer<Optix: OptixFns>(
    o: &mut Optix,
    ctx: *mut RTcontext_api,
    format: RTformat,
    name: &str,
    len: usize,
) -> RTbuffer {
    let mut output_buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_OUTPUT.0, &mut output_buffer);
    o.rtBufferSetSize1D(output_buffer, len as u64);
    o.rtBufferSetFormat(output_buffer, format);
    let mut output_buffer_var = ptr::null_mut();
    let mut name = name.to_string();
    name.push('\0');
    o.rtContextDeclareVariable(ctx, name.as_ptr() as _, &mut output_buffer_var);
    o.rtVariableSetObject(output_buffer_var, RTobject(output_buffer as _));
    output_buffer
}

const TRACE_CONTROL_PTX: &'static str = concat!(include_str!("trace_control.ptx"), "\0");

optix_test!(ignore_intersection);
unsafe fn ignore_intersection<Optix: OptixFns>(o: Optix) {
    trace_control(o, "any_hit_ignore\0", 1)
}

optix_test!(terminate_ray);
unsafe fn terminate_ray<Optix: OptixFns>(o: Optix) {
    trace_control(o, "any_hit_terminate\0", 2)
}

unsafe fn trace_control<Optix: OptixFns>(mut o: Optix, any_hit: &str, result: u32) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let mut sphere = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, b"sphere\0".as_ptr() as _, &mut sphere);
    o.rtVariableSet4f(sphere, 0.0, 0.0, 0.0, 100.0);
    let start = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let bounds = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "bounds\0");
    let intersect = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "intersect\0");
    let any_hit = create_program(&mut o, ctx, TRACE_CONTROL_PTX, any_hit);
    let closest_hit = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "closest_hit\0");
    let output_buffer = create_buffer_u32(&mut o, ctx, "output_buffer", 1);
    create_buffer_u32(&mut o, ctx, "temp_buffer", 1);
    let sphere1 = create_custom_geometry(&mut o, ctx, 1, bounds, intersect);
    let sphere2 = create_custom_geometry(&mut o, ctx, 1, bounds, intersect);
    let sphere3 = create_custom_geometry(&mut o, ctx, 1, bounds, intersect);
    let sphere4 = create_custom_geometry(&mut o, ctx, 1, bounds, intersect);
    let material = create_material(&mut o, ctx);
    setup_geometry_instance(&mut o, sphere1, material, any_hit, closest_hit);
    setup_geometry_instance(&mut o, sphere2, material, any_hit, closest_hit);
    setup_geometry_instance(&mut o, sphere3, material, any_hit, closest_hit);
    setup_geometry_instance(&mut o, sphere4, material, any_hit, closest_hit);
    let geo_group = create_geometry_group(&o, ctx, &[sphere1, sphere2, sphere3, sphere4]);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, output_buffer, &[result]);
}

optix_test!(attribute_program_runs_before_closest_hit);
unsafe fn attribute_program_runs_before_closest_hit<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let attribute_program = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "attribute1\0");
    let any_hit = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "any_hit_plus_one\0");
    let closest_hit = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "closest_hit\0");
    let output_buffer = create_buffer_u32(&mut o, ctx, "output_buffer", 1);
    let temp_buffer = create_buffer_u32(&mut o, ctx, "temp_buffer", 1);
    let material = create_material(&mut o, ctx);
    let (triangle1_instance, triangle1) = create_triangles(&mut o, ctx, 0, 1, -0.1f32);
    o.rtGeometryTrianglesSetAttributeProgram(triangle1, attribute_program);
    setup_geometry_instance(&mut o, triangle1_instance, material, any_hit, closest_hit);
    let geo_group = create_geometry_group(&o, ctx, &[triangle1_instance]);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, output_buffer, &[0xc4bb2187u32 + 1]);
    assert_buffer_eq(&o, temp_buffer, &[1]);
}

optix_test!(rollback_attributes_on_ignore);
unsafe fn rollback_attributes_on_ignore<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let attribute_program = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "attribute2\0");
    let any_hit = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "any_hit_always_ignore\0");
    let closest_hit = create_program(&mut o, ctx, TRACE_CONTROL_PTX, "closest_hit\0");
    let output_buffer = create_buffer_u32(&mut o, ctx, "output_buffer", 1);
    let temp_buffer = create_buffer_u32(&mut o, ctx, "temp_buffer", 1);
    let material = create_material(&mut o, ctx);
    let (triangle1_instance, triangle1) = create_triangles(&mut o, ctx, 0, 1, -0.1f32);
    o.rtGeometryTrianglesSetAttributeProgram(triangle1, attribute_program);
    setup_geometry_instance(&mut o, triangle1_instance, material, any_hit, closest_hit);
    let geo_group = create_geometry_group(&o, ctx, &[triangle1_instance]);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_neq(&o, output_buffer, 0xc4bb2187u32);
    assert_buffer_eq(&o, temp_buffer, &[1]);
}

unsafe fn assert_buffer_neq<T: Copy + Default + PartialEq + std::fmt::Debug, Optix: OptixFns>(
    o: &Optix,
    output_buffer: *mut RTbuffer_api,
    value: T,
) {
    let mut host_ptr = ptr::null_mut();
    let mut result = vec![T::default(); 1];
    o.rtBufferMap(output_buffer, &mut host_ptr);
    ptr::copy_nonoverlapping::<T>(host_ptr as *const T, result.as_mut_ptr(), 1);
    o.rtBufferUnmap(output_buffer);
    assert_ne!(value, result[0]);
}

const BUFFER_ID_PTX: &'static str = concat!(include_str!("buffer_id.ptx"), "\0");
const BUFFER_ID_CALL_PTX: &'static str = concat!(include_str!("buffer_id_call.ptx"), "\0");

optix_test!(buffer_id);
optix_test!(buffer_id_call);

unsafe fn buffer_id<Optix: OptixFns>(o: Optix) {
    buffer_id_impl(o, BUFFER_ID_PTX)
}
unsafe fn buffer_id_call<Optix: OptixFns>(o: Optix) {
    buffer_id_impl(o, BUFFER_ID_CALL_PTX)
}

unsafe fn buffer_id_impl<Optix: OptixFns>(mut o: Optix, text: &str) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, text, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let buffer = create_buffer_u32(&mut o, ctx, "output_buffer", 3);
    create_buffer_of_buffers(&mut o, ctx, "buffers", &[buffer][..]);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[0, 3, 0x0118378c]);
}

unsafe fn create_buffer_of_buffers<Optix: OptixFns>(
    o: &mut Optix,
    ctx: RTcontext,
    name: &str,
    buffer: &[RTbuffer],
) {
    let sub_buffers = buffer
        .iter()
        .copied()
        .map(|buffer| {
            let mut buffer_id = 0;
            unsafe { o.rtBufferGetId(buffer, &mut buffer_id) };
            buffer_id
        })
        .collect::<Vec<_>>();
    let mut main_buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_INPUT.0, &mut main_buffer);
    o.rtBufferSetSize1D(main_buffer, sub_buffers.len() as u64);
    o.rtBufferSetFormat(main_buffer, RTformat::RT_FORMAT_BUFFER_ID);
    {
        let mut host_ptr = ptr::null_mut();
        o.rtBufferMap(main_buffer, &mut host_ptr);
        ptr::copy_nonoverlapping(sub_buffers.as_ptr(), host_ptr as _, sub_buffers.len());
        o.rtBufferUnmap(main_buffer);
    }
    let mut main_buffer_var = ptr::null_mut();
    let mut name = name.to_string();
    name.push('\0');
    o.rtContextDeclareVariable(ctx, name.as_ptr() as _, &mut main_buffer_var);
    o.rtVariableSetObject(main_buffer_var, RTobject(main_buffer as _));
}

const BUFFER_ID_CALLABLE_PTX: &'static str = concat!(include_str!("buffer_id_callable.ptx"), "\0");

optix_test!(buffer_id_callable);

unsafe fn buffer_id_callable<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, BUFFER_ID_CALLABLE_PTX, "start\0");
    let callable = create_program(&mut o, ctx, BUFFER_ID_CALLABLE_PTX, "callable\0");
    let callable_id = program_get_id(&mut o, callable);
    context_set_u32(&mut o, ctx, b"program\0", callable_id as u32);
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let buffer = create_buffer_u32(&mut o, ctx, "output_buffer", 3);
    create_buffer_of_buffers(&mut o, ctx, "buffers", &[buffer][..]);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[0, 3, 0x0118378c]);
}

const TRIANGLE_FRONT_PTX: &'static str = concat!(include_str!("triangle_front.ptx"), "\0");

optix_test!(triangle_front);
unsafe fn triangle_front<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, TRIANGLE_FRONT_PTX, "start\0");
    let bounds = create_program(&mut o, ctx, TRIANGLE_FRONT_PTX, "bounds\0");
    let intersect = create_program(&mut o, ctx, TRIANGLE_FRONT_PTX, "intersect\0");
    let closest_hit = create_program(&mut o, ctx, TRIANGLE_FRONT_PTX, "closest_hit\0");
    let mut sphere = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, b"sphere\0".as_ptr() as _, &mut sphere);
    o.rtVariableSet4f(sphere, 0.0, 0.0, 0.0, 100.0);
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let output_buffer1 = create_buffer_u32(&mut o, ctx, "output_buffer1", 3);
    let output_buffer2 = create_buffer_u32(&mut o, ctx, "output_buffer2", 3);
    let output_buffer3 = create_buffer_u32(&mut o, ctx, "output_buffer3", 3);
    let sphere = create_custom_geometry(&mut o, ctx, 1, bounds, intersect);
    let (triangles_instance1, _) = create_triangles(&mut o, ctx, 0, 1, 0.0);
    let (triangles_instance2, _) = create_triangles(&mut o, ctx, 1, 1, 0.0);
    let material = create_material(&mut o, ctx);
    setup_geometry_instance(&mut o, sphere, material, ptr::null_mut(), closest_hit);
    setup_geometry_instance(
        &mut o,
        triangles_instance1,
        material,
        ptr::null_mut(),
        closest_hit,
    );
    setup_geometry_instance(
        &mut o,
        triangles_instance2,
        material,
        ptr::null_mut(),
        closest_hit,
    );
    let mut group = ptr::null_mut();
    let triangles_group =
        create_geometry_group(&o, ctx, &[triangles_instance1, triangles_instance2]);
    create_set_accelerator(&mut o, ctx, triangles_group, b"unused1\0");
    let sphere_group = create_geometry_group(&o, ctx, &[sphere]);
    create_set_accelerator(&mut o, ctx, sphere_group, b"unused2\0");
    o.rtGroupCreate(ctx, &mut group);
    o.rtGroupSetChildCount(group, 2);
    o.rtGroupSetChild(group, 0, RTobject(triangles_group as _));
    o.rtGroupSetChild(group, 1, RTobject(sphere_group as _));
    create_set_accelerator_group(&mut o, ctx, group, b"bvh\0");
    launch_2d(&mut o, ctx, 0, 3, 1);
    assert_buffer_eq(&o, output_buffer1, &[2, 2, 1]);
    assert_buffer_eq(&o, output_buffer2, &[2, 1, 1]);
    assert_buffer_eq(&o, output_buffer3, &[1, 2, 1]);
}

const TRANSFORM_PTX: &'static str = concat!(include_str!("transform.ptx"), "\0");

optix_test!(ray_transform);
unsafe fn ray_transform<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, TRANSFORM_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let bounds = create_program(&mut o, ctx, TRANSFORM_PTX, "bounds\0");
    let intersect = create_program(&mut o, ctx, TRANSFORM_PTX, "intersect\0");
    let attribute = create_program(&mut o, ctx, TRANSFORM_PTX, "attribute\0");
    let any_hit = create_program(&mut o, ctx, TRANSFORM_PTX, "any_hit\0");
    let closest_hit = create_program(&mut o, ctx, TRANSFORM_PTX, "closest_hit\0");
    let is_triangle = create_buffer_u32(&mut o, ctx, "is_triangle", 2);
    let origin0 = create_buffer_1d(&mut o, ctx, RTformat::RT_FORMAT_FLOAT3, "origin0", 3);
    let origin1 = create_buffer_1d(&mut o, ctx, RTformat::RT_FORMAT_FLOAT3, "origin1", 3);
    create_buffer_of_buffers(&mut o, ctx, "origin", &[origin0, origin1]);
    let direction0 = create_buffer_1d(&mut o, ctx, RTformat::RT_FORMAT_FLOAT3, "direction0", 3);
    let direction1 = create_buffer_1d(&mut o, ctx, RTformat::RT_FORMAT_FLOAT3, "direction1", 3);
    create_buffer_of_buffers(&mut o, ctx, "direction", &[direction0, direction1]);
    let mut sphere = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, b"sphere\0".as_ptr() as _, &mut sphere);
    o.rtVariableSet4f(sphere, 0.0, 0.0, 0.0, 100.0);
    let sphere = create_custom_geometry(&mut o, ctx, 1, bounds, intersect);
    let (triangles, triangles_primitive) = create_triangles_scaled(&mut o, ctx, 1, 1, 0.0, 100.0);
    o.rtGeometryTrianglesSetAttributeProgram(triangles_primitive, attribute);
    let material = create_material(&mut o, ctx);
    setup_geometry_instance(&mut o, sphere, material, any_hit, closest_hit);
    setup_geometry_instance(&mut o, triangles, material, any_hit, closest_hit);
    let sphere_group = create_geometry_group(&o, ctx, &[sphere]);
    let triangles_group = create_geometry_group(&o, ctx, &[triangles]);
    create_set_accelerator(&mut o, ctx, sphere_group, b"unused1\0");
    create_set_accelerator(&mut o, ctx, triangles_group, b"unused2\0");
    let transform1 = create_test_transform(&mut o, ctx);
    let transform2 = create_test_transform(&mut o, ctx);
    o.rtTransformSetChild(transform1, RTobject(sphere_group as _));
    o.rtTransformSetChild(transform2, RTobject(triangles_group as _));
    let mut group = ptr::null_mut();
    o.rtGroupCreate(ctx, &mut group);
    o.rtGroupSetChildCount(group, 2);
    o.rtGroupSetChild(group, 0, RTobject(transform1 as _));
    o.rtGroupSetChild(group, 1, RTobject(transform2 as _));
    create_set_accelerator_group(&mut o, ctx, group, b"bvh\0");
    launch_2d(&mut o, ctx, 0, 2, 1);
    assert_buffer_eq(&o, is_triangle, &[1, 0]);
    assert_buffer_eq_float(
        &mut o,
        0.000001f32,
        origin0,
        &[
            0.0, 0.0, -1.0, -0.3333333, -0.7440169, -0.3035612, 0.0, 0.0, -1.0,
        ],
    );
    assert_buffer_eq_float(
        &mut o,
        0.000001f32,
        origin1,
        &[0.0, -0.86602545, 0.0, 0.0, -0.86602545, 0.0, 1.0, 0.0, -1.0],
    );
    assert_buffer_eq_float(
        &mut o,
        0.000001f32,
        direction0,
        &[
            0.0,
            0.0,
            1.0,
            -0.24401696,
            0.4553418,
            0.11111113,
            0.0,
            0.0,
            1.0,
        ],
    );
    assert_buffer_eq_float(
        &mut o,
        0.000001f32,
        direction1,
        &[
            -0.24401696,
            0.4553418,
            0.11111113,
            -0.24401696,
            0.4553418,
            0.11111113,
            0.0,
            0.0,
            1.0,
        ],
    );
}

unsafe fn create_test_transform<Optix: OptixFns>(o: &mut Optix, ctx: RTcontext) -> RTtransform {
    let mut transform = ptr::null_mut();
    o.rtTransformCreate(ctx, &mut transform);
    let matrix = Mat4::from_scale_rotation_translation(
        Vec3::from_array([1f32, 2f32, 3f32]),
        Quat::from_axis_angle(
            Vec3::from_array([1., 1., 1.]).normalize(),
            (std::f64::consts::PI / 2f64) as f32,
        ),
        Vec3::from_array([1., 1., 1.]).normalize(),
    );
    o.rtTransformSetMatrix(
        transform,
        1,
        matrix.to_cols_array().as_ptr(),
        ptr::null_mut(),
    );
    transform
}

const GET_TRANSFORM_PTX: &'static str = concat!(include_str!("get_transform.ptx"), "\0");

optix_test!(get_transform);
unsafe fn get_transform<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, GET_TRANSFORM_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let bounds = create_program(&mut o, ctx, GET_TRANSFORM_PTX, "bounds\0");
    let intersect = create_program(&mut o, ctx, GET_TRANSFORM_PTX, "intersect\0");
    let any_hit = create_program(&mut o, ctx, GET_TRANSFORM_PTX, "any_hit\0");
    let closest_hit = create_program(&mut o, ctx, GET_TRANSFORM_PTX, "closest_hit\0");
    let object_transforms = create_buffer_1d(
        &mut o,
        ctx,
        RTformat::RT_FORMAT_FLOAT,
        "object_transforms",
        16 * 3,
    );
    let mut sphere = ptr::null_mut();
    o.rtContextDeclareVariable(ctx, b"sphere\0".as_ptr() as _, &mut sphere);
    o.rtVariableSet4f(sphere, 0.0, 0.0, 0.0, 100.0);
    let sphere = create_custom_geometry(&mut o, ctx, 1, bounds, intersect);
    let material = create_material(&mut o, ctx);
    setup_geometry_instance(&mut o, sphere, material, any_hit, closest_hit);
    let sphere_group = create_geometry_group(&o, ctx, &[sphere]);
    create_set_accelerator(&mut o, ctx, sphere_group, b"unused1\0");
    let transform1 = create_test_transform(&mut o, ctx);
    o.rtTransformSetChild(transform1, RTobject(sphere_group as _));
    let mut group = ptr::null_mut();
    o.rtGroupCreate(ctx, &mut group);
    o.rtGroupSetChildCount(group, 1);
    o.rtGroupSetChild(group, 0, RTobject(transform1 as _));
    create_set_accelerator_group(&mut o, ctx, group, b"bvh\0");
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq_float(
        &mut o,
        0.000001f32,
        object_transforms,
        &[
            0.33333337,
            -0.4880339,
            2.7320507,
            0.57735026,
            0.9106836,
            0.66666675,
            -0.73205084,
            0.57735026,
            -0.24401695,
            1.8213671,
            1.0000001,
            0.57735026,
            0.0,
            0.0,
            0.0,
            1.0,
            0.33333337,
            -0.4880339,
            2.7320507,
            0.57735026,
            0.9106836,
            0.66666675,
            -0.73205084,
            0.57735026,
            -0.24401695,
            1.8213671,
            1.0000001,
            0.57735026,
            0.0,
            0.0,
            0.0,
            1.0,
            0.33333337,
            0.9106836,
            -0.24401696,
            -0.57735026,
            -0.12200849,
            0.16666669,
            0.4553418,
            -0.28867513,
            0.3035612,
            -0.08133899,
            0.11111113,
            -0.19245009,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    );
}

const DEFAULT_VARIABLE_PTX: &'static str = concat!(include_str!("default_variable.ptx"), "\0");

optix_test!(default_variable);

unsafe fn default_variable<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, DEFAULT_VARIABLE_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let buffer = create_buffer_u32(&mut o, ctx, "var_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[55]);
}

const EXCEPTION_PTX: &'static str = concat!(include_str!("exception.ptx"), "\0");

optix_test!(exception_raygen);

unsafe fn exception_raygen<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetExceptionEnabled(ctx, optix_types::RTexception::RT_EXCEPTION_ALL, 1);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, EXCEPTION_PTX, "throw_\0");
    let exception = create_program(&mut o, ctx, EXCEPTION_PTX, "exception\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    o.rtContextSetExceptionProgram(ctx, 0, exception);
    let buffer = create_buffer_u32(&mut o, ctx, "var_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[1024]);
}

optix_test!(exception_miss);

unsafe fn exception_miss<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetExceptionEnabled(ctx, optix_types::RTexception::RT_EXCEPTION_ALL, 1);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let trace = create_program(&mut o, ctx, EXCEPTION_PTX, "trace\0");
    let exception = create_program(&mut o, ctx, EXCEPTION_PTX, "exception\0");
    let throw_ = create_program(&mut o, ctx, EXCEPTION_PTX, "throw_\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, trace);
    o.rtContextSetExceptionProgram(ctx, 0, exception);
    o.rtContextSetMissProgram(ctx, 0, throw_);
    let material = create_material(&mut o, ctx);
    let (triangle, _) = create_triangles(&mut o, ctx, 999, 1, 0.0);
    setup_geometry_instance(&mut o, triangle, material, ptr::null_mut(), ptr::null_mut());
    let geo_group = create_geometry_group(&o, ctx, &[triangle]);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    let buffer = create_buffer_u32(&mut o, ctx, "var_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[1024]);
}

optix_test!(exception_closest_hit);

unsafe fn exception_closest_hit<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetExceptionEnabled(ctx, optix_types::RTexception::RT_EXCEPTION_ALL, 1);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let trace = create_program(&mut o, ctx, EXCEPTION_PTX, "trace\0");
    let exception = create_program(&mut o, ctx, EXCEPTION_PTX, "exception\0");
    let throw_ = create_program(&mut o, ctx, EXCEPTION_PTX, "throw_\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, trace);
    o.rtContextSetExceptionProgram(ctx, 0, exception);
    let material = create_material(&mut o, ctx);
    let (triangle, _) = create_triangles(&mut o, ctx, 0, 1, 0.0);
    setup_geometry_instance(&mut o, triangle, material, ptr::null_mut(), throw_);
    let geo_group = create_geometry_group(&o, ctx, &[triangle]);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    let buffer = create_buffer_u32(&mut o, ctx, "var_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[1024]);
}

optix_test!(exception_any_hit);

unsafe fn exception_any_hit<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetExceptionEnabled(ctx, optix_types::RTexception::RT_EXCEPTION_ALL, 1);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let trace = create_program(&mut o, ctx, EXCEPTION_PTX, "trace\0");
    let exception = create_program(&mut o, ctx, EXCEPTION_PTX, "exception\0");
    let throw_ = create_program(&mut o, ctx, EXCEPTION_PTX, "throw_\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, trace);
    o.rtContextSetExceptionProgram(ctx, 0, exception);
    let material = create_material(&mut o, ctx);
    let (triangle, _) = create_triangles(&mut o, ctx, 0, 1, 0.0);
    setup_geometry_instance(&mut o, triangle, material, throw_, ptr::null_mut());
    let geo_group = create_geometry_group(&o, ctx, &[triangle]);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    let buffer = create_buffer_u32(&mut o, ctx, "var_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[1024]);
}

optix_test!(exception_attribute);

unsafe fn exception_attribute<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetExceptionEnabled(ctx, optix_types::RTexception::RT_EXCEPTION_ALL, 1);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let trace = create_program(&mut o, ctx, EXCEPTION_PTX, "trace\0");
    let exception = create_program(&mut o, ctx, EXCEPTION_PTX, "exception\0");
    let throw_ = create_program(&mut o, ctx, EXCEPTION_PTX, "throw_\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, trace);
    o.rtContextSetExceptionProgram(ctx, 0, exception);
    let material = create_material(&mut o, ctx);
    let (triangle_instance, triangle) = create_triangles(&mut o, ctx, 0, 1, 0.0);
    o.rtGeometryTrianglesSetAttributeProgram(triangle, throw_);
    setup_geometry_instance(
        &mut o,
        triangle_instance,
        material,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    let geo_group = create_geometry_group(&o, ctx, &[triangle_instance]);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    let buffer = create_buffer_u32(&mut o, ctx, "var_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[1024]);
}

optix_test!(exception_callable);

unsafe fn exception_callable<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetExceptionEnabled(ctx, optix_types::RTexception::RT_EXCEPTION_ALL, 1);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let call_callable1 = create_program(&mut o, ctx, EXCEPTION_PTX, "call_callable1\0");
    let call_callable2 = create_program(&mut o, ctx, EXCEPTION_PTX, "call_callable2\0");
    let throw_callable = create_program(&mut o, ctx, EXCEPTION_PTX, "throw_callable\0");
    let exception = create_program(&mut o, ctx, EXCEPTION_PTX, "exception\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, call_callable1);
    o.rtContextSetExceptionProgram(ctx, 0, exception);
    let call_callable2_id = program_get_id(&mut o, call_callable2);
    let throw_callable_id = program_get_id(&mut o, throw_callable);
    context_set_u32(&mut o, ctx, b"callable1\0", call_callable2_id as u32);
    context_set_u32(&mut o, ctx, b"callable2\0", throw_callable_id as u32);
    let buffer = create_buffer_u32(&mut o, ctx, "var_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[1025]);
}

optix_test!(exception_callable_subfunc);

unsafe fn exception_callable_subfunc<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetExceptionEnabled(ctx, optix_types::RTexception::RT_EXCEPTION_ALL, 1);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let call_callable1 = create_program(&mut o, ctx, EXCEPTION_PTX, "call_callable1\0");
    let call_callable2 = create_program(&mut o, ctx, EXCEPTION_PTX, "call_callable2\0");
    let throw_callable = create_program(&mut o, ctx, EXCEPTION_PTX, "throw_callable\0");
    let throw_callable_main = create_program(&mut o, ctx, EXCEPTION_PTX, "throw_callable_main\0");
    let exception = create_program(&mut o, ctx, EXCEPTION_PTX, "exception\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, call_callable1);
    o.rtContextSetExceptionProgram(ctx, 0, exception);
    let call_callable2_id = program_get_id(&mut o, call_callable2);
    let throw_callable_main_id = program_get_id(&mut o, throw_callable_main);
    let throw_callable_id = program_get_id(&mut o, throw_callable);
    context_set_u32(&mut o, ctx, b"callable1\0", call_callable2_id as u32);
    context_set_u32(&mut o, ctx, b"callable2\0", throw_callable_main_id as u32);
    context_set_u32(&mut o, ctx, b"callable3\0", throw_callable_id as u32);
    let buffer = create_buffer_u32(&mut o, ctx, "var_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[1025]);
}

const EXCEPTION_SUBFUNC_PTX: &'static str = concat!(include_str!("exception_subfunc.ptx"), "\0");

optix_test!(exception_subfunc);

unsafe fn exception_subfunc<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetExceptionEnabled(ctx, optix_types::RTexception::RT_EXCEPTION_ALL, 1);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let trace = create_program(&mut o, ctx, EXCEPTION_SUBFUNC_PTX, "start\0");
    let exception = create_program(&mut o, ctx, EXCEPTION_SUBFUNC_PTX, "exception\0");
    let throw_ = create_program(&mut o, ctx, EXCEPTION_SUBFUNC_PTX, "throw_\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, trace);
    o.rtContextSetExceptionProgram(ctx, 0, exception);
    o.rtContextSetMissProgram(ctx, 0, throw_);
    let material = create_material(&mut o, ctx);
    let (triangle, _) = create_triangles(&mut o, ctx, 999, 1, 0.0);
    setup_geometry_instance(&mut o, triangle, material, ptr::null_mut(), ptr::null_mut());
    let geo_group = create_geometry_group(&o, ctx, &[triangle]);
    create_set_accelerator(&mut o, ctx, geo_group, b"bvh\0");
    let buffer = create_buffer_u32(&mut o, ctx, "var_buffer", 1);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, buffer, &[1024]);
}

const BUFFER_MIPMAP_PTX: &'static str = concat!(include_str!("buffer_mipmap.ptx"), "\0");

// Broken for now because HIP does not support image arrays
optix_test!(buffer_mipmap);

unsafe fn buffer_mipmap<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let mut buffer = ptr::null_mut();
    o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_INPUT.0, &mut buffer);
    o.rtBufferSetSize2D(buffer, 3, 3);
    o.rtBufferSetFormat(buffer, RTformat::RT_FORMAT_UNSIGNED_INT2);
    o.rtBufferSetMipLevelCount(buffer, 2);
    let mut width = 0;
    let mut height = 0;
    o.rtBufferGetMipLevelSize2D(buffer, 0, &mut width, &mut height);
    assert_eq!((width, height), (3, 3));
    o.rtBufferGetMipLevelSize2D(buffer, 1, &mut width, &mut height);
    assert_eq!((width, height), (1, 1));
    o.rtBufferGetMipLevelSize2D(buffer, 2, &mut width, &mut height);
    assert_eq!((width, height), (1, 1));
    {
        let mut host_buffer = ptr::null_mut();
        o.rtBufferMapEx(
            buffer,
            RTbuffermapflag::RT_BUFFER_MAP_READ_WRITE.0,
            0,
            ptr::null_mut(),
            &mut host_buffer,
        );
        for i in 0..3 * 3 * 2 {
            *(host_buffer.cast::<u32>().add(i)) = (i as u32) + 1;
        }
        o.rtBufferUnmapEx(buffer, 0);
    }
    {
        let mut host_buffer = ptr::null_mut();
        o.rtBufferMapEx(
            buffer,
            RTbuffermapflag::RT_BUFFER_MAP_READ_WRITE.0,
            1,
            ptr::null_mut(),
            &mut host_buffer,
        );
        *host_buffer.cast::<(u32, u32)>() = (100, 101);
        o.rtBufferUnmapEx(buffer, 1);
    }
    let mut image = ptr::null_mut();
    o.rtTextureSamplerCreate(ctx, &mut image);
    o.rtTextureSamplerSetBuffer(image, 0, 0, buffer);
    o.rtTextureSamplerSetFilteringModes(
        image,
        RTfiltermode::RT_FILTER_NEAREST,
        RTfiltermode::RT_FILTER_NEAREST,
        RTfiltermode::RT_FILTER_NEAREST,
    );
    o.rtTextureSamplerSetIndexingMode(image, RTtextureindexmode::RT_TEXTURE_INDEX_ARRAY_INDEX);
    o.rtTextureSamplerSetReadMode(image, RTtexturereadmode::RT_TEXTURE_READ_ELEMENT_TYPE);
    let start = create_program(&mut o, ctx, BUFFER_MIPMAP_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    let mut texture_id = mem::zeroed();
    o.rtTextureSamplerGetId(image, &mut texture_id);
    context_set_u32(&mut o, ctx, b"texture_id\0", texture_id as u32);
    let output_buffer = create_buffer_1d(
        &mut o,
        ctx,
        RTformat::RT_FORMAT_UNSIGNED_INT2,
        "output_buffer",
        3,
    );
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, output_buffer, &[1, 2, 100, 101]);
}

const OOB_PTX: &'static str = concat!(include_str!("oob.ptx"), "\0");

optix_test!(oob);

unsafe fn oob<Optix: OptixFns>(mut o: Optix) {
    let ctx = create_context(&o);
    o.rtContextSetExceptionEnabled(ctx, RTexception::RT_EXCEPTION_ALL, 0);
    o.rtContextSetEntryPointCount(ctx, 1);
    o.rtContextSetRayTypeCount(ctx, 1);
    let start = create_program(&mut o, ctx, OOB_PTX, "start\0");
    o.rtContextSetRayGenerationProgram(ctx, 0, start);
    create_buffer_u32_with_values(&mut o, ctx, "index_", &[4, 2]);
    create_buffer_u32_with_values(&mut o, ctx, "input", &[10,11,12,13]);
    let output = create_buffer_u32_with_values(&mut o, ctx, "output", &[u32::MAX, u32::MAX, u32::MAX]);
    launch_2d(&mut o, ctx, 0, 1, 1);
    assert_buffer_eq(&o, output, &[0, 12, 0]);
}
