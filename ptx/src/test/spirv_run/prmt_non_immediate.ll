target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @prmt_non_immediate(ptr addrspace(4) byref(i64) %"25", ptr addrspace(4) byref(i64) %"26") #0 {
  %"9" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %1 = alloca i32, align 4, addrspace(5)
  br label %2

2:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"9", align 1
  %"10" = load i64, ptr addrspace(4) %"25", align 8
  store i64 %"10", ptr addrspace(5) %"4", align 8
  %"11" = load i64, ptr addrspace(4) %"26", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"13" = load i64, ptr addrspace(5) %"4", align 8
  %"27" = inttoptr i64 %"13" to ptr
  %"12" = load i32, ptr %"27", align 4
  store i32 %"12", ptr addrspace(5) %"6", align 4
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"28" = inttoptr i64 %"15" to ptr
  %"34" = getelementptr inbounds i8, ptr %"28", i64 4
  %"14" = load i32, ptr %"34", align 4
  store i32 %"14", ptr addrspace(5) %"7", align 4
  store i32 64, ptr addrspace(5) %1, align 4
  %"16" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"16", ptr addrspace(5) %"8", align 4
  %"18" = load i32, ptr addrspace(5) %"6", align 4
  %"19" = load i32, ptr addrspace(5) %"7", align 4
  %3 = bitcast i32 %"18" to <4 x i8>
  %4 = bitcast i32 %"19" to <4 x i8>
  %5 = shufflevector <4 x i8> %3, <4 x i8> %4, <4 x i32> <i32 0, i32 4, i32 0, i32 0>
  %"29" = bitcast <4 x i8> %5 to i32
  store i32 %"29", ptr addrspace(5) %"7", align 4
  %"20" = load i64, ptr addrspace(5) %"5", align 8
  %"21" = load i32, ptr addrspace(5) %"7", align 4
  %"32" = inttoptr i64 %"20" to ptr
  store i32 %"21", ptr %"32", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
