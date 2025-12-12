define amdgpu_kernel void @mul_wide(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"58" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i32, ptr addrspace(1) %"58", align 4
  store i32 %5, ptr addrspace(5) %"44", align 4
  %6 = load i64, ptr addrspace(5) %"42", align 8
  %"59" = inttoptr i64 %6 to ptr addrspace(1)
  %"38" = getelementptr inbounds i8, ptr addrspace(1) %"59", i64 4
  %7 = load i32, ptr addrspace(1) %"38", align 4
  store i32 %7, ptr addrspace(5) %"45", align 4
  %8 = load i32, ptr addrspace(5) %"44", align 4
  %9 = load i32, ptr addrspace(5) %"45", align 4
  %10 = sext i32 %8 to i64
  %11 = sext i32 %9 to i64
  %"53" = mul i64 %10, %11
  store i64 %"53", ptr addrspace(5) %"46", align 8
  %12 = load i64, ptr addrspace(5) %"43", align 8
  %13 = load i64, ptr addrspace(5) %"46", align 8
  %"60" = inttoptr i64 %12 to ptr
  store i64 %13, ptr %"60", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }