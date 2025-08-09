declare float @__zluda_ptx_impl_div_rn_f32(float, float) #0

define amdgpu_kernel void @div_noftz(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #1 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca float, align 4, addrspace(5)
  %"41" = alloca float, align 4, addrspace(5)
  %"42" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"43", ptr addrspace(5) %"38", align 8
  %"44" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"44", ptr addrspace(5) %"39", align 8
  %"46" = load i64, ptr addrspace(5) %"38", align 8
  %"59" = inttoptr i64 %"46" to ptr
  %"45" = load float, ptr %"59", align 4
  store float %"45", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(5) %"38", align 8
  %"60" = inttoptr i64 %"47" to ptr
  %"32" = getelementptr inbounds i8, ptr %"60", i64 4
  %"48" = load float, ptr %"32", align 4
  store float %"48", ptr addrspace(5) %"41", align 4
  %"50" = load float, ptr addrspace(5) %"40", align 4
  %"51" = load float, ptr addrspace(5) %"41", align 4
  %"49" = fmul float %"50", %"51"
  store float %"49", ptr addrspace(5) %"42", align 4
  call void @llvm.amdgcn.s.setreg(i32 6401, i32 3)
  %"53" = load float, ptr addrspace(5) %"40", align 4
  %"54" = load float, ptr addrspace(5) %"41", align 4
  %"52" = call float @__zluda_ptx_impl_div_rn_f32(float %"53", float %"54")
  store float %"52", ptr addrspace(5) %"40", align 4
  %"55" = load i64, ptr addrspace(5) %"39", align 8
  %"56" = load float, ptr addrspace(5) %"40", align 4
  %"61" = inttoptr i64 %"55" to ptr
  store float %"56", ptr %"61", align 4
  %"57" = load i64, ptr addrspace(5) %"39", align 8
  %"62" = inttoptr i64 %"57" to ptr
  %"34" = getelementptr inbounds i8, ptr %"62", i64 4
  %"58" = load float, ptr addrspace(5) %"42", align 4
  store float %"58", ptr %"34", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }