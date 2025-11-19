declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @tid(ptr addrspace(4) byref(i64) %"39") #1 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i8, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"36", ptr addrspace(5) %"41", align 4
  %2 = load i32, ptr addrspace(5) %"41", align 4
  %3 = zext i32 %2 to i64
  store i64 %3, ptr addrspace(5) %"42", align 8
  %4 = load i32, ptr addrspace(5) %"41", align 4
  %5 = trunc i32 %4 to i8
  store i8 %5, ptr addrspace(5) %"43", align 1
  %6 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %6, ptr addrspace(5) %"40", align 8
  %7 = load i64, ptr addrspace(5) %"40", align 8
  %8 = load i64, ptr addrspace(5) %"42", align 8
  %"50" = add i64 %7, %8
  store i64 %"50", ptr addrspace(5) %"40", align 8
  %9 = load i64, ptr addrspace(5) %"40", align 8
  %10 = load i8, ptr addrspace(5) %"43", align 1
  %"55" = inttoptr i64 %9 to ptr
  store i8 %10, ptr %"55", align 1
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }