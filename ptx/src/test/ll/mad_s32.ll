declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @mad_s32(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #1 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"78"

"78":                                             ; preds = %1
  %"54" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"54", ptr addrspace(5) %"48", align 4
  %"55" = load i64, ptr addrspace(4) %"47", align 4
  store i64 %"55", ptr addrspace(5) %"49", align 4
  %"57" = load i64, ptr addrspace(5) %"48", align 4
  %"72" = inttoptr i64 %"57" to ptr
  %"56" = load i32, ptr %"72", align 4
  store i32 %"56", ptr addrspace(5) %"51", align 4
  %"58" = load i64, ptr addrspace(5) %"48", align 4
  %"73" = inttoptr i64 %"58" to ptr
  %"33" = getelementptr inbounds i8, ptr %"73", i64 4
  %"59" = load i32, ptr %"33", align 4
  store i32 %"59", ptr addrspace(5) %"52", align 4
  %"60" = load i64, ptr addrspace(5) %"48", align 4
  %"74" = inttoptr i64 %"60" to ptr
  %"35" = getelementptr inbounds i8, ptr %"74", i64 8
  %"61" = load i32, ptr %"35", align 4
  store i32 %"61", ptr addrspace(5) %"53", align 4
  %"63" = load i32, ptr addrspace(5) %"51", align 4
  %"64" = load i32, ptr addrspace(5) %"52", align 4
  %"65" = load i32, ptr addrspace(5) %"53", align 4
  %2 = mul i32 %"63", %"64"
  %"62" = add i32 %2, %"65"
  store i32 %"62", ptr addrspace(5) %"50", align 4
  %"66" = load i64, ptr addrspace(5) %"49", align 4
  %"67" = load i32, ptr addrspace(5) %"50", align 4
  %"75" = inttoptr i64 %"66" to ptr
  store i32 %"67", ptr %"75", align 4
  %"68" = load i64, ptr addrspace(5) %"49", align 4
  %"76" = inttoptr i64 %"68" to ptr
  %"37" = getelementptr inbounds i8, ptr %"76", i64 4
  %"69" = load i32, ptr addrspace(5) %"50", align 4
  store i32 %"69", ptr %"37", align 4
  %"70" = load i64, ptr addrspace(5) %"49", align 4
  %"77" = inttoptr i64 %"70" to ptr
  %"39" = getelementptr inbounds i8, ptr %"77", i64 8
  %"71" = load i32, ptr addrspace(5) %"50", align 4
  store i32 %"71", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }