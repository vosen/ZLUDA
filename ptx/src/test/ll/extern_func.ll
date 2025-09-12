declare hidden [16 x i8] @foobar(i64) #0

define amdgpu_kernel void @extern_func(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"60" = alloca [16 x i8], align 16, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"53" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"53", ptr addrspace(5) %"49", align 8
  %"54" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"54", ptr addrspace(5) %"50", align 8
  %"56" = load i64, ptr addrspace(5) %"49", align 8
  %"64" = inttoptr i64 %"56" to ptr addrspace(1)
  %"55" = load i64, ptr addrspace(1) %"64", align 8
  store i64 %"55", ptr addrspace(5) %"51", align 8
  %"58" = getelementptr inbounds i8, ptr addrspace(5) %"57", i64 0
  %"59" = load i64, ptr addrspace(5) %"51", align 8
  store i64 %"59", ptr addrspace(5) %"58", align 8
  %"42" = load i64, ptr addrspace(5) %"57", align 8
  %"43" = call [16 x i8] @foobar(i64 %"42")
  br label %"45"

"45":                                             ; preds = %"44"
  store [16 x i8] %"43", ptr addrspace(5) %"60", align 1
  %"61" = load i64, ptr addrspace(5) %"60", align 8
  store i64 %"61", ptr addrspace(5) %"52", align 8
  %"62" = load i64, ptr addrspace(5) %"50", align 8
  %"63" = load i64, ptr addrspace(5) %"52", align 8
  %"67" = inttoptr i64 %"62" to ptr
  store i64 %"63", ptr %"67", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }