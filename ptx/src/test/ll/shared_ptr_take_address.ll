@shared_mem = external addrspace(3) global [0 x i8], align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @shared_ptr_take_address(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"44" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"44", ptr addrspace(5) %"39", align 4
  %"45" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"45", ptr addrspace(5) %"40", align 4
  store i64 ptrtoint (ptr addrspace(3) @shared_mem to i64), ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"39", align 4
  %"56" = inttoptr i64 %"48" to ptr addrspace(1)
  %"47" = load i64, ptr addrspace(1) %"56", align 4
  store i64 %"47", ptr addrspace(5) %"42", align 4
  %"49" = load i64, ptr addrspace(5) %"41", align 4
  %"50" = load i64, ptr addrspace(5) %"42", align 4
  %"57" = inttoptr i64 %"49" to ptr addrspace(3)
  store i64 %"50", ptr addrspace(3) %"57", align 4
  %"52" = load i64, ptr addrspace(5) %"41", align 4
  %"58" = inttoptr i64 %"52" to ptr addrspace(3)
  %"51" = load i64, ptr addrspace(3) %"58", align 4
  store i64 %"51", ptr addrspace(5) %"43", align 4
  %"53" = load i64, ptr addrspace(5) %"40", align 4
  %"54" = load i64, ptr addrspace(5) %"43", align 4
  %"59" = inttoptr i64 %"53" to ptr addrspace(1)
  store i64 %"54", ptr addrspace(1) %"59", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
