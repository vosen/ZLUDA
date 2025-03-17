define amdgpu_kernel void @cvt_s32_f32(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  %"42" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"42", ptr addrspace(5) %"38", align 4
  %"44" = load i64, ptr addrspace(5) %"37", align 4
  %"56" = inttoptr i64 %"44" to ptr
  %"55" = load float, ptr %"56", align 4
  %"43" = bitcast float %"55" to i32
  store i32 %"43", ptr addrspace(5) %"39", align 4
  %"45" = load i64, ptr addrspace(5) %"37", align 4
  %"57" = inttoptr i64 %"45" to ptr
  %"31" = getelementptr inbounds i8, ptr %"57", i64 4
  %"58" = load float, ptr %"31", align 4
  %"46" = bitcast float %"58" to i32
  store i32 %"46", ptr addrspace(5) %"40", align 4
  %"48" = load i32, ptr addrspace(5) %"39", align 4
  %"60" = bitcast i32 %"48" to float
  %2 = call float @llvm.ceil.f32(float %"60")
  %3 = fptosi float %2 to i32
  %"59" = freeze i32 %3
  store i32 %"59", ptr addrspace(5) %"39", align 4
  %"50" = load i32, ptr addrspace(5) %"40", align 4
  %"62" = bitcast i32 %"50" to float
  %4 = call float @llvm.ceil.f32(float %"62")
  %5 = fptosi float %4 to i32
  %"61" = freeze i32 %5
  store i32 %"61", ptr addrspace(5) %"40", align 4
  %"51" = load i64, ptr addrspace(5) %"38", align 4
  %"52" = load i32, ptr addrspace(5) %"39", align 4
  %"63" = inttoptr i64 %"51" to ptr addrspace(1)
  store i32 %"52", ptr addrspace(1) %"63", align 4
  %"53" = load i64, ptr addrspace(5) %"38", align 4
  %"65" = inttoptr i64 %"53" to ptr addrspace(1)
  %"33" = getelementptr inbounds i8, ptr addrspace(1) %"65", i64 4
  %"54" = load i32, ptr addrspace(5) %"40", align 4
  store i32 %"54", ptr addrspace(1) %"33", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.ceil.f32(float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }