define amdgpu_kernel void @sin(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"36", align 8
  store i64 %2, ptr addrspace(5) %"38", align 8
  %3 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %3, ptr addrspace(5) %"39", align 8
  %4 = load i64, ptr addrspace(5) %"38", align 8
  %"49" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"49", align 4
  store float %5, ptr addrspace(5) %"40", align 4
  %6 = load float, ptr addrspace(5) %"40", align 4
  %"45" = call afn float @llvm.sin.f32(float %6)
  store float %"45", ptr addrspace(5) %"40", align 4
  %7 = load i64, ptr addrspace(5) %"39", align 8
  %8 = load float, ptr addrspace(5) %"40", align 4
  %"50" = inttoptr i64 %7 to ptr
  store float %8, ptr %"50", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.sin.f32(float) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }