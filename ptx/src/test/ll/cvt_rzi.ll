define amdgpu_kernel void @cvt_rzi(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca float, align 4, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(5) %"40", align 8
  %"58" = inttoptr i64 %"47" to ptr
  %"46" = load float, ptr %"58", align 4
  store float %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 8
  %"59" = inttoptr i64 %"48" to ptr
  %"34" = getelementptr inbounds i8, ptr %"59", i64 4
  %"49" = load float, ptr %"34", align 4
  store float %"49", ptr addrspace(5) %"43", align 4
  %"51" = load float, ptr addrspace(5) %"42", align 4
  %2 = call float @llvm.trunc.f32(float %"51")
  store float %2, ptr addrspace(5) %"42", align 4
  %"53" = load float, ptr addrspace(5) %"43", align 4
  %3 = call float @llvm.trunc.f32(float %"53")
  store float %3, ptr addrspace(5) %"43", align 4
  %"54" = load i64, ptr addrspace(5) %"41", align 8
  %"55" = load float, ptr addrspace(5) %"42", align 4
  %"60" = inttoptr i64 %"54" to ptr
  store float %"55", ptr %"60", align 4
  %"56" = load i64, ptr addrspace(5) %"41", align 8
  %"61" = inttoptr i64 %"56" to ptr
  %"36" = getelementptr inbounds i8, ptr %"61", i64 4
  %"57" = load float, ptr addrspace(5) %"43", align 4
  store float %"57", ptr %"36", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.trunc.f32(float) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
