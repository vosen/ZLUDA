declare hidden <2 x i32> @__zluda_ptx_impl_shfl_sync_bfly_b32_pred(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_bfly_b32_pred(ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i1, align 1, addrspace(5)
  %"52" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"52", align 1
  %"55" = alloca i32, align 4, addrspace(5)
  store i32 3, ptr addrspace(5) %"55", align 4
  %"56" = alloca i32, align 4, addrspace(5)
  store i32 31, ptr addrspace(5) %"56", align 4
  %"57" = alloca i32, align 4, addrspace(5)
  store i32 -1, ptr addrspace(5) %"57", align 4
  %"65" = alloca i32, align 4, addrspace(5)
  store i32 1000, ptr addrspace(5) %"65", align 4
  %"71" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"71", align 4
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %"51" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"51", ptr addrspace(5) %"46", align 8
  %"53" = load i8, ptr addrspace(5) %"52", align 1
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"53")
  br label %"43"

"43":                                             ; preds = %"42"
  store i32 %"36", ptr addrspace(5) %"48", align 4
  %"60" = load i32, ptr addrspace(5) %"48", align 4
  %"61" = load i32, ptr addrspace(5) %"55", align 4
  %"62" = load i32, ptr addrspace(5) %"56", align 4
  %"63" = load i32, ptr addrspace(5) %"57", align 4
  %"83" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_bfly_b32_pred(i32 %"60", i32 %"61", i32 %"62", i32 %"63")
  %"80" = extractelement <2 x i32> %"83", i8 0
  %"84" = extractelement <2 x i32> %"83", i8 1
  %"59" = trunc i32 %"84" to i1
  store i32 %"80", ptr addrspace(5) %"49", align 4
  store i1 %"59", ptr addrspace(5) %"50", align 1
  %"64" = load i1, ptr addrspace(5) %"50", align 1
  br i1 %"64", label %"16", label %"15"

"15":                                             ; preds = %"43"
  %"67" = load i32, ptr addrspace(5) %"49", align 4
  %"68" = load i32, ptr addrspace(5) %"65", align 4
  %"66" = add i32 %"67", %"68"
  store i32 %"66", ptr addrspace(5) %"49", align 4
  br label %"16"

"16":                                             ; preds = %"15", %"43"
  %"70" = load i32, ptr addrspace(5) %"48", align 4
  %"69" = zext i32 %"70" to i64
  store i64 %"69", ptr addrspace(5) %"47", align 8
  %"73" = load i64, ptr addrspace(5) %"47", align 8
  %"74" = load i64, ptr addrspace(5) %"71", align 8
  %"72" = mul i64 %"73", %"74"
  store i64 %"72", ptr addrspace(5) %"47", align 8
  %"76" = load i64, ptr addrspace(5) %"46", align 8
  %"77" = load i64, ptr addrspace(5) %"47", align 8
  %"75" = add i64 %"76", %"77"
  store i64 %"75", ptr addrspace(5) %"46", align 8
  %"78" = load i64, ptr addrspace(5) %"46", align 8
  %"79" = load i32, ptr addrspace(5) %"49", align 4
  %"82" = inttoptr i64 %"78" to ptr
  store i32 %"79", ptr %"82", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }