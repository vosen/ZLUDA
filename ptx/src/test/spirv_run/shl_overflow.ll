target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @shl_overflow(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #0 {
  %"11" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"11", align 1
  %"12" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"12", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"49" = inttoptr i64 %"15" to ptr
  %"14" = load i32, ptr %"49", align 4
  store i32 %"14", ptr addrspace(5) %"6", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"50" = inttoptr i64 %"17" to ptr
  %"63" = getelementptr inbounds i8, ptr %"50", i64 4
  %"16" = load i32, ptr %"63", align 4
  store i32 %"16", ptr addrspace(5) %"8", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"51" = inttoptr i64 %"19" to ptr
  %"65" = getelementptr inbounds i8, ptr %"51", i64 8
  %"18" = load i32, ptr %"65", align 4
  store i32 %"18", ptr addrspace(5) %"9", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"52" = inttoptr i64 %"21" to ptr
  %"67" = getelementptr inbounds i8, ptr %"52", i64 12
  %"20" = load i32, ptr %"67", align 4
  store i32 %"20", ptr addrspace(5) %"10", align 4
  %"23" = load i32, ptr addrspace(5) %"6", align 4
  %"24" = load i32, ptr addrspace(5) %"8", align 4
  %2 = icmp ugt i32 %"24", 31
  %3 = shl i32 %"23", %"24"
  %"53" = select i1 %2, i32 0, i32 %3
  store i32 %"53", ptr addrspace(5) %"7", align 4
  %"25" = load i64, ptr addrspace(5) %"5", align 8
  %"26" = load i32, ptr addrspace(5) %"7", align 4
  %"55" = inttoptr i64 %"25" to ptr
  store i32 %"26", ptr %"55", align 4
  %"28" = load i32, ptr addrspace(5) %"6", align 4
  %"29" = load i32, ptr addrspace(5) %"9", align 4
  %4 = icmp ugt i32 %"29", 31
  %5 = shl i32 %"28", %"29"
  %"56" = select i1 %4, i32 0, i32 %5
  store i32 %"56", ptr addrspace(5) %"7", align 4
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load i32, ptr addrspace(5) %"7", align 4
  %"58" = inttoptr i64 %"30" to ptr
  %"69" = getelementptr inbounds i8, ptr %"58", i64 4
  store i32 %"31", ptr %"69", align 4
  %"33" = load i32, ptr addrspace(5) %"6", align 4
  %"34" = load i32, ptr addrspace(5) %"10", align 4
  %6 = icmp ugt i32 %"34", 31
  %7 = shl i32 %"33", %"34"
  %"59" = select i1 %6, i32 0, i32 %7
  store i32 %"59", ptr addrspace(5) %"7", align 4
  %"35" = load i64, ptr addrspace(5) %"5", align 8
  %"36" = load i32, ptr addrspace(5) %"7", align 4
  %"61" = inttoptr i64 %"35" to ptr
  %"71" = getelementptr inbounds i8, ptr %"61", i64 8
  store i32 %"36", ptr %"71", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
