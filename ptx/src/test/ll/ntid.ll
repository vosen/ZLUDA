declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @ntid(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"47" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"42", align 4
  %"56" = inttoptr i64 %"49" to ptr
  %"48" = load i32, ptr %"56", align 4
  store i32 %"48", ptr addrspace(5) %"44", align 4
  %"31" = call i32 @__zluda_ptx_impl_sreg_ntid(i8 0)
  br label %"33"

"33":                                             ; preds = %"32"
  store i32 %"31", ptr addrspace(5) %"45", align 4
  %"52" = load i32, ptr addrspace(5) %"44", align 4
  %"53" = load i32, ptr addrspace(5) %"45", align 4
  %"51" = add i32 %"52", %"53"
  store i32 %"51", ptr addrspace(5) %"44", align 4
  %"54" = load i64, ptr addrspace(5) %"43", align 4
  %"55" = load i32, ptr addrspace(5) %"44", align 4
  %"57" = inttoptr i64 %"54" to ptr
  store i32 %"55", ptr %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }