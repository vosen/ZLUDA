define amdgpu_kernel void @selp(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i16, align 2, addrspace(5)
  %"48" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %2, ptr addrspace(5) %"45", align 8
  %3 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %3, ptr addrspace(5) %"46", align 8
  %4 = load i64, ptr addrspace(5) %"45", align 8
  %"60" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"60", align 2
  store i16 %5, ptr addrspace(5) %"47", align 2
  %6 = load i64, ptr addrspace(5) %"45", align 8
  %"61" = inttoptr i64 %6 to ptr
  %"40" = getelementptr inbounds i8, ptr %"61", i64 2
  %7 = load i16, ptr %"40", align 2
  store i16 %7, ptr addrspace(5) %"48", align 2
  %8 = load i16, ptr addrspace(5) %"47", align 2
  %9 = load i16, ptr addrspace(5) %"48", align 2
  %"55" = select i1 false, i16 %8, i16 %9
  store i16 %"55", ptr addrspace(5) %"47", align 2
  %10 = load i64, ptr addrspace(5) %"46", align 8
  %11 = load i16, ptr addrspace(5) %"47", align 2
  %"62" = inttoptr i64 %10 to ptr
  store i16 %11, ptr %"62", align 2
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
