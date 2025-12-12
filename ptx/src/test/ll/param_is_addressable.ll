define amdgpu_kernel void @param_is_addressable(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"36", align 8
  store i64 %2, ptr addrspace(5) %"38", align 8
  %3 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %3, ptr addrspace(5) %"39", align 8
  %"52" = ptrtoint ptr addrspace(4) %"36" to i64
  %4 = inttoptr i64 %"52" to ptr addrspace(4)
  %"43" = addrspacecast ptr addrspace(4) %4 to ptr
  store ptr %"43", ptr addrspace(5) %"40", align 8
  %5 = load i64, ptr addrspace(5) %"40", align 8
  %"53" = inttoptr i64 %5 to ptr
  %6 = load i64, ptr %"53", align 8
  store i64 %6, ptr addrspace(5) %"40", align 8
  %7 = load i64, ptr addrspace(5) %"40", align 8
  %8 = load i64, ptr addrspace(5) %"38", align 8
  %"54" = sub i64 %7, %8
  store i64 %"54", ptr addrspace(5) %"40", align 8
  %9 = load i64, ptr addrspace(5) %"39", align 8
  %10 = load i64, ptr addrspace(5) %"40", align 8
  %"56" = inttoptr i64 %9 to ptr
  store i64 %10, ptr %"56", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }