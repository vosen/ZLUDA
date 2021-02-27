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

define protected i32 @_Z4missv(ptr addrspace(3) %"437", i32 %"438", <2 x i32> %"439", <2 x i32> %"440", ptr addrspace(5) %"441", float %"442", ptr addrspace(5) %"443", float %"444", <2 x float> %"445", <3 x float> %"446", ptr addrspace(1) %"447", ptr addrspace(1) %"448", ptr addrspace(1) %"449") #1 {
"590":
  %"544" = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %"544", align 4
  %"311" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"311", align 1
  %"312" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"312", align 1
  %"272" = alloca [36 x i8], align 4, addrspace(5)
  %"273" = alloca i64, align 8, addrspace(5)
  %"274" = alloca i64, align 8, addrspace(5)
  %"275" = alloca float, align 4, addrspace(5)
  %"276" = alloca float, align 4, addrspace(5)
  %"277" = alloca float, align 4, addrspace(5)
  %"278" = alloca float, align 4, addrspace(5)
  %"279" = alloca float, align 4, addrspace(5)
  %"280" = alloca float, align 4, addrspace(5)
  %"281" = alloca float, align 4, addrspace(5)
  %"282" = alloca float, align 4, addrspace(5)
  %"283" = alloca float, align 4, addrspace(5)
  %"284" = alloca float, align 4, addrspace(5)
  %"285" = alloca float, align 4, addrspace(5)
  %"286" = alloca float, align 4, addrspace(5)
  %"287" = alloca i32, align 4, addrspace(5)
  %"288" = alloca i32, align 4, addrspace(5)
  %"289" = alloca i32, align 4, addrspace(5)
  %"290" = alloca i32, align 4, addrspace(5)
  %"291" = alloca i32, align 4, addrspace(5)
  %"292" = alloca i64, align 8, addrspace(5)
  %"293" = alloca i64, align 8, addrspace(5)
  %"294" = alloca i64, align 8, addrspace(5)
  %"295" = alloca i64, align 8, addrspace(5)
  %"296" = alloca i64, align 8, addrspace(5)
  %"297" = alloca i64, align 8, addrspace(5)
  %"298" = alloca i64, align 8, addrspace(5)
  %"299" = alloca i64, align 8, addrspace(5)
  %"300" = alloca i64, align 8, addrspace(5)
  %"301" = alloca i64, align 8, addrspace(5)
  %"302" = alloca i64, align 8, addrspace(5)
  %"303" = alloca i64, align 8, addrspace(5)
  %"304" = alloca i64, align 8, addrspace(5)
  %"305" = alloca i32, align 4, addrspace(5)
  %"423" = alloca i64, align 8, addrspace(5)
  %"425" = alloca [3 x i32], align 4, addrspace(5)
  %"546" = ptrtoint ptr addrspace(5) %"272" to i64
  %0 = alloca i64, align 8, addrspace(5)
  store i64 %"546", ptr addrspace(5) %0, align 8
  %"545" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"545", ptr addrspace(5) %"274", align 8
  %"315" = load i64, ptr addrspace(5) %"274", align 8
  %1 = inttoptr i64 %"315" to ptr addrspace(5)
  %2 = addrspacecast ptr addrspace(5) %1 to ptr
  %"314" = ptrtoint ptr %2 to i64
  store i64 %"314", ptr addrspace(5) %"273", align 8
  %"317" = load i64, ptr addrspace(5) %"273", align 8
  %"547" = add i64 %"317", 0
  store i64 %"547", ptr addrspace(5) %"301", align 8
  %"319" = load i64, ptr addrspace(5) %"274", align 8
  %"549" = add i64 %"319", 0
  store i64 %"549", ptr addrspace(5) %"302", align 8
  %"450" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"447", i32 8)
  %"552" = ptrtoint ptr addrspace(1) %"450" to i64
  %3 = alloca i64, align 8, addrspace(5)
  store i64 %"552", ptr addrspace(5) %3, align 8
  %"551" = load i64, ptr addrspace(5) %3, align 8
  store i64 %"551", ptr addrspace(5) %"303", align 8
  %"322" = load i64, ptr addrspace(5) %"303", align 8
  %4 = inttoptr i64 %"322" to ptr addrspace(1)
  %5 = addrspacecast ptr addrspace(1) %4 to ptr
  %"321" = ptrtoint ptr %5 to i64
  store i64 %"321", ptr addrspace(5) %"294", align 8
  %6 = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %6, align 4
  %"553" = load i32, ptr addrspace(5) %6, align 4
  store i32 %"553", ptr addrspace(5) %"288", align 4
  %7 = alloca i32, align 4, addrspace(5)
  store i32 4, ptr addrspace(5) %7, align 4
  %"554" = load i32, ptr addrspace(5) %7, align 4
  store i32 %"554", ptr addrspace(5) %"289", align 4
  %8 = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %8, align 8
  %"555" = load i64, ptr addrspace(5) %8, align 8
  store i64 %"555", ptr addrspace(5) %"300", align 8
  %"327" = load i64, ptr addrspace(5) %"294", align 8
  %"328" = load i32, ptr addrspace(5) %"288", align 4
  %"329" = load i32, ptr addrspace(5) %"289", align 4
  %"330" = load i64, ptr addrspace(5) %"300", align 8
  %"331" = load i64, ptr addrspace(5) %"300", align 8
  %"332" = load i64, ptr addrspace(5) %"300", align 8
  %"333" = load i64, ptr addrspace(5) %"300", align 8
  %"326" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"327", i32 %"328", i32 %"329", i64 %"330", i64 %"331", i64 %"332", i64 %"333")
  store i64 %"326", ptr addrspace(5) %"293", align 8
  %"334" = load float, ptr addrspace(5) %"441", align 4
  store float %"334", ptr addrspace(5) %"276", align 4
  %"593" = getelementptr inbounds i8, ptr addrspace(5) %"441", i64 4
  %"335" = load float, ptr addrspace(5) %"593", align 4
  store float %"335", ptr addrspace(5) %"277", align 4
  %"595" = getelementptr inbounds i8, ptr addrspace(5) %"441", i64 8
  %"336" = load float, ptr addrspace(5) %"595", align 4
  store float %"336", ptr addrspace(5) %"278", align 4
  %"597" = getelementptr inbounds i8, ptr addrspace(5) %"441", i64 12
  %"337" = load float, ptr addrspace(5) %"597", align 4
  store float %"337", ptr addrspace(5) %"279", align 4
  %"599" = getelementptr inbounds i8, ptr addrspace(5) %"441", i64 16
  %"338" = load float, ptr addrspace(5) %"599", align 4
  store float %"338", ptr addrspace(5) %"280", align 4
  %"601" = getelementptr inbounds i8, ptr addrspace(5) %"441", i64 20
  %"339" = load float, ptr addrspace(5) %"601", align 4
  store float %"339", ptr addrspace(5) %"281", align 4
  %"603" = getelementptr inbounds i8, ptr addrspace(5) %"441", i64 24
  %"563" = load i32, ptr addrspace(5) %"603", align 4
  store i32 %"563", ptr addrspace(5) %"291", align 4
  %"605" = getelementptr inbounds i8, ptr addrspace(5) %"441", i64 28
  %"341" = load float, ptr addrspace(5) %"605", align 4
  store float %"341", ptr addrspace(5) %"282", align 4
  %"607" = getelementptr inbounds i8, ptr addrspace(5) %"441", i64 32
  %"342" = load float, ptr addrspace(5) %"607", align 4
  store float %"342", ptr addrspace(5) %"283", align 4
  %"344" = load i64, ptr addrspace(5) %"293", align 8
  %"567" = inttoptr i64 %"344" to ptr
  %"566" = load i32, ptr %"567", align 4
  store i32 %"566", ptr addrspace(5) %"290", align 4
  %"346" = load i32, ptr addrspace(5) %"290", align 4
  %"347" = load i64, ptr addrspace(5) %"300", align 8
  %"345" = call i64 @__zluda_rt_ptx_impl___rt_callable_program_from_id_v2_64(i32 %"346", i64 %"347", ptr addrspace(3) %"437")
  store i64 %"345", ptr addrspace(5) %"299", align 8
  %"348" = load i64, ptr addrspace(5) %"302", align 8
  %"349" = load float, ptr addrspace(5) %"276", align 4
  %"568" = inttoptr i64 %"348" to ptr addrspace(5)
  store float %"349", ptr addrspace(5) %"568", align 4
  %"350" = load i64, ptr addrspace(5) %"302", align 8
  %"351" = load float, ptr addrspace(5) %"277", align 4
  %"569" = inttoptr i64 %"350" to ptr addrspace(5)
  %"609" = getelementptr inbounds i8, ptr addrspace(5) %"569", i64 4
  store float %"351", ptr addrspace(5) %"609", align 4
  %"352" = load i64, ptr addrspace(5) %"302", align 8
  %"353" = load float, ptr addrspace(5) %"278", align 4
  %"570" = inttoptr i64 %"352" to ptr addrspace(5)
  %"611" = getelementptr inbounds i8, ptr addrspace(5) %"570", i64 8
  store float %"353", ptr addrspace(5) %"611", align 4
  %"354" = load i64, ptr addrspace(5) %"302", align 8
  %"355" = load float, ptr addrspace(5) %"279", align 4
  %"571" = inttoptr i64 %"354" to ptr addrspace(5)
  %"613" = getelementptr inbounds i8, ptr addrspace(5) %"571", i64 12
  store float %"355", ptr addrspace(5) %"613", align 4
  %"356" = load i64, ptr addrspace(5) %"302", align 8
  %"357" = load float, ptr addrspace(5) %"280", align 4
  %"572" = inttoptr i64 %"356" to ptr addrspace(5)
  %"615" = getelementptr inbounds i8, ptr addrspace(5) %"572", i64 16
  store float %"357", ptr addrspace(5) %"615", align 4
  %"358" = load i64, ptr addrspace(5) %"302", align 8
  %"359" = load float, ptr addrspace(5) %"281", align 4
  %"573" = inttoptr i64 %"358" to ptr addrspace(5)
  %"617" = getelementptr inbounds i8, ptr addrspace(5) %"573", i64 20
  store float %"359", ptr addrspace(5) %"617", align 4
  %"360" = load i64, ptr addrspace(5) %"302", align 8
  %"361" = load i32, ptr addrspace(5) %"291", align 4
  %"574" = inttoptr i64 %"360" to ptr addrspace(5)
  %"619" = getelementptr inbounds i8, ptr addrspace(5) %"574", i64 24
  store i32 %"361", ptr addrspace(5) %"619", align 4
  %"362" = load i64, ptr addrspace(5) %"302", align 8
  %"363" = load float, ptr addrspace(5) %"282", align 4
  %"576" = inttoptr i64 %"362" to ptr addrspace(5)
  %"621" = getelementptr inbounds i8, ptr addrspace(5) %"576", i64 28
  store float %"363", ptr addrspace(5) %"621", align 4
  %"364" = load i64, ptr addrspace(5) %"302", align 8
  %"365" = load float, ptr addrspace(5) %"283", align 4
  %"577" = inttoptr i64 %"364" to ptr addrspace(5)
  %"623" = getelementptr inbounds i8, ptr addrspace(5) %"577", i64 32
  store float %"365", ptr addrspace(5) %"623", align 4
  %"366" = load i64, ptr addrspace(5) %"301", align 8
  %"625" = getelementptr inbounds i8, ptr addrspace(5) %"423", i64 0
  store i64 %"366", ptr addrspace(5) %"625", align 8
  %"309" = load i64, ptr addrspace(5) %"423", align 8
  %"367" = load i64, ptr addrspace(5) %"299", align 8
  %"578" = inttoptr i64 %"367" to ptr addrspace(1)
  %"467" = load i64, ptr addrspace(1) %"578", align 8
  %"579" = inttoptr i64 %"367" to ptr addrspace(1)
  %9 = inttoptr i64 %"467" to ptr
  %10 = call { [3 x i32], i32 } %9(i64 %"309", ptr addrspace(3) %"437", <2 x i32> %"439", ptr addrspace(1) %"579")
  %"310" = extractvalue { [3 x i32], i32 } %10, 0
  %"466" = extractvalue { [3 x i32], i32 } %10, 1
  %"469" = icmp uge i32 %"466", 1024
  br i1 %"469", label %"470", label %"471"

