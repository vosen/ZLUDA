declare hidden i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_mode_b32(ptr addrspace(4) byref(i64) %"53") #1 {
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"53", align 8
  store i64 %2, ptr addrspace(5) %"54", align 8
  %"37" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"37", ptr addrspace(5) %"56", align 4
  %3 = load i32, ptr addrspace(5) %"56", align 4
  %"89" = call i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32 %3, i32 3, i32 7680, i32 -1)
  store i32 %"89", ptr addrspace(5) %"57", align 4
  %4 = load i32, ptr addrspace(5) %"57", align 4
  store i32 %4, ptr addrspace(5) %"58", align 4
  %5 = load i32, ptr addrspace(5) %"56", align 4
  %"91" = call i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32 %5, i32 3, i32 7199, i32 -1)
  store i32 %"91", ptr addrspace(5) %"57", align 4
  %6 = load i32, ptr addrspace(5) %"58", align 4
  %7 = load i32, ptr addrspace(5) %"57", align 4
  %"67" = add i32 %6, %7
  store i32 %"67", ptr addrspace(5) %"58", align 4
  %8 = load i32, ptr addrspace(5) %"56", align 4
  %"93" = call i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32 %8, i32 3, i32 6175, i32 -1)
  store i32 %"93", ptr addrspace(5) %"57", align 4
  %9 = load i32, ptr addrspace(5) %"58", align 4
  %10 = load i32, ptr addrspace(5) %"57", align 4
  %"72" = add i32 %9, %10
  store i32 %"72", ptr addrspace(5) %"58", align 4
  %11 = load i32, ptr addrspace(5) %"56", align 4
  %"95" = call i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32 %11, i32 3, i32 4127, i32 -1)
  store i32 %"95", ptr addrspace(5) %"57", align 4
  %12 = load i32, ptr addrspace(5) %"58", align 4
  %13 = load i32, ptr addrspace(5) %"57", align 4
  %"77" = add i32 %12, %13
  store i32 %"77", ptr addrspace(5) %"58", align 4
  %14 = load i32, ptr addrspace(5) %"56", align 4
  %15 = zext i32 %14 to i64
  store i64 %15, ptr addrspace(5) %"55", align 8
  %16 = load i64, ptr addrspace(5) %"55", align 8
  %"82" = mul i64 %16, 4
  store i64 %"82", ptr addrspace(5) %"55", align 8
  %17 = load i64, ptr addrspace(5) %"54", align 8
  %18 = load i64, ptr addrspace(5) %"55", align 8
  %"84" = add i64 %17, %18
  store i64 %"84", ptr addrspace(5) %"54", align 8
  %19 = load i64, ptr addrspace(5) %"54", align 8
  %20 = load i32, ptr addrspace(5) %"58", align 4
  %"97" = inttoptr i64 %19 to ptr
  store i32 %20, ptr %"97", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }