declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @tid(ptr addrspace(4) byref(i64) %"37") #1 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i8, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"33" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"35"

"35":                                             ; preds = %"34"
  store i32 %"33", ptr addrspace(5) %"39", align 4
  %"44" = load i32, ptr addrspace(5) %"39", align 4
  %"43" = zext i32 %"44" to i64
  store i64 %"43", ptr addrspace(5) %"40", align 8
  %"46" = load i32, ptr addrspace(5) %"39", align 4
  %"45" = trunc i32 %"46" to i8
  store i8 %"45", ptr addrspace(5) %"41", align 1
  %"47" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"47", ptr addrspace(5) %"38", align 8
  %"49" = load i64, ptr addrspace(5) %"38", align 8
  %"50" = load i64, ptr addrspace(5) %"40", align 8
  %"48" = add i64 %"49", %"50"
  store i64 %"48", ptr addrspace(5) %"38", align 8
  %"51" = load i64, ptr addrspace(5) %"38", align 8
  %"52" = load i8, ptr addrspace(5) %"41", align 1
  %"53" = inttoptr i64 %"51" to ptr
  store i8 %"52", ptr %"53", align 1
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }