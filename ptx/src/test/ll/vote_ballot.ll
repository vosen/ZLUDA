declare hidden i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_ballot(ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i1, align 1, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"50", align 8
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"40", ptr addrspace(5) %"47", align 4
  %3 = load i32, ptr addrspace(5) %"47", align 4
  %4 = icmp uge i32 %3, 34
  store i1 %4, ptr addrspace(5) %"48", align 1
  %5 = load i1, ptr addrspace(5) %"48", align 1
  %"65" = call i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1 %5, i32 -1)
  store i32 %"65", ptr addrspace(5) %"49", align 4
  %6 = load i32, ptr addrspace(5) %"47", align 4
  %7 = zext i32 %6 to i64
  %"58" = mul i64 %7, 4
  store i64 %"58", ptr addrspace(5) %"57", align 8
  %8 = load i64, ptr addrspace(5) %"50", align 8
  %9 = load i64, ptr addrspace(5) %"57", align 8
  %"60" = add i64 %8, %9
  store i64 %"60", ptr addrspace(5) %"50", align 8
  %10 = load i64, ptr addrspace(5) %"50", align 8
  %11 = load i32, ptr addrspace(5) %"49", align 4
  %"66" = inttoptr i64 %10 to ptr
  store i32 %11, ptr %"66", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
