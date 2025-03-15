declare i32 @__zluda_ptx_impl_bfi_b32(i32, i32, i32, i32) #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @bfi(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"53" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"53", ptr addrspace(5) %"47", align 4
  %"54" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"54", ptr addrspace(5) %"48", align 4
  %"56" = load i64, ptr addrspace(5) %"47", align 4
  %"70" = inttoptr i64 %"56" to ptr
  %"55" = load i32, ptr %"70", align 4
  store i32 %"55", ptr addrspace(5) %"49", align 4
  %"57" = load i64, ptr addrspace(5) %"47", align 4
  %"71" = inttoptr i64 %"57" to ptr
  %"33" = getelementptr inbounds i8, ptr %"71", i64 4
  %"58" = load i32, ptr %"33", align 4
  store i32 %"58", ptr addrspace(5) %"50", align 4
  %"59" = load i64, ptr addrspace(5) %"47", align 4
  %"72" = inttoptr i64 %"59" to ptr
  %"35" = getelementptr inbounds i8, ptr %"72", i64 8
  %"60" = load i32, ptr %"35", align 4
  store i32 %"60", ptr addrspace(5) %"51", align 4
  %"61" = load i64, ptr addrspace(5) %"47", align 4
  %"73" = inttoptr i64 %"61" to ptr
  %"37" = getelementptr inbounds i8, ptr %"73", i64 12
  %"62" = load i32, ptr %"37", align 4
  store i32 %"62", ptr addrspace(5) %"52", align 4
  %"64" = load i32, ptr addrspace(5) %"49", align 4
  %"65" = load i32, ptr addrspace(5) %"50", align 4
  %"66" = load i32, ptr addrspace(5) %"51", align 4
  %"67" = load i32, ptr addrspace(5) %"52", align 4
  %"74" = call i32 @__zluda_ptx_impl_bfi_b32(i32 %"64", i32 %"65", i32 %"66", i32 %"67")
  store i32 %"74", ptr addrspace(5) %"49", align 4
  %"68" = load i64, ptr addrspace(5) %"48", align 4
  %"69" = load i32, ptr addrspace(5) %"49", align 4
  %"77" = inttoptr i64 %"68" to ptr
  store i32 %"69", ptr %"77", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }