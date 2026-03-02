define amdgpu_kernel void @add_extended(ptr addrspace(4) byref(i64) %"70", ptr addrspace(4) byref(i64) %"71") #0 {
  %"72" = alloca i64, align 8, addrspace(5)
  %"73" = alloca i64, align 8, addrspace(5)
  %"74" = alloca i32, align 4, addrspace(5)
  %"75" = alloca i32, align 4, addrspace(5)
  %"76" = alloca i32, align 4, addrspace(5)
  %"77" = alloca i32, align 4, addrspace(5)
  %"78" = alloca i32, align 4, addrspace(5)
  %"79" = alloca i32, align 4, addrspace(5)
  %"80" = alloca i32, align 4, addrspace(5)
  %"81" = alloca i32, align 4, addrspace(5)
  %"82" = alloca i32, align 4, addrspace(5)
  %"83" = alloca i32, align 4, addrspace(5)
  %"84" = alloca i32, align 4, addrspace(5)
  %"85" = alloca i32, align 4, addrspace(5)
  %__ZLUDA_REG_CC_CF = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"69"

"69":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"70", align 8
  store i64 %2, ptr addrspace(5) %"72", align 8
  %3 = load i64, ptr addrspace(4) %"71", align 8
  store i64 %3, ptr addrspace(5) %"73", align 8
  %4 = load i64, ptr addrspace(5) %"72", align 8
  %"124" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"124", align 4
  store i32 %5, ptr addrspace(5) %"74", align 4
  %6 = load i64, ptr addrspace(5) %"72", align 8
  %"125" = inttoptr i64 %6 to ptr
  %"50" = getelementptr inbounds i8, ptr %"125", i64 4
  %7 = load i32, ptr %"50", align 4
  store i32 %7, ptr addrspace(5) %"75", align 4
  %8 = load i64, ptr addrspace(5) %"72", align 8
  %"126" = inttoptr i64 %8 to ptr
  %"52" = getelementptr inbounds i8, ptr %"126", i64 8
  %9 = load i32, ptr %"52", align 4
  store i32 %9, ptr addrspace(5) %"76", align 4
  %10 = load i64, ptr addrspace(5) %"72", align 8
  %"127" = inttoptr i64 %10 to ptr
  %"54" = getelementptr inbounds i8, ptr %"127", i64 12
  %11 = load i32, ptr %"54", align 4
  store i32 %11, ptr addrspace(5) %"77", align 4
  %12 = load i64, ptr addrspace(5) %"72", align 8
  %"128" = inttoptr i64 %12 to ptr
  %"56" = getelementptr inbounds i8, ptr %"128", i64 16
  %13 = load i32, ptr %"56", align 4
  store i32 %13, ptr addrspace(5) %"78", align 4
  %14 = load i64, ptr addrspace(5) %"72", align 8
  %"129" = inttoptr i64 %14 to ptr
  %"58" = getelementptr inbounds i8, ptr %"129", i64 20
  %15 = load i32, ptr %"58", align 4
  store i32 %15, ptr addrspace(5) %"79", align 4
  %16 = load i64, ptr addrspace(5) %"72", align 8
  %"130" = inttoptr i64 %16 to ptr
  %"60" = getelementptr inbounds i8, ptr %"130", i64 24
  %17 = load i32, ptr %"60", align 4
  store i32 %17, ptr addrspace(5) %"80", align 4
  %18 = load i64, ptr addrspace(5) %"72", align 8
  %"131" = inttoptr i64 %18 to ptr
  %"62" = getelementptr inbounds i8, ptr %"131", i64 28
  %19 = load i32, ptr %"62", align 4
  store i32 %19, ptr addrspace(5) %"81", align 4
  %20 = load i32, ptr addrspace(5) %"74", align 4
  %21 = load i32, ptr addrspace(5) %"78", align 4
  %22 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %20, i32 %21)
  %23 = extractvalue { i32, i1 } %22, 0
  %24 = extractvalue { i32, i1 } %22, 1
  store i1 %24, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %23, ptr addrspace(5) %"82", align 4
  %25 = load i32, ptr addrspace(5) %"75", align 4
  %26 = load i32, ptr addrspace(5) %"79", align 4
  %27 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %28 = zext i1 %27 to i32
  %29 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %25, i32 %26)
  %30 = extractvalue { i32, i1 } %29, 0
  %31 = extractvalue { i32, i1 } %29, 1
  %32 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %30, i32 %28)
  %33 = extractvalue { i32, i1 } %32, 0
  %34 = extractvalue { i32, i1 } %32, 1
  %35 = or i1 %31, %34
  store i1 %35, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %33, ptr addrspace(5) %"83", align 4
  %36 = load i32, ptr addrspace(5) %"76", align 4
  %37 = load i32, ptr addrspace(5) %"80", align 4
  %38 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %39 = zext i1 %38 to i32
  %40 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %36, i32 %37)
  %41 = extractvalue { i32, i1 } %40, 0
  %42 = extractvalue { i32, i1 } %40, 1
  %43 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %41, i32 %39)
  %44 = extractvalue { i32, i1 } %43, 0
  %45 = extractvalue { i32, i1 } %43, 1
  %46 = or i1 %42, %45
  store i1 %46, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %44, ptr addrspace(5) %"84", align 4
  %47 = load i32, ptr addrspace(5) %"77", align 4
  %48 = load i32, ptr addrspace(5) %"81", align 4
  %49 = add i32 %47, %48
  %50 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %51 = zext i1 %50 to i32
  %52 = add i32 %49, %51
  store i32 %52, ptr addrspace(5) %"85", align 4
  %53 = load i64, ptr addrspace(5) %"73", align 8
  %54 = load i32, ptr addrspace(5) %"82", align 4
  %"132" = inttoptr i64 %53 to ptr
  store i32 %54, ptr %"132", align 4
  %55 = load i64, ptr addrspace(5) %"73", align 8
  %"133" = inttoptr i64 %55 to ptr
  %"64" = getelementptr inbounds i8, ptr %"133", i64 4
  %56 = load i32, ptr addrspace(5) %"83", align 4
  store i32 %56, ptr %"64", align 4
  %57 = load i64, ptr addrspace(5) %"73", align 8
  %"134" = inttoptr i64 %57 to ptr
  %"66" = getelementptr inbounds i8, ptr %"134", i64 8
  %58 = load i32, ptr addrspace(5) %"84", align 4
  store i32 %58, ptr %"66", align 4
  %59 = load i64, ptr addrspace(5) %"73", align 8
  %"135" = inttoptr i64 %59 to ptr
  %"68" = getelementptr inbounds i8, ptr %"135", i64 12
  %60 = load i32, ptr addrspace(5) %"85", align 4
  store i32 %60, ptr %"68", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
