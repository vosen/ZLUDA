declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"87"

"87":                                             ; preds = %1
  %"77" = load i64, ptr addrspace(4) %"47", align 4
  store i64 %"77", ptr addrspace(5) %"49", align 4
  %"78" = load i64, ptr addrspace(4) %"48", align 4
  store i64 %"78", ptr addrspace(5) %"52", align 4
  %"61" = load i64, ptr addrspace(5) %"49", align 4
  %2 = inttoptr i64 %"61" to ptr
  %"60" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"60", ptr addrspace(5) %"50", align 8
  %"63" = load i64, ptr addrspace(5) %"52", align 4
  %3 = inttoptr i64 %"63" to ptr
  %"62" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"62", ptr addrspace(5) %"53", align 8
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"36", ptr addrspace(5) %"55", align 4
  %"66" = load i32, ptr addrspace(5) %"55", align 4
  %"65" = zext i32 %"66" to i64
  store i64 %"65", ptr addrspace(5) %"56", align 4
  %"68" = load i64, ptr addrspace(5) %"50", align 4
  %"69" = load i64, ptr addrspace(5) %"56", align 4
  %"79" = sub i64 %"68", %"69"
  store i64 %"79", ptr addrspace(5) %"51", align 4
  %"71" = load i64, ptr addrspace(5) %"53", align 4
  %"72" = load i64, ptr addrspace(5) %"56", align 4
  %"82" = sub i64 %"71", %"72"
  store i64 %"82", ptr addrspace(5) %"54", align 4
  %"73" = load i64, ptr addrspace(5) %"51", align 4
  %"85" = inttoptr i64 %"73" to ptr addrspace(1)
  %"38" = getelementptr inbounds i8, ptr addrspace(1) %"85", i64 0
  %"74" = load i64, ptr addrspace(1) %"38", align 4
  store i64 %"74", ptr addrspace(5) %"57", align 4
  %"75" = load i64, ptr addrspace(5) %"54", align 4
  %"86" = inttoptr i64 %"75" to ptr addrspace(1)
  %"40" = getelementptr inbounds i8, ptr addrspace(1) %"86", i64 0
  %"76" = load i64, ptr addrspace(5) %"57", align 4
  store i64 %"76", ptr addrspace(1) %"40", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }