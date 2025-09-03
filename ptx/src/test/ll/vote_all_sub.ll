declare hidden i1 @__zluda_ptx_impl_vote_sync_all_pred(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare hidden i32 @__zluda_ptx_impl_sreg_laneid() #0

define amdgpu_kernel void @vote_all_sub(ptr addrspace(4) byref(i64) %"53") #1 {
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i1, align 1, addrspace(5)
  %"57" = alloca i1, align 1, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"62", align 1
  %"65" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"65", align 4
  %"69" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"69", align 1
  %"73" = alloca i1, align 1, addrspace(5)
  store i1 true, ptr addrspace(5) %"73", align 1
  %"74" = alloca i32, align 4, addrspace(5)
  store i32 -1, ptr addrspace(5) %"74", align 4
  %"78" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"78", align 4
  %"79" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"79", align 4
  %"84" = alloca i64, align 8, addrspace(5)
  %"85" = alloca i32, align 4, addrspace(5)
  store i32 4, ptr addrspace(5) %"85", align 4
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"60" = load i64, ptr addrspace(4) %"53", align 8
  store i64 %"60", ptr addrspace(5) %"59", align 8
  %"38" = call i32 @__zluda_ptx_impl_sreg_laneid()
  br label %"49"

"49":                                             ; preds = %"48"
  store i32 %"38", ptr addrspace(5) %"54", align 4
  %"63" = load i8, ptr addrspace(5) %"62", align 1
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"63")
  br label %"50"

"50":                                             ; preds = %"49"
  store i32 %"40", ptr addrspace(5) %"55", align 4
  %"67" = load i32, ptr addrspace(5) %"54", align 4
  %"68" = load i32, ptr addrspace(5) %"65", align 4
  %2 = icmp eq i32 %"67", %"68"
  store i1 %2, ptr addrspace(5) %"56", align 1
  %"71" = load i1, ptr addrspace(5) %"69", align 1
  store i1 %"71", ptr addrspace(5) %"57", align 1
  %"72" = load i1, ptr addrspace(5) %"56", align 1
  br i1 %"72", label %"10", label %"19"

"19":                                             ; preds = %"50"
  %"76" = load i1, ptr addrspace(5) %"73", align 1
  %"77" = load i32, ptr addrspace(5) %"74", align 4
  %"75" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 %"76", i32 %"77")
  store i1 %"75", ptr addrspace(5) %"57", align 1
  br label %"10"

"10":                                             ; preds = %"19", %"50"
  %"81" = load i32, ptr addrspace(5) %"78", align 4
  %"82" = load i32, ptr addrspace(5) %"79", align 4
  %"83" = load i1, ptr addrspace(5) %"57", align 1
  %"80" = select i1 %"83", i32 %"81", i32 %"82"
  store i32 %"80", ptr addrspace(5) %"58", align 4
  %"87" = load i32, ptr addrspace(5) %"55", align 4
  %"88" = load i32, ptr addrspace(5) %"85", align 4
  %3 = zext i32 %"87" to i64
  %4 = zext i32 %"88" to i64
  %"86" = mul i64 %3, %4
  store i64 %"86", ptr addrspace(5) %"84", align 8
  %"90" = load i64, ptr addrspace(5) %"59", align 8
  %"91" = load i64, ptr addrspace(5) %"84", align 8
  %"89" = add i64 %"90", %"91"
  store i64 %"89", ptr addrspace(5) %"59", align 8
  %"92" = load i64, ptr addrspace(5) %"59", align 8
  %"93" = load i32, ptr addrspace(5) %"58", align 4
  %"94" = inttoptr i64 %"92" to ptr
  store i32 %"93", ptr %"94", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }