target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"24", ptr addrspace(4) byref(i64) %"25") #0 {
"34":
  %"8" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"8", align 1
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"4" = alloca [8 x i8], align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"10" = load i64, ptr addrspace(4) %"24", align 8
  store i64 %"10", ptr addrspace(5) %"5", align 8
  %"11" = load i64, ptr addrspace(4) %"25", align 8
  store i64 %"11", ptr addrspace(5) %"6", align 8
  %"13" = load i64, ptr addrspace(5) %"5", align 8
  %"27" = inttoptr i64 %"13" to ptr addrspace(1)
  %"26" = load i64, ptr addrspace(1) %"27", align 8
  store i64 %"26", ptr addrspace(5) %"7", align 8
  %"14" = load i64, ptr addrspace(5) %"7", align 8
  %"19" = add i64 %"14", 1
  %"28" = addrspacecast ptr addrspace(5) %"4" to ptr
  store i64 %"19", ptr %"28", align 8
  %"30" = addrspacecast ptr addrspace(5) %"4" to ptr
  %"38" = getelementptr inbounds i8, ptr %"30", i64 0
  %"31" = load i64, ptr %"38", align 8
  store i64 %"31", ptr addrspace(5) %"7", align 8
  %"16" = load i64, ptr addrspace(5) %"6", align 8
  %"17" = load i64, ptr addrspace(5) %"7", align 8
  %"32" = inttoptr i64 %"16" to ptr addrspace(1)
  %"40" = getelementptr inbounds i8, ptr addrspace(1) %"32", i64 0
  store i64 %"17", ptr addrspace(1) %"40", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
