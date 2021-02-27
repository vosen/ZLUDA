target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @mov_vector_cast(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
"50":
  %"15" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"15", align 1
  %"16" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"16", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"9" = alloca half, align 2, addrspace(5)
  %"10" = alloca half, align 2, addrspace(5)
  %"11" = alloca half, align 2, addrspace(5)
  %"12" = alloca half, align 2, addrspace(5)
  %"17" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"17", ptr addrspace(5) %"4", align 8
  %"18" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"18", ptr addrspace(5) %"5", align 8
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"37" = inttoptr i64 %"20" to ptr
  %"19" = load i64, ptr %"37", align 8
  store i64 %"19", ptr addrspace(5) %"6", align 8
  %"21" = load i64, ptr addrspace(5) %"6", align 8
  %0 = alloca i64, align 8, addrspace(5)
  store i64 %"21", ptr addrspace(5) %0, align 8
  %"13" = load i64, ptr addrspace(5) %0, align 8
  %"39" = bitcast i64 %"13" to <2 x i32>
  %"40" = extractelement <2 x i32> %"39", i32 0
  %"41" = extractelement <2 x i32> %"39", i32 1
  %"22" = bitcast i32 %"40" to float
  %"23" = bitcast i32 %"41" to float
  store float %"22", ptr addrspace(5) %"7", align 4
  store float %"23", ptr addrspace(5) %"8", align 4
  %"24" = load i64, ptr addrspace(5) %"6", align 8
  %1 = alloca i64, align 8, addrspace(5)
  store i64 %"24", ptr addrspace(5) %1, align 8
  %"14" = load i64, ptr addrspace(5) %1, align 8
  %"43" = bitcast i64 %"14" to <4 x i16>
  %"44" = extractelement <4 x i16> %"43", i32 0
  %"45" = extractelement <4 x i16> %"43", i32 1
  %"46" = extractelement <4 x i16> %"43", i32 2
  %"47" = extractelement <4 x i16> %"43", i32 3
  %"25" = bitcast i16 %"44" to half
  %"26" = bitcast i16 %"45" to half
  %"27" = bitcast i16 %"46" to half
  %"28" = bitcast i16 %"47" to half
  store half %"25", ptr addrspace(5) %"9", align 2
  store half %"26", ptr addrspace(5) %"10", align 2
  store half %"27", ptr addrspace(5) %"11", align 2
  store half %"28", ptr addrspace(5) %"12", align 2
  %"29" = load i64, ptr addrspace(5) %"5", align 8
  %"30" = load float, ptr addrspace(5) %"8", align 4
  %"48" = inttoptr i64 %"29" to ptr
  store float %"30", ptr %"48", align 4
  %"31" = load i64, ptr addrspace(5) %"5", align 8
  %"32" = load float, ptr addrspace(5) %"7", align 4
  %"49" = inttoptr i64 %"31" to ptr
  %"52" = getelementptr inbounds i8, ptr %"49", i64 4
  store float %"32", ptr %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
