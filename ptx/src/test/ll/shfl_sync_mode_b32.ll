@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

declare i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32, i32, i32, i32) #0

declare i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32, i32, i32, i32) #0

declare i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32, i32, i32, i32) #0

declare i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32, i32, i32, i32) #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_mode_b32(ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %"54" = load i64, ptr addrspace(4) %"48", align 4
  store i64 %"54", ptr addrspace(5) %"49", align 4
  %"31" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"46"

"46":                                             ; preds = %"45"
  store i32 %"31", ptr addrspace(5) %"51", align 4
  %"57" = load i32, ptr addrspace(5) %"51", align 4
  %"84" = call i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32 %"57", i32 3, i32 7680, i32 -1)
  store i32 %"84", ptr addrspace(5) %"52", align 4
  %"59" = load i32, ptr addrspace(5) %"52", align 4
  store i32 %"59", ptr addrspace(5) %"53", align 4
  %"61" = load i32, ptr addrspace(5) %"51", align 4
  %"86" = call i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32 %"61", i32 3, i32 7199, i32 -1)
  store i32 %"86", ptr addrspace(5) %"52", align 4
  %"63" = load i32, ptr addrspace(5) %"53", align 4
  %"64" = load i32, ptr addrspace(5) %"52", align 4
  %"62" = add i32 %"63", %"64"
  store i32 %"62", ptr addrspace(5) %"53", align 4
  %"66" = load i32, ptr addrspace(5) %"51", align 4
  %"88" = call i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32 %"66", i32 3, i32 6175, i32 -1)
  store i32 %"88", ptr addrspace(5) %"52", align 4
  %"68" = load i32, ptr addrspace(5) %"53", align 4
  %"69" = load i32, ptr addrspace(5) %"52", align 4
  %"67" = add i32 %"68", %"69"
  store i32 %"67", ptr addrspace(5) %"53", align 4
  %"71" = load i32, ptr addrspace(5) %"51", align 4
  %"90" = call i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32 %"71", i32 3, i32 4127, i32 -1)
  store i32 %"90", ptr addrspace(5) %"52", align 4
  %"73" = load i32, ptr addrspace(5) %"53", align 4
  %"74" = load i32, ptr addrspace(5) %"52", align 4
  %"72" = add i32 %"73", %"74"
  store i32 %"72", ptr addrspace(5) %"53", align 4
  %"76" = load i32, ptr addrspace(5) %"51", align 4
  %"75" = zext i32 %"76" to i64
  store i64 %"75", ptr addrspace(5) %"50", align 4
  %"78" = load i64, ptr addrspace(5) %"50", align 4
  %"77" = mul i64 %"78", 4
  store i64 %"77", ptr addrspace(5) %"50", align 4
  %"80" = load i64, ptr addrspace(5) %"49", align 4
  %"81" = load i64, ptr addrspace(5) %"50", align 4
  %"79" = add i64 %"80", %"81"
  store i64 %"79", ptr addrspace(5) %"49", align 4
  %"82" = load i64, ptr addrspace(5) %"49", align 4
  %"83" = load i32, ptr addrspace(5) %"53", align 4
  %"92" = inttoptr i64 %"82" to ptr
  store i32 %"83", ptr %"92", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }