declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @tid(ptr addrspace(4) byref(i64) %"37") #1 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i8, align 1, addrspace(5)
  %"42" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"42", align 1
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"43" = load i8, ptr addrspace(5) %"42", align 1
  %"33" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"43")
  br label %"35"

"35":                                             ; preds = %"34"
  store i32 %"33", ptr addrspace(5) %"39", align 4
  %"46" = load i32, ptr addrspace(5) %"39", align 4
  %"45" = zext i32 %"46" to i64
  store i64 %"45", ptr addrspace(5) %"40", align 8
  %"48" = load i32, ptr addrspace(5) %"39", align 4
  %"47" = trunc i32 %"48" to i8
  store i8 %"47", ptr addrspace(5) %"41", align 1
  %"49" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"49", ptr addrspace(5) %"38", align 8
  %"51" = load i64, ptr addrspace(5) %"38", align 8
  %"52" = load i64, ptr addrspace(5) %"40", align 8
  %"50" = add i64 %"51", %"52"
  store i64 %"50", ptr addrspace(5) %"38", align 8
  %"53" = load i64, ptr addrspace(5) %"38", align 8
  %"54" = load i8, ptr addrspace(5) %"41", align 1
  %"55" = inttoptr i64 %"53" to ptr
  store i8 %"54", ptr %"55", align 1
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }