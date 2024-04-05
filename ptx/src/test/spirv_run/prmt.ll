target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @prmt(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #0 {
"43":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"31", align 8
  store i64 %"11", ptr addrspace(5) %"4", align 8
  %"12" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"12", ptr addrspace(5) %"5", align 8
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"33" = inttoptr i64 %"14" to ptr
  %"13" = load i32, ptr %"33", align 4
  store i32 %"13", ptr addrspace(5) %"6", align 4
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"34" = inttoptr i64 %"16" to ptr
  %"45" = getelementptr inbounds i8, ptr %"34", i64 4
  %"15" = load i32, ptr %"45", align 4
  store i32 %"15", ptr addrspace(5) %"7", align 4
  %"18" = load i32, ptr addrspace(5) %"6", align 4
  %"19" = load i32, ptr addrspace(5) %"7", align 4
  %0 = bitcast i32 %"18" to <4 x i8>
  %1 = bitcast i32 %"19" to <4 x i8>
  %2 = shufflevector <4 x i8> %0, <4 x i8> %1, <4 x i32> <i32 4, i32 0, i32 6, i32 7>
  %"35" = bitcast <4 x i8> %2 to i32
  store i32 %"35", ptr addrspace(5) %"8", align 4
  %"21" = load i32, ptr addrspace(5) %"6", align 4
  %"22" = load i32, ptr addrspace(5) %"7", align 4
  %3 = bitcast i32 %"21" to <4 x i8>
  %4 = bitcast i32 %"22" to <4 x i8>
  %5 = shufflevector <4 x i8> %3, <4 x i8> %4, <4 x i32> <i32 4, i32 0, i32 6, i32 7>
  %6 = extractelement <4 x i8> %5, i32 0
  %7 = ashr i8 %6, 7
  %8 = insertelement <4 x i8> %5, i8 %7, i32 0
  %9 = extractelement <4 x i8> %8, i32 2
  %10 = ashr i8 %9, 7
  %11 = insertelement <4 x i8> %8, i8 %10, i32 2
  %"38" = bitcast <4 x i8> %11 to i32
  store i32 %"38", ptr addrspace(5) %"9", align 4
  %"23" = load i64, ptr addrspace(5) %"5", align 8
  %"24" = load i32, ptr addrspace(5) %"8", align 4
  %"41" = inttoptr i64 %"23" to ptr
  store i32 %"24", ptr %"41", align 4
  %"25" = load i64, ptr addrspace(5) %"5", align 8
  %"26" = load i32, ptr addrspace(5) %"9", align 4
  %"42" = inttoptr i64 %"25" to ptr
  %"47" = getelementptr inbounds i8, ptr %"42", i64 4
  store i32 %"26", ptr %"47", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
