target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@foo = protected addrspace(1) externally_initialized global [4 x i32] [i32 2, i32 3, i32 5, i32 7]
@bar = protected addrspace(1) externally_initialized global [4 x i64] [i64 ptrtoint (ptr addrspacecast (ptr addrspace(1) @foo to ptr) to i64), i64 add (i64 ptrtoint (ptr addrspacecast (ptr addrspace(1) @foo to ptr) to i64), i64 4), i64 add (i64 ptrtoint (ptr addrspacecast (ptr addrspace(1) @foo to ptr) to i64), i64 8), i64 add (i64 ptrtoint (ptr addrspacecast (ptr addrspace(1) @foo to ptr) to i64), i64 12)]

define protected amdgpu_kernel void @generic(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #0 {
"58":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"12" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"12", ptr addrspace(5) %"7", align 8
  %0 = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %0, align 4
  %"13" = load i32, ptr addrspace(5) %0, align 4
  store i32 %"13", ptr addrspace(5) %"8", align 4
  %"14" = load i64, ptr addrspace(1) @bar, align 8
  store i64 %"14", ptr addrspace(5) %"6", align 8
  %"16" = load i64, ptr addrspace(5) %"6", align 8
  %"50" = inttoptr i64 %"16" to ptr
  %"15" = load i32, ptr %"50", align 4
  store i32 %"15", ptr addrspace(5) %"9", align 4
  %"18" = load i32, ptr addrspace(5) %"8", align 4
  %"19" = load i32, ptr addrspace(5) %"9", align 4
  %"17" = mul i32 %"18", %"19"
  store i32 %"17", ptr addrspace(5) %"8", align 4
  %"20" = load i64, ptr addrspace(1) getelementptr inbounds (i8, ptr addrspace(1) @bar, i64 8), align 8
  store i64 %"20", ptr addrspace(5) %"6", align 8
  %"22" = load i64, ptr addrspace(5) %"6", align 8
  %"52" = inttoptr i64 %"22" to ptr
  %"21" = load i32, ptr %"52", align 4
  store i32 %"21", ptr addrspace(5) %"9", align 4
  %"24" = load i32, ptr addrspace(5) %"8", align 4
  %"25" = load i32, ptr addrspace(5) %"9", align 4
  %"23" = mul i32 %"24", %"25"
  store i32 %"23", ptr addrspace(5) %"8", align 4
  %"26" = load i64, ptr addrspace(1) getelementptr inbounds (i8, ptr addrspace(1) @bar, i64 16), align 8
  store i64 %"26", ptr addrspace(5) %"6", align 8
  %"28" = load i64, ptr addrspace(5) %"6", align 8
  %"54" = inttoptr i64 %"28" to ptr
  %"27" = load i32, ptr %"54", align 4
  store i32 %"27", ptr addrspace(5) %"9", align 4
  %"30" = load i32, ptr addrspace(5) %"8", align 4
  %"31" = load i32, ptr addrspace(5) %"9", align 4
  %"29" = mul i32 %"30", %"31"
  store i32 %"29", ptr addrspace(5) %"8", align 4
  %"32" = load i64, ptr addrspace(1) getelementptr inbounds (i8, ptr addrspace(1) @bar, i64 24), align 8
  store i64 %"32", ptr addrspace(5) %"6", align 8
  %"34" = load i64, ptr addrspace(5) %"6", align 8
  %"56" = inttoptr i64 %"34" to ptr
  %"33" = load i32, ptr %"56", align 4
  store i32 %"33", ptr addrspace(5) %"9", align 4
  %"36" = load i32, ptr addrspace(5) %"8", align 4
  %"37" = load i32, ptr addrspace(5) %"9", align 4
  %"35" = mul i32 %"36", %"37"
  store i32 %"35", ptr addrspace(5) %"8", align 4
  %"38" = load i64, ptr addrspace(5) %"7", align 8
  %"39" = load i32, ptr addrspace(5) %"8", align 4
  %"57" = inttoptr i64 %"38" to ptr
  store i32 %"39", ptr %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
