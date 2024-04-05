target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@shared_ex = external hidden addrspace(3) global [0 x i32]
@"5" = private addrspace(3) global i64 undef, align 4

define private i64 @"2"(i64 %"21", ptr addrspace(3) %"62", ptr addrspace(3) %"63") #0 {
"59":
  %"4" = alloca i64, align 8, addrspace(5)
  %"3" = alloca i64, align 8, addrspace(5)
  %"18" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"18", align 1
  %"6" = alloca i64, align 8, addrspace(5)
  store i64 %"21", ptr addrspace(5) %"4", align 8
  %"22" = load i64, ptr addrspace(5) %"4", align 8
  store i64 %"22", ptr addrspace(3) %"63", align 8
  %"23" = load i64, ptr addrspace(3) %"63", align 8
  store i64 %"23", ptr addrspace(5) %"6", align 8
  %"24" = load i64, ptr addrspace(3) %"62", align 8
  store i64 %"24", ptr addrspace(5) %"4", align 8
  %"26" = load i64, ptr addrspace(5) %"4", align 8
  %"27" = load i64, ptr addrspace(5) %"6", align 8
  %"51" = add i64 %"26", %"27"
  store i64 %"51", ptr addrspace(5) %"3", align 8
  %"28" = load i64, ptr addrspace(5) %"3", align 8
  ret i64 %"28"
}

define private i64 @"7"(i64 %"29", i64 %"30", ptr addrspace(3) %"64", ptr addrspace(3) %"65") #0 {
"60":
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"19" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"19", align 1
  store i64 %"29", ptr addrspace(5) %"9", align 8
  store i64 %"30", ptr addrspace(5) %"10", align 8
  %"31" = load i64, ptr addrspace(5) %"9", align 8
  store i64 %"31", ptr addrspace(3) %"64", align 8
  %"33" = load i64, ptr addrspace(5) %"10", align 8
  %"32" = call i64 @"2"(i64 %"33", ptr addrspace(3) %"64", ptr addrspace(3) %"65")
  store i64 %"32", ptr addrspace(5) %"8", align 8
  %"34" = load i64, ptr addrspace(5) %"8", align 8
  ret i64 %"34"
}

define protected amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"48", ptr addrspace(4) byref(i64) %"49") #0 {
"61":
  %"20" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"20", align 1
  %"14" = alloca i64, align 8, addrspace(5)
  %"15" = alloca i64, align 8, addrspace(5)
  %"16" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i64, align 8, addrspace(5)
  %"35" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"35", ptr addrspace(5) %"14", align 8
  %"36" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"36", ptr addrspace(5) %"15", align 8
  %"38" = load i64, ptr addrspace(5) %"14", align 8
  %"54" = inttoptr i64 %"38" to ptr addrspace(1)
  %"37" = load i64, ptr addrspace(1) %"54", align 8
  store i64 %"37", ptr addrspace(5) %"16", align 8
  %"40" = load i64, ptr addrspace(5) %"14", align 8
  %"55" = inttoptr i64 %"40" to ptr addrspace(1)
  %"67" = getelementptr inbounds i8, ptr addrspace(1) %"55", i64 8
  %"39" = load i64, ptr addrspace(1) %"67", align 8
  store i64 %"39", ptr addrspace(5) %"17", align 8
  %"42" = load i64, ptr addrspace(5) %"16", align 8
  %"43" = load i64, ptr addrspace(5) %"17", align 8
  %"56" = call i64 @"7"(i64 %"42", i64 %"43", ptr addrspace(3) @shared_ex, ptr addrspace(3) @"5")
  store i64 %"56", ptr addrspace(5) %"17", align 8
  %"44" = load i64, ptr addrspace(5) %"15", align 8
  %"45" = load i64, ptr addrspace(5) %"17", align 8
  %"58" = inttoptr i64 %"44" to ptr
  store i64 %"45", ptr %"58", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
