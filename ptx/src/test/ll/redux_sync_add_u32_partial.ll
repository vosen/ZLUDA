declare hidden i32 @__zluda_ptx_impl_redux_sync_add_u32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @redux_sync_add_u32_partial(ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i1, align 1, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"48", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"40", ptr addrspace(5) %"49", align 4
  %3 = load i32, ptr addrspace(5) %"49", align 4
  %"56" = urem i32 %3, 2
  store i32 %"56", ptr addrspace(5) %"52", align 4
  %4 = load i32, ptr addrspace(5) %"52", align 4
  %5 = icmp eq i32 %4, 0
  store i1 %5, ptr addrspace(5) %"53", align 1
  store i32 0, ptr addrspace(5) %"50", align 4
  %6 = load i1, ptr addrspace(5) %"53", align 1
  br i1 %6, label %"17", label %"18"

"17":                                             ; preds = %"46"
  %7 = load i32, ptr addrspace(5) %"49", align 4
  %"62" = call i32 @__zluda_ptx_impl_redux_sync_add_u32(i32 %7, i32 1431655765)
  store i32 %"62", ptr addrspace(5) %"50", align 4
  br label %"18"

"18":                                             ; preds = %"17", %"46"
  %8 = load i32, ptr addrspace(5) %"49", align 4
  %9 = zext i32 %8 to i64
  %"65" = mul i64 %9, 4
  store i64 %"65", ptr addrspace(5) %"64", align 8
  %10 = load i64, ptr addrspace(5) %"51", align 8
  %11 = load i64, ptr addrspace(5) %"64", align 8
  %"67" = add i64 %10, %11
  store i64 %"67", ptr addrspace(5) %"51", align 8
  %12 = load i64, ptr addrspace(5) %"51", align 8
  %13 = load i32, ptr addrspace(5) %"50", align 4
  %"72" = inttoptr i64 %12 to ptr
  store i32 %13, ptr %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }