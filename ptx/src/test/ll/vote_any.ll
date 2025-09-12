declare hidden i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_any(ptr addrspace(4) byref(i64) %"44") #1 {
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i1, align 1, addrspace(5)
  %"47" = alloca i1, align 1, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"50", ptr addrspace(5) %"49", align 8
  %"35" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"42"

"42":                                             ; preds = %"41"
  store i32 %"35", ptr addrspace(5) %"45", align 4
  %"53" = load i32, ptr addrspace(5) %"45", align 4
  %2 = icmp uge i32 %"53", 32
  store i1 %2, ptr addrspace(5) %"46", align 1
  %"55" = load i1, ptr addrspace(5) %"46", align 1
  %"54" = call i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1 %"55", i32 -1)
  store i1 %"54", ptr addrspace(5) %"47", align 1
  %"57" = load i1, ptr addrspace(5) %"47", align 1
  %"56" = select i1 %"57", i32 1, i32 0
  store i32 %"56", ptr addrspace(5) %"48", align 4
  %"60" = load i32, ptr addrspace(5) %"45", align 4
  %3 = zext i32 %"60" to i64
  %"59" = mul i64 %3, 4
  store i64 %"59", ptr addrspace(5) %"58", align 8
  %"62" = load i64, ptr addrspace(5) %"49", align 8
  %"63" = load i64, ptr addrspace(5) %"58", align 8
  %"61" = add i64 %"62", %"63"
  store i64 %"61", ptr addrspace(5) %"49", align 8
  %"64" = load i64, ptr addrspace(5) %"49", align 8
  %"65" = load i32, ptr addrspace(5) %"48", align 4
  %"66" = inttoptr i64 %"64" to ptr
  store i32 %"65", ptr %"66", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }