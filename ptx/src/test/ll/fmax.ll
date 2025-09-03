define amdgpu_kernel void @fmax(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca half, align 2, addrspace(5)
  %"43" = alloca half, align 2, addrspace(5)
  %"44" = alloca half, align 2, addrspace(5)
  %"45" = alloca half, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"46", ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"47", ptr addrspace(5) %"41", align 8
  %"49" = load i64, ptr addrspace(5) %"40", align 8
  %"58" = inttoptr i64 %"49" to ptr
  %"57" = load i16, ptr %"58", align 2
  %"48" = bitcast i16 %"57" to half
  store half %"48", ptr addrspace(5) %"42", align 2
  %"50" = load i64, ptr addrspace(5) %"40", align 8
  %"59" = inttoptr i64 %"50" to ptr
  %"36" = getelementptr inbounds i8, ptr %"59", i64 2
  %"60" = load i16, ptr %"36", align 2
  %"51" = bitcast i16 %"60" to half
  store half %"51", ptr addrspace(5) %"43", align 2
  %"53" = load half, ptr addrspace(5) %"43", align 2
  %"54" = load half, ptr addrspace(5) %"42", align 2
  %"52" = call half @llvm.maxnum.f16(half %"53", half %"54")
  store half %"52", ptr addrspace(5) %"44", align 2
  %"55" = load i64, ptr addrspace(5) %"41", align 8
  %"56" = load half, ptr addrspace(5) %"44", align 2
  %"61" = inttoptr i64 %"55" to ptr
  %"62" = bitcast half %"56" to i16
  store i16 %"62", ptr %"61", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare half @llvm.maxnum.f16(half, half) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }