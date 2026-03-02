declare hidden i32 @__zluda_ptx_impl_sreg_lanemask_le() #0

define amdgpu_kernel void @lanemask_lt(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %2, ptr addrspace(5) %"46", align 8
  %3 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %3, ptr addrspace(5) %"47", align 8
  %4 = load i64, ptr addrspace(5) %"46", align 8
  %"64" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"64", align 4
  store i32 %5, ptr addrspace(5) %"48", align 4
  %6 = load i32, ptr addrspace(5) %"48", align 4
  %"65" = add i32 %6, 1
  store i32 %"65", ptr addrspace(5) %"49", align 4
  %"40" = call i32 @__zluda_ptx_impl_sreg_lanemask_le()
  store i32 %"40", ptr addrspace(5) %"50", align 4
  %7 = load i32, ptr addrspace(5) %"49", align 4
  %8 = load i32, ptr addrspace(5) %"50", align 4
  %"68" = add i32 %7, %8
  store i32 %"68", ptr addrspace(5) %"49", align 4
  %9 = load i64, ptr addrspace(5) %"47", align 8
  %10 = load i32, ptr addrspace(5) %"49", align 4
  %"71" = inttoptr i64 %9 to ptr
  store i32 %10, ptr %"71", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
