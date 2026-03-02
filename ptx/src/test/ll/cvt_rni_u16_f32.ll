define amdgpu_kernel void @cvt_rni_u16_f32(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  %"45" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"54" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load float, ptr addrspace(1) %"54", align 4
  store float %5, ptr addrspace(5) %"44", align 4
  %6 = load float, ptr addrspace(5) %"44", align 4
  %7 = call float @llvm.roundeven.f32(float %6)
  %"50" = call i16 @llvm.fptoui.sat.i16.f32(float %7)
  store i16 %"50", ptr addrspace(5) %"45", align 2
  %8 = load i64, ptr addrspace(5) %"43", align 8
  %9 = load i16, ptr addrspace(5) %"45", align 2
  %"55" = inttoptr i64 %8 to ptr
  store i16 %9, ptr %"55", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.roundeven.f32(float) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i16 @llvm.fptoui.sat.i16.f32(float) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
