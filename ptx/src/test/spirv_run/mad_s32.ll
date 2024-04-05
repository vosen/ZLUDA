target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @mad_s32(ptr addrspace(4) byref(i64) %"52", ptr addrspace(4) byref(i64) %"53") #0 {
"75":
  %"13" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"13", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"11" = alloca i32, align 4, addrspace(5)
  %"12" = alloca i64, align 8, addrspace(5)
  %"14" = load i64, ptr addrspace(4) %"52", align 8
  store i64 %"14", ptr addrspace(5) %"4", align 8
  %"15" = load i64, ptr addrspace(4) %"53", align 8
  store i64 %"15", ptr addrspace(5) %"5", align 8
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"55" = inttoptr i64 %"17" to ptr
  %"54" = load i32, ptr %"55", align 4
  store i32 %"54", ptr addrspace(5) %"9", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"56" = inttoptr i64 %"19" to ptr
  %"77" = getelementptr inbounds i8, ptr %"56", i64 4
  %"57" = load i32, ptr %"77", align 4
  store i32 %"57", ptr addrspace(5) %"10", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"58" = inttoptr i64 %"21" to ptr
  %"79" = getelementptr inbounds i8, ptr %"58", i64 8
  %"20" = load i64, ptr %"79", align 8
  store i64 %"20", ptr addrspace(5) %"12", align 8
  %"23" = load i64, ptr addrspace(5) %"4", align 8
  %"59" = inttoptr i64 %"23" to ptr
  %"81" = getelementptr inbounds i8, ptr %"59", i64 16
  %"60" = load i32, ptr %"81", align 4
  store i32 %"60", ptr addrspace(5) %"11", align 4
  %"25" = load i32, ptr addrspace(5) %"9", align 4
  %"26" = load i32, ptr addrspace(5) %"10", align 4
  %"27" = load i32, ptr addrspace(5) %"11", align 4
  %0 = mul i32 %"25", %"26"
  %"24" = add i32 %0, %"27"
  store i32 %"24", ptr addrspace(5) %"6", align 4
  %"29" = load i32, ptr addrspace(5) %"9", align 4
  %"30" = load i32, ptr addrspace(5) %"10", align 4
  %"31" = load i32, ptr addrspace(5) %"11", align 4
  %1 = sext i32 %"29" to i64
  %2 = sext i32 %"30" to i64
  %3 = mul nsw i64 %1, %2
  %4 = lshr i64 %3, 32
  %5 = trunc i64 %4 to i32
  %"28" = add i32 %5, %"31"
  store i32 %"28", ptr addrspace(5) %"7", align 4
  %"33" = load i32, ptr addrspace(5) %"9", align 4
  %"34" = load i32, ptr addrspace(5) %"10", align 4
  %"35" = load i64, ptr addrspace(5) %"12", align 8
  %6 = sext i32 %"33" to i64
  %7 = sext i32 %"34" to i64
  %8 = mul nsw i64 %6, %7
  %"67" = add i64 %8, %"35"
  store i64 %"67", ptr addrspace(5) %"8", align 8
  %"36" = load i64, ptr addrspace(5) %"5", align 8
  %"37" = load i32, ptr addrspace(5) %"6", align 4
  %"71" = inttoptr i64 %"36" to ptr
  store i32 %"37", ptr %"71", align 4
  %"38" = load i64, ptr addrspace(5) %"5", align 8
  %"39" = load i32, ptr addrspace(5) %"7", align 4
  %"72" = inttoptr i64 %"38" to ptr
  %"83" = getelementptr inbounds i8, ptr %"72", i64 8
  store i32 %"39", ptr %"83", align 4
  %"40" = load i64, ptr addrspace(5) %"5", align 8
  %"41" = load i64, ptr addrspace(5) %"8", align 8
  %"73" = inttoptr i64 %"40" to ptr
  %"85" = getelementptr inbounds i8, ptr %"73", i64 16
  store i64 %"41", ptr %"85", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
