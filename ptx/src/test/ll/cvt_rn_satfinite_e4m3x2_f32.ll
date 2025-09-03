declare hidden i16 @__zluda_ptx_impl_cvt_rn_satfinite_e4m3x2_f32(float, float) #0

define amdgpu_kernel void @cvt_rn_satfinite_e4m3x2_f32(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #1 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca float, align 4, addrspace(5)
  %"42" = alloca float, align 4, addrspace(5)
  %"43" = alloca i16, align 2, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"48", align 4
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"44", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"45", ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"57" = inttoptr i64 %"47" to ptr
  %"46" = load float, ptr %"57", align 4
  store float %"46", ptr addrspace(5) %"41", align 4
  %"49" = load i64, ptr addrspace(5) %"39", align 8
  %"50" = load i64, ptr addrspace(5) %"48", align 8
  %"58" = inttoptr i64 %"49" to ptr
  %"35" = getelementptr inbounds i8, ptr %"58", i64 %"50"
  %"51" = load float, ptr %"35", align 4
  store float %"51", ptr addrspace(5) %"42", align 4
  %"53" = load float, ptr addrspace(5) %"41", align 4
  %"54" = load float, ptr addrspace(5) %"42", align 4
  %"59" = call i16 @__zluda_ptx_impl_cvt_rn_satfinite_e4m3x2_f32(float %"53", float %"54")
  store i16 %"59", ptr addrspace(5) %"43", align 2
  %"55" = load i64, ptr addrspace(5) %"40", align 8
  %"56" = load i16, ptr addrspace(5) %"43", align 2
  %"60" = inttoptr i64 %"55" to ptr
  store i16 %"56", ptr %"60", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }