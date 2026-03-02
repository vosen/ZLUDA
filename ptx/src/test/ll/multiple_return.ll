%struct.i32.i1 = type { i32, i1 }

define hidden %struct.i32.i1 @do_something(i32 %"13") #0 {
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"52"

"52":                                             ; preds = %1
  %"57" = add i32 %"13", 1
  store i32 %"57", ptr addrspace(5) %"55", align 4
  store i1 true, ptr addrspace(5) %"56", align 1
  %2 = load i32, ptr addrspace(5) %"55", align 4
  %3 = load i1, ptr addrspace(5) %"56", align 1
  %4 = insertvalue %struct.i32.i1 undef, i32 %2, 0
  %5 = insertvalue %struct.i32.i1 %4, i1 %3, 1
  ret %struct.i32.i1 %5
}

define amdgpu_kernel void @multiple_return(ptr addrspace(4) byref(i64) %"59", ptr addrspace(4) byref(i64) %"60") #1 {
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i32, align 4, addrspace(5)
  %"65" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"53"

"53":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"59", align 8
  store i64 %2, ptr addrspace(5) %"61", align 8
  %3 = load i64, ptr addrspace(4) %"60", align 8
  store i64 %3, ptr addrspace(5) %"62", align 8
  %4 = load i64, ptr addrspace(5) %"61", align 8
  %"77" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"77", align 4
  store i32 %5, ptr addrspace(5) %"63", align 4
  %6 = load i32, ptr addrspace(5) %"63", align 4
  %7 = call %struct.i32.i1 @do_something(i32 %6)
  %"70" = extractvalue %struct.i32.i1 %7, 0
  %"71" = extractvalue %struct.i32.i1 %7, 1
  store i32 %"70", ptr addrspace(5) %"64", align 4
  store i1 %"71", ptr addrspace(5) %"65", align 1
  br label %"54"

"54":                                             ; preds = %"53"
  %8 = load i64, ptr addrspace(5) %"62", align 8
  %9 = load i32, ptr addrspace(5) %"64", align 4
  %"78" = inttoptr i64 %8 to ptr
  store i32 %9, ptr %"78", align 4
  %10 = load i1, ptr addrspace(5) %"65", align 1
  br i1 %10, label %"22", label %"23"

"22":                                             ; preds = %"54"
  %11 = load i64, ptr addrspace(5) %"62", align 8
  %"79" = inttoptr i64 %11 to ptr
  %"50" = getelementptr inbounds i8, ptr %"79", i64 4
  store i32 123, ptr %"50", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"54"
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
