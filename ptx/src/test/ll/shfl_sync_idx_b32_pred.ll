declare hidden <2 x i32> @__zluda_ptx_impl_shfl_sync_idx_b32_pred(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_idx_b32_pred(ptr addrspace(4) byref(i64) %"50") #1 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %"42" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"42", ptr addrspace(5) %"53", align 4
  %3 = load i32, ptr addrspace(5) %"53", align 4
  %"76" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_idx_b32_pred(i32 %3, i32 12, i32 31, i32 -1)
  %"73" = extractelement <2 x i32> %"76", i8 0
  %"77" = extractelement <2 x i32> %"76", i8 1
  %4 = trunc i32 %"77" to i1
  store i32 %"73", ptr addrspace(5) %"54", align 4
  store i1 %4, ptr addrspace(5) %"55", align 1
  %5 = load i1, ptr addrspace(5) %"55", align 1
  br i1 %5, label %"18", label %"17"

"17":                                             ; preds = %"48"
  %6 = load i32, ptr addrspace(5) %"54", align 4
  %"62" = add i32 %6, 1000
  store i32 %"62", ptr addrspace(5) %"54", align 4
  br label %"18"

"18":                                             ; preds = %"17", %"48"
  %7 = load i32, ptr addrspace(5) %"53", align 4
  %8 = zext i32 %7 to i64
  store i64 %8, ptr addrspace(5) %"52", align 8
  %9 = load i64, ptr addrspace(5) %"52", align 8
  %"66" = mul i64 %9, 4
  store i64 %"66", ptr addrspace(5) %"52", align 8
  %10 = load i64, ptr addrspace(5) %"51", align 8
  %11 = load i64, ptr addrspace(5) %"52", align 8
  %"68" = add i64 %10, %11
  store i64 %"68", ptr addrspace(5) %"51", align 8
  %12 = load i64, ptr addrspace(5) %"51", align 8
  %13 = load i32, ptr addrspace(5) %"54", align 4
  %"75" = inttoptr i64 %12 to ptr
  store i32 %13, ptr %"75", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
