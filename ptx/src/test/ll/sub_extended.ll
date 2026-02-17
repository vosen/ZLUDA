define amdgpu_kernel void @sub_extended(ptr addrspace(4) byref(i64) %"67", ptr addrspace(4) byref(i64) %"68") #0 {
  %"69" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  %"71" = alloca i32, align 4, addrspace(5)
  %"72" = alloca i32, align 4, addrspace(5)
  %"73" = alloca i32, align 4, addrspace(5)
  %"74" = alloca i32, align 4, addrspace(5)
  %"75" = alloca i32, align 4, addrspace(5)
  %"76" = alloca i32, align 4, addrspace(5)
  %"77" = alloca i32, align 4, addrspace(5)
  %"78" = alloca i32, align 4, addrspace(5)
  %"79" = alloca i32, align 4, addrspace(5)
  %"80" = alloca i32, align 4, addrspace(5)
  %"81" = alloca i32, align 4, addrspace(5)
  %"82" = alloca i32, align 4, addrspace(5)
  %__ZLUDA_REG_CC_CF = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"66"

"66":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"67", align 8
  store i64 %2, ptr addrspace(5) %"69", align 8
  %3 = load i64, ptr addrspace(4) %"68", align 8
  store i64 %3, ptr addrspace(5) %"70", align 8
  %4 = load i64, ptr addrspace(5) %"69", align 8
  %"121" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"121", align 4
  store i32 %5, ptr addrspace(5) %"71", align 4
  %6 = load i64, ptr addrspace(5) %"69", align 8
  %"122" = inttoptr i64 %6 to ptr
  %"47" = getelementptr inbounds i8, ptr %"122", i64 4
  %7 = load i32, ptr %"47", align 4
  store i32 %7, ptr addrspace(5) %"72", align 4
  %8 = load i64, ptr addrspace(5) %"69", align 8
  %"123" = inttoptr i64 %8 to ptr
  %"49" = getelementptr inbounds i8, ptr %"123", i64 8
  %9 = load i32, ptr %"49", align 4
  store i32 %9, ptr addrspace(5) %"73", align 4
  %10 = load i64, ptr addrspace(5) %"69", align 8
  %"124" = inttoptr i64 %10 to ptr
  %"51" = getelementptr inbounds i8, ptr %"124", i64 12
  %11 = load i32, ptr %"51", align 4
  store i32 %11, ptr addrspace(5) %"74", align 4
  %12 = load i64, ptr addrspace(5) %"69", align 8
  %"125" = inttoptr i64 %12 to ptr
  %"53" = getelementptr inbounds i8, ptr %"125", i64 16
  %13 = load i32, ptr %"53", align 4
  store i32 %13, ptr addrspace(5) %"75", align 4
  %14 = load i64, ptr addrspace(5) %"69", align 8
  %"126" = inttoptr i64 %14 to ptr
  %"55" = getelementptr inbounds i8, ptr %"126", i64 20
  %15 = load i32, ptr %"55", align 4
  store i32 %15, ptr addrspace(5) %"76", align 4
  %16 = load i64, ptr addrspace(5) %"69", align 8
  %"127" = inttoptr i64 %16 to ptr
  %"57" = getelementptr inbounds i8, ptr %"127", i64 24
  %17 = load i32, ptr %"57", align 4
  store i32 %17, ptr addrspace(5) %"77", align 4
  %18 = load i64, ptr addrspace(5) %"69", align 8
  %"128" = inttoptr i64 %18 to ptr
  %"59" = getelementptr inbounds i8, ptr %"128", i64 28
  %19 = load i32, ptr %"59", align 4
  store i32 %19, ptr addrspace(5) %"78", align 4
  %20 = load i32, ptr addrspace(5) %"71", align 4
  %21 = load i32, ptr addrspace(5) %"75", align 4
  %22 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %20, i32 %21)
  %23 = extractvalue { i32, i1 } %22, 0
  %24 = extractvalue { i32, i1 } %22, 1
  %25 = xor i1 %24, true
  store i1 %25, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %23, ptr addrspace(5) %"79", align 4
  %26 = load i32, ptr addrspace(5) %"72", align 4
  %27 = load i32, ptr addrspace(5) %"76", align 4
  %28 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %29 = xor i1 %28, true
  %30 = zext i1 %29 to i32
  %31 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %26, i32 %27)
  %32 = extractvalue { i32, i1 } %31, 0
  %33 = extractvalue { i32, i1 } %31, 1
  %34 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %32, i32 %30)
  %35 = extractvalue { i32, i1 } %34, 0
  %36 = extractvalue { i32, i1 } %34, 1
  %37 = or i1 %33, %36
  %38 = xor i1 %37, true
  store i1 %38, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %35, ptr addrspace(5) %"80", align 4
  %39 = load i32, ptr addrspace(5) %"73", align 4
  %40 = load i32, ptr addrspace(5) %"77", align 4
  %41 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %42 = xor i1 %41, true
  %43 = zext i1 %42 to i32
  %44 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %39, i32 %40)
  %45 = extractvalue { i32, i1 } %44, 0
  %46 = extractvalue { i32, i1 } %44, 1
  %47 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %45, i32 %43)
  %48 = extractvalue { i32, i1 } %47, 0
  %49 = extractvalue { i32, i1 } %47, 1
  %50 = or i1 %46, %49
  %51 = xor i1 %50, true
  store i1 %51, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %48, ptr addrspace(5) %"81", align 4
  %52 = load i32, ptr addrspace(5) %"74", align 4
  %53 = load i32, ptr addrspace(5) %"78", align 4
  %54 = sub i32 %52, %53
  %55 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %56 = xor i1 %55, true
  %57 = zext i1 %56 to i32
  %58 = sub i32 %54, %57
  store i32 %58, ptr addrspace(5) %"82", align 4
  %59 = load i64, ptr addrspace(5) %"70", align 8
  %60 = load i32, ptr addrspace(5) %"79", align 4
  %"129" = inttoptr i64 %59 to ptr
  store i32 %60, ptr %"129", align 4
  %61 = load i64, ptr addrspace(5) %"70", align 8
  %"130" = inttoptr i64 %61 to ptr
  %"61" = getelementptr inbounds i8, ptr %"130", i64 4
  %62 = load i32, ptr addrspace(5) %"80", align 4
  store i32 %62, ptr %"61", align 4
  %63 = load i64, ptr addrspace(5) %"70", align 8
  %"131" = inttoptr i64 %63 to ptr
  %"63" = getelementptr inbounds i8, ptr %"131", i64 8
  %64 = load i32, ptr addrspace(5) %"81", align 4
  store i32 %64, ptr %"63", align 4
  %65 = load i64, ptr addrspace(5) %"70", align 8
  %"132" = inttoptr i64 %65 to ptr
  %"65" = getelementptr inbounds i8, ptr %"132", i64 12
  %66 = load i32, ptr addrspace(5) %"82", align 4
  store i32 %66, ptr %"65", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.usub.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
