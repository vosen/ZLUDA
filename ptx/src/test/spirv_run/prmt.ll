target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @prmt(ptr addrspace(4) byref(i64) %"32", ptr addrspace(4) byref(i64) %"33") #0 {
"44":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"12" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"12", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"34" = inttoptr i64 %"15" to ptr
  %"14" = load i32, ptr %"34", align 4
  store i32 %"14", ptr addrspace(5) %"6", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"35" = inttoptr i64 %"17" to ptr
  %"46" = getelementptr inbounds i8, ptr %"35", i64 4
  %"16" = load i32, ptr %"46", align 4
  store i32 %"16", ptr addrspace(5) %"7", align 4
  %"19" = load i32, ptr addrspace(5) %"6", align 4
  %"20" = load i32, ptr addrspace(5) %"7", align 4
  %0 = bitcast i32 %"19" to <4 x i8>
  %1 = bitcast i32 %"20" to <4 x i8>
  %2 = shufflevector <4 x i8> %0, <4 x i8> %1, <4 x i32> <i32 4, i32 0, i32 6, i32 7>
  %"36" = bitcast <4 x i8> %2 to i32
  store i32 %"36", ptr addrspace(5) %"8", align 4
  %"22" = load i32, ptr addrspace(5) %"6", align 4
  %"23" = load i32, ptr addrspace(5) %"7", align 4
  %3 = bitcast i32 %"22" to <4 x i8>
  %4 = bitcast i32 %"23" to <4 x i8>
  %5 = shufflevector <4 x i8> %3, <4 x i8> %4, <4 x i32> <i32 4, i32 0, i32 6, i32 7>
  %6 = extractelement <4 x i8> %5, i32 0
  %7 = ashr i8 %6, 7
  %8 = insertelement <4 x i8> %5, i8 %7, i32 0
  %9 = extractelement <4 x i8> %8, i32 2
  %10 = ashr i8 %9, 7
  %11 = insertelement <4 x i8> %8, i8 %10, i32 2
  %"39" = bitcast <4 x i8> %11 to i32
  store i32 %"39", ptr addrspace(5) %"9", align 4
  %"24" = load i64, ptr addrspace(5) %"5", align 8
  %"25" = load i32, ptr addrspace(5) %"8", align 4
  %"42" = inttoptr i64 %"24" to ptr
  store i32 %"25", ptr %"42", align 4
  %"26" = load i64, ptr addrspace(5) %"5", align 8
  %"27" = load i32, ptr addrspace(5) %"9", align 4
  %"43" = inttoptr i64 %"26" to ptr
  %"48" = getelementptr inbounds i8, ptr %"43", i64 4
  store i32 %"27", ptr %"48", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
