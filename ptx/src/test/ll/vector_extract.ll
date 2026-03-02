define amdgpu_kernel void @vector_extract(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i16, align 2, addrspace(5)
  %"54" = alloca i16, align 2, addrspace(5)
  %"55" = alloca i16, align 2, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  %"57" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %3 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %3, ptr addrspace(5) %"52", align 8
  %4 = load i64, ptr addrspace(5) %"51", align 8
  %"88" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load <4 x i8>, ptr addrspace(1) %"88", align 4
  %"89" = extractelement <4 x i8> %5, i8 0
  %"90" = extractelement <4 x i8> %5, i8 1
  %"91" = extractelement <4 x i8> %5, i8 2
  %"92" = extractelement <4 x i8> %5, i8 3
  %"61" = zext i8 %"89" to i16
  %"62" = zext i8 %"90" to i16
  %"63" = zext i8 %"91" to i16
  %"64" = zext i8 %"92" to i16
  store i16 %"61", ptr addrspace(5) %"53", align 2
  store i16 %"62", ptr addrspace(5) %"54", align 2
  store i16 %"63", ptr addrspace(5) %"55", align 2
  store i16 %"64", ptr addrspace(5) %"56", align 2
  %6 = load i16, ptr addrspace(5) %"54", align 2
  %7 = load i16, ptr addrspace(5) %"55", align 2
  %8 = load i16, ptr addrspace(5) %"56", align 2
  %9 = load i16, ptr addrspace(5) %"53", align 2
  %10 = insertelement <4 x i16> undef, i16 %6, i8 0
  %11 = insertelement <4 x i16> %10, i16 %7, i8 1
  %12 = insertelement <4 x i16> %11, i16 %8, i8 2
  %"43" = insertelement <4 x i16> %12, i16 %9, i8 3
  store <4 x i16> %"43", ptr addrspace(5) %"57", align 8
  %13 = load <4 x i16>, ptr addrspace(5) %"57", align 8
  %"71" = extractelement <4 x i16> %13, i8 0
  %"72" = extractelement <4 x i16> %13, i8 1
  %"73" = extractelement <4 x i16> %13, i8 2
  %"74" = extractelement <4 x i16> %13, i8 3
  store i16 %"71", ptr addrspace(5) %"55", align 2
  store i16 %"72", ptr addrspace(5) %"56", align 2
  store i16 %"73", ptr addrspace(5) %"53", align 2
  store i16 %"74", ptr addrspace(5) %"54", align 2
  %14 = load i16, ptr addrspace(5) %"55", align 2
  %15 = load i16, ptr addrspace(5) %"56", align 2
  %16 = load i16, ptr addrspace(5) %"53", align 2
  %17 = load i16, ptr addrspace(5) %"54", align 2
  %18 = insertelement <4 x i16> undef, i16 %14, i8 0
  %19 = insertelement <4 x i16> %18, i16 %15, i8 1
  %20 = insertelement <4 x i16> %19, i16 %16, i8 2
  %"46" = insertelement <4 x i16> %20, i16 %17, i8 3
  %"79" = extractelement <4 x i16> %"46", i8 0
  %"80" = extractelement <4 x i16> %"46", i8 1
  %"81" = extractelement <4 x i16> %"46", i8 2
  %"82" = extractelement <4 x i16> %"46", i8 3
  store i16 %"79", ptr addrspace(5) %"56", align 2
  store i16 %"80", ptr addrspace(5) %"53", align 2
  store i16 %"81", ptr addrspace(5) %"54", align 2
  store i16 %"82", ptr addrspace(5) %"55", align 2
  %21 = load i16, ptr addrspace(5) %"53", align 2
  %22 = load i16, ptr addrspace(5) %"54", align 2
  %23 = load i16, ptr addrspace(5) %"55", align 2
  %24 = load i16, ptr addrspace(5) %"56", align 2
  %"93" = trunc i16 %21 to i8
  %"94" = trunc i16 %22 to i8
  %"95" = trunc i16 %23 to i8
  %"96" = trunc i16 %24 to i8
  %25 = insertelement <4 x i8> undef, i8 %"93", i8 0
  %26 = insertelement <4 x i8> %25, i8 %"94", i8 1
  %27 = insertelement <4 x i8> %26, i8 %"95", i8 2
  %"47" = insertelement <4 x i8> %27, i8 %"96", i8 3
  %28 = load i64, ptr addrspace(5) %"52", align 8
  %"97" = inttoptr i64 %28 to ptr addrspace(1)
  store <4 x i8> %"47", ptr addrspace(1) %"97", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
