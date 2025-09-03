@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 32
@2 = addrspace(4) global i32 -1
@3 = addrspace(4) global i32 1
@4 = addrspace(4) global i32 0
@5 = addrspace(4) global i32 4

declare hidden i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_any(ptr addrspace(4) byref(i64) %"50") #1 {
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i1, align 1, addrspace(5)
  %"53" = alloca i1, align 1, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %"56" = load i64, ptr addrspace(4) %"50", align 8
  store i64 %"56", ptr addrspace(5) %"55", align 8
  %"35" = load i8, ptr addrspace(4) @0, align 1
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"35")
  br label %"48"

"48":                                             ; preds = %"47"
  store i32 %"36", ptr addrspace(5) %"51", align 4
  %"38" = load i32, ptr addrspace(4) @1, align 4
  %"59" = load i32, ptr addrspace(5) %"51", align 4
  %2 = icmp uge i32 %"59", %"38"
  store i1 %2, ptr addrspace(5) %"52", align 1
  %"40" = load i32, ptr addrspace(4) @2, align 4
  %"61" = load i1, ptr addrspace(5) %"52", align 1
  %"60" = call i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1 %"61", i32 %"40")
  store i1 %"60", ptr addrspace(5) %"53", align 1
  %"42" = load i32, ptr addrspace(4) @3, align 4
  %"44" = load i32, ptr addrspace(4) @4, align 4
  %"63" = load i1, ptr addrspace(5) %"53", align 1
  %"62" = select i1 %"63", i32 %"42", i32 %"44"
  store i32 %"62", ptr addrspace(5) %"54", align 4
  %"46" = load i32, ptr addrspace(4) @5, align 4
  %"66" = load i32, ptr addrspace(5) %"51", align 4
  %3 = zext i32 %"66" to i64
  %4 = zext i32 %"46" to i64
  %"65" = mul i64 %3, %4
  store i64 %"65", ptr addrspace(5) %"64", align 8
  %"68" = load i64, ptr addrspace(5) %"55", align 8
  %"69" = load i64, ptr addrspace(5) %"64", align 8
  %"67" = add i64 %"68", %"69"
  store i64 %"67", ptr addrspace(5) %"55", align 8
  %"70" = load i64, ptr addrspace(5) %"55", align 8
  %"71" = load i32, ptr addrspace(5) %"54", align 4
  %"72" = inttoptr i64 %"70" to ptr
  store i32 %"71", ptr %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }