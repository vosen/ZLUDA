declare hidden i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_any(ptr addrspace(4) byref(i64) %"44") #1 {
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i1, align 1, addrspace(5)
  %"47" = alloca i1, align 1, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"51", align 1
  %"54" = alloca i32, align 4, addrspace(5)
  store i32 32, ptr addrspace(5) %"54", align 4
  %"58" = alloca i32, align 4, addrspace(5)
  store i32 -1, ptr addrspace(5) %"58", align 4
  %"62" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"62", align 4
  %"63" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"63", align 4
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i32, align 4, addrspace(5)
  store i32 4, ptr addrspace(5) %"69", align 4
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"50", ptr addrspace(5) %"49", align 8
  %"52" = load i8, ptr addrspace(5) %"51", align 1
  %"35" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"52")
  br label %"42"

"42":                                             ; preds = %"41"
  store i32 %"35", ptr addrspace(5) %"45", align 4
  %"56" = load i32, ptr addrspace(5) %"45", align 4
  %"57" = load i32, ptr addrspace(5) %"54", align 4
  %2 = icmp uge i32 %"56", %"57"
  store i1 %2, ptr addrspace(5) %"46", align 1
  %"60" = load i1, ptr addrspace(5) %"46", align 1
  %"61" = load i32, ptr addrspace(5) %"58", align 4
  %"59" = call i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1 %"60", i32 %"61")
  store i1 %"59", ptr addrspace(5) %"47", align 1
  %"65" = load i32, ptr addrspace(5) %"62", align 4
  %"66" = load i32, ptr addrspace(5) %"63", align 4
  %"67" = load i1, ptr addrspace(5) %"47", align 1
  %"64" = select i1 %"67", i32 %"65", i32 %"66"
  store i32 %"64", ptr addrspace(5) %"48", align 4
  %"71" = load i32, ptr addrspace(5) %"45", align 4
  %"72" = load i32, ptr addrspace(5) %"69", align 4
  %3 = zext i32 %"71" to i64
  %4 = zext i32 %"72" to i64
  %"70" = mul i64 %3, %4
  store i64 %"70", ptr addrspace(5) %"68", align 8
  %"74" = load i64, ptr addrspace(5) %"49", align 8
  %"75" = load i64, ptr addrspace(5) %"68", align 8
  %"73" = add i64 %"74", %"75"
  store i64 %"73", ptr addrspace(5) %"49", align 8
  %"76" = load i64, ptr addrspace(5) %"49", align 8
  %"77" = load i32, ptr addrspace(5) %"48", align 4
  %"78" = inttoptr i64 %"76" to ptr
  store i32 %"77", ptr %"78", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }