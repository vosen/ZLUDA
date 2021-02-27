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

declare float @__zluda_ptx_impl__cvt_rn_f32_s32(i32) #0

declare float @__zluda_ptx_impl__cvt_rn_f32_u32(i32) #0

declare i32 @__zluda_ptx_impl__cvt_rz_s32_f32(float) #0

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

define protected i32 @_Z11closest_hitv(ptr addrspace(3) %"2127", i32 %"2128", <2 x i32> %"2129", <2 x i32> %"2130", ptr addrspace(5) %"2131", float %"2132", ptr addrspace(5) %"2133", float %"2134", <2 x float> %"2135", <3 x float> %"2136", ptr addrspace(1) %"2137", ptr addrspace(1) %"2138", ptr addrspace(1) %"2139") #1 {
"2703":
  %"2392" = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %"2392", align 4
  %"854" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"854", align 1
  %"855" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"855", align 1
  %"312" = alloca [236 x i8], align 4, addrspace(5)
  %"313" = alloca i64, align 8, addrspace(5)
  %"314" = alloca i64, align 8, addrspace(5)
  %"315" = alloca i1, align 1, addrspace(5)
  %"316" = alloca i1, align 1, addrspace(5)
  %"317" = alloca i1, align 1, addrspace(5)
  %"318" = alloca i1, align 1, addrspace(5)
  %"319" = alloca i1, align 1, addrspace(5)
  %"320" = alloca i1, align 1, addrspace(5)
  %"321" = alloca i1, align 1, addrspace(5)
  %"322" = alloca i1, align 1, addrspace(5)
  %"323" = alloca i16, align 2, addrspace(5)
  %"324" = alloca i16, align 2, addrspace(5)
  %"325" = alloca i16, align 2, addrspace(5)
  %"326" = alloca i16, align 2, addrspace(5)
  %"327" = alloca i16, align 2, addrspace(5)
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
  %"394" = alloca float, align 4, addrspace(5)
  %"395" = alloca float, align 4, addrspace(5)
  %"396" = alloca float, align 4, addrspace(5)
  %"397" = alloca float, align 4, addrspace(5)
  %"398" = alloca float, align 4, addrspace(5)
  %"399" = alloca float, align 4, addrspace(5)
  %"400" = alloca float, align 4, addrspace(5)
  %"401" = alloca float, align 4, addrspace(5)
  %"402" = alloca float, align 4, addrspace(5)
  %"403" = alloca float, align 4, addrspace(5)
  %"404" = alloca float, align 4, addrspace(5)
  %"405" = alloca float, align 4, addrspace(5)
  %"406" = alloca float, align 4, addrspace(5)
  %"407" = alloca float, align 4, addrspace(5)
  %"408" = alloca float, align 4, addrspace(5)
  %"409" = alloca float, align 4, addrspace(5)
  %"410" = alloca float, align 4, addrspace(5)
  %"411" = alloca float, align 4, addrspace(5)
  %"412" = alloca float, align 4, addrspace(5)
  %"413" = alloca float, align 4, addrspace(5)
  %"414" = alloca float, align 4, addrspace(5)
  %"415" = alloca float, align 4, addrspace(5)
  %"416" = alloca float, align 4, addrspace(5)
  %"417" = alloca float, align 4, addrspace(5)
  %"418" = alloca float, align 4, addrspace(5)
  %"419" = alloca float, align 4, addrspace(5)
  %"420" = alloca float, align 4, addrspace(5)
  %"421" = alloca float, align 4, addrspace(5)
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
  %"439" = alloca float, align 4, addrspace(5)
  %"440" = alloca float, align 4, addrspace(5)
  %"441" = alloca float, align 4, addrspace(5)
  %"442" = alloca float, align 4, addrspace(5)
  %"443" = alloca float, align 4, addrspace(5)
  %"444" = alloca float, align 4, addrspace(5)
  %"445" = alloca float, align 4, addrspace(5)
  %"446" = alloca float, align 4, addrspace(5)
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
  %"559" = alloca i32, align 4, addrspace(5)
  %"560" = alloca i32, align 4, addrspace(5)
  %"561" = alloca i32, align 4, addrspace(5)
  %"562" = alloca i32, align 4, addrspace(5)
  %"563" = alloca i32, align 4, addrspace(5)
  %"564" = alloca i32, align 4, addrspace(5)
  %"565" = alloca i32, align 4, addrspace(5)
  %"566" = alloca i32, align 4, addrspace(5)
  %"567" = alloca i32, align 4, addrspace(5)
  %"568" = alloca i32, align 4, addrspace(5)
  %"569" = alloca i32, align 4, addrspace(5)
  %"570" = alloca i32, align 4, addrspace(5)
  %"571" = alloca i32, align 4, addrspace(5)
  %"572" = alloca i32, align 4, addrspace(5)
  %"573" = alloca i32, align 4, addrspace(5)
  %"574" = alloca i32, align 4, addrspace(5)
  %"575" = alloca i32, align 4, addrspace(5)
  %"576" = alloca i32, align 4, addrspace(5)
  %"577" = alloca i32, align 4, addrspace(5)
  %"578" = alloca i32, align 4, addrspace(5)
  %"579" = alloca i32, align 4, addrspace(5)
  %"580" = alloca i32, align 4, addrspace(5)
  %"581" = alloca i32, align 4, addrspace(5)
  %"582" = alloca i32, align 4, addrspace(5)
  %"583" = alloca i32, align 4, addrspace(5)
  %"584" = alloca i32, align 4, addrspace(5)
  %"585" = alloca i32, align 4, addrspace(5)
  %"586" = alloca i32, align 4, addrspace(5)
  %"587" = alloca i32, align 4, addrspace(5)
  %"588" = alloca i32, align 4, addrspace(5)
  %"589" = alloca i32, align 4, addrspace(5)
  %"590" = alloca i32, align 4, addrspace(5)
  %"591" = alloca i32, align 4, addrspace(5)
  %"592" = alloca i32, align 4, addrspace(5)
  %"593" = alloca i32, align 4, addrspace(5)
  %"594" = alloca i32, align 4, addrspace(5)
  %"595" = alloca i32, align 4, addrspace(5)
  %"596" = alloca i32, align 4, addrspace(5)
  %"597" = alloca i32, align 4, addrspace(5)
  %"598" = alloca i32, align 4, addrspace(5)
  %"599" = alloca i32, align 4, addrspace(5)
  %"600" = alloca i32, align 4, addrspace(5)
  %"601" = alloca i32, align 4, addrspace(5)
  %"602" = alloca i32, align 4, addrspace(5)
  %"603" = alloca i32, align 4, addrspace(5)
  %"604" = alloca i32, align 4, addrspace(5)
  %"605" = alloca i32, align 4, addrspace(5)
  %"606" = alloca i32, align 4, addrspace(5)
  %"607" = alloca i64, align 8, addrspace(5)
  %"608" = alloca i64, align 8, addrspace(5)
  %"609" = alloca i64, align 8, addrspace(5)
  %"610" = alloca i64, align 8, addrspace(5)
  %"611" = alloca i64, align 8, addrspace(5)
  %"612" = alloca i64, align 8, addrspace(5)
  %"613" = alloca i64, align 8, addrspace(5)
  %"614" = alloca i64, align 8, addrspace(5)
  %"615" = alloca i64, align 8, addrspace(5)
  %"616" = alloca i64, align 8, addrspace(5)
  %"617" = alloca i64, align 8, addrspace(5)
  %"618" = alloca i64, align 8, addrspace(5)
  %"619" = alloca i64, align 8, addrspace(5)
  %"620" = alloca i64, align 8, addrspace(5)
  %"621" = alloca i64, align 8, addrspace(5)
  %"622" = alloca i64, align 8, addrspace(5)
  %"623" = alloca i64, align 8, addrspace(5)
  %"624" = alloca i64, align 8, addrspace(5)
  %"625" = alloca i64, align 8, addrspace(5)
  %"626" = alloca i64, align 8, addrspace(5)
  %"627" = alloca i64, align 8, addrspace(5)
  %"628" = alloca i64, align 8, addrspace(5)
  %"629" = alloca i64, align 8, addrspace(5)
  %"630" = alloca i64, align 8, addrspace(5)
  %"631" = alloca i64, align 8, addrspace(5)
  %"632" = alloca i64, align 8, addrspace(5)
  %"633" = alloca i64, align 8, addrspace(5)
  %"634" = alloca i64, align 8, addrspace(5)
  %"635" = alloca i64, align 8, addrspace(5)
  %"636" = alloca i64, align 8, addrspace(5)
  %"637" = alloca i64, align 8, addrspace(5)
  %"638" = alloca i64, align 8, addrspace(5)
  %"639" = alloca i64, align 8, addrspace(5)
  %"640" = alloca i64, align 8, addrspace(5)
  %"641" = alloca i64, align 8, addrspace(5)
  %"642" = alloca i64, align 8, addrspace(5)
  %"643" = alloca i64, align 8, addrspace(5)
  %"644" = alloca i64, align 8, addrspace(5)
  %"645" = alloca i64, align 8, addrspace(5)
  %"646" = alloca i64, align 8, addrspace(5)
  %"647" = alloca i64, align 8, addrspace(5)
  %"648" = alloca i64, align 8, addrspace(5)
  %"649" = alloca i64, align 8, addrspace(5)
  %"650" = alloca i64, align 8, addrspace(5)
  %"651" = alloca i64, align 8, addrspace(5)
  %"652" = alloca i64, align 8, addrspace(5)
  %"653" = alloca i64, align 8, addrspace(5)
  %"654" = alloca i64, align 8, addrspace(5)
  %"655" = alloca i64, align 8, addrspace(5)
  %"656" = alloca i64, align 8, addrspace(5)
  %"657" = alloca i64, align 8, addrspace(5)
  %"658" = alloca i64, align 8, addrspace(5)
  %"659" = alloca i64, align 8, addrspace(5)
  %"660" = alloca i64, align 8, addrspace(5)
  %"661" = alloca i64, align 8, addrspace(5)
  %"662" = alloca i64, align 8, addrspace(5)
  %"663" = alloca i64, align 8, addrspace(5)
  %"664" = alloca i64, align 8, addrspace(5)
  %"665" = alloca i64, align 8, addrspace(5)
  %"666" = alloca i64, align 8, addrspace(5)
  %"667" = alloca i64, align 8, addrspace(5)
  %"668" = alloca i64, align 8, addrspace(5)
  %"669" = alloca i64, align 8, addrspace(5)
  %"670" = alloca i64, align 8, addrspace(5)
  %"671" = alloca i64, align 8, addrspace(5)
  %"672" = alloca i64, align 8, addrspace(5)
  %"673" = alloca i64, align 8, addrspace(5)
  %"674" = alloca i64, align 8, addrspace(5)
  %"675" = alloca i64, align 8, addrspace(5)
  %"676" = alloca i64, align 8, addrspace(5)
  %"677" = alloca i64, align 8, addrspace(5)
  %"678" = alloca i64, align 8, addrspace(5)
  %"679" = alloca i64, align 8, addrspace(5)
  %"680" = alloca i64, align 8, addrspace(5)
  %"681" = alloca i64, align 8, addrspace(5)
  %"682" = alloca i64, align 8, addrspace(5)
  %"683" = alloca i64, align 8, addrspace(5)
  %"684" = alloca i64, align 8, addrspace(5)
  %"685" = alloca i64, align 8, addrspace(5)
  %"686" = alloca i64, align 8, addrspace(5)
  %"687" = alloca i64, align 8, addrspace(5)
  %"688" = alloca i64, align 8, addrspace(5)
  %"689" = alloca i64, align 8, addrspace(5)
  %"690" = alloca i64, align 8, addrspace(5)
  %"691" = alloca i64, align 8, addrspace(5)
  %"692" = alloca i64, align 8, addrspace(5)
  %"693" = alloca i64, align 8, addrspace(5)
  %"694" = alloca i64, align 8, addrspace(5)
  %"695" = alloca i64, align 8, addrspace(5)
  %"696" = alloca i64, align 8, addrspace(5)
  %"697" = alloca i64, align 8, addrspace(5)
  %"698" = alloca i64, align 8, addrspace(5)
  %"699" = alloca i64, align 8, addrspace(5)
  %"700" = alloca i64, align 8, addrspace(5)
  %"701" = alloca i64, align 8, addrspace(5)
  %"702" = alloca i32, align 4, addrspace(5)
  %"2075" = alloca i64, align 8, addrspace(5)
  %"2077" = alloca i64, align 8, addrspace(5)
  %"2079" = alloca i64, align 8, addrspace(5)
  %"707" = alloca i32, align 4, addrspace(5)
  %"2081" = alloca i64, align 8, addrspace(5)
  %"2083" = alloca i64, align 8, addrspace(5)
  %"2085" = alloca i64, align 8, addrspace(5)
  %"712" = alloca i32, align 4, addrspace(5)
  %"2087" = alloca i64, align 8, addrspace(5)
  %"2089" = alloca i64, align 8, addrspace(5)
  %"2091" = alloca i64, align 8, addrspace(5)
  %"2093" = alloca [3 x i32], align 4, addrspace(5)
  %"718" = alloca i32, align 4, addrspace(5)
  %"2097" = alloca i64, align 8, addrspace(5)
  %"2099" = alloca i64, align 8, addrspace(5)
  %"2101" = alloca i64, align 8, addrspace(5)
  %"723" = alloca i32, align 4, addrspace(5)
  %"2103" = alloca i64, align 8, addrspace(5)
  %"2105" = alloca i64, align 8, addrspace(5)
  %"2107" = alloca i64, align 8, addrspace(5)
  %"728" = alloca i32, align 4, addrspace(5)
  %"2109" = alloca i64, align 8, addrspace(5)
  %"2111" = alloca i64, align 8, addrspace(5)
  %"2113" = alloca i64, align 8, addrspace(5)
  %"2115" = alloca [3 x i32], align 4, addrspace(5)
  %"2394" = ptrtoint ptr addrspace(5) %"312" to i64
  %0 = alloca i64, align 8, addrspace(5)
  store i64 %"2394", ptr addrspace(5) %0, align 8
  %"2393" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"2393", ptr addrspace(5) %"314", align 8
  %"858" = load i64, ptr addrspace(5) %"314", align 8
  %1 = inttoptr i64 %"858" to ptr addrspace(5)
  %2 = addrspacecast ptr addrspace(5) %1 to ptr
  %"857" = ptrtoint ptr %2 to i64
  store i64 %"857", ptr addrspace(5) %"313", align 8
  %"860" = load i64, ptr addrspace(5) %"313", align 8
  %"2395" = add i64 %"860", 116
  store i64 %"2395", ptr addrspace(5) %"617", align 8
  %"862" = load i64, ptr addrspace(5) %"314", align 8
  %"2397" = add i64 %"862", 116
  store i64 %"2397", ptr addrspace(5) %"608", align 8
  %3 = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %3, align 8
  %"2399" = load i64, ptr addrspace(5) %3, align 8
  store i64 %"2399", ptr addrspace(5) %"616", align 8
  %"2140" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 12)
  %"864" = load float, ptr addrspace(1) %"2140", align 4
  store float %"864", ptr addrspace(5) %"360", align 4
  %"2142" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 12)
  %"2715" = getelementptr inbounds i8, ptr addrspace(1) %"2142", i64 4
  %"865" = load float, ptr addrspace(1) %"2715", align 4
  store float %"865", ptr addrspace(5) %"361", align 4
  %"2145" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 12)
  %"2717" = getelementptr inbounds i8, ptr addrspace(1) %"2145", i64 8
  %"866" = load float, ptr addrspace(1) %"2717", align 4
  store float %"866", ptr addrspace(5) %"362", align 4
  %4 = alloca i32, align 4, addrspace(5)
  store i32 7937, ptr addrspace(5) %4, align 4
  %"2403" = load i32, ptr addrspace(5) %4, align 4
  store i32 %"2403", ptr addrspace(5) %"563", align 4
  %5 = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %5, align 4
  %"868" = load float, ptr addrspace(5) %5, align 4
  store float %"868", ptr addrspace(5) %"556", align 4
  %"873" = load i32, ptr addrspace(5) %"563", align 4
  %"874" = load float, ptr addrspace(5) %"360", align 4
  %"875" = load float, ptr addrspace(5) %"361", align 4
  %"876" = load float, ptr addrspace(5) %"362", align 4
  %"877" = load float, ptr addrspace(5) %"556", align 4
  %6 = call %struct.f32f32f32f32 @__zluda_rt_ptx_impl___rt_transform_tuple(i32 %"873", float %"874", float %"875", float %"876", float %"877")
  %"869" = extractvalue %struct.f32f32f32f32 %6, 0
  %"870" = extractvalue %struct.f32f32f32f32 %6, 1
  %"871" = extractvalue %struct.f32f32f32f32 %6, 2
  %"872" = extractvalue %struct.f32f32f32f32 %6, 3
  store float %"869", ptr addrspace(5) %"356", align 4
  store float %"870", ptr addrspace(5) %"357", align 4
  store float %"871", ptr addrspace(5) %"358", align 4
  store float %"872", ptr addrspace(5) %"359", align 4
  %"879" = load float, ptr addrspace(5) %"357", align 4
  %"880" = load float, ptr addrspace(5) %"357", align 4
  %"878" = fmul float %"879", %"880"
  store float %"878", ptr addrspace(5) %"372", align 4
  %"882" = load float, ptr addrspace(5) %"356", align 4
  %"883" = load float, ptr addrspace(5) %"356", align 4
  %"884" = load float, ptr addrspace(5) %"372", align 4
  %"881" = call float @llvm.fma.f32(float %"882", float %"883", float %"884")
  store float %"881", ptr addrspace(5) %"373", align 4
  %"886" = load float, ptr addrspace(5) %"358", align 4
  %"887" = load float, ptr addrspace(5) %"358", align 4
  %"888" = load float, ptr addrspace(5) %"373", align 4
  %"885" = call float @llvm.fma.f32(float %"886", float %"887", float %"888")
  store float %"885", ptr addrspace(5) %"374", align 4
  %"890" = load float, ptr addrspace(5) %"374", align 4
  %7 = call afn float @llvm.sqrt.f32(float %"890")
  %"889" = fdiv arcp afn float 1.000000e+00, %7
  store float %"889", ptr addrspace(5) %"375", align 4
  %"892" = load float, ptr addrspace(5) %"375", align 4
  %"893" = load float, ptr addrspace(5) %"356", align 4
  %"891" = fmul float %"892", %"893"
  store float %"891", ptr addrspace(5) %"329", align 4
  %"895" = load float, ptr addrspace(5) %"375", align 4
  %"896" = load float, ptr addrspace(5) %"357", align 4
  %"894" = fmul float %"895", %"896"
  store float %"894", ptr addrspace(5) %"330", align 4
  %"898" = load float, ptr addrspace(5) %"375", align 4
  %"899" = load float, ptr addrspace(5) %"358", align 4
  %"897" = fmul float %"898", %"899"
  store float %"897", ptr addrspace(5) %"331", align 4
  %"2148" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 48)
  %"900" = load float, ptr addrspace(1) %"2148", align 4
  store float %"900", ptr addrspace(5) %"368", align 4
  %"2150" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 48)
  %"2719" = getelementptr inbounds i8, ptr addrspace(1) %"2150", i64 4
  %"901" = load float, ptr addrspace(1) %"2719", align 4
  store float %"901", ptr addrspace(5) %"369", align 4
  %"2153" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 48)
  %"2721" = getelementptr inbounds i8, ptr addrspace(1) %"2153", i64 8
  %"902" = load float, ptr addrspace(1) %"2721", align 4
  store float %"902", ptr addrspace(5) %"370", align 4
  %"907" = load i32, ptr addrspace(5) %"563", align 4
  %"908" = load float, ptr addrspace(5) %"368", align 4
  %"909" = load float, ptr addrspace(5) %"369", align 4
  %"910" = load float, ptr addrspace(5) %"370", align 4
  %"911" = load float, ptr addrspace(5) %"556", align 4
  %8 = call %struct.f32f32f32f32 @__zluda_rt_ptx_impl___rt_transform_tuple(i32 %"907", float %"908", float %"909", float %"910", float %"911")
  %"903" = extractvalue %struct.f32f32f32f32 %8, 0
  %"904" = extractvalue %struct.f32f32f32f32 %8, 1
  %"905" = extractvalue %struct.f32f32f32f32 %8, 2
  %"906" = extractvalue %struct.f32f32f32f32 %8, 3
  store float %"903", ptr addrspace(5) %"364", align 4
  store float %"904", ptr addrspace(5) %"365", align 4
  store float %"905", ptr addrspace(5) %"366", align 4
  store float %"906", ptr addrspace(5) %"367", align 4
  %"913" = load float, ptr addrspace(5) %"365", align 4
  %"914" = load float, ptr addrspace(5) %"365", align 4
  %"912" = fmul float %"913", %"914"
  store float %"912", ptr addrspace(5) %"376", align 4
  %"916" = load float, ptr addrspace(5) %"364", align 4
  %"917" = load float, ptr addrspace(5) %"364", align 4
  %"918" = load float, ptr addrspace(5) %"376", align 4
  %"915" = call float @llvm.fma.f32(float %"916", float %"917", float %"918")
  store float %"915", ptr addrspace(5) %"377", align 4
  %"920" = load float, ptr addrspace(5) %"366", align 4
  %"921" = load float, ptr addrspace(5) %"366", align 4
  %"922" = load float, ptr addrspace(5) %"377", align 4
  %"919" = call float @llvm.fma.f32(float %"920", float %"921", float %"922")
  store float %"919", ptr addrspace(5) %"378", align 4
  %"924" = load float, ptr addrspace(5) %"378", align 4
  %9 = call afn float @llvm.sqrt.f32(float %"924")
  %"923" = fdiv arcp afn float 1.000000e+00, %9
  store float %"923", ptr addrspace(5) %"379", align 4
  %"926" = load float, ptr addrspace(5) %"379", align 4
  %"927" = load float, ptr addrspace(5) %"364", align 4
  %"925" = fmul float %"926", %"927"
  store float %"925", ptr addrspace(5) %"380", align 4
  %"929" = load float, ptr addrspace(5) %"379", align 4
  %"930" = load float, ptr addrspace(5) %"365", align 4
  %"928" = fmul float %"929", %"930"
  store float %"928", ptr addrspace(5) %"381", align 4
  %"932" = load float, ptr addrspace(5) %"379", align 4
  %"933" = load float, ptr addrspace(5) %"366", align 4
  %"931" = fmul float %"932", %"933"
  store float %"931", ptr addrspace(5) %"382", align 4
  %"2723" = getelementptr inbounds i8, ptr addrspace(5) %"2131", i64 12
  %"934" = load float, ptr addrspace(5) %"2723", align 4
  store float %"934", ptr addrspace(5) %"383", align 4
  %"936" = load float, ptr addrspace(5) %"383", align 4
  %"937" = load float, ptr addrspace(5) %"380", align 4
  %"935" = fmul float %"936", %"937"
  store float %"935", ptr addrspace(5) %"384", align 4
  %"2725" = getelementptr inbounds i8, ptr addrspace(5) %"2131", i64 16
  %"938" = load float, ptr addrspace(5) %"2725", align 4
  store float %"938", ptr addrspace(5) %"385", align 4
  %"940" = load float, ptr addrspace(5) %"385", align 4
  %"941" = load float, ptr addrspace(5) %"381", align 4
  %"939" = fmul float %"940", %"941"
  store float %"939", ptr addrspace(5) %"386", align 4
  %"943" = load float, ptr addrspace(5) %"386", align 4
  %"942" = fsub float 0.000000e+00, %"943"
  store float %"942", ptr addrspace(5) %"387", align 4
  %"945" = load float, ptr addrspace(5) %"387", align 4
  %"946" = load float, ptr addrspace(5) %"384", align 4
  %"944" = fsub float %"945", %"946"
  store float %"944", ptr addrspace(5) %"388", align 4
  %"2727" = getelementptr inbounds i8, ptr addrspace(5) %"2131", i64 20
  %"947" = load float, ptr addrspace(5) %"2727", align 4
  store float %"947", ptr addrspace(5) %"389", align 4
  %"949" = load float, ptr addrspace(5) %"389", align 4
  %"950" = load float, ptr addrspace(5) %"382", align 4
  %"948" = fmul float %"949", %"950"
  store float %"948", ptr addrspace(5) %"390", align 4
  %"952" = load float, ptr addrspace(5) %"388", align 4
  %"953" = load float, ptr addrspace(5) %"390", align 4
  %"951" = fsub float %"952", %"953"
  store float %"951", ptr addrspace(5) %"391", align 4
  %"955" = load float, ptr addrspace(5) %"391", align 4
  %"2410" = bitcast float %"955" to i32
  %10 = alloca i32, align 4, addrspace(5)
  store i32 %"2410", ptr addrspace(5) %10, align 4
  %"954" = load i32, ptr addrspace(5) %10, align 4
  store i32 %"954", ptr addrspace(5) %"566", align 4
  %"957" = load i32, ptr addrspace(5) %"566", align 4
  %"956" = and i32 %"957", -2147483648
  store i32 %"956", ptr addrspace(5) %"567", align 4
  %"959" = load i32, ptr addrspace(5) %"567", align 4
  %"958" = or i32 %"959", 1065353216
  store i32 %"958", ptr addrspace(5) %"568", align 4
  %"961" = load i32, ptr addrspace(5) %"568", align 4
  %11 = alloca i32, align 4, addrspace(5)
  store i32 %"961", ptr addrspace(5) %11, align 4
  %"2411" = load i32, ptr addrspace(5) %11, align 4
  %"960" = bitcast i32 %"2411" to float
  store float %"960", ptr addrspace(5) %"392", align 4
  %"963" = load float, ptr addrspace(5) %"329", align 4
  %"964" = load float, ptr addrspace(5) %"392", align 4
  %"962" = fmul float %"963", %"964"
  store float %"962", ptr addrspace(5) %"332", align 4
  %"966" = load float, ptr addrspace(5) %"330", align 4
  %"967" = load float, ptr addrspace(5) %"392", align 4
  %"965" = fmul float %"966", %"967"
  store float %"965", ptr addrspace(5) %"333", align 4
  %"969" = load float, ptr addrspace(5) %"331", align 4
  %"970" = load float, ptr addrspace(5) %"392", align 4
  %"968" = fmul float %"969", %"970"
  store float %"968", ptr addrspace(5) %"334", align 4
  %"2159" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 160)
  %"2412" = load i32, ptr addrspace(1) %"2159", align 4
  %"971" = sext i32 %"2412" to i64
  store i64 %"971", ptr addrspace(5) %"613", align 8
  %"2161" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 104)
  %"2415" = ptrtoint ptr addrspace(1) %"2161" to i64
  %12 = alloca i64, align 8, addrspace(5)
  store i64 %"2415", ptr addrspace(5) %12, align 8
  %"2414" = load i64, ptr addrspace(5) %12, align 8
  store i64 %"2414", ptr addrspace(5) %"618", align 8
  %"974" = load i64, ptr addrspace(5) %"618", align 8
  %13 = inttoptr i64 %"974" to ptr addrspace(1)
  %14 = addrspacecast ptr addrspace(1) %13 to ptr
  %"973" = ptrtoint ptr %14 to i64
  store i64 %"973", ptr addrspace(5) %"612", align 8
  %15 = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %15, align 4
  %"2416" = load i32, ptr addrspace(5) %15, align 4
  store i32 %"2416", ptr addrspace(5) %"564", align 4
  %16 = alloca i32, align 4, addrspace(5)
  store i32 72, ptr addrspace(5) %16, align 4
  %"2417" = load i32, ptr addrspace(5) %16, align 4
  store i32 %"2417", ptr addrspace(5) %"565", align 4
  %"978" = load i64, ptr addrspace(5) %"612", align 8
  %"979" = load i32, ptr addrspace(5) %"564", align 4
  %"980" = load i32, ptr addrspace(5) %"565", align 4
  %"981" = load i64, ptr addrspace(5) %"613", align 8
  %"982" = load i64, ptr addrspace(5) %"616", align 8
  %"983" = load i64, ptr addrspace(5) %"616", align 8
  %"984" = load i64, ptr addrspace(5) %"616", align 8
  %"977" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"978", i32 %"979", i32 %"980", i64 %"981", i64 %"982", i64 %"983", i64 %"984")
  store i64 %"977", ptr addrspace(5) %"611", align 8
  %"986" = load i64, ptr addrspace(5) %"611", align 8
  %"2419" = inttoptr i64 %"986" to ptr
  %"2418" = load i32, ptr %"2419", align 4
  store i32 %"2418", ptr addrspace(5) %"560", align 4
  %"988" = load i64, ptr addrspace(5) %"611", align 8
  %"2420" = inttoptr i64 %"988" to ptr
  %"2729" = getelementptr inbounds i8, ptr %"2420", i64 4
  %"987" = load float, ptr %"2729", align 4
  store float %"987", ptr addrspace(5) %"393", align 4
  %"990" = load i64, ptr addrspace(5) %"611", align 8
  %"2421" = inttoptr i64 %"990" to ptr
  %"2731" = getelementptr inbounds i8, ptr %"2421", i64 8
  %"989" = load float, ptr %"2731", align 4
  store float %"989", ptr addrspace(5) %"394", align 4
  %"992" = load i64, ptr addrspace(5) %"611", align 8
  %"2422" = inttoptr i64 %"992" to ptr
  %"2733" = getelementptr inbounds i8, ptr %"2422", i64 12
  %"991" = load float, ptr %"2733", align 4
  store float %"991", ptr addrspace(5) %"395", align 4
  %"994" = load i64, ptr addrspace(5) %"611", align 8
  %"2423" = inttoptr i64 %"994" to ptr
  %"2735" = getelementptr inbounds i8, ptr %"2423", i64 16
  %"993" = load float, ptr %"2735", align 4
  store float %"993", ptr addrspace(5) %"335", align 4
  %"996" = load i64, ptr addrspace(5) %"611", align 8
  %"2424" = inttoptr i64 %"996" to ptr
  %"2737" = getelementptr inbounds i8, ptr %"2424", i64 20
  %"995" = load float, ptr %"2737", align 4
  store float %"995", ptr addrspace(5) %"336", align 4
  %"998" = load i64, ptr addrspace(5) %"611", align 8
  %"2425" = inttoptr i64 %"998" to ptr
  %"2739" = getelementptr inbounds i8, ptr %"2425", i64 24
  %"997" = load float, ptr %"2739", align 4
  store float %"997", ptr addrspace(5) %"337", align 4
  %"1000" = load i64, ptr addrspace(5) %"611", align 8
  %"2426" = inttoptr i64 %"1000" to ptr
  %"2741" = getelementptr inbounds i8, ptr %"2426", i64 28
  %"999" = load float, ptr %"2741", align 4
  store float %"999", ptr addrspace(5) %"396", align 4
  %"1002" = load i64, ptr addrspace(5) %"611", align 8
  %"2427" = inttoptr i64 %"1002" to ptr
  %"2743" = getelementptr inbounds i8, ptr %"2427", i64 32
  %"1001" = load float, ptr %"2743", align 4
  store float %"1001", ptr addrspace(5) %"397", align 4
  %"1004" = load i64, ptr addrspace(5) %"611", align 8
  %"2428" = inttoptr i64 %"1004" to ptr
  %"2745" = getelementptr inbounds i8, ptr %"2428", i64 36
  %"1003" = load float, ptr %"2745", align 4
  store float %"1003", ptr addrspace(5) %"398", align 4
  %"1006" = load i64, ptr addrspace(5) %"611", align 8
  %"2429" = inttoptr i64 %"1006" to ptr
  %"2747" = getelementptr inbounds i8, ptr %"2429", i64 40
  %"1005" = load float, ptr %"2747", align 4
  store float %"1005", ptr addrspace(5) %"399", align 4
  %"1008" = load i64, ptr addrspace(5) %"611", align 8
  %"2430" = inttoptr i64 %"1008" to ptr
  %"2749" = getelementptr inbounds i8, ptr %"2430", i64 44
  %"1007" = load float, ptr %"2749", align 4
  store float %"1007", ptr addrspace(5) %"400", align 4
  %"1010" = load i64, ptr addrspace(5) %"611", align 8
  %"2431" = inttoptr i64 %"1010" to ptr
  %"2751" = getelementptr inbounds i8, ptr %"2431", i64 48
  %"1009" = load float, ptr %"2751", align 4
  store float %"1009", ptr addrspace(5) %"401", align 4
  %"1012" = load i64, ptr addrspace(5) %"611", align 8
  %"2432" = inttoptr i64 %"1012" to ptr
  %"2753" = getelementptr inbounds i8, ptr %"2432", i64 52
  %"1011" = load float, ptr %"2753", align 4
  store float %"1011", ptr addrspace(5) %"402", align 4
  %"1014" = load i64, ptr addrspace(5) %"611", align 8
  %"2433" = inttoptr i64 %"1014" to ptr
  %"2755" = getelementptr inbounds i8, ptr %"2433", i64 56
  %"1013" = load float, ptr %"2755", align 4
  store float %"1013", ptr addrspace(5) %"403", align 4
  %"1016" = load i64, ptr addrspace(5) %"611", align 8
  %"2434" = inttoptr i64 %"1016" to ptr
  %"2757" = getelementptr inbounds i8, ptr %"2434", i64 60
  %"1015" = load float, ptr %"2757", align 4
  store float %"1015", ptr addrspace(5) %"404", align 4
  %"1018" = load i64, ptr addrspace(5) %"611", align 8
  %"2435" = inttoptr i64 %"1018" to ptr
  %"2759" = getelementptr inbounds i8, ptr %"2435", i64 64
  %"1017" = load float, ptr %"2759", align 4
  store float %"1017", ptr addrspace(5) %"405", align 4
  %"1020" = load i64, ptr addrspace(5) %"611", align 8
  %"2436" = inttoptr i64 %"1020" to ptr
  %"2761" = getelementptr inbounds i8, ptr %"2436", i64 68
  %"2437" = load i32, ptr %"2761", align 4
  store i32 %"2437", ptr addrspace(5) %"561", align 4
  %"1021" = load i64, ptr addrspace(5) %"608", align 8
  %"1022" = load i32, ptr addrspace(5) %"560", align 4
  %"2438" = inttoptr i64 %"1021" to ptr addrspace(5)
  store i32 %"1022", ptr addrspace(5) %"2438", align 4
  %"1023" = load i64, ptr addrspace(5) %"608", align 8
  %"1024" = load float, ptr addrspace(5) %"393", align 4
  %"2440" = inttoptr i64 %"1023" to ptr addrspace(5)
  %"2763" = getelementptr inbounds i8, ptr addrspace(5) %"2440", i64 4
  store float %"1024", ptr addrspace(5) %"2763", align 4
  %"1025" = load i64, ptr addrspace(5) %"608", align 8
  %"1026" = load float, ptr addrspace(5) %"394", align 4
  %"2441" = inttoptr i64 %"1025" to ptr addrspace(5)
  %"2765" = getelementptr inbounds i8, ptr addrspace(5) %"2441", i64 8
  store float %"1026", ptr addrspace(5) %"2765", align 4
  %"1027" = load i64, ptr addrspace(5) %"608", align 8
  %"1028" = load float, ptr addrspace(5) %"395", align 4
  %"2442" = inttoptr i64 %"1027" to ptr addrspace(5)
  %"2767" = getelementptr inbounds i8, ptr addrspace(5) %"2442", i64 12
  store float %"1028", ptr addrspace(5) %"2767", align 4
  %"1029" = load i64, ptr addrspace(5) %"608", align 8
  %"1030" = load float, ptr addrspace(5) %"335", align 4
  %"2443" = inttoptr i64 %"1029" to ptr addrspace(5)
  %"2769" = getelementptr inbounds i8, ptr addrspace(5) %"2443", i64 16
  store float %"1030", ptr addrspace(5) %"2769", align 4
  %"1031" = load i64, ptr addrspace(5) %"608", align 8
  %"1032" = load float, ptr addrspace(5) %"336", align 4
  %"2444" = inttoptr i64 %"1031" to ptr addrspace(5)
  %"2771" = getelementptr inbounds i8, ptr addrspace(5) %"2444", i64 20
  store float %"1032", ptr addrspace(5) %"2771", align 4
  %"1033" = load i64, ptr addrspace(5) %"608", align 8
  %"1034" = load float, ptr addrspace(5) %"337", align 4
  %"2445" = inttoptr i64 %"1033" to ptr addrspace(5)
  %"2773" = getelementptr inbounds i8, ptr addrspace(5) %"2445", i64 24
  store float %"1034", ptr addrspace(5) %"2773", align 4
  %"1035" = load i64, ptr addrspace(5) %"608", align 8
  %"1036" = load float, ptr addrspace(5) %"396", align 4
  %"2446" = inttoptr i64 %"1035" to ptr addrspace(5)
  %"2775" = getelementptr inbounds i8, ptr addrspace(5) %"2446", i64 28
  store float %"1036", ptr addrspace(5) %"2775", align 4
  %"1037" = load i64, ptr addrspace(5) %"608", align 8
  %"1038" = load float, ptr addrspace(5) %"397", align 4
  %"2447" = inttoptr i64 %"1037" to ptr addrspace(5)
  %"2777" = getelementptr inbounds i8, ptr addrspace(5) %"2447", i64 32
  store float %"1038", ptr addrspace(5) %"2777", align 4
  %"1039" = load i64, ptr addrspace(5) %"608", align 8
  %"1040" = load float, ptr addrspace(5) %"398", align 4
  %"2448" = inttoptr i64 %"1039" to ptr addrspace(5)
  %"2779" = getelementptr inbounds i8, ptr addrspace(5) %"2448", i64 36
  store float %"1040", ptr addrspace(5) %"2779", align 4
  %"1041" = load i64, ptr addrspace(5) %"608", align 8
  %"1042" = load float, ptr addrspace(5) %"399", align 4
  %"2449" = inttoptr i64 %"1041" to ptr addrspace(5)
  %"2781" = getelementptr inbounds i8, ptr addrspace(5) %"2449", i64 40
  store float %"1042", ptr addrspace(5) %"2781", align 4
  %"1043" = load i64, ptr addrspace(5) %"608", align 8
  %"1044" = load float, ptr addrspace(5) %"400", align 4
  %"2450" = inttoptr i64 %"1043" to ptr addrspace(5)
  %"2783" = getelementptr inbounds i8, ptr addrspace(5) %"2450", i64 44
  store float %"1044", ptr addrspace(5) %"2783", align 4
  %"1045" = load i64, ptr addrspace(5) %"608", align 8
  %"1046" = load float, ptr addrspace(5) %"401", align 4
  %"2451" = inttoptr i64 %"1045" to ptr addrspace(5)
  %"2785" = getelementptr inbounds i8, ptr addrspace(5) %"2451", i64 48
  store float %"1046", ptr addrspace(5) %"2785", align 4
  %"1047" = load i64, ptr addrspace(5) %"608", align 8
  %"1048" = load float, ptr addrspace(5) %"402", align 4
  %"2452" = inttoptr i64 %"1047" to ptr addrspace(5)
  %"2787" = getelementptr inbounds i8, ptr addrspace(5) %"2452", i64 52
  store float %"1048", ptr addrspace(5) %"2787", align 4
  %"1049" = load i64, ptr addrspace(5) %"608", align 8
  %"1050" = load float, ptr addrspace(5) %"403", align 4
  %"2453" = inttoptr i64 %"1049" to ptr addrspace(5)
  %"2789" = getelementptr inbounds i8, ptr addrspace(5) %"2453", i64 56
  store float %"1050", ptr addrspace(5) %"2789", align 4
  %"1051" = load i64, ptr addrspace(5) %"608", align 8
  %"1052" = load float, ptr addrspace(5) %"404", align 4
  %"2454" = inttoptr i64 %"1051" to ptr addrspace(5)
  %"2791" = getelementptr inbounds i8, ptr addrspace(5) %"2454", i64 60
  store float %"1052", ptr addrspace(5) %"2791", align 4
  %"1053" = load i64, ptr addrspace(5) %"608", align 8
  %"1054" = load float, ptr addrspace(5) %"405", align 4
  %"2455" = inttoptr i64 %"1053" to ptr addrspace(5)
  %"2793" = getelementptr inbounds i8, ptr addrspace(5) %"2455", i64 64
  store float %"1054", ptr addrspace(5) %"2793", align 4
  %"1055" = load i64, ptr addrspace(5) %"608", align 8
  %"1056" = load i32, ptr addrspace(5) %"561", align 4
  %"2456" = inttoptr i64 %"1055" to ptr addrspace(5)
  %"2795" = getelementptr inbounds i8, ptr addrspace(5) %"2456", i64 68
  store i32 %"1056", ptr addrspace(5) %"2795", align 4
  %"1058" = load i32, ptr addrspace(5) %"560", align 4
  %"1057" = icmp eq i32 %"1058", 0
  store i1 %"1057", ptr addrspace(5) %"316", align 1
  %"1059" = load i1, ptr addrspace(5) %"316", align 1
  br i1 %"1059", label %"306", label %"735"

"735":                                            ; preds = %"2703"
  %"2163" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 36)
  %"1060" = load float, ptr addrspace(1) %"2163", align 4
  store float %"1060", ptr addrspace(5) %"410", align 4
  %"2165" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 36)
  %"2797" = getelementptr inbounds i8, ptr addrspace(1) %"2165", i64 4
  %"1061" = load float, ptr addrspace(1) %"2797", align 4
  store float %"1061", ptr addrspace(5) %"411", align 4
  %17 = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %17, align 4
  %"2461" = load i32, ptr addrspace(5) %17, align 4
  store i32 %"2461", ptr addrspace(5) %"570", align 4
  %"1067" = load i32, ptr addrspace(5) %"560", align 4
  %"1068" = load i32, ptr addrspace(5) %"570", align 4
  %"1069" = load float, ptr addrspace(5) %"410", align 4
  %"1070" = load float, ptr addrspace(5) %"411", align 4
  %"1071" = load float, ptr addrspace(5) %"556", align 4
  %"1072" = load float, ptr addrspace(5) %"556", align 4
  %18 = call %struct.f32f32f32f32 @__zluda_rt_ptx_impl___rt_texture_get_f_id(i32 %"1067", i32 %"1068", float %"1069", float %"1070", float %"1071", float %"1072", ptr addrspace(3) %"2127")
  %"1063" = extractvalue %struct.f32f32f32f32 %18, 0
  %"1064" = extractvalue %struct.f32f32f32f32 %18, 1
  %"1065" = extractvalue %struct.f32f32f32f32 %18, 2
  %"1066" = extractvalue %struct.f32f32f32f32 %18, 3
  store float %"1063", ptr addrspace(5) %"406", align 4
  store float %"1064", ptr addrspace(5) %"407", align 4
  store float %"1065", ptr addrspace(5) %"408", align 4
  store float %"1066", ptr addrspace(5) %"409", align 4
  %"1074" = load float, ptr addrspace(5) %"406", align 4
  %"1073" = call afn float @llvm.log2.f32(float %"1074")
  store float %"1073", ptr addrspace(5) %"414", align 4
  %"1076" = load float, ptr addrspace(5) %"414", align 4
  %"1075" = fmul float %"1076", 0x40019999A0000000
  store float %"1075", ptr addrspace(5) %"415", align 4
  %"1078" = load float, ptr addrspace(5) %"415", align 4
  %"1077" = call afn float @llvm.exp2.f32(float %"1078")
  store float %"1077", ptr addrspace(5) %"416", align 4
  %"1080" = load float, ptr addrspace(5) %"407", align 4
  %"1079" = call afn float @llvm.log2.f32(float %"1080")
  store float %"1079", ptr addrspace(5) %"417", align 4
  %"1082" = load float, ptr addrspace(5) %"417", align 4
  %"1081" = fmul float %"1082", 0x40019999A0000000
  store float %"1081", ptr addrspace(5) %"418", align 4
  %"1084" = load float, ptr addrspace(5) %"418", align 4
  %"1083" = call afn float @llvm.exp2.f32(float %"1084")
  store float %"1083", ptr addrspace(5) %"419", align 4
  %"1086" = load float, ptr addrspace(5) %"408", align 4
  %"1085" = call afn float @llvm.log2.f32(float %"1086")
  store float %"1085", ptr addrspace(5) %"420", align 4
  %"1088" = load float, ptr addrspace(5) %"420", align 4
  %"1087" = fmul float %"1088", 0x40019999A0000000
  store float %"1087", ptr addrspace(5) %"421", align 4
  %"1090" = load float, ptr addrspace(5) %"421", align 4
  %"1089" = call afn float @llvm.exp2.f32(float %"1090")
  store float %"1089", ptr addrspace(5) %"422", align 4
  %"1091" = load i64, ptr addrspace(5) %"608", align 8
  %"1092" = load float, ptr addrspace(5) %"416", align 4
  %"2462" = inttoptr i64 %"1091" to ptr addrspace(5)
  %"2799" = getelementptr inbounds i8, ptr addrspace(5) %"2462", i64 4
  store float %"1092", ptr addrspace(5) %"2799", align 4
  %"1093" = load i64, ptr addrspace(5) %"608", align 8
  %"1094" = load float, ptr addrspace(5) %"419", align 4
  %"2463" = inttoptr i64 %"1093" to ptr addrspace(5)
  %"2801" = getelementptr inbounds i8, ptr addrspace(5) %"2463", i64 8
  store float %"1094", ptr addrspace(5) %"2801", align 4
  %"1095" = load i64, ptr addrspace(5) %"608", align 8
  %"1096" = load float, ptr addrspace(5) %"422", align 4
  %"2464" = inttoptr i64 %"1095" to ptr addrspace(5)
  %"2803" = getelementptr inbounds i8, ptr addrspace(5) %"2464", i64 12
  store float %"1096", ptr addrspace(5) %"2803", align 4
  br label %"306"

"306":                                            ; preds = %"735", %"2703"
  %"2168" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 24)
  %"1097" = load float, ptr addrspace(1) %"2168", align 4
  store float %"1097", ptr addrspace(5) %"338", align 4
  %"2170" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 24)
  %"2805" = getelementptr inbounds i8, ptr addrspace(1) %"2170", i64 4
  %"1098" = load float, ptr addrspace(1) %"2805", align 4
  store float %"1098", ptr addrspace(5) %"339", align 4
  %"2173" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 24)
  %"2807" = getelementptr inbounds i8, ptr addrspace(1) %"2173", i64 8
  %"1099" = load float, ptr addrspace(1) %"2807", align 4
  store float %"1099", ptr addrspace(5) %"340", align 4
  %"1101" = load i64, ptr addrspace(5) %"313", align 8
  %"2468" = add i64 %"1101", 188
  store i64 %"2468", ptr addrspace(5) %"619", align 8
  %"1103" = load i64, ptr addrspace(5) %"314", align 8
  %"2470" = add i64 %"1103", 188
  store i64 %"2470", ptr addrspace(5) %"620", align 8
  %"1104" = load i64, ptr addrspace(5) %"620", align 8
  %"1105" = load float, ptr addrspace(5) %"338", align 4
  %"2472" = inttoptr i64 %"1104" to ptr addrspace(5)
  store float %"1105", ptr addrspace(5) %"2472", align 4
  %"1106" = load i64, ptr addrspace(5) %"620", align 8
  %"1107" = load float, ptr addrspace(5) %"339", align 4
  %"2473" = inttoptr i64 %"1106" to ptr addrspace(5)
  %"2809" = getelementptr inbounds i8, ptr addrspace(5) %"2473", i64 4
  store float %"1107", ptr addrspace(5) %"2809", align 4
  %"1108" = load i64, ptr addrspace(5) %"620", align 8
  %"1109" = load float, ptr addrspace(5) %"340", align 4
  %"2474" = inttoptr i64 %"1108" to ptr addrspace(5)
  %"2811" = getelementptr inbounds i8, ptr addrspace(5) %"2474", i64 8
  store float %"1109", ptr addrspace(5) %"2811", align 4
  %"2176" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 0)
  %"1110" = load float, ptr addrspace(1) %"2176", align 4
  store float %"1110", ptr addrspace(5) %"423", align 4
  %"2178" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 0)
  %"2813" = getelementptr inbounds i8, ptr addrspace(1) %"2178", i64 4
  %"1111" = load float, ptr addrspace(1) %"2813", align 4
  store float %"1111", ptr addrspace(5) %"424", align 4
  %"2181" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2138", i32 0)
  %"2815" = getelementptr inbounds i8, ptr addrspace(1) %"2181", i64 8
  %"1112" = load float, ptr addrspace(1) %"2815", align 4
  store float %"1112", ptr addrspace(5) %"425", align 4
  %"1113" = load i64, ptr addrspace(5) %"620", align 8
  %"1114" = load float, ptr addrspace(5) %"423", align 4
  %"2478" = inttoptr i64 %"1113" to ptr addrspace(5)
  %"2817" = getelementptr inbounds i8, ptr addrspace(5) %"2478", i64 12
  store float %"1114", ptr addrspace(5) %"2817", align 4
  %"1115" = load i64, ptr addrspace(5) %"620", align 8
  %"1116" = load float, ptr addrspace(5) %"424", align 4
  %"2479" = inttoptr i64 %"1115" to ptr addrspace(5)
  %"2819" = getelementptr inbounds i8, ptr addrspace(5) %"2479", i64 16
  store float %"1116", ptr addrspace(5) %"2819", align 4
  %"1117" = load i64, ptr addrspace(5) %"620", align 8
  %"1118" = load float, ptr addrspace(5) %"425", align 4
  %"2480" = inttoptr i64 %"1117" to ptr addrspace(5)
  %"2821" = getelementptr inbounds i8, ptr addrspace(5) %"2480", i64 20
  store float %"1118", ptr addrspace(5) %"2821", align 4
  %"1119" = load i64, ptr addrspace(5) %"620", align 8
  %"1120" = load float, ptr addrspace(5) %"329", align 4
  %"2481" = inttoptr i64 %"1119" to ptr addrspace(5)
  %"2823" = getelementptr inbounds i8, ptr addrspace(5) %"2481", i64 24
  store float %"1120", ptr addrspace(5) %"2823", align 4
  %"1121" = load i64, ptr addrspace(5) %"620", align 8
  %"1122" = load float, ptr addrspace(5) %"330", align 4
  %"2482" = inttoptr i64 %"1121" to ptr addrspace(5)
  %"2825" = getelementptr inbounds i8, ptr addrspace(5) %"2482", i64 28
  store float %"1122", ptr addrspace(5) %"2825", align 4
  %"1123" = load i64, ptr addrspace(5) %"620", align 8
  %"1124" = load float, ptr addrspace(5) %"331", align 4
  %"2483" = inttoptr i64 %"1123" to ptr addrspace(5)
  %"2827" = getelementptr inbounds i8, ptr addrspace(5) %"2483", i64 32
  store float %"1124", ptr addrspace(5) %"2827", align 4
  %"1125" = load i64, ptr addrspace(5) %"620", align 8
  %"1126" = load float, ptr addrspace(5) %"332", align 4
  %"2484" = inttoptr i64 %"1125" to ptr addrspace(5)
  %"2829" = getelementptr inbounds i8, ptr addrspace(5) %"2484", i64 36
  store float %"1126", ptr addrspace(5) %"2829", align 4
  %"1127" = load i64, ptr addrspace(5) %"620", align 8
  %"1128" = load float, ptr addrspace(5) %"333", align 4
  %"2485" = inttoptr i64 %"1127" to ptr addrspace(5)
  %"2831" = getelementptr inbounds i8, ptr addrspace(5) %"2485", i64 40
  store float %"1128", ptr addrspace(5) %"2831", align 4
  %"1129" = load i64, ptr addrspace(5) %"620", align 8
  %"1130" = load float, ptr addrspace(5) %"334", align 4
  %"2486" = inttoptr i64 %"1129" to ptr addrspace(5)
  %"2833" = getelementptr inbounds i8, ptr addrspace(5) %"2486", i64 44
  store float %"1130", ptr addrspace(5) %"2833", align 4
  %"2835" = getelementptr inbounds i8, ptr addrspace(5) %"2131", i64 12
  %"1131" = load float, ptr addrspace(5) %"2835", align 4
  store float %"1131", ptr addrspace(5) %"426", align 4
  %"1133" = load float, ptr addrspace(5) %"426", align 4
  %"1132" = fsub float 0.000000e+00, %"1133"
  store float %"1132", ptr addrspace(5) %"427", align 4
  %"2837" = getelementptr inbounds i8, ptr addrspace(5) %"2131", i64 16
  %"1134" = load float, ptr addrspace(5) %"2837", align 4
  store float %"1134", ptr addrspace(5) %"428", align 4
  %"1136" = load float, ptr addrspace(5) %"428", align 4
  %"1135" = fsub float 0.000000e+00, %"1136"
  store float %"1135", ptr addrspace(5) %"429", align 4
  %"2839" = getelementptr inbounds i8, ptr addrspace(5) %"2131", i64 20
  %"1137" = load float, ptr addrspace(5) %"2839", align 4
  store float %"1137", ptr addrspace(5) %"430", align 4
  %"1139" = load float, ptr addrspace(5) %"430", align 4
  %"1138" = fsub float 0.000000e+00, %"1139"
  store float %"1138", ptr addrspace(5) %"431", align 4
  %"1140" = load float, ptr addrspace(5) %"427", align 4
  %"2841" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 48
  store float %"1140", ptr addrspace(5) %"2841", align 4
  %"1141" = load float, ptr addrspace(5) %"429", align 4
  %"2843" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 52
  store float %"1141", ptr addrspace(5) %"2843", align 4
  %"1142" = load float, ptr addrspace(5) %"431", align 4
  %"2845" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 56
  store float %"1142", ptr addrspace(5) %"2845", align 4
  %"2847" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 60
  %"1143" = load float, ptr addrspace(5) %"2847", align 4
  store float %"1143", ptr addrspace(5) %"432", align 4
  %"2849" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 64
  %"1144" = load float, ptr addrspace(5) %"2849", align 4
  store float %"1144", ptr addrspace(5) %"433", align 4
  %"2851" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 68
  %"1145" = load float, ptr addrspace(5) %"2851", align 4
  store float %"1145", ptr addrspace(5) %"434", align 4
  %"2853" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 12
  %"1146" = load float, ptr addrspace(5) %"2853", align 4
  store float %"1146", ptr addrspace(5) %"435", align 4
  %"1148" = load float, ptr addrspace(5) %"335", align 4
  %"1149" = load float, ptr addrspace(5) %"432", align 4
  %"1150" = load float, ptr addrspace(5) %"435", align 4
  %"1147" = call float @llvm.fma.f32(float %"1148", float %"1149", float %"1150")
  store float %"1147", ptr addrspace(5) %"436", align 4
  %"1151" = load float, ptr addrspace(5) %"436", align 4
  %"2855" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 12
  store float %"1151", ptr addrspace(5) %"2855", align 4
  %"2857" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 16
  %"1152" = load float, ptr addrspace(5) %"2857", align 4
  store float %"1152", ptr addrspace(5) %"437", align 4
  %"1154" = load float, ptr addrspace(5) %"336", align 4
  %"1155" = load float, ptr addrspace(5) %"433", align 4
  %"1156" = load float, ptr addrspace(5) %"437", align 4
  %"1153" = call float @llvm.fma.f32(float %"1154", float %"1155", float %"1156")
  store float %"1153", ptr addrspace(5) %"438", align 4
  %"1157" = load float, ptr addrspace(5) %"438", align 4
  %"2859" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 16
  store float %"1157", ptr addrspace(5) %"2859", align 4
  %"2861" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 20
  %"1158" = load float, ptr addrspace(5) %"2861", align 4
  store float %"1158", ptr addrspace(5) %"439", align 4
  %"1160" = load float, ptr addrspace(5) %"337", align 4
  %"1161" = load float, ptr addrspace(5) %"434", align 4
  %"1162" = load float, ptr addrspace(5) %"439", align 4
  %"1159" = call float @llvm.fma.f32(float %"1160", float %"1161", float %"1162")
  store float %"1159", ptr addrspace(5) %"440", align 4
  %"1163" = load float, ptr addrspace(5) %"440", align 4
  %"2863" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 20
  store float %"1163", ptr addrspace(5) %"2863", align 4
  %"1165" = load i32, ptr addrspace(5) %"561", align 4
  %"1164" = icmp eq i32 %"1165", 1
  store i1 %"1164", ptr addrspace(5) %"317", align 1
  %"1167" = load i1, ptr addrspace(5) %"317", align 1
  %"2503" = select i1 %"1167", i16 1, i16 0
  store i16 %"2503", ptr addrspace(5) %"324", align 2
  %"1168" = load i16, ptr addrspace(5) %"324", align 2
  %"2865" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 10
  %"2505" = trunc i16 %"1168" to i8
  store i8 %"2505", ptr addrspace(5) %"2865", align 1
  %"1169" = load i1, ptr addrspace(5) %"317", align 1
  br i1 %"1169", label %"308", label %"737"

"737":                                            ; preds = %"306"
  %"2506" = load i32, ptr addrspace(5) %"2133", align 4
  store i32 %"2506", ptr addrspace(5) %"571", align 4
  %"2200" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 164)
  %"2508" = load i32, ptr addrspace(1) %"2200", align 4
  store i32 %"2508", ptr addrspace(5) %"572", align 4
  %"1173" = load i32, ptr addrspace(5) %"571", align 4
  %"1174" = load i32, ptr addrspace(5) %"572", align 4
  %"1172" = icmp sge i32 %"1173", %"1174"
  store i1 %"1172", ptr addrspace(5) %"318", align 1
  %19 = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %19, align 4
  %"1175" = load float, ptr addrspace(5) %19, align 4
  store float %"1175", ptr addrspace(5) %"557", align 4
  %20 = alloca float, align 4, addrspace(5)
  store float 0.000000e+00, ptr addrspace(5) %20, align 4
  %"1176" = load float, ptr addrspace(5) %20, align 4
  store float %"1176", ptr addrspace(5) %"558", align 4
  %"1177" = load i1, ptr addrspace(5) %"318", align 1
  br i1 %"1177", label %"308", label %"739"

"739":                                            ; preds = %"737"
  %"2513" = ptrtoint ptr addrspace(5) %"2133" to i64
  %21 = alloca i64, align 8, addrspace(5)
  store i64 %"2513", ptr addrspace(5) %21, align 8
  %"2512" = load i64, ptr addrspace(5) %21, align 8
  store i64 %"2512", ptr addrspace(5) %"635", align 8
  %"2868" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 4
  %"2515" = load i32, ptr addrspace(5) %"2868", align 4
  store i32 %"2515", ptr addrspace(5) %"578", align 4
  %"1181" = load i32, ptr addrspace(5) %"578", align 4
  %22 = mul i32 %"1181", 1664525
  %"2516" = add i32 %22, 1013904223
  store i32 %"2516", ptr addrspace(5) %"579", align 4
  %"1182" = load i32, ptr addrspace(5) %"579", align 4
  %"2870" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 4
  store i32 %"1182", ptr addrspace(5) %"2870", align 4
  %"1184" = load i32, ptr addrspace(5) %"579", align 4
  %"1183" = and i32 %"1184", 16777215
  store i32 %"1183", ptr addrspace(5) %"580", align 4
  %"1186" = load i32, ptr addrspace(5) %"580", align 4
  %"1185" = call float @__zluda_ptx_impl__cvt_rn_f32_u32(i32 %"1186")
  store float %"1185", ptr addrspace(5) %"444", align 4
  %23 = alloca float, align 4, addrspace(5)
  store float 0x4170000000000000, ptr addrspace(5) %23, align 4
  %"1187" = load float, ptr addrspace(5) %23, align 4
  store float %"1187", ptr addrspace(5) %"445", align 4
  %"1189" = load float, ptr addrspace(5) %"444", align 4
  %"1190" = load float, ptr addrspace(5) %"445", align 4
  %"1188" = fdiv arcp afn float %"1189", %"1190"
  store float %"1188", ptr addrspace(5) %"446", align 4
  %"2204" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 168)
  %"2521" = load i32, ptr addrspace(1) %"2204", align 4
  store i32 %"2521", ptr addrspace(5) %"581", align 4
  %"1193" = load i32, ptr addrspace(5) %"581", align 4
  %"1192" = call float @__zluda_ptx_impl__cvt_rn_f32_s32(i32 %"1193")
  store float %"1192", ptr addrspace(5) %"447", align 4
  %"1195" = load float, ptr addrspace(5) %"446", align 4
  %"1196" = load float, ptr addrspace(5) %"447", align 4
  %"1194" = fmul float %"1195", %"1196"
  store float %"1194", ptr addrspace(5) %"448", align 4
  %"1198" = load float, ptr addrspace(5) %"448", align 4
  %"1197" = call float @llvm.floor.f32(float %"1198")
  store float %"1197", ptr addrspace(5) %"449", align 4
  %"1200" = load float, ptr addrspace(5) %"449", align 4
  %"2524" = call i32 @__zluda_ptx_impl__cvt_rz_s32_f32(float %"1200")
  store i32 %"2524", ptr addrspace(5) %"582", align 4
  %"1202" = load i32, ptr addrspace(5) %"581", align 4
  %"2525" = add i32 %"1202", -1
  store i32 %"2525", ptr addrspace(5) %"583", align 4
  %"1204" = load i32, ptr addrspace(5) %"582", align 4
  %"1205" = load i32, ptr addrspace(5) %"583", align 4
  %"2527" = call i32 @llvm.smin.i32(i32 %"1204", i32 %"1205")
  store i32 %"2527", ptr addrspace(5) %"584", align 4
  %"1207" = load i32, ptr addrspace(5) %"584", align 4
  %"2530" = call i32 @llvm.smax.i32(i32 %"1207", i32 0)
  store i32 %"2530", ptr addrspace(5) %"585", align 4
  %"1209" = load i32, ptr addrspace(5) %"585", align 4
  %"2532" = sext i32 %"1209" to i64
  store i64 %"2532", ptr addrspace(5) %"623", align 8
  %"2206" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 56)
  %"2535" = ptrtoint ptr addrspace(1) %"2206" to i64
  %24 = alloca i64, align 8, addrspace(5)
  store i64 %"2535", ptr addrspace(5) %24, align 8
  %"2534" = load i64, ptr addrspace(5) %24, align 8
  store i64 %"2534", ptr addrspace(5) %"636", align 8
  %"1212" = load i64, ptr addrspace(5) %"636", align 8
  %25 = inttoptr i64 %"1212" to ptr addrspace(1)
  %26 = addrspacecast ptr addrspace(1) %25 to ptr
  %"1211" = ptrtoint ptr %26 to i64
  store i64 %"1211", ptr addrspace(5) %"622", align 8
  %"1214" = load i64, ptr addrspace(5) %"622", align 8
  %"1215" = load i32, ptr addrspace(5) %"564", align 4
  %"1216" = load i32, ptr addrspace(5) %"565", align 4
  %"1217" = load i64, ptr addrspace(5) %"623", align 8
  %"1218" = load i64, ptr addrspace(5) %"616", align 8
  %"1219" = load i64, ptr addrspace(5) %"616", align 8
  %"1220" = load i64, ptr addrspace(5) %"616", align 8
  %"1213" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"1214", i32 %"1215", i32 %"1216", i64 %"1217", i64 %"1218", i64 %"1219", i64 %"1220")
  store i64 %"1213", ptr addrspace(5) %"621", align 8
  %"1222" = load i64, ptr addrspace(5) %"621", align 8
  %"2536" = inttoptr i64 %"1222" to ptr
  %"1221" = load float, ptr %"2536", align 4
  store float %"1221", ptr addrspace(5) %"450", align 4
  %"1224" = load i64, ptr addrspace(5) %"621", align 8
  %"2537" = inttoptr i64 %"1224" to ptr
  %"2872" = getelementptr inbounds i8, ptr %"2537", i64 4
  %"1223" = load float, ptr %"2872", align 4
  store float %"1223", ptr addrspace(5) %"451", align 4
  %"1226" = load i64, ptr addrspace(5) %"621", align 8
  %"2538" = inttoptr i64 %"1226" to ptr
  %"2874" = getelementptr inbounds i8, ptr %"2538", i64 8
  %"1225" = load float, ptr %"2874", align 4
  store float %"1225", ptr addrspace(5) %"452", align 4
  %"1228" = load i64, ptr addrspace(5) %"621", align 8
  %"2539" = inttoptr i64 %"1228" to ptr
  %"2876" = getelementptr inbounds i8, ptr %"2539", i64 12
  %"1227" = load float, ptr %"2876", align 4
  store float %"1227", ptr addrspace(5) %"453", align 4
  %"1230" = load i64, ptr addrspace(5) %"621", align 8
  %"2540" = inttoptr i64 %"1230" to ptr
  %"2878" = getelementptr inbounds i8, ptr %"2540", i64 16
  %"1229" = load float, ptr %"2878", align 4
  store float %"1229", ptr addrspace(5) %"454", align 4
  %"1232" = load i64, ptr addrspace(5) %"621", align 8
  %"2541" = inttoptr i64 %"1232" to ptr
  %"2880" = getelementptr inbounds i8, ptr %"2541", i64 20
  %"1231" = load float, ptr %"2880", align 4
  store float %"1231", ptr addrspace(5) %"455", align 4
  %"1234" = load i64, ptr addrspace(5) %"621", align 8
  %"2542" = inttoptr i64 %"1234" to ptr
  %"2882" = getelementptr inbounds i8, ptr %"2542", i64 24
  %"1233" = load float, ptr %"2882", align 4
  store float %"1233", ptr addrspace(5) %"456", align 4
  %"1236" = load i64, ptr addrspace(5) %"621", align 8
  %"2543" = inttoptr i64 %"1236" to ptr
  %"2884" = getelementptr inbounds i8, ptr %"2543", i64 28
  %"1235" = load float, ptr %"2884", align 4
  store float %"1235", ptr addrspace(5) %"457", align 4
  %"1238" = load i64, ptr addrspace(5) %"621", align 8
  %"2544" = inttoptr i64 %"1238" to ptr
  %"2886" = getelementptr inbounds i8, ptr %"2544", i64 32
  %"1237" = load float, ptr %"2886", align 4
  store float %"1237", ptr addrspace(5) %"458", align 4
  %"1240" = load i64, ptr addrspace(5) %"621", align 8
  %"2545" = inttoptr i64 %"1240" to ptr
  %"2888" = getelementptr inbounds i8, ptr %"2545", i64 36
  %"1239" = load float, ptr %"2888", align 4
  store float %"1239", ptr addrspace(5) %"459", align 4
  %"1242" = load i64, ptr addrspace(5) %"621", align 8
  %"2546" = inttoptr i64 %"1242" to ptr
  %"2890" = getelementptr inbounds i8, ptr %"2546", i64 40
  %"1241" = load float, ptr %"2890", align 4
  store float %"1241", ptr addrspace(5) %"460", align 4
  %"1244" = load i64, ptr addrspace(5) %"621", align 8
  %"2547" = inttoptr i64 %"1244" to ptr
  %"2892" = getelementptr inbounds i8, ptr %"2547", i64 44
  %"1243" = load float, ptr %"2892", align 4
  store float %"1243", ptr addrspace(5) %"461", align 4
  %"1246" = load i64, ptr addrspace(5) %"621", align 8
  %"2548" = inttoptr i64 %"1246" to ptr
  %"2894" = getelementptr inbounds i8, ptr %"2548", i64 48
  %"1245" = load float, ptr %"2894", align 4
  store float %"1245", ptr addrspace(5) %"462", align 4
  %"1248" = load i64, ptr addrspace(5) %"621", align 8
  %"2549" = inttoptr i64 %"1248" to ptr
  %"2896" = getelementptr inbounds i8, ptr %"2549", i64 52
  %"1247" = load float, ptr %"2896", align 4
  store float %"1247", ptr addrspace(5) %"463", align 4
  %"1250" = load i64, ptr addrspace(5) %"621", align 8
  %"2550" = inttoptr i64 %"1250" to ptr
  %"2898" = getelementptr inbounds i8, ptr %"2550", i64 56
  %"1249" = load float, ptr %"2898", align 4
  store float %"1249", ptr addrspace(5) %"464", align 4
  %"1252" = load i64, ptr addrspace(5) %"621", align 8
  %"2551" = inttoptr i64 %"1252" to ptr
  %"2900" = getelementptr inbounds i8, ptr %"2551", i64 60
  %"1251" = load float, ptr %"2900", align 4
  store float %"1251", ptr addrspace(5) %"465", align 4
  %"1254" = load i64, ptr addrspace(5) %"621", align 8
  %"2552" = inttoptr i64 %"1254" to ptr
  %"2902" = getelementptr inbounds i8, ptr %"2552", i64 64
  %"1253" = load float, ptr %"2902", align 4
  store float %"1253", ptr addrspace(5) %"466", align 4
  %"1256" = load i64, ptr addrspace(5) %"621", align 8
  %"2553" = inttoptr i64 %"1256" to ptr
  %"2904" = getelementptr inbounds i8, ptr %"2553", i64 68
  %"2554" = load i32, ptr %"2904", align 4
  store i32 %"2554", ptr addrspace(5) %"586", align 4
  %"1258" = load i64, ptr addrspace(5) %"313", align 8
  %"2555" = add i64 %"1258", 0
  store i64 %"2555", ptr addrspace(5) %"637", align 8
  %"1260" = load i64, ptr addrspace(5) %"314", align 8
  %"2557" = add i64 %"1260", 0
  store i64 %"2557", ptr addrspace(5) %"609", align 8
  %"1261" = load i64, ptr addrspace(5) %"609", align 8
  %"1262" = load float, ptr addrspace(5) %"450", align 4
  %"2559" = inttoptr i64 %"1261" to ptr addrspace(5)
  store float %"1262", ptr addrspace(5) %"2559", align 4
  %"1263" = load i64, ptr addrspace(5) %"609", align 8
  %"1264" = load float, ptr addrspace(5) %"451", align 4
  %"2560" = inttoptr i64 %"1263" to ptr addrspace(5)
  %"2906" = getelementptr inbounds i8, ptr addrspace(5) %"2560", i64 4
  store float %"1264", ptr addrspace(5) %"2906", align 4
  %"1265" = load i64, ptr addrspace(5) %"609", align 8
  %"1266" = load float, ptr addrspace(5) %"452", align 4
  %"2561" = inttoptr i64 %"1265" to ptr addrspace(5)
  %"2908" = getelementptr inbounds i8, ptr addrspace(5) %"2561", i64 8
  store float %"1266", ptr addrspace(5) %"2908", align 4
  %"1267" = load i64, ptr addrspace(5) %"609", align 8
  %"1268" = load float, ptr addrspace(5) %"453", align 4
  %"2562" = inttoptr i64 %"1267" to ptr addrspace(5)
  %"2910" = getelementptr inbounds i8, ptr addrspace(5) %"2562", i64 12
  store float %"1268", ptr addrspace(5) %"2910", align 4
  %"1269" = load i64, ptr addrspace(5) %"609", align 8
  %"1270" = load float, ptr addrspace(5) %"454", align 4
  %"2563" = inttoptr i64 %"1269" to ptr addrspace(5)
  %"2912" = getelementptr inbounds i8, ptr addrspace(5) %"2563", i64 16
  store float %"1270", ptr addrspace(5) %"2912", align 4
  %"1271" = load i64, ptr addrspace(5) %"609", align 8
  %"1272" = load float, ptr addrspace(5) %"455", align 4
  %"2564" = inttoptr i64 %"1271" to ptr addrspace(5)
  %"2914" = getelementptr inbounds i8, ptr addrspace(5) %"2564", i64 20
  store float %"1272", ptr addrspace(5) %"2914", align 4
  %"1273" = load i64, ptr addrspace(5) %"609", align 8
  %"1274" = load float, ptr addrspace(5) %"456", align 4
  %"2565" = inttoptr i64 %"1273" to ptr addrspace(5)
  %"2916" = getelementptr inbounds i8, ptr addrspace(5) %"2565", i64 24
  store float %"1274", ptr addrspace(5) %"2916", align 4
  %"1275" = load i64, ptr addrspace(5) %"609", align 8
  %"1276" = load float, ptr addrspace(5) %"457", align 4
  %"2566" = inttoptr i64 %"1275" to ptr addrspace(5)
  %"2918" = getelementptr inbounds i8, ptr addrspace(5) %"2566", i64 28
  store float %"1276", ptr addrspace(5) %"2918", align 4
  %"1277" = load i64, ptr addrspace(5) %"609", align 8
  %"1278" = load float, ptr addrspace(5) %"458", align 4
  %"2567" = inttoptr i64 %"1277" to ptr addrspace(5)
  %"2920" = getelementptr inbounds i8, ptr addrspace(5) %"2567", i64 32
  store float %"1278", ptr addrspace(5) %"2920", align 4
  %"1279" = load i64, ptr addrspace(5) %"609", align 8
  %"1280" = load float, ptr addrspace(5) %"459", align 4
  %"2568" = inttoptr i64 %"1279" to ptr addrspace(5)
  %"2922" = getelementptr inbounds i8, ptr addrspace(5) %"2568", i64 36
  store float %"1280", ptr addrspace(5) %"2922", align 4
  %"1281" = load i64, ptr addrspace(5) %"609", align 8
  %"1282" = load float, ptr addrspace(5) %"460", align 4
  %"2569" = inttoptr i64 %"1281" to ptr addrspace(5)
  %"2924" = getelementptr inbounds i8, ptr addrspace(5) %"2569", i64 40
  store float %"1282", ptr addrspace(5) %"2924", align 4
  %"1283" = load i64, ptr addrspace(5) %"609", align 8
  %"1284" = load float, ptr addrspace(5) %"461", align 4
  %"2570" = inttoptr i64 %"1283" to ptr addrspace(5)
  %"2926" = getelementptr inbounds i8, ptr addrspace(5) %"2570", i64 44
  store float %"1284", ptr addrspace(5) %"2926", align 4
  %"1285" = load i64, ptr addrspace(5) %"609", align 8
  %"1286" = load float, ptr addrspace(5) %"462", align 4
  %"2571" = inttoptr i64 %"1285" to ptr addrspace(5)
  %"2928" = getelementptr inbounds i8, ptr addrspace(5) %"2571", i64 48
  store float %"1286", ptr addrspace(5) %"2928", align 4
  %"1287" = load i64, ptr addrspace(5) %"609", align 8
  %"1288" = load float, ptr addrspace(5) %"463", align 4
  %"2572" = inttoptr i64 %"1287" to ptr addrspace(5)
  %"2930" = getelementptr inbounds i8, ptr addrspace(5) %"2572", i64 52
  store float %"1288", ptr addrspace(5) %"2930", align 4
  %"1289" = load i64, ptr addrspace(5) %"609", align 8
  %"1290" = load float, ptr addrspace(5) %"464", align 4
  %"2573" = inttoptr i64 %"1289" to ptr addrspace(5)
  %"2932" = getelementptr inbounds i8, ptr addrspace(5) %"2573", i64 56
  store float %"1290", ptr addrspace(5) %"2932", align 4
  %"1291" = load i64, ptr addrspace(5) %"609", align 8
  %"1292" = load float, ptr addrspace(5) %"465", align 4
  %"2574" = inttoptr i64 %"1291" to ptr addrspace(5)
  %"2934" = getelementptr inbounds i8, ptr addrspace(5) %"2574", i64 60
  store float %"1292", ptr addrspace(5) %"2934", align 4
  %"1293" = load i64, ptr addrspace(5) %"609", align 8
  %"1294" = load float, ptr addrspace(5) %"466", align 4
  %"2575" = inttoptr i64 %"1293" to ptr addrspace(5)
  %"2936" = getelementptr inbounds i8, ptr addrspace(5) %"2575", i64 64
  store float %"1294", ptr addrspace(5) %"2936", align 4
  %"1295" = load i64, ptr addrspace(5) %"609", align 8
  %"1296" = load i32, ptr addrspace(5) %"586", align 4
  %"2576" = inttoptr i64 %"1295" to ptr addrspace(5)
  %"2938" = getelementptr inbounds i8, ptr addrspace(5) %"2576", i64 68
  store i32 %"1296", ptr addrspace(5) %"2938", align 4
  %"1298" = load i32, ptr addrspace(5) %"586", align 4
  %"2578" = sext i32 %"1298" to i64
  store i64 %"2578", ptr addrspace(5) %"629", align 8
  %"2208" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 8)
  %"2581" = ptrtoint ptr addrspace(1) %"2208" to i64
  %27 = alloca i64, align 8, addrspace(5)
  store i64 %"2581", ptr addrspace(5) %27, align 8
  %"2580" = load i64, ptr addrspace(5) %27, align 8
  store i64 %"2580", ptr addrspace(5) %"638", align 8
  %"1301" = load i64, ptr addrspace(5) %"638", align 8
  %28 = inttoptr i64 %"1301" to ptr addrspace(1)
  %29 = addrspacecast ptr addrspace(1) %28 to ptr
  %"1300" = ptrtoint ptr %29 to i64
  store i64 %"1300", ptr addrspace(5) %"628", align 8
  %30 = alloca i32, align 4, addrspace(5)
  store i32 4, ptr addrspace(5) %30, align 4
  %"2582" = load i32, ptr addrspace(5) %30, align 4
  store i32 %"2582", ptr addrspace(5) %"576", align 4
  %"1304" = load i64, ptr addrspace(5) %"628", align 8
  %"1305" = load i32, ptr addrspace(5) %"564", align 4
  %"1306" = load i32, ptr addrspace(5) %"576", align 4
  %"1307" = load i64, ptr addrspace(5) %"629", align 8
  %"1308" = load i64, ptr addrspace(5) %"616", align 8
  %"1309" = load i64, ptr addrspace(5) %"616", align 8
  %"1310" = load i64, ptr addrspace(5) %"616", align 8
  %"1303" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"1304", i32 %"1305", i32 %"1306", i64 %"1307", i64 %"1308", i64 %"1309", i64 %"1310")
  store i64 %"1303", ptr addrspace(5) %"627", align 8
  %"1312" = load i64, ptr addrspace(5) %"627", align 8
  %"2584" = inttoptr i64 %"1312" to ptr
  %"2583" = load i32, ptr %"2584", align 4
  store i32 %"2583", ptr addrspace(5) %"577", align 4
  %"1314" = load i32, ptr addrspace(5) %"577", align 4
  %"1315" = load i64, ptr addrspace(5) %"616", align 8
  %"1313" = call i64 @__zluda_rt_ptx_impl___rt_callable_program_from_id_v2_64(i32 %"1314", i64 %"1315", ptr addrspace(3) %"2127")
  store i64 %"1313", ptr addrspace(5) %"633", align 8
  %"1317" = load i64, ptr addrspace(5) %"635", align 8
  %31 = inttoptr i64 %"1317" to ptr addrspace(5)
  %32 = addrspacecast ptr addrspace(5) %31 to ptr
  %"1316" = ptrtoint ptr %32 to i64
  store i64 %"1316", ptr addrspace(5) %"640", align 8
  %"1319" = load i64, ptr addrspace(5) %"313", align 8
  %"2585" = add i64 %"1319", 72
  store i64 %"2585", ptr addrspace(5) %"641", align 8
  %"1320" = load i64, ptr addrspace(5) %"637", align 8
  %"2940" = getelementptr inbounds i8, ptr addrspace(5) %"2075", i64 0
  store i64 %"1320", ptr addrspace(5) %"2940", align 8
  %"1321" = load i64, ptr addrspace(5) %"640", align 8
  %"2942" = getelementptr inbounds i8, ptr addrspace(5) %"2077", i64 0
  store i64 %"1321", ptr addrspace(5) %"2942", align 8
  %"1322" = load i64, ptr addrspace(5) %"641", align 8
  %"2944" = getelementptr inbounds i8, ptr addrspace(5) %"2079", i64 0
  store i64 %"1322", ptr addrspace(5) %"2944", align 8
  %"834" = load i64, ptr addrspace(5) %"2075", align 8
  %"835" = load i64, ptr addrspace(5) %"2077", align 8
  %"836" = load i64, ptr addrspace(5) %"2079", align 8
  %"1323" = load i64, ptr addrspace(5) %"633", align 8
  %"2587" = inttoptr i64 %"1323" to ptr addrspace(1)
  %"2265" = load i64, ptr addrspace(1) %"2587", align 8
  %"2588" = inttoptr i64 %"1323" to ptr addrspace(1)
  %33 = inttoptr i64 %"2265" to ptr
  %"2264" = call i32 %33(i64 %"834", i64 %"835", i64 %"836", ptr addrspace(3) %"2127", <2 x i32> %"2129", ptr addrspace(1) %"2588")
  %"2267" = icmp uge i32 %"2264", 1024
  br i1 %"2267", label %"2268", label %"2269"

"2268":                                           ; preds = %"739"
  ret i32 %"2264"

"2269":                                           ; preds = %"739"
  %"1325" = load i64, ptr addrspace(5) %"314", align 8
  %"2589" = add i64 %"1325", 72
  store i64 %"2589", ptr addrspace(5) %"610", align 8
  %"1327" = load i64, ptr addrspace(5) %"610", align 8
  %"2591" = inttoptr i64 %"1327" to ptr addrspace(5)
  %"1326" = load float, ptr addrspace(5) %"2591", align 4
  store float %"1326", ptr addrspace(5) %"467", align 4
  %"1329" = load float, ptr addrspace(5) %"467", align 4
  %"1330" = load float, ptr addrspace(5) %"338", align 4
  %"1328" = fsub float %"1329", %"1330"
  store float %"1328", ptr addrspace(5) %"468", align 4
  %"1332" = load i64, ptr addrspace(5) %"610", align 8
  %"2592" = inttoptr i64 %"1332" to ptr addrspace(5)
  %"2946" = getelementptr inbounds i8, ptr addrspace(5) %"2592", i64 4
  %"1331" = load float, ptr addrspace(5) %"2946", align 4
  store float %"1331", ptr addrspace(5) %"469", align 4
  %"1334" = load float, ptr addrspace(5) %"469", align 4
  %"1335" = load float, ptr addrspace(5) %"339", align 4
  %"1333" = fsub float %"1334", %"1335"
  store float %"1333", ptr addrspace(5) %"470", align 4
  %"1337" = load i64, ptr addrspace(5) %"610", align 8
  %"2593" = inttoptr i64 %"1337" to ptr addrspace(5)
  %"2948" = getelementptr inbounds i8, ptr addrspace(5) %"2593", i64 8
  %"1336" = load float, ptr addrspace(5) %"2948", align 4
  store float %"1336", ptr addrspace(5) %"471", align 4
  %"1339" = load float, ptr addrspace(5) %"471", align 4
  %"1340" = load float, ptr addrspace(5) %"340", align 4
  %"1338" = fsub float %"1339", %"1340"
  store float %"1338", ptr addrspace(5) %"472", align 4
  %"1342" = load float, ptr addrspace(5) %"470", align 4
  %"1343" = load float, ptr addrspace(5) %"470", align 4
  %"1341" = fmul float %"1342", %"1343"
  store float %"1341", ptr addrspace(5) %"473", align 4
  %"1345" = load float, ptr addrspace(5) %"468", align 4
  %"1346" = load float, ptr addrspace(5) %"468", align 4
  %"1347" = load float, ptr addrspace(5) %"473", align 4
  %"1344" = call float @llvm.fma.f32(float %"1345", float %"1346", float %"1347")
  store float %"1344", ptr addrspace(5) %"474", align 4
  %"1349" = load float, ptr addrspace(5) %"472", align 4
  %"1350" = load float, ptr addrspace(5) %"472", align 4
  %"1351" = load float, ptr addrspace(5) %"474", align 4
  %"1348" = call float @llvm.fma.f32(float %"1349", float %"1350", float %"1351")
  store float %"1348", ptr addrspace(5) %"475", align 4
  %"1353" = load float, ptr addrspace(5) %"475", align 4
  %"1352" = call afn float @llvm.sqrt.f32(float %"1353")
  store float %"1352", ptr addrspace(5) %"341", align 4
  %"1355" = load float, ptr addrspace(5) %"341", align 4
  %"1356" = load float, ptr addrspace(5) %"341", align 4
  %"1354" = fmul float %"1355", %"1356"
  store float %"1354", ptr addrspace(5) %"342", align 4
  %"1358" = load float, ptr addrspace(5) %"342", align 4
  %34 = call afn float @llvm.sqrt.f32(float %"1358")
  %"1357" = fdiv arcp afn float 1.000000e+00, %34
  store float %"1357", ptr addrspace(5) %"476", align 4
  %"1360" = load float, ptr addrspace(5) %"468", align 4
  %"1361" = load float, ptr addrspace(5) %"476", align 4
  %"1359" = fmul float %"1360", %"1361"
  store float %"1359", ptr addrspace(5) %"343", align 4
  %"1363" = load float, ptr addrspace(5) %"470", align 4
  %"1364" = load float, ptr addrspace(5) %"476", align 4
  %"1362" = fmul float %"1363", %"1364"
  store float %"1362", ptr addrspace(5) %"344", align 4
  %"1366" = load float, ptr addrspace(5) %"472", align 4
  %"1367" = load float, ptr addrspace(5) %"476", align 4
  %"1365" = fmul float %"1366", %"1367"
  store float %"1365", ptr addrspace(5) %"345", align 4
  %"1369" = load float, ptr addrspace(5) %"333", align 4
  %"1370" = load float, ptr addrspace(5) %"344", align 4
  %"1368" = fmul float %"1369", %"1370"
  store float %"1368", ptr addrspace(5) %"477", align 4
  %"1372" = load float, ptr addrspace(5) %"332", align 4
  %"1373" = load float, ptr addrspace(5) %"343", align 4
  %"1374" = load float, ptr addrspace(5) %"477", align 4
  %"1371" = call float @llvm.fma.f32(float %"1372", float %"1373", float %"1374")
  store float %"1371", ptr addrspace(5) %"478", align 4
  %"1376" = load float, ptr addrspace(5) %"334", align 4
  %"1377" = load float, ptr addrspace(5) %"345", align 4
  %"1378" = load float, ptr addrspace(5) %"478", align 4
  %"1375" = call float @llvm.fma.f32(float %"1376", float %"1377", float %"1378")
  store float %"1375", ptr addrspace(5) %"479", align 4
  %"1380" = load float, ptr addrspace(5) %"479", align 4
  %"1379" = fcmp ole float %"1380", 0.000000e+00
  store i1 %"1379", ptr addrspace(5) %"319", align 1
  %"1381" = load i1, ptr addrspace(5) %"319", align 1
  br i1 %"1381", label %"307", label %"741"

"741":                                            ; preds = %"2269"
  %"1383" = load i64, ptr addrspace(5) %"610", align 8
  %"2594" = inttoptr i64 %"1383" to ptr addrspace(5)
  %"2950" = getelementptr inbounds i8, ptr addrspace(5) %"2594", i64 12
  %"1382" = load float, ptr addrspace(5) %"2950", align 4
  store float %"1382", ptr addrspace(5) %"483", align 4
  %"1385" = load i64, ptr addrspace(5) %"610", align 8
  %"2595" = inttoptr i64 %"1385" to ptr addrspace(5)
  %"2952" = getelementptr inbounds i8, ptr addrspace(5) %"2595", i64 16
  %"1384" = load float, ptr addrspace(5) %"2952", align 4
  store float %"1384", ptr addrspace(5) %"484", align 4
  %"1387" = load float, ptr addrspace(5) %"344", align 4
  %"1388" = load float, ptr addrspace(5) %"484", align 4
  %"1386" = fmul float %"1387", %"1388"
  store float %"1386", ptr addrspace(5) %"485", align 4
  %"1390" = load float, ptr addrspace(5) %"343", align 4
  %"1391" = load float, ptr addrspace(5) %"483", align 4
  %"1392" = load float, ptr addrspace(5) %"485", align 4
  %"1389" = call float @llvm.fma.f32(float %"1390", float %"1391", float %"1392")
  store float %"1389", ptr addrspace(5) %"486", align 4
  %"1394" = load i64, ptr addrspace(5) %"610", align 8
  %"2596" = inttoptr i64 %"1394" to ptr addrspace(5)
  %"2954" = getelementptr inbounds i8, ptr addrspace(5) %"2596", i64 20
  %"1393" = load float, ptr addrspace(5) %"2954", align 4
  store float %"1393", ptr addrspace(5) %"487", align 4
  %"1396" = load float, ptr addrspace(5) %"345", align 4
  %"1397" = load float, ptr addrspace(5) %"487", align 4
  %"1398" = load float, ptr addrspace(5) %"486", align 4
  %"1395" = call float @llvm.fma.f32(float %"1396", float %"1397", float %"1398")
  store float %"1395", ptr addrspace(5) %"488", align 4
  %"1400" = load float, ptr addrspace(5) %"488", align 4
  %"1399" = fcmp oge float %"1400", 0.000000e+00
  store i1 %"1399", ptr addrspace(5) %"320", align 1
  %"1401" = load i1, ptr addrspace(5) %"320", align 1
  br i1 %"1401", label %"307", label %"743"

"743":                                            ; preds = %"741"
  %"1403" = load i64, ptr addrspace(5) %"313", align 8
  %"2597" = add i64 %"1403", 112
  store i64 %"2597", ptr addrspace(5) %"642", align 8
  %"1405" = load i64, ptr addrspace(5) %"314", align 8
  %"2599" = add i64 %"1405", 112
  store i64 %"2599", ptr addrspace(5) %"643", align 8
  %35 = alloca i16, align 2, addrspace(5)
  store i16 0, ptr addrspace(5) %35, align 2
  %"2601" = load i16, ptr addrspace(5) %35, align 2
  store i16 %"2601", ptr addrspace(5) %"325", align 2
  %"1407" = load i64, ptr addrspace(5) %"643", align 8
  %"1408" = load i16, ptr addrspace(5) %"325", align 2
  %"2602" = inttoptr i64 %"1407" to ptr addrspace(5)
  %"2603" = trunc i16 %"1408" to i8
  store i8 %"2603", ptr addrspace(5) %"2602", align 1
  %"2210" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 172)
  %"1409" = load float, ptr addrspace(1) %"2210", align 4
  store float %"1409", ptr addrspace(5) %"495", align 4
  %"1411" = load float, ptr addrspace(5) %"341", align 4
  %"1412" = load float, ptr addrspace(5) %"495", align 4
  %"1410" = fsub float %"1411", %"1412"
  store float %"1410", ptr addrspace(5) %"496", align 4
  %"2212" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 156)
  %"2605" = load i32, ptr addrspace(1) %"2212", align 4
  store i32 %"2605", ptr addrspace(5) %"587", align 4
  %36 = alloca i32, align 4, addrspace(5)
  store i32 255, ptr addrspace(5) %36, align 4
  %"2607" = load i32, ptr addrspace(5) %36, align 4
  store i32 %"2607", ptr addrspace(5) %"589", align 4
  %37 = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %37, align 4
  %"2608" = load i32, ptr addrspace(5) %37, align 4
  store i32 %"2608", ptr addrspace(5) %"590", align 4
  %"1416" = load i32, ptr addrspace(5) %"587", align 4
  %"1417" = load float, ptr addrspace(5) %"338", align 4
  %"1418" = load float, ptr addrspace(5) %"339", align 4
  %"1419" = load float, ptr addrspace(5) %"340", align 4
  %"1420" = load float, ptr addrspace(5) %"343", align 4
  %"1421" = load float, ptr addrspace(5) %"344", align 4
  %"1422" = load float, ptr addrspace(5) %"345", align 4
  %"1423" = load i32, ptr addrspace(5) %"564", align 4
  %"1424" = load float, ptr addrspace(5) %"495", align 4
  %"1425" = load float, ptr addrspace(5) %"496", align 4
  %"1426" = load i32, ptr addrspace(5) %"589", align 4
  %"1427" = load i32, ptr addrspace(5) %"590", align 4
  %"1428" = load i64, ptr addrspace(5) %"642", align 8
  %"1429" = load i32, ptr addrspace(5) %"564", align 4
  %"2258" = call i32 @__zluda_rt_ptx_impl___rt_trace_mask_flags_64(i32 %"1416", float %"1417", float %"1418", float %"1419", float %"1420", float %"1421", float %"1422", i32 %"1423", float %"1424", float %"1425", i32 %"1426", i32 %"1427", i64 %"1428", i32 %"1429", ptr addrspace(3) %"2127")
  %"2260" = icmp uge i32 %"2258", 1024
  br i1 %"2260", label %"2261", label %"2262"

"2261":                                           ; preds = %"743"
  ret i32 %"2258"

"2262":                                           ; preds = %"743"
  %"1431" = load i64, ptr addrspace(5) %"643", align 8
  %"2610" = inttoptr i64 %"1431" to ptr addrspace(5)
  %"2609" = load i8, ptr addrspace(5) %"2610", align 1
  %"1430" = zext i8 %"2609" to i16
  store i16 %"1430", ptr addrspace(5) %"326", align 2
  %"1433" = load i16, ptr addrspace(5) %"326", align 2
  %"1432" = icmp ne i16 %"1433", 0
  store i1 %"1432", ptr addrspace(5) %"321", align 1
  %"1434" = load i1, ptr addrspace(5) %"321", align 1
  br i1 %"1434", label %"307", label %"745"

"745":                                            ; preds = %"2262"
  %"1436" = load i64, ptr addrspace(5) %"610", align 8
  %"2612" = inttoptr i64 %"1436" to ptr addrspace(5)
  %"2958" = getelementptr inbounds i8, ptr addrspace(5) %"2612", i64 12
  %"1435" = load float, ptr addrspace(5) %"2958", align 4
  store float %"1435", ptr addrspace(5) %"500", align 4
  %"1438" = load float, ptr addrspace(5) %"343", align 4
  %"1439" = load float, ptr addrspace(5) %"500", align 4
  %"1437" = fmul float %"1438", %"1439"
  store float %"1437", ptr addrspace(5) %"501", align 4
  %"1441" = load i64, ptr addrspace(5) %"610", align 8
  %"2613" = inttoptr i64 %"1441" to ptr addrspace(5)
  %"2960" = getelementptr inbounds i8, ptr addrspace(5) %"2613", i64 16
  %"1440" = load float, ptr addrspace(5) %"2960", align 4
  store float %"1440", ptr addrspace(5) %"502", align 4
  %"1443" = load float, ptr addrspace(5) %"344", align 4
  %"1444" = load float, ptr addrspace(5) %"502", align 4
  %"1442" = fmul float %"1443", %"1444"
  store float %"1442", ptr addrspace(5) %"503", align 4
  %"1446" = load float, ptr addrspace(5) %"503", align 4
  %"1445" = fsub float 0.000000e+00, %"1446"
  store float %"1445", ptr addrspace(5) %"504", align 4
  %"1448" = load float, ptr addrspace(5) %"504", align 4
  %"1449" = load float, ptr addrspace(5) %"501", align 4
  %"1447" = fsub float %"1448", %"1449"
  store float %"1447", ptr addrspace(5) %"505", align 4
  %"1451" = load i64, ptr addrspace(5) %"610", align 8
  %"2614" = inttoptr i64 %"1451" to ptr addrspace(5)
  %"2962" = getelementptr inbounds i8, ptr addrspace(5) %"2614", i64 20
  %"1450" = load float, ptr addrspace(5) %"2962", align 4
  store float %"1450", ptr addrspace(5) %"506", align 4
  %"1453" = load float, ptr addrspace(5) %"345", align 4
  %"1454" = load float, ptr addrspace(5) %"506", align 4
  %"1452" = fmul float %"1453", %"1454"
  store float %"1452", ptr addrspace(5) %"507", align 4
  %"1456" = load float, ptr addrspace(5) %"505", align 4
  %"1457" = load float, ptr addrspace(5) %"507", align 4
  %"1455" = fsub float %"1456", %"1457"
  store float %"1455", ptr addrspace(5) %"508", align 4
  %"1459" = load i64, ptr addrspace(5) %"609", align 8
  %"2615" = inttoptr i64 %"1459" to ptr addrspace(5)
  %"2964" = getelementptr inbounds i8, ptr addrspace(5) %"2615", i64 60
  %"1458" = load float, ptr addrspace(5) %"2964", align 4
  store float %"1458", ptr addrspace(5) %"509", align 4
  %"1461" = load float, ptr addrspace(5) %"509", align 4
  %"1462" = load float, ptr addrspace(5) %"508", align 4
  %"1460" = fmul float %"1461", %"1462"
  store float %"1460", ptr addrspace(5) %"510", align 4
  %"1464" = load float, ptr addrspace(5) %"342", align 4
  %"1465" = load float, ptr addrspace(5) %"510", align 4
  %"1463" = fdiv arcp afn float %"1464", %"1465"
  store float %"1463", ptr addrspace(5) %"511", align 4
  %"1466" = load float, ptr addrspace(5) %"343", align 4
  %"2966" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 36
  store float %"1466", ptr addrspace(5) %"2966", align 4
  %"1467" = load float, ptr addrspace(5) %"344", align 4
  %"2968" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 40
  store float %"1467", ptr addrspace(5) %"2968", align 4
  %"1468" = load float, ptr addrspace(5) %"345", align 4
  %"2970" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 44
  store float %"1468", ptr addrspace(5) %"2970", align 4
  %"2217" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 152)
  %"2619" = load i32, ptr addrspace(1) %"2217", align 4
  %"1469" = sext i32 %"2619" to i64
  store i64 %"1469", ptr addrspace(5) %"646", align 8
  %"2219" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 32)
  %"2622" = ptrtoint ptr addrspace(1) %"2219" to i64
  %38 = alloca i64, align 8, addrspace(5)
  store i64 %"2622", ptr addrspace(5) %38, align 8
  %"2621" = load i64, ptr addrspace(5) %38, align 8
  store i64 %"2621", ptr addrspace(5) %"661", align 8
  %"1472" = load i64, ptr addrspace(5) %"661", align 8
  %39 = inttoptr i64 %"1472" to ptr addrspace(1)
  %40 = addrspacecast ptr addrspace(1) %39 to ptr
  %"1471" = ptrtoint ptr %40 to i64
  store i64 %"1471", ptr addrspace(5) %"645", align 8
  %"1474" = load i64, ptr addrspace(5) %"645", align 8
  %"1475" = load i32, ptr addrspace(5) %"564", align 4
  %"1476" = load i32, ptr addrspace(5) %"576", align 4
  %"1477" = load i64, ptr addrspace(5) %"646", align 8
  %"1478" = load i64, ptr addrspace(5) %"616", align 8
  %"1479" = load i64, ptr addrspace(5) %"616", align 8
  %"1480" = load i64, ptr addrspace(5) %"616", align 8
  %"1473" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"1474", i32 %"1475", i32 %"1476", i64 %"1477", i64 %"1478", i64 %"1479", i64 %"1480")
  store i64 %"1473", ptr addrspace(5) %"644", align 8
  %"1482" = load i64, ptr addrspace(5) %"644", align 8
  %"2624" = inttoptr i64 %"1482" to ptr
  %"2623" = load i32, ptr %"2624", align 4
  store i32 %"2623", ptr addrspace(5) %"594", align 4
  %"1484" = load i32, ptr addrspace(5) %"594", align 4
  %"1485" = load i64, ptr addrspace(5) %"616", align 8
  %"1483" = call i64 @__zluda_rt_ptx_impl___rt_callable_program_from_id_v2_64(i32 %"1484", i64 %"1485", ptr addrspace(3) %"2127")
  store i64 %"1483", ptr addrspace(5) %"650", align 8
  %"1486" = load i64, ptr addrspace(5) %"617", align 8
  %"2972" = getelementptr inbounds i8, ptr addrspace(5) %"2081", i64 0
  store i64 %"1486", ptr addrspace(5) %"2972", align 8
  %"1487" = load i64, ptr addrspace(5) %"619", align 8
  %"2974" = getelementptr inbounds i8, ptr addrspace(5) %"2083", i64 0
  store i64 %"1487", ptr addrspace(5) %"2974", align 8
  %"1488" = load i64, ptr addrspace(5) %"640", align 8
  %"2976" = getelementptr inbounds i8, ptr addrspace(5) %"2085", i64 0
  store i64 %"1488", ptr addrspace(5) %"2976", align 8
  %"837" = load i64, ptr addrspace(5) %"2081", align 8
  %"838" = load i64, ptr addrspace(5) %"2083", align 8
  %"839" = load i64, ptr addrspace(5) %"2085", align 8
  %"1489" = load i64, ptr addrspace(5) %"650", align 8
  %"2625" = inttoptr i64 %"1489" to ptr addrspace(1)
  %"2271" = load i64, ptr addrspace(1) %"2625", align 8
  %"2626" = inttoptr i64 %"1489" to ptr addrspace(1)
  %41 = inttoptr i64 %"2271" to ptr
  %"2270" = call i32 %41(i64 %"837", i64 %"838", i64 %"839", ptr addrspace(3) %"2127", <2 x i32> %"2129", ptr addrspace(1) %"2626")
  %"2273" = icmp uge i32 %"2270", 1024
  br i1 %"2273", label %"2274", label %"2275"

"2274":                                           ; preds = %"745"
  ret i32 %"2270"

"2275":                                           ; preds = %"745"
  %"2221" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 152)
  %"2627" = load i32, ptr addrspace(1) %"2221", align 4
  %"1490" = sext i32 %"2627" to i64
  store i64 %"1490", ptr addrspace(5) %"654", align 8
  %"2223" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 80)
  %"2630" = ptrtoint ptr addrspace(1) %"2223" to i64
  %42 = alloca i64, align 8, addrspace(5)
  store i64 %"2630", ptr addrspace(5) %42, align 8
  %"2629" = load i64, ptr addrspace(5) %42, align 8
  store i64 %"2629", ptr addrspace(5) %"666", align 8
  %"1493" = load i64, ptr addrspace(5) %"666", align 8
  %43 = inttoptr i64 %"1493" to ptr addrspace(1)
  %44 = addrspacecast ptr addrspace(1) %43 to ptr
  %"1492" = ptrtoint ptr %44 to i64
  store i64 %"1492", ptr addrspace(5) %"653", align 8
  %"1495" = load i64, ptr addrspace(5) %"653", align 8
  %"1496" = load i32, ptr addrspace(5) %"564", align 4
  %"1497" = load i32, ptr addrspace(5) %"576", align 4
  %"1498" = load i64, ptr addrspace(5) %"654", align 8
  %"1499" = load i64, ptr addrspace(5) %"616", align 8
  %"1500" = load i64, ptr addrspace(5) %"616", align 8
  %"1501" = load i64, ptr addrspace(5) %"616", align 8
  %"1494" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"1495", i32 %"1496", i32 %"1497", i64 %"1498", i64 %"1499", i64 %"1500", i64 %"1501")
  store i64 %"1494", ptr addrspace(5) %"652", align 8
  %"1503" = load i64, ptr addrspace(5) %"652", align 8
  %"2632" = inttoptr i64 %"1503" to ptr
  %"2631" = load i32, ptr %"2632", align 4
  store i32 %"2631", ptr addrspace(5) %"597", align 4
  %"1505" = load i32, ptr addrspace(5) %"597", align 4
  %"1506" = load i64, ptr addrspace(5) %"616", align 8
  %"1504" = call i64 @__zluda_rt_ptx_impl___rt_callable_program_from_id_v2_64(i32 %"1505", i64 %"1506", ptr addrspace(3) %"2127")
  store i64 %"1504", ptr addrspace(5) %"658", align 8
  %"1507" = load i64, ptr addrspace(5) %"617", align 8
  %"2978" = getelementptr inbounds i8, ptr addrspace(5) %"2087", i64 0
  store i64 %"1507", ptr addrspace(5) %"2978", align 8
  %"1508" = load i64, ptr addrspace(5) %"619", align 8
  %"2980" = getelementptr inbounds i8, ptr addrspace(5) %"2089", i64 0
  store i64 %"1508", ptr addrspace(5) %"2980", align 8
  %"1509" = load i64, ptr addrspace(5) %"640", align 8
  %"2982" = getelementptr inbounds i8, ptr addrspace(5) %"2091", i64 0
  store i64 %"1509", ptr addrspace(5) %"2982", align 8
  %"840" = load i64, ptr addrspace(5) %"2087", align 8
  %"841" = load i64, ptr addrspace(5) %"2089", align 8
  %"842" = load i64, ptr addrspace(5) %"2091", align 8
  %"1510" = load i64, ptr addrspace(5) %"658", align 8
  %"2633" = inttoptr i64 %"1510" to ptr addrspace(1)
  %"2277" = load i64, ptr addrspace(1) %"2633", align 8
  %"2634" = inttoptr i64 %"1510" to ptr addrspace(1)
  %45 = inttoptr i64 %"2277" to ptr
  %46 = call { [3 x i32], i32 } %45(i64 %"840", i64 %"841", i64 %"842", ptr addrspace(3) %"2127", <2 x i32> %"2129", ptr addrspace(1) %"2634")
  %"843" = extractvalue { [3 x i32], i32 } %46, 0
  %"2276" = extractvalue { [3 x i32], i32 } %46, 1
  %"2279" = icmp uge i32 %"2276", 1024
  br i1 %"2279", label %"2280", label %"2281"

"2280":                                           ; preds = %"2275"
  ret i32 %"2276"

"2281":                                           ; preds = %"2275"
  store [3 x i32] %"843", ptr addrspace(5) %"2093", align 4
  %"2984" = getelementptr inbounds i8, ptr addrspace(5) %"2093", i64 0
  %"1511" = load float, ptr addrspace(5) %"2984", align 4
  store float %"1511", ptr addrspace(5) %"512", align 4
  %"2986" = getelementptr inbounds i8, ptr addrspace(5) %"2093", i64 4
  %"1512" = load float, ptr addrspace(5) %"2986", align 4
  store float %"1512", ptr addrspace(5) %"513", align 4
  %"2988" = getelementptr inbounds i8, ptr addrspace(5) %"2093", i64 8
  %"1513" = load float, ptr addrspace(5) %"2988", align 4
  store float %"1513", ptr addrspace(5) %"514", align 4
  %"1515" = load float, ptr addrspace(5) %"511", align 4
  %"1516" = load float, ptr addrspace(5) %"511", align 4
  %"1514" = fmul float %"1515", %"1516"
  store float %"1514", ptr addrspace(5) %"515", align 4
  %"2990" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 72
  %"1517" = load float, ptr addrspace(5) %"2990", align 4
  store float %"1517", ptr addrspace(5) %"516", align 4
  %"1519" = load float, ptr addrspace(5) %"516", align 4
  %"1520" = load float, ptr addrspace(5) %"516", align 4
  %"1521" = load float, ptr addrspace(5) %"515", align 4
  %"1518" = call float @llvm.fma.f32(float %"1519", float %"1520", float %"1521")
  store float %"1518", ptr addrspace(5) %"517", align 4
  %"1523" = load float, ptr addrspace(5) %"515", align 4
  %"1524" = load float, ptr addrspace(5) %"517", align 4
  %"1522" = fdiv arcp afn float %"1523", %"1524"
  store float %"1522", ptr addrspace(5) %"518", align 4
  %"2992" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 60
  %"1525" = load float, ptr addrspace(5) %"2992", align 4
  store float %"1525", ptr addrspace(5) %"519", align 4
  %"1527" = load float, ptr addrspace(5) %"518", align 4
  %"1528" = load float, ptr addrspace(5) %"519", align 4
  %"1526" = fmul float %"1527", %"1528"
  store float %"1526", ptr addrspace(5) %"520", align 4
  %"2994" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 64
  %"1529" = load float, ptr addrspace(5) %"2994", align 4
  store float %"1529", ptr addrspace(5) %"521", align 4
  %"1531" = load float, ptr addrspace(5) %"518", align 4
  %"1532" = load float, ptr addrspace(5) %"521", align 4
  %"1530" = fmul float %"1531", %"1532"
  store float %"1530", ptr addrspace(5) %"522", align 4
  %"2996" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 68
  %"1533" = load float, ptr addrspace(5) %"2996", align 4
  store float %"1533", ptr addrspace(5) %"523", align 4
  %"1535" = load float, ptr addrspace(5) %"518", align 4
  %"1536" = load float, ptr addrspace(5) %"523", align 4
  %"1534" = fmul float %"1535", %"1536"
  store float %"1534", ptr addrspace(5) %"524", align 4
  %"1538" = load float, ptr addrspace(5) %"512", align 4
  %"1539" = load float, ptr addrspace(5) %"520", align 4
  %"1537" = fmul float %"1538", %"1539"
  store float %"1537", ptr addrspace(5) %"525", align 4
  %"1541" = load float, ptr addrspace(5) %"513", align 4
  %"1542" = load float, ptr addrspace(5) %"522", align 4
  %"1540" = fmul float %"1541", %"1542"
  store float %"1540", ptr addrspace(5) %"526", align 4
  %"1544" = load float, ptr addrspace(5) %"514", align 4
  %"1545" = load float, ptr addrspace(5) %"524", align 4
  %"1543" = fmul float %"1544", %"1545"
  store float %"1543", ptr addrspace(5) %"527", align 4
  %"1547" = load i64, ptr addrspace(5) %"610", align 8
  %"2642" = inttoptr i64 %"1547" to ptr addrspace(5)
  %"2998" = getelementptr inbounds i8, ptr addrspace(5) %"2642", i64 24
  %"1546" = load float, ptr addrspace(5) %"2998", align 4
  store float %"1546", ptr addrspace(5) %"528", align 4
  %"1549" = load float, ptr addrspace(5) %"528", align 4
  %"1550" = load float, ptr addrspace(5) %"525", align 4
  %"1548" = fmul float %"1549", %"1550"
  store float %"1548", ptr addrspace(5) %"529", align 4
  %"1552" = load i64, ptr addrspace(5) %"610", align 8
  %"2643" = inttoptr i64 %"1552" to ptr addrspace(5)
  %"3000" = getelementptr inbounds i8, ptr addrspace(5) %"2643", i64 28
  %"1551" = load float, ptr addrspace(5) %"3000", align 4
  store float %"1551", ptr addrspace(5) %"530", align 4
  %"1554" = load float, ptr addrspace(5) %"530", align 4
  %"1555" = load float, ptr addrspace(5) %"526", align 4
  %"1553" = fmul float %"1554", %"1555"
  store float %"1553", ptr addrspace(5) %"531", align 4
  %"1557" = load i64, ptr addrspace(5) %"610", align 8
  %"2644" = inttoptr i64 %"1557" to ptr addrspace(5)
  %"3002" = getelementptr inbounds i8, ptr addrspace(5) %"2644", i64 32
  %"1556" = load float, ptr addrspace(5) %"3002", align 4
  store float %"1556", ptr addrspace(5) %"532", align 4
  %"1559" = load float, ptr addrspace(5) %"532", align 4
  %"1560" = load float, ptr addrspace(5) %"527", align 4
  %"1558" = fmul float %"1559", %"1560"
  store float %"1558", ptr addrspace(5) %"533", align 4
  %47 = alloca float, align 4, addrspace(5)
  store float 0x3F50624DE0000000, ptr addrspace(5) %47, align 4
  %"1561" = load float, ptr addrspace(5) %47, align 4
  store float %"1561", ptr addrspace(5) %"534", align 4
  %"1563" = load float, ptr addrspace(5) %"534", align 4
  %"1564" = load float, ptr addrspace(5) %"511", align 4
  %"1562" = call float @llvm.maxnum.f32(float %"1563", float %"1564")
  store float %"1562", ptr addrspace(5) %"535", align 4
  %"1566" = load float, ptr addrspace(5) %"535", align 4
  %"1565" = fdiv arcp afn float 1.000000e+00, %"1566"
  store float %"1565", ptr addrspace(5) %"536", align 4
  %"1568" = load float, ptr addrspace(5) %"536", align 4
  %"1569" = load float, ptr addrspace(5) %"533", align 4
  %"1567" = fmul float %"1568", %"1569"
  store float %"1567", ptr addrspace(5) %"558", align 4
  %"1571" = load float, ptr addrspace(5) %"536", align 4
  %"1572" = load float, ptr addrspace(5) %"531", align 4
  %"1570" = fmul float %"1571", %"1572"
  store float %"1570", ptr addrspace(5) %"557", align 4
  %"1574" = load float, ptr addrspace(5) %"536", align 4
  %"1575" = load float, ptr addrspace(5) %"529", align 4
  %"1573" = fmul float %"1574", %"1575"
  store float %"1573", ptr addrspace(5) %"556", align 4
  br label %"307"

"307":                                            ; preds = %"2281", %"2262", %"741", %"2269"
  %"3004" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 12
  %"1576" = load float, ptr addrspace(5) %"3004", align 4
  store float %"1576", ptr addrspace(5) %"537", align 4
  %"1578" = load float, ptr addrspace(5) %"556", align 4
  %"1579" = load float, ptr addrspace(5) %"537", align 4
  %"1577" = fadd float %"1578", %"1579"
  store float %"1577", ptr addrspace(5) %"538", align 4
  %"1580" = load float, ptr addrspace(5) %"538", align 4
  %"3006" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 12
  store float %"1580", ptr addrspace(5) %"3006", align 4
  %"3008" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 16
  %"1581" = load float, ptr addrspace(5) %"3008", align 4
  store float %"1581", ptr addrspace(5) %"539", align 4
  %"1583" = load float, ptr addrspace(5) %"557", align 4
  %"1584" = load float, ptr addrspace(5) %"539", align 4
  %"1582" = fadd float %"1583", %"1584"
  store float %"1582", ptr addrspace(5) %"540", align 4
  %"1585" = load float, ptr addrspace(5) %"540", align 4
  %"3010" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 16
  store float %"1585", ptr addrspace(5) %"3010", align 4
  %"3012" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 20
  %"1586" = load float, ptr addrspace(5) %"3012", align 4
  store float %"1586", ptr addrspace(5) %"541", align 4
  %"1588" = load float, ptr addrspace(5) %"558", align 4
  %"1589" = load float, ptr addrspace(5) %"541", align 4
  %"1587" = fadd float %"1588", %"1589"
  store float %"1587", ptr addrspace(5) %"542", align 4
  %"1590" = load float, ptr addrspace(5) %"542", align 4
  %"3014" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 20
  store float %"1590", ptr addrspace(5) %"3014", align 4
  br label %"308"

"308":                                            ; preds = %"307", %"737", %"306"
  %"2235" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 152)
  %"2651" = load i32, ptr addrspace(1) %"2235", align 4
  %"1591" = sext i32 %"2651" to i64
  store i64 %"1591", ptr addrspace(5) %"670", align 8
  %"2237" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 128)
  %"2654" = ptrtoint ptr addrspace(1) %"2237" to i64
  %48 = alloca i64, align 8, addrspace(5)
  store i64 %"2654", ptr addrspace(5) %48, align 8
  %"2653" = load i64, ptr addrspace(5) %48, align 8
  store i64 %"2653", ptr addrspace(5) %"692", align 8
  %"1594" = load i64, ptr addrspace(5) %"692", align 8
  %49 = inttoptr i64 %"1594" to ptr addrspace(1)
  %50 = addrspacecast ptr addrspace(1) %49 to ptr
  %"1593" = ptrtoint ptr %50 to i64
  store i64 %"1593", ptr addrspace(5) %"669", align 8
  %51 = alloca i32, align 4, addrspace(5)
  store i32 4, ptr addrspace(5) %51, align 4
  %"2655" = load i32, ptr addrspace(5) %51, align 4
  store i32 %"2655", ptr addrspace(5) %"605", align 4
  %"1597" = load i64, ptr addrspace(5) %"669", align 8
  %"1598" = load i32, ptr addrspace(5) %"564", align 4
  %"1599" = load i32, ptr addrspace(5) %"605", align 4
  %"1600" = load i64, ptr addrspace(5) %"670", align 8
  %"1601" = load i64, ptr addrspace(5) %"616", align 8
  %"1602" = load i64, ptr addrspace(5) %"616", align 8
  %"1603" = load i64, ptr addrspace(5) %"616", align 8
  %"1596" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"1597", i32 %"1598", i32 %"1599", i64 %"1600", i64 %"1601", i64 %"1602", i64 %"1603")
  store i64 %"1596", ptr addrspace(5) %"668", align 8
  %"1605" = load i64, ptr addrspace(5) %"668", align 8
  %"2657" = inttoptr i64 %"1605" to ptr
  %"2656" = load i32, ptr %"2657", align 4
  store i32 %"2656", ptr addrspace(5) %"600", align 4
  %"1607" = load i32, ptr addrspace(5) %"600", align 4
  %"1608" = load i64, ptr addrspace(5) %"616", align 8
  %"1606" = call i64 @__zluda_rt_ptx_impl___rt_callable_program_from_id_v2_64(i32 %"1607", i64 %"1608", ptr addrspace(3) %"2127")
  store i64 %"1606", ptr addrspace(5) %"674", align 8
  %"2659" = ptrtoint ptr addrspace(5) %"2133" to i64
  %52 = alloca i64, align 8, addrspace(5)
  store i64 %"2659", ptr addrspace(5) %52, align 8
  %"2658" = load i64, ptr addrspace(5) %52, align 8
  store i64 %"2658", ptr addrspace(5) %"694", align 8
  %"1611" = load i64, ptr addrspace(5) %"694", align 8
  %53 = inttoptr i64 %"1611" to ptr addrspace(5)
  %54 = addrspacecast ptr addrspace(5) %53 to ptr
  %"1610" = ptrtoint ptr %54 to i64
  store i64 %"1610", ptr addrspace(5) %"695", align 8
  %"1612" = load i64, ptr addrspace(5) %"617", align 8
  %"3016" = getelementptr inbounds i8, ptr addrspace(5) %"2097", i64 0
  store i64 %"1612", ptr addrspace(5) %"3016", align 8
  %"1613" = load i64, ptr addrspace(5) %"619", align 8
  %"3018" = getelementptr inbounds i8, ptr addrspace(5) %"2099", i64 0
  store i64 %"1613", ptr addrspace(5) %"3018", align 8
  %"1614" = load i64, ptr addrspace(5) %"695", align 8
  %"3020" = getelementptr inbounds i8, ptr addrspace(5) %"2101", i64 0
  store i64 %"1614", ptr addrspace(5) %"3020", align 8
  %"844" = load i64, ptr addrspace(5) %"2097", align 8
  %"845" = load i64, ptr addrspace(5) %"2099", align 8
  %"846" = load i64, ptr addrspace(5) %"2101", align 8
  %"1615" = load i64, ptr addrspace(5) %"674", align 8
  %"2660" = inttoptr i64 %"1615" to ptr addrspace(1)
  %"2283" = load i64, ptr addrspace(1) %"2660", align 8
  %"2661" = inttoptr i64 %"1615" to ptr addrspace(1)
  %55 = inttoptr i64 %"2283" to ptr
  %"2282" = call i32 %55(i64 %"844", i64 %"845", i64 %"846", ptr addrspace(3) %"2127", <2 x i32> %"2129", ptr addrspace(1) %"2661")
  %"2285" = icmp uge i32 %"2282", 1024
  br i1 %"2285", label %"2286", label %"2287"

"2286":                                           ; preds = %"308"
  ret i32 %"2282"

"2287":                                           ; preds = %"308"
  %"2239" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 152)
  %"2662" = load i32, ptr addrspace(1) %"2239", align 4
  %"1616" = sext i32 %"2662" to i64
  store i64 %"1616", ptr addrspace(5) %"678", align 8
  %"2241" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 32)
  %"2665" = ptrtoint ptr addrspace(1) %"2241" to i64
  %56 = alloca i64, align 8, addrspace(5)
  store i64 %"2665", ptr addrspace(5) %56, align 8
  %"2664" = load i64, ptr addrspace(5) %56, align 8
  store i64 %"2664", ptr addrspace(5) %"698", align 8
  %"1619" = load i64, ptr addrspace(5) %"698", align 8
  %57 = inttoptr i64 %"1619" to ptr addrspace(1)
  %58 = addrspacecast ptr addrspace(1) %57 to ptr
  %"1618" = ptrtoint ptr %58 to i64
  store i64 %"1618", ptr addrspace(5) %"677", align 8
  %"1621" = load i64, ptr addrspace(5) %"677", align 8
  %"1622" = load i32, ptr addrspace(5) %"564", align 4
  %"1623" = load i32, ptr addrspace(5) %"605", align 4
  %"1624" = load i64, ptr addrspace(5) %"678", align 8
  %"1625" = load i64, ptr addrspace(5) %"616", align 8
  %"1626" = load i64, ptr addrspace(5) %"616", align 8
  %"1627" = load i64, ptr addrspace(5) %"616", align 8
  %"1620" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"1621", i32 %"1622", i32 %"1623", i64 %"1624", i64 %"1625", i64 %"1626", i64 %"1627")
  store i64 %"1620", ptr addrspace(5) %"676", align 8
  %"1629" = load i64, ptr addrspace(5) %"676", align 8
  %"2667" = inttoptr i64 %"1629" to ptr
  %"2666" = load i32, ptr %"2667", align 4
  store i32 %"2666", ptr addrspace(5) %"603", align 4
  %"1631" = load i32, ptr addrspace(5) %"603", align 4
  %"1632" = load i64, ptr addrspace(5) %"616", align 8
  %"1630" = call i64 @__zluda_rt_ptx_impl___rt_callable_program_from_id_v2_64(i32 %"1631", i64 %"1632", ptr addrspace(3) %"2127")
  store i64 %"1630", ptr addrspace(5) %"682", align 8
  %"1633" = load i64, ptr addrspace(5) %"617", align 8
  %"3022" = getelementptr inbounds i8, ptr addrspace(5) %"2103", i64 0
  store i64 %"1633", ptr addrspace(5) %"3022", align 8
  %"1634" = load i64, ptr addrspace(5) %"619", align 8
  %"3024" = getelementptr inbounds i8, ptr addrspace(5) %"2105", i64 0
  store i64 %"1634", ptr addrspace(5) %"3024", align 8
  %"1635" = load i64, ptr addrspace(5) %"695", align 8
  %"3026" = getelementptr inbounds i8, ptr addrspace(5) %"2107", i64 0
  store i64 %"1635", ptr addrspace(5) %"3026", align 8
  %"847" = load i64, ptr addrspace(5) %"2103", align 8
  %"848" = load i64, ptr addrspace(5) %"2105", align 8
  %"849" = load i64, ptr addrspace(5) %"2107", align 8
  %"1636" = load i64, ptr addrspace(5) %"682", align 8
  %"2668" = inttoptr i64 %"1636" to ptr addrspace(1)
  %"2289" = load i64, ptr addrspace(1) %"2668", align 8
  %"2669" = inttoptr i64 %"1636" to ptr addrspace(1)
  %59 = inttoptr i64 %"2289" to ptr
  %"2288" = call i32 %59(i64 %"847", i64 %"848", i64 %"849", ptr addrspace(3) %"2127", <2 x i32> %"2129", ptr addrspace(1) %"2669")
  %"2291" = icmp uge i32 %"2288", 1024
  br i1 %"2291", label %"2292", label %"2293"

"2292":                                           ; preds = %"2287"
  ret i32 %"2288"

"2293":                                           ; preds = %"2287"
  %"2243" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 152)
  %"2670" = load i32, ptr addrspace(1) %"2243", align 4
  %"1637" = sext i32 %"2670" to i64
  store i64 %"1637", ptr addrspace(5) %"686", align 8
  %"2245" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2137", i32 80)
  %"2673" = ptrtoint ptr addrspace(1) %"2245" to i64
  %60 = alloca i64, align 8, addrspace(5)
  store i64 %"2673", ptr addrspace(5) %60, align 8
  %"2672" = load i64, ptr addrspace(5) %60, align 8
  store i64 %"2672", ptr addrspace(5) %"700", align 8
  %"1640" = load i64, ptr addrspace(5) %"700", align 8
  %61 = inttoptr i64 %"1640" to ptr addrspace(1)
  %62 = addrspacecast ptr addrspace(1) %61 to ptr
  %"1639" = ptrtoint ptr %62 to i64
  store i64 %"1639", ptr addrspace(5) %"685", align 8
  %"1642" = load i64, ptr addrspace(5) %"685", align 8
  %"1643" = load i32, ptr addrspace(5) %"564", align 4
  %"1644" = load i32, ptr addrspace(5) %"605", align 4
  %"1645" = load i64, ptr addrspace(5) %"686", align 8
  %"1646" = load i64, ptr addrspace(5) %"616", align 8
  %"1647" = load i64, ptr addrspace(5) %"616", align 8
  %"1648" = load i64, ptr addrspace(5) %"616", align 8
  %"1641" = call i64 @__zluda_rt_ptx_impl___rt_buffer_get_64(i64 %"1642", i32 %"1643", i32 %"1644", i64 %"1645", i64 %"1646", i64 %"1647", i64 %"1648")
  store i64 %"1641", ptr addrspace(5) %"684", align 8
  %"1650" = load i64, ptr addrspace(5) %"684", align 8
  %"2675" = inttoptr i64 %"1650" to ptr
  %"2674" = load i32, ptr %"2675", align 4
  store i32 %"2674", ptr addrspace(5) %"606", align 4
  %"1652" = load i32, ptr addrspace(5) %"606", align 4
  %"1653" = load i64, ptr addrspace(5) %"616", align 8
  %"1651" = call i64 @__zluda_rt_ptx_impl___rt_callable_program_from_id_v2_64(i32 %"1652", i64 %"1653", ptr addrspace(3) %"2127")
  store i64 %"1651", ptr addrspace(5) %"690", align 8
  %"1654" = load i64, ptr addrspace(5) %"617", align 8
  %"3028" = getelementptr inbounds i8, ptr addrspace(5) %"2109", i64 0
  store i64 %"1654", ptr addrspace(5) %"3028", align 8
  %"1655" = load i64, ptr addrspace(5) %"619", align 8
  %"3030" = getelementptr inbounds i8, ptr addrspace(5) %"2111", i64 0
  store i64 %"1655", ptr addrspace(5) %"3030", align 8
  %"1656" = load i64, ptr addrspace(5) %"695", align 8
  %"3032" = getelementptr inbounds i8, ptr addrspace(5) %"2113", i64 0
  store i64 %"1656", ptr addrspace(5) %"3032", align 8
  %"850" = load i64, ptr addrspace(5) %"2109", align 8
  %"851" = load i64, ptr addrspace(5) %"2111", align 8
  %"852" = load i64, ptr addrspace(5) %"2113", align 8
  %"1657" = load i64, ptr addrspace(5) %"690", align 8
  %"2676" = inttoptr i64 %"1657" to ptr addrspace(1)
  %"2295" = load i64, ptr addrspace(1) %"2676", align 8
  %"2677" = inttoptr i64 %"1657" to ptr addrspace(1)
  %63 = inttoptr i64 %"2295" to ptr
  %64 = call { [3 x i32], i32 } %63(i64 %"850", i64 %"851", i64 %"852", ptr addrspace(3) %"2127", <2 x i32> %"2129", ptr addrspace(1) %"2677")
  %"853" = extractvalue { [3 x i32], i32 } %64, 0
  %"2294" = extractvalue { [3 x i32], i32 } %64, 1
  %"2297" = icmp uge i32 %"2294", 1024
  br i1 %"2297", label %"2298", label %"2299"

"2298":                                           ; preds = %"2293"
  ret i32 %"2294"

"2299":                                           ; preds = %"2293"
  store [3 x i32] %"853", ptr addrspace(5) %"2115", align 4
  %"3034" = getelementptr inbounds i8, ptr addrspace(5) %"2115", i64 0
  %"1658" = load float, ptr addrspace(5) %"3034", align 4
  store float %"1658", ptr addrspace(5) %"352", align 4
  %"3036" = getelementptr inbounds i8, ptr addrspace(5) %"2115", i64 4
  %"1659" = load float, ptr addrspace(5) %"3036", align 4
  store float %"1659", ptr addrspace(5) %"353", align 4
  %"3038" = getelementptr inbounds i8, ptr addrspace(5) %"2115", i64 8
  %"1660" = load float, ptr addrspace(5) %"3038", align 4
  store float %"1660", ptr addrspace(5) %"354", align 4
  %"3040" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 72
  %"1661" = load float, ptr addrspace(5) %"3040", align 4
  store float %"1661", ptr addrspace(5) %"355", align 4
  %"1663" = load float, ptr addrspace(5) %"355", align 4
  %"1662" = fcmp ogt float %"1663", 0.000000e+00
  store i1 %"1662", ptr addrspace(5) %"322", align 1
  %"1664" = load i1, ptr addrspace(5) %"322", align 1
  br i1 %"1664", label %"309", label %"747"

"747":                                            ; preds = %"2299"
  br label %"310"

"309":                                            ; preds = %"2299"
  %"1666" = load float, ptr addrspace(5) %"355", align 4
  %"1665" = fdiv arcp afn float 1.000000e+00, %"1666"
  store float %"1665", ptr addrspace(5) %"543", align 4
  %"1668" = load float, ptr addrspace(5) %"352", align 4
  %"1669" = load float, ptr addrspace(5) %"543", align 4
  %"1667" = fmul float %"1668", %"1669"
  store float %"1667", ptr addrspace(5) %"544", align 4
  %"1671" = load float, ptr addrspace(5) %"353", align 4
  %"1672" = load float, ptr addrspace(5) %"543", align 4
  %"1670" = fmul float %"1671", %"1672"
  store float %"1670", ptr addrspace(5) %"545", align 4
  %"1674" = load float, ptr addrspace(5) %"354", align 4
  %"1675" = load float, ptr addrspace(5) %"543", align 4
  %"1673" = fmul float %"1674", %"1675"
  store float %"1673", ptr addrspace(5) %"546", align 4
  %"3042" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 60
  %"1676" = load float, ptr addrspace(5) %"3042", align 4
  store float %"1676", ptr addrspace(5) %"547", align 4
  %"1678" = load float, ptr addrspace(5) %"547", align 4
  %"1679" = load float, ptr addrspace(5) %"544", align 4
  %"1677" = fmul float %"1678", %"1679"
  store float %"1677", ptr addrspace(5) %"548", align 4
  %"1680" = load float, ptr addrspace(5) %"548", align 4
  %"3044" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 60
  store float %"1680", ptr addrspace(5) %"3044", align 4
  %"3046" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 64
  %"1681" = load float, ptr addrspace(5) %"3046", align 4
  store float %"1681", ptr addrspace(5) %"549", align 4
  %"1683" = load float, ptr addrspace(5) %"549", align 4
  %"1684" = load float, ptr addrspace(5) %"545", align 4
  %"1682" = fmul float %"1683", %"1684"
  store float %"1682", ptr addrspace(5) %"550", align 4
  %"1685" = load float, ptr addrspace(5) %"550", align 4
  %"3048" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 64
  store float %"1685", ptr addrspace(5) %"3048", align 4
  %"3050" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 68
  %"1686" = load float, ptr addrspace(5) %"3050", align 4
  store float %"1686", ptr addrspace(5) %"551", align 4
  %"1688" = load float, ptr addrspace(5) %"551", align 4
  %"1689" = load float, ptr addrspace(5) %"546", align 4
  %"1687" = fmul float %"1688", %"1689"
  store float %"1687", ptr addrspace(5) %"552", align 4
  %"1690" = load float, ptr addrspace(5) %"552", align 4
  %"3052" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 68
  store float %"1690", ptr addrspace(5) %"3052", align 4
  br label %"311"

"310":                                            ; preds = %"747"
  %65 = alloca i16, align 2, addrspace(5)
  store i16 1, ptr addrspace(5) %65, align 2
  %"2688" = load i16, ptr addrspace(5) %65, align 2
  store i16 %"2688", ptr addrspace(5) %"327", align 2
  %"1692" = load i16, ptr addrspace(5) %"327", align 2
  %"3054" = getelementptr inbounds i8, ptr addrspace(5) %"2133", i64 8
  %"2690" = trunc i16 %"1692" to i8
  store i8 %"2690", ptr addrspace(5) %"3054", align 1
  br label %"311"

"311":                                            ; preds = %"310", %"309"
  ret i32 0
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.fma.f32(float, float, float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.sqrt.f32(float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.log2.f32(float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.exp2.f32(float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.floor.f32(float) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.smin.i32(i32, i32) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.smax.i32(i32, i32) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.maxnum.f32(float, float) #2

define protected i32 @__zluda_rt_ptx_impl__rollback_wrapper(ptr addrspace(3) %"2302", i32 %"2303", <2 x i32> %"2304", <2 x i32> %"2305", ptr addrspace(5) %"2306", float %"2307", ptr addrspace(5) %"2308", float %"2309", <2 x float> %"2310", <3 x float> %"2311", ptr addrspace(1) %"2312", ptr addrspace(1) %"2313", ptr addrspace(1) %"2314", i64 %"2315") #3 {
"2704":
  %"2316" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 0)
  %"2318" = load [12 x i8], ptr addrspace(1) %"2316", align 1
  %"2319" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 12)
  %"2321" = load [12 x i8], ptr addrspace(1) %"2319", align 1
  %"2322" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 24)
  %"2324" = load [12 x i8], ptr addrspace(1) %"2322", align 1
  %"2325" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 36)
  %"2327" = load [12 x i8], ptr addrspace(1) %"2325", align 1
  %"2328" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 48)
  %"2330" = load [12 x i8], ptr addrspace(1) %"2328", align 1
  %"2331" = call i32 @_Z11closest_hitv(ptr addrspace(3) %"2302", i32 %"2303", <2 x i32> %"2304", <2 x i32> %"2305", ptr addrspace(5) %"2306", float %"2307", ptr addrspace(5) %"2308", float %"2309", <2 x float> %"2310", <3 x float> %"2311", ptr addrspace(1) %"2312", ptr addrspace(1) %"2313", ptr addrspace(1) %"2314")
  %"2333" = icmp uge i32 %"2331", 1024
  br i1 %"2333", label %"2334", label %"2335"

"2334":                                           ; preds = %"2704"
  ret i32 %"2331"

"2335":                                           ; preds = %"2704"
  %"2337" = icmp eq i64 %"2315", 0
  br i1 %"2337", label %"2338", label %"2339"

"2338":                                           ; preds = %"2335"
  ret i32 0

"2339":                                           ; preds = %"2335"
  %"2696" = inttoptr i64 %"2315" to ptr addrspace(1)
  %"2341" = load i64, ptr addrspace(1) %"2696", align 8
  %"2697" = inttoptr i64 %"2315" to ptr addrspace(1)
  %0 = inttoptr i64 %"2341" to ptr
  %"2342" = call i32 %0(ptr addrspace(3) %"2302", i32 %"2303", <2 x i32> %"2304", <2 x i32> %"2305", ptr addrspace(5) %"2306", float %"2307", ptr addrspace(5) %"2308", float %"2309", <2 x float> %"2310", <3 x float> %"2311", ptr addrspace(1) %"2697", ptr addrspace(1) %"2313", ptr addrspace(1) %"2314")
  %"2344" = icmp uge i32 %"2342", 1024
  br i1 %"2344", label %"2345", label %"2346"

"2345":                                           ; preds = %"2339"
  ret i32 %"2342"

"2346":                                           ; preds = %"2339"
  %"2349" = icmp eq i32 %"2342", 1
  br i1 %"2349", label %"2350", label %"2347"

"2350":                                           ; preds = %"2346"
  %"2351" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 0)
  store [12 x i8] %"2318", ptr addrspace(1) %"2351", align 1
  %"2353" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 12)
  store [12 x i8] %"2321", ptr addrspace(1) %"2353", align 1
  %"2355" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 24)
  store [12 x i8] %"2324", ptr addrspace(1) %"2355", align 1
  %"2357" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 36)
  store [12 x i8] %"2327", ptr addrspace(1) %"2357", align 1
  %"2359" = call ptr addrspace(1) @__zluda_rt_ptx_impl__get_variable_pointer_global(ptr addrspace(1) %"2313", i32 48)
  store [12 x i8] %"2330", ptr addrspace(1) %"2359", align 1
  br label %"2347"

"2347":                                           ; preds = %"2350", %"2346"
  ret i32 %"2342"
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
