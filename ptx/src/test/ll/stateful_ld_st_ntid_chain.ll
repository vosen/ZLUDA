declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_ntid_chain(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #1 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"81"

"81":                                             ; preds = %1
  %"73" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"73", ptr addrspace(5) %"45", align 4
  %"74" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"74", ptr addrspace(5) %"48", align 4
  %"57" = load i64, ptr addrspace(5) %"45", align 4
  %2 = inttoptr i64 %"57" to ptr
  %"56" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"56", ptr addrspace(5) %"46", align 8
  %"59" = load i64, ptr addrspace(5) %"48", align 4
  %3 = inttoptr i64 %"59" to ptr
  %"58" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"58", ptr addrspace(5) %"49", align 8
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"36", ptr addrspace(5) %"51", align 4
  %"62" = load i32, ptr addrspace(5) %"51", align 4
  %"61" = zext i32 %"62" to i64
  store i64 %"61", ptr addrspace(5) %"52", align 4
  %"64" = load i64, ptr addrspace(5) %"46", align 4
  %"65" = load i64, ptr addrspace(5) %"52", align 4
  %"75" = add i64 %"64", %"65"
  store i64 %"75", ptr addrspace(5) %"47", align 4
  %"67" = load i64, ptr addrspace(5) %"49", align 4
  %"68" = load i64, ptr addrspace(5) %"52", align 4
  %"77" = add i64 %"67", %"68"
  store i64 %"77", ptr addrspace(5) %"50", align 4
  %"70" = load i64, ptr addrspace(5) %"47", align 4
  %"79" = inttoptr i64 %"70" to ptr addrspace(1)
  %"69" = load i64, ptr addrspace(1) %"79", align 4
  store i64 %"69", ptr addrspace(5) %"53", align 4
  %"71" = load i64, ptr addrspace(5) %"50", align 4
  %"72" = load i64, ptr addrspace(5) %"53", align 4
  %"80" = inttoptr i64 %"71" to ptr addrspace(1)
  store i64 %"72", ptr addrspace(1) %"80", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }