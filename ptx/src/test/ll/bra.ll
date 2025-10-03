define amdgpu_kernel void @bra(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"46" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"46", ptr addrspace(5) %"42", align 8
  %"48" = load i64, ptr addrspace(5) %"41", align 8
  %"53" = inttoptr i64 %"48" to ptr
  %"47" = load i64, ptr %"53", align 8
  store i64 %"47", ptr addrspace(5) %"43", align 8
  br label %"11"

"11":                                             ; preds = %"38"
  %"50" = load i64, ptr addrspace(5) %"43", align 8
  %"49" = add i64 %"50", 1
  store i64 %"49", ptr addrspace(5) %"44", align 8
  br label %"13"

"13":                                             ; preds = %"11"
  %"51" = load i64, ptr addrspace(5) %"42", align 8
  %"52" = load i64, ptr addrspace(5) %"44", align 8
  %"54" = inttoptr i64 %"51" to ptr
  store i64 %"52", ptr %"54", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
