declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define <2 x i32> @impl(<2 x i32> %"9") #0 {
  %"53" = alloca <2 x i32>, align 8, addrspace(5)
  %"54" = alloca <2 x i32>, align 8, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"38" = extractelement <2 x i32> %"9", i8 0
  store i32 %"38", ptr addrspace(5) %"55", align 4
  %"39" = extractelement <2 x i32> %"9", i8 1
  store i32 %"39", ptr addrspace(5) %"56", align 4
  %"60" = load i32, ptr addrspace(5) %"55", align 4
  %"61" = load i32, ptr addrspace(5) %"56", align 4
  %"59" = add i32 %"60", %"61"
  store i32 %"59", ptr addrspace(5) %"56", align 4
  %"62" = load i32, ptr addrspace(5) %"56", align 4
  %"64" = load <2 x i32>, ptr addrspace(5) %"54", align 8
  %"63" = insertelement <2 x i32> %"64", i32 %"62", i8 0
  store <2 x i32> %"63", ptr addrspace(5) %"54", align 8
  %"65" = load i32, ptr addrspace(5) %"56", align 4
  %"67" = load <2 x i32>, ptr addrspace(5) %"54", align 8
  %"66" = insertelement <2 x i32> %"67", i32 %"65", i8 1
  store <2 x i32> %"66", ptr addrspace(5) %"54", align 8
  %"68" = load <2 x i32>, ptr addrspace(5) %"54", align 8
  %"43" = extractelement <2 x i32> %"68", i8 1
  %"70" = load <2 x i32>, ptr addrspace(5) %"54", align 8
  %"69" = insertelement <2 x i32> %"70", i32 %"43", i8 0
  store <2 x i32> %"69", ptr addrspace(5) %"54", align 8
  %"72" = load <2 x i32>, ptr addrspace(5) %"54", align 8
  store <2 x i32> %"72", ptr addrspace(5) %"53", align 8
  %2 = load <2 x i32>, ptr addrspace(5) %"53", align 8
  ret <2 x i32> %2
}

define amdgpu_kernel void @vector(ptr addrspace(4) byref(i64) %"73", ptr addrspace(4) byref(i64) %"74") #1 {
  %"75" = alloca i64, align 8, addrspace(5)
  %"76" = alloca i64, align 8, addrspace(5)
  %"77" = alloca <2 x i32>, align 8, addrspace(5)
  %"78" = alloca i32, align 4, addrspace(5)
  %"79" = alloca i32, align 4, addrspace(5)
  %"80" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %"81" = load i64, ptr addrspace(4) %"73", align 4
  store i64 %"81", ptr addrspace(5) %"75", align 4
  %"82" = load i64, ptr addrspace(4) %"74", align 4
  store i64 %"82", ptr addrspace(5) %"76", align 4
  %"84" = load i64, ptr addrspace(5) %"75", align 4
  %"91" = inttoptr i64 %"84" to ptr
  %"83" = load <2 x i32>, ptr %"91", align 8
  store <2 x i32> %"83", ptr addrspace(5) %"77", align 8
  %"86" = load <2 x i32>, ptr addrspace(5) %"77", align 8
  %"85" = call <2 x i32> @impl(<2 x i32> %"86")
  store <2 x i32> %"85", ptr addrspace(5) %"77", align 8
  br label %"46"

"46":                                             ; preds = %"45"
  %"88" = load <2 x i32>, ptr addrspace(5) %"77", align 8
  %"92" = bitcast <2 x i32> %"88" to i64
  store i64 %"92", ptr addrspace(5) %"80", align 4
  %"89" = load i64, ptr addrspace(5) %"76", align 4
  %"90" = load <2 x i32>, ptr addrspace(5) %"77", align 8
  %"93" = inttoptr i64 %"89" to ptr
  store <2 x i32> %"90", ptr %"93", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }