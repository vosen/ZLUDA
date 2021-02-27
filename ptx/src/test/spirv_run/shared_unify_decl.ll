target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@shared_ex = external hidden addrspace(3) global [0 x i32]
@shared_mod = private addrspace(3) global [4 x i32] undef

define private i64 @"3"(ptr addrspace(3) %"69", ptr addrspace(3) %"70") #0 {
"62":
  %"8" = alloca i64, align 8, addrspace(5)
  %"20" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"20", align 1
  %"21" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"21", align 1
  %"9" = alloca i64, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"26" = load i64, ptr addrspace(3) %"70", align 8
  store i64 %"26", ptr addrspace(5) %"9", align 8
  %"27" = load i64, ptr addrspace(3) %"69", align 8
  store i64 %"27", ptr addrspace(5) %"10", align 8
  %"29" = load i64, ptr addrspace(5) %"10", align 8
  %"30" = load i64, ptr addrspace(5) %"9", align 8
  %"53" = add i64 %"29", %"30"
  store i64 %"53", ptr addrspace(5) %"8", align 8
  %"31" = load i64, ptr addrspace(5) %"8", align 8
  ret i64 %"31"
}

define private i64 @"5"(i64 %"32", ptr addrspace(3) %"71", ptr addrspace(3) %"72") #0 {
"63":
  %"12" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i64, align 8, addrspace(5)
  %"22" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"22", align 1
  %"23" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"23", align 1
  store i64 %"32", ptr addrspace(5) %"12", align 8
  %"33" = load i64, ptr addrspace(5) %"12", align 8
  store i64 %"33", ptr addrspace(3) %"71", align 8
  %"34" = call i64 @"3"(ptr addrspace(3) %"71", ptr addrspace(3) %"72")
  store i64 %"34", ptr addrspace(5) %"11", align 8
  %"35" = load i64, ptr addrspace(5) %"11", align 8
  ret i64 %"35"
}

define protected amdgpu_kernel void @shared_unify_decl(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
"64":
  %"24" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"24", align 1
  %"25" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"25", align 1
  %"16" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i64, align 8, addrspace(5)
  %"18" = alloca i64, align 8, addrspace(5)
  %"19" = alloca i64, align 8, addrspace(5)
  %"36" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"36", ptr addrspace(5) %"16", align 8
  %"37" = load i64, ptr addrspace(4) %"50", align 8
  store i64 %"37", ptr addrspace(5) %"17", align 8
  %"39" = load i64, ptr addrspace(5) %"16", align 8
  %"56" = inttoptr i64 %"39" to ptr addrspace(1)
  %"38" = load i64, ptr addrspace(1) %"56", align 8
  store i64 %"38", ptr addrspace(5) %"18", align 8
  %"41" = load i64, ptr addrspace(5) %"16", align 8
  %"57" = inttoptr i64 %"41" to ptr addrspace(1)
  %"74" = getelementptr inbounds i8, ptr addrspace(1) %"57", i64 8
  %"40" = load i64, ptr addrspace(1) %"74", align 8
  store i64 %"40", ptr addrspace(5) %"19", align 8
  %"42" = load i64, ptr addrspace(5) %"19", align 8
  store i64 %"42", ptr addrspace(3) @shared_mod, align 8
  %"44" = load i64, ptr addrspace(5) %"18", align 8
  %"59" = call i64 @"5"(i64 %"44", ptr addrspace(3) @shared_ex, ptr addrspace(3) @shared_mod)
  store i64 %"59", ptr addrspace(5) %"19", align 8
  %"45" = load i64, ptr addrspace(5) %"17", align 8
  %"46" = load i64, ptr addrspace(5) %"19", align 8
  %"61" = inttoptr i64 %"45" to ptr
  store i64 %"46", ptr %"61", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
