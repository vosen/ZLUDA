declare i32 @__zluda_ptx_impl_activemask() #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @activemask(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"37" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"37", ptr addrspace(5) %"35", align 4
  %"38" = call i32 @__zluda_ptx_impl_activemask()
  store i32 %"38", ptr addrspace(5) %"36", align 4
  %"39" = load i64, ptr addrspace(5) %"35", align 4
  %"40" = load i32, ptr addrspace(5) %"36", align 4
  %"41" = inttoptr i64 %"39" to ptr
  store i32 %"40", ptr %"41", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
