define hidden i64 @incr(i64 %"43") #0 {
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i64, align 8, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  store i64 %"43", ptr addrspace(5) %"65", align 8
  %"67" = load i64, ptr addrspace(5) %"65", align 8
  store i64 %"67", ptr addrspace(5) %"66", align 8
  %"69" = load i64, ptr addrspace(5) %"66", align 8
  %"68" = add i64 %"69", 1
  store i64 %"68", ptr addrspace(5) %"66", align 8
  %"70" = load i64, ptr addrspace(5) %"66", align 8
  store i64 %"70", ptr addrspace(5) %"64", align 8
  %"71" = load i64, ptr addrspace(5) %"64", align 8
  store i64 %"71", ptr addrspace(5) %"63", align 8
  %2 = load i64, ptr addrspace(5) %"63", align 8
  ret i64 %2
}

define amdgpu_kernel void @call(ptr addrspace(4) byref(i64) %"48", ptr addrspace(4) byref(i64) %"49") #1 {
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"53" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"53", ptr addrspace(5) %"50", align 8
  %"54" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"54", ptr addrspace(5) %"51", align 8
  %"56" = load i64, ptr addrspace(5) %"50", align 8
  %"72" = inttoptr i64 %"56" to ptr addrspace(1)
  %"55" = load i64, ptr addrspace(1) %"72", align 8
  store i64 %"55", ptr addrspace(5) %"52", align 8
  %"59" = load i64, ptr addrspace(5) %"52", align 8
  store i64 %"59", ptr addrspace(5) %"57", align 8
  %"40" = load i64, ptr addrspace(5) %"57", align 8
  %"41" = call i64 @incr(i64 %"40")
  br label %"45"

"45":                                             ; preds = %"44"
  store i64 %"41", ptr addrspace(5) %"58", align 8
  %"60" = load i64, ptr addrspace(5) %"58", align 8
  store i64 %"60", ptr addrspace(5) %"52", align 8
  %"61" = load i64, ptr addrspace(5) %"51", align 8
  %"62" = load i64, ptr addrspace(5) %"52", align 8
  %"75" = inttoptr i64 %"61" to ptr addrspace(1)
  store i64 %"62", ptr addrspace(1) %"75", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }