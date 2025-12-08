define amdgpu_kernel void @vector8(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %2, ptr addrspace(5) %"47", align 8
  %3 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %3, ptr addrspace(5) %"48", align 8
  %4 = load i64, ptr addrspace(5) %"47", align 8
  %"73" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load <8 x i32>, ptr addrspace(1) %"73", align 32
  %"61" = extractelement <8 x i32> %5, i8 0
  %"62" = extractelement <8 x i32> %5, i8 1
  %"63" = extractelement <8 x i32> %5, i8 2
  %"64" = extractelement <8 x i32> %5, i8 3
  %"65" = extractelement <8 x i32> %5, i8 4
  %"66" = extractelement <8 x i32> %5, i8 5
  %"67" = extractelement <8 x i32> %5, i8 6
  %"68" = extractelement <8 x i32> %5, i8 7
  store i32 %"61", ptr addrspace(5) %"49", align 4
  store i32 %"62", ptr addrspace(5) %"50", align 4
  store i32 %"63", ptr addrspace(5) %"51", align 4
  store i32 %"64", ptr addrspace(5) %"52", align 4
  store i32 %"65", ptr addrspace(5) %"53", align 4
  store i32 %"66", ptr addrspace(5) %"54", align 4
  store i32 %"67", ptr addrspace(5) %"55", align 4
  store i32 %"68", ptr addrspace(5) %"56", align 4
  %6 = load i32, ptr addrspace(5) %"56", align 4
  store i32 %6, ptr addrspace(5) %"57", align 4
  %7 = load i64, ptr addrspace(5) %"48", align 8
  %8 = load i32, ptr addrspace(5) %"57", align 4
  %"76" = inttoptr i64 %7 to ptr
  store i32 %8, ptr %"76", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
