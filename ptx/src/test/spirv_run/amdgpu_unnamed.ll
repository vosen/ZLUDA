target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@0 = protected addrspace(1) externally_initialized global [2 x i8] c"v\00", align 1
@1 = protected addrspace(1) externally_initialized global [2 x i8] c"*\00", align 1
@2 = protected addrspace(1) externally_initialized global [2 x i8] c"s\00", align 1

declare void @__zluda_ptx_impl____assertfail(i64, i64, i32, i64, i64) #0

define protected amdgpu_kernel void @amdgpu_unnamed(ptr addrspace(4) byref(i64) %"57", ptr addrspace(4) byref(i64) %"58") #1 {
"73":
  %"33" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"33", align 1
  %"14" = alloca i64, align 8, addrspace(5)
  %"15" = alloca i64, align 8, addrspace(5)
  %"16" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i64, align 8, addrspace(5)
  %"18" = alloca i1, align 1, addrspace(5)
  %"19" = alloca i64, align 8, addrspace(5)
  %"20" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i32, align 4, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"34" = load i64, ptr addrspace(4) %"57", align 8
  store i64 %"34", ptr addrspace(5) %"14", align 8
  %"35" = load i64, ptr addrspace(4) %"58", align 8
  store i64 %"35", ptr addrspace(5) %"15", align 8
  %"37" = load i64, ptr addrspace(5) %"14", align 8
  %"65" = inttoptr i64 %"37" to ptr
  %"36" = load i64, ptr %"65", align 8
  store i64 %"36", ptr addrspace(5) %"16", align 8
  %"39" = load i64, ptr addrspace(5) %"16", align 8
  %"38" = icmp uge i64 %"39", 1
  store i1 %"38", ptr addrspace(5) %"18", align 1
  %"40" = load i1, ptr addrspace(5) %"18", align 1
  br i1 %"40", label %"13", label %"27"

"27":                                             ; preds = %"73"
  %0 = alloca i64, align 8, addrspace(5)
  store i64 ptrtoint (ptr addrspace(1) @0 to i64), ptr addrspace(5) %0, align 8
  %"66" = load i64, ptr addrspace(5) %0, align 8
  store i64 %"66", ptr addrspace(5) %"19", align 8
  %"42" = load i64, ptr addrspace(5) %"19", align 8
  store i64 %"42", ptr addrspace(5) %"59", align 8
  %1 = alloca i64, align 8, addrspace(5)
  store i64 ptrtoint (ptr addrspace(1) @1 to i64), ptr addrspace(5) %1, align 8
  %"68" = load i64, ptr addrspace(5) %1, align 8
  store i64 %"68", ptr addrspace(5) %"19", align 8
  %"44" = load i64, ptr addrspace(5) %"19", align 8
  store i64 %"44", ptr addrspace(5) %"60", align 8
  store i32 1, ptr addrspace(5) %"61", align 4
  %2 = alloca i64, align 8, addrspace(5)
  store i64 ptrtoint (ptr addrspace(1) @2 to i64), ptr addrspace(5) %2, align 8
  %"70" = load i64, ptr addrspace(5) %2, align 8
  store i64 %"70", ptr addrspace(5) %"19", align 8
  %"46" = load i64, ptr addrspace(5) %"19", align 8
  store i64 %"46", ptr addrspace(5) %"62", align 8
  %"75" = getelementptr inbounds i8, ptr addrspace(5) %"63", i64 0
  store i64 1, ptr addrspace(5) %"75", align 8
  %"28" = load i64, ptr addrspace(5) %"59", align 8
  %"29" = load i64, ptr addrspace(5) %"60", align 8
  %"30" = load i32, ptr addrspace(5) %"61", align 4
  %"31" = load i64, ptr addrspace(5) %"62", align 8
  %"32" = load i64, ptr addrspace(5) %"63", align 8
  call void @__zluda_ptx_impl____assertfail(i64 %"28", i64 %"29", i32 %"30", i64 %"31", i64 %"32")
  br label %"13"

"13":                                             ; preds = %"27", %"73"
  %"48" = load i64, ptr addrspace(5) %"16", align 8
  %"47" = add i64 %"48", 1
  store i64 %"47", ptr addrspace(5) %"17", align 8
  %"49" = load i64, ptr addrspace(5) %"15", align 8
  %"50" = load i64, ptr addrspace(5) %"17", align 8
  %"72" = inttoptr i64 %"49" to ptr
  store i64 %"50", ptr %"72", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
