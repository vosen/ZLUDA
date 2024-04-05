target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define private i64 @incr(i64 %"33") #0 {
"54":
  %"20" = alloca i64, align 8, addrspace(5)
  %"19" = alloca i64, align 8, addrspace(5)
  %"22" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"22", align 1
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"16" = alloca i64, align 8, addrspace(5)
  store i64 %"33", ptr addrspace(5) %"20", align 8
  %"34" = load i64, ptr addrspace(5) %"20", align 8
  store i64 %"34", ptr addrspace(5) %"47", align 8
  %"35" = load i64, ptr addrspace(5) %"47", align 8
  store i64 %"35", ptr addrspace(5) %"16", align 8
  %"37" = load i64, ptr addrspace(5) %"16", align 8
  %"36" = add i64 %"37", 1
  store i64 %"36", ptr addrspace(5) %"16", align 8
  %"38" = load i64, ptr addrspace(5) %"16", align 8
  store i64 %"38", ptr addrspace(5) %"46", align 8
  %"39" = load i64, ptr addrspace(5) %"46", align 8
  store i64 %"39", ptr addrspace(5) %"19", align 8
  %"40" = load i64, ptr addrspace(5) %"19", align 8
  ret i64 %"40"
}

define protected amdgpu_kernel void @callprototype(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
"53":
  %"21" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"21", align 1
  %"7" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"23" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"23", ptr addrspace(5) %"7", align 8
  %"24" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"24", ptr addrspace(5) %"8", align 8
  %"26" = load i64, ptr addrspace(5) %"7", align 8
  %"48" = inttoptr i64 %"26" to ptr addrspace(1)
  %"25" = load i64, ptr addrspace(1) %"48", align 8
  store i64 %"25", ptr addrspace(5) %"9", align 8
  %"27" = load i64, ptr addrspace(5) %"9", align 8
  store i64 %"27", ptr addrspace(5) %"44", align 8
  store i64 ptrtoint (ptr @incr to i64), ptr addrspace(5) %"10", align 8
  %"17" = load i64, ptr addrspace(5) %"44", align 8
  %"29" = load i64, ptr addrspace(5) %"10", align 8
  %0 = inttoptr i64 %"29" to ptr
  %"18" = call i64 %0(i64 %"17")
  store i64 %"18", ptr addrspace(5) %"45", align 8
  %"30" = load i64, ptr addrspace(5) %"45", align 8
  store i64 %"30", ptr addrspace(5) %"9", align 8
  %"31" = load i64, ptr addrspace(5) %"8", align 8
  %"32" = load i64, ptr addrspace(5) %"9", align 8
  %"52" = inttoptr i64 %"31" to ptr addrspace(1)
  store i64 %"32", ptr addrspace(1) %"52", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
