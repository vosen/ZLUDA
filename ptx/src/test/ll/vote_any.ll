declare hidden i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_any(ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i1, align 1, addrspace(5)
  %"49" = alloca i1, align 1, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %"38" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"38", ptr addrspace(5) %"47", align 4
  %3 = load i32, ptr addrspace(5) %"47", align 4
  %4 = icmp uge i32 %3, 32
  store i1 %4, ptr addrspace(5) %"48", align 1
  %5 = load i1, ptr addrspace(5) %"48", align 1
  %"56" = call i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1 %5, i32 -1)
  store i1 %"56", ptr addrspace(5) %"49", align 1
  %6 = load i1, ptr addrspace(5) %"49", align 1
  %"58" = select i1 %6, i32 1, i32 0
  store i32 %"58", ptr addrspace(5) %"50", align 4
  %7 = load i32, ptr addrspace(5) %"47", align 4
  %8 = zext i32 %7 to i64
  %"61" = mul i64 %8, 4
  store i64 %"61", ptr addrspace(5) %"60", align 8
  %9 = load i64, ptr addrspace(5) %"51", align 8
  %10 = load i64, ptr addrspace(5) %"60", align 8
  %"63" = add i64 %9, %10
  store i64 %"63", ptr addrspace(5) %"51", align 8
  %11 = load i64, ptr addrspace(5) %"51", align 8
  %12 = load i32, ptr addrspace(5) %"50", align 4
  %"68" = inttoptr i64 %11 to ptr
  store i32 %12, ptr %"68", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }