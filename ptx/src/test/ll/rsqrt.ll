declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @rsqrt(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #1 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca double, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"40", ptr addrspace(5) %"37", align 4
  %"41" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"41", ptr addrspace(5) %"38", align 4
  %"43" = load i64, ptr addrspace(5) %"37", align 4
  %"48" = inttoptr i64 %"43" to ptr
  %"42" = load double, ptr %"48", align 8
  store double %"42", ptr addrspace(5) %"39", align 8
  %"45" = load double, ptr addrspace(5) %"39", align 8
  call void @llvm.amdgcn.s.setreg(i32 2433, i32 3)
  %"44" = call double @llvm.amdgcn.rsq.f64(double %"45")
  store double %"44", ptr addrspace(5) %"39", align 8
  %"46" = load i64, ptr addrspace(5) %"38", align 4
  %"47" = load double, ptr addrspace(5) %"39", align 8
  %"49" = inttoptr i64 %"46" to ptr
  store double %"47", ptr %"49", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.amdgcn.rsq.f64(double) #3

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }