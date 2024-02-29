target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@fn_ptrs = protected addrspace(1) externally_initialized global [2 x i64] [i64 0, i64 ptrtoint (ptr @incr to i64)], align 8

define private i64 @incr(i64 %"36") #0 {
"60":
  %"21" = alloca i64, align 8, addrspace(5)
  %"20" = alloca i64, align 8, addrspace(5)
  %"24" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"24", align 1
  %"25" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"25", align 1
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i64, align 8, addrspace(5)
  store i64 %"36", ptr addrspace(5) %"21", align 8
  %"37" = load i64, ptr addrspace(5) %"21", align 8
  store i64 %"37", ptr addrspace(5) %"52", align 8
  %"38" = load i64, ptr addrspace(5) %"52", align 8
  store i64 %"38", ptr addrspace(5) %"17", align 8
  %"40" = load i64, ptr addrspace(5) %"17", align 8
  %"39" = add i64 %"40", 1
  store i64 %"39", ptr addrspace(5) %"17", align 8
  %"41" = load i64, ptr addrspace(5) %"17", align 8
  store i64 %"41", ptr addrspace(5) %"51", align 8
  %"42" = load i64, ptr addrspace(5) %"51", align 8
  store i64 %"42", ptr addrspace(5) %"20", align 8
  %"43" = load i64, ptr addrspace(5) %"20", align 8
  ret i64 %"43"
}

define protected amdgpu_kernel void @call_global_ptr(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #0 {
"59":
  %"22" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"22", align 1
  %"23" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"23", align 1
  %"8" = alloca i64, align 8, addrspace(5)
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"26" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"26", ptr addrspace(5) %"8", align 8
  %"27" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"27", ptr addrspace(5) %"9", align 8
  %"29" = load i64, ptr addrspace(5) %"8", align 8
  %"53" = inttoptr i64 %"29" to ptr addrspace(1)
  %"28" = load i64, ptr addrspace(1) %"53", align 8
  store i64 %"28", ptr addrspace(5) %"10", align 8
  %"30" = load i64, ptr addrspace(5) %"10", align 8
  store i64 %"30", ptr addrspace(5) %"49", align 8
  %"31" = load i64, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(1) @fn_ptrs to ptr), i64 8), align 8
  store i64 %"31", ptr addrspace(5) %"11", align 8
  %"18" = load i64, ptr addrspace(5) %"49", align 8
  %"32" = load i64, ptr addrspace(5) %"11", align 8
  %0 = inttoptr i64 %"32" to ptr
  %"19" = call i64 %0(i64 %"18")
  store i64 %"19", ptr addrspace(5) %"50", align 8
  %"33" = load i64, ptr addrspace(5) %"50", align 8
  store i64 %"33", ptr addrspace(5) %"10", align 8
  %"34" = load i64, ptr addrspace(5) %"9", align 8
  %"35" = load i64, ptr addrspace(5) %"10", align 8
  %"58" = inttoptr i64 %"34" to ptr addrspace(1)
  store i64 %"35", ptr addrspace(1) %"58", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
