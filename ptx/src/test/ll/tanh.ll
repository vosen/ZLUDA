define amdgpu_kernel void @tanh(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"38" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"38", ptr addrspace(5) %"35", align 8
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"41" = load i64, ptr addrspace(5) %"35", align 8
  %"46" = inttoptr i64 %"41" to ptr
  %"40" = load float, ptr %"46", align 4
  store float %"40", ptr addrspace(5) %"37", align 4
  %"43" = load float, ptr addrspace(5) %"37", align 4
  %"42" = call afn float @__ocml_tanh_f32(float %"43")
  store float %"42", ptr addrspace(5) %"37", align 4
  %"44" = load i64, ptr addrspace(5) %"36", align 8
  %"45" = load float, ptr addrspace(5) %"37", align 4
  %"47" = inttoptr i64 %"44" to ptr
  store float %"45", ptr %"47", align 4
  ret void
}

declare float @__ocml_tanh_f32(float)

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }