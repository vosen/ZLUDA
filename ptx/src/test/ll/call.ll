define hidden i64 @incr(i64 %"49") #0 {
  %"69" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  %"71" = alloca i64, align 8, addrspace(5)
  %"72" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"52"

"52":                                             ; preds = %1
  store i64 %"49", ptr addrspace(5) %"71", align 8
  %2 = load i64, ptr addrspace(5) %"71", align 8
  store i64 %2, ptr addrspace(5) %"72", align 8
  %3 = load i64, ptr addrspace(5) %"72", align 8
  %"74" = add i64 %3, 1
  store i64 %"74", ptr addrspace(5) %"72", align 8
  %4 = load i64, ptr addrspace(5) %"72", align 8
  store i64 %4, ptr addrspace(5) %"70", align 8
  %5 = load i64, ptr addrspace(5) %"70", align 8
  store i64 %5, ptr addrspace(5) %"69", align 8
  %6 = load i64, ptr addrspace(5) %"69", align 8
  ret i64 %6
}

define amdgpu_kernel void @call(ptr addrspace(4) byref(i64) %"54", ptr addrspace(4) byref(i64) %"55") #1 {
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"54", align 8
  store i64 %2, ptr addrspace(5) %"56", align 8
  %3 = load i64, ptr addrspace(4) %"55", align 8
  store i64 %3, ptr addrspace(5) %"57", align 8
  %4 = load i64, ptr addrspace(5) %"56", align 8
  %"78" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"78", align 8
  store i64 %5, ptr addrspace(5) %"58", align 8
  %6 = load i64, ptr addrspace(5) %"58", align 8
  store i64 %6, ptr addrspace(5) %"63", align 8
  %7 = load i64, ptr addrspace(5) %"63", align 8
  %"47" = call i64 @incr(i64 %7)
  br label %"51"

"51":                                             ; preds = %"50"
  store i64 %"47", ptr addrspace(5) %"64", align 8
  %8 = load i64, ptr addrspace(5) %"64", align 8
  store i64 %8, ptr addrspace(5) %"58", align 8
  %9 = load i64, ptr addrspace(5) %"57", align 8
  %10 = load i64, ptr addrspace(5) %"58", align 8
  %"81" = inttoptr i64 %9 to ptr addrspace(1)
  store i64 %10, ptr addrspace(1) %"81", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }