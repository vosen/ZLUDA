declare hidden i1 @__zluda_ptx_impl_vote_sync_all_pred(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare hidden i32 @__zluda_ptx_impl_sreg_laneid() #0

define amdgpu_kernel void @vote_all(ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i1, align 1, addrspace(5)
  %"55" = alloca i1, align 1, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"60", align 1
  %"63" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"63", align 4
  %"67" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"67", align 1
  %"71" = alloca i32, align 4, addrspace(5)
  store i32 -2, ptr addrspace(5) %"71", align 4
  %"75" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"75", align 4
  %"76" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"76", align 4
  %"81" = alloca i64, align 8, addrspace(5)
  %"82" = alloca i32, align 4, addrspace(5)
  store i32 4, ptr addrspace(5) %"82", align 4
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %"58" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"58", ptr addrspace(5) %"57", align 8
  %"37" = call i32 @__zluda_ptx_impl_sreg_laneid()
  br label %"47"

"47":                                             ; preds = %"46"
  store i32 %"37", ptr addrspace(5) %"52", align 4
  %"61" = load i8, ptr addrspace(5) %"60", align 1
  %"39" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"61")
  br label %"48"

"48":                                             ; preds = %"47"
  store i32 %"39", ptr addrspace(5) %"53", align 4
  %"65" = load i32, ptr addrspace(5) %"52", align 4
  %"66" = load i32, ptr addrspace(5) %"63", align 4
  %2 = icmp ne i32 %"65", %"66"
  store i1 %2, ptr addrspace(5) %"54", align 1
  %"69" = load i1, ptr addrspace(5) %"67", align 1
  store i1 %"69", ptr addrspace(5) %"55", align 1
  %"70" = load i1, ptr addrspace(5) %"54", align 1
  br i1 %"70", label %"17", label %"18"

"17":                                             ; preds = %"48"
  %"73" = load i1, ptr addrspace(5) %"54", align 1
  %"74" = load i32, ptr addrspace(5) %"71", align 4
  %"72" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 %"73", i32 %"74")
  store i1 %"72", ptr addrspace(5) %"55", align 1
  br label %"18"

"18":                                             ; preds = %"17", %"48"
  %"78" = load i32, ptr addrspace(5) %"75", align 4
  %"79" = load i32, ptr addrspace(5) %"76", align 4
  %"80" = load i1, ptr addrspace(5) %"55", align 1
  %"77" = select i1 %"80", i32 %"78", i32 %"79"
  store i32 %"77", ptr addrspace(5) %"56", align 4
  %"84" = load i32, ptr addrspace(5) %"53", align 4
  %"85" = load i32, ptr addrspace(5) %"82", align 4
  %3 = zext i32 %"84" to i64
  %4 = zext i32 %"85" to i64
  %"83" = mul i64 %3, %4
  store i64 %"83", ptr addrspace(5) %"81", align 8
  %"87" = load i64, ptr addrspace(5) %"57", align 8
  %"88" = load i64, ptr addrspace(5) %"81", align 8
  %"86" = add i64 %"87", %"88"
  store i64 %"86", ptr addrspace(5) %"57", align 8
  %"89" = load i64, ptr addrspace(5) %"57", align 8
  %"90" = load i32, ptr addrspace(5) %"56", align 4
  %"91" = inttoptr i64 %"89" to ptr
  store i32 %"90", ptr %"91", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }