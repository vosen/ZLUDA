@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global i64, align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define i64 @__zluda_ptx_impl_add(i64 %"10") #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i64 %"10", ptr addrspace(3) @shared_mod, align 4
  %"49" = load i64, ptr addrspace(3) @shared_mod, align 4
  store i64 %"49", ptr addrspace(5) %"48", align 4
  %"101" = load i64, ptr addrspace(3) @shared_ex, align 4
  %"51" = load i64, ptr addrspace(5) %"48", align 4
  %"72" = add i64 %"101", %"51"
  store i64 %"72", ptr addrspace(5) %"47", align 4
  %2 = load i64, ptr addrspace(5) %"47", align 4
  ret i64 %2
}

define i64 @__zluda_ptx_impl_set_shared_temp1(i64 %"15", i64 %"16") #0 {
  %"52" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i64 %"15", ptr addrspace(3) @shared_ex, align 4
  %"53" = call i64 @__zluda_ptx_impl_add(i64 %"16")
  store i64 %"53", ptr addrspace(5) %"52", align 4
  %2 = load i64, ptr addrspace(5) %"52", align 4
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"54", ptr addrspace(4) byref(i64) %"55") #0 {
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"60" = load i64, ptr addrspace(4) %"54", align 4
  store i64 %"60", ptr addrspace(5) %"56", align 4
  %"61" = load i64, ptr addrspace(4) %"55", align 4
  store i64 %"61", ptr addrspace(5) %"57", align 4
  %"63" = load i64, ptr addrspace(5) %"56", align 4
  %"75" = inttoptr i64 %"63" to ptr addrspace(1)
  %"62" = load i64, ptr addrspace(1) %"75", align 4
  store i64 %"62", ptr addrspace(5) %"58", align 4
  %"64" = load i64, ptr addrspace(5) %"56", align 4
  %"76" = inttoptr i64 %"64" to ptr addrspace(1)
  %"40" = getelementptr inbounds i8, ptr addrspace(1) %"76", i64 8
  %"65" = load i64, ptr addrspace(1) %"40", align 4
  store i64 %"65", ptr addrspace(5) %"59", align 4
  %"67" = load i64, ptr addrspace(5) %"58", align 4
  %"68" = load i64, ptr addrspace(5) %"59", align 4
  %"77" = call i64 @__zluda_ptx_impl_set_shared_temp1(i64 %"67", i64 %"68")
  store i64 %"77", ptr addrspace(5) %"59", align 4
  %"69" = load i64, ptr addrspace(5) %"57", align 4
  %"70" = load i64, ptr addrspace(5) %"59", align 4
  %"79" = inttoptr i64 %"69" to ptr
  store i64 %"70", ptr %"79", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }