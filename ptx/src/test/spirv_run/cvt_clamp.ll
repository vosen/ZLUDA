target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare float @__zluda_ptx_impl__cvt_sat_f32_f32(float) #0

define protected amdgpu_kernel void @cvt_clamp(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #1 {
"57":
  %"7" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"7", align 1
  %"8" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"8", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"9" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"9", ptr addrspace(5) %"4", align 8
  %"10" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"10", ptr addrspace(5) %"5", align 8
  %"12" = load i64, ptr addrspace(5) %"4", align 8
  %"49" = inttoptr i64 %"12" to ptr addrspace(1)
  %"11" = load float, ptr addrspace(1) %"49", align 4
  store float %"11", ptr addrspace(5) %"6", align 4
  %"14" = load float, ptr addrspace(5) %"6", align 4
  %"13" = call float @__zluda_ptx_impl__cvt_sat_f32_f32(float %"14")
  store float %"13", ptr addrspace(5) %"6", align 4
  %"15" = load i64, ptr addrspace(5) %"5", align 8
  %"16" = load float, ptr addrspace(5) %"6", align 4
  %"50" = inttoptr i64 %"15" to ptr addrspace(1)
  store float %"16", ptr addrspace(1) %"50", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"51" = inttoptr i64 %"18" to ptr addrspace(1)
  %"62" = getelementptr inbounds i8, ptr addrspace(1) %"51", i64 4
  %"17" = load float, ptr addrspace(1) %"62", align 4
  store float %"17", ptr addrspace(5) %"6", align 4
  %"20" = load float, ptr addrspace(5) %"6", align 4
  %"19" = call float @__zluda_ptx_impl__cvt_sat_f32_f32(float %"20")
  store float %"19", ptr addrspace(5) %"6", align 4
  %"21" = load i64, ptr addrspace(5) %"5", align 8
  %"22" = load float, ptr addrspace(5) %"6", align 4
  %"52" = inttoptr i64 %"21" to ptr addrspace(1)
  %"64" = getelementptr inbounds i8, ptr addrspace(1) %"52", i64 4
  store float %"22", ptr addrspace(1) %"64", align 4
  %"24" = load i64, ptr addrspace(5) %"4", align 8
  %"53" = inttoptr i64 %"24" to ptr addrspace(1)
  %"66" = getelementptr inbounds i8, ptr addrspace(1) %"53", i64 8
  %"23" = load float, ptr addrspace(1) %"66", align 4
  store float %"23", ptr addrspace(5) %"6", align 4
  %"26" = load float, ptr addrspace(5) %"6", align 4
  %"25" = call float @__zluda_ptx_impl__cvt_sat_f32_f32(float %"26")
  store float %"25", ptr addrspace(5) %"6", align 4
  %"27" = load i64, ptr addrspace(5) %"5", align 8
  %"28" = load float, ptr addrspace(5) %"6", align 4
  %"54" = inttoptr i64 %"27" to ptr addrspace(1)
  %"68" = getelementptr inbounds i8, ptr addrspace(1) %"54", i64 8
  store float %"28", ptr addrspace(1) %"68", align 4
  %"30" = load i64, ptr addrspace(5) %"4", align 8
  %"55" = inttoptr i64 %"30" to ptr addrspace(1)
  %"70" = getelementptr inbounds i8, ptr addrspace(1) %"55", i64 12
  %"29" = load float, ptr addrspace(1) %"70", align 4
  store float %"29", ptr addrspace(5) %"6", align 4
  %"32" = load float, ptr addrspace(5) %"6", align 4
  %"31" = call float @__zluda_ptx_impl__cvt_sat_f32_f32(float %"32")
  store float %"31", ptr addrspace(5) %"6", align 4
  %"33" = load i64, ptr addrspace(5) %"5", align 8
  %"34" = load float, ptr addrspace(5) %"6", align 4
  %"56" = inttoptr i64 %"33" to ptr addrspace(1)
  %"72" = getelementptr inbounds i8, ptr addrspace(1) %"56", i64 12
  store float %"34", ptr addrspace(1) %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
