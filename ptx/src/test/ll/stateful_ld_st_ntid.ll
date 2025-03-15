declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_ntid(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"67" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"67", ptr addrspace(5) %"43", align 4
  %"68" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"68", ptr addrspace(5) %"44", align 4
  %"51" = load i64, ptr addrspace(5) %"43", align 4
  %2 = inttoptr i64 %"51" to ptr
  %"50" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"50", ptr addrspace(5) %"43", align 8
  %"53" = load i64, ptr addrspace(5) %"44", align 4
  %3 = inttoptr i64 %"53" to ptr
  %"52" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"52", ptr addrspace(5) %"44", align 8
  %"32" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"34"

"34":                                             ; preds = %"33"
  store i32 %"32", ptr addrspace(5) %"45", align 4
  %"56" = load i32, ptr addrspace(5) %"45", align 4
  %"55" = zext i32 %"56" to i64
  store i64 %"55", ptr addrspace(5) %"46", align 4
  %"58" = load i64, ptr addrspace(5) %"43", align 4
  %"59" = load i64, ptr addrspace(5) %"46", align 4
  %"69" = add i64 %"58", %"59"
  store i64 %"69", ptr addrspace(5) %"43", align 4
  %"61" = load i64, ptr addrspace(5) %"44", align 4
  %"62" = load i64, ptr addrspace(5) %"46", align 4
  %"71" = add i64 %"61", %"62"
  store i64 %"71", ptr addrspace(5) %"44", align 4
  %"64" = load i64, ptr addrspace(5) %"43", align 4
  %"73" = inttoptr i64 %"64" to ptr addrspace(1)
  %"63" = load i64, ptr addrspace(1) %"73", align 4
  store i64 %"63", ptr addrspace(5) %"47", align 4
  %"65" = load i64, ptr addrspace(5) %"44", align 4
  %"66" = load i64, ptr addrspace(5) %"47", align 4
  %"74" = inttoptr i64 %"65" to ptr addrspace(1)
  store i64 %"66", ptr addrspace(1) %"74", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }