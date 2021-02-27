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

define protected { [3 x i32], i32 } @_Z4EvalR17MaterialParameterR5StateR19PerRayData_radiance(i64 %"729", i64 %"730", i64 %"731", ptr addrspace(3) %"1435", <2 x i32> %"1436", ptr addrspace(1) %"1437") #1 {
"1506":
  %"1473" = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %"1473", align 4
  %"724" = alloca i64, align 8, addrspace(5)
  %"725" = alloca i64, align 8, addrspace(5)
  %"726" = alloca i64, align 8, addrspace(5)
  %"723" = alloca [3 x i32], align 4, addrspace(5)
  %"727" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"727", align 1
  %"728" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"728", align 1
  %"1420" = alloca [3 x i32], align 4, addrspace(5)
  %"1421" = alloca i64, align 8, addrspace(5)
  %"1422" = alloca i64, align 8, addrspace(5)
  %"1423" = alloca i64, align 8, addrspace(5)
  %"441" = alloca i1, align 1, addrspace(5)
  %"442" = alloca i1, align 1, addrspace(5)
  %"443" = alloca i1, align 1, addrspace(5)
  %"444" = alloca i1, align 1, addrspace(5)
  %"445" = alloca i1, align 1, addrspace(5)
  %"446" = alloca i1, align 1, addrspace(5)
  %"447" = alloca float, align 4, addrspace(5)
  %"448" = alloca float, align 4, addrspace(5)
  %"449" = alloca float, align 4, addrspace(5)
  %"450" = alloca float, align 4, addrspace(5)
  %"451" = alloca float, align 4, addrspace(5)
  %"452" = alloca float, align 4, addrspace(5)
  %"453" = alloca float, align 4, addrspace(5)
  %"454" = alloca float, align 4, addrspace(5)
  %"455" = alloca float, align 4, addrspace(5)
  %"456" = alloca float, align 4, addrspace(5)
  %"457" = alloca float, align 4, addrspace(5)
  %"458" = alloca float, align 4, addrspace(5)
  %"459" = alloca float, align 4, addrspace(5)
  %"460" = alloca float, align 4, addrspace(5)
  %"461" = alloca float, align 4, addrspace(5)
  %"462" = alloca float, align 4, addrspace(5)
  %"463" = alloca float, align 4, addrspace(5)
  %"464" = alloca float, align 4, addrspace(5)
  %"465" = alloca float, align 4, addrspace(5)
  %"466" = alloca float, align 4, addrspace(5)
  %"467" = alloca float, align 4, addrspace(5)
  %"468" = alloca float, align 4, addrspace(5)
  %"469" = alloca float, align 4, addrspace(5)
  %"470" = alloca float, align 4, addrspace(5)
  %"471" = alloca float, align 4, addrspace(5)
  %"472" = alloca float, align 4, addrspace(5)
  %"473" = alloca float, align 4, addrspace(5)
  %"474" = alloca float, align 4, addrspace(5)
  %"475" = alloca float, align 4, addrspace(5)
  %"476" = alloca float, align 4, addrspace(5)
  %"477" = alloca float, align 4, addrspace(5)
  %"478" = alloca float, align 4, addrspace(5)
  %"479" = alloca float, align 4, addrspace(5)
  %"480" = alloca float, align 4, addrspace(5)
  %"481" = alloca float, align 4, addrspace(5)
  %"482" = alloca float, align 4, addrspace(5)
  %"483" = alloca float, align 4, addrspace(5)
  %"484" = alloca float, align 4, addrspace(5)
  %"485" = alloca float, align 4, addrspace(5)
  %"486" = alloca float, align 4, addrspace(5)
  %"487" = alloca float, align 4, addrspace(5)
  %"488" = alloca float, align 4, addrspace(5)
  %"489" = alloca float, align 4, addrspace(5)
  %"490" = alloca float, align 4, addrspace(5)
  %"491" = alloca float, align 4, addrspace(5)
  %"492" = alloca float, align 4, addrspace(5)
  %"493" = alloca float, align 4, addrspace(5)
  %"494" = alloca float, align 4, addrspace(5)
  %"495" = alloca float, align 4, addrspace(5)
  %"496" = alloca float, align 4, addrspace(5)
  %"497" = alloca float, align 4, addrspace(5)
  %"498" = alloca float, align 4, addrspace(5)
  %"499" = alloca float, align 4, addrspace(5)
  %"500" = alloca float, align 4, addrspace(5)
  %"501" = alloca float, align 4, addrspace(5)
  %"502" = alloca float, align 4, addrspace(5)
  %"503" = alloca float, align 4, addrspace(5)
  %"504" = alloca float, align 4, addrspace(5)
  %"505" = alloca float, align 4, addrspace(5)
  %"506" = alloca float, align 4, addrspace(5)
  %"507" = alloca float, align 4, addrspace(5)
  %"508" = alloca float, align 4, addrspace(5)
  %"509" = alloca float, align 4, addrspace(5)
  %"510" = alloca float, align 4, addrspace(5)
  %"511" = alloca float, align 4, addrspace(5)
  %"512" = alloca float, align 4, addrspace(5)
  %"513" = alloca float, align 4, addrspace(5)
  %"514" = alloca float, align 4, addrspace(5)
  %"515" = alloca float, align 4, addrspace(5)
  %"516" = alloca float, align 4, addrspace(5)
  %"517" = alloca float, align 4, addrspace(5)
  %"518" = alloca float, align 4, addrspace(5)
  %"519" = alloca float, align 4, addrspace(5)
  %"520" = alloca float, align 4, addrspace(5)
  %"521" = alloca float, align 4, addrspace(5)
  %"522" = alloca float, align 4, addrspace(5)
  %"523" = alloca float, align 4, addrspace(5)
  %"524" = alloca float, align 4, addrspace(5)
  %"525" = alloca float, align 4, addrspace(5)
  %"526" = alloca float, align 4, addrspace(5)
  %"527" = alloca float, align 4, addrspace(5)
  %"528" = alloca float, align 4, addrspace(5)
  %"529" = alloca float, align 4, addrspace(5)
  %"530" = alloca float, align 4, addrspace(5)
  %"531" = alloca float, align 4, addrspace(5)
  %"532" = alloca float, align 4, addrspace(5)
  %"533" = alloca float, align 4, addrspace(5)
  %"534" = alloca float, align 4, addrspace(5)
  %"535" = alloca float, align 4, addrspace(5)
  %"536" = alloca float, align 4, addrspace(5)
  %"537" = alloca float, align 4, addrspace(5)
  %"538" = alloca float, align 4, addrspace(5)
  %"539" = alloca float, align 4, addrspace(5)
  %"540" = alloca float, align 4, addrspace(5)
  %"541" = alloca float, align 4, addrspace(5)
  %"542" = alloca float, align 4, addrspace(5)
  %"543" = alloca float, align 4, addrspace(5)
  %"544" = alloca float, align 4, addrspace(5)
  %"545" = alloca float, align 4, addrspace(5)
  %"546" = alloca float, align 4, addrspace(5)
  %"547" = alloca float, align 4, addrspace(5)
  %"548" = alloca float, align 4, addrspace(5)
  %"549" = alloca float, align 4, addrspace(5)
  %"550" = alloca float, align 4, addrspace(5)
  %"551" = alloca float, align 4, addrspace(5)
  %"552" = alloca float, align 4, addrspace(5)
  %"553" = alloca float, align 4, addrspace(5)
  %"554" = alloca float, align 4, addrspace(5)
  %"555" = alloca float, align 4, addrspace(5)
  %"556" = alloca float, align 4, addrspace(5)
  %"557" = alloca float, align 4, addrspace(5)
  %"558" = alloca float, align 4, addrspace(5)
  %"559" = alloca float, align 4, addrspace(5)
  %"560" = alloca float, align 4, addrspace(5)
  %"561" = alloca float, align 4, addrspace(5)
  %"562" = alloca float, align 4, addrspace(5)
  %"563" = alloca float, align 4, addrspace(5)
  %"564" = alloca float, align 4, addrspace(5)
  %"565" = alloca float, align 4, addrspace(5)
  %"566" = alloca float, align 4, addrspace(5)
  %"567" = alloca float, align 4, addrspace(5)
  %"568" = alloca float, align 4, addrspace(5)
  %"569" = alloca float, align 4, addrspace(5)
  %"570" = alloca float, align 4, addrspace(5)
  %"571" = alloca float, align 4, addrspace(5)
  %"572" = alloca float, align 4, addrspace(5)
  %"573" = alloca float, align 4, addrspace(5)
  %"574" = alloca float, align 4, addrspace(5)
  %"575" = alloca float, align 4, addrspace(5)
  %"576" = alloca float, align 4, addrspace(5)
  %"577" = alloca float, align 4, addrspace(5)
  %"578" = alloca float, align 4, addrspace(5)
  %"579" = alloca float, align 4, addrspace(5)
  %"580" = alloca float, align 4, addrspace(5)
  %"581" = alloca float, align 4, addrspace(5)
  %"582" = alloca float, align 4, addrspace(5)
  %"583" = alloca float, align 4, addrspace(5)
  %"584" = alloca float, align 4, addrspace(5)
  %"585" = alloca float, align 4, addrspace(5)
  %"586" = alloca float, align 4, addrspace(5)
  %"587" = alloca float, align 4, addrspace(5)
  %"588" = alloca float, align 4, addrspace(5)
  %"589" = alloca float, align 4, addrspace(5)
  %"590" = alloca float, align 4, addrspace(5)
  %"591" = alloca float, align 4, addrspace(5)
  %"592" = alloca float, align 4, addrspace(5)
  %"593" = alloca float, align 4, addrspace(5)
  %"594" = alloca float, align 4, addrspace(5)
  %"595" = alloca float, align 4, addrspace(5)
  %"596" = alloca float, align 4, addrspace(5)
  %"597" = alloca float, align 4, addrspace(5)
  %"598" = alloca float, align 4, addrspace(5)
  %"599" = alloca float, align 4, addrspace(5)
  %"600" = alloca float, align 4, addrspace(5)
  %"601" = alloca float, align 4, addrspace(5)
  %"602" = alloca float, align 4, addrspace(5)
  %"603" = alloca float, align 4, addrspace(5)
  %"604" = alloca float, align 4, addrspace(5)
  %"605" = alloca float, align 4, addrspace(5)
  %"606" = alloca float, align 4, addrspace(5)
  %"607" = alloca float, align 4, addrspace(5)
  %"608" = alloca float, align 4, addrspace(5)
  %"609" = alloca float, align 4, addrspace(5)
  %"610" = alloca float, align 4, addrspace(5)
  %"611" = alloca float, align 4, addrspace(5)
  %"612" = alloca float, align 4, addrspace(5)
  %"613" = alloca float, align 4, addrspace(5)
  %"614" = alloca float, align 4, addrspace(5)
  %"615" = alloca float, align 4, addrspace(5)
  %"616" = alloca float, align 4, addrspace(5)
  %"617" = alloca float, align 4, addrspace(5)
  %"618" = alloca float, align 4, addrspace(5)
  %"619" = alloca float, align 4, addrspace(5)
  %"620" = alloca float, align 4, addrspace(5)
  %"621" = alloca float, align 4, addrspace(5)
  %"622" = alloca float, align 4, addrspace(5)
  %"623" = alloca float, align 4, addrspace(5)
  %"624" = alloca float, align 4, addrspace(5)
  %"625" = alloca float, align 4, addrspace(5)
  %"626" = alloca float, align 4, addrspace(5)
  %"627" = alloca float, align 4, addrspace(5)
  %"628" = alloca float, align 4, addrspace(5)
  %"629" = alloca float, align 4, addrspace(5)
  %"630" = alloca float, align 4, addrspace(5)
  %"631" = alloca float, align 4, addrspace(5)
  %"632" = alloca float, align 4, addrspace(5)
  %"633" = alloca float, align 4, addrspace(5)
  %"634" = alloca float, align 4, addrspace(5)
  %"635" = alloca float, align 4, addrspace(5)
  %"636" = alloca float, align 4, addrspace(5)
  %"637" = alloca float, align 4, addrspace(5)
  %"638" = alloca float, align 4, addrspace(5)
  %"639" = alloca float, align 4, addrspace(5)
  %"640" = alloca float, align 4, addrspace(5)
  %"641" = alloca float, align 4, addrspace(5)
  %"642" = alloca float, align 4, addrspace(5)
  %"643" = alloca float, align 4, addrspace(5)
  %"644" = alloca float, align 4, addrspace(5)
  %"645" = alloca float, align 4, addrspace(5)
  %"646" = alloca float, align 4, addrspace(5)
  %"647" = alloca float, align 4, addrspace(5)
  %"648" = alloca float, align 4, addrspace(5)
  %"649" = alloca float, align 4, addrspace(5)
  %"650" = alloca float, align 4, addrspace(5)
  %"651" = alloca float, align 4, addrspace(5)
  %"652" = alloca float, align 4, addrspace(5)
  %"653" = alloca float, align 4, addrspace(5)
  %"654" = alloca float, align 4, addrspace(5)
  %"655" = alloca float, align 4, addrspace(5)
  %"656" = alloca float, align 4, addrspace(5)
  %"657" = alloca float, align 4, addrspace(5)
  %"658" = alloca float, align 4, addrspace(5)
  %"659" = alloca float, align 4, addrspace(5)
  %"660" = alloca float, align 4, addrspace(5)
  %"661" = alloca float, align 4, addrspace(5)
  %"662" = alloca float, align 4, addrspace(5)
  %"663" = alloca float, align 4, addrspace(5)
  %"664" = alloca i64, align 8, addrspace(5)
  %"665" = alloca i64, align 8, addrspace(5)
  %"666" = alloca i64, align 8, addrspace(5)
  %"667" = alloca i64, align 8, addrspace(5)
  %"668" = alloca i64, align 8, addrspace(5)
  store i64 %"729", ptr addrspace(5) %"724", align 8
  store i64 %"730", ptr addrspace(5) %"725", align 8
  store i64 %"731", ptr addrspace(5) %"726", align 8
  %"732" = load i64, ptr addrspace(5) %"724", align 8
  store i64 %"732", ptr addrspace(5) %"1421", align 8
  %"733" = load i64, ptr addrspace(5) %"725", align 8
  store i64 %"733", ptr addrspace(5) %"1422", align 8
  %"734" = load i64, ptr addrspace(5) %"726", align 8
  store i64 %"734", ptr addrspace(5) %"1423", align 8
  %"1474" = load i64, ptr addrspace(5) %"1421", align 8
  store i64 %"1474", ptr addrspace(5) %"666", align 8
  %"1476" = load i64, ptr addrspace(5) %"1422", align 8
  store i64 %"1476", ptr addrspace(5) %"667", align 8
  %"1478" = load i64, ptr addrspace(5) %"1423", align 8
  store i64 %"1478", ptr addrspace(5) %"668", align 8
  %"739" = load i64, ptr addrspace(5) %"668", align 8
  %"1480" = inttoptr i64 %"739" to ptr
  %"1508" = getelementptr inbounds i8, ptr %"1480", i64 36
  %"738" = load float, ptr %"1508", align 4
  store float %"738", ptr addrspace(5) %"448", align 4
  %"741" = load i64, ptr addrspace(5) %"667", align 8
  %"1481" = inttoptr i64 %"741" to ptr
  %"1510" = getelementptr inbounds i8, ptr %"1481", i64 36
  %"740" = load float, ptr %"1510", align 4
  store float %"740", ptr addrspace(5) %"449", align 4
  %"743" = load i64, ptr addrspace(5) %"668", align 8
  %"1482" = inttoptr i64 %"743" to ptr
  %"1512" = getelementptr inbounds i8, ptr %"1482", i64 40
  %"742" = load float, ptr %"1512", align 4
  store float %"742", ptr addrspace(5) %"450", align 4
  %"745" = load i64, ptr addrspace(5) %"667", align 8
  %"1483" = inttoptr i64 %"745" to ptr
  %"1514" = getelementptr inbounds i8, ptr %"1483", i64 40
  %"744" = load float, ptr %"1514", align 4
  store float %"744", ptr addrspace(5) %"451", align 4
  %"747" = load float, ptr addrspace(5) %"451", align 4
  %"748" = load float, ptr addrspace(5) %"450", align 4
  %"746" = fmul float %"747", %"748"
  store float %"746", ptr addrspace(5) %"497", align 4
  %"750" = load float, ptr addrspace(5) %"449", align 4
  %"751" = load float, ptr addrspace(5) %"448", align 4
  %"752" = load float, ptr addrspace(5) %"497", align 4
  %"749" = call float @llvm.fma.f32(float %"750", float %"751", float %"752")
  store float %"749", ptr addrspace(5) %"498", align 4
  %"754" = load i64, ptr addrspace(5) %"668", align 8
  %"1484" = inttoptr i64 %"754" to ptr
  %"1516" = getelementptr inbounds i8, ptr %"1484", i64 44
  %"753" = load float, ptr %"1516", align 4
  store float %"753", ptr addrspace(5) %"452", align 4
  %"756" = load i64, ptr addrspace(5) %"667", align 8
  %"1485" = inttoptr i64 %"756" to ptr
  %"1518" = getelementptr inbounds i8, ptr %"1485", i64 44
  %"755" = load float, ptr %"1518", align 4
  store float %"755", ptr addrspace(5) %"453", align 4
  %"758" = load float, ptr addrspace(5) %"453", align 4
  %"759" = load float, ptr addrspace(5) %"452", align 4
  %"760" = load float, ptr addrspace(5) %"498", align 4
  %"757" = call float @llvm.fma.f32(float %"758", float %"759", float %"760")
  store float %"757", ptr addrspace(5) %"454", align 4
  %"762" = load i64, ptr addrspace(5) %"668", align 8
  %"1486" = inttoptr i64 %"762" to ptr
  %"1520" = getelementptr inbounds i8, ptr %"1486", i64 48
  %"761" = load float, ptr %"1520", align 4
  store float %"761", ptr addrspace(5) %"455", align 4
  %"764" = load i64, ptr addrspace(5) %"668", align 8
  %"1487" = inttoptr i64 %"764" to ptr
  %"1522" = getelementptr inbounds i8, ptr %"1487", i64 52
  %"763" = load float, ptr %"1522", align 4
  store float %"763", ptr addrspace(5) %"456", align 4
  %"766" = load float, ptr addrspace(5) %"451", align 4
  %"767" = load float, ptr addrspace(5) %"456", align 4
  %"765" = fmul float %"766", %"767"
  store float %"765", ptr addrspace(5) %"499", align 4
  %"769" = load float, ptr addrspace(5) %"449", align 4
  %"770" = load float, ptr addrspace(5) %"455", align 4
  %"771" = load float, ptr addrspace(5) %"499", align 4
  %"768" = call float @llvm.fma.f32(float %"769", float %"770", float %"771")
  store float %"768", ptr addrspace(5) %"500", align 4
  %"773" = load i64, ptr addrspace(5) %"668", align 8
  %"1488" = inttoptr i64 %"773" to ptr
  %"1524" = getelementptr inbounds i8, ptr %"1488", i64 56
  %"772" = load float, ptr %"1524", align 4
  store float %"772", ptr addrspace(5) %"457", align 4
  %"775" = load float, ptr addrspace(5) %"453", align 4
  %"776" = load float, ptr addrspace(5) %"457", align 4
  %"777" = load float, ptr addrspace(5) %"500", align 4
  %"774" = call float @llvm.fma.f32(float %"775", float %"776", float %"777")
  store float %"774", ptr addrspace(5) %"458", align 4
  %"779" = load float, ptr addrspace(5) %"454", align 4
  %"778" = fcmp ole float %"779", 0.000000e+00
  store i1 %"778", ptr addrspace(5) %"442", align 1
  %"781" = load float, ptr addrspace(5) %"458", align 4
  %"780" = fcmp ole float %"781", 0.000000e+00
  store i1 %"780", ptr addrspace(5) %"443", align 1
  %"783" = load i1, ptr addrspace(5) %"443", align 1
  %"784" = load i1, ptr addrspace(5) %"442", align 1
  %"782" = or i1 %"783", %"784"
  store i1 %"782", ptr addrspace(5) %"444", align 1
  %0 = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %0, align 4
  %"785" = load float, ptr addrspace(5) %0, align 4
  store float %"785", ptr addrspace(5) %"661", align 4
  %"787" = load float, ptr addrspace(5) %"661", align 4
  %1 = alloca float, align 4, addrspace(5)
  store float %"787", ptr addrspace(5) %1, align 4
  %"786" = load float, ptr addrspace(5) %1, align 4
  store float %"786", ptr addrspace(5) %"662", align 4
  %"789" = load float, ptr addrspace(5) %"661", align 4
  %2 = alloca float, align 4, addrspace(5)
  store float %"789", ptr addrspace(5) %2, align 4
  %"788" = load float, ptr addrspace(5) %2, align 4
  store float %"788", ptr addrspace(5) %"663", align 4
  %"790" = load i1, ptr addrspace(5) %"444", align 1
  br i1 %"790", label %"440", label %"670"

