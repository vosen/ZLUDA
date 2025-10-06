declare hidden i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_ballot(ptr addrspace(4) byref(i64) %"44") #1 {
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i1, align 1, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %2, ptr addrspace(5) %"48", align 8
  %"37" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"42"

"42":                                             ; preds = %"41"
  store i32 %"37", ptr addrspace(5) %"45", align 4
  %3 = load i32, ptr addrspace(5) %"45", align 4
  %4 = icmp uge i32 %3, 34
  store i1 %4, ptr addrspace(5) %"46", align 1
  %5 = load i1, ptr addrspace(5) %"46", align 1
  %"63" = call i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1 %5, i32 -1)
  store i32 %"63", ptr addrspace(5) %"47", align 4
  %6 = load i32, ptr addrspace(5) %"45", align 4
  %7 = zext i32 %6 to i64
  %"56" = mul i64 %7, 4
  store i64 %"56", ptr addrspace(5) %"55", align 8
  %8 = load i64, ptr addrspace(5) %"48", align 8
  %9 = load i64, ptr addrspace(5) %"55", align 8
  %"58" = add i64 %8, %9
  store i64 %"58", ptr addrspace(5) %"48", align 8
  %10 = load i64, ptr addrspace(5) %"48", align 8
  %11 = load i32, ptr addrspace(5) %"47", align 4
  %"64" = inttoptr i64 %10 to ptr
  store i32 %11, ptr %"64", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }