declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define <2 x i32> @__zluda_ptx_impl_impl(<2 x i32> %"9") #0 {
  %"49" = alloca <2 x i32>, align 8, addrspace(5)
  %"50" = alloca <2 x i32>, align 8, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"37" = extractelement <2 x i32> %"9", i8 0
  store i32 %"37", ptr addrspace(5) %"51", align 4
  %"38" = extractelement <2 x i32> %"9", i8 1
  store i32 %"38", ptr addrspace(5) %"52", align 4
  %"56" = load i32, ptr addrspace(5) %"51", align 4
  %"57" = load i32, ptr addrspace(5) %"52", align 4
  %"55" = add i32 %"56", %"57"
  store i32 %"55", ptr addrspace(5) %"52", align 4
  %"58" = load i32, ptr addrspace(5) %"52", align 4
  %"60" = load <2 x i32>, ptr addrspace(5) %"50", align 8
  %"59" = insertelement <2 x i32> %"60", i32 %"58", i8 0
  store <2 x i32> %"59", ptr addrspace(5) %"50", align 8
  %"61" = load i32, ptr addrspace(5) %"52", align 4
  %"63" = load <2 x i32>, ptr addrspace(5) %"50", align 8
  %"62" = insertelement <2 x i32> %"63", i32 %"61", i8 1
  store <2 x i32> %"62", ptr addrspace(5) %"50", align 8
  %"64" = load <2 x i32>, ptr addrspace(5) %"50", align 8
  %"42" = extractelement <2 x i32> %"64", i8 1
  %"66" = load <2 x i32>, ptr addrspace(5) %"50", align 8
  %"65" = insertelement <2 x i32> %"66", i32 %"42", i8 0
  store <2 x i32> %"65", ptr addrspace(5) %"50", align 8
  %"68" = load <2 x i32>, ptr addrspace(5) %"50", align 8
  store <2 x i32> %"68", ptr addrspace(5) %"49", align 8
  %2 = load <2 x i32>, ptr addrspace(5) %"49", align 8
  ret <2 x i32> %2
}

define amdgpu_kernel void @vector(ptr addrspace(4) byref(i64) %"69", ptr addrspace(4) byref(i64) %"70") #0 {
  %"71" = alloca i64, align 8, addrspace(5)
  %"72" = alloca i64, align 8, addrspace(5)
  %"73" = alloca <2 x i32>, align 8, addrspace(5)
  %"74" = alloca i32, align 4, addrspace(5)
  %"75" = alloca i32, align 4, addrspace(5)
  %"76" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"77" = load i64, ptr addrspace(4) %"69", align 4
  store i64 %"77", ptr addrspace(5) %"71", align 4
  %"78" = load i64, ptr addrspace(4) %"70", align 4
  store i64 %"78", ptr addrspace(5) %"72", align 4
  %"80" = load i64, ptr addrspace(5) %"71", align 4
  %"87" = inttoptr i64 %"80" to ptr
  %"79" = load <2 x i32>, ptr %"87", align 8
  store <2 x i32> %"79", ptr addrspace(5) %"73", align 8
  %"82" = load <2 x i32>, ptr addrspace(5) %"73", align 8
  %"81" = call <2 x i32> @__zluda_ptx_impl_impl(<2 x i32> %"82")
  store <2 x i32> %"81", ptr addrspace(5) %"73", align 8
  %"84" = load <2 x i32>, ptr addrspace(5) %"73", align 8
  %"88" = bitcast <2 x i32> %"84" to i64
  store i64 %"88", ptr addrspace(5) %"76", align 4
  %"85" = load i64, ptr addrspace(5) %"72", align 4
  %"86" = load <2 x i32>, ptr addrspace(5) %"73", align 8
  %"89" = inttoptr i64 %"85" to ptr
  store <2 x i32> %"86", ptr %"89", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
