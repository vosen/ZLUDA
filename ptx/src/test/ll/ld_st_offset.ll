declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @ld_st_offset(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"41", align 4
  %"55" = inttoptr i64 %"48" to ptr
  %"47" = load i32, ptr %"55", align 4
  store i32 %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"41", align 4
  %"56" = inttoptr i64 %"49" to ptr
  %"30" = getelementptr inbounds i8, ptr %"56", i64 4
  %"50" = load i32, ptr %"30", align 4
  store i32 %"50", ptr addrspace(5) %"44", align 4
  %"51" = load i64, ptr addrspace(5) %"42", align 4
  %"52" = load i32, ptr addrspace(5) %"44", align 4
  %"57" = inttoptr i64 %"51" to ptr
  store i32 %"52", ptr %"57", align 4
  %"53" = load i64, ptr addrspace(5) %"42", align 4
  %"58" = inttoptr i64 %"53" to ptr
  %"32" = getelementptr inbounds i8, ptr %"58", i64 4
  %"54" = load i32, ptr addrspace(5) %"43", align 4
  store i32 %"54", ptr %"32", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
