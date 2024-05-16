target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @vector_extract(ptr addrspace(4) byref(i64) %"48", ptr addrspace(4) byref(i64) %"49") #0 {
  %"17" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i16, align 2, addrspace(5)
  %"7" = alloca i16, align 2, addrspace(5)
  %"8" = alloca i16, align 2, addrspace(5)
  %"9" = alloca i16, align 2, addrspace(5)
  %"10" = alloca <4 x i16>, align 8, addrspace(5)
  %1 = alloca <4 x i16>, align 8, addrspace(5)
  %2 = alloca <4 x i16>, align 8, addrspace(5)
  %3 = alloca <4 x i16>, align 8, addrspace(5)
  br label %4

4:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"17", align 1
  %"18" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"18", ptr addrspace(5) %"4", align 8
  %"19" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"19", ptr addrspace(5) %"5", align 8
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"50" = inttoptr i64 %"20" to ptr addrspace(1)
  %"11" = load <4 x i8>, ptr addrspace(1) %"50", align 4
  %"51" = extractelement <4 x i8> %"11", i32 0
  %"52" = extractelement <4 x i8> %"11", i32 1
  %"53" = extractelement <4 x i8> %"11", i32 2
  %"54" = extractelement <4 x i8> %"11", i32 3
  %"21" = zext i8 %"51" to i16
  %"22" = zext i8 %"52" to i16
  %"23" = zext i8 %"53" to i16
  %"24" = zext i8 %"54" to i16
  store i16 %"21", ptr addrspace(5) %"6", align 2
  store i16 %"22", ptr addrspace(5) %"7", align 2
  store i16 %"23", ptr addrspace(5) %"8", align 2
  store i16 %"24", ptr addrspace(5) %"9", align 2
  %"25" = load i16, ptr addrspace(5) %"7", align 2
  %"26" = load i16, ptr addrspace(5) %"8", align 2
  %"27" = load i16, ptr addrspace(5) %"9", align 2
  %"28" = load i16, ptr addrspace(5) %"6", align 2
  %5 = insertelement <4 x i16> undef, i16 %"25", i32 0
  %6 = insertelement <4 x i16> %5, i16 %"26", i32 1
  %7 = insertelement <4 x i16> %6, i16 %"27", i32 2
  %"12" = insertelement <4 x i16> %7, i16 %"28", i32 3
  store <4 x i16> %"12", ptr addrspace(5) %1, align 8
  %"29" = load <4 x i16>, ptr addrspace(5) %1, align 8
  store <4 x i16> %"29", ptr addrspace(5) %"10", align 8
  %"30" = load <4 x i16>, ptr addrspace(5) %"10", align 8
  store <4 x i16> %"30", ptr addrspace(5) %2, align 8
  %"13" = load <4 x i16>, ptr addrspace(5) %2, align 8
  %"31" = extractelement <4 x i16> %"13", i32 0
  %"32" = extractelement <4 x i16> %"13", i32 1
  %"33" = extractelement <4 x i16> %"13", i32 2
  %"34" = extractelement <4 x i16> %"13", i32 3
  store i16 %"31", ptr addrspace(5) %"8", align 2
  store i16 %"32", ptr addrspace(5) %"9", align 2
  store i16 %"33", ptr addrspace(5) %"6", align 2
  store i16 %"34", ptr addrspace(5) %"7", align 2
  %"35" = load i16, ptr addrspace(5) %"8", align 2
  %"36" = load i16, ptr addrspace(5) %"9", align 2
  %"37" = load i16, ptr addrspace(5) %"6", align 2
  %"38" = load i16, ptr addrspace(5) %"7", align 2
  %8 = insertelement <4 x i16> undef, i16 %"35", i32 0
  %9 = insertelement <4 x i16> %8, i16 %"36", i32 1
  %10 = insertelement <4 x i16> %9, i16 %"37", i32 2
  %"15" = insertelement <4 x i16> %10, i16 %"38", i32 3
  store <4 x i16> %"15", ptr addrspace(5) %3, align 8
  %"14" = load <4 x i16>, ptr addrspace(5) %3, align 8
  %"39" = extractelement <4 x i16> %"14", i32 0
  %"40" = extractelement <4 x i16> %"14", i32 1
  %"41" = extractelement <4 x i16> %"14", i32 2
  %"42" = extractelement <4 x i16> %"14", i32 3
  store i16 %"39", ptr addrspace(5) %"9", align 2
  store i16 %"40", ptr addrspace(5) %"6", align 2
  store i16 %"41", ptr addrspace(5) %"7", align 2
  store i16 %"42", ptr addrspace(5) %"8", align 2
  %"43" = load i16, ptr addrspace(5) %"6", align 2
  %"44" = load i16, ptr addrspace(5) %"7", align 2
  %"45" = load i16, ptr addrspace(5) %"8", align 2
  %"46" = load i16, ptr addrspace(5) %"9", align 2
  %"55" = trunc i16 %"43" to i8
  %"56" = trunc i16 %"44" to i8
  %"57" = trunc i16 %"45" to i8
  %"58" = trunc i16 %"46" to i8
  %11 = insertelement <4 x i8> undef, i8 %"55", i32 0
  %12 = insertelement <4 x i8> %11, i8 %"56", i32 1
  %13 = insertelement <4 x i8> %12, i8 %"57", i32 2
  %"16" = insertelement <4 x i8> %13, i8 %"58", i32 3
  %"47" = load i64, ptr addrspace(5) %"5", align 8
  %"59" = inttoptr i64 %"47" to ptr addrspace(1)
  store <4 x i8> %"16", ptr addrspace(1) %"59", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
