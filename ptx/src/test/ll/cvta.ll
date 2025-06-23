@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @cvta(ptr addrspace(4) byref(i64) %"30", ptr addrspace(4) byref(i64) %"31") #0 {
  %"32" = alloca i64, align 8, addrspace(5)
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"29"

"29":                                             ; preds = %1
  %"35" = load i64, ptr addrspace(4) %"30", align 4
  store i64 %"35", ptr addrspace(5) %"32", align 4
  %"36" = load i64, ptr addrspace(4) %"31", align 4
  store i64 %"36", ptr addrspace(5) %"33", align 4
  %"38" = load i64, ptr addrspace(5) %"32", align 4
  %2 = inttoptr i64 %"38" to ptr
  %"45" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"45", ptr addrspace(5) %"32", align 8
  %"40" = load i64, ptr addrspace(5) %"33", align 4
  %3 = inttoptr i64 %"40" to ptr
  %"47" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"47", ptr addrspace(5) %"33", align 8
  %"42" = load i64, ptr addrspace(5) %"32", align 4
  %"49" = inttoptr i64 %"42" to ptr addrspace(1)
  %"41" = load float, ptr addrspace(1) %"49", align 4
  store float %"41", ptr addrspace(5) %"34", align 4
  %"43" = load i64, ptr addrspace(5) %"33", align 4
  %"44" = load float, ptr addrspace(5) %"34", align 4
  %"50" = inttoptr i64 %"43" to ptr addrspace(1)
  store float %"44", ptr addrspace(1) %"50", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }