declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @vector_extract(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i16, align 2, addrspace(5)
  %"50" = alloca i16, align 2, addrspace(5)
  %"51" = alloca i16, align 2, addrspace(5)
  %"52" = alloca i16, align 2, addrspace(5)
  %"53" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"94"

"94":                                             ; preds = %1
  %"54" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"54", ptr addrspace(5) %"47", align 4
  %"55" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"55", ptr addrspace(5) %"48", align 4
  %"56" = load i64, ptr addrspace(5) %"47", align 4
  %"84" = inttoptr i64 %"56" to ptr addrspace(1)
  %"33" = load <4 x i8>, ptr addrspace(1) %"84", align 4
  %"85" = extractelement <4 x i8> %"33", i8 0
  %"86" = extractelement <4 x i8> %"33", i8 1
  %"87" = extractelement <4 x i8> %"33", i8 2
  %"88" = extractelement <4 x i8> %"33", i8 3
  %"57" = zext i8 %"85" to i16
  %"58" = zext i8 %"86" to i16
  %"59" = zext i8 %"87" to i16
  %"60" = zext i8 %"88" to i16
  store i16 %"57", ptr addrspace(5) %"49", align 2
  store i16 %"58", ptr addrspace(5) %"50", align 2
  store i16 %"59", ptr addrspace(5) %"51", align 2
  store i16 %"60", ptr addrspace(5) %"52", align 2
  %"61" = load i16, ptr addrspace(5) %"50", align 2
  %"62" = load i16, ptr addrspace(5) %"51", align 2
  %"63" = load i16, ptr addrspace(5) %"52", align 2
  %"64" = load i16, ptr addrspace(5) %"49", align 2
  %2 = insertelement <4 x i16> undef, i16 %"61", i8 0
  %3 = insertelement <4 x i16> %2, i16 %"62", i8 1
  %4 = insertelement <4 x i16> %3, i16 %"63", i8 2
  %"34" = insertelement <4 x i16> %4, i16 %"64", i8 3
  store <4 x i16> %"34", ptr addrspace(5) %"53", align 8
  %"66" = load <4 x i16>, ptr addrspace(5) %"53", align 8
  %"67" = extractelement <4 x i16> %"66", i8 0
  %"68" = extractelement <4 x i16> %"66", i8 1
  %"69" = extractelement <4 x i16> %"66", i8 2
  %"70" = extractelement <4 x i16> %"66", i8 3
  store i16 %"67", ptr addrspace(5) %"51", align 2
  store i16 %"68", ptr addrspace(5) %"52", align 2
  store i16 %"69", ptr addrspace(5) %"49", align 2
  store i16 %"70", ptr addrspace(5) %"50", align 2
  %"71" = load i16, ptr addrspace(5) %"51", align 2
  %"72" = load i16, ptr addrspace(5) %"52", align 2
  %"73" = load i16, ptr addrspace(5) %"49", align 2
  %"74" = load i16, ptr addrspace(5) %"50", align 2
  %5 = insertelement <4 x i16> undef, i16 %"71", i8 0
  %6 = insertelement <4 x i16> %5, i16 %"72", i8 1
  %7 = insertelement <4 x i16> %6, i16 %"73", i8 2
  %"37" = insertelement <4 x i16> %7, i16 %"74", i8 3
  %"75" = extractelement <4 x i16> %"37", i8 0
  %"76" = extractelement <4 x i16> %"37", i8 1
  %"77" = extractelement <4 x i16> %"37", i8 2
  %"78" = extractelement <4 x i16> %"37", i8 3
  store i16 %"75", ptr addrspace(5) %"52", align 2
  store i16 %"76", ptr addrspace(5) %"49", align 2
  store i16 %"77", ptr addrspace(5) %"50", align 2
  store i16 %"78", ptr addrspace(5) %"51", align 2
  %"79" = load i16, ptr addrspace(5) %"49", align 2
  %"80" = load i16, ptr addrspace(5) %"50", align 2
  %"81" = load i16, ptr addrspace(5) %"51", align 2
  %"82" = load i16, ptr addrspace(5) %"52", align 2
  %"89" = trunc i16 %"79" to i8
  %"90" = trunc i16 %"80" to i8
  %"91" = trunc i16 %"81" to i8
  %"92" = trunc i16 %"82" to i8
  %8 = insertelement <4 x i8> undef, i8 %"89", i8 0
  %9 = insertelement <4 x i8> %8, i8 %"90", i8 1
  %10 = insertelement <4 x i8> %9, i8 %"91", i8 2
  %"38" = insertelement <4 x i8> %10, i8 %"92", i8 3
  %"83" = load i64, ptr addrspace(5) %"48", align 4
  %"93" = inttoptr i64 %"83" to ptr addrspace(1)
  store <4 x i8> %"38", ptr addrspace(1) %"93", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }