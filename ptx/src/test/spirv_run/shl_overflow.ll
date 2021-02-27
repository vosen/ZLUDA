target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @shl_overflow(ptr addrspace(4) byref(i64) %"48", ptr addrspace(4) byref(i64) %"49") #0 {
"63":
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"12" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"12", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"13" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"13", ptr addrspace(5) %"4", align 8
  %"14" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"14", ptr addrspace(5) %"5", align 8
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"50" = inttoptr i64 %"16" to ptr
  %"15" = load i32, ptr %"50", align 4
  store i32 %"15", ptr addrspace(5) %"6", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"51" = inttoptr i64 %"18" to ptr
  %"65" = getelementptr inbounds i8, ptr %"51", i64 4
  %"17" = load i32, ptr %"65", align 4
  store i32 %"17", ptr addrspace(5) %"8", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"52" = inttoptr i64 %"20" to ptr
  %"67" = getelementptr inbounds i8, ptr %"52", i64 8
  %"19" = load i32, ptr %"67", align 4
  store i32 %"19", ptr addrspace(5) %"9", align 4
  %"22" = load i64, ptr addrspace(5) %"4", align 8
  %"53" = inttoptr i64 %"22" to ptr
  %"69" = getelementptr inbounds i8, ptr %"53", i64 12
  %"21" = load i32, ptr %"69", align 4
  store i32 %"21", ptr addrspace(5) %"10", align 4
  %"24" = load i32, ptr addrspace(5) %"6", align 4
  %"25" = load i32, ptr addrspace(5) %"8", align 4
  %0 = icmp ugt i32 %"25", 31
  %1 = shl i32 %"24", %"25"
  %"54" = select i1 %0, i32 0, i32 %1
  store i32 %"54", ptr addrspace(5) %"7", align 4
  %"26" = load i64, ptr addrspace(5) %"5", align 8
  %"27" = load i32, ptr addrspace(5) %"7", align 4
  %"56" = inttoptr i64 %"26" to ptr
  store i32 %"27", ptr %"56", align 4
  %"29" = load i32, ptr addrspace(5) %"6", align 4
  %"30" = load i32, ptr addrspace(5) %"9", align 4
  %2 = icmp ugt i32 %"30", 31
  %3 = shl i32 %"29", %"30"
  %"57" = select i1 %2, i32 0, i32 %3
  store i32 %"57", ptr addrspace(5) %"7", align 4
  %"31" = load i64, ptr addrspace(5) %"5", align 8
  %"32" = load i32, ptr addrspace(5) %"7", align 4
  %"59" = inttoptr i64 %"31" to ptr
  %"71" = getelementptr inbounds i8, ptr %"59", i64 4
  store i32 %"32", ptr %"71", align 4
  %"34" = load i32, ptr addrspace(5) %"6", align 4
  %"35" = load i32, ptr addrspace(5) %"10", align 4
  %4 = icmp ugt i32 %"35", 31
  %5 = shl i32 %"34", %"35"
  %"60" = select i1 %4, i32 0, i32 %5
  store i32 %"60", ptr addrspace(5) %"7", align 4
  %"36" = load i64, ptr addrspace(5) %"5", align 8
  %"37" = load i32, ptr addrspace(5) %"7", align 4
  %"62" = inttoptr i64 %"36" to ptr
  %"73" = getelementptr inbounds i8, ptr %"62", i64 8
  store i32 %"37", ptr %"73", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
