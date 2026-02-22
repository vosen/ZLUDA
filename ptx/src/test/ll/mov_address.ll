define amdgpu_kernel void @mov_address(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"13" = alloca [8 x i8], align 1, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"42" = ptrtoint ptr addrspace(5) %"13" to i64
  store i64 %"42", ptr addrspace(5) %"40", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
