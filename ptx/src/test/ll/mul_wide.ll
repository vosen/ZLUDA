define amdgpu_kernel void @mul_wide(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %2, ptr addrspace(5) %"45", align 8
  %3 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %3, ptr addrspace(5) %"46", align 8
  %4 = load i64, ptr addrspace(5) %"45", align 8
  %"61" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i32, ptr addrspace(1) %"61", align 4
  store i32 %5, ptr addrspace(5) %"47", align 4
  %6 = load i64, ptr addrspace(5) %"45", align 8
  %"62" = inttoptr i64 %6 to ptr addrspace(1)
  %"41" = getelementptr inbounds i8, ptr addrspace(1) %"62", i64 4
  %7 = load i32, ptr addrspace(1) %"41", align 4
  store i32 %7, ptr addrspace(5) %"48", align 4
  %8 = load i32, ptr addrspace(5) %"47", align 4
  %9 = load i32, ptr addrspace(5) %"48", align 4
  %10 = sext i32 %8 to i64
  %11 = sext i32 %9 to i64
  %"56" = mul i64 %10, %11
  store i64 %"56", ptr addrspace(5) %"49", align 8
  %12 = load i64, ptr addrspace(5) %"46", align 8
  %13 = load i64, ptr addrspace(5) %"49", align 8
  %"63" = inttoptr i64 %12 to ptr
  store i64 %13, ptr %"63", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
