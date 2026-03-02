define amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
  %"13" = alloca [8 x i8], align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"48", align 8
  %3 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %3, ptr addrspace(5) %"49", align 8
  %4 = load i64, ptr addrspace(5) %"48", align 8
  %"60" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"60", align 8
  store i64 %5, ptr addrspace(5) %"50", align 8
  %6 = load i64, ptr addrspace(5) %"50", align 8
  %"40" = add i64 %6, 1
  %"61" = addrspacecast ptr addrspace(5) %"13" to ptr
  store i64 %"40", ptr %"61", align 8
  %"63" = addrspacecast ptr addrspace(5) %"13" to ptr
  %"42" = getelementptr inbounds i8, ptr %"63", i64 0
  %7 = load i64, ptr %"42", align 8
  store i64 %7, ptr addrspace(5) %"50", align 8
  %8 = load i64, ptr addrspace(5) %"49", align 8
  %"65" = inttoptr i64 %8 to ptr addrspace(1)
  %"44" = getelementptr inbounds i8, ptr addrspace(1) %"65", i64 0
  %9 = load i64, ptr addrspace(5) %"50", align 8
  store i64 %9, ptr addrspace(1) %"44", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
