define amdgpu_kernel void @stateful_ld_st_simple(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %2, ptr addrspace(5) %"40", align 8
  %3 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %3, ptr addrspace(5) %"41", align 8
  %4 = load i64, ptr addrspace(5) %"40", align 8
  %5 = inttoptr i64 %4 to ptr
  %"55" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"55", ptr addrspace(5) %"42", align 8
  %6 = load i64, ptr addrspace(5) %"41", align 8
  %7 = inttoptr i64 %6 to ptr
  %"57" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"57", ptr addrspace(5) %"43", align 8
  %8 = load i64, ptr addrspace(5) %"42", align 8
  %"59" = inttoptr i64 %8 to ptr addrspace(1)
  %9 = load i64, ptr addrspace(1) %"59", align 8
  store i64 %9, ptr addrspace(5) %"44", align 8
  %10 = load i64, ptr addrspace(5) %"43", align 8
  %11 = load i64, ptr addrspace(5) %"44", align 8
  %"60" = inttoptr i64 %10 to ptr addrspace(1)
  store i64 %11, ptr addrspace(1) %"60", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }