target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define private <2 x i32> @"1"(<2 x i32> %"20") #0 {
"52":
  %"3" = alloca <2 x i32>, align 8, addrspace(5)
  %"2" = alloca <2 x i32>, align 8, addrspace(5)
  %"16" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"16", align 1
  %"17" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"17", align 1
  %"4" = alloca <2 x i32>, align 8, addrspace(5)
  %"5" = alloca i32, align 4, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  store <2 x i32> %"20", ptr addrspace(5) %"3", align 8
  %0 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"3", i32 0, i32 0
  %"22" = load i32, ptr addrspace(5) %0, align 4
  %1 = alloca i32, align 4, addrspace(5)
  store i32 %"22", ptr addrspace(5) %1, align 4
  %"21" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"21", ptr addrspace(5) %"5", align 4
  %2 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"3", i32 0, i32 1
  %"24" = load i32, ptr addrspace(5) %2, align 4
  %3 = alloca i32, align 4, addrspace(5)
  store i32 %"24", ptr addrspace(5) %3, align 4
  %"23" = load i32, ptr addrspace(5) %3, align 4
  store i32 %"23", ptr addrspace(5) %"6", align 4
  %"26" = load i32, ptr addrspace(5) %"5", align 4
  %"27" = load i32, ptr addrspace(5) %"6", align 4
  %"25" = add i32 %"26", %"27"
  store i32 %"25", ptr addrspace(5) %"6", align 4
  %"29" = load i32, ptr addrspace(5) %"6", align 4
  %4 = alloca i32, align 4, addrspace(5)
  store i32 %"29", ptr addrspace(5) %4, align 4
  %"28" = load i32, ptr addrspace(5) %4, align 4
  %5 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"4", i32 0, i32 0
  store i32 %"28", ptr addrspace(5) %5, align 4
  %"31" = load i32, ptr addrspace(5) %"6", align 4
  %6 = alloca i32, align 4, addrspace(5)
  store i32 %"31", ptr addrspace(5) %6, align 4
  %"30" = load i32, ptr addrspace(5) %6, align 4
  %7 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"4", i32 0, i32 1
  store i32 %"30", ptr addrspace(5) %7, align 4
  %8 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"4", i32 0, i32 1
  %"33" = load i32, ptr addrspace(5) %8, align 4
  %9 = alloca i32, align 4, addrspace(5)
  store i32 %"33", ptr addrspace(5) %9, align 4
  %"32" = load i32, ptr addrspace(5) %9, align 4
  %10 = getelementptr inbounds <2 x i32>, ptr addrspace(5) %"4", i32 0, i32 0
  store i32 %"32", ptr addrspace(5) %10, align 4
  %"35" = load <2 x i32>, ptr addrspace(5) %"4", align 8
  %11 = alloca <2 x i32>, align 8, addrspace(5)
  store <2 x i32> %"35", ptr addrspace(5) %11, align 8
  %"34" = load <2 x i32>, ptr addrspace(5) %11, align 8
  store <2 x i32> %"34", ptr addrspace(5) %"2", align 8
  %"36" = load <2 x i32>, ptr addrspace(5) %"2", align 8
  ret <2 x i32> %"36"
}

define protected amdgpu_kernel void @vector(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #0 {
"53":
  %"18" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"18", align 1
  %"19" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"19", align 1
  %"10" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i64, align 8, addrspace(5)
  %"12" = alloca <2 x i32>, align 8, addrspace(5)
  %"13" = alloca i32, align 4, addrspace(5)
  %"14" = alloca i32, align 4, addrspace(5)
  %"15" = alloca i64, align 8, addrspace(5)
  %"37" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"37", ptr addrspace(5) %"10", align 8
  %"38" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"38", ptr addrspace(5) %"11", align 8
  %"40" = load i64, ptr addrspace(5) %"10", align 8
  %"49" = inttoptr i64 %"40" to ptr
  %"39" = load <2 x i32>, ptr %"49", align 8
  store <2 x i32> %"39", ptr addrspace(5) %"12", align 8
  %"42" = load <2 x i32>, ptr addrspace(5) %"12", align 8
  %"41" = call <2 x i32> @"1"(<2 x i32> %"42")
  store <2 x i32> %"41", ptr addrspace(5) %"12", align 8
  %"44" = load <2 x i32>, ptr addrspace(5) %"12", align 8
  %"50" = bitcast <2 x i32> %"44" to i64
  %0 = alloca i64, align 8, addrspace(5)
  store i64 %"50", ptr addrspace(5) %0, align 8
  %"43" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"43", ptr addrspace(5) %"15", align 8
  %"45" = load i64, ptr addrspace(5) %"11", align 8
  %"46" = load <2 x i32>, ptr addrspace(5) %"12", align 8
  %"51" = inttoptr i64 %"45" to ptr
  store <2 x i32> %"46", ptr %"51", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
