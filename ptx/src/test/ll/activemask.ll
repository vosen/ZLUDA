declare i32 @__zluda_ptx_impl_activemask() #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @activemask(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #1 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"38" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"38", ptr addrspace(5) %"36", align 4
  %"39" = call i32 @__zluda_ptx_impl_activemask()
  store i32 %"39", ptr addrspace(5) %"37", align 4
  %"40" = load i64, ptr addrspace(5) %"36", align 4
  %"41" = load i32, ptr addrspace(5) %"37", align 4
  %"42" = inttoptr i64 %"40" to ptr
  store i32 %"41", ptr %"42", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }