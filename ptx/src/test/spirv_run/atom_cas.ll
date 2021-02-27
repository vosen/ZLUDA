target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @atom_cas(ptr addrspace(4) byref(i64) %"30", ptr addrspace(4) byref(i64) %"31") #0 {
"39":
  %"8" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"8", align 1
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"10" = load i64, ptr addrspace(4) %"30", align 8
  store i64 %"10", ptr addrspace(5) %"4", align 8
  %"11" = load i64, ptr addrspace(4) %"31", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"13" = load i64, ptr addrspace(5) %"4", align 8
  %"32" = inttoptr i64 %"13" to ptr
  %"12" = load i32, ptr %"32", align 4
  store i32 %"12", ptr addrspace(5) %"6", align 4
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"16" = load i32, ptr addrspace(5) %"6", align 4
  %"33" = inttoptr i64 %"15" to ptr
  %"41" = getelementptr inbounds i8, ptr %"33", i64 4
  %0 = cmpxchg ptr %"41", i32 %"16", i32 100 syncscope("agent-one-as") monotonic monotonic, align 4
  %"34" = extractvalue { i32, i1 } %0, 0
  store i32 %"34", ptr addrspace(5) %"6", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"36" = inttoptr i64 %"18" to ptr
  %"43" = getelementptr inbounds i8, ptr %"36", i64 4
  %"17" = load i32, ptr %"43", align 4
  store i32 %"17", ptr addrspace(5) %"7", align 4
  %"19" = load i64, ptr addrspace(5) %"5", align 8
  %"20" = load i32, ptr addrspace(5) %"6", align 4
  %"37" = inttoptr i64 %"19" to ptr
  store i32 %"20", ptr %"37", align 4
  %"21" = load i64, ptr addrspace(5) %"5", align 8
  %"22" = load i32, ptr addrspace(5) %"7", align 4
  %"38" = inttoptr i64 %"21" to ptr
  %"45" = getelementptr inbounds i8, ptr %"38", i64 4
  store i32 %"22", ptr %"45", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
