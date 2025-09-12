declare hidden i16 @__zluda_ptx_impl_cvt_rn_satfinite_e4m3x2_f32(float, float) #0

define amdgpu_kernel void @cvt_rn_satfinite_e4m3x2_f32(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #1 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca float, align 4, addrspace(5)
  %"42" = alloca float, align 4, addrspace(5)
  %"43" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"44", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"45", ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"55" = inttoptr i64 %"47" to ptr
  %"46" = load float, ptr %"55", align 4
  store float %"46", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"39", align 8
  %"56" = inttoptr i64 %"48" to ptr
  %"35" = getelementptr inbounds i8, ptr %"56", i64 4
  %"49" = load float, ptr %"35", align 4
  store float %"49", ptr addrspace(5) %"42", align 4
  %"51" = load float, ptr addrspace(5) %"41", align 4
  %"52" = load float, ptr addrspace(5) %"42", align 4
  %"57" = call i16 @__zluda_ptx_impl_cvt_rn_satfinite_e4m3x2_f32(float %"51", float %"52")
  store i16 %"57", ptr addrspace(5) %"43", align 2
  %"53" = load i64, ptr addrspace(5) %"40", align 8
  %"54" = load i16, ptr addrspace(5) %"43", align 2
  %"58" = inttoptr i64 %"53" to ptr
  store i16 %"54", ptr %"58", align 2
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }