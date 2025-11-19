define amdgpu_kernel void @cvt_rn_bf16x2_f32(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"58" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"58", align 4
  store float %5, ptr addrspace(5) %"44", align 4
  %6 = load i64, ptr addrspace(5) %"42", align 8
  %"59" = inttoptr i64 %6 to ptr
  %"38" = getelementptr inbounds i8, ptr %"59", i64 4
  %7 = load float, ptr %"38", align 4
  store float %7, ptr addrspace(5) %"45", align 4
  %8 = load float, ptr addrspace(5) %"44", align 4
  %9 = load float, ptr addrspace(5) %"45", align 4
  %10 = fptrunc float %8 to bfloat
  %11 = fptrunc float %9 to bfloat
  %12 = insertelement <2 x bfloat> poison, bfloat %10, i32 1
  %"60" = insertelement <2 x bfloat> %12, bfloat %11, i32 0
  %"53" = bitcast <2 x bfloat> %"60" to i32
  store i32 %"53", ptr addrspace(5) %"46", align 4
  %13 = load i64, ptr addrspace(5) %"43", align 8
  %14 = load i32, ptr addrspace(5) %"46", align 4
  %"61" = inttoptr i64 %13 to ptr
  store i32 %14, ptr %"61", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }