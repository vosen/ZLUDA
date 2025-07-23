declare [16 x i8] @foobar(i64) #0

define amdgpu_kernel void @extern_func(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"57" = alloca [16 x i8], align 16, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"50", ptr addrspace(5) %"46", align 8
  %"51" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"51", ptr addrspace(5) %"47", align 8
  %"53" = load i64, ptr addrspace(5) %"46", align 8
  %"61" = inttoptr i64 %"53" to ptr addrspace(1)
  %"52" = load i64, ptr addrspace(1) %"61", align 8
  store i64 %"52", ptr addrspace(5) %"48", align 8
  %"55" = getelementptr inbounds i8, ptr addrspace(5) %"54", i64 0
  %"56" = load i64, ptr addrspace(5) %"48", align 8
  store i64 %"56", ptr addrspace(5) %"55", align 8
  %"39" = load i64, ptr addrspace(5) %"54", align 8
  %"40" = call [16 x i8] @foobar(i64 %"39")
  br label %"42"

"42":                                             ; preds = %"41"
  store [16 x i8] %"40", ptr addrspace(5) %"57", align 1
  %"58" = load i64, ptr addrspace(5) %"57", align 8
  store i64 %"58", ptr addrspace(5) %"49", align 8
  %"59" = load i64, ptr addrspace(5) %"47", align 8
  %"60" = load i64, ptr addrspace(5) %"49", align 8
  %"64" = inttoptr i64 %"59" to ptr
  store i64 %"60", ptr %"64", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }