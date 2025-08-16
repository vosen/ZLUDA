declare hidden float @__zluda_ptx_impl_sqrt_rn_ftz_f32(float) #0

define amdgpu_kernel void @sqrt_rn_ftz(ptr addrspace(4) byref(i64) %"30", ptr addrspace(4) byref(i64) %"31") #1 {
  %"32" = alloca i64, align 8, addrspace(5)
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"29"

"29":                                             ; preds = %1
  %"35" = load i64, ptr addrspace(4) %"30", align 8
  store i64 %"35", ptr addrspace(5) %"32", align 8
  %"36" = load i64, ptr addrspace(4) %"31", align 8
  store i64 %"36", ptr addrspace(5) %"33", align 8
  %"38" = load i64, ptr addrspace(5) %"32", align 8
  %"43" = inttoptr i64 %"38" to ptr
  %"37" = load float, ptr %"43", align 4
  store float %"37", ptr addrspace(5) %"34", align 4
  %"40" = load float, ptr addrspace(5) %"34", align 4
  %"39" = call float @__zluda_ptx_impl_sqrt_rn_ftz_f32(float %"40")
  store float %"39", ptr addrspace(5) %"34", align 4
  %"41" = load i64, ptr addrspace(5) %"33", align 8
  %"42" = load float, ptr addrspace(5) %"34", align 4
  %"44" = inttoptr i64 %"41" to ptr
  store float %"42", ptr %"44", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }