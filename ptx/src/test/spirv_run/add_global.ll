target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@PI = protected addrspace(1) externally_initialized global float 0x400921FB60000000, align 4

define protected amdgpu_kernel void @add_global(ptr addrspace(4) byref(i64) %"20", ptr addrspace(4) byref(i64) %"21") #0 {
  %"9" = alloca i1, align 1, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"9", align 1
  %"10" = load i64, ptr addrspace(4) %"20", align 8
  store i64 %"10", ptr addrspace(5) %"5", align 8
  %"11" = load i64, ptr addrspace(4) %"21", align 8
  store i64 %"11", ptr addrspace(5) %"6", align 8
  %"13" = load i64, ptr addrspace(5) %"5", align 8
  %"22" = inttoptr i64 %"13" to ptr
  %"12" = load float, ptr %"22", align 4
  store float %"12", ptr addrspace(5) %"7", align 4
  %"14" = load float, ptr addrspace(1) @PI, align 4
  store float %"14", ptr addrspace(5) %"8", align 4
  %"16" = load float, ptr addrspace(5) %"7", align 4
  %"17" = load float, ptr addrspace(5) %"8", align 4
  %"15" = fadd float %"16", %"17"
  store float %"15", ptr addrspace(5) %"7", align 4
  %"18" = load i64, ptr addrspace(5) %"6", align 8
  %"19" = load float, ptr addrspace(5) %"7", align 4
  %"23" = inttoptr i64 %"18" to ptr
  store float %"19", ptr %"23", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
