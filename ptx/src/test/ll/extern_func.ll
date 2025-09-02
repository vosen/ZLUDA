@0 = addrspace(4) global i64 0

declare hidden [16 x i8] @foobar(i64) #0

define amdgpu_kernel void @extern_func(ptr addrspace(4) byref(i64) %"48", ptr addrspace(4) byref(i64) %"49") #1 {
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"61" = alloca [16 x i8], align 16, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %"54" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"54", ptr addrspace(5) %"50", align 8
  %"55" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"55", ptr addrspace(5) %"51", align 8
  %"57" = load i64, ptr addrspace(5) %"50", align 8
  %"65" = inttoptr i64 %"57" to ptr addrspace(1)
  %"56" = load i64, ptr addrspace(1) %"65", align 8
  store i64 %"56", ptr addrspace(5) %"52", align 8
  %"39" = load i64, ptr addrspace(4) @0, align 8
  %"59" = getelementptr inbounds i8, ptr addrspace(5) %"58", i64 %"39"
  %"60" = load i64, ptr addrspace(5) %"52", align 8
  store i64 %"60", ptr addrspace(5) %"59", align 8
  %"43" = load i64, ptr addrspace(5) %"58", align 8
  %"44" = call [16 x i8] @foobar(i64 %"43")
  br label %"46"

"46":                                             ; preds = %"45"
  store [16 x i8] %"44", ptr addrspace(5) %"61", align 1
  %"62" = load i64, ptr addrspace(5) %"61", align 8
  store i64 %"62", ptr addrspace(5) %"53", align 8
  %"63" = load i64, ptr addrspace(5) %"51", align 8
  %"64" = load i64, ptr addrspace(5) %"53", align 8
  %"68" = inttoptr i64 %"63" to ptr
  store i64 %"64", ptr %"68", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }