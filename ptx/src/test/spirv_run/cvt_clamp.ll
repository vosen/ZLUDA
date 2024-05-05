target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare float @__zluda_ptx_impl__cvt_sat_f32_f32(float) #0

define protected amdgpu_kernel void @cvt_clamp(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #1 {
  %"7" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"7", align 1
  %"8" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"8", ptr addrspace(5) %"4", align 8
  %"9" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"9", ptr addrspace(5) %"5", align 8
  %"11" = load i64, ptr addrspace(5) %"4", align 8
  %"48" = inttoptr i64 %"11" to ptr addrspace(1)
  %"10" = load float, ptr addrspace(1) %"48", align 4
  store float %"10", ptr addrspace(5) %"6", align 4
  %"13" = load float, ptr addrspace(5) %"6", align 4
  %"12" = call float @__zluda_ptx_impl__cvt_sat_f32_f32(float %"13")
  store float %"12", ptr addrspace(5) %"6", align 4
  %"14" = load i64, ptr addrspace(5) %"5", align 8
  %"15" = load float, ptr addrspace(5) %"6", align 4
  %"49" = inttoptr i64 %"14" to ptr addrspace(1)
  store float %"15", ptr addrspace(1) %"49", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"50" = inttoptr i64 %"17" to ptr addrspace(1)
  %"60" = getelementptr inbounds i8, ptr addrspace(1) %"50", i64 4
  %"16" = load float, ptr addrspace(1) %"60", align 4
  store float %"16", ptr addrspace(5) %"6", align 4
  %"19" = load float, ptr addrspace(5) %"6", align 4
  %"18" = call float @__zluda_ptx_impl__cvt_sat_f32_f32(float %"19")
  store float %"18", ptr addrspace(5) %"6", align 4
  %"20" = load i64, ptr addrspace(5) %"5", align 8
  %"21" = load float, ptr addrspace(5) %"6", align 4
  %"51" = inttoptr i64 %"20" to ptr addrspace(1)
  %"62" = getelementptr inbounds i8, ptr addrspace(1) %"51", i64 4
  store float %"21", ptr addrspace(1) %"62", align 4
  %"23" = load i64, ptr addrspace(5) %"4", align 8
  %"52" = inttoptr i64 %"23" to ptr addrspace(1)
  %"64" = getelementptr inbounds i8, ptr addrspace(1) %"52", i64 8
  %"22" = load float, ptr addrspace(1) %"64", align 4
  store float %"22", ptr addrspace(5) %"6", align 4
  %"25" = load float, ptr addrspace(5) %"6", align 4
  %"24" = call float @__zluda_ptx_impl__cvt_sat_f32_f32(float %"25")
  store float %"24", ptr addrspace(5) %"6", align 4
  %"26" = load i64, ptr addrspace(5) %"5", align 8
  %"27" = load float, ptr addrspace(5) %"6", align 4
  %"53" = inttoptr i64 %"26" to ptr addrspace(1)
  %"66" = getelementptr inbounds i8, ptr addrspace(1) %"53", i64 8
  store float %"27", ptr addrspace(1) %"66", align 4
  %"29" = load i64, ptr addrspace(5) %"4", align 8
  %"54" = inttoptr i64 %"29" to ptr addrspace(1)
  %"68" = getelementptr inbounds i8, ptr addrspace(1) %"54", i64 12
  %"28" = load float, ptr addrspace(1) %"68", align 4
  store float %"28", ptr addrspace(5) %"6", align 4
  %"31" = load float, ptr addrspace(5) %"6", align 4
  %"30" = call float @__zluda_ptx_impl__cvt_sat_f32_f32(float %"31")
  store float %"30", ptr addrspace(5) %"6", align 4
  %"32" = load i64, ptr addrspace(5) %"5", align 8
  %"33" = load float, ptr addrspace(5) %"6", align 4
  %"55" = inttoptr i64 %"32" to ptr addrspace(1)
  %"70" = getelementptr inbounds i8, ptr addrspace(1) %"55", i64 12
  store float %"33", ptr addrspace(1) %"70", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
