define amdgpu_kernel void @reg_multi(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %2, ptr addrspace(5) %"43", align 8
  %3 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %3, ptr addrspace(5) %"44", align 8
  %4 = load i64, ptr addrspace(5) %"43", align 8
  %"57" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"57", align 4
  store i32 %5, ptr addrspace(5) %"45", align 4
  %6 = load i64, ptr addrspace(5) %"43", align 8
  %"58" = inttoptr i64 %6 to ptr
  %"37" = getelementptr inbounds i8, ptr %"58", i64 4
  %7 = load i32, ptr %"37", align 4
  store i32 %7, ptr addrspace(5) %"46", align 4
  %8 = load i64, ptr addrspace(5) %"44", align 8
  %9 = load i32, ptr addrspace(5) %"45", align 4
  %"59" = inttoptr i64 %8 to ptr
  store i32 %9, ptr %"59", align 4
  %10 = load i64, ptr addrspace(5) %"44", align 8
  %"60" = inttoptr i64 %10 to ptr
  %"39" = getelementptr inbounds i8, ptr %"60", i64 4
  %11 = load i32, ptr addrspace(5) %"46", align 4
  store i32 %11, ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }