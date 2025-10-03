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
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"57" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"57", ptr addrspace(5) %"52", align 8
  %"34" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"49"

"49":                                             ; preds = %"48"
  store i32 %"34", ptr addrspace(5) %"54", align 4
  %"60" = load i32, ptr addrspace(5) %"54", align 4
  %"87" = call i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32 %"60", i32 3, i32 7680, i32 -1)
  store i32 %"87", ptr addrspace(5) %"55", align 4
  %"62" = load i32, ptr addrspace(5) %"55", align 4
  store i32 %"62", ptr addrspace(5) %"56", align 4
  %"64" = load i32, ptr addrspace(5) %"54", align 4
  %"89" = call i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32 %"64", i32 3, i32 7199, i32 -1)
  store i32 %"89", ptr addrspace(5) %"55", align 4
  %"66" = load i32, ptr addrspace(5) %"56", align 4
  %"67" = load i32, ptr addrspace(5) %"55", align 4
  %"65" = add i32 %"66", %"67"
  store i32 %"65", ptr addrspace(5) %"56", align 4
  %"69" = load i32, ptr addrspace(5) %"54", align 4
  %"91" = call i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32 %"69", i32 3, i32 6175, i32 -1)
  store i32 %"91", ptr addrspace(5) %"55", align 4
  %"71" = load i32, ptr addrspace(5) %"56", align 4
  %"72" = load i32, ptr addrspace(5) %"55", align 4
  %"70" = add i32 %"71", %"72"
  store i32 %"70", ptr addrspace(5) %"56", align 4
  %"74" = load i32, ptr addrspace(5) %"54", align 4
  %"93" = call i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32 %"74", i32 3, i32 4127, i32 -1)
  store i32 %"93", ptr addrspace(5) %"55", align 4
  %"76" = load i32, ptr addrspace(5) %"56", align 4
  %"77" = load i32, ptr addrspace(5) %"55", align 4
  %"75" = add i32 %"76", %"77"
  store i32 %"75", ptr addrspace(5) %"56", align 4
  %"79" = load i32, ptr addrspace(5) %"54", align 4
  %"78" = zext i32 %"79" to i64
  store i64 %"78", ptr addrspace(5) %"53", align 8
  %"81" = load i64, ptr addrspace(5) %"53", align 8
  %"80" = mul i64 %"81", 4
  store i64 %"80", ptr addrspace(5) %"53", align 8
  %"83" = load i64, ptr addrspace(5) %"52", align 8
  %"84" = load i64, ptr addrspace(5) %"53", align 8
  %"82" = add i64 %"83", %"84"
  store i64 %"82", ptr addrspace(5) %"52", align 8
  %"85" = load i64, ptr addrspace(5) %"52", align 8
  %"86" = load i32, ptr addrspace(5) %"56", align 4
  %"95" = inttoptr i64 %"85" to ptr
  store i32 %"86", ptr %"95", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
