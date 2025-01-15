declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @vector_extract(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #0 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i16, align 2, addrspace(5)
  %"49" = alloca i16, align 2, addrspace(5)
  %"50" = alloca i16, align 2, addrspace(5)
  %"51" = alloca i16, align 2, addrspace(5)
  %"52" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"53" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"53", ptr addrspace(5) %"46", align 4
  %"54" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"54", ptr addrspace(5) %"47", align 4
  %"55" = load i64, ptr addrspace(5) %"46", align 4
  %"83" = inttoptr i64 %"55" to ptr addrspace(1)
  %"32" = load <4 x i8>, ptr addrspace(1) %"83", align 4
  %"84" = extractelement <4 x i8> %"32", i8 0
  %"85" = extractelement <4 x i8> %"32", i8 1
  %"86" = extractelement <4 x i8> %"32", i8 2
  %"87" = extractelement <4 x i8> %"32", i8 3
  %"56" = zext i8 %"84" to i16
  %"57" = zext i8 %"85" to i16
  %"58" = zext i8 %"86" to i16
  %"59" = zext i8 %"87" to i16
  store i16 %"56", ptr addrspace(5) %"48", align 2
  store i16 %"57", ptr addrspace(5) %"49", align 2
  store i16 %"58", ptr addrspace(5) %"50", align 2
  store i16 %"59", ptr addrspace(5) %"51", align 2
  %"60" = load i16, ptr addrspace(5) %"49", align 2
  %"61" = load i16, ptr addrspace(5) %"50", align 2
  %"62" = load i16, ptr addrspace(5) %"51", align 2
  %"63" = load i16, ptr addrspace(5) %"48", align 2
  %2 = insertelement <4 x i16> undef, i16 %"60", i8 0
  %3 = insertelement <4 x i16> %2, i16 %"61", i8 1
  %4 = insertelement <4 x i16> %3, i16 %"62", i8 2
  %"33" = insertelement <4 x i16> %4, i16 %"63", i8 3
  store <4 x i16> %"33", ptr addrspace(5) %"52", align 8
  %"65" = load <4 x i16>, ptr addrspace(5) %"52", align 8
  %"66" = extractelement <4 x i16> %"65", i8 0
  %"67" = extractelement <4 x i16> %"65", i8 1
  %"68" = extractelement <4 x i16> %"65", i8 2
  %"69" = extractelement <4 x i16> %"65", i8 3
  store i16 %"66", ptr addrspace(5) %"50", align 2
  store i16 %"67", ptr addrspace(5) %"51", align 2
  store i16 %"68", ptr addrspace(5) %"48", align 2
  store i16 %"69", ptr addrspace(5) %"49", align 2
  %"70" = load i16, ptr addrspace(5) %"50", align 2
  %"71" = load i16, ptr addrspace(5) %"51", align 2
  %"72" = load i16, ptr addrspace(5) %"48", align 2
  %"73" = load i16, ptr addrspace(5) %"49", align 2
  %5 = insertelement <4 x i16> undef, i16 %"70", i8 0
  %6 = insertelement <4 x i16> %5, i16 %"71", i8 1
  %7 = insertelement <4 x i16> %6, i16 %"72", i8 2
  %"36" = insertelement <4 x i16> %7, i16 %"73", i8 3
  %"74" = extractelement <4 x i16> %"36", i8 0
  %"75" = extractelement <4 x i16> %"36", i8 1
  %"76" = extractelement <4 x i16> %"36", i8 2
  %"77" = extractelement <4 x i16> %"36", i8 3
  store i16 %"74", ptr addrspace(5) %"51", align 2
  store i16 %"75", ptr addrspace(5) %"48", align 2
  store i16 %"76", ptr addrspace(5) %"49", align 2
  store i16 %"77", ptr addrspace(5) %"50", align 2
  %"78" = load i16, ptr addrspace(5) %"48", align 2
  %"79" = load i16, ptr addrspace(5) %"49", align 2
  %"80" = load i16, ptr addrspace(5) %"50", align 2
  %"81" = load i16, ptr addrspace(5) %"51", align 2
  %"88" = trunc i16 %"78" to i8
  %"89" = trunc i16 %"79" to i8
  %"90" = trunc i16 %"80" to i8
  %"91" = trunc i16 %"81" to i8
  %8 = insertelement <4 x i8> undef, i8 %"88", i8 0
  %9 = insertelement <4 x i8> %8, i8 %"89", i8 1
  %10 = insertelement <4 x i8> %9, i8 %"90", i8 2
  %"37" = insertelement <4 x i8> %10, i8 %"91", i8 3
  %"82" = load i64, ptr addrspace(5) %"47", align 4
  %"92" = inttoptr i64 %"82" to ptr addrspace(1)
  store <4 x i8> %"37", ptr addrspace(1) %"92", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
