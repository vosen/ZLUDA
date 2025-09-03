@0 = addrspace(4) global i8 0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"66" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"66", ptr addrspace(5) %"42", align 8
  %"67" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"67", ptr addrspace(5) %"43", align 8
  %"50" = load i64, ptr addrspace(5) %"42", align 8
  %2 = inttoptr i64 %"50" to ptr
  %"49" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"49", ptr addrspace(5) %"42", align 8
  %"52" = load i64, ptr addrspace(5) %"43", align 8
  %3 = inttoptr i64 %"52" to ptr
  %"51" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"51", ptr addrspace(5) %"43", align 8
  %"35" = load i8, ptr addrspace(4) @0, align 1
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"35")
  br label %"38"

"38":                                             ; preds = %"37"
  store i32 %"36", ptr addrspace(5) %"44", align 4
  %"55" = load i32, ptr addrspace(5) %"44", align 4
  %"54" = zext i32 %"55" to i64
  store i64 %"54", ptr addrspace(5) %"45", align 8
  %"57" = load i64, ptr addrspace(5) %"42", align 8
  %"58" = load i64, ptr addrspace(5) %"45", align 8
  %"68" = add i64 %"57", %"58"
  store i64 %"68", ptr addrspace(5) %"42", align 8
  %"60" = load i64, ptr addrspace(5) %"43", align 8
  %"61" = load i64, ptr addrspace(5) %"45", align 8
  %"70" = add i64 %"60", %"61"
  store i64 %"70", ptr addrspace(5) %"43", align 8
  %"63" = load i64, ptr addrspace(5) %"42", align 8
  %"72" = inttoptr i64 %"63" to ptr addrspace(1)
  %"62" = load i64, ptr addrspace(1) %"72", align 8
  store i64 %"62", ptr addrspace(5) %"46", align 8
  %"64" = load i64, ptr addrspace(5) %"43", align 8
  %"65" = load i64, ptr addrspace(5) %"46", align 8
  %"73" = inttoptr i64 %"64" to ptr addrspace(1)
  store i64 %"65", ptr addrspace(1) %"73", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }