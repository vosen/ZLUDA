@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 0
@2 = addrspace(4) global i1 false
@3 = addrspace(4) global i32 -2
@4 = addrspace(4) global i32 1
@5 = addrspace(4) global i32 0
@6 = addrspace(4) global i32 4

declare hidden i1 @__zluda_ptx_impl_vote_sync_all_pred(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare hidden i32 @__zluda_ptx_impl_sreg_laneid() #0

define amdgpu_kernel void @vote_all(ptr addrspace(4) byref(i64) %"58") #1 {
  %"59" = alloca i32, align 4, addrspace(5)
  %"60" = alloca i32, align 4, addrspace(5)
  %"61" = alloca i1, align 1, addrspace(5)
  %"62" = alloca i1, align 1, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  %"76" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"53"

"53":                                             ; preds = %1
  %"65" = load i64, ptr addrspace(4) %"58", align 8
  store i64 %"65", ptr addrspace(5) %"64", align 8
  %"37" = call i32 @__zluda_ptx_impl_sreg_laneid()
  br label %"54"

"54":                                             ; preds = %"53"
  store i32 %"37", ptr addrspace(5) %"59", align 4
  %"39" = load i8, ptr addrspace(4) @0, align 1
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"39")
  br label %"55"

"55":                                             ; preds = %"54"
  store i32 %"40", ptr addrspace(5) %"60", align 4
  %"42" = load i32, ptr addrspace(4) @1, align 4
  %"69" = load i32, ptr addrspace(5) %"59", align 4
  %2 = icmp ne i32 %"69", %"42"
  store i1 %2, ptr addrspace(5) %"61", align 1
  %"44" = load i1, ptr addrspace(4) @2, align 1
  store i1 %"44", ptr addrspace(5) %"62", align 1
  %"71" = load i1, ptr addrspace(5) %"61", align 1
  br i1 %"71", label %"17", label %"18"

"17":                                             ; preds = %"55"
  %"46" = load i32, ptr addrspace(4) @3, align 4
  %"73" = load i1, ptr addrspace(5) %"61", align 1
  %"72" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 %"73", i32 %"46")
  store i1 %"72", ptr addrspace(5) %"62", align 1
  br label %"18"

"18":                                             ; preds = %"17", %"55"
  %"48" = load i32, ptr addrspace(4) @4, align 4
  %"50" = load i32, ptr addrspace(4) @5, align 4
  %"75" = load i1, ptr addrspace(5) %"62", align 1
  %"74" = select i1 %"75", i32 %"48", i32 %"50"
  store i32 %"74", ptr addrspace(5) %"63", align 4
  %"52" = load i32, ptr addrspace(4) @6, align 4
  %"78" = load i32, ptr addrspace(5) %"60", align 4
  %3 = zext i32 %"78" to i64
  %4 = zext i32 %"52" to i64
  %"77" = mul i64 %3, %4
  store i64 %"77", ptr addrspace(5) %"76", align 8
  %"80" = load i64, ptr addrspace(5) %"64", align 8
  %"81" = load i64, ptr addrspace(5) %"76", align 8
  %"79" = add i64 %"80", %"81"
  store i64 %"79", ptr addrspace(5) %"64", align 8
  %"82" = load i64, ptr addrspace(5) %"64", align 8
  %"83" = load i32, ptr addrspace(5) %"63", align 4
  %"84" = inttoptr i64 %"82" to ptr
  store i32 %"83", ptr %"84", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }