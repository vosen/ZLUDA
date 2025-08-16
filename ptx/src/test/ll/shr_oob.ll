define amdgpu_kernel void @shr_oob(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #0 {
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"36" = load i64, ptr addrspace(4) %"31", align 8
  store i64 %"36", ptr addrspace(5) %"33", align 8
  %"37" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"37", ptr addrspace(5) %"34", align 8
  %"39" = load i64, ptr addrspace(5) %"33", align 8
  %"44" = inttoptr i64 %"39" to ptr
  %"38" = load i16, ptr %"44", align 2
  store i16 %"38", ptr addrspace(5) %"35", align 2
  %"41" = load i16, ptr addrspace(5) %"35", align 2
  %2 = ashr i16 %"41", 15
  %3 = ashr i16 %"41", 16
  %"40" = select i1 true, i16 %2, i16 %3
  store i16 %"40", ptr addrspace(5) %"35", align 2
  %"42" = load i64, ptr addrspace(5) %"34", align 8
  %"43" = load i16, ptr addrspace(5) %"35", align 2
  %"45" = inttoptr i64 %"42" to ptr
  store i16 %"43", ptr %"45", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }