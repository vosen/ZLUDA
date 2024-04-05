target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @mov_vector_cast(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
"49":
  %"15" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"15", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"9" = alloca half, align 2, addrspace(5)
  %"10" = alloca half, align 2, addrspace(5)
  %"11" = alloca half, align 2, addrspace(5)
  %"12" = alloca half, align 2, addrspace(5)
  %"16" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"16", ptr addrspace(5) %"4", align 8
  %"17" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"17", ptr addrspace(5) %"5", align 8
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"36" = inttoptr i64 %"19" to ptr
  %"18" = load i64, ptr %"36", align 8
  store i64 %"18", ptr addrspace(5) %"6", align 8
  %"20" = load i64, ptr addrspace(5) %"6", align 8
  %0 = alloca i64, align 8, addrspace(5)
  store i64 %"20", ptr addrspace(5) %0, align 8
  %"13" = load i64, ptr addrspace(5) %0, align 8
  %"38" = bitcast i64 %"13" to <2 x i32>
  %"39" = extractelement <2 x i32> %"38", i32 0
  %"40" = extractelement <2 x i32> %"38", i32 1
  %"21" = bitcast i32 %"39" to float
  %"22" = bitcast i32 %"40" to float
  store float %"21", ptr addrspace(5) %"7", align 4
  store float %"22", ptr addrspace(5) %"8", align 4
  %"23" = load i64, ptr addrspace(5) %"6", align 8
  %1 = alloca i64, align 8, addrspace(5)
  store i64 %"23", ptr addrspace(5) %1, align 8
  %"14" = load i64, ptr addrspace(5) %1, align 8
  %"42" = bitcast i64 %"14" to <4 x i16>
  %"43" = extractelement <4 x i16> %"42", i32 0
  %"44" = extractelement <4 x i16> %"42", i32 1
  %"45" = extractelement <4 x i16> %"42", i32 2
  %"46" = extractelement <4 x i16> %"42", i32 3
  %"24" = bitcast i16 %"43" to half
  %"25" = bitcast i16 %"44" to half
  %"26" = bitcast i16 %"45" to half
  %"27" = bitcast i16 %"46" to half
  store half %"24", ptr addrspace(5) %"9", align 2
  store half %"25", ptr addrspace(5) %"10", align 2
  store half %"26", ptr addrspace(5) %"11", align 2
  store half %"27", ptr addrspace(5) %"12", align 2
  %"28" = load i64, ptr addrspace(5) %"5", align 8
  %"29" = load float, ptr addrspace(5) %"8", align 4
  %"47" = inttoptr i64 %"28" to ptr
  store float %"29", ptr %"47", align 4
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load float, ptr addrspace(5) %"7", align 4
  %"48" = inttoptr i64 %"30" to ptr
  %"51" = getelementptr inbounds i8, ptr %"48", i64 4
  store float %"31", ptr %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
