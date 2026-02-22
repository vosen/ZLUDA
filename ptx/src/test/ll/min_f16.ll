define amdgpu_kernel void @min_f16(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca half, align 2, addrspace(5)
  %"47" = alloca half, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %2, ptr addrspace(5) %"44", align 8
  %3 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"44", align 8
  %"60" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"60", align 2
  %"50" = bitcast i16 %5 to half
  store half %"50", ptr addrspace(5) %"46", align 2
  %6 = load i64, ptr addrspace(5) %"44", align 8
  %"61" = inttoptr i64 %6 to ptr
  %"40" = getelementptr inbounds i8, ptr %"61", i64 2
  %7 = load i16, ptr %"40", align 2
  %"53" = bitcast i16 %7 to half
  store half %"53", ptr addrspace(5) %"47", align 2
  %8 = load half, ptr addrspace(5) %"46", align 2
  %9 = load half, ptr addrspace(5) %"47", align 2
  %10 = call half @llvm.minnum.f16(half %8, half %9)
  store half %10, ptr addrspace(5) %"46", align 2
  %11 = load i64, ptr addrspace(5) %"45", align 8
  %12 = load half, ptr addrspace(5) %"46", align 2
  %"63" = inttoptr i64 %11 to ptr
  %"64" = bitcast half %12 to i16
  store i16 %"64", ptr %"63", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare half @llvm.minnum.f16(half, half) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
