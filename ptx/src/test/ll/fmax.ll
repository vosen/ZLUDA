define amdgpu_kernel void @fmax(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca half, align 2, addrspace(5)
  %"40" = alloca half, align 2, addrspace(5)
  %"41" = alloca half, align 2, addrspace(5)
  %"42" = alloca half, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"43", ptr addrspace(5) %"37", align 4
  %"44" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"44", ptr addrspace(5) %"38", align 4
  %"46" = load i64, ptr addrspace(5) %"37", align 4
  %"55" = inttoptr i64 %"46" to ptr
  %"54" = load i16, ptr %"55", align 2
  %"45" = bitcast i16 %"54" to half
  store half %"45", ptr addrspace(5) %"39", align 2
  %"47" = load i64, ptr addrspace(5) %"37", align 4
  %"56" = inttoptr i64 %"47" to ptr
  %"33" = getelementptr inbounds i8, ptr %"56", i64 2
  %"57" = load i16, ptr %"33", align 2
  %"48" = bitcast i16 %"57" to half
  store half %"48", ptr addrspace(5) %"40", align 2
  %"50" = load half, ptr addrspace(5) %"40", align 2
  %"51" = load half, ptr addrspace(5) %"39", align 2
  %"49" = call half @llvm.maxnum.f16(half %"50", half %"51")
  store half %"49", ptr addrspace(5) %"41", align 2
  %"52" = load i64, ptr addrspace(5) %"38", align 4
  %"53" = load half, ptr addrspace(5) %"41", align 2
  %"58" = inttoptr i64 %"52" to ptr
  %"59" = bitcast half %"53" to i16
  store i16 %"59", ptr %"58", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare half @llvm.maxnum.f16(half, half) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }