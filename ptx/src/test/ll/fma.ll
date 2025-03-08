declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @fma(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  %"46" = alloca float, align 4, addrspace(5)
  %"47" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"66"

"66":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"48", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"49", ptr addrspace(5) %"44", align 4
  %"51" = load i64, ptr addrspace(5) %"43", align 4
  %"62" = inttoptr i64 %"51" to ptr
  %"50" = load float, ptr %"62", align 4
  store float %"50", ptr addrspace(5) %"45", align 4
  %"52" = load i64, ptr addrspace(5) %"43", align 4
  %"63" = inttoptr i64 %"52" to ptr
  %"32" = getelementptr inbounds i8, ptr %"63", i64 4
  %"53" = load float, ptr %"32", align 4
  store float %"53", ptr addrspace(5) %"46", align 4
  %"54" = load i64, ptr addrspace(5) %"43", align 4
  %"64" = inttoptr i64 %"54" to ptr
  %"34" = getelementptr inbounds i8, ptr %"64", i64 8
  %"55" = load float, ptr %"34", align 4
  store float %"55", ptr addrspace(5) %"47", align 4
  %"57" = load float, ptr addrspace(5) %"45", align 4
  %"58" = load float, ptr addrspace(5) %"46", align 4
  %"59" = load float, ptr addrspace(5) %"47", align 4
  %"56" = call float @llvm.fma.f32(float %"57", float %"58", float %"59")
  store float %"56", ptr addrspace(5) %"45", align 4
  %"60" = load i64, ptr addrspace(5) %"44", align 4
  %"61" = load float, ptr addrspace(5) %"45", align 4
  %"65" = inttoptr i64 %"60" to ptr
  store float %"61", ptr %"65", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.fma.f32(float, float, float) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }