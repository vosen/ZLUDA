target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@PI = protected addrspace(1) externally_initialized global float 0x400921FB60000000, align 4

define protected amdgpu_kernel void @add_global(ptr addrspace(4) byref(i64) %"21", ptr addrspace(4) byref(i64) %"22") #0 {
"25":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"21", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"12" = load i64, ptr addrspace(4) %"22", align 8
  store i64 %"12", ptr addrspace(5) %"6", align 8
  %"14" = load i64, ptr addrspace(5) %"5", align 8
  %"23" = inttoptr i64 %"14" to ptr
  %"13" = load float, ptr %"23", align 4
  store float %"13", ptr addrspace(5) %"7", align 4
  %"15" = load float, ptr addrspace(1) @PI, align 4
  store float %"15", ptr addrspace(5) %"8", align 4
  %"17" = load float, ptr addrspace(5) %"7", align 4
  %"18" = load float, ptr addrspace(5) %"8", align 4
  %"16" = fadd float %"17", %"18"
  store float %"16", ptr addrspace(5) %"7", align 4
  %"19" = load i64, ptr addrspace(5) %"6", align 8
  %"20" = load float, ptr addrspace(5) %"7", align 4
  %"24" = inttoptr i64 %"19" to ptr
  store float %"20", ptr %"24", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
