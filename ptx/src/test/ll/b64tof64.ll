declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @b64tof64(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca double, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"41" = load double, ptr addrspace(4) %"35", align 8
  store double %"41", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"42", ptr addrspace(5) %"39", align 4
  %"44" = load double, ptr addrspace(5) %"37", align 8
  %"50" = bitcast double %"44" to i64
  store i64 %"50", ptr addrspace(5) %"38", align 4
  %"46" = load i64, ptr addrspace(5) %"38", align 4
  %"51" = inttoptr i64 %"46" to ptr
  %"45" = load i64, ptr %"51", align 4
  store i64 %"45", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(5) %"39", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"52" = inttoptr i64 %"47" to ptr
  store i64 %"48", ptr %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
