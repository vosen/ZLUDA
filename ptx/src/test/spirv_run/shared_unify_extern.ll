target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@shared_ex = external hidden addrspace(3) global [0 x i32]
@shared_mod = private addrspace(3) global [4 x i32] undef

define private i64 @"3"(ptr addrspace(3) %"62", ptr addrspace(3) %"63") #0 {
"59":
  %"4" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"17", align 1
  %"18" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"18", align 1
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"23" = load i64, ptr addrspace(3) %"63", align 8
  store i64 %"23", ptr addrspace(5) %"5", align 8
  %"24" = load i64, ptr addrspace(3) %"62", align 8
  store i64 %"24", ptr addrspace(5) %"6", align 8
  %"26" = load i64, ptr addrspace(5) %"6", align 8
  %"27" = load i64, ptr addrspace(5) %"5", align 8
  %"50" = add i64 %"26", %"27"
  store i64 %"50", ptr addrspace(5) %"4", align 8
  %"28" = load i64, ptr addrspace(5) %"4", align 8
  ret i64 %"28"
}

define private i64 @"7"(i64 %"29", ptr addrspace(3) %"64", ptr addrspace(3) %"65") #0 {
"60":
  %"9" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"19" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"19", align 1
  %"20" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"20", align 1
  store i64 %"29", ptr addrspace(5) %"9", align 8
  %"30" = load i64, ptr addrspace(5) %"9", align 8
  store i64 %"30", ptr addrspace(3) %"64", align 8
  %"31" = call i64 @"3"(ptr addrspace(3) %"64", ptr addrspace(3) %"65")
  store i64 %"31", ptr addrspace(5) %"8", align 8
  %"32" = load i64, ptr addrspace(5) %"8", align 8
  ret i64 %"32"
}

define protected amdgpu_kernel void @shared_unify_extern(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
"61":
  %"21" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"21", align 1
  %"22" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"22", align 1
  %"13" = alloca i64, align 8, addrspace(5)
  %"14" = alloca i64, align 8, addrspace(5)
  %"15" = alloca i64, align 8, addrspace(5)
  %"16" = alloca i64, align 8, addrspace(5)
  %"33" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"33", ptr addrspace(5) %"13", align 8
  %"34" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"34", ptr addrspace(5) %"14", align 8
  %"36" = load i64, ptr addrspace(5) %"13", align 8
  %"53" = inttoptr i64 %"36" to ptr addrspace(1)
  %"35" = load i64, ptr addrspace(1) %"53", align 8
  store i64 %"35", ptr addrspace(5) %"15", align 8
  %"38" = load i64, ptr addrspace(5) %"13", align 8
  %"54" = inttoptr i64 %"38" to ptr addrspace(1)
  %"67" = getelementptr inbounds i8, ptr addrspace(1) %"54", i64 8
  %"37" = load i64, ptr addrspace(1) %"67", align 8
  store i64 %"37", ptr addrspace(5) %"16", align 8
  %"39" = load i64, ptr addrspace(5) %"16", align 8
  store i64 %"39", ptr addrspace(3) @shared_mod, align 8
  %"41" = load i64, ptr addrspace(5) %"15", align 8
  %"56" = call i64 @"7"(i64 %"41", ptr addrspace(3) @shared_ex, ptr addrspace(3) @shared_mod)
  store i64 %"56", ptr addrspace(5) %"16", align 8
  %"42" = load i64, ptr addrspace(5) %"14", align 8
  %"43" = load i64, ptr addrspace(5) %"16", align 8
  %"58" = inttoptr i64 %"42" to ptr
  store i64 %"43", ptr %"58", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
