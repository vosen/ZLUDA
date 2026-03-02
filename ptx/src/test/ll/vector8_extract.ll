define amdgpu_kernel void @vector8_extract(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #0 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i16, align 2, addrspace(5)
  %"55" = alloca i16, align 2, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  %"57" = alloca i16, align 2, addrspace(5)
  %"58" = alloca i16, align 2, addrspace(5)
  %"59" = alloca i16, align 2, addrspace(5)
  %"60" = alloca i16, align 2, addrspace(5)
  %"61" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"49"

"49":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %2, ptr addrspace(5) %"52", align 8
  %3 = load i64, ptr addrspace(4) %"51", align 8
  store i64 %3, ptr addrspace(5) %"53", align 8
  %4 = load i64, ptr addrspace(5) %"52", align 8
  %"98" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load <8 x i8>, ptr addrspace(1) %"98", align 8
  %"99" = extractelement <8 x i8> %5, i8 0
  %"100" = extractelement <8 x i8> %5, i8 1
  %"101" = extractelement <8 x i8> %5, i8 2
  %"102" = extractelement <8 x i8> %5, i8 3
  %"103" = extractelement <8 x i8> %5, i8 4
  %"104" = extractelement <8 x i8> %5, i8 5
  %"105" = extractelement <8 x i8> %5, i8 6
  %"106" = extractelement <8 x i8> %5, i8 7
  %"65" = zext i8 %"99" to i16
  %"66" = zext i8 %"100" to i16
  %"67" = zext i8 %"101" to i16
  %"68" = zext i8 %"102" to i16
  %"69" = zext i8 %"103" to i16
  %"70" = zext i8 %"104" to i16
  %"71" = zext i8 %"105" to i16
  %"72" = zext i8 %"106" to i16
  store i16 %"65", ptr addrspace(5) %"54", align 2
  store i16 %"66", ptr addrspace(5) %"55", align 2
  store i16 %"67", ptr addrspace(5) %"56", align 2
  store i16 %"68", ptr addrspace(5) %"57", align 2
  store i16 %"69", ptr addrspace(5) %"58", align 2
  store i16 %"70", ptr addrspace(5) %"59", align 2
  store i16 %"71", ptr addrspace(5) %"60", align 2
  store i16 %"72", ptr addrspace(5) %"61", align 2
  %6 = load i16, ptr addrspace(5) %"57", align 2
  %7 = load i16, ptr addrspace(5) %"58", align 2
  %8 = load i16, ptr addrspace(5) %"59", align 2
  %9 = load i16, ptr addrspace(5) %"60", align 2
  %10 = load i16, ptr addrspace(5) %"61", align 2
  %11 = load i16, ptr addrspace(5) %"54", align 2
  %12 = load i16, ptr addrspace(5) %"55", align 2
  %13 = load i16, ptr addrspace(5) %"56", align 2
  %14 = insertelement <8 x i16> undef, i16 %6, i8 0
  %15 = insertelement <8 x i16> %14, i16 %7, i8 1
  %16 = insertelement <8 x i16> %15, i16 %8, i8 2
  %17 = insertelement <8 x i16> %16, i16 %9, i8 3
  %18 = insertelement <8 x i16> %17, i16 %10, i8 4
  %19 = insertelement <8 x i16> %18, i16 %11, i8 5
  %20 = insertelement <8 x i16> %19, i16 %12, i8 6
  %"47" = insertelement <8 x i16> %20, i16 %13, i8 7
  %"81" = extractelement <8 x i16> %"47", i8 0
  %"82" = extractelement <8 x i16> %"47", i8 1
  %"83" = extractelement <8 x i16> %"47", i8 2
  %"84" = extractelement <8 x i16> %"47", i8 3
  %"85" = extractelement <8 x i16> %"47", i8 4
  %"86" = extractelement <8 x i16> %"47", i8 5
  %"87" = extractelement <8 x i16> %"47", i8 6
  %"88" = extractelement <8 x i16> %"47", i8 7
  store i16 %"81", ptr addrspace(5) %"58", align 2
  store i16 %"82", ptr addrspace(5) %"59", align 2
  store i16 %"83", ptr addrspace(5) %"60", align 2
  store i16 %"84", ptr addrspace(5) %"61", align 2
  store i16 %"85", ptr addrspace(5) %"54", align 2
  store i16 %"86", ptr addrspace(5) %"55", align 2
  store i16 %"87", ptr addrspace(5) %"56", align 2
  store i16 %"88", ptr addrspace(5) %"57", align 2
  %21 = load i16, ptr addrspace(5) %"54", align 2
  %22 = load i16, ptr addrspace(5) %"55", align 2
  %23 = load i16, ptr addrspace(5) %"56", align 2
  %24 = load i16, ptr addrspace(5) %"57", align 2
  %25 = load i16, ptr addrspace(5) %"58", align 2
  %26 = load i16, ptr addrspace(5) %"59", align 2
  %27 = load i16, ptr addrspace(5) %"60", align 2
  %28 = load i16, ptr addrspace(5) %"61", align 2
  %"107" = trunc i16 %21 to i8
  %"108" = trunc i16 %22 to i8
  %"109" = trunc i16 %23 to i8
  %"110" = trunc i16 %24 to i8
  %"111" = trunc i16 %25 to i8
  %"112" = trunc i16 %26 to i8
  %"113" = trunc i16 %27 to i8
  %"114" = trunc i16 %28 to i8
  %29 = insertelement <8 x i8> undef, i8 %"107", i8 0
  %30 = insertelement <8 x i8> %29, i8 %"108", i8 1
  %31 = insertelement <8 x i8> %30, i8 %"109", i8 2
  %32 = insertelement <8 x i8> %31, i8 %"110", i8 3
  %33 = insertelement <8 x i8> %32, i8 %"111", i8 4
  %34 = insertelement <8 x i8> %33, i8 %"112", i8 5
  %35 = insertelement <8 x i8> %34, i8 %"113", i8 6
  %"48" = insertelement <8 x i8> %35, i8 %"114", i8 7
  %36 = load i64, ptr addrspace(5) %"53", align 8
  %"115" = inttoptr i64 %36 to ptr addrspace(1)
  store <8 x i8> %"48", ptr addrspace(1) %"115", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
