declare hidden i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @lanemask_lt(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %2, ptr addrspace(5) %"43", align 8
  %3 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %3, ptr addrspace(5) %"44", align 8
  %4 = load i64, ptr addrspace(5) %"43", align 8
  %"61" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"61", align 4
  store i32 %5, ptr addrspace(5) %"45", align 4
  %6 = load i32, ptr addrspace(5) %"45", align 4
  %"62" = add i32 %6, 1
  store i32 %"62", ptr addrspace(5) %"46", align 4
  %"37" = call i32 @__zluda_ptx_impl_sreg_lanemask_lt()
  store i32 %"37", ptr addrspace(5) %"47", align 4
  %7 = load i32, ptr addrspace(5) %"46", align 4
  %8 = load i32, ptr addrspace(5) %"47", align 4
  %"65" = add i32 %7, %8
  store i32 %"65", ptr addrspace(5) %"46", align 4
  %9 = load i64, ptr addrspace(5) %"44", align 8
  %10 = load i32, ptr addrspace(5) %"46", align 4
  %"68" = inttoptr i64 %9 to ptr
  store i32 %10, ptr %"68", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }