target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define private float @"1"(float %"17", float %"18") #0 {
"40":
  %"3" = alloca float, align 4, addrspace(5)
  %"4" = alloca float, align 4, addrspace(5)
  %"2" = alloca float, align 4, addrspace(5)
  %"13" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"13", align 1
  %"14" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"14", align 1
  store float %"17", ptr addrspace(5) %"3", align 4
  store float %"18", ptr addrspace(5) %"4", align 4
  %"20" = load float, ptr addrspace(5) %"3", align 4
  %"21" = load float, ptr addrspace(5) %"4", align 4
  %"19" = fadd float %"20", %"21"
  store float %"19", ptr addrspace(5) %"2", align 4
  %"22" = load float, ptr addrspace(5) %"2", align 4
  ret float %"22"
}

define protected amdgpu_kernel void @func_ptr(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
"41":
  %"15" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"15", align 1
  %"16" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"16", align 1
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i64, align 8, addrspace(5)
  %"12" = alloca i64, align 8, addrspace(5)
  %"23" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"23", ptr addrspace(5) %"8", align 8
  %"24" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"24", ptr addrspace(5) %"9", align 8
  %"26" = load i64, ptr addrspace(5) %"8", align 8
  %"38" = inttoptr i64 %"26" to ptr
  %"25" = load i64, ptr %"38", align 8
  store i64 %"25", ptr addrspace(5) %"10", align 8
  %"28" = load i64, ptr addrspace(5) %"10", align 8
  %"27" = add i64 %"28", 1
  store i64 %"27", ptr addrspace(5) %"11", align 8
  store i64 ptrtoint (ptr @"1" to i64), ptr addrspace(5) %"12", align 8
  %"31" = load i64, ptr addrspace(5) %"11", align 8
  %"32" = load i64, ptr addrspace(5) %"12", align 8
  %"30" = add i64 %"31", %"32"
  store i64 %"30", ptr addrspace(5) %"11", align 8
  %"33" = load i64, ptr addrspace(5) %"9", align 8
  %"34" = load i64, ptr addrspace(5) %"11", align 8
  %"39" = inttoptr i64 %"33" to ptr
  store i64 %"34", ptr %"39", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
