@constparams = addrspace(4) global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

define amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"52", ptr addrspace(4) byref(i64) %"53") #0 {
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  %"57" = alloca i16, align 2, addrspace(5)
  %"58" = alloca i16, align 2, addrspace(5)
  %"59" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"52", align 8
  store i64 %2, ptr addrspace(5) %"54", align 8
  %3 = load i64, ptr addrspace(4) %"53", align 8
  store i64 %3, ptr addrspace(5) %"55", align 8
  %4 = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %4, ptr addrspace(5) %"56", align 2
  %5 = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 2), align 2
  store i16 %5, ptr addrspace(5) %"57", align 2
  %6 = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 4), align 2
  store i16 %6, ptr addrspace(5) %"58", align 2
  %7 = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 6), align 2
  store i16 %7, ptr addrspace(5) %"59", align 2
  %8 = load i64, ptr addrspace(5) %"55", align 8
  %9 = load i16, ptr addrspace(5) %"56", align 2
  %"78" = inttoptr i64 %8 to ptr
  store i16 %9, ptr %"78", align 2
  %10 = load i64, ptr addrspace(5) %"55", align 8
  %"80" = inttoptr i64 %10 to ptr
  %"46" = getelementptr inbounds i8, ptr %"80", i64 2
  %11 = load i16, ptr addrspace(5) %"57", align 2
  store i16 %11, ptr %"46", align 2
  %12 = load i64, ptr addrspace(5) %"55", align 8
  %"82" = inttoptr i64 %12 to ptr
  %"48" = getelementptr inbounds i8, ptr %"82", i64 4
  %13 = load i16, ptr addrspace(5) %"58", align 2
  store i16 %13, ptr %"48", align 2
  %14 = load i64, ptr addrspace(5) %"55", align 8
  %"84" = inttoptr i64 %14 to ptr
  %"50" = getelementptr inbounds i8, ptr %"84", i64 6
  %15 = load i16, ptr addrspace(5) %"59", align 2
  store i16 %15, ptr %"50", align 2
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }