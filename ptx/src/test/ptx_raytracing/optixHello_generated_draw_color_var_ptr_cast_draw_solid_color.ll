target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

%struct.i64i64i64i64 = type { i64, i64, i64, i64 }
%struct.f32f32f32f32 = type { float, float, float, float }
%struct.f32f32 = type { float, float }
%struct.f32f32f32f32f32f32f32f32f32f32f32f32f32f32f32f32 = type { float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float }

@_ZN21rti_internal_register20reg_bitness_detectorE = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail0E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail1E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail2E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail3E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail4E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail5E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail6E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail7E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail8E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register24reg_exception_64_detail9E = private addrspace(1) externally_initialized global i64 0, align 8
@_ZN21rti_internal_register21reg_exception_detail0E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register21reg_exception_detail1E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register21reg_exception_detail2E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register21reg_exception_detail3E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register21reg_exception_detail4E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register21reg_exception_detail5E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register21reg_exception_detail6E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register21reg_exception_detail7E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register21reg_exception_detail8E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register21reg_exception_detail9E = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register14reg_rayIndex_xE = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register14reg_rayIndex_yE = private addrspace(1) externally_initialized global i32 0, align 4
@_ZN21rti_internal_register14reg_rayIndex_zE = private addrspace(1) externally_initialized global i32 0, align 4

declare ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1), i32) #0

declare ptr addrspace(3) @__zluda_rt_ptx_impl__get_variable_pointer_shared(ptr addrspace(3), i32) #0

declare i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64, i32, i32, i64, i64, i64, i64) #0

declare i64 @__zluda_rt_ptx_impl___rt_buffer_get_id_64(i32, i32, i32, i64, i64, i64, i64, ptr addrspace(3)) #0

declare %struct.i64i64i64i64 @__zluda_rt_ptx_impl___rt_buffer_get_size_64(i64, i32, i32) #0

declare %struct.i64i64i64i64 @__zluda_rt_ptx_impl___rt_buffer_get_id_size_64(i32, i32, i32, ptr addrspace(3)) #0

declare i32 @__zluda_rt_ptx_impl___rt_trace_mask_flags_64(i32, float, float, float, float, float, float, i32, float, float, i32, i32, i64, i32, ptr addrspace(3)) #0

declare i32 @__zluda_rt_ptx_impl___rt_trace_time_mask_flags_64(i32, float, float, float, float, float, float, i32, float, float, float, i32, i32, i64, i32, ptr addrspace(3)) #0

declare i32 @__zluda_rt_ptx_impl___rt_get_exception_code() #0

declare i32 @__zluda_rt_ptx_impl___rt_print_active() #0

declare i32 @__zluda_rt_ptx_impl___rt_potential_intersection(float, ptr addrspace(5), float, ptr addrspace(5)) #0

declare i32 @__zluda_rt_ptx_impl___rt_report_intersection(i32, ptr addrspace(3), <2 x i32>, <2 x i32>, ptr addrspace(5), float, ptr addrspace(5), ptr addrspace(1), ptr addrspace(1), ptr addrspace(1), ptr addrspace(5), ptr addrspace(5), ptr addrspace(5), ptr addrspace(5)) #0

declare void @__zluda_rt_ptx_impl___rt_terminate_ray() #0

declare void @__zluda_rt_ptx_impl___rt_ignore_intersection() #0

declare %struct.f32f32f32f32 @__zluda_rt_ptx_impl___rt_transform_tuple(i32, float, float, float, float) #0

declare i64 @__zluda_rt_ptx_impl___rt_callable_program_from_id_64(i32, ptr addrspace(3)) #0

declare i64 @__zluda_rt_ptx_impl___rt_callable_program_from_id_v2_64(i32, i64, ptr addrspace(3)) #0

declare %struct.f32f32f32f32 @__zluda_rt_ptx_impl___rt_texture_get_f_id(i32, i32, float, float, float, float, ptr addrspace(3)) #0

declare %struct.f32f32f32f32 @__zluda_rt_ptx_impl___rt_texture_grad_load_or_request_f_id(i32, i32, float, float, float, float, float, float, float, float, float, float, i64, ptr addrspace(3)) #0

declare %struct.f32f32f32f32 @__zluda_rt_ptx_impl___rt_texture_lod_load_or_request_f_id(i32, i32, float, float, float, float, float, i64, ptr addrspace(3)) #0

declare %struct.f32f32 @__zluda_rt_ptx_impl___rt_get_triangle_barycentrics() #0

declare i32 @__zluda_rt_ptx_impl___rt_get_primitive_index() #0

declare float @__zluda_rt_ptx_impl___rt_is_triangle_hit(<3 x float>) #0

declare float @__zluda_rt_ptx_impl___rt_is_triangle_hit_front_face(ptr addrspace(5), <3 x float>) #0

declare float @__zluda_rt_ptx_impl___rt_is_triangle_hit_back_face(ptr addrspace(5), <3 x float>) #0

declare %struct.f32f32f32f32f32f32f32f32f32f32f32f32f32f32f32f32 @__zluda_rt_ptx_impl___rt_get_transform(i32, ptr addrspace(1)) #0

declare void @__zluda_rt_ptx_impl___rt_throw(i32) #0

define protected i32 @_Z16draw_solid_colorv(ptr addrspace(3) %"276", i32 %"277", <2 x i32> %"278", <2 x i32> %"279", ptr addrspace(5) %"280", float %"281", ptr addrspace(5) %"282", float %"283", <2 x float> %"284", <3 x float> %"285", ptr addrspace(1) %"286", ptr addrspace(1) %"287", ptr addrspace(1) %"288") #1 {
"393":
  %"373" = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %"373", align 4
  %"227" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"227", align 1
  %"228" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"228", align 1
  %"206" = alloca float, align 4, addrspace(5)
  %"207" = alloca float, align 4, addrspace(5)
  %"208" = alloca float, align 4, addrspace(5)
  %"209" = alloca float, align 4, addrspace(5)
  %"210" = alloca float, align 4, addrspace(5)
  %"211" = alloca i32, align 4, addrspace(5)
  %"212" = alloca i32, align 4, addrspace(5)
  %"213" = alloca i32, align 4, addrspace(5)
  %"214" = alloca i32, align 4, addrspace(5)
  %"215" = alloca i32, align 4, addrspace(5)
  %"216" = alloca i32, align 4, addrspace(5)
  %"217" = alloca i32, align 4, addrspace(5)
  %"218" = alloca i64, align 8, addrspace(5)
  %"219" = alloca i64, align 8, addrspace(5)
  %"220" = alloca i64, align 8, addrspace(5)
  %"221" = alloca i64, align 8, addrspace(5)
  %"222" = alloca i64, align 8, addrspace(5)
  %"223" = alloca i64, align 8, addrspace(5)
  %"224" = alloca i64, align 8, addrspace(5)
  %"225" = alloca i64, align 8, addrspace(5)
  %"289" = alloca <2 x i32>, align 8, addrspace(5)
  %"290" = alloca <2 x i32>, align 8, addrspace(5)
  %0 = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %0, align 8
  %"374" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"374", ptr addrspace(5) %"224", align 8
  store <2 x i32> %"278", ptr addrspace(5) %"289", align 8
  %"375" = load i32, ptr addrspace(5) %"289", align 4
  store i32 %"375", ptr addrspace(5) %"214", align 4
  store <2 x i32> %"278", ptr addrspace(5) %"290", align 8
  %"396" = getelementptr inbounds i8, ptr addrspace(5) %"290", i64 4
  %"378" = load i32, ptr addrspace(5) %"396", align 4
  store i32 %"378", ptr addrspace(5) %"215", align 4
  %"233" = load i32, ptr addrspace(5) %"214", align 4
  %"379" = zext i32 %"233" to i64
  store i64 %"379", ptr addrspace(5) %"221", align 8
  %"235" = load i32, ptr addrspace(5) %"215", align 4
  %"381" = zext i32 %"235" to i64
  store i64 %"381", ptr addrspace(5) %"222", align 8
  %"292" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"286", i32 8)
  %"384" = ptrtoint ptr addrspace(1) %"292" to i64
  %1 = alloca i64, align 8, addrspace(5)
  store i64 %"384", ptr addrspace(5) %1, align 8
  %"383" = load i64, ptr addrspace(5) %1, align 8
  store i64 %"383", ptr addrspace(5) %"225", align 8
  %"238" = load i64, ptr addrspace(5) %"225", align 8
  %2 = inttoptr i64 %"238" to ptr addrspace(1)
  %3 = addrspacecast ptr addrspace(1) %2 to ptr
  %"237" = ptrtoint ptr %3 to i64
  store i64 %"237", ptr addrspace(5) %"220", align 8
  %4 = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %4, align 4
  %"385" = load i32, ptr addrspace(5) %4, align 4
  store i32 %"385", ptr addrspace(5) %"212", align 4
  %5 = alloca i32, align 4, addrspace(5)
  store i32 16, ptr addrspace(5) %5, align 4
  %"386" = load i32, ptr addrspace(5) %5, align 4
  store i32 %"386", ptr addrspace(5) %"213", align 4
  %"242" = load i64, ptr addrspace(5) %"220", align 8
  %"243" = load i32, ptr addrspace(5) %"212", align 4
  %"244" = load i32, ptr addrspace(5) %"213", align 4
  %"245" = load i64, ptr addrspace(5) %"221", align 8
  %"246" = load i64, ptr addrspace(5) %"222", align 8
  %"247" = load i64, ptr addrspace(5) %"224", align 8
  %"248" = load i64, ptr addrspace(5) %"224", align 8
  %"241" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"242", i32 %"243", i32 %"244", i64 %"245", i64 %"246", i64 %"247", i64 %"248")
  store i64 %"241", ptr addrspace(5) %"219", align 8
  %"294" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"286", i32 32)
  %"249" = load float, ptr addrspace(1) %"294", align 4
  store float %"249", ptr addrspace(5) %"207", align 4
  %"296" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"286", i32 32)
  %"398" = getelementptr inbounds i8, ptr addrspace(1) %"296", i64 4
  %"250" = load float, ptr addrspace(1) %"398", align 4
  store float %"250", ptr addrspace(5) %"208", align 4
  %"299" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"286", i32 32)
  %"400" = getelementptr inbounds i8, ptr addrspace(1) %"299", i64 8
  %"251" = load float, ptr addrspace(1) %"400", align 4
  store float %"251", ptr addrspace(5) %"209", align 4
  %6 = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %6, align 4
  %"252" = load float, ptr addrspace(5) %6, align 4
  store float %"252", ptr addrspace(5) %"210", align 4
  %"253" = load float, ptr addrspace(5) %"207", align 4
  %"254" = load float, ptr addrspace(5) %"208", align 4
  %"255" = load float, ptr addrspace(5) %"209", align 4
  %"256" = load float, ptr addrspace(5) %"210", align 4
  %7 = insertelement <4 x float> undef, float %"253", i32 0
  %8 = insertelement <4 x float> %7, float %"254", i32 1
  %9 = insertelement <4 x float> %8, float %"255", i32 2
  %"226" = insertelement <4 x float> %9, float %"256", i32 3
  %"257" = load i64, ptr addrspace(5) %"219", align 8
  %"390" = inttoptr i64 %"257" to ptr
  store <4 x float> %"226", ptr %"390", align 16
  ret i32 0
}

define protected i32 @__zluda_rt_ptx_impl__rollback_wrapper(ptr addrspace(3) %"308", i32 %"309", <2 x i32> %"310", <2 x i32> %"311", ptr addrspace(5) %"312", float %"313", ptr addrspace(5) %"314", float %"315", <2 x float> %"316", <3 x float> %"317", ptr addrspace(1) %"318", ptr addrspace(1) %"319", ptr addrspace(1) %"320", i64 %"321") #1 {
"394":
  %"322" = call i32 @_Z16draw_solid_colorv(ptr addrspace(3) %"308", i32 %"309", <2 x i32> %"310", <2 x i32> %"311", ptr addrspace(5) %"312", float %"313", ptr addrspace(5) %"314", float %"315", <2 x float> %"316", <3 x float> %"317", ptr addrspace(1) %"318", ptr addrspace(1) %"319", ptr addrspace(1) %"320")
  %"324" = icmp uge i32 %"322", 1024
  br i1 %"324", label %"325", label %"326"

"325":                                            ; preds = %"394"
  ret i32 %"322"

"326":                                            ; preds = %"394"
  %"328" = icmp eq i64 %"321", 0
  br i1 %"328", label %"329", label %"330"

"329":                                            ; preds = %"326"
  ret i32 0

"330":                                            ; preds = %"326"
  %"391" = inttoptr i64 %"321" to ptr addrspace(1)
  %"332" = load i64, ptr addrspace(1) %"391", align 8
  %"392" = inttoptr i64 %"321" to ptr addrspace(1)
  %0 = inttoptr i64 %"332" to ptr
  %"333" = call i32 %0(ptr addrspace(3) %"308", i32 %"309", <2 x i32> %"310", <2 x i32> %"311", ptr addrspace(5) %"312", float %"313", ptr addrspace(5) %"314", float %"315", <2 x float> %"316", <3 x float> %"317", ptr addrspace(1) %"392", ptr addrspace(1) %"319", ptr addrspace(1) %"320")
  %"335" = icmp uge i32 %"333", 1024
  br i1 %"335", label %"336", label %"337"

"336":                                            ; preds = %"330"
  ret i32 %"333"

"337":                                            ; preds = %"330"
  %"340" = icmp eq i32 %"333", 1
  br i1 %"340", label %"341", label %"338"

"341":                                            ; preds = %"337"
  br label %"338"

"338":                                            ; preds = %"341", %"337"
  ret i32 %"333"
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
