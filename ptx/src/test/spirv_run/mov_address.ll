target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @mov_address(ptr addrspace(4) byref(i64) %"9", ptr addrspace(4) byref(i64) %"10") #0 {
"12":
  %"6" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"6", align 1
  %"7" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"7", align 1
  %"4" = alloca [8 x i8], align 1, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"11" = ptrtoint ptr addrspace(5) %"4" to i64
  %0 = alloca i64, align 8, addrspace(5)
  store i64 %"11", ptr addrspace(5) %0, align 8
  %"8" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"8", ptr addrspace(5) %"5", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
