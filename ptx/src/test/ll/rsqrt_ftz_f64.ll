define amdgpu_kernel void @rsqrt_ftz_f64(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca double, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  %2 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %2, ptr addrspace(5) %"41", align 8
  %3 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %3, ptr addrspace(5) %"42", align 8
  %4 = load i64, ptr addrspace(5) %"41", align 8
  %"52" = inttoptr i64 %4 to ptr
  %5 = load double, ptr %"52", align 8
  store double %5, ptr addrspace(5) %"43", align 8
  %6 = load double, ptr addrspace(5) %"43", align 8
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
  %"48" = bitcast i64 %21 to double
  store double %"48", ptr addrspace(5) %"43", align 8
  %22 = load i64, ptr addrspace(5) %"42", align 8
  %23 = load double, ptr addrspace(5) %"43", align 8
  %"53" = inttoptr i64 %22 to ptr
  store double %23, ptr %"53", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.amdgcn.rsq.f64(double) #2

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind willreturn }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
