declare hidden i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_any(ptr addrspace(4) byref(i64) %"47") #1 {
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i1, align 1, addrspace(5)
  %"50" = alloca i1, align 1, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %2, ptr addrspace(5) %"52", align 8
  %"38" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"45"

"45":                                             ; preds = %"44"
  store i32 %"38", ptr addrspace(5) %"48", align 4
  %3 = load i32, ptr addrspace(5) %"48", align 4
  %4 = icmp uge i32 %3, 32
  store i1 %4, ptr addrspace(5) %"49", align 1
  %5 = load i1, ptr addrspace(5) %"49", align 1
  %"57" = call i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1 %5, i32 -1)
  store i1 %"57", ptr addrspace(5) %"50", align 1
  %6 = load i1, ptr addrspace(5) %"50", align 1
  %"59" = select i1 %6, i32 1, i32 0
  store i32 %"59", ptr addrspace(5) %"51", align 4
  %7 = load i32, ptr addrspace(5) %"48", align 4
  %8 = zext i32 %7 to i64
  %"62" = mul i64 %8, 4
  store i64 %"62", ptr addrspace(5) %"61", align 8
  %9 = load i64, ptr addrspace(5) %"52", align 8
  %10 = load i64, ptr addrspace(5) %"61", align 8
  %"64" = add i64 %9, %10
  store i64 %"64", ptr addrspace(5) %"52", align 8
  %11 = load i64, ptr addrspace(5) %"52", align 8
  %12 = load i32, ptr addrspace(5) %"51", align 4
  %"69" = inttoptr i64 %11 to ptr
  store i32 %12, ptr %"69", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }