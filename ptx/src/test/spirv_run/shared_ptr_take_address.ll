target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@shared_mem = external hidden addrspace(3) global [0 x i8], align 4

define protected amdgpu_kernel void @shared_ptr_take_address(ptr addrspace(4) byref(i64) %"22", ptr addrspace(4) byref(i64) %"23") #0 {
"29":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"22", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"12" = load i64, ptr addrspace(4) %"23", align 8
  store i64 %"12", ptr addrspace(5) %"6", align 8
  %0 = alloca i64, align 8, addrspace(5)
  store i64 ptrtoint (ptr addrspace(3) @shared_mem to i64), ptr addrspace(5) %0, align 8
  %"13" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"13", ptr addrspace(5) %"7", align 8
  %"15" = load i64, ptr addrspace(5) %"5", align 8
  %"25" = inttoptr i64 %"15" to ptr addrspace(1)
  %"14" = load i64, ptr addrspace(1) %"25", align 8
  store i64 %"14", ptr addrspace(5) %"8", align 8
  %"16" = load i64, ptr addrspace(5) %"7", align 8
  %"17" = load i64, ptr addrspace(5) %"8", align 8
  %"26" = inttoptr i64 %"16" to ptr addrspace(3)
  store i64 %"17", ptr addrspace(3) %"26", align 8
  %"19" = load i64, ptr addrspace(5) %"7", align 8
  %"27" = inttoptr i64 %"19" to ptr addrspace(3)
  %"18" = load i64, ptr addrspace(3) %"27", align 8
  store i64 %"18", ptr addrspace(5) %"9", align 8
  %"20" = load i64, ptr addrspace(5) %"6", align 8
  %"21" = load i64, ptr addrspace(5) %"9", align 8
  %"28" = inttoptr i64 %"20" to ptr addrspace(1)
  store i64 %"21", ptr addrspace(1) %"28", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
