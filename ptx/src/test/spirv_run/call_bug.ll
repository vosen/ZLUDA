target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define private [2 x i32] @incr(i64 %"23") #0 {
"58":
  %"16" = alloca i64, align 8, addrspace(5)
  %"15" = alloca [2 x i32], align 4, addrspace(5)
  %"19" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"19", align 1
  %"20" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"20", align 1
  %"44" = alloca [2 x i32], align 4, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  store i64 %"23", ptr addrspace(5) %"16", align 8
  %"24" = load i64, ptr addrspace(5) %"16", align 8
  store i64 %"24", ptr addrspace(5) %"45", align 8
  %"25" = load i64, ptr addrspace(5) %"45", align 8
  store i64 %"25", ptr addrspace(5) %"4", align 8
  %"27" = load i64, ptr addrspace(5) %"4", align 8
  %"26" = add i64 %"27", 1
  store i64 %"26", ptr addrspace(5) %"4", align 8
  %"28" = load i64, ptr addrspace(5) %"4", align 8
  store i64 %"28", ptr addrspace(5) %"44", align 8
  %"29" = load [2 x i32], ptr addrspace(5) %"44", align 4
  store [2 x i32] %"29", ptr addrspace(5) %"15", align 4
  %"30" = load [2 x i32], ptr addrspace(5) %"15", align 4
  ret [2 x i32] %"30"
}

define protected amdgpu_kernel void @call_bug(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
"59":
  %"21" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"21", align 1
  %"22" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"22", align 1
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca [2 x i32], align 4, addrspace(5)
  %"31" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"31", ptr addrspace(5) %"8", align 8
  %"32" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"32", ptr addrspace(5) %"9", align 8
  %"34" = load i64, ptr addrspace(5) %"8", align 8
  %"52" = inttoptr i64 %"34" to ptr addrspace(1)
  %"33" = load i64, ptr addrspace(1) %"52", align 8
  store i64 %"33", ptr addrspace(5) %"10", align 8
  %"35" = load i64, ptr addrspace(5) %"10", align 8
  store i64 %"35", ptr addrspace(5) %"48", align 8
  store i64 ptrtoint (ptr @incr to i64), ptr addrspace(5) %"11", align 8
  %"17" = load i64, ptr addrspace(5) %"48", align 8
  %"37" = load i64, ptr addrspace(5) %"11", align 8
  %0 = inttoptr i64 %"37" to ptr
  %"18" = call [2 x i32] %0(i64 %"17")
  store [2 x i32] %"18", ptr addrspace(5) %"49", align 4
  %"61" = getelementptr inbounds i8, ptr addrspace(5) %"49", i64 0
  %"38" = load i64, ptr addrspace(5) %"61", align 8
  store i64 %"38", ptr addrspace(5) %"10", align 8
  %"39" = load i64, ptr addrspace(5) %"9", align 8
  %"40" = load i64, ptr addrspace(5) %"10", align 8
  %"57" = inttoptr i64 %"39" to ptr addrspace(1)
  store i64 %"40", ptr addrspace(1) %"57", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
