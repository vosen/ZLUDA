declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @fma(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  %"46" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"47" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"47", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"48", ptr addrspace(5) %"43", align 4
  %"50" = load i64, ptr addrspace(5) %"42", align 4
  %"61" = inttoptr i64 %"50" to ptr
  %"49" = load float, ptr %"61", align 4
  store float %"49", ptr addrspace(5) %"44", align 4
  %"51" = load i64, ptr addrspace(5) %"42", align 4
  %"62" = inttoptr i64 %"51" to ptr
  %"31" = getelementptr inbounds i8, ptr %"62", i64 4
  %"52" = load float, ptr %"31", align 4
  store float %"52", ptr addrspace(5) %"45", align 4
  %"53" = load i64, ptr addrspace(5) %"42", align 4
  %"63" = inttoptr i64 %"53" to ptr
  %"33" = getelementptr inbounds i8, ptr %"63", i64 8
  %"54" = load float, ptr %"33", align 4
  store float %"54", ptr addrspace(5) %"46", align 4
  %"56" = load float, ptr addrspace(5) %"44", align 4
  %"57" = load float, ptr addrspace(5) %"45", align 4
  %"58" = load float, ptr addrspace(5) %"46", align 4
  %"55" = call float @llvm.fma.f32(float %"56", float %"57", float %"58")
  store float %"55", ptr addrspace(5) %"44", align 4
  %"59" = load i64, ptr addrspace(5) %"43", align 4
  %"60" = load float, ptr addrspace(5) %"44", align 4
  %"64" = inttoptr i64 %"59" to ptr
  store float %"60", ptr %"64", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.fma.f32(float, float, float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
