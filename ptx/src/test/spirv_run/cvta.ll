target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @cvta(ptr addrspace(4) byref(i64) %"18", ptr addrspace(4) byref(i64) %"19") #0 {
  %"7" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"7", align 1
  %"8" = load i64, ptr addrspace(4) %"18", align 8
  store i64 %"8", ptr addrspace(5) %"4", align 8
  %"9" = load i64, ptr addrspace(4) %"19", align 8
  store i64 %"9", ptr addrspace(5) %"5", align 8
  %"11" = load i64, ptr addrspace(5) %"4", align 8
  %2 = inttoptr i64 %"11" to ptr
  %3 = addrspacecast ptr %2 to ptr addrspace(1)
  %"20" = ptrtoint ptr addrspace(1) %3 to i64
  store i64 %"20", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(5) %"5", align 8
  %4 = inttoptr i64 %"13" to ptr
  %5 = addrspacecast ptr %4 to ptr addrspace(1)
  %"22" = ptrtoint ptr addrspace(1) %5 to i64
  store i64 %"22", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"24" = inttoptr i64 %"15" to ptr addrspace(1)
  %"14" = load float, ptr addrspace(1) %"24", align 4
  store float %"14", ptr addrspace(5) %"6", align 4
  %"16" = load i64, ptr addrspace(5) %"5", align 8
  %"17" = load float, ptr addrspace(5) %"6", align 4
  %"25" = inttoptr i64 %"16" to ptr addrspace(1)
  store float %"17", ptr addrspace(1) %"25", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
