target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @setp_leu(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
"40":
  %"14" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"14", align 1
  %"15" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"15", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"9" = alloca i1, align 1, addrspace(5)
  %"16" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"16", ptr addrspace(5) %"4", align 8
  %"17" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"17", ptr addrspace(5) %"5", align 8
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"37" = inttoptr i64 %"19" to ptr
  %"18" = load float, ptr %"37", align 4
  store float %"18", ptr addrspace(5) %"6", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"38" = inttoptr i64 %"21" to ptr
  %"42" = getelementptr inbounds i8, ptr %"38", i64 4
  %"20" = load float, ptr %"42", align 4
  store float %"20", ptr addrspace(5) %"7", align 4
  %"23" = load float, ptr addrspace(5) %"6", align 4
  %"24" = load float, ptr addrspace(5) %"7", align 4
  %"22" = fcmp ule float %"23", %"24"
  store i1 %"22", ptr addrspace(5) %"9", align 1
  %"25" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"25", label %"10", label %"11"

"10":                                             ; preds = %"40"
  %"27" = load float, ptr addrspace(5) %"6", align 4
  %0 = alloca float, align 4, addrspace(5)
  store float %"27", ptr addrspace(5) %0, align 4
  %"26" = load float, ptr addrspace(5) %0, align 4
  store float %"26", ptr addrspace(5) %"8", align 4
  br label %"11"

"11":                                             ; preds = %"10", %"40"
  %"28" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"28", label %"13", label %"12"

"12":                                             ; preds = %"11"
  %"30" = load float, ptr addrspace(5) %"7", align 4
  %1 = alloca float, align 4, addrspace(5)
  store float %"30", ptr addrspace(5) %1, align 4
  %"29" = load float, ptr addrspace(5) %1, align 4
  store float %"29", ptr addrspace(5) %"8", align 4
  br label %"13"

"13":                                             ; preds = %"12", %"11"
  %"31" = load i64, ptr addrspace(5) %"5", align 8
  %"32" = load float, ptr addrspace(5) %"8", align 4
  %"39" = inttoptr i64 %"31" to ptr
  store float %"32", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
