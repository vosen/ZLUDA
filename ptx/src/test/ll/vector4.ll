define amdgpu_kernel void @vector4(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca <4 x i32>, align 16, addrspace(5)
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
  %"55" = inttoptr i64 %4 to ptr
  %5 = load <4 x i32>, ptr %"55", align 16
  store <4 x i32> %5, ptr addrspace(5) %"45", align 16
  %6 = load <4 x i32>, ptr addrspace(5) %"45", align 16
  %"39" = extractelement <4 x i32> %6, i8 3
  store i32 %"39", ptr addrspace(5) %"46", align 4
  %7 = load i64, ptr addrspace(5) %"44", align 8
  %8 = load i32, ptr addrspace(5) %"46", align 4
  %"58" = inttoptr i64 %7 to ptr
  store i32 %8, ptr %"58", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
