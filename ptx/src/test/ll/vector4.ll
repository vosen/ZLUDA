define amdgpu_kernel void @vector4(ptr addrspace(4) byref(i64) %"32", ptr addrspace(4) byref(i64) %"33") #0 {
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca <4 x i32>, align 16, addrspace(5)
  %"37" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"38" = load i64, ptr addrspace(4) %"32", align 4
  store i64 %"38", ptr addrspace(5) %"34", align 4
  %"39" = load i64, ptr addrspace(4) %"33", align 4
  store i64 %"39", ptr addrspace(5) %"35", align 4
  %"41" = load i64, ptr addrspace(5) %"34", align 4
  %"46" = inttoptr i64 %"41" to ptr
  %"40" = load <4 x i32>, ptr %"46", align 16
  store <4 x i32> %"40", ptr addrspace(5) %"36", align 16
  %"42" = load <4 x i32>, ptr addrspace(5) %"36", align 16
  %"30" = extractelement <4 x i32> %"42", i8 3
  store i32 %"30", ptr addrspace(5) %"37", align 4
  %"44" = load i64, ptr addrspace(5) %"35", align 4
  %"45" = load i32, ptr addrspace(5) %"37", align 4
  %"49" = inttoptr i64 %"44" to ptr
  store i32 %"45", ptr %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }