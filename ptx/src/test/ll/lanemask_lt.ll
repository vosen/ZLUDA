declare hidden i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @lanemask_lt(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #1 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %2, ptr addrspace(5) %"44", align 8
  %3 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"44", align 8
  %"62" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"62", align 4
  store i32 %5, ptr addrspace(5) %"46", align 4
  %6 = load i32, ptr addrspace(5) %"46", align 4
  %"63" = add i32 %6, 1
  store i32 %"63", ptr addrspace(5) %"47", align 4
  %"37" = call i32 @__zluda_ptx_impl_sreg_lanemask_lt()
  br label %"40"

"40":                                             ; preds = %"39"
  store i32 %"37", ptr addrspace(5) %"48", align 4
  %7 = load i32, ptr addrspace(5) %"47", align 4
  %8 = load i32, ptr addrspace(5) %"48", align 4
  %"66" = add i32 %7, %8
  store i32 %"66", ptr addrspace(5) %"47", align 4
  %9 = load i64, ptr addrspace(5) %"45", align 8
  %10 = load i32, ptr addrspace(5) %"47", align 4
  %"69" = inttoptr i64 %9 to ptr
  store i32 %10, ptr %"69", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }