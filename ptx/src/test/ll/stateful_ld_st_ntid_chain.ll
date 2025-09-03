declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

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
  %"60" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"60", align 1
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"75" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"75", ptr addrspace(5) %"45", align 8
  %"76" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"76", ptr addrspace(5) %"48", align 8
  %"57" = load i64, ptr addrspace(5) %"45", align 8
  %2 = inttoptr i64 %"57" to ptr
  %"56" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"56", ptr addrspace(5) %"46", align 8
  %"59" = load i64, ptr addrspace(5) %"48", align 8
  %3 = inttoptr i64 %"59" to ptr
  %"58" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"58", ptr addrspace(5) %"49", align 8
  %"61" = load i8, ptr addrspace(5) %"60", align 1
  %"39" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"61")
  br label %"41"

"41":                                             ; preds = %"40"
  store i32 %"39", ptr addrspace(5) %"51", align 4
  %"64" = load i32, ptr addrspace(5) %"51", align 4
  %"63" = zext i32 %"64" to i64
  store i64 %"63", ptr addrspace(5) %"52", align 8
  %"66" = load i64, ptr addrspace(5) %"46", align 8
  %"67" = load i64, ptr addrspace(5) %"52", align 8
  %"77" = add i64 %"66", %"67"
  store i64 %"77", ptr addrspace(5) %"47", align 8
  %"69" = load i64, ptr addrspace(5) %"49", align 8
  %"70" = load i64, ptr addrspace(5) %"52", align 8
  %"79" = add i64 %"69", %"70"
  store i64 %"79", ptr addrspace(5) %"50", align 8
  %"72" = load i64, ptr addrspace(5) %"47", align 8
  %"81" = inttoptr i64 %"72" to ptr addrspace(1)
  %"71" = load i64, ptr addrspace(1) %"81", align 8
  store i64 %"71", ptr addrspace(5) %"53", align 8
  %"73" = load i64, ptr addrspace(5) %"50", align 8
  %"74" = load i64, ptr addrspace(5) %"53", align 8
  %"82" = inttoptr i64 %"73" to ptr addrspace(1)
  store i64 %"74", ptr addrspace(1) %"82", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }