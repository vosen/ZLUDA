@0 = addrspace(4) global float 5.000000e-01

define amdgpu_kernel void @constant_f32(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"41" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"41", ptr addrspace(5) %"38", align 8
  %"43" = load i64, ptr addrspace(5) %"37", align 8
  %"48" = inttoptr i64 %"43" to ptr
  %"42" = load float, ptr %"48", align 4
  store float %"42", ptr addrspace(5) %"39", align 4
  %"33" = load float, ptr addrspace(4) @0, align 4
  %"45" = load float, ptr addrspace(5) %"39", align 4
  %"44" = fmul float %"45", %"33"
  store float %"44", ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(5) %"38", align 8
  %"47" = load float, ptr addrspace(5) %"39", align 4
  %"49" = inttoptr i64 %"46" to ptr
  store float %"47", ptr %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }