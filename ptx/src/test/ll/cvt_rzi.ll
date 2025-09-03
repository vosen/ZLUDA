define amdgpu_kernel void @cvt_rzi(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca float, align 4, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"48", align 4
  %"58" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"58", align 4
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(5) %"40", align 8
  %"62" = inttoptr i64 %"47" to ptr
  %"46" = load float, ptr %"62", align 4
  store float %"46", ptr addrspace(5) %"42", align 4
  %"49" = load i64, ptr addrspace(5) %"40", align 8
  %"50" = load i64, ptr addrspace(5) %"48", align 8
  %"63" = inttoptr i64 %"49" to ptr
  %"34" = getelementptr inbounds i8, ptr %"63", i64 %"50"
  %"51" = load float, ptr %"34", align 4
  store float %"51", ptr addrspace(5) %"43", align 4
  %"53" = load float, ptr addrspace(5) %"42", align 4
  %2 = call float @llvm.trunc.f32(float %"53")
  store float %2, ptr addrspace(5) %"42", align 4
  %"55" = load float, ptr addrspace(5) %"43", align 4
  %3 = call float @llvm.trunc.f32(float %"55")
  store float %3, ptr addrspace(5) %"43", align 4
  %"56" = load i64, ptr addrspace(5) %"41", align 8
  %"57" = load float, ptr addrspace(5) %"42", align 4
  %"64" = inttoptr i64 %"56" to ptr
  store float %"57", ptr %"64", align 4
  %"59" = load i64, ptr addrspace(5) %"41", align 8
  %"60" = load i64, ptr addrspace(5) %"58", align 8
  %"65" = inttoptr i64 %"59" to ptr
  %"36" = getelementptr inbounds i8, ptr %"65", i64 %"60"
  %"61" = load float, ptr addrspace(5) %"43", align 4
  store float %"61", ptr %"36", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.trunc.f32(float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }