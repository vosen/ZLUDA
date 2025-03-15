declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @bra(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #1 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"48", ptr addrspace(5) %"44", align 4
  %"49" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"49", ptr addrspace(5) %"45", align 4
  %"51" = load i64, ptr addrspace(5) %"44", align 4
  %"56" = inttoptr i64 %"51" to ptr
  %"50" = load i64, ptr %"56", align 4
  store i64 %"50", ptr addrspace(5) %"46", align 4
  br label %"10"

"10":                                             ; preds = %"35"
  %"53" = load i64, ptr addrspace(5) %"46", align 4
  %"52" = add i64 %"53", 1
  store i64 %"52", ptr addrspace(5) %"47", align 4
  br label %"12"

"12":                                             ; preds = %"10"
  %"54" = load i64, ptr addrspace(5) %"45", align 4
  %"55" = load i64, ptr addrspace(5) %"47", align 4
  %"57" = inttoptr i64 %"54" to ptr
  store i64 %"55", ptr %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }