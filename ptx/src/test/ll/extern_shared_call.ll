@shared_mem = external addrspace(3) global [0 x i32], align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define void @incr_shared_2_global() #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"63"

"63":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(3) @shared_mem, align 4
  store i64 %"40", ptr addrspace(5) %"39", align 4
  %"42" = load i64, ptr addrspace(5) %"39", align 4
  %"41" = add i64 %"42", 2
  store i64 %"41", ptr addrspace(5) %"39", align 4
  %"43" = load i64, ptr addrspace(5) %"39", align 4
  store i64 %"43", ptr addrspace(3) @shared_mem, align 4
  ret void
}

define amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"64"

"64":                                             ; preds = %1
  %"49" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"49", ptr addrspace(5) %"46", align 4
  %"50" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"50", ptr addrspace(5) %"47", align 4
  %"52" = load i64, ptr addrspace(5) %"46", align 4
  %"59" = inttoptr i64 %"52" to ptr addrspace(1)
  %"51" = load i64, ptr addrspace(1) %"59", align 4
  store i64 %"51", ptr addrspace(5) %"48", align 4
  %"53" = load i64, ptr addrspace(5) %"48", align 4
  store i64 %"53", ptr addrspace(3) @shared_mem, align 4
  call void @incr_shared_2_global()
  %"54" = load i64, ptr addrspace(3) @shared_mem, align 4
  store i64 %"54", ptr addrspace(5) %"48", align 4
  %"55" = load i64, ptr addrspace(5) %"47", align 4
  %"56" = load i64, ptr addrspace(5) %"48", align 4
  %"62" = inttoptr i64 %"55" to ptr addrspace(1)
  store i64 %"56", ptr addrspace(1) %"62", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }