define amdgpu_kernel void @b64tof64(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca double, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"40" = load double, ptr addrspace(4) %"34", align 8
  store double %"40", ptr addrspace(5) %"36", align 8
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"38", align 8
  %"43" = load double, ptr addrspace(5) %"36", align 8
  %"49" = bitcast double %"43" to i64
  store i64 %"49", ptr addrspace(5) %"37", align 8
  %"45" = load i64, ptr addrspace(5) %"37", align 8
  %"50" = inttoptr i64 %"45" to ptr
  %"44" = load i64, ptr %"50", align 8
  store i64 %"44", ptr addrspace(5) %"39", align 8
  %"46" = load i64, ptr addrspace(5) %"38", align 8
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"51" = inttoptr i64 %"46" to ptr
  store i64 %"47", ptr %"51", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }