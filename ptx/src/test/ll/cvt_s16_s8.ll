declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvt_s16_s8(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"41" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  %"42" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"42", ptr addrspace(5) %"38", align 4
  %"44" = load i64, ptr addrspace(5) %"37", align 4
  %"49" = inttoptr i64 %"44" to ptr addrspace(1)
  %"43" = load i32, ptr addrspace(1) %"49", align 4
  store i32 %"43", ptr addrspace(5) %"40", align 4
  %"46" = load i32, ptr addrspace(5) %"40", align 4
  %2 = trunc i32 %"46" to i8
  %"50" = sext i8 %2 to i16
  %"45" = sext i16 %"50" to i32
  store i32 %"45", ptr addrspace(5) %"39", align 4
  %"47" = load i64, ptr addrspace(5) %"38", align 4
  %"48" = load i32, ptr addrspace(5) %"39", align 4
  %"52" = inttoptr i64 %"47" to ptr
  store i32 %"48", ptr %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
