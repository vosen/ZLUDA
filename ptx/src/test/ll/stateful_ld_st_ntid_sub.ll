@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
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
  %"74" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"74", ptr addrspace(5) %"46", align 4
  %"75" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"75", ptr addrspace(5) %"49", align 4
  %"58" = load i64, ptr addrspace(5) %"46", align 4
  %2 = inttoptr i64 %"58" to ptr
  %"57" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"57", ptr addrspace(5) %"47", align 8
  %"60" = load i64, ptr addrspace(5) %"49", align 4
  %3 = inttoptr i64 %"60" to ptr
  %"59" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"59", ptr addrspace(5) %"50", align 8
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"42"

"42":                                             ; preds = %"41"
  store i32 %"36", ptr addrspace(5) %"52", align 4
  %"63" = load i32, ptr addrspace(5) %"52", align 4
  %"62" = zext i32 %"63" to i64
  store i64 %"62", ptr addrspace(5) %"53", align 4
  %"65" = load i64, ptr addrspace(5) %"47", align 4
  %"66" = load i64, ptr addrspace(5) %"53", align 4
  %"76" = sub i64 %"65", %"66"
  store i64 %"76", ptr addrspace(5) %"48", align 4
  %"68" = load i64, ptr addrspace(5) %"50", align 4
  %"69" = load i64, ptr addrspace(5) %"53", align 4
  %"79" = sub i64 %"68", %"69"
  store i64 %"79", ptr addrspace(5) %"51", align 4
  %"70" = load i64, ptr addrspace(5) %"48", align 4
  %"82" = inttoptr i64 %"70" to ptr addrspace(1)
  %"38" = getelementptr inbounds i8, ptr addrspace(1) %"82", i64 0
  %"71" = load i64, ptr addrspace(1) %"38", align 4
  store i64 %"71", ptr addrspace(5) %"54", align 4
  %"72" = load i64, ptr addrspace(5) %"51", align 4
  %"83" = inttoptr i64 %"72" to ptr addrspace(1)
  %"40" = getelementptr inbounds i8, ptr addrspace(1) %"83", i64 0
  %"73" = load i64, ptr addrspace(5) %"54", align 4
  store i64 %"73", ptr addrspace(1) %"40", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }