define amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"11" = alloca [8 x i8], align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  store i64 1, ptr addrspace(5) %"49", align 4
  %"52" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"52", align 4
  %"55" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"55", align 4
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"45", ptr addrspace(5) %"42", align 8
  %"46" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"46", ptr addrspace(5) %"43", align 8
  %"48" = load i64, ptr addrspace(5) %"42", align 8
  %"60" = inttoptr i64 %"48" to ptr addrspace(1)
  %"59" = load i64, ptr addrspace(1) %"60", align 8
  store i64 %"59", ptr addrspace(5) %"44", align 8
  %"50" = load i64, ptr addrspace(5) %"44", align 8
  %"51" = load i64, ptr addrspace(5) %"49", align 8
  %"34" = add i64 %"50", %"51"
  %"61" = addrspacecast ptr addrspace(5) %"11" to ptr
  store i64 %"34", ptr %"61", align 8
  %"53" = load i64, ptr addrspace(5) %"52", align 8
  %"63" = addrspacecast ptr addrspace(5) %"11" to ptr
  %"36" = getelementptr inbounds i8, ptr %"63", i64 %"53"
  %"64" = load i64, ptr %"36", align 8
  store i64 %"64", ptr addrspace(5) %"44", align 8
  %"56" = load i64, ptr addrspace(5) %"43", align 8
  %"57" = load i64, ptr addrspace(5) %"55", align 8
  %"65" = inttoptr i64 %"56" to ptr addrspace(1)
  %"38" = getelementptr inbounds i8, ptr addrspace(1) %"65", i64 %"57"
  %"58" = load i64, ptr addrspace(5) %"44", align 8
  store i64 %"58", ptr addrspace(1) %"38", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }