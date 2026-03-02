define amdgpu_kernel void @constant_f32(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"53" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"53", align 4
  store float %5, ptr addrspace(5) %"44", align 4
  %6 = load float, ptr addrspace(5) %"44", align 4
  %"49" = fmul float %6, 5.000000e-01
  store float %"49", ptr addrspace(5) %"44", align 4
  %7 = load i64, ptr addrspace(5) %"43", align 8
  %8 = load float, ptr addrspace(5) %"44", align 4
  %"54" = inttoptr i64 %7 to ptr
  store float %8, ptr %"54", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
