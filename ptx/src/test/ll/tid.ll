declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @tid(ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i8, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"39" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"39", ptr addrspace(5) %"44", align 4
  %2 = load i32, ptr addrspace(5) %"44", align 4
  %3 = zext i32 %2 to i64
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i32, ptr addrspace(5) %"44", align 4
  %5 = trunc i32 %4 to i8
  store i8 %5, ptr addrspace(5) %"46", align 1
  %6 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %6, ptr addrspace(5) %"43", align 8
  %7 = load i64, ptr addrspace(5) %"43", align 8
  %8 = load i64, ptr addrspace(5) %"45", align 8
  %"53" = add i64 %7, %8
  store i64 %"53", ptr addrspace(5) %"43", align 8
  %9 = load i64, ptr addrspace(5) %"43", align 8
  %10 = load i8, ptr addrspace(5) %"46", align 1
  %"58" = inttoptr i64 %9 to ptr
  store i8 %10, ptr %"58", align 1
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
