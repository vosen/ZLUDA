target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @cvta(ptr addrspace(4) byref(i64) %"19", ptr addrspace(4) byref(i64) %"20") #0 {
"27":
  %"7" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"7", align 1
  %"8" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"8", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"9" = load i64, ptr addrspace(4) %"19", align 8
  store i64 %"9", ptr addrspace(5) %"4", align 8
  %"10" = load i64, ptr addrspace(4) %"20", align 8
  store i64 %"10", ptr addrspace(5) %"5", align 8
  %"12" = load i64, ptr addrspace(5) %"4", align 8
  %0 = inttoptr i64 %"12" to ptr
  %1 = addrspacecast ptr %0 to ptr addrspace(1)
  %"21" = ptrtoint ptr addrspace(1) %1 to i64
  store i64 %"21", ptr addrspace(5) %"4", align 8
  %"14" = load i64, ptr addrspace(5) %"5", align 8
  %2 = inttoptr i64 %"14" to ptr
  %3 = addrspacecast ptr %2 to ptr addrspace(1)
  %"23" = ptrtoint ptr addrspace(1) %3 to i64
  store i64 %"23", ptr addrspace(5) %"5", align 8
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"25" = inttoptr i64 %"16" to ptr addrspace(1)
  %"15" = load float, ptr addrspace(1) %"25", align 4
  store float %"15", ptr addrspace(5) %"6", align 4
  %"17" = load i64, ptr addrspace(5) %"5", align 8
  %"18" = load float, ptr addrspace(5) %"6", align 4
  %"26" = inttoptr i64 %"17" to ptr addrspace(1)
  store float %"18", ptr addrspace(1) %"26", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
