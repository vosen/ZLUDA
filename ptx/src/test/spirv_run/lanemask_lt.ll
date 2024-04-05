target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare i32 @__zluda_ptx_impl__sreg_lanemask_lt() #0

define protected amdgpu_kernel void @lanemask_lt(ptr addrspace(4) byref(i64) %"27", ptr addrspace(4) byref(i64) %"28") #1 {
"39":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"14" = load i64, ptr addrspace(4) %"27", align 8
  store i64 %"14", ptr addrspace(5) %"4", align 8
  %"15" = load i64, ptr addrspace(4) %"28", align 8
  store i64 %"15", ptr addrspace(5) %"5", align 8
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"30" = inttoptr i64 %"17" to ptr
  %"29" = load i32, ptr %"30", align 4
  store i32 %"29", ptr addrspace(5) %"6", align 4
  %"19" = load i32, ptr addrspace(5) %"6", align 4
  %"31" = add i32 %"19", 1
  store i32 %"31", ptr addrspace(5) %"7", align 4
  %"11" = call i32 @__zluda_ptx_impl__sreg_lanemask_lt()
  %0 = alloca i32, align 4, addrspace(5)
  store i32 %"11", ptr addrspace(5) %0, align 4
  %"33" = load i32, ptr addrspace(5) %0, align 4
  store i32 %"33", ptr addrspace(5) %"8", align 4
  %"22" = load i32, ptr addrspace(5) %"7", align 4
  %"23" = load i32, ptr addrspace(5) %"8", align 4
  %"34" = add i32 %"22", %"23"
  store i32 %"34", ptr addrspace(5) %"7", align 4
  %"24" = load i64, ptr addrspace(5) %"5", align 8
  %"25" = load i32, ptr addrspace(5) %"7", align 4
  %"37" = inttoptr i64 %"24" to ptr
  store i32 %"25", ptr %"37", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
