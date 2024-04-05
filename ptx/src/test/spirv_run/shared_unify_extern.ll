target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@shared_ex = external hidden addrspace(3) global [0 x i32]
@shared_mod = private addrspace(3) global [4 x i32] undef

define private i64 @"3"(ptr addrspace(3) %"59", ptr addrspace(3) %"60") #0 {
"56":
  %"4" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"17", align 1
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"20" = load i64, ptr addrspace(3) %"60", align 8
  store i64 %"20", ptr addrspace(5) %"5", align 8
  %"21" = load i64, ptr addrspace(3) %"59", align 8
  store i64 %"21", ptr addrspace(5) %"6", align 8
  %"23" = load i64, ptr addrspace(5) %"6", align 8
  %"24" = load i64, ptr addrspace(5) %"5", align 8
  %"47" = add i64 %"23", %"24"
  store i64 %"47", ptr addrspace(5) %"4", align 8
  %"25" = load i64, ptr addrspace(5) %"4", align 8
  ret i64 %"25"
}

define private i64 @"7"(i64 %"26", ptr addrspace(3) %"61", ptr addrspace(3) %"62") #0 {
"57":
  %"9" = alloca i64, align 8, addrspace(5)
  %"8" = alloca i64, align 8, addrspace(5)
  %"18" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"18", align 1
  store i64 %"26", ptr addrspace(5) %"9", align 8
  %"27" = load i64, ptr addrspace(5) %"9", align 8
  store i64 %"27", ptr addrspace(3) %"61", align 8
  %"28" = call i64 @"3"(ptr addrspace(3) %"61", ptr addrspace(3) %"62")
  store i64 %"28", ptr addrspace(5) %"8", align 8
  %"29" = load i64, ptr addrspace(5) %"8", align 8
  ret i64 %"29"
}

define protected amdgpu_kernel void @shared_unify_extern(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
"58":
  %"19" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"19", align 1
  %"13" = alloca i64, align 8, addrspace(5)
  %"14" = alloca i64, align 8, addrspace(5)
  %"15" = alloca i64, align 8, addrspace(5)
  %"16" = alloca i64, align 8, addrspace(5)
  %"30" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"30", ptr addrspace(5) %"13", align 8
  %"31" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"31", ptr addrspace(5) %"14", align 8
  %"33" = load i64, ptr addrspace(5) %"13", align 8
  %"50" = inttoptr i64 %"33" to ptr addrspace(1)
  %"32" = load i64, ptr addrspace(1) %"50", align 8
  store i64 %"32", ptr addrspace(5) %"15", align 8
  %"35" = load i64, ptr addrspace(5) %"13", align 8
  %"51" = inttoptr i64 %"35" to ptr addrspace(1)
  %"64" = getelementptr inbounds i8, ptr addrspace(1) %"51", i64 8
  %"34" = load i64, ptr addrspace(1) %"64", align 8
  store i64 %"34", ptr addrspace(5) %"16", align 8
  %"36" = load i64, ptr addrspace(5) %"16", align 8
  store i64 %"36", ptr addrspace(3) @shared_mod, align 8
  %"38" = load i64, ptr addrspace(5) %"15", align 8
  %"53" = call i64 @"7"(i64 %"38", ptr addrspace(3) @shared_ex, ptr addrspace(3) @shared_mod)
  store i64 %"53", ptr addrspace(5) %"16", align 8
  %"39" = load i64, ptr addrspace(5) %"14", align 8
  %"40" = load i64, ptr addrspace(5) %"16", align 8
  %"55" = inttoptr i64 %"39" to ptr
  store i64 %"40", ptr %"55", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
