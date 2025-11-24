declare hidden <2 x i32> @__zluda_ptx_impl_shfl_sync_idx_b32_pred(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_idx_b32_pred(ptr addrspace(4) byref(i64) %"47") #1 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %2, ptr addrspace(5) %"48", align 8
  %"39" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"39", ptr addrspace(5) %"50", align 4
  %3 = load i32, ptr addrspace(5) %"50", align 4
  %"73" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_idx_b32_pred(i32 %3, i32 12, i32 31, i32 -1)
  %"70" = extractelement <2 x i32> %"73", i8 0
  %"74" = extractelement <2 x i32> %"73", i8 1
  %4 = trunc i32 %"74" to i1
  store i32 %"70", ptr addrspace(5) %"51", align 4
  store i1 %4, ptr addrspace(5) %"52", align 1
  %5 = load i1, ptr addrspace(5) %"52", align 1
  br i1 %5, label %"17", label %"16"

"16":                                             ; preds = %"45"
  %6 = load i32, ptr addrspace(5) %"51", align 4
  %"59" = add i32 %6, 1000
  store i32 %"59", ptr addrspace(5) %"51", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"45"
  %7 = load i32, ptr addrspace(5) %"50", align 4
  %8 = zext i32 %7 to i64
  store i64 %8, ptr addrspace(5) %"49", align 8
  %9 = load i64, ptr addrspace(5) %"49", align 8
  %"63" = mul i64 %9, 4
  store i64 %"63", ptr addrspace(5) %"49", align 8
  %10 = load i64, ptr addrspace(5) %"48", align 8
  %11 = load i64, ptr addrspace(5) %"49", align 8
  %"65" = add i64 %10, %11
  store i64 %"65", ptr addrspace(5) %"48", align 8
  %12 = load i64, ptr addrspace(5) %"48", align 8
  %13 = load i32, ptr addrspace(5) %"51", align 4
  %"72" = inttoptr i64 %12 to ptr
  store i32 %13, ptr %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }