@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define [2 x i32] @do_something(i32 %"10") #0 {
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %"48" = add i32 %"10", 1
  store i32 %"48", ptr addrspace(5) %"46", align 4
  store i1 true, ptr addrspace(5) %"47", align 1
  %2 = load i32, ptr addrspace(5) %"46", align 4
  %3 = load i1, ptr addrspace(5) %"47", align 1
  %4 = insertvalue [2 x i32] poison, i32 %2, 0
  %5 = zext i1 %3 to i32
  %6 = insertvalue [2 x i32] %4, i32 %5, 1
  ret [2 x i32] %6
}

define amdgpu_kernel void @multiple_return(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"57" = load i64, ptr addrspace(4) %"50", align 4
  store i64 %"57", ptr addrspace(5) %"52", align 4
  %"58" = load i64, ptr addrspace(4) %"51", align 4
  store i64 %"58", ptr addrspace(5) %"53", align 4
  %"60" = load i64, ptr addrspace(5) %"52", align 4
  %"68" = inttoptr i64 %"60" to ptr
  %"59" = load i32, ptr %"68", align 4
  store i32 %"59", ptr addrspace(5) %"54", align 4
  %"63" = load i32, ptr addrspace(5) %"54", align 4
  %2 = call [2 x i32] @do_something(i32 %"63")
  %"61" = extractvalue [2 x i32] %2, 0
  %3 = extractvalue [2 x i32] %2, 1
  %"62" = trunc i32 %3 to i1
  store i32 %"61", ptr addrspace(5) %"55", align 4
  store i1 %"62", ptr addrspace(5) %"56", align 1
  br label %"45"

"45":                                             ; preds = %"44"
  %"64" = load i64, ptr addrspace(5) %"53", align 4
  %"65" = load i32, ptr addrspace(5) %"55", align 4
  %"69" = inttoptr i64 %"64" to ptr
  store i32 %"65", ptr %"69", align 4
  %"66" = load i1, ptr addrspace(5) %"56", align 1
  br i1 %"66", label %"19", label %"20"

"19":                                             ; preds = %"45"
  %"67" = load i64, ptr addrspace(5) %"53", align 4
  %"70" = inttoptr i64 %"67" to ptr
  %"41" = getelementptr inbounds i8, ptr %"70", i64 4
  store i32 123, ptr %"41", align 4
  br label %"20"

"20":                                             ; preds = %"19", %"45"
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }