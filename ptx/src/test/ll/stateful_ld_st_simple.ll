declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_simple(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"43" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"43", ptr addrspace(5) %"38", align 4
  %"44" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"44", ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(5) %"38", align 4
  %2 = inttoptr i64 %"46" to ptr
  %"53" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"53", ptr addrspace(5) %"40", align 8
  %"48" = load i64, ptr addrspace(5) %"39", align 4
  %3 = inttoptr i64 %"48" to ptr
  %"55" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"55", ptr addrspace(5) %"41", align 8
  %"50" = load i64, ptr addrspace(5) %"40", align 4
  %"57" = inttoptr i64 %"50" to ptr addrspace(1)
  %"49" = load i64, ptr addrspace(1) %"57", align 4
  store i64 %"49", ptr addrspace(5) %"42", align 4
  %"51" = load i64, ptr addrspace(5) %"41", align 4
  %"52" = load i64, ptr addrspace(5) %"42", align 4
  %"58" = inttoptr i64 %"51" to ptr addrspace(1)
  store i64 %"52", ptr addrspace(1) %"58", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
