declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvt_f64_f32(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca float, align 4, addrspace(5)
  %"40" = alloca double, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"41" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  %"42" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"42", ptr addrspace(5) %"38", align 4
  %"44" = load i64, ptr addrspace(5) %"37", align 4
  %"49" = inttoptr i64 %"44" to ptr addrspace(1)
  %"43" = load float, ptr addrspace(1) %"49", align 4
  store float %"43", ptr addrspace(5) %"39", align 4
  %"46" = load float, ptr addrspace(5) %"39", align 4
  %"45" = fpext float %"46" to double
  store double %"45", ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(5) %"38", align 4
  %"48" = load double, ptr addrspace(5) %"40", align 8
  %"50" = inttoptr i64 %"47" to ptr
  store double %"48", ptr %"50", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
