target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define private float @"1"(float %"15", float %"16") #0 {
"38":
  %"3" = alloca float, align 4, addrspace(5)
  %"4" = alloca float, align 4, addrspace(5)
  %"2" = alloca float, align 4, addrspace(5)
  %"13" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"13", align 1
  store float %"15", ptr addrspace(5) %"3", align 4
  store float %"16", ptr addrspace(5) %"4", align 4
  %"18" = load float, ptr addrspace(5) %"3", align 4
  %"19" = load float, ptr addrspace(5) %"4", align 4
  %"17" = fadd float %"18", %"19"
  store float %"17", ptr addrspace(5) %"2", align 4
  %"20" = load float, ptr addrspace(5) %"2", align 4
  ret float %"20"
}

define protected amdgpu_kernel void @func_ptr(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
"39":
  %"14" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"14", align 1
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i64, align 8, addrspace(5)
  %"12" = alloca i64, align 8, addrspace(5)
  %"21" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"21", ptr addrspace(5) %"8", align 8
  %"22" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"22", ptr addrspace(5) %"9", align 8
  %"24" = load i64, ptr addrspace(5) %"8", align 8
  %"36" = inttoptr i64 %"24" to ptr
  %"23" = load i64, ptr %"36", align 8
  store i64 %"23", ptr addrspace(5) %"10", align 8
  %"26" = load i64, ptr addrspace(5) %"10", align 8
  %"25" = add i64 %"26", 1
  store i64 %"25", ptr addrspace(5) %"11", align 8
  store i64 ptrtoint (ptr @"1" to i64), ptr addrspace(5) %"12", align 8
  %"29" = load i64, ptr addrspace(5) %"11", align 8
  %"30" = load i64, ptr addrspace(5) %"12", align 8
  %"28" = add i64 %"29", %"30"
  store i64 %"28", ptr addrspace(5) %"11", align 8
  %"31" = load i64, ptr addrspace(5) %"9", align 8
  %"32" = load i64, ptr addrspace(5) %"11", align 8
  %"37" = inttoptr i64 %"31" to ptr
  store i64 %"32", ptr %"37", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
