define amdgpu_kernel void @fma(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca float, align 4, addrspace(5)
  %"41" = alloca float, align 4, addrspace(5)
  %"42" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"43", ptr addrspace(5) %"38", align 4
  %"44" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"44", ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(5) %"38", align 4
  %"57" = inttoptr i64 %"46" to ptr
  %"45" = load float, ptr %"57", align 4
  store float %"45", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(5) %"38", align 4
  %"58" = inttoptr i64 %"47" to ptr
  %"32" = getelementptr inbounds i8, ptr %"58", i64 4
  %"48" = load float, ptr %"32", align 4
  store float %"48", ptr addrspace(5) %"41", align 4
  %"49" = load i64, ptr addrspace(5) %"38", align 4
  %"59" = inttoptr i64 %"49" to ptr
  %"34" = getelementptr inbounds i8, ptr %"59", i64 8
  %"50" = load float, ptr %"34", align 4
  store float %"50", ptr addrspace(5) %"42", align 4
  %"52" = load float, ptr addrspace(5) %"40", align 4
  %"53" = load float, ptr addrspace(5) %"41", align 4
  %"54" = load float, ptr addrspace(5) %"42", align 4
  %"51" = call float @llvm.fma.f32(float %"52", float %"53", float %"54")
  store float %"51", ptr addrspace(5) %"40", align 4
  %"55" = load i64, ptr addrspace(5) %"39", align 4
  %"56" = load float, ptr addrspace(5) %"40", align 4
  %"60" = inttoptr i64 %"55" to ptr
  store float %"56", ptr %"60", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.fma.f32(float, float, float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }