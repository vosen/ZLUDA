target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare i32 @__zluda_ptx_impl__vote_sync_ballot_b32_32(i1, i32) #0

define protected amdgpu_kernel void @vote_ballot(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"10" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"10", align 1
  %"11" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"42" = call i32 @__zluda_ptx_impl__vote_sync_ballot_b32_32(i1 true, i32 1)
  store i32 %"42", ptr addrspace(5) %"6", align 4
  %"43" = call i32 @__zluda_ptx_impl__vote_sync_ballot_b32_32(i1 false, i32 16777215)
  store i32 %"43", ptr addrspace(5) %"7", align 4
  %"44" = call i32 @__zluda_ptx_impl__vote_sync_ballot_b32_32(i1 true, i32 2)
  store i32 %"44", ptr addrspace(5) %"8", align 4
  %"45" = call i32 @__zluda_ptx_impl__vote_sync_ballot_b32_32(i1 true, i32 3)
  store i32 %"45", ptr addrspace(5) %"9", align 4
  %"16" = load i64, ptr addrspace(5) %"5", align 8
  %"17" = load i32, ptr addrspace(5) %"6", align 4
  %"46" = inttoptr i64 %"16" to ptr
  %"55" = getelementptr inbounds i8, ptr %"46", i64 0
  store i32 %"17", ptr %"55", align 4
  %"18" = load i64, ptr addrspace(5) %"5", align 8
  %"19" = load i32, ptr addrspace(5) %"7", align 4
  %"47" = inttoptr i64 %"18" to ptr
  %"57" = getelementptr inbounds i8, ptr %"47", i64 4
  store i32 %"19", ptr %"57", align 4
  %"20" = load i64, ptr addrspace(5) %"5", align 8
  %"21" = load i32, ptr addrspace(5) %"8", align 4
  %"48" = inttoptr i64 %"20" to ptr
  %"59" = getelementptr inbounds i8, ptr %"48", i64 8
  store i32 %"21", ptr %"59", align 4
  %"22" = load i64, ptr addrspace(5) %"5", align 8
  %"23" = load i32, ptr addrspace(5) %"9", align 4
  %"49" = inttoptr i64 %"22" to ptr
  %"61" = getelementptr inbounds i8, ptr %"49", i64 12
  store i32 %"23", ptr %"61", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
