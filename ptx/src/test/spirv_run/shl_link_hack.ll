target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare i32 @__zluda_ptx_impl__atom_relaxed_gpu_generic_inc(ptr, i32) #0

define protected amdgpu_kernel void @shl_link_hack(ptr addrspace(4) byref(i64) %"22", ptr addrspace(4) byref(i64) %"23") #1 {
"29":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"10" = load i64, ptr addrspace(4) %"22", align 8
  store i64 %"10", ptr addrspace(5) %"4", align 8
  %"11" = load i64, ptr addrspace(4) %"23", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"13" = load i64, ptr addrspace(5) %"5", align 8
  %"24" = inttoptr i64 %"13" to ptr
  %"12" = call i32 @__zluda_ptx_impl__atom_relaxed_gpu_generic_inc(ptr %"24", i32 2000000)
  store i32 %"12", ptr addrspace(5) %"8", align 4
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"25" = inttoptr i64 %"15" to ptr
  %"14" = load i64, ptr %"25", align 8
  store i64 %"14", ptr addrspace(5) %"6", align 8
  %"17" = load i64, ptr addrspace(5) %"6", align 8
  %0 = shl i64 %"17", 2
  %"26" = select i1 false, i64 0, i64 %0
  store i64 %"26", ptr addrspace(5) %"7", align 8
  %"18" = load i64, ptr addrspace(5) %"5", align 8
  %"19" = load i64, ptr addrspace(5) %"7", align 8
  %"28" = inttoptr i64 %"18" to ptr
  store i64 %"19", ptr %"28", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
