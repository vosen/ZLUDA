declare hidden i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_mode_b32(ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"58", align 1
  %"61" = alloca i32, align 4, addrspace(5)
  store i32 3, ptr addrspace(5) %"61", align 4
  %"62" = alloca i32, align 4, addrspace(5)
  store i32 7680, ptr addrspace(5) %"62", align 4
  %"63" = alloca i32, align 4, addrspace(5)
  store i32 -1, ptr addrspace(5) %"63", align 4
  %"71" = alloca i32, align 4, addrspace(5)
  store i32 3, ptr addrspace(5) %"71", align 4
  %"72" = alloca i32, align 4, addrspace(5)
  store i32 7199, ptr addrspace(5) %"72", align 4
  %"73" = alloca i32, align 4, addrspace(5)
  store i32 -1, ptr addrspace(5) %"73", align 4
  %"82" = alloca i32, align 4, addrspace(5)
  store i32 3, ptr addrspace(5) %"82", align 4
  %"83" = alloca i32, align 4, addrspace(5)
  store i32 6175, ptr addrspace(5) %"83", align 4
  %"84" = alloca i32, align 4, addrspace(5)
  store i32 -1, ptr addrspace(5) %"84", align 4
  %"93" = alloca i32, align 4, addrspace(5)
  store i32 3, ptr addrspace(5) %"93", align 4
  %"94" = alloca i32, align 4, addrspace(5)
  store i32 4127, ptr addrspace(5) %"94", align 4
  %"95" = alloca i32, align 4, addrspace(5)
  store i32 -1, ptr addrspace(5) %"95", align 4
  %"106" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"106", align 4
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"57" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"57", ptr addrspace(5) %"52", align 8
  %"59" = load i8, ptr addrspace(5) %"58", align 1
  %"34" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"59")
  br label %"49"

"49":                                             ; preds = %"48"
  store i32 %"34", ptr addrspace(5) %"54", align 4
  %"65" = load i32, ptr addrspace(5) %"54", align 4
  %"66" = load i32, ptr addrspace(5) %"61", align 4
  %"67" = load i32, ptr addrspace(5) %"62", align 4
  %"68" = load i32, ptr addrspace(5) %"63", align 4
  %"115" = call i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32 %"65", i32 %"66", i32 %"67", i32 %"68")
  store i32 %"115", ptr addrspace(5) %"55", align 4
  %"70" = load i32, ptr addrspace(5) %"55", align 4
  store i32 %"70", ptr addrspace(5) %"56", align 4
  %"75" = load i32, ptr addrspace(5) %"54", align 4
  %"76" = load i32, ptr addrspace(5) %"71", align 4
  %"77" = load i32, ptr addrspace(5) %"72", align 4
  %"78" = load i32, ptr addrspace(5) %"73", align 4
  %"117" = call i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32 %"75", i32 %"76", i32 %"77", i32 %"78")
  store i32 %"117", ptr addrspace(5) %"55", align 4
  %"80" = load i32, ptr addrspace(5) %"56", align 4
  %"81" = load i32, ptr addrspace(5) %"55", align 4
  %"79" = add i32 %"80", %"81"
  store i32 %"79", ptr addrspace(5) %"56", align 4
  %"86" = load i32, ptr addrspace(5) %"54", align 4
  %"87" = load i32, ptr addrspace(5) %"82", align 4
  %"88" = load i32, ptr addrspace(5) %"83", align 4
  %"89" = load i32, ptr addrspace(5) %"84", align 4
  %"119" = call i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32 %"86", i32 %"87", i32 %"88", i32 %"89")
  store i32 %"119", ptr addrspace(5) %"55", align 4
  %"91" = load i32, ptr addrspace(5) %"56", align 4
  %"92" = load i32, ptr addrspace(5) %"55", align 4
  %"90" = add i32 %"91", %"92"
  store i32 %"90", ptr addrspace(5) %"56", align 4
  %"97" = load i32, ptr addrspace(5) %"54", align 4
  %"98" = load i32, ptr addrspace(5) %"93", align 4
  %"99" = load i32, ptr addrspace(5) %"94", align 4
  %"100" = load i32, ptr addrspace(5) %"95", align 4
  %"121" = call i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32 %"97", i32 %"98", i32 %"99", i32 %"100")
  store i32 %"121", ptr addrspace(5) %"55", align 4
  %"102" = load i32, ptr addrspace(5) %"56", align 4
  %"103" = load i32, ptr addrspace(5) %"55", align 4
  %"101" = add i32 %"102", %"103"
  store i32 %"101", ptr addrspace(5) %"56", align 4
  %"105" = load i32, ptr addrspace(5) %"54", align 4
  %"104" = zext i32 %"105" to i64
  store i64 %"104", ptr addrspace(5) %"53", align 8
  %"108" = load i64, ptr addrspace(5) %"53", align 8
  %"109" = load i64, ptr addrspace(5) %"106", align 8
  %"107" = mul i64 %"108", %"109"
  store i64 %"107", ptr addrspace(5) %"53", align 8
  %"111" = load i64, ptr addrspace(5) %"52", align 8
  %"112" = load i64, ptr addrspace(5) %"53", align 8
  %"110" = add i64 %"111", %"112"
  store i64 %"110", ptr addrspace(5) %"52", align 8
  %"113" = load i64, ptr addrspace(5) %"52", align 8
  %"114" = load i32, ptr addrspace(5) %"56", align 4
  %"123" = inttoptr i64 %"113" to ptr
  store i32 %"114", ptr %"123", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }