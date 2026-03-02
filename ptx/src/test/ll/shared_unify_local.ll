@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global i64, align 4

define hidden i64 @add(i64 %"13") #0 {
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  store i64 %"13", ptr addrspace(3) @shared_mod, align 8
  %2 = load i64, ptr addrspace(3) @shared_mod, align 8
  store i64 %2, ptr addrspace(5) %"57", align 8
  %3 = load i64, ptr addrspace(3) @shared_ex, align 8
  %4 = load i64, ptr addrspace(5) %"57", align 8
  %"81" = add i64 %3, %4
  store i64 %"81", ptr addrspace(5) %"56", align 8
  %5 = load i64, ptr addrspace(5) %"56", align 8
  ret i64 %5
}

define hidden i64 @set_shared_temp1(i64 %"18", i64 %"19") #0 {
  %"61" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"52"

"52":                                             ; preds = %1
  store i64 %"18", ptr addrspace(3) @shared_ex, align 8
  %"62" = call i64 @add(i64 %"19")
  store i64 %"62", ptr addrspace(5) %"61", align 8
  br label %"53"

"53":                                             ; preds = %"52"
  %2 = load i64, ptr addrspace(5) %"61", align 8
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"63", ptr addrspace(4) byref(i64) %"64") #1 {
  %"65" = alloca i64, align 8, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"54"

"54":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"63", align 8
  store i64 %2, ptr addrspace(5) %"65", align 8
  %3 = load i64, ptr addrspace(4) %"64", align 8
  store i64 %3, ptr addrspace(5) %"66", align 8
  %4 = load i64, ptr addrspace(5) %"65", align 8
  %"84" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"84", align 8
  store i64 %5, ptr addrspace(5) %"67", align 8
  %6 = load i64, ptr addrspace(5) %"65", align 8
  %"85" = inttoptr i64 %6 to ptr addrspace(1)
  %"50" = getelementptr inbounds i8, ptr addrspace(1) %"85", i64 8
  %7 = load i64, ptr addrspace(1) %"50", align 8
  store i64 %7, ptr addrspace(5) %"68", align 8
  %8 = load i64, ptr addrspace(5) %"67", align 8
  %9 = load i64, ptr addrspace(5) %"68", align 8
  %"86" = call i64 @set_shared_temp1(i64 %8, i64 %9)
  store i64 %"86", ptr addrspace(5) %"68", align 8
  br label %"55"

"55":                                             ; preds = %"54"
  %10 = load i64, ptr addrspace(5) %"66", align 8
  %11 = load i64, ptr addrspace(5) %"68", align 8
  %"88" = inttoptr i64 %10 to ptr
  store i64 %11, ptr %"88", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
