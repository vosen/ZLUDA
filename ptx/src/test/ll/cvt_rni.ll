declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvt_rni(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"41", align 4
  %"59" = inttoptr i64 %"48" to ptr
  %"47" = load float, ptr %"59", align 4
  store float %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"41", align 4
  %"60" = inttoptr i64 %"49" to ptr
  %"30" = getelementptr inbounds i8, ptr %"60", i64 4
  %"50" = load float, ptr %"30", align 4
  store float %"50", ptr addrspace(5) %"44", align 4
  %"52" = load float, ptr addrspace(5) %"43", align 4
  %2 = call float @llvm.roundeven.f32(float %"52")
  %"51" = freeze float %2
  store float %"51", ptr addrspace(5) %"43", align 4
  %"54" = load float, ptr addrspace(5) %"44", align 4
  %3 = call float @llvm.roundeven.f32(float %"54")
  %"53" = freeze float %3
  store float %"53", ptr addrspace(5) %"44", align 4
  %"55" = load i64, ptr addrspace(5) %"42", align 4
  %"56" = load float, ptr addrspace(5) %"43", align 4
  %"61" = inttoptr i64 %"55" to ptr
  store float %"56", ptr %"61", align 4
  %"57" = load i64, ptr addrspace(5) %"42", align 4
  %"62" = inttoptr i64 %"57" to ptr
  %"32" = getelementptr inbounds i8, ptr %"62", i64 4
  %"58" = load float, ptr addrspace(5) %"44", align 4
  store float %"58", ptr %"32", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.roundeven.f32(float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
