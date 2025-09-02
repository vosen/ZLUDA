@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 0
@2 = addrspace(4) global i1 false
@3 = addrspace(4) global i1 true
@4 = addrspace(4) global i32 -1
@5 = addrspace(4) global i32 1
@6 = addrspace(4) global i32 0
@7 = addrspace(4) global i32 4

declare hidden i1 @__zluda_ptx_impl_vote_sync_all_pred(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare hidden i32 @__zluda_ptx_impl_sreg_laneid() #0

define amdgpu_kernel void @vote_all_sub(ptr addrspace(4) byref(i64) %"60") #1 {
  %"61" = alloca i32, align 4, addrspace(5)
  %"62" = alloca i32, align 4, addrspace(5)
  %"63" = alloca i1, align 1, addrspace(5)
  %"64" = alloca i1, align 1, addrspace(5)
  %"65" = alloca i32, align 4, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"77" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"55"

"55":                                             ; preds = %1
  %"67" = load i64, ptr addrspace(4) %"60", align 8
  store i64 %"67", ptr addrspace(5) %"66", align 8
  %"38" = call i32 @__zluda_ptx_impl_sreg_laneid()
  br label %"56"

"56":                                             ; preds = %"55"
  store i32 %"38", ptr addrspace(5) %"61", align 4
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 ptrtoint (ptr addrspace(4) @0 to i8))
  br label %"57"

"57":                                             ; preds = %"56"
  store i32 %"40", ptr addrspace(5) %"62", align 4
  %"42" = load i32, ptr addrspace(4) @1, align 4
  %"71" = load i32, ptr addrspace(5) %"61", align 4
  %2 = icmp eq i32 %"71", %"42"
  store i1 %2, ptr addrspace(5) %"63", align 1
  %"44" = load i1, ptr addrspace(4) @2, align 1
  store i1 %"44", ptr addrspace(5) %"64", align 1
  %"73" = load i1, ptr addrspace(5) %"63", align 1
  br i1 %"73", label %"10", label %"19"

"19":                                             ; preds = %"57"
  %"46" = load i1, ptr addrspace(4) @3, align 1
  %"48" = load i32, ptr addrspace(4) @4, align 4
  %"74" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 %"46", i32 %"48")
  store i1 %"74", ptr addrspace(5) %"64", align 1
  br label %"10"

"10":                                             ; preds = %"19", %"57"
  %"50" = load i32, ptr addrspace(4) @5, align 4
  %"52" = load i32, ptr addrspace(4) @6, align 4
  %"76" = load i1, ptr addrspace(5) %"64", align 1
  %"75" = select i1 %"76", i32 %"50", i32 %"52"
  store i32 %"75", ptr addrspace(5) %"65", align 4
  %"54" = load i32, ptr addrspace(4) @7, align 4
  %"79" = load i32, ptr addrspace(5) %"62", align 4
  %3 = zext i32 %"79" to i64
  %4 = zext i32 %"54" to i64
  %"78" = mul i64 %3, %4
  store i64 %"78", ptr addrspace(5) %"77", align 8
  %"81" = load i64, ptr addrspace(5) %"66", align 8
  %"82" = load i64, ptr addrspace(5) %"77", align 8
  %"80" = add i64 %"81", %"82"
  store i64 %"80", ptr addrspace(5) %"66", align 8
  %"83" = load i64, ptr addrspace(5) %"66", align 8
  %"84" = load i32, ptr addrspace(5) %"65", align 4
  %"86" = inttoptr i64 %"83" to ptr
  store i32 %"84", ptr %"86", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }