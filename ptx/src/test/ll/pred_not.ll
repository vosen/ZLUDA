define amdgpu_kernel void @pred_not(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #0 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i1, align 1, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"56", align 4
  %"66" = alloca i64, align 8, addrspace(5)
  store i64 1, ptr addrspace(5) %"66", align 4
  %"70" = alloca i64, align 8, addrspace(5)
  store i64 2, ptr addrspace(5) %"70", align 4
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %"52" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"52", ptr addrspace(5) %"46", align 8
  %"53" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"53", ptr addrspace(5) %"47", align 8
  %"55" = load i64, ptr addrspace(5) %"46", align 8
  %"75" = inttoptr i64 %"55" to ptr
  %"54" = load i64, ptr %"75", align 8
  store i64 %"54", ptr addrspace(5) %"48", align 8
  %"57" = load i64, ptr addrspace(5) %"46", align 8
  %"58" = load i64, ptr addrspace(5) %"56", align 8
  %"76" = inttoptr i64 %"57" to ptr
  %"40" = getelementptr inbounds i8, ptr %"76", i64 %"58"
  %"59" = load i64, ptr %"40", align 8
  store i64 %"59", ptr addrspace(5) %"49", align 8
  %"61" = load i64, ptr addrspace(5) %"48", align 8
  %"62" = load i64, ptr addrspace(5) %"49", align 8
  %2 = icmp ult i64 %"61", %"62"
  store i1 %2, ptr addrspace(5) %"51", align 1
  %"64" = load i1, ptr addrspace(5) %"51", align 1
  %"63" = xor i1 %"64", true
  store i1 %"63", ptr addrspace(5) %"51", align 1
  %"65" = load i1, ptr addrspace(5) %"51", align 1
  br i1 %"65", label %"17", label %"18"

"17":                                             ; preds = %"43"
  %"68" = load i64, ptr addrspace(5) %"66", align 8
  store i64 %"68", ptr addrspace(5) %"50", align 8
  br label %"18"

"18":                                             ; preds = %"17", %"43"
  %"69" = load i1, ptr addrspace(5) %"51", align 1
  br i1 %"69", label %"20", label %"19"

"19":                                             ; preds = %"18"
  %"72" = load i64, ptr addrspace(5) %"70", align 8
  store i64 %"72", ptr addrspace(5) %"50", align 8
  br label %"20"

"20":                                             ; preds = %"19", %"18"
  %"73" = load i64, ptr addrspace(5) %"47", align 8
  %"74" = load i64, ptr addrspace(5) %"50", align 8
  %"77" = inttoptr i64 %"73" to ptr
  store i64 %"74", ptr %"77", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }