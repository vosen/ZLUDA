define amdgpu_kernel void @or(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %2, ptr addrspace(5) %"41", align 8
  %3 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %3, ptr addrspace(5) %"42", align 8
  %4 = load i64, ptr addrspace(5) %"41", align 8
  %"56" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"56", align 8
  store i64 %5, ptr addrspace(5) %"43", align 8
  %6 = load i64, ptr addrspace(5) %"41", align 8
  %"57" = inttoptr i64 %6 to ptr
  %"37" = getelementptr inbounds i8, ptr %"57", i64 8
  %7 = load i64, ptr %"37", align 8
  store i64 %7, ptr addrspace(5) %"44", align 8
  %8 = load i64, ptr addrspace(5) %"43", align 8
  %9 = load i64, ptr addrspace(5) %"44", align 8
  %"58" = or i64 %8, %9
  store i64 %"58", ptr addrspace(5) %"43", align 8
  %10 = load i64, ptr addrspace(5) %"42", align 8
  %11 = load i64, ptr addrspace(5) %"43", align 8
  %"61" = inttoptr i64 %10 to ptr
  store i64 %11, ptr %"61", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }