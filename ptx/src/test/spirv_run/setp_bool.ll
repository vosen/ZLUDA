target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @setp_bool(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #0 {
  %"16" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"9" = alloca i1, align 1, addrspace(5)
  %"10" = alloca i1, align 1, addrspace(5)
  %"11" = alloca i1, align 1, addrspace(5)
  %1 = alloca i1, align 1, addrspace(5)
  %2 = alloca float, align 4, addrspace(5)
  %3 = alloca float, align 4, addrspace(5)
  br label %4

4:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"16", align 1
  %"17" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"17", ptr addrspace(5) %"4", align 8
  %"18" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"18", ptr addrspace(5) %"5", align 8
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"46" = inttoptr i64 %"20" to ptr
  %"19" = load float, ptr %"46", align 4
  store float %"19", ptr addrspace(5) %"6", align 4
  %"22" = load i64, ptr addrspace(5) %"4", align 8
  %"47" = inttoptr i64 %"22" to ptr
  %"51" = getelementptr inbounds i8, ptr %"47", i64 4
  %"21" = load float, ptr %"51", align 4
  store float %"21", ptr addrspace(5) %"7", align 4
  %"24" = load i64, ptr addrspace(5) %"4", align 8
  %"48" = inttoptr i64 %"24" to ptr
  %"53" = getelementptr inbounds i8, ptr %"48", i64 8
  %"23" = load float, ptr %"53", align 4
  store float %"23", ptr addrspace(5) %"8", align 4
  store i1 false, ptr addrspace(5) %1, align 1
  %"25" = load i1, ptr addrspace(5) %1, align 1
  store i1 %"25", ptr addrspace(5) %"9", align 1
  %"28" = load float, ptr addrspace(5) %"6", align 4
  %"29" = load float, ptr addrspace(5) %"7", align 4
  %"30" = load i1, ptr addrspace(5) %"9", align 1
  %5 = fcmp ogt float %"28", %"29"
  %6 = xor i1 %5, true
  %"26" = and i1 %5, %"30"
  %"27" = and i1 %6, %"30"
  store i1 %"26", ptr addrspace(5) %"10", align 1
  store i1 %"27", ptr addrspace(5) %"11", align 1
  %"31" = load i1, ptr addrspace(5) %"10", align 1
  br i1 %"31", label %"12", label %"13"

"12":                                             ; preds = %4
  %"33" = load float, ptr addrspace(5) %"6", align 4
  store float %"33", ptr addrspace(5) %2, align 4
  %"32" = load float, ptr addrspace(5) %2, align 4
  store float %"32", ptr addrspace(5) %"8", align 4
  br label %"13"

"13":                                             ; preds = %"12", %4
  %"34" = load i1, ptr addrspace(5) %"11", align 1
  br i1 %"34", label %"14", label %"15"

"14":                                             ; preds = %"13"
  %"36" = load float, ptr addrspace(5) %"7", align 4
  store float %"36", ptr addrspace(5) %3, align 4
  %"35" = load float, ptr addrspace(5) %3, align 4
  store float %"35", ptr addrspace(5) %"8", align 4
  br label %"15"

"15":                                             ; preds = %"14", %"13"
  %"37" = load i64, ptr addrspace(5) %"5", align 8
  %"38" = load float, ptr addrspace(5) %"8", align 4
  %"49" = inttoptr i64 %"37" to ptr
  store float %"38", ptr %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="preserve-sign,preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
