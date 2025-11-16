declare hidden i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_mode_b32(ptr addrspace(4) byref(i64) %"54") #1 {
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"54", align 8
  store i64 %2, ptr addrspace(5) %"55", align 8
  %"37" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"52"

"52":                                             ; preds = %"51"
  store i32 %"37", ptr addrspace(5) %"57", align 4
  %3 = load i32, ptr addrspace(5) %"57", align 4
  %"90" = call i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32 %3, i32 3, i32 7680, i32 -1)
  store i32 %"90", ptr addrspace(5) %"58", align 4
  %4 = load i32, ptr addrspace(5) %"58", align 4
  store i32 %4, ptr addrspace(5) %"59", align 4
  %5 = load i32, ptr addrspace(5) %"57", align 4
  %"92" = call i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32 %5, i32 3, i32 7199, i32 -1)
  store i32 %"92", ptr addrspace(5) %"58", align 4
  %6 = load i32, ptr addrspace(5) %"59", align 4
  %7 = load i32, ptr addrspace(5) %"58", align 4
  %"68" = add i32 %6, %7
  store i32 %"68", ptr addrspace(5) %"59", align 4
  %8 = load i32, ptr addrspace(5) %"57", align 4
  %"94" = call i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32 %8, i32 3, i32 6175, i32 -1)
  store i32 %"94", ptr addrspace(5) %"58", align 4
  %9 = load i32, ptr addrspace(5) %"59", align 4
  %10 = load i32, ptr addrspace(5) %"58", align 4
  %"73" = add i32 %9, %10
  store i32 %"73", ptr addrspace(5) %"59", align 4
  %11 = load i32, ptr addrspace(5) %"57", align 4
  %"96" = call i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32 %11, i32 3, i32 4127, i32 -1)
  store i32 %"96", ptr addrspace(5) %"58", align 4
  %12 = load i32, ptr addrspace(5) %"59", align 4
  %13 = load i32, ptr addrspace(5) %"58", align 4
  %"78" = add i32 %12, %13
  store i32 %"78", ptr addrspace(5) %"59", align 4
  %14 = load i32, ptr addrspace(5) %"57", align 4
  %"81" = zext i32 %14 to i64
  store i64 %"81", ptr addrspace(5) %"56", align 8
  %15 = load i64, ptr addrspace(5) %"56", align 8
  %"83" = mul i64 %15, 4
  store i64 %"83", ptr addrspace(5) %"56", align 8
  %16 = load i64, ptr addrspace(5) %"55", align 8
  %17 = load i64, ptr addrspace(5) %"56", align 8
  %"85" = add i64 %16, %17
  store i64 %"85", ptr addrspace(5) %"55", align 8
  %18 = load i64, ptr addrspace(5) %"55", align 8
  %19 = load i32, ptr addrspace(5) %"59", align 4
  %"98" = inttoptr i64 %18 to ptr
  store i32 %19, ptr %"98", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }