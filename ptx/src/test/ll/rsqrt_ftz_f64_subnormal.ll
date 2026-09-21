define amdgpu_kernel void @rsqrt_ftz_f64_subnormal(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca double, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  %2 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %2, ptr addrspace(5) %"43", align 8
  %3 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %3, ptr addrspace(5) %"44", align 8
  %4 = load i64, ptr addrspace(5) %"43", align 8
  %"64" = inttoptr i64 %4 to ptr
  %5 = load double, ptr %"64", align 8
  store double %5, ptr addrspace(5) %"45", align 8
  %6 = load double, ptr addrspace(5) %"45", align 8
  %7 = fcmp uno double %6, %6
  %8 = bitcast double %6 to i64
  %9 = and i64 %8, 9218868437227405312
  %10 = icmp eq i64 %9, 0
  %11 = and i64 %8, 4503599627370495
  %12 = icmp ne i64 %11, 0
  %13 = and i1 %10, %12
  %14 = and i64 %8, -9223372036854775808
  %15 = and i64 %8, -4294967296
  %16 = select i1 %13, i64 %14, i64 %15
  %17 = bitcast i64 %16 to double
  %18 = call double @llvm.amdgcn.rsq.f64(double %17)
  %19 = bitcast double %18 to i64
  %20 = and i64 %19, -4294967296
  %21 = select i1 %7, i64 9223372032559808512, i64 %20
  %"50" = bitcast i64 %21 to double
  store double %"50", ptr addrspace(5) %"45", align 8
  %22 = load i64, ptr addrspace(5) %"44", align 8
  %23 = load double, ptr addrspace(5) %"45", align 8
  %"65" = inttoptr i64 %22 to ptr
  store double %23, ptr %"65", align 8
  %24 = load i64, ptr addrspace(5) %"43", align 8
  %"54" = add i64 %24, 8
  store i64 %"54", ptr addrspace(5) %"43", align 8
  %25 = load i64, ptr addrspace(5) %"44", align 8
  %"56" = add i64 %25, 8
  store i64 %"56", ptr addrspace(5) %"44", align 8
  %26 = load i64, ptr addrspace(5) %"43", align 8
  %"66" = inttoptr i64 %26 to ptr
  %27 = load double, ptr %"66", align 8
  store double %27, ptr addrspace(5) %"45", align 8
  %28 = load double, ptr addrspace(5) %"45", align 8
  %29 = fcmp uno double %28, %28
  %30 = bitcast double %28 to i64
  %31 = and i64 %30, 9218868437227405312
  %32 = icmp eq i64 %31, 0
  %33 = and i64 %30, 4503599627370495
  %34 = icmp ne i64 %33, 0
  %35 = and i1 %32, %34
  %36 = and i64 %30, -9223372036854775808
  %37 = and i64 %30, -4294967296
  %38 = select i1 %35, i64 %36, i64 %37
  %39 = bitcast i64 %38 to double
  %40 = call double @llvm.amdgcn.rsq.f64(double %39)
  %41 = bitcast double %40 to i64
  %42 = and i64 %41, -4294967296
  %43 = select i1 %29, i64 9223372032559808512, i64 %42
  %"60" = bitcast i64 %43 to double
  store double %"60", ptr addrspace(5) %"45", align 8
  %44 = load i64, ptr addrspace(5) %"44", align 8
  %45 = load double, ptr addrspace(5) %"45", align 8
  %"67" = inttoptr i64 %44 to ptr
  store double %45, ptr %"67", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.amdgcn.rsq.f64(double) #2

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind willreturn }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
