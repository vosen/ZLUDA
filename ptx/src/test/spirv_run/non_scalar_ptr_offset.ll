target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @non_scalar_ptr_offset(ptr addrspace(4) byref(i64) %"23", ptr addrspace(4) byref(i64) %"24") #0 {
"27":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"23", align 8
  store i64 %"11", ptr addrspace(5) %"4", align 8
  %"12" = load i64, ptr addrspace(4) %"24", align 8
  store i64 %"12", ptr addrspace(5) %"5", align 8
  %"13" = load i64, ptr addrspace(5) %"4", align 8
  %"25" = inttoptr i64 %"13" to ptr addrspace(1)
  %"29" = getelementptr inbounds i8, ptr addrspace(1) %"25", i64 8
  %"8" = load <2 x i32>, ptr addrspace(1) %"29", align 8
  %"14" = extractelement <2 x i32> %"8", i32 0
  %"15" = extractelement <2 x i32> %"8", i32 1
  store i32 %"14", ptr addrspace(5) %"6", align 4
  store i32 %"15", ptr addrspace(5) %"7", align 4
  %"17" = load i32, ptr addrspace(5) %"6", align 4
  %"18" = load i32, ptr addrspace(5) %"7", align 4
  %"16" = add i32 %"17", %"18"
  store i32 %"16", ptr addrspace(5) %"6", align 4
  %"19" = load i64, ptr addrspace(5) %"5", align 8
  %"20" = load i32, ptr addrspace(5) %"6", align 4
  %"26" = inttoptr i64 %"19" to ptr addrspace(1)
  store i32 %"20", ptr addrspace(1) %"26", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
