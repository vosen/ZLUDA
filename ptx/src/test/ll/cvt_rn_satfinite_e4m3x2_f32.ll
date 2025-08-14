declare i16 @__zluda_ptx_impl_cvt_f32_to_e4m3_satfinite(float, float) #0

define amdgpu_kernel void @cvt_rn_satfinite_e4m3x2_f32(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #1 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca float, align 4, addrspace(5)
  %"39" = alloca float, align 4, addrspace(5)
  %"40" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"41", ptr addrspace(5) %"36", align 8
  %"42" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"42", ptr addrspace(5) %"37", align 8
  %"44" = load i64, ptr addrspace(5) %"36", align 8
  %"52" = inttoptr i64 %"44" to ptr
  %"43" = load float, ptr %"52", align 4
  store float %"43", ptr addrspace(5) %"38", align 4
  %"45" = load i64, ptr addrspace(5) %"36", align 8
  %"53" = inttoptr i64 %"45" to ptr
  %"32" = getelementptr inbounds i8, ptr %"53", i64 4
  %"46" = load float, ptr %"32", align 4
  store float %"46", ptr addrspace(5) %"39", align 4
  %"48" = load float, ptr addrspace(5) %"38", align 4
  %"49" = load float, ptr addrspace(5) %"39", align 4
  %"54" = call i16 @__zluda_ptx_impl_cvt_f32_to_e4m3_satfinite(float %"48", float %"49")
  store i16 %"54", ptr addrspace(5) %"40", align 2
  %"50" = load i64, ptr addrspace(5) %"37", align 8
  %"51" = load i16, ptr addrspace(5) %"40", align 2
  %"55" = inttoptr i64 %"50" to ptr
  store i16 %"51", ptr %"55", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
