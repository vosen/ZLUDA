target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define private [2 x i32] @incr(i64 %"21") #0 {
"56":
  %"16" = alloca i64, align 8, addrspace(5)
  %"15" = alloca [2 x i32], align 4, addrspace(5)
  %"19" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"19", align 1
  %"42" = alloca [2 x i32], align 4, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  store i64 %"21", ptr addrspace(5) %"16", align 8
  %"22" = load i64, ptr addrspace(5) %"16", align 8
  store i64 %"22", ptr addrspace(5) %"43", align 8
  %"23" = load i64, ptr addrspace(5) %"43", align 8
  store i64 %"23", ptr addrspace(5) %"4", align 8
  %"25" = load i64, ptr addrspace(5) %"4", align 8
  %"24" = add i64 %"25", 1
  store i64 %"24", ptr addrspace(5) %"4", align 8
  %"26" = load i64, ptr addrspace(5) %"4", align 8
  store i64 %"26", ptr addrspace(5) %"42", align 8
  %"27" = load [2 x i32], ptr addrspace(5) %"42", align 4
  store [2 x i32] %"27", ptr addrspace(5) %"15", align 4
  %"28" = load [2 x i32], ptr addrspace(5) %"15", align 4
  ret [2 x i32] %"28"
}

define protected amdgpu_kernel void @call_bug(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #0 {
"57":
  %"20" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"20", align 1
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca [2 x i32], align 4, addrspace(5)
  %"29" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"29", ptr addrspace(5) %"8", align 8
  %"30" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"30", ptr addrspace(5) %"9", align 8
  %"32" = load i64, ptr addrspace(5) %"8", align 8
  %"50" = inttoptr i64 %"32" to ptr addrspace(1)
  %"31" = load i64, ptr addrspace(1) %"50", align 8
  store i64 %"31", ptr addrspace(5) %"10", align 8
  %"33" = load i64, ptr addrspace(5) %"10", align 8
  store i64 %"33", ptr addrspace(5) %"46", align 8
  store i64 ptrtoint (ptr @incr to i64), ptr addrspace(5) %"11", align 8
  %"17" = load i64, ptr addrspace(5) %"46", align 8
  %"35" = load i64, ptr addrspace(5) %"11", align 8
  %0 = inttoptr i64 %"35" to ptr
  %"18" = call [2 x i32] %0(i64 %"17")
  store [2 x i32] %"18", ptr addrspace(5) %"47", align 4
  %"59" = getelementptr inbounds i8, ptr addrspace(5) %"47", i64 0
  %"36" = load i64, ptr addrspace(5) %"59", align 8
  store i64 %"36", ptr addrspace(5) %"10", align 8
  %"37" = load i64, ptr addrspace(5) %"9", align 8
  %"38" = load i64, ptr addrspace(5) %"10", align 8
  %"55" = inttoptr i64 %"37" to ptr addrspace(1)
  store i64 %"38", ptr addrspace(1) %"55", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
