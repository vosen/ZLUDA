target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@"4" = private addrspace(3) global [1024 x i8] undef, align 4

define protected amdgpu_kernel void @atom_add_f16(ptr addrspace(4) byref(i64) %"26", ptr addrspace(4) byref(i64) %"27") #0 {
"37":
  %"8" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"8", align 1
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca half, align 2, addrspace(5)
  %"9" = load i64, ptr addrspace(4) %"26", align 8
  store i64 %"9", ptr addrspace(5) %"5", align 8
  %"10" = load i64, ptr addrspace(4) %"27", align 8
  store i64 %"10", ptr addrspace(5) %"6", align 8
  %"12" = load i64, ptr addrspace(5) %"5", align 8
  %"28" = inttoptr i64 %"12" to ptr
  %"39" = getelementptr inbounds i8, ptr %"28", i64 2
  %"29" = load i16, ptr %"39", align 2
  %"11" = bitcast i16 %"29" to half
  store half %"11", ptr addrspace(5) %"7", align 2
  %"14" = load i64, ptr addrspace(5) %"5", align 8
  %"15" = load half, ptr addrspace(5) %"7", align 2
  %"30" = inttoptr i64 %"14" to ptr
  %"13" = atomicrmw fadd ptr %"30", half %"15" syncscope("agent-one-as") monotonic, align 2
  store half %"13", ptr addrspace(5) %"7", align 2
  %"16" = load i64, ptr addrspace(5) %"6", align 8
  %"17" = load half, ptr addrspace(5) %"7", align 2
  %"31" = inttoptr i64 %"16" to ptr
  %"32" = bitcast half %"17" to i16
  store i16 %"32", ptr %"31", align 2
  %"19" = load i64, ptr addrspace(5) %"5", align 8
  %"34" = inttoptr i64 %"19" to ptr
  %"33" = load i16, ptr %"34", align 2
  %"18" = bitcast i16 %"33" to half
  store half %"18", ptr addrspace(5) %"7", align 2
  %"20" = load i64, ptr addrspace(5) %"6", align 8
  %"21" = load half, ptr addrspace(5) %"7", align 2
  %"35" = inttoptr i64 %"20" to ptr
  %"41" = getelementptr inbounds i8, ptr %"35", i64 2
  %"36" = bitcast half %"21" to i16
  store i16 %"36", ptr %"41", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
