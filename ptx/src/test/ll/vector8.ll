define amdgpu_kernel void @vector8(ptr addrspace(4) byref(i64) %"48", ptr addrspace(4) byref(i64) %"49") #0 {
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i32, align 4, addrspace(5)
  %"60" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"48", align 8
  store i64 %2, ptr addrspace(5) %"50", align 8
  %3 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %3, ptr addrspace(5) %"51", align 8
  %4 = load i64, ptr addrspace(5) %"50", align 8
  %"76" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load <8 x i32>, ptr addrspace(1) %"76", align 32
  %"64" = extractelement <8 x i32> %5, i8 0
  %"65" = extractelement <8 x i32> %5, i8 1
  %"66" = extractelement <8 x i32> %5, i8 2
  %"67" = extractelement <8 x i32> %5, i8 3
  %"68" = extractelement <8 x i32> %5, i8 4
  %"69" = extractelement <8 x i32> %5, i8 5
  %"70" = extractelement <8 x i32> %5, i8 6
  %"71" = extractelement <8 x i32> %5, i8 7
  store i32 %"64", ptr addrspace(5) %"52", align 4
  store i32 %"65", ptr addrspace(5) %"53", align 4
  store i32 %"66", ptr addrspace(5) %"54", align 4
  store i32 %"67", ptr addrspace(5) %"55", align 4
  store i32 %"68", ptr addrspace(5) %"56", align 4
  store i32 %"69", ptr addrspace(5) %"57", align 4
  store i32 %"70", ptr addrspace(5) %"58", align 4
  store i32 %"71", ptr addrspace(5) %"59", align 4
  %6 = load i32, ptr addrspace(5) %"59", align 4
  store i32 %6, ptr addrspace(5) %"60", align 4
  %7 = load i64, ptr addrspace(5) %"51", align 8
  %8 = load i32, ptr addrspace(5) %"60", align 4
  %"79" = inttoptr i64 %7 to ptr
  store i32 %8, ptr %"79", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
