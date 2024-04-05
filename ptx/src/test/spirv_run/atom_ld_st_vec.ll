target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @atom_ld_st_vec(ptr addrspace(4) byref(i64) %"19", ptr addrspace(4) byref(i64) %"20") #0 {
"23":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"19", align 8
  store i64 %"11", ptr addrspace(5) %"4", align 8
  %"12" = load i64, ptr addrspace(4) %"20", align 8
  store i64 %"12", ptr addrspace(5) %"5", align 8
  %"13" = load i64, ptr addrspace(5) %"4", align 8
  %"21" = inttoptr i64 %"13" to ptr
  %0 = load atomic i128, ptr %"21" syncscope("agent-one-as") acquire, align 16
  %"8" = bitcast i128 %0 to <2 x i64>
  %"14" = extractelement <2 x i64> %"8", i32 0
  %"15" = extractelement <2 x i64> %"8", i32 1
  store i64 %"14", ptr addrspace(5) %"6", align 8
  store i64 %"15", ptr addrspace(5) %"7", align 8
  %"16" = load i64, ptr addrspace(5) %"6", align 8
  %"17" = load i64, ptr addrspace(5) %"7", align 8
  %1 = insertelement <2 x i64> undef, i64 %"16", i32 0
  %"9" = insertelement <2 x i64> %1, i64 %"17", i32 1
  %"18" = load i64, ptr addrspace(5) %"5", align 8
  %"22" = inttoptr i64 %"18" to ptr
  %2 = bitcast <2 x i64> %"9" to i128
  store atomic i128 %2, ptr %"22" syncscope("agent-one-as") release, align 16
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
