define amdgpu_kernel void @param_is_addressable(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"38" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"38", ptr addrspace(5) %"35", align 8
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"49" = ptrtoint ptr addrspace(4) %"33" to i64
  %2 = inttoptr i64 %"49" to ptr addrspace(4)
  %"40" = addrspacecast ptr addrspace(4) %2 to ptr
  store ptr %"40", ptr addrspace(5) %"37", align 8
  %"43" = load i64, ptr addrspace(5) %"37", align 8
  %"50" = inttoptr i64 %"43" to ptr
  %"42" = load i64, ptr %"50", align 8
  store i64 %"42", ptr addrspace(5) %"37", align 8
  %"45" = load i64, ptr addrspace(5) %"37", align 8
  %"46" = load i64, ptr addrspace(5) %"35", align 8
  %"51" = sub i64 %"45", %"46"
  store i64 %"51", ptr addrspace(5) %"37", align 8
  %"47" = load i64, ptr addrspace(5) %"36", align 8
  %"48" = load i64, ptr addrspace(5) %"37", align 8
  %"53" = inttoptr i64 %"47" to ptr
  store i64 %"48", ptr %"53", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }