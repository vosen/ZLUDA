define amdgpu_kernel void @malformed_label(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  br label %"12"

"12":                                             ; preds = %"38"
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"54" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"54", align 8
  store i64 %5, ptr addrspace(5) %"44", align 8
  %6 = load i64, ptr addrspace(5) %"44", align 8
  %"50" = add i64 %6, 1
  store i64 %"50", ptr addrspace(5) %"45", align 8
  %7 = load i64, ptr addrspace(5) %"43", align 8
  %8 = load i64, ptr addrspace(5) %"45", align 8
  %"55" = inttoptr i64 %7 to ptr
  store i64 %8, ptr %"55", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }