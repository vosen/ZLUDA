%struct.i32.i1 = type { i32, i1 }

define hidden %struct.i32.i1 @do_something(i32 %"12") #0 {
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"49"

"49":                                             ; preds = %1
  %"54" = add i32 %"12", 1
  store i32 %"54", ptr addrspace(5) %"52", align 4
  store i1 true, ptr addrspace(5) %"53", align 1
  %2 = load i32, ptr addrspace(5) %"52", align 4
  %3 = load i1, ptr addrspace(5) %"53", align 1
  %4 = insertvalue %struct.i32.i1 undef, i32 %2, 0
  %5 = insertvalue %struct.i32.i1 %4, i1 %3, 1
  ret %struct.i32.i1 %5
}

define amdgpu_kernel void @multiple_return(ptr addrspace(4) byref(i64) %"56", ptr addrspace(4) byref(i64) %"57") #1 {
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i32, align 4, addrspace(5)
  %"61" = alloca i32, align 4, addrspace(5)
  %"62" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"56", align 8
  store i64 %2, ptr addrspace(5) %"58", align 8
  %3 = load i64, ptr addrspace(4) %"57", align 8
  store i64 %3, ptr addrspace(5) %"59", align 8
  %4 = load i64, ptr addrspace(5) %"58", align 8
  %"74" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"74", align 4
  store i32 %5, ptr addrspace(5) %"60", align 4
  %6 = load i32, ptr addrspace(5) %"60", align 4
  %7 = call %struct.i32.i1 @do_something(i32 %6)
  %"67" = extractvalue %struct.i32.i1 %7, 0
  %"68" = extractvalue %struct.i32.i1 %7, 1
  store i32 %"67", ptr addrspace(5) %"61", align 4
  store i1 %"68", ptr addrspace(5) %"62", align 1
  br label %"51"

"51":                                             ; preds = %"50"
  %8 = load i64, ptr addrspace(5) %"59", align 8
  %9 = load i32, ptr addrspace(5) %"61", align 4
  %"75" = inttoptr i64 %8 to ptr
  store i32 %9, ptr %"75", align 4
  %10 = load i1, ptr addrspace(5) %"62", align 1
  br i1 %10, label %"21", label %"22"

"21":                                             ; preds = %"51"
  %11 = load i64, ptr addrspace(5) %"59", align 8
  %"76" = inttoptr i64 %11 to ptr
  %"47" = getelementptr inbounds i8, ptr %"76", i64 4
  store i32 123, ptr %"47", align 4
  br label %"22"

"22":                                             ; preds = %"21", %"51"
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }