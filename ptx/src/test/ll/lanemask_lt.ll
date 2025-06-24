declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @lanemask_lt(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #1 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"43", ptr addrspace(5) %"38", align 4
  %"44" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"44", ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(5) %"38", align 4
  %"56" = inttoptr i64 %"46" to ptr
  %"55" = load i32, ptr %"56", align 4
  store i32 %"55", ptr addrspace(5) %"40", align 4
  %"48" = load i32, ptr addrspace(5) %"40", align 4
  %"57" = add i32 %"48", 1
  store i32 %"57", ptr addrspace(5) %"41", align 4
  %"31" = call i32 @__zluda_ptx_impl_sreg_lanemask_lt()
  br label %"34"

"34":                                             ; preds = %"33"
  store i32 %"31", ptr addrspace(5) %"42", align 4
  %"51" = load i32, ptr addrspace(5) %"41", align 4
  %"52" = load i32, ptr addrspace(5) %"42", align 4
  %"60" = add i32 %"51", %"52"
  store i32 %"60", ptr addrspace(5) %"41", align 4
  %"53" = load i64, ptr addrspace(5) %"39", align 4
  %"54" = load i32, ptr addrspace(5) %"41", align 4
  %"63" = inttoptr i64 %"53" to ptr
  store i32 %"54", ptr %"63", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }