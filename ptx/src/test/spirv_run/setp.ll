target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @setp(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"14" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i1, align 1, addrspace(5)
  %1 = alloca i64, align 8, addrspace(5)
  %2 = alloca i64, align 8, addrspace(5)
  br label %3

3:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"14", align 1
  %"15" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"15", ptr addrspace(5) %"4", align 8
  %"16" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"16", ptr addrspace(5) %"5", align 8
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"36" = inttoptr i64 %"18" to ptr
  %"17" = load i64, ptr %"36", align 8
  store i64 %"17", ptr addrspace(5) %"6", align 8
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"37" = inttoptr i64 %"20" to ptr
  %"40" = getelementptr inbounds i8, ptr %"37", i64 8
  %"19" = load i64, ptr %"40", align 8
  store i64 %"19", ptr addrspace(5) %"7", align 8
  %"22" = load i64, ptr addrspace(5) %"6", align 8
  %"23" = load i64, ptr addrspace(5) %"7", align 8
  %"21" = icmp ult i64 %"22", %"23"
  store i1 %"21", ptr addrspace(5) %"9", align 1
  %"24" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"24", label %"10", label %"11"

"10":                                             ; preds = %3
  store i64 1, ptr addrspace(5) %1, align 8
  %"25" = load i64, ptr addrspace(5) %1, align 8
  store i64 %"25", ptr addrspace(5) %"8", align 8
  br label %"11"

"11":                                             ; preds = %"10", %3
  %"26" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"26", label %"13", label %"12"

"12":                                             ; preds = %"11"
  store i64 2, ptr addrspace(5) %2, align 8
  %"27" = load i64, ptr addrspace(5) %2, align 8
  store i64 %"27", ptr addrspace(5) %"8", align 8
  br label %"13"

"13":                                             ; preds = %"12", %"11"
  %"28" = load i64, ptr addrspace(5) %"5", align 8
  %"29" = load i64, ptr addrspace(5) %"8", align 8
  %"38" = inttoptr i64 %"28" to ptr
  store i64 %"29", ptr %"38", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
