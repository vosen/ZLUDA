%struct.i32.i1 = type { i32, i1 }

define hidden %struct.i32.i1 @do_something(i32 %"11") #0 {
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %"51" = add i32 %"11", 1
  store i32 %"51", ptr addrspace(5) %"49", align 4
  store i1 true, ptr addrspace(5) %"50", align 1
  %2 = load i32, ptr addrspace(5) %"49", align 4
  %3 = load i1, ptr addrspace(5) %"50", align 1
  %4 = insertvalue %struct.i32.i1 undef, i32 %2, 0
  %5 = insertvalue %struct.i32.i1 %4, i1 %3, 1
  ret %struct.i32.i1 %5
}

define amdgpu_kernel void @multiple_return(ptr addrspace(4) byref(i64) %"53", ptr addrspace(4) byref(i64) %"54") #1 {
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %"60" = load i64, ptr addrspace(4) %"53", align 8
  store i64 %"60", ptr addrspace(5) %"55", align 8
  %"61" = load i64, ptr addrspace(4) %"54", align 8
  store i64 %"61", ptr addrspace(5) %"56", align 8
  %"63" = load i64, ptr addrspace(5) %"55", align 8
  %"71" = inttoptr i64 %"63" to ptr
  %"62" = load i32, ptr %"71", align 4
  store i32 %"62", ptr addrspace(5) %"57", align 4
  %"66" = load i32, ptr addrspace(5) %"57", align 4
  %2 = call %struct.i32.i1 @do_something(i32 %"66")
  %"64" = extractvalue %struct.i32.i1 %2, 0
  %"65" = extractvalue %struct.i32.i1 %2, 1
  store i32 %"64", ptr addrspace(5) %"58", align 4
  store i1 %"65", ptr addrspace(5) %"59", align 1
  br label %"48"

"48":                                             ; preds = %"47"
  %"67" = load i64, ptr addrspace(5) %"56", align 8
  %"68" = load i32, ptr addrspace(5) %"58", align 4
  %"72" = inttoptr i64 %"67" to ptr
  store i32 %"68", ptr %"72", align 4
  %"69" = load i1, ptr addrspace(5) %"59", align 1
  br i1 %"69", label %"20", label %"21"

"20":                                             ; preds = %"48"
  %"70" = load i64, ptr addrspace(5) %"56", align 8
  %"73" = inttoptr i64 %"70" to ptr
  %"44" = getelementptr inbounds i8, ptr %"73", i64 4
  store i32 123, ptr %"44", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"48"
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }