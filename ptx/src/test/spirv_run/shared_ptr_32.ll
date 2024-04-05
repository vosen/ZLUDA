target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@"4" = private addrspace(3) global [128 x i8] undef, align 4

define protected amdgpu_kernel void @shared_ptr_32(ptr addrspace(4) byref(i64) %"24", ptr addrspace(4) byref(i64) %"25") #0 {
"31":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"24", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"12" = load i64, ptr addrspace(4) %"25", align 8
  store i64 %"12", ptr addrspace(5) %"6", align 8
  %0 = alloca i32, align 4, addrspace(5)
  store i32 ptrtoint (ptr addrspace(3) @"4" to i32), ptr addrspace(5) %0, align 4
  %"13" = load i32, ptr addrspace(5) %0, align 4
  store i32 %"13", ptr addrspace(5) %"7", align 4
  %"15" = load i64, ptr addrspace(5) %"5", align 8
  %"27" = inttoptr i64 %"15" to ptr addrspace(1)
  %"14" = load i64, ptr addrspace(1) %"27", align 8
  store i64 %"14", ptr addrspace(5) %"8", align 8
  %"16" = load i32, ptr addrspace(5) %"7", align 4
  %"17" = load i64, ptr addrspace(5) %"8", align 8
  %"28" = inttoptr i32 %"16" to ptr addrspace(3)
  store i64 %"17", ptr addrspace(3) %"28", align 8
  %"19" = load i32, ptr addrspace(5) %"7", align 4
  %"29" = inttoptr i32 %"19" to ptr addrspace(3)
  %"33" = getelementptr inbounds i8, ptr addrspace(3) %"29", i64 0
  %"18" = load i64, ptr addrspace(3) %"33", align 8
  store i64 %"18", ptr addrspace(5) %"9", align 8
  %"20" = load i64, ptr addrspace(5) %"6", align 8
  %"21" = load i64, ptr addrspace(5) %"9", align 8
  %"30" = inttoptr i64 %"20" to ptr addrspace(1)
  store i64 %"21", ptr addrspace(1) %"30", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
