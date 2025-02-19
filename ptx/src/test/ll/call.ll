declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define i64 @__zluda_ptx_impl_incr(i64 %"42") #0 {
  %"65" = alloca i64, align 8, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i64 %"42", ptr addrspace(5) %"67", align 4
  %"69" = load i64, ptr addrspace(5) %"67", align 4
  store i64 %"69", ptr addrspace(5) %"68", align 4
  %"71" = load i64, ptr addrspace(5) %"68", align 4
  %"70" = add i64 %"71", 1
  store i64 %"70", ptr addrspace(5) %"68", align 4
  %"72" = load i64, ptr addrspace(5) %"68", align 4
  store i64 %"72", ptr addrspace(5) %"66", align 4
  %"73" = load i64, ptr addrspace(5) %"66", align 4
  store i64 %"73", ptr addrspace(5) %"65", align 4
  %2 = load i64, ptr addrspace(5) %"65", align 4
  ret i64 %2
}

define amdgpu_kernel void @call(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #0 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"55" = load i64, ptr addrspace(4) %"50", align 4
  store i64 %"55", ptr addrspace(5) %"52", align 4
  %"56" = load i64, ptr addrspace(4) %"51", align 4
  store i64 %"56", ptr addrspace(5) %"53", align 4
  %"58" = load i64, ptr addrspace(5) %"52", align 4
  %"74" = inttoptr i64 %"58" to ptr addrspace(1)
  %"57" = load i64, ptr addrspace(1) %"74", align 4
  store i64 %"57", ptr addrspace(5) %"54", align 4
  %"61" = load i64, ptr addrspace(5) %"54", align 4
  store i64 %"61", ptr addrspace(5) %"59", align 4
  %"39" = load i64, ptr addrspace(5) %"59", align 4
  %"40" = call i64 @__zluda_ptx_impl_incr(i64 %"39")
  store i64 %"40", ptr addrspace(5) %"60", align 4
  %"62" = load i64, ptr addrspace(5) %"60", align 4
  store i64 %"62", ptr addrspace(5) %"54", align 4
  %"63" = load i64, ptr addrspace(5) %"53", align 4
  %"64" = load i64, ptr addrspace(5) %"54", align 4
  %"77" = inttoptr i64 %"63" to ptr addrspace(1)
  store i64 %"64", ptr addrspace(1) %"77", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
