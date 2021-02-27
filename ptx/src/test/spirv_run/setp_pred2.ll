target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @setp_pred2(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
"42":
  %"15" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"15", align 1
  %"16" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"16", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"9" = alloca i1, align 1, addrspace(5)
  %"10" = alloca i1, align 1, addrspace(5)
  %"17" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"17", ptr addrspace(5) %"4", align 8
  %"18" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"18", ptr addrspace(5) %"5", align 8
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"39" = inttoptr i64 %"20" to ptr
  %"19" = load float, ptr %"39", align 4
  store float %"19", ptr addrspace(5) %"6", align 4
  %"22" = load i64, ptr addrspace(5) %"4", align 8
  %"40" = inttoptr i64 %"22" to ptr
  %"44" = getelementptr inbounds i8, ptr %"40", i64 4
  %"21" = load float, ptr %"44", align 4
  store float %"21", ptr addrspace(5) %"7", align 4
  %"25" = load float, ptr addrspace(5) %"6", align 4
  %"26" = load float, ptr addrspace(5) %"7", align 4
  %"23" = fcmp ogt float %"25", %"26"
  %"24" = xor i1 %"23", true
  store i1 %"23", ptr addrspace(5) %"9", align 1
  store i1 %"24", ptr addrspace(5) %"10", align 1
  %"27" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"27", label %"11", label %"12"

"11":                                             ; preds = %"42"
  %"29" = load float, ptr addrspace(5) %"6", align 4
  %0 = alloca float, align 4, addrspace(5)
  store float %"29", ptr addrspace(5) %0, align 4
  %"28" = load float, ptr addrspace(5) %0, align 4
  store float %"28", ptr addrspace(5) %"8", align 4
  br label %"12"

"12":                                             ; preds = %"11", %"42"
  %"30" = load i1, ptr addrspace(5) %"10", align 1
  br i1 %"30", label %"13", label %"14"

"13":                                             ; preds = %"12"
  %"32" = load float, ptr addrspace(5) %"7", align 4
  %1 = alloca float, align 4, addrspace(5)
  store float %"32", ptr addrspace(5) %1, align 4
  %"31" = load float, ptr addrspace(5) %1, align 4
  store float %"31", ptr addrspace(5) %"8", align 4
  br label %"14"

"14":                                             ; preds = %"13", %"12"
  %"33" = load i64, ptr addrspace(5) %"5", align 8
  %"34" = load float, ptr addrspace(5) %"8", align 4
  %"41" = inttoptr i64 %"33" to ptr
  store float %"34", ptr %"41", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
