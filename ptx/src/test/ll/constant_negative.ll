define amdgpu_kernel void @constant_negative(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #0 {
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"36" = load i64, ptr addrspace(4) %"31", align 4
  store i64 %"36", ptr addrspace(5) %"33", align 4
  %"37" = load i64, ptr addrspace(4) %"32", align 4
  store i64 %"37", ptr addrspace(5) %"34", align 4
  %"39" = load i64, ptr addrspace(5) %"33", align 4
  %"44" = inttoptr i64 %"39" to ptr
  %"38" = load i32, ptr %"44", align 4
  store i32 %"38", ptr addrspace(5) %"35", align 4
  %"41" = load i32, ptr addrspace(5) %"35", align 4
  %"40" = mul i32 %"41", -1
  store i32 %"40", ptr addrspace(5) %"35", align 4
  %"42" = load i64, ptr addrspace(5) %"34", align 4
  %"43" = load i32, ptr addrspace(5) %"35", align 4
  %"45" = inttoptr i64 %"42" to ptr
  store i32 %"43", ptr %"45", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }