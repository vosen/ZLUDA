define amdgpu_kernel void @rsqrt(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca double, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %2, ptr addrspace(5) %"41", align 8
  %3 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %3, ptr addrspace(5) %"42", align 8
  %4 = load i64, ptr addrspace(5) %"41", align 8
  %"52" = inttoptr i64 %4 to ptr
  %5 = load double, ptr %"52", align 8
  store double %5, ptr addrspace(5) %"43", align 8
  %6 = load double, ptr addrspace(5) %"43", align 8
  %"48" = call double @llvm.amdgcn.rsq.f64(double %6)
  store double %"48", ptr addrspace(5) %"43", align 8
  %7 = load i64, ptr addrspace(5) %"42", align 8
  %8 = load double, ptr addrspace(5) %"43", align 8
  %"53" = inttoptr i64 %7 to ptr
  store double %8, ptr %"53", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.amdgcn.rsq.f64(double) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
