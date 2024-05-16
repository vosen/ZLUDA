target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@constparams = protected addrspace(4) externally_initialized global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

define protected amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"11" = alloca i1, align 1, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i16, align 2, addrspace(5)
  %"8" = alloca i16, align 2, addrspace(5)
  %"9" = alloca i16, align 2, addrspace(5)
  %"10" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"11", align 1
  %"12" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"12", ptr addrspace(5) %"5", align 8
  %"13" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"13", ptr addrspace(5) %"6", align 8
  %"14" = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %"14", ptr addrspace(5) %"7", align 2
  %"15" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 2), align 2
  store i16 %"15", ptr addrspace(5) %"8", align 2
  %"16" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 4), align 2
  store i16 %"16", ptr addrspace(5) %"9", align 2
  %"17" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 6), align 2
  store i16 %"17", ptr addrspace(5) %"10", align 2
  %"18" = load i64, ptr addrspace(5) %"6", align 8
  %"19" = load i16, ptr addrspace(5) %"7", align 2
  %"44" = inttoptr i64 %"18" to ptr
  store i16 %"19", ptr %"44", align 2
  %"20" = load i64, ptr addrspace(5) %"6", align 8
  %"21" = load i16, ptr addrspace(5) %"8", align 2
  %"46" = inttoptr i64 %"20" to ptr
  %"59" = getelementptr inbounds i8, ptr %"46", i64 2
  store i16 %"21", ptr %"59", align 2
  %"22" = load i64, ptr addrspace(5) %"6", align 8
  %"23" = load i16, ptr addrspace(5) %"9", align 2
  %"48" = inttoptr i64 %"22" to ptr
  %"61" = getelementptr inbounds i8, ptr %"48", i64 4
  store i16 %"23", ptr %"61", align 2
  %"24" = load i64, ptr addrspace(5) %"6", align 8
  %"25" = load i16, ptr addrspace(5) %"10", align 2
  %"50" = inttoptr i64 %"24" to ptr
  %"63" = getelementptr inbounds i8, ptr %"50", i64 6
  store i16 %"25", ptr %"63", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
