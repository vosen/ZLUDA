declare i32 @__zluda_ptx_impl_bfe_u32(i32, i32, i32) #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @bfe(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #1 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"49" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"49", ptr addrspace(5) %"44", align 4
  %"50" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"50", ptr addrspace(5) %"45", align 4
  %"52" = load i64, ptr addrspace(5) %"44", align 4
  %"63" = inttoptr i64 %"52" to ptr
  %"51" = load i32, ptr %"63", align 4
  store i32 %"51", ptr addrspace(5) %"46", align 4
  %"53" = load i64, ptr addrspace(5) %"44", align 4
  %"64" = inttoptr i64 %"53" to ptr
  %"32" = getelementptr inbounds i8, ptr %"64", i64 4
  %"54" = load i32, ptr %"32", align 4
  store i32 %"54", ptr addrspace(5) %"47", align 4
  %"55" = load i64, ptr addrspace(5) %"44", align 4
  %"65" = inttoptr i64 %"55" to ptr
  %"34" = getelementptr inbounds i8, ptr %"65", i64 8
  %"56" = load i32, ptr %"34", align 4
  store i32 %"56", ptr addrspace(5) %"48", align 4
  %"58" = load i32, ptr addrspace(5) %"46", align 4
  %"59" = load i32, ptr addrspace(5) %"47", align 4
  %"60" = load i32, ptr addrspace(5) %"48", align 4
  %"57" = call i32 @__zluda_ptx_impl_bfe_u32(i32 %"58", i32 %"59", i32 %"60")
  store i32 %"57", ptr addrspace(5) %"46", align 4
  %"61" = load i64, ptr addrspace(5) %"45", align 4
  %"62" = load i32, ptr addrspace(5) %"46", align 4
  %"66" = inttoptr i64 %"61" to ptr
  store i32 %"62", ptr %"66", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }