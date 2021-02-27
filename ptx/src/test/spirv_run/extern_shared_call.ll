target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@shared_mem = external hidden addrspace(3) global [0 x i32], align 4

define private void @"2"(ptr addrspace(3) %"37") #0 {
"35":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"3" = alloca i64, align 8, addrspace(5)
  %"14" = load i64, ptr addrspace(3) %"37", align 8
  store i64 %"14", ptr addrspace(5) %"3", align 8
  %"16" = load i64, ptr addrspace(5) %"3", align 8
  %"15" = add i64 %"16", 2
  store i64 %"15", ptr addrspace(5) %"3", align 8
  %"17" = load i64, ptr addrspace(5) %"3", align 8
  store i64 %"17", ptr addrspace(3) %"37", align 8
  ret void
}

define protected amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"27", ptr addrspace(4) byref(i64) %"28") #0 {
"36":
  %"12" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"12", align 1
  %"13" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"13", align 1
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"18" = load i64, ptr addrspace(4) %"27", align 8
  store i64 %"18", ptr addrspace(5) %"7", align 8
  %"19" = load i64, ptr addrspace(4) %"28", align 8
  store i64 %"19", ptr addrspace(5) %"8", align 8
  %"21" = load i64, ptr addrspace(5) %"7", align 8
  %"31" = inttoptr i64 %"21" to ptr addrspace(1)
  %"20" = load i64, ptr addrspace(1) %"31", align 8
  store i64 %"20", ptr addrspace(5) %"9", align 8
  %"22" = load i64, ptr addrspace(5) %"9", align 8
  store i64 %"22", ptr addrspace(3) @shared_mem, align 8
  call void @"2"(ptr addrspace(3) @shared_mem)
  %"23" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"23", ptr addrspace(5) %"9", align 8
  %"24" = load i64, ptr addrspace(5) %"8", align 8
  %"25" = load i64, ptr addrspace(5) %"9", align 8
  %"34" = inttoptr i64 %"24" to ptr addrspace(1)
  store i64 %"25", ptr addrspace(1) %"34", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
