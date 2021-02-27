target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @shr_u32(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
"46":
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
  %"13" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"13", ptr addrspace(5) %"4", align 8
  %"14" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"14", ptr addrspace(5) %"5", align 8
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"39" = inttoptr i64 %"16" to ptr
  %"15" = load i32, ptr %"39", align 4
  store i32 %"15", ptr addrspace(5) %"6", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"40" = inttoptr i64 %"18" to ptr
  %"48" = getelementptr inbounds i8, ptr %"40", i64 4
  %"17" = load i32, ptr %"48", align 4
  store i32 %"17", ptr addrspace(5) %"7", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"41" = inttoptr i64 %"20" to ptr
  %"50" = getelementptr inbounds i8, ptr %"41", i64 8
  %"19" = load i32, ptr %"50", align 4
  store i32 %"19", ptr addrspace(5) %"8", align 4
  %"22" = load i32, ptr addrspace(5) %"6", align 4
  %"23" = load i32, ptr addrspace(5) %"7", align 4
  %0 = icmp ugt i32 %"23", 31
  %1 = lshr i32 %"22", %"23"
  %"21" = select i1 %0, i32 0, i32 %1
  store i32 %"21", ptr addrspace(5) %"9", align 4
  %"25" = load i32, ptr addrspace(5) %"6", align 4
  %"26" = load i32, ptr addrspace(5) %"8", align 4
  %2 = icmp ugt i32 %"26", 31
  %3 = lshr i32 %"25", %"26"
  %"24" = select i1 %2, i32 0, i32 %3
  store i32 %"24", ptr addrspace(5) %"10", align 4
  %"27" = load i64, ptr addrspace(5) %"5", align 8
  %"28" = load i32, ptr addrspace(5) %"9", align 4
  %"44" = inttoptr i64 %"27" to ptr
  store i32 %"28", ptr %"44", align 4
  %"29" = load i64, ptr addrspace(5) %"5", align 8
  %"30" = load i32, ptr addrspace(5) %"10", align 4
  %"45" = inttoptr i64 %"29" to ptr
  %"52" = getelementptr inbounds i8, ptr %"45", i64 4
  store i32 %"30", ptr %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
