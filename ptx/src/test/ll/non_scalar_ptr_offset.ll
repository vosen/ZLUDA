define amdgpu_kernel void @non_scalar_ptr_offset(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"56" = inttoptr i64 %4 to ptr addrspace(1)
  %"38" = getelementptr inbounds i8, ptr addrspace(1) %"56", i64 8
  %5 = load <2 x i32>, ptr addrspace(1) %"38", align 8
  %"49" = extractelement <2 x i32> %5, i8 0
  %"50" = extractelement <2 x i32> %5, i8 1
  store i32 %"49", ptr addrspace(5) %"44", align 4
  store i32 %"50", ptr addrspace(5) %"45", align 4
  %6 = load i32, ptr addrspace(5) %"44", align 4
  %7 = load i32, ptr addrspace(5) %"45", align 4
  %"51" = add i32 %6, %7
  store i32 %"51", ptr addrspace(5) %"44", align 4
  %8 = load i64, ptr addrspace(5) %"43", align 8
  %9 = load i32, ptr addrspace(5) %"44", align 4
  %"57" = inttoptr i64 %8 to ptr addrspace(1)
  store i32 %9, ptr addrspace(1) %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }