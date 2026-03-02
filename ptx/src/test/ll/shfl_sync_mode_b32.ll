declare hidden i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_mode_b32(ptr addrspace(4) byref(i64) %"56") #1 {
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i32, align 4, addrspace(5)
  %"60" = alloca i32, align 4, addrspace(5)
  %"61" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"54"

"54":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"56", align 8
  store i64 %2, ptr addrspace(5) %"57", align 8
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"40", ptr addrspace(5) %"59", align 4
  %3 = load i32, ptr addrspace(5) %"59", align 4
  %"92" = call i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32 %3, i32 3, i32 7680, i32 -1)
  store i32 %"92", ptr addrspace(5) %"60", align 4
  %4 = load i32, ptr addrspace(5) %"60", align 4
  store i32 %4, ptr addrspace(5) %"61", align 4
  %5 = load i32, ptr addrspace(5) %"59", align 4
  %"94" = call i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32 %5, i32 3, i32 7199, i32 -1)
  store i32 %"94", ptr addrspace(5) %"60", align 4
  %6 = load i32, ptr addrspace(5) %"61", align 4
  %7 = load i32, ptr addrspace(5) %"60", align 4
  %"70" = add i32 %6, %7
  store i32 %"70", ptr addrspace(5) %"61", align 4
  %8 = load i32, ptr addrspace(5) %"59", align 4
  %"96" = call i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32 %8, i32 3, i32 6175, i32 -1)
  store i32 %"96", ptr addrspace(5) %"60", align 4
  %9 = load i32, ptr addrspace(5) %"61", align 4
  %10 = load i32, ptr addrspace(5) %"60", align 4
  %"75" = add i32 %9, %10
  store i32 %"75", ptr addrspace(5) %"61", align 4
  %11 = load i32, ptr addrspace(5) %"59", align 4
  %"98" = call i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32 %11, i32 3, i32 4127, i32 -1)
  store i32 %"98", ptr addrspace(5) %"60", align 4
  %12 = load i32, ptr addrspace(5) %"61", align 4
  %13 = load i32, ptr addrspace(5) %"60", align 4
  %"80" = add i32 %12, %13
  store i32 %"80", ptr addrspace(5) %"61", align 4
  %14 = load i32, ptr addrspace(5) %"59", align 4
  %15 = zext i32 %14 to i64
  store i64 %15, ptr addrspace(5) %"58", align 8
  %16 = load i64, ptr addrspace(5) %"58", align 8
  %"85" = mul i64 %16, 4
  store i64 %"85", ptr addrspace(5) %"58", align 8
  %17 = load i64, ptr addrspace(5) %"57", align 8
  %18 = load i64, ptr addrspace(5) %"58", align 8
  %"87" = add i64 %17, %18
  store i64 %"87", ptr addrspace(5) %"57", align 8
  %19 = load i64, ptr addrspace(5) %"57", align 8
  %20 = load i32, ptr addrspace(5) %"61", align 4
  %"100" = inttoptr i64 %19 to ptr
  store i32 %20, ptr %"100", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
