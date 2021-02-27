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

define protected void @_Z16robust_intersecti(i32 %"455", ptr addrspace(3) %"747", <2 x i32> %"748", <2 x i32> %"749", ptr addrspace(5) %"750", float %"751", ptr addrspace(5) %"752", float %"753", ptr addrspace(1) %"754", ptr addrspace(1) %"755", ptr addrspace(1) %"756", ptr addrspace(5) %"757", ptr addrspace(5) %"758", ptr addrspace(5) %"759") #1 {
"892":
  %"863" = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %"863", align 4
  %"452" = alloca i32, align 4, addrspace(5)
  %"453" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"453", align 1
  %"454" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"454", align 1
  %"738" = alloca i32, align 4, addrspace(5)
  %"302" = alloca i1, align 1, addrspace(5)
  %"303" = alloca i1, align 1, addrspace(5)
  %"304" = alloca i1, align 1, addrspace(5)
  %"305" = alloca i1, align 1, addrspace(5)
  %"306" = alloca i1, align 1, addrspace(5)
  %"307" = alloca i1, align 1, addrspace(5)
  %"308" = alloca i1, align 1, addrspace(5)
  %"309" = alloca i1, align 1, addrspace(5)
  %"310" = alloca float, align 4, addrspace(5)
  %"311" = alloca float, align 4, addrspace(5)
  %"312" = alloca float, align 4, addrspace(5)
  %"313" = alloca float, align 4, addrspace(5)
  %"314" = alloca float, align 4, addrspace(5)
  %"315" = alloca float, align 4, addrspace(5)
  %"316" = alloca float, align 4, addrspace(5)
  %"317" = alloca float, align 4, addrspace(5)
  %"318" = alloca float, align 4, addrspace(5)
  %"319" = alloca float, align 4, addrspace(5)
  %"320" = alloca float, align 4, addrspace(5)
  %"321" = alloca float, align 4, addrspace(5)
  %"322" = alloca float, align 4, addrspace(5)
  %"323" = alloca float, align 4, addrspace(5)
  %"324" = alloca float, align 4, addrspace(5)
  %"325" = alloca float, align 4, addrspace(5)
  %"326" = alloca float, align 4, addrspace(5)
  %"327" = alloca float, align 4, addrspace(5)
  %"328" = alloca float, align 4, addrspace(5)
  %"329" = alloca float, align 4, addrspace(5)
  %"330" = alloca float, align 4, addrspace(5)
  %"331" = alloca float, align 4, addrspace(5)
  %"332" = alloca float, align 4, addrspace(5)
  %"333" = alloca float, align 4, addrspace(5)
  %"334" = alloca float, align 4, addrspace(5)
  %"335" = alloca float, align 4, addrspace(5)
  %"336" = alloca float, align 4, addrspace(5)
  %"337" = alloca float, align 4, addrspace(5)
  %"338" = alloca float, align 4, addrspace(5)
  %"339" = alloca float, align 4, addrspace(5)
  %"340" = alloca float, align 4, addrspace(5)
  %"341" = alloca float, align 4, addrspace(5)
  %"342" = alloca float, align 4, addrspace(5)
  %"343" = alloca float, align 4, addrspace(5)
  %"344" = alloca float, align 4, addrspace(5)
  %"345" = alloca float, align 4, addrspace(5)
  %"346" = alloca float, align 4, addrspace(5)
  %"347" = alloca float, align 4, addrspace(5)
  %"348" = alloca float, align 4, addrspace(5)
  %"349" = alloca float, align 4, addrspace(5)
  %"350" = alloca float, align 4, addrspace(5)
  %"351" = alloca float, align 4, addrspace(5)
  %"352" = alloca float, align 4, addrspace(5)
  %"353" = alloca float, align 4, addrspace(5)
  %"354" = alloca float, align 4, addrspace(5)
  %"355" = alloca float, align 4, addrspace(5)
  %"356" = alloca float, align 4, addrspace(5)
  %"357" = alloca float, align 4, addrspace(5)
  %"358" = alloca float, align 4, addrspace(5)
  %"359" = alloca float, align 4, addrspace(5)
  %"360" = alloca float, align 4, addrspace(5)
  %"361" = alloca float, align 4, addrspace(5)
  %"362" = alloca float, align 4, addrspace(5)
  %"363" = alloca float, align 4, addrspace(5)
  %"364" = alloca float, align 4, addrspace(5)
  %"365" = alloca float, align 4, addrspace(5)
  %"366" = alloca float, align 4, addrspace(5)
  %"367" = alloca float, align 4, addrspace(5)
  %"368" = alloca float, align 4, addrspace(5)
  %"369" = alloca float, align 4, addrspace(5)
  %"370" = alloca float, align 4, addrspace(5)
  %"371" = alloca float, align 4, addrspace(5)
  %"372" = alloca float, align 4, addrspace(5)
  %"373" = alloca float, align 4, addrspace(5)
  %"374" = alloca float, align 4, addrspace(5)
  %"375" = alloca float, align 4, addrspace(5)
  %"376" = alloca float, align 4, addrspace(5)
  %"377" = alloca float, align 4, addrspace(5)
  %"378" = alloca float, align 4, addrspace(5)
  %"379" = alloca float, align 4, addrspace(5)
  %"380" = alloca float, align 4, addrspace(5)
  %"381" = alloca float, align 4, addrspace(5)
  %"382" = alloca float, align 4, addrspace(5)
  %"383" = alloca float, align 4, addrspace(5)
  %"384" = alloca float, align 4, addrspace(5)
  %"385" = alloca float, align 4, addrspace(5)
  %"386" = alloca float, align 4, addrspace(5)
  %"387" = alloca float, align 4, addrspace(5)
  %"388" = alloca float, align 4, addrspace(5)
  %"389" = alloca float, align 4, addrspace(5)
  %"390" = alloca float, align 4, addrspace(5)
  %"391" = alloca float, align 4, addrspace(5)
  %"392" = alloca float, align 4, addrspace(5)
  %"393" = alloca float, align 4, addrspace(5)
  %"394" = alloca i32, align 4, addrspace(5)
  %"395" = alloca i32, align 4, addrspace(5)
  %"396" = alloca i32, align 4, addrspace(5)
  %"397" = alloca i32, align 4, addrspace(5)
  %"398" = alloca i32, align 4, addrspace(5)
  %"399" = alloca i32, align 4, addrspace(5)
  %"400" = alloca i32, align 4, addrspace(5)
  store i32 %"455", ptr addrspace(5) %"452", align 4
  %"456" = load i32, ptr addrspace(5) %"452", align 4
  store i32 %"456", ptr addrspace(5) %"738", align 4
  %"760" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"754", i32 16)
  %"450" = load <4 x float>, ptr addrspace(1) %"760", align 16
  %"457" = extractelement <4 x float> %"450", i32 0
  %"458" = extractelement <4 x float> %"450", i32 1
  %"459" = extractelement <4 x float> %"450", i32 2
  %"460" = extractelement <4 x float> %"450", i32 3
  store float %"457", ptr addrspace(5) %"335", align 4
  store float %"458", ptr addrspace(5) %"336", align 4
  store float %"459", ptr addrspace(5) %"337", align 4
  store float %"460", ptr addrspace(5) %"338", align 4
  %"461" = load float, ptr addrspace(5) %"750", align 4
  store float %"461", ptr addrspace(5) %"342", align 4
  %"463" = load float, ptr addrspace(5) %"342", align 4
  %"464" = load float, ptr addrspace(5) %"335", align 4
  %"462" = fsub float %"463", %"464"
  store float %"462", ptr addrspace(5) %"312", align 4
  %"895" = getelementptr inbounds i8, ptr addrspace(5) %"750", i64 4
  %"465" = load float, ptr addrspace(5) %"895", align 4
  store float %"465", ptr addrspace(5) %"343", align 4
  %"467" = load float, ptr addrspace(5) %"343", align 4
  %"468" = load float, ptr addrspace(5) %"336", align 4
  %"466" = fsub float %"467", %"468"
  store float %"466", ptr addrspace(5) %"313", align 4
  %"897" = getelementptr inbounds i8, ptr addrspace(5) %"750", i64 8
  %"469" = load float, ptr addrspace(5) %"897", align 4
  store float %"469", ptr addrspace(5) %"344", align 4
  %"471" = load float, ptr addrspace(5) %"344", align 4
  %"472" = load float, ptr addrspace(5) %"337", align 4
  %"470" = fsub float %"471", %"472"
  store float %"470", ptr addrspace(5) %"314", align 4
  %"899" = getelementptr inbounds i8, ptr addrspace(5) %"750", i64 12
  %"473" = load float, ptr addrspace(5) %"899", align 4
  store float %"473", ptr addrspace(5) %"345", align 4
  %"901" = getelementptr inbounds i8, ptr addrspace(5) %"750", i64 16
  %"474" = load float, ptr addrspace(5) %"901", align 4
  store float %"474", ptr addrspace(5) %"346", align 4
  %"476" = load float, ptr addrspace(5) %"346", align 4
  %"477" = load float, ptr addrspace(5) %"346", align 4
  %"475" = fmul float %"476", %"477"
  store float %"475", ptr addrspace(5) %"347", align 4
  %"479" = load float, ptr addrspace(5) %"345", align 4
  %"480" = load float, ptr addrspace(5) %"345", align 4
  %"481" = load float, ptr addrspace(5) %"347", align 4
  %"478" = call float @llvm.fma.f32(float %"479", float %"480", float %"481")
  store float %"478", ptr addrspace(5) %"348", align 4
  %"903" = getelementptr inbounds i8, ptr addrspace(5) %"750", i64 20
  %"482" = load float, ptr addrspace(5) %"903", align 4
  store float %"482", ptr addrspace(5) %"349", align 4
  %"484" = load float, ptr addrspace(5) %"349", align 4
  %"485" = load float, ptr addrspace(5) %"349", align 4
  %"486" = load float, ptr addrspace(5) %"348", align 4
  %"483" = call float @llvm.fma.f32(float %"484", float %"485", float %"486")
  store float %"483", ptr addrspace(5) %"350", align 4
  %"488" = load float, ptr addrspace(5) %"350", align 4
  %0 = call afn float @llvm.sqrt.f32(float %"488")
  %"487" = fdiv arcp afn float 1.000000e+00, %0
  store float %"487", ptr addrspace(5) %"315", align 4
  %"490" = load float, ptr addrspace(5) %"345", align 4
  %"491" = load float, ptr addrspace(5) %"315", align 4
  %"489" = fmul float %"490", %"491"
  store float %"489", ptr addrspace(5) %"316", align 4
  %"493" = load float, ptr addrspace(5) %"315", align 4
  %"494" = load float, ptr addrspace(5) %"346", align 4
  %"492" = fmul float %"493", %"494"
  store float %"492", ptr addrspace(5) %"317", align 4
  %"496" = load float, ptr addrspace(5) %"315", align 4
  %"497" = load float, ptr addrspace(5) %"349", align 4
  %"495" = fmul float %"496", %"497"
  store float %"495", ptr addrspace(5) %"318", align 4
  %"499" = load float, ptr addrspace(5) %"313", align 4
  %"500" = load float, ptr addrspace(5) %"317", align 4
  %"498" = fmul float %"499", %"500"
  store float %"498", ptr addrspace(5) %"351", align 4
  %"502" = load float, ptr addrspace(5) %"312", align 4
  %"503" = load float, ptr addrspace(5) %"316", align 4
  %"504" = load float, ptr addrspace(5) %"351", align 4
  %"501" = call float @llvm.fma.f32(float %"502", float %"503", float %"504")
  store float %"501", ptr addrspace(5) %"352", align 4
  %"506" = load float, ptr addrspace(5) %"314", align 4
  %"507" = load float, ptr addrspace(5) %"318", align 4
  %"508" = load float, ptr addrspace(5) %"352", align 4
  %"505" = call float @llvm.fma.f32(float %"506", float %"507", float %"508")
  store float %"505", ptr addrspace(5) %"392", align 4
  %"510" = load float, ptr addrspace(5) %"313", align 4
  %"511" = load float, ptr addrspace(5) %"313", align 4
  %"509" = fmul float %"510", %"511"
  store float %"509", ptr addrspace(5) %"353", align 4
  %"513" = load float, ptr addrspace(5) %"312", align 4
  %"514" = load float, ptr addrspace(5) %"312", align 4
  %"515" = load float, ptr addrspace(5) %"353", align 4
  %"512" = call float @llvm.fma.f32(float %"513", float %"514", float %"515")
  store float %"512", ptr addrspace(5) %"354", align 4
  %"517" = load float, ptr addrspace(5) %"314", align 4
  %"518" = load float, ptr addrspace(5) %"314", align 4
  %"519" = load float, ptr addrspace(5) %"354", align 4
  %"516" = call float @llvm.fma.f32(float %"517", float %"518", float %"519")
  store float %"516", ptr addrspace(5) %"355", align 4
  %"521" = load float, ptr addrspace(5) %"338", align 4
  %"522" = load float, ptr addrspace(5) %"338", align 4
  %"520" = fmul float %"521", %"522"
  store float %"520", ptr addrspace(5) %"320", align 4
  %"524" = load float, ptr addrspace(5) %"355", align 4
  %"525" = load float, ptr addrspace(5) %"320", align 4
  %"523" = fsub float %"524", %"525"
  store float %"523", ptr addrspace(5) %"356", align 4
  %"527" = load float, ptr addrspace(5) %"392", align 4
  %"528" = load float, ptr addrspace(5) %"392", align 4
  %"526" = fmul float %"527", %"528"
  store float %"526", ptr addrspace(5) %"357", align 4
  %"530" = load float, ptr addrspace(5) %"357", align 4
  %"531" = load float, ptr addrspace(5) %"356", align 4
  %"529" = fsub float %"530", %"531"
  store float %"529", ptr addrspace(5) %"321", align 4
  %"533" = load float, ptr addrspace(5) %"321", align 4
  %"532" = fcmp ule float %"533", 0.000000e+00
  store i1 %"532", ptr addrspace(5) %"303", align 1
  %1 = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %1, align 4
  %"534" = load float, ptr addrspace(5) %1, align 4
  store float %"534", ptr addrspace(5) %"393", align 4
  %"535" = load i1, ptr addrspace(5) %"303", align 1
  br i1 %"535", label %"301", label %"402"

"402":                                            ; preds = %"892"
  %"537" = load float, ptr addrspace(5) %"321", align 4
  %"536" = call afn float @llvm.sqrt.f32(float %"537")
  store float %"536", ptr addrspace(5) %"391", align 4
  %"539" = load float, ptr addrspace(5) %"392", align 4
  %"538" = fsub float 0.000000e+00, %"539"
  store float %"538", ptr addrspace(5) %"359", align 4
  %"541" = load float, ptr addrspace(5) %"359", align 4
  %"542" = load float, ptr addrspace(5) %"391", align 4
  %"540" = fsub float %"541", %"542"
  store float %"540", ptr addrspace(5) %"323", align 4
  %"544" = load float, ptr addrspace(5) %"323", align 4
  %"543" = call float @llvm.fabs.f32(float %"544")
  store float %"543", ptr addrspace(5) %"324", align 4
  %"546" = load float, ptr addrspace(5) %"338", align 4
  %"545" = fmul float %"546", 1.000000e+01
  store float %"545", ptr addrspace(5) %"325", align 4
  %"548" = load float, ptr addrspace(5) %"324", align 4
  %"549" = load float, ptr addrspace(5) %"325", align 4
  %"547" = fcmp ule float %"548", %"549"
  store i1 %"547", ptr addrspace(5) %"304", align 1
  %"550" = load i1, ptr addrspace(5) %"304", align 1
  br i1 %"550", label %"299", label %"404"

"404":                                            ; preds = %"402"
  %"552" = load float, ptr addrspace(5) %"316", align 4
  %"553" = load float, ptr addrspace(5) %"323", align 4
  %"554" = load float, ptr addrspace(5) %"312", align 4
  %"551" = call float @llvm.fma.f32(float %"552", float %"553", float %"554")
  store float %"551", ptr addrspace(5) %"361", align 4
  %"556" = load float, ptr addrspace(5) %"317", align 4
  %"557" = load float, ptr addrspace(5) %"323", align 4
  %"558" = load float, ptr addrspace(5) %"313", align 4
  %"555" = call float @llvm.fma.f32(float %"556", float %"557", float %"558")
  store float %"555", ptr addrspace(5) %"362", align 4
  %"560" = load float, ptr addrspace(5) %"318", align 4
  %"561" = load float, ptr addrspace(5) %"323", align 4
  %"562" = load float, ptr addrspace(5) %"314", align 4
  %"559" = call float @llvm.fma.f32(float %"560", float %"561", float %"562")
  store float %"559", ptr addrspace(5) %"363", align 4
  %"564" = load float, ptr addrspace(5) %"317", align 4
  %"565" = load float, ptr addrspace(5) %"362", align 4
  %"563" = fmul float %"564", %"565"
  store float %"563", ptr addrspace(5) %"364", align 4
  %"567" = load float, ptr addrspace(5) %"316", align 4
  %"568" = load float, ptr addrspace(5) %"361", align 4
  %"569" = load float, ptr addrspace(5) %"364", align 4
  %"566" = call float @llvm.fma.f32(float %"567", float %"568", float %"569")
  store float %"566", ptr addrspace(5) %"365", align 4
  %"571" = load float, ptr addrspace(5) %"318", align 4
  %"572" = load float, ptr addrspace(5) %"363", align 4
  %"573" = load float, ptr addrspace(5) %"365", align 4
  %"570" = call float @llvm.fma.f32(float %"571", float %"572", float %"573")
  store float %"570", ptr addrspace(5) %"392", align 4
  %"575" = load float, ptr addrspace(5) %"362", align 4
  %"576" = load float, ptr addrspace(5) %"362", align 4
  %"574" = fmul float %"575", %"576"
  store float %"574", ptr addrspace(5) %"366", align 4
  %"578" = load float, ptr addrspace(5) %"361", align 4
  %"579" = load float, ptr addrspace(5) %"361", align 4
  %"580" = load float, ptr addrspace(5) %"366", align 4
  %"577" = call float @llvm.fma.f32(float %"578", float %"579", float %"580")
  store float %"577", ptr addrspace(5) %"367", align 4
  %"582" = load float, ptr addrspace(5) %"363", align 4
  %"583" = load float, ptr addrspace(5) %"363", align 4
  %"584" = load float, ptr addrspace(5) %"367", align 4
  %"581" = call float @llvm.fma.f32(float %"582", float %"583", float %"584")
  store float %"581", ptr addrspace(5) %"368", align 4
  %"586" = load float, ptr addrspace(5) %"368", align 4
  %"587" = load float, ptr addrspace(5) %"320", align 4
  %"585" = fsub float %"586", %"587"
  store float %"585", ptr addrspace(5) %"369", align 4
  %"589" = load float, ptr addrspace(5) %"392", align 4
  %"590" = load float, ptr addrspace(5) %"392", align 4
  %"588" = fmul float %"589", %"590"
  store float %"588", ptr addrspace(5) %"370", align 4
  %"592" = load float, ptr addrspace(5) %"370", align 4
  %"593" = load float, ptr addrspace(5) %"369", align 4
  %"591" = fsub float %"592", %"593"
  store float %"591", ptr addrspace(5) %"327", align 4
  %"595" = load float, ptr addrspace(5) %"327", align 4
  %"594" = fcmp ule float %"595", 0.000000e+00
  store i1 %"594", ptr addrspace(5) %"305", align 1
  %"596" = load i1, ptr addrspace(5) %"305", align 1
  br i1 %"596", label %"299", label %"406"

"406":                                            ; preds = %"404"
  %"598" = load float, ptr addrspace(5) %"327", align 4
  %"597" = call afn float @llvm.sqrt.f32(float %"598")
  store float %"597", ptr addrspace(5) %"391", align 4
  %"600" = load float, ptr addrspace(5) %"392", align 4
  %"599" = fsub float 0.000000e+00, %"600"
  store float %"599", ptr addrspace(5) %"371", align 4
  %"602" = load float, ptr addrspace(5) %"371", align 4
  %"603" = load float, ptr addrspace(5) %"391", align 4
  %"601" = fsub float %"602", %"603"
  store float %"601", ptr addrspace(5) %"393", align 4
  br label %"299"

"299":                                            ; preds = %"406", %"404", %"402"
  %"605" = load float, ptr addrspace(5) %"323", align 4
  %"606" = load float, ptr addrspace(5) %"393", align 4
  %"604" = fadd float %"605", %"606"
  store float %"604", ptr addrspace(5) %"333", align 4
  %"608" = load float, ptr addrspace(5) %"315", align 4
  %"609" = load float, ptr addrspace(5) %"333", align 4
  %"607" = fmul float %"608", %"609"
  store float %"607", ptr addrspace(5) %"372", align 4
  %"611" = load float, ptr addrspace(5) %"372", align 4
  %"610" = call i32 @__zluda_rt_ptx_impl___rt_potential_intersection(float %"611", ptr addrspace(5) %"750", float %"753", ptr addrspace(5) %"863")
  store i32 %"610", ptr addrspace(5) %"395", align 4
  %"613" = load i32, ptr addrspace(5) %"395", align 4
  %"612" = icmp eq i32 %"613", 0
  store i1 %"612", ptr addrspace(5) %"306", align 1
  %"614" = load i1, ptr addrspace(5) %"306", align 1
  br i1 %"614", label %"300", label %"408"

"408":                                            ; preds = %"299"
  %"616" = load float, ptr addrspace(5) %"316", align 4
  %"617" = load float, ptr addrspace(5) %"333", align 4
  %"618" = load float, ptr addrspace(5) %"312", align 4
  %"615" = call float @llvm.fma.f32(float %"616", float %"617", float %"618")
  store float %"615", ptr addrspace(5) %"373", align 4
  %"620" = load float, ptr addrspace(5) %"317", align 4
  %"621" = load float, ptr addrspace(5) %"333", align 4
  %"622" = load float, ptr addrspace(5) %"313", align 4
  %"619" = call float @llvm.fma.f32(float %"620", float %"621", float %"622")
  store float %"619", ptr addrspace(5) %"374", align 4
  %"624" = load float, ptr addrspace(5) %"318", align 4
  %"625" = load float, ptr addrspace(5) %"333", align 4
  %"626" = load float, ptr addrspace(5) %"314", align 4
  %"623" = call float @llvm.fma.f32(float %"624", float %"625", float %"626")
  store float %"623", ptr addrspace(5) %"375", align 4
  %"628" = load float, ptr addrspace(5) %"338", align 4
  %"627" = fdiv arcp afn float 1.000000e+00, %"628"
  store float %"627", ptr addrspace(5) %"376", align 4
  %"630" = load float, ptr addrspace(5) %"376", align 4
  %"631" = load float, ptr addrspace(5) %"373", align 4
  %"629" = fmul float %"630", %"631"
  store float %"629", ptr addrspace(5) %"377", align 4
  %"633" = load float, ptr addrspace(5) %"376", align 4
  %"634" = load float, ptr addrspace(5) %"374", align 4
  %"632" = fmul float %"633", %"634"
  store float %"632", ptr addrspace(5) %"378", align 4
  %"636" = load float, ptr addrspace(5) %"376", align 4
  %"637" = load float, ptr addrspace(5) %"375", align 4
  %"635" = fmul float %"636", %"637"
  store float %"635", ptr addrspace(5) %"379", align 4
  %"638" = load float, ptr addrspace(5) %"377", align 4
  %"767" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 12)
  store float %"638", ptr addrspace(1) %"767", align 4
  %"639" = load float, ptr addrspace(5) %"378", align 4
  %"769" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 12)
  %"905" = getelementptr inbounds i8, ptr addrspace(1) %"769", i64 4
  store float %"639", ptr addrspace(1) %"905", align 4
  %"640" = load float, ptr addrspace(5) %"379", align 4
  %"772" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 12)
  %"907" = getelementptr inbounds i8, ptr addrspace(1) %"772", i64 8
  store float %"640", ptr addrspace(1) %"907", align 4
  %"641" = load float, ptr addrspace(5) %"377", align 4
  %"775" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 0)
  store float %"641", ptr addrspace(1) %"775", align 4
  %"642" = load float, ptr addrspace(5) %"378", align 4
  %"777" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 0)
  %"909" = getelementptr inbounds i8, ptr addrspace(1) %"777", i64 4
  store float %"642", ptr addrspace(1) %"909", align 4
  %"643" = load float, ptr addrspace(5) %"379", align 4
  %"780" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 0)
  %"911" = getelementptr inbounds i8, ptr addrspace(1) %"780", i64 8
  store float %"643", ptr addrspace(1) %"911", align 4
  %2 = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %2, align 4
  %"878" = load i32, ptr addrspace(5) %2, align 4
  store i32 %"878", ptr addrspace(5) %"397", align 4
  %"646" = load i32, ptr addrspace(5) %"397", align 4
  %"645" = call i32 @__zluda_rt_ptx_impl___rt_report_intersection(i32 %"646", ptr addrspace(3) %"747", <2 x i32> %"748", <2 x i32> %"749", ptr addrspace(5) %"750", float %"751", ptr addrspace(5) %"752", ptr addrspace(1) %"754", ptr addrspace(1) %"755", ptr addrspace(1) %"756", ptr addrspace(5) %"757", ptr addrspace(5) %"758", ptr addrspace(5) %"759", ptr addrspace(5) %"863")
  store i32 %"645", ptr addrspace(5) %"396", align 4
  %"648" = load i32, ptr addrspace(5) %"396", align 4
  %"647" = icmp ne i32 %"648", 0
  store i1 %"647", ptr addrspace(5) %"307", align 1
  %"649" = load i1, ptr addrspace(5) %"307", align 1
  br i1 %"649", label %"301", label %"410"

"410":                                            ; preds = %"408"
  br label %"300"

"300":                                            ; preds = %"410", %"299"
  %"651" = load float, ptr addrspace(5) %"324", align 4
  %"652" = load float, ptr addrspace(5) %"325", align 4
  %"650" = fcmp ogt float %"651", %"652"
  store i1 %"650", ptr addrspace(5) %"308", align 1
  %"654" = load float, ptr addrspace(5) %"323", align 4
  %"655" = load i1, ptr addrspace(5) %"308", align 1
  %"653" = select i1 %"655", float %"654", float 0.000000e+00
  store float %"653", ptr addrspace(5) %"381", align 4
  %"657" = load float, ptr addrspace(5) %"391", align 4
  %"658" = load float, ptr addrspace(5) %"392", align 4
  %"656" = fsub float %"657", %"658"
  store float %"656", ptr addrspace(5) %"382", align 4
  %"660" = load float, ptr addrspace(5) %"381", align 4
  %"661" = load float, ptr addrspace(5) %"382", align 4
  %"659" = fadd float %"660", %"661"
  store float %"659", ptr addrspace(5) %"334", align 4
  %"663" = load float, ptr addrspace(5) %"315", align 4
  %"664" = load float, ptr addrspace(5) %"334", align 4
  %"662" = fmul float %"663", %"664"
  store float %"662", ptr addrspace(5) %"380", align 4
  %"666" = load float, ptr addrspace(5) %"380", align 4
  %"665" = call i32 @__zluda_rt_ptx_impl___rt_potential_intersection(float %"666", ptr addrspace(5) %"750", float %"753", ptr addrspace(5) %"863")
  store i32 %"665", ptr addrspace(5) %"398", align 4
  %"668" = load i32, ptr addrspace(5) %"398", align 4
  %"667" = icmp eq i32 %"668", 0
  store i1 %"667", ptr addrspace(5) %"309", align 1
  %"669" = load i1, ptr addrspace(5) %"309", align 1
  br i1 %"669", label %"301", label %"412"

"412":                                            ; preds = %"300"
  %"671" = load float, ptr addrspace(5) %"316", align 4
  %"672" = load float, ptr addrspace(5) %"334", align 4
  %"673" = load float, ptr addrspace(5) %"312", align 4
  %"670" = call float @llvm.fma.f32(float %"671", float %"672", float %"673")
  store float %"670", ptr addrspace(5) %"383", align 4
  %"675" = load float, ptr addrspace(5) %"317", align 4
  %"676" = load float, ptr addrspace(5) %"334", align 4
  %"677" = load float, ptr addrspace(5) %"313", align 4
  %"674" = call float @llvm.fma.f32(float %"675", float %"676", float %"677")
  store float %"674", ptr addrspace(5) %"384", align 4
  %"679" = load float, ptr addrspace(5) %"318", align 4
  %"680" = load float, ptr addrspace(5) %"334", align 4
  %"681" = load float, ptr addrspace(5) %"314", align 4
  %"678" = call float @llvm.fma.f32(float %"679", float %"680", float %"681")
  store float %"678", ptr addrspace(5) %"385", align 4
  %"683" = load float, ptr addrspace(5) %"338", align 4
  %"682" = fdiv arcp afn float 1.000000e+00, %"683"
  store float %"682", ptr addrspace(5) %"386", align 4
  %"685" = load float, ptr addrspace(5) %"386", align 4
  %"686" = load float, ptr addrspace(5) %"383", align 4
  %"684" = fmul float %"685", %"686"
  store float %"684", ptr addrspace(5) %"387", align 4
  %"688" = load float, ptr addrspace(5) %"386", align 4
  %"689" = load float, ptr addrspace(5) %"384", align 4
  %"687" = fmul float %"688", %"689"
  store float %"687", ptr addrspace(5) %"388", align 4
  %"691" = load float, ptr addrspace(5) %"386", align 4
  %"692" = load float, ptr addrspace(5) %"385", align 4
  %"690" = fmul float %"691", %"692"
  store float %"690", ptr addrspace(5) %"389", align 4
  %"693" = load float, ptr addrspace(5) %"387", align 4
  %"783" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 12)
  store float %"693", ptr addrspace(1) %"783", align 4
  %"694" = load float, ptr addrspace(5) %"388", align 4
  %"785" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 12)
  %"913" = getelementptr inbounds i8, ptr addrspace(1) %"785", i64 4
  store float %"694", ptr addrspace(1) %"913", align 4
  %"695" = load float, ptr addrspace(5) %"389", align 4
  %"788" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 12)
  %"915" = getelementptr inbounds i8, ptr addrspace(1) %"788", i64 8
  store float %"695", ptr addrspace(1) %"915", align 4
  %"696" = load float, ptr addrspace(5) %"387", align 4
  %"791" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 0)
  store float %"696", ptr addrspace(1) %"791", align 4
  %"697" = load float, ptr addrspace(5) %"388", align 4
  %"793" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 0)
  %"917" = getelementptr inbounds i8, ptr addrspace(1) %"793", i64 4
  store float %"697", ptr addrspace(1) %"917", align 4
  %"698" = load float, ptr addrspace(5) %"389", align 4
  %"796" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"755", i32 0)
  %"919" = getelementptr inbounds i8, ptr addrspace(1) %"796", i64 8
  store float %"698", ptr addrspace(1) %"919", align 4
  %3 = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %3, align 4
  %"887" = load i32, ptr addrspace(5) %3, align 4
  store i32 %"887", ptr addrspace(5) %"400", align 4
  %"701" = load i32, ptr addrspace(5) %"400", align 4
  %"700" = call i32 @__zluda_rt_ptx_impl___rt_report_intersection(i32 %"701", ptr addrspace(3) %"747", <2 x i32> %"748", <2 x i32> %"749", ptr addrspace(5) %"750", float %"751", ptr addrspace(5) %"752", ptr addrspace(1) %"754", ptr addrspace(1) %"755", ptr addrspace(1) %"756", ptr addrspace(5) %"757", ptr addrspace(5) %"758", ptr addrspace(5) %"759", ptr addrspace(5) %"863")
  store i32 %"700", ptr addrspace(5) %"399", align 4
  br label %"301"

"301":                                            ; preds = %"412", %"300", %"408", %"892"
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.fma.f32(float, float, float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.sqrt.f32(float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.fabs.f32(float) #2

define protected void @__zluda_rt_ptx_impl__rollback_wrapper(i32 %"802", ptr addrspace(3) %"803", <2 x i32> %"804", <2 x i32> %"805", ptr addrspace(5) %"806", float %"807", ptr addrspace(5) %"808", float %"809", ptr addrspace(1) %"810", ptr addrspace(1) %"811", ptr addrspace(1) %"812", ptr addrspace(5) %"813", ptr addrspace(5) %"814", ptr addrspace(5) %"815") #3 {
"893":
  %"816" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"811", i32 0)
  %"818" = load [12 x i8], ptr addrspace(1) %"816", align 1
  %"819" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"811", i32 12)
  %"821" = load [12 x i8], ptr addrspace(1) %"819", align 1
  call void @_Z16robust_intersecti(i32 %"802", ptr addrspace(3) %"803", <2 x i32> %"804", <2 x i32> %"805", ptr addrspace(5) %"806", float %"807", ptr addrspace(5) %"808", float %"809", ptr addrspace(1) %"810", ptr addrspace(1) %"811", ptr addrspace(1) %"812", ptr addrspace(5) %"813", ptr addrspace(5) %"814", ptr addrspace(5) %"815")
  %"823" = load i32, ptr addrspace(5) %"814", align 4
  %"826" = icmp eq i32 %"823", 2
  br i1 %"826", label %"827", label %"824"

"827":                                            ; preds = %"893"
  %"828" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"811", i32 0)
  store [12 x i8] %"818", ptr addrspace(1) %"828", align 1
  %"830" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"811", i32 12)
  store [12 x i8] %"821", ptr addrspace(1) %"830", align 1
  br label %"824"

"824":                                            ; preds = %"827", %"893"
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
