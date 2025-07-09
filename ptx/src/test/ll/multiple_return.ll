define { i64, i64 } @do_something(i64 %"10") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %"44" = add i64 %"10", 1
  store i64 %"44", ptr addrspace(5) %"42", align 4
  %"45" = add i64 %"10", 2
  store i64 %"45", ptr addrspace(5) %"43", align 4
  %2 = load i64, ptr addrspace(5) %"42", align 4
  %3 = load i64, ptr addrspace(5) %"43", align 4
  %4 = insertvalue { i64, i64 } poison, i64 %2, 0
  %5 = insertvalue { i64, i64 } %4, i64 %3, 1
  ret { i64, i64 } %5
}

define amdgpu_kernel void @multiple_return(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #1 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"53" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"53", ptr addrspace(5) %"48", align 4
  %"54" = load i64, ptr addrspace(4) %"47", align 4
  store i64 %"54", ptr addrspace(5) %"49", align 4
  %"56" = load i64, ptr addrspace(5) %"48", align 4
  %"64" = inttoptr i64 %"56" to ptr
  %"55" = load i64, ptr %"64", align 4
  store i64 %"55", ptr addrspace(5) %"50", align 4
  %"59" = load i64, ptr addrspace(5) %"50", align 4
  %2 = call { i64, i64 } @do_something(i64 %"59")
  %"57" = extractvalue { i64, i64 } %2, 0
  %"58" = extractvalue { i64, i64 } %2, 1
  store i64 %"57", ptr addrspace(5) %"51", align 4
  store i64 %"58", ptr addrspace(5) %"52", align 4
  br label %"41"

"41":                                             ; preds = %"40"
  %"60" = load i64, ptr addrspace(5) %"49", align 4
  %"61" = load i64, ptr addrspace(5) %"51", align 4
  %"65" = inttoptr i64 %"60" to ptr
  store i64 %"61", ptr %"65", align 4
  %"62" = load i64, ptr addrspace(5) %"49", align 4
  %"66" = inttoptr i64 %"62" to ptr
  %"38" = getelementptr inbounds i8, ptr %"66", i64 8
  %"63" = load i64, ptr addrspace(5) %"52", align 4
  store i64 %"63", ptr %"38", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }