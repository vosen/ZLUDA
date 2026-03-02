define amdgpu_kernel void @stateful_neg_offset(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %2, ptr addrspace(5) %"44", align 8
  %3 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"44", align 8
  %5 = inttoptr i64 %4 to ptr
  %"66" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"66", ptr addrspace(5) %"46", align 8
  %6 = load i64, ptr addrspace(5) %"45", align 8
  %7 = inttoptr i64 %6 to ptr
  %"68" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"68", ptr addrspace(5) %"47", align 8
  %8 = load i64, ptr addrspace(5) %"46", align 8
  %9 = load i64, ptr addrspace(5) %"47", align 8
  %"56" = add i64 %8, %9
  store i64 %"56", ptr addrspace(5) %"48", align 8
  %10 = load i64, ptr addrspace(5) %"46", align 8
  %11 = load i64, ptr addrspace(5) %"47", align 8
  %"59" = sub i64 %10, %11
  store i64 %"59", ptr addrspace(5) %"48", align 8
  %12 = load i64, ptr addrspace(5) %"46", align 8
  %"70" = inttoptr i64 %12 to ptr addrspace(1)
  %13 = load i64, ptr addrspace(1) %"70", align 8
  store i64 %13, ptr addrspace(5) %"49", align 8
  %14 = load i64, ptr addrspace(5) %"47", align 8
  %15 = load i64, ptr addrspace(5) %"49", align 8
  %"71" = inttoptr i64 %14 to ptr addrspace(1)
  store i64 %15, ptr addrspace(1) %"71", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
