@constparams = addrspace(4) externally_initialized global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

define amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"55", ptr addrspace(4) byref(i64) %"56") #0 {
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i16, align 2, addrspace(5)
  %"60" = alloca i16, align 2, addrspace(5)
  %"61" = alloca i16, align 2, addrspace(5)
  %"62" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"54"

"54":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"55", align 8
  store i64 %2, ptr addrspace(5) %"57", align 8
  %3 = load i64, ptr addrspace(4) %"56", align 8
  store i64 %3, ptr addrspace(5) %"58", align 8
  %4 = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %4, ptr addrspace(5) %"59", align 2
  %5 = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 2), align 2
  store i16 %5, ptr addrspace(5) %"60", align 2
  %6 = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 4), align 2
  store i16 %6, ptr addrspace(5) %"61", align 2
  %7 = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 6), align 2
  store i16 %7, ptr addrspace(5) %"62", align 2
  %8 = load i64, ptr addrspace(5) %"58", align 8
  %9 = load i16, ptr addrspace(5) %"59", align 2
  %"81" = inttoptr i64 %8 to ptr
  store i16 %9, ptr %"81", align 2
  %10 = load i64, ptr addrspace(5) %"58", align 8
  %"83" = inttoptr i64 %10 to ptr
  %"49" = getelementptr inbounds i8, ptr %"83", i64 2
  %11 = load i16, ptr addrspace(5) %"60", align 2
  store i16 %11, ptr %"49", align 2
  %12 = load i64, ptr addrspace(5) %"58", align 8
  %"85" = inttoptr i64 %12 to ptr
  %"51" = getelementptr inbounds i8, ptr %"85", i64 4
  %13 = load i16, ptr addrspace(5) %"61", align 2
  store i16 %13, ptr %"51", align 2
  %14 = load i64, ptr addrspace(5) %"58", align 8
  %"87" = inttoptr i64 %14 to ptr
  %"53" = getelementptr inbounds i8, ptr %"87", i64 6
  %15 = load i16, ptr addrspace(5) %"62", align 2
  store i16 %15, ptr %"53", align 2
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
