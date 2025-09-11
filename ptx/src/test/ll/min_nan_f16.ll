define amdgpu_kernel void @min_nan_f16(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca half, align 2, addrspace(5)
  %"41" = alloca half, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(5) %"38", align 8
  %"54" = inttoptr i64 %"45" to ptr
  %"53" = load i16, ptr %"54", align 2
  %"44" = bitcast i16 %"53" to half
  store half %"44", ptr addrspace(5) %"40", align 2
  %"46" = load i64, ptr addrspace(5) %"38", align 8
  %"55" = inttoptr i64 %"46" to ptr
  %"34" = getelementptr inbounds i8, ptr %"55", i64 2
  %"56" = load i16, ptr %"34", align 2
  %"47" = bitcast i16 %"56" to half
  store half %"47", ptr addrspace(5) %"41", align 2
  %"49" = load half, ptr addrspace(5) %"40", align 2
  %"50" = load half, ptr addrspace(5) %"41", align 2
  %2 = call half @llvm.minnum.f16(half %"49", half %"50")
  %3 = fcmp uno half %"49", %"50"
  %"48" = select i1 %3, half 0xH7E00, half %2
  store half %"48", ptr addrspace(5) %"40", align 2
  %"51" = load i64, ptr addrspace(5) %"39", align 8
  %"52" = load half, ptr addrspace(5) %"40", align 2
  %"57" = inttoptr i64 %"51" to ptr
  %"58" = bitcast half %"52" to i16
  store i16 %"58", ptr %"57", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare half @llvm.minnum.f16(half, half) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
