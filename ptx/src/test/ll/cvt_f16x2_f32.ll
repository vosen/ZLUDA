define amdgpu_kernel void @cvt_f16x2_f32(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca <2 x half>, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"59" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"59", align 4
  %"49" = bitcast float %5 to i32
  store i32 %"49", ptr addrspace(5) %"44", align 4
  %6 = load i64, ptr addrspace(5) %"42", align 8
  %"60" = inttoptr i64 %6 to ptr
  %"38" = getelementptr inbounds i8, ptr %"60", i64 4
  %7 = load float, ptr %"38", align 4
  %"52" = bitcast float %7 to i32
  store i32 %"52", ptr addrspace(5) %"45", align 4
  %8 = load i32, ptr addrspace(5) %"44", align 4
  %9 = load i32, ptr addrspace(5) %"45", align 4
  %"62" = bitcast i32 %8 to float
  %"63" = bitcast i32 %9 to float
  %10 = fptrunc float %"62" to half
  %11 = fptrunc float %"63" to half
  %12 = insertelement <2 x half> poison, half %10, i32 1
  %"53" = insertelement <2 x half> %12, half %11, i32 0
  store <2 x half> %"53", ptr addrspace(5) %"46", align 4
  %13 = load i64, ptr addrspace(5) %"43", align 8
  %14 = load <2 x half>, ptr addrspace(5) %"46", align 4
  %"64" = inttoptr i64 %13 to ptr addrspace(1)
  %"65" = bitcast <2 x half> %14 to i32
  store i32 %"65", ptr addrspace(1) %"64", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }