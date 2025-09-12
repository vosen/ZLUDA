define amdgpu_kernel void @vector_operand(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i16, align 2, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(5) %"38", align 8
  %"50" = inttoptr i64 %"45" to ptr
  %"44" = load i16, ptr %"50", align 2
  store i16 %"44", ptr addrspace(5) %"40", align 2
  %"46" = load i16, ptr addrspace(5) %"40", align 2
  %"34" = insertelement <2 x i16> <i16 22136, i16 undef>, i16 %"46", i8 1
  %"51" = bitcast <2 x i16> %"34" to i32
  store i32 %"51", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"39", align 8
  %"49" = load i32, ptr addrspace(5) %"41", align 4
  %"52" = inttoptr i64 %"48" to ptr
  store i32 %"49", ptr %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }