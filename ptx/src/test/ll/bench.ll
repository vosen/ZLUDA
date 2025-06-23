@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @bench(ptr addrspace(4) byref(i64) %"55", ptr addrspace(4) byref(i64) %"56") #1 {
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca float, align 4, addrspace(5)
  %"60" = alloca float, align 4, addrspace(5)
  %"61" = alloca float, align 4, addrspace(5)
  %"62" = alloca float, align 4, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"97"

"97":                                             ; preds = %1
  %"65" = load i64, ptr addrspace(4) %"55", align 4
  store i64 %"65", ptr addrspace(5) %"57", align 4
  %"66" = load i64, ptr addrspace(4) %"56", align 4
  store i64 %"66", ptr addrspace(5) %"58", align 4
  %"68" = load i64, ptr addrspace(5) %"57", align 4
  %"91" = inttoptr i64 %"68" to ptr
  %"67" = load float, ptr %"91", align 4
  store float %"67", ptr addrspace(5) %"59", align 4
  %"69" = load i64, ptr addrspace(5) %"57", align 4
  %"92" = inttoptr i64 %"69" to ptr
  %"39" = getelementptr inbounds i8, ptr %"92", i64 4
  %"70" = load float, ptr %"39", align 4
  store float %"70", ptr addrspace(5) %"60", align 4
  %"71" = load i64, ptr addrspace(5) %"57", align 4
  %"93" = inttoptr i64 %"71" to ptr
  %"41" = getelementptr inbounds i8, ptr %"93", i64 8
  %"72" = load float, ptr %"41", align 4
  store float %"72", ptr addrspace(5) %"61", align 4
  %"73" = load i64, ptr addrspace(5) %"57", align 4
  %"94" = inttoptr i64 %"73" to ptr
  %"43" = getelementptr inbounds i8, ptr %"94", i64 12
  %"74" = load float, ptr %"43", align 4
  store float %"74", ptr addrspace(5) %"62", align 4
  store i32 0, ptr addrspace(5) %"63", align 4
  br label %"10"

"10":                                             ; preds = %"21", %"97"
  %"77" = load float, ptr addrspace(5) %"59", align 4
  %"78" = load float, ptr addrspace(5) %"60", align 4
  call void asm sideeffect "s_denorm_mode 0", "~{mode}"()
  %"76" = fmul float %"77", %"78"
  store float %"76", ptr addrspace(5) %"59", align 4
  %"80" = load float, ptr addrspace(5) %"61", align 4
  %"81" = load float, ptr addrspace(5) %"62", align 4
  call void asm sideeffect "s_denorm_mode 11", "~{mode}"()
  %"79" = fmul float %"80", %"81"
  store float %"79", ptr addrspace(5) %"61", align 4
  %"83" = load i32, ptr addrspace(5) %"63", align 4
  %"82" = add i32 %"83", 1
  store i32 %"82", ptr addrspace(5) %"63", align 4
  %"85" = load i32, ptr addrspace(5) %"63", align 4
  %"84" = icmp eq i32 %"85", 100000000
  store i1 %"84", ptr addrspace(5) %"64", align 1
  %"86" = load i1, ptr addrspace(5) %"64", align 1
  br i1 %"86", label %"11", label %"21"

"21":                                             ; preds = %"10"
  br label %"10"

"11":                                             ; preds = %"10"
  %"87" = load i64, ptr addrspace(5) %"58", align 4
  %"88" = load float, ptr addrspace(5) %"59", align 4
  %"95" = inttoptr i64 %"87" to ptr
  store float %"88", ptr %"95", align 4
  %"89" = load i64, ptr addrspace(5) %"58", align 4
  %"96" = inttoptr i64 %"89" to ptr
  %"48" = getelementptr inbounds i8, ptr %"96", i64 4
  %"90" = load float, ptr addrspace(5) %"61", align 4
  store float %"90", ptr %"48", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }