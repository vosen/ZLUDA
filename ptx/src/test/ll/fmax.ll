define amdgpu_kernel void @fmax(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #0 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca half, align 2, addrspace(5)
  %"49" = alloca half, align 2, addrspace(5)
  %"50" = alloca half, align 2, addrspace(5)
  %"51" = alloca half, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %2, ptr addrspace(5) %"46", align 8
  %3 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %3, ptr addrspace(5) %"47", align 8
  %4 = load i64, ptr addrspace(5) %"46", align 8
  %"64" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"64", align 2
  %"54" = bitcast i16 %5 to half
  store half %"54", ptr addrspace(5) %"48", align 2
  %6 = load i64, ptr addrspace(5) %"46", align 8
  %"65" = inttoptr i64 %6 to ptr
  %"42" = getelementptr inbounds i8, ptr %"65", i64 2
  %7 = load i16, ptr %"42", align 2
  %"57" = bitcast i16 %7 to half
  store half %"57", ptr addrspace(5) %"49", align 2
  %8 = load half, ptr addrspace(5) %"49", align 2
  %9 = load half, ptr addrspace(5) %"48", align 2
  %10 = call half @llvm.maxnum.f16(half %8, half %9)
  store half %10, ptr addrspace(5) %"50", align 2
  %11 = load i64, ptr addrspace(5) %"47", align 8
  %12 = load half, ptr addrspace(5) %"50", align 2
  %"67" = inttoptr i64 %11 to ptr
  %"68" = bitcast half %12 to i16
  store i16 %"68", ptr %"67", align 2
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare half @llvm.maxnum.f16(half, half) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
