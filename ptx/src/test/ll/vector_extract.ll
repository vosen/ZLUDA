@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @vector_extract(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i16, align 2, addrspace(5)
  %"45" = alloca i16, align 2, addrspace(5)
  %"46" = alloca i16, align 2, addrspace(5)
  %"47" = alloca i16, align 2, addrspace(5)
  %"48" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %"49" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"49", ptr addrspace(5) %"42", align 4
  %"50" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"50", ptr addrspace(5) %"43", align 4
  %"51" = load i64, ptr addrspace(5) %"42", align 4
  %"79" = inttoptr i64 %"51" to ptr addrspace(1)
  %"33" = load <4 x i8>, ptr addrspace(1) %"79", align 4
  %"80" = extractelement <4 x i8> %"33", i8 0
  %"81" = extractelement <4 x i8> %"33", i8 1
  %"82" = extractelement <4 x i8> %"33", i8 2
  %"83" = extractelement <4 x i8> %"33", i8 3
  %"52" = zext i8 %"80" to i16
  %"53" = zext i8 %"81" to i16
  %"54" = zext i8 %"82" to i16
  %"55" = zext i8 %"83" to i16
  store i16 %"52", ptr addrspace(5) %"44", align 2
  store i16 %"53", ptr addrspace(5) %"45", align 2
  store i16 %"54", ptr addrspace(5) %"46", align 2
  store i16 %"55", ptr addrspace(5) %"47", align 2
  %"56" = load i16, ptr addrspace(5) %"45", align 2
  %"57" = load i16, ptr addrspace(5) %"46", align 2
  %"58" = load i16, ptr addrspace(5) %"47", align 2
  %"59" = load i16, ptr addrspace(5) %"44", align 2
  %2 = insertelement <4 x i16> undef, i16 %"56", i8 0
  %3 = insertelement <4 x i16> %2, i16 %"57", i8 1
  %4 = insertelement <4 x i16> %3, i16 %"58", i8 2
  %"34" = insertelement <4 x i16> %4, i16 %"59", i8 3
  store <4 x i16> %"34", ptr addrspace(5) %"48", align 8
  %"61" = load <4 x i16>, ptr addrspace(5) %"48", align 8
  %"62" = extractelement <4 x i16> %"61", i8 0
  %"63" = extractelement <4 x i16> %"61", i8 1
  %"64" = extractelement <4 x i16> %"61", i8 2
  %"65" = extractelement <4 x i16> %"61", i8 3
  store i16 %"62", ptr addrspace(5) %"46", align 2
  store i16 %"63", ptr addrspace(5) %"47", align 2
  store i16 %"64", ptr addrspace(5) %"44", align 2
  store i16 %"65", ptr addrspace(5) %"45", align 2
  %"66" = load i16, ptr addrspace(5) %"46", align 2
  %"67" = load i16, ptr addrspace(5) %"47", align 2
  %"68" = load i16, ptr addrspace(5) %"44", align 2
  %"69" = load i16, ptr addrspace(5) %"45", align 2
  %5 = insertelement <4 x i16> undef, i16 %"66", i8 0
  %6 = insertelement <4 x i16> %5, i16 %"67", i8 1
  %7 = insertelement <4 x i16> %6, i16 %"68", i8 2
  %"37" = insertelement <4 x i16> %7, i16 %"69", i8 3
  %"70" = extractelement <4 x i16> %"37", i8 0
  %"71" = extractelement <4 x i16> %"37", i8 1
  %"72" = extractelement <4 x i16> %"37", i8 2
  %"73" = extractelement <4 x i16> %"37", i8 3
  store i16 %"70", ptr addrspace(5) %"47", align 2
  store i16 %"71", ptr addrspace(5) %"44", align 2
  store i16 %"72", ptr addrspace(5) %"45", align 2
  store i16 %"73", ptr addrspace(5) %"46", align 2
  %"74" = load i16, ptr addrspace(5) %"44", align 2
  %"75" = load i16, ptr addrspace(5) %"45", align 2
  %"76" = load i16, ptr addrspace(5) %"46", align 2
  %"77" = load i16, ptr addrspace(5) %"47", align 2
  %"84" = trunc i16 %"74" to i8
  %"85" = trunc i16 %"75" to i8
  %"86" = trunc i16 %"76" to i8
  %"87" = trunc i16 %"77" to i8
  %8 = insertelement <4 x i8> undef, i8 %"84", i8 0
  %9 = insertelement <4 x i8> %8, i8 %"85", i8 1
  %10 = insertelement <4 x i8> %9, i8 %"86", i8 2
  %"38" = insertelement <4 x i8> %10, i8 %"87", i8 3
  %"78" = load i64, ptr addrspace(5) %"43", align 4
  %"88" = inttoptr i64 %"78" to ptr addrspace(1)
  store <4 x i8> %"38", ptr addrspace(1) %"88", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }