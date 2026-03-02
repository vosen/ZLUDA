@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global [4 x i32]

define hidden i64 @add() #0 {
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %2 = load i64, ptr addrspace(3) @shared_mod, align 8
  store i64 %2, ptr addrspace(5) %"56", align 8
  %3 = load i64, ptr addrspace(3) @shared_ex, align 8
  store i64 %3, ptr addrspace(5) %"57", align 8
  %4 = load i64, ptr addrspace(5) %"57", align 8
  %5 = load i64, ptr addrspace(5) %"56", align 8
  %"84" = add i64 %4, %5
  store i64 %"84", ptr addrspace(5) %"55", align 8
  %6 = load i64, ptr addrspace(5) %"55", align 8
  ret i64 %6
}

define hidden i64 @set_shared_temp1(i64 %"18") #0 {
  %"63" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  store i64 %"18", ptr addrspace(3) @shared_ex, align 8
  %"64" = call i64 @add()
  store i64 %"64", ptr addrspace(5) %"63", align 8
  br label %"52"

"52":                                             ; preds = %"51"
  %2 = load i64, ptr addrspace(5) %"63", align 8
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_extern(ptr addrspace(4) byref(i64) %"65", ptr addrspace(4) byref(i64) %"66") #1 {
  %"67" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"53"

"53":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"65", align 8
  store i64 %2, ptr addrspace(5) %"67", align 8
  %3 = load i64, ptr addrspace(4) %"66", align 8
  store i64 %3, ptr addrspace(5) %"68", align 8
  %4 = load i64, ptr addrspace(5) %"67", align 8
  %"87" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"87", align 8
  store i64 %5, ptr addrspace(5) %"69", align 8
  %6 = load i64, ptr addrspace(5) %"67", align 8
  %"88" = inttoptr i64 %6 to ptr addrspace(1)
  %"49" = getelementptr inbounds i8, ptr addrspace(1) %"88", i64 8
  %7 = load i64, ptr addrspace(1) %"49", align 8
  store i64 %7, ptr addrspace(5) %"70", align 8
  %8 = load i64, ptr addrspace(5) %"70", align 8
  store i64 %8, ptr addrspace(3) @shared_mod, align 8
  %9 = load i64, ptr addrspace(5) %"69", align 8
  %"90" = call i64 @set_shared_temp1(i64 %9)
  store i64 %"90", ptr addrspace(5) %"70", align 8
  br label %"54"

"54":                                             ; preds = %"53"
  %10 = load i64, ptr addrspace(5) %"68", align 8
  %11 = load i64, ptr addrspace(5) %"70", align 8
  %"92" = inttoptr i64 %10 to ptr
  store i64 %11, ptr %"92", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
