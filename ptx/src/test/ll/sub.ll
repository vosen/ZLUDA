declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @sub(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"42" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"42", ptr addrspace(5) %"38", align 4
  %"43" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"43", ptr addrspace(5) %"39", align 4
  %"45" = load i64, ptr addrspace(5) %"38", align 4
  %"50" = inttoptr i64 %"45" to ptr
  %"44" = load i64, ptr %"50", align 4
  store i64 %"44", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(5) %"40", align 4
  %"46" = sub i64 %"47", 1
  store i64 %"46", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"39", align 4
  %"49" = load i64, ptr addrspace(5) %"41", align 4
  %"51" = inttoptr i64 %"48" to ptr
  store i64 %"49", ptr %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
