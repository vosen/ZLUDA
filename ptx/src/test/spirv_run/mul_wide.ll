target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @mul_wide(ptr addrspace(4) byref(i64) %"24", ptr addrspace(4) byref(i64) %"25") #0 {
"30":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"24", align 8
  store i64 %"11", ptr addrspace(5) %"4", align 8
  %"12" = load i64, ptr addrspace(4) %"25", align 8
  store i64 %"12", ptr addrspace(5) %"5", align 8
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"26" = inttoptr i64 %"14" to ptr addrspace(1)
  %"13" = load i32, ptr addrspace(1) %"26", align 4
  store i32 %"13", ptr addrspace(5) %"6", align 4
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"27" = inttoptr i64 %"16" to ptr addrspace(1)
  %"32" = getelementptr inbounds i8, ptr addrspace(1) %"27", i64 4
  %"15" = load i32, ptr addrspace(1) %"32", align 4
  store i32 %"15", ptr addrspace(5) %"7", align 4
  %"18" = load i32, ptr addrspace(5) %"6", align 4
  %"19" = load i32, ptr addrspace(5) %"7", align 4
  %0 = sext i32 %"18" to i64
  %1 = sext i32 %"19" to i64
  %"17" = mul nsw i64 %0, %1
  store i64 %"17", ptr addrspace(5) %"8", align 8
  %"20" = load i64, ptr addrspace(5) %"5", align 8
  %"21" = load i64, ptr addrspace(5) %"8", align 8
  %"28" = inttoptr i64 %"20" to ptr
  store i64 %"21", ptr %"28", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
