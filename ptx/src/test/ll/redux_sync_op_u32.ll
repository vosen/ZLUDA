declare hidden i32 @__zluda_ptx_impl_redux_sync_add_u32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_max_u32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_redux_sync_min_u32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @redux_sync_op_u32(ptr addrspace(4) byref(i64) %"49") #1 {
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"55", align 8
  %"42" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"42", ptr addrspace(5) %"50", align 4
  %3 = load i32, ptr addrspace(5) %"50", align 4
  %"58" = call i32 @__zluda_ptx_impl_redux_sync_add_u32(i32 %3, i32 -1)
  store i32 %"58", ptr addrspace(5) %"51", align 4
  %4 = load i32, ptr addrspace(5) %"50", align 4
  %"60" = call i32 @__zluda_ptx_impl_redux_sync_min_u32(i32 %4, i32 -1)
  store i32 %"60", ptr addrspace(5) %"52", align 4
  %5 = load i32, ptr addrspace(5) %"50", align 4
  %"62" = call i32 @__zluda_ptx_impl_redux_sync_max_u32(i32 %5, i32 -1)
  store i32 %"62", ptr addrspace(5) %"53", align 4
  %6 = load i32, ptr addrspace(5) %"51", align 4
  %7 = load i32, ptr addrspace(5) %"52", align 4
  %"64" = add i32 %6, %7
  store i32 %"64", ptr addrspace(5) %"54", align 4
  %8 = load i32, ptr addrspace(5) %"54", align 4
  %9 = load i32, ptr addrspace(5) %"53", align 4
  %"67" = add i32 %8, %9
  store i32 %"67", ptr addrspace(5) %"54", align 4
  %10 = load i32, ptr addrspace(5) %"50", align 4
  %11 = zext i32 %10 to i64
  %"71" = mul i64 %11, 4
  store i64 %"71", ptr addrspace(5) %"70", align 8
  %12 = load i64, ptr addrspace(5) %"55", align 8
  %13 = load i64, ptr addrspace(5) %"70", align 8
  %"73" = add i64 %12, %13
  store i64 %"73", ptr addrspace(5) %"55", align 8
  %14 = load i64, ptr addrspace(5) %"55", align 8
  %15 = load i32, ptr addrspace(5) %"54", align 4
  %"78" = inttoptr i64 %14 to ptr
  store i32 %15, ptr %"78", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
