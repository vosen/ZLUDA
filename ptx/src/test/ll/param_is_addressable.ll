define amdgpu_kernel void @param_is_addressable(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %2, ptr addrspace(5) %"41", align 8
  %3 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %3, ptr addrspace(5) %"42", align 8
  %"55" = ptrtoint ptr addrspace(4) %"39" to i64
  %4 = inttoptr i64 %"55" to ptr addrspace(4)
  %"46" = addrspacecast ptr addrspace(4) %4 to ptr
  store ptr %"46", ptr addrspace(5) %"43", align 8
  %5 = load i64, ptr addrspace(5) %"43", align 8
  %"56" = inttoptr i64 %5 to ptr
  %6 = load i64, ptr %"56", align 8
  store i64 %6, ptr addrspace(5) %"43", align 8
  %7 = load i64, ptr addrspace(5) %"43", align 8
  %8 = load i64, ptr addrspace(5) %"41", align 8
  %"57" = sub i64 %7, %8
  store i64 %"57", ptr addrspace(5) %"43", align 8
  %9 = load i64, ptr addrspace(5) %"42", align 8
  %10 = load i64, ptr addrspace(5) %"43", align 8
  %"59" = inttoptr i64 %9 to ptr
  store i64 %10, ptr %"59", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
