target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @sad(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
"56":
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"12" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"12", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"41" = inttoptr i64 %"15" to ptr
  %"40" = load i32, ptr %"41", align 4
  store i32 %"40", ptr addrspace(5) %"6", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"42" = inttoptr i64 %"17" to ptr
  %"58" = getelementptr inbounds i8, ptr %"42", i64 4
  %"43" = load i32, ptr %"58", align 4
  store i32 %"43", ptr addrspace(5) %"7", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"44" = inttoptr i64 %"19" to ptr
  %"60" = getelementptr inbounds i8, ptr %"44", i64 8
  %"45" = load i32, ptr %"60", align 4
  store i32 %"45", ptr addrspace(5) %"8", align 4
  %"21" = load i32, ptr addrspace(5) %"6", align 4
  %"22" = load i32, ptr addrspace(5) %"7", align 4
  %"23" = load i32, ptr addrspace(5) %"8", align 4
  %0 = icmp ugt i32 %"21", %"22"
  %1 = sub i32 %"21", %"22"
  %2 = sub i32 %"22", %"21"
  %3 = select i1 %0, i32 %1, i32 %2
  %"46" = add i32 %"23", %3
  store i32 %"46", ptr addrspace(5) %"9", align 4
  %"25" = load i32, ptr addrspace(5) %"6", align 4
  %"26" = load i32, ptr addrspace(5) %"7", align 4
  %"27" = load i32, ptr addrspace(5) %"8", align 4
  %4 = icmp sgt i32 %"25", %"26"
  %5 = sub i32 %"25", %"26"
  %6 = sub i32 %"26", %"25"
  %7 = select i1 %4, i32 %5, i32 %6
  %"50" = add i32 %"27", %7
  store i32 %"50", ptr addrspace(5) %"10", align 4
  %"28" = load i64, ptr addrspace(5) %"5", align 8
  %"29" = load i32, ptr addrspace(5) %"9", align 4
  %"54" = inttoptr i64 %"28" to ptr
  store i32 %"29", ptr %"54", align 4
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load i32, ptr addrspace(5) %"10", align 4
  %"55" = inttoptr i64 %"30" to ptr
  %"62" = getelementptr inbounds i8, ptr %"55", i64 4
  store i32 %"31", ptr %"62", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
