define hidden <2 x i32> @impl(<2 x i32> %"11") #0 {
  %"53" = alloca <2 x i32>, align 8, addrspace(5)
  %"54" = alloca <2 x i32>, align 8, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %"44" = extractelement <2 x i32> %"11", i8 0
  store i32 %"44", ptr addrspace(5) %"55", align 4
  %"45" = extractelement <2 x i32> %"11", i8 1
  store i32 %"45", ptr addrspace(5) %"56", align 4
  %2 = load i32, ptr addrspace(5) %"55", align 4
  %3 = load i32, ptr addrspace(5) %"56", align 4
  %"59" = add i32 %2, %3
  store i32 %"59", ptr addrspace(5) %"56", align 4
  %4 = load i32, ptr addrspace(5) %"56", align 4
  %5 = load <2 x i32>, ptr addrspace(5) %"54", align 8
  %"63" = insertelement <2 x i32> %5, i32 %4, i8 0
  store <2 x i32> %"63", ptr addrspace(5) %"54", align 8
  %6 = load i32, ptr addrspace(5) %"56", align 4
  %7 = load <2 x i32>, ptr addrspace(5) %"54", align 8
  %"66" = insertelement <2 x i32> %7, i32 %6, i8 1
  store <2 x i32> %"66", ptr addrspace(5) %"54", align 8
  %8 = load <2 x i32>, ptr addrspace(5) %"54", align 8
  %"49" = extractelement <2 x i32> %8, i8 1
  %9 = load <2 x i32>, ptr addrspace(5) %"54", align 8
  %"69" = insertelement <2 x i32> %9, i32 %"49", i8 0
  store <2 x i32> %"69", ptr addrspace(5) %"54", align 8
  %10 = load <2 x i32>, ptr addrspace(5) %"54", align 8
  store <2 x i32> %10, ptr addrspace(5) %"53", align 8
  %11 = load <2 x i32>, ptr addrspace(5) %"53", align 8
  ret <2 x i32> %11
}

define amdgpu_kernel void @vector(ptr addrspace(4) byref(i64) %"73", ptr addrspace(4) byref(i64) %"74") #1 {
  %"75" = alloca i64, align 8, addrspace(5)
  %"76" = alloca i64, align 8, addrspace(5)
  %"77" = alloca <2 x i32>, align 8, addrspace(5)
  %"78" = alloca i32, align 4, addrspace(5)
  %"79" = alloca i32, align 4, addrspace(5)
  %"80" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"73", align 8
  store i64 %2, ptr addrspace(5) %"75", align 8
  %3 = load i64, ptr addrspace(4) %"74", align 8
  store i64 %3, ptr addrspace(5) %"76", align 8
  %4 = load i64, ptr addrspace(5) %"75", align 8
  %"91" = inttoptr i64 %4 to ptr
  %5 = load <2 x i32>, ptr %"91", align 8
  store <2 x i32> %5, ptr addrspace(5) %"77", align 8
  %6 = load <2 x i32>, ptr addrspace(5) %"77", align 8
  %"85" = call <2 x i32> @impl(<2 x i32> %6)
  store <2 x i32> %"85", ptr addrspace(5) %"77", align 8
  br label %"52"

"52":                                             ; preds = %"51"
  %7 = load <2 x i32>, ptr addrspace(5) %"77", align 8
  %"92" = bitcast <2 x i32> %7 to i64
  store i64 %"92", ptr addrspace(5) %"80", align 8
  %8 = load i64, ptr addrspace(5) %"76", align 8
  %9 = load <2 x i32>, ptr addrspace(5) %"77", align 8
  %"93" = inttoptr i64 %8 to ptr
  store <2 x i32> %9, ptr %"93", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }