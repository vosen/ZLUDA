define amdgpu_kernel void @min_f16(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca half, align 2, addrspace(5)
  %"44" = alloca half, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %2, ptr addrspace(5) %"41", align 8
  %3 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %3, ptr addrspace(5) %"42", align 8
  %4 = load i64, ptr addrspace(5) %"41", align 8
  %"57" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"57", align 2
  %"47" = bitcast i16 %5 to half
  store half %"47", ptr addrspace(5) %"43", align 2
  %6 = load i64, ptr addrspace(5) %"41", align 8
  %"58" = inttoptr i64 %6 to ptr
  %"37" = getelementptr inbounds i8, ptr %"58", i64 2
  %7 = load i16, ptr %"37", align 2
  %"50" = bitcast i16 %7 to half
  store half %"50", ptr addrspace(5) %"44", align 2
  %8 = load half, ptr addrspace(5) %"43", align 2
  %9 = load half, ptr addrspace(5) %"44", align 2
  %10 = call half @llvm.minnum.f16(half %8, half %9)
  store half %10, ptr addrspace(5) %"43", align 2
  %11 = load i64, ptr addrspace(5) %"42", align 8
  %12 = load half, ptr addrspace(5) %"43", align 2
  %"60" = inttoptr i64 %11 to ptr
  %"61" = bitcast half %12 to i16
  store i16 %"61", ptr %"60", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare half @llvm.minnum.f16(half, half) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }