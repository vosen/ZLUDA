@shared_mem = external addrspace(3) global [0 x i32], align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define void @incr_shared_2_global() #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(3) @shared_mem, align 4
  store i64 %"43", ptr addrspace(5) %"42", align 4
  %"45" = load i64, ptr addrspace(5) %"42", align 4
  %"44" = add i64 %"45", 2
  store i64 %"44", ptr addrspace(5) %"42", align 4
  %"46" = load i64, ptr addrspace(5) %"42", align 4
  store i64 %"46", ptr addrspace(3) @shared_mem, align 4
  ret void
}

define amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"52" = load i64, ptr addrspace(4) %"47", align 4
  store i64 %"52", ptr addrspace(5) %"49", align 4
  %"53" = load i64, ptr addrspace(4) %"48", align 4
  store i64 %"53", ptr addrspace(5) %"50", align 4
  %"55" = load i64, ptr addrspace(5) %"49", align 4
  %"62" = inttoptr i64 %"55" to ptr addrspace(1)
  %"54" = load i64, ptr addrspace(1) %"62", align 4
  store i64 %"54", ptr addrspace(5) %"51", align 4
  %"56" = load i64, ptr addrspace(5) %"51", align 4
  store i64 %"56", ptr addrspace(3) @shared_mem, align 4
  call void @incr_shared_2_global()
  br label %"35"

"35":                                             ; preds = %"34"
  %"57" = load i64, ptr addrspace(3) @shared_mem, align 4
  store i64 %"57", ptr addrspace(5) %"51", align 4
  %"58" = load i64, ptr addrspace(5) %"50", align 4
  %"59" = load i64, ptr addrspace(5) %"51", align 4
  %"65" = inttoptr i64 %"58" to ptr addrspace(1)
  store i64 %"59", ptr addrspace(1) %"65", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }