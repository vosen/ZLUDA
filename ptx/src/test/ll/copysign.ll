define amdgpu_kernel void @copysign(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %2, ptr addrspace(5) %"41", align 8
  %3 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %3, ptr addrspace(5) %"42", align 8
  %4 = load i64, ptr addrspace(5) %"41", align 8
  %"56" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"56", align 4
  store float %5, ptr addrspace(5) %"43", align 4
  %6 = load i64, ptr addrspace(5) %"41", align 8
  %"57" = inttoptr i64 %6 to ptr
  %"37" = getelementptr inbounds i8, ptr %"57", i64 4
  %7 = load float, ptr %"37", align 4
  store float %7, ptr addrspace(5) %"44", align 4
  %8 = load float, ptr addrspace(5) %"43", align 4
  %9 = load float, ptr addrspace(5) %"44", align 4
  %"51" = call float @llvm.copysign.f32(float %9, float %8)
  store float %"51", ptr addrspace(5) %"43", align 4
  %10 = load i64, ptr addrspace(5) %"42", align 8
  %11 = load float, ptr addrspace(5) %"43", align 4
  %"58" = inttoptr i64 %10 to ptr
  store float %11, ptr %"58", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.copysign.f32(float, float) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }