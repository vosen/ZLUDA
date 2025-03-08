@shared_mem = external addrspace(3) global [0 x i32]

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @extern_shared(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #1 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"53"

"53":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"41", ptr addrspace(5) %"38", align 4
  %"42" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"42", ptr addrspace(5) %"39", align 4
  %"44" = load i64, ptr addrspace(5) %"38", align 4
  %"49" = inttoptr i64 %"44" to ptr addrspace(1)
  %"43" = load i64, ptr addrspace(1) %"49", align 4
  store i64 %"43", ptr addrspace(5) %"40", align 4
  %"45" = load i64, ptr addrspace(5) %"40", align 4
  store i64 %"45", ptr addrspace(3) @shared_mem, align 4
  %"46" = load i64, ptr addrspace(3) @shared_mem, align 4
  store i64 %"46", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(5) %"39", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"52" = inttoptr i64 %"47" to ptr addrspace(1)
  store i64 %"48", ptr addrspace(1) %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }