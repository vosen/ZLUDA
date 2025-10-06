@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global i64, align 4

define hidden i64 @add(i64 %"12") #0 {
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  store i64 %"12", ptr addrspace(3) @shared_mod, align 8
  %2 = load i64, ptr addrspace(3) @shared_mod, align 8
  store i64 %2, ptr addrspace(5) %"54", align 8
  %3 = load i64, ptr addrspace(3) @shared_ex, align 8
  %4 = load i64, ptr addrspace(5) %"54", align 8
  %"78" = add i64 %3, %4
  store i64 %"78", ptr addrspace(5) %"53", align 8
  %5 = load i64, ptr addrspace(5) %"53", align 8
  ret i64 %5
}

define hidden i64 @set_shared_temp1(i64 %"17", i64 %"18") #0 {
  %"58" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"49"

"49":                                             ; preds = %1
  store i64 %"17", ptr addrspace(3) @shared_ex, align 8
  %"59" = call i64 @add(i64 %"18")
  store i64 %"59", ptr addrspace(5) %"58", align 8
  br label %"50"

"50":                                             ; preds = %"49"
  %2 = load i64, ptr addrspace(5) %"58", align 8
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"60", ptr addrspace(4) byref(i64) %"61") #1 {
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"60", align 8
  store i64 %2, ptr addrspace(5) %"62", align 8
  %3 = load i64, ptr addrspace(4) %"61", align 8
  store i64 %3, ptr addrspace(5) %"63", align 8
  %4 = load i64, ptr addrspace(5) %"62", align 8
  %"81" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"81", align 8
  store i64 %5, ptr addrspace(5) %"64", align 8
  %6 = load i64, ptr addrspace(5) %"62", align 8
  %"82" = inttoptr i64 %6 to ptr addrspace(1)
  %"47" = getelementptr inbounds i8, ptr addrspace(1) %"82", i64 8
  %7 = load i64, ptr addrspace(1) %"47", align 8
  store i64 %7, ptr addrspace(5) %"65", align 8
  %8 = load i64, ptr addrspace(5) %"64", align 8
  %9 = load i64, ptr addrspace(5) %"65", align 8
  %"83" = call i64 @set_shared_temp1(i64 %8, i64 %9)
  store i64 %"83", ptr addrspace(5) %"65", align 8
  br label %"52"

"52":                                             ; preds = %"51"
  %10 = load i64, ptr addrspace(5) %"63", align 8
  %11 = load i64, ptr addrspace(5) %"65", align 8
  %"85" = inttoptr i64 %10 to ptr
  store i64 %11, ptr %"85", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }