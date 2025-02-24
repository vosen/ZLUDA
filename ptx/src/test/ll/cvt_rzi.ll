declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvt_rzi(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"64"

"64":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"47" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"42", align 4
  %"60" = inttoptr i64 %"49" to ptr
  %"48" = load float, ptr %"60", align 4
  store float %"48", ptr addrspace(5) %"44", align 4
  %"50" = load i64, ptr addrspace(5) %"42", align 4
  %"61" = inttoptr i64 %"50" to ptr
  %"31" = getelementptr inbounds i8, ptr %"61", i64 4
  %"51" = load float, ptr %"31", align 4
  store float %"51", ptr addrspace(5) %"45", align 4
  %"53" = load float, ptr addrspace(5) %"44", align 4
  %2 = call float @llvm.trunc.f32(float %"53")
  %"52" = freeze float %2
  store float %"52", ptr addrspace(5) %"44", align 4
  %"55" = load float, ptr addrspace(5) %"45", align 4
  %3 = call float @llvm.trunc.f32(float %"55")
  %"54" = freeze float %3
  store float %"54", ptr addrspace(5) %"45", align 4
  %"56" = load i64, ptr addrspace(5) %"43", align 4
  %"57" = load float, ptr addrspace(5) %"44", align 4
  %"62" = inttoptr i64 %"56" to ptr
  store float %"57", ptr %"62", align 4
  %"58" = load i64, ptr addrspace(5) %"43", align 4
  %"63" = inttoptr i64 %"58" to ptr
  %"33" = getelementptr inbounds i8, ptr %"63", i64 4
  %"59" = load float, ptr addrspace(5) %"45", align 4
  store float %"59", ptr %"33", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.trunc.f32(float) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }