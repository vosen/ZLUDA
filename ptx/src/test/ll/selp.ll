define amdgpu_kernel void @selp(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i16, align 2, addrspace(5)
  %"45" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"57" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"57", align 2
  store i16 %5, ptr addrspace(5) %"44", align 2
  %6 = load i64, ptr addrspace(5) %"42", align 8
  %"58" = inttoptr i64 %6 to ptr
  %"37" = getelementptr inbounds i8, ptr %"58", i64 2
  %7 = load i16, ptr %"37", align 2
  store i16 %7, ptr addrspace(5) %"45", align 2
  %8 = load i16, ptr addrspace(5) %"44", align 2
  %9 = load i16, ptr addrspace(5) %"45", align 2
  %"52" = select i1 false, i16 %8, i16 %9
  store i16 %"52", ptr addrspace(5) %"44", align 2
  %10 = load i64, ptr addrspace(5) %"43", align 8
  %11 = load i16, ptr addrspace(5) %"44", align 2
  %"59" = inttoptr i64 %10 to ptr
  store i16 %11, ptr %"59", align 2
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }