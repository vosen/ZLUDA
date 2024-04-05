target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@"4" = private addrspace(3) global [1024 x i8] undef, align 4

define protected amdgpu_kernel void @atom_add(ptr addrspace(4) byref(i64) %"28", ptr addrspace(4) byref(i64) %"29") #0 {
"37":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"10" = load i64, ptr addrspace(4) %"28", align 8
  store i64 %"10", ptr addrspace(5) %"5", align 8
  %"11" = load i64, ptr addrspace(4) %"29", align 8
  store i64 %"11", ptr addrspace(5) %"6", align 8
  %"13" = load i64, ptr addrspace(5) %"5", align 8
  %"30" = inttoptr i64 %"13" to ptr
  %"12" = load i32, ptr %"30", align 4
  store i32 %"12", ptr addrspace(5) %"7", align 4
  %"15" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = inttoptr i64 %"15" to ptr
  %"39" = getelementptr inbounds i8, ptr %"31", i64 4
  %"14" = load i32, ptr %"39", align 4
  store i32 %"14", ptr addrspace(5) %"8", align 4
  %"16" = load i32, ptr addrspace(5) %"7", align 4
  store i32 %"16", ptr addrspace(3) @"4", align 4
  %"18" = load i32, ptr addrspace(5) %"8", align 4
  %"17" = atomicrmw add ptr addrspace(3) @"4", i32 %"18" syncscope("agent-one-as") monotonic, align 4
  store i32 %"17", ptr addrspace(5) %"7", align 4
  %"19" = load i32, ptr addrspace(3) @"4", align 4
  store i32 %"19", ptr addrspace(5) %"8", align 4
  %"20" = load i64, ptr addrspace(5) %"6", align 8
  %"21" = load i32, ptr addrspace(5) %"7", align 4
  %"35" = inttoptr i64 %"20" to ptr
  store i32 %"21", ptr %"35", align 4
  %"22" = load i64, ptr addrspace(5) %"6", align 8
  %"23" = load i32, ptr addrspace(5) %"8", align 4
  %"36" = inttoptr i64 %"22" to ptr
  %"41" = getelementptr inbounds i8, ptr %"36", i64 4
  store i32 %"23", ptr %"41", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
