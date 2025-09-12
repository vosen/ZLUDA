define amdgpu_kernel void @local_align(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"11" = alloca [8 x i8], align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"40" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(5) %"36", align 8
  %"45" = inttoptr i64 %"42" to ptr
  %"41" = load i64, ptr %"45", align 8
  store i64 %"41", ptr addrspace(5) %"38", align 8
  %"43" = load i64, ptr addrspace(5) %"37", align 8
  %"44" = load i64, ptr addrspace(5) %"38", align 8
  %"46" = inttoptr i64 %"43" to ptr
  store i64 %"44", ptr %"46", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }