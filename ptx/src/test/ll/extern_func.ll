declare hidden [4 x i32] @foobar(i64) #0

define amdgpu_kernel void @extern_func(ptr addrspace(4) byref(i64) %"53", ptr addrspace(4) byref(i64) %"54") #1 {
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"66" = alloca [4 x i32], align 16, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"53", align 8
  store i64 %2, ptr addrspace(5) %"55", align 8
  %3 = load i64, ptr addrspace(4) %"54", align 8
  store i64 %3, ptr addrspace(5) %"56", align 8
  %4 = load i64, ptr addrspace(5) %"55", align 8
  %"70" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"70", align 8
  store i64 %5, ptr addrspace(5) %"57", align 8
  %"64" = getelementptr inbounds i8, ptr addrspace(5) %"63", i64 0
  %6 = load i64, ptr addrspace(5) %"57", align 8
  store i64 %6, ptr addrspace(5) %"64", align 8
  %7 = load i64, ptr addrspace(5) %"63", align 8
  %"49" = call [4 x i32] @foobar(i64 %7)
  br label %"51"

"51":                                             ; preds = %"50"
  store [4 x i32] %"49", ptr addrspace(5) %"66", align 4
  %8 = load i64, ptr addrspace(5) %"66", align 8
  store i64 %8, ptr addrspace(5) %"58", align 8
  %9 = load i64, ptr addrspace(5) %"56", align 8
  %10 = load i64, ptr addrspace(5) %"58", align 8
  %"73" = inttoptr i64 %9 to ptr
  store i64 %10, ptr %"73", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
