@0 = addrspace(4) global i8 0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_chain(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"74" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"74", ptr addrspace(5) %"46", align 8
  %"75" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"75", ptr addrspace(5) %"49", align 8
  %"58" = load i64, ptr addrspace(5) %"46", align 8
  %2 = inttoptr i64 %"58" to ptr
  %"57" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"57", ptr addrspace(5) %"47", align 8
  %"60" = load i64, ptr addrspace(5) %"49", align 8
  %3 = inttoptr i64 %"60" to ptr
  %"59" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"59", ptr addrspace(5) %"50", align 8
  %"39" = load i8, ptr addrspace(4) @0, align 1
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"39")
  br label %"42"

"42":                                             ; preds = %"41"
  store i32 %"40", ptr addrspace(5) %"52", align 4
  %"63" = load i32, ptr addrspace(5) %"52", align 4
  %"62" = zext i32 %"63" to i64
  store i64 %"62", ptr addrspace(5) %"53", align 8
  %"65" = load i64, ptr addrspace(5) %"47", align 8
  %"66" = load i64, ptr addrspace(5) %"53", align 8
  %"76" = add i64 %"65", %"66"
  store i64 %"76", ptr addrspace(5) %"48", align 8
  %"68" = load i64, ptr addrspace(5) %"50", align 8
  %"69" = load i64, ptr addrspace(5) %"53", align 8
  %"78" = add i64 %"68", %"69"
  store i64 %"78", ptr addrspace(5) %"51", align 8
  %"71" = load i64, ptr addrspace(5) %"48", align 8
  %"80" = inttoptr i64 %"71" to ptr addrspace(1)
  %"70" = load i64, ptr addrspace(1) %"80", align 8
  store i64 %"70", ptr addrspace(5) %"54", align 8
  %"72" = load i64, ptr addrspace(5) %"51", align 8
  %"73" = load i64, ptr addrspace(5) %"54", align 8
  %"81" = inttoptr i64 %"72" to ptr addrspace(1)
  store i64 %"73", ptr addrspace(1) %"81", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }