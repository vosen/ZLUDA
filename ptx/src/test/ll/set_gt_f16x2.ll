define amdgpu_kernel void @set_gt_f16x2(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca <2 x half>, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"48", align 8
  %3 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %3, ptr addrspace(5) %"49", align 8
  %4 = load i64, ptr addrspace(5) %"48", align 8
  %"65" = inttoptr i64 %4 to ptr
  %"42" = getelementptr inbounds i8, ptr %"65", i64 4
  %5 = load i32, ptr %"42", align 4
  store i32 %5, ptr addrspace(5) %"50", align 4
  %6 = load i64, ptr addrspace(5) %"48", align 8
  %"66" = inttoptr i64 %6 to ptr
  %"44" = getelementptr inbounds i8, ptr %"66", i64 8
  %7 = load i32, ptr %"44", align 4
  store i32 %7, ptr addrspace(5) %"51", align 4
  %8 = load i32, ptr addrspace(5) %"50", align 4
  %9 = load i32, ptr addrspace(5) %"51", align 4
  %"68" = bitcast i32 %8 to <2 x half>
  %"69" = bitcast i32 %9 to <2 x half>
  %10 = extractelement <2 x half> %"68", i8 0
  %11 = extractelement <2 x half> %"68", i8 1
  %12 = extractelement <2 x half> %"69", i8 0
  %13 = extractelement <2 x half> %"69", i8 1
  %14 = fcmp ogt half %10, %12
  %15 = fcmp ogt half %11, %13
  %16 = select i1 %14, half 0xH3C00, half 0xH0000
  %17 = select i1 %15, half 0xH3C00, half 0xH0000
  %18 = insertelement <2 x half> poison, half %16, i8 0
  %19 = insertelement <2 x half> %18, half %17, i8 1
  %"60" = bitcast <2 x half> %19 to i32
  store i32 %"60", ptr addrspace(5) %"52", align 4
  %20 = load i64, ptr addrspace(5) %"49", align 8
  %21 = load i32, ptr addrspace(5) %"52", align 4
  %"70" = inttoptr i64 %20 to ptr
  store i32 %21, ptr %"70", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
