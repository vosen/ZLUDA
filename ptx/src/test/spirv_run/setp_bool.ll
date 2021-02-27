target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @setp_bool(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
"51":
  %"16" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"16", align 1
  %"17" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"17", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"9" = alloca i1, align 1, addrspace(5)
  %"10" = alloca i1, align 1, addrspace(5)
  %"11" = alloca i1, align 1, addrspace(5)
  %"18" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"18", ptr addrspace(5) %"4", align 8
  %"19" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"19", ptr addrspace(5) %"5", align 8
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"47" = inttoptr i64 %"21" to ptr
  %"20" = load float, ptr %"47", align 4
  store float %"20", ptr addrspace(5) %"6", align 4
  %"23" = load i64, ptr addrspace(5) %"4", align 8
  %"48" = inttoptr i64 %"23" to ptr
  %"53" = getelementptr inbounds i8, ptr %"48", i64 4
  %"22" = load float, ptr %"53", align 4
  store float %"22", ptr addrspace(5) %"7", align 4
  %"25" = load i64, ptr addrspace(5) %"4", align 8
  %"49" = inttoptr i64 %"25" to ptr
  %"55" = getelementptr inbounds i8, ptr %"49", i64 8
  %"24" = load float, ptr %"55", align 4
  store float %"24", ptr addrspace(5) %"8", align 4
  %0 = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %0, align 1
  %"26" = load i1, ptr addrspace(5) %0, align 1
  store i1 %"26", ptr addrspace(5) %"9", align 1
  %"29" = load float, ptr addrspace(5) %"6", align 4
  %"30" = load float, ptr addrspace(5) %"7", align 4
  %"31" = load i1, ptr addrspace(5) %"9", align 1
  %1 = fcmp ogt float %"29", %"30"
  %2 = xor i1 %1, true
  %"27" = and i1 %1, %"31"
  %"28" = and i1 %2, %"31"
  store i1 %"27", ptr addrspace(5) %"10", align 1
  store i1 %"28", ptr addrspace(5) %"11", align 1
  %"32" = load i1, ptr addrspace(5) %"10", align 1
  br i1 %"32", label %"12", label %"13"

"12":                                             ; preds = %"51"
  %"34" = load float, ptr addrspace(5) %"6", align 4
  %3 = alloca float, align 4, addrspace(5)
  store float %"34", ptr addrspace(5) %3, align 4
  %"33" = load float, ptr addrspace(5) %3, align 4
  store float %"33", ptr addrspace(5) %"8", align 4
  br label %"13"

"13":                                             ; preds = %"12", %"51"
  %"35" = load i1, ptr addrspace(5) %"11", align 1
  br i1 %"35", label %"14", label %"15"

"14":                                             ; preds = %"13"
  %"37" = load float, ptr addrspace(5) %"7", align 4
  %4 = alloca float, align 4, addrspace(5)
  store float %"37", ptr addrspace(5) %4, align 4
  %"36" = load float, ptr addrspace(5) %4, align 4
  store float %"36", ptr addrspace(5) %"8", align 4
  br label %"15"

"15":                                             ; preds = %"14", %"13"
  %"38" = load i64, ptr addrspace(5) %"5", align 8
  %"39" = load float, ptr addrspace(5) %"8", align 4
  %"50" = inttoptr i64 %"38" to ptr
  store float %"39", ptr %"50", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
