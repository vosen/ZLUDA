declare hidden <2 x i32> @__zluda_ptx_impl_shfl_sync_up_b32_pred(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_up_b32_pred(ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %"51" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"51", ptr addrspace(5) %"46", align 8
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"43"

"43":                                             ; preds = %"42"
  store i32 %"36", ptr addrspace(5) %"48", align 4
  %"55" = load i32, ptr addrspace(5) %"48", align 4
  %"71" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_up_b32_pred(i32 %"55", i32 3, i32 0, i32 -1)
  %"68" = extractelement <2 x i32> %"71", i8 0
  %"72" = extractelement <2 x i32> %"71", i8 1
  %"54" = trunc i32 %"72" to i1
  store i32 %"68", ptr addrspace(5) %"49", align 4
  store i1 %"54", ptr addrspace(5) %"50", align 1
  %"56" = load i1, ptr addrspace(5) %"50", align 1
  br i1 %"56", label %"16", label %"15"

"15":                                             ; preds = %"43"
  %"58" = load i32, ptr addrspace(5) %"49", align 4
  %"57" = add i32 %"58", 1000
  store i32 %"57", ptr addrspace(5) %"49", align 4
  br label %"16"

"16":                                             ; preds = %"15", %"43"
  %"60" = load i32, ptr addrspace(5) %"48", align 4
  %"59" = zext i32 %"60" to i64
  store i64 %"59", ptr addrspace(5) %"47", align 8
  %"62" = load i64, ptr addrspace(5) %"47", align 8
  %"61" = mul i64 %"62", 4
  store i64 %"61", ptr addrspace(5) %"47", align 8
  %"64" = load i64, ptr addrspace(5) %"46", align 8
  %"65" = load i64, ptr addrspace(5) %"47", align 8
  %"63" = add i64 %"64", %"65"
  store i64 %"63", ptr addrspace(5) %"46", align 8
  %"66" = load i64, ptr addrspace(5) %"46", align 8
  %"67" = load i32, ptr addrspace(5) %"49", align 4
  %"70" = inttoptr i64 %"66" to ptr
  store i32 %"67", ptr %"70", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
