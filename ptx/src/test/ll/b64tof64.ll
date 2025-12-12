define amdgpu_kernel void @b64tof64(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca double, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %2 = load double, ptr addrspace(4) %"37", align 8
  store double %2, ptr addrspace(5) %"39", align 8
  %3 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %3, ptr addrspace(5) %"41", align 8
  %4 = load double, ptr addrspace(5) %"39", align 8
  %"52" = bitcast double %4 to i64
  store i64 %"52", ptr addrspace(5) %"40", align 8
  %5 = load i64, ptr addrspace(5) %"40", align 8
  %"53" = inttoptr i64 %5 to ptr
  %6 = load i64, ptr %"53", align 8
  store i64 %6, ptr addrspace(5) %"42", align 8
  %7 = load i64, ptr addrspace(5) %"41", align 8
  %8 = load i64, ptr addrspace(5) %"42", align 8
  %"54" = inttoptr i64 %7 to ptr
  store i64 %8, ptr %"54", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }