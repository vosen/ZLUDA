define hidden i64 @incr(i64 %"46") #0 {
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"49"

"49":                                             ; preds = %1
  store i64 %"46", ptr addrspace(5) %"68", align 8
  %"70" = load i64, ptr addrspace(5) %"68", align 8
  store i64 %"70", ptr addrspace(5) %"69", align 8
  %"72" = load i64, ptr addrspace(5) %"69", align 8
  %"71" = add i64 %"72", 1
  store i64 %"71", ptr addrspace(5) %"69", align 8
  %"73" = load i64, ptr addrspace(5) %"69", align 8
  store i64 %"73", ptr addrspace(5) %"67", align 8
  %"74" = load i64, ptr addrspace(5) %"67", align 8
  store i64 %"74", ptr addrspace(5) %"66", align 8
  %2 = load i64, ptr addrspace(5) %"66", align 8
  ret i64 %2
}

define amdgpu_kernel void @call(ptr addrspace(4) byref(i64) %"51", ptr addrspace(4) byref(i64) %"52") #1 {
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %"56" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"56", ptr addrspace(5) %"53", align 8
  %"57" = load i64, ptr addrspace(4) %"52", align 8
  store i64 %"57", ptr addrspace(5) %"54", align 8
  %"59" = load i64, ptr addrspace(5) %"53", align 8
  %"75" = inttoptr i64 %"59" to ptr addrspace(1)
  %"58" = load i64, ptr addrspace(1) %"75", align 8
  store i64 %"58", ptr addrspace(5) %"55", align 8
  %"62" = load i64, ptr addrspace(5) %"55", align 8
  store i64 %"62", ptr addrspace(5) %"60", align 8
  %"43" = load i64, ptr addrspace(5) %"60", align 8
  %"44" = call i64 @incr(i64 %"43")
  br label %"48"

"48":                                             ; preds = %"47"
  store i64 %"44", ptr addrspace(5) %"61", align 8
  %"63" = load i64, ptr addrspace(5) %"61", align 8
  store i64 %"63", ptr addrspace(5) %"55", align 8
  %"64" = load i64, ptr addrspace(5) %"54", align 8
  %"65" = load i64, ptr addrspace(5) %"55", align 8
  %"78" = inttoptr i64 %"64" to ptr addrspace(1)
  store i64 %"65", ptr addrspace(1) %"78", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }