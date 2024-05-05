target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare i32 @__zluda_ptx_impl__sreg_ntid(i8) #0

define protected amdgpu_kernel void @ntid(ptr addrspace(4) byref(i64) %"25", ptr addrspace(4) byref(i64) %"26") #1 {
  %"9" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %1 = alloca i32, align 4, addrspace(5)
  br label %2

2:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"9", align 1
  %"15" = load i64, ptr addrspace(4) %"25", align 8
  store i64 %"15", ptr addrspace(5) %"4", align 8
  %"16" = load i64, ptr addrspace(4) %"26", align 8
  store i64 %"16", ptr addrspace(5) %"5", align 8
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"27" = inttoptr i64 %"18" to ptr
  %"17" = load i32, ptr %"27", align 4
  store i32 %"17", ptr addrspace(5) %"6", align 4
  %"11" = call i32 @__zluda_ptx_impl__sreg_ntid(i8 0)
  store i32 %"11", ptr addrspace(5) %1, align 4
  %"19" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"19", ptr addrspace(5) %"7", align 4
  %"21" = load i32, ptr addrspace(5) %"6", align 4
  %"22" = load i32, ptr addrspace(5) %"7", align 4
  %"20" = add i32 %"21", %"22"
  store i32 %"20", ptr addrspace(5) %"6", align 4
  %"23" = load i64, ptr addrspace(5) %"5", align 8
  %"24" = load i32, ptr addrspace(5) %"6", align 4
  %"28" = inttoptr i64 %"23" to ptr
  store i32 %"24", ptr %"28", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
