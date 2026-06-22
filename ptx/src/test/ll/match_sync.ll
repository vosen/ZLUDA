declare hidden i32 @__zluda_ptx_impl_match_any_sync_b32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @match_sync(ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"47", align 8
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"40", ptr addrspace(5) %"49", align 4
  %3 = load i32, ptr addrspace(5) %"49", align 4
  %"67" = and i32 %3, 1
  store i32 %"67", ptr addrspace(5) %"50", align 4
  %4 = load i32, ptr addrspace(5) %"50", align 4
  %"69" = call i32 @__zluda_ptx_impl_match_any_sync_b32(i32 %4, i32 -1)
  store i32 %"69", ptr addrspace(5) %"51", align 4
  %5 = load i32, ptr addrspace(5) %"49", align 4
  %6 = zext i32 %5 to i64
  store i64 %6, ptr addrspace(5) %"48", align 8
  %7 = load i64, ptr addrspace(5) %"48", align 8
  %"60" = mul i64 %7, 4
  store i64 %"60", ptr addrspace(5) %"48", align 8
  %8 = load i64, ptr addrspace(5) %"47", align 8
  %9 = load i64, ptr addrspace(5) %"48", align 8
  %"62" = add i64 %8, %9
  store i64 %"62", ptr addrspace(5) %"47", align 8
  %10 = load i64, ptr addrspace(5) %"47", align 8
  %11 = load i32, ptr addrspace(5) %"51", align 4
  %"71" = inttoptr i64 %10 to ptr
  store i32 %11, ptr %"71", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
