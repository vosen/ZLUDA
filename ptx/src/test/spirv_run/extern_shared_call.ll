target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@shared_mem = external hidden addrspace(3) global [0 x i32], align 4

define private void @"2"(ptr addrspace(3) %"33") #0 {
  %"10" = alloca i1, align 1, addrspace(5)
  %"3" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"10", align 1
  %"12" = load i64, ptr addrspace(3) %"33", align 8
  store i64 %"12", ptr addrspace(5) %"3", align 8
  %"14" = load i64, ptr addrspace(5) %"3", align 8
  %"13" = add i64 %"14", 2
  store i64 %"13", ptr addrspace(5) %"3", align 8
  %"15" = load i64, ptr addrspace(5) %"3", align 8
  store i64 %"15", ptr addrspace(3) %"33", align 8
  ret void
}

define protected amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"25", ptr addrspace(4) byref(i64) %"26") #0 {
  %"11" = alloca i1, align 1, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"11", align 1
  %"16" = load i64, ptr addrspace(4) %"25", align 8
  store i64 %"16", ptr addrspace(5) %"7", align 8
  %"17" = load i64, ptr addrspace(4) %"26", align 8
  store i64 %"17", ptr addrspace(5) %"8", align 8
  %"19" = load i64, ptr addrspace(5) %"7", align 8
  %"29" = inttoptr i64 %"19" to ptr addrspace(1)
  %"18" = load i64, ptr addrspace(1) %"29", align 8
  store i64 %"18", ptr addrspace(5) %"9", align 8
  %"20" = load i64, ptr addrspace(5) %"9", align 8
  store i64 %"20", ptr addrspace(3) @shared_mem, align 8
  call void @"2"(ptr addrspace(3) @shared_mem)
  %"21" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"21", ptr addrspace(5) %"9", align 8
  %"22" = load i64, ptr addrspace(5) %"8", align 8
  %"23" = load i64, ptr addrspace(5) %"9", align 8
  %"32" = inttoptr i64 %"22" to ptr addrspace(1)
  store i64 %"23", ptr addrspace(1) %"32", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
