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

define amdgpu_kernel void @vote_all_sub(ptr addrspace(4) byref(i64) %"61") #1 {
  %"62" = alloca i32, align 4, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i1, align 1, addrspace(5)
  %"65" = alloca i1, align 1, addrspace(5)
  %"66" = alloca i32, align 4, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  %"78" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"56"

"56":                                             ; preds = %1
  %"68" = load i64, ptr addrspace(4) %"61", align 8
  store i64 %"68", ptr addrspace(5) %"67", align 8
  %"38" = call i32 @__zluda_ptx_impl_sreg_laneid()
  br label %"57"

"57":                                             ; preds = %"56"
  store i32 %"38", ptr addrspace(5) %"62", align 4
  %"40" = load i8, ptr addrspace(4) @0, align 1
  %"41" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"40")
  br label %"58"

"58":                                             ; preds = %"57"
  store i32 %"41", ptr addrspace(5) %"63", align 4
  %"43" = load i32, ptr addrspace(4) @1, align 4
  %"72" = load i32, ptr addrspace(5) %"62", align 4
  %2 = icmp eq i32 %"72", %"43"
  store i1 %2, ptr addrspace(5) %"64", align 1
  %"45" = load i1, ptr addrspace(4) @2, align 1
  store i1 %"45", ptr addrspace(5) %"65", align 1
  %"74" = load i1, ptr addrspace(5) %"64", align 1
  br i1 %"74", label %"10", label %"19"

"19":                                             ; preds = %"58"
  %"47" = load i1, ptr addrspace(4) @3, align 1
  %"49" = load i32, ptr addrspace(4) @4, align 4
  %"75" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 %"47", i32 %"49")
  store i1 %"75", ptr addrspace(5) %"65", align 1
  br label %"10"

"10":                                             ; preds = %"19", %"58"
  %"51" = load i32, ptr addrspace(4) @5, align 4
  %"53" = load i32, ptr addrspace(4) @6, align 4
  %"77" = load i1, ptr addrspace(5) %"65", align 1
  %"76" = select i1 %"77", i32 %"51", i32 %"53"
  store i32 %"76", ptr addrspace(5) %"66", align 4
  %"55" = load i32, ptr addrspace(4) @7, align 4
  %"80" = load i32, ptr addrspace(5) %"63", align 4
  %3 = zext i32 %"80" to i64
  %4 = zext i32 %"55" to i64
  %"79" = mul i64 %3, %4
  store i64 %"79", ptr addrspace(5) %"78", align 8
  %"82" = load i64, ptr addrspace(5) %"67", align 8
  %"83" = load i64, ptr addrspace(5) %"78", align 8
  %"81" = add i64 %"82", %"83"
  store i64 %"81", ptr addrspace(5) %"67", align 8
  %"84" = load i64, ptr addrspace(5) %"67", align 8
  %"85" = load i32, ptr addrspace(5) %"66", align 4
  %"86" = inttoptr i64 %"84" to ptr
  store i32 %"85", ptr %"86", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }