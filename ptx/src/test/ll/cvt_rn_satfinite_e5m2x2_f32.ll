declare hidden i16 @__zluda_ptx_impl_cvt_rn_satfinite_e5m2x2_f32(float, float) #0

define amdgpu_kernel void @cvt_rn_satfinite_e5m2x2_f32(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  %"46" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"58" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"58", align 4
  store float %5, ptr addrspace(5) %"44", align 4
  %6 = load i64, ptr addrspace(5) %"42", align 8
  %"59" = inttoptr i64 %6 to ptr
  %"38" = getelementptr inbounds i8, ptr %"59", i64 4
  %7 = load float, ptr %"38", align 4
  store float %7, ptr addrspace(5) %"45", align 4
  %8 = load float, ptr addrspace(5) %"44", align 4
  %9 = load float, ptr addrspace(5) %"45", align 4
  %"60" = call i16 @__zluda_ptx_impl_cvt_rn_satfinite_e5m2x2_f32(float %8, float %9)
  store i16 %"60", ptr addrspace(5) %"46", align 2
  %10 = load i64, ptr addrspace(5) %"43", align 8
  %11 = load i16, ptr addrspace(5) %"46", align 2
  %"61" = inttoptr i64 %10 to ptr
  store i16 %11, ptr %"61", align 2
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }