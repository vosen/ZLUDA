declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @malformed_label(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"47" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  br label %"10"

"10":                                             ; preds = %"32"
  %"49" = load i64, ptr addrspace(5) %"42", align 4
  %"54" = inttoptr i64 %"49" to ptr
  %"48" = load i64, ptr %"54", align 4
  store i64 %"48", ptr addrspace(5) %"44", align 4
  %"51" = load i64, ptr addrspace(5) %"44", align 4
  %"50" = add i64 %"51", 1
  store i64 %"50", ptr addrspace(5) %"45", align 4
  %"52" = load i64, ptr addrspace(5) %"43", align 4
  %"53" = load i64, ptr addrspace(5) %"45", align 4
  %"55" = inttoptr i64 %"52" to ptr
  store i64 %"53", ptr %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }