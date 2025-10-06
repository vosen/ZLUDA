define amdgpu_kernel void @vector_extract(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i16, align 2, addrspace(5)
  %"51" = alloca i16, align 2, addrspace(5)
  %"52" = alloca i16, align 2, addrspace(5)
  %"53" = alloca i16, align 2, addrspace(5)
  %"54" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"48", align 8
  %3 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %3, ptr addrspace(5) %"49", align 8
  %4 = load i64, ptr addrspace(5) %"48", align 8
  %"85" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load <4 x i8>, ptr addrspace(1) %"85", align 4
  %"86" = extractelement <4 x i8> %5, i8 0
  %"87" = extractelement <4 x i8> %5, i8 1
  %"88" = extractelement <4 x i8> %5, i8 2
  %"89" = extractelement <4 x i8> %5, i8 3
  %"58" = zext i8 %"86" to i16
  %"59" = zext i8 %"87" to i16
  %"60" = zext i8 %"88" to i16
  %"61" = zext i8 %"89" to i16
  store i16 %"58", ptr addrspace(5) %"50", align 2
  store i16 %"59", ptr addrspace(5) %"51", align 2
  store i16 %"60", ptr addrspace(5) %"52", align 2
  store i16 %"61", ptr addrspace(5) %"53", align 2
  %6 = load i16, ptr addrspace(5) %"51", align 2
  %7 = load i16, ptr addrspace(5) %"52", align 2
  %8 = load i16, ptr addrspace(5) %"53", align 2
  %9 = load i16, ptr addrspace(5) %"50", align 2
  %10 = insertelement <4 x i16> undef, i16 %6, i8 0
  %11 = insertelement <4 x i16> %10, i16 %7, i8 1
  %12 = insertelement <4 x i16> %11, i16 %8, i8 2
  %"40" = insertelement <4 x i16> %12, i16 %9, i8 3
  store <4 x i16> %"40", ptr addrspace(5) %"54", align 8
  %13 = load <4 x i16>, ptr addrspace(5) %"54", align 8
  %"68" = extractelement <4 x i16> %13, i8 0
  %"69" = extractelement <4 x i16> %13, i8 1
  %"70" = extractelement <4 x i16> %13, i8 2
  %"71" = extractelement <4 x i16> %13, i8 3
  store i16 %"68", ptr addrspace(5) %"52", align 2
  store i16 %"69", ptr addrspace(5) %"53", align 2
  store i16 %"70", ptr addrspace(5) %"50", align 2
  store i16 %"71", ptr addrspace(5) %"51", align 2
  %14 = load i16, ptr addrspace(5) %"52", align 2
  %15 = load i16, ptr addrspace(5) %"53", align 2
  %16 = load i16, ptr addrspace(5) %"50", align 2
  %17 = load i16, ptr addrspace(5) %"51", align 2
  %18 = insertelement <4 x i16> undef, i16 %14, i8 0
  %19 = insertelement <4 x i16> %18, i16 %15, i8 1
  %20 = insertelement <4 x i16> %19, i16 %16, i8 2
  %"43" = insertelement <4 x i16> %20, i16 %17, i8 3
  %"76" = extractelement <4 x i16> %"43", i8 0
  %"77" = extractelement <4 x i16> %"43", i8 1
  %"78" = extractelement <4 x i16> %"43", i8 2
  %"79" = extractelement <4 x i16> %"43", i8 3
  store i16 %"76", ptr addrspace(5) %"53", align 2
  store i16 %"77", ptr addrspace(5) %"50", align 2
  store i16 %"78", ptr addrspace(5) %"51", align 2
  store i16 %"79", ptr addrspace(5) %"52", align 2
  %21 = load i16, ptr addrspace(5) %"50", align 2
  %22 = load i16, ptr addrspace(5) %"51", align 2
  %23 = load i16, ptr addrspace(5) %"52", align 2
  %24 = load i16, ptr addrspace(5) %"53", align 2
  %"90" = trunc i16 %21 to i8
  %"91" = trunc i16 %22 to i8
  %"92" = trunc i16 %23 to i8
  %"93" = trunc i16 %24 to i8
  %25 = insertelement <4 x i8> undef, i8 %"90", i8 0
  %26 = insertelement <4 x i8> %25, i8 %"91", i8 1
  %27 = insertelement <4 x i8> %26, i8 %"92", i8 2
  %"44" = insertelement <4 x i8> %27, i8 %"93", i8 3
  %28 = load i64, ptr addrspace(5) %"49", align 8
  %"94" = inttoptr i64 %28 to ptr addrspace(1)
  store <4 x i8> %"44", ptr addrspace(1) %"94", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }