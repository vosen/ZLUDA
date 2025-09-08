declare hidden i32 @__zluda_ptx_impl_redux_sync_min_s32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_max_s32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_add_s32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @redux_sync_op_s32(ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %"54" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"54", ptr addrspace(5) %"53", align 8
  %"37" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"44"

"44":                                             ; preds = %"43"
  store i32 %"37", ptr addrspace(5) %"47", align 4
  %"57" = load i32, ptr addrspace(5) %"47", align 4
  %"56" = sub i32 %"57", 5
  store i32 %"56", ptr addrspace(5) %"48", align 4
  %"59" = load i32, ptr addrspace(5) %"48", align 4
  %"58" = call i32 @__zluda_ptx_impl_redux_sync_add_s32(i32 %"59", i32 -1)
  store i32 %"58", ptr addrspace(5) %"49", align 4
  %"61" = load i32, ptr addrspace(5) %"48", align 4
  %"60" = call i32 @__zluda_ptx_impl_redux_sync_min_s32(i32 %"61", i32 -1)
  store i32 %"60", ptr addrspace(5) %"50", align 4
  %"63" = load i32, ptr addrspace(5) %"48", align 4
  %"62" = call i32 @__zluda_ptx_impl_redux_sync_max_s32(i32 %"63", i32 -1)
  store i32 %"62", ptr addrspace(5) %"51", align 4
  %"65" = load i32, ptr addrspace(5) %"49", align 4
  %"66" = load i32, ptr addrspace(5) %"50", align 4
  %"64" = add i32 %"65", %"66"
  store i32 %"64", ptr addrspace(5) %"52", align 4
  %"68" = load i32, ptr addrspace(5) %"52", align 4
  %"69" = load i32, ptr addrspace(5) %"51", align 4
  %"67" = add i32 %"68", %"69"
  store i32 %"67", ptr addrspace(5) %"52", align 4
  %"72" = load i32, ptr addrspace(5) %"47", align 4
  %2 = zext i32 %"72" to i64
  %"71" = mul i64 %2, 4
  store i64 %"71", ptr addrspace(5) %"70", align 8
  %"74" = load i64, ptr addrspace(5) %"53", align 8
  %"75" = load i64, ptr addrspace(5) %"70", align 8
  %"73" = add i64 %"74", %"75"
  store i64 %"73", ptr addrspace(5) %"53", align 8
  %"76" = load i64, ptr addrspace(5) %"53", align 8
  %"77" = load i32, ptr addrspace(5) %"52", align 4
  %"79" = inttoptr i64 %"76" to ptr
  store i32 %"77", ptr %"79", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
