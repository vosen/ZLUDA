@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 4

define amdgpu_kernel void @cvt_rzi(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"46", ptr addrspace(5) %"42", align 8
  %"47" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"47", ptr addrspace(5) %"43", align 8
  %"49" = load i64, ptr addrspace(5) %"42", align 8
  %"60" = inttoptr i64 %"49" to ptr
  %"48" = load float, ptr %"60", align 4
  store float %"48", ptr addrspace(5) %"44", align 4
  %"34" = load i64, ptr addrspace(4) @0, align 8
  %"50" = load i64, ptr addrspace(5) %"42", align 8
  %"61" = inttoptr i64 %"50" to ptr
  %"35" = getelementptr inbounds i8, ptr %"61", i64 %"34"
  %"51" = load float, ptr %"35", align 4
  store float %"51", ptr addrspace(5) %"45", align 4
  %"53" = load float, ptr addrspace(5) %"44", align 4
  %2 = call float @llvm.trunc.f32(float %"53")
  store float %2, ptr addrspace(5) %"44", align 4
  %"55" = load float, ptr addrspace(5) %"45", align 4
  %3 = call float @llvm.trunc.f32(float %"55")
  store float %3, ptr addrspace(5) %"45", align 4
  %"56" = load i64, ptr addrspace(5) %"43", align 8
  %"57" = load float, ptr addrspace(5) %"44", align 4
  %"62" = inttoptr i64 %"56" to ptr
  store float %"57", ptr %"62", align 4
  %"37" = load i64, ptr addrspace(4) @1, align 8
  %"58" = load i64, ptr addrspace(5) %"43", align 8
  %"63" = inttoptr i64 %"58" to ptr
  %"38" = getelementptr inbounds i8, ptr %"63", i64 %"37"
  %"59" = load float, ptr addrspace(5) %"45", align 4
  store float %"59", ptr %"38", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.trunc.f32(float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }