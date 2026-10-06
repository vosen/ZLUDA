define amdgpu_kernel void @sink(ptr addrspace(4) byref(i64) %"51", ptr addrspace(4) byref(i64) %"52") #0 {
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"51", align 8
  store i64 %2, ptr addrspace(5) %"53", align 8
  %3 = load i64, ptr addrspace(4) %"52", align 8
  store i64 %3, ptr addrspace(5) %"54", align 8
  %4 = load i64, ptr addrspace(5) %"53", align 8
  %"77" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"77", align 8
  store i64 %5, ptr addrspace(5) %"55", align 8
  %6 = load i64, ptr addrspace(5) %"55", align 8
  %"41" = bitcast i64 %6 to <2 x i32>
  %7 = load i64, ptr addrspace(5) %"55", align 8
  %"42" = bitcast i64 %7 to <2 x i32>
  %"80" = extractelement <2 x i32> %"42", i8 0
  store i32 %"80", ptr addrspace(5) %"56", align 4
  %8 = load i64, ptr addrspace(5) %"55", align 8
  %"43" = bitcast i64 %8 to <2 x i32>
  %"82" = extractelement <2 x i32> %"43", i8 1
  store i32 %"82", ptr addrspace(5) %"57", align 4
  %9 = load i64, ptr addrspace(5) %"53", align 8
  %"83" = inttoptr i64 %9 to ptr
  %10 = load <4 x i32>, ptr %"83", align 16
  %"69" = extractelement <4 x i32> %10, i8 2
  store i32 %"69", ptr addrspace(5) %"58", align 4
  %11 = load i64, ptr addrspace(5) %"53", align 8
  %"84" = inttoptr i64 %11 to ptr
  %12 = load <4 x i32>, ptr %"84", align 16
  %13 = load i64, ptr addrspace(5) %"54", align 8
  %14 = load i32, ptr addrspace(5) %"57", align 4
  %"85" = inttoptr i64 %13 to ptr
  store i32 %14, ptr %"85", align 4
  %15 = load i64, ptr addrspace(5) %"54", align 8
  %"86" = inttoptr i64 %15 to ptr
  %"47" = getelementptr inbounds i8, ptr %"86", i64 4
  %16 = load i32, ptr addrspace(5) %"56", align 4
  store i32 %16, ptr %"47", align 4
  %17 = load i64, ptr addrspace(5) %"54", align 8
  %"87" = inttoptr i64 %17 to ptr
  %"49" = getelementptr inbounds i8, ptr %"87", i64 8
  %18 = load i32, ptr addrspace(5) %"58", align 4
  store i32 %18, ptr %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