"670":                                            ; preds = %"1506"
  %"792" = load float, ptr addrspace(5) %"455", align 4
  %"793" = load float, ptr addrspace(5) %"448", align 4
  %"791" = fadd float %"792", %"793"
  store float %"791", ptr addrspace(5) %"504", align 4
  %"795" = load float, ptr addrspace(5) %"456", align 4
  %"796" = load float, ptr addrspace(5) %"450", align 4
  %"794" = fadd float %"795", %"796"
  store float %"794", ptr addrspace(5) %"505", align 4
  %"798" = load float, ptr addrspace(5) %"505", align 4
  %"799" = load float, ptr addrspace(5) %"505", align 4
  %"797" = fmul float %"798", %"799"
  store float %"797", ptr addrspace(5) %"506", align 4
  %"801" = load float, ptr addrspace(5) %"504", align 4
  %"802" = load float, ptr addrspace(5) %"504", align 4
  %"803" = load float, ptr addrspace(5) %"506", align 4
  %"800" = call float @llvm.fma.f32(float %"801", float %"802", float %"803")
  store float %"800", ptr addrspace(5) %"507", align 4
  %"805" = load float, ptr addrspace(5) %"457", align 4
  %"806" = load float, ptr addrspace(5) %"452", align 4
  %"804" = fadd float %"805", %"806"
  store float %"804", ptr addrspace(5) %"508", align 4
  %"808" = load float, ptr addrspace(5) %"508", align 4
  %"809" = load float, ptr addrspace(5) %"508", align 4
  %"810" = load float, ptr addrspace(5) %"507", align 4
  %"807" = call float @llvm.fma.f32(float %"808", float %"809", float %"810")
  store float %"807", ptr addrspace(5) %"509", align 4
  %"812" = load float, ptr addrspace(5) %"509", align 4
  %3 = call afn float @llvm.sqrt.f32(float %"812")
  %"811" = fdiv arcp afn float 1.000000e+00, %3
  store float %"811", ptr addrspace(5) %"510", align 4
  %"814" = load float, ptr addrspace(5) %"504", align 4
  %"815" = load float, ptr addrspace(5) %"510", align 4
  %"813" = fmul float %"814", %"815"
  store float %"813", ptr addrspace(5) %"511", align 4
  %"817" = load float, ptr addrspace(5) %"505", align 4
  %"818" = load float, ptr addrspace(5) %"510", align 4
  %"816" = fmul float %"817", %"818"
  store float %"816", ptr addrspace(5) %"512", align 4
  %"820" = load float, ptr addrspace(5) %"508", align 4
  %"821" = load float, ptr addrspace(5) %"510", align 4
  %"819" = fmul float %"820", %"821"
  store float %"819", ptr addrspace(5) %"513", align 4
  %"823" = load float, ptr addrspace(5) %"451", align 4
  %"824" = load float, ptr addrspace(5) %"512", align 4
  %"822" = fmul float %"823", %"824"
  store float %"822", ptr addrspace(5) %"514", align 4
  %"826" = load float, ptr addrspace(5) %"449", align 4
  %"827" = load float, ptr addrspace(5) %"511", align 4
  %"828" = load float, ptr addrspace(5) %"514", align 4
  %"825" = call float @llvm.fma.f32(float %"826", float %"827", float %"828")
  store float %"825", ptr addrspace(5) %"515", align 4
  %"830" = load float, ptr addrspace(5) %"453", align 4
  %"831" = load float, ptr addrspace(5) %"513", align 4
  %"832" = load float, ptr addrspace(5) %"515", align 4
  %"829" = call float @llvm.fma.f32(float %"830", float %"831", float %"832")
  store float %"829", ptr addrspace(5) %"459", align 4
  %"834" = load float, ptr addrspace(5) %"450", align 4
  %"835" = load float, ptr addrspace(5) %"512", align 4
  %"833" = fmul float %"834", %"835"
  store float %"833", ptr addrspace(5) %"516", align 4
  %"837" = load float, ptr addrspace(5) %"448", align 4
  %"838" = load float, ptr addrspace(5) %"511", align 4
  %"839" = load float, ptr addrspace(5) %"516", align 4
  %"836" = call float @llvm.fma.f32(float %"837", float %"838", float %"839")
  store float %"836", ptr addrspace(5) %"517", align 4
  %"841" = load float, ptr addrspace(5) %"452", align 4
  %"842" = load float, ptr addrspace(5) %"513", align 4
  %"843" = load float, ptr addrspace(5) %"517", align 4
  %"840" = call float @llvm.fma.f32(float %"841", float %"842", float %"843")
  store float %"840", ptr addrspace(5) %"460", align 4
  %"845" = load i64, ptr addrspace(5) %"666", align 8
  %"1489" = add i64 %"845", 4
  store i64 %"1489", ptr addrspace(5) %"665", align 8
  %"847" = load i64, ptr addrspace(5) %"666", align 8
  %"1491" = inttoptr i64 %"847" to ptr
  %"1526" = getelementptr inbounds i8, ptr %"1491", i64 4
  %"846" = load float, ptr %"1526", align 4
  store float %"846", ptr addrspace(5) %"461", align 4
  %"849" = load i64, ptr addrspace(5) %"666", align 8
  %"1492" = inttoptr i64 %"849" to ptr
  %"1528" = getelementptr inbounds i8, ptr %"1492", i64 8
  %"848" = load float, ptr %"1528", align 4
  store float %"848", ptr addrspace(5) %"462", align 4
  %"851" = load float, ptr addrspace(5) %"462", align 4
  %"850" = fmul float %"851", 0x3FE3333340000000
  store float %"850", ptr addrspace(5) %"518", align 4
  %"853" = load float, ptr addrspace(5) %"461", align 4
  %"854" = load float, ptr addrspace(5) %"518", align 4
  %"852" = call float @llvm.fma.f32(float %"853", float 0x3FD3333340000000, float %"854")
  store float %"852", ptr addrspace(5) %"519", align 4
  %"856" = load i64, ptr addrspace(5) %"666", align 8
  %"1493" = inttoptr i64 %"856" to ptr
  %"1530" = getelementptr inbounds i8, ptr %"1493", i64 12
  %"855" = load float, ptr %"1530", align 4
  store float %"855", ptr addrspace(5) %"463", align 4
  %"858" = load float, ptr addrspace(5) %"463", align 4
  %"859" = load float, ptr addrspace(5) %"519", align 4
  %"857" = call float @llvm.fma.f32(float %"858", float 0x3FB99999A0000000, float %"859")
  store float %"857", ptr addrspace(5) %"464", align 4
  %"861" = load float, ptr addrspace(5) %"464", align 4
  %"860" = fcmp ule float %"861", 0.000000e+00
  store i1 %"860", ptr addrspace(5) %"445", align 1
  %4 = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %4, align 4
  %"862" = load float, ptr addrspace(5) %4, align 4
  store float %"862", ptr addrspace(5) %"503", align 4
  %"864" = load float, ptr addrspace(5) %"503", align 4
  %5 = alloca float, align 4, addrspace(5)
  store float %"864", ptr addrspace(5) %5, align 4
  %"863" = load float, ptr addrspace(5) %5, align 4
  store float %"863", ptr addrspace(5) %"657", align 4
  %"866" = load float, ptr addrspace(5) %"503", align 4
  %6 = alloca float, align 4, addrspace(5)
  store float %"866", ptr addrspace(5) %6, align 4
  %"865" = load float, ptr addrspace(5) %6, align 4
  store float %"865", ptr addrspace(5) %"658", align 4
  %"868" = load float, ptr addrspace(5) %"503", align 4
  %7 = alloca float, align 4, addrspace(5)
  store float %"868", ptr addrspace(5) %7, align 4
  %"867" = load float, ptr addrspace(5) %7, align 4
  store float %"867", ptr addrspace(5) %"659", align 4
  %"869" = load i1, ptr addrspace(5) %"445", align 1
  br i1 %"869", label %"438", label %"672"

"672":                                            ; preds = %"670"
  %"871" = load float, ptr addrspace(5) %"464", align 4
  %"870" = fdiv arcp afn float 1.000000e+00, %"871"
  store float %"870", ptr addrspace(5) %"520", align 4
  %"873" = load float, ptr addrspace(5) %"461", align 4
  %"874" = load float, ptr addrspace(5) %"520", align 4
  %"872" = call float @llvm.fma.f32(float %"873", float %"874", float -1.000000e+00)
  store float %"872", ptr addrspace(5) %"657", align 4
  %"876" = load float, ptr addrspace(5) %"462", align 4
  %"877" = load float, ptr addrspace(5) %"520", align 4
  %"875" = call float @llvm.fma.f32(float %"876", float %"877", float -1.000000e+00)
  store float %"875", ptr addrspace(5) %"658", align 4
  %"879" = load float, ptr addrspace(5) %"463", align 4
  %"880" = load float, ptr addrspace(5) %"520", align 4
  %"878" = call float @llvm.fma.f32(float %"879", float %"880", float -1.000000e+00)
  store float %"878", ptr addrspace(5) %"659", align 4
  br label %"438"

"438":                                            ; preds = %"672", %"670"
  %"882" = load i64, ptr addrspace(5) %"665", align 8
  %"1494" = inttoptr i64 %"882" to ptr
  %"1532" = getelementptr inbounds i8, ptr %"1494", i64 32
  %"881" = load float, ptr %"1532", align 4
  store float %"881", ptr addrspace(5) %"522", align 4
  %"884" = load float, ptr addrspace(5) %"522", align 4
  %"883" = fmul float %"884", 0x3FB47AE140000000
  store float %"883", ptr addrspace(5) %"523", align 4
  %"886" = load i64, ptr addrspace(5) %"665", align 8
  %"1495" = inttoptr i64 %"886" to ptr
  %"1534" = getelementptr inbounds i8, ptr %"1495", i64 40
  %"885" = load float, ptr %"1534", align 4
  store float %"885", ptr addrspace(5) %"524", align 4
  %"888" = load float, ptr addrspace(5) %"657", align 4
  %"889" = load float, ptr addrspace(5) %"524", align 4
  %"887" = call float @llvm.fma.f32(float %"888", float %"889", float 1.000000e+00)
  store float %"887", ptr addrspace(5) %"525", align 4
  %8 = alloca float, align 4, addrspace(5)
  store float 1.000000e+00, ptr addrspace(5) %8, align 4
  %"890" = load float, ptr addrspace(5) %8, align 4
  store float %"890", ptr addrspace(5) %"526", align 4
  %"892" = load float, ptr addrspace(5) %"658", align 4
  %"893" = load float, ptr addrspace(5) %"524", align 4
  %"891" = call float @llvm.fma.f32(float %"892", float %"893", float 1.000000e+00)
  store float %"891", ptr addrspace(5) %"527", align 4
  %"895" = load float, ptr addrspace(5) %"659", align 4
  %"896" = load float, ptr addrspace(5) %"524", align 4
  %"894" = call float @llvm.fma.f32(float %"895", float %"896", float 1.000000e+00)
  store float %"894", ptr addrspace(5) %"528", align 4
  %"898" = load float, ptr addrspace(5) %"523", align 4
  %"899" = load float, ptr addrspace(5) %"525", align 4
  %"897" = fmul float %"898", %"899"
  store float %"897", ptr addrspace(5) %"529", align 4
  %"901" = load float, ptr addrspace(5) %"523", align 4
  %"902" = load float, ptr addrspace(5) %"527", align 4
  %"900" = fmul float %"901", %"902"
  store float %"900", ptr addrspace(5) %"530", align 4
  %"904" = load float, ptr addrspace(5) %"523", align 4
  %"905" = load float, ptr addrspace(5) %"528", align 4
  %"903" = fmul float %"904", %"905"
  store float %"903", ptr addrspace(5) %"531", align 4
  %"907" = load float, ptr addrspace(5) %"461", align 4
  %"908" = load float, ptr addrspace(5) %"529", align 4
  %"906" = fsub float %"907", %"908"
  store float %"906", ptr addrspace(5) %"532", align 4
  %"910" = load float, ptr addrspace(5) %"462", align 4
  %"911" = load float, ptr addrspace(5) %"530", align 4
  %"909" = fsub float %"910", %"911"
  store float %"909", ptr addrspace(5) %"533", align 4
  %"913" = load float, ptr addrspace(5) %"463", align 4
  %"914" = load float, ptr addrspace(5) %"531", align 4
  %"912" = fsub float %"913", %"914"
  store float %"912", ptr addrspace(5) %"534", align 4
  %"916" = load i64, ptr addrspace(5) %"665", align 8
  %"1496" = inttoptr i64 %"916" to ptr
  %"1536" = getelementptr inbounds i8, ptr %"1496", i64 24
  %"915" = load float, ptr %"1536", align 4
  store float %"915", ptr addrspace(5) %"471", align 4
  %"918" = load float, ptr addrspace(5) %"471", align 4
  %"919" = load float, ptr addrspace(5) %"532", align 4
  %"920" = load float, ptr addrspace(5) %"529", align 4
  %"917" = call float @llvm.fma.f32(float %"918", float %"919", float %"920")
  store float %"917", ptr addrspace(5) %"535", align 4
  %"922" = load float, ptr addrspace(5) %"471", align 4
  %"923" = load float, ptr addrspace(5) %"533", align 4
  %"924" = load float, ptr addrspace(5) %"530", align 4
  %"921" = call float @llvm.fma.f32(float %"922", float %"923", float %"924")
  store float %"921", ptr addrspace(5) %"536", align 4
  %"926" = load float, ptr addrspace(5) %"471", align 4
  %"927" = load float, ptr addrspace(5) %"534", align 4
  %"928" = load float, ptr addrspace(5) %"531", align 4
  %"925" = call float @llvm.fma.f32(float %"926", float %"927", float %"928")
  store float %"925", ptr addrspace(5) %"537", align 4
  %"930" = load i64, ptr addrspace(5) %"665", align 8
  %"1497" = inttoptr i64 %"930" to ptr
  %"1538" = getelementptr inbounds i8, ptr %"1497", i64 52
  %"929" = load float, ptr %"1538", align 4
  store float %"929", ptr addrspace(5) %"538", align 4
  %"932" = load float, ptr addrspace(5) %"657", align 4
  %"933" = load float, ptr addrspace(5) %"538", align 4
  %"931" = call float @llvm.fma.f32(float %"932", float %"933", float 1.000000e+00)
  store float %"931", ptr addrspace(5) %"539", align 4
  %"935" = load float, ptr addrspace(5) %"658", align 4
  %"936" = load float, ptr addrspace(5) %"538", align 4
  %"934" = call float @llvm.fma.f32(float %"935", float %"936", float 1.000000e+00)
  store float %"934", ptr addrspace(5) %"540", align 4
  %"938" = load float, ptr addrspace(5) %"659", align 4
  %"939" = load float, ptr addrspace(5) %"538", align 4
  %"937" = call float @llvm.fma.f32(float %"938", float %"939", float 1.000000e+00)
  store float %"937", ptr addrspace(5) %"541", align 4
  %"941" = load float, ptr addrspace(5) %"526", align 4
  %"942" = load float, ptr addrspace(5) %"454", align 4
  %"940" = fsub float %"941", %"942"
  store float %"940", ptr addrspace(5) %"542", align 4
  %"944" = load float, ptr addrspace(5) %"542", align 4
  %"945" = load float, ptr addrspace(5) %"526", align 4
  %"943" = call float @llvm.minnum.f32(float %"944", float %"945")
  store float %"943", ptr addrspace(5) %"543", align 4
  %"947" = load float, ptr addrspace(5) %"503", align 4
  %"948" = load float, ptr addrspace(5) %"543", align 4
  %"946" = call float @llvm.maxnum.f32(float %"947", float %"948")
  store float %"946", ptr addrspace(5) %"545", align 4
  %"950" = load float, ptr addrspace(5) %"545", align 4
  %"951" = load float, ptr addrspace(5) %"545", align 4
  %"949" = fmul float %"950", %"951"
  store float %"949", ptr addrspace(5) %"546", align 4
  %"953" = load float, ptr addrspace(5) %"546", align 4
  %"954" = load float, ptr addrspace(5) %"546", align 4
  %"952" = fmul float %"953", %"954"
  store float %"952", ptr addrspace(5) %"547", align 4
  %"956" = load float, ptr addrspace(5) %"545", align 4
  %"957" = load float, ptr addrspace(5) %"547", align 4
  %"955" = fmul float %"956", %"957"
  store float %"955", ptr addrspace(5) %"548", align 4
  %"959" = load float, ptr addrspace(5) %"526", align 4
  %"960" = load float, ptr addrspace(5) %"458", align 4
  %"958" = fsub float %"959", %"960"
  store float %"958", ptr addrspace(5) %"549", align 4
  %"962" = load float, ptr addrspace(5) %"549", align 4
  %"963" = load float, ptr addrspace(5) %"526", align 4
  %"961" = call float @llvm.minnum.f32(float %"962", float %"963")
  store float %"961", ptr addrspace(5) %"550", align 4
  %"965" = load float, ptr addrspace(5) %"503", align 4
  %"966" = load float, ptr addrspace(5) %"550", align 4
  %"964" = call float @llvm.maxnum.f32(float %"965", float %"966")
  store float %"964", ptr addrspace(5) %"551", align 4
  %"968" = load float, ptr addrspace(5) %"551", align 4
  %"969" = load float, ptr addrspace(5) %"551", align 4
  %"967" = fmul float %"968", %"969"
  store float %"967", ptr addrspace(5) %"552", align 4
  %"971" = load float, ptr addrspace(5) %"552", align 4
  %"972" = load float, ptr addrspace(5) %"552", align 4
  %"970" = fmul float %"971", %"972"
  store float %"970", ptr addrspace(5) %"553", align 4
  %"974" = load float, ptr addrspace(5) %"551", align 4
  %"975" = load float, ptr addrspace(5) %"553", align 4
  %"973" = fmul float %"974", %"975"
  store float %"973", ptr addrspace(5) %"554", align 4
  %"977" = load float, ptr addrspace(5) %"460", align 4
  %"978" = load float, ptr addrspace(5) %"460", align 4
  %"976" = fadd float %"977", %"978"
  store float %"976", ptr addrspace(5) %"555", align 4
  %"980" = load float, ptr addrspace(5) %"460", align 4
  %"981" = load float, ptr addrspace(5) %"555", align 4
  %"979" = fmul float %"980", %"981"
  store float %"979", ptr addrspace(5) %"556", align 4
  %"983" = load i64, ptr addrspace(5) %"665", align 8
  %"1498" = inttoptr i64 %"983" to ptr
  %"1540" = getelementptr inbounds i8, ptr %"1498", i64 36
  %"982" = load float, ptr %"1540", align 4
  store float %"982", ptr addrspace(5) %"557", align 4
  %"985" = load float, ptr addrspace(5) %"556", align 4
  %"986" = load float, ptr addrspace(5) %"557", align 4
  %"984" = call float @llvm.fma.f32(float %"985", float %"986", float 5.000000e-01)
  store float %"984", ptr addrspace(5) %"558", align 4
  %"988" = load float, ptr addrspace(5) %"558", align 4
  %"987" = fadd float %"988", -1.000000e+00
  store float %"987", ptr addrspace(5) %"559", align 4
  %"990" = load float, ptr addrspace(5) %"548", align 4
  %"991" = load float, ptr addrspace(5) %"559", align 4
  %"989" = call float @llvm.fma.f32(float %"990", float %"991", float 1.000000e+00)
  store float %"989", ptr addrspace(5) %"560", align 4
  %"993" = load float, ptr addrspace(5) %"554", align 4
  %"994" = load float, ptr addrspace(5) %"559", align 4
  %"992" = call float @llvm.fma.f32(float %"993", float %"994", float 1.000000e+00)
  store float %"992", ptr addrspace(5) %"561", align 4
  %"996" = load float, ptr addrspace(5) %"560", align 4
  %"997" = load float, ptr addrspace(5) %"561", align 4
  %"995" = fmul float %"996", %"997"
  store float %"995", ptr addrspace(5) %"472", align 4
  %"999" = load float, ptr addrspace(5) %"460", align 4
  %"1000" = load float, ptr addrspace(5) %"460", align 4
  %"998" = fmul float %"999", %"1000"
  store float %"998", ptr addrspace(5) %"562", align 4
  %"1002" = load float, ptr addrspace(5) %"562", align 4
  %"1003" = load float, ptr addrspace(5) %"557", align 4
  %"1001" = call float @llvm.fma.f32(float %"1002", float %"1003", float -1.000000e+00)
  store float %"1001", ptr addrspace(5) %"563", align 4
  %"1005" = load float, ptr addrspace(5) %"548", align 4
  %"1006" = load float, ptr addrspace(5) %"563", align 4
  %"1004" = call float @llvm.fma.f32(float %"1005", float %"1006", float 1.000000e+00)
  store float %"1004", ptr addrspace(5) %"564", align 4
  %"1008" = load float, ptr addrspace(5) %"554", align 4
  %"1009" = load float, ptr addrspace(5) %"563", align 4
  %"1007" = call float @llvm.fma.f32(float %"1008", float %"1009", float 1.000000e+00)
  store float %"1007", ptr addrspace(5) %"565", align 4
  %"1011" = load float, ptr addrspace(5) %"564", align 4
  %"1012" = load float, ptr addrspace(5) %"565", align 4
  %"1010" = fmul float %"1011", %"1012"
  store float %"1010", ptr addrspace(5) %"566", align 4
  %"1014" = load float, ptr addrspace(5) %"458", align 4
  %"1015" = load float, ptr addrspace(5) %"454", align 4
  %"1013" = fadd float %"1014", %"1015"
  store float %"1013", ptr addrspace(5) %"567", align 4
  %"1017" = load float, ptr addrspace(5) %"567", align 4
  %"1016" = fdiv arcp afn float 1.000000e+00, %"1017"
  store float %"1016", ptr addrspace(5) %"568", align 4
  %"1019" = load float, ptr addrspace(5) %"568", align 4
  %"1018" = fadd float %"1019", -5.000000e-01
  store float %"1018", ptr addrspace(5) %"569", align 4
  %"1021" = load float, ptr addrspace(5) %"569", align 4
  %"1022" = load float, ptr addrspace(5) %"566", align 4
  %"1020" = call float @llvm.fma.f32(float %"1021", float %"1022", float 5.000000e-01)
  store float %"1020", ptr addrspace(5) %"570", align 4
  %"1024" = load float, ptr addrspace(5) %"570", align 4
  %"1023" = fmul float %"1024", 1.250000e+00
  store float %"1023", ptr addrspace(5) %"473", align 4
  %9 = alloca float, align 4, addrspace(5)
  store float 0x3F50624DE0000000, ptr addrspace(5) %9, align 4
  %"1025" = load float, ptr addrspace(5) %9, align 4
  store float %"1025", ptr addrspace(5) %"571", align 4
  %"1027" = load float, ptr addrspace(5) %"571", align 4
  %"1028" = load float, ptr addrspace(5) %"557", align 4
  %"1026" = call float @llvm.maxnum.f32(float %"1027", float %"1028")
  store float %"1026", ptr addrspace(5) %"572", align 4
  %"1030" = load float, ptr addrspace(5) %"572", align 4
  %"1031" = load float, ptr addrspace(5) %"572", align 4
  %"1029" = fmul float %"1030", %"1031"
  store float %"1029", ptr addrspace(5) %"573", align 4
  %"1033" = load float, ptr addrspace(5) %"573", align 4
  %"1032" = fadd float %"1033", -1.000000e+00
  store float %"1032", ptr addrspace(5) %"574", align 4
  %"1035" = load float, ptr addrspace(5) %"459", align 4
  %"1036" = load float, ptr addrspace(5) %"574", align 4
  %"1034" = fmul float %"1035", %"1036"
  store float %"1034", ptr addrspace(5) %"575", align 4
  %"1038" = load float, ptr addrspace(5) %"459", align 4
  %"1039" = load float, ptr addrspace(5) %"575", align 4
  %"1037" = call float @llvm.fma.f32(float %"1038", float %"1039", float 1.000000e+00)
  store float %"1037", ptr addrspace(5) %"576", align 4
  %"1041" = load float, ptr addrspace(5) %"576", align 4
  %"1040" = fmul float %"1041", 0x400921FB60000000
  store float %"1040", ptr addrspace(5) %"577", align 4
  %"1043" = load float, ptr addrspace(5) %"576", align 4
  %"1044" = load float, ptr addrspace(5) %"577", align 4
  %"1042" = fmul float %"1043", %"1044"
  store float %"1042", ptr addrspace(5) %"578", align 4
  %"1046" = load float, ptr addrspace(5) %"573", align 4
  %"1047" = load float, ptr addrspace(5) %"578", align 4
  %"1045" = fdiv arcp afn float %"1046", %"1047"
  store float %"1045", ptr addrspace(5) %"474", align 4
  %"1049" = load float, ptr addrspace(5) %"526", align 4
  %"1050" = load float, ptr addrspace(5) %"460", align 4
  %"1048" = fsub float %"1049", %"1050"
  store float %"1048", ptr addrspace(5) %"579", align 4
  %"1052" = load float, ptr addrspace(5) %"579", align 4
  %"1053" = load float, ptr addrspace(5) %"526", align 4
  %"1051" = call float @llvm.minnum.f32(float %"1052", float %"1053")
  store float %"1051", ptr addrspace(5) %"580", align 4
  %"1055" = load float, ptr addrspace(5) %"503", align 4
  %"1056" = load float, ptr addrspace(5) %"580", align 4
  %"1054" = call float @llvm.maxnum.f32(float %"1055", float %"1056")
  store float %"1054", ptr addrspace(5) %"581", align 4
  %"1058" = load float, ptr addrspace(5) %"581", align 4
  %"1059" = load float, ptr addrspace(5) %"581", align 4
  %"1057" = fmul float %"1058", %"1059"
  store float %"1057", ptr addrspace(5) %"582", align 4
  %"1061" = load float, ptr addrspace(5) %"582", align 4
  %"1062" = load float, ptr addrspace(5) %"582", align 4
  %"1060" = fmul float %"1061", %"1062"
  store float %"1060", ptr addrspace(5) %"583", align 4
  %"1064" = load float, ptr addrspace(5) %"581", align 4
  %"1065" = load float, ptr addrspace(5) %"583", align 4
  %"1063" = fmul float %"1064", %"1065"
  store float %"1063", ptr addrspace(5) %"475", align 4
  %"1067" = load float, ptr addrspace(5) %"526", align 4
  %"1068" = load float, ptr addrspace(5) %"535", align 4
  %"1066" = fsub float %"1067", %"1068"
  store float %"1066", ptr addrspace(5) %"584", align 4
  %"1070" = load float, ptr addrspace(5) %"526", align 4
  %"1071" = load float, ptr addrspace(5) %"536", align 4
  %"1069" = fsub float %"1070", %"1071"
  store float %"1069", ptr addrspace(5) %"585", align 4
  %"1073" = load float, ptr addrspace(5) %"526", align 4
  %"1074" = load float, ptr addrspace(5) %"537", align 4
  %"1072" = fsub float %"1073", %"1074"
  store float %"1072", ptr addrspace(5) %"586", align 4
  %"1076" = load float, ptr addrspace(5) %"584", align 4
  %"1077" = load float, ptr addrspace(5) %"475", align 4
  %"1078" = load float, ptr addrspace(5) %"535", align 4
  %"1075" = call float @llvm.fma.f32(float %"1076", float %"1077", float %"1078")
  store float %"1075", ptr addrspace(5) %"476", align 4
  %"1080" = load float, ptr addrspace(5) %"585", align 4
  %"1081" = load float, ptr addrspace(5) %"475", align 4
  %"1082" = load float, ptr addrspace(5) %"536", align 4
  %"1079" = call float @llvm.fma.f32(float %"1080", float %"1081", float %"1082")
  store float %"1079", ptr addrspace(5) %"477", align 4
  %"1084" = load float, ptr addrspace(5) %"586", align 4
  %"1085" = load float, ptr addrspace(5) %"475", align 4
  %"1086" = load float, ptr addrspace(5) %"537", align 4
  %"1083" = call float @llvm.fma.f32(float %"1084", float %"1085", float %"1086")
  store float %"1083", ptr addrspace(5) %"478", align 4
  %"1088" = load float, ptr addrspace(5) %"557", align 4
  %"1087" = call float @llvm.fma.f32(float %"1088", float 5.000000e-01, float 5.000000e-01)
  store float %"1087", ptr addrspace(5) %"587", align 4
  %"1090" = load float, ptr addrspace(5) %"587", align 4
  %"1091" = load float, ptr addrspace(5) %"587", align 4
  %"1089" = fmul float %"1090", %"1091"
  store float %"1089", ptr addrspace(5) %"588", align 4
  %"1093" = load float, ptr addrspace(5) %"588", align 4
  %"1094" = load float, ptr addrspace(5) %"588", align 4
  %"1092" = fmul float %"1093", %"1094"
  store float %"1092", ptr addrspace(5) %"589", align 4
  %"1096" = load float, ptr addrspace(5) %"454", align 4
  %"1097" = load float, ptr addrspace(5) %"454", align 4
  %"1095" = fmul float %"1096", %"1097"
  store float %"1095", ptr addrspace(5) %"479", align 4
  %"1099" = load float, ptr addrspace(5) %"479", align 4
  %"1100" = load float, ptr addrspace(5) %"589", align 4
  %"1098" = fadd float %"1099", %"1100"
  store float %"1098", ptr addrspace(5) %"590", align 4
  %"1102" = load float, ptr addrspace(5) %"479", align 4
  %"1103" = load float, ptr addrspace(5) %"589", align 4
  %"1101" = fmul float %"1102", %"1103"
  store float %"1101", ptr addrspace(5) %"591", align 4
  %"1105" = load float, ptr addrspace(5) %"590", align 4
  %"1106" = load float, ptr addrspace(5) %"591", align 4
  %"1104" = fsub float %"1105", %"1106"
  store float %"1104", ptr addrspace(5) %"592", align 4
  %"1108" = load float, ptr addrspace(5) %"592", align 4
  %"1107" = call afn float @llvm.sqrt.f32(float %"1108")
  store float %"1107", ptr addrspace(5) %"593", align 4
  %"1110" = load float, ptr addrspace(5) %"454", align 4
  %"1111" = load float, ptr addrspace(5) %"593", align 4
  %"1109" = fadd float %"1110", %"1111"
  store float %"1109", ptr addrspace(5) %"594", align 4
  %"1113" = load float, ptr addrspace(5) %"594", align 4
  %"1112" = fdiv arcp afn float 1.000000e+00, %"1113"
  store float %"1112", ptr addrspace(5) %"595", align 4
  %"1115" = load float, ptr addrspace(5) %"458", align 4
  %"1116" = load float, ptr addrspace(5) %"458", align 4
  %"1114" = fmul float %"1115", %"1116"
  store float %"1114", ptr addrspace(5) %"480", align 4
  %"1118" = load float, ptr addrspace(5) %"480", align 4
  %"1119" = load float, ptr addrspace(5) %"589", align 4
  %"1117" = fadd float %"1118", %"1119"
  store float %"1117", ptr addrspace(5) %"596", align 4
  %"1121" = load float, ptr addrspace(5) %"480", align 4
  %"1122" = load float, ptr addrspace(5) %"589", align 4
  %"1120" = fmul float %"1121", %"1122"
  store float %"1120", ptr addrspace(5) %"597", align 4
  %"1124" = load float, ptr addrspace(5) %"596", align 4
  %"1125" = load float, ptr addrspace(5) %"597", align 4
  %"1123" = fsub float %"1124", %"1125"
  store float %"1123", ptr addrspace(5) %"598", align 4
  %"1127" = load float, ptr addrspace(5) %"598", align 4
  %"1126" = call afn float @llvm.sqrt.f32(float %"1127")
  store float %"1126", ptr addrspace(5) %"599", align 4
  %"1129" = load float, ptr addrspace(5) %"458", align 4
  %"1130" = load float, ptr addrspace(5) %"599", align 4
  %"1128" = fadd float %"1129", %"1130"
  store float %"1128", ptr addrspace(5) %"600", align 4
  %"1132" = load float, ptr addrspace(5) %"600", align 4
  %"1131" = fdiv arcp afn float 1.000000e+00, %"1132"
  store float %"1131", ptr addrspace(5) %"601", align 4
  %"1134" = load float, ptr addrspace(5) %"595", align 4
  %"1135" = load float, ptr addrspace(5) %"601", align 4
  %"1133" = fmul float %"1134", %"1135"
  store float %"1133", ptr addrspace(5) %"481", align 4
  %"1137" = load i64, ptr addrspace(5) %"665", align 8
  %"1499" = inttoptr i64 %"1137" to ptr
  %"1542" = getelementptr inbounds i8, ptr %"1499", i64 48
  %"1136" = load float, ptr %"1542", align 4
  store float %"1136", ptr addrspace(5) %"602", align 4
  %"1139" = load float, ptr addrspace(5) %"475", align 4
  %"1140" = load float, ptr addrspace(5) %"602", align 4
  %"1138" = fmul float %"1139", %"1140"
  store float %"1138", ptr addrspace(5) %"603", align 4
  %"1142" = load float, ptr addrspace(5) %"539", align 4
  %"1143" = load float, ptr addrspace(5) %"603", align 4
  %"1141" = fmul float %"1142", %"1143"
  store float %"1141", ptr addrspace(5) %"482", align 4
  %"1145" = load float, ptr addrspace(5) %"540", align 4
  %"1146" = load float, ptr addrspace(5) %"603", align 4
  %"1144" = fmul float %"1145", %"1146"
  store float %"1144", ptr addrspace(5) %"483", align 4
  %"1148" = load float, ptr addrspace(5) %"541", align 4
  %"1149" = load float, ptr addrspace(5) %"603", align 4
  %"1147" = fmul float %"1148", %"1149"
  store float %"1147", ptr addrspace(5) %"484", align 4
  %"1151" = load i64, ptr addrspace(5) %"665", align 8
  %"1500" = inttoptr i64 %"1151" to ptr
  %"1544" = getelementptr inbounds i8, ptr %"1500", i64 60
  %"1150" = load float, ptr %"1544", align 4
  store float %"1150", ptr addrspace(5) %"604", align 4
  %"1153" = load float, ptr addrspace(5) %"571", align 4
  %"1152" = fsub float %"1153", 0x3FB99999A0000000
  store float %"1152", ptr addrspace(5) %"605", align 4
  %"1155" = load float, ptr addrspace(5) %"604", align 4
  %"1156" = load float, ptr addrspace(5) %"605", align 4
  %"1154" = call float @llvm.fma.f32(float %"1155", float %"1156", float 0x3FB99999A0000000)
  store float %"1154", ptr addrspace(5) %"485", align 4
  %"1158" = load float, ptr addrspace(5) %"485", align 4
  %"1157" = fcmp oge float %"1158", 1.000000e+00
  store i1 %"1157", ptr addrspace(5) %"446", align 1
  %10 = alloca float, align 4, addrspace(5)
  store float 0x3FD45F3060000000, ptr addrspace(5) %10, align 4
  %"1159" = load float, ptr addrspace(5) %10, align 4
  store float %"1159", ptr addrspace(5) %"660", align 4
  %"1160" = load i1, ptr addrspace(5) %"446", align 1
  br i1 %"1160", label %"439", label %"674"

"674":                                            ; preds = %"438"
  %"1162" = load float, ptr addrspace(5) %"485", align 4
  %"1163" = load float, ptr addrspace(5) %"485", align 4
  %"1161" = fmul float %"1162", %"1163"
  store float %"1161", ptr addrspace(5) %"606", align 4
  %"1165" = load float, ptr addrspace(5) %"606", align 4
  %"1164" = fadd float %"1165", -1.000000e+00
  store float %"1164", ptr addrspace(5) %"607", align 4
  %"1167" = load float, ptr addrspace(5) %"459", align 4
  %"1168" = load float, ptr addrspace(5) %"607", align 4
  %"1166" = fmul float %"1167", %"1168"
  store float %"1166", ptr addrspace(5) %"608", align 4
  %"1170" = load float, ptr addrspace(5) %"459", align 4
  %"1171" = load float, ptr addrspace(5) %"608", align 4
  %"1169" = call float @llvm.fma.f32(float %"1170", float %"1171", float 1.000000e+00)
  store float %"1169", ptr addrspace(5) %"609", align 4
  %"1173" = load float, ptr addrspace(5) %"606", align 4
  %"1172" = call afn float @llvm.log2.f32(float %"1173")
  store float %"1172", ptr addrspace(5) %"610", align 4
  %"1175" = load float, ptr addrspace(5) %"610", align 4
  %"1174" = fmul float %"1175", 0x3FE62E4300000000
  store float %"1174", ptr addrspace(5) %"611", align 4
  %"1177" = load float, ptr addrspace(5) %"611", align 4
  %"1176" = fmul float %"1177", 0x400921FB60000000
  store float %"1176", ptr addrspace(5) %"612", align 4
  %"1179" = load float, ptr addrspace(5) %"609", align 4
  %"1180" = load float, ptr addrspace(5) %"612", align 4
  %"1178" = fmul float %"1179", %"1180"
  store float %"1178", ptr addrspace(5) %"613", align 4
  %"1182" = load float, ptr addrspace(5) %"607", align 4
  %"1183" = load float, ptr addrspace(5) %"613", align 4
  %"1181" = fdiv arcp afn float %"1182", %"1183"
  store float %"1181", ptr addrspace(5) %"660", align 4
  br label %"439"

"439":                                            ; preds = %"674", %"438"
  %"1185" = load float, ptr addrspace(5) %"526", align 4
  %"1184" = fsub float %"1185", 0x3FA47AE140000000
  store float %"1184", ptr addrspace(5) %"615", align 4
  %"1187" = load float, ptr addrspace(5) %"475", align 4
  %"1188" = load float, ptr addrspace(5) %"615", align 4
  %"1186" = call float @llvm.fma.f32(float %"1187", float %"1188", float 0x3FA47AE140000000)
  store float %"1186", ptr addrspace(5) %"616", align 4
  %"1190" = load float, ptr addrspace(5) %"479", align 4
  %"1189" = fmul float %"1190", 6.250000e-02
  store float %"1189", ptr addrspace(5) %"617", align 4
  %"1192" = load float, ptr addrspace(5) %"479", align 4
  %"1191" = fadd float %"1192", 6.250000e-02
  store float %"1191", ptr addrspace(5) %"618", align 4
  %"1194" = load float, ptr addrspace(5) %"618", align 4
  %"1195" = load float, ptr addrspace(5) %"617", align 4
  %"1193" = fsub float %"1194", %"1195"
  store float %"1193", ptr addrspace(5) %"619", align 4
  %"1197" = load float, ptr addrspace(5) %"619", align 4
  %"1196" = call afn float @llvm.sqrt.f32(float %"1197")
  store float %"1196", ptr addrspace(5) %"620", align 4
  %"1199" = load float, ptr addrspace(5) %"454", align 4
  %"1200" = load float, ptr addrspace(5) %"620", align 4
  %"1198" = fadd float %"1199", %"1200"
  store float %"1198", ptr addrspace(5) %"621", align 4
  %"1202" = load float, ptr addrspace(5) %"621", align 4
  %"1201" = fdiv arcp afn float 1.000000e+00, %"1202"
  store float %"1201", ptr addrspace(5) %"622", align 4
  %"1204" = load float, ptr addrspace(5) %"480", align 4
  %"1203" = fmul float %"1204", 6.250000e-02
  store float %"1203", ptr addrspace(5) %"623", align 4
  %"1206" = load float, ptr addrspace(5) %"480", align 4
  %"1205" = fadd float %"1206", 6.250000e-02
  store float %"1205", ptr addrspace(5) %"624", align 4
  %"1208" = load float, ptr addrspace(5) %"624", align 4
  %"1209" = load float, ptr addrspace(5) %"623", align 4
  %"1207" = fsub float %"1208", %"1209"
  store float %"1207", ptr addrspace(5) %"625", align 4
  %"1211" = load float, ptr addrspace(5) %"625", align 4
  %"1210" = call afn float @llvm.sqrt.f32(float %"1211")
  store float %"1210", ptr addrspace(5) %"626", align 4
  %"1213" = load float, ptr addrspace(5) %"458", align 4
  %"1214" = load float, ptr addrspace(5) %"626", align 4
  %"1212" = fadd float %"1213", %"1214"
  store float %"1212", ptr addrspace(5) %"627", align 4
  %"1216" = load float, ptr addrspace(5) %"627", align 4
  %"1215" = fdiv arcp afn float 1.000000e+00, %"1216"
  store float %"1215", ptr addrspace(5) %"628", align 4
  %"1218" = load float, ptr addrspace(5) %"622", align 4
  %"1219" = load float, ptr addrspace(5) %"628", align 4
  %"1217" = fmul float %"1218", %"1219"
  store float %"1217", ptr addrspace(5) %"629", align 4
  %"1221" = load float, ptr addrspace(5) %"473", align 4
  %"1222" = load float, ptr addrspace(5) %"472", align 4
  %"1220" = fsub float %"1221", %"1222"
  store float %"1220", ptr addrspace(5) %"630", align 4
  %"1224" = load i64, ptr addrspace(5) %"665", align 8
  %"1501" = inttoptr i64 %"1224" to ptr
  %"1546" = getelementptr inbounds i8, ptr %"1501", i64 28
  %"1223" = load float, ptr %"1546", align 4
  store float %"1223", ptr addrspace(5) %"631", align 4
  %"1226" = load float, ptr addrspace(5) %"630", align 4
  %"1227" = load float, ptr addrspace(5) %"631", align 4
  %"1228" = load float, ptr addrspace(5) %"472", align 4
  %"1225" = call float @llvm.fma.f32(float %"1226", float %"1227", float %"1228")
  store float %"1225", ptr addrspace(5) %"632", align 4
  %"1230" = load float, ptr addrspace(5) %"632", align 4
  %"1229" = fmul float %"1230", 0x3FD45F3060000000
  store float %"1229", ptr addrspace(5) %"633", align 4
  %"1232" = load float, ptr addrspace(5) %"461", align 4
  %"1233" = load float, ptr addrspace(5) %"633", align 4
  %"1234" = load float, ptr addrspace(5) %"482", align 4
  %"1231" = call float @llvm.fma.f32(float %"1232", float %"1233", float %"1234")
  store float %"1231", ptr addrspace(5) %"634", align 4
  %"1236" = load float, ptr addrspace(5) %"462", align 4
  %"1237" = load float, ptr addrspace(5) %"633", align 4
  %"1238" = load float, ptr addrspace(5) %"483", align 4
  %"1235" = call float @llvm.fma.f32(float %"1236", float %"1237", float %"1238")
  store float %"1235", ptr addrspace(5) %"635", align 4
  %"1240" = load float, ptr addrspace(5) %"463", align 4
  %"1241" = load float, ptr addrspace(5) %"633", align 4
  %"1242" = load float, ptr addrspace(5) %"484", align 4
  %"1239" = call float @llvm.fma.f32(float %"1240", float %"1241", float %"1242")
  store float %"1239", ptr addrspace(5) %"636", align 4
  %"1244" = load float, ptr addrspace(5) %"526", align 4
  %"1245" = load float, ptr addrspace(5) %"471", align 4
  %"1243" = fsub float %"1244", %"1245"
  store float %"1243", ptr addrspace(5) %"637", align 4
  %"1247" = load float, ptr addrspace(5) %"637", align 4
  %"1248" = load float, ptr addrspace(5) %"634", align 4
  %"1246" = fmul float %"1247", %"1248"
  store float %"1246", ptr addrspace(5) %"638", align 4
  %"1250" = load float, ptr addrspace(5) %"637", align 4
  %"1251" = load float, ptr addrspace(5) %"635", align 4
  %"1249" = fmul float %"1250", %"1251"
  store float %"1249", ptr addrspace(5) %"639", align 4
  %"1253" = load float, ptr addrspace(5) %"637", align 4
  %"1254" = load float, ptr addrspace(5) %"636", align 4
  %"1252" = fmul float %"1253", %"1254"
  store float %"1252", ptr addrspace(5) %"640", align 4
  %"1256" = load float, ptr addrspace(5) %"476", align 4
  %"1257" = load float, ptr addrspace(5) %"481", align 4
  %"1255" = fmul float %"1256", %"1257"
  store float %"1255", ptr addrspace(5) %"641", align 4
  %"1259" = load float, ptr addrspace(5) %"477", align 4
  %"1260" = load float, ptr addrspace(5) %"481", align 4
  %"1258" = fmul float %"1259", %"1260"
  store float %"1258", ptr addrspace(5) %"642", align 4
  %"1262" = load float, ptr addrspace(5) %"478", align 4
  %"1263" = load float, ptr addrspace(5) %"481", align 4
  %"1261" = fmul float %"1262", %"1263"
  store float %"1261", ptr addrspace(5) %"643", align 4
  %"1265" = load float, ptr addrspace(5) %"474", align 4
  %"1266" = load float, ptr addrspace(5) %"641", align 4
  %"1267" = load float, ptr addrspace(5) %"638", align 4
  %"1264" = call float @llvm.fma.f32(float %"1265", float %"1266", float %"1267")
  store float %"1264", ptr addrspace(5) %"644", align 4
  %"1269" = load float, ptr addrspace(5) %"474", align 4
  %"1270" = load float, ptr addrspace(5) %"642", align 4
  %"1271" = load float, ptr addrspace(5) %"639", align 4
  %"1268" = call float @llvm.fma.f32(float %"1269", float %"1270", float %"1271")
  store float %"1268", ptr addrspace(5) %"645", align 4
  %"1273" = load float, ptr addrspace(5) %"474", align 4
  %"1274" = load float, ptr addrspace(5) %"643", align 4
  %"1275" = load float, ptr addrspace(5) %"640", align 4
  %"1272" = call float @llvm.fma.f32(float %"1273", float %"1274", float %"1275")
  store float %"1272", ptr addrspace(5) %"646", align 4
  %"1277" = load i64, ptr addrspace(5) %"665", align 8
  %"1502" = inttoptr i64 %"1277" to ptr
  %"1548" = getelementptr inbounds i8, ptr %"1502", i64 56
  %"1276" = load float, ptr %"1548", align 4
  store float %"1276", ptr addrspace(5) %"647", align 4
  %"1279" = load float, ptr addrspace(5) %"647", align 4
  %"1278" = fmul float %"1279", 2.500000e-01
  store float %"1278", ptr addrspace(5) %"648", align 4
  %"1281" = load float, ptr addrspace(5) %"629", align 4
  %"1282" = load float, ptr addrspace(5) %"648", align 4
  %"1280" = fmul float %"1281", %"1282"
  store float %"1280", ptr addrspace(5) %"649", align 4
  %"1284" = load float, ptr addrspace(5) %"616", align 4
  %"1285" = load float, ptr addrspace(5) %"649", align 4
  %"1283" = fmul float %"1284", %"1285"
  store float %"1283", ptr addrspace(5) %"650", align 4
  %"1287" = load float, ptr addrspace(5) %"660", align 4
  %"1288" = load float, ptr addrspace(5) %"650", align 4
  %"1289" = load float, ptr addrspace(5) %"644", align 4
  %"1286" = call float @llvm.fma.f32(float %"1287", float %"1288", float %"1289")
  store float %"1286", ptr addrspace(5) %"651", align 4
  %"1291" = load float, ptr addrspace(5) %"660", align 4
  %"1292" = load float, ptr addrspace(5) %"650", align 4
  %"1293" = load float, ptr addrspace(5) %"645", align 4
  %"1290" = call float @llvm.fma.f32(float %"1291", float %"1292", float %"1293")
  store float %"1290", ptr addrspace(5) %"652", align 4
  %"1295" = load float, ptr addrspace(5) %"660", align 4
  %"1296" = load float, ptr addrspace(5) %"650", align 4
  %"1297" = load float, ptr addrspace(5) %"646", align 4
  %"1294" = call float @llvm.fma.f32(float %"1295", float %"1296", float %"1297")
  store float %"1294", ptr addrspace(5) %"653", align 4
  %"1299" = load float, ptr addrspace(5) %"454", align 4
  %"1300" = load float, ptr addrspace(5) %"526", align 4
  %"1298" = call float @llvm.minnum.f32(float %"1299", float %"1300")
  store float %"1298", ptr addrspace(5) %"654", align 4
  %11 = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %11, align 4
  %"1301" = load float, ptr addrspace(5) %11, align 4
  store float %"1301", ptr addrspace(5) %"655", align 4
  %"1303" = load float, ptr addrspace(5) %"655", align 4
  %"1304" = load float, ptr addrspace(5) %"654", align 4
  %"1302" = call float @llvm.maxnum.f32(float %"1303", float %"1304")
  store float %"1302", ptr addrspace(5) %"656", align 4
  %"1306" = load float, ptr addrspace(5) %"656", align 4
  %"1307" = load float, ptr addrspace(5) %"653", align 4
  %"1305" = fmul float %"1306", %"1307"
  store float %"1305", ptr addrspace(5) %"663", align 4
  %"1309" = load float, ptr addrspace(5) %"656", align 4
  %"1310" = load float, ptr addrspace(5) %"652", align 4
  %"1308" = fmul float %"1309", %"1310"
  store float %"1308", ptr addrspace(5) %"662", align 4
  %"1312" = load float, ptr addrspace(5) %"656", align 4
  %"1313" = load float, ptr addrspace(5) %"651", align 4
  %"1311" = fmul float %"1312", %"1313"
  store float %"1311", ptr addrspace(5) %"661", align 4
  br label %"440"

"440":                                            ; preds = %"439", %"1506"
  %"1314" = load float, ptr addrspace(5) %"661", align 4
  %"1550" = getelementptr inbounds i8, ptr addrspace(5) %"1420", i64 0
  store float %"1314", ptr addrspace(5) %"1550", align 4
  %"1315" = load float, ptr addrspace(5) %"662", align 4
  %"1552" = getelementptr inbounds i8, ptr addrspace(5) %"1420", i64 4
  store float %"1315", ptr addrspace(5) %"1552", align 4
  %"1316" = load float, ptr addrspace(5) %"663", align 4
  %"1554" = getelementptr inbounds i8, ptr addrspace(5) %"1420", i64 8
  store float %"1316", ptr addrspace(5) %"1554", align 4
  %"1317" = load [3 x i32], ptr addrspace(5) %"1420", align 4
  store [3 x i32] %"1317", ptr addrspace(5) %"723", align 4
  %"1318" = load [3 x i32], ptr addrspace(5) %"723", align 4
  %12 = insertvalue { [3 x i32], i32 } undef, [3 x i32] %"1318", 0
  %13 = insertvalue { [3 x i32], i32 } %12, i32 0, 1
  ret { [3 x i32], i32 } %13
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.fma.f32(float, float, float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.sqrt.f32(float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.minnum.f32(float, float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.maxnum.f32(float, float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.log2.f32(float) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
