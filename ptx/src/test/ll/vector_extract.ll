define amdgpu_kernel void @vector_extract(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i16, align 2, addrspace(5)
  %"48" = alloca i16, align 2, addrspace(5)
  %"49" = alloca i16, align 2, addrspace(5)
  %"50" = alloca i16, align 2, addrspace(5)
  %"51" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %"52" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"52", ptr addrspace(5) %"45", align 8
  %"53" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"53", ptr addrspace(5) %"46", align 8
  %"54" = load i64, ptr addrspace(5) %"45", align 8
  %"82" = inttoptr i64 %"54" to ptr addrspace(1)
  %"36" = load <4 x i8>, ptr addrspace(1) %"82", align 4
  %"83" = extractelement <4 x i8> %"36", i8 0
  %"84" = extractelement <4 x i8> %"36", i8 1
  %"85" = extractelement <4 x i8> %"36", i8 2
  %"86" = extractelement <4 x i8> %"36", i8 3
  %"55" = zext i8 %"83" to i16
  %"56" = zext i8 %"84" to i16
  %"57" = zext i8 %"85" to i16
  %"58" = zext i8 %"86" to i16
  store i16 %"55", ptr addrspace(5) %"47", align 2
  store i16 %"56", ptr addrspace(5) %"48", align 2
  store i16 %"57", ptr addrspace(5) %"49", align 2
  store i16 %"58", ptr addrspace(5) %"50", align 2
  %"59" = load i16, ptr addrspace(5) %"48", align 2
  %"60" = load i16, ptr addrspace(5) %"49", align 2
  %"61" = load i16, ptr addrspace(5) %"50", align 2
  %"62" = load i16, ptr addrspace(5) %"47", align 2
  %2 = insertelement <4 x i16> undef, i16 %"59", i8 0
  %3 = insertelement <4 x i16> %2, i16 %"60", i8 1
  %4 = insertelement <4 x i16> %3, i16 %"61", i8 2
  %"37" = insertelement <4 x i16> %4, i16 %"62", i8 3
  store <4 x i16> %"37", ptr addrspace(5) %"51", align 8
  %"64" = load <4 x i16>, ptr addrspace(5) %"51", align 8
  %"65" = extractelement <4 x i16> %"64", i8 0
  %"66" = extractelement <4 x i16> %"64", i8 1
  %"67" = extractelement <4 x i16> %"64", i8 2
  %"68" = extractelement <4 x i16> %"64", i8 3
  store i16 %"65", ptr addrspace(5) %"49", align 2
  store i16 %"66", ptr addrspace(5) %"50", align 2
  store i16 %"67", ptr addrspace(5) %"47", align 2
  store i16 %"68", ptr addrspace(5) %"48", align 2
  %"69" = load i16, ptr addrspace(5) %"49", align 2
  %"70" = load i16, ptr addrspace(5) %"50", align 2
  %"71" = load i16, ptr addrspace(5) %"47", align 2
  %"72" = load i16, ptr addrspace(5) %"48", align 2
  %5 = insertelement <4 x i16> undef, i16 %"69", i8 0
  %6 = insertelement <4 x i16> %5, i16 %"70", i8 1
  %7 = insertelement <4 x i16> %6, i16 %"71", i8 2
  %"40" = insertelement <4 x i16> %7, i16 %"72", i8 3
  %"73" = extractelement <4 x i16> %"40", i8 0
  %"74" = extractelement <4 x i16> %"40", i8 1
  %"75" = extractelement <4 x i16> %"40", i8 2
  %"76" = extractelement <4 x i16> %"40", i8 3
  store i16 %"73", ptr addrspace(5) %"50", align 2
  store i16 %"74", ptr addrspace(5) %"47", align 2
  store i16 %"75", ptr addrspace(5) %"48", align 2
  store i16 %"76", ptr addrspace(5) %"49", align 2
  %"77" = load i16, ptr addrspace(5) %"47", align 2
  %"78" = load i16, ptr addrspace(5) %"48", align 2
  %"79" = load i16, ptr addrspace(5) %"49", align 2
  %"80" = load i16, ptr addrspace(5) %"50", align 2
  %"87" = trunc i16 %"77" to i8
  %"88" = trunc i16 %"78" to i8
  %"89" = trunc i16 %"79" to i8
  %"90" = trunc i16 %"80" to i8
  %8 = insertelement <4 x i8> undef, i8 %"87", i8 0
  %9 = insertelement <4 x i8> %8, i8 %"88", i8 1
  %10 = insertelement <4 x i8> %9, i8 %"89", i8 2
  %"41" = insertelement <4 x i8> %10, i8 %"90", i8 3
  %"81" = load i64, ptr addrspace(5) %"46", align 8
  %"91" = inttoptr i64 %"81" to ptr addrspace(1)
  store <4 x i8> %"41", ptr addrspace(1) %"91", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }