declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @selp_true(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i16, align 2, addrspace(5)
  %"43" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"44" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"44", ptr addrspace(5) %"40", align 4
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"47" = load i64, ptr addrspace(5) %"40", align 4
  %"55" = inttoptr i64 %"47" to ptr
  %"46" = load i16, ptr %"55", align 2
  store i16 %"46", ptr addrspace(5) %"42", align 2
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"56" = inttoptr i64 %"48" to ptr
  %"30" = getelementptr inbounds i8, ptr %"56", i64 2
  %"49" = load i16, ptr %"30", align 2
  store i16 %"49", ptr addrspace(5) %"43", align 2
  %"51" = load i16, ptr addrspace(5) %"42", align 2
  %"52" = load i16, ptr addrspace(5) %"43", align 2
  %"50" = select i1 true, i16 %"51", i16 %"52"
  store i16 %"50", ptr addrspace(5) %"42", align 2
  %"53" = load i64, ptr addrspace(5) %"41", align 4
  %"54" = load i16, ptr addrspace(5) %"42", align 2
  %"57" = inttoptr i64 %"53" to ptr
  store i16 %"54", ptr %"57", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
