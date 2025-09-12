@foobar = addrspace(1) global [4 x float] [float 1.000000e+00, float 0.000000e+00, float 0.000000e+00, float 0.000000e+00]

define amdgpu_kernel void @global_array_f32(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  store i64 ptrtoint (ptr addrspace(1) @foobar to i64), ptr addrspace(5) %"38", align 8
  %"42" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"42", ptr addrspace(5) %"39", align 8
  %"43" = load i64, ptr addrspace(5) %"38", align 8
  %"48" = inttoptr i64 %"43" to ptr addrspace(1)
  %"34" = getelementptr inbounds i8, ptr addrspace(1) %"48", i64 4
  %"44" = load float, ptr addrspace(1) %"34", align 4
  store float %"44", ptr addrspace(5) %"40", align 4
  %"45" = load i64, ptr addrspace(5) %"39", align 8
  %"46" = load float, ptr addrspace(5) %"40", align 4
  %"49" = inttoptr i64 %"45" to ptr
  store float %"46", ptr %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }