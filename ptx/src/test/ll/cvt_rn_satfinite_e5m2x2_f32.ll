@0 = addrspace(4) global i64 4

declare hidden i16 @__zluda_ptx_impl_cvt_rn_satfinite_e5m2x2_f32(float, float) #0

define amdgpu_kernel void @cvt_rn_satfinite_e5m2x2_f32(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #1 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca float, align 4, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  %"44" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"45", ptr addrspace(5) %"40", align 8
  %"46" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"46", ptr addrspace(5) %"41", align 8
  %"48" = load i64, ptr addrspace(5) %"40", align 8
  %"56" = inttoptr i64 %"48" to ptr
  %"47" = load float, ptr %"56", align 4
  store float %"47", ptr addrspace(5) %"42", align 4
  %"35" = load i64, ptr addrspace(4) @0, align 8
  %"49" = load i64, ptr addrspace(5) %"40", align 8
  %"57" = inttoptr i64 %"49" to ptr
  %"36" = getelementptr inbounds i8, ptr %"57", i64 %"35"
  %"50" = load float, ptr %"36", align 4
  store float %"50", ptr addrspace(5) %"43", align 4
  %"52" = load float, ptr addrspace(5) %"42", align 4
  %"53" = load float, ptr addrspace(5) %"43", align 4
  %"58" = call i16 @__zluda_ptx_impl_cvt_rn_satfinite_e5m2x2_f32(float %"52", float %"53")
  store i16 %"58", ptr addrspace(5) %"44", align 2
  %"54" = load i64, ptr addrspace(5) %"41", align 8
  %"55" = load i16, ptr addrspace(5) %"44", align 2
  %"59" = inttoptr i64 %"54" to ptr
  store i16 %"55", ptr %"59", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }