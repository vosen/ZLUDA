define amdgpu_kernel void @add_non_coherent(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  store i64 1, ptr addrspace(5) %"45", align 4
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"44" = load i64, ptr addrspace(5) %"37", align 8
  %"51" = inttoptr i64 %"44" to ptr addrspace(1)
  %"43" = load i64, ptr addrspace(1) %"51", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"48" = load i64, ptr addrspace(5) %"45", align 8
  %"46" = add i64 %"47", %"48"
  store i64 %"46", ptr addrspace(5) %"40", align 8
  %"49" = load i64, ptr addrspace(5) %"38", align 8
  %"50" = load i64, ptr addrspace(5) %"40", align 8
  %"52" = inttoptr i64 %"49" to ptr addrspace(1)
  store i64 %"50", ptr addrspace(1) %"52", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }