define amdgpu_kernel void @mad_extended(ptr addrspace(4) byref(i64) %"61", ptr addrspace(4) byref(i64) %"62") #0 {
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i32, align 4, addrspace(5)
  %"66" = alloca i32, align 4, addrspace(5)
  %"67" = alloca i32, align 4, addrspace(5)
  %"68" = alloca i32, align 4, addrspace(5)
  %"69" = alloca i32, align 4, addrspace(5)
  %"70" = alloca i32, align 4, addrspace(5)
  %"71" = alloca i32, align 4, addrspace(5)
  %"72" = alloca i32, align 4, addrspace(5)
  %"73" = alloca i32, align 4, addrspace(5)
  %"74" = alloca i32, align 4, addrspace(5)
  %__ZLUDA_REG_CC_CF = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"60"

"60":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"61", align 8
  store i64 %2, ptr addrspace(5) %"63", align 8
  %3 = load i64, ptr addrspace(4) %"62", align 8
  store i64 %3, ptr addrspace(5) %"64", align 8
  %4 = load i64, ptr addrspace(5) %"63", align 8
  %"113" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"113", align 4
  store i32 %5, ptr addrspace(5) %"65", align 4
  %6 = load i64, ptr addrspace(5) %"63", align 8
  %"114" = inttoptr i64 %6 to ptr
  %"45" = getelementptr inbounds i8, ptr %"114", i64 4
  %7 = load i32, ptr %"45", align 4
  store i32 %7, ptr addrspace(5) %"66", align 4
  %8 = load i64, ptr addrspace(5) %"63", align 8
  %"115" = inttoptr i64 %8 to ptr
  %"47" = getelementptr inbounds i8, ptr %"115", i64 8
  %9 = load i32, ptr %"47", align 4
  store i32 %9, ptr addrspace(5) %"67", align 4
  %10 = load i64, ptr addrspace(5) %"63", align 8
  %"116" = inttoptr i64 %10 to ptr
  %"49" = getelementptr inbounds i8, ptr %"116", i64 12
  %11 = load i32, ptr %"49", align 4
  store i32 %11, ptr addrspace(5) %"68", align 4
  %12 = load i64, ptr addrspace(5) %"63", align 8
  %"117" = inttoptr i64 %12 to ptr
  %"51" = getelementptr inbounds i8, ptr %"117", i64 16
  %13 = load i32, ptr %"51", align 4
  store i32 %13, ptr addrspace(5) %"69", align 4
  %14 = load i64, ptr addrspace(5) %"63", align 8
  %"118" = inttoptr i64 %14 to ptr
  %"53" = getelementptr inbounds i8, ptr %"118", i64 20
  %15 = load i32, ptr %"53", align 4
  store i32 %15, ptr addrspace(5) %"70", align 4
  %16 = load i32, ptr addrspace(5) %"65", align 4
  %17 = load i32, ptr addrspace(5) %"66", align 4
  %18 = load i32, ptr addrspace(5) %"67", align 4
  %19 = mul i32 %16, %17
  %20 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %19, i32 %18)
  %21 = extractvalue { i32, i1 } %20, 0
  %22 = extractvalue { i32, i1 } %20, 1
  store i1 %22, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %21, ptr addrspace(5) %"71", align 4
  %23 = load i32, ptr addrspace(5) %"65", align 4
  %24 = load i32, ptr addrspace(5) %"66", align 4
  %25 = load i32, ptr addrspace(5) %"68", align 4
  %26 = mul i32 %23, %24
  %27 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %28 = zext i1 %27 to i32
  %29 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %26, i32 %25)
  %30 = extractvalue { i32, i1 } %29, 0
  %31 = extractvalue { i32, i1 } %29, 1
  %32 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %30, i32 %28)
  %33 = extractvalue { i32, i1 } %32, 0
  %34 = extractvalue { i32, i1 } %32, 1
  %35 = or i1 %31, %34
  store i1 %35, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %33, ptr addrspace(5) %"72", align 4
  %36 = load i32, ptr addrspace(5) %"65", align 4
  %37 = load i32, ptr addrspace(5) %"66", align 4
  %38 = load i32, ptr addrspace(5) %"69", align 4
  %39 = mul i32 %36, %37
  %40 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %41 = zext i1 %40 to i32
  %42 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %39, i32 %38)
  %43 = extractvalue { i32, i1 } %42, 0
  %44 = extractvalue { i32, i1 } %42, 1
  %45 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %43, i32 %41)
  %46 = extractvalue { i32, i1 } %45, 0
  %47 = extractvalue { i32, i1 } %45, 1
  %48 = or i1 %44, %47
  store i1 %48, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %46, ptr addrspace(5) %"73", align 4
  %49 = load i32, ptr addrspace(5) %"65", align 4
  %50 = load i32, ptr addrspace(5) %"66", align 4
  %51 = load i32, ptr addrspace(5) %"70", align 4
  %52 = mul i32 %49, %50
  %53 = add i32 %52, %51
  %54 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %55 = zext i1 %54 to i32
  %56 = add i32 %53, %55
  store i32 %56, ptr addrspace(5) %"74", align 4
  %57 = load i64, ptr addrspace(5) %"64", align 8
  %58 = load i32, ptr addrspace(5) %"71", align 4
  %"119" = inttoptr i64 %57 to ptr
  store i32 %58, ptr %"119", align 4
  %59 = load i64, ptr addrspace(5) %"64", align 8
  %"120" = inttoptr i64 %59 to ptr
  %"55" = getelementptr inbounds i8, ptr %"120", i64 4
  %60 = load i32, ptr addrspace(5) %"72", align 4
  store i32 %60, ptr %"55", align 4
  %61 = load i64, ptr addrspace(5) %"64", align 8
  %"121" = inttoptr i64 %61 to ptr
  %"57" = getelementptr inbounds i8, ptr %"121", i64 8
  %62 = load i32, ptr addrspace(5) %"73", align 4
  store i32 %62, ptr %"57", align 4
  %63 = load i64, ptr addrspace(5) %"64", align 8
  %"122" = inttoptr i64 %63 to ptr
  %"59" = getelementptr inbounds i8, ptr %"122", i64 12
  %64 = load i32, ptr addrspace(5) %"74", align 4
  store i32 %64, ptr %"59", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
