target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @atom_cas(ptr addrspace(4) byref(i64) %"29", ptr addrspace(4) byref(i64) %"30") #0 {
  %"8" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"8", align 1
  %"9" = load i64, ptr addrspace(4) %"29", align 8
  store i64 %"9", ptr addrspace(5) %"4", align 8
  %"10" = load i64, ptr addrspace(4) %"30", align 8
  store i64 %"10", ptr addrspace(5) %"5", align 8
  %"12" = load i64, ptr addrspace(5) %"4", align 8
  %"31" = inttoptr i64 %"12" to ptr
  %"11" = load i32, ptr %"31", align 4
  store i32 %"11", ptr addrspace(5) %"6", align 4
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"15" = load i32, ptr addrspace(5) %"6", align 4
  %"32" = inttoptr i64 %"14" to ptr
  %"39" = getelementptr inbounds i8, ptr %"32", i64 4
  %2 = cmpxchg ptr %"39", i32 %"15", i32 100 syncscope("agent-one-as") monotonic monotonic, align 4
  %"33" = extractvalue { i32, i1 } %2, 0
  store i32 %"33", ptr addrspace(5) %"6", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"35" = inttoptr i64 %"17" to ptr
  %"41" = getelementptr inbounds i8, ptr %"35", i64 4
  %"16" = load i32, ptr %"41", align 4
  store i32 %"16", ptr addrspace(5) %"7", align 4
  %"18" = load i64, ptr addrspace(5) %"5", align 8
  %"19" = load i32, ptr addrspace(5) %"6", align 4
  %"36" = inttoptr i64 %"18" to ptr
  store i32 %"19", ptr %"36", align 4
  %"20" = load i64, ptr addrspace(5) %"5", align 8
  %"21" = load i32, ptr addrspace(5) %"7", align 4
  %"37" = inttoptr i64 %"20" to ptr
  %"43" = getelementptr inbounds i8, ptr %"37", i64 4
  store i32 %"21", ptr %"43", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
