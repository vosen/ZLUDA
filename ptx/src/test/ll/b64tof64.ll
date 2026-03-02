define amdgpu_kernel void @b64tof64(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca double, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load double, ptr addrspace(4) %"40", align 8
  store double %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"44", align 8
  %4 = load double, ptr addrspace(5) %"42", align 8
  %"55" = bitcast double %4 to i64
  store i64 %"55", ptr addrspace(5) %"43", align 8
  %5 = load i64, ptr addrspace(5) %"43", align 8
  %"56" = inttoptr i64 %5 to ptr
  %6 = load i64, ptr %"56", align 8
  store i64 %6, ptr addrspace(5) %"45", align 8
  %7 = load i64, ptr addrspace(5) %"44", align 8
  %8 = load i64, ptr addrspace(5) %"45", align 8
  %"57" = inttoptr i64 %7 to ptr
  store i64 %8, ptr %"57", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
