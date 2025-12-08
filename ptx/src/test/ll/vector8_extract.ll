define amdgpu_kernel void @vector8_extract(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #0 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i16, align 2, addrspace(5)
  %"52" = alloca i16, align 2, addrspace(5)
  %"53" = alloca i16, align 2, addrspace(5)
  %"54" = alloca i16, align 2, addrspace(5)
  %"55" = alloca i16, align 2, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  %"57" = alloca i16, align 2, addrspace(5)
  %"58" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %2, ptr addrspace(5) %"49", align 8
  %3 = load i64, ptr addrspace(4) %"48", align 8
  store i64 %3, ptr addrspace(5) %"50", align 8
  %4 = load i64, ptr addrspace(5) %"49", align 8
  %"95" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load <8 x i8>, ptr addrspace(1) %"95", align 8
  %"96" = extractelement <8 x i8> %5, i8 0
  %"97" = extractelement <8 x i8> %5, i8 1
  %"98" = extractelement <8 x i8> %5, i8 2
  %"99" = extractelement <8 x i8> %5, i8 3
  %"100" = extractelement <8 x i8> %5, i8 4
  %"101" = extractelement <8 x i8> %5, i8 5
  %"102" = extractelement <8 x i8> %5, i8 6
  %"103" = extractelement <8 x i8> %5, i8 7
  %"62" = zext i8 %"96" to i16
  %"63" = zext i8 %"97" to i16
  %"64" = zext i8 %"98" to i16
  %"65" = zext i8 %"99" to i16
  %"66" = zext i8 %"100" to i16
  %"67" = zext i8 %"101" to i16
  %"68" = zext i8 %"102" to i16
  %"69" = zext i8 %"103" to i16
  store i16 %"62", ptr addrspace(5) %"51", align 2
  store i16 %"63", ptr addrspace(5) %"52", align 2
  store i16 %"64", ptr addrspace(5) %"53", align 2
  store i16 %"65", ptr addrspace(5) %"54", align 2
  store i16 %"66", ptr addrspace(5) %"55", align 2
  store i16 %"67", ptr addrspace(5) %"56", align 2
  store i16 %"68", ptr addrspace(5) %"57", align 2
  store i16 %"69", ptr addrspace(5) %"58", align 2
  %6 = load i16, ptr addrspace(5) %"54", align 2
  %7 = load i16, ptr addrspace(5) %"55", align 2
  %8 = load i16, ptr addrspace(5) %"56", align 2
  %9 = load i16, ptr addrspace(5) %"57", align 2
  %10 = load i16, ptr addrspace(5) %"58", align 2
  %11 = load i16, ptr addrspace(5) %"51", align 2
  %12 = load i16, ptr addrspace(5) %"52", align 2
  %13 = load i16, ptr addrspace(5) %"53", align 2
  %14 = insertelement <8 x i16> undef, i16 %6, i8 0
  %15 = insertelement <8 x i16> %14, i16 %7, i8 1
  %16 = insertelement <8 x i16> %15, i16 %8, i8 2
  %17 = insertelement <8 x i16> %16, i16 %9, i8 3
  %18 = insertelement <8 x i16> %17, i16 %10, i8 4
  %19 = insertelement <8 x i16> %18, i16 %11, i8 5
  %20 = insertelement <8 x i16> %19, i16 %12, i8 6
  %"44" = insertelement <8 x i16> %20, i16 %13, i8 7
  %"78" = extractelement <8 x i16> %"44", i8 0
  %"79" = extractelement <8 x i16> %"44", i8 1
  %"80" = extractelement <8 x i16> %"44", i8 2
  %"81" = extractelement <8 x i16> %"44", i8 3
  %"82" = extractelement <8 x i16> %"44", i8 4
  %"83" = extractelement <8 x i16> %"44", i8 5
  %"84" = extractelement <8 x i16> %"44", i8 6
  %"85" = extractelement <8 x i16> %"44", i8 7
  store i16 %"78", ptr addrspace(5) %"55", align 2
  store i16 %"79", ptr addrspace(5) %"56", align 2
  store i16 %"80", ptr addrspace(5) %"57", align 2
  store i16 %"81", ptr addrspace(5) %"58", align 2
  store i16 %"82", ptr addrspace(5) %"51", align 2
  store i16 %"83", ptr addrspace(5) %"52", align 2
  store i16 %"84", ptr addrspace(5) %"53", align 2
  store i16 %"85", ptr addrspace(5) %"54", align 2
  %21 = load i16, ptr addrspace(5) %"51", align 2
  %22 = load i16, ptr addrspace(5) %"52", align 2
  %23 = load i16, ptr addrspace(5) %"53", align 2
  %24 = load i16, ptr addrspace(5) %"54", align 2
  %25 = load i16, ptr addrspace(5) %"55", align 2
  %26 = load i16, ptr addrspace(5) %"56", align 2
  %27 = load i16, ptr addrspace(5) %"57", align 2
  %28 = load i16, ptr addrspace(5) %"58", align 2
  %"104" = trunc i16 %21 to i8
  %"105" = trunc i16 %22 to i8
  %"106" = trunc i16 %23 to i8
  %"107" = trunc i16 %24 to i8
  %"108" = trunc i16 %25 to i8
  %"109" = trunc i16 %26 to i8
  %"110" = trunc i16 %27 to i8
  %"111" = trunc i16 %28 to i8
  %29 = insertelement <8 x i8> undef, i8 %"104", i8 0
  %30 = insertelement <8 x i8> %29, i8 %"105", i8 1
  %31 = insertelement <8 x i8> %30, i8 %"106", i8 2
  %32 = insertelement <8 x i8> %31, i8 %"107", i8 3
  %33 = insertelement <8 x i8> %32, i8 %"108", i8 4
  %34 = insertelement <8 x i8> %33, i8 %"109", i8 5
  %35 = insertelement <8 x i8> %34, i8 %"110", i8 6
  %"45" = insertelement <8 x i8> %35, i8 %"111", i8 7
  %36 = load i64, ptr addrspace(5) %"50", align 8
  %"112" = inttoptr i64 %36 to ptr addrspace(1)
  store <8 x i8> %"45", ptr addrspace(1) %"112", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
