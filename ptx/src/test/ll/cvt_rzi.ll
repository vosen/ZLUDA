define amdgpu_kernel void @cvt_rzi(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca float, align 4, addrspace(5)
  %"40" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 3)
  %"41" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  %"42" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"42", ptr addrspace(5) %"38", align 4
  %"44" = load i64, ptr addrspace(5) %"37", align 4
  %"55" = inttoptr i64 %"44" to ptr
  %"43" = load float, ptr %"55", align 4
  store float %"43", ptr addrspace(5) %"39", align 4
  %"45" = load i64, ptr addrspace(5) %"37", align 4
  %"56" = inttoptr i64 %"45" to ptr
  %"31" = getelementptr inbounds i8, ptr %"56", i64 4
  %"46" = load float, ptr %"31", align 4
  store float %"46", ptr addrspace(5) %"40", align 4
  %"48" = load float, ptr addrspace(5) %"39", align 4
  %2 = call float @llvm.trunc.f32(float %"48")
  %"47" = freeze float %2
  store float %"47", ptr addrspace(5) %"39", align 4
  %"50" = load float, ptr addrspace(5) %"40", align 4
  %3 = call float @llvm.trunc.f32(float %"50")
  %"49" = freeze float %3
  store float %"49", ptr addrspace(5) %"40", align 4
  %"51" = load i64, ptr addrspace(5) %"38", align 4
  %"52" = load float, ptr addrspace(5) %"39", align 4
  %"57" = inttoptr i64 %"51" to ptr
  store float %"52", ptr %"57", align 4
  %"53" = load i64, ptr addrspace(5) %"38", align 4
  %"58" = inttoptr i64 %"53" to ptr
  %"33" = getelementptr inbounds i8, ptr %"58", i64 4
  %"54" = load float, ptr addrspace(5) %"40", align 4
  store float %"54", ptr %"33", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.trunc.f32(float) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind willreturn }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }