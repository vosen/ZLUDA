@shared_mem1 = external addrspace(3) global [128 x i8], align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @shared_ptr_32(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"63"

"63":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"47", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"48", ptr addrspace(5) %"43", align 4
  store i32 ptrtoint (ptr addrspace(3) @shared_mem1 to i32), ptr addrspace(5) %"44", align 4
  %"51" = load i64, ptr addrspace(5) %"42", align 4
  %"59" = inttoptr i64 %"51" to ptr addrspace(1)
  %"50" = load i64, ptr addrspace(1) %"59", align 4
  store i64 %"50", ptr addrspace(5) %"45", align 4
  %"52" = load i32, ptr addrspace(5) %"44", align 4
  %"53" = load i64, ptr addrspace(5) %"45", align 4
  %"60" = inttoptr i32 %"52" to ptr addrspace(3)
  store i64 %"53", ptr addrspace(3) %"60", align 4
  %"54" = load i32, ptr addrspace(5) %"44", align 4
  %"61" = inttoptr i32 %"54" to ptr addrspace(3)
  %"33" = getelementptr inbounds i8, ptr addrspace(3) %"61", i64 0
  %"55" = load i64, ptr addrspace(3) %"33", align 4
  store i64 %"55", ptr addrspace(5) %"46", align 4
  %"56" = load i64, ptr addrspace(5) %"43", align 4
  %"57" = load i64, ptr addrspace(5) %"46", align 4
  %"62" = inttoptr i64 %"56" to ptr addrspace(1)
  store i64 %"57", ptr addrspace(1) %"62", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }