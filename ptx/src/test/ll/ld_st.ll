declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @ld_st(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #1 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"40", ptr addrspace(5) %"37", align 4
  %"41" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"41", ptr addrspace(5) %"38", align 4
  %"43" = load i64, ptr addrspace(5) %"37", align 4
  %"46" = inttoptr i64 %"43" to ptr
  %"42" = load i64, ptr %"46", align 4
  store i64 %"42", ptr addrspace(5) %"39", align 4
  %"44" = load i64, ptr addrspace(5) %"38", align 4
  %"45" = load i64, ptr addrspace(5) %"39", align 4
  %"47" = inttoptr i64 %"44" to ptr
  store i64 %"45", ptr %"47", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }