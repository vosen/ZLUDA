declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_ntid_chain(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"72" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"72", ptr addrspace(5) %"44", align 4
  %"73" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"73", ptr addrspace(5) %"47", align 4
  %"56" = load i64, ptr addrspace(5) %"44", align 4
  %2 = inttoptr i64 %"56" to ptr
  %"55" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"55", ptr addrspace(5) %"45", align 8
  %"58" = load i64, ptr addrspace(5) %"47", align 4
  %3 = inttoptr i64 %"58" to ptr
  %"57" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"57", ptr addrspace(5) %"48", align 8
  %"35" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"35", ptr addrspace(5) %"50", align 4
  %"61" = load i32, ptr addrspace(5) %"50", align 4
  %"60" = zext i32 %"61" to i64
  store i64 %"60", ptr addrspace(5) %"51", align 4
  %"63" = load i64, ptr addrspace(5) %"45", align 4
  %"64" = load i64, ptr addrspace(5) %"51", align 4
  %"74" = add i64 %"63", %"64"
  store i64 %"74", ptr addrspace(5) %"46", align 4
  %"66" = load i64, ptr addrspace(5) %"48", align 4
  %"67" = load i64, ptr addrspace(5) %"51", align 4
  %"76" = add i64 %"66", %"67"
  store i64 %"76", ptr addrspace(5) %"49", align 4
  %"69" = load i64, ptr addrspace(5) %"46", align 4
  %"78" = inttoptr i64 %"69" to ptr addrspace(1)
  %"68" = load i64, ptr addrspace(1) %"78", align 4
  store i64 %"68", ptr addrspace(5) %"52", align 4
  %"70" = load i64, ptr addrspace(5) %"49", align 4
  %"71" = load i64, ptr addrspace(5) %"52", align 4
  %"79" = inttoptr i64 %"70" to ptr addrspace(1)
  store i64 %"71", ptr addrspace(1) %"79", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