"470":                                            ; preds = %"590"
  ret i32 %"466"

"471":                                            ; preds = %"590"
  store [3 x i32] %"310", ptr addrspace(5) %"425", align 4
  %"627" = getelementptr inbounds i8, ptr addrspace(5) %"425", i64 0
  %"368" = load float, ptr addrspace(5) %"627", align 4
  store float %"368", ptr addrspace(5) %"284", align 4
  %"629" = getelementptr inbounds i8, ptr addrspace(5) %"425", i64 4
  %"369" = load float, ptr addrspace(5) %"629", align 4
  store float %"369", ptr addrspace(5) %"285", align 4
  %"631" = getelementptr inbounds i8, ptr addrspace(5) %"425", i64 8
  %"370" = load float, ptr addrspace(5) %"631", align 4
  store float %"370", ptr addrspace(5) %"286", align 4
  %"371" = load float, ptr addrspace(5) %"284", align 4
  store float %"371", ptr addrspace(5) %"443", align 4
  %"372" = load float, ptr addrspace(5) %"285", align 4
  %"633" = getelementptr inbounds i8, ptr addrspace(5) %"443", i64 4
  store float %"372", ptr addrspace(5) %"633", align 4
  %"373" = load float, ptr addrspace(5) %"286", align 4
  %"635" = getelementptr inbounds i8, ptr addrspace(5) %"443", i64 8
  store float %"373", ptr addrspace(5) %"635", align 4
  ret i32 0
}

define protected i32 @__zluda_rt_ptx_impl__rollback_wrapper(ptr addrspace(3) %"474", i32 %"475", <2 x i32> %"476", <2 x i32> %"477", ptr addrspace(5) %"478", float %"479", ptr addrspace(5) %"480", float %"481", <2 x float> %"482", <3 x float> %"483", ptr addrspace(1) %"484", ptr addrspace(1) %"485", ptr addrspace(1) %"486", i64 %"487") #1 {
"591":
  %"488" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"485", i32 0)
  %"490" = load [12 x i8], ptr addrspace(1) %"488", align 1
  %"491" = call i32 @_Z4missv(ptr addrspace(3) %"474", i32 %"475", <2 x i32> %"476", <2 x i32> %"477", ptr addrspace(5) %"478", float %"479", ptr addrspace(5) %"480", float %"481", <2 x float> %"482", <3 x float> %"483", ptr addrspace(1) %"484", ptr addrspace(1) %"485", ptr addrspace(1) %"486")
  %"493" = icmp uge i32 %"491", 1024
  br i1 %"493", label %"494", label %"495"

"494":                                            ; preds = %"591"
  ret i32 %"491"

"495":                                            ; preds = %"591"
  %"497" = icmp eq i64 %"487", 0
  br i1 %"497", label %"498", label %"499"

"498":                                            ; preds = %"495"
  ret i32 0

"499":                                            ; preds = %"495"
  %"587" = inttoptr i64 %"487" to ptr addrspace(1)
  %"501" = load i64, ptr addrspace(1) %"587", align 8
  %"588" = inttoptr i64 %"487" to ptr addrspace(1)
  %0 = inttoptr i64 %"501" to ptr
  %"502" = call i32 %0(ptr addrspace(3) %"474", i32 %"475", <2 x i32> %"476", <2 x i32> %"477", ptr addrspace(5) %"478", float %"479", ptr addrspace(5) %"480", float %"481", <2 x float> %"482", <3 x float> %"483", ptr addrspace(1) %"588", ptr addrspace(1) %"485", ptr addrspace(1) %"486")
  %"504" = icmp uge i32 %"502", 1024
  br i1 %"504", label %"505", label %"506"

"505":                                            ; preds = %"499"
  ret i32 %"502"

"506":                                            ; preds = %"499"
  %"509" = icmp eq i32 %"502", 1
  br i1 %"509", label %"510", label %"507"

"510":                                            ; preds = %"506"
  %"511" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"485", i32 0)
  store [12 x i8] %"490", ptr addrspace(1) %"511", align 1
  br label %"507"

"507":                                            ; preds = %"510", %"506"
  ret i32 %"502"
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
