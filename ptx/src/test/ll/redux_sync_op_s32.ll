declare hidden i32 @__zluda_ptx_impl_redux_sync_min_s32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_max_s32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_add_s32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @redux_sync_op_s32(ptr addrspace(4) byref(i64) %"49") #1 {
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"73" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"56", align 8
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"47"

"47":                                             ; preds = %"46"
  store i32 %"40", ptr addrspace(5) %"50", align 4
  %3 = load i32, ptr addrspace(5) %"50", align 4
  %"59" = sub i32 %3, 5
  store i32 %"59", ptr addrspace(5) %"51", align 4
  %4 = load i32, ptr addrspace(5) %"51", align 4
  %"61" = call i32 @__zluda_ptx_impl_redux_sync_add_s32(i32 %4, i32 -1)
  store i32 %"61", ptr addrspace(5) %"52", align 4
  %5 = load i32, ptr addrspace(5) %"51", align 4
  %"63" = call i32 @__zluda_ptx_impl_redux_sync_min_s32(i32 %5, i32 -1)
  store i32 %"63", ptr addrspace(5) %"53", align 4
  %6 = load i32, ptr addrspace(5) %"51", align 4
  %"65" = call i32 @__zluda_ptx_impl_redux_sync_max_s32(i32 %6, i32 -1)
  store i32 %"65", ptr addrspace(5) %"54", align 4
  %7 = load i32, ptr addrspace(5) %"52", align 4
  %8 = load i32, ptr addrspace(5) %"53", align 4
  %"67" = add i32 %7, %8
  store i32 %"67", ptr addrspace(5) %"55", align 4
  %9 = load i32, ptr addrspace(5) %"55", align 4
  %10 = load i32, ptr addrspace(5) %"54", align 4
  %"70" = add i32 %9, %10
  store i32 %"70", ptr addrspace(5) %"55", align 4
  %11 = load i32, ptr addrspace(5) %"50", align 4
  %12 = zext i32 %11 to i64
  %"74" = mul i64 %12, 4
  store i64 %"74", ptr addrspace(5) %"73", align 8
  %13 = load i64, ptr addrspace(5) %"56", align 8
  %14 = load i64, ptr addrspace(5) %"73", align 8
  %"76" = add i64 %13, %14
  store i64 %"76", ptr addrspace(5) %"56", align 8
  %15 = load i64, ptr addrspace(5) %"56", align 8
  %16 = load i32, ptr addrspace(5) %"55", align 4
  %"82" = inttoptr i64 %15 to ptr
  store i32 %16, ptr %"82", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }