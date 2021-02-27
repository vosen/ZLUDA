target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@shared_ex = external hidden addrspace(3) global [0 x i32]
@"5" = private addrspace(3) global i64 undef, align 4

define private i64 @"2"(i64 %"24", ptr addrspace(3) %"65", ptr addrspace(3) %"66") #0 {
"62":
  %"4" = alloca i64, align 8, addrspace(5)
  %"3" = alloca i64, align 8, addrspace(5)
  %"18" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"18", align 1
  %"19" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"19", align 1
  %"6" = alloca i64, align 8, addrspace(5)
  store i64 %"24", ptr addrspace(5) %"4", align 8
  %"25" = load i64, ptr addrspace(5) %"4", align 8
  store i64 %"25", ptr addrspace(3) %"66", align 8
  %"26" = load i64, ptr addrspace(3) %"66", align 8
  store i64 %"26", ptr addrspace(5) %"6", align 8
  %"27" = load i64, ptr addrspace(3) %"65", align 8
  store i64 %"27", ptr addrspace(5) %"4", align 8
  %"29" = load i64, ptr addrspace(5) %"4", align 8
  %"30" = load i64, ptr addrspace(5) %"6", align 8
  %"54" = add i64 %"29", %"30"
  store i64 %"54", ptr addrspace(5) %"3", align 8
  %"31" = load i64, ptr addrspace(5) %"3", align 8
  ret i64 %"31"
}

define private i64 @"7"(i64 %"32", i64 %"33", ptr addrspace(3) %"67", ptr addrspace(3) %"68") #0 {
"63":
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"20" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"20", align 1
  %"21" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"21", align 1
  store i64 %"32", ptr addrspace(5) %"9", align 8
  store i64 %"33", ptr addrspace(5) %"10", align 8
  %"34" = load i64, ptr addrspace(5) %"9", align 8
  store i64 %"34", ptr addrspace(3) %"67", align 8
  %"36" = load i64, ptr addrspace(5) %"10", align 8
  %"35" = call i64 @"2"(i64 %"36", ptr addrspace(3) %"67", ptr addrspace(3) %"68")
  store i64 %"35", ptr addrspace(5) %"8", align 8
  %"37" = load i64, ptr addrspace(5) %"8", align 8
  ret i64 %"37"
}

define protected amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"51", ptr addrspace(4) byref(i64) %"52") #0 {
"64":
  %"22" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"22", align 1
  %"23" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"23", align 1
  %"14" = alloca i64, align 8, addrspace(5)
  %"15" = alloca i64, align 8, addrspace(5)
  %"16" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i64, align 8, addrspace(5)
  %"38" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"38", ptr addrspace(5) %"14", align 8
  %"39" = load i64, ptr addrspace(4) %"52", align 8
  store i64 %"39", ptr addrspace(5) %"15", align 8
  %"41" = load i64, ptr addrspace(5) %"14", align 8
  %"57" = inttoptr i64 %"41" to ptr addrspace(1)
  %"40" = load i64, ptr addrspace(1) %"57", align 8
  store i64 %"40", ptr addrspace(5) %"16", align 8
  %"43" = load i64, ptr addrspace(5) %"14", align 8
  %"58" = inttoptr i64 %"43" to ptr addrspace(1)
  %"70" = getelementptr inbounds i8, ptr addrspace(1) %"58", i64 8
  %"42" = load i64, ptr addrspace(1) %"70", align 8
  store i64 %"42", ptr addrspace(5) %"17", align 8
  %"45" = load i64, ptr addrspace(5) %"16", align 8
  %"46" = load i64, ptr addrspace(5) %"17", align 8
  %"59" = call i64 @"7"(i64 %"45", i64 %"46", ptr addrspace(3) @shared_ex, ptr addrspace(3) @"5")
  store i64 %"59", ptr addrspace(5) %"17", align 8
  %"47" = load i64, ptr addrspace(5) %"15", align 8
  %"48" = load i64, ptr addrspace(5) %"17", align 8
  %"61" = inttoptr i64 %"47" to ptr
  store i64 %"48", ptr %"61", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
