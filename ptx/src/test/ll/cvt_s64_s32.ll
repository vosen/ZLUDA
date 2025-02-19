declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvt_s64_s32(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"41" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  %"42" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"42", ptr addrspace(5) %"38", align 4
  %"44" = load i64, ptr addrspace(5) %"37", align 4
  %"50" = inttoptr i64 %"44" to ptr
  %"49" = load i32, ptr %"50", align 4
  store i32 %"49", ptr addrspace(5) %"39", align 4
  %"46" = load i32, ptr addrspace(5) %"39", align 4
  %"45" = sext i32 %"46" to i64
  store i64 %"45", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(5) %"38", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"51" = inttoptr i64 %"47" to ptr
  store i64 %"48", ptr %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
