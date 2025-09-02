@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 32
@2 = addrspace(4) global i32 -1
@3 = addrspace(4) global i32 1
@4 = addrspace(4) global i32 0
@5 = addrspace(4) global i32 4

declare hidden i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_any(ptr addrspace(4) byref(i64) %"49") #1 {
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i1, align 1, addrspace(5)
  %"52" = alloca i1, align 1, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %"55" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"55", ptr addrspace(5) %"54", align 8
  %"35" = call i32 @__zluda_ptx_impl_sreg_tid(i8 ptrtoint (ptr addrspace(4) @0 to i8))
  br label %"47"

"47":                                             ; preds = %"46"
  store i32 %"35", ptr addrspace(5) %"50", align 4
  %"37" = load i32, ptr addrspace(4) @1, align 4
  %"58" = load i32, ptr addrspace(5) %"50", align 4
  %2 = icmp uge i32 %"58", %"37"
  store i1 %2, ptr addrspace(5) %"51", align 1
  %"39" = load i32, ptr addrspace(4) @2, align 4
  %"60" = load i1, ptr addrspace(5) %"51", align 1
  %"59" = call i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1 %"60", i32 %"39")
  store i1 %"59", ptr addrspace(5) %"52", align 1
  %"41" = load i32, ptr addrspace(4) @3, align 4
  %"43" = load i32, ptr addrspace(4) @4, align 4
  %"62" = load i1, ptr addrspace(5) %"52", align 1
  %"61" = select i1 %"62", i32 %"41", i32 %"43"
  store i32 %"61", ptr addrspace(5) %"53", align 4
  %"45" = load i32, ptr addrspace(4) @5, align 4
  %"65" = load i32, ptr addrspace(5) %"50", align 4
  %3 = zext i32 %"65" to i64
  %4 = zext i32 %"45" to i64
  %"64" = mul i64 %3, %4
  store i64 %"64", ptr addrspace(5) %"63", align 8
  %"67" = load i64, ptr addrspace(5) %"54", align 8
  %"68" = load i64, ptr addrspace(5) %"63", align 8
  %"66" = add i64 %"67", %"68"
  store i64 %"66", ptr addrspace(5) %"54", align 8
  %"69" = load i64, ptr addrspace(5) %"54", align 8
  %"70" = load i32, ptr addrspace(5) %"53", align 4
  %"72" = inttoptr i64 %"69" to ptr
  store i32 %"70", ptr %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }