declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @ld_st_implicit(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"40" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"40", ptr addrspace(5) %"37", align 4
  %"41" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"41", ptr addrspace(5) %"38", align 4
  store i64 81985529216486895, ptr addrspace(5) %"39", align 4
  %"44" = load i64, ptr addrspace(5) %"37", align 4
  %"48" = inttoptr i64 %"44" to ptr addrspace(1)
  %"47" = load float, ptr addrspace(1) %"48", align 4
  %2 = bitcast float %"47" to i32
  %"43" = zext i32 %2 to i64
  store i64 %"43", ptr addrspace(5) %"39", align 4
  %"45" = load i64, ptr addrspace(5) %"38", align 4
  %"46" = load i64, ptr addrspace(5) %"39", align 4
  %"49" = inttoptr i64 %"45" to ptr addrspace(1)
  %3 = trunc i64 %"46" to i32
  %"50" = bitcast i32 %3 to float
  store float %"50", ptr addrspace(1) %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
