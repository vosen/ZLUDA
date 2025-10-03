define amdgpu_kernel void @constant_f32(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"40" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(5) %"36", align 8
  %"47" = inttoptr i64 %"42" to ptr
  %"41" = load float, ptr %"47", align 4
  store float %"41", ptr addrspace(5) %"38", align 4
  %"44" = load float, ptr addrspace(5) %"38", align 4
  %"43" = fmul float %"44", 5.000000e-01
  store float %"43", ptr addrspace(5) %"38", align 4
  %"45" = load i64, ptr addrspace(5) %"37", align 8
  %"46" = load float, ptr addrspace(5) %"38", align 4
  %"48" = inttoptr i64 %"45" to ptr
  store float %"46", ptr %"48", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
