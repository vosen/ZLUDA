%struct.i32.i1 = type { i32, i1 }

define hidden %struct.i32.i1 @do_something(i32 %"11") #0 {
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i1, align 1, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"51", align 4
  %"54" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"54", align 4
  %"55" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"55", align 4
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %"53" = load i32, ptr addrspace(5) %"51", align 4
  %"52" = add i32 %"11", %"53"
  store i32 %"52", ptr addrspace(5) %"49", align 4
  %"57" = load i32, ptr addrspace(5) %"54", align 4
  %"58" = load i32, ptr addrspace(5) %"55", align 4
  %2 = icmp eq i32 %"57", %"58"
  store i1 %2, ptr addrspace(5) %"50", align 1
  %3 = load i32, ptr addrspace(5) %"49", align 4
  %4 = load i1, ptr addrspace(5) %"50", align 1
  %5 = insertvalue %struct.i32.i1 undef, i32 %3, 0
  %6 = insertvalue %struct.i32.i1 %5, i1 %4, 1
  ret %struct.i32.i1 %6
}

define amdgpu_kernel void @multiple_return(ptr addrspace(4) byref(i64) %"59", ptr addrspace(4) byref(i64) %"60") #1 {
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i32, align 4, addrspace(5)
  %"65" = alloca i1, align 1, addrspace(5)
  %"76" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"76", align 4
  %"79" = alloca i32, align 4, addrspace(5)
  store i32 123, ptr addrspace(5) %"79", align 4
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %"66" = load i64, ptr addrspace(4) %"59", align 8
  store i64 %"66", ptr addrspace(5) %"61", align 8
  %"67" = load i64, ptr addrspace(4) %"60", align 8
  store i64 %"67", ptr addrspace(5) %"62", align 8
  %"69" = load i64, ptr addrspace(5) %"61", align 8
  %"81" = inttoptr i64 %"69" to ptr
  %"68" = load i32, ptr %"81", align 4
  store i32 %"68", ptr addrspace(5) %"63", align 4
  %"72" = load i32, ptr addrspace(5) %"63", align 4
  %2 = call %struct.i32.i1 @do_something(i32 %"72")
  %"70" = extractvalue %struct.i32.i1 %2, 0
  %"71" = extractvalue %struct.i32.i1 %2, 1
  store i32 %"70", ptr addrspace(5) %"64", align 4
  store i1 %"71", ptr addrspace(5) %"65", align 1
  br label %"48"

"48":                                             ; preds = %"47"
  %"73" = load i64, ptr addrspace(5) %"62", align 8
  %"74" = load i32, ptr addrspace(5) %"64", align 4
  %"82" = inttoptr i64 %"73" to ptr
  store i32 %"74", ptr %"82", align 4
  %"75" = load i1, ptr addrspace(5) %"65", align 1
  br i1 %"75", label %"20", label %"21"

"20":                                             ; preds = %"48"
  %"77" = load i64, ptr addrspace(5) %"62", align 8
  %"78" = load i64, ptr addrspace(5) %"76", align 8
  %"83" = inttoptr i64 %"77" to ptr
  %"44" = getelementptr inbounds i8, ptr %"83", i64 %"78"
  %"80" = load i32, ptr addrspace(5) %"79", align 4
  store i32 %"80", ptr %"44", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"48"
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }