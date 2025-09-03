define amdgpu_kernel void @cvt_s32_f32(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
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
  %"63" = inttoptr i64 %"47" to ptr
  %"62" = load float, ptr %"63", align 4
  %"46" = bitcast float %"62" to i32
  store i32 %"46", ptr addrspace(5) %"42", align 4
  %"49" = load i64, ptr addrspace(5) %"40", align 8
  %"50" = load i64, ptr addrspace(5) %"48", align 8
  %"64" = inttoptr i64 %"49" to ptr
  %"34" = getelementptr inbounds i8, ptr %"64", i64 %"50"
  %"65" = load float, ptr %"34", align 4
  %"51" = bitcast float %"65" to i32
  store i32 %"51", ptr addrspace(5) %"43", align 4
  %"53" = load i32, ptr addrspace(5) %"42", align 4
  %"67" = bitcast i32 %"53" to float
  %2 = call float @llvm.ceil.f32(float %"67")
  %3 = fptosi float %2 to i32
  %"66" = freeze i32 %3
  store i32 %"66", ptr addrspace(5) %"42", align 4
  %"55" = load i32, ptr addrspace(5) %"43", align 4
  %"69" = bitcast i32 %"55" to float
  %4 = call float @llvm.ceil.f32(float %"69")
  %5 = fptosi float %4 to i32
  %"68" = freeze i32 %5
  store i32 %"68", ptr addrspace(5) %"43", align 4
  %"56" = load i64, ptr addrspace(5) %"41", align 8
  %"57" = load i32, ptr addrspace(5) %"42", align 4
  %"70" = inttoptr i64 %"56" to ptr addrspace(1)
  store i32 %"57", ptr addrspace(1) %"70", align 4
  %"59" = load i64, ptr addrspace(5) %"41", align 8
  %"60" = load i64, ptr addrspace(5) %"58", align 8
  %"72" = inttoptr i64 %"59" to ptr addrspace(1)
  %"36" = getelementptr inbounds i8, ptr addrspace(1) %"72", i64 %"60"
  %"61" = load i32, ptr addrspace(5) %"43", align 4
  store i32 %"61", ptr addrspace(1) %"36", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.ceil.f32(float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }