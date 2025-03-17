define amdgpu_kernel void @rsqrt(ptr addrspace(4) byref(i64) %"30", ptr addrspace(4) byref(i64) %"31") #0 {
  %"32" = alloca i64, align 8, addrspace(5)
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca double, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"29"

"29":                                             ; preds = %1
  %"35" = load i64, ptr addrspace(4) %"30", align 4
  store i64 %"35", ptr addrspace(5) %"32", align 4
  %"36" = load i64, ptr addrspace(4) %"31", align 4
  store i64 %"36", ptr addrspace(5) %"33", align 4
  %"38" = load i64, ptr addrspace(5) %"32", align 4
  %"43" = inttoptr i64 %"38" to ptr
  %"37" = load double, ptr %"43", align 8
  store double %"37", ptr addrspace(5) %"34", align 8
  %"40" = load double, ptr addrspace(5) %"34", align 8
  %"39" = call double @llvm.amdgcn.rsq.f64(double %"40")
  store double %"39", ptr addrspace(5) %"34", align 8
  %"41" = load i64, ptr addrspace(5) %"33", align 4
  %"42" = load double, ptr addrspace(5) %"34", align 8
  %"44" = inttoptr i64 %"41" to ptr
  store double %"42", ptr %"44", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.amdgcn.rsq.f64(double) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }