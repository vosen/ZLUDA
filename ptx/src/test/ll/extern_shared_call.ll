@shared_mem = external addrspace(3) global [0 x i32], align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define void @__zluda_ptx_impl_incr_shared_2_global() #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"39" = load i64, ptr addrspace(3) @shared_mem, align 4
  store i64 %"39", ptr addrspace(5) %"38", align 4
  %"41" = load i64, ptr addrspace(5) %"38", align 4
  %"40" = add i64 %"41", 2
  store i64 %"40", ptr addrspace(5) %"38", align 4
  %"42" = load i64, ptr addrspace(5) %"38", align 4
  store i64 %"42", ptr addrspace(3) @shared_mem, align 4
  ret void
}

define amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"48" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"48", ptr addrspace(5) %"45", align 4
  %"49" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"49", ptr addrspace(5) %"46", align 4
  %"51" = load i64, ptr addrspace(5) %"45", align 4
  %"58" = inttoptr i64 %"51" to ptr addrspace(1)
  %"50" = load i64, ptr addrspace(1) %"58", align 4
  store i64 %"50", ptr addrspace(5) %"47", align 4
  %"52" = load i64, ptr addrspace(5) %"47", align 4
  store i64 %"52", ptr addrspace(3) @shared_mem, align 4
  call void @__zluda_ptx_impl_incr_shared_2_global()
  %"53" = load i64, ptr addrspace(3) @shared_mem, align 4
  store i64 %"53", ptr addrspace(5) %"47", align 4
  %"54" = load i64, ptr addrspace(5) %"46", align 4
  %"55" = load i64, ptr addrspace(5) %"47", align 4
  %"61" = inttoptr i64 %"54" to ptr addrspace(1)
  store i64 %"55", ptr addrspace(1) %"61", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
