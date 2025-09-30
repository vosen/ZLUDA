define amdgpu_kernel void @cvt_s32_f32(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(5) %"40", align 8
  %"59" = inttoptr i64 %"47" to ptr
  %"58" = load float, ptr %"59", align 4
  %"46" = bitcast float %"58" to i32
  store i32 %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 8
  %"60" = inttoptr i64 %"48" to ptr
  %"34" = getelementptr inbounds i8, ptr %"60", i64 4
  %"61" = load float, ptr %"34", align 4
  %"49" = bitcast float %"61" to i32
  store i32 %"49", ptr addrspace(5) %"43", align 4
  %"51" = load i32, ptr addrspace(5) %"42", align 4
  %"63" = bitcast i32 %"51" to float
  %2 = call float @llvm.ceil.f32(float %"63")
  %3 = fptosi float %2 to i32
  %"62" = freeze i32 %3
  store i32 %"62", ptr addrspace(5) %"42", align 4
  %"53" = load i32, ptr addrspace(5) %"43", align 4
  %"65" = bitcast i32 %"53" to float
  %4 = call float @llvm.ceil.f32(float %"65")
  %5 = fptosi float %4 to i32
  %"64" = freeze i32 %5
  store i32 %"64", ptr addrspace(5) %"43", align 4
  %"54" = load i64, ptr addrspace(5) %"41", align 8
  %"55" = load i32, ptr addrspace(5) %"42", align 4
  %"66" = inttoptr i64 %"54" to ptr addrspace(1)
  store i32 %"55", ptr addrspace(1) %"66", align 4
  %"56" = load i64, ptr addrspace(5) %"41", align 8
  %"68" = inttoptr i64 %"56" to ptr addrspace(1)
  %"36" = getelementptr inbounds i8, ptr addrspace(1) %"68", i64 4
  %"57" = load i32, ptr addrspace(5) %"43", align 4
  store i32 %"57", ptr addrspace(1) %"36", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.ceil.f32(float) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
