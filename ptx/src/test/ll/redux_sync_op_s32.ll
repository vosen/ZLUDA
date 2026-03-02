declare hidden i32 @__zluda_ptx_impl_redux_sync_add_s32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_max_s32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_min_s32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @redux_sync_op_s32(ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"75" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"49"

"49":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"51", align 8
  store i64 %2, ptr addrspace(5) %"58", align 8
  %"43" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"43", ptr addrspace(5) %"52", align 4
  %3 = load i32, ptr addrspace(5) %"52", align 4
  %"61" = sub i32 %3, 5
  store i32 %"61", ptr addrspace(5) %"53", align 4
  %4 = load i32, ptr addrspace(5) %"53", align 4
  %"63" = call i32 @__zluda_ptx_impl_redux_sync_add_s32(i32 %4, i32 -1)
  store i32 %"63", ptr addrspace(5) %"54", align 4
  %5 = load i32, ptr addrspace(5) %"53", align 4
  %"65" = call i32 @__zluda_ptx_impl_redux_sync_min_s32(i32 %5, i32 -1)
  store i32 %"65", ptr addrspace(5) %"55", align 4
  %6 = load i32, ptr addrspace(5) %"53", align 4
  %"67" = call i32 @__zluda_ptx_impl_redux_sync_max_s32(i32 %6, i32 -1)
  store i32 %"67", ptr addrspace(5) %"56", align 4
  %7 = load i32, ptr addrspace(5) %"54", align 4
  %8 = load i32, ptr addrspace(5) %"55", align 4
  %"69" = add i32 %7, %8
  store i32 %"69", ptr addrspace(5) %"57", align 4
  %9 = load i32, ptr addrspace(5) %"57", align 4
  %10 = load i32, ptr addrspace(5) %"56", align 4
  %"72" = add i32 %9, %10
  store i32 %"72", ptr addrspace(5) %"57", align 4
  %11 = load i32, ptr addrspace(5) %"52", align 4
  %12 = zext i32 %11 to i64
  %"76" = mul i64 %12, 4
  store i64 %"76", ptr addrspace(5) %"75", align 8
  %13 = load i64, ptr addrspace(5) %"58", align 8
  %14 = load i64, ptr addrspace(5) %"75", align 8
  %"78" = add i64 %13, %14
  store i64 %"78", ptr addrspace(5) %"58", align 8
  %15 = load i64, ptr addrspace(5) %"58", align 8
  %16 = load i32, ptr addrspace(5) %"57", align 4
  %"84" = inttoptr i64 %15 to ptr
  store i32 %16, ptr %"84", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
