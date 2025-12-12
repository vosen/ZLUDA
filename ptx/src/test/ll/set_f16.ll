define amdgpu_kernel void @set_f16(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i16, align 2, addrspace(5)
  %"54" = alloca i16, align 2, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %3 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %3, ptr addrspace(5) %"52", align 8
  %4 = load i64, ptr addrspace(5) %"51", align 8
  %"79" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"79", align 2
  store i16 %5, ptr addrspace(5) %"53", align 2
  %6 = load i64, ptr addrspace(5) %"51", align 8
  %"80" = inttoptr i64 %6 to ptr
  %"41" = getelementptr inbounds i8, ptr %"80", i64 2
  %7 = load i16, ptr %"41", align 2
  store i16 %7, ptr addrspace(5) %"54", align 2
  %8 = load i64, ptr addrspace(5) %"51", align 8
  %"81" = inttoptr i64 %8 to ptr
  %"43" = getelementptr inbounds i8, ptr %"81", i64 4
  %9 = load i32, ptr %"43", align 4
  store i32 %9, ptr addrspace(5) %"55", align 4
  %10 = load i64, ptr addrspace(5) %"51", align 8
  %"82" = inttoptr i64 %10 to ptr
  %"45" = getelementptr inbounds i8, ptr %"82", i64 8
  %11 = load i32, ptr %"45", align 4
  store i32 %11, ptr addrspace(5) %"56", align 4
  %12 = load i16, ptr addrspace(5) %"53", align 2
  %13 = load i16, ptr addrspace(5) %"54", align 2
  %"84" = bitcast i16 %12 to half
  %"85" = bitcast i16 %13 to half
  %14 = fcmp ugt half %"84", %"85"
  %15 = select i1 %14, i32 -1, i32 0
  store i32 %15, ptr addrspace(5) %"57", align 4
  %16 = load i32, ptr addrspace(5) %"55", align 4
  %17 = load i32, ptr addrspace(5) %"56", align 4
  %"87" = bitcast i32 %16 to <2 x half>
  %"88" = bitcast i32 %17 to <2 x half>
  %18 = extractelement <2 x half> %"87", i8 0
  %19 = extractelement <2 x half> %"87", i8 1
  %20 = extractelement <2 x half> %"88", i8 0
  %21 = extractelement <2 x half> %"88", i8 1
  %22 = fcmp ugt half %18, %20
  %23 = fcmp ugt half %19, %21
  %24 = select i1 %22, i16 -1, i16 0
  %25 = select i1 %23, i16 -1, i16 0
  %26 = insertelement <2 x i16> poison, i16 %24, i8 0
  %27 = insertelement <2 x i16> %26, i16 %25, i8 1
  %28 = bitcast <2 x i16> %27 to i32
  store i32 %28, ptr addrspace(5) %"58", align 4
  %29 = load i64, ptr addrspace(5) %"52", align 8
  %30 = load i32, ptr addrspace(5) %"57", align 4
  %"89" = inttoptr i64 %29 to ptr
  store i32 %30, ptr %"89", align 4
  %31 = load i64, ptr addrspace(5) %"52", align 8
  %"91" = inttoptr i64 %31 to ptr
  %"47" = getelementptr inbounds i8, ptr %"91", i64 4
  %32 = load i32, ptr addrspace(5) %"58", align 4
  store i32 %32, ptr %"47", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }