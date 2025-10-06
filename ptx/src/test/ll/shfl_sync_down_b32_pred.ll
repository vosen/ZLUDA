declare hidden <2 x i32> @__zluda_ptx_impl_shfl_sync_down_b32_pred(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_down_b32_pred(ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"48", align 8
  store i64 %2, ptr addrspace(5) %"49", align 8
  %"39" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"46"

"46":                                             ; preds = %"45"
  store i32 %"39", ptr addrspace(5) %"51", align 4
  %3 = load i32, ptr addrspace(5) %"51", align 4
  %"74" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_down_b32_pred(i32 %3, i32 3, i32 31, i32 -1)
  %"71" = extractelement <2 x i32> %"74", i8 0
  %"75" = extractelement <2 x i32> %"74", i8 1
  %"57" = trunc i32 %"75" to i1
  store i32 %"71", ptr addrspace(5) %"52", align 4
  store i1 %"57", ptr addrspace(5) %"53", align 1
  %4 = load i1, ptr addrspace(5) %"53", align 1
  br i1 %4, label %"17", label %"16"

"16":                                             ; preds = %"46"
  %5 = load i32, ptr addrspace(5) %"52", align 4
  %"60" = add i32 %5, 1000
  store i32 %"60", ptr addrspace(5) %"52", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"46"
  %6 = load i32, ptr addrspace(5) %"51", align 4
  %"62" = zext i32 %6 to i64
  store i64 %"62", ptr addrspace(5) %"50", align 8
  %7 = load i64, ptr addrspace(5) %"50", align 8
  %"64" = mul i64 %7, 4
  store i64 %"64", ptr addrspace(5) %"50", align 8
  %8 = load i64, ptr addrspace(5) %"49", align 8
  %9 = load i64, ptr addrspace(5) %"50", align 8
  %"66" = add i64 %8, %9
  store i64 %"66", ptr addrspace(5) %"49", align 8
  %10 = load i64, ptr addrspace(5) %"49", align 8
  %11 = load i32, ptr addrspace(5) %"52", align 4
  %"73" = inttoptr i64 %10 to ptr
  store i32 %11, ptr %"73", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }