target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @ld_st_implicit(ptr addrspace(4) byref(i64) %"16", ptr addrspace(4) byref(i64) %"17") #0 {
"22":
  %"7" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"7", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"8" = load i64, ptr addrspace(4) %"16", align 8
  store i64 %"8", ptr addrspace(5) %"4", align 8
  %"9" = load i64, ptr addrspace(4) %"17", align 8
  store i64 %"9", ptr addrspace(5) %"5", align 8
  %0 = alloca i64, align 8, addrspace(5)
  store i64 81985529216486895, ptr addrspace(5) %0, align 8
  %"10" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"10", ptr addrspace(5) %"6", align 8
  %"12" = load i64, ptr addrspace(5) %"4", align 8
  %"19" = inttoptr i64 %"12" to ptr addrspace(1)
  %"18" = load float, ptr addrspace(1) %"19", align 4
  %"23" = bitcast float %"18" to i32
  %"11" = zext i32 %"23" to i64
  store i64 %"11", ptr addrspace(5) %"6", align 8
  %"13" = load i64, ptr addrspace(5) %"5", align 8
  %"14" = load i64, ptr addrspace(5) %"6", align 8
  %"20" = inttoptr i64 %"13" to ptr addrspace(1)
  %"25" = trunc i64 %"14" to i32
  %"21" = bitcast i32 %"25" to float
  store float %"21", ptr addrspace(1) %"20", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
