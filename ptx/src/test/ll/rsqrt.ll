define amdgpu_kernel void @rsqrt(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca double, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"38" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"38", ptr addrspace(5) %"35", align 8
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"41" = load i64, ptr addrspace(5) %"35", align 8
  %"46" = inttoptr i64 %"41" to ptr
  %"40" = load double, ptr %"46", align 8
  store double %"40", ptr addrspace(5) %"37", align 8
  %"43" = load double, ptr addrspace(5) %"37", align 8
  %"42" = call double @llvm.amdgcn.rsq.f64(double %"43")
  store double %"42", ptr addrspace(5) %"37", align 8
  %"44" = load i64, ptr addrspace(5) %"36", align 8
  %"45" = load double, ptr addrspace(5) %"37", align 8
  %"47" = inttoptr i64 %"44" to ptr
  store double %"45", ptr %"47", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.amdgcn.rsq.f64(double) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
