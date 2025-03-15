declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_ntid_chain(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"75" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"75", ptr addrspace(5) %"47", align 4
  %"76" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"76", ptr addrspace(5) %"50", align 4
  %"59" = load i64, ptr addrspace(5) %"47", align 4
  %2 = inttoptr i64 %"59" to ptr
  %"58" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"58", ptr addrspace(5) %"48", align 8
  %"61" = load i64, ptr addrspace(5) %"50", align 4
  %3 = inttoptr i64 %"61" to ptr
  %"60" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"60", ptr addrspace(5) %"51", align 8
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"38"

"38":                                             ; preds = %"37"
  store i32 %"36", ptr addrspace(5) %"53", align 4
  %"64" = load i32, ptr addrspace(5) %"53", align 4
  %"63" = zext i32 %"64" to i64
  store i64 %"63", ptr addrspace(5) %"54", align 4
  %"66" = load i64, ptr addrspace(5) %"48", align 4
  %"67" = load i64, ptr addrspace(5) %"54", align 4
  %"77" = add i64 %"66", %"67"
  store i64 %"77", ptr addrspace(5) %"49", align 4
  %"69" = load i64, ptr addrspace(5) %"51", align 4
  %"70" = load i64, ptr addrspace(5) %"54", align 4
  %"79" = add i64 %"69", %"70"
  store i64 %"79", ptr addrspace(5) %"52", align 4
  %"72" = load i64, ptr addrspace(5) %"49", align 4
  %"81" = inttoptr i64 %"72" to ptr addrspace(1)
  %"71" = load i64, ptr addrspace(1) %"81", align 4
  store i64 %"71", ptr addrspace(5) %"55", align 4
  %"73" = load i64, ptr addrspace(5) %"52", align 4
  %"74" = load i64, ptr addrspace(5) %"55", align 4
  %"82" = inttoptr i64 %"73" to ptr addrspace(1)
  store i64 %"74", ptr addrspace(1) %"82", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }