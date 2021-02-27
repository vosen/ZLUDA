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

define protected i32 @_Z20closest_hit_radiancev(ptr addrspace(3) %"306", i32 %"307", <2 x i32> %"308", <2 x i32> %"309", ptr addrspace(5) %"310", float %"311", ptr addrspace(5) %"312", float %"313", <2 x float> %"314", <3 x float> %"315", ptr addrspace(1) %"316", ptr addrspace(1) %"317", ptr addrspace(1) %"318") #1 {
"417":
  %"405" = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %"405", align 4
  %"235" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"235", align 1
  %"236" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"236", align 1
  %"214" = alloca float, align 4, addrspace(5)
  %"215" = alloca float, align 4, addrspace(5)
  %"216" = alloca float, align 4, addrspace(5)
  %"217" = alloca float, align 4, addrspace(5)
  %"218" = alloca float, align 4, addrspace(5)
  %"219" = alloca float, align 4, addrspace(5)
  %"220" = alloca float, align 4, addrspace(5)
  %"221" = alloca float, align 4, addrspace(5)
  %"222" = alloca float, align 4, addrspace(5)
  %"223" = alloca float, align 4, addrspace(5)
  %"224" = alloca float, align 4, addrspace(5)
  %"225" = alloca float, align 4, addrspace(5)
  %"226" = alloca float, align 4, addrspace(5)
  %"227" = alloca float, align 4, addrspace(5)
  %"228" = alloca float, align 4, addrspace(5)
  %"229" = alloca float, align 4, addrspace(5)
  %"230" = alloca float, align 4, addrspace(5)
  %"231" = alloca float, align 4, addrspace(5)
  %"232" = alloca float, align 4, addrspace(5)
  %"233" = alloca i32, align 4, addrspace(5)
  %"234" = alloca i32, align 4, addrspace(5)
  %"319" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"317", i32 0)
  %"237" = load float, ptr addrspace(1) %"319", align 4
  store float %"237", ptr addrspace(5) %"219", align 4
  %"321" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"317", i32 0)
  %"420" = getelementptr inbounds i8, ptr addrspace(1) %"321", i64 4
  %"238" = load float, ptr addrspace(1) %"420", align 4
  store float %"238", ptr addrspace(5) %"220", align 4
  %"324" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"317", i32 0)
  %"422" = getelementptr inbounds i8, ptr addrspace(1) %"324", i64 8
  %"239" = load float, ptr addrspace(1) %"422", align 4
  store float %"239", ptr addrspace(5) %"221", align 4
  %0 = alloca i32, align 4, addrspace(5)
  store i32 7937, ptr addrspace(5) %0, align 4
  %"409" = load i32, ptr addrspace(5) %0, align 4
  store i32 %"409", ptr addrspace(5) %"234", align 4
  %1 = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %1, align 4
  %"241" = load float, ptr addrspace(5) %1, align 4
  store float %"241", ptr addrspace(5) %"222", align 4
  %"246" = load i32, ptr addrspace(5) %"234", align 4
  %"247" = load float, ptr addrspace(5) %"219", align 4
  %"248" = load float, ptr addrspace(5) %"220", align 4
  %"249" = load float, ptr addrspace(5) %"221", align 4
  %"250" = load float, ptr addrspace(5) %"222", align 4
  %2 = call %struct.f32f32f32f32 @__zluda_rt_ptx_impl___rt_transform_tuple(i32 %"246", float %"247", float %"248", float %"249", float %"250")
  %"242" = extractvalue %struct.f32f32f32f32 %2, 0
  %"243" = extractvalue %struct.f32f32f32f32 %2, 1
  %"244" = extractvalue %struct.f32f32f32f32 %2, 2
  %"245" = extractvalue %struct.f32f32f32f32 %2, 3
  store float %"242", ptr addrspace(5) %"215", align 4
  store float %"243", ptr addrspace(5) %"216", align 4
  store float %"244", ptr addrspace(5) %"217", align 4
  store float %"245", ptr addrspace(5) %"218", align 4
  %"252" = load float, ptr addrspace(5) %"216", align 4
  %"253" = load float, ptr addrspace(5) %"216", align 4
  %"251" = fmul float %"252", %"253"
  store float %"251", ptr addrspace(5) %"223", align 4
  %"255" = load float, ptr addrspace(5) %"215", align 4
  %"256" = load float, ptr addrspace(5) %"215", align 4
  %"257" = load float, ptr addrspace(5) %"223", align 4
  %"254" = call float @llvm.fma.f32(float %"255", float %"256", float %"257")
  store float %"254", ptr addrspace(5) %"224", align 4
  %"259" = load float, ptr addrspace(5) %"217", align 4
  %"260" = load float, ptr addrspace(5) %"217", align 4
  %"261" = load float, ptr addrspace(5) %"224", align 4
  %"258" = call float @llvm.fma.f32(float %"259", float %"260", float %"261")
  store float %"258", ptr addrspace(5) %"225", align 4
  %"263" = load float, ptr addrspace(5) %"225", align 4
  %3 = call afn float @llvm.sqrt.f32(float %"263")
  %"262" = fdiv arcp afn float 1.000000e+00, %3
  store float %"262", ptr addrspace(5) %"226", align 4
  %"265" = load float, ptr addrspace(5) %"226", align 4
  %"266" = load float, ptr addrspace(5) %"215", align 4
  %"264" = fmul float %"265", %"266"
  store float %"264", ptr addrspace(5) %"227", align 4
  %"268" = load float, ptr addrspace(5) %"226", align 4
  %"269" = load float, ptr addrspace(5) %"216", align 4
  %"267" = fmul float %"268", %"269"
  store float %"267", ptr addrspace(5) %"228", align 4
  %"271" = load float, ptr addrspace(5) %"226", align 4
  %"272" = load float, ptr addrspace(5) %"217", align 4
  %"270" = fmul float %"271", %"272"
  store float %"270", ptr addrspace(5) %"229", align 4
  %"274" = load float, ptr addrspace(5) %"227", align 4
  %"273" = call float @llvm.fma.f32(float %"274", float 5.000000e-01, float 5.000000e-01)
  store float %"273", ptr addrspace(5) %"230", align 4
  %"276" = load float, ptr addrspace(5) %"228", align 4
  %"275" = call float @llvm.fma.f32(float %"276", float 5.000000e-01, float 5.000000e-01)
  store float %"275", ptr addrspace(5) %"231", align 4
  %"278" = load float, ptr addrspace(5) %"229", align 4
  %"277" = call float @llvm.fma.f32(float %"278", float 5.000000e-01, float 5.000000e-01)
  store float %"277", ptr addrspace(5) %"232", align 4
  %"279" = load float, ptr addrspace(5) %"230", align 4
  store float %"279", ptr addrspace(5) %"312", align 4
  %"280" = load float, ptr addrspace(5) %"231", align 4
  %"424" = getelementptr inbounds i8, ptr addrspace(5) %"312", i64 4
  store float %"280", ptr addrspace(5) %"424", align 4
  %"281" = load float, ptr addrspace(5) %"232", align 4
  %"426" = getelementptr inbounds i8, ptr addrspace(5) %"312", i64 8
  store float %"281", ptr addrspace(5) %"426", align 4
  ret i32 0
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.fma.f32(float, float, float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.sqrt.f32(float) #2

define protected i32 @__zluda_rt_ptx_impl__rollback_wrapper(ptr addrspace(3) %"335", i32 %"336", <2 x i32> %"337", <2 x i32> %"338", ptr addrspace(5) %"339", float %"340", ptr addrspace(5) %"341", float %"342", <2 x float> %"343", <3 x float> %"344", ptr addrspace(1) %"345", ptr addrspace(1) %"346", ptr addrspace(1) %"347", i64 %"348") #3 {
"418":
  %"349" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"346", i32 0)
  %"351" = load [12 x i8], ptr addrspace(1) %"349", align 1
  %"352" = call i32 @_Z20closest_hit_radiancev(ptr addrspace(3) %"335", i32 %"336", <2 x i32> %"337", <2 x i32> %"338", ptr addrspace(5) %"339", float %"340", ptr addrspace(5) %"341", float %"342", <2 x float> %"343", <3 x float> %"344", ptr addrspace(1) %"345", ptr addrspace(1) %"346", ptr addrspace(1) %"347")
  %"354" = icmp uge i32 %"352", 1024
  br i1 %"354", label %"355", label %"356"

"355":                                            ; preds = %"418"
  ret i32 %"352"

"356":                                            ; preds = %"418"
  %"358" = icmp eq i64 %"348", 0
  br i1 %"358", label %"359", label %"360"

"359":                                            ; preds = %"356"
  ret i32 0

"360":                                            ; preds = %"356"
  %"414" = inttoptr i64 %"348" to ptr addrspace(1)
  %"362" = load i64, ptr addrspace(1) %"414", align 8
  %"415" = inttoptr i64 %"348" to ptr addrspace(1)
  %0 = inttoptr i64 %"362" to ptr
  %"363" = call i32 %0(ptr addrspace(3) %"335", i32 %"336", <2 x i32> %"337", <2 x i32> %"338", ptr addrspace(5) %"339", float %"340", ptr addrspace(5) %"341", float %"342", <2 x float> %"343", <3 x float> %"344", ptr addrspace(1) %"415", ptr addrspace(1) %"346", ptr addrspace(1) %"347")
  %"365" = icmp uge i32 %"363", 1024
  br i1 %"365", label %"366", label %"367"

"366":                                            ; preds = %"360"
  ret i32 %"363"

"367":                                            ; preds = %"360"
  %"370" = icmp eq i32 %"363", 1
  br i1 %"370", label %"371", label %"368"

"371":                                            ; preds = %"367"
  %"372" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"346", i32 0)
  store [12 x i8] %"351", ptr addrspace(1) %"372", align 1
  br label %"368"

"368":                                            ; preds = %"371", %"367"
  ret i32 %"363"
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
