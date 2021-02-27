target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare i32 @__zluda_ptx_impl__atom_relaxed_gpu_generic_inc(ptr, i32) #0

declare i32 @__zluda_ptx_impl__atom_relaxed_gpu_global_inc(ptr addrspace(1), i32) #0

define protected amdgpu_kernel void @atom_inc(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #1 {
"39":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"31", align 8
  store i64 %"11", ptr addrspace(5) %"4", align 8
  %"12" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"12", ptr addrspace(5) %"5", align 8
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"33" = inttoptr i64 %"14" to ptr
  %"13" = call i32 @__zluda_ptx_impl__atom_relaxed_gpu_generic_inc(ptr %"33", i32 101)
  store i32 %"13", ptr addrspace(5) %"6", align 4
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"34" = inttoptr i64 %"16" to ptr addrspace(1)
  %"15" = call i32 @__zluda_ptx_impl__atom_relaxed_gpu_global_inc(ptr addrspace(1) %"34", i32 101)
  store i32 %"15", ptr addrspace(5) %"7", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"35" = inttoptr i64 %"18" to ptr
  %"17" = load i32, ptr %"35", align 4
  store i32 %"17", ptr addrspace(5) %"8", align 4
  %"19" = load i64, ptr addrspace(5) %"5", align 8
  %"20" = load i32, ptr addrspace(5) %"6", align 4
  %"36" = inttoptr i64 %"19" to ptr
  store i32 %"20", ptr %"36", align 4
  %"21" = load i64, ptr addrspace(5) %"5", align 8
  %"22" = load i32, ptr addrspace(5) %"7", align 4
  %"37" = inttoptr i64 %"21" to ptr
  %"49" = getelementptr inbounds i8, ptr %"37", i64 4
  store i32 %"22", ptr %"49", align 4
  %"23" = load i64, ptr addrspace(5) %"5", align 8
  %"24" = load i32, ptr addrspace(5) %"8", align 4
  %"38" = inttoptr i64 %"23" to ptr
  %"51" = getelementptr inbounds i8, ptr %"38", i64 8
  store i32 %"24", ptr %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
