@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 3
@2 = addrspace(4) global i32 7680
@3 = addrspace(4) global i32 -1
@4 = addrspace(4) global i32 3
@5 = addrspace(4) global i32 7199
@6 = addrspace(4) global i32 -1
@7 = addrspace(4) global i32 3
@8 = addrspace(4) global i32 6175
@9 = addrspace(4) global i32 -1
@10 = addrspace(4) global i32 3
@11 = addrspace(4) global i32 4127
@12 = addrspace(4) global i32 -1
@13 = addrspace(4) global i64 4

declare hidden i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_mode_b32(ptr addrspace(4) byref(i64) %"64") #1 {
  %"65" = alloca i64, align 8, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i32, align 4, addrspace(5)
  %"68" = alloca i32, align 4, addrspace(5)
  %"69" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"61"

"61":                                             ; preds = %1
  %"70" = load i64, ptr addrspace(4) %"64", align 8
  store i64 %"70", ptr addrspace(5) %"65", align 8
  %"34" = call i32 @__zluda_ptx_impl_sreg_tid(i8 ptrtoint (ptr addrspace(4) @0 to i8))
  br label %"62"

"62":                                             ; preds = %"61"
  store i32 %"34", ptr addrspace(5) %"67", align 4
  %"36" = load i32, ptr addrspace(4) @1, align 4
  %"38" = load i32, ptr addrspace(4) @2, align 4
  %"40" = load i32, ptr addrspace(4) @3, align 4
  %"73" = load i32, ptr addrspace(5) %"67", align 4
  %"101" = call i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32 %"73", i32 %"36", i32 %"38", i32 %"40")
  store i32 %"101", ptr addrspace(5) %"68", align 4
  %"75" = load i32, ptr addrspace(5) %"68", align 4
  store i32 %"75", ptr addrspace(5) %"69", align 4
  %"42" = load i32, ptr addrspace(4) @4, align 4
  %"44" = load i32, ptr addrspace(4) @5, align 4
  %"46" = load i32, ptr addrspace(4) @6, align 4
  %"77" = load i32, ptr addrspace(5) %"67", align 4
  %"103" = call i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32 %"77", i32 %"42", i32 %"44", i32 %"46")
  store i32 %"103", ptr addrspace(5) %"68", align 4
  %"79" = load i32, ptr addrspace(5) %"69", align 4
  %"80" = load i32, ptr addrspace(5) %"68", align 4
  %"78" = add i32 %"79", %"80"
  store i32 %"78", ptr addrspace(5) %"69", align 4
  %"48" = load i32, ptr addrspace(4) @7, align 4
  %"50" = load i32, ptr addrspace(4) @8, align 4
  %"52" = load i32, ptr addrspace(4) @9, align 4
  %"82" = load i32, ptr addrspace(5) %"67", align 4
  %"105" = call i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32 %"82", i32 %"48", i32 %"50", i32 %"52")
  store i32 %"105", ptr addrspace(5) %"68", align 4
  %"84" = load i32, ptr addrspace(5) %"69", align 4
  %"85" = load i32, ptr addrspace(5) %"68", align 4
  %"83" = add i32 %"84", %"85"
  store i32 %"83", ptr addrspace(5) %"69", align 4
  %"54" = load i32, ptr addrspace(4) @10, align 4
  %"56" = load i32, ptr addrspace(4) @11, align 4
  %"58" = load i32, ptr addrspace(4) @12, align 4
  %"87" = load i32, ptr addrspace(5) %"67", align 4
  %"107" = call i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32 %"87", i32 %"54", i32 %"56", i32 %"58")
  store i32 %"107", ptr addrspace(5) %"68", align 4
  %"89" = load i32, ptr addrspace(5) %"69", align 4
  %"90" = load i32, ptr addrspace(5) %"68", align 4
  %"88" = add i32 %"89", %"90"
  store i32 %"88", ptr addrspace(5) %"69", align 4
  %"92" = load i32, ptr addrspace(5) %"67", align 4
  %"91" = zext i32 %"92" to i64
  store i64 %"91", ptr addrspace(5) %"66", align 8
  %"60" = load i64, ptr addrspace(4) @13, align 8
  %"94" = load i64, ptr addrspace(5) %"66", align 8
  %"93" = mul i64 %"94", %"60"
  store i64 %"93", ptr addrspace(5) %"66", align 8
  %"96" = load i64, ptr addrspace(5) %"65", align 8
  %"97" = load i64, ptr addrspace(5) %"66", align 8
  %"95" = add i64 %"96", %"97"
  store i64 %"95", ptr addrspace(5) %"65", align 8
  %"98" = load i64, ptr addrspace(5) %"65", align 8
  %"99" = load i32, ptr addrspace(5) %"69", align 4
  %"109" = inttoptr i64 %"98" to ptr
  store i32 %"99", ptr %"109", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }