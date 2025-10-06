define amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"12" = alloca [8 x i8], align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %2, ptr addrspace(5) %"45", align 8
  %3 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %3, ptr addrspace(5) %"46", align 8
  %4 = load i64, ptr addrspace(5) %"45", align 8
  %"57" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"57", align 8
  store i64 %5, ptr addrspace(5) %"47", align 8
  %6 = load i64, ptr addrspace(5) %"47", align 8
  %"37" = add i64 %6, 1
  %"58" = addrspacecast ptr addrspace(5) %"12" to ptr
  store i64 %"37", ptr %"58", align 8
  %"60" = addrspacecast ptr addrspace(5) %"12" to ptr
  %"39" = getelementptr inbounds i8, ptr %"60", i64 0
  %7 = load i64, ptr %"39", align 8
  store i64 %7, ptr addrspace(5) %"47", align 8
  %8 = load i64, ptr addrspace(5) %"46", align 8
  %"62" = inttoptr i64 %8 to ptr addrspace(1)
  %"41" = getelementptr inbounds i8, ptr addrspace(1) %"62", i64 0
  %9 = load i64, ptr addrspace(5) %"47", align 8
  store i64 %9, ptr addrspace(1) %"41", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }