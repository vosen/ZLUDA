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

define protected void @_Z6boundsiPf(i32 %"456", i64 %"457", ptr addrspace(3) %"553", ptr addrspace(1) %"554") #1 {
"613":
  %"590" = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %"590", align 4
  %"452" = alloca i32, align 4, addrspace(5)
  %"453" = alloca i64, align 8, addrspace(5)
  %"454" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"454", align 1
  %"455" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"455", align 1
  %"543" = alloca i32, align 4, addrspace(5)
  %"544" = alloca i64, align 8, addrspace(5)
  %"419" = alloca i1, align 1, addrspace(5)
  %"420" = alloca i1, align 1, addrspace(5)
  %"421" = alloca i1, align 1, addrspace(5)
  %"422" = alloca float, align 4, addrspace(5)
  %"423" = alloca float, align 4, addrspace(5)
  %"424" = alloca float, align 4, addrspace(5)
  %"425" = alloca float, align 4, addrspace(5)
  %"426" = alloca float, align 4, addrspace(5)
  %"427" = alloca float, align 4, addrspace(5)
  %"428" = alloca float, align 4, addrspace(5)
  %"429" = alloca float, align 4, addrspace(5)
  %"430" = alloca float, align 4, addrspace(5)
  %"431" = alloca float, align 4, addrspace(5)
  %"432" = alloca float, align 4, addrspace(5)
  %"433" = alloca float, align 4, addrspace(5)
  %"434" = alloca float, align 4, addrspace(5)
  %"435" = alloca float, align 4, addrspace(5)
  %"436" = alloca float, align 4, addrspace(5)
  %"437" = alloca float, align 4, addrspace(5)
  %"438" = alloca float, align 4, addrspace(5)
  %"439" = alloca i32, align 4, addrspace(5)
  %"440" = alloca i32, align 4, addrspace(5)
  %"441" = alloca i32, align 4, addrspace(5)
  %"442" = alloca i64, align 8, addrspace(5)
  %"443" = alloca i64, align 8, addrspace(5)
  %"444" = alloca i64, align 8, addrspace(5)
  store i32 %"456", ptr addrspace(5) %"452", align 4
  store i64 %"457", ptr addrspace(5) %"453", align 8
  %"458" = load i32, ptr addrspace(5) %"452", align 4
  store i32 %"458", ptr addrspace(5) %"543", align 4
  %"459" = load i64, ptr addrspace(5) %"453", align 8
  store i64 %"459", ptr addrspace(5) %"544", align 8
  %"591" = load i64, ptr addrspace(5) %"544", align 8
  store i64 %"591", ptr addrspace(5) %"444", align 8
  %"462" = load i64, ptr addrspace(5) %"444", align 8
  %0 = inttoptr i64 %"462" to ptr
  %1 = addrspacecast ptr %0 to ptr addrspace(1)
  %"461" = ptrtoint ptr addrspace(1) %1 to i64
  store i64 %"461", ptr addrspace(5) %"443", align 8
  %"555" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"554", i32 0)
  %"451" = load <4 x float>, ptr addrspace(1) %"555", align 16
  %"463" = extractelement <4 x float> %"451", i32 0
  %"464" = extractelement <4 x float> %"451", i32 1
  %"465" = extractelement <4 x float> %"451", i32 2
  %"466" = extractelement <4 x float> %"451", i32 3
  store float %"463", ptr addrspace(5) %"428", align 4
  store float %"464", ptr addrspace(5) %"429", align 4
  store float %"465", ptr addrspace(5) %"430", align 4
  store float %"466", ptr addrspace(5) %"431", align 4
  %"468" = load float, ptr addrspace(5) %"431", align 4
  %"467" = fcmp ule float %"468", 0.000000e+00
  store i1 %"467", ptr addrspace(5) %"420", align 1
  %"469" = load i1, ptr addrspace(5) %"420", align 1
  br i1 %"469", label %"417", label %"446"

"446":                                            ; preds = %"613"
  %"471" = load float, ptr addrspace(5) %"431", align 4
  %"470" = call float @llvm.fabs.f32(float %"471")
  store float %"470", ptr addrspace(5) %"432", align 4
  %"473" = load float, ptr addrspace(5) %"432", align 4
  %"472" = fcmp une float %"473", 0x7FF0000000000000
  store i1 %"472", ptr addrspace(5) %"421", align 1
  %"474" = load i1, ptr addrspace(5) %"421", align 1
  br i1 %"474", label %"416", label %"448"

"448":                                            ; preds = %"446"
  br label %"417"

"416":                                            ; preds = %"446"
  %"476" = load float, ptr addrspace(5) %"428", align 4
  %"477" = load float, ptr addrspace(5) %"431", align 4
  %"475" = fsub float %"476", %"477"
  store float %"475", ptr addrspace(5) %"433", align 4
  %"478" = load i64, ptr addrspace(5) %"443", align 8
  %"479" = load float, ptr addrspace(5) %"433", align 4
  %"593" = inttoptr i64 %"478" to ptr addrspace(1)
  store float %"479", ptr addrspace(1) %"593", align 4
  %"481" = load float, ptr addrspace(5) %"429", align 4
  %"482" = load float, ptr addrspace(5) %"431", align 4
  %"480" = fsub float %"481", %"482"
  store float %"480", ptr addrspace(5) %"434", align 4
  %"483" = load i64, ptr addrspace(5) %"443", align 8
  %"484" = load float, ptr addrspace(5) %"434", align 4
  %"594" = inttoptr i64 %"483" to ptr addrspace(1)
  %"615" = getelementptr inbounds i8, ptr addrspace(1) %"594", i64 4
  store float %"484", ptr addrspace(1) %"615", align 4
  %"486" = load float, ptr addrspace(5) %"430", align 4
  %"487" = load float, ptr addrspace(5) %"431", align 4
  %"485" = fsub float %"486", %"487"
  store float %"485", ptr addrspace(5) %"435", align 4
  %"488" = load i64, ptr addrspace(5) %"443", align 8
  %"489" = load float, ptr addrspace(5) %"435", align 4
  %"595" = inttoptr i64 %"488" to ptr addrspace(1)
  %"617" = getelementptr inbounds i8, ptr addrspace(1) %"595", i64 8
  store float %"489", ptr addrspace(1) %"617", align 4
  %"491" = load float, ptr addrspace(5) %"428", align 4
  %"492" = load float, ptr addrspace(5) %"431", align 4
  %"490" = fadd float %"491", %"492"
  store float %"490", ptr addrspace(5) %"436", align 4
  %"493" = load i64, ptr addrspace(5) %"443", align 8
  %"494" = load float, ptr addrspace(5) %"436", align 4
  %"596" = inttoptr i64 %"493" to ptr addrspace(1)
  %"619" = getelementptr inbounds i8, ptr addrspace(1) %"596", i64 16
  store float %"494", ptr addrspace(1) %"619", align 4
  %"496" = load float, ptr addrspace(5) %"429", align 4
  %"497" = load float, ptr addrspace(5) %"431", align 4
  %"495" = fadd float %"496", %"497"
  store float %"495", ptr addrspace(5) %"437", align 4
  %"498" = load i64, ptr addrspace(5) %"443", align 8
  %"499" = load float, ptr addrspace(5) %"437", align 4
  %"597" = inttoptr i64 %"498" to ptr addrspace(1)
  %"621" = getelementptr inbounds i8, ptr addrspace(1) %"597", i64 20
  store float %"499", ptr addrspace(1) %"621", align 4
  %"501" = load float, ptr addrspace(5) %"430", align 4
  %"502" = load float, ptr addrspace(5) %"431", align 4
  %"500" = fadd float %"501", %"502"
  store float %"500", ptr addrspace(5) %"438", align 4
  %"503" = load i64, ptr addrspace(5) %"443", align 8
  %"504" = load float, ptr addrspace(5) %"438", align 4
  %"598" = inttoptr i64 %"503" to ptr addrspace(1)
  %"623" = getelementptr inbounds i8, ptr addrspace(1) %"598", i64 24
  store float %"504", ptr addrspace(1) %"623", align 4
  br label %"418"

"417":                                            ; preds = %"448", %"613"
  %2 = alloca i32, align 4, addrspace(5)
  store i32 2096152002, ptr addrspace(5) %2, align 4
  %"599" = load i32, ptr addrspace(5) %2, align 4
  store i32 %"599", ptr addrspace(5) %"440", align 4
  %"506" = load i64, ptr addrspace(5) %"443", align 8
  %"507" = load i32, ptr addrspace(5) %"440", align 4
  %"600" = inttoptr i64 %"506" to ptr addrspace(1)
  store i32 %"507", ptr addrspace(1) %"600", align 4
  %"508" = load i64, ptr addrspace(5) %"443", align 8
  %"509" = load i32, ptr addrspace(5) %"440", align 4
  %"602" = inttoptr i64 %"508" to ptr addrspace(1)
  %"625" = getelementptr inbounds i8, ptr addrspace(1) %"602", i64 4
  store i32 %"509", ptr addrspace(1) %"625", align 4
  %"510" = load i64, ptr addrspace(5) %"443", align 8
  %"511" = load i32, ptr addrspace(5) %"440", align 4
  %"604" = inttoptr i64 %"510" to ptr addrspace(1)
  %"627" = getelementptr inbounds i8, ptr addrspace(1) %"604", i64 8
  store i32 %"511", ptr addrspace(1) %"627", align 4
  %3 = alloca i32, align 4, addrspace(5)
  store i32 -51331646, ptr addrspace(5) %3, align 4
  %"606" = load i32, ptr addrspace(5) %3, align 4
  store i32 %"606", ptr addrspace(5) %"441", align 4
  %"513" = load i64, ptr addrspace(5) %"443", align 8
  %"514" = load i32, ptr addrspace(5) %"441", align 4
  %"607" = inttoptr i64 %"513" to ptr addrspace(1)
  %"629" = getelementptr inbounds i8, ptr addrspace(1) %"607", i64 16
  store i32 %"514", ptr addrspace(1) %"629", align 4
  %"515" = load i64, ptr addrspace(5) %"443", align 8
  %"516" = load i32, ptr addrspace(5) %"441", align 4
  %"609" = inttoptr i64 %"515" to ptr addrspace(1)
  %"631" = getelementptr inbounds i8, ptr addrspace(1) %"609", i64 20
  store i32 %"516", ptr addrspace(1) %"631", align 4
  %"517" = load i64, ptr addrspace(5) %"443", align 8
  %"518" = load i32, ptr addrspace(5) %"441", align 4
  %"611" = inttoptr i64 %"517" to ptr addrspace(1)
  %"633" = getelementptr inbounds i8, ptr addrspace(1) %"611", i64 24
  store i32 %"518", ptr addrspace(1) %"633", align 4
  br label %"418"

"418":                                            ; preds = %"417", %"416"
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.fabs.f32(float) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
