declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @mul_wide(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"45" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"45", ptr addrspace(5) %"40", align 4
  %"46" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"46", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"56" = inttoptr i64 %"48" to ptr addrspace(1)
  %"47" = load i32, ptr addrspace(1) %"56", align 4
  store i32 %"47", ptr addrspace(5) %"42", align 4
  %"49" = load i64, ptr addrspace(5) %"40", align 4
  %"57" = inttoptr i64 %"49" to ptr addrspace(1)
  %"31" = getelementptr inbounds i8, ptr addrspace(1) %"57", i64 4
  %"50" = load i32, ptr addrspace(1) %"31", align 4
  store i32 %"50", ptr addrspace(5) %"43", align 4
  %"52" = load i32, ptr addrspace(5) %"42", align 4
  %"53" = load i32, ptr addrspace(5) %"43", align 4
  %2 = sext i32 %"52" to i64
  %3 = sext i32 %"53" to i64
  %"51" = mul i64 %2, %3
  store i64 %"51", ptr addrspace(5) %"44", align 4
  %"54" = load i64, ptr addrspace(5) %"41", align 4
  %"55" = load i64, ptr addrspace(5) %"44", align 4
  %"58" = inttoptr i64 %"54" to ptr
  store i64 %"55", ptr %"58", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
