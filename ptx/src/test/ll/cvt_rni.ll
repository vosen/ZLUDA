define amdgpu_kernel void @cvt_rni(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #0 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca float, align 4, addrspace(5)
  %"49" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %2, ptr addrspace(5) %"46", align 8
  %3 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %3, ptr addrspace(5) %"47", align 8
  %4 = load i64, ptr addrspace(5) %"46", align 8
  %"64" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"64", align 4
  store float %5, ptr addrspace(5) %"48", align 4
  %6 = load i64, ptr addrspace(5) %"46", align 8
  %"65" = inttoptr i64 %6 to ptr
  %"40" = getelementptr inbounds i8, ptr %"65", i64 4
  %7 = load float, ptr %"40", align 4
  store float %7, ptr addrspace(5) %"49", align 4
  %8 = load float, ptr addrspace(5) %"48", align 4
  %9 = call float @llvm.roundeven.f32(float %8)
  store float %9, ptr addrspace(5) %"48", align 4
  %10 = load float, ptr addrspace(5) %"49", align 4
  %11 = call float @llvm.roundeven.f32(float %10)
  store float %11, ptr addrspace(5) %"49", align 4
  %12 = load i64, ptr addrspace(5) %"47", align 8
  %13 = load float, ptr addrspace(5) %"48", align 4
  %"66" = inttoptr i64 %12 to ptr
  store float %13, ptr %"66", align 4
  %14 = load i64, ptr addrspace(5) %"47", align 8
  %"67" = inttoptr i64 %14 to ptr
  %"42" = getelementptr inbounds i8, ptr %"67", i64 4
  %15 = load float, ptr addrspace(5) %"49", align 4
  store float %15, ptr %"42", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.roundeven.f32(float) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
