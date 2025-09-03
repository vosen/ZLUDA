declare hidden i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_ballot(ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i1, align 1, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"47", align 1
  %"50" = alloca i32, align 4, addrspace(5)
  store i32 34, ptr addrspace(5) %"50", align 4
  %"54" = alloca i32, align 4, addrspace(5)
  store i32 -1, ptr addrspace(5) %"54", align 4
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i32, align 4, addrspace(5)
  store i32 4, ptr addrspace(5) %"59", align 4
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"46", ptr addrspace(5) %"45", align 8
  %"48" = load i8, ptr addrspace(5) %"47", align 1
  %"34" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"48")
  br label %"39"

"39":                                             ; preds = %"38"
  store i32 %"34", ptr addrspace(5) %"42", align 4
  %"52" = load i32, ptr addrspace(5) %"42", align 4
  %"53" = load i32, ptr addrspace(5) %"50", align 4
  %2 = icmp uge i32 %"52", %"53"
  store i1 %2, ptr addrspace(5) %"43", align 1
  %"56" = load i1, ptr addrspace(5) %"43", align 1
  %"57" = load i32, ptr addrspace(5) %"54", align 4
  %"68" = call i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1 %"56", i32 %"57")
  store i32 %"68", ptr addrspace(5) %"44", align 4
  %"61" = load i32, ptr addrspace(5) %"42", align 4
  %"62" = load i32, ptr addrspace(5) %"59", align 4
  %3 = zext i32 %"61" to i64
  %4 = zext i32 %"62" to i64
  %"60" = mul i64 %3, %4
  store i64 %"60", ptr addrspace(5) %"58", align 8
  %"64" = load i64, ptr addrspace(5) %"45", align 8
  %"65" = load i64, ptr addrspace(5) %"58", align 8
  %"63" = add i64 %"64", %"65"
  store i64 %"63", ptr addrspace(5) %"45", align 8
  %"66" = load i64, ptr addrspace(5) %"45", align 8
  %"67" = load i32, ptr addrspace(5) %"44", align 4
  %"69" = inttoptr i64 %"66" to ptr
  store i32 %"67", ptr %"69", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }