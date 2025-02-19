declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvta(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"39" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"39", ptr addrspace(5) %"36", align 4
  %"40" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"40", ptr addrspace(5) %"37", align 4
  %"42" = load i64, ptr addrspace(5) %"36", align 4
  %2 = inttoptr i64 %"42" to ptr
  %"49" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"49", ptr addrspace(5) %"36", align 8
  %"44" = load i64, ptr addrspace(5) %"37", align 4
  %3 = inttoptr i64 %"44" to ptr
  %"51" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"51", ptr addrspace(5) %"37", align 8
  %"46" = load i64, ptr addrspace(5) %"36", align 4
  %"53" = inttoptr i64 %"46" to ptr addrspace(1)
  %"45" = load float, ptr addrspace(1) %"53", align 4
  store float %"45", ptr addrspace(5) %"38", align 4
  %"47" = load i64, ptr addrspace(5) %"37", align 4
  %"48" = load float, ptr addrspace(5) %"38", align 4
  %"54" = inttoptr i64 %"47" to ptr addrspace(1)
  store float %"48", ptr addrspace(1) %"54", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
