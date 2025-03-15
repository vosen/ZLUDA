declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @mad_s32(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"55" = load i64, ptr addrspace(4) %"47", align 4
  store i64 %"55", ptr addrspace(5) %"49", align 4
  %"56" = load i64, ptr addrspace(4) %"48", align 4
  store i64 %"56", ptr addrspace(5) %"50", align 4
  %"58" = load i64, ptr addrspace(5) %"49", align 4
  %"73" = inttoptr i64 %"58" to ptr
  %"57" = load i32, ptr %"73", align 4
  store i32 %"57", ptr addrspace(5) %"52", align 4
  %"59" = load i64, ptr addrspace(5) %"49", align 4
  %"74" = inttoptr i64 %"59" to ptr
  %"33" = getelementptr inbounds i8, ptr %"74", i64 4
  %"60" = load i32, ptr %"33", align 4
  store i32 %"60", ptr addrspace(5) %"53", align 4
  %"61" = load i64, ptr addrspace(5) %"49", align 4
  %"75" = inttoptr i64 %"61" to ptr
  %"35" = getelementptr inbounds i8, ptr %"75", i64 8
  %"62" = load i32, ptr %"35", align 4
  store i32 %"62", ptr addrspace(5) %"54", align 4
  %"64" = load i32, ptr addrspace(5) %"52", align 4
  %"65" = load i32, ptr addrspace(5) %"53", align 4
  %"66" = load i32, ptr addrspace(5) %"54", align 4
  %2 = mul i32 %"64", %"65"
  %"63" = add i32 %2, %"66"
  store i32 %"63", ptr addrspace(5) %"51", align 4
  %"67" = load i64, ptr addrspace(5) %"50", align 4
  %"68" = load i32, ptr addrspace(5) %"51", align 4
  %"76" = inttoptr i64 %"67" to ptr
  store i32 %"68", ptr %"76", align 4
  %"69" = load i64, ptr addrspace(5) %"50", align 4
  %"77" = inttoptr i64 %"69" to ptr
  %"37" = getelementptr inbounds i8, ptr %"77", i64 4
  %"70" = load i32, ptr addrspace(5) %"51", align 4
  store i32 %"70", ptr %"37", align 4
  %"71" = load i64, ptr addrspace(5) %"50", align 4
  %"78" = inttoptr i64 %"71" to ptr
  %"39" = getelementptr inbounds i8, ptr %"78", i64 8
  %"72" = load i32, ptr addrspace(5) %"51", align 4
  store i32 %"72", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }