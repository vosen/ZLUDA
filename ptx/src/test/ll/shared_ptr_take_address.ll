@shared_mem = external addrspace(3) global [0 x i8], align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @shared_ptr_take_address(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"46", ptr addrspace(5) %"41", align 4
  %"47" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"47", ptr addrspace(5) %"42", align 4
  store i64 ptrtoint (ptr addrspace(3) @shared_mem to i64), ptr addrspace(5) %"43", align 4
  %"50" = load i64, ptr addrspace(5) %"41", align 4
  %"58" = inttoptr i64 %"50" to ptr addrspace(1)
  %"49" = load i64, ptr addrspace(1) %"58", align 4
  store i64 %"49", ptr addrspace(5) %"44", align 4
  %"51" = load i64, ptr addrspace(5) %"43", align 4
  %"52" = load i64, ptr addrspace(5) %"44", align 4
  %"59" = inttoptr i64 %"51" to ptr addrspace(3)
  store i64 %"52", ptr addrspace(3) %"59", align 4
  %"54" = load i64, ptr addrspace(5) %"43", align 4
  %"60" = inttoptr i64 %"54" to ptr addrspace(3)
  %"53" = load i64, ptr addrspace(3) %"60", align 4
  store i64 %"53", ptr addrspace(5) %"45", align 4
  %"55" = load i64, ptr addrspace(5) %"42", align 4
  %"56" = load i64, ptr addrspace(5) %"45", align 4
  %"61" = inttoptr i64 %"55" to ptr addrspace(1)
  store i64 %"56", ptr addrspace(1) %"61", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }