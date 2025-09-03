define amdgpu_kernel void @fma(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"50", align 4
  %"54" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"54", align 4
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"46", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"47", ptr addrspace(5) %"42", align 8
  %"49" = load i64, ptr addrspace(5) %"41", align 8
  %"64" = inttoptr i64 %"49" to ptr
  %"48" = load float, ptr %"64", align 4
  store float %"48", ptr addrspace(5) %"43", align 4
  %"51" = load i64, ptr addrspace(5) %"41", align 8
  %"52" = load i64, ptr addrspace(5) %"50", align 8
  %"65" = inttoptr i64 %"51" to ptr
  %"35" = getelementptr inbounds i8, ptr %"65", i64 %"52"
  %"53" = load float, ptr %"35", align 4
  store float %"53", ptr addrspace(5) %"44", align 4
  %"55" = load i64, ptr addrspace(5) %"41", align 8
  %"56" = load i64, ptr addrspace(5) %"54", align 8
  %"66" = inttoptr i64 %"55" to ptr
  %"37" = getelementptr inbounds i8, ptr %"66", i64 %"56"
  %"57" = load float, ptr %"37", align 4
  store float %"57", ptr addrspace(5) %"45", align 4
  %"59" = load float, ptr addrspace(5) %"43", align 4
  %"60" = load float, ptr addrspace(5) %"44", align 4
  %"61" = load float, ptr addrspace(5) %"45", align 4
  %"58" = call float @llvm.fma.f32(float %"59", float %"60", float %"61")
  store float %"58", ptr addrspace(5) %"43", align 4
  %"62" = load i64, ptr addrspace(5) %"42", align 8
  %"63" = load float, ptr addrspace(5) %"43", align 4
  %"67" = inttoptr i64 %"62" to ptr
  store float %"63", ptr %"67", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.fma.f32(float, float, float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }