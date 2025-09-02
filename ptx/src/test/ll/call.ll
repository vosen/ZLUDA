@0 = addrspace(4) global i64 1

define hidden i64 @incr(i64 %"47") #0 {
  %"67" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  store i64 %"47", ptr addrspace(5) %"69", align 8
  %"71" = load i64, ptr addrspace(5) %"69", align 8
  store i64 %"71", ptr addrspace(5) %"70", align 8
  %"41" = load i64, ptr addrspace(4) @0, align 8
  %"73" = load i64, ptr addrspace(5) %"70", align 8
  %"72" = add i64 %"73", %"41"
  store i64 %"72", ptr addrspace(5) %"70", align 8
  %"74" = load i64, ptr addrspace(5) %"70", align 8
  store i64 %"74", ptr addrspace(5) %"68", align 8
  %"75" = load i64, ptr addrspace(5) %"68", align 8
  store i64 %"75", ptr addrspace(5) %"67", align 8
  %2 = load i64, ptr addrspace(5) %"67", align 8
  ret i64 %2
}

define amdgpu_kernel void @call(ptr addrspace(4) byref(i64) %"52", ptr addrspace(4) byref(i64) %"53") #1 {
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"57" = load i64, ptr addrspace(4) %"52", align 8
  store i64 %"57", ptr addrspace(5) %"54", align 8
  %"58" = load i64, ptr addrspace(4) %"53", align 8
  store i64 %"58", ptr addrspace(5) %"55", align 8
  %"60" = load i64, ptr addrspace(5) %"54", align 8
  %"76" = inttoptr i64 %"60" to ptr addrspace(1)
  %"59" = load i64, ptr addrspace(1) %"76", align 8
  store i64 %"59", ptr addrspace(5) %"56", align 8
  %"63" = load i64, ptr addrspace(5) %"56", align 8
  store i64 %"63", ptr addrspace(5) %"61", align 8
  %"44" = load i64, ptr addrspace(5) %"61", align 8
  %"45" = call i64 @incr(i64 %"44")
  br label %"49"

"49":                                             ; preds = %"48"
  store i64 %"45", ptr addrspace(5) %"62", align 8
  %"64" = load i64, ptr addrspace(5) %"62", align 8
  store i64 %"64", ptr addrspace(5) %"56", align 8
  %"65" = load i64, ptr addrspace(5) %"55", align 8
  %"66" = load i64, ptr addrspace(5) %"56", align 8
  %"79" = inttoptr i64 %"65" to ptr addrspace(1)
  store i64 %"66", ptr addrspace(1) %"79", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }