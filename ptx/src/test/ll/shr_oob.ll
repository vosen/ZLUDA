define amdgpu_kernel void @shr_oob(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"40" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(5) %"36", align 8
  %"47" = inttoptr i64 %"42" to ptr
  %"41" = load i16, ptr %"47", align 2
  store i16 %"41", ptr addrspace(5) %"38", align 2
  %"44" = load i16, ptr addrspace(5) %"38", align 2
  %2 = ashr i16 %"44", 15
  %3 = ashr i16 %"44", 16
  %"43" = select i1 true, i16 %2, i16 %3
  store i16 %"43", ptr addrspace(5) %"38", align 2
  %"45" = load i64, ptr addrspace(5) %"37", align 8
  %"46" = load i16, ptr addrspace(5) %"38", align 2
  %"48" = inttoptr i64 %"45" to ptr
  store i16 %"46", ptr %"48", align 2
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }