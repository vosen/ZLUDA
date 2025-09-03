@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 34
@2 = addrspace(4) global i32 -1
@3 = addrspace(4) global i32 4

declare hidden i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_ballot(ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i1, align 1, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"50", ptr addrspace(5) %"49", align 8
  %"34" = load i8, ptr addrspace(4) @0, align 1
  %"35" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"34")
  br label %"43"

"43":                                             ; preds = %"42"
  store i32 %"35", ptr addrspace(5) %"46", align 4
  %"37" = load i32, ptr addrspace(4) @1, align 4
  %"53" = load i32, ptr addrspace(5) %"46", align 4
  %2 = icmp uge i32 %"53", %"37"
  store i1 %2, ptr addrspace(5) %"47", align 1
  %"39" = load i32, ptr addrspace(4) @2, align 4
  %"55" = load i1, ptr addrspace(5) %"47", align 1
  %"64" = call i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1 %"55", i32 %"39")
  store i32 %"64", ptr addrspace(5) %"48", align 4
  %"41" = load i32, ptr addrspace(4) @3, align 4
  %"58" = load i32, ptr addrspace(5) %"46", align 4
  %3 = zext i32 %"58" to i64
  %4 = zext i32 %"41" to i64
  %"57" = mul i64 %3, %4
  store i64 %"57", ptr addrspace(5) %"56", align 8
  %"60" = load i64, ptr addrspace(5) %"49", align 8
  %"61" = load i64, ptr addrspace(5) %"56", align 8
  %"59" = add i64 %"60", %"61"
  store i64 %"59", ptr addrspace(5) %"49", align 8
  %"62" = load i64, ptr addrspace(5) %"49", align 8
  %"63" = load i32, ptr addrspace(5) %"48", align 4
  %"65" = inttoptr i64 %"62" to ptr
  store i32 %"63", ptr %"65", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }