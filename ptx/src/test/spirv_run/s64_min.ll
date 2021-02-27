target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @s64_min(ptr addrspace(4) byref(i64) %"13", ptr addrspace(4) byref(i64) %"14") #0 {
"16":
  %"6" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"6", align 1
  %"7" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"7", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"8" = load i64, ptr addrspace(4) %"14", align 8
  store i64 %"8", ptr addrspace(5) %"4", align 8
  %0 = alloca i64, align 8, addrspace(5)
  store i64 -9223372036854775808, ptr addrspace(5) %0, align 8
  %"9" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"9", ptr addrspace(5) %"5", align 8
  %"10" = load i64, ptr addrspace(5) %"4", align 8
  %"11" = load i64, ptr addrspace(5) %"5", align 8
  %"15" = inttoptr i64 %"10" to ptr
  store i64 %"11", ptr %"15", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
