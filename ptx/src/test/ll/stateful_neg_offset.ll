define amdgpu_kernel void @stateful_neg_offset(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
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
  %"63" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"63", ptr addrspace(5) %"43", align 8
  %6 = load i64, ptr addrspace(5) %"42", align 8
  %7 = inttoptr i64 %6 to ptr
  %"65" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"65", ptr addrspace(5) %"44", align 8
  %8 = load i64, ptr addrspace(5) %"43", align 8
  %9 = load i64, ptr addrspace(5) %"44", align 8
  %"53" = add i64 %8, %9
  store i64 %"53", ptr addrspace(5) %"45", align 8
  %10 = load i64, ptr addrspace(5) %"43", align 8
  %11 = load i64, ptr addrspace(5) %"44", align 8
  %"56" = sub i64 %10, %11
  store i64 %"56", ptr addrspace(5) %"45", align 8
  %12 = load i64, ptr addrspace(5) %"43", align 8
  %"67" = inttoptr i64 %12 to ptr addrspace(1)
  %13 = load i64, ptr addrspace(1) %"67", align 8
  store i64 %13, ptr addrspace(5) %"46", align 8
  %14 = load i64, ptr addrspace(5) %"44", align 8
  %15 = load i64, ptr addrspace(5) %"46", align 8
  %"68" = inttoptr i64 %14 to ptr addrspace(1)
  store i64 %15, ptr addrspace(1) %"68", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }