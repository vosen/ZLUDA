target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

%struct.i64i32 = type { i64, i32 }

define private %struct.i64i32 @"1"(i32 %"39", i32 %"40") #0 {
"62":
  %"18" = alloca i32, align 4, addrspace(5)
  %"19" = alloca i32, align 4, addrspace(5)
  %"16" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i32, align 4, addrspace(5)
  %"22" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"22", align 1
  %"20" = alloca i32, align 4, addrspace(5)
  store i32 %"39", ptr addrspace(5) %"18", align 4
  store i32 %"40", ptr addrspace(5) %"19", align 4
  %"42" = load i32, ptr addrspace(5) %"18", align 4
  %"43" = load i32, ptr addrspace(5) %"19", align 4
  %"41" = add i32 %"42", %"43"
  store i32 %"41", ptr addrspace(5) %"20", align 4
  %"45" = load i32, ptr addrspace(5) %"20", align 4
  %"44" = zext i32 %"45" to i64
  store i64 %"44", ptr addrspace(5) %"16", align 8
  %"47" = load i32, ptr addrspace(5) %"18", align 4
  %"48" = load i32, ptr addrspace(5) %"19", align 4
  %"46" = mul i32 %"47", %"48"
  store i32 %"46", ptr addrspace(5) %"17", align 4
  %"49" = load i64, ptr addrspace(5) %"16", align 8
  %"50" = load i32, ptr addrspace(5) %"17", align 4
  %0 = insertvalue %struct.i64i32 undef, i64 %"49", 0
  %1 = insertvalue %struct.i64i32 %0, i32 %"50", 1
  ret %struct.i64i32 %1
}

define protected amdgpu_kernel void @call_multi_return(ptr addrspace(4) byref(i64) %"55", ptr addrspace(4) byref(i64) %"56") #0 {
"61":
  %"21" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"21", align 1
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i32, align 4, addrspace(5)
  %"12" = alloca i32, align 4, addrspace(5)
  %"13" = alloca i64, align 8, addrspace(5)
  %"14" = alloca i64, align 8, addrspace(5)
  %"15" = alloca i32, align 4, addrspace(5)
  %"23" = load i64, ptr addrspace(4) %"55", align 8
  store i64 %"23", ptr addrspace(5) %"9", align 8
  %"24" = load i64, ptr addrspace(4) %"56", align 8
  store i64 %"24", ptr addrspace(5) %"10", align 8
  %"26" = load i64, ptr addrspace(5) %"9", align 8
  %"57" = inttoptr i64 %"26" to ptr addrspace(1)
  %"25" = load i32, ptr addrspace(1) %"57", align 4
  store i32 %"25", ptr addrspace(5) %"11", align 4
  %"28" = load i64, ptr addrspace(5) %"9", align 8
  %"58" = inttoptr i64 %"28" to ptr addrspace(1)
  %"64" = getelementptr inbounds i8, ptr addrspace(1) %"58", i64 4
  %"27" = load i32, ptr addrspace(1) %"64", align 4
  store i32 %"27", ptr addrspace(5) %"12", align 4
  %"31" = load i32, ptr addrspace(5) %"11", align 4
  %"32" = load i32, ptr addrspace(5) %"12", align 4
  %0 = call %struct.i64i32 @"1"(i32 %"31", i32 %"32")
  %"29" = extractvalue %struct.i64i32 %0, 0
  %"30" = extractvalue %struct.i64i32 %0, 1
  store i64 %"29", ptr addrspace(5) %"13", align 8
  store i32 %"30", ptr addrspace(5) %"15", align 4
  %"34" = load i32, ptr addrspace(5) %"15", align 4
  %"33" = zext i32 %"34" to i64
  store i64 %"33", ptr addrspace(5) %"14", align 8
  %"35" = load i64, ptr addrspace(5) %"10", align 8
  %"36" = load i64, ptr addrspace(5) %"13", align 8
  %"59" = inttoptr i64 %"35" to ptr addrspace(1)
  store i64 %"36", ptr addrspace(1) %"59", align 8
  %"37" = load i64, ptr addrspace(5) %"10", align 8
  %"38" = load i64, ptr addrspace(5) %"14", align 8
  %"60" = inttoptr i64 %"37" to ptr addrspace(1)
  %"66" = getelementptr inbounds i8, ptr addrspace(1) %"60", i64 8
  store i64 %"38", ptr addrspace(1) %"66", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
