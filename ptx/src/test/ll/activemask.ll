declare i32 @__zluda_ptx_impl_activemask() #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @activemask(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #1 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"28"

"28":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"39", ptr addrspace(5) %"37", align 4
  %"40" = call i32 @__zluda_ptx_impl_activemask()
  store i32 %"40", ptr addrspace(5) %"38", align 4
  %"41" = load i64, ptr addrspace(5) %"37", align 4
  %"42" = load i32, ptr addrspace(5) %"38", align 4
  %"43" = inttoptr i64 %"41" to ptr
  store i32 %"42", ptr %"43", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }