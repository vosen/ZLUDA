define amdgpu_kernel void @fmax(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca half, align 2, addrspace(5)
  %"43" = alloca half, align 2, addrspace(5)
  %"44" = alloca half, align 2, addrspace(5)
  %"45" = alloca half, align 2, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  store i64 2, ptr addrspace(5) %"50", align 4
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"46", ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"47", ptr addrspace(5) %"41", align 8
  %"49" = load i64, ptr addrspace(5) %"40", align 8
  %"60" = inttoptr i64 %"49" to ptr
  %"59" = load i16, ptr %"60", align 2
  %"48" = bitcast i16 %"59" to half
  store half %"48", ptr addrspace(5) %"42", align 2
  %"51" = load i64, ptr addrspace(5) %"40", align 8
  %"52" = load i64, ptr addrspace(5) %"50", align 8
  %"61" = inttoptr i64 %"51" to ptr
  %"36" = getelementptr inbounds i8, ptr %"61", i64 %"52"
  %"62" = load i16, ptr %"36", align 2
  %"53" = bitcast i16 %"62" to half
  store half %"53", ptr addrspace(5) %"43", align 2
  %"55" = load half, ptr addrspace(5) %"43", align 2
  %"56" = load half, ptr addrspace(5) %"42", align 2
  %"54" = call half @llvm.maxnum.f16(half %"55", half %"56")
  store half %"54", ptr addrspace(5) %"44", align 2
  %"57" = load i64, ptr addrspace(5) %"41", align 8
  %"58" = load half, ptr addrspace(5) %"44", align 2
  %"63" = inttoptr i64 %"57" to ptr
  %"64" = bitcast half %"58" to i16
  store i16 %"64", ptr %"63", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare half @llvm.maxnum.f16(half, half) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }