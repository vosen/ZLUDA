@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #1 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"62" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"62", ptr addrspace(5) %"38", align 4
  %"63" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"63", ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(5) %"38", align 4
  %2 = inttoptr i64 %"46" to ptr
  %"45" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"45", ptr addrspace(5) %"38", align 8
  %"48" = load i64, ptr addrspace(5) %"39", align 4
  %3 = inttoptr i64 %"48" to ptr
  %"47" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"47", ptr addrspace(5) %"39", align 8
  %"32" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"34"

"34":                                             ; preds = %"33"
  store i32 %"32", ptr addrspace(5) %"40", align 4
  %"51" = load i32, ptr addrspace(5) %"40", align 4
  %"50" = zext i32 %"51" to i64
  store i64 %"50", ptr addrspace(5) %"41", align 4
  %"53" = load i64, ptr addrspace(5) %"38", align 4
  %"54" = load i64, ptr addrspace(5) %"41", align 4
  %"64" = add i64 %"53", %"54"
  store i64 %"64", ptr addrspace(5) %"38", align 4
  %"56" = load i64, ptr addrspace(5) %"39", align 4
  %"57" = load i64, ptr addrspace(5) %"41", align 4
  %"66" = add i64 %"56", %"57"
  store i64 %"66", ptr addrspace(5) %"39", align 4
  %"59" = load i64, ptr addrspace(5) %"38", align 4
  %"68" = inttoptr i64 %"59" to ptr addrspace(1)
  %"58" = load i64, ptr addrspace(1) %"68", align 4
  store i64 %"58", ptr addrspace(5) %"42", align 4
  %"60" = load i64, ptr addrspace(5) %"39", align 4
  %"61" = load i64, ptr addrspace(5) %"42", align 4
  %"69" = inttoptr i64 %"60" to ptr addrspace(1)
  store i64 %"61", ptr addrspace(1) %"69", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }