declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @add_ftz(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #1 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca float, align 4, addrspace(5)
  %"47" = alloca float, align 4, addrspace(5)
  %"48" = alloca float, align 4, addrspace(5)
  %"49" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"70"

"70":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"50", ptr addrspace(5) %"44", align 4
  %"51" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"51", ptr addrspace(5) %"45", align 4
  %"53" = load i64, ptr addrspace(5) %"44", align 4
  %"66" = inttoptr i64 %"53" to ptr
  %"52" = load float, ptr %"66", align 4
  store float %"52", ptr addrspace(5) %"46", align 4
  %"54" = load i64, ptr addrspace(5) %"44", align 4
  %"67" = inttoptr i64 %"54" to ptr
  %"33" = getelementptr inbounds i8, ptr %"67", i64 4
  %"55" = load float, ptr %"33", align 4
  store float %"55", ptr addrspace(5) %"47", align 4
  %"57" = load float, ptr addrspace(5) %"46", align 4
  %"58" = load float, ptr addrspace(5) %"47", align 4
  %"56" = fadd float %"57", %"58"
  store float %"56", ptr addrspace(5) %"48", align 4
  %"60" = load float, ptr addrspace(5) %"46", align 4
  %"61" = load float, ptr addrspace(5) %"47", align 4
  call void @llvm.amdgcn.s.setreg(i32 2305, i32 3)
  %"59" = fadd float %"60", %"61"
  store float %"59", ptr addrspace(5) %"49", align 4
  %"62" = load i64, ptr addrspace(5) %"45", align 4
  %"63" = load float, ptr addrspace(5) %"48", align 4
  %"68" = inttoptr i64 %"62" to ptr
  store float %"63", ptr %"68", align 4
  %"64" = load i64, ptr addrspace(5) %"45", align 4
  %"69" = inttoptr i64 %"64" to ptr
  %"35" = getelementptr inbounds i8, ptr %"69", i64 4
  %"65" = load float, ptr addrspace(5) %"49", align 4
  store float %"65", ptr %"35", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }