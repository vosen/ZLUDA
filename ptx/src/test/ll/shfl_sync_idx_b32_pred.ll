@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 12
@2 = addrspace(4) global i32 31
@3 = addrspace(4) global i32 -1
@4 = addrspace(4) global i32 1000
@5 = addrspace(4) global i64 4

declare hidden <2 x i32> @__zluda_ptx_impl_shfl_sync_idx_b32_pred(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_idx_b32_pred(ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"57" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"57", ptr addrspace(5) %"52", align 8
  %"36" = load i8, ptr addrspace(4) @0, align 1
  %"37" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"36")
  br label %"49"

"49":                                             ; preds = %"48"
  store i32 %"37", ptr addrspace(5) %"54", align 4
  %"39" = load i32, ptr addrspace(4) @1, align 4
  %"41" = load i32, ptr addrspace(4) @2, align 4
  %"43" = load i32, ptr addrspace(4) @3, align 4
  %"61" = load i32, ptr addrspace(5) %"54", align 4
  %"77" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_idx_b32_pred(i32 %"61", i32 %"39", i32 %"41", i32 %"43")
  %"74" = extractelement <2 x i32> %"77", i8 0
  %"78" = extractelement <2 x i32> %"77", i8 1
  %"60" = trunc i32 %"78" to i1
  store i32 %"74", ptr addrspace(5) %"55", align 4
  store i1 %"60", ptr addrspace(5) %"56", align 1
  %"62" = load i1, ptr addrspace(5) %"56", align 1
  br i1 %"62", label %"16", label %"15"

"15":                                             ; preds = %"49"
  %"45" = load i32, ptr addrspace(4) @4, align 4
  %"64" = load i32, ptr addrspace(5) %"55", align 4
  %"63" = add i32 %"64", %"45"
  store i32 %"63", ptr addrspace(5) %"55", align 4
  br label %"16"

"16":                                             ; preds = %"15", %"49"
  %"66" = load i32, ptr addrspace(5) %"54", align 4
  %"65" = zext i32 %"66" to i64
  store i64 %"65", ptr addrspace(5) %"53", align 8
  %"47" = load i64, ptr addrspace(4) @5, align 8
  %"68" = load i64, ptr addrspace(5) %"53", align 8
  %"67" = mul i64 %"68", %"47"
  store i64 %"67", ptr addrspace(5) %"53", align 8
  %"70" = load i64, ptr addrspace(5) %"52", align 8
  %"71" = load i64, ptr addrspace(5) %"53", align 8
  %"69" = add i64 %"70", %"71"
  store i64 %"69", ptr addrspace(5) %"52", align 8
  %"72" = load i64, ptr addrspace(5) %"52", align 8
  %"73" = load i32, ptr addrspace(5) %"55", align 4
  %"76" = inttoptr i64 %"72" to ptr
  store i32 %"73", ptr %"76", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }