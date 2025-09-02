define amdgpu_kernel void @cvt_rni_u16_f32(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca float, align 4, addrspace(5)
  %"39" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"40", ptr addrspace(5) %"36", align 8
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"37", align 8
  %"43" = load i64, ptr addrspace(5) %"36", align 8
  %"48" = inttoptr i64 %"43" to ptr addrspace(1)
  %"42" = load float, ptr addrspace(1) %"48", align 4
  store float %"42", ptr addrspace(5) %"38", align 4
  %"45" = load float, ptr addrspace(5) %"38", align 4
  %2 = call float @llvm.roundeven.f32(float %"45")
  %"44" = call i16 @llvm.fptoui.sat.i16.f32(float %2)
  store i16 %"44", ptr addrspace(5) %"39", align 2
  %"46" = load i64, ptr addrspace(5) %"37", align 8
  %"47" = load i16, ptr addrspace(5) %"39", align 2
  %"49" = inttoptr i64 %"46" to ptr
  store i16 %"47", ptr %"49", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.roundeven.f32(float) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i16 @llvm.fptoui.sat.i16.f32(float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }