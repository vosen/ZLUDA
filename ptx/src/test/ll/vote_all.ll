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

define amdgpu_kernel void @vote_all(ptr addrspace(4) byref(i64) %"57") #1 {
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i32, align 4, addrspace(5)
  %"60" = alloca i1, align 1, addrspace(5)
  %"61" = alloca i1, align 1, addrspace(5)
  %"62" = alloca i32, align 4, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"75" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"52"

"52":                                             ; preds = %1
  %"64" = load i64, ptr addrspace(4) %"57", align 8
  store i64 %"64", ptr addrspace(5) %"63", align 8
  %"37" = call i32 @__zluda_ptx_impl_sreg_laneid()
  br label %"53"

"53":                                             ; preds = %"52"
  store i32 %"37", ptr addrspace(5) %"58", align 4
  %"39" = call i32 @__zluda_ptx_impl_sreg_tid(i8 ptrtoint (ptr addrspace(4) @0 to i8))
  br label %"54"

"54":                                             ; preds = %"53"
  store i32 %"39", ptr addrspace(5) %"59", align 4
  %"41" = load i32, ptr addrspace(4) @1, align 4
  %"68" = load i32, ptr addrspace(5) %"58", align 4
  %2 = icmp ne i32 %"68", %"41"
  store i1 %2, ptr addrspace(5) %"60", align 1
  %"43" = load i1, ptr addrspace(4) @2, align 1
  store i1 %"43", ptr addrspace(5) %"61", align 1
  %"70" = load i1, ptr addrspace(5) %"60", align 1
  br i1 %"70", label %"17", label %"18"

"17":                                             ; preds = %"54"
  %"45" = load i32, ptr addrspace(4) @3, align 4
  %"72" = load i1, ptr addrspace(5) %"60", align 1
  %"71" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 %"72", i32 %"45")
  store i1 %"71", ptr addrspace(5) %"61", align 1
  br label %"18"

"18":                                             ; preds = %"17", %"54"
  %"47" = load i32, ptr addrspace(4) @4, align 4
  %"49" = load i32, ptr addrspace(4) @5, align 4
  %"74" = load i1, ptr addrspace(5) %"61", align 1
  %"73" = select i1 %"74", i32 %"47", i32 %"49"
  store i32 %"73", ptr addrspace(5) %"62", align 4
  %"51" = load i32, ptr addrspace(4) @6, align 4
  %"77" = load i32, ptr addrspace(5) %"59", align 4
  %3 = zext i32 %"77" to i64
  %4 = zext i32 %"51" to i64
  %"76" = mul i64 %3, %4
  store i64 %"76", ptr addrspace(5) %"75", align 8
  %"79" = load i64, ptr addrspace(5) %"63", align 8
  %"80" = load i64, ptr addrspace(5) %"75", align 8
  %"78" = add i64 %"79", %"80"
  store i64 %"78", ptr addrspace(5) %"63", align 8
  %"81" = load i64, ptr addrspace(5) %"63", align 8
  %"82" = load i32, ptr addrspace(5) %"62", align 4
  %"84" = inttoptr i64 %"81" to ptr
  store i32 %"82", ptr %"84", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }