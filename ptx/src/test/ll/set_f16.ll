define amdgpu_kernel void @set_f16(ptr addrspace(4) byref(i64) %"52", ptr addrspace(4) byref(i64) %"53") #0 {
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  %"57" = alloca i16, align 2, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i32, align 4, addrspace(5)
  %"60" = alloca i32, align 4, addrspace(5)
  %"61" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"52", align 8
  store i64 %2, ptr addrspace(5) %"54", align 8
  %3 = load i64, ptr addrspace(4) %"53", align 8
  store i64 %3, ptr addrspace(5) %"55", align 8
  %4 = load i64, ptr addrspace(5) %"54", align 8
  %"82" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"82", align 2
  store i16 %5, ptr addrspace(5) %"56", align 2
  %6 = load i64, ptr addrspace(5) %"54", align 8
  %"83" = inttoptr i64 %6 to ptr
  %"44" = getelementptr inbounds i8, ptr %"83", i64 2
  %7 = load i16, ptr %"44", align 2
  store i16 %7, ptr addrspace(5) %"57", align 2
  %8 = load i64, ptr addrspace(5) %"54", align 8
  %"84" = inttoptr i64 %8 to ptr
  %"46" = getelementptr inbounds i8, ptr %"84", i64 4
  %9 = load i32, ptr %"46", align 4
  store i32 %9, ptr addrspace(5) %"58", align 4
  %10 = load i64, ptr addrspace(5) %"54", align 8
  %"85" = inttoptr i64 %10 to ptr
  %"48" = getelementptr inbounds i8, ptr %"85", i64 8
  %11 = load i32, ptr %"48", align 4
  store i32 %11, ptr addrspace(5) %"59", align 4
  %12 = load i16, ptr addrspace(5) %"56", align 2
  %13 = load i16, ptr addrspace(5) %"57", align 2
  %"87" = bitcast i16 %12 to half
  %"88" = bitcast i16 %13 to half
  %14 = fcmp ugt half %"87", %"88"
  %15 = select i1 %14, i32 -1, i32 0
  store i32 %15, ptr addrspace(5) %"60", align 4
  %16 = load i32, ptr addrspace(5) %"58", align 4
  %17 = load i32, ptr addrspace(5) %"59", align 4
  %"90" = bitcast i32 %16 to <2 x half>
  %"91" = bitcast i32 %17 to <2 x half>
  %18 = extractelement <2 x half> %"90", i8 0
  %19 = extractelement <2 x half> %"90", i8 1
  %20 = extractelement <2 x half> %"91", i8 0
  %21 = extractelement <2 x half> %"91", i8 1
  %22 = fcmp ugt half %18, %20
  %23 = fcmp ugt half %19, %21
  %24 = select i1 %22, i16 -1, i16 0
  %25 = select i1 %23, i16 -1, i16 0
  %26 = insertelement <2 x i16> poison, i16 %24, i8 0
  %27 = insertelement <2 x i16> %26, i16 %25, i8 1
  %28 = bitcast <2 x i16> %27 to i32
  store i32 %28, ptr addrspace(5) %"61", align 4
  %29 = load i64, ptr addrspace(5) %"55", align 8
  %30 = load i32, ptr addrspace(5) %"60", align 4
  %"92" = inttoptr i64 %29 to ptr
  store i32 %30, ptr %"92", align 4
  %31 = load i64, ptr addrspace(5) %"55", align 8
  %"94" = inttoptr i64 %31 to ptr
  %"50" = getelementptr inbounds i8, ptr %"94", i64 4
  %32 = load i32, ptr addrspace(5) %"61", align 4
  store i32 %32, ptr %"50", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
