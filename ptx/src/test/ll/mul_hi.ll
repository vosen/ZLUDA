define amdgpu_kernel void @mul_hi(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %2, ptr addrspace(5) %"43", align 8
  %3 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %3, ptr addrspace(5) %"44", align 8
  %4 = load i64, ptr addrspace(5) %"43", align 8
  %"55" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"55", align 8
  store i64 %5, ptr addrspace(5) %"45", align 8
  %6 = load i64, ptr addrspace(5) %"45", align 8
  %7 = zext i64 %6 to i128
  %8 = mul i128 %7, 2
  %9 = lshr i128 %8, 64
  %"51" = trunc i128 %9 to i64
  store i64 %"51", ptr addrspace(5) %"46", align 8
  %10 = load i64, ptr addrspace(5) %"44", align 8
  %11 = load i64, ptr addrspace(5) %"46", align 8
  %"56" = inttoptr i64 %10 to ptr
  store i64 %11, ptr %"56", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
