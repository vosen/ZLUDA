target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @pred_not(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
"42":
  %"14" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"14", align 1
  %"15" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"15", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i1, align 1, addrspace(5)
  %"16" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"16", ptr addrspace(5) %"4", align 8
  %"17" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"17", ptr addrspace(5) %"5", align 8
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"39" = inttoptr i64 %"19" to ptr
  %"18" = load i64, ptr %"39", align 8
  store i64 %"18", ptr addrspace(5) %"6", align 8
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"40" = inttoptr i64 %"21" to ptr
  %"44" = getelementptr inbounds i8, ptr %"40", i64 8
  %"20" = load i64, ptr %"44", align 8
  store i64 %"20", ptr addrspace(5) %"7", align 8
  %"23" = load i64, ptr addrspace(5) %"6", align 8
  %"24" = load i64, ptr addrspace(5) %"7", align 8
  %"22" = icmp ult i64 %"23", %"24"
  store i1 %"22", ptr addrspace(5) %"9", align 1
  %"26" = load i1, ptr addrspace(5) %"9", align 1
  %"25" = xor i1 %"26", true
  store i1 %"25", ptr addrspace(5) %"9", align 1
  %"27" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"27", label %"10", label %"11"

"10":                                             ; preds = %"42"
  %0 = alloca i64, align 8, addrspace(5)
  store i64 1, ptr addrspace(5) %0, align 8
  %"28" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"28", ptr addrspace(5) %"8", align 8
  br label %"11"

"11":                                             ; preds = %"10", %"42"
  %"29" = load i1, ptr addrspace(5) %"9", align 1
  br i1 %"29", label %"13", label %"12"

"12":                                             ; preds = %"11"
  %1 = alloca i64, align 8, addrspace(5)
  store i64 2, ptr addrspace(5) %1, align 8
  %"30" = load i64, ptr addrspace(5) %1, align 8
  store i64 %"30", ptr addrspace(5) %"8", align 8
  br label %"13"

"13":                                             ; preds = %"12", %"11"
  %"31" = load i64, ptr addrspace(5) %"5", align 8
  %"32" = load i64, ptr addrspace(5) %"8", align 8
  %"41" = inttoptr i64 %"31" to ptr
  store i64 %"32", ptr %"41", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
