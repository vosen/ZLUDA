define amdgpu_kernel void @cvta(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %2, ptr addrspace(5) %"41", align 8
  %3 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %3, ptr addrspace(5) %"42", align 8
  %4 = load i64, ptr addrspace(5) %"41", align 8
  %5 = inttoptr i64 %4 to ptr
  %6 = addrspacecast ptr %5 to ptr addrspace(1)
  %"54" = ptrtoint ptr addrspace(1) %6 to i64
  store i64 %"54", ptr addrspace(5) %"41", align 8
  %7 = load i64, ptr addrspace(5) %"42", align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = addrspacecast ptr %8 to ptr addrspace(1)
  %"56" = ptrtoint ptr addrspace(1) %9 to i64
  store i64 %"56", ptr addrspace(5) %"42", align 8
  %10 = load i64, ptr addrspace(5) %"41", align 8
  %"58" = inttoptr i64 %10 to ptr addrspace(1)
  %11 = load float, ptr addrspace(1) %"58", align 4
  store float %11, ptr addrspace(5) %"43", align 4
  %12 = load i64, ptr addrspace(5) %"42", align 8
  %13 = load float, ptr addrspace(5) %"43", align 4
  %"59" = inttoptr i64 %12 to ptr addrspace(1)
  store float %13, ptr addrspace(1) %"59", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
