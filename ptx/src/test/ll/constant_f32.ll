define amdgpu_kernel void @constant_f32(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #0 {
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"36" = load i64, ptr addrspace(4) %"31", align 8
  store i64 %"36", ptr addrspace(5) %"33", align 8
  %"37" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"37", ptr addrspace(5) %"34", align 8
  %"39" = load i64, ptr addrspace(5) %"33", align 8
  %"44" = inttoptr i64 %"39" to ptr
  %"38" = load float, ptr %"44", align 4
  store float %"38", ptr addrspace(5) %"35", align 4
  %"41" = load float, ptr addrspace(5) %"35", align 4
  %"40" = fmul float %"41", 5.000000e-01
  store float %"40", ptr addrspace(5) %"35", align 4
  %"42" = load i64, ptr addrspace(5) %"34", align 8
  %"43" = load float, ptr addrspace(5) %"35", align 4
  %"45" = inttoptr i64 %"42" to ptr
  store float %"43", ptr %"45", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }