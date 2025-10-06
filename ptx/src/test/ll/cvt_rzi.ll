define amdgpu_kernel void @cvt_rzi(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  %"46" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %2, ptr addrspace(5) %"43", align 8
  %3 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %3, ptr addrspace(5) %"44", align 8
  %4 = load i64, ptr addrspace(5) %"43", align 8
  %"61" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"61", align 4
  store float %5, ptr addrspace(5) %"45", align 4
  %6 = load i64, ptr addrspace(5) %"43", align 8
  %"62" = inttoptr i64 %6 to ptr
  %"37" = getelementptr inbounds i8, ptr %"62", i64 4
  %7 = load float, ptr %"37", align 4
  store float %7, ptr addrspace(5) %"46", align 4
  %8 = load float, ptr addrspace(5) %"45", align 4
  %9 = call float @llvm.trunc.f32(float %8)
  store float %9, ptr addrspace(5) %"45", align 4
  %10 = load float, ptr addrspace(5) %"46", align 4
  %11 = call float @llvm.trunc.f32(float %10)
  store float %11, ptr addrspace(5) %"46", align 4
  %12 = load i64, ptr addrspace(5) %"44", align 8
  %13 = load float, ptr addrspace(5) %"45", align 4
  %"63" = inttoptr i64 %12 to ptr
  store float %13, ptr %"63", align 4
  %14 = load i64, ptr addrspace(5) %"44", align 8
  %"64" = inttoptr i64 %14 to ptr
  %"39" = getelementptr inbounds i8, ptr %"64", i64 4
  %15 = load float, ptr addrspace(5) %"46", align 4
  store float %15, ptr %"39", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.trunc.f32(float) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }