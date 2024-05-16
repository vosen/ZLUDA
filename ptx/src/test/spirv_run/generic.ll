target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@foo = protected addrspace(1) externally_initialized global [4 x i32] [i32 2, i32 3, i32 5, i32 7]
@bar = protected addrspace(1) externally_initialized global [4 x i64] [i64 ptrtoint (ptr addrspacecast (ptr addrspace(1) @foo to ptr) to i64), i64 add (i64 ptrtoint (ptr addrspacecast (ptr addrspace(1) @foo to ptr) to i64), i64 4), i64 add (i64 ptrtoint (ptr addrspacecast (ptr addrspace(1) @foo to ptr) to i64), i64 8), i64 add (i64 ptrtoint (ptr addrspacecast (ptr addrspace(1) @foo to ptr) to i64), i64 12)]

define protected amdgpu_kernel void @generic(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
  %"10" = alloca i1, align 1, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %1 = alloca i32, align 4, addrspace(5)
  br label %2

2:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"10", align 1
  %"11" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"11", ptr addrspace(5) %"7", align 8
  store i32 1, ptr addrspace(5) %1, align 4
  %"12" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"12", ptr addrspace(5) %"8", align 4
  %"13" = load i64, ptr addrspace(1) @bar, align 8
  store i64 %"13", ptr addrspace(5) %"6", align 8
  %"15" = load i64, ptr addrspace(5) %"6", align 8
  %"49" = inttoptr i64 %"15" to ptr
  %"14" = load i32, ptr %"49", align 4
  store i32 %"14", ptr addrspace(5) %"9", align 4
  %"17" = load i32, ptr addrspace(5) %"8", align 4
  %"18" = load i32, ptr addrspace(5) %"9", align 4
  %"16" = mul i32 %"17", %"18"
  store i32 %"16", ptr addrspace(5) %"8", align 4
  %"19" = load i64, ptr addrspace(1) getelementptr inbounds (i8, ptr addrspace(1) @bar, i64 8), align 8
  store i64 %"19", ptr addrspace(5) %"6", align 8
  %"21" = load i64, ptr addrspace(5) %"6", align 8
  %"51" = inttoptr i64 %"21" to ptr
  %"20" = load i32, ptr %"51", align 4
  store i32 %"20", ptr addrspace(5) %"9", align 4
  %"23" = load i32, ptr addrspace(5) %"8", align 4
  %"24" = load i32, ptr addrspace(5) %"9", align 4
  %"22" = mul i32 %"23", %"24"
  store i32 %"22", ptr addrspace(5) %"8", align 4
  %"25" = load i64, ptr addrspace(1) getelementptr inbounds (i8, ptr addrspace(1) @bar, i64 16), align 8
  store i64 %"25", ptr addrspace(5) %"6", align 8
  %"27" = load i64, ptr addrspace(5) %"6", align 8
  %"53" = inttoptr i64 %"27" to ptr
  %"26" = load i32, ptr %"53", align 4
  store i32 %"26", ptr addrspace(5) %"9", align 4
  %"29" = load i32, ptr addrspace(5) %"8", align 4
  %"30" = load i32, ptr addrspace(5) %"9", align 4
  %"28" = mul i32 %"29", %"30"
  store i32 %"28", ptr addrspace(5) %"8", align 4
  %"31" = load i64, ptr addrspace(1) getelementptr inbounds (i8, ptr addrspace(1) @bar, i64 24), align 8
  store i64 %"31", ptr addrspace(5) %"6", align 8
  %"33" = load i64, ptr addrspace(5) %"6", align 8
  %"55" = inttoptr i64 %"33" to ptr
  %"32" = load i32, ptr %"55", align 4
  store i32 %"32", ptr addrspace(5) %"9", align 4
  %"35" = load i32, ptr addrspace(5) %"8", align 4
  %"36" = load i32, ptr addrspace(5) %"9", align 4
  %"34" = mul i32 %"35", %"36"
  store i32 %"34", ptr addrspace(5) %"8", align 4
  %"37" = load i64, ptr addrspace(5) %"7", align 8
  %"38" = load i32, ptr addrspace(5) %"8", align 4
  %"56" = inttoptr i64 %"37" to ptr
  store i32 %"38", ptr %"56", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
