declare hidden i32 @__zluda_ptx_impl_redux_sync_max_u32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_add_u32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_min_u32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @redux_sync_op_u32(ptr addrspace(4) byref(i64) %"44") #1 {
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"51" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"51", ptr addrspace(5) %"50", align 8
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"42"

"42":                                             ; preds = %"41"
  store i32 %"36", ptr addrspace(5) %"45", align 4
  %"54" = load i32, ptr addrspace(5) %"45", align 4
  %"53" = call i32 @__zluda_ptx_impl_redux_sync_add_u32(i32 %"54", i32 -1)
  store i32 %"53", ptr addrspace(5) %"46", align 4
  %"56" = load i32, ptr addrspace(5) %"45", align 4
  %"55" = call i32 @__zluda_ptx_impl_redux_sync_min_u32(i32 %"56", i32 -1)
  store i32 %"55", ptr addrspace(5) %"47", align 4
  %"58" = load i32, ptr addrspace(5) %"45", align 4
  %"57" = call i32 @__zluda_ptx_impl_redux_sync_max_u32(i32 %"58", i32 -1)
  store i32 %"57", ptr addrspace(5) %"48", align 4
  %"60" = load i32, ptr addrspace(5) %"46", align 4
  %"61" = load i32, ptr addrspace(5) %"47", align 4
  %"59" = add i32 %"60", %"61"
  store i32 %"59", ptr addrspace(5) %"49", align 4
  %"63" = load i32, ptr addrspace(5) %"49", align 4
  %"64" = load i32, ptr addrspace(5) %"48", align 4
  %"62" = add i32 %"63", %"64"
  store i32 %"62", ptr addrspace(5) %"49", align 4
  %"67" = load i32, ptr addrspace(5) %"45", align 4
  %2 = zext i32 %"67" to i64
  %"66" = mul i64 %2, 4
  store i64 %"66", ptr addrspace(5) %"65", align 8
  %"69" = load i64, ptr addrspace(5) %"50", align 8
  %"70" = load i64, ptr addrspace(5) %"65", align 8
  %"68" = add i64 %"69", %"70"
  store i64 %"68", ptr addrspace(5) %"50", align 8
  %"71" = load i64, ptr addrspace(5) %"50", align 8
  %"72" = load i32, ptr addrspace(5) %"49", align 4
  %"73" = inttoptr i64 %"71" to ptr
  store i32 %"72", ptr %"73", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }