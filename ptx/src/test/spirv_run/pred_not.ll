target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @pred_not(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
"41":
  %"14" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"14", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i1, align 1, addrspace(5)
  %"15" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"15", ptr addrspace(5) %"4", align 8
  %"16" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"16", ptr addrspace(5) %"5", align 8
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"38" = inttoptr i64 %"18" to ptr
  %"17" = load i64, ptr %"38", align 8
  store i64 %"17", ptr addrspace(5) %"6", align 8
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"39" = inttoptr i64 %"20" to ptr
  %"43" = getelementptr inbounds i8, ptr %"39", i64 8
  %"19" = load i64, ptr %"43", align 8
  store i64 %"19", ptr addrspace(5) %"7", align 8
  %"22" = load i64, ptr addrspace(5) %"6", align 8
  %"23" = load i64, ptr addrspace(5) %"7", align 8
  %"21" = icmp ult i64 %"22", %"23"
  store i1 %"21", ptr addrspace(5) %"9", align 1
  %"25" = load i1, ptr addrspace(5) %"9", align 1
  %"24" = xor i1 %"25", true
  store i1 %"24", ptr addrspace(5) %"9", align 1
  %"26" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"26", label %"10", label %"11"

"10":                                             ; preds = %"41"
  %0 = alloca i64, align 8, addrspace(5)
  store i64 1, ptr addrspace(5) %0, align 8
  %"27" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"27", ptr addrspace(5) %"8", align 8
  br label %"11"

"11":                                             ; preds = %"10", %"41"
  %"28" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"28", label %"13", label %"12"

"12":                                             ; preds = %"11"
  %1 = alloca i64, align 8, addrspace(5)
  store i64 2, ptr addrspace(5) %1, align 8
  %"29" = load i64, ptr addrspace(5) %1, align 8
  store i64 %"29", ptr addrspace(5) %"8", align 8
  br label %"13"

"13":                                             ; preds = %"12", %"11"
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load i64, ptr addrspace(5) %"8", align 8
  %"40" = inttoptr i64 %"30" to ptr
  store i64 %"31", ptr %"40", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
