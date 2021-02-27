target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @mad_s32(ptr addrspace(4) byref(i64) %"53", ptr addrspace(4) byref(i64) %"54") #0 {
"76":
  %"13" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"13", align 1
  %"14" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"14", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"11" = alloca i32, align 4, addrspace(5)
  %"12" = alloca i64, align 8, addrspace(5)
  %"15" = load i64, ptr addrspace(4) %"53", align 8
  store i64 %"15", ptr addrspace(5) %"4", align 8
  %"16" = load i64, ptr addrspace(4) %"54", align 8
  store i64 %"16", ptr addrspace(5) %"5", align 8
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"56" = inttoptr i64 %"18" to ptr
  %"55" = load i32, ptr %"56", align 4
  store i32 %"55", ptr addrspace(5) %"9", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"57" = inttoptr i64 %"20" to ptr
  %"78" = getelementptr inbounds i8, ptr %"57", i64 4
  %"58" = load i32, ptr %"78", align 4
  store i32 %"58", ptr addrspace(5) %"10", align 4
  %"22" = load i64, ptr addrspace(5) %"4", align 8
  %"59" = inttoptr i64 %"22" to ptr
  %"80" = getelementptr inbounds i8, ptr %"59", i64 8
  %"21" = load i64, ptr %"80", align 8
  store i64 %"21", ptr addrspace(5) %"12", align 8
  %"24" = load i64, ptr addrspace(5) %"4", align 8
  %"60" = inttoptr i64 %"24" to ptr
  %"82" = getelementptr inbounds i8, ptr %"60", i64 16
  %"61" = load i32, ptr %"82", align 4
  store i32 %"61", ptr addrspace(5) %"11", align 4
  %"26" = load i32, ptr addrspace(5) %"9", align 4
  %"27" = load i32, ptr addrspace(5) %"10", align 4
  %"28" = load i32, ptr addrspace(5) %"11", align 4
  %0 = mul i32 %"26", %"27"
  %"25" = add i32 %0, %"28"
  store i32 %"25", ptr addrspace(5) %"6", align 4
  %"30" = load i32, ptr addrspace(5) %"9", align 4
  %"31" = load i32, ptr addrspace(5) %"10", align 4
  %"32" = load i32, ptr addrspace(5) %"11", align 4
  %1 = sext i32 %"30" to i64
  %2 = sext i32 %"31" to i64
  %3 = mul nsw i64 %1, %2
  %4 = lshr i64 %3, 32
  %5 = trunc i64 %4 to i32
  %"29" = add i32 %5, %"32"
  store i32 %"29", ptr addrspace(5) %"7", align 4
  %"34" = load i32, ptr addrspace(5) %"9", align 4
  %"35" = load i32, ptr addrspace(5) %"10", align 4
  %"36" = load i64, ptr addrspace(5) %"12", align 8
  %6 = sext i32 %"34" to i64
  %7 = sext i32 %"35" to i64
  %8 = mul nsw i64 %6, %7
  %"68" = add i64 %8, %"36"
  store i64 %"68", ptr addrspace(5) %"8", align 8
  %"37" = load i64, ptr addrspace(5) %"5", align 8
  %"38" = load i32, ptr addrspace(5) %"6", align 4
  %"72" = inttoptr i64 %"37" to ptr
  store i32 %"38", ptr %"72", align 4
  %"39" = load i64, ptr addrspace(5) %"5", align 8
  %"40" = load i32, ptr addrspace(5) %"7", align 4
  %"73" = inttoptr i64 %"39" to ptr
  %"84" = getelementptr inbounds i8, ptr %"73", i64 8
  store i32 %"40", ptr %"84", align 4
  %"41" = load i64, ptr addrspace(5) %"5", align 8
  %"42" = load i64, ptr addrspace(5) %"8", align 8
  %"74" = inttoptr i64 %"41" to ptr
  %"86" = getelementptr inbounds i8, ptr %"74", i64 16
  store i64 %"42", ptr %"86", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
