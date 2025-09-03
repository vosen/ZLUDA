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

define amdgpu_kernel void @shfl_sync_mode_b32(ptr addrspace(4) byref(i64) %"65") #1 {
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i32, align 4, addrspace(5)
  %"69" = alloca i32, align 4, addrspace(5)
  %"70" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"62"

"62":                                             ; preds = %1
  %"71" = load i64, ptr addrspace(4) %"65", align 8
  store i64 %"71", ptr addrspace(5) %"66", align 8
  %"34" = load i8, ptr addrspace(4) @0, align 1
  %"35" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"34")
  br label %"63"

"63":                                             ; preds = %"62"
  store i32 %"35", ptr addrspace(5) %"68", align 4
  %"37" = load i32, ptr addrspace(4) @1, align 4
  %"39" = load i32, ptr addrspace(4) @2, align 4
  %"41" = load i32, ptr addrspace(4) @3, align 4
  %"74" = load i32, ptr addrspace(5) %"68", align 4
  %"101" = call i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32 %"74", i32 %"37", i32 %"39", i32 %"41")
  store i32 %"101", ptr addrspace(5) %"69", align 4
  %"76" = load i32, ptr addrspace(5) %"69", align 4
  store i32 %"76", ptr addrspace(5) %"70", align 4
  %"43" = load i32, ptr addrspace(4) @4, align 4
  %"45" = load i32, ptr addrspace(4) @5, align 4
  %"47" = load i32, ptr addrspace(4) @6, align 4
  %"78" = load i32, ptr addrspace(5) %"68", align 4
  %"103" = call i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32 %"78", i32 %"43", i32 %"45", i32 %"47")
  store i32 %"103", ptr addrspace(5) %"69", align 4
  %"80" = load i32, ptr addrspace(5) %"70", align 4
  %"81" = load i32, ptr addrspace(5) %"69", align 4
  %"79" = add i32 %"80", %"81"
  store i32 %"79", ptr addrspace(5) %"70", align 4
  %"49" = load i32, ptr addrspace(4) @7, align 4
  %"51" = load i32, ptr addrspace(4) @8, align 4
  %"53" = load i32, ptr addrspace(4) @9, align 4
  %"83" = load i32, ptr addrspace(5) %"68", align 4
  %"105" = call i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32 %"83", i32 %"49", i32 %"51", i32 %"53")
  store i32 %"105", ptr addrspace(5) %"69", align 4
  %"85" = load i32, ptr addrspace(5) %"70", align 4
  %"86" = load i32, ptr addrspace(5) %"69", align 4
  %"84" = add i32 %"85", %"86"
  store i32 %"84", ptr addrspace(5) %"70", align 4
  %"55" = load i32, ptr addrspace(4) @10, align 4
  %"57" = load i32, ptr addrspace(4) @11, align 4
  %"59" = load i32, ptr addrspace(4) @12, align 4
  %"88" = load i32, ptr addrspace(5) %"68", align 4
  %"107" = call i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32 %"88", i32 %"55", i32 %"57", i32 %"59")
  store i32 %"107", ptr addrspace(5) %"69", align 4
  %"90" = load i32, ptr addrspace(5) %"70", align 4
  %"91" = load i32, ptr addrspace(5) %"69", align 4
  %"89" = add i32 %"90", %"91"
  store i32 %"89", ptr addrspace(5) %"70", align 4
  %"93" = load i32, ptr addrspace(5) %"68", align 4
  %"92" = zext i32 %"93" to i64
  store i64 %"92", ptr addrspace(5) %"67", align 8
  %"61" = load i64, ptr addrspace(4) @13, align 8
  %"95" = load i64, ptr addrspace(5) %"67", align 8
  %"94" = mul i64 %"95", %"61"
  store i64 %"94", ptr addrspace(5) %"67", align 8
  %"97" = load i64, ptr addrspace(5) %"66", align 8
  %"98" = load i64, ptr addrspace(5) %"67", align 8
  %"96" = add i64 %"97", %"98"
  store i64 %"96", ptr addrspace(5) %"66", align 8
  %"99" = load i64, ptr addrspace(5) %"66", align 8
  %"100" = load i32, ptr addrspace(5) %"70", align 4
  %"109" = inttoptr i64 %"99" to ptr
  store i32 %"100", ptr %"109", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }