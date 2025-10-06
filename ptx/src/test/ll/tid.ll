declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @tid(ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i8, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"38"

"38":                                             ; preds = %"37"
  store i32 %"36", ptr addrspace(5) %"42", align 4
  %2 = load i32, ptr addrspace(5) %"42", align 4
  %"46" = zext i32 %2 to i64
  store i64 %"46", ptr addrspace(5) %"43", align 8
  %3 = load i32, ptr addrspace(5) %"42", align 4
  %"48" = trunc i32 %3 to i8
  store i8 %"48", ptr addrspace(5) %"44", align 1
  %4 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %4, ptr addrspace(5) %"41", align 8
  %5 = load i64, ptr addrspace(5) %"41", align 8
  %6 = load i64, ptr addrspace(5) %"43", align 8
  %"51" = add i64 %5, %6
  store i64 %"51", ptr addrspace(5) %"41", align 8
  %7 = load i64, ptr addrspace(5) %"41", align 8
  %8 = load i8, ptr addrspace(5) %"44", align 1
  %"56" = inttoptr i64 %7 to ptr
  store i8 %8, ptr %"56", align 1
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }