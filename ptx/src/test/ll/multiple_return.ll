%struct.i32.i1 = type { i32, i1 }

@0 = addrspace(4) global i32 1
@1 = addrspace(4) global i32 0
@2 = addrspace(4) global i32 0
@3 = addrspace(4) global i64 4
@4 = addrspace(4) global i32 123

define hidden %struct.i32.i1 @do_something(i32 %"11") #0 {
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  %"41" = load i32, ptr addrspace(4) @0, align 4
  %"56" = add i32 %"11", %"41"
  store i32 %"56", ptr addrspace(5) %"54", align 4
  %"43" = load i32, ptr addrspace(4) @1, align 4
  %"45" = load i32, ptr addrspace(4) @2, align 4
  %2 = icmp eq i32 %"43", %"45"
  store i1 %2, ptr addrspace(5) %"55", align 1
  %3 = load i32, ptr addrspace(5) %"54", align 4
  %4 = load i1, ptr addrspace(5) %"55", align 1
  %5 = insertvalue %struct.i32.i1 undef, i32 %3, 0
  %6 = insertvalue %struct.i32.i1 %5, i1 %4, 1
  ret %struct.i32.i1 %6
}

define amdgpu_kernel void @multiple_return(ptr addrspace(4) byref(i64) %"58", ptr addrspace(4) byref(i64) %"59") #1 {
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i32, align 4, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"52"

"52":                                             ; preds = %1
  %"65" = load i64, ptr addrspace(4) %"58", align 8
  store i64 %"65", ptr addrspace(5) %"60", align 8
  %"66" = load i64, ptr addrspace(4) %"59", align 8
  store i64 %"66", ptr addrspace(5) %"61", align 8
  %"68" = load i64, ptr addrspace(5) %"60", align 8
  %"76" = inttoptr i64 %"68" to ptr
  %"67" = load i32, ptr %"76", align 4
  store i32 %"67", ptr addrspace(5) %"62", align 4
  %"71" = load i32, ptr addrspace(5) %"62", align 4
  %2 = call %struct.i32.i1 @do_something(i32 %"71")
  %"69" = extractvalue %struct.i32.i1 %2, 0
  %"70" = extractvalue %struct.i32.i1 %2, 1
  store i32 %"69", ptr addrspace(5) %"63", align 4
  store i1 %"70", ptr addrspace(5) %"64", align 1
  br label %"53"

"53":                                             ; preds = %"52"
  %"72" = load i64, ptr addrspace(5) %"61", align 8
  %"73" = load i32, ptr addrspace(5) %"63", align 4
  %"77" = inttoptr i64 %"72" to ptr
  store i32 %"73", ptr %"77", align 4
  %"74" = load i1, ptr addrspace(5) %"64", align 1
  br i1 %"74", label %"20", label %"21"

"20":                                             ; preds = %"53"
  %"47" = load i64, ptr addrspace(4) @3, align 8
  %"75" = load i64, ptr addrspace(5) %"61", align 8
  %"78" = inttoptr i64 %"75" to ptr
  %"48" = getelementptr inbounds i8, ptr %"78", i64 %"47"
  %"50" = load i32, ptr addrspace(4) @4, align 4
  store i32 %"50", ptr %"48", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"53"
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }