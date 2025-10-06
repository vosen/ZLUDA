define amdgpu_kernel void @mad_wide(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %2, ptr addrspace(5) %"45", align 8
  %3 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %3, ptr addrspace(5) %"46", align 8
  %4 = load i64, ptr addrspace(5) %"45", align 8
  %"65" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"65", align 4
  store i32 %5, ptr addrspace(5) %"48", align 4
  %6 = load i64, ptr addrspace(5) %"45", align 8
  %"66" = inttoptr i64 %6 to ptr
  %"39" = getelementptr inbounds i8, ptr %"66", i64 4
  %7 = load i32, ptr %"39", align 4
  store i32 %7, ptr addrspace(5) %"49", align 4
  %8 = load i64, ptr addrspace(5) %"45", align 8
  %"67" = inttoptr i64 %8 to ptr
  %"41" = getelementptr inbounds i8, ptr %"67", i64 8
  %9 = load i64, ptr %"41", align 8
  store i64 %9, ptr addrspace(5) %"50", align 8
  %10 = load i32, ptr addrspace(5) %"48", align 4
  %11 = load i32, ptr addrspace(5) %"49", align 4
  %12 = load i64, ptr addrspace(5) %"50", align 8
  %13 = sext i32 %10 to i64
  %14 = sext i32 %11 to i64
  %15 = mul i64 %13, %14
  %"59" = add i64 %15, %12
  store i64 %"59", ptr addrspace(5) %"47", align 8
  %16 = load i64, ptr addrspace(5) %"46", align 8
  %17 = load i64, ptr addrspace(5) %"47", align 8
  %"68" = inttoptr i64 %16 to ptr
  store i64 %17, ptr %"68", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }