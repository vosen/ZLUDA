declare hidden i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_ballot(ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i1, align 1, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"46", ptr addrspace(5) %"45", align 8
  %"34" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"39"

"39":                                             ; preds = %"38"
  store i32 %"34", ptr addrspace(5) %"42", align 4
  %"49" = load i32, ptr addrspace(5) %"42", align 4
  %2 = icmp uge i32 %"49", 34
  store i1 %2, ptr addrspace(5) %"43", align 1
  %"51" = load i1, ptr addrspace(5) %"43", align 1
  %"60" = call i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1 %"51", i32 -1)
  store i32 %"60", ptr addrspace(5) %"44", align 4
  %"54" = load i32, ptr addrspace(5) %"42", align 4
  %3 = zext i32 %"54" to i64
  %"53" = mul i64 %3, 4
  store i64 %"53", ptr addrspace(5) %"52", align 8
  %"56" = load i64, ptr addrspace(5) %"45", align 8
  %"57" = load i64, ptr addrspace(5) %"52", align 8
  %"55" = add i64 %"56", %"57"
  store i64 %"55", ptr addrspace(5) %"45", align 8
  %"58" = load i64, ptr addrspace(5) %"45", align 8
  %"59" = load i32, ptr addrspace(5) %"44", align 4
  %"61" = inttoptr i64 %"58" to ptr
  store i32 %"59", ptr %"61", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }