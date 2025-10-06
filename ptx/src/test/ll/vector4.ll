define amdgpu_kernel void @vector4(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca <4 x i32>, align 16, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %2, ptr addrspace(5) %"40", align 8
  %3 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %3, ptr addrspace(5) %"41", align 8
  %4 = load i64, ptr addrspace(5) %"40", align 8
  %"52" = inttoptr i64 %4 to ptr
  %5 = load <4 x i32>, ptr %"52", align 16
  store <4 x i32> %5, ptr addrspace(5) %"42", align 16
  %6 = load <4 x i32>, ptr addrspace(5) %"42", align 16
  %"36" = extractelement <4 x i32> %6, i8 3
  store i32 %"36", ptr addrspace(5) %"43", align 4
  %7 = load i64, ptr addrspace(5) %"41", align 8
  %8 = load i32, ptr addrspace(5) %"43", align 4
  %"55" = inttoptr i64 %7 to ptr
  store i32 %8, ptr %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }