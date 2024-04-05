target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define private <2 x i32> @"1"(<2 x i32> %"18") #0 {
"50":
  %"3" = alloca <2 x i32>, align 8, addrspace(5)
  %"2" = alloca <2 x i32>, align 8, addrspace(5)
  %"16" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"16", align 1
  %"4" = alloca <2 x i32>, align 8, addrspace(5)
  %"5" = alloca i32, align 4, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  store <2 x i32> %"18", ptr addrspace(5) %"3", align 8
  %0 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"3", i32 0, i32 0
  %"20" = load i32, ptr addrspace(5) %0, align 4
  %1 = alloca i32, align 4, addrspace(5)
  store i32 %"20", ptr addrspace(5) %1, align 4
  %"19" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"19", ptr addrspace(5) %"5", align 4
  %2 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"3", i32 0, i32 1
  %"22" = load i32, ptr addrspace(5) %2, align 4
  %3 = alloca i32, align 4, addrspace(5)
  store i32 %"22", ptr addrspace(5) %3, align 4
  %"21" = load i32, ptr addrspace(5) %3, align 4
  store i32 %"21", ptr addrspace(5) %"6", align 4
  %"24" = load i32, ptr addrspace(5) %"5", align 4
  %"25" = load i32, ptr addrspace(5) %"6", align 4
  %"23" = add i32 %"24", %"25"
  store i32 %"23", ptr addrspace(5) %"6", align 4
  %"27" = load i32, ptr addrspace(5) %"6", align 4
  %4 = alloca i32, align 4, addrspace(5)
  store i32 %"27", ptr addrspace(5) %4, align 4
  %"26" = load i32, ptr addrspace(5) %4, align 4
  %5 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"4", i32 0, i32 0
  store i32 %"26", ptr addrspace(5) %5, align 4
  %"29" = load i32, ptr addrspace(5) %"6", align 4
  %6 = alloca i32, align 4, addrspace(5)
  store i32 %"29", ptr addrspace(5) %6, align 4
  %"28" = load i32, ptr addrspace(5) %6, align 4
  %7 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"4", i32 0, i32 1
  store i32 %"28", ptr addrspace(5) %7, align 4
  %8 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"4", i32 0, i32 1
  %"31" = load i32, ptr addrspace(5) %8, align 4
  %9 = alloca i32, align 4, addrspace(5)
  store i32 %"31", ptr addrspace(5) %9, align 4
  %"30" = load i32, ptr addrspace(5) %9, align 4
  %10 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"4", i32 0, i32 0
  store i32 %"30", ptr addrspace(5) %10, align 4
  %"33" = load <2 x i32>, ptr addrspace(5) %"4", align 8
  %11 = alloca <2 x i32>, align 8, addrspace(5)
  store <2 x i32> %"33", ptr addrspace(5) %11, align 8
  %"32" = load <2 x i32>, ptr addrspace(5) %11, align 8
  store <2 x i32> %"32", ptr addrspace(5) %"2", align 8
  %"34" = load <2 x i32>, ptr addrspace(5) %"2", align 8
  ret <2 x i32> %"34"
}

define protected amdgpu_kernel void @vector(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
"51":
  %"17" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"17", align 1
  %"10" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i64, align 8, addrspace(5)
  %"12" = alloca <2 x i32>, align 8, addrspace(5)
  %"13" = alloca i32, align 4, addrspace(5)
  %"14" = alloca i32, align 4, addrspace(5)
  %"15" = alloca i64, align 8, addrspace(5)
  %"35" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"35", ptr addrspace(5) %"10", align 8
  %"36" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"36", ptr addrspace(5) %"11", align 8
  %"38" = load i64, ptr addrspace(5) %"10", align 8
  %"47" = inttoptr i64 %"38" to ptr
  %"37" = load <2 x i32>, ptr %"47", align 8
  store <2 x i32> %"37", ptr addrspace(5) %"12", align 8
  %"40" = load <2 x i32>, ptr addrspace(5) %"12", align 8
  %"39" = call <2 x i32> @"1"(<2 x i32> %"40")
  store <2 x i32> %"39", ptr addrspace(5) %"12", align 8
  %"42" = load <2 x i32>, ptr addrspace(5) %"12", align 8
  %"48" = bitcast <2 x i32> %"42" to i64
  %0 = alloca i64, align 8, addrspace(5)
  store i64 %"48", ptr addrspace(5) %0, align 8
  %"41" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"41", ptr addrspace(5) %"15", align 8
  %"43" = load i64, ptr addrspace(5) %"11", align 8
  %"44" = load <2 x i32>, ptr addrspace(5) %"12", align 8
  %"49" = inttoptr i64 %"43" to ptr
  store <2 x i32> %"44", ptr %"49", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
