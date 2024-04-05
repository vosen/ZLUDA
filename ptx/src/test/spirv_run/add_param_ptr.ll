target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @add_param_ptr(ptr addrspace(4) byref(i64) %"26", ptr addrspace(4) byref(i64) %"27") #0 {
"38":
  %"8" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"8", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"31" = ptrtoint ptr addrspace(4) %"26" to i64
  %0 = alloca i64, align 8, addrspace(5)
  store i64 %"31", ptr addrspace(5) %0, align 8
  %"30" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"30", ptr addrspace(5) %"4", align 8
  %"33" = ptrtoint ptr addrspace(4) %"27" to i64
  %1 = alloca i64, align 8, addrspace(5)
  store i64 %"33", ptr addrspace(5) %1, align 8
  %"32" = load i64, ptr addrspace(5) %1, align 8
  store i64 %"32", ptr addrspace(5) %"5", align 8
  %"12" = load i64, ptr addrspace(5) %"4", align 8
  %"34" = inttoptr i64 %"12" to ptr addrspace(4)
  %"40" = getelementptr inbounds i8, ptr addrspace(4) %"34", i64 0
  %"11" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"11", ptr addrspace(5) %"4", align 8
  %"14" = load i64, ptr addrspace(5) %"5", align 8
  %"35" = inttoptr i64 %"14" to ptr addrspace(4)
  %"42" = getelementptr inbounds i8, ptr addrspace(4) %"35", i64 0
  %"13" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"36" = inttoptr i64 %"16" to ptr
  %"15" = load i64, ptr %"36", align 8
  store i64 %"15", ptr addrspace(5) %"6", align 8
  %"18" = load i64, ptr addrspace(5) %"6", align 8
  %"17" = add i64 %"18", 1
  store i64 %"17", ptr addrspace(5) %"7", align 8
  %"19" = load i64, ptr addrspace(5) %"5", align 8
  %"20" = load i64, ptr addrspace(5) %"7", align 8
  %"37" = inttoptr i64 %"19" to ptr
  store i64 %"20", ptr %"37", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
