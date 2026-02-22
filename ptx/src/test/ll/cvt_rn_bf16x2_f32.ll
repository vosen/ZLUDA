define amdgpu_kernel void @cvt_rn_bf16x2_f32(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca float, align 4, addrspace(5)
  %"48" = alloca float, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %2, ptr addrspace(5) %"45", align 8
  %3 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %3, ptr addrspace(5) %"46", align 8
  %4 = load i64, ptr addrspace(5) %"45", align 8
  %"61" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"61", align 4
  store float %5, ptr addrspace(5) %"47", align 4
  %6 = load i64, ptr addrspace(5) %"45", align 8
  %"62" = inttoptr i64 %6 to ptr
  %"41" = getelementptr inbounds i8, ptr %"62", i64 4
  %7 = load float, ptr %"41", align 4
  store float %7, ptr addrspace(5) %"48", align 4
  %8 = load float, ptr addrspace(5) %"47", align 4
  %9 = load float, ptr addrspace(5) %"48", align 4
  %10 = fptrunc float %8 to bfloat
  %11 = fptrunc float %9 to bfloat
  %12 = insertelement <2 x bfloat> poison, bfloat %10, i32 1
  %"63" = insertelement <2 x bfloat> %12, bfloat %11, i32 0
  %"56" = bitcast <2 x bfloat> %"63" to i32
  store i32 %"56", ptr addrspace(5) %"49", align 4
  %13 = load i64, ptr addrspace(5) %"46", align 8
  %14 = load i32, ptr addrspace(5) %"49", align 4
  %"64" = inttoptr i64 %13 to ptr
  store i32 %14, ptr %"64", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
