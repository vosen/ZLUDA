define amdgpu_kernel void @cvta(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
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
  %2 = inttoptr i64 %"41" to ptr
  %"48" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"48", ptr addrspace(5) %"35", align 8
  %"43" = load i64, ptr addrspace(5) %"36", align 8
  %3 = inttoptr i64 %"43" to ptr
  %"50" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"50", ptr addrspace(5) %"36", align 8
  %"45" = load i64, ptr addrspace(5) %"35", align 8
  %"52" = inttoptr i64 %"45" to ptr addrspace(1)
  %"44" = load float, ptr addrspace(1) %"52", align 4
  store float %"44", ptr addrspace(5) %"37", align 4
  %"46" = load i64, ptr addrspace(5) %"36", align 8
  %"47" = load float, ptr addrspace(5) %"37", align 4
  %"53" = inttoptr i64 %"46" to ptr addrspace(1)
  store float %"47", ptr addrspace(1) %"53", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }