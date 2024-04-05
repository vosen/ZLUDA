target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @setp_gt(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
"39":
  %"14" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"14", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"9" = alloca i1, align 1, addrspace(5)
  %"15" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"15", ptr addrspace(5) %"4", align 8
  %"16" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"16", ptr addrspace(5) %"5", align 8
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"36" = inttoptr i64 %"18" to ptr
  %"17" = load float, ptr %"36", align 4
  store float %"17", ptr addrspace(5) %"6", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"37" = inttoptr i64 %"20" to ptr
  %"41" = getelementptr inbounds i8, ptr %"37", i64 4
  %"19" = load float, ptr %"41", align 4
  store float %"19", ptr addrspace(5) %"7", align 4
  %"22" = load float, ptr addrspace(5) %"6", align 4
  %"23" = load float, ptr addrspace(5) %"7", align 4
  %"21" = fcmp ogt float %"22", %"23"
  store i1 %"21", ptr addrspace(5) %"9", align 1
  %"24" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"24", label %"10", label %"11"

"10":                                             ; preds = %"39"
  %"26" = load float, ptr addrspace(5) %"6", align 4
  %0 = alloca float, align 4, addrspace(5)
  store float %"26", ptr addrspace(5) %0, align 4
  %"25" = load float, ptr addrspace(5) %0, align 4
  store float %"25", ptr addrspace(5) %"8", align 4
  br label %"11"

"11":                                             ; preds = %"10", %"39"
  %"27" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"27", label %"13", label %"12"

"12":                                             ; preds = %"11"
  %"29" = load float, ptr addrspace(5) %"7", align 4
  %1 = alloca float, align 4, addrspace(5)
  store float %"29", ptr addrspace(5) %1, align 4
  %"28" = load float, ptr addrspace(5) %1, align 4
  store float %"28", ptr addrspace(5) %"8", align 4
  br label %"13"

"13":                                             ; preds = %"12", %"11"
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load float, ptr addrspace(5) %"8", align 4
  %"38" = inttoptr i64 %"30" to ptr
  store float %"31", ptr %"38", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
