@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 12
@2 = addrspace(4) global i32 31
@3 = addrspace(4) global i32 -1
@4 = addrspace(4) global i32 1000
@5 = addrspace(4) global i64 4

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
  br label %"47"

"47":                                             ; preds = %1
  %"56" = load i64, ptr addrspace(4) %"50", align 8
  store i64 %"56", ptr addrspace(5) %"51", align 8
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 ptrtoint (ptr addrspace(4) @0 to i8))
  br label %"48"

"48":                                             ; preds = %"47"
  store i32 %"36", ptr addrspace(5) %"53", align 4
  %"38" = load i32, ptr addrspace(4) @1, align 4
  %"40" = load i32, ptr addrspace(4) @2, align 4
  %"42" = load i32, ptr addrspace(4) @3, align 4
  %"60" = load i32, ptr addrspace(5) %"53", align 4
  %"77" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_idx_b32_pred(i32 %"60", i32 %"38", i32 %"40", i32 %"42")
  %"74" = extractelement <2 x i32> %"77", i8 0
  %"78" = extractelement <2 x i32> %"77", i8 1
  %"59" = trunc i32 %"78" to i1
  store i32 %"74", ptr addrspace(5) %"54", align 4
  store i1 %"59", ptr addrspace(5) %"55", align 1
  %"61" = load i1, ptr addrspace(5) %"55", align 1
  br i1 %"61", label %"16", label %"15"

"15":                                             ; preds = %"48"
  %"44" = load i32, ptr addrspace(4) @4, align 4
  %"63" = load i32, ptr addrspace(5) %"54", align 4
  %"62" = add i32 %"63", %"44"
  store i32 %"62", ptr addrspace(5) %"54", align 4
  br label %"16"

"16":                                             ; preds = %"15", %"48"
  %"65" = load i32, ptr addrspace(5) %"53", align 4
  %"64" = zext i32 %"65" to i64
  store i64 %"64", ptr addrspace(5) %"52", align 8
  %"46" = load i64, ptr addrspace(4) @5, align 8
  %"67" = load i64, ptr addrspace(5) %"52", align 8
  %"66" = mul i64 %"67", %"46"
  store i64 %"66", ptr addrspace(5) %"52", align 8
  %"69" = load i64, ptr addrspace(5) %"51", align 8
  %"70" = load i64, ptr addrspace(5) %"52", align 8
  %"68" = add i64 %"69", %"70"
  store i64 %"68", ptr addrspace(5) %"51", align 8
  %"71" = load i64, ptr addrspace(5) %"51", align 8
  %"72" = load i32, ptr addrspace(5) %"54", align 4
  %"76" = inttoptr i64 %"71" to ptr
  store i32 %"72", ptr %"76", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }