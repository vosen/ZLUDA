@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 34
@2 = addrspace(4) global i32 -1
@3 = addrspace(4) global i32 4

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
  %"49" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"49", ptr addrspace(5) %"48", align 8
  %"34" = call i32 @__zluda_ptx_impl_sreg_tid(i8 ptrtoint (ptr addrspace(4) @0 to i8))
  br label %"42"

"42":                                             ; preds = %"41"
  store i32 %"34", ptr addrspace(5) %"45", align 4
  %"36" = load i32, ptr addrspace(4) @1, align 4
  %"52" = load i32, ptr addrspace(5) %"45", align 4
  %2 = icmp uge i32 %"52", %"36"
  store i1 %2, ptr addrspace(5) %"46", align 1
  %"38" = load i32, ptr addrspace(4) @2, align 4
  %"54" = load i1, ptr addrspace(5) %"46", align 1
  %"64" = call i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1 %"54", i32 %"38")
  store i32 %"64", ptr addrspace(5) %"47", align 4
  %"40" = load i32, ptr addrspace(4) @3, align 4
  %"57" = load i32, ptr addrspace(5) %"45", align 4
  %3 = zext i32 %"57" to i64
  %4 = zext i32 %"40" to i64
  %"56" = mul i64 %3, %4
  store i64 %"56", ptr addrspace(5) %"55", align 8
  %"59" = load i64, ptr addrspace(5) %"48", align 8
  %"60" = load i64, ptr addrspace(5) %"55", align 8
  %"58" = add i64 %"59", %"60"
  store i64 %"58", ptr addrspace(5) %"48", align 8
  %"61" = load i64, ptr addrspace(5) %"48", align 8
  %"62" = load i32, ptr addrspace(5) %"47", align 4
  %"65" = inttoptr i64 %"61" to ptr
  store i32 %"62", ptr %"65", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }