@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_chain(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"70" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"70", ptr addrspace(5) %"42", align 4
  %"71" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"71", ptr addrspace(5) %"45", align 4
  %"54" = load i64, ptr addrspace(5) %"42", align 4
  %2 = inttoptr i64 %"54" to ptr
  %"53" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"53", ptr addrspace(5) %"43", align 8
  %"56" = load i64, ptr addrspace(5) %"45", align 4
  %3 = inttoptr i64 %"56" to ptr
  %"55" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"55", ptr addrspace(5) %"46", align 8
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"38"

"38":                                             ; preds = %"37"
  store i32 %"36", ptr addrspace(5) %"48", align 4
  %"59" = load i32, ptr addrspace(5) %"48", align 4
  %"58" = zext i32 %"59" to i64
  store i64 %"58", ptr addrspace(5) %"49", align 4
  %"61" = load i64, ptr addrspace(5) %"43", align 4
  %"62" = load i64, ptr addrspace(5) %"49", align 4
  %"72" = add i64 %"61", %"62"
  store i64 %"72", ptr addrspace(5) %"44", align 4
  %"64" = load i64, ptr addrspace(5) %"46", align 4
  %"65" = load i64, ptr addrspace(5) %"49", align 4
  %"74" = add i64 %"64", %"65"
  store i64 %"74", ptr addrspace(5) %"47", align 4
  %"67" = load i64, ptr addrspace(5) %"44", align 4
  %"76" = inttoptr i64 %"67" to ptr addrspace(1)
  %"66" = load i64, ptr addrspace(1) %"76", align 4
  store i64 %"66", ptr addrspace(5) %"50", align 4
  %"68" = load i64, ptr addrspace(5) %"47", align 4
  %"69" = load i64, ptr addrspace(5) %"50", align 4
  %"77" = inttoptr i64 %"68" to ptr addrspace(1)
  store i64 %"69", ptr addrspace(1) %"77", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }