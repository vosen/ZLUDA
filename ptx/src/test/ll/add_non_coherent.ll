declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @add_non_coherent(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #1 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"53"

"53":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"43", ptr addrspace(5) %"39", align 4
  %"44" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"44", ptr addrspace(5) %"40", align 4
  %"46" = load i64, ptr addrspace(5) %"39", align 4
  %"51" = inttoptr i64 %"46" to ptr addrspace(1)
  %"45" = load i64, ptr addrspace(1) %"51", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"41", align 4
  %"47" = add i64 %"48", 1
  store i64 %"47", ptr addrspace(5) %"42", align 4
  %"49" = load i64, ptr addrspace(5) %"40", align 4
  %"50" = load i64, ptr addrspace(5) %"42", align 4
  %"52" = inttoptr i64 %"49" to ptr addrspace(1)
  store i64 %"50", ptr addrspace(1) %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }