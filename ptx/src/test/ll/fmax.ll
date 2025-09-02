@0 = addrspace(4) global i64 2

define amdgpu_kernel void @fmax(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca half, align 2, addrspace(5)
  %"44" = alloca half, align 2, addrspace(5)
  %"45" = alloca half, align 2, addrspace(5)
  %"46" = alloca half, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"47", ptr addrspace(5) %"41", align 8
  %"48" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"48", ptr addrspace(5) %"42", align 8
  %"50" = load i64, ptr addrspace(5) %"41", align 8
  %"59" = inttoptr i64 %"50" to ptr
  %"58" = load i16, ptr %"59", align 2
  %"49" = bitcast i16 %"58" to half
  store half %"49", ptr addrspace(5) %"43", align 2
  %"36" = load i64, ptr addrspace(4) @0, align 8
  %"51" = load i64, ptr addrspace(5) %"41", align 8
  %"60" = inttoptr i64 %"51" to ptr
  %"37" = getelementptr inbounds i8, ptr %"60", i64 %"36"
  %"61" = load i16, ptr %"37", align 2
  %"52" = bitcast i16 %"61" to half
  store half %"52", ptr addrspace(5) %"44", align 2
  %"54" = load half, ptr addrspace(5) %"44", align 2
  %"55" = load half, ptr addrspace(5) %"43", align 2
  %"53" = call half @llvm.maxnum.f16(half %"54", half %"55")
  store half %"53", ptr addrspace(5) %"45", align 2
  %"56" = load i64, ptr addrspace(5) %"42", align 8
  %"57" = load half, ptr addrspace(5) %"45", align 2
  %"62" = inttoptr i64 %"56" to ptr
  %"63" = bitcast half %"57" to i16
  store i16 %"63", ptr %"62", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare half @llvm.maxnum.f16(half, half) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }