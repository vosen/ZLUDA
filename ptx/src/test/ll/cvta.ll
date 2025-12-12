define amdgpu_kernel void @cvta(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"36", align 8
  store i64 %2, ptr addrspace(5) %"38", align 8
  %3 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %3, ptr addrspace(5) %"39", align 8
  %4 = load i64, ptr addrspace(5) %"38", align 8
  %5 = inttoptr i64 %4 to ptr
  %"51" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"51", ptr addrspace(5) %"38", align 8
  %6 = load i64, ptr addrspace(5) %"39", align 8
  %7 = inttoptr i64 %6 to ptr
  %"53" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"53", ptr addrspace(5) %"39", align 8
  %8 = load i64, ptr addrspace(5) %"38", align 8
  %"55" = inttoptr i64 %8 to ptr addrspace(1)
  %9 = load float, ptr addrspace(1) %"55", align 4
  store float %9, ptr addrspace(5) %"40", align 4
  %10 = load i64, ptr addrspace(5) %"39", align 8
  %11 = load float, ptr addrspace(5) %"40", align 4
  %"56" = inttoptr i64 %10 to ptr addrspace(1)
  store float %11, ptr addrspace(1) %"56", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }