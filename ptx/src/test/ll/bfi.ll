declare i32 @__zluda_ptx_impl_bfi_b32(i32, i32, i32, i32) #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @bfi(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"77"

"77":                                             ; preds = %1
  %"52" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"52", ptr addrspace(5) %"46", align 4
  %"53" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"53", ptr addrspace(5) %"47", align 4
  %"55" = load i64, ptr addrspace(5) %"46", align 4
  %"69" = inttoptr i64 %"55" to ptr
  %"54" = load i32, ptr %"69", align 4
  store i32 %"54", ptr addrspace(5) %"48", align 4
  %"56" = load i64, ptr addrspace(5) %"46", align 4
  %"70" = inttoptr i64 %"56" to ptr
  %"33" = getelementptr inbounds i8, ptr %"70", i64 4
  %"57" = load i32, ptr %"33", align 4
  store i32 %"57", ptr addrspace(5) %"49", align 4
  %"58" = load i64, ptr addrspace(5) %"46", align 4
  %"71" = inttoptr i64 %"58" to ptr
  %"35" = getelementptr inbounds i8, ptr %"71", i64 8
  %"59" = load i32, ptr %"35", align 4
  store i32 %"59", ptr addrspace(5) %"50", align 4
  %"60" = load i64, ptr addrspace(5) %"46", align 4
  %"72" = inttoptr i64 %"60" to ptr
  %"37" = getelementptr inbounds i8, ptr %"72", i64 12
  %"61" = load i32, ptr %"37", align 4
  store i32 %"61", ptr addrspace(5) %"51", align 4
  %"63" = load i32, ptr addrspace(5) %"48", align 4
  %"64" = load i32, ptr addrspace(5) %"49", align 4
  %"65" = load i32, ptr addrspace(5) %"50", align 4
  %"66" = load i32, ptr addrspace(5) %"51", align 4
  %"73" = call i32 @__zluda_ptx_impl_bfi_b32(i32 %"63", i32 %"64", i32 %"65", i32 %"66")
  store i32 %"73", ptr addrspace(5) %"48", align 4
  %"67" = load i64, ptr addrspace(5) %"47", align 4
  %"68" = load i32, ptr addrspace(5) %"48", align 4
  %"76" = inttoptr i64 %"67" to ptr
  store i32 %"68", ptr %"76", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }