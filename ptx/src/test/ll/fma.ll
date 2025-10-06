define amdgpu_kernel void @fma(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca float, align 4, addrspace(5)
  %"47" = alloca float, align 4, addrspace(5)
  %"48" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %2, ptr addrspace(5) %"44", align 8
  %3 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"44", align 8
  %"63" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"63", align 4
  store float %5, ptr addrspace(5) %"46", align 4
  %6 = load i64, ptr addrspace(5) %"44", align 8
  %"64" = inttoptr i64 %6 to ptr
  %"38" = getelementptr inbounds i8, ptr %"64", i64 4
  %7 = load float, ptr %"38", align 4
  store float %7, ptr addrspace(5) %"47", align 4
  %8 = load i64, ptr addrspace(5) %"44", align 8
  %"65" = inttoptr i64 %8 to ptr
  %"40" = getelementptr inbounds i8, ptr %"65", i64 8
  %9 = load float, ptr %"40", align 4
  store float %9, ptr addrspace(5) %"48", align 4
  %10 = load float, ptr addrspace(5) %"46", align 4
  %11 = load float, ptr addrspace(5) %"47", align 4
  %12 = load float, ptr addrspace(5) %"48", align 4
  %"57" = call float @llvm.fma.f32(float %10, float %11, float %12)
  store float %"57", ptr addrspace(5) %"46", align 4
  %13 = load i64, ptr addrspace(5) %"45", align 8
  %14 = load float, ptr addrspace(5) %"46", align 4
  %"66" = inttoptr i64 %13 to ptr
  store float %14, ptr %"66", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.fma.f32(float, float, float) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }