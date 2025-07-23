define amdgpu_kernel void @b64tof64(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #0 {
  %"33" = alloca double, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"37" = load double, ptr addrspace(4) %"31", align 8
  store double %"37", ptr addrspace(5) %"33", align 8
  %"38" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"38", ptr addrspace(5) %"35", align 8
  %"40" = load double, ptr addrspace(5) %"33", align 8
  %"46" = bitcast double %"40" to i64
  store i64 %"46", ptr addrspace(5) %"34", align 8
  %"42" = load i64, ptr addrspace(5) %"34", align 8
  %"47" = inttoptr i64 %"42" to ptr
  %"41" = load i64, ptr %"47", align 8
  store i64 %"41", ptr addrspace(5) %"36", align 8
  %"43" = load i64, ptr addrspace(5) %"35", align 8
  %"44" = load i64, ptr addrspace(5) %"36", align 8
  %"48" = inttoptr i64 %"43" to ptr
  store i64 %"44", ptr %"48", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
