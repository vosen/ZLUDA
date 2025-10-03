define amdgpu_kernel void @vector4(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca <4 x i32>, align 16, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"44" = load i64, ptr addrspace(5) %"37", align 8
  %"49" = inttoptr i64 %"44" to ptr
  %"43" = load <4 x i32>, ptr %"49", align 16
  store <4 x i32> %"43", ptr addrspace(5) %"39", align 16
  %"45" = load <4 x i32>, ptr addrspace(5) %"39", align 16
  %"33" = extractelement <4 x i32> %"45", i8 3
  store i32 %"33", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(5) %"38", align 8
  %"48" = load i32, ptr addrspace(5) %"40", align 4
  %"52" = inttoptr i64 %"47" to ptr
  store i32 %"48", ptr %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
