target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define private i64 @incr(i64 %"29") #0 {
"49":
  %"18" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i64, align 8, addrspace(5)
  %"20" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"20", align 1
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"14" = alloca i64, align 8, addrspace(5)
  store i64 %"29", ptr addrspace(5) %"18", align 8
  %"30" = load i64, ptr addrspace(5) %"18", align 8
  store i64 %"30", ptr addrspace(5) %"43", align 8
  %"31" = load i64, ptr addrspace(5) %"43", align 8
  store i64 %"31", ptr addrspace(5) %"14", align 8
  %"33" = load i64, ptr addrspace(5) %"14", align 8
  %"32" = add i64 %"33", 1
  store i64 %"32", ptr addrspace(5) %"14", align 8
  %"34" = load i64, ptr addrspace(5) %"14", align 8
  store i64 %"34", ptr addrspace(5) %"42", align 8
  %"35" = load i64, ptr addrspace(5) %"42", align 8
  store i64 %"35", ptr addrspace(5) %"17", align 8
  %"36" = load i64, ptr addrspace(5) %"17", align 8
  ret i64 %"36"
}

define protected amdgpu_kernel void @call(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
"48":
  %"19" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"19", align 1
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"21" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"21", ptr addrspace(5) %"7", align 8
  %"22" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"22", ptr addrspace(5) %"8", align 8
  %"24" = load i64, ptr addrspace(5) %"7", align 8
  %"44" = inttoptr i64 %"24" to ptr addrspace(1)
  %"23" = load i64, ptr addrspace(1) %"44", align 8
  store i64 %"23", ptr addrspace(5) %"9", align 8
  %"25" = load i64, ptr addrspace(5) %"9", align 8
  store i64 %"25", ptr addrspace(5) %"40", align 8
  %"15" = load i64, ptr addrspace(5) %"40", align 8
  %"16" = call i64 @incr(i64 %"15")
  store i64 %"16", ptr addrspace(5) %"41", align 8
  %"26" = load i64, ptr addrspace(5) %"41", align 8
  store i64 %"26", ptr addrspace(5) %"9", align 8
  %"27" = load i64, ptr addrspace(5) %"8", align 8
  %"28" = load i64, ptr addrspace(5) %"9", align 8
  %"47" = inttoptr i64 %"27" to ptr addrspace(1)
  store i64 %"28", ptr addrspace(1) %"47", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
