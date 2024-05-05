target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @setp_pred2(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"15" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"9" = alloca i1, align 1, addrspace(5)
  %"10" = alloca i1, align 1, addrspace(5)
  %1 = alloca float, align 4, addrspace(5)
  %2 = alloca float, align 4, addrspace(5)
  br label %3

3:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"15", align 1
  %"16" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"16", ptr addrspace(5) %"4", align 8
  %"17" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"17", ptr addrspace(5) %"5", align 8
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"38" = inttoptr i64 %"19" to ptr
  %"18" = load float, ptr %"38", align 4
  store float %"18", ptr addrspace(5) %"6", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"39" = inttoptr i64 %"21" to ptr
  %"42" = getelementptr inbounds i8, ptr %"39", i64 4
  %"20" = load float, ptr %"42", align 4
  store float %"20", ptr addrspace(5) %"7", align 4
  %"24" = load float, ptr addrspace(5) %"6", align 4
  %"25" = load float, ptr addrspace(5) %"7", align 4
  %"22" = fcmp ogt float %"24", %"25"
  %"23" = xor i1 %"22", true
  store i1 %"22", ptr addrspace(5) %"9", align 1
  store i1 %"23", ptr addrspace(5) %"10", align 1
  %"26" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"26", label %"11", label %"12"

"11":                                             ; preds = %3
  %"28" = load float, ptr addrspace(5) %"6", align 4
  store float %"28", ptr addrspace(5) %1, align 4
  %"27" = load float, ptr addrspace(5) %1, align 4
  store float %"27", ptr addrspace(5) %"8", align 4
  br label %"12"

"12":                                             ; preds = %"11", %3
  %"29" = load i1, ptr addrspace(5) %"10", align 1
  br i1 %"29", label %"13", label %"14"

"13":                                             ; preds = %"12"
  %"31" = load float, ptr addrspace(5) %"7", align 4
  store float %"31", ptr addrspace(5) %2, align 4
  %"30" = load float, ptr addrspace(5) %2, align 4
  store float %"30", ptr addrspace(5) %"8", align 4
  br label %"14"

"14":                                             ; preds = %"13", %"12"
  %"32" = load i64, ptr addrspace(5) %"5", align 8
  %"33" = load float, ptr addrspace(5) %"8", align 4
  %"40" = inttoptr i64 %"32" to ptr
  store float %"33", ptr %"40", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
