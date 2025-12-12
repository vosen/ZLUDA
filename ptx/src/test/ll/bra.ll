define amdgpu_kernel void @bra(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %2, ptr addrspace(5) %"44", align 8
  %3 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"44", align 8
  %"56" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"56", align 8
  store i64 %5, ptr addrspace(5) %"46", align 8
  br label %"12"

"12":                                             ; preds = %"41"
  %6 = load i64, ptr addrspace(5) %"46", align 8
  %"52" = add i64 %6, 1
  store i64 %"52", ptr addrspace(5) %"47", align 8
  br label %"14"

"14":                                             ; preds = %"12"
  %7 = load i64, ptr addrspace(5) %"45", align 8
  %8 = load i64, ptr addrspace(5) %"47", align 8
  %"57" = inttoptr i64 %7 to ptr
  store i64 %8, ptr %"57", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }