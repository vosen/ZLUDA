target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @vshr(ptr addrspace(4) byref(i64) %"30", ptr addrspace(4) byref(i64) %"31") #0 {
"39":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"12" = load i64, ptr addrspace(4) %"30", align 8
  store i64 %"12", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(4) %"31", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"33" = inttoptr i64 %"15" to ptr
  %"32" = load i32, ptr %"33", align 4
  store i32 %"32", ptr addrspace(5) %"7", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"34" = inttoptr i64 %"17" to ptr
  %"41" = getelementptr inbounds i8, ptr %"34", i64 4
  %"35" = load i32, ptr %"41", align 4
  store i32 %"35", ptr addrspace(5) %"8", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"36" = inttoptr i64 %"19" to ptr
  %"43" = getelementptr inbounds i8, ptr %"36", i64 8
  %"37" = load i32, ptr %"43", align 4
  store i32 %"37", ptr addrspace(5) %"9", align 4
  %"21" = load i32, ptr addrspace(5) %"7", align 4
  %"22" = load i32, ptr addrspace(5) %"8", align 4
  %"23" = load i32, ptr addrspace(5) %"9", align 4
  %0 = icmp ugt i32 %"22", 31
  %1 = lshr i32 %"21", %"22"
  %2 = select i1 %0, i32 0, i32 %1
  %"20" = add i32 %2, %"23"
  store i32 %"20", ptr addrspace(5) %"6", align 4
  %"24" = load i64, ptr addrspace(5) %"5", align 8
  %"25" = load i32, ptr addrspace(5) %"6", align 4
  %"38" = inttoptr i64 %"24" to ptr
  store i32 %"25", ptr %"38", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
